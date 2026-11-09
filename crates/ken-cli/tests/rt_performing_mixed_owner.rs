//! Distinct provenance owners at one call cannot bypass a later planner guard.
#![cfg(target_os = "linux")]

/// Promise class: durable invariant. MEASURED: a Legacy-elaborated fixture
/// whose core term reaches both FsReadFile operations refuses with the typed
/// computational-occurrence PlannerInvariant before native object emission.
/// CLAIMED: W3's instance key does not admit a program that the subsequent
/// consumer cannot resolve. THE GAP: this fixture's separate provenance
/// owners cannot exercise the guard for one owner at one call emitted under
/// both predeclared and specialization owners: at call 386, source 717 has
/// provenance Pred4/emission Spec0 while source 731 has Pred5/Pred5. That
/// same-provenance mixed-emission residual is unexercised by this fixture.
#[test]
fn distinct_provenance_stage_refuses_at_unresolved_consumer_before_native_execution() {
    std::thread::Builder::new()
        .name("rt-performing-mixed-owner".to_owned())
        // The sibling branched-source baseline needed 2112 KiB; 16 MiB
        // supplies over sevenfold headroom for this semantic, not depth, test.
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            use std::os::unix::ffi::OsStrExt as _;
            let source = include_str!("fixtures/rt_performing_mixed_owner_relay.ken");
            let root = tempfile::tempdir().expect("temporary native-build root");
            std::fs::write(root.path().join("source"), b"0123456789abcdef")
                .expect("the fixture file is readable");
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let mut interpreter_env = ken_elaborator::ElabEnv::new().expect("checked prelude");
            let interpreter_results = interpreter_env
                .elaborate_file_v1(source)
                .expect("the relay elaborates into a checked core term");
            let interpreted = ken_cli::run_elaborated_program_effect_observation(
                interpreter_env,
                interpreter_results,
                &[],
                &[],
                root.path().as_os_str().as_bytes(),
                &mut host,
            )
            .expect("the interpreter executes the mixed-owner source");
            assert_eq!(
                interpreted
                    .effect_trace
                    .iter()
                    .filter(|event| event.operation == ken_runtime::HostOpV1::FsReadFile)
                    .count(),
                2,
                "the interpreter must reach both source reads"
            );
            let mut native_env = ken_elaborator::ElabEnv::new().expect("checked prelude");
            let native_results = native_env
                .elaborate_file_v1(source)
                .expect("the relay elaborates into a checked core term");
            let elaborated =
                ken_elaborator::compiler_driver::ElaboratedNativeProgramV1::from_elaborated_env(
                    native_env,
                    ken_elaborator::compiler_driver::CompilerSource::new("src/main.ken", source),
                    native_results,
                );
            let built = ken_elaborator::compiler_driver::prepare_native_program_from_elaborated_v1(
                "rt_performing_mixed_owner",
                elaborated,
            )
            .and_then(|preparation| {
                ken_elaborator::compiler_driver::complete_native_program_preparation(
                    preparation,
                    root.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
            });
            let error =
                built.expect_err("the unresolved consumer must refuse before native execution");
            assert_eq!(error.code().as_str(), "N0003");
            let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(packaging) =
                error
            else {
                panic!("the unresolved-consumer refusal must occur during native object planning")
            };
            assert_eq!(
                packaging.stage,
                ken_runtime::ObjectLinkerPackagingStage::ObjectEmission
            );
            assert_eq!(
                packaging.source_cause,
                Some(ken_runtime::LoweringCause::PlannerInvariant)
            );
            assert!(
                packaging.reason.contains(
                    "an exact detached required consumer has no computational occurrence"
                ),
                "the refusal must be the computational-occurrence wall: {}",
                packaging.reason
            );
        })
        .expect("spawn stated-stack mixed-owner fixture")
        .join()
        .expect("mixed-owner fixture thread");
}
