#!/usr/bin/env python3
"""Focused controls for duration shard selection."""
import csv
import importlib.util
import json
import re
import statistics
from datetime import datetime
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


def timing_artifact(records, shard=1, run_id=1):
    return {
        "run_id": run_id,
        "shard": shard,
        "shard_count": SHARD_COUNT,
        "unit": "seconds",
        "records": [
            {**record, "result": record.get("result", "PASS")}
            for record in records
        ],
        "fallbacks": [],
    }


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
            evidence = timing_artifact([
                {"test_id": f"{binary_id} {testcase}", "seconds": 1}
                for binary_id, _, testcase, _ in rows
            ])
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
            (root / "evidence.json").write_text(json.dumps(timing_artifact([{"test_id": "fixture::native t", "seconds": 1}])))
            result = subprocess.run([sys.executable, str(SCRIPT), "inventory.json", "evidence.json"], cwd=root, text=True, stderr=subprocess.PIPE, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stderr.strip(), "filtered live inventory selected zero testcases")

    def test_workspace_timing_sources_use_the_per_test_upper_envelope(self):
        older = _planner.read_durations(
            "docs/program/evidence/ci-workspace-timings-36265192923.tsv"
        )
        latest = _planner.read_durations(
            "docs/program/evidence/ci-workspace-timings-36276921102.tsv"
        )
        l2_completed = _planner.read_durations(
            "docs/program/evidence/ci-workspace-timings-36279697268-completed-shards.tsv"
        )
        previous = _planner.read_durations(
            "docs/program/evidence/ci-workspace-timings-36285524404.tsv"
        )
        current = _planner.read_durations(
            "docs/program/evidence/ci-workspace-timings-36295180542.tsv"
        )
        combined = _planner.read_durations(
            [
                "docs/program/evidence/ci-workspace-timings-36265192923.tsv",
                "docs/program/evidence/ci-workspace-timings-36276921102.tsv",
                "docs/program/evidence/ci-workspace-timings-36279697268-completed-shards.tsv",
                "docs/program/evidence/ci-workspace-timings-36285524404.tsv",
                "docs/program/evidence/ci-workspace-timings-36295180542.tsv",
            ]
        )
        self.assertEqual(len(older), 4148)
        self.assertEqual(len(latest), 4151)
        self.assertEqual(len(l2_completed), 1844)
        self.assertEqual(len(current), 4152)
        self.assertEqual(len(combined), 4154)
        self.assertEqual(len(set(combined) - set(current)), 2)
        old_faster_name = (
            "ken-elaborator::r3_c2_source_mixed_branch "
            "r3_4b_observation_feature_is_native_artifact_identical"
        )
        self.assertEqual(older[old_faster_name], 192.381)
        self.assertEqual(latest[old_faster_name], 316.023)
        self.assertEqual(l2_completed[old_faster_name], 293.142)
        self.assertEqual(combined[old_faster_name], 316.023)
        long_test = (
            "ken-elaborator::lang_mod_strict_resolution_d0 "
            "catalog_ambient_passthrough_migration_census"
        )
        self.assertEqual(previous[long_test], 423.314)
        self.assertEqual(current[long_test], 591.502)
        self.assertEqual(combined[long_test], 591.502)

    def test_workspace_plan_lpt_balances_latest_live_defaults(self):
        source = Path(
            "docs/program/evidence/ci-workspace-timings-36295180542.tsv"
        ).resolve()
        suites = {}
        latest = {}
        by_shard = {}
        for line in source.read_text(encoding="utf-8").splitlines():
            match = _planner.NEXTTEST_TIMING_ROW.fullmatch(line)
            self.assertIsNotNone(match, line)
            binary_id, _, name = match.group("test_id").partition(" ")
            suite = suites.setdefault(binary_id, {
                "binary-id": binary_id,
                "binary-name": binary_id.rpartition("::")[2] or binary_id,
                "testcases": {},
            })
            self.assertNotIn(name, suite["testcases"])
            suite["testcases"][name] = {"filter-match": {"status": "matches"}}
            seconds = float(match.group("seconds"))
            latest[f"{binary_id} {name}"] = seconds
            shard = int(match.group("shard"))
            count, total = by_shard.get(shard, (0, 0.0))
            by_shard[shard] = (count + 1, total + seconds)
        # Model current-main identities absent from run 362951: the re-enabled
        # ds5b test, three H1 tests, four inline interp tests, four later
        # additions, and two current cat1 fixture probes. The H1 host tests
        # were renamed on main.
        active_tests = (
            (
                "ken-elaborator::ds5b_dependent_match_refinement_acceptance",
                "two_vector_zip_recursive_step_convoy_fixture",
            ),
            (
                "ken-elaborator::cat_bsearch_acceptance",
                "ordered_search_publishes_exactly_its_authorized_surface",
            ),
            (
                "ken-elaborator::cc7_argparse_acceptance",
                "render_host_identity_checks_flat_and_qualified_alias_forgery",
            ),
            (
                "ken-elaborator::cc8_env_config_decoder_acceptance",
                "render_host_identity_checks_flat_and_qualified_alias_forgery",
            ),
            (
                "ken-interp",
                "eval::ds5b_cast_regular_tests::equal_inductive_type_app_cast_ignores_neutral_proof",
            ),
            (
                "ken-interp",
                "eval::ds5b_cast_regular_tests::distinct_inductive_type_app_indices_do_not_cast",
            ),
            (
                "ken-interp",
                "eval::ds5b_cast_regular_tests::neutral_inductive_type_app_index_does_not_cast",
            ),
            (
                "ken-interp",
                "eval::ds5b_cast_regular_tests::unknown_proof_blocks_equal_inductive_type_app_cast",
            ),
            (
                "ken-elaborator::cat5_parsing_package",
                "cat5_source_id_host_read_rejects_forged_qualified_provider_alias",
            ),
            (
                "ken-elaborator::cat_deque_acceptance",
                "deque_host_read_rejects_forged_qualified_provider_alias",
            ),
            (
                "ken-elaborator::cc5_pretty_doc_acceptance",
                "render_host_read_ignores_forged_flat_fixture_alias",
            ),
            (
                "ken-elaborator::map_build_acceptance",
                "checked_map_host_read_ignores_forged_flat_aliases",
            ),
            (
                "ken-elaborator::cat1_lawful_functors_package",
                "transport_fixture_aliases_reject_forged_qualified_provider_key",
            ),
            (
                "ken-elaborator::cat1_lawful_functors_package",
                "lawful_fixture_withhold_rejects_forged_qualified_provider_key",
            ),
        )
        for binary_id, name in active_tests:
            active_suite = suites[binary_id]
            self.assertNotIn(name, active_suite["testcases"])
            self.assertNotIn(f"{binary_id} {name}", latest)
            active_suite["testcases"][name] = {
                "filter-match": {"status": "matches"}
            }
        expected_shards = {
            1: (592, 2598.135), 2: (593, 1760.703),
            3: (593, 2037.521), 4: (594, 2437.975),
            5: (594, 2572.145), 6: (593, 1459.319),
            7: (593, 1650.851),
        }
        self.assertEqual(set(by_shard), set(expected_shards))
        for shard, (count, seconds) in expected_shards.items():
            self.assertEqual(by_shard[shard][0], count)
            self.assertAlmostEqual(by_shard[shard][1], seconds)
        summaries = (
            Path("docs/program/evidence/ci-nextest-summaries-36295180542.tsv")
            .read_text(encoding="utf-8").splitlines()
        )
        summary_counts = {}
        for line in summaries:
            job, summary = line.split("\t", 1)
            match = re.fullmatch(r"test shard (\d+)/7", job)
            if match:
                count_match = re.search(r"(\d+) tests run: (\d+) passed", summary)
                self.assertIsNotNone(count_match, summary)
                self.assertEqual(count_match.group(1), count_match.group(2))
                summary_counts[int(match.group(1))] = int(count_match.group(1))
        self.assertEqual(
            summary_counts,
            {shard: count for shard, (count, _) in expected_shards.items()},
        )
        inventory = {
            "test-count": sum(len(suite["testcases"]) for suite in suites.values()),
            "rust-suites": {str(index): suite for index, suite in enumerate(suites.values())},
        }
        identities = {
            (suite["binary-id"], name)
            for suite in suites.values()
            for name in suite["testcases"]
        }
        self.assertEqual(len(identities), len(latest) + len(active_tests))
        measured_by_binary = {}
        for binary_id, name in identities:
            rendered = f"{binary_id} {name}"
            if rendered in latest:
                measured_by_binary.setdefault(binary_id, []).append(latest[rendered])
        global_median = statistics.median(latest.values())
        binary_medians = {
            binary_id: statistics.median(values)
            for binary_id, values in measured_by_binary.items()
        }
        planned_durations = {}
        expected_fallbacks = []
        for binary_id, name in sorted(identities, key=lambda pair: f"{pair[0]} {pair[1]}"):
            rendered = f"{binary_id} {name}"
            if rendered in latest:
                planned_durations[rendered] = latest[rendered]
                continue
            method = "binary-median" if binary_id in binary_medians else "global-median"
            seconds = binary_medians.get(binary_id, global_median)
            planned_durations[rendered] = seconds
            expected_fallbacks.append({
                "test_id": rendered,
                "seconds": seconds,
                "method": method,
            })

        with tempfile.TemporaryDirectory() as temporary:
            inventory_path = Path(temporary) / "inventory.json"
            inventory_path.write_text(json.dumps(inventory), encoding="utf-8")

            def run_plan():
                output_dir = Path(temporary) / "lpt-plan"
                command = [
                    sys.executable,
                    str(SCRIPT),
                    str(inventory_path),
                    str(source),
                    "--output-dir",
                    str(output_dir),
                ]
                result = subprocess.run(
                    command,
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    check=False,
                )
                self.assertEqual(result.returncode, 0, result.stderr)
                assignment = json.loads(result.stdout)
                self.assertEqual(
                    json.loads((output_dir / "assignments.json").read_text()),
                    assignment,
                )
                return assignment, result.stderr

            assignment, balance_stderr = run_plan()
            balanced_plan = assignment["bins"]
            self.assertEqual(assignment["fallbacks"], expected_fallbacks)

        def assigned_bins(plan):
            planned = [
                tuple(identity)
                for shard in plan
                for identity in shard["tests"]
            ]
            self.assertEqual(len(planned), len(identities))
            self.assertEqual(len(set(planned)), len(identities))
            self.assertEqual(set(planned), identities)
            return {
                tuple(identity): shard["bin"]
                for shard in plan
                for identity in shard["tests"]
            }

        balanced_bins = assigned_bins(balanced_plan)
        expected_lpt = [set() for _ in range(SHARD_COUNT)]
        expected_lpt_loads = [0.0] * SHARD_COUNT
        lpt_rows = sorted(
            (
                f"{binary_id} {name}",
                binary_id,
                name,
            )
            for binary_id, name in identities
        )
        lpt_rows.sort(key=lambda row: (-planned_durations[row[0]], row[0]) )
        for rendered, binary_id, name in lpt_rows:
            shard = min(
                range(SHARD_COUNT),
                key=lambda index: (expected_lpt_loads[index], index),
            )
            expected_lpt[shard].add((binary_id, name))
            expected_lpt_loads[shard] += planned_durations[rendered]
        self.assertEqual(
            [
                {tuple(identity) for identity in shard["tests"]}
                for shard in balanced_plan
            ],
            expected_lpt,
        )

        def planned_loads(plan):
            return [
                sum(
                    planned_durations[f"{binary_id} {name}"]
                    for binary_id, name in shard["tests"]
                )
                for shard in plan
            ]

        balanced_loads = planned_loads(balanced_plan)
        binary_fallback_count = sum(
            fallback["method"] == "binary-median" for fallback in expected_fallbacks
        )
        global_fallback_count = len(expected_fallbacks) - binary_fallback_count
        self.assertIn(
            f"using per-binary median for {binary_fallback_count} and global median "
            f"for {global_fallback_count} of {len(active_tests)} unmeasured workspace tests",
            balance_stderr,
        )
        self.assertAlmostEqual(sum(balanced_loads), sum(planned_durations.values()))
        self.assertEqual(len(balanced_plan), SHARD_COUNT)
        for shard, expected_load in zip(balanced_plan, balanced_loads):
            self.assertAlmostEqual(shard["seconds"], expected_load, delta=0.00051)
        envelope_loads = [shard["seconds"] for shard in balanced_plan]
        primary_durations = _planner.read_durations([source])
        active_rendered = {f"{binary_id} {name}" for binary_id, name in active_tests}
        for rendered in active_rendered:
            self.assertNotIn(rendered, primary_durations)
        self.assertAlmostEqual(
            sum(envelope_loads), sum(planned_durations.values()), delta=0.0036
        )
        envelope_average = sum(envelope_loads) / SHARD_COUNT
        self.assertLess(max(envelope_loads) - min(envelope_loads), envelope_average * 0.01)
        latest_average = sum(balanced_loads) / SHARD_COUNT
        self.assertAlmostEqual(latest_average, envelope_average, delta=0.00051)

        with Path("docs/program/evidence/ci-job-timeline-36295180542.tsv").open(
            encoding="utf-8"
        ) as timeline_source:
            timeline = list(csv.DictReader(timeline_source, delimiter="\t"))
        shard_one = next(row for row in timeline if row["job_name"] == "test shard 1/7")
        wall = (
            datetime.fromisoformat(shard_one["completed_at"].replace("Z", "+00:00"))
            - datetime.fromisoformat(shard_one["started_at"].replace("Z", "+00:00"))
        ).total_seconds()
        summary_by_job = dict(
            line.split("\t", 1)
            for line in Path(
                "docs/program/evidence/ci-nextest-summaries-36295180542.tsv"
            ).read_text(encoding="utf-8").splitlines()
        )
        summary = re.search(
            r"Summary \[\s*([0-9.]+)s\]", summary_by_job["test shard 1/7"]
        )
        self.assertIsNotNone(summary)
        non_test_wall = wall - float(summary.group(1))
        calibration = float(summary.group(1)) / by_shard[1][1]
        projected_wall = max(balanced_loads) * calibration + non_test_wall
        average_projection = latest_average * calibration + non_test_wall
        self.assertGreaterEqual(projected_wall, average_projection)
        self.assertLess(projected_wall - average_projection, average_projection * 0.01)
        self.assertTrue(all(active_test in balanced_bins for active_test in active_tests))

    def test_queue_free_run_has_no_measured_test_at_floor_bound(self):
        evidence = Path("docs/program/evidence")
        with (evidence / "ci-first-wave-36295180542.tsv").open(
            encoding="utf-8"
        ) as source:
            first_wave = list(csv.DictReader(source, delimiter="\t"))
        expected_first_wave = {
            *(f"test shard {shard}/7" for shard in range(1, 8)),
            *(f"native-slow (rt_parity_native) {shard}/10" for shard in range(1, 11)),
            "native-slow (px8f auxiliary controls)",
            "optional z3 process adapter",
            "work-item tracker and ignored-row sweep",
        }
        self.assertEqual({row["job_name"] for row in first_wave}, expected_first_wave)
        self.assertEqual(len(first_wave), 20)
        self.assertTrue(all(row["conclusion"] == "success" for row in first_wave))
        self.assertLessEqual(max(float(row["offset_seconds"]) for row in first_wave), 15)

        with (evidence / "ci-job-timeline-36295180542.tsv").open(
            encoding="utf-8"
        ) as source:
            timeline = list(csv.DictReader(source, delimiter="\t"))
        workspace_walls = {}
        for row in timeline:
            match = re.fullmatch(r"test shard (\d+)/7", row["job_name"])
            if not match:
                continue
            started = datetime.fromisoformat(row["started_at"].replace("Z", "+00:00"))
            completed = datetime.fromisoformat(row["completed_at"].replace("Z", "+00:00"))
            workspace_walls[int(match.group(1))] = (completed - started).total_seconds()
        self.assertEqual(workspace_walls[1], 1207)
        runner_up = max(wall for shard, wall in workspace_walls.items() if shard != 1)
        self.assertEqual(runner_up, 1000)

        summaries = (evidence / "ci-nextest-summaries-36295180542.tsv").read_text(
            encoding="utf-8"
        ).splitlines()
        summary_by_job = dict(line.split("\t", 1) for line in summaries)
        shard_one = summary_by_job["test shard 1/7"]
        nextest = re.search(r"Summary \[\s*([0-9.]+)s\]", shard_one)
        self.assertIsNotNone(nextest, shard_one)
        non_test_wall = workspace_walls[1] - float(nextest.group(1))
        floor_bound = runner_up - non_test_wall
        shard_one_times = [
            float(match.group("seconds"))
            for line in (evidence / "ci-workspace-timings-36295180542.tsv")
            .read_text(encoding="utf-8").splitlines()
            if (match := _planner.NEXTTEST_TIMING_ROW.fullmatch(line))
            and int(match.group("shard")) == 1
        ]
        self.assertEqual(max(shard_one_times), 591.502)
        self.assertAlmostEqual(floor_bound, 684.048)
        self.assertLess(max(shard_one_times), floor_bound)

    def test_l2_workspace_input_contains_only_complete_shards(self):
        completed_path = Path(
            "docs/program/evidence/ci-workspace-timings-36279697268-completed-shards.tsv"
        )
        raw_path = Path(
            "docs/program/evidence/ci-workspace-timings-36279697268-all-rows.tsv"
        )
        completed_rows = [line.split("\t") for line in completed_path.read_text().splitlines()]
        raw_rows = [line.split("\t") for line in raw_path.read_text().splitlines()]
        expected = {
            2: (461, 1606.639),
            4: (460, 1135.431),
            5: (461, 1844.008),
            8: (462, 2634.947),
        }
        by_shard = {}
        for shard in expected:
            rows = [row for row in completed_rows if int(row[0]) == shard]
            by_shard[shard] = (len(rows), sum(float(row[1]) for row in rows))
        self.assertEqual(set(by_shard), set(expected))
        for shard, (count, seconds) in expected.items():
            self.assertEqual(by_shard[shard][0], count)
            self.assertAlmostEqual(by_shard[shard][1], seconds)
        self.assertEqual(len(completed_rows), 1844)
        self.assertEqual(
            completed_rows,
            [row for row in raw_rows if int(row[0]) in expected],
        )
        partial_rows = [row for row in raw_rows if int(row[0]) not in expected]
        self.assertEqual(len(partial_rows), 477)
        self.assertEqual(sum(row[4] == "FAIL" for row in partial_rows), 5)

    def test_full_workspace_source_reconciles_with_nextest_summaries(self):
        path = Path("docs/program/evidence/ci-workspace-timings-36285524404.tsv")
        expected = {
            1: (414, 1375.775), 2: (415, 1326.913),
            3: (414, 1101.731), 4: (414, 1486.407),
            5: (415, 1876.155), 6: (416, 1935.796),
            7: (417, 1851.646), 8: (416, 1935.338),
            9: (415, 2119.818), 10: (416, 1650.780),
        }
        by_shard = {}
        identities = []
        for line in path.read_text(encoding="utf-8").splitlines():
            match = _planner.NEXTTEST_TIMING_ROW.fullmatch(line)
            self.assertIsNotNone(match, line)
            shard = int(match.group("shard"))
            identity = match.group("test_id")
            identities.append(identity)
            count, seconds = by_shard.get(shard, (0, 0.0))
            by_shard[shard] = (count + 1, seconds + float(match.group("seconds")))
        self.assertEqual(len(identities), 4152)
        self.assertEqual(len(set(identities)), 4152)
        self.assertEqual(set(by_shard), set(expected))
        for shard, (count, seconds) in expected.items():
            self.assertEqual(by_shard[shard][0], count)
            self.assertAlmostEqual(by_shard[shard][1], seconds)

        summaries = Path(
            "docs/program/evidence/ci-nextest-summaries-36285524404.tsv"
        ).read_text(encoding="utf-8").splitlines()
        summary_counts = {}
        for line in summaries:
            job, summary = line.split("\t", 1)
            match = re.fullmatch(r"test shard (\d+)/10", job)
            if match:
                count_match = re.search(r"(\d+) tests run: (\d+) passed", summary)
                self.assertIsNotNone(count_match, summary)
                self.assertEqual(count_match.group(1), count_match.group(2))
                summary_counts[int(match.group(1))] = int(count_match.group(1))
        self.assertEqual(summary_counts, {shard: count for shard, (count, _) in expected.items()})

    def test_terminal_timing_json_uses_only_pass_for_planner_weights(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            first = timing_artifact(
                [{"test_id": "fixture::binary slow", "seconds": 9.5, "result": "FAIL"}],
                shard=1,
                run_id=73,
            )
            second = timing_artifact(
                [{"test_id": "fixture::binary fast", "seconds": 0.0, "result": "PASS"}],
                shard=2,
                run_id=73,
            )
            first_path = root / "shard-1.json"
            second_path = root / "shard-2.json"
            first_path.write_text(json.dumps(first), encoding="utf-8")
            second_path.write_text(json.dumps(second), encoding="utf-8")
            durations = _planner.read_durations([first_path, second_path])
            self.assertEqual(durations, {"fixture::binary fast": 0.0})

            duplicate = timing_artifact(
                [{"test_id": "fixture::binary slow", "seconds": 2.0}],
                shard=2,
                run_id=73,
            )
            second_path.write_text(json.dumps(duplicate), encoding="utf-8")
            with self.assertRaisesRegex(
                SystemExit, "duplicate terminal identity in run 73 shards 1 and 2"
            ):
                _planner.read_durations([first_path, second_path])

            first["unit"] = "milliseconds"
            first_path.write_text(json.dumps(first), encoding="utf-8")
            with self.assertRaisesRegex(SystemExit, "unit must be"):
                _planner.read_durations([first_path])

    def test_failed_json_timing_uses_measured_median_fallback(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "timings.json"
            artifact = timing_artifact(
                [
                    {"test_id": "fixture::binary candidate", "seconds": 0.125},
                    {"test_id": "fixture::binary peer", "seconds": 0.5},
                ]
            )
            artifact["records"][0]["result"] = "PASS"
            path.write_text(json.dumps(artifact), encoding="utf-8")
            passing_durations = _planner.read_durations([path])
            artifact["records"][0]["result"] = "FAIL"
            path.write_text(json.dumps(artifact), encoding="utf-8")
            failed_durations = _planner.read_durations([path])

        live = [
            ("fixture::binary candidate", "fixture::binary", "candidate"),
            ("fixture::binary peer", "fixture::binary", "peer"),
        ]
        passing, passing_fallbacks = _planner.resolve_durations(live, passing_durations)
        failed, failed_fallbacks = _planner.resolve_durations(live, failed_durations)
        self.assertEqual(passing["fixture::binary candidate"], 0.125)
        self.assertNotIn("fixture::binary candidate", {row["test_id"] for row in passing_fallbacks})
        self.assertEqual(failed["fixture::binary candidate"], 0.5)
        self.assertEqual(
            failed_fallbacks,
            [{
                "test_id": "fixture::binary candidate",
                "seconds": 0.5,
                "method": "binary-median",
            }],
        )

    def test_workspace_tsv_fail_status_is_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "timings.tsv"
            path.write_text(
                "1\t1.000\tfixture::ordinary\tsample\tFAIL\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(SystemExit, "malformed workspace timing row"):
                _planner.read_durations([path])

    def test_unseen_tests_use_per_binary_and_global_medians(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            suites = {
                name: {
                    "binary-id": "fixture::ordinary",
                    "binary-name": "ordinary",
                    "testcases": {name: {"filter-match": {"status": "matches"}}},
                }
                for name in ("known", "middle", "slow", "unseen")
            }
            suites["other"] = {
                "binary-id": "fixture::other",
                "binary-name": "other",
                "testcases": {"measured": {"filter-match": {"status": "matches"}}},
            }
            suites["global"] = {
                "binary-id": "fixture::unmeasured",
                "binary-name": "unmeasured",
                "testcases": {"global": {"filter-match": {"status": "matches"}}},
            }
            (root / "inventory.json").write_text(
                json.dumps({"test-count": 6, "rust-suites": suites})
            )
            (root / "timings.tsv").write_text(
                "1 PASS [ 10.000s] (1/4) fixture::ordinary known\n"
                "1 PASS [ 20.000s] (2/4) fixture::ordinary middle\n"
                "1 PASS [100.000s] (3/4) fixture::ordinary slow\n"
                "2 PASS [ 40.000s] (1/1) fixture::other measured\n"
                "2 PASS [ 20.000s] (1/1) fixture::other retired\n"
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
        self.assertIn(
            "using per-binary median for 1 and global median for 1 of 2 unmeasured",
            result.stderr,
        )
        self.assertIn("dropping timing rows absent from current inventory", result.stderr)
        plan = json.loads(result.stdout)
        self.assertEqual(sum(shard["seconds"] for shard in plan["bins"]), 220.0)
        self.assertEqual(
            plan["fallbacks"],
            [
                {
                    "test_id": "fixture::ordinary unseen",
                    "seconds": 20.0,
                    "method": "binary-median",
                },
                {
                    "test_id": "fixture::unmeasured global",
                    "seconds": 30.0,
                    "method": "global-median",
                },
            ],
        )
        assigned = {
            tuple(identity): shard["bin"]
            for shard in plan["bins"]
            for identity in shard["tests"]
        }
        self.assertEqual(
            assigned,
            {
                ("fixture::ordinary", "slow"): 1,
                ("fixture::other", "measured"): 2,
                ("fixture::unmeasured", "global"): 3,
                ("fixture::ordinary", "middle"): 4,
                ("fixture::ordinary", "unseen"): 5,
                ("fixture::ordinary", "known"): 6,
            },
        )
        with self.assertRaisesRegex(
            SystemExit, "cannot estimate unmeasured tests without any live timing rows"
        ):
            _planner.resolve_durations(
                [("fixture::never-measured test", "fixture::never-measured", "test")],
                {},
            )

    def test_non_map_testcases_has_exact_error(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            value = {"test-count": 1, "rust-suites": {"bad": {"binary-id": "fixture::bad", "binary-name": "ordinary", "testcases": []}}}
            (root / "inventory.json").write_text(json.dumps(value))
            (root / "evidence.json").write_text(json.dumps(timing_artifact([{"test_id": "fixture::ordinary sample", "seconds": 1}])))
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
                (root / "evidence.json").write_text(json.dumps(timing_artifact([{"test_id": f"fixture::ordinary test_{i}", "seconds": 1} for i in range(size)])))
                result = subprocess.run([sys.executable, str(SCRIPT), "inventory.json", "evidence.json", "--output-dir", "out"], cwd=root, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
                assignment = json.loads((root / "out" / "assignments.json").read_text())
            self.assertEqual(result.returncode, 0, result.stderr)
            bins = assignment["bins"]
            self.assertEqual(len(bins), SHARD_COUNT)
            self.assertEqual(sum(len(item["tests"]) for item in bins), size)
            self.assertCountEqual(
                [tuple(identity) for item in bins for identity in item["tests"]],
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
