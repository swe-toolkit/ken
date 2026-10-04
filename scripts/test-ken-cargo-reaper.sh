#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

mkdir -p "$scratch/tmp/ken-runtime-old" \
  "$scratch/tmp/ken-runtime-new" \
  "$scratch/tmp/rt-scalar-ac0" \
  "$scratch/bin" "$scratch/locks"
printf evidence > "$scratch/tmp/ken-runtime-old/data"
printf evidence > "$scratch/tmp/rt-scalar-ac0/log"
touch -d '3 hours ago' "$scratch/tmp/ken-runtime-old"
touch -d '3 hours ago' "$scratch/tmp/rt-scalar-ac0"

cat > "$scratch/bin/cargo" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "$scratch/bin/cargo"
real_flock=$(command -v flock)
cat > "$scratch/bin/flock" <<SH
#!/usr/bin/env bash
: > "$scratch/waiting-for-lock"
exec "$real_flock" "\$@"
SH
chmod +x "$scratch/bin/flock"

# A waiting invocation must not reap anything before it owns the build lock.
exec 9>"$scratch/locks/build.lock"
"$real_flock" 9
PATH="$scratch/bin:$PATH" \
KEN_TMPDIR="$scratch/tmp" \
KEN_LOCK_DIR="$scratch/locks" \
KEN_BUILD_WAIT=30 \
  "$root/scripts/ken-cargo" build -p ken-kernel &
wrapper=$!
for _ in {1..100}; do
  [[ -e "$scratch/waiting-for-lock" ]] && break
  sleep 0.02
done
[[ -e "$scratch/waiting-for-lock" ]]
[[ -d "$scratch/tmp/ken-runtime-old" ]]
[[ -d "$scratch/tmp/rt-scalar-ac0" ]]
"$real_flock" -u 9
wait "$wrapper"

# Once the exclusive lock is acquired, old named scratch is reaped, while
# unrelated evidence and recent scratch survive.
[[ ! -e "$scratch/tmp/ken-runtime-old" ]]
[[ -d "$scratch/tmp/rt-scalar-ac0" ]]
[[ -d "$scratch/tmp/ken-runtime-new" ]]
printf 'ken-cargo reaper: passed\n'
