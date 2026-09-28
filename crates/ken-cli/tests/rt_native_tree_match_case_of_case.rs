// Durable native/interpreter differential for an ExitCode case-of-case.
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStringExt;
// Spec: 42 §3.3 (only the selected method runs), 45 §4 (native agrees with
// the interpreter). An inner runtime Bool match must keep its Success/Failure
// constructor identity until the outer tree-producing Match selects a branch.

#[cfg(target_os = "linux")]
const SOURCE: &str = r#"program capabilities FS APartial
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

#[cfg(target_os = "linux")]
#[test]
fn exit_code_case_of_case_selects_both_effectful_arms_natively() {
    let dir = tempfile::tempdir().unwrap();
    let (artifact, route_hits) = ken_runtime::with_exit_code_case_of_case_route_count(|| {
        ken_cli::build_native_program(
            SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt-native-tree-case-of-case",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    let artifact = artifact.expect("a checked nested ExitCode match emits a linked native artifact");
    assert!(route_hits > 0, "the checked fixture must reach case-of-case composition");
    eprintln!("RT_TREE_ROUTE_CENSUS fixture_build_hits={route_hits}");

    // Promise class: durable invariant. Both arms are selected by runtime
    // input, so one built artifact must agree with the interpreter on each.
    // MEASURED: console bytes, exit status, and actual dispatched effect ops.
    // CLAIMED: composing the inner and outer Matches preserves constructor
    // selection; THE GAP: a build-only check would not see crossed branches.
    for (byte, stdout, exit) in [(1_u8, b"accepted\n".as_slice(), 0),
                                  (2_u8, b"rejected\n".as_slice(), 7)] {
        let arguments = vec![vec![byte]];
        let native = ken_runtime::run_bound_process_effect_observation(
            &artifact.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: vec![std::ffi::OsString::from_vec(vec![byte])],
                environment: Vec::new(),
                cwd: dir.path().to_owned(),
                plan_hash: artifact.plan_transport_hash,
            },
        ).expect("the linked process must execute each input");
        let mut host = ken_interp::PosixHost::new_at(dir.path());
        let interpreted = ken_cli::run_program_effect_observation(
            SOURCE,
            ken_cli::SourceFormat::Ken,
            &arguments,
            &[],
            dir.path().as_os_str().as_encoded_bytes(),
            &mut host,
        ).expect("the same checked input runs through the interpreter");
        let native_ops: Vec<_> = native.effect_trace.iter().map(|event| event.operation).collect();
        let interp_ops: Vec<_> = interpreted.effect_trace.iter().map(|event| event.operation).collect();
        assert_eq!(native.stdout, stdout, "wrong native branch for byte {byte}: {native:?}");
        assert_eq!(interpreted.stdout, stdout, "wrong interpreter branch for byte {byte}");
        assert_eq!(native.exit_status, exit, "wrong native exit for byte {byte}: {native:?}");
        assert_eq!(interpreted.exit_status, exit, "wrong interpreter exit for byte {byte}");
        assert_eq!(native.stdout, interpreted.stdout);
        assert_eq!(native.exit_status, interpreted.exit_status);
        assert_eq!(native.terminal_error, interpreted.terminal_error);
        assert_eq!(native.terminal_exit, interpreted.terminal_exit);
        assert_eq!(native_ops, vec![ken_runtime::HostOpV1::ConsoleWrite]);
        assert_eq!(native_ops, interp_ops, "the actual effects must agree for byte {byte}");
    }
}
