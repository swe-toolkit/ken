//! A computed host operation in a response owner's K makes its returned-Vis
//! protocol Underived; that classification is not permission to drop the op.
#![cfg(target_os = "linux")]

/// Promise class: durable invariant. MEASURED: the planner marks an owner
/// Underived, while the independent interpreter actually performs FsReadFile.
/// CLAIMED: native either performs the same ordered effects and terminal
/// outcome or refuses before execution. THE GAP: the typed Underived record
/// must belong to this compiling program, not merely occur in an unrelated
/// fixture; the captured one-plan diagnostic and independent operation trace
/// bind the two observations to this source.
#[test]
fn computed_returned_vis_operation_is_performed_or_refused_without_dropping_it() {
    std::thread::Builder::new()
        .name("rt-performing-underived".to_owned())
        // The sibling branched-source baseline needed 2112 KiB; this 16 MiB
        // grant exceeds that measured peak by over sevenfold. This test is
        // about returned host operations, not the ambient libtest stack.
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            use std::os::unix::ffi::OsStrExt as _;
            let source = include_str!("fixtures/rt_performing_underived_computed_read.ken");
            let root = tempfile::tempdir().expect("temporary native-build root");
            std::fs::write(root.path().join("source"), b"0123456789abcdef")
                .expect("fixture file is readable");
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let interpreted = ken_cli::run_program_effect_observation(
                source, ken_cli::SourceFormat::Ken, &[], &[],
                root.path().as_os_str().as_bytes(), &mut host,
            ).expect("the computed read executes in the interpreter");
            let non_release = |observation: &ken_runtime::EffectObservation| {
                observation.effect_trace.iter()
                    .filter(|event| event.operation != ken_runtime::HostOpV1::ResourceRelease)
                    .cloned().collect::<Vec<_>>()
            };
            let release_set = |observation: &ken_runtime::EffectObservation| {
                let mut releases = observation.effect_trace.iter()
                    .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
                    .map(|event| format!("{:?}", (
                        &event.resource_bindings, &event.request, &event.outcome,
                    ))).collect::<Vec<_>>();
                releases.sort();
                releases
            };
            assert!(non_release(&interpreted).iter().any(|event|
                event.operation == ken_runtime::HostOpV1::FsReadAt
            ), "the interpreter must reach the baseline forwarded operation");
            let (built, diagnostics) = ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_cli::build_native_program(
                    source, ken_cli::SourceFormat::Ken, "rt_performing_underived",
                    root.path(), ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
            });
            eprintln!("UNDERIVED_BUILD error={:?} diagnostics={}", built.as_ref().err(), diagnostics.len());
            if let Some(plan) = diagnostics.first() {
                eprintln!("UNDERIVED_OWNER_SET {:?}", plan.static_response_owners.iter().map(|owner|
                    (owner.owner, owner.response, owner.selected_caller.as_str())).collect::<Vec<_>>());
                eprintln!("UNDERIVED_OWNER_ROWS {:?}", plan.all_static_response_rows.iter().map(|row|
                    (row.vis_origin, row.k_body_origin, row.operation.as_str())).collect::<Vec<_>>());
                eprintln!("UNDERIVED_CLASS {:?}", plan.underived_owner_origins);
            }
            assert_eq!(diagnostics.len(), 1, "one program publishes one plan");
            assert!(!diagnostics[0].underived_owner_origins.is_empty(),
                "the computed K return must actually reach an Underived response owner: {diagnostics:?}");
            match built {
                Ok(compiled) => {
                    let native = ken_runtime::run_bound_process_effect_observation(
                        &compiled.artifact,
                        &ken_runtime::NativeEffectRunOptionsV1 {
                            arguments: Vec::new(), environment: Vec::new(),
                            cwd: root.path().to_owned(), plan_hash: compiled.plan_transport_hash,
                        },
                    ).expect("native returns a recorded terminal");
                    assert_eq!(non_release(&native), non_release(&interpreted));
                    assert_eq!(release_set(&native), release_set(&interpreted));
                    assert_eq!(native.exit_status, interpreted.exit_status);
                    assert_eq!(native.terminal_exit, interpreted.terminal_exit);
                    assert_eq!(native.terminal_error, interpreted.terminal_error);
                }
                Err(error) => {
                    // A typed pre-execution native refusal is honest; an
                    // emitted artifact that silently omits FsReadFile is not.
                    let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(packaging) =
                        &error else { panic!("the native refusal must occur during object packaging: {error:?}") };
                    assert_eq!(packaging.stage, ken_runtime::ObjectLinkerPackagingStage::ObjectEmission);
                    assert!(packaging.source_cause.is_some(), "the refusal needs a typed native cause");
                }
            }
        })
        .expect("spawn stated-stack Underived fixture")
        .join()
        .expect("Underived fixture thread");
}
