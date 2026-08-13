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
from unittest import mock


SCRIPT = Path(__file__).with_name("ci-ignored-sweep.py")
SPEC = importlib.util.spec_from_file_location("ci_ignored_sweep", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
SWEEP = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SWEEP)


def listing(
    identities: set[tuple[str, str, str]],
    nonmatching_count: int = 0,
) -> dict[str, object]:
    suites: dict[str, object] = {}
    for index, (package, binary, test) in enumerate(sorted(identities)):
        suites[str(index)] = {
            "package-name": package,
            "binary-name": binary,
            "binary-id": binary,
            "testcases": {
                test: {
                    "ignored": True,
                    "filter-match": {"status": "matches"},
                }
            },
        }
    if nonmatching_count:
        suites["nonmatching"] = {
            "package-name": "fixture-package",
            "binary-name": "ordinary-tests",
            "binary-id": "fixture-package::ordinary-tests",
            "testcases": {
                f"ordinary_test_{index}": {
                    "ignored": False,
                    "filter-match": {"status": "mismatch"},
                }
                for index in range(nonmatching_count)
            },
        }
    return {
        "test-count": len(identities) + nonmatching_count,
        "rust-suites": suites,
    }


def write_selected_listing(path: Path, *human_identities: str) -> None:
    identities = {
        ("fixture-package", binary_id, test)
        for binary_id, test in (
            human_identity.split(" ", 1) for human_identity in human_identities
        )
    }
    filler = 0
    while len(identities) < 47:
        identities.add(
            ("fixture-package", "all_failing", f"ignored_row_{filler}")
        )
        filler += 1
    path.write_text(json.dumps(listing(identities)), encoding="utf-8")


def registry_identities(rows: list[dict[str, str]]) -> set[tuple[str, str, str]]:
    identities: set[tuple[str, str, str]] = set()
    for row in rows:
        package, remainder = row["test_path"].split("::", 1)
        if package == "ken-interp" or (
            package == "ken-elaborator"
            and remainder.startswith("r3_c2_source_mixed_branch::")
        ):
            binary, test = remainder.split("::", 1)
        else:
            binary, test = "ken_runtime_lib", remainder
        identities.add((package, binary, test))
    return identities


class IgnoredSweepTests(unittest.TestCase):
    def test_row_claims_resolve_exactly_once_and_only_on_tests(self) -> None:
        with tempfile.TemporaryDirectory(dir=SWEEP.ROOT) as directory:
            root = Path(directory)
            rust_root = root / "crates"
            conformance_root = root / "conformance"
            rust_root.mkdir()
            surface_root = conformance_root / "surface"
            runtime_root = conformance_root / "runtime"
            surface_root.mkdir(parents=True)
            runtime_root.mkdir()
            (rust_root / "rows.rs").write_text(
                "// surface/example/ignored-documentation\n"
                "fn helper() {}\n\n"
                "/// surface/example/doc-covered\n"
                "/// explanatory continuation\n"
                "#[test]\n"
                "#[ignore]\n"
                "fn doc_covered_test() {}\n\n"
                "// surface/example/comment-covered\n"
                "// explanatory continuation\n"
                "#[test]\n"
                "fn comment_covered_test() {}\n\n"
                "/// runtime/example/runtime-covered\n"
                "#[test]\n"
                "fn runtime_covered_test() {}\n",
                encoding="utf-8",
            )
            (surface_root / "seed.md").write_text(
                "### surface/example/doc-covered (soundness)\n"
                "### surface/example/comment-covered [NODE]\n",
                encoding="utf-8",
            )
            (runtime_root / "seed.md").write_text(
                "### runtime/example/runtime-covered (property)\n",
                encoding="utf-8",
            )

            self.assertEqual(
                SWEEP.verify_row_claims(rust_root, conformance_root), 3
            )

    def test_row_claim_resolution_names_missing_and_duplicate_claims(self) -> None:
        with tempfile.TemporaryDirectory(dir=SWEEP.ROOT) as directory:
            root = Path(directory)
            rust_root = root / "crates"
            conformance_root = root / "conformance"
            rust_root.mkdir()
            runtime_root = conformance_root / "runtime"
            runtime_root.mkdir(parents=True)
            rust = rust_root / "rows.rs"
            rust.write_text(
                "// runtime/example/fabricated-row\n"
                "#[test]\n"
                "fn fabricated_test() {}\n",
                encoding="utf-8",
            )

            with self.assertRaises(SWEEP.SweepError) as missing:
                SWEEP.verify_row_claims(rust_root, conformance_root)
            self.assertIn("fabricated_test", str(missing.exception))
            self.assertIn(
                "runtime/example/fabricated-row", str(missing.exception)
            )
            self.assertIn("resolves to 0", str(missing.exception))

            first = runtime_root / "first.md"
            second = runtime_root / "second.md"
            heading = "### runtime/example/fabricated-row\n"
            first.write_text(heading, encoding="utf-8")
            second.write_text(heading, encoding="utf-8")
            with self.assertRaisesRegex(SWEEP.SweepError, "resolves to 2"):
                SWEEP.verify_row_claims(rust_root, conformance_root)

    def test_row_namespaces_come_from_directories_and_exclude_file_citations(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(dir=SWEEP.ROOT) as directory:
            root = Path(directory)
            rust_root = root / "crates"
            conformance_root = root / "conformance"
            added_root = conformance_root / "new-namespace"
            rust_root.mkdir()
            added_root.mkdir(parents=True)
            (rust_root / "rows.rs").write_text(
                "/// conformance/surface/numbers/seed-numbers.md\n"
                "/// spec/30-surface/35-numbers.md\n"
                "// new-namespace/example/derived\n"
                "#[test]\n"
                "fn derived_namespace_test() {}\n",
                encoding="utf-8",
            )
            (added_root / "seed.md").write_text(
                "### new-namespace/example/derived\n"
                "### surface/example/not-in-a-derived-namespace\n",
                encoding="utf-8",
            )

            namespaces = set(SWEEP.conformance_namespaces(conformance_root))
            self.assertEqual(namespaces, {"new-namespace"})
            self.assertTrue(namespaces.isdisjoint({"conformance", "spec"}))
            claim_re, _ = SWEEP.conformance_row_patterns(conformance_root)
            claims = SWEEP.rust_test_row_claims(rust_root, claim_re)
            tokens = {row for _, _, _, row in claims}
            self.assertEqual(tokens, {"new-namespace/example/derived"})
            SWEEP.assert_row_tokens_are_row_ids(
                tokens, namespaces, "fixture claims"
            )
            self.assertEqual(
                SWEEP.verify_row_claims(rust_root, conformance_root), 1
            )

    def test_row_claims_and_headings_reject_markdown_file_paths(self) -> None:
        with tempfile.TemporaryDirectory(dir=SWEEP.ROOT) as directory:
            root = Path(directory)
            rust_root = root / "crates"
            conformance_root = root / "conformance"
            surface_root = conformance_root / "surface"
            rust_root.mkdir()
            surface_root.mkdir(parents=True)
            rust = rust_root / "rows.rs"
            heading = surface_root / "seed.md"
            rust.write_text(
                "/// surface/numbers/seed-numbers.md\n"
                "#[test]\n"
                "fn namespace_relative_file_path() {}\n",
                encoding="utf-8",
            )
            heading.write_text(
                "### surface/numbers/ordinary-row\n", encoding="utf-8"
            )

            with self.assertRaisesRegex(SWEEP.SweepError, "file-path citations"):
                SWEEP.verify_row_claims(rust_root, conformance_root)

            rust.write_text(
                "/// surface/numbers/ordinary-row\n"
                "#[test]\n"
                "fn ordinary_row() {}\n",
                encoding="utf-8",
            )
            heading.write_text(
                "### surface/numbers/ordinary-row\n"
                "### surface/numbers/seed-numbers.md\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(
                SWEEP.SweepError, "headings contain markdown file paths"
            ):
                SWEEP.verify_row_claims(rust_root, conformance_root)

    def test_file_path_roots_cannot_be_conformance_namespaces(self) -> None:
        with tempfile.TemporaryDirectory(dir=SWEEP.ROOT) as directory:
            conformance_root = Path(directory) / "conformance"
            (conformance_root / "surface").mkdir(parents=True)
            (conformance_root / "spec").mkdir()

            with self.assertRaisesRegex(SWEEP.SweepError, "citation roots: spec"):
                SWEEP.conformance_namespaces(conformance_root)

    def test_workspace_population_comes_from_every_metadata_member(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            first = root / "crates" / "first"
            second = root / "tools" / "second"
            first.mkdir(parents=True)
            second.mkdir(parents=True)
            document = {
                "workspace_members": ["first-id", "second-id"],
                "packages": [
                    {
                        "id": "first-id",
                        "name": "first",
                        "manifest_path": str(first / "Cargo.toml"),
                    },
                    {
                        "id": "second-id",
                        "name": "second",
                        "manifest_path": str(second / "Cargo.toml"),
                    },
                ],
            }

            self.assertEqual(
                SWEEP.workspace_packages_from_metadata(document, root),
                {"first": Path("crates/first"), "second": Path("tools/second")},
            )
            document["workspace_members"].append("missing-id")
            with self.assertRaisesRegex(
                SWEEP.SweepError, "omitted workspace member packages"
            ):
                SWEEP.workspace_packages_from_metadata(document, root)

    def test_source_census_receives_every_workspace_package_path(self) -> None:
        packages = {
            "first": Path("crates/first"),
            "second": Path("tools/second"),
        }
        completed = subprocess.CompletedProcess(
            args=[], returncode=0, stdout="", stderr=""
        )
        with mock.patch.object(SWEEP.subprocess, "run", return_value=completed) as run:
            self.assertEqual(SWEEP.ignored_attribute_count(packages), 0)

        command = run.call_args.args[0]
        self.assertEqual(command[-2:], ["crates/first", "tools/second"])
        self.assertNotIn("crates/ken-runtime", command)

    def test_blocked_relation_requires_exact_source_reason_agreement(self) -> None:
        with tempfile.TemporaryDirectory(dir=SWEEP.ROOT) as directory:
            package_root = Path(directory)
            source = package_root / "src" / "lib.rs"
            source.parent.mkdir()
            source.write_text(
                "#[test]\n"
                '#[ignore = "blocked at exact_relation before completion"]\n'
                "fn blocked_control() {}\n",
                encoding="utf-8",
            )
            rows = [
                {
                    "test_path": "fixture::tests::blocked_control",
                    "class": "blocked-upstream-relation",
                    "readmission": "exact_relation",
                }
            ]
            relative_root = package_root.relative_to(SWEEP.ROOT)

            self.assertEqual(
                SWEEP.verify_blocked_upstream_relations(
                    rows, {"fixture": relative_root}
                ),
                1,
            )
            rows[0]["readmission"] = "different_relation"
            with self.assertRaisesRegex(
                SWEEP.SweepError, "does not name readmission relation"
            ):
                SWEEP.verify_blocked_upstream_relations(
                    rows, {"fixture": relative_root}
                )

            rows[0]["readmission"] = "exact"
            with self.assertRaisesRegex(
                SWEEP.SweepError, "does not name readmission relation"
            ):
                SWEEP.verify_blocked_upstream_relations(
                    rows, {"fixture": relative_root}
                )

    def test_checked_in_registry_has_all_declared_classes(self) -> None:
        rows = SWEEP.load_registry(SWEEP.DEFAULT_REGISTRY)
        classes = [row["class"] for row in rows]
        self.assertEqual(classes.count("policy-cost"), 1)
        self.assertEqual(classes.count("placeholder-no-assertions"), 3)
        self.assertEqual(classes.count("blocked-upstream-relation"), 1)
        self.assertEqual(len({row["test_path"] for row in rows}), 5)
        for row in rows:
            if row["class"] == "placeholder-no-assertions":
                self.assertIn("assert", row["readmission"])
            if row["class"] == "blocked-upstream-relation":
                self.assertRegex(
                    row["readmission"], r"^[A-Za-z][A-Za-z0-9_-]*$"
                )

    def test_blocked_relation_registry_rejects_non_symbol_readmission(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            registry = Path(directory) / "registry.toml"
            registry.write_text(
                "version = 1\n\n"
                "[[exemption]]\n"
                'test_path = "fixture::tests::blocked_control"\n'
                'class = "blocked-upstream-relation"\n'
                'readmission = "wait for the relation"\n',
                encoding="utf-8",
            )
            with self.assertRaisesRegex(
                SWEEP.SweepError, "must be one exact relation symbol"
            ):
                SWEEP.load_registry(registry)

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
        self.assertNotIn(
            f"package(={cost_identity[0]}) & binary(={cost_identity[1]})",
            renamed_expression,
        )

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

    def test_listing_matching_count_must_equal_the_anchored_derivation(self) -> None:
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
            all_listing.write_text(
                json.dumps(listing(all_identities, 2646 - len(all_identities))),
                encoding="utf-8",
            )
            selected_listing.write_text(
                json.dumps(
                    listing(
                        selected_identities,
                        2646 - len(selected_identities),
                    )
                ),
                encoding="utf-8",
            )
            SWEEP.verify_lists(all_listing, selected_listing, 47, rows)
            with self.assertRaises(SWEEP.SweepError) as mismatch:
                SWEEP.verify_lists(all_listing, selected_listing, 46, rows)
            diagnostic = str(mismatch.exception)
            self.assertIn("source attribute census reports 51 ignored rows", diagnostic)
            self.assertIn("2646 total discovered tests", diagnostic)
            self.assertIn("52 rows matching the ignored-only filter", diagnostic)
            self.assertIn("ken-runtime::ken-runtime::base_debt_0", diagnostic)

            selected_document = json.loads(selected_listing.read_text())
            selected_document["test-count"] = 2645
            selected_listing.write_text(
                json.dumps(selected_document), encoding="utf-8"
            )
            with self.assertRaisesRegex(
                SWEEP.SweepError, "listings disagree on total discovered tests"
            ):
                SWEEP.verify_lists(all_listing, selected_listing, 47, rows)

    def test_listing_derives_one_exact_human_identity_per_selected_row(self) -> None:
        document = listing({("fixture-package", "binary-name", "selected_test")})
        suite = next(iter(document["rust-suites"].values()))
        suite["binary-id"] = "fixture-package::binary-id"

        with tempfile.TemporaryDirectory() as directory:
            selected_listing = Path(directory) / "selected.json"
            selected_listing.write_text(json.dumps(document), encoding="utf-8")
            count, _, human_identities = SWEEP.read_listing(selected_listing)
            self.assertEqual(count, 1)
            self.assertEqual(
                set(human_identities),
                {"fixture-package::binary-id selected_test"},
            )

            duplicate = json.loads(json.dumps(document))
            duplicate["test-count"] = 2
            duplicate["rust-suites"]["second"] = {
                "package-name": "other-package",
                "binary-name": "other-binary",
                "binary-id": "fixture-package::binary-id",
                "testcases": {
                    "selected_test": {
                        "ignored": True,
                        "filter-match": {"status": "matches"},
                    }
                },
            }
            selected_listing.write_text(json.dumps(duplicate), encoding="utf-8")
            with self.assertRaisesRegex(SWEEP.SweepError, "multiple selected rows"):
                SWEEP.read_listing(selected_listing)

    def test_listing_total_is_not_the_ignored_matching_population(self) -> None:
        identities = {
            ("fixture-package", "ignored-tests", f"ignored_test_{index}")
            for index in range(51)
        }
        with tempfile.TemporaryDirectory() as directory:
            all_listing = Path(directory) / "all.json"
            all_listing.write_text(
                json.dumps(listing(identities, 2646 - len(identities))),
                encoding="utf-8",
            )
            discovered, matching, _ = SWEEP.read_listing(all_listing)

        self.assertEqual(discovered, 2646)
        self.assertEqual(len(matching), 51)

    def test_completed_report_names_passing_rows(self) -> None:
        log_text = """
        Starting 47 tests across 4 binaries
        PASS [  0.001s] (1/47) l1_acceptance sec24_char_excludes_surrogates
        Final status:
        PASS [  0.001s] l1_acceptance sec24_char_excludes_surrogates
        Summary [  1.000s] 47 tests run: 1 passed, 46 failed
        """
        with tempfile.TemporaryDirectory() as directory:
            selected_listing = Path(directory) / "selected.json"
            log = Path(directory) / "run.log"
            write_selected_listing(
                selected_listing,
                "l1_acceptance sec24_char_excludes_surrogates",
            )
            log.write_text(log_text, encoding="utf-8")
            output = io.StringIO()
            with redirect_stdout(output):
                SWEEP.report(selected_listing, log, 47, 100)

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
            selected_listing = Path(directory) / "selected.json"
            log = Path(directory) / "run.log"
            write_selected_listing(selected_listing, "l1_acceptance repaired_row")
            log.write_text(
                "Summary [ 0.001s] 0 tests run: 0 passed, 0 failed\n",
                encoding="utf-8",
            )
            with self.assertRaises(SWEEP.SweepError):
                SWEEP.report(selected_listing, log, 47, 0)
            log.write_text(
                "Summary [ 0.001s] 47 tests run: 0 passed, 47 failed\n",
                encoding="utf-8",
            )
            with self.assertRaises(SWEEP.SweepError):
                SWEEP.report(selected_listing, log, 47, 4)
            log.write_text(
                "PASS [ 0.001s] (1/46) l1_acceptance repaired_row\n"
                "Summary [ 1.000s] 47 tests run: 1 passed, 46 failed\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(SWEEP.SweepError, "counter 1/46"):
                SWEEP.report(selected_listing, log, 47, 100)
            for malformed in ("(x/47) row", "(1/x) row", "(1/47 row"):
                log.write_text(
                    f"PASS [ 0.001s] {malformed}\n"
                    "Summary [ 1.000s] 47 tests run: 1 passed, 46 failed\n",
                    encoding="utf-8",
                )
                with self.assertRaisesRegex(SWEEP.SweepError, "malformed"):
                    SWEEP.report(selected_listing, log, 47, 100)

            for summary_passed, status_line in (
                (1, "an unrelated line that matches nothing"),
                (0, "PASS [ 0.001s] l1_acceptance repaired_row"),
            ):
                with self.subTest(summary_passed=summary_passed):
                    log.write_text(
                        f"{status_line}\n"
                        f"Summary [ 1.000s] 47 tests run: {summary_passed} passed, "
                        f"{47 - summary_passed} failed\n",
                        encoding="utf-8",
                    )
                    with self.assertRaisesRegex(
                        SWEEP.SweepError, "summary reports"
                    ):
                        SWEEP.report(selected_listing, log, 47, 100)

    def test_pass_line_census_assigns_every_input_class(self) -> None:
        identity = "l1_acceptance sec24_char_excludes_surrogates"
        finding_logs = {
            "well-formed counter": [f"PASS [ 0.001s] (1/47) {identity}"],
            "no counter": [f"PASS [ 0.001s] {identity}"],
            "duplicate live/final identity": [
                f"PASS [ 0.001s] (1/47) {identity}",
                f"PASS [ 0.001s] {identity}",
            ],
        }
        instrument_lines = {
            "counter mismatch": "PASS [ 0.001s] (1/46) suite repaired_row",
            "counter index zero": "PASS [ 0.001s] (0/47) suite repaired_row",
            "counter out of range": "PASS [ 0.001s] (48/47) suite repaired_row",
            "non-numeric counter": "PASS [ 0.001s] (x/47) suite repaired_row",
            "unterminated counter": "PASS [ 0.001s] (1/47 suite repaired_row",
            "empty identity": "PASS [ 0.001s]   ",
            "decorated identity": "PASS [ 0.001s] [1/47] suite repaired_row",
            "malformed status": "PASS suite repaired_row",
        }

        with tempfile.TemporaryDirectory() as directory:
            selected_listing = Path(directory) / "selected.json"
            log = Path(directory) / "run.log"
            write_selected_listing(selected_listing, identity)
            for label, lines in finding_logs.items():
                log.write_text(
                    "\n".join(lines)
                    + "\nSummary [ 1.000s] 47 tests run: 1 passed, 46 failed\n",
                    encoding="utf-8",
                )
                output = io.StringIO()
                with redirect_stdout(output):
                    SWEEP.report(selected_listing, log, 47, 100)
                report = output.getvalue()
                self.assertIn("47 selected; 1 passed", report, label)
                self.assertEqual(report.count(f"- {identity}"), 1, label)
                self.assertEqual(
                    report.count("::notice title=Ignored row now passes::"),
                    1,
                    label,
                )

            for label, line in instrument_lines.items():
                with self.subTest(input_class=label):
                    log.write_text(
                        f"{line}\n"
                        "Summary [ 1.000s] 47 tests run: 1 passed, 46 failed\n",
                        encoding="utf-8",
                    )
                    with self.assertRaises(SWEEP.SweepError):
                        SWEEP.report(selected_listing, log, 47, 100)

            for label, fabricated in (
                ("decorated test token", "l1_acceptance [1/47]"),
                ("fabricated test token", "l1_acceptance not_a_listed_test"),
            ):
                with self.subTest(input_class=label):
                    log.write_text(
                        f"PASS [ 0.001s] {fabricated}\n"
                        "Summary [ 1.000s] 47 tests run: 1 passed, 46 failed\n",
                        encoding="utf-8",
                    )
                    with self.assertRaisesRegex(
                        SWEEP.SweepError, "not in the selected listing"
                    ):
                        SWEEP.report(selected_listing, log, 47, 100)

            log.write_text(
                "an unrelated line that matches nothing\n"
                "Summary [ 1.000s] 47 tests run: 0 passed, 47 failed\n",
                encoding="utf-8",
            )
            output = io.StringIO()
            with redirect_stdout(output):
                SWEEP.report(selected_listing, log, 47, 100)
            self.assertIn("47 selected; 0 passed", output.getvalue())
            self.assertIn("No ignored row passed", output.getvalue())

    def test_cli_exit_contract_distinguishes_all_three_outcomes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            nominal_log = root / "nominal.log"
            finding_log = root / "finding.log"
            malformed_log = root / "malformed.log"
            selected_listing = root / "selected.json"
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
            malformed_log.write_text(
                "PASS [ 0.001s] (x/47) l1_acceptance repaired_row\n"
                "Summary [ 1.000s] 47 tests run: 1 passed, 46 failed\n",
                encoding="utf-8",
            )
            all_listing.write_text(json.dumps(listing(set())), encoding="utf-8")
            write_selected_listing(selected_listing, "l1_acceptance repaired_row")
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
                [
                    sys.executable,
                    str(SCRIPT),
                    "report",
                    str(selected_listing),
                    str(finding_log),
                    "47",
                    "100",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            malformed = subprocess.run(
                [
                    sys.executable,
                    str(SCRIPT),
                    "report",
                    str(selected_listing),
                    str(malformed_log),
                    "47",
                    "100",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            nominal = subprocess.run(
                [
                    sys.executable,
                    str(SCRIPT),
                    "report",
                    str(selected_listing),
                    str(nominal_log),
                    "47",
                    "100",
                ],
                check=False,
                capture_output=True,
                text=True,
            )

        self.assertEqual(
            (
                instrument.returncode,
                malformed.returncode,
                finding.returncode,
                nominal.returncode,
            ),
            (2, 2, 0, 0),
        )
        self.assertIn("resolves to 0", instrument.stderr)
        self.assertIn("malformed nextest progress counter", malformed.stderr)
        self.assertIn("::notice title=Ignored row now passes::", finding.stdout)
        self.assertIn("No ignored row passed", nominal.stdout)


if __name__ == "__main__":
    unittest.main()
