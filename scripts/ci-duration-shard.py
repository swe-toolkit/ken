#!/usr/bin/env python3
"""Assign a filtered live nextest inventory to deterministic duration bins."""
import heapq
import json
import os
import re
import sys
import warnings


N = 9
# Run 36265192923's largest workspace test was 549.960s; use 600s for
# unseen tests until measured rather than the much smaller suite median.
DEFAULT_DURATION_SECONDS = 600.0
WORKSPACE_TIMING_ROW = re.compile(
    r"^(?P<shard>\d+)\s+PASS\s+\[\s*(?P<seconds>[0-9.]+)s\s*\]"
    r"\s+\(\s*\d+/\d+\)\s+(?P<test_id>.+)$"
)
EXCLUDED_BINARIES = {
    "rt_parity_native",
    "px8f_buffer_native",
    "px8f_write_partition",
}


def tests(value):
    suites = value.get("rust-suites")
    if not isinstance(suites, dict) or not suites:
        raise SystemExit("nextest listing has no non-empty rust-suites map")
    count = value.get("test-count")
    if not isinstance(count, int) or isinstance(count, bool) or count <= 0:
        raise SystemExit("nextest listing has no positive test-count")
    seen = set()
    discovered = 0
    for suite in suites.values():
        if not isinstance(suite, dict):
            raise SystemExit("nextest listing contains a malformed rust suite")
        binary_id = suite.get("binary-id")
        binary_name = suite.get("binary-name")
        testcases = suite.get("testcases")
        if not isinstance(binary_id, str) or not binary_id:
            raise SystemExit("nextest rust suite has no non-empty binary-id")
        if not isinstance(binary_name, str) or not binary_name:
            raise SystemExit("nextest rust suite has no non-empty binary-name")
        if not isinstance(testcases, dict):
            raise SystemExit("nextest rust suite has no testcase map")
        for name, metadata in testcases.items():
            if not isinstance(name, str) or not name or not isinstance(metadata, dict):
                raise SystemExit("nextest rust suite has an invalid testcase record")
            filter_match = metadata.get("filter-match")
            status = filter_match.get("status") if isinstance(filter_match, dict) else None
            if status not in {"matches", "mismatch"}:
                raise SystemExit("nextest testcase has invalid filter-match status")
            identity = (binary_id, name)
            if identity in seen:
                raise SystemExit(f"duplicate canonical identity: {binary_id} {name}")
            seen.add(identity)
            discovered += 1
            if status == "matches" and binary_name not in EXCLUDED_BINARIES:
                yield identity
    if discovered != count:
        raise SystemExit("nextest test-count differs from discovered testcases")


def filtered_projection(inventory, output):
    value = json.load(open(inventory))
    list(tests(value))
    for suite in value["rust-suites"].values():
        if suite["binary-name"] in EXCLUDED_BINARIES:
            for metadata in suite["testcases"].values():
                metadata["filter-match"]["status"] = "mismatch"
    with open(output, "w") as file:
        json.dump(value, file)


def empty_projection(inventory, output):
    value = json.load(open(inventory))
    # Reuse the selector's schema validation before projecting zero matches.
    list(tests(value))
    for suite in value["rust-suites"].values():
        for metadata in suite["testcases"].values():
            metadata["filter-match"]["status"] = "mismatch"
    with open(output, "w") as file:
        json.dump(value, file)


def selected_projection(inventory, assignment_path, shard, output):
    value = json.load(open(inventory))
    assignment = json.load(open(assignment_path))
    planned = {tuple(item) for item in assignment["bins"][shard - 1]["tests"]}
    for suite in value["rust-suites"].values():
        for name, metadata in suite["testcases"].items():
            metadata["filter-match"]["status"] = "matches" if (suite["binary-id"], name) in planned else "mismatch"
    with open(output, "w") as file:
        json.dump(value, file)


def read_durations(path):
    if path.endswith(".tsv"):
        durations = {}
        with open(path, encoding="utf-8") as source:
            rows = source.readlines()
        for line_number, line in enumerate(rows, 1):
            match = WORKSPACE_TIMING_ROW.fullmatch(line.rstrip("\n"))
            if not match:
                raise SystemExit(f"{path}:{line_number}: malformed workspace timing row")
            test_id = match.group("test_id")
            if test_id in durations:
                raise SystemExit(f"{path}:{line_number}: duplicate timing row {test_id}")
            seconds = float(match.group("seconds"))
            if seconds <= 0:
                raise SystemExit(f"{path}:{line_number}: duration must be positive")
            durations[test_id] = seconds
        return durations
    evidence = json.load(open(path))
    records = evidence.get("records")
    if not isinstance(records, list) or not records:
        raise SystemExit("duration evidence has no records")
    durations = {}
    for row in records:
        test_id = row.get("test_id")
        seconds = row.get("seconds")
        if not isinstance(test_id, str) or not test_id:
            raise SystemExit("duration evidence has an invalid test_id")
        if not isinstance(seconds, (int, float)) or isinstance(seconds, bool) or seconds <= 0:
            raise SystemExit(f"duration evidence has an invalid duration for {test_id}")
        if test_id in durations:
            raise SystemExit(f"duration evidence has duplicate row {test_id}")
        durations[test_id] = float(seconds)
    return durations


def validate_plan(assignment_path, shard, selected_path):
    assignment = json.load(open(assignment_path))
    bins = assignment.get("bins")
    if not isinstance(bins, list) or not 1 <= shard <= len(bins):
        raise SystemExit("assignment has no requested shard")
    planned = {tuple(identity) for identity in bins[shard - 1].get("tests", [])}
    realized = set(tests(json.load(open(selected_path))))
    if realized != planned:
        raise SystemExit("planned shard identities differ from realized selection")


def main():
    if len(sys.argv) == 4 and sys.argv[1] == "project-filtered":
        filtered_projection(sys.argv[2], sys.argv[3])
        return
    if len(sys.argv) == 4 and sys.argv[1] == "project-empty":
        empty_projection(sys.argv[2], sys.argv[3])
        return
    if len(sys.argv) == 6 and sys.argv[1] == "project-selected":
        selected_projection(sys.argv[2], sys.argv[3], int(sys.argv[4]), sys.argv[5])
        return
    if len(sys.argv) == 5 and sys.argv[1] == "validate-plan":
        validate_plan(sys.argv[2], int(sys.argv[3]), sys.argv[4])
        return
    inventory = json.load(open(sys.argv[1]))
    durations = read_durations(sys.argv[2])
    bins = [(0.0, index, []) for index in range(N)]
    heapq.heapify(bins)
    live = sorted((f"{binary_id} {name}", binary_id, name) for binary_id, name in tests(inventory))
    if not live:
        raise SystemExit("filtered live inventory selected zero testcases")
    live_ids = {row[0] for row in live}
    stale = sorted(
        test_id for test_id in set(durations) - live_ids
        if test_id.split(" ", 1)[0].rpartition("::")[2] not in EXCLUDED_BINARIES
    )
    if stale:
        warnings.warn(
            "dropping timing rows absent from current inventory: " + ", ".join(stale),
            RuntimeWarning,
            stacklevel=2,
        )
    missing = sorted(live_ids - set(durations))
    if missing:
        warnings.warn(
            f"using {DEFAULT_DURATION_SECONDS}s default for {len(missing)} unmeasured workspace tests",
            RuntimeWarning,
            stacklevel=2,
        )
    for rendered, binary_id, name in sorted(
        live, key=lambda row: (-durations.get(row[0], DEFAULT_DURATION_SECONDS), row[0])
    ):
        total, index, selected = heapq.heappop(bins)
        selected.append((binary_id, name))
        heapq.heappush(
            bins,
            (total + durations.get(rendered, DEFAULT_DURATION_SECONDS), index, selected),
        )
    result = []
    for total, index, selected in sorted(bins, key=lambda x: x[1]):
        terms = [f"(binary_id(={binary}) & test(={name}))" for binary, name in selected]
        result.append({
            "bin": index + 1,
            "seconds": round(total, 3),
            "tests": selected,
            "filter": " | ".join(terms),
        })
    output = sys.argv[3] if len(sys.argv) > 3 else None
    if output:
        os.makedirs(output, exist_ok=True)
        limit = os.sysconf("SC_ARG_MAX") // 4
        for item in result:
            expression = item["filter"]
            if len(expression.encode()) > limit:
                raise SystemExit(f"bin {item['bin']} filter exceeds argv guard {limit}")
            with open(os.path.join(output, f"bin-{item['bin']}.expr"), "w") as file:
                file.write(expression)
    assignment = {"bins": result}
    if output:
        with open(os.path.join(output, "assignments.json"), "w") as file:
            json.dump(assignment, file)
    print(json.dumps(assignment, indent=2))


if __name__ == "__main__":
    main()
