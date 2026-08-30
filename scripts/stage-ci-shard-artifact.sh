#!/usr/bin/env bash
set -euo pipefail
shard=$1
unfiltered=$2
inventory=$3
selected=$4
dir="realized-shard-${shard}"
mkdir "$dir"
cp "$unfiltered" "$inventory" "$selected" "$dir/"
