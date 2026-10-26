#!/usr/bin/env python3
"""Fail-safe changed-path gate for the CI `z3-process-adapter` job.

Usage: ci-z3-relevant.py PATHS_JSON CARGO_METADATA_JSON

PATHS_JSON is the changed-path list classify-paths already writes.
CARGO_METADATA_JSON is `cargo metadata --format-version 1 --no-deps` for the
checked-out tree. Prints `false` only when every changed path is known not to
be an input of that job; prints `true` otherwise, including on any error.
The reason is logged to stderr as one `CI_Z3_RELEVANT` line.

The job runs `cargo test -p ken-elaborator --features z3-process` (one
integration test and the whole lib suite), so a path is relevant when it is:

  - under the directory of a crate in ken-elaborator's build closure: its
    normal and build path dependencies, followed transitively, plus
    ken-elaborator's own dev-dependencies and their closure. The closure is
    computed here from the metadata, so a new dependency is picked up;
  - any workspace member's Cargo.toml: `--locked` resolves the whole
    workspace, so an unlocked requirement in any member fails this job too;
  - under a root that closure crates read at build or test time
    (READ_ROOTS);
  - a toolchain or Cargo input, the image source, ci.yml, or a script the
    job or its gate runs (RELEVANT_FILES, RELEVANT_PREFIXES).

A path is irrelevant only when it is under a non-closure workspace member or
under a root in the IRRELEVANT lists. Anything else, and any path, list or
metadata this script cannot read, counts as relevant.
"""
import json
import os
import sys

ROOT_PACKAGE = "ken-elaborator"

# Read at build or test time by sources in the closure crates, outside those
# crates (census of include_str!/include_bytes!, CARGO_MANIFEST_DIR joins and
# relative paths in src/, tests/ and build.rs of every closure crate):
#   catalog/      modules.rs, r_layer_tests/*, seal2_tests/*, many tests/*
#   conformance/  r_layer_tests/kenfmt_b3_layout.rs, tests/cat_*, ken-interp
#   spec/, examples/rosetta/
#                 r_layer_tests/kenfmt_b4_splicing.rs (lib test, whole corpus)
#   library/      tests/lang_mod_strict_resolution_d0.rs
#   crates/ken-cli/tests/fixtures/
#                 tests/rt_bracket_normalized_regions.rs,
#                 tests/rt_bracket_region_transport.rs
#   Cargo.lock, Cargo.toml   ken-host/build.rs, tests/sec4_acceptance.rs
READ_ROOTS = (
    "catalog/",
    "conformance/",
    "spec/",
    "examples/",
    "library/",
    "crates/ken-cli/tests/fixtures/",
)
RELEVANT_PREFIXES = READ_ROOTS + (".cargo/",)
RELEVANT_FILES = frozenset({
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain",
    "rust-toolchain.toml",
    "Dockerfile",
    ".github/workflows/ci.yml",
    "scripts/ci-z3-relevant.py",
    "scripts/ci-doc-only.py",
    "scripts/ci-tested-merge.py",
    "scripts/ci-resource-monitor.py",
})
IRRELEVANT_PREFIXES = (
    "docs/",
    "agent/",
    "research/",
    "tooling/",
    "scripts/",
    ".github/",
    ".config/",
    ".githooks/",
    ".devcontainer/",
    ".agents/",
    ".claude/",
    ".codex/",
    "local/",
)
IRRELEVANT_FILES = frozenset({
    "AGENTS.md",
    "CLAUDE.md",
    "CLEAN-ROOM.md",
    "CONTRIBUTING.md",
    "README.md",
    "LICENSE",
    "moot.toml",
    ".gitignore",
})


class Unclassifiable(Exception):
    pass


def require(cond, reason):
    if not cond:
        raise Unclassifiable(reason)


def relative_dir(path, root):
    rel = os.path.relpath(path, root).replace(os.sep, "/")
    require(rel not in ("", ".") and not rel.startswith("../") and rel != "..",
            f"{path} is not strictly inside the workspace root")
    return rel


def closure(metadata):
    """Return (closure dirs, every member dir, every member Cargo.toml)."""
    require(isinstance(metadata, dict), "metadata is not an object")
    root = metadata.get("workspace_root")
    require(isinstance(root, str) and os.path.isabs(root), "workspace_root is malformed")
    member_ids = metadata.get("workspace_members")
    packages = metadata.get("packages")
    require(isinstance(member_ids, list) and isinstance(packages, list), "metadata lists are malformed")
    by_dir = {}
    for pkg in packages:
        require(isinstance(pkg, dict), "package is not an object")
        if pkg.get("id") not in member_ids:
            continue
        manifest = pkg.get("manifest_path")
        require(isinstance(manifest, str) and manifest.endswith("Cargo.toml"), "manifest_path is malformed")
        by_dir[relative_dir(os.path.dirname(manifest), root)] = pkg
    require(len(by_dir) == len(member_ids), "workspace members do not map to distinct directories")
    roots = [d for d, p in by_dir.items() if p.get("name") == ROOT_PACKAGE]
    require(len(roots) == 1, f"expected one {ROOT_PACKAGE} member, found {len(roots)}")

    seen = set()
    todo = [(roots[0], True)]
    while todo:
        d, is_root = todo.pop()
        if d in seen:
            continue
        seen.add(d)
        if d not in by_dir:  # a path dependency outside the members
            continue
        deps = by_dir[d].get("dependencies")
        require(isinstance(deps, list), f"dependencies of {d} are malformed")
        for dep in deps:
            require(isinstance(dep, dict), "dependency is not an object")
            if dep.get("path") is None:
                continue
            kind = dep.get("kind")
            require(kind in (None, "build", "dev"), f"unknown dependency kind {kind!r}")
            if kind == "dev" and not is_root:
                continue
            todo.append((relative_dir(dep["path"], root), False))
    manifests = frozenset(d + "/Cargo.toml" for d in by_dir)
    return frozenset(seen), frozenset(by_dir), manifests


def valid_path(path):
    parts = path.split("/") if isinstance(path, str) else []
    return bool(parts) and not path.startswith("/") and not any(p in ("", ".", "..") for p in parts)


def under(path, dirs):
    return any(path.startswith(d + "/") for d in dirs)


def classify(paths, metadata):
    """Return (z3_relevant, reason). Raises nothing: every failure is relevant."""
    try:
        require(isinstance(paths, list), "changed paths are not a list")
        require(paths, "changed-path list is empty")
        closure_dirs, member_dirs, manifests = closure(metadata)
        for path in paths:
            require(valid_path(path), f"path {path!r} is malformed")
            if (path in RELEVANT_FILES or path in manifests
                    or path.startswith(RELEVANT_PREFIXES) or under(path, closure_dirs)):
                return True, f"run: {path} is an input of the job"
            if not (under(path, member_dirs) or path in IRRELEVANT_FILES
                    or path.startswith(IRRELEVANT_PREFIXES)):
                return True, f"run: {path} is not a known non-input"
        return False, (f"skip: none of {len(paths)} changed paths is an input; closure "
                       + ",".join(sorted(closure_dirs)))
    except Exception as exc:
        return True, f"run: {type(exc).__name__}: {exc}"


def load(path):
    with open(path) as f:
        return json.load(f)


def main(argv):
    if len(argv) != 3:
        relevant, reason = True, "run: usage: ci-z3-relevant.py PATHS_JSON CARGO_METADATA_JSON"
    else:
        try:
            paths = load(argv[1])
            metadata = load(argv[2])
        except Exception as exc:
            relevant, reason = True, f"run: {type(exc).__name__}: {exc}"
        else:
            relevant, reason = classify(paths, metadata)
    verdict = "true" if relevant else "false"
    line = f"CI_Z3_RELEVANT z3_relevant={verdict} {reason}"
    print(line, file=sys.stderr)
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        try:
            with open(summary, "a") as f:
                f.write(line + "\n")
        except OSError:
            pass
    print(verdict)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
