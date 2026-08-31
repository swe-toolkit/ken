#!/usr/bin/env python3
"""Fail-closed changed-path classifier for the CI doc-only path."""
import argparse
import json
import sys

# These paths contain prose or agent coordination only; everything else is full CI.
ALLOW_PREFIXES = ("docs/", "agent/", "library/")
DENY_PREFIXES = ("docs/program/evidence/", "crates/", "catalog/", "spec/", "conformance/", ".github/", "scripts/")


def classify(paths):
    if not paths:
        return "full"
    for path in paths:
        if not isinstance(path, str) or not path or path.startswith("/") or ".." in path.split("/") or "." in path.split("/") or "" in path.split("/"):
            return "full"
        if path.startswith(DENY_PREFIXES) or not path.startswith(ALLOW_PREFIXES):
            return "full"
    return "doc-only"


parser = argparse.ArgumentParser()
parser.add_argument("paths_json")
args = parser.parse_args()
try:
    paths = json.load(open(args.paths_json))
    if not isinstance(paths, list):
        raise ValueError("paths must be a list")
    print(classify(paths))
except Exception:
    print("full")
