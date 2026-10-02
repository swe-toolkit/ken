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

// AC-1: a closed Failure 5 through a composed Ret and the nearest runtime
// Failure byte must agree with the interpreter on both selected bytes.
#[cfg(target_os = "linux")]
fn root_exit_discriminator_source(runtime_failure: bool) -> String {
    let source = SHARED_BIND_SOURCE
        .replace(
            "proc decide (byte : UInt8)",
            "proc decide (byte : UInt8) (fallback : ExitCode) (runtime_code : UInt8)",
        )
        .replace("False |-> Failure 7", "False |-> fallback")
        .replace("(print_line \"accepted\")", "(print_line \"ok\")")
        .replace("ExitCode Success);", "ExitCode (Failure 5));")
        .replace("(print_line \"rejected\")", "(print_line \"bad\")")
        .replace(
            "Some byte |-> decide byte",
            "Some byte |-> decide byte (Failure 9) byte",
        );
    assert_eq!(source.matches("ExitCode (Failure 5));").count(), 1);
    if runtime_failure {
        source.replace(
            "ExitCode (Failure 5));",
            "ExitCode (Failure runtime_code));",
        )
    } else {
        source
    }
}

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

// An ordinary Inner case selects the ExitCode; the root has no authority to
// project Inner as an exit code. The same-answer variant still executes both
// Inner arms rather than inferring selection from its terminal code alone.
#[cfg(target_os = "linux")]
const CHECKED_INNER_SOURCE: &str = r#"program capabilities FS APartial
data Inner = Hit | Miss
fn main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit APartial (Failure 90);
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit APartial (Failure 91);
        Cons argument _more |-> match bytes_at argument 0 {
          None |-> host_exit APartial (Failure 92);
          Some byte |-> host_exit APartial (match (match eq_int (uint8_to_int byte) 1 {
            True |-> Hit;
            False |-> Miss
          }) {
            Hit |-> Failure 3;
            Miss |-> Failure 4
          })
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
    // R2 now refuses this source-identity slot allocation at generic
    // transfer before the old source-join boundary can inspect the Option.
    // Preserve the fail-closed, no-artifact contract and name the new arm.
    let error = build.expect_err("untested Option family must refuse before artifact emission");
    let message = format!("{error:?}");
    assert!(
        message.contains("RecursiveResidual: a source slot constructor reached generic transfer")
            && message.contains("without its creation-site suffix"),
        "the source-store guard must refuse before the prior join boundary: {message}"
    );
    assert!(
        std::fs::read_dir(dir.path()).unwrap().next().is_none(),
        "no object or executable may be emitted for this build refusal"
    );
    eprintln!("RT_TREE_OPTION_REFUSAL {message}");

    shared_bind_exit_code_arm_matches_interpreter(2, b"rejected\n", 7);
}

// Promise class: durable invariant. MEASURED: both selected bytes in each
// shared-bind variant, with full native/interpreter observation parity and
// distinct terminal codes. CLAIMED: a closed ExitCode and a runtime ExitCode
// reach the same checked root boundary. THE GAP: the two fixtures alone do not
// cover other producer families; the Option refusal is a separate pin.
#[cfg(target_os = "linux")]
#[test]
fn root_exit_closed_and_runtime_failure_agree_on_both_bytes() {
    for (runtime_failure, label) in [(false, "closed"), (true, "runtime")] {
        let source = root_exit_discriminator_source(runtime_failure);
        let dir = tempfile::tempdir().unwrap();
        let (artifact, hits) = ken_runtime::with_exit_code_case_of_case_route_count(|| {
            ken_cli::build_native_program(
                &source,
                ken_cli::SourceFormat::Ken,
                "rt-root-exit-closed-and-runtime",
                dir.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
        });
        assert!(hits > 0, "both ExitCode producers must use D1");
        let artifact = artifact.expect("both checked variants emit an artifact");
        for (byte, exit, stdout) in [
            (
                1_u8,
                if runtime_failure { 1 } else { 5 },
                b"ok\n".as_slice(),
            ),
            (2_u8, 9, b"bad\n".as_slice()),
        ] {
            let mut host = ken_interp::PosixHost::new_at(dir.path());
            let interpreted = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &[b"ken".to_vec(), vec![byte]],
                &[],
                dir.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .expect("same checked source executes in interpreter");
            assert_eq!(interpreted.exit_status, exit);
            assert_eq!(interpreted.stdout, stdout);
            assert_eq!(interpreted.effect_trace.len(), 1);
            let native = ken_runtime::run_bound_process_effect_observation(
                &artifact.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: vec![std::ffi::OsString::from_vec(vec![byte])],
                    environment: Vec::new(),
                    cwd: dir.path().to_owned(),
                    plan_hash: artifact.plan_transport_hash,
                },
            );
            eprintln!("RT_ROOT_EXIT_DISCRIMINATOR {label} byte={byte} native={native:?}");
            assert_eq!(native.expect("native exit must not trap"), interpreted);
        }
    }
}

// Spec: 42 §3.3 and 45 §4. Promise class: durable invariant. MEASURED:
// both checked variants (distinct and same-answer) on both bytes with full
// native/interpreter observation parity. CLAIMED: the Inner match remains an
// ordinary non-exit result and only the outer ExitCode reaches the root. THE
// GAP: parity does not itself prove which constructor was projected in the
// emitter; synthetic refusal pins and the root non-exit negative cover those
// separate boundaries.
#[cfg(target_os = "linux")]
#[test]
fn checked_inner_nonexit_result_and_outer_exit_match_interpreter() {
    for (same_answer, exits) in [(false, [3, 4]), (true, [3, 3])] {
        let source = if same_answer {
            CHECKED_INNER_SOURCE.replace("Miss |-> Failure 4", "Miss |-> Failure 3")
        } else {
            CHECKED_INNER_SOURCE.to_owned()
        };
        let dir = tempfile::tempdir().unwrap();
        let artifact = ken_cli::build_native_program(
            &source,
            ken_cli::SourceFormat::Ken,
            "rt-root-exit-checked-inner",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
        .expect("both checked Inner programs emit native artifacts");
        for (byte, exit) in [1_u8, 2_u8].into_iter().zip(exits) {
            let mut host = ken_interp::PosixHost::new_at(dir.path());
            let interpreted = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &[b"ken".to_vec(), vec![byte]],
                &[],
                dir.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .expect("checked Inner variant runs in the interpreter");
            assert_eq!(interpreted.exit_status, exit);
            assert_eq!(interpreted.stdout, b"");
            let native = ken_runtime::run_bound_process_effect_observation(
                &artifact.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: vec![std::ffi::OsString::from_vec(vec![byte])],
                    environment: Vec::new(),
                    cwd: dir.path().to_owned(),
                    plan_hash: artifact.plan_transport_hash,
                },
            )
            .expect("checked Inner variant runs natively");
            assert_eq!(native, interpreted, "same_answer={same_answer} byte={byte}");
        }
    }
}

// Promise class: durable invariant. MEASURED: D1 hits and full observation
// parity for the Success arm (byte 1). CLAIMED: the root projects a checked
// Success despite its non-root Ret transfer. THE GAP: it says nothing about
// the Failure payload mapping; the closed and dynamic Failure pins cover it.
#[cfg(target_os = "linux")]
#[test]
fn shared_bind_success_arm_matches_interpreter_after_root_exit_projection() {
    shared_bind_exit_code_arm_matches_interpreter(1, b"accepted\n", 0);
}

// Spec: 42 §3.3 and 45 §4. Promise class: durable invariant. MEASURED: a
// checked Ret field whose closed constructor is either Failure 0 or Success
// is returned through the same non-root result edge, then compared with the
// interpreter's terminal status and effects. CLAIMED: root projection uses
// the shared process_exit_status mapping, including Failure 0 -> status 1,
// and distinguishes both constructor identities. THE GAP: source checks do
// not prove all malformed carried payloads refuse; the root negative and
// separately measured identity-swap mutation constrain that boundary.
#[cfg(target_os = "linux")]
#[test]
fn root_exit_ret_carried_success_and_failure_zero_match_interpreter() {
    for (result, exit) in [("Success", 0), ("(Failure 0)", 1)] {
        let source = root_exit_discriminator_source(false).replace(
            "ExitCode (Failure 5));",
            &format!("ExitCode {result});"),
        );
        let dir = tempfile::tempdir().unwrap();
        let (build, hits) = ken_runtime::with_exit_code_case_of_case_route_count(|| {
            ken_cli::build_native_program(
                &source,
                ken_cli::SourceFormat::Ken,
                "rt-root-exit-ret-carried-zero-success",
                dir.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
        });
        assert!(
            hits > 0,
            "{result} must reach the D1 producer; build={:?}",
            build.as_ref().map(|_| ()).map_err(|err| format!("{err:?}"))
        );
        let artifact = build.expect("checked Ret-carried exit emits an artifact");
        let mut host = ken_interp::PosixHost::new_at(dir.path());
        let interpreted = ken_cli::run_program_effect_observation(
            &source,
            ken_cli::SourceFormat::Ken,
            &[b"ken".to_vec(), vec![1]],
            &[],
            dir.path().as_os_str().as_encoded_bytes(),
            &mut host,
        )
        .expect("same checked source executes in interpreter");
        assert_eq!(interpreted.exit_status, exit, "{result} interpreter status");
        assert_eq!(interpreted.stdout, b"ok\n", "{result} interpreter arm");
        assert_eq!(interpreted.effect_trace.len(), 1);
        let native = ken_runtime::run_bound_process_effect_observation(
            &artifact.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: vec![std::ffi::OsString::from_vec(vec![1])],
                environment: Vec::new(),
                cwd: dir.path().to_owned(),
                plan_hash: artifact.plan_transport_hash,
            },
        );
        assert_eq!(native.expect("root projection must not trap"), interpreted);
    }
}
