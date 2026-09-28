#!/usr/bin/env python3
"""Assign a filtered live nextest inventory to deterministic duration bins."""
import csv
import json
import math
import os
import re
import statistics
import sys
import warnings

import ci_workspace_timings


N = 7
# Use the plain quadratic convex marginal cost, without fitting its power to
# the A/B exploratory scores or the 90s target.
WALL_BALANCE_POWER = 2.0
WALL_CALIBRATION_COLUMNS = [
    "run_id",
    "shard",
    "build_seconds",
    "selection_seconds",
    "test_step_seconds",
    "job_wall_seconds",
]
# CI uses each identity's maximum passing duration across full-CI sources.
# A single-source observation is retained; unmeasured live identities use the
# same-binary median or the global median when their binary has no observations.
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


def inventory_delta(live_ids, baseline_ids):
    added = sorted(live_ids - baseline_ids)
    removed = sorted(baseline_ids - live_ids)
    return {
        "baseline_count": len(baseline_ids),
        "live_count": len(live_ids),
        "added_count": len(added),
        "removed_count": len(removed),
        "added": added,
        "removed": removed,
    }


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


def read_duration_sources(paths, source_shards=None):
    if isinstance(paths, (str, os.PathLike)):
        paths = [paths]
    paths = [os.fspath(path) for path in paths]
    if not paths:
        raise SystemExit("duration evidence has no input files")
    samples = {}
    run_test_shards = {}
    run_fallbacks = {}
    for path in paths:
        observed = {}
        if path.endswith(".tsv"):
            source_id = f"file:{path}"
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
                if source_shards is not None:
                    source_shards[(test_id, source_id)] = int(
                        (match or tsv_match).group("shard")
                    )
        else:
            try:
                with open(path, encoding="utf-8") as source:
                    evidence = json.load(source)
                artifact = ci_workspace_timings.validate_artifact(evidence)
            except (OSError, json.JSONDecodeError, ci_workspace_timings.TimingArtifactError) as error:
                raise SystemExit(f"{path}: invalid duration artifact: {error}") from error
            source_id = f"run:{artifact['run_id']}"
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
                if source_shards is not None:
                    source_shards[(test_id, source_id)] = artifact["shard"]
        for test_id, seconds in observed.items():
            prior = samples.setdefault(test_id, {}).get(source_id)
            samples[test_id][source_id] = seconds if prior is None else max(prior, seconds)
    return samples


def read_durations(paths):
    return {
        test_id: max(samples.values())
        for test_id, samples in read_duration_sources(paths).items()
    }


def resolve_duration_sources(live, samples):
    """Use measured maxima, then binary or global medians for missing tests."""
    by_binary = {}
    measured_live = {}
    single_run = []
    for rendered, binary_id, _ in live:
        observations = samples.get(rendered, {})
        if not observations:
            continue
        duration = max(observations.values())
        measured_live[rendered] = duration
        by_binary.setdefault(binary_id, []).append(duration)
        if len(observations) == 1:
            source_id, single_seconds = next(iter(observations.items()))
            single_run.append({
                "test_id": rendered,
                "seconds": single_seconds,
                "source": source_id,
            })
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
    return resolved, fallbacks, single_run


def resolve_durations(live, measured):
    """Legacy map adapter for controls that do not need source attribution."""
    samples = {
        test_id: {"legacy": seconds}
        for test_id, seconds in measured.items()
    }
    resolved, fallbacks, _ = resolve_duration_sources(live, samples)
    return resolved, fallbacks


def latest_source_changes(live, samples, resolved, fallbacks, source_shards):
    """Report computed fallback-to-measured transitions and >10s weight changes."""
    run_sources = sorted(
        {
            source_id
            for observations in samples.values()
            for source_id in observations
            if source_id.startswith("run:") and source_id.partition(":")[2].isdigit()
        },
        key=lambda source_id: int(source_id.partition(":")[2]),
    )
    if len(run_sources) < 2:
        return None, [], []
    latest_source = run_sources[-1]
    previous_samples = {}
    for test_id, observations in samples.items():
        previous = {
            source_id: seconds
            for source_id, seconds in observations.items()
            if source_id != latest_source
        }
        if previous:
            previous_samples[test_id] = previous
    if not any(test_id in previous_samples for test_id, _, _ in live):
        return latest_source, [], []

    previous_resolved, previous_fallbacks, _ = resolve_duration_sources(
        live, previous_samples
    )
    previous_fallbacks = {
        row["test_id"]: row for row in previous_fallbacks
    }
    current_fallbacks = {row["test_id"]: row for row in fallbacks}
    fallback_to_measured = []
    weight_changes = []
    for test_id, _, _ in live:
        previous_method = previous_fallbacks.get(test_id, {}).get(
            "method", "measured"
        )
        current_method = current_fallbacks.get(test_id, {}).get(
            "method", "measured"
        )
        previous_seconds = previous_resolved[test_id]
        current_seconds = resolved[test_id]
        if test_id in previous_fallbacks and latest_source in samples.get(test_id, {}):
            fallback_to_measured.append({
                "test_id": test_id,
                "source": latest_source,
                "previous_method": previous_method,
                "previous_seconds": round(previous_seconds, 3),
                "source_seconds": samples[test_id][latest_source],
                "source_shard": source_shards.get((test_id, latest_source)),
                "current_seconds": round(current_seconds, 3),
            })
        delta_seconds = current_seconds - previous_seconds
        if abs(delta_seconds) > 10.0:
            weight_changes.append({
                "test_id": test_id,
                "source": latest_source,
                "previous_method": previous_method,
                "previous_seconds": round(previous_seconds, 3),
                "current_method": current_method,
                "current_seconds": round(current_seconds, 3),
                "delta_seconds": round(delta_seconds, 3),
                "source_shard": source_shards.get((test_id, latest_source)),
            })
    return latest_source, fallback_to_measured, weight_changes


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
    """Fit the run-local relation from terminal test work to Test-step time."""
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

    # Fit the Test step to total terminal work plus a longest-test tail feature.
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

    setup = [
        row["job_wall_seconds"] - row["test_step_seconds"]
        for row in observations.values()
    ]
    return {
        "run_id": run_id,
        "intercept_seconds": intercept,
        "sum_coefficient": sum_coefficient,
        "longest_test_coefficient": longest_coefficient,
        "setup_seconds": statistics.median(setup),
    }


def project_test_wall_seconds(test_sum_seconds, longest_test_seconds, model):
    predicted = (
        model["intercept_seconds"]
        + model["sum_coefficient"] * test_sum_seconds
        + model["longest_test_coefficient"] * longest_test_seconds
    )
    return max(0.0, predicted)


def _plan_bins(live, durations, wall_model=None, objective="default"):
    """Assign every live test once, using only the supplied training weights."""
    if objective == "default":
        objective = "sum-lpt" if wall_model is None else "convex-test-wall"
    if objective not in {"sum-lpt", "legacy-test-wall-lpt", "convex-test-wall"}:
        raise SystemExit(f"unknown workspace shard objective: {objective}")
    if objective != "sum-lpt" and wall_model is None:
        raise SystemExit("test-wall objectives require a fitted model")

    bins = [(0.0, index, 0.0, 0.0, []) for index in range(N)]
    ordered = sorted(live, key=lambda row: (-durations[row[0]], row[0]))
    for rendered, binary_id, name in ordered:
        duration = durations[rendered]
        if objective == "sum-lpt":
            index = min(range(N), key=lambda i: (bins[i][2], i))
        elif objective == "legacy-test-wall-lpt":
            index = min(
                range(N),
                key=lambda i: (
                    project_test_wall_seconds(bins[i][2], bins[i][3], wall_model),
                    i,
                ),
            )
        else:
            def marginal_cost(index):
                _, _, total, longest, _ = bins[index]
                current = project_test_wall_seconds(total, longest, wall_model)
                projected = project_test_wall_seconds(
                    total + duration, max(longest, duration), wall_model
                )
                marginal = projected ** WALL_BALANCE_POWER - current ** WALL_BALANCE_POWER
                return marginal, index

            index = min(range(N), key=marginal_cost)
        _, _, total, longest, selected = bins[index]
        total += duration
        longest = max(longest, duration)
        selected.append((binary_id, name))
        score = (
            project_test_wall_seconds(total, longest, wall_model)
            if wall_model is not None else total
        )
        bins[index] = (score, index, total, longest, selected)
    return bins


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
    wall_calibration_artifacts = None
    population_baseline = None
    population_reference = None
    if "--population-reference" in arguments:
        option = arguments.index("--population-reference")
        if option < 2 or option + 1 >= len(arguments):
            raise SystemExit(
                "--population-reference must follow timing files and name an inventory"
            )
        population_reference = arguments[option + 1]
        del arguments[option : option + 2]
    if "--population-baseline" in arguments:
        option = arguments.index("--population-baseline")
        if option < 2 or option + 1 >= len(arguments):
            raise SystemExit(
                "--population-baseline must follow timing files and name an inventory"
            )
        population_baseline = arguments[option + 1]
        del arguments[option : option + 2]
    if "--wall-calibration-artifacts" in arguments:
        option = arguments.index("--wall-calibration-artifacts")
        if option < 2 or option + 1 >= len(arguments):
            raise SystemExit(
                "--wall-calibration-artifacts must follow timing files and name a directory"
            )
        calibration_dir = arguments[option + 1]
        wall_calibration_artifacts = [
            os.path.join(calibration_dir, f"shard-{shard}.json")
            for shard in range(1, N + 1)
        ]
        del arguments[option : option + 2]
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
    if wall_calibration_artifacts is not None and wall_calibration is None:
        raise SystemExit("--wall-calibration-artifacts requires --wall-calibration")
    if (population_baseline is not None or population_reference is not None) and output is None:
        raise SystemExit("population inventory options require --output-dir")
    if len(arguments) < 2:
        raise SystemExit(
            "usage: ci-duration-shard.py INVENTORY TIMING... "
            "[--population-baseline INVENTORY] [--population-reference INVENTORY] "
            "[--wall-calibration TSV [--wall-calibration-artifacts DIR]] "
            "[--output-dir DIR]"
        )
    inventory = json.load(open(arguments[0]))
    timing_paths = arguments[1:]
    source_shards = {}
    samples = read_duration_sources(timing_paths, source_shards)
    durations = {
        test_id: max(observations.values())
        for test_id, observations in samples.items()
    }
    # Keep the fixed model-fit source out of the per-test weight population.
    calibration_paths = wall_calibration_artifacts or timing_paths
    wall_model = (
        fit_workspace_wall_model(calibration_paths, wall_calibration)
        if wall_calibration is not None else None
    )
    live = sorted((f"{binary_id} {name}", binary_id, name) for binary_id, name in tests(inventory))
    if not live:
        raise SystemExit("filtered live inventory selected zero testcases")
    live_ids = {row[0] for row in live}
    baseline_ids = None
    if population_baseline is not None:
        try:
            with open(population_baseline, encoding="utf-8") as source:
                baseline_inventory = json.load(source)
        except (OSError, json.JSONDecodeError) as error:
            raise SystemExit(
                f"{population_baseline}: invalid population baseline: {error}"
            ) from error
        baseline_ids = {
            f"{binary_id} {name}"
            for binary_id, name in tests(baseline_inventory)
        }
    reference_ids = None
    if population_reference is not None:
        try:
            with open(population_reference, encoding="utf-8") as source:
                reference_inventory = json.load(source)
        except (OSError, json.JSONDecodeError) as error:
            raise SystemExit(
                f"{population_reference}: invalid population reference: {error}"
            ) from error
        reference_ids = {
            f"{binary_id} {name}"
            for binary_id, name in tests(reference_inventory)
        }
    population_delta = (
        inventory_delta(live_ids, baseline_ids) if baseline_ids is not None else None
    )
    population_reference_delta = (
        inventory_delta(live_ids, reference_ids) if reference_ids is not None else None
    )
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
    planned_durations, fallbacks, single_run = resolve_duration_sources(live, samples)
    comparison_source, fallback_to_measured, weight_changes_over_10s = (
        latest_source_changes(
            live, samples, planned_durations, fallbacks, source_shards
        )
    )
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
    bins = _plan_bins(live, planned_durations, wall_model)

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
    assigned_shards = {
        f"{binary_id} {name}": item["bin"]
        for item in result
        for binary_id, name in item["tests"]
    }
    for row in [*fallback_to_measured, *weight_changes_over_10s]:
        row["planned_shard"] = assigned_shards[row["test_id"]]
    assignment = {"bins": result, "fallbacks": fallbacks}
    if output:
        with open(os.path.join(output, "assignments.json"), "w") as file:
            json.dump(assignment, file)
        planning_evidence = {
            "population_baseline": population_baseline,
            "population_delta": population_delta,
            "population_reference": population_reference,
            "population_reference_delta": population_reference_delta,
            "measurement_sources": sorted({
                source_id
                for observations in samples.values()
                for source_id in observations
            }),
            "fallbacks": fallbacks,
            "fallback_to_measured": fallback_to_measured,
            "weight_comparison_source": comparison_source,
            "weight_changes_over_10s": weight_changes_over_10s,
            "single_run": single_run,
            "stale_timing_identities": stale,
        }
        with open(os.path.join(output, "planning-evidence.json"), "w") as file:
            json.dump(planning_evidence, file)
    print(json.dumps(assignment, indent=2))


if __name__ == "__main__":
    main()
