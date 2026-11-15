use super::*;
use std::io::{BufRead, BufReader, Read};
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};

#[test]
fn spawn_plan_passes_exact_argv_and_env_and_reports_exit() {
    let mut plan = SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
    plan.push_argument(b"caller-zero".to_vec()).unwrap();
    plan.push_argument(b"-c".to_vec()).unwrap();
    plan.push_argument(
        b"printf '%s:%s:%s' \"$0\" \"${PX10_SELECTED-<unset>}\" \"${HOME-<unset>}\"".to_vec(),
    )
    .unwrap();
    plan.push_environment(b"PX10_SELECTED".to_vec(), b"chosen".to_vec())
        .unwrap();
    plan.set_stdio(1, ChildStdioV1::Pipe).unwrap();
    let mut child = plan.spawn().unwrap();
    let mut output = Vec::new();
    std::fs::File::from(child.take_pipe(1).unwrap())
        .read_to_end(&mut output)
        .unwrap();
    assert_eq!(output, b"caller-zero:chosen:<unset>");
    assert_eq!(child.wait().unwrap(), ChildExitV1::Exited(0));
    assert_eq!(child.wait().unwrap(), ChildExitV1::Exited(0));
    assert_eq!(
        child.send_signal(libc::SIGKILL).unwrap_err().raw_os_error(),
        Some(libc::ESRCH)
    );
    assert!(child.take_pipe(1).is_err());
    child.settle(false).unwrap();
}

#[test]
fn spawn_plan_closes_unselected_parent_fds_and_restores_sigpipe() {
    let parent_fd = std::fs::File::open("/dev/null").unwrap();
    // SAFETY: fcntl duplicates to a new descriptor at least 100 without CLOEXEC.
    let inherited = unsafe { libc::fcntl(parent_fd.as_raw_fd(), libc::F_DUPFD, 100) };
    assert!(inherited >= 100);
    let mut plan = SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
    plan.push_argument(b"caller-zero".to_vec()).unwrap();
    plan.push_argument(b"-c".to_vec()).unwrap();
    plan.push_argument(b"printf 'ready\\n'; read input || :".to_vec())
        .unwrap();
    plan.set_stdio(0, ChildStdioV1::Pipe).unwrap();
    plan.set_stdio(1, ChildStdioV1::Pipe).unwrap();
    let mut child = plan.spawn().unwrap();
    let pid = child.pid();
    // Promise class: durable invariant. The held descriptor denotes the
    // process posix_spawn returned, independently of the recorded PID.
    let held_fd = match &child.identity {
        ChildIdentityV1::Held(fd) => fd.as_raw_fd(),
        ChildIdentityV1::ReapedAtSpawn => panic!("blocked child was auto-reaped"),
    };
    let info = std::fs::read_to_string(format!("/proc/self/fdinfo/{held_fd}")).unwrap();
    let held_pid = info
        .lines()
        .find_map(|line| line.strip_prefix("Pid:").map(str::trim))
        .unwrap()
        .parse::<libc::pid_t>()
        .unwrap();
    assert_eq!(held_pid, pid);
    let mut out = std::fs::File::from(child.take_pipe(1).unwrap());
    let mut ready = [0; 6];
    out.read_exact(&mut ready).unwrap();
    assert_eq!(&ready, b"ready\n");
    let mut fds = std::fs::read_dir(format!("/proc/{pid}/fd"))
        .unwrap()
        .map(|fd| {
            fd.unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    fds.sort_unstable();
    assert_eq!(fds, [0, 1, 2]);
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).unwrap();
    let ignored = status
        .lines()
        .find(|line| line.starts_with("SigIgn:"))
        .unwrap();
    let bits = u64::from_str_radix(ignored.split_whitespace().nth(1).unwrap(), 16).unwrap();
    assert_eq!(bits & (1 << (libc::SIGPIPE - 1)), 0);
    // SAFETY: the inherited descriptor is uniquely owned here.
    unsafe { libc::close(inherited) };
    drop(child.take_pipe(0));
    assert_eq!(child.wait().unwrap(), ChildExitV1::Exited(0));
    child.settle(false).unwrap();
}

#[test]
fn spawn_plan_rejects_missing_program_and_malformed_inputs() {
    let mut missing = SpawnPlanV1::new(b"/path/does-not-exist-px10".to_vec()).unwrap();
    missing.push_argument(b"caller-zero".to_vec()).unwrap();
    assert_eq!(
        missing.spawn().unwrap_err().raw_os_error(),
        Some(libc::ENOENT)
    );
    let mut empty = SpawnPlanV1::new(b"/bin/true".to_vec()).unwrap();
    assert_eq!(
        empty.clone().spawn().unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(
        empty.push_argument(b"x\0y".to_vec()).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(
        empty
            .push_environment(b"BAD=KEY".to_vec(), vec![])
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(
        empty.set_stdio(3, ChildStdioV1::Null).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    empty.set_stdio(0, ChildStdioV1::Null).unwrap();
    assert_eq!(
        empty.set_stdio(0, ChildStdioV1::Pipe).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
}

fn shell_plan(script: &str) -> SpawnPlanV1 {
    let mut plan = SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
    plan.push_argument(b"sh".to_vec()).unwrap();
    plan.push_argument(b"-c".to_vec()).unwrap();
    plan.push_argument(script.as_bytes().to_vec()).unwrap();
    plan
}

fn with_pidfd_fault<T>(step: PidfdStep, errno: i32, run: impl FnOnce() -> T) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            PIDFD_FAULT.with(|fault| fault.set(None));
        }
    }
    PIDFD_FAULT.with(|fault| {
        assert!(fault.get().is_none());
        fault.set(Some((step, errno)));
    });
    let _restore = Restore;
    run()
}

struct Decoy {
    shell: std::process::Child,
    pid: libc::pid_t,
    fd: OwnedFd,
}

impl Decoy {
    fn new() -> Self {
        // sleep is a grandchild, not a child of this test process. Its
        // parent shell reaps it, and we hold a pidfd for safe test cleanup.
        let mut shell = Command::new("/bin/sh")
            .arg("-c")
            .arg("sleep 30 >/dev/null 2>&1 & echo $!; wait")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut pid_line = String::new();
        BufReader::new(shell.stdout.take().unwrap())
            .read_line(&mut pid_line)
            .unwrap();
        let pid = pid_line.trim().parse::<libc::pid_t>().unwrap();
        let fd = pidfd_open(pid).unwrap();
        // The explicit ECHILD control makes a waitpid(PID) mutation
        // discriminate at once, without ever reaping a child of this test.
        assert_eq!(
            unsafe { libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG) },
            -1
        );
        assert_eq!(
            io::Error::last_os_error().raw_os_error(),
            Some(libc::ECHILD)
        );
        Self { shell, pid, fd }
    }

    fn assert_running(&self) {
        let mut pollfd = libc::pollfd {
            fd: self.fd.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        assert_eq!(unsafe { libc::poll(&mut pollfd, 1, 0) }, 0, "decoy exited");
    }
}

impl Drop for Decoy {
    fn drop(&mut self) {
        let _ = pidfd_send_signal(self.fd.as_raw_fd(), libc::SIGKILL);
        let _ = self.shell.wait();
    }
}

/// Promise class: durable invariant. The terminal-settlement kill and reap
/// address the held child even when the recorded PID belongs to a non-child.
#[test]
fn terminal_settle_kills_and_reaps_held_child_not_spoofed_decoy() {
    let decoy = Decoy::new();
    let mut child = shell_plan("exec sleep 4").spawn().unwrap();
    child.pid = decoy.pid;
    assert_eq!(child.settle_exit(true).unwrap(), ChildExitV1::Signaled(9));
    decoy.assert_running();
}

/// Promise class: durable invariant. An unavailable pidfd fails before
/// posix_spawn; the same missing executable reaches ENOENT with a real probe.
#[test]
fn pidfd_probe_unavailable_refuses_before_spawning() {
    let mut missing = SpawnPlanV1::new(b"/path/does-not-exist-pidfd".to_vec()).unwrap();
    missing.push_argument(b"not-executed".to_vec()).unwrap();
    LAST_MINT_PID.with(|last| last.set(None));
    let error = with_pidfd_fault(PidfdStep::Probe, libc::ENOSYS, || {
        missing.clone().spawn().unwrap_err()
    });
    assert_eq!(error.kind(), io::ErrorKind::Unsupported);
    assert_eq!(
        crate::effect_v1::io_error_identity_v1(&error),
        crate::IoErrorIdentityV1::Unsupported
    );
    LAST_MINT_PID.with(|last| assert_eq!(last.get(), None));
    assert_eq!(
        missing.spawn().unwrap_err().raw_os_error(),
        Some(libc::ENOENT)
    );
}

/// Promise class: durable invariant. A child whose mint reports ESRCH keeps
/// its body pipes and yields ECHILD; terminal settlement sends nothing.
#[test]
fn pidfd_mint_esrch_keeps_body_and_does_not_signal() {
    let decoy = Decoy::new();
    let mut plan = shell_plan("printf 'child-ran'");
    plan.set_stdio(1, ChildStdioV1::Pipe).unwrap();
    let mut child = with_pidfd_fault(PidfdStep::Mint, libc::ESRCH, || plan.spawn().unwrap());
    assert!(matches!(child.identity, ChildIdentityV1::ReapedAtSpawn));
    let real_pid = child.pid();
    let mut body = Vec::new();
    std::fs::File::from(child.take_pipe(1).unwrap())
        .read_to_end(&mut body)
        .unwrap();
    assert_eq!(body, b"child-ran");
    assert_eq!(child.wait().unwrap_err().raw_os_error(), Some(libc::ECHILD));
    child.pid = decoy.pid;
    assert_eq!(
        child.settle_exit(true).unwrap_err().raw_os_error(),
        Some(libc::ECHILD)
    );
    decoy.assert_running();
    // The test seam simulates ESRCH; reap the real child that actually ran.
    reap_unheld_child(real_pid);
}

/// Promise class: durable invariant. A mint failure under ordinary SIGCHLD
/// kills and reaps its child before the original errno is returned.
#[test]
fn pidfd_mint_emfile_cleans_up_child_before_error() {
    assert_eq!(sigchld_auto_reaps().unwrap(), false);
    LAST_MINT_PID.with(|last| last.set(None));
    let error = with_pidfd_fault(PidfdStep::Mint, libc::EMFILE, || {
        shell_plan("exec sleep 30").spawn().unwrap_err()
    });
    assert_eq!(error.raw_os_error(), Some(libc::EMFILE));
    let pid = LAST_MINT_PID.with(|last| last.get().unwrap());
    assert_eq!(
        unsafe { libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG) },
        -1
    );
    assert_eq!(
        io::Error::last_os_error().raw_os_error(),
        Some(libc::ECHILD)
    );
}

/// Promise class: durable invariant. Only the re-executed test process may
/// change SIGCHLD; a held child auto-reaps there, and settlement after exit
/// cannot signal an unrelated process or turn ECHILD into a false success.
#[test]
fn ignored_sigchld_auto_reaps_held_child_without_a_pid_kill() {
    const CHILD: &str = "KEN_PIDFD_AUTOREAP_TEST_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let output = Command::new(std::env::current_exe().unwrap())
            .arg("--exact")
            .arg("process_v1::tests::ignored_sigchld_auto_reaps_held_child_without_a_pid_kill")
            .arg("--nocapture")
            .env(CHILD, "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
        return;
    }
    // SAFETY: this subprocess runs exactly one test; other test-process
    // threads and their child waits cannot observe its SIGCHLD disposition.
    assert_ne!(
        unsafe { libc::signal(libc::SIGCHLD, libc::SIG_IGN) },
        libc::SIG_ERR
    );
    let mut child = shell_plan("exec sleep 1").spawn().unwrap();
    let held = match &child.identity {
        ChildIdentityV1::Held(fd) => fd.as_raw_fd(),
        ChildIdentityV1::ReapedAtSpawn => panic!("child must be held before it exits"),
    };
    let mut pollfd = libc::pollfd {
        fd: held,
        events: libc::POLLIN,
        revents: 0,
    };
    assert_eq!(unsafe { libc::poll(&mut pollfd, 1, 5000) }, 1);
    assert_eq!(child.wait().unwrap_err().raw_os_error(), Some(libc::ECHILD));
    assert_eq!(
        child.settle_exit(true).unwrap_err().raw_os_error(),
        Some(libc::ECHILD)
    );
    // Under auto-reap, an injected mint failure must not attempt a PID kill.
    // Hold the still-running child with a second pidfd for safe cleanup.
    let mint_error = with_pidfd_fault(PidfdStep::Mint, libc::EMFILE, || {
        shell_plan("exec sleep 30").spawn().unwrap_err()
    });
    assert_eq!(mint_error.raw_os_error(), Some(libc::EMFILE));
    let pid = LAST_MINT_PID.with(|last| last.get().unwrap());
    let held = pidfd_open(pid).expect("auto-reap mint failure left child running");
    let mut running = libc::pollfd {
        fd: held.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    assert_eq!(unsafe { libc::poll(&mut running, 1, 0) }, 0);
    pidfd_send_signal(held.as_raw_fd(), libc::SIGKILL).unwrap();
}
