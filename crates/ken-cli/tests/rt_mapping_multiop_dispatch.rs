//! RT-MAPPING-MULTIOP-DISPATCH: older-family repeated-stage witness.
//!
//! Promise class: durable behavioral invariant. The same statically bounded
//! response owner that executes Mapping access chains must execute two and three
//! repeated `FsHandleMetadata` effects and preserve interpreter parity.

#![cfg(target_os = "linux")]

use std::os::unix::ffi::OsStrExt;

const SOURCE_TEMPLATE: &str = r#"program capabilities FS AFull
const body_ok_io : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyOk Unit Unit MkUnit)

proc metadata_chain (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
__CHAIN__

fn finish (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Err error |-> host_exit AFull (Failure 91);
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> host_exit AFull Success;
      ResourceBracketBodyError error |-> host_exit AFull (Failure 92);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 93);
      ResourceBracketBodyAndReleaseError body_error release_error |->
        host_exit AFull (Failure 94)
    }
  }

proc stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "held.bin")
      ResourceMetadata metadata_chain)
    (\outcome. finish outcome)

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |-> stage cap
  }
"#;

const TWO_CHAIN: &str = r#"  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError FileMetadata) (ResourceBodyResult Unit Unit)
    (resourceMetadata AFull file)
    (\first. bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError FileMetadata) (ResourceBodyResult Unit Unit)
      (resourceMetadata AFull file)
      (\second. body_ok_io))"#;

const THREE_CHAIN: &str = r#"  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError FileMetadata) (ResourceBodyResult Unit Unit)
    (resourceMetadata AFull file)
    (\first. bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError FileMetadata) (ResourceBodyResult Unit Unit)
      (resourceMetadata AFull file)
      (\second. bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result ResourceError FileMetadata) (ResourceBodyResult Unit Unit)
        (resourceMetadata AFull file)
        (\third. body_ok_io)))"#;

// Baseline provisioning, not a depth claim. Both programs built twice at
// 16 MiB during D0 without a stack failure; the named 16 MiB headroom makes the
// local Builder stack, rather than the ambient test-harness default, operative.
const FS_REPEAT_STACK_MEASURED_PEAK_BYTES: usize = 16 * 1024 * 1024;
const FS_REPEAT_STACK_HEADROOM_BYTES: usize = 16 * 1024 * 1024;
const FS_REPEAT_STACK_BYTES: usize =
    FS_REPEAT_STACK_MEASURED_PEAK_BYTES + FS_REPEAT_STACK_HEADROOM_BYTES;

fn run_case(case: &str, chain: &str, repeats: usize) {
    let source = SOURCE_TEMPLATE.replace("__CHAIN__", chain);
    let root = tempfile::Builder::new()
        .prefix(&format!("ken-rt-multiop-{case}-"))
        .tempdir()
        .expect("creates repeated-metadata root");
    std::fs::write(root.path().join("held.bin"), b"held resource")
        .expect("writes repeated-metadata source");

    let output = ken_cli::build_native_program(
        &source,
        ken_cli::SourceFormat::Ken,
        &format!("rt_multiop_{}", case.replace('-', "_")),
        root.path(),
    )
    .expect("the repeated-metadata program builds natively");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("the repeated-metadata native artifact executes");

    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_bytes(),
        &mut host,
    )
    .expect("the repeated-metadata program executes in the interpreter");

    assert_eq!((native.exit_status, interpreted.exit_status), (0, 0));
    assert_eq!(native.terminal_error, None);
    assert_eq!(interpreted.terminal_error, None);
    assert_eq!(native.terminal_exit, interpreted.terminal_exit);
    assert_eq!(native.effect_trace, interpreted.effect_trace);

    let operations = native
        .effect_trace
        .iter()
        .map(|event| event.operation)
        .collect::<Vec<_>>();
    let mut expected = vec![ken_runtime::HostOpV1::FsOpen];
    expected.extend(std::iter::repeat_n(
        ken_runtime::HostOpV1::FsHandleMetadata,
        repeats,
    ));
    expected.push(ken_runtime::HostOpV1::ResourceRelease);
    assert_eq!(operations, expected);
    for event in native
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::FsHandleMetadata)
    {
        let ken_runtime::CanonicalOutcomeV1::Success(ken_runtime::CanonicalReplyV1::FileMetadata(
            metadata,
        )) = &event.outcome
        else {
            panic!("metadata operation returned the wrong outcome: {event:?}");
        };
        assert_eq!(metadata.size, 13);
        assert_eq!(format!("{:?}", metadata.kind), "File");
    }
}

/// MEASURED: exact two- and three-effect older-FS programs execute in both
/// engines with identical results, exits, ordered traces, and terminal release.
/// CLAIMED: bounded repeated-stage dispatch is a general response-owner
/// capability rather than Mapping-only wiring. THE GAP: heterogeneous Mapping
/// planner agreement is independently exercised by the Mapping matrix.
#[test]
fn repeated_fs_metadata_effects_match_the_interpreter() {
    std::thread::Builder::new()
        .name("rt-multiop-fs-repeat".to_string())
        .stack_size(FS_REPEAT_STACK_BYTES)
        .spawn(|| {
            run_case("two-fs-metadata", TWO_CHAIN, 2);
            run_case("three-fs-metadata", THREE_CHAIN, 3);
        })
        .expect("starts repeated-metadata worker")
        .join()
        .expect("repeated-metadata worker completes");
}
