#![cfg(feature = "z3-process")]

use std::{
    fs,
    io::{ErrorKind, Read, Write},
    os::{
        fd::AsRawFd,
        unix::{net::UnixStream, process::CommandExt},
    },
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use ken_elaborator::{
    attempt_d_with_z3_process, attempt_obligation,
    error::Span,
    extract::{ObligationId, ObligationTriple, ProvKind, Provenance},
    prover::Verdict,
    ElabEnv, Z3ProcessConfig,
};
use ken_kernel::Term;
use num_bigint::BigInt;
use tempfile::TempDir;

const STARTUP_SAFE_STUB_TIMEOUT: Duration = Duration::from_secs(5);
const STUB_WRITER_READY_TIMEOUT: Duration = Duration::from_secs(5);
// Forking also copies unrelated test pipes until exec; keep this control's
// pause below the installed solver's two-second deadline.
const FORCED_FORK_HOLD: Duration = Duration::from_millis(500);
// The delayed stub sleeps for one second and emits a valid refuting model;
// this shorter deadline makes enforced timeout the only Unknown outcome.
const DELIBERATE_TIMEOUT_PROBE: Duration = Duration::from_millis(100);
const STUB_WRITER_SCRIPT: &str = r#"
import os, sys

temporary_path, path, contents, ready_path = sys.argv[1:]
with open(temporary_path, "wb") as stub:
    stub.write(contents.encode("utf-8"))
    os.fchmod(stub.fileno(), 0o755)
    with open(ready_path, "wb") as ready:
        ready.write(b"open")
    if sys.stdin.readline().strip() != "publish":
        raise RuntimeError("parent did not release stub writer")
os.replace(temporary_path, path)
"#;

extern "C" {
    #[link_name = "write"]
    fn raw_write(fd: i32, buf: *const u8, count: usize) -> isize;
}

fn equality(elab: &mut ElabEnv) -> ObligationTriple {
    let int_ty = Term::const_(elab.numeric_env.int_id, vec![]);
    let goal = Term::pi(
        int_ty.clone(),
        Term::Eq(
            Box::new(int_ty),
            Box::new(Term::var(0)),
            Box::new(Term::IntLit(BigInt::from(0))),
        ),
    );
    ObligationTriple {
        id: ObligationId("z3.process".into()),
        hole_id: elab.env.fresh_id(),
        context: vec![],
        phi: goal.clone(),
        goal_closed: goal,
        provenance: Provenance {
            kind: ProvKind::Prove,
            span: Span::zero(),
        },
    }
}

fn two_binder_equality(elab: &mut ElabEnv) -> ObligationTriple {
    let int_ty = Term::const_(elab.numeric_env.int_id, vec![]);
    let goal = Term::pi(
        int_ty.clone(),
        Term::pi(
            int_ty.clone(),
            Term::Eq(
                Box::new(int_ty),
                Box::new(Term::var(1)),
                Box::new(Term::var(0)),
            ),
        ),
    );
    ObligationTriple {
        id: ObligationId("z3.process.two-binder".into()),
        hole_id: elab.env.fresh_id(),
        context: vec![],
        phi: goal.clone(),
        goal_closed: goal,
        provenance: Provenance {
            kind: ProvKind::Prove,
            span: Span::zero(),
        },
    }
}

fn write_executable_stub(dir: &TempDir, name: &str, contents: &str) -> std::path::PathBuf {
    write_executable_stub_inner(dir, name, contents, None)
}

fn write_executable_stub_inner(
    dir: &TempDir,
    name: &str,
    contents: &str,
    mut during_open: Option<&mut dyn FnMut()>,
) -> std::path::PathBuf {
    let path = dir.path().join(name);
    // Keep writable descriptors out of this multi-threaded test process: a
    // sibling's fork would otherwise inherit the stub inode before rename.
    let temporary_path = dir.path().join(format!("{name}.tmp"));
    let ready_path = dir.path().join(format!("{name}.writer-ready"));
    if let Err(error) = fs::remove_file(&ready_path) {
        assert_eq!(error.kind(), ErrorKind::NotFound);
    }

    let mut writer = Command::new("/usr/bin/python3")
        .args(["-c", STUB_WRITER_SCRIPT])
        .arg(&temporary_path)
        .arg(&path)
        .arg(contents)
        .arg(&ready_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn isolated stub writer");
    let mut writer_stdin = writer.stdin.take().expect("stub writer stdin");

    let started = Instant::now();
    while !ready_path.exists() {
        assert!(
            started.elapsed() < STUB_WRITER_READY_TIMEOUT,
            "isolated stub writer did not open its temporary file"
        );
        thread::sleep(Duration::from_millis(1));
    }

    let hook_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if let Some(hook) = during_open.as_mut() {
            (*hook)();
        }
    }));
    writer_stdin
        .write_all(b"publish\n")
        .expect("release isolated stub writer");
    drop(writer_stdin);
    let output = writer
        .wait_with_output()
        .expect("wait for isolated stub writer");
    assert!(
        output.status.success(),
        "isolated stub writer failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    fs::remove_file(&ready_path).expect("remove stub-writer ready marker");
    if let Err(panic) = hook_result {
        std::panic::resume_unwind(panic);
    }
    path
}

fn spawn_fork_blocker() -> thread::JoinHandle<()> {
    let (mut parent_signal, child_signal) = UnixStream::pair().expect("fork marker pipe");
    let signal_fd = child_signal.as_raw_fd();
    let blocker = thread::spawn(move || {
        let _keep_signal_fd_open = child_signal;
        let mut command = Command::new("/bin/true");
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        unsafe {
            command.pre_exec(move || {
                let marker = b"F";
                let written = raw_write(signal_fd, marker.as_ptr(), marker.len());
                if written != marker.len() as isize {
                    return Err(std::io::Error::last_os_error());
                }
                thread::sleep(FORCED_FORK_HOLD);
                Ok(())
            });
        }
        let mut child = command.spawn().expect("spawn fork-inheritance blocker");
        assert!(
            child
                .wait()
                .expect("wait for fork-inheritance blocker")
                .success(),
            "fork-inheritance blocker failed"
        );
    });
    let mut marker = [0];
    parent_signal
        .read_exact(&mut marker)
        .expect("blocker reached pre-exec after fork");
    blocker
}

fn stub(dir: &TempDir, body: &str) -> Z3ProcessConfig {
    let path = write_executable_stub(
        dir,
        "z3-stub",
        &format!("#!/bin/sh\ncat >/dev/null\n{body}\n"),
    );
    Z3ProcessConfig {
        program: path,
        timeout: STARTUP_SAFE_STUB_TIMEOUT,
    }
}

fn delayed_valid_stub(dir: &TempDir) -> Z3ProcessConfig {
    let path = write_executable_stub(
        dir,
        "z3-delayed-stub",
        "#!/usr/bin/python3\nimport sys, time\nsys.stdin.read()\ntime.sleep(1)\nprint('sat')\nprint('((k0 1))')\n",
    );
    Z3ProcessConfig {
        program: path,
        timeout: DELIBERATE_TIMEOUT_PROBE,
    }
}

fn assert_unknown(config: Z3ProcessConfig) {
    let mut elab = ElabEnv::new().expect("numeric environment");
    let obligation = equality(&mut elab);
    let before = elab.env.trusted_base().len();
    let verdict = attempt_d_with_z3_process(&mut elab.env, &obligation, &config);
    let Verdict::Unknown { hole_id } = verdict else {
        panic!("solver failure must use the Unknown baseline");
    };
    let after = elab.env.trusted_base();
    assert_eq!(after.len(), before + 1);
    assert!(after.contains(&hole_id));
}

/// Promise class: durable process-isolation invariant.
///
/// MEASURED: while the stub writer subprocess holds its temporary file open, a
/// sibling child forks and pauses before exec; after publication, the adapter
/// reaches Disproved. CLAIMED: that fork cannot inherit the writer descriptor.
/// THE GAP: the writer subprocess, rather than the test process, owns the fd.
#[test]
fn sibling_fork_during_stub_write_does_not_inherit_writer_descriptor() {
    let dir = TempDir::new().expect("stub directory");
    let mut elab = ElabEnv::new().expect("numeric environment");
    let obligation = equality(&mut elab);
    let contents = "#!/bin/sh\ncat >/dev/null\nprintf 'sat\\n((k0 1))\\n'\n";
    let mut blocker = None;
    let path = {
        let mut fork_while_open = || blocker = Some(spawn_fork_blocker());
        write_executable_stub_inner(&dir, "z3-fork-stub", contents, Some(&mut fork_while_open))
    };
    let blocker = blocker.expect("fork blocker started in the writer-open window");
    let config = Z3ProcessConfig {
        program: path.clone(),
        timeout: STARTUP_SAFE_STUB_TIMEOUT,
    };
    let before = elab.env.trusted_base().len();
    let started = Instant::now();
    let verdict = attempt_d_with_z3_process(&mut elab.env, &obligation, &config);
    let adapter_elapsed = started.elapsed();
    let direct_probe = Command::new(&path).args(["-in", "-smt2"]).output();
    blocker.join().expect("fork blocker thread");

    assert!(
        matches!(&verdict, Verdict::Disproved { .. }),
        "published stub should execute after sibling fork; verdict={verdict:?}; elapsed={adapter_elapsed:?}; direct probe={direct_probe:?}"
    );
    assert_eq!(elab.env.trusted_base().len(), before);
    let output = direct_probe.expect("direct spawn of published stub");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "sat\n((k0 1))\n");
}

/// Promise class: durable soundness invariant.
///
/// MEASURED: a parsed refuting assignment reaches Disproved with zero trusted
/// growth, while a parsed non-refuting assignment reaches Unknown. CLAIMED:
/// solver model text is only an untrusted candidate. THE GAP: the existing
/// witness seam and kernel refutation check must remain the verdict authority.
#[test]
fn parsed_model_is_candidate_not_verdict() {
    let dir = TempDir::new().expect("stub directory");
    let mut elab = ElabEnv::new().expect("numeric environment");
    let obligation = equality(&mut elab);

    let refuting = stub(&dir, "printf 'sat\\n((k0 1))\\n'");
    let before = elab.env.trusted_base().len();
    let verdict = attempt_d_with_z3_process(&mut elab.env, &obligation, &refuting);
    let process_probe = if matches!(&verdict, Verdict::Disproved { .. }) {
        None
    } else {
        Some(
            Command::new(&refuting.program)
                .args(["-in", "-smt2"])
                .output(),
        )
    };
    assert!(
        matches!(&verdict, Verdict::Disproved { .. }),
        "expected kernel-checked refutation, got {verdict:?}; direct stub spawn probe: {process_probe:?}"
    );
    assert_eq!(elab.env.trusted_base().len(), before);

    let wrong = stub(&dir, "printf 'sat\\n((k0 0))\\n'");
    assert_unknown(wrong);
}

/// Promise class: durable fail-closed boundary.
///
/// MEASURED: each process/protocol failure reaches one Unknown hole. CLAIMED:
/// enabling the optional adapter cannot turn solver unavailability or bad
/// output into a build failure or trusted verdict. THE GAP: each fixture must
/// reach a distinct timeout or parser boundary rather than one proxy.
#[test]
fn every_process_and_protocol_failure_is_unknown() {
    let dir = TempDir::new().expect("stub directory");
    assert_unknown(stub(&dir, "printf 'unknown\\n'"));
    assert_unknown(stub(&dir, "printf 'sat\\nnot-a-model\\n'"));
    assert_unknown(delayed_valid_stub(&dir));
}

/// Promise class: durable determinism invariant.
///
/// MEASURED: three fresh executions of one solver response return Disproved
/// with no trusted growth. CLAIMED: identical solver proposals produce an
/// identical Ken verdict. THE GAP: a real solver version may choose a different
/// valid candidate, but the kernel check, not candidate identity, decides it.
#[test]
fn identical_input_and_candidate_have_deterministic_verdict() {
    let dir = TempDir::new().expect("stub directory");
    let config = stub(&dir, "printf 'sat\\n((k0 1))\\n'");
    let mut elab = ElabEnv::new().expect("numeric environment");
    let obligation = equality(&mut elab);
    let before = elab.env.trusted_base().len();
    for _ in 0..3 {
        assert!(matches!(
            attempt_d_with_z3_process(&mut elab.env, &obligation, &config),
            Verdict::Disproved { .. }
        ));
    }
    assert_eq!(elab.env.trusted_base().len(), before);
}

/// Promise class: transition sentinel for the CI-installed Z3 process.
///
/// MEASURED: the configured external binary can propose a model that the
/// kernel accepts as a refutation. CLAIMED: CI installs a working process
/// adapter, not merely stub coverage. THE GAP: this deliberately says nothing
/// about throughput or expanding the translated goal population.
#[test]
fn installed_z3_round_trip_reaches_kernel_checked_refutation() {
    match Command::new("z3").arg("-version").output() {
        Err(error) if error.kind() == ErrorKind::NotFound => {
            eprintln!("skipping installed-Z3 round trip: z3 is absent from PATH");
            return;
        }
        Err(error) => panic!("failed to probe installed z3: {error}"),
        Ok(output) => assert!(output.status.success(), "installed z3 probe failed"),
    }

    let mut elab = ElabEnv::new().expect("numeric environment");
    let obligation = equality(&mut elab);
    let before = elab.env.trusted_base().len();
    let result = attempt_obligation(&mut elab.env, &obligation);
    assert!(matches!(result.verdict, Verdict::Disproved { .. }));
    assert_eq!(elab.env.trusted_base().len(), before);

    let two_binder = two_binder_equality(&mut elab);
    let before_two = elab.env.trusted_base().len();
    let two_result = attempt_obligation(&mut elab.env, &two_binder);
    assert!(matches!(two_result.verdict, Verdict::Disproved { .. }));
    assert_eq!(elab.env.trusted_base().len(), before_two);
}

#[test]
fn missing_binary_is_not_a_build_requirement() {
    let path = Path::new("/definitely/not/a/z3/binary");
    assert_unknown(Z3ProcessConfig {
        program: path.into(),
        timeout: STARTUP_SAFE_STUB_TIMEOUT,
    });
}
