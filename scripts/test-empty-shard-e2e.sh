#!/usr/bin/env bash
set -euo pipefail
root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT
cd "$root"
cat > inventory.json <<'EOF'
{"test-count":1,"rust-suites":{"empty":{"binary-id":"fixture::empty","binary-name":"ordinary","testcases":{}},"live":{"binary-id":"fixture::live","binary-name":"ordinary","testcases":{"t":{"filter-match":{"status":"matches"}}}}}}
EOF
cp inventory.json unfiltered-inventory.json
cat > evidence.json <<'EOF'
{"records":[{"test_id":"fixture::live t","seconds":1}]}
EOF
python3 "$OLDPWD/scripts/ci-duration-shard.py" inventory.json evidence.json filters >/dev/null
python3 - <<'PY'
import json
plan=json.load(open('filters/assignments.json'))
assert len(plan['bins']) == 8
empty=next(i+1 for i,b in enumerate(plan['bins']) if not b['tests'])
open('empty-index','w').write(str(empty))
PY
empty=$(<empty-index)
python3 "$OLDPWD/scripts/ci-duration-shard.py" project-empty inventory.json "selected-$empty.json"
python3 "$OLDPWD/scripts/ci-duration-shard.py" validate-plan filters/assignments.json "$empty" "selected-$empty.json"
for n in $(seq 1 8); do
  planned=$(python3 -c "import json; print(len(json.load(open('filters/assignments.json'))['bins'][$n - 1]['tests']))")
  if [ "$planned" -eq 0 ]; then
    python3 "$OLDPWD/scripts/ci-duration-shard.py" project-empty inventory.json "selected-$n.json"
  else
    cp inventory.json "selected-$n.json"
  fi
  python3 "$OLDPWD/scripts/ci-duration-shard.py" validate-plan filters/assignments.json "$n" "selected-$n.json"
  "$OLDPWD/scripts/stage-ci-shard-artifact.sh" "$n" unfiltered-inventory.json inventory.json "selected-$n.json"
done
mkdir realized-shards
mv realized-shard-* realized-shards/
python3 "$OLDPWD/scripts/check-ci-shard-union.py"
mkdir bin
cat > bin/cargo <<'EOF'
#!/usr/bin/env bash
echo "$*" >> "$LOG"
EOF
chmod +x bin/cargo
: > dispatch.log
for n in $(seq 1 8); do
  expected_planned=$(python3 -c "import json; print(len(json.load(open('filters/assignments.json'))['bins'][$n - 1]['tests']))")
  expected_expression=$(<"filters/bin-$n.expr")
  dispatched_planned=$expected_planned
  LOG="$root/dispatch.log" PATH="$root/bin:$PATH" "$OLDPWD/scripts/run-ci-shard.sh" "$dispatched_planned" "$expected_expression"
  if [ "$expected_planned" -eq 0 ]; then
    ! grep -Fqx "nextest run --workspace --locked -E $expected_expression" dispatch.log
  else
    grep -Fqx "nextest run --workspace --locked -E $expected_expression" dispatch.log
  fi
done
# Immutable expected plan metadata catches a zeroed dispatch of a nonempty bin.
expected_expression=$(<filters/bin-1.expr)
: > mutation.log
LOG="$root/mutation.log" PATH="$root/bin:$PATH" "$OLDPWD/scripts/run-ci-shard.sh" 0 "$expected_expression"
if grep -Fqx "nextest run --workspace --locked -E $expected_expression" mutation.log; then
  exit 1
fi
