//! Promise class: durable invariant (38 §1.9, seed-mapping case 1).
//! MEASURED: both engines write and read a private file-backed region, then
//! read the unchanged source through ordinary §1.3 I/O after both releases.
//! CLAIMED: file-backed mapWrite is process-local and cannot write through to
//! the source file. THE GAP: a write-through production mutant must change the
//! bytes on disk and redden the file-read assertion, not fail at acquisition.

#![cfg(target_os = "linux")]

const SOURCE: &str = include_str!("fixtures/abi_s6_mapping_file_backed_cow.ken");
const ORIGINAL: &[u8] = b"KENFILE!";
const PRIVATE_WRITE: &[u8] = b"ABCD";
const WRITTEN_FILE: &[u8] = b"KEABCDE!";

type Observation = ken_runtime::EffectObservation;

fn differential(source: &str) -> Result<(Observation, Observation, Vec<u8>, Vec<u8>), String> {
    let build = tempfile::tempdir().map_err(|error| format!("build root: {error}"))?;
    let native_root = tempfile::tempdir().map_err(|error| format!("native root: {error}"))?;
    let interpreted_root =
        tempfile::tempdir().map_err(|error| format!("interpreter root: {error}"))?;
    for root in [native_root.path(), interpreted_root.path()] {
        std::fs::write(root.join("mapped.bin"), ORIGINAL)
            .map_err(|error| format!("seed mapping file: {error}"))?;
    }
    let built = ken_cli::build_native_program(
        source,
        ken_cli::SourceFormat::Ken,
        "abi_s6_mapping_file_backed_cow",
        build.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .map_err(|error| format!("native build: {error:?}"))?;
    let native = ken_runtime::run_bound_process_effect_observation(
        &built.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: native_root.path().to_owned(),
            plan_hash: built.plan_transport_hash,
        },
    )
    .map_err(|error| format!("native execution: {error:?}"))?;
    let mut host = ken_interp::PosixHost::new_at(interpreted_root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        interpreted_root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .map_err(|error| format!("interpreter execution: {error:?}"))?;
    let interpreted_file = std::fs::read(interpreted_root.path().join("mapped.bin"))
        .map_err(|error| format!("interpreter file readback: {error}"))?;
    let native_file = std::fs::read(native_root.path().join("mapped.bin"))
        .map_err(|error| format!("native file readback: {error}"))?;
    Ok((interpreted, native, interpreted_file, native_file))
}

#[test]
fn file_backed_mapping_write_is_private_on_both_engines() {
    use ken_runtime::{CanonicalOutcomeV1, CanonicalReplyV1, CanonicalRequestV1, HostOpV1};

    let (interpreted, native, interpreted_file, native_file) =
        differential(SOURCE).expect("private file-backed mapping differential");
    for lane in [&interpreted, &native] {
        assert_eq!(
            lane.exit_status, 60,
            "effect trace: {:?}",
            lane.effect_trace
        );
        assert_eq!(lane.terminal_error, None);
        assert_eq!(
            lane.effect_trace
                .iter()
                .map(|event| event.operation)
                .collect::<Vec<_>>(),
            vec![
                HostOpV1::FsOpen,
                HostOpV1::MappingAcquireFile,
                HostOpV1::MappingWriteView,
                HostOpV1::MappingReadView,
                HostOpV1::ResourceRelease,
                HostOpV1::ResourceRelease,
                HostOpV1::FsReadFile,
            ],
        );
        assert_eq!(
            lane.effect_trace[0].request,
            CanonicalRequestV1::FsOpen {
                path: b"mapped.bin".to_vec(),
                mode: ken_host::FsOpenModeV1::Read,
            },
        );
        assert_eq!(
            lane.effect_trace[1].request,
            CanonicalRequestV1::MappingAcquireFile {
                length: 8,
                protection: ken_host::MappingProtectionV1::Writable,
            },
        );
        assert!(matches!(
            &lane.effect_trace[1].outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::ResourceAcquired { .. })
        ));
        assert_eq!(
            lane.effect_trace[2].request,
            CanonicalRequestV1::MappingWriteView {
                start: 2,
                bytes: PRIVATE_WRITE.to_vec(),
            },
        );
        assert!(matches!(
            &lane.effect_trace[2].outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit)
        ));
        assert!(matches!(
            &lane.effect_trace[3].outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::Bytes(bytes))
                if bytes.as_slice() == PRIVATE_WRITE
        ));
        assert!(matches!(
            &lane.effect_trace[6].outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::Bytes(_))
        ));
    }
    let read_back = [&interpreted, &native].map(|lane| match &lane.effect_trace[6].outcome {
        CanonicalOutcomeV1::Success(CanonicalReplyV1::Bytes(bytes)) => bytes.as_slice(),
        other => panic!("expected an ordinary file-read reply, got {other:?}"),
    });
    assert_eq!(
        read_back,
        [ORIGINAL, ORIGINAL],
        "both engines' §1.3 file reads must return original bytes, not {WRITTEN_FILE:?}; disk: interpreter={interpreted_file:?}, native={native_file:?}"
    );
    assert_eq!(
        [interpreted_file.as_slice(), native_file.as_slice()],
        [ORIGINAL, ORIGINAL],
        "neither backing file may be changed by the mapping write"
    );
    assert_eq!(interpreted.effect_trace, native.effect_trace);
    assert_eq!(interpreted.filesystem_delta, native.filesystem_delta);
}
