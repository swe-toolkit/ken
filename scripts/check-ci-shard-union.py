#!/usr/bin/env python3
"""Verify that the realized nextest shard selections partition one inventory."""

from __future__ import annotations

import json
from pathlib import Path
import sys


SHARD_COUNT = 8
ROOT = Path("realized-shards")


class ShardCheckError(RuntimeError):
    """The artifact evidence cannot establish a realized shard partition."""


def canonical_identities(path: Path) -> set[tuple[str, str]]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ShardCheckError(f"{path}: invalid JSON") from error
    if not isinstance(value, dict):
        raise ShardCheckError(f"{path}: listing is not an object")
    count = value.get("test-count")
    suites = value.get("rust-suites")
    if not isinstance(count, int) or isinstance(count, bool) or count <= 0:
        raise ShardCheckError(f"{path}: test-count must be a positive integer")
    if not isinstance(suites, dict) or not suites:
        raise ShardCheckError(f"{path}: rust-suites must be a non-empty map")

    identities: set[tuple[str, str]] = set()
    for suite in suites.values():
        if not isinstance(suite, dict):
            raise ShardCheckError(f"{path}: rust-suites contains a malformed suite")
        binary_id = suite.get("binary-id")
        testcases = suite.get("testcases")
        if not isinstance(binary_id, str) or not binary_id:
            raise ShardCheckError(f"{path}: suite has no non-empty binary-id")
        if not isinstance(testcases, dict) or not testcases:
            raise ShardCheckError(f"{path}: suite has no non-empty testcases")
        for testcase in testcases:
            if not isinstance(testcase, str) or not testcase:
                raise ShardCheckError(f"{path}: suite has an invalid testcase")
            identity = (binary_id, testcase)
            if identity in identities:
                raise ShardCheckError(
                    f"{path}: duplicate canonical identity {binary_id!r} {testcase!r}"
                )
            identities.add(identity)
    if len(identities) > count:
        raise ShardCheckError(
            f"{path}: {len(identities)} testcases exceed test-count {count}"
        )
    return identities


def artifact_paths(root: Path) -> list[tuple[Path, Path]]:
    expected = {f"realized-shard-{index}" for index in range(1, SHARD_COUNT + 1)}
    if not root.is_dir():
        raise ShardCheckError(f"{root}: realized-shards directory is missing")
    artifacts = {path.name: path for path in root.iterdir() if path.is_dir()}
    if set(artifacts) != expected:
        missing = sorted(expected - set(artifacts))
        extra = sorted(set(artifacts) - expected)
        raise ShardCheckError(
            "expected exactly eight realized-shard artifacts"
            + (f"; missing: {', '.join(missing)}" if missing else "")
            + (f"; extra: {', '.join(extra)}" if extra else "")
        )
    paths: list[tuple[Path, Path]] = []
    for index in range(1, SHARD_COUNT + 1):
        artifact = artifacts[f"realized-shard-{index}"]
        inventory = artifact / "inventory.json"
        selected = artifact / f"selected-{index}.json"
        if not inventory.is_file() or not selected.is_file():
            raise ShardCheckError(
                f"{artifact}: expected inventory.json and selected-{index}.json"
            )
        paths.append((inventory, selected))
    return paths


def main() -> int:
    try:
        artifacts = artifact_paths(ROOT)
        inventories = [canonical_identities(inventory) for inventory, _ in artifacts]
        if any(inventory != inventories[0] for inventory in inventories[1:]):
            raise ShardCheckError("unfiltered inventories differ")
        union: set[tuple[str, str]] = set()
        for _, selected_path in artifacts:
            selected = canonical_identities(selected_path)
            if union & selected:
                raise ShardCheckError("realized shard selections overlap")
            union |= selected
        if union != inventories[0]:
            raise ShardCheckError("realized shard union differs from inventory")
    except ShardCheckError as error:
        print(f"realized-shard check failed: {error}", file=sys.stderr)
        return 2
    print(f"realized shard partition verified: {len(union)} canonical test identities")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
