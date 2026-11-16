#!/usr/bin/env python3
"""Run nextest on a tiny isolated crate to prove exact Z3 exclusions.

This CI-only fixture has the same package/binary id and test path as one real
excluded test, plus a suffix sibling. It compiles and runs under nextest: an
excluded test must not execute, while its suffix sibling must execute.
"""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

import ci_workspace_timings

SCRIPT = Path(__file__).with_name("ci-z3-lib-scope.py")
spec = importlib.util.spec_from_file_location("ci_z3_lib_scope", SCRIPT)
scope = importlib.util.module_from_spec(spec)
spec.loader.exec_module(scope)

NAME = "ac7_whole_catalog_is_parse_preserved_idempotent_and_width_bounded"
PREFIX = "r_layer_tests::kenfmt_b3_layout::"
assert PREFIX + NAME in scope.EXCLUDED
SUFFIX = PREFIX + NAME + "_suffix_sibling"


def check_fixture_listings(full_listing, selected_listing):
    """Check this two-test fixture, not the real inventory's four-name floor."""
    discovered, full_selected = ci_workspace_timings.listed_identities(full_listing)
    selected_discovered, selected = ci_workspace_timings.listed_identities(selected_listing)
    excluded = f"{scope.BINARY_ID} {PREFIX}{NAME}"
    sibling = f"{scope.BINARY_ID} {SUFFIX}"
    if (discovered != {excluded, sibling} or selected_discovered != discovered
            or full_selected != discovered or selected != {sibling}):
        raise AssertionError(
            f"nextest exact filter lost suffix sibling or retained exclusion: {selected}"
        )


def stub_listing_check():
    """Local control for the fixture's inventory contract without nextest."""
    def listing(matches):
        return {
            "test-count": 2,
            "rust-suites": {"lib": {
                "binary-id": scope.BINARY_ID,
                "testcases": {
                    name: {"filter-match": {"status": "matches" if name in matches else "mismatch"}}
                    for name in (PREFIX + NAME, SUFFIX)
                },
            }},
        }

    full = listing({PREFIX + NAME, SUFFIX})
    check_fixture_listings(full, listing({SUFFIX}))
    try:
        check_fixture_listings(full, listing(set()))  # substring skip loses sibling
    except AssertionError:
        pass
    else:
        raise AssertionError("substring skip did not red the fixture inventory check")
    print("Z3 fixture stub: exact skip retains suffix; substring skip reddens")


def run():
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        (root / "src").mkdir()
        (root / "Cargo.toml").write_text(
            '[package]\nname = "ken-elaborator"\nversion = "0.0.0"\nedition = "2021"\n[workspace]\n'
        )
        (root / "src" / "lib.rs").write_text(f'''
#[cfg(test)] mod r_layer_tests {{
    mod kenfmt_b3_layout {{
        #[test] fn {NAME}() {{
            std::fs::write(std::env::var("CI_Z3_FILTER_PROBE_MARKER").unwrap(), "excluded").unwrap();
        }}
        #[test] fn {NAME}_suffix_sibling() {{
            std::fs::write(std::env::var("CI_Z3_FILTER_PROBE_MARKER").unwrap(), "sibling").unwrap();
        }}
    }}
}}
''')
        manifest = str(root / "Cargo.toml")
        marker = root / "executed.txt"
        env = {**os.environ, "CI_Z3_FILTER_PROBE_MARKER": str(marker)}
        base = ["cargo", "nextest"]
        filter_expr = scope.filter_expression()
        args = ["--manifest-path", manifest, "--lib"]
        full = subprocess.run(base + ["list", *args, "--message-format", "json"],
                              check=True, text=True, capture_output=True, env=env)
        filtered = subprocess.run(base + ["list", *args, "--message-format", "json", "-E", filter_expr],
                                  check=True, text=True, capture_output=True, env=env)
        full_listing = json.loads(full.stdout)
        selected_listing = json.loads(filtered.stdout)
        check_fixture_listings(full_listing, selected_listing)
        # The four-live-exclusion floor is checked by ci-z3-lib-scope.py on
        # the real lib inventory in this job; this tiny crate holds only one.
        subprocess.run(base + ["run", *args, "-E", filter_expr], check=True, env=env)
        if marker.read_text() != "sibling":
            raise AssertionError("suffix sibling did not execute exclusively")
        print("Z3 exact filter probe: measured exclusion skipped, suffix sibling executed")


if __name__ == "__main__":
    if sys.argv[1:] == ["--stub-listing"]:
        stub_listing_check()
    elif len(sys.argv) == 1:
        run()
    else:
        raise SystemExit("usage: test-ci-z3-lib-filter.py [--stub-listing]")
