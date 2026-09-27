#!/usr/bin/env bash
set -euo pipefail
root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT
mkdir "$root/bin"
cat > "$root/bin/cargo" <<'EOF'
#!/usr/bin/env bash
echo "$*" >> "$LOG"
printf 'PASS [ 0.125s] (1/1) fixture::ordinary sample\n'
exit "${CARGO_EXIT:-0}"
EOF
chmod +x "$root/bin/cargo"
LOG="$root/log" PATH="$root/bin:$PATH" \
  scripts/run-ci-shard.sh 0 ignored "$root/empty.log"
[[ ! -e "$root/log" ]]
test -f "$root/empty.log"
test ! -s "$root/empty.log"
LOG="$root/log" PATH="$root/bin:$PATH" \
  scripts/run-ci-shard.sh 1 '(binary_id(=x) & test(=y))' "$root/pass.log"
[[ $(wc -l < "$root/log") -eq 1 ]]
grep -Fx 'nextest run --workspace --locked --status-level none --final-status-level pass --color never -E (binary_id(=x) & test(=y))' "$root/log"
grep -Fx 'PASS [ 0.125s] (1/1) fixture::ordinary sample' "$root/pass.log"
set +e
LOG="$root/log" CARGO_EXIT=17 PATH="$root/bin:$PATH" \
  scripts/run-ci-shard.sh 1 '(binary_id(=x) & test(=y))' "$root/fail.log"
status=$?
set -e
test "$status" -eq 17
grep -Fx 'PASS [ 0.125s] (1/1) fixture::ordinary sample' "$root/fail.log"
printf a > "$root/unfiltered.json"
printf b > "$root/inventory.json"
printf c > "$root/selected.json"
(
  cd "$root"
  "$OLDPWD/scripts/stage-ci-shard-artifact.sh" 8 unfiltered.json inventory.json selected.json
)
test -f "$root/realized-shard-8/unfiltered.json"
test -f "$root/realized-shard-8/inventory.json"
test -f "$root/realized-shard-8/selected.json"
