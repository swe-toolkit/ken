#!/usr/bin/env python3
import json, glob, sys
files = sorted(glob.glob('realized-shards/**/inventory.json', recursive=True))
selected = sorted(glob.glob('realized-shards/**/selected-*.json', recursive=True))
if len(files) != 8 or len(selected) != 8:
    raise SystemExit(f'expected 8 inventory and selection artifacts, got {len(files)} and {len(selected)}')
def ids(path):
    value=json.load(open(path)); return {(s['package-name'],s['binary-name'],n) for s in value['rust-suites'].values() for n in s['testcases']}
inventories=[ids(p) for p in files]
if any(x != inventories[0] for x in inventories[1:]): raise SystemExit('unfiltered inventories differ')
parts=[ids(p) for p in selected]
union=set()
for part in parts:
    if union & part: raise SystemExit('realized shard selections overlap')
    union |= part
if union != inventories[0]: raise SystemExit('realized shard union differs from inventory')
