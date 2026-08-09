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
SUMMARY_RE = re.compile(
    r"\b(?P<total>\d+) tests? run:\s+(?P<passed>\d+) passed\b"
)
PASS_RE = re.compile(
    r"^\s*PASS\s+\[[^]]+\]\s+(?P<payload>.+?)\s*$"
)
PASS_PREFIX_RE = re.compile(r"^\s*PASS(?:\s|$)")
COUNTER_RE = re.compile(
    r"^\((?P<index>\d+)/(?P<total>\d+)\)\s+(?P<identity>.+)$"
)


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

    required = {"test_path", "class", "readmission"}
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
        if "::" not in row["test_path"]:
            raise SweepError(f"test_path is not keyed by package: {row['test_path']}")
        seen.add(row["test_path"])
    return rows


def possible_test_paths(identity: tuple[str, str, str]) -> set[str]:
    package, binary, test = identity
    return {f"{package}::{test}", f"{package}::{binary}::{test}"}


def resolve_exemptions(
    rows: list[dict[str, str]],
    identities: set[tuple[str, str, str]],
) -> set[tuple[str, str, str]]:
    resolved: set[tuple[str, str, str]] = set()
    for row in rows:
        matches = {
            identity
            for identity in identities
            if row["test_path"] in possible_test_paths(identity)
        }
        if len(matches) != 1:
            raise SweepError(
                f"registry test_path {row['test_path']!r} resolves to "
                f"{len(matches)} ignored listing rows; expected exactly one"
            )
        resolved.update(matches)
    if len(resolved) != len(rows):
        raise SweepError("multiple registry test_paths resolve to one listing row")
    return resolved


def filter_expression(
    rows: list[dict[str, str]],
    identities: set[tuple[str, str, str]],
) -> str:
    exemptions = sorted(resolve_exemptions(rows, identities))
    members = [
        f"(package(={package}) & binary(={binary}) & test(={test}))"
        for package, binary, test in exemptions
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


def read_listing(
    path: Path,
) -> tuple[
    int,
    set[tuple[str, str, str]],
    dict[str, tuple[str, str, str]],
]:
    with path.open("rb") as source:
        listing = json.load(source)
    count = listing.get("test-count")
    if not isinstance(count, int):
        raise SweepError(f"{path} has no integer test-count")
    identities: set[tuple[str, str, str]] = set()
    human_identities: dict[str, tuple[str, str, str]] = {}
    suites = listing.get("rust-suites")
    if not isinstance(suites, dict):
        raise SweepError(f"{path} has no rust-suites map")
    for suite in suites.values():
        package = suite.get("package-name")
        binary = suite.get("binary-name")
        binary_id = suite.get("binary-id")
        testcases = suite.get("testcases")
        if (
            not isinstance(package, str)
            or not isinstance(binary, str)
            or not isinstance(binary_id, str)
            or not binary_id
            or not isinstance(testcases, dict)
        ):
            raise SweepError(f"{path} contains a malformed suite")
        for test, metadata in testcases.items():
            if not isinstance(test, str) or not test or not isinstance(metadata, dict):
                raise SweepError(f"{path} contains malformed test metadata")
            filter_match = metadata.get("filter-match", {})
            if metadata.get("ignored") and filter_match.get("status") == "matches":
                identity = (package, binary, test)
                identities.add(identity)
                human_identity = f"{binary_id} {test}"
                prior = human_identities.get(human_identity)
                if prior is not None and prior != identity:
                    raise SweepError(
                        f"{path} maps human identity {human_identity!r} to "
                        "multiple selected rows"
                    )
                human_identities[human_identity] = identity
    if len(identities) > count:
        raise SweepError(
            f"{path} reports {count} total discovered tests but exposes "
            f"{len(identities)} matching ignored identities"
        )
    return count, identities, human_identities


def matching_identity_evidence(
    identities: set[tuple[str, str, str]],
) -> str:
    rendered = [
        f"{package}::{binary}::{test}"
        for package, binary, test in sorted(identities)
    ]
    limit = 20
    evidence = [f"  - {identity}" for identity in rendered[:limit]]
    if len(rendered) > limit:
        evidence.append(f"  - ... {len(rendered) - limit} more")
    return "\n".join(evidence)


def verify_lists(
    all_path: Path,
    selected_path: Path,
    expected: int,
    rows: list[dict[str, str]],
) -> None:
    all_discovered, all_identities, _ = read_listing(all_path)
    selected_discovered, selected_identities, _ = read_listing(selected_path)
    all_matching = len(all_identities)
    selected_matching = len(selected_identities)
    source_ignored = expected + len(rows)
    if all_discovered != selected_discovered:
        raise SweepError(
            "nextest listings disagree on total discovered tests: "
            f"unfiltered ignored listing reports {all_discovered}, selected "
            f"listing reports {selected_discovered}"
        )
    if all_matching != source_ignored:
        raise SweepError(
            f"source attribute census reports {source_ignored} ignored rows; "
            f"nextest listing reports {all_discovered} total discovered tests and "
            f"{all_matching} rows matching the ignored-only filter. Matching "
            "listing identities:\n"
            f"{matching_identity_evidence(all_identities)}"
        )
    if selected_matching != expected:
        raise SweepError(
            f"anchored source census minus {len(rows)} registry exemptions "
            f"expects {expected} selected ignored rows; nextest listing reports "
            f"{selected_discovered} total discovered tests and "
            f"{selected_matching} rows matching the sweep filter. Matching "
            "selected identities:\n"
            f"{matching_identity_evidence(selected_identities)}"
        )
    if selected_matching == 0:
        raise SweepError("nextest selected zero ignored rows")
    exemptions = resolve_exemptions(rows, all_identities)
    if exemptions & selected_identities:
        raise SweepError("a registered exemption remains selected")
    if selected_identities != all_identities - exemptions:
        raise SweepError("selected identities are not population minus registry")
    print(
        f"ignored sweep selection: {selected_matching} selected of "
        f"{all_matching} ignored-only matches from {all_discovered} total "
        "discovered tests; source census and registry subtraction agree"
    )


def report(
    selected_path: Path,
    path: Path,
    expected: int,
    exit_status: int,
) -> None:
    selected_discovered, selected_identities, selected_human_identities = (
        read_listing(selected_path)
    )
    selected_matching = len(selected_identities)
    if selected_matching != expected:
        raise SweepError(
            f"selected listing reports {selected_discovered} total discovered "
            f"tests and {selected_matching} rows matching the sweep filter; "
            f"expected {expected} matching rows"
        )
    text = path.read_text(encoding="utf-8", errors="replace")
    summaries = [
        (int(match.group("total")), int(match.group("passed")))
        for match in SUMMARY_RE.finditer(text)
    ]
    if not summaries:
        raise SweepError("nextest output has no completed-run summary")
    observed, summary_passed = summaries[-1]
    if observed != expected:
        raise SweepError(
            f"nextest summary reports {observed} rows; expected {expected}"
        )
    if exit_status not in (0, 100):
        raise SweepError(
            f"nextest exit {exit_status} is neither complete success nor test failure"
        )

    passing: list[str] = []
    for line in text.splitlines():
        if not PASS_PREFIX_RE.match(line):
            continue
        match = PASS_RE.fullmatch(line)
        if match is None:
            raise SweepError(f"malformed nextest PASS status line: {line.strip()}")
        payload = match.group("payload").strip()
        if not payload:
            raise SweepError("nextest PASS status has no identity")
        if payload.startswith("("):
            counter = COUNTER_RE.fullmatch(payload)
            if counter is None:
                raise SweepError(f"malformed nextest progress counter: {payload}")
            index = int(counter.group("index"))
            total = int(counter.group("total"))
            if total != expected or not 1 <= index <= total:
                raise SweepError(
                    f"nextest progress counter {index}/{total} is inconsistent "
                    f"with expected population {expected}"
                )
            identity = counter.group("identity")
        else:
            identity = payload
        if identity not in selected_human_identities:
            raise SweepError(
                f"nextest PASS identity is not in the selected listing: {identity}"
            )
        if identity not in passing:
            passing.append(identity)
    if len(passing) != summary_passed:
        raise SweepError(
            f"nextest summary reports {summary_passed} passed rows but "
            f"status output identifies {len(passing)} unique selected rows"
        )
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
    filter_parser = subcommands.add_parser("filter")
    filter_parser.add_argument("all_listing", type=Path)
    subcommands.add_parser("expected")
    verify = subcommands.add_parser("verify-list")
    verify.add_argument("all_listing", type=Path)
    verify.add_argument("selected_listing", type=Path)
    verify.add_argument("expected", type=int)
    report_parser = subcommands.add_parser("report")
    report_parser.add_argument("selected_listing", type=Path)
    report_parser.add_argument("log", type=Path)
    report_parser.add_argument("expected", type=int)
    report_parser.add_argument("exit_status", type=int)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        rows = load_registry(args.registry)
        if args.command == "filter":
            _, identities, _ = read_listing(args.all_listing)
            print(filter_expression(rows, identities))
        elif args.command == "expected":
            print(expected_count(rows))
        elif args.command == "verify-list":
            verify_lists(
                args.all_listing, args.selected_listing, args.expected, rows
            )
        elif args.command == "report":
            report(
                args.selected_listing,
                args.log,
                args.expected,
                args.exit_status,
            )
        else:
            raise AssertionError(args.command)
    except (OSError, ValueError, SweepError) as error:
        print(f"ignored-sweep instrument error: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
