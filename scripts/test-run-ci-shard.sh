#!/usr/bin/env bash
set -euo pipefail
root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT
mkdir "$root/bin"
cat > "$root/bin/cargo" <<'EOF'
#!/usr/bin/env bash
echo "$*" >> "$LOG"
EOF
chmod +x "$root/bin/cargo"
LOG="$root/log" PATH="$root/bin:$PATH" scripts/run-ci-shard.sh 0 ignored
[[ ! -e "$root/log" ]]
LOG="$root/log" PATH="$root/bin:$PATH" scripts/run-ci-shard.sh 1 '(binary_id(=x) & test(=y))'
[[ $(wc -l < "$root/log") -eq 1 ]]
grep -Fx 'nextest run --workspace --locked -E (binary_id(=x) & test(=y))' "$root/log"
