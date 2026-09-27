#!/usr/bin/env python3
"""Assign a filtered live nextest inventory to deterministic duration bins."""
import csv
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
WALL_MODEL_TOLERANCE_SECONDS = 60.0
WALL_CALIBRATION_COLUMNS = [
    "run_id",
    "shard",
    "build_seconds",
    "selection_seconds",
    "test_step_seconds",
    "job_wall_seconds",
]
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


def _solve_linear_system(matrix, values):
    size = len(values)
    augmented = [list(row) + [values[index]] for index, row in enumerate(matrix)]
    for column in range(size):
        pivot = max(range(column, size), key=lambda row: abs(augmented[row][column]))
        if abs(augmented[pivot][column]) < 1e-12:
            raise SystemExit("workspace wall calibration is singular")
        augmented[column], augmented[pivot] = augmented[pivot], augmented[column]
        divisor = augmented[column][column]
        augmented[column] = [entry / divisor for entry in augmented[column]]
        for row in range(size):
            if row == column:
                continue
            factor = augmented[row][column]
            augmented[row] = [
                entry - factor * pivot_entry
                for entry, pivot_entry in zip(augmented[row], augmented[column])
            ]
    return [augmented[index][-1] for index in range(size)]


def _workspace_run_workloads(timing_paths):
    if len(timing_paths) != N:
        raise SystemExit("workspace wall calibration requires seven timing artifacts")
    workloads = {}
    all_identities = set()
    fallback_records = None
    run_ids = set()
    for path in timing_paths:
        try:
            with open(path, encoding="utf-8") as source:
                artifact = json.load(source)
            artifact = ci_workspace_timings.validate_artifact(artifact)
        except (OSError, json.JSONDecodeError, ci_workspace_timings.TimingArtifactError) as error:
            raise SystemExit(f"{path}: invalid wall-calibration timing artifact: {error}") from error
        run_ids.add(artifact["run_id"])
        if fallback_records is None:
            fallback_records = artifact["fallbacks"]
        elif artifact["fallbacks"] != fallback_records:
            raise SystemExit("wall-calibration artifacts disagree on fallback identities")
        shard = artifact["shard"]
        if shard in workloads:
            raise SystemExit(f"duplicate wall-calibration timing shard {shard}")
        records = artifact["records"]
        if not records or any(record["result"] != "PASS" for record in records):
            raise SystemExit(f"wall-calibration shard {shard} needs only passing timings")
        identities = {record["test_id"] for record in records}
        if all_identities & identities:
            raise SystemExit("wall-calibration timing shards have duplicate identities")
        all_identities |= identities
        seconds = [float(record["seconds"]) for record in records]
        workloads[shard] = {
            "test_sum_seconds": sum(seconds),
            "longest_test_seconds": max(seconds),
        }
    if len(run_ids) != 1 or set(workloads) != set(range(1, N + 1)):
        raise SystemExit("wall-calibration artifacts must be one complete seven-shard run")
    return next(iter(run_ids)), workloads


def fit_workspace_wall_model(timing_paths, calibration_path):
    """Fit a test-wall model and enforce its per-shard reprojection tolerance."""
    run_id, workloads = _workspace_run_workloads(timing_paths)
    try:
        with open(calibration_path, encoding="utf-8", newline="") as source:
            reader = csv.DictReader(source, delimiter="\t")
            if reader.fieldnames != WALL_CALIBRATION_COLUMNS:
                raise SystemExit("workspace wall calibration has invalid columns")
            rows = list(reader)
    except OSError as error:
        raise SystemExit(f"{calibration_path}: cannot read wall calibration: {error}") from error
    observations = {}
    for row in rows:
        try:
            row_run_id = int(row["run_id"])
            shard = int(row["shard"])
            build_seconds = float(row["build_seconds"])
            selection_seconds = float(row["selection_seconds"])
            test_seconds = float(row["test_step_seconds"])
            job_seconds = float(row["job_wall_seconds"])
        except (KeyError, TypeError, ValueError) as error:
            raise SystemExit(f"{calibration_path}: malformed wall calibration row") from error
        numeric = (build_seconds, selection_seconds, test_seconds, job_seconds)
        if (
            row_run_id != run_id
            or not 1 <= shard <= N
            or shard in observations
            or any(not math.isfinite(value) or value < 0 for value in numeric)
            or test_seconds <= 0
            or job_seconds < test_seconds
            or build_seconds + selection_seconds > job_seconds
        ):
            raise SystemExit(f"{calibration_path}: invalid wall calibration row for shard {shard}")
        observations[shard] = {
            **workloads.get(shard, {}),
            "build_seconds": build_seconds,
            "selection_seconds": selection_seconds,
            "test_step_seconds": test_seconds,
            "job_wall_seconds": job_seconds,
        }
    if set(observations) != set(range(1, N + 1)):
        raise SystemExit("workspace wall calibration must contain each shard exactly once")

    # Fit the observed Test step to total work plus its indivisible slowest test.
    # Build, selection, and other setup remain a separate measured overhead.
    features = [
        [1.0, row["test_sum_seconds"], row["longest_test_seconds"]]
        for row in (observations[shard] for shard in range(1, N + 1))
    ]
    measured = [observations[shard]["test_step_seconds"] for shard in range(1, N + 1)]
    size = len(features[0])
    normal = [
        [sum(row[left] * row[right] for row in features) for right in range(size)]
        for left in range(size)
    ]
    target = [
        sum(features[index][column] * measured[index] for index in range(N))
        for column in range(size)
    ]
    intercept, sum_coefficient, longest_coefficient = _solve_linear_system(normal, target)
    coefficients = {
        "intercept_seconds": intercept,
        "sum_coefficient": sum_coefficient,
        "longest_test_coefficient": longest_coefficient,
    }
    if any(not math.isfinite(value) for value in coefficients.values()) or min(
        sum_coefficient, longest_coefficient
    ) <= 0:
        raise SystemExit("workspace wall calibration produced invalid workload coefficients")

    setup = []
    errors = {}
    for shard, row in observations.items():
        pretest = row["job_wall_seconds"] - row["test_step_seconds"]
        setup.append(pretest)
        predicted_test = project_test_wall_seconds(
            row["test_sum_seconds"], row["longest_test_seconds"], coefficients
        )
        predicted_job = pretest + predicted_test
        error = abs(predicted_job - row["job_wall_seconds"])
        errors[shard] = error
        if error > WALL_MODEL_TOLERANCE_SECONDS:
            raise SystemExit(
                f"wall reprojection shard {shard} differs by {error:.3f}s "
                f"(limit {WALL_MODEL_TOLERANCE_SECONDS:.0f}s)"
            )
    return {
        "run_id": run_id,
        "intercept_seconds": intercept,
        "sum_coefficient": sum_coefficient,
        "longest_test_coefficient": longest_coefficient,
        "setup_seconds": statistics.median(setup),
        "calibration_errors": errors,
    }


def project_test_wall_seconds(test_sum_seconds, longest_test_seconds, model):
    predicted = (
        model["intercept_seconds"]
        + model["sum_coefficient"] * test_sum_seconds
        + model["longest_test_coefficient"] * longest_test_seconds
    )
    return max(0.0, predicted)


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
    wall_calibration = None
    if "--wall-calibration" in arguments:
        option = arguments.index("--wall-calibration")
        if option < 2 or option + 1 >= len(arguments):
            raise SystemExit("--wall-calibration must follow timing files and name a TSV")
        wall_calibration = arguments[option + 1]
        del arguments[option : option + 2]
    if "--output-dir" in arguments:
        option = arguments.index("--output-dir")
        if option != len(arguments) - 2 or option < 2:
            raise SystemExit("--output-dir must follow at least one timing file")
        output = arguments[-1]
        arguments = arguments[:-2]
    if len(arguments) < 2:
        raise SystemExit(
            "usage: ci-duration-shard.py INVENTORY TIMING... "
            "[--wall-calibration TSV] [--output-dir DIR]"
        )
    inventory = json.load(open(arguments[0]))
    timing_paths = arguments[1:]
    durations = read_durations(timing_paths)
    wall_model = (
        fit_workspace_wall_model(timing_paths, wall_calibration)
        if wall_calibration is not None else None
    )
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
    bins = []
    for index in range(N):
        score = 0.0
        bins.append((score, index, 0.0, 0.0, []))
    heapq.heapify(bins)
    ordered = sorted(live, key=lambda row: (-planned_durations[row[0]], row[0]))
    for rendered, binary_id, name in ordered:
        _, index, total, longest, selected = heapq.heappop(bins)
        duration = planned_durations[rendered]
        total += duration
        longest = max(longest, duration)
        selected.append((binary_id, name))
        score = (
            project_test_wall_seconds(total, longest, wall_model)
            if wall_model is not None else total
        )
        heapq.heappush(bins, (score, index, total, longest, selected))

    result = []
    for _, index, total, longest, selected in sorted(bins, key=lambda item: item[1]):
        terms = [f"(binary_id(={binary}) & test(={name}))" for binary, name in selected]
        item = {
            "bin": index + 1,
            "seconds": round(total, 3),
            "tests": selected,
            "filter": " | ".join(terms),
        }
        if wall_model is not None:
            projected_test = project_test_wall_seconds(total, longest, wall_model)
            item.update({
                "longest_test_seconds": round(longest, 3),
                "projected_test_seconds": round(projected_test, 3),
                "projected_job_seconds": round(
                    projected_test + wall_model["setup_seconds"], 3
                ),
            })
        result.append(item)
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
