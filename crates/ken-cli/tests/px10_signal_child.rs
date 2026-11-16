//! PX10-PIDFD-CHILD-SIGNAL public source, interpreter/native differential.
//! Promise classes: normative compatibility values for Linux signal numbers
//! and ESRCH; durable invariants for resource authority and typed outcomes.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

const TEMPLATE: &str = include_str!("fixtures/px10_signal_child.ken");
const SIGNAL_BODY: &str = r#"
  host_bind AFull (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (signal_child AFull child __SIGNAL__)
    (\sent. match sent {
      Ok _ |->
        host_bind AFull (Result ResourceError ChildExit) (ResourceBodyResult Unit Unit)
          (wait_child AFull child)
          (\waited. match waited {
            Ok (ChildExit.Signaled _) |->
              host_pure AFull (ResourceBodyResult Unit Unit)
                (ResourceBodyOk Unit Unit MkUnit);
            Ok _ |-> host_pure AFull (ResourceBodyResult Unit Unit)
              (ResourceBodyErr Unit Unit MkUnit);
            Err _ |-> host_pure AFull (ResourceBodyResult Unit Unit)
              (ResourceBodyErr Unit Unit MkUnit)
          });
      Err _ |-> host_pure AFull (ResourceBodyResult Unit Unit)
        (ResourceBodyErr Unit Unit MkUnit)
    })
"#;
const WAITED_BODY: &str = r#"
  host_bind AFull (Result ResourceError ChildExit) (ResourceBodyResult Unit Unit)
    (wait_child AFull child)
    (\waited. match waited {
      Ok (ChildExit.Exited _) |->
        host_bind AFull (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
          (signal_child AFull child SignalKill)
          (\sent. match sent {
            Err (ResourceError.ResourceHostIO (IOError.Other code)) |->
              match eq_int code 3 {
                True |-> host_pure AFull (ResourceBodyResult Unit Unit)
                  (ResourceBodyOk Unit Unit MkUnit);
                False |-> host_pure AFull (ResourceBodyResult Unit Unit)
                  (ResourceBodyErr Unit Unit MkUnit)
              };
            Ok _ |-> host_pure AFull (ResourceBodyResult Unit Unit)
              (ResourceBodyErr Unit Unit MkUnit);
            Err _ |-> host_pure AFull (ResourceBodyResult Unit Unit)
              (ResourceBodyErr Unit Unit MkUnit)
          });
      Ok _ |-> host_pure AFull (ResourceBodyResult Unit Unit)
        (ResourceBodyErr Unit Unit MkUnit);
      Err _ |-> host_pure AFull (ResourceBodyResult Unit Unit)
        (ResourceBodyErr Unit Unit MkUnit)
    })
"#;

fn checked_catalog_roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn case_source(case: &str) -> String {
    const SLEEP_PLAN: &str = "spawn_plan (bytes_encode \"/bin/sleep\") \
             (Cons Bytes (bytes_encode \"caller-zero\") \
             (Cons Bytes (bytes_encode \"30\") (Nil Bytes)))";
    let (program, body) = if case == "waited" {
        (
            "spawn_plan (bytes_encode \"/bin/true\") \
             (Cons Bytes (bytes_encode \"caller-zero\") (Nil Bytes))",
            WAITED_BODY.to_owned(),
        )
    } else {
        let constructor = match case {
            "hangup" => "SignalHangup",
            "interrupt" => "SignalInterrupt",
            "quit" => "SignalQuit",
            "kill" => "SignalKill",
            "user1" => "SignalUser1",
            "user2" => "SignalUser2",
            "terminate" => "SignalTerminate",
            _ => unreachable!("closed case inventory"),
        };
        (SLEEP_PLAN, SIGNAL_BODY.replace("__SIGNAL__", constructor))
    };
    TEMPLATE
        .replace("__PROCESS_PLAN__", program)
        .replace("__CHILD_BODY__", &body)
        .replacen(
            "program capabilities FS AFull, Process\n",
            "program capabilities FS AFull, Process, RootExecution Allow\n",
            1,
        )
}

fn check_signal_case(
    case: &str,
    expected: ken_host::CanonicalOutcomeV1,
    wait: ken_host::ChildExitV1,
) {
    let root = tempfile::tempdir().expect("isolated child signal root");
    let source = case_source(case);
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &checked_catalog_roots(),
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("checked signal program runs in the interpreter");
    assert_eq!(
        interpreted.exit_status, 0,
        "interpreter: {:?}",
        interpreted.effect_trace
    );
    let built = ken_cli::build_native_program_with_roots(
        &source,
        ken_cli::SourceFormat::Ken,
        &checked_catalog_roots(),
        &format!("px10_signal_{case}"),
        root.path().join("native"),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("checked signal program builds natively");
    let native = ken_runtime::run_bound_process_effect_observation(
        &built.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: built.plan_transport_hash,
        },
    )
    .expect("checked signal program executes natively");
    assert_eq!(native.exit_status, 0, "native: {:?}", native.effect_trace);
    assert_eq!(native.terminal_error, interpreted.terminal_error);
    assert_eq!(native.terminal_exit, interpreted.terminal_exit);
    assert_eq!(native.effect_trace, interpreted.effect_trace);
    let signal = native
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_host::HostOpV1::ProcessSignal)
        .collect::<Vec<_>>();
    let waited = native
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_host::HostOpV1::ProcessWait)
        .collect::<Vec<_>>();
    assert_eq!(
        signal.len(),
        1,
        "one source signal: {:?}",
        native.effect_trace
    );
    assert_eq!(
        waited.len(),
        1,
        "one source wait: {:?}",
        native.effect_trace
    );
    assert_eq!(signal[0].outcome, expected);
    assert_eq!(
        waited[0].outcome,
        ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ChildExit(wait))
    );
}

/// Promise class: normative compatibility vector. The five constructor
/// scalars not exercised by the native differential remain distinct and map
/// to the exact Linux wait result in a checked interpreter program.
#[test]
fn other_five_signal_constructors_map_to_exact_numbers_in_interpreter() {
    for (case, number) in [
        ("hangup", 1),
        ("interrupt", 2),
        ("quit", 3),
        ("user1", 10),
        ("user2", 12),
    ] {
        let root = tempfile::tempdir().expect("isolated child signal root");
        let source = case_source(case);
        let mut host = ken_interp::PosixHost::new_at(root.path());
        let observed = ken_cli::run_program_effect_observation(
            &source,
            ken_cli::SourceFormat::Ken,
            &checked_catalog_roots(),
            &[],
            &[],
            root.path().as_os_str().as_encoded_bytes(),
            &mut host,
        )
        .expect("checked signal source executes");
        assert_eq!(
            observed.exit_status, 0,
            "{case}: {:?}",
            observed.effect_trace
        );
        let signal = observed
            .effect_trace
            .iter()
            .filter(|event| event.operation == ken_host::HostOpV1::ProcessSignal)
            .collect::<Vec<_>>();
        let waited = observed
            .effect_trace
            .iter()
            .filter(|event| event.operation == ken_host::HostOpV1::ProcessWait)
            .collect::<Vec<_>>();
        assert_eq!(signal.len(), 1, "{case} source signal");
        assert_eq!(waited.len(), 1, "{case} source wait");
        assert_eq!(
            signal[0].outcome,
            ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::Unit)
        );
        assert_eq!(
            waited[0].outcome,
            ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ChildExit(
                ken_host::ChildExitV1::Signaled(number)
            ))
        );
    }
}

/// Promise class: durable invariant with normative Linux number 15. A Ken
/// `SignalTerminate` returns Ok Unit and wait returns Signaled 15 in both
/// executors with identical canonical effect sequences.
#[test]
fn signal_terminate_then_wait_matches_interpreter_and_native() {
    std::thread::Builder::new()
        .name("px10-signal-terminate".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            check_signal_case(
                "terminate",
                ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::Unit),
                ken_host::ChildExitV1::Signaled(15),
            )
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Promise class: durable invariant with normative Linux number 9.
#[test]
fn signal_kill_then_wait_matches_interpreter_and_native() {
    std::thread::Builder::new()
        .name("px10-signal-kill".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            check_signal_case(
                "kill",
                ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::Unit),
                ken_host::ChildExitV1::Signaled(9),
            )
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Promise class: durable invariant. A waited child remains a typed resource
/// but cannot be signaled; both engines return Other 3, never Ok Unit.
#[test]
fn signal_after_wait_returns_esrch_on_both_executors() {
    std::thread::Builder::new()
        .name("px10-signal-after-wait".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            check_signal_case(
                "waited",
                ken_host::CanonicalOutcomeV1::Error(ken_host::SemanticErrorV1::Io(
                    ken_host::IoErrorIdentityV1::Other(libc::ESRCH),
                )),
                ken_host::ChildExitV1::Exited(0),
            )
        })
        .unwrap()
        .join()
        .unwrap();
}
