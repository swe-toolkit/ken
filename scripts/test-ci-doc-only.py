#!/usr/bin/env python3
"""Causal model controls for doc-only required-context reporting."""
import json, subprocess, tempfile, os
REQUIRED = ('build + test', 'conformance suite', 'clean-room provenance check', 'path-guard')
def classifier(paths):
    f=tempfile.NamedTemporaryFile(mode='w', delete=False); json.dump(paths,f);f.close()
    try:return subprocess.check_output(['python3','scripts/ci-doc-only.py',f.name],text=True).strip()
    finally: os.unlink(f.name)
def contexts(mode, removed=()):
    return {name: ('pending' if name in removed else 'success') for name in REQUIRED}
def mergeable(report): return all(report[x] == 'success' for x in REQUIRED)
assert classifier(['docs/x.md']) == 'doc-only'
assert classifier(['crates/x.rs']) == 'full'
assert classifier(['.github/workflows/ci.yml']) == 'full'
assert classifier([]) == 'full'
assert classifier(['docs/x.md','../ambiguous']) == 'full'
for context in REQUIRED:
    report=contexts('doc-only', [context])
    assert report[context] == 'pending' and not mergeable(report)
assert mergeable(contexts('doc-only'))
print('doc-only classifier and required-context controls passed')
