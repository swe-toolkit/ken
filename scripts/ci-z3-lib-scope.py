#!/usr/bin/env python3
"""Exact lib-test exclusions for the feature-on Z3 job.

`filter` emits a nextest expression. `check FULL SELECTED` verifies that the
actual nextest selection (not merely the expression's text) differs from the
live lib-test inventory by exactly the four measured no-route identities.
"""
import json
import sys

import ci_workspace_timings

BINARY_ID = "ken-elaborator"
EXCLUDED = frozenset({
    "r_layer_tests::lang_mod_catalog_evidence_frontier::catalog_evidence_frontier_is_closed_and_honest",
    "r_layer_tests::lang_mod_catalog_evidence_frontier::population_pin_rejects_omission_replacement_and_duplicate",
    "r_layer_tests::kenfmt_b3_layout::ac7_whole_catalog_is_parse_preserved_idempotent_and_width_bounded",
    "r_layer_tests::kenfmt_b4_splicing::ac6_whole_literate_corpus_preserves_prose_ast_and_idempotence_read_only",
})


def filter_expression():
    tests = " | ".join(f"test(={name})" for name in sorted(EXCLUDED))
    return f"not (binary_id(={BINARY_ID}) & ({tests}))"


def check(full, selected):
    all_full, live = ci_workspace_timings.listed_identities(full)
    all_selected, actual = ci_workspace_timings.listed_identities(selected)
    expected_excluded = {f"{BINARY_ID} {name}" for name in EXCLUDED}
    if all_full != all_selected:
        raise ValueError("nextest filtered and unfiltered lib inventories differ")
    if not expected_excluded <= live:
        raise ValueError("measured Z3 exclusions missing from the lib inventory")
    expected = live - expected_excluded
    if actual != expected:
        missing = sorted(expected - actual)
        unexpected = sorted(actual - expected)
        raise ValueError(
            f"Z3 lib selection differs from exact exclusions: missing={missing}, unexpected={unexpected}"
        )
    if not actual:
        raise ValueError("Z3 lib selection is empty")
    return len(actual)


def main(argv):
    if argv[1:] == ["filter"]:
        print(filter_expression())
        return 0
    if len(argv) == 4 and argv[1] == "check":
        try:
            with open(argv[2], encoding="utf-8") as source:
                full = json.load(source)
            with open(argv[3], encoding="utf-8") as source:
                selected = json.load(source)
            count = check(full, selected)
        except (OSError, ValueError, ci_workspace_timings.TimingArtifactError) as error:
            print(f"Z3 lib selection failed: {error}", file=sys.stderr)
            return 2
        print(f"Z3 lib selection verified: {count} selected, {len(EXCLUDED)} exact exclusions")
        return 0
    print("usage: ci-z3-lib-scope.py filter | check FULL_JSON SELECTED_JSON", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
