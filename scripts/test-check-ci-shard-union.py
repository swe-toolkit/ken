#!/usr/bin/env python3
"""Focused fixtures for scripts/check-ci-shard-union.py."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("check-ci-shard-union.py").resolve()


def listing(*identities: tuple[str, str]) -> dict[str, object]:
    suites: dict[str, object] = {}
    for index, (binary_id, testcase) in enumerate(identities):
        suite = suites.setdefault(
            f"suite-{index}", {"binary-id": binary_id, "testcases": {}}
        )
        suite["testcases"][testcase] = {}
    return {"test-count": len(identities), "rust-suites": suites}


class RealizedShardFixtures(unittest.TestCase):
    def fixture(self) -> tempfile.TemporaryDirectory[str]:
        temporary = tempfile.TemporaryDirectory()
        root = Path(temporary.name) / "realized-shards"
        population = [("fixture::bin", f"test_{index}") for index in range(1, 9)]
        for index, identity in enumerate(population, start=1):
            artifact = root / f"realized-shard-{index}"
            artifact.mkdir(parents=True)
            (artifact / "inventory.json").write_text(json.dumps(listing(*population)))
            (artifact / f"selected-{index}.json").write_text(json.dumps(listing(identity)))
        return temporary

    def run_fixture(self, temporary: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(SCRIPT)],
            cwd=temporary,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )

    def test_success(self) -> None:
        with self.fixture() as temporary:
            result = self.run_fixture(temporary)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_missing_and_extra_artifact_fail(self) -> None:
        with self.fixture() as temporary:
            root = Path(temporary) / "realized-shards"
            (root / "realized-shard-8").rename(root / "unexpected")
            result = self.run_fixture(temporary)
        self.assertEqual(result.returncode, 2)
        self.assertIn("expected exactly eight", result.stderr)

    def test_inventory_mismatch_fails(self) -> None:
        with self.fixture() as temporary:
            root = Path(temporary) / "realized-shards" / "realized-shard-2"
            (root / "inventory.json").write_text(json.dumps(listing(("fixture::bin", "other"))))
            result = subprocess.run([sys.executable, str(SCRIPT)], cwd=temporary, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        self.assertEqual(result.returncode, 2)
        self.assertIn("unfiltered inventories differ", result.stderr)

    def test_selected_overlap_fails(self) -> None:
        with self.fixture() as temporary:
            path = Path(temporary) / "realized-shards" / "realized-shard-2" / "selected-2.json"
            path.write_text(json.dumps(listing(("fixture::bin", "test_1"))))
            result = subprocess.run([sys.executable, str(SCRIPT)], cwd=temporary, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        self.assertEqual(result.returncode, 2)
        self.assertIn("selections overlap", result.stderr)

    def test_union_missing_and_extra_fail(self) -> None:
        with self.fixture() as temporary:
            path = Path(temporary) / "realized-shards" / "realized-shard-8" / "selected-8.json"
            path.write_text(json.dumps(listing(("fixture::bin", "extra"))))
            result = subprocess.run([sys.executable, str(SCRIPT)], cwd=temporary, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        self.assertEqual(result.returncode, 2)
        self.assertIn("union differs", result.stderr)

    def test_duplicate_canonical_identity_fails(self) -> None:
        with self.fixture() as temporary:
            path = Path(temporary) / "realized-shards" / "realized-shard-1" / "inventory.json"
            duplicate = {
                "test-count": 2,
                "rust-suites": {
                    "one": {"binary-id": "fixture::bin", "testcases": {"test_1": {}}},
                    "two": {"binary-id": "fixture::bin", "testcases": {"test_1": {}}},
                },
            }
            path.write_text(json.dumps(duplicate))
            result = subprocess.run([sys.executable, str(SCRIPT)], cwd=temporary, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        self.assertEqual(result.returncode, 2)
        self.assertIn("duplicate canonical identity", result.stderr)


if __name__ == "__main__":
    unittest.main()
