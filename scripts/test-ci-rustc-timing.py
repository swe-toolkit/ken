#!/usr/bin/env python3
"""Behavioral tests for CI rustc timing collection."""

import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
import unittest
from contextlib import redirect_stdout
from io import StringIO

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "ci-rustc-timing.py"
SPEC = importlib.util.spec_from_file_location("ci_rustc_timing", SCRIPT)
TIMING = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TIMING)


FAKE_RUSTC = """#!/usr/bin/env python3
import sys
import time
args = sys.argv[1:]
for arg in args:
    if arg.startswith('--sleep='):
        time.sleep(float(arg.partition('=')[2]))
sys.exit(int(next((arg.partition('=')[2] for arg in args if arg.startswith('--exit=')), '0')))
"""


class RustcTimingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.workspace = self.root / "workspace"
        self.package = self.workspace / "crates" / "ken-demo"
        self.package.mkdir(parents=True)
        (self.package / "Cargo.toml").write_text("[package]\nname='ken-demo'\n")
        self.source = self.package / "src" / "lib.rs"
        self.source.parent.mkdir()
        self.source.write_text("pub fn demo() {}\n")
        self.third_party = self.root / "registry" / "dep-1.0"
        self.third_party.mkdir(parents=True)
        (self.third_party / "Cargo.toml").write_text("[package]\nname='dep'\n")
        self.compiler = self.root / "fake-rustc"
        self.compiler.write_text(FAKE_RUSTC)
        self.compiler.chmod(0o755)
        self.log = self.root / "rustc-timings.jsonl"
        self.env = os.environ.copy()
        self.env.update({
            "CI_RUSTC_TIMING_LOG": str(self.log),
            "CI_RUSTC_TIMING_WORKSPACE": str(self.workspace),
            "GITHUB_WORKSPACE": str(self.workspace),
        })

    def invoke(self, manifest, *args):
        self.env["CARGO_MANIFEST_DIR"] = str(manifest)
        return subprocess.run(
            [sys.executable, str(SCRIPT), str(self.compiler), *args],
            env=self.env,
            check=False,
            capture_output=True,
            text=True,
        )

    def records(self):
        if not self.log.exists():
            return []
        return [json.loads(line) for line in self.log.read_text().splitlines()]

    def test_classifies_external_workspace_and_test_units(self):
        workspace = self.invoke(self.package, "--crate-name", "local_lib", str(self.source))
        external = self.invoke(self.third_party, "--crate-name", "dep_lib", str(self.source))
        test = self.invoke(self.package, "--crate-name", "integration", "--test", str(self.source))
        self.assertEqual([workspace.returncode, external.returncode, test.returncode], [0, 0, 0])
        records = self.records()
        self.assertEqual(
            [row["category"] for row in records],
            ["workspace_crate_chain", "third_party_dependencies", "test_binaries"],
        )
        self.assertEqual(
            [row["package"] for row in records],
            ["crates/ken-demo", "external/dep-1.0", "crates/ken-demo"],
        )

    def test_classifies_workspace_source_when_manifest_env_is_absent(self):
        self.env.pop("CARGO_MANIFEST_DIR", None)
        result = subprocess.run(
            [
                sys.executable,
                str(SCRIPT),
                str(self.compiler),
                "--crate-name",
                "local_lib",
                str(self.source),
            ],
            env=self.env,
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 0)
        record = self.records()[0]
        self.assertEqual(record["category"], "workspace_crate_chain")
        self.assertEqual(record["package"], "crates/ken-demo")

    def test_concurrent_compiler_processes_append_complete_records(self):
        def compile_unit(index):
            return self.invoke(
                self.package,
                "--crate-name",
                f"unit_{index}",
                "--sleep=0.01",
            )

        with ThreadPoolExecutor(max_workers=8) as pool:
            results = list(pool.map(compile_unit, range(8)))
        self.assertEqual([result.returncode for result in results], [0] * 8)
        records = self.records()
        self.assertEqual(len(records), 8)
        self.assertEqual({row["crate_name"] for row in records}, {f"unit_{i}" for i in range(8)})

    def test_propagates_compiler_failure_and_records_it(self):
        result = self.invoke(self.package, "--crate-name", "broken", "--exit=7")
        self.assertEqual(result.returncode, 7)
        row = self.records()[0]
        self.assertEqual(row["return_code"], 7)
        self.assertGreaterEqual(row["end_ns"], row["start_ns"])

    def test_metadata_probe_is_not_counted_as_a_compile_unit(self):
        result = subprocess.run(
            [sys.executable, str(SCRIPT), str(self.compiler), "--version"],
            env=self.env,
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 0)
        self.assertEqual(self.records(), [])

    def test_report_fails_closed_when_no_timing_units_were_recorded(self):
        output = StringIO()
        with redirect_stdout(output):
            result = TIMING.report(str(self.root / "missing.jsonl"), "1")
        self.assertEqual(result, 1)
        self.assertIn("status=missing-timing-categories", output.getvalue())

    def test_report_emits_all_three_categories_and_overlap_safe_wall_time(self):
        intervals = [(0, 10), (5, 15), (20, 25)]
        self.assertEqual(TIMING.interval_union_seconds(intervals), 2.0e-8)
        self.invoke(self.third_party, "--crate-name", "dep", "--sleep=0.01")
        self.invoke(self.package, "--crate-name", "lib", "--sleep=0.01")
        self.invoke(self.package, "--crate-name", "test", "--test", "--sleep=0.01")
        output = StringIO()
        with redirect_stdout(output):
            TIMING.report(str(self.log), "3")
        lines = output.getvalue().splitlines()
        self.assertEqual(len([line for line in lines if " category=" in line]), 3)
        for category in TIMING.CATEGORIES:
            self.assertIn(f"category={category}", output.getvalue())


if __name__ == "__main__":
    unittest.main()
