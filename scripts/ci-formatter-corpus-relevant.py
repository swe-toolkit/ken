#!/usr/bin/env python3
"""Run whole-corpus formatter tests when their implementation inputs change.

Usage: ci-formatter-corpus-relevant.py PATHS_JSON CARGO_METADATA_JSON

Print true unless every changed path is a known non-implementation input.
The metadata is required so an unreadable workspace description cannot silently
skip a gate. Catalog and other corpus edits remain in the sharded tests; this
gate intentionally tracks formatter implementation changes, not corpus churn.
"""
import importlib.util
import json
import os
from pathlib import Path
import sys

spec = importlib.util.spec_from_file_location(
    "ci_z3_relevant", Path(__file__).with_name("ci-z3-relevant.py")
)
_z3 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(_z3)

# The two corpus tests call the formatter, lexer, parser, layout, literate
# extractor and resolver through ken-elaborator. Follow its build/test closure
# dynamically (including newly added path dependencies) rather than guessing
# which of those implementation inputs could change the result.
INPUT_PREFIXES = (".cargo/",)
INPUT_FILES = frozenset({
    "Cargo.toml", "Cargo.lock", "rust-toolchain", "rust-toolchain.toml",
    ".github/workflows/ci.yml", "scripts/ci-formatter-corpus-relevant.py",
    "scripts/ci-z3-relevant.py", "scripts/ci-changed-paths.py",
    "scripts/ci-duration-shard.py", "scripts/check-ci-shard-union.py",
    "scripts/ci-workspace-timings.py", "scripts/ci_workspace_timings.py",
    "scripts/run-ci-shard.sh", "scripts/stage-ci-shard-artifact.sh",
})
# Unknown paths must run the tests. These roots are inputs/data or code outside
# the formatter closure; catalog-only edits are deliberately non-implementation.
NON_INPUT_PREFIXES = (
    "catalog/", "conformance/", "spec/", "examples/", "library/",
    "docs/", "agent/", "research/", "tooling/", "local/",
    ".agents/", ".claude/", ".codex/", ".devcontainer/",
)
NON_INPUT_FILES = frozenset({"AGENTS.md", "CLAUDE.md", "CLEAN-ROOM.md", "README.md", "LICENSE"})


def classify(paths, metadata):
    try:
        if not isinstance(paths, list) or not paths:
            raise ValueError("changed paths must be a non-empty list")
        closure_dirs, member_dirs, manifests = _z3.closure(metadata)
        if not all(isinstance(path, str) and path and not path.startswith("/")
                   and all(part not in ("", ".", "..") for part in path.split("/"))
                   for path in paths):
            raise ValueError("changed path malformed")
        for path in paths:
            if (path in INPUT_FILES or path.startswith(INPUT_PREFIXES)
                    or path in manifests or _z3.under(path, closure_dirs)):
                return True, f"run: {path} is an implementation input"
            if not (path in NON_INPUT_FILES or path.startswith(NON_INPUT_PREFIXES)
                    or _z3.under(path, member_dirs)):
                return True, f"run: {path} is not a known non-input"
        return False, f"skip: {len(paths)} changed paths are known non-inputs"
    except Exception as exc:
        return True, f"run: {exc}"


def main(argv):
    try:
        if len(argv) != 3:
            raise ValueError("usage: PATHS_JSON CARGO_METADATA_JSON")
        with open(argv[1], encoding="utf-8") as file:
            paths = json.load(file)
        with open(argv[2], encoding="utf-8") as file:
            metadata = json.load(file)
        relevant, reason = classify(paths, metadata)
    except (OSError, ValueError) as exc:
        relevant, reason = True, f"run: {exc}"
    print(f"CI_FORMATTER_CORPUS relevant={str(relevant).lower()} {reason}", file=sys.stderr)
    print(str(relevant).lower())
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
