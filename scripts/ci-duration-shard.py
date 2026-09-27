#!/usr/bin/env python3
"""Assign a filtered live nextest inventory to deterministic duration bins."""
import heapq
import json
import math
import os
import re
import statistics
import sys
import warnings

import ci_workspace_timings


N = 7
# Use the latest complete full-CI per-test source for LPT. Missing tests use
# their binary's measured median, or the global measured median for unseen bins.
NEXTTEST_TIMING_ROW = re.compile(
    r"^(?P<shard>\d+)\s+PASS\s+\[\s*(?P<seconds>[0-9.]+)s\s*\]"
    r"\s+\(\s*\d+/\d+\)\s+(?P<test_id>.+)$"
)
WORKSPACE_TSV_TIMING_ROW = re.compile(
    r"^(?P<shard>\d+)\t(?P<seconds>[0-9.]+)\t"
    r"(?P<binary_id>\S+)\t(?P<name>\S+)\tPASS$"
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


def read_durations(paths):
    if isinstance(paths, (str, os.PathLike)):
        paths = [paths]
    paths = [os.fspath(path) for path in paths]
    if not paths:
        raise SystemExit("duration evidence has no input files")
    durations = {}
    run_test_shards = {}
    run_fallbacks = {}
    for path in paths:
        observed = {}
        if path.endswith(".tsv"):
            with open(path, encoding="utf-8") as source:
                rows = source.readlines()
            for line_number, line in enumerate(rows, 1):
                row = line.rstrip("\n")
                match = NEXTTEST_TIMING_ROW.fullmatch(row)
                tsv_match = WORKSPACE_TSV_TIMING_ROW.fullmatch(row)
                if not match and not tsv_match:
                    raise SystemExit(f"{path}:{line_number}: malformed workspace timing row")
                if match:
                    test_id = match.group("test_id")
                else:
                    test_id = f"{tsv_match.group('binary_id')} {tsv_match.group('name')}"
                if test_id in observed:
                    raise SystemExit(f"{path}:{line_number}: duplicate timing row {test_id}")
                seconds = float((match or tsv_match).group("seconds"))
                if not math.isfinite(seconds) or seconds <= 0:
                    raise SystemExit(f"{path}:{line_number}: duration must be positive and finite")
                observed[test_id] = seconds
        else:
            try:
                with open(path, encoding="utf-8") as source:
                    evidence = json.load(source)
                artifact = ci_workspace_timings.validate_artifact(evidence)
            except (OSError, json.JSONDecodeError, ci_workspace_timings.TimingArtifactError) as error:
                raise SystemExit(f"{path}: invalid duration artifact: {error}") from error
            prior_fallbacks = run_fallbacks.setdefault(artifact["run_id"], artifact["fallbacks"])
            if prior_fallbacks != artifact["fallbacks"]:
                raise SystemExit(
                    f"{path}: fallback identities differ within run {artifact['run_id']}"
                )
            records = artifact["records"]
            for row in records:
                test_id = row.get("test_id")
                seconds = row.get("seconds")
                if not isinstance(test_id, str) or not test_id:
                    raise SystemExit(f"{path}: duration evidence has an invalid test_id")
                if not isinstance(seconds, (int, float)) or isinstance(seconds, bool) or seconds < 0:
                    raise SystemExit(f"{path}: duration evidence has an invalid duration for {test_id}")
                provenance = (artifact["run_id"], test_id)
                if provenance in run_test_shards:
                    prior_shard = run_test_shards[provenance]
                    raise SystemExit(
                        f"{path}: duplicate terminal identity in run {artifact['run_id']} "
                        f"shards {prior_shard} and {artifact['shard']}: {test_id}"
                    )
                run_test_shards[provenance] = artifact["shard"]
                if test_id in observed:
                    raise SystemExit(f"{path}: duration evidence has duplicate row {test_id}")
                if row["result"] == "FAIL":
                    # Keep failed terminals validated and provenance-checked, but
                    # do not treat potentially early-aborted work as a full-test
                    # timing. The planner assigns this identity a median fallback.
                    continue
                observed[test_id] = float(seconds)
        for test_id, seconds in observed.items():
            durations[test_id] = max(durations.get(test_id, seconds), seconds)
    return durations


def resolve_durations(live, measured):
    """Fill missing live identities from per-binary then global medians."""
    by_binary = {}
    measured_live = {}
    for rendered, binary_id, _ in live:
        if rendered in measured:
            duration = measured[rendered]
            measured_live[rendered] = duration
            by_binary.setdefault(binary_id, []).append(duration)
    if not measured_live:
        raise SystemExit("cannot estimate unmeasured tests without any live timing rows")

    global_median = statistics.median(measured_live.values())
    binary_medians = {
        binary_id: statistics.median(values)
        for binary_id, values in by_binary.items()
    }
    resolved = dict(measured_live)
    fallbacks = []
    for rendered, binary_id, _ in live:
        if rendered in resolved:
            continue
        method = "binary-median" if binary_id in binary_medians else "global-median"
        seconds = binary_medians.get(binary_id, global_median)
        resolved[rendered] = seconds
        fallbacks.append({"test_id": rendered, "seconds": seconds, "method": method})
    return resolved, fallbacks


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
    arguments = sys.argv[1:]
    output = None
    if "--output-dir" in arguments:
        option = arguments.index("--output-dir")
        if option != len(arguments) - 2 or option < 2:
            raise SystemExit("--output-dir must follow at least one timing file")
        output = arguments[-1]
        arguments = arguments[:-2]
    if len(arguments) < 2:
        raise SystemExit(
            "usage: ci-duration-shard.py INVENTORY TIMING... [--output-dir DIR]"
        )
    inventory = json.load(open(arguments[0]))
    durations = read_durations(arguments[1:])
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
    planned_durations, fallbacks = resolve_durations(live, durations)
    if fallbacks:
        binary_fallbacks = sum(
            fallback["method"] == "binary-median" for fallback in fallbacks
        )
        global_fallbacks = len(fallbacks) - binary_fallbacks
        warnings.warn(
            f"using per-binary median for {binary_fallbacks} and global median "
            f"for {global_fallbacks} of {len(fallbacks)} unmeasured workspace tests",
            RuntimeWarning,
            stacklevel=2,
        )
    bins = [(0.0, index, []) for index in range(N)]
    heapq.heapify(bins)
    ordered = sorted(live, key=lambda row: (-planned_durations[row[0]], row[0]))
    for rendered, binary_id, name in ordered:
        total, index, selected = heapq.heappop(bins)
        selected.append((binary_id, name))
        heapq.heappush(
            bins,
            (total + planned_durations[rendered], index, selected),
        )

    result = []
    for total, index, selected in sorted(bins, key=lambda item: item[1]):
        terms = [f"(binary_id(={binary}) & test(={name}))" for binary, name in selected]
        result.append({
            "bin": index + 1,
            "seconds": round(total, 3),
            "tests": selected,
            "filter": " | ".join(terms),
        })
    if output:
        os.makedirs(output, exist_ok=True)
        limit = os.sysconf("SC_ARG_MAX") // 4
        for item in result:
            expression = item["filter"]
            if len(expression.encode()) > limit:
                raise SystemExit(f"bin {item['bin']} filter exceeds argv guard {limit}")
            with open(os.path.join(output, f"bin-{item['bin']}.expr"), "w") as file:
                file.write(expression)
    assignment = {"bins": result, "fallbacks": fallbacks}
    if output:
        with open(os.path.join(output, "assignments.json"), "w") as file:
            json.dump(assignment, file)
    print(json.dumps(assignment, indent=2))


if __name__ == "__main__":
    main()
