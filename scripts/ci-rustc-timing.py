#!/usr/bin/env python3
"""Measure rustc unit wall times and summarize the shard build categories.

Cargo invokes this as RUSTC_WRAPPER. The recorded intervals classify compiler
units as third-party dependencies, workspace crate chain, or test binaries.
"""

import json
import os
from pathlib import Path
import subprocess
import sys
import time

CATEGORIES = (
    "third_party_dependencies",
    "workspace_crate_chain",
    "test_binaries",
)


def is_within(path, parent):
    if not path or not parent:
        return False
    try:
        return os.path.commonpath((os.path.realpath(path), os.path.realpath(parent))) == os.path.realpath(parent)
    except (OSError, ValueError):
        return False


def source_manifest(args):
    for argument in args:
        if not argument.endswith(".rs") or not os.path.isfile(argument):
            continue
        path = Path(argument).resolve().parent
        for directory in (path, *path.parents):
            if (directory / "Cargo.toml").is_file():
                return str(directory)
    return None


def crate_name(args):
    for index, argument in enumerate(args[:-1]):
        if argument == "--crate-name":
            return args[index + 1]
    return os.environ.get("CARGO_PKG_NAME", "unknown")


def manifest_for(args):
    return os.environ.get("CARGO_MANIFEST_DIR") or source_manifest(args)


def package_label(manifest, workspace):
    if not manifest:
        return "unknown"
    if is_within(manifest, workspace):
        return os.path.relpath(os.path.realpath(manifest), os.path.realpath(workspace))
    return f"external/{Path(manifest).name}"


def category_for(args, workspace, manifest=None):
    if "--test" in args:
        return "test_binaries"
    manifest = manifest or manifest_for(args)
    if is_within(manifest, workspace):
        return "workspace_crate_chain"
    return "third_party_dependencies"


def append_record(path, record):
    if not path:
        raise RuntimeError("CI_RUSTC_TIMING_LOG is required")
    encoded = (json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n").encode()
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_APPEND, 0o644)
    try:
        remaining = memoryview(encoded)
        while remaining:
            written = os.write(descriptor, remaining)
            if written <= 0:
                raise OSError("short write to rustc timing log")
            remaining = remaining[written:]
    finally:
        os.close(descriptor)


def run_compiler(argv):
    if not argv:
        print("ci-rustc-timing: Cargo did not pass a rustc executable", file=sys.stderr)
        return 2
    compiler, *args = argv
    # Cargo probes rustc for version/capabilities as well as compiling units.
    # Do not count metadata queries as build work.
    if "--crate-name" not in args:
        return subprocess.run([compiler, *args], close_fds=False, check=False).returncode

    workspace = os.environ.get("CI_RUSTC_TIMING_WORKSPACE") or os.environ.get("GITHUB_WORKSPACE")
    manifest = manifest_for(args)
    category = category_for(args, workspace, manifest)
    start_ns = time.monotonic_ns()
    try:
        result = subprocess.run([compiler, *args], close_fds=False, check=False)
        return_code = result.returncode
    except OSError as error:
        print(f"ci-rustc-timing: cannot run rustc: {error}", file=sys.stderr)
        return_code = 127
    end_ns = time.monotonic_ns()
    record = {
        "category": category,
        "crate_name": crate_name(args),
        "package": package_label(manifest, workspace),
        "start_ns": start_ns,
        "end_ns": end_ns,
        "return_code": return_code,
    }
    try:
        append_record(os.environ.get("CI_RUSTC_TIMING_LOG"), record)
    except OSError as error:
        print(f"ci-rustc-timing: cannot record compilation timing: {error}", file=sys.stderr)
        return 125
    return return_code


def interval_union_seconds(intervals):
    if not intervals:
        return 0.0
    intervals = sorted(intervals)
    total = 0
    start, end = intervals[0]
    for next_start, next_end in intervals[1:]:
        if next_start <= end:
            end = max(end, next_end)
        else:
            total += end - start
            start, end = next_start, next_end
    total += end - start
    return total / 1_000_000_000


def read_records(path):
    records = []
    with open(path, encoding="utf-8") as source:
        for line_number, line in enumerate(source, 1):
            try:
                record = json.loads(line)
            except json.JSONDecodeError as error:
                raise ValueError(f"{path}:{line_number}: malformed JSON timing row") from error
            category = record.get("category")
            start = record.get("start_ns")
            end = record.get("end_ns")
            return_code = record.get("return_code")
            if (
                category not in CATEGORIES
                or not isinstance(start, int)
                or isinstance(start, bool)
                or not isinstance(end, int)
                or isinstance(end, bool)
                or end < start
                or not isinstance(return_code, int)
                or isinstance(return_code, bool)
            ):
                raise ValueError(f"{path}:{line_number}: invalid compiler timing row")
            records.append(record)
    return records


def report(path, shard):
    if not Path(path).exists():
        records = []
    else:
        records = read_records(path)
    all_intervals = [(row["start_ns"], row["end_ns"]) for row in records]
    build_span = (
        (max(end for _, end in all_intervals) - min(start for start, _ in all_intervals))
        / 1_000_000_000
        if all_intervals else 0.0
    )
    print(
        f"CI_BUILD_TIMING shard={shard} method=rustc-wrapper "
        f"compiler_build_span_seconds={build_span:.3f} compiler_units={len(records)} "
        "category_wall_intervals_may_overlap=true"
    )
    for category in CATEGORIES:
        rows = [row for row in records if row["category"] == category]
        intervals = [(row["start_ns"], row["end_ns"]) for row in rows]
        unit_seconds = sum(end - start for start, end in intervals) / 1_000_000_000
        active_seconds = interval_union_seconds(intervals)
        failed = sum(row["return_code"] != 0 for row in rows)
        status = "measured" if rows else "no-units-observed"
        print(
            f"CI_BUILD_TIMING shard={shard} category={category} status={status} "
            f"compiler_units={len(rows)} unit_wall_seconds_sum={unit_seconds:.3f} "
            f"active_wall_seconds={active_seconds:.3f} failed_units={failed}"
        )
    missing = [category for category in CATEGORIES if not any(
        row["category"] == category for row in records
    )]
    if missing:
        print(
            f"CI_BUILD_TIMING shard={shard} status=missing-timing-categories "
            f"categories={','.join(missing)}"
        )
        return 1
    return 0


def main(argv):
    if argv and argv[0] == "report":
        if len(argv) != 3:
            print("usage: ci-rustc-timing.py report LOG SHARD", file=sys.stderr)
            return 2
        try:
            return report(argv[1], argv[2])
        except (OSError, ValueError) as error:
            print(f"ci-rustc-timing: {error}", file=sys.stderr)
            return 1
    return run_compiler(argv)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
