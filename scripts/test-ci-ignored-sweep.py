#!/usr/bin/env python3
"""Targeted unit controls for scripts/ci-ignored-sweep.py."""

from __future__ import annotations

from contextlib import redirect_stdout
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("ci-ignored-sweep.py")
SPEC = importlib.util.spec_from_file_location("ci_ignored_sweep", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
SWEEP = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SWEEP)


def listing(identities: set[tuple[str, str, str]]) -> dict[str, object]:
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


def registry_identities(rows: list[dict[str, str]]) -> set[tuple[str, str, str]]:
    identities: set[tuple[str, str, str]] = set()
    for row in rows:
        package, remainder = row["test_path"].split("::", 1)
        if package == "ken-interp":
            binary, test = remainder.split("::", 1)
        else:
            binary, test = "ken_runtime_lib", remainder
        identities.add((package, binary, test))
    return identities


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
        identities = registry_identities(rows)
        expression = SWEEP.filter_expression(rows, identities)
        self.assertTrue(expression.startswith("not ("))
        for package, binary, test in identities:
            self.assertIn(f"package(={package})", expression)
            self.assertIn(f"binary(={binary})", expression)
            self.assertIn(f"test(={test})", expression)

        cost_identity = next(
            identity for identity in identities if identity[0] == "ken-runtime"
        )
        renamed_identity = (cost_identity[0], "renamed_lib_target", cost_identity[2])
        renamed_identities = identities - {cost_identity} | {renamed_identity}
        renamed_expression = SWEEP.filter_expression(rows, renamed_identities)
        self.assertIn("binary(=renamed_lib_target)", renamed_expression)
        self.assertNotIn(f"binary(={cost_identity[1]})", renamed_expression)

    def test_registry_resolution_rejects_missing_and_ambiguous_paths(self) -> None:
        row = {
            "test_path": "ken-runtime::module::test_name",
            "class": "policy-cost",
            "readmission": "not applicable",
        }
        with self.assertRaisesRegex(SWEEP.SweepError, "resolves to 0"):
            SWEEP.resolve_exemptions([row], set())
        ambiguous = {
            ("ken-runtime", "first", "module::test_name"),
            ("ken-runtime", "second", "module::test_name"),
        }
        with self.assertRaisesRegex(SWEEP.SweepError, "resolves to 2"):
            SWEEP.resolve_exemptions([row], ambiguous)

    def test_list_count_must_equal_the_anchored_derivation(self) -> None:
        rows = SWEEP.load_registry(SWEEP.DEFAULT_REGISTRY)
        exempt = registry_identities(rows)
        selected_identities = {
            ("ken-runtime", "ken-runtime", f"base_debt_{index}")
            for index in range(47)
        }
        all_identities = selected_identities | exempt

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
        Final status:
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
        self.assertEqual(report.count(f"- {identity}"), 1)
        notice = (
            f"::notice title=Ignored row now passes::{identity}; route to the "
            "owner node named by its ignore attribute, or to the Steward when "
            "no live node is named"
        )
        self.assertEqual(report.count(notice), 1)

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

    def test_cli_exit_contract_distinguishes_all_three_outcomes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            nominal_log = root / "nominal.log"
            finding_log = root / "finding.log"
            all_listing = root / "all.json"
            missing_registry = root / "missing.toml"
            nominal_log.write_text(
                "Summary [ 1.000s] 47 tests run: 0 passed, 47 failed\n",
                encoding="utf-8",
            )
            finding_log.write_text(
                "nextest output\n"
                "PASS [ 0.001s] l1_acceptance repaired_row\n"
                "Summary [ 1.000s] 47 tests run: 1 passed, 46 failed\n",
                encoding="utf-8",
            )
            all_listing.write_text(json.dumps(listing(set())), encoding="utf-8")
            missing_registry.write_text(
                "version = 1\n\n"
                "[[exemption]]\n"
                'test_path = "ken-runtime::missing::row"\n'
                'class = "policy-cost"\n'
                'readmission = "not applicable"\n',
                encoding="utf-8",
            )

            instrument = subprocess.run(
                [
                    sys.executable,
                    str(SCRIPT),
                    "--registry",
                    str(missing_registry),
                    "filter",
                    str(all_listing),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            finding = subprocess.run(
                [sys.executable, str(SCRIPT), "report", str(finding_log), "47", "100"],
                check=False,
                capture_output=True,
                text=True,
            )
            nominal = subprocess.run(
                [sys.executable, str(SCRIPT), "report", str(nominal_log), "47", "100"],
                check=False,
                capture_output=True,
                text=True,
            )

        self.assertEqual(
            (instrument.returncode, finding.returncode, nominal.returncode),
            (2, 0, 0),
        )
        self.assertIn("resolves to 0", instrument.stderr)
        self.assertIn("::notice title=Ignored row now passes::", finding.stdout)
        self.assertIn("No ignored row passed", nominal.stdout)


if __name__ == "__main__":
    unittest.main()
