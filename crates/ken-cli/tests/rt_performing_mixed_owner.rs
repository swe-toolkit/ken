//! A relay source in a mixed-owner stage cannot borrow a performing owner.
#![cfg(target_os = "linux")]

/// Promise class: durable invariant. MEASURED: a source-stage fixture whose
/// interpreter trace performs two FsReadFile operations reaches the typed
/// mixed-owner relay refusal before native object emission. CLAIMED: a relay
/// cannot silently promote a P2 source under the wrong owner. THE GAP: an
/// unrelated planner refusal could have the same code; the test-only guard
/// observation names the source and full edge at the mixed-stage gate.
#[test]
fn mixed_owner_relay_source_refuses_before_native_execution() {
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
            let interpreted = ken_cli::run_program_effect_observation(
                source, ken_cli::SourceFormat::Ken, &[], &[],
                root.path().as_os_str().as_bytes(), &mut host,
            ).expect("the interpreter executes the mixed-owner source");
            assert_eq!(interpreted.effect_trace.iter().filter(|event|
                event.operation == ken_runtime::HostOpV1::FsReadFile
            ).count(), 2, "the interpreter must reach both source reads");
            let (built, refusals) = ken_runtime::with_mixed_relay_refusal_observations(|| {
                ken_cli::build_native_program(
                    source, ken_cli::SourceFormat::Ken, "rt_performing_mixed_owner",
                    root.path(), ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
            });
            assert_eq!(refusals.len(), 1, "one exact mixed-owner pair/edge must reach the guard");
            assert!(!refusals[0].edge_identity.is_empty(), "the refused edge is causal, not source-only");
            let error = built.expect_err("mixed-owner relay source must refuse before native execution");
            assert_eq!(error.code().as_str(), "N0003");
            let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(packaging) = error
            else { panic!("the mixed-stage refusal must occur during native object planning") };
            assert_eq!(packaging.stage, ken_runtime::ObjectLinkerPackagingStage::ObjectEmission);
            assert_eq!(packaging.source_cause, Some(ken_runtime::LoweringCause::PlannerInvariant));
        })
        .expect("spawn stated-stack mixed-owner fixture")
        .join()
        .expect("mixed-owner fixture thread");
}
