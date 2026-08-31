#!/usr/bin/env python3
import json, os, subprocess, tempfile

def classify(paths):
    f=tempfile.NamedTemporaryFile(mode='w',delete=False); json.dump(paths,f); f.close()
    try: return subprocess.check_output(['python3','scripts/ci-doc-only.py',f.name], text=True).strip()
    finally: os.unlink(f.name)
for paths, expected in [(['docs/a.md'],'doc-only'),(['agent/a.md'],'doc-only'),(['library/a.md'],'doc-only'),(['crates/a.rs'],'full'),(['.github/workflows/ci.yml'],'full'),(['docs/program/evidence/ci-shard-duration-balance-33230600665.json'],'full'),([], 'full'),(['docs//a.md'],'full'),(['docs/./a.md'],'full'),(['../x'],'full')]:
    assert classify(paths)==expected, paths
print('ci doc-only classifier controls passed')
