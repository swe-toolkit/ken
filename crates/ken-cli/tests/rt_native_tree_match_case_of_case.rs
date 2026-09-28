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
