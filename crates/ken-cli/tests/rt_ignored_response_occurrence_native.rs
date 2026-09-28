//! A two-sequential-bracket witness for distinct host response occurrences.
//!
//! The source is checked by the ordinary compiler, not a hand-authored runtime
//! tree. Its native first refusal is a transition sentinel for the independent
//! carried-IH successor, not a claim that either bracket runs natively yet.

#[cfg(target_os = "linux")]
const TWO_BUFFER_WITNESS: &str = include_str!("rt_ignored_two_buffer_witness.ken");

// Measured on this exact checked witness: a stated 2 MiB worker aborts from
// stack overflow; 4 MiB completes the planner and reaches BoundaryCarrier.
// Provision that measured upper bound plus 4 MiB of local headroom. This
// baseline is not a depth/stack claim, and the explicit Builder budget takes
// precedence over ambient RUST_MIN_STACK.
#[cfg(target_os = "linux")]
const WITNESS_STACK_BYTES: usize = 4 * 1024 * 1024 + 4 * 1024 * 1024;

#[cfg(target_os = "linux")]
#[test]
fn sequential_brackets_reach_the_carried_ih_successor_boundary() {
    std::thread::Builder::new()
        .name("rt-ignored-sequential-brackets".to_string())
        .stack_size(WITNESS_STACK_BYTES)
        .spawn(run_sequential_brackets_witness)
        .expect("spawn stated-stack witness")
        .join()
        .expect("sequential-brackets witness thread");
}

#[cfg(target_os = "linux")]
fn run_sequential_brackets_witness() {
    let output = tempfile::Builder::new()
        .prefix("ken-rt-ignored-two-buffer-")
        .tempdir()
        .expect("unique native output root");
    let (compiled, diagnostics) = ken_runtime::with_static_response_feasibility_diagnostics(|| {
        ken_cli::build_native_program(
            TWO_BUFFER_WITNESS,
            ken_cli::SourceFormat::Ken,
            "rt_ignored_two_buffer_witness",
            output.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    let rows = diagnostics
        .iter()
        .flat_map(|plan| &plan.all_static_response_rows)
        .filter(|row| row.operation == "ResourceRelease")
        .collect::<Vec<_>>();
    assert_eq!(
        rows.len(),
        2,
        "the actual checked witness must plan two response-Vis rows"
    );
    assert_ne!(rows[0].vis_origin, rows[1].vis_origin);
    assert_ne!(
        rows[0].producer_call_origin, rows[1].producer_call_origin,
        "two occurrences of one response constructor must select distinct calls"
    );
    assert_ne!(
        rows[0].selected_dispatch_root, rows[1].selected_dispatch_root,
        "the selected calls must belong to distinct dispatch roots"
    );
    for row in &rows {
        assert!(
            row.eliminating_cm.is_some(),
            "a selected root must have a Vis-case CM: {row:?}"
        );
        assert!(
            row.cm_scrutinee_contains_vis,
            "the eliminating CM's scrutinee must contain the very Vis it dispatches: {row:?}"
        );
    }
    assert_ne!(rows[0].eliminating_cm, rows[1].eliminating_cm);

    let error = compiled.expect_err("the independent carried-IH successor is still required");
    let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(error) = error else {
        panic!("the native refusal must occur in packaging, not earlier: {error:?}");
    };
    assert_eq!(
        error.stage,
        ken_runtime::ObjectLinkerPackagingStage::ObjectEmission,
        "the planner must admit both host response occurrences"
    );
    assert_eq!(error.field, "checked_process_object");
    assert_eq!(
        error.reason,
        "unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive hypothesis is an eliminated value, not a callable, so it takes no arguments, but the call provides 1",
        "this refusal is the new first native boundary, not a successful run"
    );
}
