#!/usr/bin/env python3
"""Verify realized nextest shard selections partition the filtered inventory."""

from __future__ import annotations

import json
from pathlib import Path
import sys


SHARD_COUNT = 8
ROOT = Path("realized-shards")
EXCLUDED_BINARIES = {
    "rt_parity_native",
    "px8f_buffer_native",
    "px8f_write_partition",
}


class ShardCheckError(RuntimeError):
    pass


def read_listing(path: Path) -> tuple[set[tuple[str, str]], set[tuple[str, str]], set[tuple[str, str]], dict[tuple[str, str], str]]:
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

    discovered: set[tuple[str, str]] = set()
    selected: set[tuple[str, str]] = set()
    excluded: set[tuple[str, str]] = set()
    classifications: dict[tuple[str, str], str] = {}
    for suite in suites.values():
        if not isinstance(suite, dict):
            raise ShardCheckError(f"{path}: rust-suites contains a malformed suite")
        binary_id = suite.get("binary-id")
        binary_name = suite.get("binary-name")
        testcases = suite.get("testcases")
        if not isinstance(binary_id, str) or not binary_id:
            raise ShardCheckError(f"{path}: suite has no non-empty binary-id")
        if not isinstance(binary_name, str) or not binary_name:
            raise ShardCheckError(f"{path}: suite has no non-empty binary-name")
        if not isinstance(testcases, dict) or not testcases:
            raise ShardCheckError(f"{path}: suite has no non-empty testcases")
        for testcase, metadata in testcases.items():
            if not isinstance(testcase, str) or not testcase:
                raise ShardCheckError(f"{path}: suite has an invalid testcase")
            if not isinstance(metadata, dict):
                raise ShardCheckError(f"{path}: testcase metadata is not an object")
            filter_match = metadata.get("filter-match")
            status = filter_match.get("status") if isinstance(filter_match, dict) else None
            if status not in {"matches", "mismatch"}:
                raise ShardCheckError(f"{path}: testcase has invalid filter-match status")
            identity = (binary_id, testcase)
            if identity in discovered:
                raise ShardCheckError(
                    f"{path}: duplicate canonical identity {binary_id!r} {testcase!r}"
                )
            discovered.add(identity)
            classifications[identity] = binary_name
            if status == "matches":
                selected.add(identity)
                if binary_name in EXCLUDED_BINARIES:
                    excluded.add(identity)
    if len(discovered) != count:
        raise ShardCheckError(
            f"{path}: test-count {count} differs from {len(discovered)} discovered testcases"
        )
    return discovered, selected, excluded, classifications


def artifact_paths(root: Path) -> list[tuple[Path, Path, Path]]:
    expected = {f"realized-shard-{index}" for index in range(1, SHARD_COUNT + 1)}
    artifacts = {path.name: path for path in root.iterdir() if path.is_dir()} if root.is_dir() else {}
    if set(artifacts) != expected:
        raise ShardCheckError("expected exactly eight realized-shard artifacts")
    paths = []
    for index in range(1, SHARD_COUNT + 1):
        artifact = artifacts[f"realized-shard-{index}"]
        members = (
            artifact / "unfiltered-inventory.json",
            artifact / "inventory.json",
            artifact / f"selected-{index}.json",
        )
        if not all(member.is_file() for member in members):
            raise ShardCheckError(f"{artifact}: required artifact member is missing")
        paths.append(members)
    return paths


def main() -> int:
    try:
        artifacts = artifact_paths(ROOT)
        unfiltered = [read_listing(paths[0]) for paths in artifacts]
        inventories = [read_listing(paths[1]) for paths in artifacts]
        selections = [read_listing(paths[2]) for paths in artifacts]
        authority_discovered, authority_live, authority_excluded, authority_classes = unfiltered[0]
        if any(
            (rows[0], rows[1], rows[2], rows[3]) != (
                authority_discovered, authority_live, authority_excluded, authority_classes
            )
            for rows in unfiltered[1:]
        ):
            raise ShardCheckError("unfiltered inventories differ")
        population = authority_live - authority_excluded
        for discovered, selected, _, _ in inventories:
            if discovered != authority_discovered:
                raise ShardCheckError("filtered and unfiltered discovered inventories differ")
            if selected != population:
                raise ShardCheckError("filtered inventory differs from unfiltered live complement")
        for discovered, _, _, _ in selections:
            if discovered != authority_discovered:
                raise ShardCheckError("selected listing differs from unfiltered authority")
        union: set[tuple[str, str]] = set()
        for _, selected, _, _ in selections:
            if union & selected:
                raise ShardCheckError("realized shard selections overlap")
            union |= selected
        if union != population:
            raise ShardCheckError("realized shard union differs from filtered inventory")
    except ShardCheckError as error:
        print(f"realized-shard check failed: {error}", file=sys.stderr)
        return 2
    print(f"realized shard partition verified: {len(union)} canonical test identities")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
