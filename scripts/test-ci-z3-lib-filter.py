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
        discovered, full_selected = ci_workspace_timings.listed_identities(full_listing)
        _, selected = ci_workspace_timings.listed_identities(selected_listing)
        excluded = f"{scope.BINARY_ID} {PREFIX}{NAME}"
        sibling = f"{scope.BINARY_ID} {SUFFIX}"
        if discovered != {excluded, sibling} or full_selected != {excluded, sibling} or selected != {sibling}:
            raise AssertionError(f"nextest exact filter lost suffix sibling or retained exclusion: {selected}")
        scope.check(full_listing, selected_listing)
        subprocess.run(base + ["run", *args, "-E", filter_expr], check=True, env=env)
        if marker.read_text() != "sibling":
            raise AssertionError("suffix sibling did not execute exclusively")
        print("Z3 exact filter probe: measured exclusion skipped, suffix sibling executed")


if __name__ == "__main__":
    run()
