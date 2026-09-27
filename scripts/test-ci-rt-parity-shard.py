#!/usr/bin/env python3
"""Behavioral tests for measured native-parity runner assignment."""
from __future__ import annotations

import importlib.util
import json
import re
from pathlib import Path
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("ci-rt-parity-shard.py").resolve()
_SPEC = importlib.util.spec_from_file_location("ci_rt_parity_shard", SCRIPT)
_SHARD = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_SHARD)


class DurationPlanTests(unittest.TestCase):
    def test_duration_plan_is_complete_deterministic_and_balanced(self):
        durations = {f"test_{index}": float(index * 10) for index in range(1, 21)}
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

    def test_l2_full_parity_source_reconciles_and_excludes_auxiliary_rows(self):
        path = Path("docs/program/evidence/ci-rt-parity-timings-36279697268.tsv")
        rows = [line.split("\t") for line in path.read_text().splitlines()]
        self.assertEqual(len(rows), 192)
        self.assertTrue(all(row[-1] == "PASS" for row in rows))
        parity = [row for row in rows if row[2] == "ken-cli::rt_parity_native"]
        auxiliary = [row for row in rows if row[2] != "ken-cli::rt_parity_native"]
        self.assertEqual(len(parity), 185)
        self.assertEqual(len({row[3] for row in parity}), 185)
        self.assertEqual(len(auxiliary), 7)
        expected = {
            1: (22, 3233.455), 2: (23, 1734.407),
            3: (23, 3338.692), 4: (23, 2673.518),
            5: (23, 3446.208), 6: (23, 3310.331),
            7: (25, 2519.943), 8: (23, 3370.876),
        }
        for shard, (count, seconds) in expected.items():
            shard_rows = [row for row in parity if int(row[0]) == shard]
            self.assertEqual(len(shard_rows), count)
            self.assertAlmostEqual(sum(float(row[1]) for row in shard_rows), seconds)
        self.assertAlmostEqual(sum(float(row[1]) for row in auxiliary), 1247.352)
        self.assertEqual(len(_SHARD.read_timings([path])), 185)

    def test_nine_shard_source_excludes_auxiliary_rows(self):
        path = Path("docs/program/evidence/ci-rt-parity-timings-36285524404.tsv")
        rows = [line.split("\t") for line in path.read_text().splitlines()]
        self.assertEqual(len(rows), 192)
        parity = [row for row in rows if row[2] == "ken-cli::rt_parity_native"]
        auxiliary = [row for row in rows if row[0] == "px8f-auxiliary-controls"]
        self.assertEqual(len(parity), 185)
        self.assertEqual(len({row[3] for row in parity}), 185)
        self.assertEqual(len(auxiliary), 7)
        self.assertEqual(len({(row[2], row[3]) for row in auxiliary}), 7)
        expected = {
            1: (19, 2966.888), 2: (21, 2831.419),
            3: (22, 2991.597), 4: (21, 2966.448),
            5: (21, 2972.978), 6: (20, 1436.009),
            7: (20, 2999.523), 8: (21, 2998.976),
            9: (20, 2962.657),
        }
        for shard, (count, seconds) in expected.items():
            shard_rows = [row for row in parity if int(row[0]) == shard]
            self.assertEqual(len(shard_rows), count)
            self.assertAlmostEqual(sum(float(row[1]) for row in shard_rows), seconds)
        self.assertAlmostEqual(sum(float(row[1]) for row in auxiliary), 1282.260)
        self.assertEqual(len(_SHARD.read_timings([path])), 185)

        summaries = Path(
            "docs/program/evidence/ci-nextest-summaries-36285524404.tsv"
        ).read_text(encoding="utf-8").splitlines()
        summary_counts = {}
        auxiliary_passes = 0
        for line in summaries:
            job, summary = line.split("\t", 1)
            match = re.fullmatch(r"native-slow \(rt_parity_native\) (\d+)/9", job)
            if match or job == "native-slow (px8f auxiliary controls)":
                count_match = re.search(r"(\d+) tests? run: (\d+) passed", summary)
                self.assertIsNotNone(count_match, summary)
                self.assertEqual(count_match.group(1), count_match.group(2))
                if match:
                    summary_counts[int(match.group(1))] = int(count_match.group(1))
                else:
                    auxiliary_passes += int(count_match.group(1))
        self.assertEqual(summary_counts, {shard: count for shard, (count, _) in expected.items()})
        self.assertEqual(auxiliary_passes, 7)

    def test_ten_shard_source_reconciles_and_excludes_auxiliary_rows(self):
        path = Path("docs/program/evidence/ci-rt-parity-timings-36295180542.tsv")
        rows = [line.split("\t") for line in path.read_text().splitlines()]
        self.assertEqual(len(rows), 192)
        parity = [row for row in rows if row[2] == "ken-cli::rt_parity_native"]
        auxiliary = [row for row in rows if row[0] == "px8f-auxiliary-controls"]
        self.assertEqual(len(parity), 185)
        self.assertEqual(len({row[3] for row in parity}), 185)
        self.assertEqual(len(auxiliary), 7)
        expected = {
            1: (18, 2543.098), 2: (19, 2730.011),
            3: (19, 2072.710), 4: (18, 2722.716),
            5: (18, 2720.972), 6: (19, 2653.108),
            7: (18, 2632.172), 8: (18, 2683.425),
            9: (20, 1331.125), 10: (18, 1426.959),
        }
        for shard, (count, seconds) in expected.items():
            shard_rows = [row for row in parity if int(row[0]) == shard]
            self.assertEqual(len(shard_rows), count)
            self.assertAlmostEqual(sum(float(row[1]) for row in shard_rows), seconds)
        self.assertAlmostEqual(sum(float(row[1]) for row in auxiliary), 982.761)
        self.assertEqual(len(_SHARD.read_timings([path])), 185)

        summaries = Path(
            "docs/program/evidence/ci-nextest-summaries-36295180542.tsv"
        ).read_text(encoding="utf-8").splitlines()
        summary_counts = {}
        auxiliary_passes = 0
        for line in summaries:
            job, summary = line.split("\t", 1)
            match = re.fullmatch(r"native-slow \(rt_parity_native\) (\d+)/10", job)
            if match or job == "native-slow (px8f auxiliary controls)":
                count_match = re.search(r"(\d+) tests? run: (\d+) passed", summary)
                self.assertIsNotNone(count_match, summary)
                self.assertEqual(count_match.group(1), count_match.group(2))
                if match:
                    summary_counts[int(match.group(1))] = int(count_match.group(1))
                else:
                    auxiliary_passes += int(count_match.group(1))
        self.assertEqual(summary_counts, {shard: count for shard, (count, _) in expected.items()})
        self.assertEqual(auxiliary_passes, 7)

    def test_tsv_fail_status_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "timings.tsv"
            path.write_text(
                "".join(
                    f"{shard}\t1.000\tken-cli::rt_parity_native\tprobe_{shard}\t"
                    f"{'FAIL' if shard == 1 else 'PASS'}\n"
                    for shard in range(1, 9)
                ),
                encoding="utf-8",
            )
            with self.assertRaisesRegex(ValueError, "malformed parity timing row"):
                _SHARD.read_timings([path])

    def test_six_run_envelope_handles_timing_noise(self):
        paths = [
            Path("docs/program/evidence/ci-rt-parity-timings-36260020054.tsv"),
            Path("docs/program/evidence/ci-rt-parity-timings-36265192923.tsv"),
            Path("docs/program/evidence/ci-rt-parity-timings-36276921102.tsv"),
            Path("docs/program/evidence/ci-rt-parity-timings-36279697268.tsv"),
            Path("docs/program/evidence/ci-rt-parity-timings-36285524404.tsv"),
            Path("docs/program/evidence/ci-rt-parity-timings-36295180542.tsv"),
        ]
        timings = _SHARD.read_timings(paths)
        self.assertEqual(len(timings), 185)
        self.assertAlmostEqual(
            timings["composed_return_ret_sink_lookup_controls_refuse"], 544.099
        )
        self.assertAlmostEqual(
            timings["checked_ih_inheritance_and_fresh_result_route_are_byte_inert"],
            348.624,
        )
        plan = _SHARD.make_plan(
            "ken-cli::rt_parity_native",
            sorted(timings),
            timings,
            [
                "36260020054", "36265192923", "36276921102",
                "36279697268", "36285524404", "36295180542",
            ],
        )
        assigned = [name for shard in plan["bins"] for _, name in shard["tests"]]
        self.assertEqual(len(plan["bins"]), 10)
        self.assertCountEqual(assigned, timings)
        self.assertEqual(
            plan["timing_sources"],
            [
                "36260020054", "36265192923", "36276921102",
                "36279697268", "36285524404", "36295180542",
            ],
        )
        loads = [shard["seconds"] for shard in plan["bins"]]
        self.assertLess(max(loads) - min(loads), 35.0)


if __name__ == "__main__":
    unittest.main()
