#!/usr/bin/env python3
"""Assign a live nextest JSON inventory to deterministic duration-balanced bins."""
import json, sys, statistics, heapq, os
from pathlib import Path

N = 8

def tests(value):
    # nextest's binary-id is its authoritative filter identity. Display fields
    # are deliberately not reconstructed into a substitute key.
    suites = value.get("rust-suites")
    if not isinstance(suites, dict) or not suites:
        raise SystemExit("nextest listing has no non-empty rust-suites map")
    seen = set()
    for suite in suites.values():
        if not isinstance(suite, dict):
            raise SystemExit("nextest listing contains a malformed rust suite")
        binary_id = suite.get("binary-id")
        testcases = suite.get("testcases")
        if not isinstance(binary_id, str) or not binary_id:
            raise SystemExit("nextest rust suite has no non-empty binary-id")
        if not isinstance(testcases, dict) or not testcases:
            raise SystemExit("nextest rust suite has no non-empty testcases")
        for name in testcases:
            if not isinstance(name, str) or not name:
                raise SystemExit("nextest rust suite has an invalid testcase")
            identity = (binary_id, name)
            if identity in seen:
                raise SystemExit(f"duplicate canonical identity: {binary_id} {name}")
            seen.add(identity)
            yield identity

def main():
    inventory = json.load(open(sys.argv[1]))
    evidence = json.load(open(sys.argv[2]))
    durations = {r["test_id"]: r["seconds"] for r in evidence["records"]}
    median = statistics.median(durations.values())
    bins = [(0.0, index, []) for index in range(N)]
    heapq.heapify(bins)
    live = sorted(
        (f"{binary_id} {name}", binary_id, name)
        for binary_id, name in tests(inventory)
    )
    for rendered, binary_id, name in sorted(
        live, key=lambda x: (-durations.get(x[0], median), x[0])
    ):
        total, index, selected = heapq.heappop(bins)
        selected.append((binary_id, name))
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
