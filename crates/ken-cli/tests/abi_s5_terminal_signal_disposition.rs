//! ABI-S5, spec 38 §1.10; normative compatibility vectors for two executable
//! paths. The oracle reads the live process, not Rust source or a declared set.
#![cfg(target_os = "linux")]

use std::fs;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const BLOCKED_READER: &str = include_str!("fixtures/abi_s5_block_on_stdin.ken");
const RESOURCE_PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn ken_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_ken"))
}

// An inherited ignore must not satisfy the test's expected ignore. Establish
// the same clean starting state for both paths, regardless of the CI runner.
fn launch_clean(command: &mut Command) -> io::Result<Child> {
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // SAFETY: only async-signal-safe libc calls run after fork and before exec;
    // this callback touches no Rust allocation or synchronization state.
    unsafe {
        command.pre_exec(|| {
            // Linux's /proc signal masks cover signals 1..=64. Reset all
            // catchable dispositions, including inherited real-time ignores;
            // glibc reserves 32 and 33 and refuses changes to their actions.
            for number in 1..=64 {
                if number == libc::SIGKILL
                    || number == libc::SIGSTOP
                    || number == 32
                    || number == 33
                {
                    continue;
                }
                if libc::signal(number, libc::SIG_DFL) == libc::SIG_ERR {
                    return Err(io::Error::last_os_error());
                }
            }
            let mut empty: libc::sigset_t = std::mem::zeroed();
            if libc::sigemptyset(&mut empty) == -1
                || libc::sigprocmask(libc::SIG_SETMASK, &empty, std::ptr::null_mut()) == -1
            {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
        command.spawn()
    }
}

fn process_children(pid: u32) -> Vec<u32> {
    let path = format!("/proc/{pid}/task/{pid}/children");
    fs::read_to_string(path)
        .unwrap_or_default()
        .split_whitespace()
        .map(|word| word.parse().expect("kernel child pid"))
        .collect()
}

fn task_ids(pid: u32) -> io::Result<Vec<u32>> {
    fs::read_dir(format!("/proc/{pid}/task"))?
        .map(|entry| {
            entry?
                .file_name()
                .to_str()
                .expect("numeric task id")
                .parse()
                .map_err(io::Error::other)
        })
        .collect()
}

fn blocked_in_stdin_read(pid: u32) -> bool {
    let Ok(tasks) = task_ids(pid) else {
        return false;
    };
    tasks.into_iter().any(|tid| {
        fs::read_to_string(format!("/proc/{pid}/task/{tid}/syscall"))
            .ok()
            .is_some_and(|syscall| syscall.split_whitespace().take(2).eq(["0", "0x0"]))
    })
}

fn wait_until_read(parent: &mut Child, worker: bool) -> u32 {
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let candidates = if worker {
            process_children(parent.id())
        } else {
            vec![parent.id()]
        };
        for pid in candidates {
            if blocked_in_stdin_read(pid) {
                return pid;
            }
        }
        if let Some(status) = parent.try_wait().expect("poll live Ken process") {
            panic!("Ken exited before the stdin read: {status}");
        }
        if Instant::now() >= deadline {
            parent.kill().expect("kill timed-out blocked-read probe");
            panic!("Ken did not reach a live read(0) within 120 seconds");
        }
        thread::sleep(Duration::from_millis(25));
    }
}

fn status_mask(pid: u32, task: Option<u32>, field: &str) -> u64 {
    let path = task.map_or_else(
        || format!("/proc/{pid}/status"),
        |tid| format!("/proc/{pid}/task/{tid}/status"),
    );
    let status = fs::read_to_string(path).expect("live process status");
    let line = status
        .lines()
        .find(|line| line.starts_with(&format!("{field}:")))
        .expect("signal field in live status");
    u64::from_str_radix(line.split_once(':').unwrap().1.trim(), 16).expect("kernel signal-mask hex")
}

fn signal_bit(number: libc::c_int) -> u64 {
    1u64 << (number - 1)
}

fn assert_live_posture(pid: u32, worker: bool) {
    const LOW: u64 = (1u64 << 31) - 1;
    let ignored = status_mask(pid, None, "SigIgn");
    let caught = status_mask(pid, None, "SigCgt");
    assert_eq!(
        ignored & LOW,
        signal_bit(libc::SIGPIPE),
        "pid {pid}: ignored"
    );
    assert_eq!(ignored & !LOW, 0, "pid {pid}: high ignored signals");
    let expected_caught = if worker {
        signal_bit(libc::SIGSEGV) | signal_bit(libc::SIGBUS)
    } else {
        0
    };
    assert_eq!(caught & LOW, expected_caught, "pid {pid}: caught");
    // glibc reserves 32 and 33; these are not Ken-established dispositions.
    let reserved = signal_bit(32) | signal_bit(33);
    assert_eq!(
        caught & !LOW & !reserved,
        0,
        "pid {pid}: high caught signals"
    );
    let threads = task_ids(pid).expect("live task directory");
    assert!(
        !threads.is_empty(),
        "observed process must have live threads"
    );
    for tid in threads {
        assert_eq!(
            status_mask(pid, Some(tid), "SigBlk"),
            0,
            "pid {pid}, tid {tid}: mask"
        );
    }
}

fn assert_reader_exits(mut child: Child) {
    drop(child.stdin.take());
    let output = child.wait_with_output().expect("wait after closing stdin");
    assert_eq!(
        output.status.code(),
        Some(41),
        "read Ok arm must execute; status: {:?}, stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn fixture(dir: &Path) -> PathBuf {
    let source = dir.join("blocked-reader.ken");
    fs::write(&source, BLOCKED_READER).expect("write checked Ken reader");
    source
}

#[test]
fn ken_run_worker_signal_posture_is_exact() {
    let dir = tempfile::tempdir().expect("isolated Ken source");
    let source = fixture(dir.path());
    let mut command = Command::new(ken_bin());
    command.arg("run").arg(source);
    let mut parent = launch_clean(&mut command).expect("launch clean CLI parent");
    let worker = wait_until_read(&mut parent, true);
    assert_ne!(
        worker,
        parent.id(),
        "the program runs in the worker, not the CLI parent"
    );
    assert_live_posture(worker, true);
    assert_live_posture(parent.id(), true);
    assert_reader_exits(parent);
}

#[test]
fn native_artifact_signal_posture_is_exact() {
    let dir = tempfile::tempdir().expect("isolated native build");
    let source = fixture(dir.path());
    let profile = dir.path().join("resource-profile.json");
    fs::write(&profile, RESOURCE_PROFILE).expect("write native resource profile");
    let artifact_dir = dir.path().join("artifact");
    let output = Command::new(ken_bin())
        .arg("native-build")
        .arg(&source)
        .arg(&artifact_dir)
        .arg(&profile)
        .output()
        .expect("build C-started artifact");
    assert_eq!(
        output.status.code(),
        Some(0),
        "build stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let executable = String::from_utf8(output.stdout).expect("artifact path is UTF-8");
    let executable = Path::new(executable.trim());
    assert!(executable.is_file(), "native artifact must exist");
    let mut command = Command::new(executable);
    let mut child = launch_clean(&mut command).expect("launch clean native artifact");
    let artifact_pid = wait_until_read(&mut child, false);
    assert_eq!(artifact_pid, child.id());
    assert_live_posture(artifact_pid, false);
    assert_reader_exits(child);
}
