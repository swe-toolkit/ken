//! Distinct provenance owners at one call execute with interpreter parity.
#![cfg(target_os = "linux")]

/// Promise class: durable invariant. MEASURED: native trace, release set,
/// exit status, and terminal equal the interpreter's for the distinct-
/// provenance stage. CLAIMED: W3's instance key admits this program because
/// the required consumer resolves at the worker boundary's selecting
/// ComputationalMatch continuation (Plane A); M-PLANEA restores the typed
/// PlannerInvariant refusal. THE GAP: this fixture's separate provenance
/// owners cannot exercise the guard for one owner at one call emitted under
/// both predeclared and specialization owners: at call 386, source 717 has
/// provenance Pred4/emission Spec0 while source 731 has Pred5/Pred5. That
/// same-provenance mixed-emission residual is unexercised by this fixture.
#[test]
fn distinct_provenance_stage_matches_interpreter_natively() {
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
                source,
                ken_cli::SourceFormat::Ken,
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
            let compiled = ken_cli::build_native_program(
                source,
                ken_cli::SourceFormat::Ken,
                "rt_performing_mixed_owner",
                root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .expect(
                "the distinct-provenance stage resolves its consumer at the selecting continuation",
            );
            let native = ken_runtime::run_bound_process_effect_observation(
                &compiled.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: Vec::new(),
                    cwd: root.path().to_owned(),
                    plan_hash: compiled.plan_transport_hash,
                },
            )
            .expect("the native artifact executes to a recorded terminal");
            let non_release = |observation: &ken_runtime::EffectObservation| {
                observation
                    .effect_trace
                    .iter()
                    .filter(|event| event.operation != ken_runtime::HostOpV1::ResourceRelease)
                    .cloned()
                    .collect::<Vec<_>>()
            };
            let release_set = |observation: &ken_runtime::EffectObservation| {
                let mut events = observation
                    .effect_trace
                    .iter()
                    .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
                    .map(|event| {
                        format!(
                            "{:?}",
                            (&event.resource_bindings, &event.request, &event.outcome)
                        )
                    })
                    .collect::<Vec<_>>();
                events.sort();
                events
            };
            let interpreter_ops = non_release(&interpreted);
            assert_eq!(
                interpreter_ops
                    .iter()
                    .map(|event| event.operation)
                    .collect::<Vec<_>>(),
                vec![
                    ken_runtime::HostOpV1::FsOpen,
                    ken_runtime::HostOpV1::BufferAllocate,
                    ken_runtime::HostOpV1::FsReadFile,
                    ken_runtime::HostOpV1::FsReadFile,
                ],
                "the oracle really performs both source reads"
            );
            assert_eq!(
                interpreted.exit_status, 72,
                "fixture's distinct-provenance success branch"
            );
            assert_eq!(release_set(&native), release_set(&interpreted));
            let native_ops = non_release(&native);
            assert_eq!(
                native_ops, interpreter_ops,
                "the stage must perform both reads"
            );
            assert_eq!(native.exit_status, interpreted.exit_status);
            assert_eq!(native.terminal_exit, interpreted.terminal_exit);
            assert_eq!(native.terminal_error, interpreted.terminal_error);
        })
        .expect("spawn stated-stack mixed-owner fixture")
        .join()
        .expect("mixed-owner fixture thread");
}
