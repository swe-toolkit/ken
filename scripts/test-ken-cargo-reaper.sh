#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
scratch=$(mktemp -d)
real_flock=$(command -v flock)
wrapper=
cleanup() {
  local status=$?
  trap - EXIT
  if [[ -n "$wrapper" ]]; then
    "$real_flock" -u 9 2>/dev/null || true
    wait "$wrapper" 2>/dev/null || true
  fi
  rm -rf -- "$scratch"
  exit "$status"
}
trap cleanup EXIT

assert_dir() {
  [[ -d "$1" ]] || { echo "expected directory: $1" >&2; exit 1; }
}
assert_absent() {
  [[ ! -e "$1" ]] || { echo "expected path to be reaped: $1" >&2; exit 1; }
}

mkdir -p "$scratch/tmp/ken-runtime-old" \
  "$scratch/tmp/ken-runtime-active" \
  "$scratch/tmp/ken-runtime-new" \
  "$scratch/tmp/rt-scalar-ac0" \
  "$scratch/bin" "$scratch/locks"
printf evidence > "$scratch/tmp/ken-runtime-old/data"
printf evidence > "$scratch/tmp/ken-runtime-active/log"
printf evidence > "$scratch/tmp/rt-scalar-ac0/log"
touch -d '3 hours ago' "$scratch/tmp/ken-runtime-old/data"
touch -d '3 hours ago' "$scratch/tmp/ken-runtime-old"
touch -d '3 hours ago' "$scratch/tmp/ken-runtime-active"
touch -d '3 hours ago' "$scratch/tmp/rt-scalar-ac0"
touch -d '3 hours ago' "$scratch/tmp/rt-scalar-ac0/log"
touch "$scratch/tmp/ken-runtime-active/log"

cat > "$scratch/bin/cargo" <<'SH'
#!/usr/bin/env bash
exit 0
SH
cat > "$scratch/bin/sem" <<'SH'
#!/usr/bin/env bash
: > "$KEN_TEST_SEM_RAN"
while [[ "$#" -gt 0 && "$1" != "--" ]]; do
  shift
done
[[ "$#" -gt 0 ]] || exit 64
shift
exec "$@"
SH
chmod +x "$scratch/bin/cargo" "$scratch/bin/sem"
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
if [[ ! -d "$scratch/tmp/ken-runtime-old" ]]; then
  echo 'reaper ran before the build lock was acquired' >&2
  exit 1
fi
assert_dir "$scratch/tmp/rt-scalar-ac0"
"$real_flock" -u 9
wait "$wrapper"

# Once the exclusive lock is acquired, old named scratch is reaped, while
# unrelated evidence and recent scratch survive.
assert_absent "$scratch/tmp/ken-runtime-old"
assert_dir "$scratch/tmp/ken-runtime-active"
assert_dir "$scratch/tmp/rt-scalar-ac0"
assert_dir "$scratch/tmp/ken-runtime-new"

# The concurrent semaphore route intentionally skips best-effort cleanup.
mkdir -p "$scratch/slots/tmp/ken-runtime-old" "$scratch/slots/locks"
printf evidence > "$scratch/slots/tmp/ken-runtime-old/data"
touch -d '3 hours ago' "$scratch/slots/tmp/ken-runtime-old/data"
touch -d '3 hours ago' "$scratch/slots/tmp/ken-runtime-old"
PATH="$scratch/bin:$PATH" \
KEN_BUILD_SLOTS=2 \
KEN_TEST_SEM_RAN="$scratch/sem-ran" \
KEN_TMPDIR="$scratch/slots/tmp" \
KEN_LOCK_DIR="$scratch/slots/locks" \
  "$root/scripts/ken-cargo" build -p ken-kernel
[[ -e "$scratch/sem-ran" ]]
assert_dir "$scratch/slots/tmp/ken-runtime-old"
printf 'ken-cargo reaper: passed\n'
