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
DEFAULT_RUST_ROOT = ROOT / "crates"
DEFAULT_CONFORMANCE_ROOT = ROOT / "conformance"
ALLOWED_CLASSES = {
    "blocked-upstream-relation",
    "policy-cost",
    "placeholder-no-assertions",
    # A run-exemption for a MACRO-GENERATED ignored test that is deliberately inert
    # until a named readmission (e.g. a deferred mutation control retired-with-tie
    # pending a later increment). Unlike blocked-upstream-relation, its readmission
    # is NOT cross-checked against a source #[ignore] reason -- ignored_test_reasons()
    # keys off `fn NAME(` in source and is fundamentally blind to macro-generated
    # tests (the same blindness this whole fix removes from the count). The staleness
    # safeguard is recovered via nextest ground truth instead: resolve_exemptions
    # requires every exempted test to still resolve to exactly one --run-ignored=only
    # identity, so when the readmission lands and the test is un-ignored it leaves the
    # nextest ignored set and the sweep FAILS until the exemption is removed. The
    # readmission label documents the lift condition.
    "deferred-inert-control",
}
FILE_PATH_ROOTS = {"conformance", "spec"}
SUMMARY_RE = re.compile(
    r"\b(?P<total>\d+) tests? run:\s+(?P<passed>\d+) passed\b"
)
PASS_RE = re.compile(
    r"^\s*PASS\s+\[[^]]+\]\s+(?P<payload>.+?)\s*$"
)
PASS_PREFIX_RE = re.compile(r"^\s*PASS(?:\s|$)")
# nextest right-aligns the running index to the width of the total, so a
# single-digit index prints with a leading space once the total reaches two
# digits (e.g. "( 3/34)"). Tolerate that interior whitespace; the population
# consistency check below (index/total vs expected) remains the real invariant.
COUNTER_RE = re.compile(
    r"^\(\s*(?P<index>\d+)\s*/\s*(?P<total>\d+)\)\s+(?P<identity>.+)$"
)
LEVEL_THREE_HEADING_RE = re.compile(r"^###\s+(?P<token>\S+)(?:\s.*)?$")
TEST_ATTRIBUTE_RE = re.compile(r"^\s*#\[test\]\s*$")
ATTRIBUTE_OR_COMMENT_RE = re.compile(r"^\s*(?:#\[|//)")
TEST_FN_RE = re.compile(r"^\s*fn\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\(")
IGNORE_REASON_RE = re.compile(
    r'^\s*#\[ignore\s*=\s*"(?P<reason>[^"]+)"\]\s*$'
)
RELATION_SYMBOL_RE = re.compile(r"^[A-Za-z][A-Za-z0-9_-]*$")


class SweepError(RuntimeError):
    """The sweep instrument could not establish its claimed measurement."""


def workspace_packages_from_metadata(
    document: object, root: Path
) -> dict[str, Path]:
    if not isinstance(document, dict):
        raise SweepError("cargo metadata output is not an object")
    members = document.get("workspace_members")
    packages = document.get("packages")
    if not isinstance(members, list) or not all(
        isinstance(member, str) and member for member in members
    ):
        raise SweepError("cargo metadata has no valid workspace_members list")
    if not isinstance(packages, list):
        raise SweepError("cargo metadata has no packages list")
    member_ids = set(members)
    resolved: dict[str, Path] = {}
    resolved_ids: set[str] = set()
    for package in packages:
        if not isinstance(package, dict) or package.get("id") not in member_ids:
            continue
        package_id = package["id"]
        name = package.get("name")
        manifest_path = package.get("manifest_path")
        if (
            not isinstance(package_id, str)
            or not isinstance(name, str)
            or not name
            or not isinstance(manifest_path, str)
            or not manifest_path
        ):
            raise SweepError("cargo metadata contains a malformed workspace package")
        try:
            package_root = Path(manifest_path).resolve().parent.relative_to(
                root.resolve()
            )
        except ValueError as error:
            raise SweepError(
                f"workspace package {name!r} is outside the repository root"
            ) from error
        if name in resolved:
            raise SweepError(f"duplicate workspace package name: {name}")
        resolved[name] = package_root
        resolved_ids.add(package_id)
    missing = sorted(member_ids - resolved_ids)
    if missing:
        raise SweepError(
            "cargo metadata omitted workspace member packages: "
            + ", ".join(missing)
        )
    if not resolved:
        raise SweepError("cargo metadata resolved zero workspace packages")
    return resolved


def cargo_workspace_packages() -> dict[str, Path]:
    result = subprocess.run(
        [
            "cargo",
            "metadata",
            "--locked",
            "--no-deps",
            "--format-version",
            "1",
        ],
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode != 0:
        raise SweepError(result.stderr.strip() or "cargo metadata failed")
    try:
        document = json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise SweepError("cargo metadata returned malformed JSON") from error
    return workspace_packages_from_metadata(document, ROOT)


def conformance_namespaces(root: Path) -> tuple[str, ...]:
    namespaces = tuple(
        sorted(path.name for path in root.iterdir() if path.is_dir())
    )
    if not namespaces:
        raise SweepError(f"{root} contains no conformance namespace directories")
    collisions = FILE_PATH_ROOTS.intersection(namespaces)
    if collisions:
        raise SweepError(
            "conformance namespaces collide with file-path citation roots: "
            + ", ".join(sorted(collisions))
        )
    return namespaces


def conformance_row_patterns(
    root: Path,
) -> tuple[re.Pattern[str], re.Pattern[str]]:
    namespaces = conformance_namespaces(root)
    namespace = "|".join(re.escape(name) for name in namespaces)
    row = rf"(?P<row>(?:{namespace})/\S+)"
    return (
        re.compile(rf"^\s*//(?:/)?\s+{row}"),
        re.compile(rf"^###\s+{row}(?:\s.*)?$"),
    )


def assert_row_tokens_are_row_ids(
    tokens: set[str], namespaces: set[str], source: str
) -> None:
    outside_namespaces = sorted(
        token for token in tokens if token.partition("/")[0] not in namespaces
    )
    if outside_namespaces:
        raise SweepError(
            f"{source} matched tokens outside conformance namespaces: "
            + ", ".join(outside_namespaces)
        )
    file_paths = sorted(token for token in tokens if token.endswith(".md"))
    if file_paths:
        raise SweepError(
            f"{source} matched file-path citations as conformance row ids: "
            + ", ".join(file_paths)
        )


def rust_test_row_claims(
    root: Path, row_claim_re: re.Pattern[str]
) -> list[tuple[str, int, str, str]]:
    claims: list[tuple[str, int, str, str]] = []
    for path in sorted(root.rglob("*.rs")):
        lines = path.read_text(encoding="utf-8").splitlines()
        for index, line in enumerate(lines):
            function = TEST_FN_RE.match(line)
            if function is None:
                continue
            prefix: list[tuple[int, str]] = []
            cursor = index - 1
            while cursor >= 0 and ATTRIBUTE_OR_COMMENT_RE.match(lines[cursor]):
                prefix.append((cursor, lines[cursor]))
                cursor -= 1
            if not any(TEST_ATTRIBUTE_RE.match(candidate) for _, candidate in prefix):
                continue
            for claim_index, candidate in reversed(prefix):
                claim = row_claim_re.match(candidate)
                if claim is not None:
                    claims.append(
                        (
                            str(path.relative_to(ROOT)),
                            claim_index + 1,
                            function.group("name"),
                            claim.group("row"),
                        )
                    )
    return claims


def conformance_row_headings(
    root: Path, row_heading_re: re.Pattern[str]
) -> dict[str, list[tuple[str, int]]]:
    headings: dict[str, list[tuple[str, int]]] = {}
    level_three_tokens: set[str] = set()
    for path in sorted(root.rglob("*.md")):
        for line_number, line in enumerate(
            path.read_text(encoding="utf-8").splitlines(), start=1
        ):
            level_three = LEVEL_THREE_HEADING_RE.match(line)
            if level_three is not None:
                level_three_tokens.add(level_three.group("token"))
            heading = row_heading_re.match(line)
            if heading is not None:
                headings.setdefault(heading.group("row"), []).append(
                    (str(path.relative_to(ROOT)), line_number)
                )
    file_paths = sorted(
        token for token in level_three_tokens if token.endswith(".md")
    )
    if file_paths:
        raise SweepError(
            "level-three conformance headings contain markdown file paths: "
            + ", ".join(file_paths)
        )
    return headings


def verify_row_claims(rust_root: Path, conformance_root: Path) -> int:
    namespaces = set(conformance_namespaces(conformance_root))
    row_claim_re, row_heading_re = conformance_row_patterns(conformance_root)
    claims = rust_test_row_claims(rust_root, row_claim_re)
    if not claims:
        raise SweepError("Rust test row-claim census resolved zero claims")
    claim_tokens = {row for _, _, _, row in claims}
    assert_row_tokens_are_row_ids(claim_tokens, namespaces, "Rust test claims")
    headings = conformance_row_headings(conformance_root, row_heading_re)
    assert_row_tokens_are_row_ids(
        set(headings), namespaces, "conformance headings"
    )
    errors: list[str] = []
    for path, line, test, row in claims:
        matches = headings.get(row, [])
        if len(matches) != 1:
            locations = ", ".join(f"{match_path}:{match_line}" for match_path, match_line in matches)
            errors.append(
                f"{path}:{line} test {test} claims {row!r}, which resolves to "
                f"{len(matches)} conformance headings"
                + (f": {locations}" if locations else "")
            )
    if errors:
        raise SweepError("row-claim resolution failed:\n" + "\n".join(errors))
    print(
        f"conformance row claims: {len(claims)} Rust test claims resolved "
        f"to exactly one heading ({len(claim_tokens)} distinct row tokens)"
    )
    return len(claims)


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
        if (
            row["class"] == "blocked-upstream-relation"
            and RELATION_SYMBOL_RE.fullmatch(row["readmission"]) is None
        ):
            raise SweepError(
                "blocked-upstream-relation readmission must be one exact "
                f"relation symbol: {row['readmission']!r}"
            )
        if row["test_path"] in seen:
            raise SweepError(f"duplicate test_path: {row['test_path']}")
        if "::" not in row["test_path"]:
            raise SweepError(f"test_path is not keyed by package: {row['test_path']}")
        seen.add(row["test_path"])
    return rows


def ignored_test_reasons(package_root: Path) -> dict[str, list[tuple[str, int, str]]]:
    reasons: dict[str, list[tuple[str, int, str]]] = {}
    for path in sorted(package_root.rglob("*.rs")):
        lines = path.read_text(encoding="utf-8").splitlines()
        for index, line in enumerate(lines):
            function = TEST_FN_RE.match(line)
            if function is None:
                continue
            cursor = index - 1
            while cursor >= 0 and ATTRIBUTE_OR_COMMENT_RE.match(lines[cursor]):
                ignored = IGNORE_REASON_RE.match(lines[cursor])
                if ignored is not None:
                    reasons.setdefault(function.group("name"), []).append(
                        (
                            str(path.relative_to(ROOT)),
                            cursor + 1,
                            ignored.group("reason"),
                        )
                    )
                cursor -= 1
    return reasons


def verify_blocked_upstream_relations(
    rows: list[dict[str, str]], workspace_packages: dict[str, Path]
) -> int:
    blocked = [
        row for row in rows if row["class"] == "blocked-upstream-relation"
    ]
    package_reasons: dict[str, dict[str, list[tuple[str, int, str]]]] = {}
    errors: list[str] = []
    for row in blocked:
        package, _, test = row["test_path"].partition("::")
        function = test.rpartition("::")[2]
        package_root = workspace_packages.get(package)
        if package_root is None:
            errors.append(
                f"registry test_path {row['test_path']!r} names no workspace package"
            )
            continue
        if package not in package_reasons:
            package_reasons[package] = ignored_test_reasons(ROOT / package_root)
        reasons = package_reasons[package]
        matches = reasons.get(function, [])
        if len(matches) != 1:
            errors.append(
                f"registry test_path {row['test_path']!r} resolves to "
                f"{len(matches)} source #[ignore = \"...\"] reasons; expected exactly one"
            )
            continue
        path, line, reason = matches[0]
        relation = re.compile(
            rf"(?<![A-Za-z0-9_-]){re.escape(row['readmission'])}"
            r"(?![A-Za-z0-9_-])"
        )
        if relation.search(reason) is None:
            errors.append(
                f"{path}:{line} ignore reason for {row['test_path']!r} does not "
                f"name readmission relation {row['readmission']!r}"
            )
    if errors:
        raise SweepError(
            "blocked-upstream-relation verification failed:\n" + "\n".join(errors)
        )
    print(
        f"blocked upstream relations: {len(blocked)} registry readmissions "
        "agree with their source ignore reasons"
    )
    return len(blocked)


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


def expected_count(all_listing: Path, rows: list[dict[str, str]]) -> int:
    # Count ignored rows from nextest --list --run-ignored=only GROUND TRUTH, not a
    # static source grep. The former anchored `#[ignore` git-grep could not see a
    # macro-leading-token #[ignore] (e.g. generated_entry_checked_case!(#[ignore=..]
    # name, ...)), so it undercounted macro-generated ignored tests and disagreed with
    # nextest. nextest is the authority on generated-test identity, so the selected
    # count is the nextest ignored population minus the registry run-exemptions.
    _, all_identities, _ = read_listing(all_listing)
    expected = len(all_identities) - len(rows)
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
    # `expected` is the nextest ignored population minus the registry run-exemptions,
    # computed by `expected_count` from the SAME all-listing this reads; adding the
    # registry back must reconstruct the nextest ignored total. A mismatch means a
    # stale `expected` (computed from a different listing) -- a consistency guard, no
    # longer a source-grep-vs-nextest reconciliation.
    expected_total = expected + len(rows)
    if all_discovered != selected_discovered:
        raise SweepError(
            "nextest listings disagree on total discovered tests: "
            f"unfiltered ignored listing reports {all_discovered}, selected "
            f"listing reports {selected_discovered}"
        )
    if all_matching != expected_total:
        raise SweepError(
            f"the passed expected count plus {len(rows)} registry exemptions is "
            f"{expected_total}, but the nextest ignored-only listing reports "
            f"{all_matching} rows (of {all_discovered} discovered) -- a stale "
            "expected count (computed from a different listing). Matching "
            "listing identities:\n"
            f"{matching_identity_evidence(all_identities)}"
        )
    if selected_matching != expected:
        raise SweepError(
            f"nextest ignored population minus {len(rows)} registry exemptions "
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
        "discovered tests; nextest ground truth and registry subtraction agree"
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
    expected_parser = subcommands.add_parser("expected")
    expected_parser.add_argument("all_listing", type=Path)
    verify = subcommands.add_parser("verify-list")
    verify.add_argument("all_listing", type=Path)
    verify.add_argument("selected_listing", type=Path)
    verify.add_argument("expected", type=int)
    report_parser = subcommands.add_parser("report")
    report_parser.add_argument("selected_listing", type=Path)
    report_parser.add_argument("log", type=Path)
    report_parser.add_argument("expected", type=int)
    report_parser.add_argument("exit_status", type=int)
    subcommands.add_parser("verify-row-claims")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        if args.command == "verify-row-claims":
            verify_row_claims(DEFAULT_RUST_ROOT, DEFAULT_CONFORMANCE_ROOT)
            verify_blocked_upstream_relations(
                load_registry(args.registry), cargo_workspace_packages()
            )
            return 0
        rows = load_registry(args.registry)
        if args.command == "filter":
            _, identities, _ = read_listing(args.all_listing)
            print(filter_expression(rows, identities))
        elif args.command == "expected":
            print(expected_count(args.all_listing, rows))
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
