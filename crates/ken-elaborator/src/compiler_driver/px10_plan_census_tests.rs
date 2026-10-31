//! PX10-NATIVE-SITE-IDENTITY AC-2: how far the three px10 programs plan.
//!
//! The Process gate refuses all three before object emission, so this census
//! hands each preparation straight to the emission the gate guards. Nothing
//! here runs a native program; each still stops at a wall the
//! PX10-NATIVE-BRANCH-ALTERNATIVES node owns.

use super::*;

const SMOKE: &str = include_str!("../../../ken-cli/tests/fixtures/px10_child_wait_smoke.ken");
const ARGV_ENV_PIPE: &str = include_str!("../../../ken-cli/tests/fixtures/px10_argv_env_pipe.ken");
const LIVE_CHILD: &str = include_str!("../../../ken-cli/tests/fixtures/px10_live_child.ken");

fn emit_below_gate(
    name: &str,
    source: &str,
) -> Result<NativeProgramBuildOutput, NativeProgramBuildError> {
    let preparation =
        prepare_native_program_sources(name, vec![CompilerSource::new("src/main.ken", source)])
            .expect("the px10 program prepares");
    assert!(
        preparation.plan.process_declared,
        "the census must plan a program the Process gate refuses"
    );
    let root = tempfile::tempdir().expect("census output root");
    emit_native_program_preparation(
        preparation,
        root.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
}

fn in_census_thread(name: &str, run: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name(name.to_owned())
        // The px10 native-build suites measured 3292 KiB resident stack for
        // these programs; 16 MiB is the stack they state.
        .stack_size(16 * 1024 * 1024)
        .spawn(run)
        .expect("spawn census thread")
        .join()
        .expect("census thread");
}

fn assert_object_emission_refusal(
    built: Result<NativeProgramBuildOutput, NativeProgramBuildError>,
) {
    let error = built.expect_err("each px10 program still stops before an artifact");
    let NativeProgramBuildError::Packaging(packaging) = error else {
        panic!("the census refusal must come from native object planning: {error:?}");
    };
    assert_eq!(
        packaging.stage,
        ken_runtime::ObjectLinkerPackagingStage::ObjectEmission
    );
    assert_eq!(
        packaging.source_cause,
        Some(ken_runtime::LoweringCause::PlannerInvariant)
    );
}

/// Promise class: transition sentinel, retired by
/// PX10-NATIVE-BRANCH-ALTERNATIVES. MEASURED: the smoke program's static
/// transition plan completes, recording one feasibility diagnostic with no
/// returned-Vis protocol error. CLAIMED: the return fold is found by the tail
/// up-walk from the acquire dispatch, and a returned bracket settlement is owned
/// by the Vis whose response returns it, not by the protocol root. THE GAP:
/// lowering then refuses at the detached-result seat, so nothing executes.
#[test]
fn px10_smoke_plans_past_return_fold_and_returned_settlement() {
    in_census_thread("px10-census-smoke", || {
        let (built, diagnostics) =
            ken_runtime::with_static_response_feasibility_diagnostics(|| {
                emit_below_gate("px10_census_smoke", SMOKE)
            });
        assert_eq!(
            diagnostics.len(),
            1,
            "the smoke plan completes exactly once"
        );
        for protocol in &diagnostics[0].returned_vis_protocols {
            assert!(
                protocol.error.is_none(),
                "returned-Vis protocol refused: {protocol:?}"
            );
        }
        assert_object_emission_refusal(built);
    });
}

/// Promise class: transition sentinel, retired by
/// PX10-NATIVE-BRANCH-ALTERNATIVES. MEASURED: argv_env_pipe and live_child
/// each reach exactly one typed mixed-owner relay refusal. CLAIMED: planning
/// gets past the return fold and, for argv_env_pipe, past the host-response
/// leaf walk through closure continuations, to the mixed-owner transport
/// stage. THE GAP: that stage stays refused here.
#[test]
fn px10_argv_and_live_plan_to_the_mixed_owner_stage() {
    for (name, source) in [
        ("px10_census_argv_env_pipe", ARGV_ENV_PIPE),
        ("px10_census_live_child", LIVE_CHILD),
    ] {
        in_census_thread(name, move || {
            let (built, refusals) = ken_runtime::with_mixed_relay_refusal_observations(|| {
                emit_below_gate(name, source)
            });
            assert_eq!(
                refusals.len(),
                1,
                "{name}: one mixed-owner stage reaches the guard"
            );
            assert!(
                !refusals[0].edge_identity.is_empty(),
                "{name}: the refused edge is causal"
            );
            assert_object_emission_refusal(built);
        });
    }
}
