#!/usr/bin/env python3
"""Behavioral tests for measured native-parity runner assignment."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("ci-rt-parity-shard.py").resolve()
_SPEC = importlib.util.spec_from_file_location("ci_rt_parity_shard", SCRIPT)
_SHARD = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_SHARD)


class DurationPlanTests(unittest.TestCase):
    def test_duration_plan_is_complete_deterministic_and_balanced(self):
        durations = {f"test_{index}": float(index * 10) for index in range(1, 13)}
        plan = _SHARD.make_plan("fixture::rt_parity_native", list(durations), durations)
        self.assertEqual(len(plan["bins"]), 6)
        assigned = [name for shard in plan["bins"] for _, name in shard["tests"]]
        self.assertCountEqual(assigned, durations)
        self.assertEqual(len(assigned), len(set(assigned)))
        self.assertEqual(
            plan,
            _SHARD.make_plan("fixture::rt_parity_native", list(durations), durations),
        )
        self.assertLessEqual(
            max(shard["seconds"] for shard in plan["bins"])
            - min(shard["seconds"] for shard in plan["bins"]),
            60,
        )

    def test_timing_and_inventory_must_match_exactly(self):
        with self.assertRaisesRegex(ValueError, "timing/inventory mismatch"):
            _SHARD.make_plan("fixture::rt_parity_native", ["a", "b"], {"a": 1.0})
        with self.assertRaisesRegex(ValueError, "timing/inventory mismatch"):
            _SHARD.make_plan("fixture::rt_parity_native", ["a"], {"a": 1.0, "b": 2.0})

    def test_tsv_rejects_duplicate_timing_names(self):
        row = "1 PASS [ 1.000s] (1/1) ken-cli::rt_parity_native sample\n"
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "timings.tsv"
            path.write_text(row + row, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "duplicate test timing"):
                _SHARD.read_timings(path)

    def test_tsv_records_real_run_and_all_185_tests(self):
        path = Path("docs/program/evidence/ci-rt-parity-timings-36260020054.tsv")
        timings = _SHARD.read_timings(path)
        self.assertEqual(len(timings), 185)
        self.assertAlmostEqual(sum(timings.values()), 21608.358)


if __name__ == "__main__":
    unittest.main()
