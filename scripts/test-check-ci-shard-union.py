#!/usr/bin/env python3
"""Focused schema-faithful fixtures for realized shard partition evidence."""
from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("check-ci-shard-union.py").resolve()


def listing(rows, matches):
    suites = {}
    for index, (binary_id, testcase) in enumerate(rows):
        suites[f"suite-{index}"] = {
            "binary-id": binary_id,
            "binary-name": binary_id.rpartition("::")[2],
            "testcases": {
                testcase: {"filter-match": {"status": "matches" if (binary_id, testcase) in matches else "mismatch"}}
            },
        }
    return {"test-count": len(rows), "rust-suites": suites}


class Fixtures(unittest.TestCase):
    def fixture(self, empty_index=None):
        temporary = tempfile.TemporaryDirectory()
        root = Path(temporary.name) / "realized-shards"
        ordinary = [("fixture::bin", f"test_{index}") for index in range(1, 8 if empty_index else 9)]
        native = [
            (f"fixture::{name}", "native_test")
            for name in ("rt_parity_native", "px8f_buffer_native", "px8f_write_partition")
        ]
        rows = ordinary + native
        assignments = list(ordinary)
        for index in range(1, 9):
            identity = None if index == empty_index else assignments.pop(0)
            artifact = root / f"realized-shard-{index}"
            artifact.mkdir(parents=True)
            selected = set() if identity is None else {identity}
            for name, matches in (("unfiltered-inventory.json", set(rows)), ("inventory.json", set(ordinary)), (f"selected-{index}.json", selected)):
                value = listing(rows, matches)
                value["rust-suites"]["empty"] = {"binary-id": "fixture::empty", "binary-name": "ordinary", "testcases": {}}
                (artifact / name).write_text(json.dumps(value))
        return temporary

    def run_fixture(self, temporary):
        return subprocess.run([sys.executable, str(SCRIPT)], cwd=temporary, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)

    def assert_red(self, mutate, message, empty_index=None):
        with self.fixture(empty_index=empty_index) as temporary:
            mutate(Path(temporary) / "realized-shards")
            result = self.run_fixture(temporary)
        self.assertEqual(result.returncode, 2)
        self.assertIn(message, result.stderr)

    def test_success(self):
        with self.fixture() as temporary:
            result = self.run_fixture(temporary)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_each_empty_shard_position_is_a_valid_partition(self):
        for index in range(1, 9):
            with self.fixture(empty_index=index) as temporary:
                result = self.run_fixture(temporary)
            self.assertEqual(result.returncode, 0, f"empty shard {index}: {result.stderr}")

    def test_empty_position_mutations_red(self):
        empty = 8
        self.assert_red(lambda root: (root / "realized-shard-8" / "unfiltered-inventory.json").unlink(), "member is missing", empty)
        self.assert_red(lambda root: (root / "realized-shard-8" / "inventory.json").unlink(), "member is missing", empty)
        self.assert_red(lambda root: (root / "realized-shard-8" / "selected-8.json").unlink(), "member is missing", empty)
        def selected_truncation(root):
            path = root / "realized-shard-8" / "selected-8.json"
            value = json.loads(path.read_text())
            del value["rust-suites"]["suite-8"]
            value["test-count"] -= 1
            path.write_text(json.dumps(value))
        self.assert_red(selected_truncation, "selected listing differs from unfiltered authority", empty)
        def empty_match(root):
            path = root / "realized-shard-8" / "selected-8.json"
            value = json.loads(path.read_text())
            value["rust-suites"]["suite-0"]["testcases"]["test_1"]["filter-match"]["status"] = "matches"
            path.write_text(json.dumps(value))
        self.assert_red(empty_match, "realized shard selections overlap", empty)
        def sibling_loss(root):
            path = root / "realized-shard-7" / "selected-7.json"
            value = json.loads(path.read_text())
            for suite in value["rust-suites"].values():
                for metadata in suite["testcases"].values():
                    metadata["filter-match"]["status"] = "mismatch"
            path.write_text(json.dumps(value))
        self.assert_red(sibling_loss, "union differs", empty)
        def authority_truncation(root):
            path = root / "realized-shard-8" / "unfiltered-inventory.json"
            value = json.loads(path.read_text())
            del value["rust-suites"]["suite-8"]
            value["test-count"] -= 1
            path.write_text(json.dumps(value))
        self.assert_red(authority_truncation, "unfiltered inventories differ", empty)
        def sibling_overlap(root):
            path = root / "realized-shard-7" / "selected-7.json"
            value = json.loads(path.read_text())
            for suite in value["rust-suites"].values():
                for metadata in suite["testcases"].values():
                    metadata["filter-match"]["status"] = "mismatch"
            value["rust-suites"]["suite-0"]["testcases"]["test_1"]["filter-match"]["status"] = "matches"
            path.write_text(json.dumps(value))
        self.assert_red(sibling_overlap, "realized shard selections overlap", empty)

    def test_missing_or_extra_artifact_and_member_red(self):
        self.assert_red(lambda root: (root / "realized-shard-8").rename(root / "extra"), "exactly eight")
        self.assert_red(lambda root: (root / "realized-shard-1" / "inventory.json").unlink(), "member is missing")

    def test_invalid_json_object_and_schema_rows_red(self):
        self.assert_red(lambda root: (root / "realized-shard-1" / "inventory.json").write_text("not json"), "invalid JSON")
        self.assert_red(lambda root: (root / "realized-shard-1" / "inventory.json").write_text("[]"), "not an object")
        def null_metadata(root):
            path = root / "realized-shard-1" / "inventory.json"
            value = json.loads(path.read_text())
            next(iter(value["rust-suites"].values()))["testcases"]["test_1"] = None
            path.write_text(json.dumps(value))
        self.assert_red(null_metadata, "metadata is not an object")
        def empty_suites(root):
            path = root / "realized-shard-1" / "inventory.json"
            value = json.loads(path.read_text())
            value["rust-suites"] = {}
            path.write_text(json.dumps(value))
        self.assert_red(empty_suites, "rust-suites must be a non-empty")
        def non_map_testcases(root):
            path = root / "realized-shard-1" / "inventory.json"
            value = json.loads(path.read_text())
            value["rust-suites"]["suite-0"]["testcases"] = []
            path.write_text(json.dumps(value))
        self.assert_red(non_map_testcases, "suite has no testcase map")
        def invalid_status(root):
            path = root / "realized-shard-1" / "inventory.json"
            value = json.loads(path.read_text())
            testcase = next(iter(next(iter(value["rust-suites"].values()))["testcases"].values()))
            testcase["filter-match"]["status"] = "unknown"
            path.write_text(json.dumps(value))
        self.assert_red(invalid_status, "invalid filter-match status")

    def test_count_duplicate_and_inventory_mismatch_red(self):
        def wrong_count(root):
            path = root / "realized-shard-1" / "inventory.json"; value = json.loads(path.read_text()); value["test-count"] = 7; path.write_text(json.dumps(value))
        self.assert_red(wrong_count, "differs from")
        def duplicate(root):
            path = root / "realized-shard-1" / "inventory.json"; value = json.loads(path.read_text()); value["rust-suites"]["duplicate"] = {"binary-id": "fixture::bin", "binary-name": "bin", "testcases": {"test_1": {"filter-match": {"status": "matches"}}}}; value["test-count"] = 9; path.write_text(json.dumps(value))
        self.assert_red(duplicate, "duplicate canonical identity")
        def mismatch(root):
            path = root / "realized-shard-2" / "inventory.json"; value = json.loads(path.read_text()); next(iter(value["rust-suites"].values()))["binary-id"] = "other::bin"; path.write_text(json.dumps(value))
        self.assert_red(mismatch, "filtered and unfiltered")

    def test_unfiltered_classification_disagreement_red(self):
        def classification(root):
            path = root / "realized-shard-2" / "unfiltered-inventory.json"
            value = json.loads(path.read_text())
            value["rust-suites"]["suite-8"]["binary-name"] = "ordinary"
            path.write_text(json.dumps(value))
        self.assert_red(classification, "unfiltered inventories differ")

    def test_complement_relations_reach_their_own_errors(self):
        def ordinary_classification(root):
            path = root / "realized-shard-2" / "unfiltered-inventory.json"
            value = json.loads(path.read_text())
            value["rust-suites"]["suite-0"]["binary-name"] = "other_ordinary"
            path.write_text(json.dumps(value))
        self.assert_red(ordinary_classification, "unfiltered inventories differ")
        def ordinary_removed(root):
            path = root / "realized-shard-1" / "inventory.json"
            value = json.loads(path.read_text())
            value["rust-suites"]["suite-0"]["testcases"]["test_1"]["filter-match"]["status"] = "mismatch"
            path.write_text(json.dumps(value))
        self.assert_red(ordinary_removed, "filtered inventory differs from unfiltered live complement")
        def ordinary_over_excluded(root):
            for artifact in root.iterdir():
                path = artifact / "unfiltered-inventory.json"
                value = json.loads(path.read_text())
                value["rust-suites"]["suite-0"]["binary-name"] = "rt_parity_native"
                path.write_text(json.dumps(value))
        self.assert_red(ordinary_over_excluded, "filtered inventory differs from unfiltered live complement")
        for index in (8, 9, 10):
            def native_included(root, index=index):
                path = root / "realized-shard-1" / "inventory.json"
                value = json.loads(path.read_text())
                value["rust-suites"][f"suite-{index}"]["testcases"]["native_test"]["filter-match"]["status"] = "matches"
                path.write_text(json.dumps(value))
            self.assert_red(native_included, "filtered inventory differs from unfiltered live complement")
        def selected_subset(root):
            path = root / "realized-shard-1" / "selected-1.json"
            value = json.loads(path.read_text())
            del value["rust-suites"]["suite-8"]
            value["test-count"] -= 1
            path.write_text(json.dumps(value))
        self.assert_red(selected_subset, "selected listing differs from unfiltered authority")

    def test_overlap_and_union_missing_extra_red(self):
        def overlap(root):
            path = root / "realized-shard-2" / "selected-2.json"; value = json.loads(path.read_text());
            for suite in value["rust-suites"].values():
                if suite["testcases"]:
                    suite["testcases"][next(iter(suite["testcases"]))]["filter-match"]["status"] = "mismatch"
            next(iter(value["rust-suites"].values()))["testcases"]["test_1"]["filter-match"]["status"] = "matches"; path.write_text(json.dumps(value))
        self.assert_red(overlap, "selections overlap")
        def union_extra(root):
            path = root / "realized-shard-8" / "selected-8.json"; value = json.loads(path.read_text()); suite = next(iter(value["rust-suites"].values())); suite["testcases"] = {"extra": {"filter-match": {"status": "matches"}}}; path.write_text(json.dumps(value))
        self.assert_red(union_extra, "selected listing differs from unfiltered authority")
        def union_loss(root):
            path = root / "realized-shard-8" / "selected-8.json"
            value = json.loads(path.read_text())
            for suite in value["rust-suites"].values():
                for metadata in suite["testcases"].values():
                    metadata["filter-match"]["status"] = "mismatch"
            path.write_text(json.dumps(value))
        self.assert_red(union_loss, "union differs")
        def union_extra_native(root):
            path = root / "realized-shard-8" / "selected-8.json"
            value = json.loads(path.read_text())
            value["rust-suites"]["suite-8"]["testcases"]["native_test"]["filter-match"]["status"] = "matches"
            path.write_text(json.dumps(value))
        self.assert_red(union_extra_native, "union differs")


if __name__ == "__main__":
    unittest.main()
