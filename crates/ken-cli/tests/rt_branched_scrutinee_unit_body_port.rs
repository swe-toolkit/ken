//! `RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT` D1 witness.
//!
//! MEASURED: the observer records resolver entry, plain-Match branch entry, successful
//! plain-Match arm-body lookup, and the direct non-`Construct` route-1 return
//! while this checked Ken source lowers.
//! CLAIMED: D2 enters the carried child's owning plain `Match`, walks its arms,
//! then advances past route 1.
//!
//! ⭐ **Updated by `RT-BRANCH-LOCAL-DECLARED-CALLABLE` `D1`.** Before that cut
//! the walk STOPPED at arm 0, because arm 0 constructs a different constructor
//! than the selected case and its missing recursive position vetoed the whole
//! source. The branch-local partition makes an out-of-bucket arm non-vetoing, so
//! the walk now reaches arm 1. The number below is that property, not a
//! snapshot: it goes back to 1 exactly if the partition is reverted.

#![cfg(target_os = "linux")]

const BRANCHED_SCRUTINEE_SOURCE: &str = include_str!("fixtures/d0_branched_scrutinee_source.ken");

// ── STACK FLOOR, AND WHAT IT COSTS ──────────────────────────────────────────
//
// These tests drive the elaborator's recursive descent and had NO stack grant:
// they ran on libtest's ambient 2 MiB thread and passed on an ACCIDENTAL
// margin, which an unrelated elaborator change then consumed. That is the same
// shape as the guard deleted from `abi_s6_mapping_surface_native` — a test
// whose need sits just under the ambient default has no signal of its own
// until something else spends the remainder, and the arriving change wears the
// blame.
//
// MEASURED, both arms built and the binaries invoked directly, bisected on
// `RUST_MIN_STACK` at 64 KiB resolution:
//
//     without the change   1984 KiB     with it   2112 KiB     delta 128 KiB
//     libtest ambient      2048 KiB     -> 1984 < 2048 < 2112, so it tips
//
// The floor below is DERIVED RATHER THAN CHOSEN, and written as arithmetic so
// the spend is legible. The property the old 3.1% margin never had was that
// anybody could see what they were spending.
//
// THIS IS A STOPGAP AND ITS SUBJECT IS NOT THIS FILE. `1984 KiB` is consumed
// before any test-specific work begins and is IDENTICAL across three files in
// two crates — a shared dominating consumer. Raising floors buys time; it does
// not address a baseline that eats 97% of the ambient stack.
const MEASURED_NEED_KIB: usize = 2112;
/// The bisection could not resolve finer than this, so the true need may be
/// up to one step above what was observed.
const BISECTION_RESOLUTION_KIB: usize = 64;
/// One elaborator change of the size that tipped these cost 128 KiB here.
const OBSERVED_INCREMENT_KIB: usize = 128;
/// How many further such changes this floor is buying room for. State the
/// number rather than a round total: this is the quantity being spent.
const INCREMENTS_OF_HEADROOM: usize = 8;
const TEST_STACK_BYTES: usize = (MEASURED_NEED_KIB
    + BISECTION_RESOLUTION_KIB
    + INCREMENTS_OF_HEADROOM * OBSERVED_INCREMENT_KIB)
    * 1024;

/// Run a test body on a thread with the floor above.
///
/// The thread is NAMED: an overflow on an unnamed `Builder` reports
/// `thread '<unknown>'`, which cost this investigation a step when exactly
/// that happened elsewhere. A future overflow here names its own test.
fn with_stack_floor(name: &str, body: impl FnOnce() + Send + 'static) {
    let handle = std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(TEST_STACK_BYTES)
        .spawn(body)
        .expect("spawning the floored test body must succeed");
    if let Err(payload) = handle.join() {
        std::panic::resume_unwind(payload);
    }
}

#[test]
fn two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1() {
    with_stack_floor(
        "two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1",
        two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1_body,
    );
}

fn two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1_body() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (result, route1) = ken_runtime::with_branched_scrutinee_unit_body_route1(|| {
        ken_cli::build_native_program(
            BRANCHED_SCRUTINEE_SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt_branched_scrutinee_unit_body",
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });

    // M6 completes the whole program, so this observer now sees later composed
    // and transport-driven resolver invocations too. Select the original
    // semantic row by the properties this test has always asserted: it enters
    // the plain-Match branch, walks both arms, and advances past direct route 1.
    // Do not freeze the incidental total row count.
    let intended: Vec<_> = route1
        .iter()
        .filter(|row| !row.route1 && row.match_branch_entered)
        .collect();
    assert_eq!(
        intended.len(),
        1,
        "exactly one resolver row must be the branched-scrutinee path that advances \
         past route 1: {route1:?}"
    );
    let intended = intended[0];
    // `RT-BRANCH-LOCAL-DECLARED-CALLABLE` `D1`: both arms are walked. Measured
    // at the bucket filter, arm 0 constructs `ITree::Ret` while the selected
    // case is `ITree::Vis` — the exact `Ret`/`Vis` asymmetry the node was cut
    // for. `Ret` is out of the bucket, so its missing recursive position no
    // longer vetoes, and arm 1 (`ITree::Vis`, in the bucket) is reached. A
    // revert of the partition returns this to 1, which is what makes the
    // assertion discriminating rather than decorative.
    assert_eq!(
        intended.match_arms_walked, 2,
        "D1 must walk past the out-of-bucket arm 0 and reach arm 1: {intended:?}"
    );
    eprintln!(
        "RT_BRANCHED_SCRUTINEE_UNIT_BODY_ROUTE1 entered={} route1={} match_arms_walked={}",
        1,
        usize::from(intended.route1),
        intended.match_arms_walked,
    );
    result
        .expect("M6 must complete after preserving the branched-scrutinee path through both arms");
}

#[test]
fn suppressing_match_branch_entry_is_a_recorder_positive_control() {
    with_stack_floor(
        "suppressing_match_branch_entry_is_a_recorder_positive_control",
        suppressing_match_branch_entry_is_a_recorder_positive_control_body,
    );
}

fn suppressing_match_branch_entry_is_a_recorder_positive_control_body() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (((result, rows), diagnostics), sequence) =
        ken_runtime::with_settlement_footprint_sequence(|| {
            ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_runtime::with_branched_scrutinee_unit_body_route1(|| {
                    ken_runtime::with_branched_scrutinee_unit_body_match_branch_entry_suppressed(|| {
                        ken_cli::build_native_program(
                            BRANCHED_SCRUTINEE_SOURCE,
                            ken_cli::SourceFormat::Ken,
                            "rt_branched_scrutinee_unit_body_mutation",
                            root.path(),
                            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                        )
                    })
                })
            })
        });
    result.expect("the suppressed branch-entry build must complete before attribution");
    assert_eq!(diagnostics.len(), 1, "one branched response plan authenticates the Tail");
    use ken_runtime::SettlementFootprintEvent as Event;
    use std::collections::BTreeMap;
    let authenticated = &diagnostics[0].bracket_settlement_vis_origins;
    if std::env::var_os("RT_FOOTPRINT_DETERMINISM").is_some() {
        eprintln!("RT_SETTLEMENT_FOOTPRINT_SEQUENCE {sequence:?}");
    }
    let consumed = Event::Consumed { settlement: 1, construct: 717 };
    let arrival = Event::ResidualArrival { origin: 523 };
    let route = Event::Route1 { origin: 889, route1: true };
    let mut segments = vec![BTreeMap::<Event, usize>::new()];
    let mut non_consumed = BTreeMap::<Event, usize>::new();
    for &event in &sequence {
        if let Event::Consumed { construct, .. } = event {
            assert_eq!(event, consumed, "only settlement 1 owns this Tail");
            assert!(authenticated.contains(&construct),
                "branched Tail Vis {construct} was not authenticated");
            segments.push(BTreeMap::new());
        } else {
            *non_consumed.entry(event).or_default() += 1;
        }
        *segments.last_mut().expect("segment zero exists").entry(event).or_default() += 1;
    }
    // Transition sentinel. MEASURED: the complete unified lowering event log,
    // including site-A arrival 523 between Consumed and Route1(889,true).
    // CLAIMED: only the authenticated Tail produces two exact segment footprints
    // and the ordinary d26 event population stays empty. THE GAP: this is
    // compile-time work, not executed native semantics; the settlement forward
    // edge retires both footprint events together.
    // (a) Exact authenticated settlement count.
    assert_eq!(segments.len(), 3, "two settlement-consumed branched Tails");
    // (b) Segment zero and both complete segments as measured (not adjacency).
    assert!(segments[0].is_empty(), "the d26 ordinary prefix is empty");
    let expected = BTreeMap::from([(consumed, 1), (arrival, 1), (route, 1)]);
    for (index, segment) in segments.iter().enumerate().skip(1) {
        assert_eq!(*segment, expected,
            "branched segment {index} changed from its measured full multiset");
    }
    // (c) F is the complete M-2' delta and its removal leaves no ordinary
    // events; do not project away the residual arrival or any route event.
    for footprint in [arrival, route] {
        assert_eq!(non_consumed.remove(&footprint), Some(2),
            "branched counterfactual footprint member changed: {footprint:?}");
    }
    assert!(non_consumed.is_empty(),
        "branched non-footprint entries changed: {non_consumed:?}");
    assert_eq!(sequence.iter().filter(|event| matches!(event, Event::Route1 { .. })).count(),
        rows.iter().filter(|row| row.route1).count(),
        "each selected route has one source-origin event");
    // The suppressed build's row count is not this control's subject. Before
    // RT-NATIVE-CONTINUATION-ENV-CARRIAGE I-2 the suppressed build aborted at
    // the first resolver entry, so one row was recorded. From I-2 the residual
    // route compiles the whole program and records the later
    // continuation-context entries too: the same three entries the unsuppressed
    // run records. Select by the recorded property, as the sibling test does,
    // and do not freeze the total.
    assert!(!rows.is_empty(), "the resolver entry remains observable");
    assert_eq!(rows.iter().filter(|row| row.route1).count(), 2,
        "exactly two selected routes follow the authenticated Tail: {rows:?}");
    assert!(
        rows.iter()
            .all(|row| !row.match_branch_entered && row.match_arms_walked == 0),
        "the pre-recorder suppression is a positive control for branch-entry \
         recording: {rows:?}"
    );
}

/// Promise class: durable invariant. MEASURED: native and interpreter observe
/// the same ordered non-release events, including FsReadAt and its result,
/// and the same release set and terminal outcome. CLAIMED: a reached relay
/// performs the host operation through its exact native response owner.
/// THE GAP: the interpreter's independent effect observation must include the
/// read; the assertion below establishes that before comparing both paths.
#[test]
fn branched_scrutinee_forwarded_read_matches_interpreter() {
    with_stack_floor(
        "branched_scrutinee_forwarded_read_matches_interpreter",
        branched_scrutinee_forwarded_read_matches_interpreter_body,
    );
}

fn branched_scrutinee_forwarded_read_matches_interpreter_body() {
    use std::os::unix::ffi::OsStrExt as _;

    let source = BRANCHED_SCRUTINEE_SOURCE.replacen(
        "(MkBufferWindow (8 : Int) (4 : Int))",
        "(MkBufferWindow (0 : Int) (4 : Int))",
        1,
    );
    assert_ne!(
        source, BRANCHED_SCRUTINEE_SOURCE,
        "the in-bounds fixture differs"
    );
    let root = tempfile::tempdir().expect("temporary native-build root");
    std::fs::write(root.path().join("source"), b"0123456789abcdef")
        .expect("the fixture source file is readable");
    let compiled = ken_cli::build_native_program(
        &source,
        ken_cli::SourceFormat::Ken,
        "rt_branched_scrutinee_forwarded_read",
        root.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("the forwarded read has a planned performing owner");
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
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_bytes(),
        &mut host,
    )
    .expect("the in-bounds fixture executes in the interpreter");
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
            ken_runtime::HostOpV1::FsReadAt,
        ],
        "the oracle really performs the forwarded read"
    );
    assert_eq!(
        interpreted.exit_status, 72,
        "fixture's ignored-read success branch"
    );
    assert_eq!(release_set(&native), release_set(&interpreted));
    let native_ops = non_release(&native);
    assert_eq!(native_ops, interpreter_ops, "the relay must perform FsReadAt");
    assert_eq!(native.exit_status, interpreted.exit_status);
    assert_eq!(native.terminal_exit, interpreted.terminal_exit);
    assert_eq!(native.terminal_error, interpreted.terminal_error);
}

/// Promise class: durable invariant. MEASURED: disabling the exact relay
/// promotion on a fixture with a reached forwarded read records an actual
/// suppression and refuses the native build at the planner boundary. CLAIMED:
/// the read's performing owner is necessary, not inferred by lowering. THE
/// GAP: the control must reach the relay promotion producer; the application
/// count and the unchanged baseline parity fixture discriminate that case.
#[test]
fn suppressing_relay_promotion_refuses_the_forwarded_read_before_lowering() {
    let source = BRANCHED_SCRUTINEE_SOURCE.replacen(
        "(MkBufferWindow (8 : Int) (4 : Int))",
        "(MkBufferWindow (0 : Int) (4 : Int))",
        1,
    );
    assert_ne!(source, BRANCHED_SCRUTINEE_SOURCE);
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (result, applications) = ken_runtime::with_suppressed_relay_promotion(|| {
        ken_cli::build_native_program(
            &source,
            ken_cli::SourceFormat::Ken,
            "rt_branched_relay_promotion_suppressed",
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    assert!(applications > 0, "the mutation must reach a relay source");
    let error = result.expect_err("a forwarded read with no performer must refuse before lowering");
    assert_eq!(error.code().as_str(), "N0003");
    let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(packaging) = error
    else { panic!("the refusal must come from native planning") };
    assert_eq!(packaging.stage, ken_runtime::ObjectLinkerPackagingStage::ObjectEmission);
    assert_eq!(packaging.source_cause, Some(ken_runtime::LoweringCause::PlannerInvariant));
}

/// Promise class: durable invariant. MEASURED: the reached source Construct
/// lowers, and retargeting its selected owner call to the original K first
/// reaches a finished-code identity refusal. With only that earlier refusal
/// suppressed, the relay selected-incoming guard records and refuses the
/// unentered owner. CLAIMED: a lowered relay source needs an incoming owner
/// call, even if a preceding Result check would otherwise refuse its K.
/// THE GAP: suppression alone must be inert; the retarget mutation changes
/// the actual resolved callee, not just the guard's recorded evidence.
#[test]
fn lowering_relay_source_without_entering_owner_refuses_coverage() {
    with_stack_floor("rt-relay-lowered-without-owner", || {
        let source = BRANCHED_SCRUTINEE_SOURCE.replacen(
            "(MkBufferWindow (8 : Int) (4 : Int))",
            "(MkBufferWindow (0 : Int) (4 : Int))", 1,
        );
        assert_ne!(source, BRANCHED_SCRUTINEE_SOURCE);
        let inert_root = tempfile::tempdir().expect("suppression-only native-build root");
        let (inert_result, inert_applications) = ken_runtime::with_result_contract_layer_mutation(
            ken_runtime::ResultContractLayerMutation::SuppressMissingContractRefusal,
            || ken_cli::build_native_program(
                &source, ken_cli::SourceFormat::Ken,
                "rt_relay_suppression_inert", inert_root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            ),
        );
        assert_eq!(inert_applications, 0, "sound relay build has no missing contract");
        assert!(ken_runtime::result_contract_layer_mutation_is_exact());
        inert_result.expect("suppression alone must preserve the relay native build");
        drop(inert_root);

        for (label, isolated) in [("retarget-only", false), ("retarget-isolated", true)] {
            let root = tempfile::tempdir().expect("temporary native-build root");
            let run = || ken_runtime::with_relay_owner_coverage_refusal_observations(|| {
                ken_runtime::with_static_response_caller_retarget_mutation(
                    ken_runtime::StaticResponseCallerRetargetMutation::RestoreRelayPromotedKTarget,
                    || ken_cli::build_native_program(
                        &source, ken_cli::SourceFormat::Ken,
                        "rt_relay_lowered_without_owner", root.path(),
                        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                    ),
                )
            });
            let (((built, applications), coverage), layer_applications) = if isolated {
                ken_runtime::with_result_contract_layer_mutation(
                    ken_runtime::ResultContractLayerMutation::SuppressMissingContractRefusal,
                    run,
                )
            } else {
                (run(), 0)
            };
            assert!(applications > 0, "{label}: the reached relay must select its owner call");
            assert!(ken_runtime::static_response_caller_retarget_mutation_is_exact());
            assert!(ken_runtime::result_contract_layer_mutation_is_exact());
            let error = built.expect_err("a lowered relay without its owner must refuse");
            let expected_code = if isolated { "N0003" } else { "N0004" };
            assert_eq!(error.code().as_str(), expected_code, "{label}: wrong refusal code");
            let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(packaging) = error
            else { panic!("{label}: refusal must occur during native object closeout") };
            assert_eq!(packaging.stage, ken_runtime::ObjectLinkerPackagingStage::ObjectEmission);
            if isolated {
                assert!(layer_applications > 0, "{label}: no earlier refusal was suppressed");
                assert_eq!(coverage.len(), 1, "{label}: the exact relay guard must refuse");
                assert!(coverage[0].source_lowered, "{label}: source Construct must lower");
                assert!(!coverage[0].selected_caller.is_empty(), "{label}: exact incoming edge");
                assert_eq!(packaging.source_cause, Some(ken_runtime::LoweringCause::PlannerInvariant));
                assert!(
                    packaging.reason.contains("a forward-declared response owner has no verified selected incoming call"),
                    "{label}: wrong closeout refusal: {:?}", packaging.reason
                );
            } else {
                assert_eq!(layer_applications, 0);
                assert!(coverage.is_empty(), "{label}: earlier verifier must refuse first");
                assert_eq!(packaging.source_cause, Some(ken_runtime::LoweringCause::FinishedCode));
                let published = packaging.reason
                    .split_once(" publishes actual constructor identity ")
                    .and_then(|(_, after)| after.split_once(" at "))
                    .map(|(identity, _)| identity)
                    .expect("retarget-only: missing published constructor identity");
                let demanded = packaging.reason
                    .split_once("but its pending Result obligation demands ")
                    .map(|(_, identity)| identity)
                    .expect("retarget-only: missing demanded constructor identity");
                assert_ne!(published, demanded, "retarget-only: wrong K identity must differ");
            }
        }
    });
}
