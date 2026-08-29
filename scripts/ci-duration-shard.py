#!/usr/bin/env python3
"""Assign a live nextest JSON inventory to deterministic duration-balanced bins."""
import json, sys, statistics, heapq
from pathlib import Path

N = 8

def tests(value):
    # nextest's list JSON groups tests under suites; preserve canonical fields.
    for suite in value.get("rust-suites", {}).values():
        package = suite["package-name"]
        binary = suite["binary-name"]
        identity = package if binary == package else f"{package}::{binary}"
        for name in suite["testcases"]:
            yield identity, name

def main():
    inventory = json.load(open(sys.argv[1]))
    evidence = json.load(open(sys.argv[2]))
    durations = {r["test_id"]: r["seconds"] for r in evidence["records"]}
    median = statistics.median(durations.values())
    bins = [(0.0, index, []) for index in range(N)]
    heapq.heapify(bins)
    live = sorted((f"{identity} {name}", identity, name) for identity, name in tests(inventory))
    for rendered, identity, name in sorted(live, key=lambda x: (-durations.get(x[0], median), x[0])):
        total, index, selected = heapq.heappop(bins)
        selected.append((identity, name))
        heapq.heappush(bins, (total + durations.get(rendered, median), index, selected))
    print(json.dumps({"bins": [{"bin": index + 1, "tests": selected} for _, index, selected in sorted(bins, key=lambda x:x[1])]}, indent=2))
if __name__ == "__main__": main()
