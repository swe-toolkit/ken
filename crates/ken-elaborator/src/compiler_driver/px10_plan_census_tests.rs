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

/// Promise class: transition sentinel, retired when PX10-NATIVE-SPAWN-PARITY
/// promotes Process and removes the labelled-transport containment.
/// MEASURED: removing the Process bit from the checked smoke preparation
/// refuses its issued transport before object finish, on the named construct.
/// CLAIMED: the census-only emission route cannot ship unexecuted transport
/// merely because a caller mistakenly treats the plan as Process-free.
/// THE GAP: this test does not establish labelled-transport execution parity.
#[test]
fn px10_set_issuing_smoke_without_process_gate_refuses_containment() {
    in_census_thread("px10-containment-without-process-bit", || {
        let mut preparation = prepare_native_program_sources(
            "px10_containment_without_process_bit",
            vec![CompilerSource::new("src/main.ken", SMOKE)],
        )
        .expect("smoke prepares");
        assert!(
            preparation.plan.process_declared,
            "the original plan is gated"
        );
        preparation.plan.process_declared = false;
        let root = tempfile::tempdir().expect("census output root");
        let error = emit_native_program_preparation(
            preparation,
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
        .expect_err("an ungated set-issuing plan cannot emit an artifact");
        let NativeProgramBuildError::Packaging(error) = error else {
            panic!("expected a packaging refusal, got {error:?}");
        };
        assert_eq!(
            error.stage,
            ken_runtime::ObjectLinkerPackagingStage::ObjectEmission
        );
        assert_eq!(
            error.source_cause,
            Some(ken_runtime::LoweringCause::Fragment)
        );
        assert!(error.reason.contains("ProducerAlternativeSet"), "{error:?}");
    });
}

/// Promise class: durable invariant. MEASURED: a real smoke
/// planner issues two-member sets, builds beneath the Process gate, and
/// rejects readers that add one foreign same-result identity or drop a
/// member. CLAIMED: a reader cannot interpret an issued labelled word with
/// a different member set. THE GAP: these two probes do not enumerate every
/// possible foreign identity; they pin both cardinality directions at the
/// actual issued-set resolver.
#[test]
fn px10_issued_set_reader_mismatch_refuses_at_planning() {
    in_census_thread("px10-reader-member-mismatch", || {
        let (built, readers) =
            ken_runtime::with_producer_alternative_reader_negative_observations(|| {
                emit_below_gate("px10_reader_member_mismatch", SMOKE)
            });
        built.expect("the unchanged smoke fixture builds beneath its Process gate");
        assert_eq!(readers.len(), 3, "smoke must issue three reader sets");
        for reader in readers {
            assert_eq!(
                reader.issued_members, 2,
                "smoke reader set at {}",
                reader.result
            );
            assert!(
                reader
                    .extra_member_refusal
                    .as_deref()
                    .is_some_and(|reason| reason.contains(
                        "the result reader's expected member set differs from the issued producer alternative set"
                    )),
                "foreign reader at {}: {reader:?}", reader.result
            );
            assert!(
                reader
                    .dropped_member_refusal
                    .as_deref()
                    .is_some_and(|reason| reason.contains(
                        "a single-producer result reader names a result with an issued producer alternative set"
                    )),
                "singleton reader at {}: {reader:?}", reader.result
            );
        }
    });
}

/// Promise class: transition sentinel. MEASURED: the unavailable Process
/// operation reaches native dispatch only on this census route, not through
/// a product program (emission or the Process gate refuses first). Its host
/// status is decoded as ProcessPlanOpen, and the executable names the operation
/// rather than reporting malformed borrowed input. CLAIMED: when this refusal
/// reaches a native process, the diagnostic preserves its cause. THE GAP: no
/// labelled-transport execution parity or product admission is established.
#[test]
fn unavailable_process_operation_names_its_host_refusal() {
    in_census_thread("px10-unavailable-native-diagnostic", || {
        let preparation = prepare_native_program_sources(
            "px10_unavailable_native_diagnostic",
            vec![CompilerSource::new("src/main.ken", SMOKE)],
        )
        .expect("px10 smoke prepares");
        assert!(
            preparation.plan.process_declared,
            "the Process gate still owns admission"
        );
        let root = tempfile::tempdir().expect("census output root");
        let output = emit_native_program_preparation(
            preparation,
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
        .expect("smoke emits only beneath its Process gate");
        let error = ken_runtime::run_bound_process_effect_observation(
            &output.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: Vec::new(),
                environment: Vec::new(),
                cwd: root.path().to_owned(),
                plan_hash: output.plan_transport_hash,
            },
        )
        .expect_err("the unavailable operation cannot run natively");
        let ran = std::process::Command::new(&output.artifact.executable_path)
            .current_dir(root.path())
            .output()
            .expect("linked census artifact runs");
        assert!(
            matches!(
                &error,
                ken_runtime::NativeEffectRunErrorV1::OperationUnavailable {
                    operation: ken_host::HostOpV1::ProcessPlanOpen
                }
            ),
            "the exact host refusal must survive: {error:?}; stderr: {:?}",
            String::from_utf8_lossy(&ran.stderr)
        );
        assert_eq!(ran.status.code(), Some(1), "stderr: {:?}", ran.stderr);
        assert_eq!(
            ran.stderr,
            b"ken native trap: host operation ProcessPlanOpen (0x0601) is unavailable to native code\n"
        );
        assert!(!String::from_utf8_lossy(&ran.stderr).contains("malformed borrowed process input"));
    });
}

/// Promise class: regression. A NativeTested ConsoleWrite still completes and
/// yields its one unchanged event. The unavailable-op refusal has no product
/// witness today: emission or the Process gate refuses first.
#[test]
fn native_tested_console_write_retains_its_single_event() {
    const SOURCE: &str = r#"program capabilities FS APartial
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "unavailable-control")
"#;
    in_census_thread("native-tested-unavailable-control", || {
        let preparation = prepare_native_program_sources(
            "native_tested_unavailable_control",
            vec![CompilerSource::new("src/main.ken", SOURCE)],
        )
        .expect("NativeTested Console program prepares");
        assert!(
            !preparation.plan.process_declared,
            "Console does not use the Process gate"
        );
        let root = tempfile::tempdir().expect("control output root");
        let output = emit_native_program_preparation(
            preparation,
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
        .expect("NativeTested Console program emits");
        let observed = ken_runtime::run_bound_process_effect_observation(
            &output.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: Vec::new(),
                environment: Vec::new(),
                cwd: root.path().to_owned(),
                plan_hash: output.plan_transport_hash,
            },
        )
        .expect("NativeTested Console program runs");
        assert_eq!(observed.exit_status, 0);
        assert_eq!(observed.stdout, b"unavailable-control\n");
        assert_eq!(
            observed.effect_trace,
            vec![ken_runtime::EffectEvent {
                sequence: 0,
                operation: ken_host::HostOpV1::ConsoleWrite,
                capability: None,
                resource_bindings: Vec::new(),
                request: ken_runtime::CanonicalRequestV1::ConsoleWrite {
                    stream: ken_runtime::ConsoleStreamV1::Stdout,
                    bytes: b"unavailable-control\n".to_vec(),
                },
                outcome: ken_runtime::CanonicalOutcomeV1::Success(
                    ken_runtime::CanonicalReplyV1::Unit,
                ),
            }]
        );
    });
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
