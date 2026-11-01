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

/// The object-emission refusal each px10 program stops at, with its wall text.
fn assert_object_emission_wall(
    built: Result<NativeProgramBuildOutput, NativeProgramBuildError>,
    wall: &str,
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
    assert!(
        packaging.reason.contains(wall),
        "expected the wall `{wall}`, got: {}",
        packaging.reason
    );
}

/// Each static region exit (a region's settlement Vis) has exactly one owner
/// per plane, and the planes name exactly `expected`.
fn assert_region_exit_owners(
    name: &str,
    observed: &[ken_runtime::RegionExitOwnerObservation],
    expected: &[(ken_runtime::RegionExitPlane, u32, u32)],
) {
    let mut owners = std::collections::BTreeMap::<_, Vec<u32>>::new();
    for row in observed {
        owners
            .entry((row.plane, row.exit))
            .or_default()
            .push(row.owner);
    }
    for ((plane, exit), found) in &owners {
        assert_eq!(
            found.len(),
            1,
            "{name}: exit {exit} has {found:?} as {plane:?} owners"
        );
    }
    let observed: Vec<_> = observed
        .iter()
        .map(|row| (row.plane, row.exit, row.owner))
        .collect();
    assert_eq!(observed, expected, "{name}: region exit owners");
}

/// Promise class: transition sentinel, retired by
/// PX10-NATIVE-BRANCH-ALTERNATIVES. MEASURED: the smoke program's static
/// transition plan completes, recording one feasibility diagnostic with no
/// returned-Vis protocol error, and lowering stops at the P2 wall. Its one
/// region exit, settlement 647, resumes through fold 12 and is returned by Vis
/// 682. CLAIMED: the return fold is found by the tail up-walk from the acquire
/// dispatch, a returned bracket settlement is owned by the Vis whose response
/// returns it, and every specialized closure call reaches its graph-derived
/// target. THE GAP: the detached-result seat refuses, so nothing executes.
#[test]
fn px10_smoke_plans_past_return_fold_and_returned_settlement() {
    use ken_runtime::RegionExitPlane::{ReturnFold, ReturnedVisSettlement};
    in_census_thread("px10-census-smoke", || {
        let ((built, owners), diagnostics) =
            ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_runtime::with_region_exit_owner_observations(|| {
                    emit_below_gate("px10_census_smoke", SMOKE)
                })
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
        assert_region_exit_owners(
            "smoke",
            &owners,
            &[(ReturnFold, 647, 12), (ReturnedVisSettlement, 647, 682)],
        );
        assert_object_emission_wall(
            built,
            "the detached-result seat projected 2 undischarged causal calls onto one unit result",
        );
    });
}

/// Promise class: transition sentinel, retired by this WP's Increment 2.
/// MEASURED: argv_env_pipe reaches the exact W7 caller-cut wall, while
/// live_child reaches the P2 detached-result seat with two producers for
/// result 1104. Each retains its exact static region-exit owners.
/// CLAIMED: the W3 instance key admits the distinct provenance owners at
/// one call without weakening either downstream one-producer guard.
/// THE GAP: neither program is built or executed natively at these walls.
#[test]
fn px10_argv_and_live_advance_to_w7_and_p2() {
    use ken_runtime::RegionExitPlane::{ReturnFold, ReturnedVisSettlement};
    for (name, source, wall, expected, seat) in [
        (
            "px10_census_argv_env_pipe",
            ARGV_ENV_PIPE,
            "one checked-IH caller transport has more than one detached source return cut",
            &[
                (ReturnFold, 1160, 571),
                (ReturnFold, 1429, 1165),
                (ReturnedVisSettlement, 1160, 1984),
                (ReturnedVisSettlement, 1429, 1537),
            ][..],
            None,
        ),
        (
            "px10_census_live_child",
            LIVE_CHILD,
            "the detached-result seat projected 2 undischarged causal calls onto one unit result",
            &[
                (ReturnFold, 601, 12),
                (ReturnFold, 870, 606),
                (ReturnedVisSettlement, 601, 1488),
                (ReturnedVisSettlement, 870, 1043),
            ][..],
            Some(ken_runtime::DetachedResultSeatRefusalObservation {
                result: 1104,
                constructs: vec![1073, 1101],
            }),
        ),
    ] {
        in_census_thread(name, move || {
            let ((built, owners), seat_refusals) =
                ken_runtime::with_detached_result_seat_refusals(|| {
                    ken_runtime::with_region_exit_owner_observations(|| {
                        emit_below_gate(name, source)
                    })
                });
            assert_object_emission_wall(built, wall);
            assert_eq!(
                seat_refusals,
                seat.into_iter().collect::<Vec<_>>(),
                "{name}: exact downstream detached-result seat"
            );
            assert_region_exit_owners(name, &owners, expected);
        });
    }
}
