#!/usr/bin/env bash
set -euo pipefail
root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT
repo=$(pwd)
cd "$root"
cat > inventory.json <<'EOF'
{"test-count":1,"rust-suites":{"empty":{"binary-id":"fixture::empty","binary-name":"ordinary","testcases":{}},"live":{"binary-id":"fixture::live","binary-name":"ordinary","testcases":{"t":{"filter-match":{"status":"matches"}}}}}}
EOF
python3 - <<'PY2'
import json
raw=json.load(open("inventory.json"))
raw["test-count"] = 2
raw["rust-suites"]["native"] = {"binary-id":"fixture::native","binary-name":"rt_parity_native","testcases":{"n":{"filter-match":{"status":"matches"}}}}
open("unfiltered-inventory.json", "w").write(json.dumps(raw))
raw["rust-suites"]["native"]["testcases"]["n"]["filter-match"]["status"] = "mismatch"
open("inventory.json", "w").write(json.dumps(raw))
PY2
cat > evidence.json <<'EOF'
{"records":[{"test_id":"fixture::live t","seconds":1}]}
EOF
python3 "$repo/scripts/ci-duration-shard.py" inventory.json evidence.json filters >/dev/null
python3 - <<'PY'
import json
plan=json.load(open('filters/assignments.json'))
assert len(plan['bins']) == 8
empty=next(i+1 for i,b in enumerate(plan['bins']) if not b['tests'])
open('empty-index','w').write(str(empty))
PY
empty=$(<empty-index)
python3 "$repo/scripts/ci-duration-shard.py" project-empty inventory.json "selected-$empty.json"
python3 "$repo/scripts/ci-duration-shard.py" validate-plan filters/assignments.json "$empty" "selected-$empty.json"
for n in $(seq 1 8); do
  planned=$(python3 -c "import json; print(len(json.load(open('filters/assignments.json'))['bins'][$n - 1]['tests']))")
  if [ "$planned" -eq 0 ]; then
    python3 "$repo/scripts/ci-duration-shard.py" project-empty inventory.json "selected-$n.json"
  else
    cp inventory.json "selected-$n.json"
  fi
  python3 "$repo/scripts/ci-duration-shard.py" validate-plan filters/assignments.json "$n" "selected-$n.json"
  "$repo/scripts/stage-ci-shard-artifact.sh" "$n" unfiltered-inventory.json inventory.json "selected-$n.json"
done
mkdir realized-shards
mv realized-shard-* realized-shards/
python3 "$repo/scripts/check-ci-shard-union.py"
assert_expected_command() {
  grep -Fqx "nextest run --workspace --locked -E $1" "$2"
}
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
  LOG="$root/dispatch.log" PATH="$root/bin:$PATH" "$repo/scripts/run-ci-shard.sh" "$dispatched_planned" "$expected_expression"
  if [ "$expected_planned" -eq 0 ]; then
    ! grep -Fqx "nextest run --workspace --locked -E $expected_expression" dispatch.log
  else
    assert_expected_command "$expected_expression" dispatch.log
  fi
done
# Immutable expected plan metadata catches a zeroed dispatch of a nonempty bin.
expected_expression=$(<filters/bin-1.expr)
: > mutation.log
LOG="$root/mutation.log" PATH="$root/bin:$PATH" "$repo/scripts/run-ci-shard.sh" 0 "$expected_expression"
if assert_expected_command "$expected_expression" mutation.log; then
  exit 1
fi
# Mutating the workflow boundary to pass filtered inventory as raw authority
# loses the explicitly named raw member and the real union checker reddens.
mkdir mutation
cp inventory.json mutation/inventory.json
cp selected-1.json mutation/selected-1.json
(
  cd mutation
  "$repo/scripts/stage-ci-shard-artifact.sh" 1 inventory.json inventory.json selected-1.json
  mkdir realized-shards
  mv realized-shard-1 realized-shards/
  if python3 "$repo/scripts/check-ci-shard-union.py"; then exit 1; fi
)
# Both eight-artifact mutations retain seven valid siblings.
mkdir mutation-eight
cp -a realized-shards mutation-eight/
(
  cd mutation-eight
  cp ../inventory.json raw-source.json
  "$repo/scripts/stage-ci-shard-artifact.sh" 1 raw-source.json ../inventory.json ../selected-1.json
  rm -rf realized-shards/realized-shard-1
  mv realized-shard-1 realized-shards/
  if python3 "$repo/scripts/check-ci-shard-union.py"; then exit 1; fi
)
mkdir mutation-content
cp -a realized-shards mutation-content/
(
  cd mutation-content
  cp ../inventory.json unfiltered-inventory.json
  "$repo/scripts/stage-ci-shard-artifact.sh" 1 unfiltered-inventory.json ../inventory.json ../selected-1.json
  rm -rf realized-shards/realized-shard-1
  mv realized-shard-1 realized-shards/
  if python3 "$repo/scripts/check-ci-shard-union.py"; then exit 1; fi
)
