#!/usr/bin/env python3
"""Focused controls for workspace nextest terminal timing artifacts."""
from __future__ import annotations

import copy
import unittest

import ci_workspace_timings as timings


class WorkspaceTimingControls(unittest.TestCase):
    def listing(self, identities, selected=None):
        selected = set(identities if selected is None else selected)
        suites = {}
        for index, (binary_id, name) in enumerate(identities):
            suites[str(index)] = {
                "binary-id": binary_id,
                "binary-name": binary_id.rpartition("::")[2] or binary_id,
                "testcases": {
                    name: {
                        "filter-match": {
                            "status": "matches" if (binary_id, name) in selected else "mismatch"
                        }
                    }
                },
            }
        return {"test-count": len(identities), "rust-suites": suites}

    def test_terminal_outcomes_ignore_slow_events_and_collapse_retries(self):
        identities = {"fixture::binary first", "fixture::binary second"}
        log = "\n".join(
            (
                "SLOW [> 60.000s] (───) fixture::binary first",
                "RETRY [ 0.500s] (───) fixture::binary first",
                "PASS [ 0.250s] (2/2) fixture::binary first",
                "FAIL [ 2.000s] (1/2) fixture::binary second",
            )
        )
        records = timings.parse_terminal_log(log, identities)
        self.assertEqual(
            records,
            [
                {"test_id": "fixture::binary first", "seconds": 0.25, "result": "PASS"},
                {"test_id": "fixture::binary second", "seconds": 2.0, "result": "FAIL"},
            ],
        )
        plan = {
            "bins": [{"tests": []} for _ in range(7)],
            "fallbacks": [
                {
                    "test_id": "fixture::binary first",
                    "seconds": 0.5,
                    "method": "binary-median",
                },
                {
                    "test_id": "fixture::binary second",
                    "seconds": 1.5,
                    "method": "global-median",
                },
            ],
        }
        plan["bins"][0]["tests"] = [["fixture::binary", "second"]]
        plan["bins"][2]["tests"] = [["fixture::binary", "first"]]
        shard_log = "SLOW [> 60.000s] (───) fixture::binary first\nRETRY [ 0.500s] (───) fixture::binary first\nPASS [ 0.250s] (1/1) fixture::binary first"
        artifact = timings.emit_artifact(
            shard_log,
            self.listing([("fixture::binary", "first")]),
            987,
            3,
            plan,
        )
        self.assertEqual(artifact["run_id"], 987)
        self.assertEqual(artifact["shard"], 3)
        self.assertEqual(artifact["shard_count"], 7)
        self.assertEqual(artifact["unit"], "seconds")
        self.assertEqual(len(artifact["records"]), 1)
        self.assertEqual(artifact["fallbacks"], plan["fallbacks"])

    def test_duplicate_terminal_identity_fails_closed(self):
        selected = self.listing([("fixture::binary", "test")])
        duplicate = "PASS [ 1.000s] (1/1) fixture::binary test\nPASS [ 1.000s] (1/1) fixture::binary test"
        with self.assertRaisesRegex(
            timings.TimingArtifactError, "duplicate terminal identity in nextest output"
        ):
            timings.emit_artifact(duplicate, selected, 1, 1)

    def test_slow_only_and_missing_or_extra_terminal_identities_fail(self):
        selected = self.listing([("fixture::binary", "test")])
        slow_only = "SLOW [> 60.000s] (───) fixture::binary test"
        with self.assertRaisesRegex(timings.TimingArtifactError, "missing terminal results"):
            timings.emit_artifact(slow_only, selected, 1, 1)
        passing = "PASS [ 1.000s] (1/1) fixture::binary other"
        with self.assertRaisesRegex(timings.TimingArtifactError, "unselected terminal identity"):
            timings.emit_artifact(passing, selected, 1, 1)
        with self.assertRaisesRegex(timings.TimingArtifactError, "missing terminal results"):
            timings.emit_artifact("", selected, 1, 1)

    def test_malformed_terminal_rows_and_ordinals_fail_closed(self):
        selected = self.listing([("fixture::binary", "test")])
        with self.assertRaisesRegex(timings.TimingArtifactError, "malformed terminal result"):
            timings.emit_artifact("PASS [bad] (1/1) fixture::binary test", selected, 1, 1)
        with self.assertRaisesRegex(timings.TimingArtifactError, "invalid terminal ordinal"):
            timings.emit_artifact("PASS [ 1.000s] (0/0) fixture::binary test", selected, 1, 1)
        with self.assertRaisesRegex(timings.TimingArtifactError, "unsupported status SKIP"):
            timings.emit_artifact("SKIP [ 1.000s] (1/1) fixture::binary test", selected, 1, 1)

    def test_empty_shard_emits_a_valid_empty_terminal_set(self):
        selected = self.listing(
            [("fixture::binary", "test")], selected=set()
        )
        artifact = timings.emit_artifact("", selected, 1, 7)
        self.assertEqual(artifact["records"], [])
        timings.validate_artifact(artifact, expected_ids=set(), expected_shard=7)

    def test_artifact_rejects_bad_provenance_units_statuses_and_identities(self):
        selected = self.listing([("fixture::binary", "test")])
        good = timings.emit_artifact(
            "PASS [ 1.000s] (1/1) fixture::binary test", selected, 123, 1
        )
        cases = (
            (lambda value: value.pop("unit"), "invalid top-level fields"),
            (lambda value: value.update(unit="milliseconds"), "unit must be"),
            (lambda value: value.update(run_id=0), "run_id must be"),
            (lambda value: value.update(shard=8), "invalid shard provenance"),
            (lambda value: value.update(shard_count=6), "invalid shard provenance"),
            (lambda value: value["records"][0].update(result="SLOW"), "invalid terminal result"),
            (lambda value: value["records"][0].update(seconds=float("nan")), "invalid seconds"),
            (lambda value: value["records"][0].update(seconds=True), "invalid seconds"),
            (lambda value: value["records"][0].update(test_id="binary-only"), "noncanonical test_id"),
            (lambda value: value["records"].append(copy.deepcopy(value["records"][0])), "duplicate terminal identity"),
        )
        for mutate, message in cases:
            with self.subTest(message=message):
                candidate = copy.deepcopy(good)
                mutate(candidate)
                with self.assertRaisesRegex(timings.TimingArtifactError, message):
                    timings.validate_artifact(candidate)
        with self.assertRaisesRegex(timings.TimingArtifactError, "run_id does not match"):
            timings.validate_artifact(good, expected_run_id=124)
        with self.assertRaisesRegex(timings.TimingArtifactError, "shard does not match"):
            timings.validate_artifact(good, expected_shard=2)
        with self.assertRaisesRegex(timings.TimingArtifactError, "terminal identities differ"):
            timings.validate_artifact(good, expected_ids=set())
        fallback = {
            "test_id": "fixture::binary unseen",
            "seconds": 2.0,
            "method": "binary-median",
        }
        for mutate, message in (
            (lambda value: value.update(fallbacks=[{"test_id": fallback["test_id"]}]), "fallback record 1 has invalid fields"),
            (lambda value: value.update(fallbacks=[{**fallback, "method": "default"}]), "invalid method"),
            (lambda value: value.update(fallbacks=[{**fallback, "seconds": float("nan")}]), "invalid seconds"),
            (lambda value: value.update(fallbacks=[fallback, copy.deepcopy(fallback)]), "duplicate fallback identity"),
        ):
            with self.subTest(message=message):
                candidate = copy.deepcopy(good)
                mutate(candidate)
                with self.assertRaisesRegex(timings.TimingArtifactError, message):
                    timings.validate_artifact(candidate)

    def test_selected_inventory_rejects_duplicate_or_malformed_identities(self):
        listing = self.listing([("fixture::binary", "test")])
        listing["rust-suites"]["duplicate"] = {
            "binary-id": "fixture::binary",
            "binary-name": "binary",
            "testcases": {"test": {"filter-match": {"status": "mismatch"}}},
        }
        listing["test-count"] += 1
        with self.assertRaisesRegex(timings.TimingArtifactError, "duplicate identity"):
            timings.selected_identities(listing)

        listing = self.listing([("fixture::binary", "test")])
        listing["rust-suites"]["0"]["testcases"]["test"]["filter-match"]["status"] = "unknown"
        with self.assertRaisesRegex(timings.TimingArtifactError, "invalid filter-match status"):
            timings.selected_identities(listing)


if __name__ == "__main__":
    unittest.main()
