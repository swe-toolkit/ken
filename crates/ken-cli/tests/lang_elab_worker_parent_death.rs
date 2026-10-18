//! A caller's death stops its re-exec worker and any later host effects.
//! Promise class: durable CLI lifecycle acceptance, parent-death ruling
//! evt_1bv82pwc0dca6 and startup correction evt_3m6y438gmkd8v.

#![cfg(target_os = "linux")]

use std::fs::File;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

const FAULT_MARKER: &str = "KEN_CLI_WORKER_CHILD_FAULT_FD";
const EFFECT: &[u8] = b"effect after supervisor kill\n";
// Adversary R2 observed the effect at about 15 s on the debug base; a fixed
// 30 s deadline gives 15 s of headroom and still pins an effect after death.
const EFFECT_DEADLINE: Duration = Duration::from_secs(30);
const GONE_DEADLINE: Duration = Duration::from_secs(2);

fn deep_program(with_main: bool) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("deep program directory");
    let path = dir.path().join("deep.ken");
    let depth = 4000;
    let mut source = String::with_capacity(6 * depth + 400);
    if with_main {
        source.push_str("program capabilities FS APartial\n");
    }
    source.push_str("const deep : Nat = ");
    for _ in 0..depth {
        source.push_str("Suc (");
    }
    source.push_str("Zero");
    for _ in 0..depth {
        source.push(')');
    }
    source.push('\n');
    if with_main {
        source.push_str(
            "proc main (_input : ProcessInput) (_caps : ProgramCaps APartial) : HostIO APartial ExitCode visits [Console] = host_program APartial (print_line \"effect after supervisor kill\")\n",
        );
    }
    std::fs::write(&path, source).expect("write deep program");
    (dir, path)
}

// The shell execs the binary, so the observed pid is the CLI parent pid. Its
// main thread is stated at 8 MiB; the worker is independently stated at
// 256 MiB. Removing RUST_MIN_STACK and the private marker controls ambient
// stack policy and prevents the old env-spoofing path from biasing this test.
fn ken_command() -> Command {
    let mut ken = Command::new("sh");
    ken.arg("-c")
        .arg("ulimit -c 0; ulimit -s 8192 || exit 77; exec \"$@\"")
        .arg("ken-worker-parent-death")
        .arg(env!("CARGO_BIN_EXE_ken"))
        .env_remove("RUST_MIN_STACK")
        .env_remove(FAULT_MARKER);
    ken
}

fn signal(pid: u32, sig: libc::c_int) {
    // SAFETY: kill(2) accepts a positive pid and the named POSIX signal.
    assert_eq!(
        unsafe { libc::kill(pid as libc::pid_t, sig) },
        0,
        "kill {pid}"
    );
}

fn process_gone(pid: u32) -> bool {
    let path = format!("/proc/{pid}/stat");
    match std::fs::read_to_string(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
        Ok(stat) => {
            stat.rsplit_once(") ")
                .and_then(|(_, rest)| rest.chars().next())
                == Some('Z')
        }
        Err(error) => panic!("read child proc stat: {error}"),
    }
}

fn child_pid(parent: &mut Child) -> u32 {
    let pid = parent.id();
    let children = format!("/proc/{pid}/task/{pid}/children");
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if let Ok(text) = std::fs::read_to_string(&children) {
            if let Some(child) = text.split_whitespace().next() {
                return child.parse::<u32>().expect("child pid");
            }
        }
        if let Some(status) = parent.try_wait().expect("check parent") {
            panic!("parent exited before spawning its worker: {status:?}");
        }
        assert!(
            Instant::now() < deadline,
            "child did not appear in {children}"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn wait_gone(pid: u32, limit: Duration) -> bool {
    let deadline = Instant::now() + limit;
    while Instant::now() < deadline {
        if process_gone(pid) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    process_gone(pid)
}

fn wait_parent(parent: &mut Child) -> ExitStatus {
    let deadline = Instant::now() + Duration::from_secs(45);
    loop {
        if let Some(status) = parent.try_wait().expect("wait for CLI parent") {
            return status;
        }
        if Instant::now() >= deadline {
            let _ = parent.kill();
            let _ = parent.wait();
            panic!("CLI parent exceeded 45 s bound");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn stop_straggler(pid: u32) {
    if !process_gone(pid) {
        // A base or mutation failure leaves an orphan; clean it before
        // reporting the red instead of leaking a running Ken into CI.
        unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
        assert!(wait_gone(pid, GONE_DEADLINE), "straggler {pid} not reaped");
    }
}

fn start_parent(command: &str, path: &Path, output: &Path) -> Child {
    let stdout = File::create(output).expect("effect output file");
    ken_command()
        .arg(command)
        .arg(path)
        .stdin(Stdio::null())
        .stdout(stdout)
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn Ken CLI parent")
}

/// MEASURED: normal deep run returns 0 and emits the exact main effect.
/// CLAIMED: the new lifeline does not kill an active parent's worker.
/// THE GAP: it says nothing about a dead parent; P0-P3 cover that side.
#[test]
fn normal_run_preserves_output_and_status() {
    let (_dir, path) = deep_program(true);
    let output = ken_command()
        .arg("run")
        .arg(path)
        .output()
        .expect("normal deep run");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(output.stdout, EFFECT);
}

/// MEASURED: after SIGKILL to the CLI pid alone, its child is gone within
/// two seconds and the delayed effect is still absent at the 30-s deadline.
/// CLAIMED: parent death closes the sole lifeline writer in steady state.
/// THE GAP: P0 separately pins death before child-role authentication.
#[test]
fn parent_kill_stops_run_child_and_delayed_effect() {
    let (_dir, path) = deep_program(true);
    let output = tempfile::tempdir().expect("effect observation directory");
    let effect_path = output.path().join("effect.out");
    let mut parent = start_parent("run", &path, &effect_path);
    let child = child_pid(&mut parent);
    signal(parent.id(), libc::SIGKILL);
    let status = wait_parent(&mut parent);
    let gone = wait_gone(child, GONE_DEADLINE);
    std::thread::sleep(EFFECT_DEADLINE);
    let effect = std::fs::read(&effect_path).expect("read delayed effect output");
    stop_straggler(child);
    assert_eq!(status.signal(), Some(libc::SIGKILL), "{status:?}");
    assert!(
        effect.is_empty(),
        "effect after parent SIGKILL: {effect:?}; child_gone_within_2s={gone}"
    );
    assert!(gone, "child {child} persisted after parent SIGKILL");
}

/// MEASURED: a parent SIGTERM is observed as TERM by its caller; its child
/// is gone within two seconds. CLAIMED: the pipe covers catchable signals
/// without forwarding them. THE GAP: P1 pins the delayed-effect absence.
#[test]
fn parent_term_stops_run_child_without_forwarding() {
    let (_dir, path) = deep_program(true);
    let output = tempfile::tempdir().expect("effect observation directory");
    let mut parent = start_parent("run", &path, &output.path().join("term.out"));
    let child = child_pid(&mut parent);
    signal(parent.id(), libc::SIGTERM);
    let status = wait_parent(&mut parent);
    let gone = wait_gone(child, GONE_DEADLINE);
    stop_straggler(child);
    assert_eq!(status.signal(), Some(libc::SIGTERM), "{status:?}");
    assert!(gone, "child {child} persisted after parent SIGTERM");
}

/// MEASURED: a long-running check child is gone within two seconds of the
/// parent SIGKILL. CLAIMED: no elaborating child accumulates after timeout.
/// THE GAP: P1 separately tests an actual Ken host effect under run.
#[test]
fn parent_kill_stops_check_child() {
    let (_dir, path) = deep_program(false);
    let output = tempfile::tempdir().expect("check observation directory");
    let mut parent = start_parent("check", &path, &output.path().join("check.out"));
    let child = child_pid(&mut parent);
    signal(parent.id(), libc::SIGKILL);
    let status = wait_parent(&mut parent);
    let gone = wait_gone(child, GONE_DEADLINE);
    stop_straggler(child);
    assert_eq!(status.signal(), Some(libc::SIGKILL), "{status:?}");
    assert!(gone, "check child {child} persisted after parent SIGKILL");
}

fn pipe_pair() -> (File, File) {
    let mut fds = [0 as libc::c_int; 2];
    // SAFETY: pipe fills the two integer slots, each owned by one File below.
    assert_eq!(
        unsafe { libc::pipe(fds.as_mut_ptr()) },
        0,
        "create test pipe"
    );
    unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
}

fn identity(pipe: &File) -> (u64, u64) {
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: fstat writes only to this local stat object on success.
    assert_eq!(unsafe { libc::fstat(pipe.as_raw_fd(), &mut stat) }, 0);
    assert_eq!(stat.st_mode & libc::S_IFMT, libc::S_IFIFO);
    (stat.st_dev as u64, stat.st_ino as u64)
}

/// MEASURED: a direct child holding both correctly identified pipes, but
/// naming a reaped and hence foreign parent, dies by SIGKILL before any
/// worker output or delayed effect. CLAIMED: parent death before role() is
/// closed by pipe identity, not a live getppid test. THE GAP: this constructs
/// the post-death state; P1-P3 observe ordinary parent death in flight.
#[test]
fn already_dead_parent_before_child_role_kills_worker() {
    let (_dir, path) = deep_program(true);
    let output = tempfile::tempdir().expect("post-death observation directory");
    let effect_path = output.path().join("effect.out");
    let stderr_path = output.path().join("stderr.out");
    let (fault_reader, fault_writer) = pipe_pair();
    let (lifeline_reader, lifeline_writer) = pipe_pair();
    let (fault_dev, fault_ino) = identity(&fault_writer);
    let (life_dev, life_ino) = identity(&lifeline_reader);
    // SAFETY: set FD_CLOEXEC on the sole writer before the child spawns.
    assert_ne!(
        unsafe { libc::fcntl(lifeline_writer.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC) },
        -1,
        "mark lifeline writer CLOEXEC"
    );
    drop(lifeline_writer); // The parent is already dead when the child starts.
    let mut unrelated = Command::new("true")
        .spawn()
        .expect("spawn unrelated parent");
    let reaped_pid = unrelated.id();
    assert!(unrelated.wait().expect("reap unrelated parent").success());
    assert_ne!(reaped_pid, std::process::id(), "pid is not this harness");
    let marker = format!(
        "{}:{reaped_pid}:{fault_dev}:{fault_ino}:{}:{life_dev}:{life_ino}",
        fault_writer.as_raw_fd(),
        lifeline_reader.as_raw_fd()
    );
    let effect_file = File::create(&effect_path).expect("P0 effect output");
    let stderr_file = File::create(&stderr_path).expect("P0 stderr output");
    let mut child = ken_command()
        .arg("run")
        .arg(path)
        .env(FAULT_MARKER, marker)
        .stdout(effect_file)
        .stderr(stderr_file)
        .spawn()
        .expect("spawn worker with already-dead parent marker");
    let status = wait_parent(&mut child);
    let effect = std::fs::read(&effect_path).expect("P0 effect file");
    let stderr = std::fs::read(&stderr_path).expect("P0 stderr file");
    drop((fault_reader, fault_writer, lifeline_reader));
    assert!(effect.is_empty(), "effect after parent death: {effect:?}");
    assert!(
        stderr.is_empty(),
        "worker stderr after parent death: {stderr:?}"
    );
    assert_eq!(status.signal(), Some(libc::SIGKILL), "{status:?}");
}
