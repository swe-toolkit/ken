//! Direct-emission source controls for RT-PLANNER-SEED-BINDING-ORDER.
//!
//! The coordinate-keyed gather fallback is not reached by these fixtures:
//! their green result is a positive control on direct emission, not an AC-1
//! red/green witness. The response and gather consumers are pinned separately.

const SAME_TYPE: &str = r#"program capabilities FS APartial
proc send_pair (left : Bytes) (right : Bytes)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) ExitCode
    (host_console APartial (Result IOError Unit)
      (write Stdout (bytes_encode "begin:")))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) ExitCode
      (host_console APartial (Result IOError Unit) (write Stdout left))
      (\_. bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        (Result IOError Unit) ExitCode
        (host_console APartial (Result IOError Unit) (write Stdout right))
        (\_. host_exit APartial Success)))

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  send_pair (bytes_encode "left/") (bytes_encode "right")
"#;

const MIXED_TYPE: &str = r#"program capabilities FS APartial
proc send_and_exit (bytes : Bytes) (code : ExitCode)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) ExitCode
    (host_console APartial (Result IOError Unit)
      (write Stdout (bytes_encode "begin:")))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) ExitCode
      (host_console APartial (Result IOError Unit) (write Stdout bytes))
      (\_. host_exit APartial code))

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  send_and_exit (bytes_encode "mixed") (Failure 47)
"#;

fn assert_direct_source(source: &str, label: &str, expected_stdout: &[u8], exit: i32) {
    let dir = tempfile::Builder::new()
        .prefix("ken-rt-planner-seed-")
        .tempdir()
        .expect("fixture directory");
    let output = ken_cli::build_native_program(
        source,
        ken_cli::SourceFormat::Ken,
        label,
        dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("two-parameter direct-emission source builds a linked artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("the native fixture executes");
    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let reference = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec()],
        &[],
        b"/",
        &mut host,
    )
    .expect("the same checked source executes through the reference path");
    // The native host reports ConsoleWrite request bytes in its trace but
    // does not aggregate them into EffectObservation.stdout. Compare the
    // complete effect trace and termination, then check the reference's
    // aggregate bytes (the same writes the native trace records).
    assert_eq!(native.effect_trace, reference.effect_trace);
    assert_eq!(native.terminal_exit, reference.terminal_exit);
    assert_eq!(native.terminal_error, reference.terminal_error);
    assert_eq!(native.exit_status, reference.exit_status);
    assert_eq!(reference.stdout, expected_stdout);
    assert_eq!(native.exit_status, exit);
    assert_eq!(
        native.effect_trace.iter().map(|event| event.operation).collect::<Vec<_>>(),
        vec![ken_runtime::HostOpV1::ConsoleWrite; if label == "same" { 3 } else { 2 }],
    );
}

/// Promise class: durable behavior. Two distinct Bytes values must not alias.
#[test]
fn two_same_type_parameters_read_after_a_bind_keep_their_values() {
    assert_direct_source(SAME_TYPE, "same", b"begin:left/right", 0);
}

/// Promise class: durable behavior. A Bytes and an ExitCode remain distinct.
#[test]
fn two_different_type_parameters_read_after_a_bind_keep_their_values() {
    assert_direct_source(MIXED_TYPE, "mixed", b"begin:mixed", 47);
}
