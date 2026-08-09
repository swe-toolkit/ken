#!/usr/bin/env python3
"""Targeted unit controls for scripts/ci-ignored-sweep.py."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("ci-ignored-sweep.py")
SPEC = importlib.util.spec_from_file_location("ci_ignored_sweep", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
SWEEP = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SWEEP)


class IgnoredSweepTests(unittest.TestCase):
    def test_checked_in_registry_has_one_cost_and_three_placeholders(self) -> None:
        rows = SWEEP.load_registry(SWEEP.DEFAULT_REGISTRY)
        classes = [row["class"] for row in rows]
        self.assertEqual(classes.count("policy-cost"), 1)
        self.assertEqual(classes.count("placeholder-no-assertions"), 3)
        self.assertEqual(len({row["test_path"] for row in rows}), 4)
        for row in rows:
            if row["class"] == "placeholder-no-assertions":
                self.assertIn("assert", row["readmission"])

    def test_filter_is_sweep_local_and_contains_every_registry_identity(self) -> None:
        rows = SWEEP.load_registry(SWEEP.DEFAULT_REGISTRY)
        expression = SWEEP.filter_expression(rows)
        self.assertTrue(expression.startswith("not ("))
        for row in rows:
            self.assertIn(f"package(={row['package']})", expression)
            self.assertIn(f"test(={row['test']})", expression)

    def test_list_count_must_equal_the_anchored_derivation(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            listing = Path(directory) / "list.json"
            listing.write_text(json.dumps({"test-count": 47}), encoding="utf-8")
            SWEEP.verify_list(listing, 47)
            with self.assertRaises(SWEEP.SweepError):
                SWEEP.verify_list(listing, 46)

    def test_completed_report_names_passing_rows(self) -> None:
        log_text = """
        PASS [  0.001s] l1_acceptance sec24_char_excludes_surrogates
        Summary [  1.000s] 47 tests run: 1 passed, 46 failed
        """
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "run.log"
            log.write_text(log_text, encoding="utf-8")
            SWEEP.report(log, 47, 100)

    def test_report_rejects_zero_or_incomplete_measurements(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "run.log"
            log.write_text(
                "Summary [ 0.001s] 0 tests run: 0 passed, 0 failed\n",
                encoding="utf-8",
            )
            with self.assertRaises(SWEEP.SweepError):
                SWEEP.report(log, 47, 0)
            log.write_text(
                "Summary [ 0.001s] 47 tests run: 0 passed, 47 failed\n",
                encoding="utf-8",
            )
            with self.assertRaises(SWEEP.SweepError):
                SWEEP.report(log, 47, 4)


if __name__ == "__main__":
    unittest.main()
