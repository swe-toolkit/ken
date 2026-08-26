#!/usr/bin/env python3
"""Reproduce CAT-REUSE-CENSUS closure, identities, depths, and grouping."""

from __future__ import annotations

import argparse
import collections
import functools
import hashlib
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

EXPECTED_EVIDENCE_REF = "ed5b4063f434cc7a2311143367928ee98f64fd86"
CENSUS_REL = Path("docs/program/cat-reuse-census.md")
PACKAGE_PREFIX = "catalog/packages/"
ROW_START = re.compile(r"^\| (\d+) \| `([^`]+\.ken\.md)` \|")
PACKAGE_TABLE_HEADER = (
    "| # | Package | Prelude redundancy | Sibling reimplementation | "
    "Arrangement and one-line evidence |"
)
PACKAGE_TABLE_SEPARATOR = "|---:|---|---|---|---|"
ITEM_WITH_RISK = re.compile(r"^(.+?) \[(low|higher)\]$")
ARRANGEMENT = re.compile(r"`(TD|BU)`")
ARRANGEMENT_WITNESS = re.compile(
    r"^\| `([^`]+\.ken\.md)` \| `([^`]+)` \| `([^`]+)` \| `(TD|BU)` \|$"
)
TARGET_ATOM = r"[A-Za-z][A-Za-z0-9]*\.[A-Za-z0-9_:/-]+"
TARGET_EXPRESSION = re.compile(
    rf"^({TARGET_ATOM})(?: specialized by ({TARGET_ATOM}))?$"
)
GROUP_ITEM = re.compile(r"^\s+- `([^`]+\.ken\.md#[^`]+)`$")
PUBLIC_STATUS = re.compile(
    r"^`?\[(ambient|all-public|mixed|absent|all-private)\]`?\s+"
)
PUBLIC_DETAIL = re.compile(
    r"^`\[(ambient|all-public|mixed|absent|all-private)\]` "
    r"public=([^;]+); private=([^;]+); absent=([^;]+); ambient=([^;]+)$"
)
STANDALONE_STATUS = re.compile(r"^`?\[(installed|ok|higher)\]`?\s+")


@dataclass(frozen=True)
class ProviderSpec:
    path: str
    source_kind: str


PROVIDERS = {
    "P": ProviderSpec("crates/ken-elaborator/src/prelude.rs", "rust"),
    "A": ProviderSpec(
        "catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md", "ken"
    ),
    "T": ProviderSpec("catalog/packages/Core/Logic/Transport.ken.md", "ken"),
    "LC": ProviderSpec(
        "catalog/packages/Core/Classes/LawfulClasses.ken.md", "ken"
    ),
    "N": ProviderSpec(
        "catalog/packages/Core/Classes/LawfulClasses.ken.md", "ken"
    ),
    "O": ProviderSpec("catalog/packages/Data/Numeric/Nat/Order.ken.md", "ken"),
    "D": ProviderSpec("catalog/packages/Data/Collections/Derived.ken.md", "ken"),
    "LF": ProviderSpec(
        "catalog/packages/Core/Classes/LawfulFunctors.ken.md", "ken"
    ),
    "BK": ProviderSpec("catalog/packages/Data/Binary/BytesKeys.ken.md", "ken"),
    "SC": ProviderSpec("catalog/packages/Data/Sums/Combinators.ken.md", "ken"),
    "C": ProviderSpec("catalog/packages/Capability/Parsing/Cursor.ken.md", "ken"),
    "Cmp": ProviderSpec("catalog/packages/Core/Logic/Compare.ken.md", "ken"),
    "OR": ProviderSpec("catalog/packages/Core/Logic/OrdResult.ken.md", "ken"),
}

# These targets are deliberately absent at the evidence ref. Their absence is
# part of the recorded prerequisite, not permission to accept arbitrary names.
EXPECTED_MUTATIONS = (
    ("missing-row", "PACKAGE_CLOSURE"),
    ("duplicate-package", "PACKAGE_CLOSURE"),
    ("missing-risk-tag", "ITEM_RISK"),
    ("wrong-rollup", "ROLLUP_SIBLING"),
    ("nonexistent-target", "TARGET_NOT_FOUND"),
    ("missing-public-depth", "PROVIDER_PUBLIC_DEPTH"),
    ("missing-standalone-depth", "PROVIDER_STANDALONE_DEPTH"),
    ("malformed-provider", "PROVIDER_SHAPE"),
    ("duplicate-provider", "PROVIDER_DUPLICATE"),
    ("higher-in-low-group", "LOW_GROUP_HIGHER"),
    ("evidence-ref-drift", "EVIDENCE_REF"),
    ("fabricated-provider-identity", "PROVIDER_IDENTITY"),
    ("nonexistent-local", "LOCAL_NOT_FOUND"),
    ("malformed-composite", "COMPOSITE_COMPONENTS"),
    ("omitted-composite", "COMPOSITE_COMPONENTS"),
    ("wrong-composite-parent", "COMPOSITE_PARENTAGE"),
    ("incomplete-absence-set", "ABSENCE_SET"),
    ("arrangement-flip", "ARRANGEMENT_MISMATCH"),
    ("unknown-risk-tag", "ITEM_RISK"),
    ("malformed-package-row", "PACKAGE_ROW_SHAPE"),
    ("duplicate-evidence-output", "EVIDENCE_REF_OCCURRENCE"),
    ("duplicate-population-output", "REPORTED_OUTPUT_OCCURRENCE"),
    ("ignored-target-suffix", "ITEM_TARGET_SYNTAX"),
    ("altered-public-detail", "PROVIDER_PUBLIC_DETAIL"),
)

EXPECTED_ABSENT_TARGETS = {
    "LC.bool_or::eq_true_of_or",
    "N.Ord-Nat",
    "N.leq_nat",
    "N.leq_nat::antisym",
    "N.leq_nat::refl",
    "N.leq_nat::trans",
    "N.total_leq_nat",
}


class CensusError(Exception):
    """A census invariant failed through a named discriminator arm."""

    def __init__(self, code: str, detail: str):
        super().__init__(f"{code}: {detail}")
        self.code = code
        self.detail = detail


@dataclass(frozen=True)
class ProviderRecord:
    code: str
    identity: str
    public_depth: str
    standalone_depth: str


@dataclass(frozen=True)
class SourceInventory:
    names: frozenset[str]
    public_names: frozenset[str]
    declaration_order: tuple[str, ...]
    data_constructors: tuple[tuple[str, tuple[str, ...]], ...]

    def constructors_for(self, parent: str) -> tuple[str, ...] | None:
        return dict(self.data_constructors).get(parent)


@dataclass(frozen=True)
class Item:
    package: str
    local: str
    risk: str
    targets: tuple[str, ...]

    @property
    def key(self) -> str:
        return f"{self.package}#{self.local}"


@dataclass(frozen=True)
class Report:
    evidence_ref: str
    population: int
    population_digest: str
    prelude_packages: int
    prelude_items: int
    prelude_low: int
    prelude_higher: int
    sibling_packages: int
    sibling_items: int
    sibling_low: int
    sibling_higher: int
    top_down: int
    bottom_up: int
    provider_codes: int
    referenced_codes: int
    qualified_targets: int
    present_targets: int
    absent_targets: int
    low_groups: int
    grouped_low_items: int
    local_items: int


def repository_root() -> Path:
    return Path(__file__).resolve().parent.parent


@functools.lru_cache(maxsize=None)
def git_show(root: Path, evidence_ref: str, path: str) -> str:
    result = subprocess.run(
        ["git", "-C", str(root), "show", f"{evidence_ref}:{path}"],
        check=True,
        text=True,
        capture_output=True,
    )
    return result.stdout


def package_population(root: Path, evidence_ref: str) -> list[str]:
    result = subprocess.run(
        [
            "git",
            "-C",
            str(root),
            "ls-tree",
            "-r",
            "--name-only",
            evidence_ref,
            "--",
            "catalog/packages",
        ],
        check=True,
        text=True,
        capture_output=True,
    )
    return sorted(
        path.removeprefix(PACKAGE_PREFIX)
        for path in result.stdout.splitlines()
        if path.endswith(".ken.md")
    )


def population_digest(population: list[str]) -> str:
    full_paths = "".join(f"{PACKAGE_PREFIX}{path}\n" for path in population)
    return hashlib.sha256(full_paths.encode()).hexdigest()


def extracted_ken(text: str) -> str:
    lines: list[str] = []
    inside = False
    for line in text.splitlines():
        if not inside and line.strip() == "```ken":
            inside = True
        elif inside and line.strip() == "```":
            inside = False
        elif inside:
            lines.append(line)
    return "\n".join(lines)


def ken_inventory(text: str) -> SourceInventory:
    code = extracted_ken(text)
    names: set[str] = set()
    public: set[str] = set()
    ordered: list[tuple[int, str]] = []
    direct = re.compile(
        r"^\s*(pub\s+)?(fn|const|theorem|lemma|data|class|view|proc)\s+"
        r"([A-Za-z_][A-Za-z0-9_]*)",
        re.MULTILINE,
    )
    direct_matches = list(direct.finditer(code))
    for match in direct_matches:
        name = match.group(3)
        names.add(name)
        ordered.append((match.start(), name))
        if match.group(1):
            public.add(name)

    attached = re.compile(
        r"^\s*(pub\s+)?proof\s+([A-Za-z_][A-Za-z0-9_]*)\s+for\s+"
        r"([A-Za-z_][A-Za-z0-9_]*)",
        re.MULTILINE,
    )
    for match in attached.finditer(code):
        name = f"{match.group(3)}::{match.group(2)}"
        names.add(name)
        ordered.append((match.start(), name))
        if match.group(1):
            public.add(name)

    simple_instance = re.compile(
        r"^\s*instance\s+([A-Za-z_][A-Za-z0-9_]*)\s+"
        r"([A-Z][A-Za-z0-9_]*)\s*\{",
        re.MULTILINE,
    )
    for match in simple_instance.finditer(code):
        name = f"{match.group(1)}-{match.group(2)}"
        names.add(name)
        ordered.append((match.start(), name))

    all_declaration_starts = sorted(position for position, _ in ordered)
    data_constructors: list[tuple[str, tuple[str, ...]]] = []
    for match in direct_matches:
        if match.group(2) != "data":
            continue
        parent = match.group(3)
        next_starts = [start for start in all_declaration_starts if start > match.start()]
        end = next_starts[0] if next_starts else len(code)
        region = code[match.start():end]
        constructors: list[str] = []
        header_rhs = region.split("\n", 1)[0].partition("=")[2]
        for part in header_rhs.split("|"):
            ctor_match = re.match(r"\s*([A-Z][A-Za-z0-9_]*)\b", part)
            if ctor_match:
                constructors.append(ctor_match.group(1))
        for line in region.splitlines()[1:]:
            ctor_match = re.match(r"\s*\|?\s*([A-Z][A-Za-z0-9_]*)\b", line)
            if ctor_match:
                constructor = ctor_match.group(1)
                if constructor not in constructors:
                    constructors.append(constructor)
        for constructor in constructors:
            names.add(constructor)
            if match.group(1):
                public.add(constructor)
        data_constructors.append((parent, tuple(constructors)))

    exports = re.compile(r"^\s*export\s+(.+)$", re.MULTILINE)
    for match in exports.finditer(code):
        for name in match.group(1).split(","):
            exported = name.strip()
            if exported in names:
                public.add(exported)
    declaration_order = tuple(name for _, name in sorted(ordered))
    return SourceInventory(
        frozenset(names),
        frozenset(public),
        declaration_order,
        tuple(data_constructors),
    )


def rust_inventory(text: str) -> SourceInventory:
    ordered = [
        match.group(1)
        for match in re.finditer(
            r'"(?:pub\s+)?(?:fn|const|data|class)\s+'
            r"([A-Za-z_][A-Za-z0-9_]*)",
            text,
        )
    ]
    names = frozenset(ordered)
    return SourceInventory(names, names, tuple(ordered), ())


def source_inventories(
    root: Path, evidence_ref: str
) -> dict[str, SourceInventory]:
    by_path: dict[tuple[str, str], SourceInventory] = {}
    result: dict[str, SourceInventory] = {}
    for code, spec in PROVIDERS.items():
        key = (spec.path, spec.source_kind)
        if key not in by_path:
            source = git_show(root, evidence_ref, spec.path)
            by_path[key] = (
                ken_inventory(source)
                if spec.source_kind == "ken"
                else rust_inventory(source)
            )
        result[code] = by_path[key]
    return result


def exact_output_block(text: str) -> str:
    blocks = re.findall(
        r"Exact output on this candidate:\n\n```text\n(.*?)\n```",
        text,
        re.DOTALL,
    )
    if len(blocks) != 1:
        raise CensusError(
            "REPORTED_OUTPUT_SECTION",
            f"exact-output blocks={len(blocks)}, expected=1",
        )
    return blocks[0]


def unique_match(
    pattern: re.Pattern[str], text: str, code: str, field: str
) -> re.Match[str]:
    matches = list(pattern.finditer(text))
    if len(matches) != 1:
        raise CensusError(code, f"{field} occurrences={len(matches)}, expected=1")
    return matches[0]


def unique_output_match(
    pattern: re.Pattern[str], output: str, prefix: str, field: str
) -> re.Match[str]:
    lines = [line for line in output.splitlines() if line.startswith(prefix)]
    if len(lines) != 1:
        raise CensusError(
            "REPORTED_OUTPUT_OCCURRENCE",
            f"{field} occurrences={len(lines)}, expected=1",
        )
    match = pattern.fullmatch(lines[0])
    if match is None:
        raise CensusError(
            "REPORTED_OUTPUT_SHAPE", f"{field} is malformed: {lines[0]}"
        )
    return match


def parse_evidence_ref(text: str) -> str:
    intro = unique_match(
        re.compile(r"exact source base\n`([0-9a-f]{40})`"),
        text,
        "EVIDENCE_REF_OCCURRENCE",
        "source-base claim",
    )
    claimed = intro.group(1)
    output_block = exact_output_block(text)
    evidence_lines = [
        line for line in output_block.splitlines() if line.startswith("evidence_ref=")
    ]
    if len(evidence_lines) != 1:
        raise CensusError(
            "EVIDENCE_REF_OCCURRENCE",
            f"reported evidence_ref occurrences={len(evidence_lines)}, expected=1",
        )
    output = re.fullmatch(r"evidence_ref=([0-9a-f]{40})", evidence_lines[0])
    if output is None:
        raise CensusError(
            "EVIDENCE_REF", f"reported evidence_ref is malformed: {evidence_lines[0]}"
        )
    if output.group(1) != claimed:
        raise CensusError(
            "EVIDENCE_REF",
            f"source claim {claimed} and reported output disagree",
        )
    command = unique_match(
        re.compile(
            r"git diff --exit-code \\\n  ([0-9a-f]{40}) HEAD -- catalog/packages"
        ),
        text,
        "EVIDENCE_REF_OCCURRENCE",
        "reproduction operand",
    )
    if command.group(1) != claimed:
        raise CensusError(
            "EVIDENCE_REF",
            f"source claim {claimed} and reproduction operand disagree",
        )
    if claimed != EXPECTED_EVIDENCE_REF:
        raise CensusError(
            "EVIDENCE_REF",
            f"claimed {claimed}, authorized {EXPECTED_EVIDENCE_REF}",
        )
    return claimed


def ledger_section(text: str) -> str:
    parts = text.split("## 2. Canonical provider and prerequisite ledger", 1)
    if len(parts) != 2:
        raise CensusError("PROVIDER_SECTION", "provider ledger section is absent")
    return parts[1].split("## 3. Complete per-package census", 1)[0]


def expected_provider_identity(spec: ProviderSpec) -> str:
    if spec.source_kind == "rust":
        return f"compiler-prelude:{spec.path}"
    return (
        spec.path.removeprefix(PACKAGE_PREFIX)
        .removesuffix(".ken.md")
        .replace("/", ".")
    )


def parse_provider_records(text: str) -> dict[str, ProviderRecord]:
    records: dict[str, ProviderRecord] = {}
    for line in ledger_section(text).splitlines():
        if not line.startswith("|"):
            continue
        if line.startswith("| Code ") or line.startswith("|---"):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) != 4:
            raise CensusError("PROVIDER_SHAPE", f"provider row has {len(cells)} cells: {line}")
        code_match = re.fullmatch(r"`([^`]+)`", cells[0])
        if code_match is None:
            raise CensusError("PROVIDER_SHAPE", f"provider code is malformed: {line}")
        code = code_match.group(1)
        if code in records:
            raise CensusError("PROVIDER_DUPLICATE", f"duplicate provider row: {code}")
        identity_match = re.fullmatch(r"`([^`]+)`", cells[1])
        if identity_match is None:
            raise CensusError(
                "PROVIDER_IDENTITY", f"{code} identity is not one exact code span"
            )
        if not cells[2] or PUBLIC_STATUS.match(cells[2]) is None:
            raise CensusError(
                "PROVIDER_PUBLIC_DEPTH", f"{code} public/export depth is missing"
            )
        if PUBLIC_DETAIL.fullmatch(cells[2]) is None:
            raise CensusError(
                "PROVIDER_PUBLIC_DETAIL",
                f"{code} public/export detail is not the exact source manifest",
            )
        if not cells[3] or STANDALONE_STATUS.match(cells[3]) is None:
            raise CensusError(
                "PROVIDER_STANDALONE_DEPTH",
                f"{code} standalone/ownership depth is missing",
            )
        records[code] = ProviderRecord(code, cells[1], cells[2], cells[3])

    missing = sorted(set(PROVIDERS) - set(records))
    unexpected = sorted(set(records) - set(PROVIDERS))
    if missing or unexpected:
        raise CensusError(
            "PROVIDER_SET", f"missing={missing} unexpected={unexpected}"
        )
    for code, spec in PROVIDERS.items():
        identity = re.fullmatch(r"`([^`]+)`", records[code].identity).group(1)
        expected = expected_provider_identity(spec)
        if identity != expected:
            raise CensusError(
                "PROVIDER_IDENTITY",
                f"{code} declares {identity}, exact provider is {expected}",
            )
    return records


def parse_table_rows(text: str) -> list[tuple[int, str, str, str, str]]:
    sections = text.split("## 3. Complete per-package census")
    if len(sections) != 2:
        raise CensusError(
            "PACKAGE_TABLE_SECTION",
            f"package census headings={len(sections) - 1}, expected=1",
        )
    bounded = sections[1].split("### 3.1 Arrangement source witnesses")
    if len(bounded) != 2:
        raise CensusError(
            "PACKAGE_TABLE_SECTION",
            f"arrangement boundaries={len(bounded) - 1}, expected=1",
        )
    table_section = bounded[0]
    if table_section.count(PACKAGE_TABLE_HEADER) != 1:
        raise CensusError("PACKAGE_TABLE_SHAPE", "package table header is not unique")
    after_header = table_section.split(PACKAGE_TABLE_HEADER, 1)[1]
    if after_header.count(PACKAGE_TABLE_SEPARATOR) != 1:
        raise CensusError(
            "PACKAGE_TABLE_SHAPE", "package table separator is not unique"
        )
    body = after_header.split(PACKAGE_TABLE_SEPARATOR, 1)[1]

    rows: list[tuple[int, str, str, str, str]] = []
    for line in body.splitlines():
        if not line.strip():
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if (
            not line.startswith("|")
            or not line.endswith("|")
            or len(cells) != 5
            or re.fullmatch(r"\d+", cells[0]) is None
        ):
            raise CensusError("PACKAGE_ROW_SHAPE", f"malformed package row: {line}")
        package_match = re.fullmatch(r"`([^`]+\.ken\.md)`", cells[1])
        if package_match is None:
            raise CensusError("PACKAGE_ROW_SHAPE", f"malformed package row: {line}")
        rows.append(
            (int(cells[0]), package_match.group(1), cells[2], cells[3], cells[4])
        )
    return rows


def parse_arrangement_witnesses(
    text: str,
) -> dict[str, tuple[str, str, str]]:
    parts = text.split("### 3.1 Arrangement source witnesses", 1)
    if len(parts) != 2:
        raise CensusError("ARRANGEMENT_SECTION", "source-witness section is absent")
    section = parts[1].split("## 4. Rollup", 1)[0]
    witnesses: dict[str, tuple[str, str, str]] = {}
    for line in section.splitlines():
        match = ARRANGEMENT_WITNESS.fullmatch(line)
        if match is None:
            continue
        package, first, headline, tag = match.groups()
        if package in witnesses:
            raise CensusError(
                "ARRANGEMENT_DUPLICATE", f"duplicate witness for {package}"
            )
        witnesses[package] = (first, headline, tag)
    return witnesses


def parse_item(raw_item: str, package: str) -> Item:
    raw = raw_item.strip().strip("`")
    risk_match = ITEM_WITH_RISK.fullmatch(raw)
    if risk_match is None or "[" in risk_match.group(1) or "]" in risk_match.group(1):
        raise CensusError("ITEM_RISK", f"{package} item has invalid risk syntax: {raw}")
    without_tag, risk = risk_match.groups()
    if without_tag.count("→") != 1:
        raise CensusError("ITEM_ARROW", f"{package} item needs one arrow: {raw}")
    local, target_text = (part.strip() for part in without_tag.split("→", 1))
    target_match = TARGET_EXPRESSION.fullmatch(target_text)
    if not local or target_match is None:
        raise CensusError(
            "ITEM_TARGET_SYNTAX",
            f"{package} target is not a complete target expression: {target_text}",
        )
    targets = tuple(target for target in target_match.groups() if target is not None)
    return Item(package, local, risk, targets)


def parse_item_cell(cell: str, package: str) -> list[Item]:
    if cell == "—":
        return []
    return [parse_item(item, package) for item in cell.split(";")]


def target_components(target: str) -> tuple[str, tuple[str, ...]]:
    code, local = target.split(".", 1)
    return code, tuple(local.split("/"))


def validate_composite(
    identity: str,
    names: tuple[str, ...],
    inventory: SourceInventory,
) -> None:
    if len(names) == 1:
        return
    if len(set(names)) != len(names):
        raise CensusError(
            "COMPOSITE_COMPONENTS", f"{identity} repeats a component: {names}"
        )
    parent = names[0]
    constructors = inventory.constructors_for(parent)
    if constructors is None:
        raise CensusError(
            "COMPOSITE_PARENTAGE", f"{identity} parent {parent} is not source data"
        )
    for component in names[1:]:
        actual_parents = [
            data_name
            for data_name, data_constructors in inventory.data_constructors
            if component in data_constructors
        ]
        if actual_parents != [parent]:
            raise CensusError(
                "COMPOSITE_PARENTAGE",
                f"{identity} component {component} parents={actual_parents}",
            )
    expected = (parent, *constructors)
    if names != expected:
        raise CensusError(
            "COMPOSITE_COMPONENTS", f"{identity} components={names} expected={expected}"
        )


def validate_targets(
    targets: set[str], inventories: dict[str, SourceInventory]
) -> tuple[int, int]:
    present = absent = 0
    for target in sorted(targets):
        code, names = target_components(target)
        if code not in inventories:
            raise CensusError("TARGET_PROVIDER", f"unknown provider code in {target}")
        source_names = inventories[code].names
        validate_composite(target, names, inventories[code])
        if target in EXPECTED_ABSENT_TARGETS:
            unexpectedly_present = sorted(name for name in names if name in source_names)
            if unexpectedly_present:
                raise CensusError(
                    "TARGET_EXPECTED_ABSENT",
                    f"{target} unexpectedly exists as {unexpectedly_present}",
                )
            absent += 1
            continue
        missing = sorted(name for name in names if name not in source_names)
        if missing:
            raise CensusError(
                "TARGET_NOT_FOUND", f"{target} missing exact-source names {missing}"
            )
        present += 1
    return present, absent


def validate_absence_set(targets: set[str]) -> None:
    referenced = targets & EXPECTED_ABSENT_TARGETS
    if referenced != EXPECTED_ABSENT_TARGETS:
        missing = sorted(EXPECTED_ABSENT_TARGETS - referenced)
        unexpected = sorted(referenced - EXPECTED_ABSENT_TARGETS)
        raise CensusError(
            "ABSENCE_SET", f"missing={missing} unexpected={unexpected}"
        )


def catalog_source_inventories(
    root: Path, evidence_ref: str, population: list[str]
) -> dict[str, SourceInventory]:
    return {
        package: ken_inventory(
            git_show(root, evidence_ref, f"{PACKAGE_PREFIX}{package}")
        )
        for package in population
    }


def validate_local_items(
    items: list[Item], inventories: dict[str, SourceInventory]
) -> None:
    for item in items:
        names = tuple(item.local.split("/"))
        inventory = inventories[item.package]
        validate_composite(item.key, names, inventory)
        missing = sorted(name for name in names if name not in inventory.names)
        if missing:
            raise CensusError(
                "LOCAL_NOT_FOUND",
                f"{item.key} missing exact-source names {missing}",
            )


def validate_arrangements(
    text: str,
    rows: list[tuple[int, str, str, str, str]],
    inventories: dict[str, SourceInventory],
) -> tuple[int, int]:
    witnesses = parse_arrangement_witnesses(text)
    packages = {row[1] for row in rows}
    if set(witnesses) != packages:
        raise CensusError(
            "ARRANGEMENT_CLOSURE",
            f"missing={sorted(packages - set(witnesses))} "
            f"unexpected={sorted(set(witnesses) - packages)}",
        )
    top_down = bottom_up = 0
    for _, package, _, _, arrangement in rows:
        source_order = inventories[package].declaration_order
        if not source_order:
            raise CensusError(
                "ARRANGEMENT_SOURCE", f"{package} has no checked declarations"
            )
        first, headline, witnessed_tag = witnesses[package]
        if first != source_order[0]:
            raise CensusError(
                "ARRANGEMENT_SOURCE",
                f"{package} first={first}, exact source first={source_order[0]}",
            )
        if headline not in source_order:
            raise CensusError(
                "ARRANGEMENT_SOURCE",
                f"{package} headline {headline} is not a source declaration",
            )
        derived = "TD" if headline == first else "BU"
        if derived == "BU" and source_order.index(headline) <= 0:
            raise CensusError(
                "ARRANGEMENT_SOURCE",
                f"{package} headline {headline} does not follow {first}",
            )
        tags = ARRANGEMENT.findall(arrangement)
        if len(tags) != 1:
            raise CensusError("ARRANGEMENT_TAG", f"{package} tags={tags}")
        if witnessed_tag != derived or tags[0] != derived:
            raise CensusError(
                "ARRANGEMENT_MISMATCH",
                f"{package} table={tags[0]} witness={witnessed_tag} source={derived}",
            )
        top_down += derived == "TD"
        bottom_up += derived == "BU"
    return top_down, bottom_up


def manifest_names(names: set[str]) -> str:
    return ",".join(sorted(names)) if names else "-"


def validate_public_depths(
    records: dict[str, ProviderRecord],
    targets: set[str],
    inventories: dict[str, SourceInventory],
) -> None:
    for code in sorted(PROVIDERS):
        local_names: set[str] = set()
        public: set[str] = set()
        private: set[str] = set()
        absent: set[str] = set()
        ambient: set[str] = set()
        for target in targets:
            target_code, names = target_components(target)
            if target_code != code:
                continue
            suffix = target.split(".", 1)[1]
            if code == "P":
                ambient.add(suffix)
                continue
            if target in EXPECTED_ABSENT_TARGETS:
                absent.add(suffix)
                continue
            local_names.update(names)
            visibility = [
                name in inventories[code].public_names for name in names
            ]
            if all(visibility):
                public.add(suffix)
            elif not any(visibility):
                private.add(suffix)
            else:
                raise CensusError(
                    "PROVIDER_PUBLIC_DETAIL",
                    f"{code}.{suffix} has mixed component visibility",
                )
        if code == "P":
            measured = "ambient"
        elif not local_names:
            measured = "absent"
        else:
            public_count = sum(
                name in inventories[code].public_names for name in local_names
            )
            if public_count == len(local_names):
                measured = "all-public"
            elif public_count == 0:
                measured = "all-private"
            else:
                measured = "mixed"
        declared = PUBLIC_STATUS.match(records[code].public_depth).group(1)
        if declared != measured:
            raise CensusError(
                "PROVIDER_PUBLIC_MISMATCH",
                f"{code} declares {declared}, exact source measures {measured}",
            )
        expected_detail = (
            f"`[{measured}]` public={manifest_names(public)}; "
            f"private={manifest_names(private)}; "
            f"absent={manifest_names(absent)}; "
            f"ambient={manifest_names(ambient)}"
        )
        if records[code].public_depth != expected_detail:
            raise CensusError(
                "PROVIDER_PUBLIC_DETAIL",
                f"{code} detail={records[code].public_depth!r}, "
                f"exact source={expected_detail!r}",
            )


def parse_low_groups(text: str, risk_by_key: dict[str, str]) -> tuple[int, int]:
    parts = text.split("### 4.4 Proposed low-risk work groups", 1)
    if len(parts) != 2:
        raise CensusError("LOW_GROUP_SECTION", "low-risk group section is absent")
    section = parts[1].split("### 4.5", 1)[0]
    groups: dict[int, list[str]] = {}
    current: int | None = None
    for line in section.splitlines():
        heading = re.match(r"^(\d+)\. \*\*[^*]+\*\*$", line)
        if heading:
            current = int(heading.group(1))
            if current in groups:
                raise CensusError("LOW_GROUP_DUPLICATE", f"duplicate group {current}")
            groups[current] = []
            continue
        item = GROUP_ITEM.match(line)
        if item:
            if current is None:
                raise CensusError("LOW_GROUP_SHAPE", "item appears before a group")
            groups[current].append(item.group(1))
            continue
        if current is not None and line.strip():
            raise CensusError(
                "LOW_GROUP_SHAPE", f"unparsed content in group {current}: {line}"
            )
    if not groups or sorted(groups) != list(range(1, len(groups) + 1)):
        raise CensusError("LOW_GROUP_SHAPE", f"non-contiguous groups: {sorted(groups)}")
    empty = sorted(group for group, items in groups.items() if not items)
    if empty:
        raise CensusError("LOW_GROUP_SHAPE", f"groups without exact items: {empty}")

    grouped = [item for items in groups.values() for item in items]
    duplicates = sorted(key for key, count in collections.Counter(grouped).items() if count > 1)
    if duplicates:
        raise CensusError("LOW_GROUP_DUPLICATE", f"duplicate grouped items: {duplicates}")
    unknown = sorted(set(grouped) - set(risk_by_key))
    if unknown:
        raise CensusError("LOW_GROUP_UNKNOWN", f"unknown grouped items: {unknown}")
    higher = sorted(key for key in grouped if risk_by_key[key] == "higher")
    if higher:
        raise CensusError("LOW_GROUP_HIGHER", f"higher-risk grouped items: {higher}")
    expected_low = {key for key, risk in risk_by_key.items() if risk == "low"}
    missing = sorted(expected_low - set(grouped))
    unexpected = sorted(set(grouped) - expected_low)
    if missing or unexpected:
        raise CensusError(
            "LOW_GROUP_CLOSURE", f"missing={missing} unexpected={unexpected}"
        )
    return len(groups), len(grouped)


def require_rollup(text: str, report: Report) -> None:
    patterns = {
        "prelude": re.compile(
            r"^\| Prelude redundancy \| (\d+) \| (\d+), all `low` \|$",
            re.MULTILINE,
        ),
        "sibling": re.compile(
            r"^\| Sibling reimplementation \| (\d+) \| "
            r"(\d+) total: (\d+) `low`, (\d+) `higher` \|$",
            re.MULTILINE,
        ),
        "arrangement": re.compile(
            r"^\| Checked arrangement \| (\d+) \| "
            r"(\d+) `TD`, (\d+) `BU` \|$",
            re.MULTILINE,
        ),
    }
    matches = {name: pattern.search(text) for name, pattern in patterns.items()}
    missing = [name for name, match in matches.items() if match is None]
    if missing:
        raise CensusError("ROLLUP_SHAPE", f"absent/malformed rows: {missing}")
    measured = {
        "prelude": (report.prelude_packages, report.prelude_items),
        "sibling": (
            report.sibling_packages,
            report.sibling_items,
            report.sibling_low,
            report.sibling_higher,
        ),
        "arrangement": (report.population, report.top_down, report.bottom_up),
    }
    if report.prelude_low != report.prelude_items or report.prelude_higher:
        raise CensusError(
            "ROLLUP_PRELUDE_RISK",
            f"low={report.prelude_low} higher={report.prelude_higher}",
        )
    for name, match in matches.items():
        reported = tuple(int(value) for value in match.groups())
        if reported != measured[name]:
            raise CensusError(
                f"ROLLUP_{name.upper()}", f"reported={reported} measured={measured[name]}"
            )


def require_reported_output(text: str, report: Report) -> None:
    output = exact_output_block(text)
    patterns = {
        "population": (
            "population=",
            re.compile(
                r"population=(\d+) rows=(\d+) unique=(\d+) sha256=([0-9a-f]{64})"
            ),
        ),
        "prelude": (
            "prelude=",
            re.compile(r"prelude=packages:(\d+) items:(\d+) low:(\d+) higher:(\d+)"),
        ),
        "sibling": (
            "sibling=",
            re.compile(r"sibling=packages:(\d+) items:(\d+) low:(\d+) higher:(\d+)"),
        ),
        "arrangement": (
            "arrangement=",
            re.compile(r"arrangement=TD:(\d+) BU:(\d+)"),
        ),
        "providers": (
            "providers=",
            re.compile(r"providers=ledger:(\d+) referenced:(\d+)"),
        ),
        "targets": (
            "targets=",
            re.compile(r"targets=qualified:(\d+) present:(\d+) absent:(\d+)"),
        ),
        "groups": (
            "low_groups=",
            re.compile(r"low_groups=groups:(\d+) items:(\d+)"),
        ),
        "locals": ("locals=", re.compile(r"locals=items:(\d+)")),
    }
    matches = {
        name: unique_output_match(pattern, output, prefix, f"reported {name}")
        for name, (prefix, pattern) in patterns.items()
    }
    measured = {
        "population": (
            report.population,
            report.population,
            report.population,
            report.population_digest,
        ),
        "prelude": (
            report.prelude_packages,
            report.prelude_items,
            report.prelude_low,
            report.prelude_higher,
        ),
        "sibling": (
            report.sibling_packages,
            report.sibling_items,
            report.sibling_low,
            report.sibling_higher,
        ),
        "arrangement": (report.top_down, report.bottom_up),
        "providers": (report.provider_codes, report.referenced_codes),
        "targets": (
            report.qualified_targets,
            report.present_targets,
            report.absent_targets,
        ),
        "groups": (report.low_groups, report.grouped_low_items),
        "locals": (report.local_items,),
    }
    for name, match in matches.items():
        reported = tuple(
            int(value) if value.isdigit() else value for value in match.groups()
        )
        if reported != measured[name]:
            raise CensusError(
                f"REPORTED_{name.upper()}", f"reported={reported} measured={measured[name]}"
            )

    coverage = output.splitlines().count("CAT-REUSE-CENSUS coverage PASS")
    if coverage != 1:
        raise CensusError(
            "REPORTED_OUTPUT_OCCURRENCE",
            f"coverage header occurrences={coverage}, expected=1",
        )
    mutation_lines = [line for line in output.splitlines() if line.startswith("mutation=")]
    expected_mutation_lines = [
        f"mutation={name} expected={code} PASS"
        for name, code in EXPECTED_MUTATIONS
    ]
    if mutation_lines != expected_mutation_lines:
        raise CensusError(
            "REPORTED_MUTATIONS",
            "reported mutation rows do not equal the "
            f"{len(expected_mutation_lines)} expected rows",
        )
    self_test_line = f"self_test=PASS mutations={len(EXPECTED_MUTATIONS)}/{len(EXPECTED_MUTATIONS)}"
    if output.splitlines().count(self_test_line) != 1:
        raise CensusError(
            "REPORTED_OUTPUT_OCCURRENCE",
            f"self-test summary must occur exactly once as {self_test_line!r}",
        )
    expected_output = [
        "CAT-REUSE-CENSUS coverage PASS",
        f"evidence_ref={report.evidence_ref}",
        f"population={report.population} rows={report.population} "
        f"unique={report.population} sha256={report.population_digest}",
        "prelude="
        f"packages:{report.prelude_packages} items:{report.prelude_items} "
        f"low:{report.prelude_low} higher:{report.prelude_higher}",
        "sibling="
        f"packages:{report.sibling_packages} items:{report.sibling_items} "
        f"low:{report.sibling_low} higher:{report.sibling_higher}",
        f"arrangement=TD:{report.top_down} BU:{report.bottom_up}",
        f"providers=ledger:{report.provider_codes} referenced:{report.referenced_codes}",
        f"targets=qualified:{report.qualified_targets} "
        f"present:{report.present_targets} absent:{report.absent_targets}",
        f"low_groups=groups:{report.low_groups} items:{report.grouped_low_items}",
        f"locals=items:{report.local_items}",
        *expected_mutation_lines,
        self_test_line,
    ]
    if output.splitlines() != expected_output:
        raise CensusError(
            "REPORTED_OUTPUT_EXACT",
            "exact-output block contains missing, extra, reordered, or stale fields",
        )


def validate(text: str, root: Path) -> Report:
    evidence_ref = parse_evidence_ref(text)
    population = package_population(root, evidence_ref)
    records = parse_provider_records(text)
    rows = parse_table_rows(text)
    if not rows:
        raise CensusError("PACKAGE_ROWS", "per-package table is empty")
    numbers = [row[0] for row in rows]
    if numbers != list(range(1, len(rows) + 1)):
        raise CensusError("ROW_NUMBERS", f"non-contiguous row numbers: {numbers}")
    packages = [row[1] for row in rows]
    duplicates = sorted(
        package
        for package, count in collections.Counter(packages).items()
        if count != 1
    )
    missing = sorted(set(population) - set(packages))
    unexpected = sorted(set(packages) - set(population))
    if duplicates or missing or unexpected or len(packages) != len(population):
        raise CensusError(
            "PACKAGE_CLOSURE",
            f"rows={len(packages)} population={len(population)} "
            f"duplicates={duplicates} missing={missing} unexpected={unexpected}",
        )

    prelude_packages = sibling_packages = 0
    prelude_items: list[Item] = []
    sibling_items: list[Item] = []
    for _, package, prelude, sibling, arrangement in rows:
        if not prelude or not sibling or not arrangement:
            raise CensusError("PACKAGE_AXIS", f"{package} has an empty axis")
        prelude_packages += prelude != "—"
        sibling_packages += sibling != "—"
        prelude_items.extend(parse_item_cell(prelude, package))
        sibling_items.extend(parse_item_cell(sibling, package))

    all_items = prelude_items + sibling_items
    local_inventories = catalog_source_inventories(root, evidence_ref, population)
    validate_local_items(all_items, local_inventories)
    keys = [item.key for item in all_items]
    duplicate_keys = sorted(
        key for key, count in collections.Counter(keys).items() if count > 1
    )
    if duplicate_keys:
        raise CensusError("ITEM_DUPLICATE", f"duplicate local items: {duplicate_keys}")
    targets = {target for item in all_items for target in item.targets}
    referenced_codes = {target.split(".", 1)[0] for target in targets}
    unknown_codes = sorted(referenced_codes - set(records))
    if unknown_codes:
        raise CensusError("TARGET_PROVIDER", f"unknown provider codes: {unknown_codes}")
    validate_absence_set(targets)
    inventories = source_inventories(root, evidence_ref)
    present_targets, absent_targets = validate_targets(targets, inventories)
    validate_public_depths(records, targets, inventories)
    top_down, bottom_up = validate_arrangements(text, rows, local_inventories)

    digest = population_digest(population)
    digest_match = unique_match(
        re.compile(r"population list has SHA-256\n`([0-9a-f]{64})`"),
        text,
        "POPULATION_DIGEST_OCCURRENCE",
        "population digest claim",
    )
    if digest_match.group(1) != digest:
        raise CensusError(
            "POPULATION_DIGEST",
            f"reported={digest_match.group(1)} measured={digest}",
        )
    intro = unique_match(
        re.compile(r"base contains \*\*(\d+)\*\* `\*\.ken\.md` packages"),
        text,
        "INTRO_COUNT_OCCURRENCE",
        "introductory package count",
    )
    if int(intro.group(1)) != len(population):
        raise CensusError("INTRO_COUNT", "introductory package count is stale")

    risk_by_key = {item.key: item.risk for item in all_items}
    low_groups, grouped_low_items = parse_low_groups(text, risk_by_key)
    report = Report(
        evidence_ref=evidence_ref,
        population=len(population),
        population_digest=digest,
        prelude_packages=prelude_packages,
        prelude_items=len(prelude_items),
        prelude_low=sum(item.risk == "low" for item in prelude_items),
        prelude_higher=sum(item.risk == "higher" for item in prelude_items),
        sibling_packages=sibling_packages,
        sibling_items=len(sibling_items),
        sibling_low=sum(item.risk == "low" for item in sibling_items),
        sibling_higher=sum(item.risk == "higher" for item in sibling_items),
        top_down=top_down,
        bottom_up=bottom_up,
        provider_codes=len(records),
        referenced_codes=len(referenced_codes),
        qualified_targets=len(targets),
        present_targets=present_targets,
        absent_targets=absent_targets,
        low_groups=low_groups,
        grouped_low_items=grouped_low_items,
        local_items=len(all_items),
    )
    require_rollup(text, report)
    require_reported_output(text, report)
    return report


def replace_provider_cell(text: str, code: str, cell: int, value: str) -> str:
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if line.startswith(f"| `{code}` |"):
            cells = [part.strip() for part in line.strip().strip("|").split("|")]
            cells[cell] = value
            lines[index] = "| " + " | ".join(cells) + " |"
            return "\n".join(lines)
    raise CensusError("SELF_TEST_SETUP", f"provider row not found: {code}")


def expect_failure(
    name: str,
    text: str,
    root: Path,
    expected_code: str,
) -> tuple[str, str]:
    try:
        validate(text, root)
    except CensusError as error:
        if error.code != expected_code:
            raise CensusError(
                "SELF_TEST_WRONG_ARM",
                f"{name} expected {expected_code}, got {error.code}: {error.detail}",
            ) from error
        return name, error.code
    raise CensusError("SELF_TEST_FALSE_GREEN", f"{name} unexpectedly passed")


def replace_exact_once(text: str, old: str, new: str, name: str) -> str:
    occurrences = text.count(old)
    if occurrences != 1:
        raise CensusError(
            "SELF_TEST_SETUP",
            f"{name} anchor occurrences={occurrences}, expected=1",
        )
    return text.replace(old, new, 1)


def self_test(text: str, root: Path) -> list[tuple[str, str]]:
    lines = text.splitlines()
    row_indexes = [index for index, line in enumerate(lines) if ROW_START.match(line)]
    if len(row_indexes) < 2:
        raise CensusError("SELF_TEST_SETUP", "need at least two package rows")
    mutations: list[tuple[str, str, str]] = []

    missing_row = lines.copy()
    del missing_row[row_indexes[-1]]
    mutations.append(("missing-row", "\n".join(missing_row), "PACKAGE_CLOSURE"))

    duplicate_package = lines.copy()
    first = ROW_START.match(lines[row_indexes[0]]).group(2)
    second = ROW_START.match(lines[row_indexes[1]]).group(2)
    duplicate_package[row_indexes[1]] = duplicate_package[row_indexes[1]].replace(
        f"`{second}`", f"`{first}`", 1
    )
    mutations.append(
        ("duplicate-package", "\n".join(duplicate_package), "PACKAGE_CLOSURE")
    )

    missing_risk = lines.copy()
    risk_row = next(index for index in row_indexes if "[higher]" in lines[index])
    missing_risk[risk_row] = missing_risk[risk_row].replace("[higher]", "", 1)
    mutations.append(("missing-risk-tag", "\n".join(missing_risk), "ITEM_RISK"))

    wrong_rollup = text.replace(
        "58 total: 31 `low`, 27 `higher`",
        "57 total: 31 `low`, 27 `higher`",
        1,
    )
    mutations.append(("wrong-rollup", wrong_rollup, "ROLLUP_SIBLING"))

    nonexistent = text.replace("`add→A.add [low]`", "`add→A.not_a_declaration [low]`", 1)
    mutations.append(("nonexistent-target", nonexistent, "TARGET_NOT_FOUND"))

    mutations.append(
        (
            "missing-public-depth",
            replace_provider_cell(text, "A", 2, ""),
            "PROVIDER_PUBLIC_DEPTH",
        )
    )
    mutations.append(
        (
            "missing-standalone-depth",
            replace_provider_cell(text, "N", 3, ""),
            "PROVIDER_STANDALONE_DEPTH",
        )
    )

    malformed = re.sub(
        r"^\| `N` \|.*$", "| N | malformed |", text, count=1, flags=re.MULTILINE
    )
    mutations.append(("malformed-provider", malformed, "PROVIDER_SHAPE"))

    n_row = next(line for line in lines if line.startswith("| `N` |"))
    duplicate_provider = text.replace(n_row, f"{n_row}\n{n_row}", 1)
    mutations.append(
        ("duplicate-provider", duplicate_provider, "PROVIDER_DUPLICATE")
    )

    higher_group = text.replace(
        "1. **Prelude and functional-floor reuse**",
        "1. **Prelude and functional-floor reuse**\n"
        "   - `Algorithm/Numeric/Gcd.ken.md#subst`",
        1,
    )
    mutations.append(("higher-in-low-group", higher_group, "LOW_GROUP_HIGHER"))

    evidence_drift = text.replace(
        "exact source base\n`ed5b4063f434cc7a2311143367928ee98f64fd86`",
        "exact source base\n`a7603b31d9f3f52568145b704a6f1a9bdde0ef06`",
        1,
    )
    mutations.append(("evidence-ref-drift", evidence_drift, "EVIDENCE_REF"))

    bogus_provider = text.replace(
        "| `A` | `Data.Numeric.Nat.Arithmetic` |",
        "| `A` | `Data.Numeric.Nat.ArithmeticBogus` |",
        1,
    )
    mutations.append(
        ("fabricated-provider-identity", bogus_provider, "PROVIDER_IDENTITY")
    )

    nonexistent_local = text.replace(
        "pretty_nat_add", "not_a_local_declaration"
    )
    mutations.append(
        ("nonexistent-local", nonexistent_local, "LOCAL_NOT_FOUND")
    )

    malformed_composite = text.replace(
        "OR.OrdResult/Lt/Eq/Gt", "OR.OrdResult/Lt/Eq/Eq", 1
    )
    mutations.append(
        ("malformed-composite", malformed_composite, "COMPOSITE_COMPONENTS")
    )

    omitted_composite = text.replace(
        "OR.OrdResult/Lt/Eq/Gt", "OR.OrdResult/Lt/Eq", 1
    )
    mutations.append(
        ("omitted-composite", omitted_composite, "COMPOSITE_COMPONENTS")
    )

    wrong_parent = text.replace(
        "OR.OrdResult/Lt/Eq/Gt", "OR.OrdResult/Lt/Eq/True", 1
    )
    mutations.append(
        ("wrong-composite-parent", wrong_parent, "COMPOSITE_PARENTAGE")
    )

    incomplete_absence = text.replace(
        "N.leq_nat::refl [higher]", "N.leq_nat::trans [higher]", 1
    )
    mutations.append(("incomplete-absence-set", incomplete_absence, "ABSENCE_SET"))

    arrangement_flip = text.replace(
        "| 7 | `Capability/Console/Text.ken.md` | — | — | `TD` —",
        "| 7 | `Capability/Console/Text.ken.md` | — | — | `BU` —",
        1,
    ).replace("36 `TD`, 12 `BU`", "35 `TD`, 13 `BU`", 1).replace(
        "arrangement=TD:36 BU:12", "arrangement=TD:35 BU:13", 1
    )
    mutations.append(
        ("arrangement-flip", arrangement_flip, "ARRANGEMENT_MISMATCH")
    )

    unknown_risk = text.replace(
        "`add→A.add [low]`", "`add→A.add [low] [medium]`", 1
    )
    mutations.append(("unknown-risk-tag", unknown_risk, "ITEM_RISK"))

    malformed_row = lines.copy()
    malformed_row.insert(
        row_indexes[0] + 1,
        lines[row_indexes[0]].replace("| 1 |", "| x |", 1),
    )
    mutations.append(
        ("malformed-package-row", "\n".join(malformed_row), "PACKAGE_ROW_SHAPE")
    )

    duplicate_evidence = replace_exact_once(
        text,
        f"evidence_ref={EXPECTED_EVIDENCE_REF}",
        f"evidence_ref={EXPECTED_EVIDENCE_REF}\n"
        "evidence_ref=a7603b31d9f3f52568145b704a6f1a9bdde0ef06",
        "duplicate-evidence-output",
    )
    mutations.append(
        (
            "duplicate-evidence-output",
            duplicate_evidence,
            "EVIDENCE_REF_OCCURRENCE",
        )
    )

    population_line = (
        "population=48 rows=48 unique=48 "
        "sha256=50ca4604db917bf7e2758e075c269c064ac181f871c246bea73d0d4b7e197333"
    )
    duplicate_population = replace_exact_once(
        text,
        population_line,
        population_line
        + "\npopulation=47 rows=47 unique=47 "
        "sha256=0000000000000000000000000000000000000000000000000000000000000000",
        "duplicate-population-output",
    )
    mutations.append(
        (
            "duplicate-population-output",
            duplicate_population,
            "REPORTED_OUTPUT_OCCURRENCE",
        )
    )

    ignored_suffix = replace_exact_once(
        text,
        "`add→A.add [low]`",
        "`add→A.add or not_a_declaration [low]`",
        "ignored-target-suffix",
    )
    mutations.append(
        ("ignored-target-suffix", ignored_suffix, "ITEM_TARGET_SYNTAX")
    )

    altered_public = replace_provider_cell(
        text,
        "A",
        2,
        "`[all-public]` public=-; private=add,mul; absent=-; ambient=-",
    )
    mutations.append(
        ("altered-public-detail", altered_public, "PROVIDER_PUBLIC_DETAIL")
    )

    specifications = tuple((name, expected) for name, _, expected in mutations)
    if specifications != EXPECTED_MUTATIONS:
        raise CensusError(
            "SELF_TEST_SETUP",
            f"mutation specifications differ: {specifications}",
        )
    return [
        expect_failure(name, mutated, root, expected)
        for name, mutated, expected in mutations
    ]


def print_report(report: Report, mutations: list[tuple[str, str]]) -> None:
    print("CAT-REUSE-CENSUS coverage PASS")
    print(f"evidence_ref={report.evidence_ref}")
    print(
        f"population={report.population} rows={report.population} "
        f"unique={report.population} sha256={report.population_digest}"
    )
    print(
        "prelude="
        f"packages:{report.prelude_packages} items:{report.prelude_items} "
        f"low:{report.prelude_low} higher:{report.prelude_higher}"
    )
    print(
        "sibling="
        f"packages:{report.sibling_packages} items:{report.sibling_items} "
        f"low:{report.sibling_low} higher:{report.sibling_higher}"
    )
    print(f"arrangement=TD:{report.top_down} BU:{report.bottom_up}")
    print(
        f"providers=ledger:{report.provider_codes} referenced:{report.referenced_codes}"
    )
    print(
        f"targets=qualified:{report.qualified_targets} "
        f"present:{report.present_targets} absent:{report.absent_targets}"
    )
    print(f"low_groups=groups:{report.low_groups} items:{report.grouped_low_items}")
    print(f"locals=items:{report.local_items}")
    if mutations:
        for name, code in mutations:
            print(f"mutation={name} expected={code} PASS")
        print(f"self_test=PASS mutations={len(mutations)}/{len(mutations)}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="run named, single-fault mutations and assert their exact error arms",
    )
    args = parser.parse_args()
    root = repository_root()
    text = (root / CENSUS_REL).read_text()
    report = validate(text, root)
    mutations = self_test(text, root) if args.self_test else []
    print_report(report, mutations)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (CensusError, subprocess.CalledProcessError) as error:
        print(f"CAT-REUSE-CENSUS coverage FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
