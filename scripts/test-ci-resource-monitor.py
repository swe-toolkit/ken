#!/usr/bin/env python3
"""Behavioral tests for CI job resource sampling."""

import importlib.util
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "ci-resource-monitor.py"
SPEC = importlib.util.spec_from_file_location("ci_resource_monitor", SCRIPT)
MONITOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MONITOR)


class ResourceMonitorTests(unittest.TestCase):
    def test_sample_measures_filesystem_and_memory(self):
        with tempfile.TemporaryDirectory() as directory:
            sample = MONITOR.take_sample(directory)
        self.assertGreaterEqual(sample["disk_used_bytes"], 0)
        self.assertGreater(sample["memory_used_bytes"], 0)
        self.assertTrue(sample["memory_source"])

    def test_peak_state_keeps_independent_high_water_marks(self):
        state = {
            "sample_count": 0,
            "peak_disk_used_bytes": 0,
            "peak_memory_used_bytes": 0,
            "peak_disk_sampled_at": None,
            "peak_memory_sampled_at": None,
            "memory_source": None,
        }
        first = {
            "sampled_at": "t1",
            "disk_used_bytes": 100,
            "memory_used_bytes": 200,
            "memory_source": "sample-a",
        }
        second = {
            "sampled_at": "t2",
            "disk_used_bytes": 90,
            "memory_used_bytes": 250,
            "memory_source": "sample-b",
        }
        MONITOR.update_peaks(state, first)
        MONITOR.update_peaks(state, second)
        self.assertEqual(state["sample_count"], 2)
        self.assertEqual(state["peak_disk_used_bytes"], 100)
        self.assertEqual(state["peak_disk_sampled_at"], "t1")
        self.assertEqual(state["peak_memory_used_bytes"], 250)
        self.assertEqual(state["peak_memory_sampled_at"], "t2")

    def test_start_finish_lifecycle_emits_peak_record(self):
        with tempfile.TemporaryDirectory() as directory:
            env = os.environ.copy()
            env.update({
                "RUNNER_TEMP": directory,
                "GITHUB_WORKSPACE": directory,
                "GITHUB_JOB": "resource-monitor-test",
            })
            started = subprocess.run(
                [sys.executable, str(SCRIPT), "start"],
                env=env,
                capture_output=True,
                text=True,
                check=True,
            )
            (Path(directory) / "sample-data").write_bytes(b"x" * (64 * 1024))
            time.sleep(1.05)
            finished = subprocess.run(
                [sys.executable, str(SCRIPT), "finish"],
                env=env,
                capture_output=True,
                text=True,
                check=True,
            )
            self.assertIn("CI_RESOURCE_MONITOR event=start", started.stdout)
            self.assertIn("CI_RESOURCE_PEAK", finished.stdout)
            self.assertIn("peak_disk_used_bytes=", finished.stdout)
            self.assertIn("peak_memory_used_mib=", finished.stdout)
            self.assertIn("samples=", finished.stdout)


if __name__ == "__main__":
    unittest.main()
