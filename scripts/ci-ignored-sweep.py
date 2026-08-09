#!/usr/bin/env python3
"""Build and verify the ignored-test sweep's selection and report."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import subprocess
import sys
import tomllib


ROOT = Path(__file__).resolve().parent.parent
DEFAULT_REGISTRY = ROOT / ".github" / "ignored-test-exemptions.toml"
POPULATION_PATHS = (
    "crates/ken-cli",
    "crates/ken-verify",
    "crates/ken-runtime",
    "crates/ken-interp",
)
ALLOWED_CLASSES = {"policy-cost", "placeholder-no-assertions"}
SUMMARY_RE = re.compile(r"\b(\d+) tests? run:")
PASS_RE = re.compile(r"^\s*PASS\s+\[[^]]+\]\s+(.+?)\s*$", re.MULTILINE)


class SweepError(RuntimeError):
    """The sweep instrument could not establish its claimed measurement."""


def load_registry(path: Path) -> list[dict[str, str]]:
    with path.open("rb") as source:
        document = tomllib.load(source)
    if document.get("version") != 1:
        raise SweepError("registry version must be 1")
    rows = document.get("exemption")
    if not isinstance(rows, list) or not rows:
        raise SweepError("registry must contain at least one exemption")

    required = {
        "test_path",
        "package",
        "binary",
        "test",
        "class",
        "readmission",
    }
    seen: set[str] = set()
    for row in rows:
        if not isinstance(row, dict) or set(row) != required:
            raise SweepError(f"registry row must contain exactly {sorted(required)}")
        if not all(isinstance(row[field], str) and row[field] for field in required):
            raise SweepError("registry fields must be non-empty strings")
        if row["class"] not in ALLOWED_CLASSES:
            raise SweepError(f"unknown exemption class: {row['class']}")
        if row["test_path"] in seen:
            raise SweepError(f"duplicate test_path: {row['test_path']}")
        if not row["test_path"].startswith(f"{row['package']}::"):
            raise SweepError(f"test_path is not keyed by package: {row['test_path']}")
        seen.add(row["test_path"])
    return rows


def filter_expression(rows: list[dict[str, str]]) -> str:
    members = [
        f"(package(={row['package']}) & binary(={row['binary']}) & "
        f"test(={row['test']}))"
        for row in rows
    ]
    return f"not ({' + '.join(members)})"


def ignored_attribute_count() -> int:
    command = [
        "git",
        "grep",
        "-nE",
        r"^[[:space:]]*#\[ignore",
        "--",
        *POPULATION_PATHS,
    ]
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode not in (0, 1):
        raise SweepError(result.stderr.strip() or "anchored git grep failed")
    return sum(1 for line in result.stdout.splitlines() if line)


def expected_count(rows: list[dict[str, str]]) -> int:
    expected = ignored_attribute_count() - len(rows)
    if expected <= 0:
        raise SweepError(
            f"derived selected count must be positive, got {expected}"
        )
    return expected


def read_listing(path: Path) -> tuple[int, set[tuple[str, str, str]]]:
    with path.open("rb") as source:
        listing = json.load(source)
    count = listing.get("test-count")
    if not isinstance(count, int):
        raise SweepError(f"{path} has no integer test-count")
    identities: set[tuple[str, str, str]] = set()
    suites = listing.get("rust-suites")
    if not isinstance(suites, dict):
        raise SweepError(f"{path} has no rust-suites map")
    for suite in suites.values():
        package = suite.get("package-name")
        binary = suite.get("binary-name")
        testcases = suite.get("testcases")
        if (
            not isinstance(package, str)
            or not isinstance(binary, str)
            or not isinstance(testcases, dict)
        ):
            raise SweepError(f"{path} contains a malformed suite")
        for test, metadata in testcases.items():
            if not isinstance(metadata, dict):
                raise SweepError(f"{path} contains malformed test metadata")
            filter_match = metadata.get("filter-match", {})
            if metadata.get("ignored") and filter_match.get("status") == "matches":
                identities.add((package, binary, test))
    if len(identities) != count:
        raise SweepError(
            f"{path} reports {count} tests but exposes {len(identities)} matching "
            "ignored identities"
        )
    return count, identities


def verify_lists(
    all_path: Path,
    selected_path: Path,
    expected: int,
    rows: list[dict[str, str]],
) -> None:
    all_count, all_identities = read_listing(all_path)
    selected, selected_identities = read_listing(selected_path)
    if all_count != expected + len(rows):
        raise SweepError(
            f"nextest discovered {all_count} ignored rows; anchored derivation "
            f"expects {expected + len(rows)}"
        )
    if selected != expected:
        raise SweepError(
            f"nextest selected {selected} ignored rows; anchored derivation expects "
            f"{expected}"
        )
    if selected == 0:
        raise SweepError("nextest selected zero ignored rows")
    exemptions = {
        (row["package"], row["binary"], row["test"]) for row in rows
    }
    missing = exemptions - all_identities
    if missing:
        raise SweepError(f"registry identities absent from ignored population: {missing}")
    if exemptions & selected_identities:
        raise SweepError("a registered exemption remains selected")
    if selected_identities != all_identities - exemptions:
        raise SweepError("selected identities are not population minus registry")
    print(
        f"ignored sweep selection: {selected} of {all_count} rows; "
        "anchored derivation and registry subtraction agree"
    )


def report(path: Path, expected: int, exit_status: int) -> None:
    text = path.read_text(encoding="utf-8", errors="replace")
    summaries = [int(match.group(1)) for match in SUMMARY_RE.finditer(text)]
    if not summaries:
        raise SweepError("nextest output has no completed-run summary")
    observed = summaries[-1]
    if observed != expected:
        raise SweepError(
            f"nextest summary reports {observed} rows; expected {expected}"
        )
    if exit_status not in (0, 100):
        raise SweepError(
            f"nextest exit {exit_status} is neither complete success nor test failure"
        )

    passing = [match.group(1) for match in PASS_RE.finditer(text)]
    print(f"Ignored-row sweep completed: {observed} selected; {len(passing)} passed.")
    if passing:
        print("Passing ignored rows need owner routing:")
        for identity in passing:
            print(f"- {identity}")
            print(
                "::notice title=Ignored row now passes::"
                f"{identity}; route to the owner node named by its ignore attribute, "
                "or to the Steward when no live node is named"
            )
    else:
        print("No ignored row passed in this run.")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--registry", type=Path, default=DEFAULT_REGISTRY, help=argparse.SUPPRESS
    )
    subcommands = parser.add_subparsers(dest="command", required=True)
    subcommands.add_parser("filter")
    subcommands.add_parser("expected")
    verify = subcommands.add_parser("verify-list")
    verify.add_argument("all_listing", type=Path)
    verify.add_argument("selected_listing", type=Path)
    verify.add_argument("expected", type=int)
    report_parser = subcommands.add_parser("report")
    report_parser.add_argument("log", type=Path)
    report_parser.add_argument("expected", type=int)
    report_parser.add_argument("exit_status", type=int)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        rows = load_registry(args.registry)
        if args.command == "filter":
            print(filter_expression(rows))
        elif args.command == "expected":
            print(expected_count(rows))
        elif args.command == "verify-list":
            verify_lists(
                args.all_listing, args.selected_listing, args.expected, rows
            )
        elif args.command == "report":
            report(args.log, args.expected, args.exit_status)
        else:
            raise AssertionError(args.command)
    except (OSError, ValueError, SweepError) as error:
        print(f"ignored-sweep instrument error: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
