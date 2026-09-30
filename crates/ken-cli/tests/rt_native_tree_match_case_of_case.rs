#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStringExt;

// Spec: 42 §3.3 (only the selected method runs), 45 §4 (native agrees with
// the interpreter). A direct-exit nested Match already runs through the
// ordinary Match emitter, not the D1 producer-local composition route.
#[cfg(target_os = "linux")]
const SOURCE: &str = r#"program capabilities FS APartial
proc host_byte (chosen : UInt8) : HostIO APartial UInt8 visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit UInt8
    (host_console APartial Unit (print_line "seed"))
    (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      UInt8 chosen)

proc decide (chosen : UInt8) : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    UInt8 ExitCode (host_byte chosen)
    (\byte. match (match eq_int (uint8_to_int byte) 1 {
      True |-> Success;
      False |-> Failure 7
    }) {
      Success |-> bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode
        (host_console APartial Unit (print_line "accepted"))
        (\_. host_exit APartial Success);
      Failure code |-> bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode
        (host_console APartial Unit (print_line "rejected"))
        (\_. host_exit APartial (Failure code))
    })

proc main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit APartial (Failure 90);
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit APartial (Failure 91);
        Cons argument _more |-> match bytes_at argument 0 {
          None |-> host_exit APartial (Failure 92);
          Some byte |-> decide byte
        }
      }
    }
  }
"#;

// The shared ExitCode bind from the checked R2 fixture at 4f101bba0. Unlike
// SOURCE, its two effectful arms return through one continuation; the D1
// producer-local route must be taken rather than the ordinary Match emitter.
#[cfg(target_os = "linux")]
const SHARED_BIND_SOURCE: &str = r#"program capabilities FS APartial
proc decide (byte : UInt8) : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    ExitCode ExitCode
    (match (match eq_int (uint8_to_int byte) 1 {
      True |-> Success;
      False |-> Failure 7
    }) {
      Success |-> bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode
        (host_console APartial Unit (print_line "accepted"))
        (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
          (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
          ExitCode Success);
      Failure code |-> bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode
        (host_console APartial Unit (print_line "rejected"))
        (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
          (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
          ExitCode (Failure code))
    })
    (\code. host_exit APartial code)

proc main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit APartial (Failure 90);
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit APartial (Failure 91);
        Cons argument _more |-> match bytes_at argument 0 {
          None |-> host_exit APartial (Failure 92);
          Some byte |-> decide byte
        }
      }
    }
  }
"#;

// The same shared-bind and ProcessInput harness as SHARED_BIND_SOURCE, but the
// inner result and outer case family are Option. Neither arm has native parity
// authorization: both remain on the pre-D1 fail-closed build path.
#[cfg(target_os = "linux")]
const OPTION_OUTER_SOURCE: &str = r#"program capabilities FS APartial
proc decide (byte : UInt8) : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    ExitCode ExitCode
    (match (match eq_int (uint8_to_int byte) 1 {
      True |-> Some UInt8 byte;
      False |-> None UInt8
    }) {
      Some b |-> bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode
        (host_console APartial Unit (print_line "some"))
        (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
          (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
          ExitCode (Failure 3));
      None |-> bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode
        (host_console APartial Unit (print_line "none"))
        (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
          (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
          ExitCode (Failure 4))
    })
    (\code. host_exit APartial code)

proc main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit APartial (Failure 90);
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit APartial (Failure 91);
        Cons argument _more |-> match bytes_at argument 0 {
          None |-> host_exit APartial (Failure 92);
          Some byte |-> decide byte
        }
      }
    }
  }
"#;

#[cfg(target_os = "linux")]
#[test]
fn console_direct_exit_nested_match_uses_existing_route() {
    let dir = tempfile::tempdir().unwrap();
    let (artifact, d1_hits) = ken_runtime::with_exit_code_case_of_case_route_count(|| {
        ken_cli::build_native_program(
            SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt-tree-exit-direct-exit",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    // Promise class: durable invariant for the ordinary-versus-producer route
    // boundary. MEASURED: D1 hits, actual effects, stdout and exit on both
    // dynamic arms. CLAIMED: this existing direct-exit route stays separate
    // from D1 and agrees with the interpreter; THE GAP: a build-only assertion
    // would miss crossed arms, while parity alone would miss a route takeover.
    assert_eq!(
        d1_hits, 0,
        "D1 must not absorb the existing ordinary Match route"
    );
    let artifact = artifact.expect("a checked direct-exit nested Match emits an artifact");
    for (byte, stdout, exit) in [
        (1_u8, b"seed\naccepted\n".as_slice(), 0),
        (2_u8, b"seed\nrejected\n".as_slice(), 7),
    ] {
        let native = ken_runtime::run_bound_process_effect_observation(
            &artifact.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: vec![std::ffi::OsString::from_vec(vec![byte])],
                environment: Vec::new(),
                cwd: dir.path().to_owned(),
                plan_hash: artifact.plan_transport_hash,
            },
        )
        .expect("native artifact runs on this byte");
        let mut host = ken_interp::PosixHost::new_at(dir.path());
        let interpreted = ken_cli::run_program_effect_observation(
            SOURCE,
            ken_cli::SourceFormat::Ken,
            &[b"ken".to_vec(), vec![byte]],
            &[],
            dir.path().as_os_str().as_encoded_bytes(),
            &mut host,
        )
        .expect("same checked program runs in interpreter on this byte");
        let native_ops: Vec<_> = native
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect();
        let interp_ops: Vec<_> = interpreted
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect();
        assert_eq!(
            native.stdout, stdout,
            "wrong native branch for byte {byte}: {native:?}"
        );
        assert_eq!(
            interpreted.stdout, stdout,
            "wrong interpreter branch for byte {byte}"
        );
        assert_eq!(
            native.exit_status, exit,
            "wrong native exit for byte {byte}: {native:?}"
        );
        assert_eq!(
            interpreted.exit_status, exit,
            "wrong interpreter exit for byte {byte}"
        );
        assert_eq!(native.stdout, interpreted.stdout);
        assert_eq!(native.exit_status, interpreted.exit_status);
        assert_eq!(native.terminal_error, interpreted.terminal_error);
        assert_eq!(native.terminal_exit, interpreted.terminal_exit);
        assert_eq!(
            native_ops,
            vec![
                ken_runtime::HostOpV1::ConsoleWrite,
                ken_runtime::HostOpV1::ConsoleWrite
            ]
        );
        assert_eq!(
            native_ops, interp_ops,
            "the effects must agree for byte {byte}"
        );
    }
}

// Spec: 42 §3.3 and §6 (one selected arm, ordered effects); 45 §4
// (native/interpreter agreement). Both tests use this checked source and the
// same full observation oracle, but own distinct runtime branches.
#[cfg(target_os = "linux")]
fn shared_bind_exit_code_arm_matches_interpreter(byte: u8, stdout: &[u8], exit: i32) {
    let dir = tempfile::tempdir().unwrap();
    let mut host = ken_interp::PosixHost::new_at(dir.path());
    let interpreted = ken_cli::run_program_effect_observation(
        SHARED_BIND_SOURCE,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec(), vec![byte]],
        &[],
        dir.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("shared-bind source runs in the interpreter for this byte");
    let interp_ops: Vec<_> = interpreted
        .effect_trace
        .iter()
        .map(|event| event.operation)
        .collect();
    assert_eq!(
        interpreted.stdout, stdout,
        "interpreter arm for byte {byte}"
    );
    assert_eq!(
        interpreted.exit_status, exit,
        "interpreter exit for byte {byte}"
    );
    assert_eq!(interp_ops, vec![ken_runtime::HostOpV1::ConsoleWrite]);

    let (artifact, d1_hits) = ken_runtime::with_exit_code_case_of_case_route_count(|| {
        ken_cli::build_native_program(
            SHARED_BIND_SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt-tree-exit-shared-bind",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    eprintln!("RT_TREE_ROUTE_CENSUS shared_bind_hits={d1_hits}");
    assert!(
        d1_hits > 0,
        "the shared bind must execute D1, not only compile"
    );
    let artifact = artifact.expect("the D1 shared-bind ExitCode fixture emits a native artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &artifact.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: vec![std::ffi::OsString::from_vec(vec![byte])],
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: artifact.plan_transport_hash,
        },
    )
    .expect("the checked native artifact runs on this byte");
    let native_ops: Vec<_> = native
        .effect_trace
        .iter()
        .map(|event| event.operation)
        .collect();
    assert_eq!(
        native.stdout, stdout,
        "wrong native branch for byte {byte}: {native:?}"
    );
    assert_eq!(
        native.exit_status, exit,
        "wrong native exit for byte {byte}: {native:?}"
    );
    assert_eq!(native.stdout, interpreted.stdout);
    assert_eq!(native.exit_status, interpreted.exit_status);
    assert_eq!(native.terminal_error, interpreted.terminal_error);
    assert_eq!(native.terminal_exit, interpreted.terminal_exit);
    assert_eq!(native_ops, vec![ken_runtime::HostOpV1::ConsoleWrite]);
    assert_eq!(
        native_ops, interp_ops,
        "the effects must agree for byte {byte}"
    );
}

// Promise class: durable invariant. MEASURED: D1 hits and native/interpreter
// stdout, exit, terminal status and effects for the Failure arm (byte 2).
// CLAIMED: TREE-MATCH preserves this arm through its shared continuation.
// THE GAP: this cannot certify the Success arm; that ROOT-EXIT witness is below.
#[cfg(target_os = "linux")]
#[test]
fn shared_bind_failure_arm_matches_interpreter_on_d1_route() {
    shared_bind_exit_code_arm_matches_interpreter(2, b"rejected\n", 7);
}

// Promise class: transition sentinel until a separate Option-family parity
// decision. MEASURED: both Option arms execute in the interpreter, while the
// native build refuses at the previously checked planned-source-join boundary
// without an artifact. CLAIMED: D1 no longer admits this untested family; THE
// GAP: this refusal does not establish native Option parity. The paired
// ExitCode byte-2 run proves the selected D1 family still emits and agrees.
#[cfg(target_os = "linux")]
#[test]
fn option_outer_family_refuses_before_artifact_while_exit_code_uses_d1() {
    let dir = tempfile::tempdir().unwrap();
    for (byte, stdout, exit) in [
        (1_u8, b"some\n".as_slice(), 3),
        (2_u8, b"none\n".as_slice(), 4),
    ] {
        let mut host = ken_interp::PosixHost::new_at(dir.path());
        let interpreted = ken_cli::run_program_effect_observation(
            OPTION_OUTER_SOURCE,
            ken_cli::SourceFormat::Ken,
            &[b"ken".to_vec(), vec![byte]],
            &[],
            dir.path().as_os_str().as_encoded_bytes(),
            &mut host,
        )
        .expect("the same checked Option source runs in the interpreter");
        assert_eq!(interpreted.stdout, stdout, "Option byte {byte}");
        assert_eq!(interpreted.exit_status, exit, "Option byte {byte}");
        let operations: Vec<_> = interpreted
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect();
        assert_eq!(operations, vec![ken_runtime::HostOpV1::ConsoleWrite]);
    }

    let (build, d1_hits) = ken_runtime::with_exit_code_case_of_case_route_count(|| {
        ken_cli::build_native_program(
            OPTION_OUTER_SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt-tree-option-outer-refusal",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    assert_eq!(d1_hits, 0, "Option must not enter the ExitCode D1 route");
    let error = build.expect_err("untested Option family must refuse before artifact emission");
    let message = format!("{error:?}");
    assert!(
        message.contains("planned source join")
            && message.contains("neither emitted nor statically unselected"),
        "the prior join boundary must refuse: {message}"
    );
    assert!(
        std::fs::read_dir(dir.path()).unwrap().next().is_none(),
        "no object or executable may be emitted for this build refusal"
    );
    eprintln!("RT_TREE_OPTION_REFUSAL {message}");

    shared_bind_exit_code_arm_matches_interpreter(2, b"rejected\n", 7);
}

// ROOT-EXIT AC-1 witness, deliberately ignored until its root-boundary decode
// accepts the carried Success constructor. The native byte-1 stdout and one
// ConsoleWrite already match the interpreter, but the checked root guard
// expects tag 2 and sees tag 5, then emits an unclassified -1 terminal.
// Promise class: durable invariant when enabled. MEASURED then: D1 hits and
// full native/interpreter parity for the Success arm (byte 1). CLAIMED: root
// exit projection accepts this specialization-result edge. THE GAP today:
// correct pre-guard effects are not terminal parity; an ignored row is no green.
#[cfg(target_os = "linux")]
#[test]
#[ignore = "RT-ROOT-EXIT-PROJECTION-KEYED-ON-JOIN: tag-5 Success at root guard expects tag 2; native parity pending"]
fn shared_bind_success_arm_matches_interpreter_after_root_exit_projection() {
    shared_bind_exit_code_arm_matches_interpreter(1, b"accepted\n", 0);
}
