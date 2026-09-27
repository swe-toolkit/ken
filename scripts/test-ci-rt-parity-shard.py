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
        self.assertEqual(len(plan["bins"]), _SHARD.SHARD_COUNT)
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

    def test_unseen_test_uses_explicit_default_on_least_loaded_shard(self):
        timings = {
            "longest": 100.0,
            "known_80": 80.0,
            "known_70": 70.0,
            "known_60": 60.0,
            "known_50": 50.0,
        }
        names = [*timings, "unseen"]
        with self.assertWarnsRegex(
            RuntimeWarning, "using 90.0s default for 1 unmeasured parity tests"
        ):
            plan = _SHARD.make_plan("fixture::rt_parity_native", names, timings)
        assignment = {
            name: shard["shard"]
            for shard in plan["bins"]
            for _, name in shard["tests"]
        }
        self.assertEqual(_SHARD.DEFAULT_DURATION_SECONDS, 90.0)
        self.assertEqual(assignment["unseen"], 2)
        self.assertEqual(plan["bins"][1]["seconds"], 90.0)

    def test_stale_timing_rows_warn_and_are_dropped(self):
        with self.assertWarnsRegex(RuntimeWarning, "dropping timing rows absent from current inventory: stale"):
            plan = _SHARD.make_plan(
                "fixture::rt_parity_native", ["current"], {"current": 10.0, "stale": 99.0}
            )
        assigned = [name for shard in plan["bins"] for _, name in shard["tests"]]
        self.assertEqual(assigned, ["current"])

    def test_tsv_rejects_duplicate_timing_names(self):
        row = "1 PASS [ 1.000s] (1/1) ken-cli::rt_parity_native sample\n"
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "timings.tsv"
            path.write_text(row + row, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "duplicate test timing"):
                _SHARD.read_timings([path])

    def test_tsv_records_real_run_and_all_185_tests(self):
        path = Path("docs/program/evidence/ci-rt-parity-timings-36260020054.tsv")
        timings = _SHARD.read_timings([path])
        self.assertEqual(len(timings), 185)
        self.assertAlmostEqual(sum(timings.values()), 21608.358)

    def test_eight_shard_source_excludes_seven_auxiliary_rows(self):
        path = Path("docs/program/evidence/ci-rt-parity-timings-36276921102.tsv")
        timings = _SHARD.read_timings([path])
        self.assertEqual(len(timings), 185)
        self.assertFalse(any("px8f" in name for name in timings))
        self.assertAlmostEqual(
            timings["composed_return_ret_sink_lookup_controls_refuse"], 516.955
        )

    def test_three_run_envelope_handles_timing_noise(self):
        paths = [
            Path("docs/program/evidence/ci-rt-parity-timings-36260020054.tsv"),
            Path("docs/program/evidence/ci-rt-parity-timings-36265192923.tsv"),
            Path("docs/program/evidence/ci-rt-parity-timings-36276921102.tsv"),
        ]
        timings = _SHARD.read_timings(paths)
        self.assertEqual(len(timings), 185)
        self.assertAlmostEqual(
            timings["composed_return_ret_sink_lookup_controls_refuse"], 540.754
        )
        plan = _SHARD.make_plan(
            "ken-cli::rt_parity_native",
            sorted(timings),
            timings,
            ["36260020054", "36265192923", "36276921102"],
        )
        assigned = [name for shard in plan["bins"] for _, name in shard["tests"]]
        self.assertEqual(len(plan["bins"]), 9)
        self.assertCountEqual(assigned, timings)
        self.assertEqual(plan["timing_sources"], ["36260020054", "36265192923", "36276921102"])
        self.assertAlmostEqual(max(shard["seconds"] for shard in plan["bins"]), 2955.313)


if __name__ == "__main__":
    unittest.main()
