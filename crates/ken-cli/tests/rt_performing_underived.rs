//! A test-only failure at the returned-Vis protocol producer makes every
//! reader see an Underived installed owner without changing the program IR.
//! A natural user-source Underived owner has not been constructed.
#![cfg(target_os = "linux")]

/// Promise class: transition sentinel. The forced origin numbers are operands
/// of this fixed AC-MAIN fixture, not a general inventory of owner origins.
/// Retire or rederive them if a natural user-source Underived owner is found
/// or this fixture's owner enumeration changes.
/// MEASURED: the unforced source installs both owners without Underived; the
/// same source forced at each owner's protocol producer either refuses before
/// artifact emission or executes with interpreter-equivalent effects and
/// terminal outcome. CLAIMED: classifying an installed owner Underived cannot
/// silently drop an interpreter-performed host operation. THE GAP: a build
/// refusal does not witness native execution; only an emitted artifact is
/// compared to the independent interpreter trace, and the classification
/// itself must be observed on any published plan.
#[test]
fn forced_underived_owner_either_refuses_or_preserves_performed_operations() {
    std::thread::Builder::new()
        .name("rt-performing-underived".to_owned())
        // The sibling branched-source baseline needed 2112 KiB; this 16 MiB
        // grant exceeds that measured peak by over sevenfold. This test is
        // about returned host operations, not the ambient libtest stack.
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            use std::os::unix::ffi::OsStrExt as _;
            let source = include_str!("fixtures/rt_performing_underived_acmain.ken");
            let root = tempfile::tempdir().expect("temporary native-build root");
            std::fs::write(root.path().join("source"), b"0123456789abcdef")
                .expect("fixture file is readable");
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let interpreted = ken_cli::run_program_effect_observation(
                source,
                ken_cli::SourceFormat::Ken,
                &[],
                &[],
                root.path().as_os_str().as_bytes(),
                &mut host,
            )
            .expect("the forwarded read executes in the interpreter");
            let non_release = |observation: &ken_runtime::EffectObservation| {
                observation
                    .effect_trace
                    .iter()
                    .filter(|event| event.operation != ken_runtime::HostOpV1::ResourceRelease)
                    .cloned()
                    .collect::<Vec<_>>()
            };
            let release_set = |observation: &ken_runtime::EffectObservation| {
                let mut releases = observation
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
                releases.sort();
                releases
            };
            assert!(
                non_release(&interpreted)
                    .iter()
                    .any(|event| event.operation == ken_runtime::HostOpV1::FsReadAt),
                "the independent interpreter must perform the forwarded read"
            );

            // The unforced row is the owner-installation control. Each forced
            // row changes only the protocol classification of that same source.
            for forced_owner in [None, Some(898_u32), Some(776_u32)] {
                let build = || {
                    ken_cli::build_native_program(
                        source,
                        ken_cli::SourceFormat::Ken,
                        "rt_performing_underived",
                        root.path(),
                        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                    )
                };
                let observe = || ken_runtime::with_static_response_feasibility_diagnostics(build);
                let (built, diagnostics) = match forced_owner {
                    Some(owner) => ken_runtime::with_forced_underived_owner(owner, observe),
                    None => observe(),
                };
                if let Some(plan) = diagnostics.first() {
                    assert_eq!(diagnostics.len(), 1, "one program publishes one plan");
                    assert_eq!(
                        plan.underived_owner_origins,
                        forced_owner.into_iter().collect::<Vec<_>>(),
                        "every published plan must classify exactly the forced owner"
                    );
                    if forced_owner.is_none() {
                        let rows = plan
                            .all_static_response_rows
                            .iter()
                            .map(|row| (row.vis_origin, row.operation.as_str()))
                            .collect::<Vec<_>>();
                        assert!(
                            rows.contains(&(776, "FsReadAt"))
                                && rows.contains(&(898, "BufferAllocate")),
                            "the unforced fixture must install both forced operands: {rows:?}"
                        );
                        assert_eq!(
                            plan.static_response_owners.len(),
                            2,
                            "both baseline response owners must be installed"
                        );
                    }
                } else {
                    assert!(
                        forced_owner.is_some() && built.is_err(),
                        "only a forced pre-plan refusal may omit the diagnostic"
                    );
                }

                match built {
                    Ok(compiled) => {
                        assert_eq!(
                            diagnostics.len(),
                            1,
                            "an emitted artifact must carry its classification"
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
                        .expect("native returns a recorded terminal");
                        assert_eq!(
                            non_release(&native),
                            non_release(&interpreted),
                            "forced owner {forced_owner:?} cannot omit an interpreter-performed operation"
                        );
                        assert_eq!(release_set(&native), release_set(&interpreted));
                        assert_eq!(native.exit_status, interpreted.exit_status);
                        assert_eq!(native.terminal_exit, interpreted.terminal_exit);
                        assert_eq!(native.terminal_error, interpreted.terminal_error);
                        assert!(
                            forced_owner.is_none(),
                            "an Underived owner must refuse before artifact emission"
                        );
                    }
                    Err(error) => {
                        assert!(forced_owner.is_some(), "unforced AC-MAIN must build: {error:?}");
                        eprintln!("forced Underived owner {forced_owner:?} refused before artifact: {error}");
                        let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(
                            packaging,
                        ) = &error
                        else {
                            panic!("the forced native refusal needs a typed packaging cause: {error:?}")
                        };
                        assert!(
                            packaging.source_cause.is_some(),
                            "the refusal needs a typed native cause: {packaging:?}"
                        );
                        assert!(
                            error.to_string().contains("underivable returned-Vis protocol"),
                            "each forced owner must refuse at the Underived settlement guard: {error:?}"
                        );
                        if diagnostics.is_empty() {
                            assert!(
                                error.to_string().contains("test-forced underivable returned-Vis protocol"),
                                "a pre-plan refusal must reach the forced protocol producer: {error:?}"
                            );
                        }
                    }
                }
            }
        })
        .expect("spawn stated-stack Underived fixture")
        .join()
        .expect("Underived fixture thread");
}
