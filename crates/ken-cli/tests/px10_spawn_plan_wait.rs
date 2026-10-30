//! PX10 interpreter-first child resource smoke. Promise: durable invariant.
//! MEASURED: declared Process reaches the shared host dispatcher in the
//! interpreter, completes a child bracket and waits for normal exit.
//! CLAIMED: a public plan can execute under the declared Process grant.
//! THE GAP: this smoke uses null stdio; AC-1 pins exact pipe bytes separately.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

const SMOKE: &str = include_str!("fixtures/px10_child_wait_smoke.ken");
const ARGV_ENV_PIPE: &str = include_str!("fixtures/px10_argv_env_pipe.ken");
const LIVE_CHILD: &str = include_str!("fixtures/px10_live_child.ken");

#[test]
fn declared_process_smoke_executes_and_reaps_in_interpreter() {
    let root = tempfile::tempdir().expect("isolated interpreter root");
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let observed = ken_cli::run_program_effect_observation(
        SMOKE,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("checked Process program executes in the interpreter");
    assert_eq!(observed.exit_status, 0, "{:?}", observed.effect_trace);
    assert_eq!(observed.terminal_error, None);
    assert_eq!(
        observed
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        [
            ken_host::HostOpV1::ProcessPlanOpen,
            ken_host::HostOpV1::ProcessPlanPush,
            ken_host::HostOpV1::ProcessPlanPush,
            ken_host::HostOpV1::ProcessPlanPush,
            ken_host::HostOpV1::ProcessPlanPush,
            ken_host::HostOpV1::ProcessSpawn,
            ken_host::HostOpV1::ProcessWait,
            ken_host::HostOpV1::ResourceRelease,
        ],
    );
    assert!(matches!(
        observed.effect_trace[6].outcome,
        ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ChildExit(
            ken_host::ChildExitV1::Exited(0)
        ))
    ));
}

/// Promise: durable invariant. MEASURED: a native build of the same checked
/// Process source refuses with an identified unavailable operation before an
/// artifact exists. CLAIMED: native Process never silently falls through to
/// the unfinished bracket planner. THE GAP: a scratch build with the successor
/// return-fold repair must yield this same refusal, not a changed planner text.
/// THE GAP: the gate keys on the declared grant, so a program declaring
/// Process without using it is also conservatively refused.
#[test]
fn native_process_op_is_unavailable_before_object_emission() {
    use ken_elaborator::compiler_driver::NativeProgramBuildError;

    let root = tempfile::tempdir().expect("isolated native output root");
    let error = ken_cli::build_native_program(
        SMOKE,
        ken_cli::SourceFormat::Ken,
        "px10_spawn_plan_wait_smoke",
        root.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect_err("Process must not produce a partial native artifact");
    let NativeProgramBuildError::Unavailable(lane) = error else {
        panic!("expected typed host-op availability refusal, found: {error:?}");
    };
    assert_eq!(lane.lane, "host_effect_lowering_unavailable");
    assert!(lane.reason.contains("ProcessPlanOpen"), "{lane:?}");
    assert!(lane.reason.contains("RepresentedUnavailable"), "{lane:?}");
    assert_eq!(
        std::fs::read_dir(root.path()).expect("output root").count(),
        0
    );
}

/// Promise class: durable invariant; `Signaled 15` is the specified UInt8
/// compatibility value. MEASURED: checked Ken `withChild`/`wait_child` returns
/// the typed signal constructor after the fixture sends SIGTERM to itself.
/// CLAIMED: signal termination is not converted to an ordinary exit status.
/// THE GAP: only SIGTERM is measured; other signal numbers remain under the
/// same host decode and are not asserted by this test.
#[test]
fn ac1_checked_interpreter_wait_reports_signaled_sigterm() {
    let source = SMOKE
        .replace(
            "bytes_encode \"/bin/true\"",
            &format!(
                "bytes_encode \"{}\"",
                env!("CARGO_BIN_EXE_px10-child-probe")
            ),
        )
        .replace(
            "(Cons Bytes (bytes_encode \"caller-zero\") (Nil Bytes))",
            "(Cons Bytes (bytes_encode \"caller-zero\") \
             (Cons Bytes (bytes_encode \"sigterm\") (Nil Bytes)))",
        );
    let root = tempfile::tempdir().expect("isolated interpreter root");
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let observed = ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("checked child SIGTERM program executes");
    assert_eq!(observed.exit_status, 0, "{:?}", observed.effect_trace);
    let exits = observed
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_host::HostOpV1::ProcessWait)
        .map(|event| &event.outcome)
        .collect::<Vec<_>>();
    assert_eq!(exits.len(), 1, "one real wait: {exits:?}");
    assert!(
        matches!(
            exits[0],
            ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ChildExit(
                ken_host::ChildExitV1::Signaled(15)
            ))
        ),
        "signal is a distinct typed result: {exits:?}"
    );
}

/// Promise class: durable invariant. MEASURED: the checked interpreter reads
/// the child's stdout pipe into its owned buffer, freezes every read span,
/// reaches EOF, and returns `Exited 0`; the concatenated frozen bytes equal
/// the child's argv/env record. CLAIMED: stdout, exact argv[0]/argv[2],
/// replacement environment and absence of parent overlay survive spawn.
/// THE GAP: the fixture's record is one small write; read chunk sizes are not
/// asserted and other argv/environment byte shapes need separate controls.
#[test]
fn ac1_checked_interpreter_stdout_pipe_preserves_argv_and_environment() {
    let source = ARGV_ENV_PIPE.replace(
        "PX10_CHILD_PROBE_PATH",
        env!("CARGO_BIN_EXE_px10-child-probe"),
    );
    let root = tempfile::tempdir().expect("isolated interpreter root");
    let old = std::env::var_os("PX10_PARENT_ONLY");
    std::env::set_var("PX10_PARENT_ONLY", "parent-sentinel");
    // The new nested child/buffer fixture overflowed the harness 2 MiB
    // stack. Its completed 16 MiB run measured 3292 KiB resident stack
    // (thread smaps); the stated 16 MiB leaves more than 12 MiB headroom.
    let result = std::thread::Builder::new()
        .name("px10-ac1-stdout-pipe".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(move || {
            let mut host = ken_interp::PosixHost::new_at(root.path());
            ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &[],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .map_err(|error| format!("{error:?}"))
        })
        .expect("stated interpreter fixture stack")
        .join()
        .expect("interpreter fixture thread completes");
    if let Some(value) = old {
        std::env::set_var("PX10_PARENT_ONLY", value);
    } else {
        std::env::remove_var("PX10_PARENT_ONLY");
    }
    let observed = result.expect("checked stdout-pipe program executes");
    assert_eq!(observed.exit_status, 0, "{:?}", observed.effect_trace);
    assert_eq!(observed.terminal_error, None);
    let freezes = observed
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_host::HostOpV1::BufferFreeze)
        .map(|event| match &event.outcome {
            ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::Bytes(bytes)) => {
                bytes.as_slice()
            }
            other => panic!("spanBytes failed after pipe read: {other:?}"),
        })
        .collect::<Vec<_>>();
    assert!(!freezes.is_empty(), "at least one read span must be frozen");
    let output = freezes.concat();
    assert_eq!(output,
        b"argv0=caller-zero\nargv1=payload\nselected=chosen\nreplaced=<unset>\nparent-only=<unset>\n"
    );
    let reads = observed
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_host::HostOpV1::ResourceStreamRead)
        .map(|event| &event.outcome)
        .collect::<Vec<_>>();
    assert!(
        reads.iter().any(|outcome| matches!(outcome,
        ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ReadProgress(
            ken_host::ReadProgressV1::ReadSome { transferred, .. }
        )) if transferred.get() > 0)),
        "pipe must transfer bytes: {reads:?}"
    );
    assert!(
        reads.iter().any(|outcome| matches!(
            outcome,
            ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ReadProgress(
                ken_host::ReadProgressV1::ReadEof
            ))
        )),
        "pipe must reach EOF: {reads:?}"
    );
    let waits = observed
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_host::HostOpV1::ProcessWait)
        .map(|event| &event.outcome)
        .collect::<Vec<_>>();
    assert_eq!(waits.len(), 1);
    assert!(
        matches!(
            waits[0],
            ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ChildExit(
                ken_host::ChildExitV1::Exited(0)
            ))
        ),
        "normal child exit: {waits:?}"
    );
}

struct NonCloexecFd9 {
    previous: Option<std::os::fd::OwnedFd>,
}

impl NonCloexecFd9 {
    fn install() -> Self {
        use std::os::fd::{AsRawFd, FromRawFd};
        // SAFETY: if fd 9 is already open, this creates one independently
        // owned duplicate. EBADF means that fd 9 was initially unused.
        let backup = unsafe { libc::fcntl(9, libc::F_DUPFD_CLOEXEC, 30) };
        let previous = if backup == -1 {
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::EBADF)
            );
            None
        } else {
            Some(unsafe { std::os::fd::OwnedFd::from_raw_fd(backup) })
        };
        let source = std::fs::File::open("/dev/null").expect("fd 9 control source");
        let high = unsafe { libc::fcntl(source.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 20) };
        assert!(high >= 20, "isolated high descriptor for fd 9");
        drop(source);
        let high = unsafe { std::os::fd::OwnedFd::from_raw_fd(high) };
        assert_eq!(unsafe { libc::dup2(high.as_raw_fd(), 9) }, 9);
        assert_eq!(
            unsafe { libc::fcntl(9, libc::F_GETFD) } & libc::FD_CLOEXEC,
            0
        );
        Self { previous }
    }
}

impl Drop for NonCloexecFd9 {
    fn drop(&mut self) {
        use std::os::fd::AsRawFd;
        // SAFETY: restore the original fd 9, or close the one we installed.
        if let Some(previous) = &self.previous {
            assert_eq!(unsafe { libc::dup2(previous.as_raw_fd(), 9) }, 9);
        } else {
            assert_eq!(unsafe { libc::close(9) }, 0);
        }
    }
}

extern "C" fn caught_usr2(_signal: libc::c_int) {}

struct SignalBoundary {
    old_pipe: libc::sigaction,
    old_hup: libc::sigaction,
    old_usr2: libc::sigaction,
    old_mask: libc::sigset_t,
}

impl SignalBoundary {
    fn install() -> Self {
        let old_pipe = Self::set(libc::SIGPIPE, libc::SIG_IGN);
        let old_hup = Self::set(libc::SIGHUP, libc::SIG_IGN);
        let old_usr2 = Self::set(libc::SIGUSR2, caught_usr2 as libc::sighandler_t);
        let mut old_mask = unsafe { std::mem::zeroed() };
        let mut blocked = unsafe { std::mem::zeroed() };
        // SAFETY: both sets are writable, and the mask changes only the
        // spawning thread. The old mask is restored before its thread exits.
        unsafe {
            assert_eq!(libc::sigemptyset(&mut blocked), 0);
            assert_eq!(libc::sigaddset(&mut blocked, libc::SIGUSR1), 0);
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_SETMASK, &blocked, &mut old_mask),
                0
            );
        }
        Self {
            old_pipe,
            old_hup,
            old_usr2,
            old_mask,
        }
    }

    fn set(signal: libc::c_int, disposition: libc::sighandler_t) -> libc::sigaction {
        let mut new_action: libc::sigaction = unsafe { std::mem::zeroed() };
        new_action.sa_sigaction = disposition;
        let mut previous = unsafe { std::mem::zeroed() };
        // SAFETY: the actions are initialized and valid for this process.
        unsafe {
            assert_eq!(libc::sigemptyset(&mut new_action.sa_mask), 0);
            assert_eq!(libc::sigaction(signal, &new_action, &mut previous), 0);
        }
        previous
    }
}

impl Drop for SignalBoundary {
    fn drop(&mut self) {
        // SAFETY: restore all original actions and the spawning-thread mask.
        unsafe {
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_SETMASK, &self.old_mask, std::ptr::null_mut()),
                0
            );
            assert_eq!(
                libc::sigaction(libc::SIGUSR2, &self.old_usr2, std::ptr::null_mut()),
                0
            );
            assert_eq!(
                libc::sigaction(libc::SIGHUP, &self.old_hup, std::ptr::null_mut()),
                0
            );
            assert_eq!(
                libc::sigaction(libc::SIGPIPE, &self.old_pipe, std::ptr::null_mut()),
                0
            );
        }
    }
}

fn signal_bits(status: &str, name: &str) -> u64 {
    let line = status
        .lines()
        .find(|line| line.starts_with(name))
        .unwrap_or_else(|| panic!("missing {name} in proc status"));
    u64::from_str_radix(line.split_whitespace().nth(1).expect("signal hex mask"), 16)
        .expect("signal hex mask parses")
}

fn signal_bit(signal: libc::c_int) -> u64 {
    1u64 << (signal - 1)
}

struct HeldChild(libc::pid_t);
impl Drop for HeldChild {
    fn drop(&mut self) {
        // SAFETY: the pid comes from the live child's out-of-band readiness
        // record. A completed process may already have exited by teardown.
        let rc = unsafe { libc::kill(self.0, libc::SIGTERM) };
        assert!(rc == 0 || std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH));
    }
}

/// Promise class: durable invariant. MEASURED: after verified non-CLOEXEC fd
/// 9 and independently verified spawning-thread signal actions/mask, a live
/// checked `withChild` process holds only fds 0–2, has default SIGPIPE,
/// inherited ignored SIGHUP, reset caught SIGUSR2, and the exact inherited
/// SIGUSR1-only mask while its Ken body blocks on wait. CLAIMED: spawning
/// closes every unselected descriptor and preserves the inherited signal
/// boundary without clearing the mask. THE GAP: this observes one controlled
/// shell child on Linux; the two independent production-side mutations below
/// test the fd and SIGPIPE directions of this same live-child pin.
#[test]
fn ac2_checked_interpreter_live_child_fd_and_signal_boundary() {
    use std::os::unix::ffi::OsStrExt;
    use std::time::{Duration, Instant};

    let root = tempfile::tempdir().expect("isolated interpreter root");
    let ready = root.path().join("px10-child-pid");
    let ready_path = ready.to_str().expect("UTF-8 temp path");
    assert!(
        !ready_path.contains(['"', '$', ' ', '\n']),
        "safe fixture path"
    );
    let source = LIVE_CHILD.replace("PX10_READY_FILE", ready_path);
    let _fd9 = NonCloexecFd9::install();
    let (sender, receiver) = std::sync::mpsc::channel();
    // Same nested child/buffer shape as AC-1: 3292 KiB resident on the
    // completed AC-1 run, provisioned at 16 MiB with >12 MiB headroom.
    let runner = std::thread::Builder::new()
        .name("px10-ac2-spawning-thread".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(move || {
            let _signals = SignalBoundary::install();
            let tid = unsafe { libc::syscall(libc::SYS_gettid) } as libc::pid_t;
            sender.send(tid).expect("publish spawning thread tid");
            let mut host = ken_interp::PosixHost::new_at(root.path());
            ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &[],
                &[],
                root.path().as_os_str().as_bytes(),
                &mut host,
            )
            .map_err(|error| format!("{error:?}"))
        })
        .expect("stated interpreter fixture stack");
    let tid = receiver
        .recv_timeout(Duration::from_secs(10))
        .expect("spawn worker establishes signals before Ken executes");
    let deadline = Instant::now() + Duration::from_secs(30);
    let pid = loop {
        if let Ok(text) = std::fs::read_to_string(&ready) {
            break text.parse::<libc::pid_t>().expect("child pid record");
        }
        assert!(
            Instant::now() < deadline,
            "checked child must publish readiness"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    let child_guard = HeldChild(pid);
    let parent_status = std::fs::read_to_string(format!("/proc/self/task/{tid}/status"))
        .expect("spawning-thread control status");
    let child_status = std::fs::read_to_string(format!("/proc/{pid}/status"))
        .expect("child remains live for signal inspection");
    let mut fds = std::fs::read_dir(format!("/proc/{pid}/fd"))
        .expect("child remains live for fd inspection")
        .map(|entry| {
            entry
                .expect("fd entry")
                .file_name()
                .to_str()
                .unwrap()
                .parse::<i32>()
                .expect("fd number")
        })
        .collect::<Vec<_>>();
    fds.sort_unstable();
    drop(child_guard);
    let observed = runner
        .join()
        .expect("interpreter worker completes")
        .expect("checked child wait returns after SIGTERM");
    assert_eq!(observed.exit_status, 0, "{:?}", observed.effect_trace);
    assert!(
        observed.effect_trace.iter().any(|event| matches!(
            (&event.operation, &event.outcome),
            (ken_host::HostOpV1::ResourceStreamRead,
                ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ReadProgress(
                    ken_host::ReadProgressV1::ReadSome { transferred, .. }
                ))) if transferred.get() > 0
        )),
        "Ken reads the live child's ready pipe before waiting"
    );
    assert!(
        observed.effect_trace.iter().any(|event| matches!(
            (&event.operation, &event.outcome),
            (
                ken_host::HostOpV1::ProcessWait,
                ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ChildExit(
                    ken_host::ChildExitV1::Signaled(15)
                ))
            )
        )),
        "Ken waits for the terminated live child"
    );

    assert_eq!(
        unsafe { libc::fcntl(9, libc::F_GETFD) } & libc::FD_CLOEXEC,
        0,
        "the spawning process retained the non-CLOEXEC fd 9 precondition"
    );
    assert_eq!(
        fds,
        [0, 1, 2],
        "unselected parent fd 9 must not reach the child"
    );
    let parent_ignored = signal_bits(&parent_status, "SigIgn:");
    let parent_caught = signal_bits(&parent_status, "SigCgt:");
    let parent_mask = signal_bits(&parent_status, "SigBlk:");
    assert_ne!(
        parent_ignored & signal_bit(libc::SIGPIPE),
        0,
        "parent ignores SIGPIPE"
    );
    assert_ne!(
        parent_ignored & signal_bit(libc::SIGHUP),
        0,
        "parent ignores SIGHUP"
    );
    assert_ne!(
        parent_caught & signal_bit(libc::SIGUSR2),
        0,
        "parent catches SIGUSR2"
    );
    assert_eq!(
        parent_mask,
        signal_bit(libc::SIGUSR1),
        "spawning-thread mask control"
    );
    let child_ignored = signal_bits(&child_status, "SigIgn:");
    let child_caught = signal_bits(&child_status, "SigCgt:");
    assert_eq!(
        child_ignored & signal_bit(libc::SIGPIPE),
        0,
        "child SIGPIPE must be default, not ignored"
    );
    assert_eq!(
        child_caught & signal_bit(libc::SIGPIPE),
        0,
        "child SIGPIPE must be default, not caught"
    );
    assert_ne!(
        child_ignored & signal_bit(libc::SIGHUP),
        0,
        "other ignored dispositions must remain inherited"
    );
    assert_eq!(
        child_caught & signal_bit(libc::SIGUSR2),
        0,
        "caught SIGUSR2 must reset on exec"
    );
    assert_eq!(
        signal_bits(&child_status, "SigBlk:"),
        parent_mask,
        "the child must inherit the spawning thread's exact signal mask"
    );
}
