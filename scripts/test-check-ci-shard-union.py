#!/usr/bin/env python3
"""Focused schema-faithful fixtures for realized shard partition evidence."""
from __future__ import annotations

import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("check-ci-shard-union.py").resolve()

# Import the checker module (hyphenated filename) to reuse the SINGLE source of
# truth for the required-arm roster, so this test cannot silently diverge from the
# check it exercises. `__name__` is not "__main__" here, so main() does not run.
_spec = importlib.util.spec_from_file_location("check_ci_shard_union", SCRIPT)
_checker = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_checker)
REQUIRED_RT_PARITY_ARMS = _checker.REQUIRED_RT_PARITY_ARMS
SHARD_COUNT = _checker.SHARD_COUNT
RT_PARITY_SHARD_COUNT = _checker.RT_PARITY_SHARD_COUNT


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
        ordinary = [
            ("fixture::bin", f"test_{index}")
            for index in range(1, SHARD_COUNT if empty_index else SHARD_COUNT + 1)
        ]
        native = [
            (f"fixture::{name}", "native_test")
            for name in ("rt_parity_native", "px8f_buffer_native", "px8f_write_partition")
        ] + [
            # The decomposed rt_parity control arms: discovered + excluded (own job),
            # so the arm-coverage roster check sees them without the shard partition
            # selecting them. Populated from the checker's own roster so success stays
            # in lockstep with the required set.
            ("fixture::rt_parity_native", arm)
            for arm in sorted(REQUIRED_RT_PARITY_ARMS)
        ]
        rows = ordinary + native
        parity_rows = [
            ("fixture::rt_parity_native", f"parity_test_{index}")
            for index in range(1, RT_PARITY_SHARD_COUNT * 2 + 1)
        ]
        parity_root = Path(temporary.name) / "realized-rt-parity"
        for index in range(1, RT_PARITY_SHARD_COUNT + 1):
            artifact = parity_root / f"rt-parity-shard-{index}"
            artifact.mkdir(parents=True)
            per_shard = len(parity_rows) // RT_PARITY_SHARD_COUNT
            selected = set(parity_rows[(index - 1) * per_shard : index * per_shard])
            for filename, matches in (
                ("inventory.json", set(parity_rows)),
                (f"selected-{index}.json", selected),
            ):
                (artifact / filename).write_text(json.dumps(listing(parity_rows, matches)))
        assignments = list(ordinary)
        for index in range(1, SHARD_COUNT + 1):
            identity = None if index == empty_index else assignments.pop(0)
            artifact = root / f"realized-shard-{index}"
            artifact.mkdir(parents=True)
            selected = set() if identity is None else {identity}
            for name, matches in (("unfiltered-inventory.json", set(rows)), ("inventory.json", set(ordinary)), (f"selected-{index}.json", selected)):
                value = listing(rows, matches)
                value["rust-suites"]["empty"] = {"binary-id": "fixture::empty", "binary-name": "ordinary", "testcases": {}}
                (artifact / name).write_text(json.dumps(value))
            terminal_records = [
                {"test_id": f"{binary_id} {testcase}", "seconds": 1.0, "result": "PASS"}
                for binary_id, testcase in sorted(selected)
            ]
            (artifact / "workspace-timings.json").write_text(json.dumps({
                "run_id": 123,
                "shard": index,
                "shard_count": SHARD_COUNT,
                "unit": "seconds",
                "records": terminal_records,
                "fallbacks": [],
            }))
        return temporary

    def run_fixture(self, temporary):
        return subprocess.run(
            [sys.executable, str(SCRIPT)], cwd=temporary, text=True,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False,
            env={**os.environ, "GITHUB_RUN_ID": "123"},
        )

    def refresh_timing(self, root, shard):
        selected_path = root / f"realized-shard-{shard}" / f"selected-{shard}.json"
        listing = json.loads(selected_path.read_text())
        records = sorted(
            f"{suite['binary-id']} {name}"
            for suite in listing["rust-suites"].values()
            for name, metadata in suite["testcases"].items()
            if metadata["filter-match"]["status"] == "matches"
        )
        artifact = {
            "run_id": 123,
            "shard": shard,
            "shard_count": SHARD_COUNT,
            "unit": "seconds",
            "records": [
                {"test_id": test_id, "seconds": 1.0, "result": "PASS"}
                for test_id in records
            ],
            "fallbacks": [],
        }
        (selected_path.parent / "workspace-timings.json").write_text(json.dumps(artifact))

    def assert_red(self, mutate, message, empty_index=None):
        with self.fixture(empty_index=empty_index) as temporary:
            mutate(Path(temporary) / "realized-shards")
            result = self.run_fixture(temporary)
        self.assertEqual(result.returncode, 2)
        self.assertIn(message, result.stderr)

    def test_rt_parity_runner_sets_partition_the_full_suite(self):
        with self.fixture() as temporary:
            result = self.run_fixture(temporary)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(
            f"{RT_PARITY_SHARD_COUNT * 2} rt_parity_native identities across {RT_PARITY_SHARD_COUNT} shards",
            result.stdout,
        )

    def test_rt_parity_empty_shard_reds(self):
        def empty_shard(root):
            shard = RT_PARITY_SHARD_COUNT
            path = root.parent / "realized-rt-parity" / f"rt-parity-shard-{shard}" / f"selected-{shard}.json"
            value = json.loads(path.read_text())
            for suite in value["rust-suites"].values():
                for metadata in suite["testcases"].values():
                    metadata["filter-match"]["status"] = "mismatch"
            path.write_text(json.dumps(value))
        self.assert_red(empty_shard, "rt_parity_native shard selection is empty")

    def test_rt_parity_overlap_and_omission_red(self):
        def overlap(root):
            path = root.parent / "realized-rt-parity" / "rt-parity-shard-2" / "selected-2.json"
            value = json.loads(path.read_text())
            value["rust-suites"]["suite-0"]["testcases"]["parity_test_1"]["filter-match"]["status"] = "matches"
            path.write_text(json.dumps(value))
        self.assert_red(overlap, "rt_parity_native shard selections overlap")

        def omission(root):
            shard = RT_PARITY_SHARD_COUNT
            path = root.parent / "realized-rt-parity" / f"rt-parity-shard-{shard}" / f"selected-{shard}.json"
            value = json.loads(path.read_text())
            omitted = f"parity_test_{(shard - 1) * 2 + 1}"
            for suite in value["rust-suites"].values():
                if omitted in suite["testcases"]:
                    suite["testcases"][omitted]["filter-match"]["status"] = "mismatch"
            path.write_text(json.dumps(value))
        self.assert_red(omission, "rt_parity_native shard union differs from full suite")

    def test_workspace_timing_artifacts_fail_closed(self):
        def mutate_artifact(change):
            def mutate(root):
                path = root / "realized-shard-1" / "workspace-timings.json"
                value = json.loads(path.read_text())
                change(value)
                path.write_text(json.dumps(value))
            return mutate

        self.assert_red(
            mutate_artifact(lambda value: value.update(run_id=124)),
            "run_id does not match this run",
        )
        self.assert_red(
            mutate_artifact(lambda value: value.update(shard=2)),
            "shard does not match its artifact",
        )
        self.assert_red(
            mutate_artifact(lambda value: value.update(shard_count=6)),
            "invalid shard provenance",
        )
        self.assert_red(
            mutate_artifact(lambda value: value.update(unit="milliseconds")),
            'unit must be "seconds"',
        )
        self.assert_red(
            mutate_artifact(lambda value: value["records"].append(dict(value["records"][0]))),
            "duplicate terminal identity",
        )
        self.assert_red(
            mutate_artifact(lambda value: value["records"].clear()),
            "terminal identities differ from selected inventory",
        )
        self.assert_red(
            mutate_artifact(lambda value: value["records"][0].update(result="SLOW")),
            "invalid terminal result",
        )
        self.assert_red(
            mutate_artifact(lambda value: value["records"][0].update(seconds=float("nan"))),
            "invalid seconds",
        )
        self.assert_red(
            mutate_artifact(lambda value: value["records"][0].pop("result")),
            "invalid fields",
        )

        def fallback_disagreement(root):
            path = root / "realized-shard-2" / "workspace-timings.json"
            value = json.loads(path.read_text())
            value["fallbacks"] = [
                {
                    "test_id": "fixture::bin test_1",
                    "seconds": 1.0,
                    "method": "binary-median",
                }
            ]
            path.write_text(json.dumps(value))

        self.assert_red(
            fallback_disagreement,
            "workspace timing artifacts disagree on fallback identities",
        )

        def unknown_fallback(root):
            for shard in range(1, SHARD_COUNT + 1):
                path = root / f"realized-shard-{shard}" / "workspace-timings.json"
                value = json.loads(path.read_text())
                value["fallbacks"] = [
                    {
                        "test_id": "fixture::bin not_live",
                        "seconds": 1.0,
                        "method": "global-median",
                    }
                ]
                path.write_text(json.dumps(value))

        self.assert_red(
            unknown_fallback,
            "workspace fallback identity is absent from the live inventory",
        )

    def test_success(self):
        with self.fixture() as temporary:
            result = self.run_fixture(temporary)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_each_empty_shard_position_is_a_valid_partition(self):
        for index in range(1, SHARD_COUNT + 1):
            with self.fixture(empty_index=index) as temporary:
                result = self.run_fixture(temporary)
            self.assertEqual(result.returncode, 0, f"empty shard {index}: {result.stderr}")

    def test_empty_position_mutations_red(self):
        empty = SHARD_COUNT
        target = SHARD_COUNT - 1
        self.assert_red(lambda root: (root / f"realized-shard-{target}" / "unfiltered-inventory.json").unlink(), "member is missing", empty)
        self.assert_red(lambda root: (root / f"realized-shard-{target}" / "inventory.json").unlink(), "member is missing", empty)
        self.assert_red(lambda root: (root / f"realized-shard-{target}" / f"selected-{target}.json").unlink(), "member is missing", empty)
        def selected_truncation(root):
            path = root / f"realized-shard-{target}" / f"selected-{target}.json"
            value = json.loads(path.read_text())
            del value["rust-suites"][f"suite-{SHARD_COUNT - 1}"]
            value["test-count"] -= 1
            path.write_text(json.dumps(value))
            self.refresh_timing(root, target)
        self.assert_red(selected_truncation, "selected listing differs from unfiltered authority", empty)
        def empty_match(root):
            path = root / f"realized-shard-{target}" / f"selected-{target}.json"
            value = json.loads(path.read_text())
            value["rust-suites"]["suite-0"]["testcases"]["test_1"]["filter-match"]["status"] = "matches"
            path.write_text(json.dumps(value))
            self.refresh_timing(root, target)
        self.assert_red(empty_match, "realized shard selections overlap", empty)
        def sibling_loss(root):
            path = root / f"realized-shard-{target}" / f"selected-{target}.json"
            value = json.loads(path.read_text())
            for suite in value["rust-suites"].values():
                for metadata in suite["testcases"].values():
                    metadata["filter-match"]["status"] = "mismatch"
            path.write_text(json.dumps(value))
            self.refresh_timing(root, target)
        self.assert_red(sibling_loss, "union differs", empty)
        def authority_truncation(root):
            path = root / f"realized-shard-{target}" / "unfiltered-inventory.json"
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
            self.refresh_timing(root, SHARD_COUNT)
        self.assert_red(sibling_overlap, "realized shard selections overlap", empty)

    def test_missing_or_extra_artifact_and_member_red(self):
        self.assert_red(
            lambda root: (root / f"realized-shard-{SHARD_COUNT}").rename(root / "extra"),
            f"expected exactly {SHARD_COUNT}",
        )
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
        for index in (SHARD_COUNT, SHARD_COUNT + 1, SHARD_COUNT + 2):
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
            self.refresh_timing(root, 1)
        self.assert_red(selected_subset, "selected listing differs from unfiltered authority")

    def test_overlap_and_union_missing_extra_red(self):
        def overlap(root):
            path = root / "realized-shard-2" / "selected-2.json"; value = json.loads(path.read_text());
            for suite in value["rust-suites"].values():
                if suite["testcases"]:
                    suite["testcases"][next(iter(suite["testcases"]))]["filter-match"]["status"] = "mismatch"
            next(iter(value["rust-suites"].values()))["testcases"]["test_1"]["filter-match"]["status"] = "matches"; path.write_text(json.dumps(value))
            self.refresh_timing(root, 2)
        self.assert_red(overlap, "selections overlap")
        target = SHARD_COUNT
        def union_extra(root):
            path = root / f"realized-shard-{target}" / f"selected-{target}.json"; value = json.loads(path.read_text()); suite = next(iter(value["rust-suites"].values())); suite["testcases"] = {"extra": {"filter-match": {"status": "matches"}}}; path.write_text(json.dumps(value))
            self.refresh_timing(root, target)
        self.assert_red(union_extra, "selected listing differs from unfiltered authority")
        def union_loss(root):
            path = root / f"realized-shard-{target}" / f"selected-{target}.json"
            value = json.loads(path.read_text())
            for suite in value["rust-suites"].values():
                for metadata in suite["testcases"].values():
                    metadata["filter-match"]["status"] = "mismatch"
            path.write_text(json.dumps(value))
            self.refresh_timing(root, target)
        self.assert_red(union_loss, "union differs")
        def union_extra_native(root):
            path = root / f"realized-shard-{target}" / f"selected-{target}.json"
            value = json.loads(path.read_text())
            value["rust-suites"][f"suite-{SHARD_COUNT}"]["testcases"]["native_test"]["filter-match"]["status"] = "matches"
            path.write_text(json.dumps(value))
            self.refresh_timing(root, target)
        self.assert_red(union_extra_native, "union differs")

    def test_dropped_rt_parity_arm_reds(self):
        # AC-NO-FALSE-GREEN: a decomposed rt_parity control arm silently removed at
        # the source (no per-mutation test emitted) is NOT caught by the union check
        # -- it shrinks the discovered inventory and the union together -- but the
        # independent required-arm roster reds. Drop one arm's suite from every
        # artifact member and confirm the roster check fails with a diagnosed message.
        dropped = sorted(REQUIRED_RT_PARITY_ARMS)[0]

        def drop_arm(root):
            for artifact in sorted(root.iterdir()):
                if not artifact.is_dir():
                    continue
                for member in sorted(artifact.iterdir()):
                    if member.suffix != ".json" or member.name == "workspace-timings.json":
                        continue
                    value = json.loads(member.read_text())
                    suites = value["rust-suites"]
                    doomed = [
                        sid
                        for sid, suite in suites.items()
                        if dropped in suite.get("testcases", {})
                    ]
                    for sid in doomed:
                        del suites[sid]
                        value["test-count"] -= 1
                    member.write_text(json.dumps(value))

        self.assert_red(drop_arm, "control arms missing")

    def test_success_covers_every_required_arm(self):
        # The clean fixture already exercises the roster (its native rows carry every
        # required arm); assert here that the roster is actually non-empty and that the
        # success path reports the arm count, so the coverage check cannot pass
        # vacuously on an empty roster.
        self.assertTrue(REQUIRED_RT_PARITY_ARMS)
        with self.fixture() as temporary:
            result = self.run_fixture(temporary)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(
            f"{len(REQUIRED_RT_PARITY_ARMS)} required rt_parity control arms present",
            result.stdout,
        )


if __name__ == "__main__":
    unittest.main()
