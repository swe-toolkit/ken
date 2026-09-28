#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStringExt;

// Spec: 42 §3.3 (only the selected method runs), 45 §4 (native agrees with
// the interpreter). The inner runtime Bool Match must retain the constructor
// identity consumed by the outer tree-producing Match. The host operation
// supplies the byte; each outer arm has its own effect and direct host_exit.
#[cfg(target_os = "linux")]
fn source(option_family: bool) -> String {
    let (on_true, on_false, success_case, failure_case) = if option_family {
        ("Some UInt8 byte", "None UInt8", "Some chosen", "None")
    } else {
        ("Success", "Failure 7", "Success", "Failure code")
    };
    let failure_exit = if option_family { "Failure 7" } else { "Failure code" };
    format!(r#"program capabilities FS APartial
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
    (\byte. match (match eq_int (uint8_to_int byte) 1 {{
      True |-> {on_true};
      False |-> {on_false}
    }}) {{
      {success_case} |-> bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode
        (host_console APartial Unit (print_line "accepted"))
        (\_. host_exit APartial Success);
      {failure_case} |-> bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode
        (host_console APartial Unit (print_line "rejected"))
        (\_. host_exit APartial ({failure_exit}))
    }})

proc main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  match input {{
    MkProcessInput arguments _environment _cwd |-> match arguments {{
      Nil |-> host_exit APartial (Failure 90);
      Cons _argv0 rest |-> match rest {{
        Nil |-> host_exit APartial (Failure 91);
        Cons argument _more |-> match bytes_at argument 0 {{
          None |-> host_exit APartial (Failure 92);
          Some byte |-> decide byte
        }}
      }}
    }}
  }}
"#)
}

#[cfg(target_os = "linux")]
fn verify_two_arms(option_family: bool) {
    let source = source(option_family);
    let dir = tempfile::tempdir().unwrap();
    let (artifact, route_hits) = ken_runtime::with_exit_code_case_of_case_route_count(|| {
        ken_cli::build_native_program(
            &source,
            ken_cli::SourceFormat::Ken,
            if option_family { "rt-tree-option-direct-exit" } else { "rt-tree-exit-direct-exit" },
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    eprintln!("RT_TREE_ROUTE_CENSUS option={option_family} fixture_build_hits={route_hits} artifact={:?}", artifact.as_ref().map(|_| "linked").map_err(|error| format!("{error:?}")));
    assert!(route_hits > 0, "the checked fixture must reach case-of-case composition");
    let artifact = artifact.expect("a checked nested match emits a linked native artifact");

    // Promise class: durable invariant. Both runtime inputs select distinct
    // effects and exit codes on one native artifact and the interpreter.
    // MEASURED: constructor-route count, effect ops, stdout, and exit status.
    // CLAIMED: the outer Match observes the actual constructor, not the exit
    // scalar; THE GAP: mere artifact emission misses a crossed/static branch.
    for (byte, stdout, exit) in [(1_u8, b"seed\naccepted\n".as_slice(), 0),
                                 (2_u8, b"seed\nrejected\n".as_slice(), 7)] {
        let native = ken_runtime::run_bound_process_effect_observation(
            &artifact.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: vec![std::ffi::OsString::from_vec(vec![byte])],
                environment: Vec::new(),
                cwd: dir.path().to_owned(),
                plan_hash: artifact.plan_transport_hash,
            },
        ).expect("native artifact runs on this byte");
        let mut host = ken_interp::PosixHost::new_at(dir.path());
        let interpreted = ken_cli::run_program_effect_observation(
            &source,
            ken_cli::SourceFormat::Ken,
            &[b"ken".to_vec(), vec![byte]],
            &[],
            dir.path().as_os_str().as_encoded_bytes(),
            &mut host,
        ).expect("same checked program runs in interpreter on this byte");
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
        assert_eq!(native_ops, vec![ken_runtime::HostOpV1::ConsoleWrite, ken_runtime::HostOpV1::ConsoleWrite]);
        assert_eq!(native_ops, interp_ops, "the effects must agree for byte {byte}");
    }
}

#[cfg(target_os = "linux")]
#[test]
fn exit_code_inner_match_direct_exit_branches_match_interpreter() {
    verify_two_arms(false);
}

#[cfg(target_os = "linux")]
#[test]
fn option_inner_match_direct_exit_branches_match_interpreter() {
    verify_two_arms(true);
}
