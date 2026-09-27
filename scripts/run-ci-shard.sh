#!/usr/bin/env bash
# Execute only a validated non-empty planned shard selection.
set -euo pipefail
planned=$1
filter=$2
log_path=${3:-}
if [ "$planned" -eq 0 ]; then
  if [ -n "$log_path" ]; then
    mkdir -p "$(dirname "$log_path")"
    : > "$log_path"
  fi
  exit 0
fi
command=(
  cargo nextest run --workspace --locked
  --status-level none --final-status-level pass --color never
  -E "$filter"
)
if [ -z "$log_path" ]; then
  "${command[@]}"
else
  mkdir -p "$(dirname "$log_path")"
  set +e
  "${command[@]}" 2>&1 | tee "$log_path"
  pipe_status=("${PIPESTATUS[@]}")
  result=${pipe_status[0]}
  tee_result=${pipe_status[1]}
  set -e
  if [ "$result" -ne 0 ]; then
    exit "$result"
  fi
  exit "$tee_result"
fi
