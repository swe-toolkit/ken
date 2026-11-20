#!/usr/bin/env python3
"""Join nextest's exact (binary,test) outcomes to the I4 scratch artifact.

Usage: python3 scripts/lang-delete-i4-step1b-ledger.py --log JOB_LOG ...
       --artifact REALIZED_SHARD_DIR ... --output REPORT.tsv
       [--prior-no-record PRIOR_23.tsv] [--require-shards 7]

Read-only postprocessing: no source migration, GitHub access or test run.
The shard artifact is the primary record source: passing tests and spawned
children may emit no captured output in the nextest job log. Refuse duplicate
identities or missing artifact files rather than silently claiming zero hits.
"""
import argparse
import collections
import csv
import json
from pathlib import Path
import re

STAT = re.compile(r"\b(PASS|FAIL) \[\s*\d+(?:\.\d+)?s\] \(\s*\d+/\d+\) (\S+) (\S+)\s*$")
TEST_FAILED = re.compile(r"^\s*test (\S+) \.\.\. FAILED\s*$")
PANIC = re.compile(r"\b(?:thread .* panicked at|panicked at)\b")
HOME_COUNT = re.compile(r"\bLEGACY_CENSUS_HOME_COUNT=(\d+)\s*$")
HOME = re.compile(r"\bLEGACY_CENSUS_HOME index=(\d+) home=(\S+) name=(\S+)\s*$")


def content(line):
    # The Actions read log prefixes every source line with step and UTC time.
    return line.split("Z ", 1)[-1].rstrip("\r\n")


def read_job(path):
    outcomes = {}
    first_panics = {}
    active_name = None
    for raw in path.read_text(errors="replace").splitlines():
        line = content(raw)
        if "LEGACY_CENSUS_SINK_ERROR" in line:
            raise SystemExit(f"FAIL: census artifact sink failed in {path}: {line}")
        match = STAT.search(line)
        if match:
            _, binary, name = match.groups()
            key = (binary, name)
            if key in outcomes:
                raise SystemExit(f"duplicate nextest terminal identity in {path}: {key}")
            outcomes[key] = match[1]
        failed = TEST_FAILED.match(line)
        if failed:
            active_name = failed[1]
        elif "stdout ───" in line:
            # The next test begins a new captured block. The failed test's
            # stderr follows its own stdout before the next such header.
            active_name = None
        elif PANIC.search(line) and active_name is not None:
            first_panics.setdefault(active_name, line.strip())
    if not outcomes:
        raise SystemExit(f"no nextest terminal outcomes in {path}")
    return outcomes, first_panics


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--log", type=Path, action="append", required=True)
    parser.add_argument("--artifact", type=Path, action="append", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--prior-no-record", type=Path)
    parser.add_argument("--require-shards", type=int)
    args = parser.parse_args()
    if len(args.log) != len(args.artifact):
        raise SystemExit("FAIL: supply one job log and one realized-shard artifact per shard")
    if args.require_shards is not None and len(args.log) != args.require_shards:
        raise SystemExit(f"FAIL: expected {args.require_shards} shard jobs, got {len(args.log)}")

    outcomes = {}
    panics = collections.defaultdict(set)
    for log in args.log:
        found, found_panics = read_job(log)
        for key, outcome in found.items():
            if key in outcomes:
                raise SystemExit(f"FAIL: selected test duplicated across shards: {key}")
            outcomes[key] = outcome
        for name, line in found_panics.items():
            panics[name].add(line)

    records = collections.defaultdict(list)
    for artifact in args.artifact:
        path = artifact / "i4-census" / "records.jsonl" if artifact.is_dir() else artifact
        if not path.is_file():
            raise SystemExit(f"FAIL: no child-safe record artifact in {artifact}")
        for number, line in enumerate(path.read_text(errors="replace").splitlines(), 1):
            try:
                record = json.loads(line)
            except json.JSONDecodeError as error:
                raise SystemExit(f"FAIL: corrupt record {path}:{number}: {error}") from error
            if record.get("kind") not in {"S", "STANDALONE"}:
                raise SystemExit(f"FAIL: unexpected instrument event {path}:{number}: {record}")
            binary, name = record.get("binary"), record.get("test")
            if not binary or not name:
                raise SystemExit(f"FAIL: unowned census record {path}:{number}: {record}")
            key = (binary, name)
            if key not in outcomes:
                raise SystemExit(f"FAIL: census record not selected by nextest: {key} at {path}:{number}")
            records[key].append(record)

    duplicate_names = collections.Counter(name for _, name in outcomes)
    for key, outcome in outcomes.items():
        if outcome == "FAIL" and duplicate_names[key[1]] != 1:
            raise SystemExit(f"FAIL: first panic ambiguous across binaries for {key[1]}")
    expected_c2 = set()
    if args.prior_no_record:
        expected_c2 = {
            tuple(line.split("\t", 1))
            for line in args.prior_no_record.read_text().splitlines()
            if line and not line.startswith("#")
        }
        if len(expected_c2) != 23:
            raise SystemExit(f"FAIL: C2 prior no-record identity count is {len(expected_c2)}, not 23")

    args.output.parent.mkdir(parents=True, exist_ok=True)
    failed = 0
    with args.output.open("w", newline="") as output:
        writer = csv.writer(output, delimiter="\t")
        writer.writerow(("binary", "test", "outcome", "record_count",
                         "first_panic_line_json", "records_json"))
        for key, outcome in sorted(outcomes.items()):
            rows = records.get(key, [])
            if outcome != "FAIL" and not rows:
                continue
            if outcome == "FAIL":
                failed += 1
            panic_lines = panics.get(key[1], set()) if outcome == "FAIL" else set()
            first_panic = min(panic_lines) if panic_lines else "<no panic line observed>"
            writer.writerow((*key, outcome, len(rows), json.dumps(first_panic),
                             json.dumps(rows, sort_keys=True, separators=(",", ":"))))
    for key in expected_c2:
        if outcomes.get(key) != "FAIL" or records.get(key):
            raise SystemExit(f"C2 unexpected outcome or records for prior no-record FAIL: {key}")
    classes = collections.defaultdict(collections.Counter)
    bare = collections.defaultdict(lambda: collections.Counter())
    for key, rows in records.items():
        for row in rows:
            payload = row["payload"]
            if row["kind"] == "STANDALONE":
                classes[payload["status"]][payload["name"]] += 1
            elif payload["site"] == "S-BARE":
                bare[payload["name"]][f"{key[0]}|{key[1]}"] += 1
    home_rows, home_counts = [], []
    for log in args.log:
        for raw in log.read_text(errors="replace").splitlines():
            line = content(raw)
            if match := HOME_COUNT.search(line):
                home_counts.append(int(match[1]))
            if match := HOME.search(line):
                home_rows.append((int(match[1]), match[2], match[3]))
    if args.require_shards is not None and not home_counts:
        raise SystemExit("FAIL: full-shard run omitted the live home inventory output")
    if home_counts:
        if len(home_counts) != 1 or len(home_rows) != home_counts[0]:
            raise SystemExit(f"FAIL: incomplete derived home inventory: {home_counts} counts, {len(home_rows)} rows")
        if [index for index, _, _ in home_rows] != list(range(len(home_rows))):
            raise SystemExit("FAIL: home inventory indices missing, duplicated or reordered")
        if len({(home, name) for _, home, name in home_rows}) != len(home_rows):
            raise SystemExit("FAIL: duplicate home/name inventory pair")
        home_output = args.output.with_suffix(".home.tsv")
        with home_output.open("w", newline="") as output:
            writer = csv.writer(output, delimiter="\t")
            writer.writerow(("index", "home", "name", "S-BARE_arrivals", "row_id_to_count_json"))
            for index, home, name in home_rows:
                per_row = dict(sorted(bare[name].items()))
                writer.writerow((index, home, name, sum(per_row.values()), json.dumps(per_row)))
    summary = args.output.with_suffix(".summary.txt")
    summary.write_text(
        f"nextest identities={len(outcomes)} failed={failed} "
        f"recorded_rows={len(records)} c2_no_record_fail={len(expected_c2)} "
        f"S-BARE_arrivals={sum(sum(counts.values()) for counts in bare.values())} "
        f"home_entries={len(home_rows)}\n"
        + "".join(
            f"STANDALONE {status} total={sum(names.values())} names={json.dumps(dict(sorted(names.items())))}\n"
            for status, names in sorted(classes.items())
        )
    )
    print(summary.read_text(), end="")
    print(f"ledger={args.output}")


if __name__ == "__main__":
    main()
