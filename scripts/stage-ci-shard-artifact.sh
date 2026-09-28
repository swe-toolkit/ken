#!/usr/bin/env bash
set -euo pipefail
if [ "$#" -lt 4 ] || [ "$#" -gt 5 ]; then
  printf 'usage: %s SHARD UNFILTERED INVENTORY SELECTED [PLANNING-EVIDENCE]\n' "$0" >&2
  exit 2
fi
shard=$1
unfiltered=$2
inventory=$3
selected=$4
dir="realized-shard-${shard}"
mkdir "$dir"
cp "$unfiltered" "$inventory" "$selected" "$dir/"
if [ "$#" -eq 5 ]; then
  cp "$5" "$dir/"
fi
