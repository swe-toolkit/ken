#!/usr/bin/env python3
"""Sample per-job runner disk and memory peaks for CI logs."""

import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import threading
import time
from datetime import datetime, timezone

SAMPLE_INTERVAL_SECONDS = 1.0


def utc_now():
    return datetime.now(timezone.utc).isoformat()


def cgroup_memory_peak():
    candidates = (
        (Path("/sys/fs/cgroup/memory.peak"), "cgroup-v2-peak"),
        (Path("/sys/fs/cgroup/memory.max_usage_in_bytes"), "cgroup-v1-peak"),
    )
    for path, source in candidates:
        try:
            value = int(path.read_text(encoding="ascii").strip())
        except (OSError, ValueError):
            continue
        if value >= 0:
            return value, source
    return None


def cgroup_memory_current():
    candidates = (
        (Path("/sys/fs/cgroup/memory.current"), "cgroup-v2-current-sampled"),
        (
            Path("/sys/fs/cgroup/memory/memory.usage_in_bytes"),
            "cgroup-v1-current-sampled",
        ),
    )
    for path, source in candidates:
        try:
            value = int(path.read_text(encoding="ascii").strip())
        except (OSError, ValueError):
            continue
        if value >= 0:
            return value, source
    return None


def proc_memory_current():
    values = {}
    try:
        for line in Path("/proc/meminfo").read_text(encoding="ascii").splitlines():
            key, _, remainder = line.partition(":")
            if key in {"MemTotal", "MemAvailable"}:
                values[key] = int(remainder.strip().split()[0]) * 1024
    except (OSError, ValueError, IndexError):
        return None
    if "MemTotal" not in values or "MemAvailable" not in values:
        return None
    return max(0, values["MemTotal"] - values["MemAvailable"]), "proc-meminfo-sampled"


def memory_usage():
    peak = cgroup_memory_peak()
    if peak is not None:
        return peak
    current = cgroup_memory_current()
    if current is not None:
        return current
    current = proc_memory_current()
    if current is not None:
        return current
    raise RuntimeError("no readable cgroup or /proc memory measurement")


def take_sample(workspace):
    disk_used = shutil.disk_usage(workspace).used
    memory_used, memory_source = memory_usage()
    return {
        "sampled_at": utc_now(),
        "disk_used_bytes": disk_used,
        "memory_used_bytes": memory_used,
        "memory_source": memory_source,
    }


def write_state(path, state):
    path = Path(path)
    temporary = path.with_name(f"{path.name}.{os.getpid()}.tmp")
    temporary.write_text(json.dumps(state, sort_keys=True), encoding="utf-8")
    os.replace(temporary, path)


def update_peaks(state, sample):
    state["sample_count"] += 1
    state["last_sampled_at"] = sample["sampled_at"]
    if sample["disk_used_bytes"] >= state["peak_disk_used_bytes"]:
        state["peak_disk_used_bytes"] = sample["disk_used_bytes"]
        state["peak_disk_sampled_at"] = sample["sampled_at"]
    if sample["memory_used_bytes"] >= state["peak_memory_used_bytes"]:
        state["peak_memory_used_bytes"] = sample["memory_used_bytes"]
        state["peak_memory_sampled_at"] = sample["sampled_at"]
        state["memory_source"] = sample["memory_source"]


def sample_worker(state_path, workspace):
    stopping = threading.Event()
    signal.signal(signal.SIGTERM, lambda _signum, _frame: stopping.set())
    signal.signal(signal.SIGINT, lambda _signum, _frame: stopping.set())
    state = {
        "started_at": utc_now(),
        "sample_interval_seconds": SAMPLE_INTERVAL_SECONDS,
        "sample_count": 0,
        "peak_disk_used_bytes": 0,
        "peak_memory_used_bytes": 0,
        "peak_disk_sampled_at": None,
        "peak_memory_sampled_at": None,
        "last_sampled_at": None,
        "memory_source": None,
        "stopped": False,
    }
    while True:
        update_peaks(state, take_sample(workspace))
        write_state(state_path, state)
        if stopping.is_set():
            break
        stopping.wait(SAMPLE_INTERVAL_SECONDS)
    state["stopped"] = True
    state["finished_at"] = utc_now()
    write_state(state_path, state)


def monitor_paths():
    runner_temp = os.environ.get("RUNNER_TEMP")
    if not runner_temp:
        raise RuntimeError("RUNNER_TEMP is required")
    directory = Path(runner_temp)
    directory.mkdir(parents=True, exist_ok=True)
    return (
        directory / "ci-resource-monitor.json",
        directory / "ci-resource-monitor.pid",
        directory / "ci-resource-monitor.log",
    )


def start_monitor():
    workspace = os.environ.get("GITHUB_WORKSPACE", os.getcwd())
    state_path, pid_path, log_path = monitor_paths()
    if pid_path.exists():
        raise RuntimeError(f"resource monitor already started: {pid_path}")
    state_path.unlink(missing_ok=True)
    with log_path.open("ab") as log:
        process = subprocess.Popen(
            [sys.executable, os.path.abspath(__file__), "_sample", str(state_path), workspace],
            stdin=subprocess.DEVNULL,
            stdout=log,
            stderr=subprocess.STDOUT,
            start_new_session=True,
        )
    pid_path.write_text(str(process.pid), encoding="ascii")
    deadline = time.monotonic() + 5.0
    while time.monotonic() < deadline:
        if state_path.exists():
            state = json.loads(state_path.read_text(encoding="utf-8"))
            if state.get("sample_count", 0) > 0:
                print(
                    "CI_RESOURCE_MONITOR event=start "
                    f"job={os.environ.get('GITHUB_JOB', 'local')} "
                    f"interval_seconds={SAMPLE_INTERVAL_SECONDS:g}"
                )
                return
        return_code = process.poll()
        if return_code is not None:
            log_text = log_path.read_text(encoding="utf-8", errors="replace")
            raise RuntimeError(
                f"resource monitor exited during startup ({return_code}): {log_text}"
            )
        time.sleep(0.05)
    raise RuntimeError("resource monitor did not produce an initial sample")


def finish_monitor():
    state_path, pid_path, _log_path = monitor_paths()
    if not pid_path.exists():
        raise RuntimeError("resource monitor PID record is missing")
    pid = int(pid_path.read_text(encoding="ascii").strip())
    try:
        os.kill(pid, signal.SIGTERM)
    except ProcessLookupError:
        pass

    deadline = time.monotonic() + 5.0
    state = None
    while time.monotonic() < deadline:
        if state_path.exists():
            state = json.loads(state_path.read_text(encoding="utf-8"))
            if state.get("stopped"):
                break
        time.sleep(0.05)
    if state is None or not state.get("stopped"):
        raise RuntimeError("resource monitor did not stop cleanly")

    mib = 1024 * 1024
    gib = 1024 * 1024 * 1024
    print(
        "CI_RESOURCE_PEAK "
        f"job={os.environ.get('GITHUB_JOB', 'local')} "
        f"samples={state['sample_count']} "
        f"interval_seconds={state['sample_interval_seconds']:g} "
        f"peak_disk_used_bytes={state['peak_disk_used_bytes']} "
        f"peak_disk_used_gib={state['peak_disk_used_bytes'] / gib:.3f} "
        f"peak_memory_used_bytes={state['peak_memory_used_bytes']} "
        f"peak_memory_used_mib={state['peak_memory_used_bytes'] / mib:.1f} "
        f"memory_source={state['memory_source']} "
        f"disk_peak_sampled_at={state['peak_disk_sampled_at']} "
        f"memory_peak_sampled_at={state['peak_memory_sampled_at']}"
    )
    pid_path.unlink(missing_ok=True)


def main(argv):
    if argv == ["start"]:
        start_monitor()
        return 0
    if argv == ["finish"]:
        finish_monitor()
        return 0
    if len(argv) == 3 and argv[0] == "_sample":
        sample_worker(argv[1], argv[2])
        return 0
    print(f"usage: {Path(sys.argv[0]).name} start|finish", file=sys.stderr)
    return 2


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (OSError, RuntimeError, ValueError, json.JSONDecodeError) as error:
        print(f"ci-resource-monitor: {error}", file=sys.stderr)
        raise SystemExit(1)
