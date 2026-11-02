//! PX10-NATIVE-SITE-IDENTITY AC-2: how far the three px10 programs plan.
//!
//! The Process gate refuses all three before object emission, so this census
//! hands each preparation straight to the emission the gate guards. Nothing
//! here runs a native program; emission beneath that gate establishes no parity.

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
/// PX10-NATIVE-BRANCH-ALTERNATIVES. MEASURED: the smoke plan completes,
/// recording one feasibility diagnostic with no returned-Vis protocol error,
/// and builds an artifact beneath the separate Process admission gate. Its
/// settlement 647 resumes through fold 12 and is returned by Vis 682.
/// CLAIMED: the return fold and returned bracket settlement retain their
/// checked owners. THE GAP: the artifact is not run; this proves no parity.
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
        let _built = built.expect("smoke emits an artifact beneath the Process gate");
    });
}

/// Promise class: transition sentinel, retired by this WP's Increment 2.
/// MEASURED: argv_env_pipe and live_child build artifacts beneath the separate
/// Process gate after argv's generated context-4 Result contract becomes None.
/// Both retain exact static region-exit owners. CLAIMED: the planner cannot
/// confuse an ordinary successor Vis under Excluded{Relay} with a completed
/// K Ret publication. THE GAP: neither program runs natively here.
#[test]
fn px10_argv_and_live_emit_below_process_gate() {
    use ken_runtime::RegionExitPlane::{ReturnFold, ReturnedVisSettlement};
    for (name, source, expected, seat) in [
        (
            "px10_census_argv_env_pipe",
            ARGV_ENV_PIPE,
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
            &[
                (ReturnFold, 601, 12),
                (ReturnFold, 870, 606),
                (ReturnedVisSettlement, 601, 1488),
                (ReturnedVisSettlement, 870, 1043),
            ][..],
            None,
        ),
    ] {
        in_census_thread(name, move || {
            let (((built, owners), seat_refusals), diagnostics) =
                ken_runtime::with_static_response_feasibility_diagnostics(|| {
                    ken_runtime::with_detached_result_seat_refusals(|| {
                        ken_runtime::with_region_exit_owner_observations(|| {
                            emit_below_gate(name, source)
                        })
                    })
                });
            if name == "px10_census_argv_env_pipe" {
                let contracts = diagnostics
                    .iter()
                    .flat_map(|diagnostic| &diagnostic.generated_context_result_contracts)
                    .filter(|(context, _)| *context == 4)
                    .collect::<Vec<_>>();
                assert!(!contracts.is_empty(), "the argv K context must be queried");
                assert!(
                    contracts.iter().all(|(_, contract)| contract.is_none()),
                    "an ordinary successor Vis is not a completed K Ret: {contracts:?}"
                );
            }
            let _built = built.expect("px10 program emits an artifact beneath the Process gate");
            assert_eq!(
                seat_refusals,
                seat.into_iter().collect::<Vec<_>>(),
                "{name}: exact downstream detached-result seat"
            );
            assert_region_exit_owners(name, &owners, expected);
        });
    }
}
