#!/usr/bin/env python3
"""Build a deterministic duration-balanced rt_parity_native shard plan."""
from __future__ import annotations

import heapq
import json
import re
import sys
import warnings
from pathlib import Path

SHARD_COUNT = 8
SOURCE_SHARD_COUNT = 6
# The maximum per-test observations from runs 36260020054 and 36265192923
# are used to reduce sensitivity to the observed timing noise. Run 36260020054's
# median was 83.595s; use a round 90s estimate for unseen tests until measured.
DEFAULT_DURATION_SECONDS = 90.0
NEXTTEST_TIMING_ROW = re.compile(
    r"^(?P<shard>[1-6])\s+PASS\s+\[\s*(?P<seconds>[0-9.]+)s\s*\]"
    r"\s+\(\s*\d+/\d+\)\s+ken-cli::rt_parity_native\s+(?P<name>\S+)\s*$"
)
TSV_TIMING_ROW = re.compile(
    r"^(?P<shard>[1-6])\t(?P<seconds>[0-9.]+)\t"
    r"ken-cli::rt_parity_native\t(?P<name>\S+)\s*$"
)


def read_inventory(path: Path) -> tuple[str, list[str]]:
    value = json.loads(path.read_text(encoding="utf-8"))
    suites = value.get("rust-suites")
    count = value.get("test-count")
    if not isinstance(suites, dict) or not suites:
        raise ValueError("inventory has no non-empty rust-suites map")
    if not isinstance(count, int) or isinstance(count, bool) or count <= 0:
        raise ValueError("inventory has no positive test-count")
    identities: list[tuple[str, str]] = []
    discovered = 0
    for suite in suites.values():
        if not isinstance(suite, dict):
            raise ValueError("inventory contains a malformed rust suite")
        binary_id = suite.get("binary-id")
        binary_name = suite.get("binary-name")
        testcases = suite.get("testcases")
        if not isinstance(binary_id, str) or not binary_id:
            raise ValueError("inventory suite has no binary-id")
        if not isinstance(binary_name, str) or not binary_name:
            raise ValueError("inventory suite has no binary-name")
        if not isinstance(testcases, dict):
            raise ValueError("inventory suite has no testcase map")
        discovered += len(testcases)
        if binary_name != "rt_parity_native":
            continue
        for name in testcases:
            if not isinstance(name, str) or not name:
                raise ValueError("inventory has an invalid testcase name")
            identities.append((binary_id, name))
    if discovered != count:
        raise ValueError("inventory test-count differs from discovered testcases")
    if not identities:
        raise ValueError("inventory contains no rt_parity_native tests")
    binary_ids = {binary_id for binary_id, _ in identities}
    if len(binary_ids) != 1:
        raise ValueError("inventory has multiple rt_parity_native binary identities")
    names = [name for _, name in identities]
    if len(names) != len(set(names)):
        raise ValueError("inventory has duplicate rt_parity_native test names")
    return next(iter(binary_ids)), names


def read_timings(paths: list[Path]) -> dict[str, float]:
    observations: dict[str, list[float]] = {}
    for path in paths:
        timings: dict[str, float] = {}
        shard_counts = [0] * SOURCE_SHARD_COUNT
        for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            match = NEXTTEST_TIMING_ROW.fullmatch(line) or TSV_TIMING_ROW.fullmatch(line)
            if not match:
                raise ValueError(f"{path}:{line_number}: malformed parity timing row")
            name = match.group("name")
            shard_counts[int(match.group("shard")) - 1] += 1
            if name in timings:
                raise ValueError(f"{path}:{line_number}: duplicate test timing {name}")
            seconds = float(match.group("seconds"))
            if seconds <= 0:
                raise ValueError(f"{path}:{line_number}: duration must be positive")
            timings[name] = seconds
        if sorted(shard_counts) != [30, 31, 31, 31, 31, 31]:
            raise ValueError(f"{path}: unexpected source-shard counts: {shard_counts}")
        for name, seconds in timings.items():
            observations.setdefault(name, []).append(seconds)
    return {name: max(values) for name, values in observations.items()}


def make_plan(binary_id: str, names: list[str], timings: dict[str, float]) -> dict:
    inventory_names = set(names)
    stale = sorted(set(timings) - inventory_names)
    if stale:
        warnings.warn(
            "dropping timing rows absent from current inventory: " + ", ".join(stale),
            RuntimeWarning,
            stacklevel=2,
        )
    missing = sorted(inventory_names - set(timings))
    if missing:
        warnings.warn(
            f"using {DEFAULT_DURATION_SECONDS}s default for {len(missing)} unmeasured parity tests",
            RuntimeWarning,
            stacklevel=2,
        )
    durations = {
        name: timings.get(name, DEFAULT_DURATION_SECONDS) for name in names
    }
    bins: list[tuple[float, int, list[str]]] = [
        (0.0, index, []) for index in range(SHARD_COUNT)
    ]
    heapq.heapify(bins)
    for name in sorted(names, key=lambda item: (-durations[item], item)):
        total, index, assigned = heapq.heappop(bins)
        assigned.append(name)
        heapq.heappush(bins, (total + durations[name], index, assigned))
    result = []
    for total, index, assigned in sorted(bins, key=lambda item: item[1]):
        terms = [f"(binary_id(={binary_id}) & test(={name}))" for name in assigned]
        result.append({
            "shard": index + 1,
            "seconds": round(total, 3),
            "tests": [[binary_id, name] for name in sorted(assigned)],
            "filter": " | ".join(terms),
        })
    return {
        "timing_sources": ["36260020054", "36265192923"],
        "shard_count": SHARD_COUNT,
        "bins": result,
    }


def main() -> int:
    try:
        if len(sys.argv) < 4:
            raise ValueError(
                "usage: ci-rt-parity-shard.py INVENTORY TIMINGS.tsv [TIMINGS.tsv ...] OUTPUT.json"
            )
        inventory = Path(sys.argv[1])
        timing_files = [Path(argument) for argument in sys.argv[2:-1]]
        output = Path(sys.argv[-1])
        binary_id, names = read_inventory(inventory)
        timings = read_timings(timing_files)
        plan = make_plan(binary_id, names, timings)
        output.write_text(json.dumps(plan, indent=2) + "\n", encoding="utf-8")
        print(json.dumps(plan, indent=2))
    except (OSError, json.JSONDecodeError, ValueError) as error:
        print(f"rt_parity duration plan failed: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
