#![cfg(feature = "z3-process")]

use std::{
    fs, io::ErrorKind, os::unix::fs::PermissionsExt, path::Path, process::Command, time::Duration,
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
// The delayed stub sleeps for one second and emits a valid refuting model;
// this shorter deadline makes enforced timeout the only Unknown outcome.
const DELIBERATE_TIMEOUT_PROBE: Duration = Duration::from_millis(100);

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
    let path = dir.path().join(name);
    let temporary_path = dir.path().join(format!("{name}.tmp"));
    fs::write(&temporary_path, contents).expect("write temporary stub");
    let mut permissions = fs::metadata(&temporary_path)
        .expect("temporary stub metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&temporary_path, permissions).expect("make temporary stub executable");
    fs::rename(&temporary_path, &path).expect("publish stub atomically");
    path
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
