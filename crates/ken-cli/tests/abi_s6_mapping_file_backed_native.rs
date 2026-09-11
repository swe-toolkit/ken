//! ABI-S6 D5b: checked file-backed MAP_PRIVATE mapping differential.
//!
//! Promise class: normative compatibility vector from `38 §1.9` and
//! `conformance/surface/ffi-io/seed-mapping.md` case 1. The same checked program
//! runs through the native mmap-of-fd path and the interpreter's in-process
//! model. Both must observe a private write through `mapBytes`, while an ordinary
//! file read and the backing file itself retain the original bytes.

#![cfg(target_os = "linux")]

const SOURCE: &str = r#"program capabilities FS AFull
const body_ok_io : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyOk Unit Unit MkUnit)

const body_error_io : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyErr Unit Unit MkUnit)

proc after_mapping_write (mapping : MappingHandle)
  (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok unit |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
      (mapBytes AFull mapping (MkMappingWindow (2 : Int) (3 : Int)))
      (\read. match read {
        Err error |-> body_error_io;
        Ok bytes |-> body_ok_io
      })
  }

proc mapping_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (mapWrite AFull mapping (2 : Int) (bytes_encode "NEW"))
    (\outcome. after_mapping_write mapping outcome)

fn mapping_bracket_body
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Err error |-> body_error_io;
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> body_ok_io;
      ResourceBracketBodyError error |-> body_error_io;
      ResourceBracketReleaseError error |-> body_error_io;
      ResourceBracketBodyAndReleaseError body_error release_error |-> body_error_io
    }
  }

proc file_body (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withMapping AFull Unit Unit
      (FileBacked file (0 : Int) (8 : Int)) ReadWrite mapping_body)
    (\outcome. mapping_bracket_body outcome)

fn finish_file_read (outcome : Result FileError Bytes) : HostIO AFull ExitCode =
  match outcome {
    Err error |-> host_exit AFull (Failure 95);
    Ok bytes |-> host_exit AFull Success
  }

proc after_file_bracket (cap : Cap AFull)
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err error |-> host_exit AFull (Failure 91);
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError Bytes) ExitCode
        (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
          (Result FileError Bytes)
          (read_bytes AFull cap (bytes_encode "mapped.bin")))
        (\read. finish_file_read read);
      ResourceBracketBodyError error |-> host_exit AFull (Failure 92);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 93);
      ResourceBracketBodyAndReleaseError body_error release_error |->
        host_exit AFull (Failure 94)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
      (withResource AFull Unit Unit cap (bytes_encode "mapped.bin")
        (ResourceWriteCreate CreateOrKeep) file_body)
      (\outcome. after_file_bracket cap outcome)
  }
"#;

const ORIGINAL: &[u8] = b"abcdefgh";
const PRIVATE_WRITE: &[u8] = b"NEW";

struct Differential {
    interpreted: ken_runtime::EffectObservation,
    native: ken_runtime::EffectObservation,
    native_backing: Vec<u8>,
    interpreted_backing: Vec<u8>,
}

fn differential() -> Differential {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-file-cow-")
        .tempdir()
        .expect("creates temporary root");
    let path = root.path().join("mapped.bin");
    std::fs::write(&path, ORIGINAL).expect("writes native backing fixture");
    let output = ken_cli::build_native_program(
        SOURCE,
        ken_cli::SourceFormat::Ken,
        "abi_s6_d5b_file_cow",
        root.path(),
    )
    .expect("D5b checked source lowers natively");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("D5b native file mapping executes");
    let native_backing = std::fs::read(&path).expect("reads native backing file");

    std::fs::write(&path, ORIGINAL).expect("resets interpreter backing fixture");
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        SOURCE,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("D5b interpreted file mapping executes");
    let interpreted_backing = std::fs::read(&path).expect("reads interpreted backing file");

    Differential {
        interpreted,
        native,
        native_backing,
        interpreted_backing,
    }
}

fn operation_events(
    observation: &ken_runtime::EffectObservation,
    operation: ken_runtime::HostOpV1,
) -> Vec<&ken_runtime::EffectEvent> {
    observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == operation)
        .collect()
}

/// MEASURED: both engines execute the same checked nested resource brackets;
/// MappingAcquireFile uses the held FsHandle, mapWrite changes [2,5), mapBytes
/// returns `NEW`, both terminal releases succeed, and the later FsReadFile plus
/// the on-disk file still carry `abcdefgh`. CLAIMED: D5b is real file-backed
/// MAP_PRIVATE COW with copied views and no write-through. THE GAP: external
/// concurrent truncation is outside this single-invocation fixture; short-source
/// refusal and source-lineage revocation remain pinned at the shared dispatcher.
///
/// Mutation control: replace `file_map_flags_v1()`'s `MAP_PRIVATE` with
/// `MAP_SHARED` and run this unchanged test. The native backing and FsReadFile
/// assertions redden while the in-mapping read stays green, distinguishing COW
/// from the tempting write-through implementation.
#[test]
fn file_backed_mapping_is_private_in_both_engines_and_preserves_the_file() {
    use ken_runtime::HostOpV1::{
        FsOpen, FsReadFile, MappingAcquireFile, MappingReadView, MappingWriteView, ResourceRelease,
    };

    let result = differential();
    assert_eq!(
        (result.native.exit_status, result.interpreted.exit_status),
        (0, 0),
        "both engines take the checked program's success branch"
    );
    assert_eq!(result.native.terminal_error, None);
    assert_eq!(result.interpreted.terminal_error, None);
    assert_eq!(result.native_backing, ORIGINAL);
    assert_eq!(result.interpreted_backing, ORIGINAL);

    for observation in [&result.native, &result.interpreted] {
        assert_eq!(
            observation
                .effect_trace
                .iter()
                .map(|event| event.operation)
                .collect::<Vec<_>>(),
            vec![
                FsOpen,
                MappingAcquireFile,
                MappingWriteView,
                MappingReadView,
                ResourceRelease,
                ResourceRelease,
                FsReadFile,
            ],
            "nested mapping and file brackets settle before ordinary file read"
        );
        let acquisitions = operation_events(observation, MappingAcquireFile);
        assert_eq!(acquisitions.len(), 1);
        assert!(matches!(
            acquisitions[0].request,
            ken_runtime::CanonicalRequestV1::MappingAcquireFile {
                length: 8,
                protection: _,
            }
        ));
        let mapped_reads = operation_events(observation, MappingReadView);
        assert_eq!(mapped_reads.len(), 1);
        assert!(matches!(
            &mapped_reads[0].outcome,
            ken_runtime::CanonicalOutcomeV1::Success(
                ken_runtime::CanonicalReplyV1::Bytes(bytes)
            ) if bytes.as_slice() == PRIVATE_WRITE
        ));
        let file_reads = operation_events(observation, FsReadFile);
        assert_eq!(file_reads.len(), 1);
        assert!(matches!(
            &file_reads[0].outcome,
            ken_runtime::CanonicalOutcomeV1::Success(
                ken_runtime::CanonicalReplyV1::Bytes(bytes)
            ) if bytes.as_slice() == ORIGINAL
        ));
        assert_eq!(operation_events(observation, ResourceRelease).len(), 2);
    }

    assert_eq!(
        result.native.effect_trace, result.interpreted.effect_trace,
        "native mmap and interpreter model preserve exact opaque trace semantics"
    );
    assert_eq!(
        result.native.terminal_exit,
        result.interpreted.terminal_exit
    );
}
