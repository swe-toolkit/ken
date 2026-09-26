#!/usr/bin/env python3
"""Focused controls for duration shard selection."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("ci-duration-shard.py").resolve()
_spec = importlib.util.spec_from_file_location("ci_duration_shard", SCRIPT)
_planner = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_planner)
SHARD_COUNT = _planner.N


class DurationShardControls(unittest.TestCase):
    def test_live_native_binary_cannot_enter_a_shard(self):
        rows = [("fixture::ordinary", "ordinary", "ordinary_test", "matches")]
        rows.extend(
            (f"fixture::{name}", name, "native_test", "matches")
            for name in (
                "rt_parity_native",
                "px8f_buffer_native",
                "px8f_write_partition",
            )
        )
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            suites = {
                str(index): {
                    "binary-id": binary_id,
                    "binary-name": binary_name,
                    "testcases": {testcase: {"filter-match": {"status": status}}},
                }
                for index, (binary_id, binary_name, testcase, status) in enumerate(rows)
            }
            inventory = {"test-count": len(rows), "rust-suites": suites}
            evidence = {
                "records": [
                    {"test_id": f"{binary_id} {testcase}", "seconds": 1}
                    for binary_id, _, testcase, _ in rows
                ]
            }
            (root / "inventory.json").write_text(json.dumps(inventory))
            (root / "evidence.json").write_text(json.dumps(evidence))
            result = subprocess.run(
                [sys.executable, str(SCRIPT), "inventory.json", "evidence.json"],
                cwd=root, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                check=False,
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        filters = [row["filter"] for row in json.loads(result.stdout)["bins"]]
        self.assertTrue(any("fixture::ordinary" in item for item in filters))
        for binary in ("rt_parity_native", "px8f_buffer_native", "px8f_write_partition"):
            self.assertFalse(any(binary in item for item in filters))

    def test_empty_eligible_population_is_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inventory = {"test-count": 1, "rust-suites": {"empty": {"binary-id": "fixture::empty", "binary-name": "ordinary", "testcases": {}}, "native": {"binary-id": "fixture::native", "binary-name": "rt_parity_native", "testcases": {"t": {"filter-match": {"status": "matches"}}}}}}
            (root / "inventory.json").write_text(json.dumps(inventory))
            (root / "evidence.json").write_text(json.dumps({"records": [{"test_id": "fixture::native t", "seconds": 1}]}))
            result = subprocess.run([sys.executable, str(SCRIPT), "inventory.json", "evidence.json"], cwd=root, text=True, stderr=subprocess.PIPE, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stderr.strip(), "filtered live inventory selected zero testcases")

    def test_current_workspace_timing_artifact_has_full_run_population(self):
        durations = _planner.read_durations(
            "docs/program/evidence/ci-workspace-timings-36265192923.tsv"
        )
        self.assertEqual(len(durations), 4148)
        self.assertAlmostEqual(sum(durations.values()), 16069.293)
        self.assertEqual(
            durations[
                "ken-elaborator::lang_mod_strict_resolution_d0 "
                "catalog_ambient_passthrough_migration_census"
            ],
            549.96,
        )

    def test_unseen_tests_use_default_and_stale_rows_warn_and_drop(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            suites = {
                name: {
                    "binary-id": "fixture::ordinary",
                    "binary-name": "ordinary",
                    "testcases": {name: {"filter-match": {"status": "matches"}}},
                }
                for name in ("known", "unseen")
            }
            (root / "inventory.json").write_text(
                json.dumps({"test-count": 2, "rust-suites": suites})
            )
            (root / "timings.tsv").write_text(
                "1 PASS [ 10.000s] (1/1) fixture::ordinary known\n"
                "1 PASS [ 20.000s] (1/1) fixture::ordinary retired\n"
            )
            result = subprocess.run(
                [sys.executable, str(SCRIPT), "inventory.json", "timings.tsv"],
                cwd=root,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("600.0s default for 1 unmeasured workspace tests", result.stderr)
        self.assertIn("dropping timing rows absent from current inventory", result.stderr)
        plan = json.loads(result.stdout)
        self.assertEqual(
            sum(shard["seconds"] for shard in plan["bins"]),
            610.0,
        )
        assigned = [name for shard in plan["bins"] for _, name in shard["tests"]]
        self.assertCountEqual(assigned, ["known", "unseen"])

    def test_non_map_testcases_has_exact_error(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            value = {"test-count": 1, "rust-suites": {"bad": {"binary-id": "fixture::bad", "binary-name": "ordinary", "testcases": []}}}
            (root / "inventory.json").write_text(json.dumps(value))
            (root / "evidence.json").write_text(json.dumps({"records": [{"test_id": "x", "seconds": 1}]}))
            result = subprocess.run([sys.executable, str(SCRIPT), "inventory.json", "evidence.json"], cwd=root, text=True, stderr=subprocess.PIPE, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stderr.strip(), "nextest rust suite has no testcase map")

    def test_small_eligible_populations_keep_all_planned_bins(self):
        for size in range(1, SHARD_COUNT + 2):
            with tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                rows = [("fixture::ordinary", "ordinary", f"test_{i}", "matches") for i in range(size)]
                suites = {str(i): {"binary-id": binary_id, "binary-name": binary_name, "testcases": {testcase: {"filter-match": {"status": status}}}} for i, (binary_id, binary_name, testcase, status) in enumerate(rows)}
                suites["empty"] = {"binary-id": "fixture::empty", "binary-name": "ordinary", "testcases": {}}
                (root / "inventory.json").write_text(json.dumps({"test-count": size, "rust-suites": suites}))
                (root / "evidence.json").write_text(json.dumps({"records": [{"test_id": f"fixture::ordinary test_{i}", "seconds": 1} for i in range(size)]}))
                result = subprocess.run([sys.executable, str(SCRIPT), "inventory.json", "evidence.json", "out"], cwd=root, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
                assignment = json.loads((root / "out" / "assignments.json").read_text())
            self.assertEqual(result.returncode, 0, result.stderr)
            bins = assignment["bins"]
            self.assertEqual(len(bins), SHARD_COUNT)
            self.assertEqual(sum(len(item["tests"]) for item in bins), size)
            self.assertEqual(
                sorted(tuple(identity) for item in bins for identity in item["tests"]),
                [("fixture::ordinary", f"test_{i}") for i in range(size)],
            )
            self.assertEqual(sum(not item["tests"] for item in bins), max(0, SHARD_COUNT - size))

    def test_validate_plan_accepts_exact_and_rejects_dispositions(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            base = {"test-count": 1, "rust-suites": {"s": {"binary-id": "fixture::bin", "binary-name": "ordinary", "testcases": {"t": {"filter-match": {"status": "matches"}}}}}}
            (root / "selected.json").write_text(json.dumps(base))
            (root / "assignment.json").write_text(
                json.dumps({"bins": [{"tests": [["fixture::bin", "t"]]}], "x": 1})
            )
            command = [sys.executable, str(SCRIPT), "validate-plan", "assignment.json", "1", "selected.json"]
            self.assertEqual(subprocess.run(command, cwd=root, check=False).returncode, 0)
            base["rust-suites"]["s"]["testcases"]["t"]["filter-match"]["status"] = "mismatch"
            (root / "selected.json").write_text(json.dumps(base))
            self.assertNotEqual(subprocess.run(command, cwd=root, check=False).returncode, 0)
            (root / "assignment.json").write_text(json.dumps({"bins": [{"tests": []}]}))
            base["extra"] = {"preserve": [1, {"nested": True}]}
            base["rust-suites"]["s"]["suite-metadata"] = {"keep": "me"}
            base["rust-suites"]["s"]["testcases"]["t"]["metadata"] = {"keep": "me"}
            base["rust-suites"]["s"]["testcases"]["t"]["filter-match"]["status"] = "matches"
            base["rust-suites"]["second"] = {"binary-id": "fixture::second", "binary-name": "ordinary", "testcases": {"u": {"filter-match": {"status": "mismatch"}, "metadata": {"also": "kept"}}}}
            base["rust-suites"]["empty"] = {"binary-id": "fixture::empty", "binary-name": "ordinary", "testcases": {}}
            base["test-count"] = 2
            expected = json.loads(json.dumps(base))
            for suite in expected["rust-suites"].values():
                for metadata in suite["testcases"].values():
                    metadata["filter-match"]["status"] = "mismatch"
            (root / "source.json").write_text(json.dumps(base))
            self.assertEqual(subprocess.run([sys.executable, str(SCRIPT), "project-empty", "source.json", "selected.json"], cwd=root, check=False).returncode, 0)
            projected = json.loads((root / "selected.json").read_text())
            self.assertEqual(projected, expected)
            self.assertEqual(subprocess.run(command, cwd=root, check=False).returncode, 0)
            base["rust-suites"]["s"]["testcases"]["t"]["filter-match"]["status"] = "matches"
            (root / "selected.json").write_text(json.dumps(base))
            self.assertNotEqual(subprocess.run(command, cwd=root, check=False).returncode, 0)
            (root / "assignment.json").write_text(json.dumps({"bins": [{"tests": [["fixture::bin", "wrong"]]}]}))
            self.assertNotEqual(subprocess.run(command, cwd=root, check=False).returncode, 0)


if __name__ == "__main__":
    unittest.main()
