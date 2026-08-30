#!/usr/bin/env bash
# Execute only a validated non-empty planned shard selection.
set -euo pipefail
planned=$1
filter=$2
if [ "$planned" -eq 0 ]; then
  exit 0
fi
cargo nextest run --workspace --locked -E "$filter"
