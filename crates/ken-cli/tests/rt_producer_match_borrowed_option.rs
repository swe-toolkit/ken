#![cfg(target_os = "linux")]

use std::os::unix::ffi::OsStringExt;

// Baseline provisioning: this exact two-arm row passes at 16 MiB (measured
// upper bound on its required stack), while the Architect measured default
// 2 MiB overflow. The existing Runtime helper's 256 MiB provision therefore
// supplies at least 240 MiB headroom over the measured passing bound. This is
// not a default-stack claim; Builder::stack_size overrides ambient defaults.
const BUILD_STACK_BYTES: usize = 256 * 1024 * 1024;

const SOURCE: &str = r#"program capabilities FS APartial
proc read_byte (cap : Cap APartial) (chosen : UInt8)
  : HostIO APartial UInt8 visits [FS] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result FileError Bytes) UInt8
    (inject_l (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp
      (Result FileError Bytes)
      (readFile APartial cap (bytes_encode "ac0.bin")))
    (\read. match read {
      Err _ |-> Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        UInt8 chosen;
      Ok bytes |-> match bytes_at bytes 0 {
        None |-> Ret (Coproduct (FSOp APartial) AmbientOp)
          (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
          UInt8 chosen;
        Some byte |-> Ret (Coproduct (FSOp APartial) AmbientOp)
          (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
          UInt8 byte
      }
    })

proc decide (byte : UInt8) : HostIO APartial ExitCode visits [Console] =
  match (match eq_int (uint8_to_int byte) 1 {
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
  }

proc main (input : ProcessInput) (caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS, Console] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit APartial (Failure 90);
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit APartial (Failure 91);
        Cons argument _more |-> match bytes_at argument 0 {
          None |-> host_exit APartial (Failure 92);
          Some chosen |-> match caps {
            MkProgramCaps cap |-> bind (Coproduct (FSOp APartial) AmbientOp)
              (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
              UInt8 ExitCode (read_byte cap chosen) (\byte. decide byte)
          }
        }
      }
    }
  }
"#;

// Promise class: durable native/interpreter differential (spec 42 §5, 45 §4).
// MEASURED: the real FS read selects both Option arms, then native and
// interpreter agree on stdout, exit, and complete effect trace; D1 route 0.
// CLAIMED: a borrowed Option in the tree producer continues through the
// remaining direct-exit Match rather than refusing or dropping its effects.
// THE GAP: this fixture covers one `bytes_at` producer and its two dynamic
// branches, not every possible borrowed Option source or a default-stack build.
#[test]
fn fs_read_byte_borrowed_option_producer_matches_interpreter() {
    std::thread::Builder::new()
        .name("rt-producer-borrowed-option".to_owned())
        .stack_size(BUILD_STACK_BYTES)
        .spawn(assert_fs_read_byte_borrowed_option_producer)
        .expect("spawn stated-stack native fixture")
        .join()
        .expect("native fixture thread");
}

fn assert_fs_read_byte_borrowed_option_producer() {
    let dir = tempfile::tempdir().unwrap();
    let (build, d1_hits) = ken_runtime::with_exit_code_case_of_case_route_count(|| {
        ken_cli::build_native_program(
            SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt-producer-match-borrowed-option",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    assert_eq!(
        d1_hits, 0,
        "borrowed Option must not depend on the D1 route"
    );
    let build = build.expect("the BorrowedOption producer links a native artifact");

    for (arm, contents, expected_stdout, expected_exit) in [
        ("Some", vec![1u8], b"accepted\n".as_slice(), 0),
        ("None", Vec::new(), b"rejected\n".as_slice(), 7),
    ] {
        std::fs::write(dir.path().join("ac0.bin"), contents).unwrap();
        let native = ken_runtime::run_bound_process_effect_observation(
            &build.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: vec![std::ffi::OsString::from_vec(vec![2u8])],
                environment: Vec::new(),
                cwd: dir.path().to_owned(),
                plan_hash: build.plan_transport_hash,
            },
        )
        .unwrap_or_else(|error| panic!("{arm}: native artifact runs: {error:?}"));
        let mut host = ken_interp::PosixHost::new_at(dir.path());
        let interpreted = ken_cli::run_program_effect_observation(
            SOURCE,
            ken_cli::SourceFormat::Ken,
            &[b"ken".to_vec(), vec![2u8]],
            &[],
            dir.path().as_os_str().as_encoded_bytes(),
            &mut host,
        )
        .unwrap_or_else(|error| panic!("{arm}: interpreter runs: {error:?}"));
        assert_eq!(interpreted.stdout, expected_stdout, "{arm}: reference arm");
        assert_eq!(
            interpreted.exit_status, expected_exit,
            "{arm}: reference exit"
        );
        assert_eq!(native.stdout, interpreted.stdout, "{arm}: stdout parity");
        assert_eq!(
            native.exit_status, interpreted.exit_status,
            "{arm}: exit parity"
        );
        assert_eq!(
            native.effect_trace, interpreted.effect_trace,
            "{arm}: effect parity"
        );
        assert_eq!(
            native.terminal_error, interpreted.terminal_error,
            "{arm}: terminal parity"
        );
        let operations: Vec<_> = interpreted
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect();
        assert_eq!(
            operations,
            [
                ken_runtime::HostOpV1::FsReadFile,
                ken_runtime::HostOpV1::ConsoleWrite
            ],
            "{arm}: the FS read and selected console effect must execute"
        );
    }
}
