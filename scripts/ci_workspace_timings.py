#!/usr/bin/env python3
"""Normalize and validate terminal workspace nextest timing artifacts."""
from __future__ import annotations

import json
import math
import re
import sys
from pathlib import Path
from typing import Any


SHARD_COUNT = 7
TERMINAL_LINE = re.compile(
    r"^\s*(?P<status>PASS|FAIL|SLOW)\s+\[\s*"
    r"(?P<seconds>(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+))s\s*\]"
    r"\s+\(\s*(?P<index>[0-9]+)/(?P<count>[0-9]+)\s*\)"
    r"\s+(?P<test_id>\S.+?)\s*$"
)
SKIP_LINE = re.compile(
    r"^\s*SKIP \[         \] \(─+\) (?P<test_id>\S.+?)\s*$"
)
SLOW_EVENT_LINE = re.compile(
    r"^\s*SLOW\s+\[\s*>\s*"
    r"(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)s\s*\]"
    r"\s+\(───\)\s+\S.+?\s*$"
)
STATUS_LINE = re.compile(r"^\s*(?P<status>[A-Z][A-Z0-9_-]*)\s+\[")
NONTERMINAL_STATUSES = {"RETRY", "RUN", "START"}
ARTIFACT_KEYS = {"run_id", "shard", "shard_count", "unit", "records", "fallbacks"}
RECORD_KEYS = {"test_id", "seconds", "result"}
FALLBACK_KEYS = {"test_id", "seconds", "method"}
FALLBACK_METHODS = {"binary-median", "global-median"}


class TimingArtifactError(ValueError):
    """An invalid or incomplete workspace timing artifact."""


def _positive_int(value: Any, name: str) -> int:
    if not isinstance(value, int) or isinstance(value, bool) or value <= 0:
        raise TimingArtifactError(f"{name} must be a positive integer")
    return value


def listed_identities(listing: Any) -> tuple[set[str], set[str]]:
    if not isinstance(listing, dict):
        raise TimingArtifactError("selected nextest listing must be an object")
    count = listing.get("test-count")
    if not isinstance(count, int) or isinstance(count, bool) or count <= 0:
        raise TimingArtifactError("selected listing test-count must be a positive integer")
    suites = listing.get("rust-suites")
    if not isinstance(suites, dict) or not suites:
        raise TimingArtifactError("selected listing rust-suites must be a non-empty map")

    discovered: set[tuple[str, str]] = set()
    all_ids: set[str] = set()
    selected: set[str] = set()
    for suite in suites.values():
        if not isinstance(suite, dict):
            raise TimingArtifactError("selected listing contains a malformed suite")
        binary_id = suite.get("binary-id")
        testcases = suite.get("testcases")
        if (
            not isinstance(binary_id, str)
            or not binary_id.strip()
            or binary_id != binary_id.strip()
            or any(character.isspace() for character in binary_id)
        ):
            raise TimingArtifactError("selected listing suite has no canonical binary-id")
        if not isinstance(testcases, dict):
            raise TimingArtifactError("selected listing suite has no testcase map")
        for name, metadata in testcases.items():
            if not isinstance(name, str) or not name.strip() or name != name.strip():
                raise TimingArtifactError("selected listing has an invalid testcase name")
            if not isinstance(metadata, dict):
                raise TimingArtifactError("selected listing testcase metadata is malformed")
            filter_match = metadata.get("filter-match")
            status = filter_match.get("status") if isinstance(filter_match, dict) else None
            if status not in {"matches", "mismatch"}:
                raise TimingArtifactError("selected listing has an invalid filter-match status")
            identity = (binary_id, name)
            if identity in discovered:
                raise TimingArtifactError(
                    f"selected listing has duplicate identity {binary_id} {name}"
                )
            discovered.add(identity)
            rendered = f"{binary_id} {name}"
            all_ids.add(rendered)
            if status == "matches":
                selected.add(rendered)
    if len(discovered) != count:
        raise TimingArtifactError(
            f"selected listing test-count {count} differs from {len(discovered)} discovered"
        )
    return all_ids, selected


def selected_identities(listing: Any) -> set[str]:
    return listed_identities(listing)[1]


def parse_terminal_log(
    log_text: str, expected_ids: set[str], known_ids: set[str] | None = None
) -> list[dict[str, Any]]:
    """Keep selected terminal rows; validate and ignore unselected SKIP rows."""
    known_ids = expected_ids if known_ids is None else known_ids
    terminal: dict[str, dict[str, Any]] = {}
    for line_number, line in enumerate(log_text.splitlines(), 1):
        prefix = STATUS_LINE.match(line)
        if prefix is None:
            continue
        status = prefix.group("status")
        if status == "SKIP":
            match = SKIP_LINE.fullmatch(line)
            if match is None:
                raise TimingArtifactError(
                    f"nextest output line {line_number}: malformed skipped result"
                )
            test_id = match.group("test_id")
            if test_id not in known_ids:
                raise TimingArtifactError(
                    f"nextest output line {line_number}: unknown skipped identity {test_id}"
                )
            if test_id in expected_ids:
                raise TimingArtifactError(
                    f"nextest output line {line_number}: selected identity was skipped {test_id}"
                )
            continue
        match = None
        if status in {"PASS", "FAIL", "SLOW"}:
            match = TERMINAL_LINE.fullmatch(line)
        if status == "SLOW" and match is None:
            # A threshold notification has dashes; an ordinal marks a terminal result.
            if SLOW_EVENT_LINE.fullmatch(line):
                continue
            raise TimingArtifactError(
                f"nextest output line {line_number}: malformed slow result"
            )
        if status in NONTERMINAL_STATUSES:
            continue
        if status not in {"PASS", "FAIL", "SLOW"}:
            raise TimingArtifactError(
                f"nextest output line {line_number}: unsupported status {status}"
            )
        if match is None:
            raise TimingArtifactError(
                f"nextest output line {line_number}: malformed terminal result"
            )
        index = int(match.group("index"))
        count = int(match.group("count"))
        if count <= 0 or not 1 <= index <= count:
            raise TimingArtifactError(
                f"nextest output line {line_number}: invalid terminal ordinal"
            )
        test_id = match.group("test_id").strip()
        binary_id, separator, test_name = test_id.partition(" ")
        if (
            not separator
            or not binary_id
            or binary_id != binary_id.strip()
            or any(character.isspace() for character in binary_id)
            or not test_name.strip()
            or test_name != test_name.strip()
        ):
            raise TimingArtifactError(
                f"nextest output line {line_number}: malformed canonical test_id"
            )
        if test_id not in expected_ids:
            raise TimingArtifactError(
                f"nextest output line {line_number}: unselected terminal identity {test_id}"
            )
        seconds = float(match.group("seconds"))
        if not math.isfinite(seconds) or seconds < 0:
            raise TimingArtifactError(
                f"nextest output line {line_number}: invalid terminal duration"
            )
        if test_id in terminal:
            raise TimingArtifactError(
                f"duplicate terminal identity in nextest output: {test_id}"
            )
        terminal[test_id] = {
            "test_id": test_id,
            "seconds": seconds,
            "result": "PASS" if status == "SLOW" else status,
        }
    missing = sorted(expected_ids - set(terminal))
    if missing:
        raise TimingArtifactError(
            "missing terminal results for selected identities: " + ", ".join(missing)
        )
    return [terminal[test_id] for test_id in sorted(terminal)]


def validate_artifact(
    value: Any,
    *,
    expected_run_id: int | None = None,
    expected_shard: int | None = None,
    expected_ids: set[str] | None = None,
) -> dict[str, Any]:
    if not isinstance(value, dict) or set(value) != ARTIFACT_KEYS:
        raise TimingArtifactError("timing artifact has invalid top-level fields")
    run_id = _positive_int(value.get("run_id"), "run_id")
    shard = _positive_int(value.get("shard"), "shard")
    shard_count = _positive_int(value.get("shard_count"), "shard_count")
    if shard_count != SHARD_COUNT or shard > shard_count:
        raise TimingArtifactError("timing artifact has invalid shard provenance")
    if value.get("unit") != "seconds":
        raise TimingArtifactError('timing artifact unit must be "seconds"')
    if expected_run_id is not None and run_id != expected_run_id:
        raise TimingArtifactError("timing artifact run_id does not match this run")
    if expected_shard is not None and shard != expected_shard:
        raise TimingArtifactError("timing artifact shard does not match its artifact")
    records = value.get("records")
    if not isinstance(records, list):
        raise TimingArtifactError("timing artifact records must be a list")
    seen: set[str] = set()
    for index, record in enumerate(records, 1):
        if not isinstance(record, dict) or set(record) != RECORD_KEYS:
            raise TimingArtifactError(f"timing record {index} has invalid fields")
        test_id = record.get("test_id")
        if not isinstance(test_id, str) or not test_id.strip():
            raise TimingArtifactError(f"timing record {index} has an invalid test_id")
        binary_id, separator, test_name = test_id.partition(" ")
        if (
            not separator
            or not binary_id
            or binary_id != binary_id.strip()
            or any(character.isspace() for character in binary_id)
            or not test_name.strip()
            or test_name != test_name.strip()
            or test_id != test_id.strip()
        ):
            raise TimingArtifactError(f"timing record {index} has a noncanonical test_id")
        if test_id in seen:
            raise TimingArtifactError(f"duplicate terminal identity: {test_id}")
        seen.add(test_id)
        seconds = record.get("seconds")
        if not isinstance(seconds, (int, float)) or isinstance(seconds, bool):
            raise TimingArtifactError(f"timing record {index} has invalid seconds")
        try:
            numeric_seconds = float(seconds)
        except (OverflowError, ValueError):
            raise TimingArtifactError(f"timing record {index} has invalid seconds") from None
        if not math.isfinite(numeric_seconds) or numeric_seconds < 0:
            raise TimingArtifactError(f"timing record {index} has invalid seconds")
        if record.get("result") not in {"PASS", "FAIL"}:
            raise TimingArtifactError(f"timing record {index} has invalid terminal result")
    fallbacks = value.get("fallbacks")
    if not isinstance(fallbacks, list):
        raise TimingArtifactError("timing artifact fallbacks must be a list")
    seen_fallbacks: set[str] = set()
    for index, fallback in enumerate(fallbacks, 1):
        if not isinstance(fallback, dict) or set(fallback) != FALLBACK_KEYS:
            raise TimingArtifactError(f"fallback record {index} has invalid fields")
        test_id = fallback.get("test_id")
        if not isinstance(test_id, str) or not test_id.strip():
            raise TimingArtifactError(f"fallback record {index} has invalid test_id")
        binary_id, separator, test_name = test_id.partition(" ")
        if (
            not separator
            or not binary_id
            or binary_id != binary_id.strip()
            or any(character.isspace() for character in binary_id)
            or not test_name.strip()
            or test_name != test_name.strip()
            or test_id != test_id.strip()
        ):
            raise TimingArtifactError(f"fallback record {index} has noncanonical test_id")
        if test_id in seen_fallbacks:
            raise TimingArtifactError(f"duplicate fallback identity: {test_id}")
        seen_fallbacks.add(test_id)
        seconds = fallback.get("seconds")
        if not isinstance(seconds, (int, float)) or isinstance(seconds, bool):
            raise TimingArtifactError(f"fallback record {index} has invalid seconds")
        try:
            numeric_seconds = float(seconds)
        except (OverflowError, ValueError):
            raise TimingArtifactError(f"fallback record {index} has invalid seconds") from None
        if not math.isfinite(numeric_seconds) or numeric_seconds < 0:
            raise TimingArtifactError(f"fallback record {index} has invalid seconds")
        if fallback.get("method") not in FALLBACK_METHODS:
            raise TimingArtifactError(f"fallback record {index} has invalid method")
    if expected_ids is not None and seen != expected_ids:
        missing = sorted(expected_ids - seen)
        extra = sorted(seen - expected_ids)
        details = []
        if missing:
            details.append("missing=" + ", ".join(missing))
        if extra:
            details.append("extra=" + ", ".join(extra))
        raise TimingArtifactError(
            "terminal identities differ from selected inventory: " + "; ".join(details)
        )
    return value


def emit_artifact(
    log_text: str,
    listing: Any,
    run_id: int,
    shard: int,
    plan: Any | None = None,
) -> dict[str, Any]:
    run_id = _positive_int(run_id, "run_id")
    shard = _positive_int(shard, "shard")
    if shard > SHARD_COUNT:
        raise TimingArtifactError("shard must be between 1 and 7")
    known_ids, expected = listed_identities(listing)
    fallback_records = []
    if plan is not None:
        if not isinstance(plan, dict) or set(plan) != {"bins", "fallbacks"}:
            raise TimingArtifactError("duration plan has invalid top-level fields")
        bins = plan.get("bins")
        if not isinstance(bins, list) or len(bins) != SHARD_COUNT:
            raise TimingArtifactError("duration plan must contain seven bins")
        all_planned: set[str] = set()
        planned_for_shard: set[str] = set()
        for index, bin_record in enumerate(bins, 1):
            if not isinstance(bin_record, dict) or not isinstance(bin_record.get("tests"), list):
                raise TimingArtifactError(f"duration plan bin {index} is malformed")
            bin_ids = set()
            for identity in bin_record["tests"]:
                if not isinstance(identity, (list, tuple)) or len(identity) != 2:
                    raise TimingArtifactError(f"duration plan bin {index} has an invalid identity")
                binary_id, test_name = identity
                if (
                    not isinstance(binary_id, str)
                    or not binary_id
                    or any(character.isspace() for character in binary_id)
                    or not isinstance(test_name, str)
                    or not test_name.strip()
                    or test_name != test_name.strip()
                ):
                    raise TimingArtifactError(f"duration plan bin {index} has a noncanonical identity")
                test_id = f"{binary_id} {test_name}"
                if test_id in all_planned:
                    raise TimingArtifactError(f"duration plan duplicates identity: {test_id}")
                all_planned.add(test_id)
                bin_ids.add(test_id)
            if index == shard:
                planned_for_shard = bin_ids
        if planned_for_shard != expected:
            raise TimingArtifactError("duration plan bin differs from selected inventory")
        plan_fallbacks = plan.get("fallbacks")
        if not isinstance(plan_fallbacks, list):
            raise TimingArtifactError("duration plan fallbacks must be a list")
        seen_fallbacks = set()
        fallback_records = []
        for index, fallback in enumerate(plan_fallbacks, 1):
            if not isinstance(fallback, dict) or set(fallback) != FALLBACK_KEYS:
                raise TimingArtifactError(f"duration plan fallback {index} is malformed")
            test_id = fallback.get("test_id")
            seconds = fallback.get("seconds")
            method = fallback.get("method")
            if not isinstance(test_id, str) or test_id not in all_planned:
                raise TimingArtifactError(f"duration plan fallback {index} has an unknown identity")
            if test_id in seen_fallbacks:
                raise TimingArtifactError(f"duration plan duplicates fallback identity: {test_id}")
            if not isinstance(seconds, (int, float)) or isinstance(seconds, bool):
                raise TimingArtifactError(f"duration plan fallback {index} has invalid seconds")
            try:
                numeric_seconds = float(seconds)
            except (OverflowError, ValueError):
                raise TimingArtifactError(
                    f"duration plan fallback {index} has invalid seconds"
                ) from None
            if not math.isfinite(numeric_seconds) or numeric_seconds < 0:
                raise TimingArtifactError(f"duration plan fallback {index} has invalid seconds")
            if method not in FALLBACK_METHODS:
                raise TimingArtifactError(f"duration plan fallback {index} has invalid method")
            seen_fallbacks.add(test_id)
            fallback_records.append(fallback)
    artifact = {
        "run_id": run_id,
        "shard": shard,
        "shard_count": SHARD_COUNT,
        "unit": "seconds",
        "records": parse_terminal_log(log_text, expected, known_ids),
        "fallbacks": fallback_records,
    }
    return validate_artifact(
        artifact,
        expected_run_id=run_id,
        expected_shard=shard,
        expected_ids=expected,
    )


def load_artifact(path: str | Path) -> dict[str, Any]:
    try:
        value = json.loads(Path(path).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise TimingArtifactError(f"{path}: invalid timing artifact JSON") from error
    return validate_artifact(value)


def main(argv: list[str] | None = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    if len(argv) != 7 or argv[0] != "emit":
        print(
            "usage: ci-workspace-timings.py emit LOG SELECTED PLAN RUN_ID SHARD OUTPUT",
            file=sys.stderr,
        )
        return 2
    _, log_path, selected_path, plan_path, run_raw, shard_raw, output_path = argv
    try:
        run_id = int(run_raw)
        shard = int(shard_raw)
        log_text = Path(log_path).read_text(encoding="utf-8")
        listing = json.loads(Path(selected_path).read_text(encoding="utf-8"))
        plan = json.loads(Path(plan_path).read_text(encoding="utf-8"))
        artifact = emit_artifact(log_text, listing, run_id, shard, plan)
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(json.dumps(artifact, indent=2) + "\n", encoding="utf-8")
    except (OSError, json.JSONDecodeError, ValueError) as error:
        print(f"workspace timing artifact failed: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
