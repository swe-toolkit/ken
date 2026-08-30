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
open("raw.json", "w").write(json.dumps(raw))
open("inventory.json", "w").write(json.dumps(raw))
PY2
python3 "$repo/scripts/ci-duration-shard.py" project-filtered raw.json inventory.json
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
  python3 "$repo/scripts/ci-duration-shard.py" project-selected inventory.json filters/assignments.json "$n" "selected-$n.json"
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
# Both eight-artifact mutations retain seven valid siblings.
mkdir mutation-eight
cp -a realized-shards mutation-eight/
(
  cd mutation-eight
  cp ../unfiltered-inventory.json raw-source.json
  "$repo/scripts/stage-ci-shard-artifact.sh" 1 raw-source.json ../inventory.json ../selected-1.json
  rm -rf realized-shards/realized-shard-1
  mv realized-shard-1 realized-shards/
  set +e
  python3 "$repo/scripts/check-ci-shard-union.py" 2>err
  status=$?
  set -e
  test "$status" -eq 2
  grep -Fx 'realized-shard check failed: realized-shards/realized-shard-1: required artifact member is missing' err
  "$repo/scripts/stage-ci-shard-artifact.sh" 1 ../unfiltered-inventory.json ../inventory.json ../selected-1.json
  rm -rf realized-shards/realized-shard-1
  mv realized-shard-1 realized-shards/
  python3 "$repo/scripts/check-ci-shard-union.py"
)
# Pre-fix construction substitutes the filtered projection for raw authority; real union must refuse it.
mkdir mutation-content
cp -a realized-shards mutation-content/
(
  cd mutation-content
  cp ../inventory.json unfiltered-inventory.json
  "$repo/scripts/stage-ci-shard-artifact.sh" 1 unfiltered-inventory.json ../inventory.json ../selected-1.json
  rm -rf realized-shards/realized-shard-1
  mv realized-shard-1 realized-shards/
  set +e
  python3 "$repo/scripts/check-ci-shard-union.py" 2>err
  status=$?
  set -e
  test "$status" -eq 2
  grep -Fx 'realized-shard check failed: unfiltered inventories differ' err
)
# Pre-fix old-shape filtered listings omitted excluded native discovery rows.
mkdir mutation-old-shape
cp -a realized-shards mutation-old-shape/
(
  cd mutation-old-shape
  python3 - <<'PY'
import json
v=json.load(open('../inventory.json'))
del v['rust-suites']['native']; v['test-count'] -= 1
import os
os.mkdir('old')
open('old/inventory.json','w').write(json.dumps(v))
PY
  for n in $(seq 1 8); do
    "$repo/scripts/stage-ci-shard-artifact.sh" "$n" ../unfiltered-inventory.json old/inventory.json ../selected-$n.json
    rm -rf realized-shards/realized-shard-$n
    mv realized-shard-$n realized-shards/
  done
  set +e; python3 "$repo/scripts/check-ci-shard-union.py" 2>err; status=$?; set -e
  test "$status" -eq 2
  grep -Fx 'realized-shard check failed: filtered and unfiltered discovered inventories differ' err
  mkdir fixed
  for n in $(seq 1 8); do
    python3 "$repo/scripts/ci-duration-shard.py" project-filtered ../unfiltered-inventory.json fixed/inventory.json
    "$repo/scripts/stage-ci-shard-artifact.sh" "$n" ../unfiltered-inventory.json fixed/inventory.json ../selected-$n.json
    rm -rf realized-shards/realized-shard-$n
    mv realized-shard-$n realized-shards/
  done
  python3 "$repo/scripts/check-ci-shard-union.py"
)
exit 0
# Old selected-list shape omitted excluded-native discovery while authority stayed fixed.
mkdir mutation-selected-shape
cp -a realized-shards mutation-selected-shape/
(
  cd mutation-selected-shape
  python3 - <<'PY'
import json, os
v=json.load(open('../inventory.json')); del v['rust-suites']['native']; v['test-count'] -= 1
os.mkdir('old'); open('old/selected.json','w').write(json.dumps(v))
PY
  for n in $(seq 1 8); do
    "$repo/scripts/stage-ci-shard-artifact.sh" "$n" ../unfiltered-inventory.json ../inventory.json old/selected.json
    rm -rf realized-shards/realized-shard-$n; mv realized-shard-$n realized-shards/
  done
  set +e; python3 "$repo/scripts/check-ci-shard-union.py" 2>err; status=$?; set -e
  test "$status" -eq 2
  grep -Fx 'realized-shard check failed: selected listing differs from unfiltered authority' err
  for n in $(seq 1 8); do
    python3 "$repo/scripts/ci-duration-shard.py" project-selected ../inventory.json ../filters/assignments.json "$n" selected.json
    "$repo/scripts/stage-ci-shard-artifact.sh" "$n" ../unfiltered-inventory.json ../inventory.json selected.json
    rm -rf realized-shards/realized-shard-$n; mv realized-shard-$n realized-shards/
  done
  python3 "$repo/scripts/check-ci-shard-union.py"
)
