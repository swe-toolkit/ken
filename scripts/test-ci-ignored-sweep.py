#!/usr/bin/env python3
"""Targeted unit controls for scripts/ci-ignored-sweep.py."""

from __future__ import annotations

from contextlib import redirect_stdout
import importlib.util
import io
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
            self.assertIn(f"binary(={row['binary']})", expression)
            self.assertIn(f"test(={row['test']})", expression)

    def test_list_count_must_equal_the_anchored_derivation(self) -> None:
        rows = SWEEP.load_registry(SWEEP.DEFAULT_REGISTRY)
        exempt = {
            (row["package"], row["binary"], row["test"]) for row in rows
        }
        selected_identities = {
            ("ken-runtime", "ken-runtime", f"base_debt_{index}")
            for index in range(47)
        }
        all_identities = selected_identities | exempt

        def listing(
            identities: set[tuple[str, str, str]],
        ) -> dict[str, object]:
            suites: dict[str, object] = {}
            for index, (package, binary, test) in enumerate(sorted(identities)):
                suites[str(index)] = {
                    "package-name": package,
                    "binary-name": binary,
                    "testcases": {
                        test: {
                            "ignored": True,
                            "filter-match": {"status": "matches"},
                        }
                    },
                }
            return {"test-count": len(identities), "rust-suites": suites}

        with tempfile.TemporaryDirectory() as directory:
            all_listing = Path(directory) / "all.json"
            selected_listing = Path(directory) / "selected.json"
            all_listing.write_text(json.dumps(listing(all_identities)), encoding="utf-8")
            selected_listing.write_text(
                json.dumps(listing(selected_identities)), encoding="utf-8"
            )
            SWEEP.verify_lists(all_listing, selected_listing, 47, rows)
            with self.assertRaises(SWEEP.SweepError):
                SWEEP.verify_lists(all_listing, selected_listing, 46, rows)

    def test_completed_report_names_passing_rows(self) -> None:
        log_text = """
        Starting 47 tests across 4 binaries
        PASS [  0.001s] l1_acceptance sec24_char_excludes_surrogates
        Summary [  1.000s] 47 tests run: 1 passed, 46 failed
        """
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "run.log"
            log.write_text(log_text, encoding="utf-8")
            output = io.StringIO()
            with redirect_stdout(output):
                SWEEP.report(log, 47, 100)

        report = output.getvalue()
        identity = "l1_acceptance sec24_char_excludes_surrogates"
        self.assertIn("47 selected; 1 passed", report)
        self.assertIn(f"- {identity}", report)
        self.assertIn(
            f"::notice title=Ignored row now passes::{identity}; route to the "
            "owner node named by its ignore attribute, or to the Steward when "
            "no live node is named",
            report,
        )

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
