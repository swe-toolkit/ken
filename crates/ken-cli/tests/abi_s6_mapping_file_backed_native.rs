//! Promise class: normative compatibility vector, 38 §1.9. A checked
//! file-backed mapping reads the real file through both engines, then the
//! inner ResourceBracketOk consumer turns the result into exit 60. Changing
//! the consumer's success arm to a body-Ok must instead produce exit 0.

#![cfg(target_os = "linux")]

const SOURCE: &str = include_str!("fixtures/abi_s6_mapping_file_backed.ken");

fn differential(
    source: &str,
) -> Result<
    (
        ken_runtime::EffectObservation,
        ken_runtime::EffectObservation,
    ),
    String,
> {
    let build = tempfile::tempdir().map_err(|error| format!("build root: {error}"))?;
    let native_root = tempfile::tempdir().map_err(|error| format!("native root: {error}"))?;
    let interpreted_root =
        tempfile::tempdir().map_err(|error| format!("interpreter root: {error}"))?;
    for root in [native_root.path(), interpreted_root.path()] {
        std::fs::write(root.join("mapped.bin"), b"KENFILE!")
            .map_err(|error| format!("seed mapping file: {error}"))?;
    }
    let built = ken_cli::build_native_program(
        source,
        ken_cli::SourceFormat::Ken,
        "abi_s6_mapping_file_backed",
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
    Ok((interpreted, native))
}

#[test]
fn file_backed_mapping_executes_its_inner_result_consumer_on_both_engines() {
    let (interpreted, native) = differential(SOURCE).expect("file-backed differential");
    assert_eq!(interpreted.exit_status, 60);
    assert_eq!(native.exit_status, 60);
    assert_eq!(interpreted.terminal_error, None);
    assert_eq!(native.terminal_error, None);
    assert_eq!(interpreted.effect_trace, native.effect_trace);
    assert_eq!(interpreted.filesystem_delta, native.filesystem_delta);
    assert_eq!(
        native
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::FsOpen,
            ken_runtime::HostOpV1::MappingAcquireFile,
            ken_runtime::HostOpV1::MappingReadView,
            ken_runtime::HostOpV1::ResourceRelease,
            ken_runtime::HostOpV1::ResourceRelease,
        ],
    );
    assert!(native.filesystem_delta.is_empty());
    assert!(matches!(
        &native.effect_trace[2].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(ken_runtime::CanonicalReplyV1::Bytes(bytes))
            if bytes.as_slice() == b"NFIL"
    ));
}

#[test]
fn inner_resource_bracket_ok_arm_is_not_skipped_or_reapplied() {
    let mutated = SOURCE.replace(
        "ResourceBracketOk _ |-> body_err_io 60",
        "ResourceBracketOk _ |-> body_ok_io",
    );
    let (interpreted, native) = differential(&mutated).expect("consumer-arm mutation differential");
    assert_eq!(interpreted.exit_status, 0);
    assert_eq!(native.exit_status, 0);
    assert_eq!(interpreted.effect_trace, native.effect_trace);
}
