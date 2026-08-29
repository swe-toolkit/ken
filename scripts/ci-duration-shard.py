#!/usr/bin/env python3
"""Assign a live nextest JSON inventory to deterministic duration-balanced bins."""
import json, sys, statistics, heapq, os
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
    result = []
    for _, index, selected in sorted(bins, key=lambda x:x[1]):
        terms = [f"(binary_id(={binary}) & test(={name}))" for binary, name in selected]
        result.append({"bin": index + 1, "tests": selected, "filter": " | ".join(terms)})
    output = sys.argv[3] if len(sys.argv) > 3 else None
    if output:
        os.makedirs(output, exist_ok=True)
        arg_max = os.sysconf("SC_ARG_MAX")
        limit = arg_max // 4
        for item in result:
            expression = item["filter"]
            if len(expression.encode()) > limit:
                raise SystemExit(f"bin {item['bin']} filter exceeds argv guard {limit}")
            with open(os.path.join(output, f"bin-{item['bin']}.expr"), "w") as file:
                file.write(expression)
    print(json.dumps({"bins": result}, indent=2))
if __name__ == "__main__": main()
