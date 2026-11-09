#![cfg(target_os = "linux")]

use ken_elaborator::compiler_driver::NativeProgramBuildError;
use ken_runtime::{
    LoweringCause, ObjectLinkerPackagingStage, PendingRefusal,
    SelectedPendingCallOutcomeObservation,
};

const SOURCE: &str = include_str!("fixtures/px10_native_tested_pending_overlap.ken");

/// Promise class: durable invariant while R3 makes an issued set the only
/// authority for its result. MEASURED: an actual NativeTested build reaches
/// pending admission and refuses at its own producer with a typed cause.
/// CLAIMED: the second pending-owner route cannot silently override the set.
/// THE GAP: no executable labelled parity follows from an object refusal.
#[test]
fn issued_set_refuses_second_pending_route_authority() {
    std::thread::Builder::new()
        .name("px10-pending-alternative-guard".to_owned())
        // Existing PX10 census measured 3292 KiB; 16 MiB includes 12 MiB headroom.
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            let dir = tempfile::tempdir().expect("guard output directory");
            let (result, rows) = ken_runtime::with_selected_pending_call_admissions(|| {
                ken_cli::build_native_program_with_roots(
                    SOURCE,
                    ken_cli::SourceFormat::Ken,
                    &checked_catalog_roots(),
                    "px10_native_tested_pending_overlap",
                    dir.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
            });
            let producers = rows
                .iter()
                .filter_map(|row| {
                    matches!(
                        &row.outcome,
                        SelectedPendingCallOutcomeObservation::Refused(
                            PendingRefusal::ProducerHasAlternativeSet
                        )
                    )
                    .then_some(row.producer)
                })
                .collect::<Vec<_>>();
            let [producer] = producers.as_slice() else {
                panic!("exactly one issued-set producer must refuse a second route: {rows:#?}");
            };
            let error = match result.expect_err("pending-owner route must not compete with R3") {
                NativeProgramBuildError::Packaging(error) => error,
                other => panic!("expected an object-emission refusal, got {other:?}"),
            };
            assert_eq!(error.stage, ObjectLinkerPackagingStage::ObjectEmission);
            assert_eq!(error.source_cause, Some(LoweringCause::PlannerInvariant));
            assert!(error.reason.contains("PendingCallAdmission"), "{error:?}");
            assert!(
                error.reason.contains("ProducerHasAlternativeSet"),
                "{error:?}"
            );
            assert!(
                error
                    .reason
                    .contains(&format!("producer StaticOriginId({producer})")),
                "the refusal must name its observed source producer: {error:?}"
            );
        })
        .expect("guard worker")
        .join()
        .expect("guard worker completed");
}

fn checked_catalog_roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")]
}
