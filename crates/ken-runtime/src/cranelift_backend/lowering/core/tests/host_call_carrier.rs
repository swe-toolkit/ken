//! Host-call boundary, carrier production and trap-identity end-to-end
//! lowering tests (`RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1, module 2 of 5,
//! split from `control.rs`: `typed_trap_exit_*`, `rtfp_*`, `b2f_*`, `d4_*`,
//! `governed_nested_brackets_*`, `d6a_*`/`d6b_*` (first wave), `rt_scale_b_*`,
//! the host-result-carrier `d8_*` five, `d7_*`, `ac1b_*`, `rt_d2_*`,
//! `d2_ac6_*`, `d5_c2_*`/`d5_c4_*`/`d5_the_*`).

use super::*;
use crate::cranelift_backend::lowering::units::{
    continuation_case_binder_run, ContinuationCaseBinderSource,
};
use crate::{CraneliftRunReport, RuntimeSymbolMetadata};
use crate::nc5_seed_examples;

#[test]
fn a_trap_arm_and_its_trap_free_twin_both_functionize() {
    let fixture = |trap_arm| RuntimeExpr::Match {
        // Calling the lexical closure makes the scrutinee cross a declared-unit
        // edge. The match must therefore emit both arms from the carried
        // representation instead of selecting the known constructor while
        // compiling.
        scrutinee: Box::new(RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::TrapTwin::Left".to_string(),
                        args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
                    }),
                }),
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Var(0)),
        }),
        cases: vec![
            crate::RuntimeMatchCase {
                constructor: "ctor:fixture::TrapTwin::Left".to_string(),
                binders: 1,
                // This arm's result crosses its own declared-unit edge, so the
                // pre-emission D8 plan fixes the Match join to CarrierWord.
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::LexicalClosure {
                        captures: Vec::new(),
                        params: Vec::new(),
                        body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                    }),
                    args: Vec::new(),
                },
            },
            crate::RuntimeMatchCase {
                constructor: "ctor:fixture::TrapTwin::Right".to_string(),
                binders: 0,
                body: if trap_arm {
                    RuntimeExpr::Trap(RuntimeTrap {
                        code: RuntimeTrapCode::ExplicitTrap,
                        message: "functionized trap arm".to_string(),
                    })
                } else {
                    RuntimeExpr::Value(RuntimeValue::Bool(false))
                },
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "trap-twin default".to_string(),
        },
    };
    let without_trap = fixture(false);
    let with_trap = fixture(true);
    let mut all_trap = fixture(true);
    let RuntimeExpr::Match { cases, .. } = &mut all_trap else {
        unreachable!("trap twin fixture is a Match");
    };
    cases[0].body = RuntimeExpr::Trap(RuntimeTrap {
        code: RuntimeTrapCode::ExplicitTrap,
        message: "functionized first trap arm".to_string(),
    });

    // Promise class: durable invariant. Any extension preserving the declared
    // unit boundary and terminal trap semantics keeps this pair green.
    //
    // MEASURED: otherwise-identical carried matches both select functionized
    // emission and compile into complete declared-unit bundles.
    // CLAIMED: a source Trap arm is terminal CFG, not retained authority or a
    // value predecessor of the carried join.
    // THE GAP: successful compilation proves the trap did not enter the merge,
    // because the carrier producer rejects Trap; the separate D8 topology
    // controls prove the all-trap/no-merge boundary.
    for (name, expr) in [
        ("trap-free", &without_trap),
        ("trap-carrying", &with_trap),
    ] {
        let plan = plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &crate::NativeProcessSymbols::legacy_prelude(),
            AbiRootIngress::Value,
            true,
        )
        .expect("trap twin plans");
        let token = plan
            .join_plan_token(plan.root_static_origin().expect("trap twin root"))
            .expect("trap twin root is a join");
        assert_eq!(token.representation, JoinResultRepresentation::CarrierWord);
        assert!(token.has_continuing_predecessor);
        ac11_compiles(expr).unwrap_or_else(|error| {
            panic!("{name} twin failed functionized emission: {error}")
        });
    }
    let all_trap_plan = plan_static_transition_graph_with_symbols(
        &all_trap,
        &BTreeMap::new(),
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("all-trap carried match plans");
    let all_trap_token = all_trap_plan
        .join_plan_token(
            all_trap_plan
                .root_static_origin()
                .expect("all-trap root"),
        )
        .expect("all-trap root is a join");
    assert!(!all_trap_token.has_continuing_predecessor);
    ac11_compiles(&all_trap)
        .unwrap_or_else(|error| panic!("all-trap carried match emitted a merge: {error}"));
}

fn trap_exit_fixture(trapping: bool) -> RuntimeExample {
    let selected = "ctor:fixture::TrapExit::Selected".to_string();
    let skipped = "ctor:fixture::TrapExit::Skipped".to_string();
    let exact_trap = RuntimeTrap {
        code: RuntimeTrapCode::ExplicitTrap,
        message: "functionized nested unit trap identity".to_string(),
    };
    let selected_body = if trapping {
        RuntimeExpr::Trap(exact_trap.clone())
    } else {
        RuntimeExpr::Value(RuntimeValue::Bool(false))
    };
    RuntimeExample {
        name: if trapping {
            "functionized-nested-trap"
        } else {
            "functionized-trap-free-sibling"
        }
        .to_string(),
        checked_core_shape: "D2 typed trap-exit authority fixture".to_string(),
        ir: RuntimeExpr::Let {
            // The trap lives in the lexical body's unit. Its return crosses
            // that unit's caller and then the root adapter.
            value: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Match {
                        scrutinee: Box::new(RuntimeExpr::Construct {
                            constructor: selected.clone(),
                            args: Vec::new(),
                        }),
                        cases: vec![
                            crate::RuntimeMatchCase {
                                constructor: selected,
                                binders: 0,
                                body: selected_body,
                            },
                            crate::RuntimeMatchCase {
                                constructor: skipped,
                                binders: 0,
                                body: RuntimeExpr::Value(RuntimeValue::Bool(true)),
                            },
                        ],
                        default: RuntimeTrap {
                            code: RuntimeTrapCode::PatternMatchFailure,
                            message: "functionized trap-exit default".to_string(),
                        },
                    }),
                }),
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Var(0)),
        },
        observation: if trapping {
            RuntimeObservation::Trapped(exact_trap)
        } else {
            RuntimeObservation::Returned(RuntimeGroundValue::Bool(false))
        },
    }
}

struct TrapExitMutationReset;

impl Drop for TrapExitMutationReset {
    fn drop(&mut self) {
        set_trap_frame_binding_mutation(TrapFrameBindingMutation::Exact);
        set_trap_identity_mutation(TrapIdentityMutation::Exact);
        set_trap_caller_protocol_mutation(TrapCallerProtocolMutation::Exact);
    }
}

fn run_trap_exit_fixture(
    fixture: &RuntimeExample,
    frame: TrapFrameBindingMutation,
    identity: TrapIdentityMutation,
    protocol: TrapCallerProtocolMutation,
) -> Result<crate::CraneliftRunReport, CraneliftBackendError> {
    let _reset = TrapExitMutationReset;
    set_trap_frame_binding_mutation(frame);
    set_trap_identity_mutation(identity);
    set_trap_caller_protocol_mutation(protocol);
    run_example_with_seed_observation(fixture, &NativeSeedEnvironment::empty())
}

#[test]
fn typed_trap_exit_preserves_the_planner_identity_across_two_unit_calls() {
    let fixture = trap_exit_fixture(true);
    let plan = plan_static_transition_graph_with_symbols(
        &fixture.ir,
        &BTreeMap::new(),
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("nested trap fixture plans");
    let exact_trap = match &fixture.observation {
        RuntimeObservation::Trapped(trap) => trap,
        _ => unreachable!("the trapping fixture has a trap observation"),
    };
    assert!(
        plan.trap_identity(exact_trap)
            .expect("the selected trap is inventoried")
            .abi_word()
            > 0
    );
    let report = run_trap_exit_fixture(
        &fixture,
        TrapFrameBindingMutation::Exact,
        TrapIdentityMutation::Exact,
        TrapCallerProtocolMutation::Exact,
    )
    .expect("the nested unit trap reaches the JIT root");
    assert_eq!(report.observation, fixture.observation);
}

#[test]
fn typed_trap_exit_rejects_a_deleted_or_root_misclassified_unit_lane() {
    let fixture = trap_exit_fixture(true);
    let deleted = run_trap_exit_fixture(
        &fixture,
        TrapFrameBindingMutation::DeleteUnitLane,
        TrapIdentityMutation::Exact,
        TrapCallerProtocolMutation::Exact,
    )
    .expect_err("deleting a unit lane must fail before root translation");
    assert!(deleted
        .to_string()
        .contains("trap branch has no generated-unit TrapWord lane"));

    let misclassified = run_trap_exit_fixture(
        &fixture,
        TrapFrameBindingMutation::MisclassifyUnitAsRoot,
        TrapIdentityMutation::Exact,
        TrapCallerProtocolMutation::Exact,
    )
    .expect_err("a unit must not acquire root authority");
    assert!(misclassified
        .to_string()
        .contains("unit trap frame was bound to a function without unit authority"));
}

#[test]
fn typed_trap_exit_identity_and_caller_protocol_mutations_are_discriminating() {
    let trapping = trap_exit_fixture(true);
    for identity in [TrapIdentityMutation::Zero, TrapIdentityMutation::Substitute] {
        let mutated = run_trap_exit_fixture(
            &trapping,
            TrapFrameBindingMutation::Exact,
            identity,
            TrapCallerProtocolMutation::Exact,
        );
        assert!(
            mutated
                .map(|report| report.observation != trapping.observation)
                .unwrap_or(true),
            "{identity:?} still reconstructed the selected RuntimeTrap"
        );
    }
    let reversed = run_trap_exit_fixture(
        &trapping,
        TrapFrameBindingMutation::Exact,
        TrapIdentityMutation::Exact,
        TrapCallerProtocolMutation::ReadResultBeforeTrap,
    );
    assert!(
        reversed
            .map(|report| report.observation != trapping.observation)
            .unwrap_or(true),
        "reading Result before TrapWord preserved a trapping observation"
    );

    let trap_free = trap_exit_fixture(false);
    let exact = run_trap_exit_fixture(
        &trap_free,
        TrapFrameBindingMutation::Exact,
        TrapIdentityMutation::Exact,
        TrapCallerProtocolMutation::Exact,
    )
    .expect("the trap-free sibling returns normally");
    assert_eq!(exact.observation, trap_free.observation);
    let stale = run_trap_exit_fixture(
        &trap_free,
        TrapFrameBindingMutation::Exact,
        TrapIdentityMutation::Exact,
        TrapCallerProtocolMutation::LeaveStaleTrap,
    );
    assert!(
        stale
            .map(|report| report.observation != trap_free.observation)
            .unwrap_or(true),
        "omitting the callee TrapWord clear preserved the trap-free result"
    );
}

#[test]
fn the_generated_root_translates_a_runtime_reached_trap_exactly() {
    let trap = RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "generated root Match trap".to_string(),
    };
    let fixture = RuntimeExample {
        name: "generated-root-trap".to_string(),
        checked_core_shape: "generated root trap translation fixture".to_string(),
        // `RT-PRODUCER-MATCH-PORT` `D3` CHANGED WHAT THIS ROW MEASURES, and
        // the change is a real coverage loss reported rather than papered over.
        //
        // The program is unchanged and still reaches its trap. What moved is the
        // LANE: its producer-`Call` scrutinee is ported, so it now translates the
        // trap through the FUNCTIONIZED root instead of the retained one.
        //
        // I could not retarget it onto a surviving residual. The two closed
        // shapes I tried both refused for unrelated reasons -- a free `Var(0)` in
        // the shared active-recursor helper, then a non-constructor scrutinee --
        // and `run_example_with_seed_observation` takes no declaration map, so
        // the lane cannot be retained by an unevaluated declaration either.
        //
        // ⇒ **Runtime-reached trap translation on the RETAINED root now has no
        // witness here.** `RT-DESCENT-RETIRE` deletes that root, so the gap is
        // bounded by that node; it is stated so nobody reads the rename as
        // coverage moving when it is coverage ending.
        ir: RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::RetainedRoot::Miss".to_string(),
                        args: Vec::new(),
                    }),
                }),
                args: Vec::new(),
            }),
            cases: Vec::new(),
            default: trap.clone(),
        },
        observation: RuntimeObservation::Trapped(trap),
    };
    let report = run_example_with_seed_observation(&fixture, &NativeSeedEnvironment::empty())
        .expect("the functionized root translates its planner trap identity");
    assert_eq!(report.observation, fixture.observation);
}

#[test]
fn every_generated_root_and_unit_signature_is_two_pointers_to_one_word() {
    let module = new_jit_module().expect("JIT module");
    let signature = crate::cranelift_backend::lowering::units::unit_signature(&module);
    let pointer = module.target_config().pointer_type();
    assert_eq!(signature.params.len(), 2);
    assert!(
        signature
            .params
            .iter()
            .all(|parameter| parameter.value_type == pointer)
    );
    assert_eq!(signature.returns.len(), 1);
    assert_eq!(signature.returns[0].value_type, types::I64);

    let units = include_str!("../../units.rs");
    assert!(
        units.contains("let sig = unit_signature(module);"),
        "the adapter or unit definitions stopped sharing the closed signature"
    );
    assert!(
        !units.contains("GeneratedRootIngressV1"),
        "a launch-ingress type entered the internal unit implementation"
    );
}

#[cfg(test)]
const RTFP_DECLARATION: &str = "decl:fixture::RTFP::twice";
#[cfg(test)]
const RTFP_CALL_TEMPLATE: u64 = 700;
/// `semantic_position` 1 — the outermost checked frame, visited FIRST.
#[cfg(test)]
const RTFP_OUTER_FRAME: u64 = 10;
/// `semantic_position` 0 — checked postorder's first frame, visited LAST.
#[cfg(test)]
const RTFP_INNER_FRAME: u64 = 11;

#[cfg(test)]
fn rtfp_cases(body: i64) -> Vec<crate::RuntimeComputationalMatchCase> {
    vec![crate::RuntimeComputationalMatchCase {
        constructor: "ctor:fixture::Succ".to_string(),
        argument_binders: 1,
        recursive_positions: vec![0],
        // ⭐ The ONLY field that differs between the two frames.
        body: RuntimeExpr::Value(RuntimeValue::Int(body.into())),
    }]
}

#[cfg(test)]
fn rtfp_default() -> RuntimeTrap {
    RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "no runtime match case selected for ind:fixture::Nat".to_string(),
    }
}

#[cfg(test)]
fn rtfp_frame(
    frame_id: u64,
    semantic_position: u64,
    input: u8,
    output: u8,
    parent: Option<u64>,
) -> crate::OrientedSubcontinuationFramePlanV1 {
    let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
        frame_id,
        segment_site_id: 9,
        declaration: RTFP_DECLARATION.to_string(),
        checked_occurrence_path: vec![frame_id],
        semantic_position,
        input_interface: oriented_test_interface(input),
        output_interface: oriented_test_interface(output),
        // ⭐ Identical for both frames — computed from the shared header, never
        // from the body. That equality is `AC-F1`, and it is what makes the
        // fingerprint useless as a selector.
        runtime_frame_fingerprint: crate::compiler_private_computational_match_frame_fingerprint(
            &rtfp_cases(0),
            &rtfp_default(),
        ),
        occurrence_binding_fingerprint: 0,
        control_witness: parent.map_or(
            crate::OrientedControlWitnessV1::DistinguishedRoot,
            crate::OrientedControlWitnessV1::ParentFrame,
        ),
    };
    frame.occurrence_binding_fingerprint =
        crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
    frame
}

#[cfg(test)]
fn rtfp_plan() -> crate::OrientedSubcontinuationPlanV1 {
    let mut call = crate::CheckedRecursiveInvocationTemplateV1 {
        call_template_id: RTFP_CALL_TEMPLATE,
        declaration: RTFP_DECLARATION.to_string(),
        checked_occurrence_path: vec![5],
        callee: RTFP_DECLARATION.to_string(),
        level_instantiation: Vec::new(),
        recursion_group: "scc:fixture::RTFP".to_string(),
        scc_index: 0,
        admission: 0,
        arity: 1,
        local_telescope: Vec::new(),
        result_interface: oriented_test_interface(2),
        callee_segment_site_id: 9,
        // ⚠ Ascending `semantic_position`, exactly as `erasure.rs:1149` sorts.
        callee_frame_templates: vec![RTFP_INNER_FRAME, RTFP_OUTER_FRAME],
        caller_interface: oriented_test_interface(2),
        // ⚠ `validate_marker_locations` rejects an empty occurrence list, so an
        // empty one would make every REJECTION control below green on a fixture
        // that could never have lowered in the first place. The positive
        // control is what surfaced that.
        runtime_marker_locations: vec![crate::CheckedRuntimeMarkerLocationV1 {
            declaration: RTFP_DECLARATION.to_string(),
            runtime_path: vec![0, 1],
        }],
        occurrence_binding_fingerprint: 0,
    };
    call.occurrence_binding_fingerprint =
        crate::compiler_private_recursive_call_binding_fingerprint(&call);
    crate::OrientedSubcontinuationPlanV1 {
        representation_rule_version:
            crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
        frames: vec![
            rtfp_frame(RTFP_OUTER_FRAME, 1, 1, 2, None),
            rtfp_frame(RTFP_INNER_FRAME, 0, 0, 1, Some(RTFP_OUTER_FRAME)),
        ],
        recursive_calls: vec![call],
        computational_ih_slots: Vec::new(),
        computational_ih_calls: Vec::new(),
    }
}

#[cfg(test)]
fn rtfp_layer(
    frame_id: Option<u64>,
    body: i64,
    role: RecursorLayerRole,
) -> ComputationalRecursorLayer {
    ComputationalRecursorLayer {
        cases: rtfp_cases(body),
        default: rtfp_default(),
        outer_env: Vec::new(),
        static_origin: inert_test_static_origin(),
        provenance: RecursorFrameProvenance(frame_id.unwrap_or(0)),
        role,
        checked_frame_id: frame_id,
        checked_invocation_id: None,
        checked_invocation_source: None,
        checked_invocation_depth: 0,
        semantic_pending: true,
    }
}

#[cfg(test)]
fn rtfp_invocation() -> CheckedRecursiveInvocationInstance {
    CheckedRecursiveInvocationInstance {
        source: InvocationTemplateRef::SameSccCall(RTFP_CALL_TEMPLATE),
        invocation_instance_id: 0,
        semantic_depth: 0,
        dynamic_splice_edge: None,
    }
}

/// `selection_frame` is visited first, `wrapper_frame` second.
#[cfg(test)]
fn rtfp_segment(
    selection_frame: Option<u64>,
    wrapper_frame: Option<u64>,
) -> RecursorInvocationSegment {
    let origin = RecursorProducerOriginId(70);
    RecursorInvocationSegment::new(
        origin,
        0,
        rtfp_layer(
            selection_frame,
            1,
            RecursorLayerRole::SelectsOccurrence { origin },
        ),
        RecursorUnwindStack {
            later_wrappers_in_construction_order: vec![rtfp_layer(
                wrapper_frame,
                2,
                RecursorLayerRole::ExitsScope {
                    origin,
                    scope_origin: RecursorProducerOriginId(71),
                    parent_scope: None,
                },
            )],
        },
        ContinuationCursorId(7),
        None,
        None,
    )
}

#[cfg(test)]
fn rtfp_compose(
    plan: &crate::OrientedSubcontinuationPlanV1,
    segment: RecursorInvocationSegment,
) -> Result<InstalledOrientedSubcontinuationSegment, CraneliftBackendError> {
    compose_oriented_subcontinuation(
        Some(plan),
        Some(rtfp_invocation()),
        ContinuationActivationId(8),
        segment,
        Vec::new(),
    )
}

#[cfg(test)]
fn rtfp_reason(
    result: Result<InstalledOrientedSubcontinuationSegment, CraneliftBackendError>,
) -> String {
    match result {
        Ok(_) => panic!("this fixture must reject"),
        Err(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason })) => {
            assert_eq!(construct, "OrientedSubcontinuationPlanV1");
            reason
        }
        Err(other) => panic!("unexpected error class: {other:?}"),
    }
}

#[test]
fn rtfp_the_two_frames_are_header_identical_and_body_distinct() {
    // ⭐ Non-vacuity for every control below. If the fingerprints differed, the
    // permutation control would redden at the *fingerprint* check and prove
    // nothing about the order check; if the bodies agreed, there would be no
    // permutation to catch.
    let plan = rtfp_plan();
    let outer = plan.frame(RTFP_OUTER_FRAME).expect("outer frame");
    let inner = plan.frame(RTFP_INNER_FRAME).expect("inner frame");
    assert_eq!(
        outer.runtime_frame_fingerprint, inner.runtime_frame_fingerprint,
        "AC-F1: two same-family frames must share one header fingerprint"
    );
    assert_ne!(
        format!("{:?}", rtfp_cases(1)),
        format!("{:?}", rtfp_cases(2)),
        "the two bodies must actually differ"
    );
    assert_ne!(RTFP_OUTER_FRAME, RTFP_INNER_FRAME, "identities are distinct");
}

#[test]
fn rtfp_both_exact_occurrences_lower_under_equal_header_fingerprints() {
    let plan = rtfp_plan();
    let installed = rtfp_compose(&plan, rtfp_segment(Some(RTFP_OUTER_FRAME), Some(RTFP_INNER_FRAME)))
        .expect("two header-identical frames with distinct transported ids must both lower");
    assert_eq!(
        installed
            .semantic_frames
            .iter()
            .map(|frame| frame.checked_frame_id.unwrap())
            .collect::<Vec<_>>(),
        vec![RTFP_INNER_FRAME, RTFP_OUTER_FRAME],
        "checked composition order is postorder: inner then outer"
    );
}

#[test]
fn rtfp_a_cleared_transported_identity_rejects_before_cfg() {
    let plan = rtfp_plan();
    let reason = rtfp_reason(rtfp_compose(&plan, rtfp_segment(None, Some(RTFP_INNER_FRAME))));
    assert!(
        reason.contains("no checked frame identity"),
        "a dropped identity must be named as such, not recovered by inference: {reason}"
    );
}

#[test]
fn rtfp_exchanging_the_two_occurrence_identities_rejects() {
    // ⭐ THE PERMUTATION NET. The exchanged set is still exactly `expected`, and
    // both layers still pass the fingerprint compatibility check because the
    // two frames are header-identical by construction. ⛔ So a set-only check
    // CANNOT fail this fixture — only the occurrence-order check can.
    let plan = rtfp_plan();
    let reason = rtfp_reason(rtfp_compose(
        &plan,
        rtfp_segment(Some(RTFP_INNER_FRAME), Some(RTFP_OUTER_FRAME)),
    ));
    assert!(
        reason.contains("out of their planned occurrence order"),
        "the ORDER check must be the detector that fires, not coverage or the \
         fingerprint: {reason}"
    );
}

#[test]
fn rtfp_header_drift_after_identity_selection_rejects_by_fingerprint() {
    let plan = rtfp_plan();
    let mut segment = rtfp_segment(Some(RTFP_OUTER_FRAME), Some(RTFP_INNER_FRAME));
    // Identity is still exact and correctly ordered; only the header moved.
    segment.selection.default.message.push_str(" (drifted)");
    let reason = rtfp_reason(rtfp_compose(&plan, segment));
    assert!(
        reason.contains("does not match its checked frame template"),
        "post-selection header drift must still reject by fingerprint: {reason}"
    );
}

// ─── RT-FNSPLIT-B2F AC-2 — the emitted-unit population, measured BEHAVIOURALLY ─

/// **`AC-2`'s real property, defended by an oracle that source text cannot
/// move.**
///
/// ⭐ **Why this test exists next to a census that already "covers" `AC-2`.**
/// `correspondence_adds_no_emitted_unit_to_the_production_census` counts how
/// many times three spellings occur in seven files. That is a claim about
/// *repository text*: splitting a call across lines evades every needle, a
/// mention inside a comment inflates them, and in no configuration does it
/// observe a single emitted function. ⇒ It is a **tripwire**. This test is the
/// evidence: it counts units at the point of emission, so the number it asserts
/// is a property of the compiled module.
///
/// **MEASURED:** for two programs that differ *only* in whether they contain a
/// retained closure body, the `(declared, defined)` unit counts `B2F` actually
/// emitted.
/// **CLAIMED:** every declared target unit is defined, and the population tracks
/// the program's static structure rather than being a constant.
/// **THE GAP:** ⛔ this says nothing about whether a unit's *body* is correct,
/// nor that the population equals `entries ∪ StaticBody targets` — the latter is
/// `B2O`'s enforced equality (`validate_function_units`), consumed here rather
/// than re-asserted, because planning refuses to build a graph that violates it
/// and a re-assertion would be green on every input that can reach `B2F`.
#[test]
fn b2f_emits_one_defined_target_unit_per_planned_function_unit() {
    fn units_emitted(expr: &RuntimeExpr) -> (usize, usize) {
        let module = new_jit_module().expect("jit module");
        compile_expr_into_module(
            module,
            "b2f_unit_population_probe",
            Linkage::Local,
            expr,
            &NativeSeedEnvironment::empty(),
            BTreeMap::new(),
            None,
            false,
            None,
            None,
            None,
        )
        .expect("compile");
        crate::cranelift_backend::lowering::units::b2f_last_unit_emission()
    }

    // The two fixtures differ in exactly one thing: the second reaches the same
    // leaf value through a *called* lexical closure, which is what mints a
    // `StaticBody` edge and therefore a second function unit.
    //
    // ⚠ The closure is CALLED rather than returned, and that is required rather
    // than stylistic: a closure at the root is rejected outright
    // ("closures are callable but not observable ground values in native
    // lowering"), so a fixture that merely mentions one never reaches emission
    // and would have measured nothing while looking like a discriminator.
    let leaf = RuntimeExpr::Value(RuntimeValue::Bool(true));
    let with_closure = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: Vec::new(),
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        }),
        args: Vec::new(),
    };

    let (leaf_declared, leaf_defined) = units_emitted(&leaf);
    let (closure_declared, closure_defined) = units_emitted(&with_closure);

    // ⛔ Every declared unit is defined. A bundle that declares `n` and defines
    // `n-1` leaves an undefined symbol, which is why the recorder carries two
    // numbers instead of one.
    assert_eq!(
        leaf_declared, leaf_defined,
        "AC-2 -- a declared target unit was never defined (leaf program)"
    );
    assert_eq!(
        closure_declared, closure_defined,
        "AC-2 -- a declared target unit was never defined (closure program)"
    );

    // ⭐ POSITIVE CONTROL / NON-VACUITY. Without this the assertions above are
    // satisfied by emitting nothing at all, for any program, forever -- a
    // negative check passes for any reason. The discriminator is that the count
    // MOVES with the program's static structure.
    assert!(
        leaf_declared >= 1,
        "AC-2 -- even a leaf program has a root scheduling entry, so the \
         population is never empty; measured {leaf_declared}"
    );
    assert!(
        closure_declared > leaf_declared,
        "AC-2 -- NON-VACUITY: a retained closure body mints a `StaticBody` edge \
         and therefore an additional function unit. If these are equal the \
         population is not tracking the program and every count above is \
         satisfied by a constant. measured leaf={leaf_declared} \
         closure={closure_declared}"
    );
}

/// **`RT-DECL-CLOSURE-PORT` `D4` — a `LexicalClosure`-bodied transparent
/// declaration retains a callable binding and still runs.**
///
/// Promise class: durable invariant.
///
/// MEASURED: both closure seed forms reach `Lowered::DeclarationClosure`, and a
/// program that calls a lexical-closure declaration returns the value its body
/// computes -- both for a parameter and for a capture.
/// CLAIMED: extending the retained binding to the second seed form did not
/// change what these programs compute on the surviving functionized path.
#[test]
fn d4_a_lexical_closure_declaration_retains_a_binding_and_still_runs() {
    // The parameter case: the argument reaches the body.
    let by_parameter = "decl:fixture::d4::by_parameter".to_string();
    let parameter_declaration = RuntimeDeclaration {
        symbol: by_parameter.clone(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(RuntimeExpr::Var(0)),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };
    // The capture case: a closed capture expression, lowered in the
    // declaration's own empty environment and bound behind the parameters.
    let by_capture = "decl:fixture::d4::by_capture".to_string();
    let capture_declaration = RuntimeDeclaration {
        symbol: by_capture.clone(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: vec![RuntimeExpr::Value(RuntimeValue::Int((58).into()))],
                params: Vec::new(),
                body: Box::new(RuntimeExpr::Var(0)),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };

    // Distinct values, so a test that ran the wrong declaration cannot pass.
    for (label, symbol, declaration, expected) in [
        ("parameter", &by_parameter, &parameter_declaration, 41i64),
        ("capture", &by_capture, &capture_declaration, 58i64),
    ] {
        let args = if label == "parameter" {
            vec![RuntimeExpr::Value(RuntimeValue::Int((41).into()))]
        } else {
            Vec::new()
        };
        let expr = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::DeclarationRef {
                symbol: symbol.clone(),
            }),
            args,
        };
        let compiled = compile_expr_into_module(
            new_jit_module().expect("JIT module"),
            "d4_lexical_closure_declaration",
            Linkage::Local,
            &expr,
            &NativeSeedEnvironment::empty(),
            BTreeMap::from([(symbol.as_str(), declaration)]),
            None,
            false,
            None,
            None,
            None,
        )
        .unwrap_or_else(|error| {
            panic!("the {label} lexical-closure declaration must compile: {error:?}")
        });
        assert_eq!(
            compiled.run(None).expect("the declaration call runs").0,
            RuntimeObservation::Returned(RuntimeGroundValue::Int(expected.into())),
            "D4: the {label} case must still compute its own value after the \
             retained binding was extended to the LexicalClosure seed form"
        );
    }
}

// ─── RT-FNSPLIT-B2F AC-11 — the producer walk can REJECT, and does not over-reject ─

#[test]
fn governed_nested_brackets_n3_through_n7_emit_complete_functionized_bundles() {
    for depth in 3..=7 {
        let expr =
            crate::cranelift_backend::planning::governed_nested_resource_bracket(depth);
        recursive_port_process_compiles(&expr).unwrap_or_else(|error| {
            panic!("governed depth {depth} did not compile: {error}")
        });

        let (declared, defined) =
            crate::cranelift_backend::lowering::units::b2f_last_unit_emission();
        let resolved =
            crate::cranelift_backend::lowering::units::b2f_last_call_edge_resolution();
        let recursive_calls = recursive_position_unit_calls();
        let (carried_unchanged, specialized_productions) =
            d8_join_conversion_counts();
        eprintln!(
            "RT_FNSPLIT_RECUR_PORT n={depth} authority=FunctionizedUnits \
             declared={declared} defined={defined} resolved_calls={resolved} \
             recursive_position_calls={recursive_calls} \
             carried_unchanged={carried_unchanged} \
             specialized_productions={specialized_productions}"
        );

        assert!(declared > 1, "depth {depth} emitted no retained body units");
        assert_eq!(
            defined, declared,
            "depth {depth} left a declared unit undefined"
        );
        assert!(
            resolved > 0,
            "depth {depth} resolved no graph-derived call edges"
        );
        assert!(
            recursive_calls > 0,
            "depth {depth} re-lowered every recursive position inline instead \
             of emitting a declared unit call"
        );
        assert!(
            carried_unchanged > 0,
            "depth {depth} never forwarded a carried predecessor unchanged"
        );
        assert_eq!(
            specialized_productions, 0,
            "the governed bracket's sibling is a trap, not a specialized merge \
             predecessor"
        );
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6a` — a specialization's case environment
/// binds the induction hypothesis and the selected recursive constructor
/// argument as TWO static-worker members, and they LEAD the environment.**
///
/// The subject is the **assembled lowering environment**, not the plan.
/// [`continuation_case_binder_run`]'s own rows pin the plan; they cannot see
/// whether the plan's `SelectedRecursiveArgument` reached a real binding, nor
/// where in the installed environment it landed. Those are the facts `D6a`
/// adds, so they need an oracle downstream of the plan.
///
/// **The discriminators, chosen before the positive case:**
/// - Pre-`D6a` the argument position was **skipped**, so the environment held
///   **one** static worker and the outer frame sat one slot early. The count
///   clause reds on that.
/// - ⭐ A repair that appends the new member to the **tail** instead of placing
///   it in the argument segment produces `[worker, outer.., worker]`. The
///   "nothing after the leading pair" clause reds on that.
///
///   ⛔ **WITHDRAWN, and replaced by what was measured.** An earlier version of
///   this note said the tail append would leave the *last* index resolving to a
///   plausible value while an earlier one was silently wrong, so that a row
///   keyed only on the reported `Var(2)` would pass. That is **false for this
///   mechanism**. MEASURED by running exactly that mutation: the typed
///   static-worker binding refuses it, because the tail position is read in a
///   *value* position and [`LoweringEnvironmentBinding::value_at`] fails closed
///   there — *"a Var in value position is a value-producing position and a
///   static worker binding has no value representation"*. The mis-repair is
///   loud, not silent. This clause is therefore retained for its **actual**
///   discriminator — it names the wrong ORDER directly, at the plan level,
///   without depending on whether some later read happens to fail closed.
/// - A repair that binds the argument as a value rather than a callable
///   produces one worker and one more `Carried`. The count clause reds.
///
/// ⛔ **STATED GAP — this witness is ROUTE-DEGENERATE, and that is lawful.**
/// MEASURED here: every specialization renders as
/// `[StaticWorker(RawWorker), StaticWorker(RawWorker), Carried..]`, because
/// `continuation_context_for` issues **no** generated execution context for any
/// of them. By the route law on [`StaticWorkerCallRoute`], the induction
/// hypothesis then lawfully carries `RawWorker` too — the selected recursive
/// argument always does — so the pair is *equal by rule, not by accident*.
///
/// ⛔ **Equal rendered routes are therefore NOT evidence that one binding was
/// reused for both members**, and this row does not claim otherwise. CLAIMED
/// here: only the **membership and order** of the two workers, which are
/// exactly the facts that still separate the members when the routes coincide.
/// THE GAP: [`StaticWorkerCallRoute`]'s two arms are *present and correct* but
/// not *discriminated* by this witness, so nothing here would catch a route set
/// wrongly.
///
/// That gap is closed elsewhere rather than left open —
/// [`d5a_the_landed_object_fixture_consumes_its_ih_marker_before_emitting_the_worker_call`]
/// runs on a plan that **does** issue a context, and asserts the exact mixed
/// pair `[StaticWorker(GeneratedContext), .., StaticWorker(RawWorker), ..]`.
/// `D6b` then asserts it **structurally** rather than as rendered text, in
/// [`d6b_the_mixed_pair_is_over_one_body_and_only_a_retarget_makes_the_two_tables_disagree`]:
/// the two routes carry their body origins with them, so that row settles the
/// further question a rendered pair cannot — whether the mixed pair is over
/// **one** worker body or two. This row remains the membership-and-order
/// control that holds even where no context is in play.
///
/// **Promise class: durable invariant.** Every clause is a relation over the
/// entries — how many static workers, and where they sit relative to the rest.
/// A fixture that grows fields, inputs or units keeps it green; only a change
/// to which bindings a case environment installs, or to their order, reds it,
/// and that is a contract decision.
#[test]
fn d6a_a_specialization_binds_two_leading_static_workers_for_the_ih_and_its_recursive_argument() {
    let expr = crate::cranelift_backend::planning::governed_nested_resource_bracket(3);
    reset_d5a_trace();
    recursive_port_process_compiles(&expr)
        .unwrap_or_else(|error| panic!("the governed depth-3 fixture must compile: {error}"));
    let trace = take_d5a_trace();

    let bodies = trace
        .iter()
        .filter(|entry| entry.contains("SPEC-BODY"))
        .collect::<Vec<_>>();
    assert!(
        !bodies.is_empty(),
        "the fixture must actually reach the specialization-body seat, or this row proves \
         nothing about the environment it installs: {trace:?}"
    );

    let mut with_workers = 0usize;
    for body in &bodies {
        let (_, rendered) = body
            .split_once("env=[")
            .unwrap_or_else(|| panic!("every SPEC-BODY entry renders its environment: {body}"));
        let rendered = rendered
            .strip_suffix(']')
            .unwrap_or_else(|| panic!("the rendered environment is bracketed: {body}"));
        let entries = rendered.split(", ").collect::<Vec<_>>();
        let workers = entries
            .iter()
            .filter(|entry| entry.starts_with("StaticWorker"))
            .count();
        if workers == 0 {
            continue;
        }
        with_workers += 1;
        assert_eq!(
            workers, 2,
            "this case has one recursive constructor argument, so its environment binds two \
             static workers: the induction hypothesis, then the argument itself. One means \
             the argument position was skipped and the IH stood in for it -- the pre-`D6a` \
             defect, which shifted every later binder down one slot: {body}"
        );
        assert!(
            entries[0].starts_with("StaticWorker") && entries[1].starts_with("StaticWorker"),
            "the IH prefix and the argument segment both precede the outer frame, so the two \
             workers are entries 0 and 1: {body}"
        );
        assert!(
            entries[2..]
                .iter()
                .all(|entry| !entry.starts_with("StaticWorker")),
            "nothing after the leading pair is a static worker. A member appended to the \
             outer-frame tail instead of placed in the argument segment lands here. This \
             clause names that wrong ORDER directly, at the assembled environment, rather \
             than relying on a later read to reject it: {body}"
        );
    }
    assert!(
        with_workers > 0,
        "no specialization body installed a static worker at all, so every clause above ran \
         vacuously: {bodies:?}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6b` — the governed positive: the plan and
/// the ASSEMBLED environment agree entry for entry.**
///
/// The whole run for the governed depth-3 case is
/// `[IH, ScopeArgument, BufferAllocate success payload]` — `Var(0)` the
/// induction hypothesis, `Var(1)` the selected recursive constructor argument,
/// `Var(2)` this frame's single continuation input, which the planner projects
/// from the outer `BufferAllocate` `Result::Ok` success binder.
///
/// ⭐ **Two oracles, and the row is the agreement between them.** The plan side
/// calls [`continuation_case_binder_run`] on the case's own coordinates; the
/// assembled side reads what `define_continuation_bodies` actually installed.
/// The pre-existing binder-run rows pin only the first, and are green even if
/// production stops calling the plan function; the `D6a` environment row pins
/// only the second, and is green under any plan that happens to produce two
/// leading workers. Neither alone says the assembled environment IS the plan.
/// This row asserts the correspondence **positionally**: `IH` and
/// `SelectedRecursiveArgument` must land on static workers, `ContinuationInput`
/// on a carried operand.
///
/// ⛔ **STATED GAP, and it is THIS FIXTURE's alone — `Var(1)` is proven here as
/// a BINDING, not as a call.** MEASURED on the governed fixture: every
/// specialization's emitted static-worker call has callee `Var(0)`, the
/// induction hypothesis; `Var(1)` is bound and never invoked. So this row pins
/// that the argument occupies its exact position with a static-worker binding,
/// which is what `D6a` represents and what a consumer would resolve — it does
/// **not** and cannot pin what calling it emits.
///
/// ⛔ **WITHDRAWN — the unscoped reading of the sentence above.** It used to say
/// *"no witness in this suite calls a `SelectedRecursiveArgument`"*, and that is
/// **false**: the composed source-machine path calls it lawfully, with its exact
/// raw arguments and captures, and
/// [`d8g_the_composed_selected_argument_reaches_its_target_at_the_shared_emitter`]
/// asserts it is emitted from **two** bodies. The gap is a property of the
/// **governed** fixture, which is route-degenerate and never invokes the member;
/// it was never a property of the suite. Architect `evt_6grnfx2psztcn`; the
/// delivering work is `D8d`/`D8e`/`D8j`.
///
/// ⇒ Nothing here is owed. A reader wanting the call, rather than the binding,
/// should go to the `D8e`/`D8j` evidence — **not** to
/// [`d6b_calling_the_selected_recursive_argument_in_the_ordinary_unit_copy_fails_closed_at_the_carrier`],
/// which is a local fail-closed control over one copy of one case body and
/// generalizes to nothing.
///
/// **Promise class: durable invariant.** The assertion is a correspondence
/// between two derivations of the same law. A fixture that grows fields or
/// inputs keeps it green; only a divergence between the plan and the assembly,
/// or a change to the law itself, reds it.
#[test]
fn d6b_the_governed_case_environment_is_the_binder_run_it_was_planned_from() {
    // The governed depth-3 case's own coordinates, as the trace reports them:
    // one constructor argument, which is the recursive one; an empty ordinary
    // envelope, since the only field is the recursive one the envelope
    // excludes by contract; and this frame's single continuation input.
    let run = continuation_case_binder_run(1, &[0], 0, &[], 1)
        .expect("the governed depth-3 case's own coordinates are a lawful binder run");
    assert_eq!(
        run,
        vec![
            ContinuationCaseBinderSource::InductionHypothesis,
            ContinuationCaseBinderSource::SelectedRecursiveArgument { source_position: 0 },
            ContinuationCaseBinderSource::ContinuationInput(0),
        ],
        "the whole run is [IH, ScopeArgument, outer success payload]. The pre-`D6a` run \
         omitted the middle member, which put the continuation input at `Var(1)` and left \
         `Var(2)` unbound -- the exact reported failure"
    );

    let expr = crate::cranelift_backend::planning::governed_nested_resource_bracket(3);
    reset_d5a_trace();
    recursive_port_process_compiles(&expr)
        .unwrap_or_else(|error| panic!("the governed depth-3 fixture must compile: {error}"));
    let trace = take_d5a_trace();

    let mut checked = 0usize;
    for body in trace.iter().filter(|entry| entry.contains("SPEC-BODY")) {
        // Only the specializations whose coordinates are the ones the run above
        // was derived from. A body with a different shape is a different law
        // instance and asserting this vector against it would be a coincidence.
        if !body.contains("binders=1 ordinary=0 envelope=[]") {
            continue;
        }
        let (_, rendered) = body
            .split_once("env=[")
            .unwrap_or_else(|| panic!("every SPEC-BODY entry renders its environment: {body}"));
        let entries = rendered
            .strip_suffix(']')
            .unwrap_or_else(|| panic!("the rendered environment is bracketed: {body}"))
            .split(", ")
            .collect::<Vec<_>>();
        assert!(
            entries.len() >= run.len(),
            "the assembled environment is at least as long as the planned run; a shorter one \
             means a member the plan names was never installed: {body}"
        );
        for (position, source) in run.iter().enumerate() {
            let entry = entries[position];
            let expected_worker = matches!(
                source,
                ContinuationCaseBinderSource::InductionHypothesis
                    | ContinuationCaseBinderSource::SelectedRecursiveArgument { .. }
            );
            assert_eq!(
                entry.starts_with("StaticWorker"),
                expected_worker,
                "at run position {position} the plan says {source:?}, so the assembled \
                 environment must hold {} there. A disagreement means the assembly is not the \
                 plan -- which no binder-run row and no environment-shape row can see on its \
                 own: {body}",
                if expected_worker {
                    "a static worker"
                } else {
                    "a value operand"
                }
            );
            checked += 1;
        }
    }
    assert!(
        checked > 0,
        "no specialization carried the coordinates this run was derived from, so every clause \
         above ran vacuously: {trace:?}"
    );
}

fn rt_scale_b_peak_rss_kib() -> Result<usize, String> {
    let status = std::fs::read_to_string("/proc/self/status")
        .map_err(|error| format!("could not read /proc/self/status: {error}"))?;
    let line = status
        .lines()
        .find(|line| line.starts_with("VmHWM:"))
        .ok_or_else(|| "VmHWM is absent from /proc/self/status".to_string())?;
    line.split_whitespace()
        .nth(1)
        .ok_or_else(|| "VmHWM has no numeric field".to_string())?
        .parse()
        .map_err(|error| format!("VmHWM is not numeric: {error}"))
}

#[test]
fn rt_scale_b_governed_n3_through_n7_collect_every_d2_metric() {
    const WORKER_ENV: &str = "KEN_RT_SCALE_B_EMISSION_WORKER";
    const DEPTH_ENV: &str = "KEN_RT_SCALE_B_EMISSION_DEPTH";
    const FORCE_INDETERMINATE_ENV: &str =
        "KEN_RT_SCALE_B_FORCE_INDETERMINATE";
    const OMIT_RESULT_ENV: &str = "KEN_RT_SCALE_B_OMIT_RESULT";
    const REQUIRED_FIELDS: [&str; 43] = [
        "compile_wall_ns=",
        "peak_rss_kib=",
        "distinct_interned_semantic_states=",
        "defined_helpers=",
        "emitted_helpers=",
        "production_functions=",
        "native_int_functions=",
        "boundary_value_functions=",
        "functionized_root_adapters=",
        "functionized_unit_bodies=",
        "clif_instructions=",
        "clif_bytes=",
        "descriptor_construction_work=",
        "descriptor_comparison_work=",
        "total_dfg_values=",
        "total_instructions=",
        "total_blocks=",
        "static_nodes=",
        "edges=",
        "planned_helpers=",
        "persistent_store_nodes=",
        "out_of_line_evidence_records=",
        "max_helpers_per_static_source=",
        "helper_key_bytes=",
        "activation_frame_bytes=",
        "store_node_bytes=",
        "helper_key_schemas=",
        "frame_schemas=",
        "store_node_schemas=",
        "static_node_id_bytes=",
        "persistent_node_id_bytes=",
        "max_logical_chain_depth=",
        "max_environment_depth=",
        "max_continuation_depth=",
        "max_path_depth=",
        "max_cleanup_depth=",
        "max_affine_depth=",
        "max_source_return_depth=",
        "source_return_resume_nodes=",
        "source_return_owned_resume_edges=",
        "terminal_outgoing_edges=",
        "recursive_lowering_frames=",
        "stack_bytes=",
    ];

    if std::env::var_os(WORKER_ENV).is_none() {
        let run_worker =
            |depth: usize, force_indeterminate: bool, omit_result: bool| {
                let executable = std::env::current_exe().unwrap_or_else(|error| {
                    panic!(
                        "RT_SCALE_B could_not_determine: test executable \
                         could not be located: {error}"
                    )
                });
                let test_name = std::thread::current()
                    .name()
                    .expect("libtest names every test thread")
                    .to_string();
                let mut command = std::process::Command::new("prlimit");
                command
                    .args([
                        "--cpu=30:30",
                        "--as=4294967296:4294967296",
                        "--stack=8388608:8388608",
                        "--",
                    ])
                    .arg(executable)
                    .args(["--exact", &test_name, "--nocapture", "--test-threads=1"])
                    .env(WORKER_ENV, "1")
                    .env(DEPTH_ENV, depth.to_string())
                    .env_remove("RUST_MIN_STACK");
                if force_indeterminate {
                    command.env(FORCE_INDETERMINATE_ENV, "1");
                }
                if omit_result {
                    command.env(OMIT_RESULT_ENV, "1");
                }
                command
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped());
                let mut child = command.spawn().unwrap_or_else(|error| {
                    panic!(
                        "RT_SCALE_B could_not_determine n={depth}: \
                         prlimit worker could not start: {error}"
                    )
                });
                let deadline =
                    std::time::Instant::now() + std::time::Duration::from_secs(45);
                loop {
                    match child.try_wait() {
                        Ok(Some(_)) => {
                            break child.wait_with_output().unwrap_or_else(|error| {
                                panic!(
                                    "RT_SCALE_B could_not_determine n={depth}: \
                                     worker output could not be collected: {error}"
                                )
                            });
                        }
                        Ok(None) if std::time::Instant::now() < deadline => {
                            std::thread::sleep(std::time::Duration::from_millis(25));
                        }
                        Ok(None) => {
                            let _ = child.kill();
                            break child.wait_with_output().unwrap_or_else(|error| {
                                panic!(
                                    "RT_SCALE_B could_not_determine n={depth}: \
                                     timed-out worker could not be reaped: {error}"
                                )
                            });
                        }
                        Err(error) => {
                            let _ = child.kill();
                            panic!(
                                "RT_SCALE_B could_not_determine n={depth}: \
                                 worker status could not be observed: {error}"
                            );
                        }
                    }
                }
            };

        // Promise class: durable invariant and fail-closed measurement gate.
        //
        // MEASURED: five separately bounded product-stack workers complete
        // FunctionizedUnits emission and publish one typed snapshot containing
        // every D2 field.  The forced and omitted-result controls establish
        // that a failed or missing collection is not a silent pass.
        //
        // CLAIMED: the corrected governed family has crossed the real S4/D4
        // exit: RT-SCALE-B can measure completed emission at every n=3..7.
        //
        // THE GAP: this is collection capability, not the later D5 scaling
        // verdict.  Five rows alone prove no asymptotic exponent.
        let forced = run_worker(3, true, false);
        let forced_report = format!(
            "{}{}",
            String::from_utf8_lossy(&forced.stdout),
            String::from_utf8_lossy(&forced.stderr)
        );
        assert!(
            !forced.status.success() && forced_report.contains("could_not_determine"),
            "forced indeterminacy must fail with the stable third-outcome \
             spelling; status={:?}, report={forced_report}",
            forced.status
        );

        let omitted = run_worker(3, false, true);
        let omitted_report = format!(
            "{}{}",
            String::from_utf8_lossy(&omitted.stdout),
            String::from_utf8_lossy(&omitted.stderr)
        );
        assert!(
            omitted.status.success()
                && !omitted_report.contains("status=measured_complete"),
            "missing result data must remain distinguishable from a complete \
             row; status={:?}, report={omitted_report}",
            omitted.status
        );

        let mut rows = Vec::new();
        for depth in 3..=7 {
            let measured = run_worker(depth, false, false);
            let measured_report = format!(
                "{}{}",
                String::from_utf8_lossy(&measured.stdout),
                String::from_utf8_lossy(&measured.stderr)
            );
            eprint!("{measured_report}");
            assert!(
                measured.status.success()
                    && measured_report.contains(&format!(
                        "RT_SCALE_B_RESULT status=measured_complete n={depth}"
                    )),
                "RT_SCALE_B could_not_determine n={depth}: bounded worker \
                 failed or omitted its complete-result sentinel; status={:?}",
                measured.status
            );
            for field in REQUIRED_FIELDS {
                assert!(
                    measured_report.contains(field),
                    "RT_SCALE_B could_not_determine n={depth}: completed row \
                    omitted required field {field}"
                );
            }
            let result = measured_report
                .lines()
                .find(|line| {
                    line.contains(&format!(
                        "RT_SCALE_B_RESULT status=measured_complete n={depth}"
                    ))
                })
                .unwrap_or_else(|| {
                    panic!(
                        "RT_SCALE_B could_not_determine n={depth}: complete \
                         result line is absent"
                    )
                });
            let row = result
                .split_whitespace()
                .filter_map(|field| field.split_once('='))
                .filter_map(|(name, value)| {
                    value
                        .parse::<isize>()
                        .ok()
                        .map(|value| (name.to_string(), value))
                })
                .collect::<BTreeMap<_, _>>();
            for field in REQUIRED_FIELDS {
                let name = field.trim_end_matches('=');
                assert!(
                    row.contains_key(name),
                    "RT_SCALE_B could_not_determine n={depth}: {name} is not a \
                     numeric completed-row field"
                );
            }
            rows.push(row);
        }

        let metric_values = |name: &str| {
            rows.iter()
                .map(|row| {
                    *row.get(name).unwrap_or_else(|| {
                        panic!("completed rows omitted metric {name}")
                    })
                })
                .collect::<Vec<_>>()
        };
        let differences = |values: &[isize]| {
            let first = values
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>();
            let second = first
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>();
            (first, second)
        };
        for field in REQUIRED_FIELDS {
            let name = field.trim_end_matches('=');
            let (first, second) = differences(&metric_values(name));
            eprintln!(
                "RT_SCALE_B_DIFF metric={name} first={first:?} second={second:?}"
            );
        }

        // The four structural invariants are the discriminator. The measured
        // rows corroborate them; they are not an exponent inferred from five
        // points.
        let helper_key_bytes = metric_values("helper_key_bytes");
        assert!(
            helper_key_bytes.iter().all(|width| *width == 12),
            "structural invariant 1: PlannedHelperKey gained payload beyond \
             its closed transition/edge tag and static ID"
        );
        for name in [
            "helper_key_bytes",
            "activation_frame_bytes",
            "store_node_bytes",
            "static_node_id_bytes",
            "persistent_node_id_bytes",
            "helper_key_schemas",
            "frame_schemas",
            "store_node_schemas",
        ] {
            let values = metric_values(name);
            assert!(
                values.windows(2).all(|pair| pair[0] == pair[1]),
                "structural invariant 2: {name} is not constant across n=3..7"
            );
        }
        let persistent_nodes = metric_values("persistent_store_nodes");
        let (persistent_first, persistent_second) =
            differences(&persistent_nodes);
        assert!(
            persistent_first.iter().all(|difference| *difference >= 0)
                && persistent_second.iter().all(|difference| *difference == 0),
            "structural invariant 3: total persistent nodes are not affine"
        );
        let logical_depth = metric_values("max_logical_chain_depth");
        let (_, logical_second) = differences(&logical_depth);
        assert!(
            logical_second.iter().all(|difference| *difference <= 0),
            "structural invariant 4: logical chain depth grows faster than \
             affine across the governed family"
        );
        return;
    }

    let depth = std::env::var(DEPTH_ENV)
        .ok()
        .and_then(|depth| depth.parse::<usize>().ok())
        .filter(|depth| (3..=7).contains(depth))
        .unwrap_or_else(|| {
            panic!(
                "RT_SCALE_B could_not_determine: worker depth is absent or \
                 outside n=3..7"
            )
        });
    if std::env::var_os(FORCE_INDETERMINATE_ENV).is_some() {
        panic!(
            "RT_SCALE_B could_not_determine n={depth}: forced fail-closed \
             positive control"
        );
    }
    if std::env::var_os(OMIT_RESULT_ENV).is_some() {
        return;
    }

    let row = std::thread::Builder::new()
        .name(format!("rt-scale-b-emission-n{depth}-8-mib"))
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            let expr =
                crate::cranelift_backend::planning::governed_nested_resource_bracket(
                    depth,
                );
            let started = std::time::Instant::now();
            recursive_port_process_compiles(&expr).unwrap_or_else(|error| {
                panic!(
                    "RT_SCALE_B could_not_determine n={depth}: completed \
                     emission failed: {error}"
                )
            });
            let compile_wall_ns = usize::try_from(started.elapsed().as_nanos())
                .expect("one bounded compile duration fits usize");
            let peak_rss_kib = rt_scale_b_peak_rss_kib().unwrap_or_else(|error| {
                panic!(
                    "RT_SCALE_B could_not_determine n={depth}: peak RSS \
                     collection failed: {error}"
                )
            });
            let metrics =
                crate::cranelift_backend::lowering::scale_b_last_emission_metrics()
                    .unwrap_or_else(|| {
                        panic!(
                            "RT_SCALE_B could_not_determine n={depth}: \
                             completed-object metric snapshot is absent"
                        )
                    });
            (compile_wall_ns, peak_rss_kib, metrics)
        })
        .unwrap_or_else(|error| {
            panic!(
                "RT_SCALE_B could_not_determine n={depth}: 8 MiB product-stack \
                 worker could not start: {error}"
            )
        })
        .join()
        .unwrap_or_else(|_| {
            panic!(
                "RT_SCALE_B could_not_determine n={depth}: 8 MiB product-stack \
                 worker panicked"
            )
        });

    let (compile_wall_ns, peak_rss_kib, metrics) = row;
    assert!(compile_wall_ns > 0, "compile wall time was not collected");
    assert!(peak_rss_kib > 0, "peak RSS was not collected");
    assert!(
        metrics.authority_functionized,
        "completed row came from the retained authority"
    );
    assert_eq!(
        metrics.emitted_helpers, metrics.plan.defined_helpers,
        "planned helper definitions and emitted unit bodies disagree"
    );
    assert_eq!(
        metrics.production_functions,
        metrics.native_int_functions
            + metrics.boundary_value_functions
            + metrics.functionized_root_adapters
            + metrics.functionized_unit_bodies,
        "the completed denominator must equal the closed typed emitter \
         population"
    );
    assert_eq!(
        metrics.functionized_unit_bodies, metrics.emitted_helpers,
        "the emitted-helper numerator must be the unit-body population"
    );
    assert_eq!(
        metrics.functionized_root_adapters, 1,
        "one completed governed object must contain one public root adapter"
    );
    assert_eq!(
        metrics.native_int_functions, 7,
        "the closed native-Int graph must contribute all seven definitions"
    );
    assert_eq!(
        metrics.boundary_value_functions, 30,
        "the closed boundary-value graph must contribute all thirty \
         definitions"
    );
    for (name, value) in [
        (
            "distinct_interned_semantic_states",
            metrics.plan.distinct_interned_semantic_states,
        ),
        ("defined_helpers", metrics.plan.defined_helpers),
        ("emitted_helpers", metrics.emitted_helpers),
        ("clif_instructions", metrics.clif_instructions),
        ("clif_bytes", metrics.clif_bytes),
        (
            "descriptor_construction_work",
            metrics.plan.descriptor_construction_work,
        ),
        (
            "descriptor_comparison_work",
            metrics.plan.descriptor_comparison_work,
        ),
        ("total_dfg_values", metrics.total_dfg_values),
        ("total_instructions", metrics.total_instructions),
        ("total_blocks", metrics.total_blocks),
        ("static_nodes", metrics.plan.static_nodes),
        ("edges", metrics.plan.edges),
        ("planned_helpers", metrics.plan.planned_helpers),
        (
            "persistent_store_nodes",
            metrics.plan.persistent_store_nodes,
        ),
    ] {
        assert!(value > 0, "required D2 metric {name} was not collected");
    }

    let plan = &metrics.plan;
    eprintln!(
        "RT_SCALE_B_RESULT status=measured_complete n={depth} \
         authority=FunctionizedUnits compile_wall_ns={compile_wall_ns} \
         peak_rss_kib={peak_rss_kib} \
         distinct_interned_semantic_states={} defined_helpers={} \
         emitted_helpers={} production_functions={} native_int_functions={} \
         boundary_value_functions={} functionized_root_adapters={} \
         functionized_unit_bodies={} \
         clif_instructions={} clif_bytes={} descriptor_construction_work={} \
         descriptor_comparison_work={} total_dfg_values={} \
         total_instructions={} total_blocks={} static_nodes={} edges={} \
         planned_helpers={} persistent_store_nodes={} \
         out_of_line_evidence_records={} max_helpers_per_static_source={} \
         helper_key_bytes={} activation_frame_bytes={} store_node_bytes={} \
         helper_key_schemas={} frame_schemas={} store_node_schemas={} \
         static_node_id_bytes={} persistent_node_id_bytes={} \
         max_logical_chain_depth={} max_environment_depth={} \
         max_continuation_depth={} max_path_depth={} max_cleanup_depth={} \
         max_affine_depth={} max_source_return_depth={} \
         source_return_resume_nodes={} source_return_owned_resume_edges={} \
         terminal_outgoing_edges={} recursive_lowering_frames={} \
         stack_bytes=8388608",
        plan.distinct_interned_semantic_states,
        plan.defined_helpers,
        metrics.emitted_helpers,
        metrics.production_functions,
        metrics.native_int_functions,
        metrics.boundary_value_functions,
        metrics.functionized_root_adapters,
        metrics.functionized_unit_bodies,
        metrics.clif_instructions,
        metrics.clif_bytes,
        plan.descriptor_construction_work,
        plan.descriptor_comparison_work,
        metrics.total_dfg_values,
        metrics.total_instructions,
        metrics.total_blocks,
        plan.static_nodes,
        plan.edges,
        plan.planned_helpers,
        plan.persistent_store_nodes,
        plan.out_of_line_evidence_records,
        plan.max_helpers_per_static_source,
        plan.helper_key_bytes,
        plan.activation_frame_bytes,
        plan.store_node_bytes,
        plan.helper_key_schemas,
        plan.frame_schemas,
        plan.store_node_schemas,
        plan.static_node_id_bytes,
        plan.persistent_node_id_bytes,
        plan.max_logical_chain_depth,
        plan.max_environment_depth,
        plan.max_continuation_depth,
        plan.max_path_depth,
        plan.max_cleanup_depth,
        plan.max_affine_depth,
        plan.max_source_return_depth,
        plan.source_return_resume_nodes,
        plan.source_return_owned_resume_edges,
        plan.terminal_outgoing_edges,
        plan.recursive_lowering_frames,
    );
}

fn d8_mixed_host_result_join_fixture(swapped: bool) -> RuntimeExpr {
    let carried = crate::RuntimeMatchCase {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        binders: 1,
        body: RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: Vec::new(),
                body: Box::new(RuntimeExpr::Value(crate::RuntimeValue::Int(11.into()))),
            }),
            args: Vec::new(),
        },
    };
    let specialized = crate::RuntimeMatchCase {
        constructor: "ctor:prelude::Result::Err".to_string(),
        binders: 1,
        body: RuntimeExpr::Value(crate::RuntimeValue::Int(7.into())),
    };
    RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Effect {
            family: "FS".to_string(),
            operation: ken_host::HostOpV1::BufferAllocate,
            capability: None,
            args: vec![RuntimeExpr::Value(crate::RuntimeValue::Int(1.into()))],
        }),
        cases: if swapped {
            vec![specialized, carried]
        } else {
            vec![carried, specialized]
        },
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8 mixed HostResult default".to_string(),
        },
    }
}

#[test]
fn d8_mixed_host_result_uses_one_uniform_carrier_conversion_per_predecessor() {
    for swapped in [false, true] {
        let expr = d8_mixed_host_result_join_fixture(swapped);
        recursive_port_process_compiles(&expr).expect("D8 mixed HostResult compiles");
        assert_eq!(
            d8_join_conversion_counts(),
            (1, 1),
            "arm order changed carried pass-through or specialized production"
        );
        assert_eq!(d8_join_merge_count(), 1, "mixed join emitted no unique merge");
    }
}

/// **MEASURED:** a dynamic HostResult creates its planned merge through the
/// central materialized-block recorder. A test-only false dead disposition then
/// reaches completed-CFG validation, which observes the real entry-reachable
/// merge block and rejects it.
///
/// **CLAIMED:** the HostResult merge belongs to the complete population over
/// which the materialized-but-dead CFG proof quantifies.
///
/// **THE GAP:** the false dead disposition is only a reachability witness; it
/// does not prove population membership by itself. Replacing the recorder call
/// with direct block-parameter appends recreates the missed production path:
/// the block list becomes empty, this compile succeeds, and the control reds at
/// `expect_err`.
#[test]
fn d8_dynamic_host_result_merge_enters_materialized_dead_cfg_population() {
    set_d8_join_consumption_mutation(
        JoinConsumptionMutation::DispositionDynamicHostResultMerge,
    );
    let result = recursive_port_process_compiles(&d8_mixed_host_result_join_fixture(false));
    set_d8_join_consumption_mutation(JoinConsumptionMutation::Exact);

    let reachable_dead =
        result.expect_err("a dispositioned dynamic HostResult merge must reach CFG validation");
    assert!(
        matches!(
            reachable_dead,
            CraneliftBackendError::Backend(BackendFailure::Module(ref detail))
                if detail.contains("materialized-but-dead source join")
                    && detail.contains("retained a reachable block")
        ),
        "dynamic HostResult population control reached the wrong boundary: \
         {reachable_dead:?}"
    );
}

#[test]
fn d8_all_trap_host_result_emits_no_merge_or_predecessor_conversion() {
    let mut expr = d8_mixed_host_result_join_fixture(false);
    let RuntimeExpr::Match { cases, .. } = &mut expr else {
        unreachable!("D8 fixture is a Match");
    };
    for case in cases {
        case.body = RuntimeExpr::Trap(RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8 all-trap arm".to_string(),
        });
    }
    recursive_port_process_compiles(&expr).expect("D8 all-trap HostResult compiles");
    assert_eq!(d8_join_merge_count(), 0);
    assert_eq!(d8_join_conversion_counts(), (0, 0));
}

#[test]
fn d8_unsupported_carrier_production_publishes_no_unit_function() {
    let mut expr = d8_mixed_host_result_join_fixture(false);
    let RuntimeExpr::Match { cases, .. } = &mut expr else {
        unreachable!("D8 fixture is a Match");
    };
    let specialized = cases
        .iter_mut()
        .find(|case| case.constructor == "ctor:prelude::Result::Err")
        .expect("D8 fixture has an Err arm");
    specialized.body = RuntimeExpr::Closure {
        captures: Vec::new(),
        params: Vec::new(),
        body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
    };

    let failure =
        recursive_port_process_compiles(&expr).expect_err("closure carrier transfer must fail");
    assert!(matches!(
        failure,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "Closure",
            ref reason,
        }) if reason.contains("a closure cannot cross the boundary")
    ));
    let (declared, defined) =
        crate::cranelift_backend::lowering::units::b2f_last_unit_emission();
    assert!(declared > 0, "fixture never reached the unit emission path");
    assert_eq!(
        defined, 0,
        "unsupported carrier production defined a partial unit population"
    );
}

// RETIRED by the RT-FNSPLIT-RECUR-PORT successor repair: caller-name counts
// over repository text are not a behavioral representation proof. The borrowed
// ingress `bytes_at` control exercises a CarrierWord predecessor through the
// borrowed Option merge instead.
#[cfg(any())]
fn d8_join_helpers_have_the_closed_typed_caller_population() {
    let helpers = include_str!("../../mod.rs");
    let callers = include_str!("../../core.rs");
    for name in [
        "merge_branch_value",
        "merge_scalar_branch",
        "merge_planned_scalar_branch",
    ] {
        assert_eq!(
            helpers.matches(&format!("fn {name}(")).count(),
            1,
            "D8 join helper family changed: {name}"
        );
    }
    assert_eq!(callers.matches(".merge_branch_value(").count(), 4);
    assert_eq!(callers.matches(".merge_scalar_branch(").count(), 10);
    assert_eq!(
        callers.matches(".merge_planned_scalar_branch(").count(),
        1
    );
    assert_eq!(
        helpers.matches("plan: &JoinPlanToken").count(),
        3,
        "every D8 helper must require the unmintable typed plan token"
    );
}

fn d8_known_if_with_dead_join_sibling(selected: bool) -> RuntimeExpr {
    let dead = RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: Vec::new(),
                body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(7.into()))),
            }),
            args: Vec::new(),
        }),
        body: Box::new(RuntimeExpr::If {
            scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(false))),
            then_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Int(11.into()))),
            else_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Int(13.into()))),
        }),
    };
    let live = RuntimeExpr::Value(RuntimeValue::Int(3.into()));
    RuntimeExpr::If {
        scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(selected))),
        then_expr: Box::new(if selected { live.clone() } else { dead.clone() }),
        else_expr: Box::new(if selected { dead } else { live }),
    }
}

fn d8_dead_nested_join(value: i64) -> RuntimeExpr {
    RuntimeExpr::If {
        scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        then_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Int(value.into()))),
        else_expr: Box::new(RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: Vec::new(),
                body: Box::new(RuntimeExpr::Value(RuntimeValue::Int((value + 1).into()))),
            }),
            args: Vec::new(),
        }),
    }
}

fn d8_known_bool_match_with_dead_join_case(selected: bool) -> RuntimeExpr {
    let live = RuntimeExpr::Value(RuntimeValue::Int(3.into()));
    let dead = d8_dead_nested_join(5);
    RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(selected))),
        cases: vec![
            RuntimeMatchCase {
                constructor: "ctor:prelude::Bool::True".to_string(),
                binders: 0,
                body: if selected { live.clone() } else { dead.clone() },
            },
            RuntimeMatchCase {
                constructor: "ctor:prelude::Bool::False".to_string(),
                binders: 0,
                body: if selected { dead } else { live },
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8 known Bool match default".to_string(),
        },
    }
}

fn d8_known_constructor_match_with_dead_join_case(matching: bool) -> RuntimeExpr {
    RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: if matching {
                "ctor:fixture::D8::Selected".to_string()
            } else {
                "ctor:fixture::D8::Missing".to_string()
            },
            args: Vec::new(),
        }),
        cases: vec![
            RuntimeMatchCase {
                constructor: "ctor:fixture::D8::Dead".to_string(),
                binders: 0,
                body: d8_dead_nested_join(7),
            },
            RuntimeMatchCase {
                constructor: "ctor:fixture::D8::Selected".to_string(),
                binders: 0,
                body: RuntimeExpr::Value(RuntimeValue::Int(3.into())),
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8 known constructor match default".to_string(),
        },
    }
}

fn d8_producer_match_with_dead_join_case(constructor_scrutinee: bool) -> RuntimeExpr {
    let selected_constructor = "ctor:fixture::D8::ProducerSelected".to_string();
    let inner_scrutinee = if constructor_scrutinee {
        RuntimeExpr::Construct {
            constructor: selected_constructor.clone(),
            args: Vec::new(),
        }
    } else {
        RuntimeExpr::Value(RuntimeValue::Bool(true))
    };
    let selected_case = RuntimeMatchCase {
        constructor: if constructor_scrutinee {
            selected_constructor
        } else {
            "ctor:prelude::Bool::True".to_string()
        },
        binders: 0,
        body: RuntimeExpr::Construct {
            constructor: "ctor:fixture::D8::Wrap".to_string(),
            args: Vec::new(),
        },
    };
    let dead_case = RuntimeMatchCase {
        constructor: if constructor_scrutinee {
            "ctor:fixture::D8::ProducerDead".to_string()
        } else {
            "ctor:prelude::Bool::False".to_string()
        },
        binders: 0,
        body: RuntimeExpr::If {
            scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
            then_expr: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::D8::Wrap".to_string(),
                args: Vec::new(),
            }),
            else_expr: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::D8::Wrap".to_string(),
                        args: Vec::new(),
                    }),
                }),
                args: Vec::new(),
            }),
        },
    };
    RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Match {
            scrutinee: Box::new(inner_scrutinee),
            cases: vec![selected_case, dead_case],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "D8 producer match default".to_string(),
            },
        }),
        cases: vec![RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Wrap".to_string(),
            binders: 0,
            body: RuntimeExpr::Value(RuntimeValue::Int(11.into())),
        }],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8 producer consumer default".to_string(),
        },
    }
}

fn d8_producer_no_match_with_dead_join_case() -> RuntimeExpr {
    RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::D8::ProducerMissing".to_string(),
                args: Vec::new(),
            }),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::D8::ProducerDead".to_string(),
                binders: 0,
                body: RuntimeExpr::If {
                    scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                    then_expr: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::D8::Wrap".to_string(),
                        args: Vec::new(),
                    }),
                    else_expr: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::D8::Wrap".to_string(),
                        args: Vec::new(),
                    }),
                },
            }],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "D8 producer no-match default".to_string(),
            },
        }),
        cases: vec![RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Wrap".to_string(),
            binders: 0,
            body: RuntimeExpr::Value(RuntimeValue::Int(11.into())),
        }],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8 producer no-match consumer default".to_string(),
        },
    }
}

fn d8_source_machine_with_match(body: RuntimeExpr) -> RuntimeExpr {
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::D8::Node".to_string(),
            args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
        }),
        cases: vec![crate::RuntimeComputationalMatchCase {
            constructor: "ctor:fixture::D8::Node".to_string(),
            argument_binders: 1,
            recursive_positions: vec![0],
            body,
        }],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8 source-machine match default".to_string(),
        },
    }
}

fn d8_recursive_computational_revisit_with_join() -> RuntimeExpr {
    let aggregate = RuntimeExpr::Construct {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let later_case_body = RuntimeExpr::If {
        scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        then_expr: Box::new(aggregate.clone()),
        else_expr: Box::new(RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: Vec::new(),
                body: Box::new(aggregate),
            }),
            args: Vec::new(),
        }),
    };
    host_result_closure_match(recursive_computational_result_depth(2, later_case_body))
}

/// MEASURED: successful FunctionizedUnits emission closes two facts separately.
/// Each generated function consumes each owner-bound join token at most once,
/// then, after its reached-case union closes, partitions its complete semantic
/// owner population exactly once into reachable and statically unselected
/// joins. A token-only materialization may later be classified dead. If the
/// mutation attaches that dead materialization to the entry block, completed
/// CFG validation rejects it as reachable. The retired active-recursor route
/// closed the same recorded case population at its generated root boundary
/// without applying function-owner equality across the owner boundaries it
/// deliberately inlined. Its population fixture recursively
/// revisits one ComputationalMatch and puts a source join in the second selected
/// case.
///
/// CLAIMED: every required planned FunctionizedUnits source join has
/// owner-bound, exactly-once materialization and final semantic disposition.
/// Materialized-then-dead is lawful only when no generated merge block remains
/// entry-reachable, has a live predecessor, or contributes a reachable use.
///
/// THE GAP: owner equality and token consumption do not establish semantic
/// reachability. The known-true/known-false `If` pair plus ordinary and producer
/// `Match` discriminators place both a `Call` and nested `If` in dead source
/// subtrees. Population-side mutations leave one such subtree or case
/// unclassified and red at generated-function closure. Set equality still
/// supplies omission/wrong-owner closure. A route-specific reached-edge removal
/// mutation proves that recursive source-machine selections contribute to the
/// same union; it now pairs the false dead classification with a reachable
/// materialization and reds at completed-CFG validation. The token-only
/// materialized/dead fixture and its entry-attachment mutation distinguish the
/// lawful ordering from a live block. The insertion guard in
/// `consume_join_plan` supplies the independent duplicate direction.
///
/// Promise class: durable invariant.
#[test]
fn d8_every_required_join_plan_is_consumed_exactly_once() {
    let expr = d8_mixed_host_result_join_fixture(false);
    recursive_port_process_compiles(&expr).expect("the exact consumption set compiles");

    for selected in [true, false] {
        recursive_port_process_compiles(&d8_known_if_with_dead_join_sibling(selected))
            .unwrap_or_else(|error| {
                panic!(
                    "known-{selected} If did not disposition its dead Call/nested-If sibling: \
                     {error}"
                )
            });
    }

    for selected in [true, false] {
        recursive_port_process_compiles(&d8_known_bool_match_with_dead_join_case(selected))
            .unwrap_or_else(|error| {
                panic!(
                    "known-{selected} Bool Match did not disposition its dead nested-join case: \
                     {error}"
                )
            });
    }
    recursive_port_process_compiles(&d8_known_constructor_match_with_dead_join_case(true))
        .expect("known constructor Match dispositions every nonselected case");
    recursive_port_process_compiles(&d8_known_constructor_match_with_dead_join_case(false))
        .expect("no-match/default route dispositions every source case");
    for constructor_scrutinee in [false, true] {
        recursive_port_process_compiles(&d8_producer_match_with_dead_join_case(
            constructor_scrutinee,
        ))
        .unwrap_or_else(|error| {
            panic!(
                "{} producer Match did not disposition its dead nested-join case: {error}",
                if constructor_scrutinee {
                    "known-constructor"
                } else {
                    "known-Bool"
                }
            )
        });
    }
    recursive_port_process_compiles(&d8_producer_no_match_with_dead_join_case())
        .expect("producer no-match/default route dispositions every source case");
    for (route, body) in [
        (
            "known-Bool",
            d8_known_bool_match_with_dead_join_case(true),
        ),
        (
            "known-constructor",
            d8_known_constructor_match_with_dead_join_case(true),
        ),
        (
            "no-match/default",
            d8_known_constructor_match_with_dead_join_case(false),
        ),
    ] {
        recursive_port_process_compiles(&d8_source_machine_with_match(body))
            .unwrap_or_else(|error| {
                panic!("source-machine {route} Match did not disposition dead cases: {error}")
            });
    }
    recursive_port_process_compiles(&d8_recursive_computational_revisit_with_join())
        .expect("a recursive computational revisit unions its later selected case");

    set_d8_join_consumption_mutation(
        JoinConsumptionMutation::OmitSourceMachineComputationalMatchSelection,
    );
    let recursive_revisit_result =
        recursive_port_process_compiles(&d8_recursive_computational_revisit_with_join());
    set_d8_join_consumption_mutation(JoinConsumptionMutation::Exact);
    let recursive_revisit_omitted = recursive_revisit_result
        .expect_err("omitting a recursive revisit selection must fail at function closure");
    assert!(
        matches!(
            recursive_revisit_omitted,
            CraneliftBackendError::Backend(BackendFailure::Module(ref detail))
                if detail.contains("materialized-but-dead source join")
                    && detail.contains("retained a reachable block")
        ),
        "recursive-revisit reached-edge mutation reached the wrong boundary: \
         {recursive_revisit_omitted:?}"
    );

    set_d8_join_consumption_mutation(JoinConsumptionMutation::IncludeStaticallyUnselected);
    let dead_included =
        recursive_port_process_compiles(&d8_known_if_with_dead_join_sibling(true))
            .expect_err("including a dead sibling join must fail at function closure");
    set_d8_join_consumption_mutation(JoinConsumptionMutation::Exact);
    assert!(
        matches!(
            dead_included,
            CraneliftBackendError::Backend(BackendFailure::Module(ref detail))
                if detail.contains("neither emitted nor statically unselected")
        ),
        "dead-sibling population mutation reached the wrong boundary: {dead_included:?}"
    );

    for (route, expression) in [
        (
            "ordinary known-Bool",
            d8_known_bool_match_with_dead_join_case(true),
        ),
        (
            "ordinary known-constructor",
            d8_known_constructor_match_with_dead_join_case(true),
        ),
        (
            "ordinary no-match/default",
            d8_known_constructor_match_with_dead_join_case(false),
        ),
        (
            "producer known-Bool",
            d8_producer_match_with_dead_join_case(false),
        ),
        (
            "producer known-constructor",
            d8_producer_match_with_dead_join_case(true),
        ),
        (
            "producer no-match/default",
            d8_producer_no_match_with_dead_join_case(),
        ),
        (
            "source-machine known-Bool",
            d8_source_machine_with_match(d8_known_bool_match_with_dead_join_case(true)),
        ),
        (
            "source-machine known-constructor",
            d8_source_machine_with_match(d8_known_constructor_match_with_dead_join_case(true)),
        ),
        (
            "source-machine no-match/default",
            d8_source_machine_with_match(d8_known_constructor_match_with_dead_join_case(false)),
        ),
    ] {
        set_d8_join_consumption_mutation(
            JoinConsumptionMutation::OmitFirstStaticallyUnselectedMatchCase,
        );
        let result = recursive_port_process_compiles(&expression);
        set_d8_join_consumption_mutation(JoinConsumptionMutation::Exact);
        let dead_case_omitted = match result {
            Ok(()) => panic!("{route} accepted an omitted dead Match case"),
            Err(error) => error,
        };
        assert!(
            matches!(
                dead_case_omitted,
                CraneliftBackendError::Backend(BackendFailure::Module(ref detail))
                    if detail.contains("neither emitted nor statically unselected")
            ),
            "{route} dead-case omission reached the wrong boundary: {dead_case_omitted:?}"
        );
    }

    // A statically selected `If` still belongs to the planner's closed join
    // population, but no merge helper needs to reborrow its token. Skipping
    // that traversal entry therefore reaches the end-of-function equality
    // check rather than an earlier token-use guard.
    let omission_fixture = RuntimeExpr::If {
        scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        then_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Int(3.into()))),
        else_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Int(5.into()))),
    };

    set_d8_join_consumption_mutation(
        JoinConsumptionMutation::MaterializeFirstUnselectedMatchJoin,
    );
    let materialized_then_dead =
        recursive_port_process_compiles(&d8_known_bool_match_with_dead_join_case(true));
    set_d8_join_consumption_mutation(JoinConsumptionMutation::Exact);
    materialized_then_dead.expect(
        "token-only materialization may become dead after final semantic selection",
    );

    set_d8_join_consumption_mutation(
        JoinConsumptionMutation::AttachEntryToFirstMaterializedDead,
    );
    let reachable_dead_result =
        recursive_port_process_compiles(&d8_known_bool_match_with_dead_join_case(true));
    set_d8_join_consumption_mutation(JoinConsumptionMutation::Exact);
    let reachable_dead = reachable_dead_result
        .expect_err("an entry-reachable materialized-but-dead join must fail");
    assert!(
        matches!(
            reachable_dead,
            CraneliftBackendError::Backend(BackendFailure::Module(ref detail))
                if detail.contains("materialized-but-dead source join")
                    && detail.contains("retained a reachable block")
        ),
        "materialized-dead reachability mutation reached the wrong boundary: \
         {reachable_dead:?}"
    );

    set_d8_join_consumption_mutation(JoinConsumptionMutation::SkipFirst);
    let omitted = recursive_port_process_compiles(&omission_fixture)
        .expect_err("skipping one real consumption must fail at function closure");
    set_d8_join_consumption_mutation(JoinConsumptionMutation::Exact);
    assert!(
        matches!(
            omitted,
            CraneliftBackendError::Backend(BackendFailure::Module(ref detail))
                if detail.contains("left planned source join")
        ),
        "omission mutation reached the wrong boundary: {omitted:?}"
    );

    set_d8_join_consumption_mutation(JoinConsumptionMutation::DuplicateFirst);
    let duplicate = recursive_port_process_compiles(&expr)
        .expect_err("consuming one real join twice must fail at token consumption");
    set_d8_join_consumption_mutation(JoinConsumptionMutation::Exact);
    assert!(
        matches!(
            duplicate,
            CraneliftBackendError::Backend(BackendFailure::Module(ref detail))
                if detail.contains("more than once")
        ),
        "duplicate mutation reached the wrong boundary: {duplicate:?}"
    );
}

/// **`AC-11` clause 3 — an unrepresentable transfer is refused BEFORE any unit
/// is declared.**
///
/// ⭐ **Why the timing is the property and not a detail.** The late refusal that
/// also rejects these fixtures lives in `lower_expr`'s `ImportedDeclarationRef`
/// arm — which is the recursive-descent inliner that **`D6`/`S7` removes**. A
/// refusal performed by the authority being retired is not a property of the
/// surviving boundary, so "it is rejected either way" is true today and becomes
/// false at `S7`, silently, with no test reddening at the moment the hole opens.
/// ⇒ The check must be shown to refuse *on the pre-emission side*, and only a
/// timing discriminator can show that.
///
/// ⛔⛔ **The first version of this control could not measure that, and reported
/// a confident number for the wrong thing.** It compiled a successful sentinel
/// to force the unit counter nonzero, then read the counter back after the
/// failing compile — but no pre-emission refusal path *writes* that counter, so
/// the reading was the sentinel's own `1`. "Refused before emission" and
/// "declared a unit, then refused" produced the **identical** value. ⇒ The
/// measured `holeA = 1` / `holeB = 1` was **stale recorder state, not late
/// refusal**, and the conclusion drawn from it — that the walk is inert — was
/// unsupported in both directions. See `units::b2f_open_compile_attempt`.
///
/// ⭐ **The repair is an attempt epoch stamped at the emission seam**, which
/// makes three outcomes distinct: `None` (never reached emission), `Some(0)`
/// (reached it, refused before declaring), `Some(n > 0)` (declared, then
/// refused). ⛔ `None` is **not** a pass — it would mean the fixture died even
/// earlier, for a reason unrelated to the walk.
///
/// ⛔ **Without the accepted rows this test is worthless.** A walk that rejects
/// every program satisfies both rejection rows and is a catastrophic
/// over-rejection; the paired intra-module fixtures are what distinguish
/// "rejects an unrepresentable transfer" from "rejects".
///
/// **MEASURED:** six compiles — a wrapped import and a bare-body import are
/// refused with `Some(0)` units declared in their own attempt; the same two
/// shapes with an intra-module value are accepted; and a successful compile
/// reports `Some(n > 0)` in its own attempt, so `Some(0)` is a real reading and
/// not a counter that never moves.
/// **CLAIMED:** the producer walk decides on the value that reaches the slot,
/// not on the occurrence's own top-level shape, and it decides **before** the
/// switch-over can emit or call a unit.
/// **THE GAP:** ⛔ this exercises the `If` pass-through only. A `Match` arm is
/// not traced (see `producers_of`), so an import reaching a slot through a match
/// arm is **not** covered by this test or by the walk. ⛔ And the `Parameter`
/// transfer population is **empty** until `S5` supplies call sites, so clause 1
/// is discharged here for `Capture` and `Result` only.
#[test]
fn an_unrepresentable_transfer_is_refused_before_any_unit_is_declared() {
    // ⭐ Hole A. Binder-free: no de Bruijn reading makes this `If`'s result
    // anything but the imported value, yet its top-level shape is `If`, so a
    // check on the capture child's own shape admits it.
    let wrapped_import = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::LexicalClosure {
            captures: vec![RuntimeExpr::If {
                scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                then_expr: Box::new(ac11_imported()),
                else_expr: Box::new(ac11_imported()),
            }],
            params: Vec::new(),
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        }),
        args: Vec::new(),
    };
    // ⭐ Hole B. No wrapper at all: `C4` iterates capture children, and there
    // are none, so the unit's own result slot is never carrier-checked.
    let bare_body_import = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: Vec::new(),
            body: Box::new(ac11_imported()),
        }),
        args: Vec::new(),
    };
    // ⭐ The two POSITIVE CONTROLS: identical shapes, intra-module values.
    let wrapped_intra_module = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::LexicalClosure {
            captures: vec![RuntimeExpr::If {
                scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                then_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                else_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(false))),
            }],
            params: Vec::new(),
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        }),
        args: Vec::new(),
    };
    let bare_body_intra_module = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: Vec::new(),
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        }),
        args: Vec::new(),
    };

    assert!(
        ac11_compiles(&wrapped_intra_module).is_ok(),
        "AC-11 -- POSITIVE CONTROL: a binder-free wrapper over intra-module \
         values must still compile. If this fails the walk is rejecting on the \
         wrapper rather than on what flows through it, and both rejection rows \
         below are satisfied for the wrong reason."
    );
    assert!(
        ac11_compiles(&bare_body_intra_module).is_ok(),
        "AC-11 -- POSITIVE CONTROL: a closure body producing an intra-module \
         value must still compile."
    );

    // ⭐⭐ THE DISCRIMINATOR IS *WHEN*, NOT *WHETHER* — and reading a shared
    // counter cannot answer *when*, because a compile that refuses early does
    // not write it. Every reading below is stamped with the attempt that
    // produced it, so a stale value reads as `None` instead of as a count.
    fn units_declared_when_refused(expr: &RuntimeExpr) -> Option<usize> {
        let epoch = crate::cranelift_backend::lowering::units::b2f_open_compile_attempt();
        assert!(ac11_compiles(expr).is_err(), "fixture must be refused");
        crate::cranelift_backend::lowering::units::b2f_units_declared_in_attempt(epoch)
    }

    // ⛔ POSITIVE CONTROL ON THE INSTRUMENT ITSELF, and it is not optional: the
    // rejection rows below assert `Some(0)`, which is exactly what a stamp that
    // fires alongside a counter that never increments would also report. This
    // row proves the counter moves within a single stamped attempt, so `Some(0)`
    // is a measurement rather than a reader that is stuck at zero.
    let instrument_epoch = crate::cranelift_backend::lowering::units::b2f_open_compile_attempt();
    ac11_compiles(&wrapped_intra_module).expect("instrument control compiles");
    let declared_when_accepted =
        crate::cranelift_backend::lowering::units::b2f_units_declared_in_attempt(instrument_epoch);
    assert!(
        matches!(declared_when_accepted, Some(n) if n > 0),
        "AC-11 clause 3 -- INSTRUMENT CONTROL: a compile that runs to completion \
         must report a NONZERO declaration count inside its own attempt. Got \
         {declared_when_accepted:?}. If this is Some(0) the counter is dead and \
         every `Some(0)` below is vacuous; if it is None the seam stamp never \
         fired and the epoch reads nothing at all."
    );

    let wrapped = ac11_compiles(&wrapped_import);
    assert!(
        matches!(wrapped, Err(CraneliftBackendError::Unsupported(_))),
        "AC-11 -- HOLE A: an imported value reaching a Capture slot through a \
         binder-free `If` must be refused before emission. Checking the capture \
         child's own top-level shape admits this: {wrapped:?}"
    );
    let bare = ac11_compiles(&bare_body_import);
    assert!(
        matches!(bare, Err(CraneliftBackendError::Unsupported(_))),
        "AC-11 -- HOLE B: an imported value reaching the unit's own Result slot \
         must be refused before emission. It needs no wrapper, and a check that \
         iterates capture children never sees it: {bare:?}"
    );

    // ⛔ CLAUSE 3. `Some(0)` means the compile reached the emission seam and was
    // refused there, before the bundle was forward-declared — i.e. before the
    // switch-over could emit or call anything. `Some(n > 0)` means the program
    // got past the walk and was refused *later*, by the recursive-descent
    // inliner that `D6`/`S7` deletes, which is a guarantee that expires.
    //
    // ⭐ This is a DURABLE INVARIANT, not a sentinel. It does not pin a count
    // that today's code happens to produce; it pins the side of the emission
    // boundary the refusal must come from, which every intended extension of
    // this node must preserve. Removing `lower_expr`'s late arm at `S7` must
    // leave it green — that is the whole point of asserting it now.
    assert_eq!(
        units_declared_when_refused(&wrapped_import),
        Some(0),
        "AC-11 clause 3 -- HOLE A: the refusal must come from the pre-emission \
         walk, with zero units declared in this compile's own attempt. \
         Some(n>0) means the walk let it through and the late `lower_expr` arm \
         refused it instead -- a refusal performed by the authority S7 removes. \
         None means the compile never reached the emission seam at all."
    );
    assert_eq!(
        units_declared_when_refused(&bare_body_import),
        Some(0),
        "AC-11 clause 3 -- HOLE B: an imported value reaching the unit's own \
         Result slot must be refused pre-emission, with zero units declared in \
         this compile's own attempt."
    );
}

// ─── RT-FNSPLIT-B2F D3 — artifact-static seed material, measured BEHAVIOURALLY ─

/// A program that captures one seed symbol and returns it, compiled against an
/// environment that binds that symbol to `value`.
///
/// ⚠ The closure is **called**, not returned, for the same reason the unit
/// fixture above calls its closure: a closure is not an observable ground value
/// at the root, so a fixture that merely mentions one never reaches emission and
/// would measure nothing while looking like a discriminator.
#[cfg(test)]
fn b2f_seed_capture_program(symbol: &str, value: RuntimeGroundValue) -> NativeSeedEnvironment {
    let mut env = NativeSeedEnvironment::empty();
    env.insert(symbol, value);
    env
}

/// **`AC-2`, data half — the minted artifact-static population, counted at the
/// point of emission.**
///
/// ⭐ **This is the instrument the amended `AC-2` names as PRIMARY**, and the
/// reason is the failure direction rather than the needle list: the source-text
/// census's default branch is *"needle not found ⇒ nothing emitted"*, so it
/// fails **open** for every emission spelling nobody enumerated. `D3`'s data
/// objects were exactly such a spelling — the census read complete across every
/// row while it could not see a single one. This counter observes what the
/// module **contains**, so an unanticipated spelling cannot hide in it.
///
/// **MEASURED:** the `(declared, defined)` artifact-static object counts for two
/// compiles that differ only in whether the seed environment is empty.
/// **CLAIMED:** one read-only artifact-static object is minted and defined per
/// seed-environment entry.
/// **THE GAP:** ⛔ this says nothing about the object's *contents*, nor about
/// whether any emitted code reads it. Contents are pinned by the encoder tests
/// in `seed_material`; the reading is
/// **`RT-FNSPLIT-B2F` `D4` — the resolved call-edge population is DERIVED from
/// the program, not a constant this node carries.**
///
/// ⛔ **This deliberately does NOT re-assert `B2O`'s four edge-classification
/// laws.** `validate_function_units` enforces all four as `return Err` arms in
/// landed production bytes, so planning **refuses to construct** a violating
/// graph — ⇒ a `B2F` control asserting "a `StaticBody` edge crosses owners"
/// would be green on every input that can reach emission and would test nothing
/// while reading as coverage. The frame says so in terms.
///
/// ⭐ **What survives the re-home is one-for-one consumption**, and that is what
/// this measures: the number of call edges emission resolves moves with the
/// program's own structure. A closure body is a distinct owner and therefore a
/// call edge; a bare ground value is one unit with nothing to call.
///
/// **MEASURED:** two compiles — a called closure resolves a nonzero call-edge
/// count, and a bare ground value resolves exactly zero.
/// **CLAIMED:** the call-edge population is projected from the planner's
/// validated `StaticBody` edges rather than derived a second time here.
/// **THE GAP:** ⛔ **this shows the count is not constant; it does not show the
/// count is EXACTLY the `StaticBody` edge population.** `SemanticOwner` and the
/// edge list are planner-private — deliberately, so the emitter cannot classify
/// owners itself — so no control in `lowering` can count the planner's edges
/// independently. ⇒ Exactness rests on `emittable_call_edges` filtering on
/// `EdgeKind::StaticBody` and failing closed otherwise, which is **argument, not
/// measurement**, and is recorded as such.
#[test]
fn the_resolved_call_edge_population_moves_with_the_program() {
    fn call_edges_for(expr: &RuntimeExpr) -> usize {
        ac11_compiles(expr).expect("fixture compiles");
        crate::cranelift_backend::lowering::units::b2f_last_call_edge_resolution()
    }

    // A called closure: its body is a distinct owner, so the planner records a
    // `StaticBody` edge into it and emission must resolve a call to that unit.
    let with_closure_body = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: Vec::new(),
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        }),
        args: Vec::new(),
    };
    // ⛔ THE POSITIVE CONTROL, and without it the row above is worthless: a
    // resolver that returned some fixed nonzero number for every program would
    // satisfy it. This is the same shape with nothing to call.
    let without_closure_body = RuntimeExpr::Value(RuntimeValue::Bool(true));

    let with = call_edges_for(&with_closure_body);
    let without = call_edges_for(&without_closure_body);

    assert!(
        with > 0,
        "D4 -- a program whose closure body is a distinct function unit must \
         resolve at least one cross-owner call edge; got {with}. Zero means \
         emission is not consuming the planner's StaticBody edges at all."
    );
    assert_eq!(
        without, 0,
        "D4 -- POSITIVE CONTROL: a bare ground value is a single unit with \
         nothing to call, so it must resolve zero call edges. A nonzero count \
         here means the population is not derived from the program."
    );
}


/// `a_seed_capture_borrows_from_artifact_static_storage_rather_than_folding` below.
#[test]
fn b2f_mints_one_defined_artifact_static_object_per_seed_environment_entry() {
    fn objects_emitted(env: &NativeSeedEnvironment) -> (usize, usize) {
        let module = new_jit_module().expect("jit module");
        compile_expr_into_module(
            module,
            "b2f_seed_material_population_probe",
            Linkage::Local,
            &RuntimeExpr::Value(RuntimeValue::Bool(true)),
            env,
            BTreeMap::new(),
            None,
            false,
            None,
            None,
            None,
        )
        .expect("compile");
        crate::cranelift_backend::lowering::seed_material::b2f_last_seed_material_emission()
    }

    let (empty_declared, empty_defined) = objects_emitted(&NativeSeedEnvironment::empty());
    let seeded = b2f_seed_capture_program("s", RuntimeGroundValue::Int(7i64.into()));
    let (seeded_declared, seeded_defined) = objects_emitted(&seeded);

    // ⛔ Every declared object is defined. A declaration without a definition
    // leaves an undefined symbol the borrow would resolve to, which is why the
    // recorder carries two numbers rather than one.
    assert_eq!(
        empty_declared, empty_defined,
        "D3 -- a declared artifact-static object was never defined (empty environment)"
    );
    assert_eq!(
        seeded_declared, seeded_defined,
        "D3 -- a declared artifact-static object was never defined (seeded environment)"
    );

    // ⭐ POSITIVE CONTROL / NON-VACUITY, in both directions. Without the first,
    // every assertion above is satisfied by minting nothing at all for any
    // input, forever. Without the second, they are satisfied by minting a fixed
    // object regardless of the environment.
    assert_eq!(
        empty_declared, 0,
        "D3 -- an empty seed environment has nothing to mint; measured {empty_declared}"
    );
    assert_eq!(
        seeded_declared, 1,
        "D3 -- NON-VACUITY: one environment entry must mint exactly one \
         artifact-static object. If this is 0 the population is not tracking the \
         environment and every count above is satisfied by minting nothing."
    );
}

/// **`D3` — the minted material is IN the artifact, verified against the module
/// rather than against our own bookkeeping.**
///
/// ⛔⛔ **This test exists because a counter cannot detect the deletion of the
/// call it counts, and that is measured rather than argued.** Removing the
/// `define_data` call while leaving the adjacent `defined += 1` reachable left
/// `b2f_last_seed_material_emission` reporting `(1, 1)` — both the counter and
/// the call are `seed_material`'s own code, so a mutation can remove one and
/// leave the other. ⚠ What caught that mutation instead was a **SIGSEGV** in the
/// test binary when the artifact ran against the undefined symbol: an undefined
/// data symbol is caught by neither the module nor the counter, only by the
/// hardware. ⭐ Loud, but undiagnostic — and a crash is not a control.
///
/// ⇒ **The fix is to ask a different party what it holds.**
/// `JITModule::get_finalized_data` reads the module's own finalized memory, so
/// the definition either happened or the comparison fails.
///
/// **MEASURED:** for every object this compile minted, the bytes the finalized
/// module holds at that `DataId` equal the byte image handed to `define_data`.
/// **CLAIMED:** the encoded seed material is really present in the artifact.
/// **THE GAP:** ⛔ the *expected* side is still this crate's encoder output, so
/// this cannot catch an encoding that is wrong in the same way on both sides.
/// That residual is covered by the encoder's own tag/offset/nesting tests, whose
/// expectations are written out independently of the encoder.
#[test]
fn minted_seed_material_is_present_in_the_finalized_artifact() {
    let env = b2f_seed_capture_program("s", RuntimeGroundValue::Int(0x0123_4567_89ab_cdefi64.into()));
    let module = new_jit_module().expect("jit module");
    let compiled = compile_expr_into_module(
        module,
        "b2f_seed_material_readback_probe",
        Linkage::Local,
        &RuntimeExpr::Value(RuntimeValue::Bool(true)),
        &env,
        BTreeMap::new(),
        None,
        false,
        None,
        None,
        None,
    )
    .expect("compile");

    let images = crate::cranelift_backend::lowering::seed_material::b2f_last_seed_material_images();
    // ⭐ POSITIVE CONTROL, first: without it every assertion below is vacuously
    // satisfied by an empty image list, for any mutation, forever.
    assert_eq!(
        images.len(),
        1,
        "D3 -- one environment entry must mint one image to read back"
    );

    let mut compiled = compiled;
    compiled
        .module
        .finalize_definitions()
        .expect("jit finalizes");
    for (id, expected) in images {
        let (pointer, length) = compiled.module.get_finalized_data(id);
        // SAFETY: `finalize_definitions` has run, so the module guarantees this
        // pointer/length names its own finalized data for `id`.
        let actual = unsafe { std::slice::from_raw_parts(pointer, length) };
        assert_eq!(
            actual, expected,
            "D3 -- the artifact does not hold the bytes that were defined for \
             this seed object. Either the definition never happened or something \
             overwrote it; in both cases a capture would borrow from storage \
             whose contents are not the seed value."
        );
    }
}

/// **`AC-12` — the emitted code OBEYS `BorrowedForActivation` +
/// `ArtifactStatic`, with a positive control.**
///
/// ⛔ **An assertion that reads the mode back out of `AbiCarrier::ownership` or
/// `storage_owner` discharges nothing** — both are `const fn`s over a closed
/// enum, so re-reading them measures the declaration with the declaration. The
/// observable difference between obeying those two modes and ignoring them is
/// whether the capture's value arrives by a **load from durable storage** or by
/// a constant folded into the instruction stream, and that is what this counts.
///
/// **MEASURED:** how many loads from artifact-static storage the emitter issued
/// while compiling a program with a seed capture, versus one without.
/// **CLAIMED:** a seed capture's scalar value is read out of artifact-static
/// material rather than folded in at compile time.
/// **THE GAP:** ⛔ a load that is emitted and then discarded downstream — if a
/// specialization ever substituted `Lowered`'s `known` field for the loaded
/// value in emitted code — would still be counted here. ⭐ **That residual is
/// closed by measurement, not by argument:** corrupting the minted payload byte
/// image (`push_word(out, (*small ^ 1) as u64)` in `seed_material::encode_into`)
/// reddens
/// `values::cranelift_runs_closure_seed_with_explicit_runtime_capture_environment`
/// and `artifact::api::tests::program_runner_preflights_metadata_before_backend_lowering`,
/// which are **runtime** observations. ⇒ The program's answer is a function of
/// the minted bytes. ⛔ That mutation is deliberately NOT committed as a test:
/// it needs a perturbation hook inside production, and a hook that can fold the
/// value instead is precisely the second authority `D3` removes.
///
/// ---
///
/// ⛔⛔ **THIS TEST IS `D3`'s SOLE MECHANICAL DEFENDER. MEASURED, NOT ESTIMATED.**
///
/// Replacing `self.artifact_static_payload(builder, symbol)?` with
/// `builder.ins().iconst(types::I64, *small)` in `lower_seed_capture` — i.e.
/// reverting `D3` wholesale and going back to compile-time folding — reddens
/// **exactly this test and nothing else, out of 496 others.**
///
/// ⇒ ⭐ **Weakening, relaxing or renaming this control leaves `D3` unpinned in a
/// single edit, and no other test in the crate would notice.** The seed material
/// would still be minted, still be read-only, still be counted by
/// `b2f_last_seed_material_emission`, and still be byte-compared by
/// `minted_seed_material_is_present_in_the_finalized_artifact` — because all
/// three of those observe the *material*, and none of them observes whether the
/// emitted code **reads** it. That distinction is the whole of `AC-12`, and it
/// lives here alone.
#[test]
fn a_seed_capture_borrows_from_artifact_static_storage_rather_than_folding() {
    fn loads_during(expr: &RuntimeExpr, env: &NativeSeedEnvironment) -> usize {
        let before = crate::cranelift_backend::lowering::seed_material::b2f_artifact_static_loads();
        let module = new_jit_module().expect("jit module");
        compile_expr_into_module(
            module,
            "b2f_artifact_static_borrow_probe",
            Linkage::Local,
            expr,
            env,
            BTreeMap::new(),
            None,
            false,
            None,
            None,
            None,
        )
        .expect("compile");
        // ⚠ A difference of two readings, because the counter is monotone across
        // the process and other tests on this thread contribute to it.
        crate::cranelift_backend::lowering::seed_material::b2f_artifact_static_loads() - before
    }

    // The two fixtures differ in exactly one thing: whether the program performs
    // a seed capture. Both compile the same shape and both reach emission.
    let no_capture = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::Closure {
            captures: Vec::new(),
            params: Vec::new(),
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        }),
        args: Vec::new(),
    };
    let with_capture = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::Closure {
            captures: vec!["s".to_string()],
            params: Vec::new(),
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        }),
        args: Vec::new(),
    };
    let env = b2f_seed_capture_program("s", RuntimeGroundValue::Int(7i64.into()));

    let without = loads_during(&no_capture, &env);
    let with = loads_during(&with_capture, &env);

    // ⭐ POSITIVE CONTROL first, because the interesting assertion is the
    // negative one and a negative check passes for any reason -- including a
    // counter that is never incremented at all.
    assert!(
        with >= 1,
        "AC-12 -- a seed capture must READ its value out of artifact-static \
         storage. Zero loads means the value was folded into the instruction \
         stream, which is `OwnedByFrame` behaviour on a slot the ABI declares \
         `BorrowedForActivation` from `ArtifactStatic`."
    );
    assert_eq!(
        without, 0,
        "AC-12 -- NON-VACUITY: a program with no seed capture must issue no \
         artifact-static load. If this is non-zero the counter is measuring \
         something other than the capture path and the assertion above is \
         satisfied for the wrong reason; measured {without}"
    );
}

/// **`D4` — substituting only the emitted `FuncRef` reds the `4b` equality.**
///
/// The planner identity, the declared call contract, the header, slots,
/// offsets, inputs and owner are all retained; the call is still emitted. The
/// single thing that moves is the callee identity, so the finished-CLIF oracle
/// rejects because the decoded `FuncId` disagrees with `identity.target()` —
/// ⛔ not because "a target changed", which is what an oracle reading the map
/// it mutates would have reported.
///
/// **Promise class: durable invariant.** It pins the *relation* between the
/// planner-issued target and the emitted callee, which every intended
/// extension of this seam preserves.
#[test]
fn d4_substituting_the_emitted_funcref_reds_the_emission_equality() {
    assert_emission_mutation_reds(
        ContinuationEmissionMutation::SubstituteEmittedFuncRef,
        "disagrees with the planner-issued continuation target",
    );
}

#[cfg(test)]
fn d7_ctor(name: &str) -> RuntimeExpr {
    RuntimeExpr::Construct {
        constructor: name.to_string(),
        args: Vec::new(),
    }
}

#[cfg(test)]
fn d7_run(expr: &RuntimeExpr) -> String {
    match crate::cranelift_backend::artifact::compile_expr_for_lowering_tests(
        expr,
        &NativeSeedEnvironment::empty(),
    ) {
        Err(error) => format!("COMPILE-ERR {error:?}"),
        Ok(compiled) => match compiled.run(None) {
            Err(error) => format!("RUN-ERR {error:?}"),
            Ok(observation) => format!("OK {observation:?}"),
        },
    }
}

/// **`RT-CONTSPEC-WITNESS` `D7` — the two same-shaped targets in one lawful,
/// executing callable population.**
///
/// This is the fixture precondition the frame assigns to this seam, and it is
/// the thing [[RT-CONTSPEC-ACTIVATE]] lacked: its generated function held
/// exactly one call target, so a same-shaped redirect refused *before* the call
/// seam and was never a control at all.
///
/// ⭐ **The second target is bound in the ENCLOSING scope, not as a second
/// field of the aggregate, and that is forced rather than stylistic.** Two
/// closures inside one `Construct` refuse with *"a closure cannot cross the
/// boundary: it is runtime-local and live-domain only, and it has no durable
/// lane"* — measured here at both `recursive_positions` configurations, `[0]`
/// and `[0, 1]`. That is the same ordinary-`Closure` wall that stopped seam 2's
/// six shapes. Binding the sibling in the enclosing scope keeps one closure per
/// aggregate and lowers lawfully.
///
/// Both targets are same-shaped under `RT-WORKER-BIND`'s definition: declared
/// arity 1, capture count 0. They differ only in the constructor their body
/// returns, so **which one runs is observable in the result** — `Alpha` for the
/// exact target, `Beta` for the other.
#[cfg(test)]
fn d7_two_same_shaped_targets_in_one_population() -> RuntimeExpr {
    // One closure in the aggregate (the lawful ACTIVATE shape), and a SECOND
    // same-shaped closure bound and called in the enclosing scope, so the two
    // same-shaped targets never share an aggregate.
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["unit".to_string()],
            body: Box::new(d7_ctor("ctor:fixture::d7::Beta")),
        }),
        body: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::d7::Node".to_string(),
                args: vec![RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["unit".to_string()],
                    body: Box::new(d7_ctor("ctor:fixture::d7::Alpha")),
                }],
            }),
            cases: vec![
                crate::RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::d7::Node".to_string(),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: RuntimeExpr::Call {
                        callee: Box::new(RuntimeExpr::Var(0)),
                        args: vec![d7_ctor("ctor:prelude::Unit::MkUnit")],
                    },
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::d7::Alpha".to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: d7_ctor("ctor:fixture::d7::Alpha"),
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::d7::Beta".to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: d7_ctor("ctor:fixture::d7::Beta"),
                },
            ],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "d7 variant c is total".to_string(),
            },
        }),
    }
}

#[cfg(test)]
const D7_ALPHA: &str =
    "OK (Returned(Constructor { constructor: \"ctor:fixture::d7::Alpha\", args: [] }), Some(517))";
#[cfg(test)]
const D7_BETA: &str =
    "OK (Returned(Constructor { constructor: \"ctor:fixture::d7::Beta\", args: [] }), Some(517))";

/// **`RT-CONTSPEC-WITNESS` `AC-9` — a wrong same-shaped target changes an
/// EXECUTED result.** Architect ruling 2026-08-08.
///
/// Both runs **execute**. The mutation binds a distinct same-shaped body under
/// the exact declared continuation `FuncId`, preserving the causal token,
/// specialization id, declared `FuncId`, header, slots, offsets, inputs, owner
/// and the emitted call — so `verify_emitted_continuation_calls` stays enabled
/// and green naturally, and **the observed answer moves `Alpha` to `Beta`.**
///
/// ⭐ **Why this is `AC-9` and the call-seam redirect is not.** The finished-CLIF
/// equality gate proves planner-identity to emitted-callee routing. It cannot
/// see which *body* is bound to a declared function, which is
/// `RT-CONTSPEC-ACTIVATE`'s own stated residual. This asserts the property that
/// residual leaves open: **the declared target's behaviour depends on the body
/// bound to it, so same-shaped bodies cannot alias unnoticed.**
///
/// ⛔ **The application counter is load-bearing, not bookkeeping.** A mutation
/// that applied to nothing would leave the program returning `Alpha`, and an
/// unchanged result would then read as "the substitution had no effect" when it
/// means "no substitution happened" — opposite conclusions from identical
/// evidence. Asserting `0` before and `1` after is what separates them.
///
/// The trailing re-run is the scope check: a mutation that leaked would redden
/// every later test in this thread instead of this one.
///
/// **Promise class: durable invariant.** The subject is that a declared
/// continuation executes the body bound to it. Every intended extension that
/// keeps the declaration-to-body binding honest keeps this green.
#[test]
fn d7_binding_a_distinct_same_shaped_body_changes_the_executed_result() {
    use crate::cranelift_backend::lowering::{
        d7_definition_binding_substitutions, reset_d7_definition_binding_substitutions,
    };
    let witness = d7_two_same_shaped_targets_in_one_population();

    reset_d7_definition_binding_substitutions();
    let exact = d7_run(&witness);
    assert_eq!(
        exact, D7_ALPHA,
        "positive control: the exact assembly must EXECUTE to the exact target's answer"
    );
    assert_eq!(
        d7_definition_binding_substitutions(),
        0,
        "the exact run must apply no substitution; a control that fires unasked is measuring \
         something other than the mutation"
    );

    let mutated = with_continuation_emission_mutation(
        ContinuationEmissionMutation::SubstituteContinuationBodyDefinition,
        || d7_run(&witness),
    );
    assert_eq!(
        d7_definition_binding_substitutions(),
        1,
        "the mutation must actually have bound a distinct body; without this a no-op passes \
         by returning the exact answer"
    );
    assert_eq!(
        mutated, D7_BETA,
        "AC-9: binding the other same-shaped body under the same declared function must change \
         the EXECUTED result. A compile-time refusal here would be the wrong oracle"
    );
    assert_ne!(exact, mutated, "the two runs must be distinguishable");

    reset_d7_definition_binding_substitutions();
    assert_eq!(
        d7_run(&witness),
        D7_ALPHA,
        "the mutation must not leak past its scope"
    );
}

/// **`D7` part one — the fixture precondition is discharged: two same-shaped
/// targets in one lawful population that actually EXECUTES.**
///
/// The frame assigns this precondition to this seam, and it is the half of
/// `AC-9` that was genuinely missing. The assembly runs and returns the exact
/// target's constructor.
///
/// **Promise class: durable invariant.** The subject is that this population is
/// lawful and executable, and that the executed answer names which of the two
/// same-shaped targets ran. Any extension preserving the declared-call contract
/// keeps it green.
#[test]
fn d7_the_two_same_shaped_target_population_executes() {
    assert_eq!(
        d7_run(&d7_two_same_shaped_targets_in_one_population()),
        "OK (Returned(Constructor { constructor: \"ctor:fixture::d7::Alpha\", args: [] }), Some(517))",
        "the two-target population must execute and return the EXACT target's \
         constructor; if this stops executing, D7's precondition has regressed \
         and the redirect below is measuring nothing"
    );
}

/// **`D7` part two — the same-shaped redirect now REACHES the call seam.**
///
/// ⭐ **This is the state change this seam produced, and it is not `AC-9`.**
/// In [[RT-CONTSPEC-ACTIVATE]] the same-shaped redirect refused with *"found no
/// distinct same-shaped call target"* **before** reaching the call, so it
/// proved nothing about targets; that is why the Architect replaced it with
/// [`ContinuationEmissionMutation::SubstituteEmittedFuncRef`]. With a real
/// two-target population it now resolves a distinct same-shaped target and the
/// call seam is entered.
///
/// ⛔ **REACHABILITY ONLY. This row observes no executed result and is not a
/// behavioural oracle.** The redirect is caught by the finished-CLIF
/// emission-equality gate -- the emitted callee disagrees with the declared
/// continuation target -- so **the mutated arm never executes**. Read as a
/// behavioural claim it would be [[RT-CONTSPEC-ACTIVATE]]'s `AC-2` restated,
/// which observes the mutation changing the field it mutates.
///
/// ⇒ **`AC-9` is discharged, and not here.** Its executed witness is
/// `d7_binding_a_distinct_same_shaped_body_changes_the_executed_result`, which
/// perturbs the declaration-to-body binding this gate cannot see. **These two
/// rows are complementary and neither substitutes for the other:** this one
/// says the seam is reached, that one says the bound body determines the
/// answer.
///
/// **Promise class: transition sentinel.** It reds when the emission-equality
/// gate stops being what catches a same-shaped call-site redirect. That is a
/// real event to be told about -- it would mean the reachability claim above
/// needs re-establishing, and that `AC-9`'s witness is no longer isolated from
/// the static gate. **It retires when the call-site redirect stops being a
/// meaningful perturbation of this seam**, not when `AC-9` is discharged; `AC-9`
/// already is.
#[test]
fn d7_the_same_shaped_redirect_reaches_the_call_seam_and_is_caught_by_the_emission_oracle() {
    let witness = d7_two_same_shaped_targets_in_one_population();
    let rendered = with_continuation_emission_mutation(
        ContinuationEmissionMutation::RedirectToDistinctSameShapedTarget,
        || d7_run(&witness),
    );
    assert!(
        !rendered.contains("found no DISTINCT target"),
        "the redirect must REACH the call seam, not refuse before it for want of \
         a second same-shaped target -- that pre-call refusal is what made this \
         control vacuous in ACTIVATE. got: {rendered}"
    );
    assert!(
        rendered.contains("disagrees with the planner-issued continuation target"),
        "the redirect must be caught by the emission-equality oracle, naming the \
         target disagreement. got: {rendered}"
    );
}

/// **`4b` closure — an emitted call that is not recorded is still caught.**
///
/// Without this the gate would be complete only over the set it built itself:
/// an unrecorded emission would be invisible and the records would look
/// exhaustive because nothing disagreed with them.
#[test]
fn d4_an_unrecorded_continuation_emission_reds_the_clif_sweep() {
    assert_emission_mutation_reds(
        ContinuationEmissionMutation::SuppressEmissionRecord,
        "are not the ones recorded against planned causal tokens",
    );
}

/// **`4b` closeout — a per-function verification that never accumulates is
/// caught by whole-pass set equality.**
///
/// ⭐ This is the control that separates "each function checked its own
/// emissions" from "every planned token was emitted somewhere". The per-
/// function gate stays green here; only the closeout notices.
///
/// ⚠ **`D8k` moved which clause catches it, and the move is the point.** The
/// closeout no longer asserts `emitted == planned`; it asserts the disjoint
/// partition `call obligations = direct-emitted ⊎ composed-consumed`. A token that was
/// never accumulated is now caught as one that is in NEITHER half, which is a
/// strictly more informative reading of the same defect — the message names
/// both populations, so a reader can tell "nothing was emitted" from "it was
/// discharged the other way". ⛔ The control is not weakened: it still asserts
/// a set relation and still names the clause it must reach.
#[test]
fn d4_failing_to_accumulate_emissions_reds_the_closeout_set_equality() {
    assert_emission_mutation_reds(
        ContinuationEmissionMutation::SuppressEmissionAccumulation,
        "were neither directly emitted nor compositionally consumed",
    );
}

/// **`D3` affine seam — claiming one causal token twice reds.**
#[test]
fn d4_claiming_the_same_causal_token_twice_reds_the_ledger() {
    assert_emission_mutation_reds(
        ContinuationEmissionMutation::ClaimTokenTwice,
        "claimed twice",
    );
}

/// **`D3` owner seam — claiming under a unit that does not own the token
/// reds.**
///
/// ⚠ Under today's structure a wrong owner surfaces as a **leftover** claim at
/// `close()` rather than at `claim_exact`, because selection is already by the
/// token's own owner. The assertion below is deliberately on the text the
/// production path actually produces, not on the one the seam's name suggests.
///
/// ⭐ **`D5a`: the owner this compares is now the EMISSION owner**, in the
/// generalized domain, not the raw source-occurrence provenance owner. The
/// substring moved with the diagnostic for that reason — the control still
/// measures the same seam, and the reason it names is now the sharper one.
#[test]
fn d4_claiming_under_a_unit_that_does_not_own_the_token_reds() {
    assert_emission_mutation_reds(
        ContinuationEmissionMutation::ClaimUnderWrongOwner,
        "is not its emission owner",
    );
}

/// **`AC-1b` — the executable lowering witness population, stated as its own
/// population and never borrowed from the planner census.**
///
/// ⛔ `AC-1a`'s nested planner census is `2/2/2/2` and is asserted by
/// `contspec_planner_closes_ordered_keys_units_and_causal_edges_dormantly`.
/// This is a **different fixture**. Reporting either count as the other is the
/// conflation the Architect corrected at `evt_6bf2mmehjzy3k`; a one-token
/// emission population is sufficient and non-vacuous for a universal
/// per-identity routing property, and must not be inflated to two.
///
/// The equality itself is enforced in production by
/// `ContinuationClaimLedger::close`, over sets rather than lengths. What this
/// test adds is that the witness **compiles**, so that closeout was actually
/// executed rather than skipped for want of a continuation.
#[test]
fn ac1b_the_executable_lowering_witness_closes_its_one_token_population() {
    assert!(
        ac11_compiles(&contspec_emission_witness()).is_ok(),
        "the one-token emission witness must compile, which is what runs the \
         planned/resolved/declared/emitted set equality at closeout"
    );
}

/// **`RT-RECURSOR-TRANSPORT` `D1` — a CLOSED active computational recursor.**
///
/// The classification fixtures above scrutinise `Var(0)`, which is free at the
/// root: they are perfectly good for asking the classifier a question and
/// cannot be compiled or run. `D1` needs an **executed** outcome, so this is
/// the same shape closed over a real constructor.
#[cfg(test)]
fn rt_closed_active_recursor() -> RuntimeExpr {
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::rt::Node".to_string(),
            args: vec![RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["unit".to_string()],
                body: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::rt::Leaf".to_string(),
                    args: Vec::new(),
                }),
            }],
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::rt::Node".to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(0)),
                    args: vec![RuntimeExpr::Construct {
                        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                        args: Vec::new(),
                    }],
                },
            },
            crate::RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::rt::Leaf".to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::Construct {
                    constructor: "ctor:fixture::rt::Leaf".to_string(),
                    args: Vec::new(),
                },
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "rt closed active recursor".to_string(),
        },
    }
}

/// **The exact `D1` position-A witness** — an ordinary `Match` consuming an
/// active recursor, built as a **closed, executable expression**.
///
/// ⚠ "Closed" describes this **expression**: it scrutinises a real constructor
/// rather than a free `Var`, so it can be compiled and run. It says nothing
/// about the `MatchScrutineeRecursor` class. The bare word stood alone here
/// until 2026-08-08, next to a withdrawn claim that *"position A closes"*, where
/// it read as the class closing.
#[cfg(test)]
fn rt_match_scrutinee_recursor_executable() -> RuntimeExpr {
    RuntimeExpr::Match {
        scrutinee: Box::new(rt_closed_active_recursor()),
        cases: vec![crate::RuntimeMatchCase {
            constructor: "ctor:fixture::rt::Leaf".to_string(),
            binders: 0,
            body: RuntimeExpr::Value(RuntimeValue::Int(7.into())),
        }],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "rt match scrutinee recursor".to_string(),
        },
    }
}

/// Pulls `key=` through the next space out of one trace entry.
#[cfg(test)]
fn rt_trace_field<'a>(entry: &'a str, key: &str) -> Option<&'a str> {
    let rest = entry.split_once(key)?.1;
    Some(rest.split(' ').next().unwrap_or(rest))
}


/// Runs `expr` and returns the **decoded observation** only.
///
/// ⛔ The raw native result token is deliberately dropped. It is a lane-internal
/// pre-decode value -- the two lanes encode the same result differently, and
/// `Leaf` arrives as token `0` on the retained lane and `517` on the
/// functionized one. Comparing tokens across lanes would manufacture a
/// difference where the semantics agree; the decoded `RuntimeObservation` is
/// what the lanes are supposed to agree on, so that is what this returns.
#[cfg(test)]
fn rt_run(expr: &RuntimeExpr) -> String {
    match crate::cranelift_backend::artifact::compile_expr_for_lowering_tests(
        expr,
        &NativeSeedEnvironment::empty(),
    ) {
        Err(error) => format!("COMPILE-ERR {error:?}"),
        Ok(compiled) => match compiled.run(None) {
            Err(error) => format!("RUN-ERR {error:?}"),
            Ok((observation, _token)) => format!("OK {observation:?}"),
        },
    }
}

/// **`RT-MATCH-SCRUTINEE-DISPOSITION` `AC-6` — the exact executable
/// intersection witness now selects `FunctionizedUnits` unaided.**
///
/// `RT-RECURSOR-TRANSPORT` established this witness's functionized result with
/// a selector exclusion. `D3-narrow` removes the need for that exclusion
/// because the ordinary producer route already accepts this scrutinee. The
/// empty residual set and exact executed result are the transition sentinel.
///
/// **Promise class: durable invariant.** This intersection witness remains on
/// the ordinary route and executes to the same decoded result.
#[test]
fn rt_d2_trace_shows_the_marker_propagated_and_never_reaching_the_composed_consumer() {
    use crate::cranelift_backend::lowering::core::{
        reset_rt_d2_backedge_propagations, rt_d2_backedge_propagations,
    };
    use crate::cranelift_backend::lowering::{reset_d5a_trace, take_d5a_trace};
    let witness = rt_match_scrutinee_recursor_executable();

    reset_d5a_trace();
    reset_rt_d2_backedge_propagations();
    let functionized = rt_run(&witness);
    let trace = take_d5a_trace();

    assert_eq!(functionized, "OK Returned(Int(Small(7)))", "the repaired run must execute");
    assert_eq!(
        rt_d2_backedge_propagations(),
        1,
        "exactly one active-resume backedge propagation"
    );

    let (a, a_entry) = trace
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.contains("RT-D2 A INSTALLED"))
        .unwrap_or_else(|| panic!("no install step: {trace:#?}"));
    let continuation = rt_trace_field(a_entry, "continuation_origin=")
        .unwrap_or_else(|| panic!("no continuation origin: {a_entry}"));
    let position = rt_trace_field(a_entry, "recursive_position=")
        .unwrap_or_else(|| panic!("no recursive position: {a_entry}"));
    let body = rt_trace_field(a_entry, "body=Some(")
        .map(|body| body.trim_end_matches(')'))
        .unwrap_or_else(|| panic!("no body: {a_entry}"));
    assert!(
        a_entry.contains("owner=Some(Predeclared("),
        "the emission owner is what makes an absent generated context expected: {a_entry}"
    );

    let step = |from: usize, needle: &str, must: &[&str]| -> usize {
        trace
            .iter()
            .enumerate()
            .skip(from + 1)
            .find(|(_, entry)| entry.contains(needle) && must.iter().all(|f| entry.contains(f)))
            .map(|(index, _)| index)
            .unwrap_or_else(|| panic!("no {needle} after {from} with {must:?}: {trace:#?}"))
    };

    let returned = step(
        a,
        "RT-D2 B RETURNED",
        &[
            &format!("body={body}"),
            &format!("continuation_origin={continuation}"),
            &format!("recursive_position={position}"),
            "phase=Carried",
        ],
    );
    let applied = step(
        returned,
        "RT-D2 C APPLY-RECURSOR-SELECTION",
        &[&format!("layer_origin={continuation}")],
    );
    let resumed = step(
        applied,
        "RT-D2 D COMPUTATIONAL-MATCH-SCRUTINEE",
        &[&format!("match_origin={continuation}"), "input[phase=Carried"],
    );
    assert!(a < returned && returned < applied && applied < resumed, "the chain must be ordered");

    assert!(
        !trace
            .iter()
            .any(|entry| entry.contains("RT-D2 E COMPOSED-CONSUMER")
                && entry.contains("actual_kind=RecursiveBackedge")),
        "THE ABSENCE IS THE REPAIR: the protocol marker must never reach the composed consumer. \
         trace: {trace:#?}"
    );
}

/// **`AC-6.1` — the canonical explicit-seed-env positive, through the port.**
///
/// The reachability count is what makes this a control rather than a
/// restatement that the seed still works: `closure-capture-application` also
/// returned `7` on the retired lane, so the observation alone never told the
/// two paths apart.
///
/// `D3` retired the variant, so this program has no route back to that lane and
/// the count can no longer be discriminated here by disarming anything. `AC-7`
/// carries that instead, reaching this same arm with no witness in the tree.
///
/// **What the two assertions prove TOGETHER, and neither alone.** The count is
/// taken before `call_declared_unit`, which can still refuse, so a count of 1
/// on its own would not establish that a call instruction exists. Paired with a
/// successful run that returns the seed's declared observation, it does: the
/// program went through this arm AND the typed call path completed.
///
/// **Promise class: durable invariant.** The ported call must produce the seed's
/// declared observation. `D3` removes the witness, not the property.
#[test]
fn d2_ac6_1_the_canonical_seed_runs_through_the_ported_callee_unit() {
    let example = nc5_seed_examples()
        .into_iter()
        .find(|example| example.name == "closure-capture-application")
        .expect("seed exists");

    let (outcome, ports) = d2_run_ported(&example, &NativeSeedEnvironment::nc5_seed());
    let report = outcome.expect("the ported callee unit compiles and runs");
    assert_eq!(
        report.observation, example.observation,
        "AC-6.1: the ported call must produce the seed's declared observation"
    );
    assert!(report.verifier_passed);
    assert_eq!(
        ports, 1,
        "AC-6.1: the ported arm must reach its handoff exactly once. With the successful run \
         asserted above, this pair is the evidence for a completed typed unit call"
    );

    // `RT-SEED-CALL-PORT` `D3` INVERTED this half, and the inversion is the
    // activation. At `D2` it read: witness disarmed, the selector retains the
    // lane, the port must NOT run -- and that was the discriminating proof the
    // count measured the port rather than compilations.
    //
    // With `SeedClosureCall` retired there is no route back to the retained
    // lane for this program, so the old assertion is not merely stale, it is
    // unstatable. The discriminator moves to `AC-7` below, which reaches this
    // same arm with no witness in the tree at all.
}

/// **`AC-6.2` — the missing-capture loud refusal, raised by the port itself.**
///
/// The refusal comes from `D2`'s arm resolving a seed symbol through
/// `lower_seed_capture`, not from the retained lane -- and since `D3` retired
/// the variant there is no longer any route back to that lane for this program,
/// so no witness is involved. The port
/// count must be **zero**: the arm counts at its handoff point, after arity and
/// every capture have resolved, so a capture refusal lands strictly before the
/// count. That zero is evidence of refusal BEFORE the handoff -- a distinct and
/// weaker claim than "nothing was emitted", which this counter cannot make.
///
/// **Promise class: durable invariant.** An unresolvable seed capture must fail
/// closed and loudly, whichever lane reaches it.
#[test]
fn d2_ac6_2_a_missing_seed_capture_refuses_loudly_before_the_ported_handoff() {
    let example = nc5_seed_examples()
        .into_iter()
        .find(|example| example.name == "closure-capture-application")
        .expect("seed exists");

    let (outcome, ports) = d2_run_ported(&example, &NativeSeedEnvironment::empty());
    let error = outcome.expect_err("a missing seed capture must refuse");
    assert!(
        matches!(
            error,
            CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "Closure",
                ..
            })
        ),
        "AC-6.2: the refusal must name the Closure construct, not a generic backend failure: \
         {error:?}"
    );
    assert_eq!(
        ports, 0,
        "AC-6.2: the refusal must precede the handoff -- a nonzero count means this arm passed \
         inputs to the typed call path for a closure whose captures never resolved"
    );
}

/// **`AC-6.3` — the ORDER-SENSITIVE `Parameter ++ Capture` control.**
///
/// This is the one `AC-6` calls not optional, and the reason is a live blindness
/// rather than a hypothesis: the canonical seed computes `5 + 2 = 7`, and
/// because addition is commutative it returns `7` under either input order. It
/// cannot see the defect `D2` point 3 exists to prevent.
///
/// `sub_int` with argument `5` and capture `2` is the smallest witness that can.
/// The ruled order yields `3`; a `Capture ++ Parameter` swap yields `-3`. The
/// assertion is on the exact value, so a swap does not merely change a count --
/// it names a different number.
///
/// **Promise class: durable invariant.** It pins the ABI input order of a
/// ported seed-callee unit, which is a contract of the transport rather than a
/// property of this fixture.
#[test]
fn d2_ac6_3_the_ported_unit_receives_parameters_before_captures() {
    let example = d2_order_sensitive_example();

    let (outcome, ports) = d2_run_ported(&example, &NativeSeedEnvironment::nc5_seed());
    let report = outcome.expect("the order-sensitive fixture compiles and runs");
    assert_eq!(
        ports, 1,
        "AC-6.3: the fixture must reach the ported arm's handoff, or the value below is not \
         about the ported transport at all"
    );
    assert_eq!(
        report.observation,
        RuntimeObservation::Returned(RuntimeGroundValue::Int((3).into())),
        "AC-6.3: 5 - 2 = 3 under the ruled Parameter ++ Capture order. A -3 here is the exact \
         Capture ++ Parameter swap this control exists to catch, and no commutative operator \
         could have distinguished them"
    );
    assert!(report.verifier_passed);
}












// ── Control 1: the exact fixture, ACTIVATED ───────────────────────────────
//
// ⭐⭐ **This row is where `D6` is visible as one fact.** It used to read: the
// fixture carried exactly the retired residual and production selected the
// monolithic route. Both halves have inverted, and the inversion is the node.
//
// ⛔ It asserts the **empty set** and not merely the retired variant's absence.
// The `D5` fixture is a closure-seed transparent declaration containing a
// checked recursive declaration call — precisely the shape the campaign's
// surviving variants also key on — so "no residual fires" is a real claim about
// every classifier on a nontrivial program, not a restatement of the deletion.

#[test]
fn d5_c2_the_witness_reaches_the_seam_and_emits_the_exact_planner_target() {
    let entry = d5_entry();
    let declaration = d5_declaration();
    let carrier = d5_frame_carrier();
    let declarations = BTreeMap::from([
        (D5_DECLARATION, &declaration),
        (D5_FRAME_CARRIER, &carrier),
    ]);
    let unreferenced = RuntimeExpr::Value(RuntimeValue::Int((1).into()));
    let thunk = RuntimeDeclaration {
        symbol: "decl:fixture::d5::thunk".to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::Value(RuntimeValue::Int((3).into())),
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };
    let thunk_entry = RuntimeExpr::DeclarationRef {
        symbol: thunk.symbol.clone(),
    };
    let plain = RuntimeDeclaration {
        symbol: D5_DECLARATION.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["n".to_string()],
                body: Box::new(RuntimeExpr::Var(0)),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };
    // The three discriminating fixtures, all compiling after `D2a`.
    for (label, entry, decls) in [
        (
            "closure-seed, referenced",
            &entry,
            BTreeMap::from([(D5_DECLARATION, &plain)]),
        ),
        (
            "closure-seed, unreferenced",
            &unreferenced,
            BTreeMap::from([(D5_DECLARATION, &plain)]),
        ),
        (
            "non-closure thunk",
            &thunk_entry,
            BTreeMap::from([(thunk.symbol.as_str(), &thunk)]),
        ),
    ] {
        compile_expr_into_module(
            new_jit_module().expect("JIT module"),
            "d5_c2_population",
            Linkage::Local,
            entry,
            &NativeSeedEnvironment::empty(),
            decls,
            None,
            false,
            None,
            None,
            None,
        )
        .map(|_| ())
        .unwrap_or_else(|error| {
            panic!("D5 control 2: the {label} fixture must compile after D2a: {error:?}")
        });
    }

    // The checked self-call, end to end.
    let (outcome, emitted) = d5_compile(d5_plan(), None);
    outcome.unwrap_or_else(|error| {
        panic!("D5 control 2: the exact checked self-call must compile: {error}")
    });

    // The independent planner side: which unit does the plan say a
    // declaration reference resolves to? ⛔ Derived from the plan's own
    // `CallableDeclaration` descriptors, never from the emitter's map.
    let declaration_units = d5_planned_callable_declaration_origins(&entry, &declarations);
    assert!(
        !declaration_units.is_empty(),
        "D5 control 2: the plan must own at least one CallableDeclaration \
         unit, or the comparison below has nothing to compare against"
    );
    assert!(
        !emitted.is_empty(),
        "D5 control 2: reaching the seam is the point — an empty emission \
         record means the compile succeeded without ever calling a \
         declaration-owned unit, and every negative would then be green for \
         the wrong reason"
    );
    for (reference, target, _func) in &emitted {
        assert!(
            declaration_units.contains(target),
            "D5 control 2: the call emitted for reference {reference:?} went \
             to {target:?}, which is not one of the planner-resolved \
             declaration-owned callable units {declaration_units:?}"
        );
    };
}

/// Every origin the plan classifies as a declaration-owned callable unit.
///
/// ⛔ Read from the static transition plan's own ABI descriptors — the
/// independent side of control 2's emitted-target comparison.
#[cfg(test)]
fn d5_planned_callable_declaration_origins(
    entry: &RuntimeExpr,
    declarations: &BTreeMap<&str, &RuntimeDeclaration>,
) -> Vec<StaticOriginId> {
    let plan = plan_static_transition_graph_with_symbols(
        entry,
        declarations,
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the D5 fixture plans");
    plan.emittable_units()
        .expect("the plan exposes its units")
        .into_iter()
        .filter(|unit| {
            matches!(
                unit.definition(),
                AbiUnitDefinition::CallableDeclaration { .. }
            )
        })
        .map(|unit| unit.body_occurrence())
        .collect()
}




// ── The MUTUAL same-SCC fixture ───────────────────────────────────────────
//
// ⭐⭐ **A self-call fixture cannot discriminate `recursion_group`, and this is
// why the ruling forbids a self-call-only shortcut.** D5's same-SCC check asks
// whether the callee is itself a recursive member of the caller's group. With
// one template, renaming its `recursion_group` moves the template AND its own
// witness together, so the check still passes and the mutation is inert. Two
// templates in one group make the witness a DIFFERENT template, and the rename
// then separates them.
//
// ⚠ Two declarations, two call templates (A→B and B→A), two frames in two
// segments. `mutual_a` and `mutual_b` differ in arity so a control cannot
// confuse them, and neither is the callee of its own template.

#[cfg(test)]
const D5_MUTUAL_A: &str = "decl:fixture::d5::mutual_a";
#[cfg(test)]
const D5_MUTUAL_B: &str = "decl:fixture::d5::mutual_b";
#[cfg(test)]
const D5_MUTUAL_A_TEMPLATE: u64 = 910;
#[cfg(test)]
const D5_MUTUAL_B_TEMPLATE: u64 = 911;
#[cfg(test)]
const D5_MUTUAL_A_FRAME: u64 = 92;
#[cfg(test)]
const D5_MUTUAL_B_FRAME: u64 = 93;

/// One member of the mutual pair: a lexical closure whose body is a checked
/// call to the *other* member.
#[cfg(test)]
fn d5_mutual_declaration(symbol: &str, callee: &str, template: u64) -> RuntimeDeclaration {
    RuntimeDeclaration {
        symbol: symbol.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: vec![RuntimeExpr::Value(RuntimeValue::Int((7).into()))],
                params: vec!["n".to_string()],
                body: Box::new(RuntimeExpr::CheckedRecursiveInvocation {
                    call_template_id: template,
                    checked_occurrence_path: vec![5],
                    body: Box::new(RuntimeExpr::Call {
                        callee: Box::new(RuntimeExpr::DeclarationRef {
                            symbol: callee.to_string(),
                        }),
                        args: vec![RuntimeExpr::Var(0)],
                    }),
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

/// The carrier for the pair's two frame markers.
///
/// ⚠ Both live here, nested, for the same reason the self-call fixture's single
/// marker does: the transport validator requires one Runtime frame marker per
/// planned frame, and putting a `ComputationalMatch` inside the declarations
/// under test would drag the computational-recursor lane into a fixture about
/// declaration calls. The plan's `frame.declaration` is what binds a frame to
/// its declaration, not where the marker physically sits.
#[cfg(test)]
fn d5_mutual_frame_carrier() -> RuntimeDeclaration {
    let inner = RuntimeExpr::CheckedSubcontinuationFrame {
        frame_id: D5_MUTUAL_B_FRAME,
        body: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::D5::Only".to_string(),
                args: vec![RuntimeExpr::Value(RuntimeValue::Int((0).into()))],
            }),
            cases: d5_cases(),
            default: d5_default(),
        }),
    };
    RuntimeDeclaration {
        symbol: D5_FRAME_CARRIER.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::CheckedSubcontinuationFrame {
                frame_id: D5_MUTUAL_A_FRAME,
                body: Box::new(RuntimeExpr::ComputationalMatch {
                    scrutinee: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::D5::Only".to_string(),
                        args: vec![RuntimeExpr::Value(RuntimeValue::Int((0).into()))],
                    }),
                    cases: vec![crate::RuntimeComputationalMatchCase {
                        constructor: "ctor:fixture::D5::Only".to_string(),
                        argument_binders: 1,
                        recursive_positions: Vec::new(),
                        body: inner,
                    }],
                    default: d5_default(),
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

#[cfg(test)]
fn d5_mutual_frame(frame_id: u64, declaration: &str, segment: u64, cases_of_outer: bool)
-> crate::OrientedSubcontinuationFramePlanV1 {
    let (cases, default) = if cases_of_outer {
        (
            vec![crate::RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::D5::Only".to_string(),
                argument_binders: 1,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::CheckedSubcontinuationFrame {
                    frame_id: D5_MUTUAL_B_FRAME,
                    body: Box::new(RuntimeExpr::ComputationalMatch {
                        scrutinee: Box::new(RuntimeExpr::Construct {
                            constructor: "ctor:fixture::D5::Only".to_string(),
                            args: vec![RuntimeExpr::Value(RuntimeValue::Int((0).into()))],
                        }),
                        cases: d5_cases(),
                        default: d5_default(),
                    }),
                },
            }],
            d5_default(),
        )
    } else {
        (d5_cases(), d5_default())
    };
    let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
        frame_id,
        segment_site_id: segment,
        declaration: declaration.to_string(),
        checked_occurrence_path: vec![frame_id],
        semantic_position: frame_id,
        input_interface: oriented_test_interface(1),
        output_interface: oriented_test_interface(2),
        runtime_frame_fingerprint: crate::compiler_private_computational_match_frame_fingerprint(
            &cases, &default,
        ),
        occurrence_binding_fingerprint: 0,
        control_witness: crate::OrientedControlWitnessV1::DistinguishedRoot,
    };
    frame.occurrence_binding_fingerprint =
        crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
    frame
}

#[cfg(test)]
fn d5_mutual_template(
    template: u64,
    declaration: &str,
    callee: &str,
    callee_frame: u64,
    callee_segment: u64,
) -> crate::CheckedRecursiveInvocationTemplateV1 {
    crate::CheckedRecursiveInvocationTemplateV1 {
        call_template_id: template,
        declaration: declaration.to_string(),
        checked_occurrence_path: vec![5],
        callee: callee.to_string(),
        level_instantiation: Vec::new(),
        recursion_group: "scc:fixture::d5::mutual".to_string(),
        scc_index: 0,
        admission: 1,
        arity: 1,
        local_telescope: vec![oriented_test_interface(1)],
        result_interface: oriented_test_interface(2),
        callee_segment_site_id: callee_segment,
        callee_frame_templates: vec![callee_frame],
        caller_interface: oriented_test_interface(2),
        runtime_marker_locations: vec![crate::CheckedRuntimeMarkerLocationV1 {
            declaration: declaration.to_string(),
            runtime_path: vec![3],
        }],
        occurrence_binding_fingerprint: 0,
    }
}

/// The mutual plan, re-fingerprinted after `edit`.
///
/// `edit` receives both templates in `(A→B, B→A)` order.
#[cfg(test)]
fn d5_mutual_plan_with(
    edit: impl FnOnce(
        &mut crate::CheckedRecursiveInvocationTemplateV1,
        &mut crate::CheckedRecursiveInvocationTemplateV1,
    ),
    refingerprint: bool,
) -> crate::OrientedSubcontinuationPlanV1 {
    let mut a = d5_mutual_template(
        D5_MUTUAL_A_TEMPLATE,
        D5_MUTUAL_A,
        D5_MUTUAL_B,
        D5_MUTUAL_B_FRAME,
        11,
    );
    let mut b = d5_mutual_template(
        D5_MUTUAL_B_TEMPLATE,
        D5_MUTUAL_B,
        D5_MUTUAL_A,
        D5_MUTUAL_A_FRAME,
        10,
    );
    a.occurrence_binding_fingerprint =
        crate::compiler_private_recursive_call_binding_fingerprint(&a);
    b.occurrence_binding_fingerprint =
        crate::compiler_private_recursive_call_binding_fingerprint(&b);
    edit(&mut a, &mut b);
    // ⛔ `refingerprint: false` is how an UPSTREAM-attributed control is built:
    // it leaves the stale fingerprint, which is exactly what
    // `OrientedSubcontinuationPlanV1::validate` owns.
    if refingerprint {
        a.occurrence_binding_fingerprint =
            crate::compiler_private_recursive_call_binding_fingerprint(&a);
        b.occurrence_binding_fingerprint =
            crate::compiler_private_recursive_call_binding_fingerprint(&b);
    }
    crate::OrientedSubcontinuationPlanV1 {
        representation_rule_version:
            crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
        frames: vec![
            d5_mutual_frame(D5_MUTUAL_A_FRAME, D5_MUTUAL_A, 10, true),
            d5_mutual_frame(D5_MUTUAL_B_FRAME, D5_MUTUAL_B, 11, false),
        ],
        recursive_calls: vec![a, b],
        computational_ih_slots: Vec::new(),
        computational_ih_calls: Vec::new(),
    }
}

/// Compile the mutual fixture, returning the outcome and the emitted calls.
#[cfg(test)]
fn d5_mutual_compile(
    plan: crate::OrientedSubcontinuationPlanV1,
) -> (
    Result<(), String>,
    Vec<(StaticOriginId, StaticOriginId, cranelift_codegen::ir::FuncRef)>,
) {
    let a = d5_mutual_declaration(D5_MUTUAL_A, D5_MUTUAL_B, D5_MUTUAL_A_TEMPLATE);
    let b = d5_mutual_declaration(D5_MUTUAL_B, D5_MUTUAL_A, D5_MUTUAL_B_TEMPLATE);
    let carrier = d5_mutual_frame_carrier();
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D5_MUTUAL_A.to_string(),
        }),
        args: vec![RuntimeExpr::Value(RuntimeValue::Int((5).into()))],
    };
    let declarations = BTreeMap::from([
        (D5_MUTUAL_A, &a),
        (D5_MUTUAL_B, &b),
        (D5_FRAME_CARRIER, &carrier),
    ]);
    reset_d5_emitted_declaration_calls();
    let outcome = compile_expr_into_module(
        new_jit_module().expect("JIT module"),
        "d5_mutual",
        Linkage::Local,
        &entry,
        &NativeSeedEnvironment::empty(),
        declarations,
        None,
        false,
        None,
        None,
        Some(plan),
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"));
    (outcome, d5_emitted_declaration_calls())
}

/// **The mutual positive.** Both checked cross-calls reconcile and emit.
#[test]
fn d5_c2_mutual_same_scc_calls_reconcile_and_emit() {
    let (outcome, emitted) = d5_mutual_compile(d5_mutual_plan_with(|_, _| {}, true));
    outcome.unwrap_or_else(|error| {
        panic!("D5: the mutual same-SCC pair must compile: {error}")
    });
    assert_eq!(
        emitted.len(),
        3,
        "D5: the entry's unchecked call plus both checked cross-calls. \
         Anything fewer and a mutation control below cannot distinguish a \
         refusal from a path that was never taken: {emitted:?}"
    );
}

// ── Control 4, the checked-plan half, on the MUTUAL fixture ───────────────
//
// ⭐ Each row names the axis, the mutation, and **which authority owns the
// refusal**. That last column is the point: the Architect's ruling requires
// interface / segment / frame-template / occurrence-fingerprint mutations to
// stay attributed to `OrientedSubcontinuationPlanV1::validate`, and forbids
// relabelling an upstream diagnostic as a D5-local first refusal. What D5 owes
// for those is a proof that the mutation **reaches** the canonical validator
// and that **no declaration-unit call is emitted**.
//
// ⛔ The emitted-call count is MEASURED for every row, never assumed. A refusal
// that arrives after a call was already written is a different fact from a
// refusal before emission, and only the count can tell them apart.

#[test]
fn d5_c4_checked_plan_mutations_each_reach_their_own_authority() {
    // (label, mutation, refingerprint, expected reason fragment, owning plane)
    type Edit = fn(
        &mut crate::CheckedRecursiveInvocationTemplateV1,
        &mut crate::CheckedRecursiveInvocationTemplateV1,
    );
    let rows: Vec<(&str, Edit, bool, &str, &str)> = vec![
        // ── D5-local: the same-SCC facts nothing upstream closes ──────────
        //
        // ⭐⭐ This row is the whole reason the mutual fixture exists. On a
        // self-call fixture the template is its OWN group witness, so renaming
        // the group moves both together and the mutation is inert. Here the
        // witness is the other template, and the rename separates them.
        (
            "recursion_group (needs the mutual fixture)",
            |a, _b| a.recursion_group = "scc:fixture::d5::elsewhere".to_string(),
            true,
            "callee is not a recursive member of its own recursion group",
            "D5",
        ),
        (
            "scc_index",
            |a, _b| a.scc_index = 7,
            true,
            "disagrees about its scc index",
            "D5",
        ),
        (
            "admission",
            |a, _b| a.admission = 9,
            true,
            "disagrees about its admission",
            "D5",
        ),
        (
            "arity",
            |a, _b| a.arity = 2,
            true,
            "callee or arity is stale",
            "enter_checked_recursive_invocation",
        ),
        // ── transplant: the marker names a callee the call does not ───────
        //
        // ⚠ The callee is moved **together with its frame binding**, on
        // purpose. Moving `callee` alone leaves the plan internally
        // inconsistent (`callee_frame_templates` still names the other
        // declaration's frame), so `validate` refuses first and the row would
        // measure the plan's consistency law instead of the transplant
        // ([[a-mutation-on-the-discriminator-input-measures-the-consistency-law-not-the-decision]]).
        // Self-consistent, it survives to the lowering, where the marker wraps
        // a call to `mutual_b` while the template claims `mutual_a`.
        (
            "transplant (callee, self-consistent)",
            |a, _b| {
                a.callee = D5_MUTUAL_A.to_string();
                a.callee_frame_templates = vec![D5_MUTUAL_A_FRAME];
                a.callee_segment_site_id = 10;
            },
            true,
            "callee or arity is stale",
            "enter_checked_recursive_invocation",
        ),
        // ── upstream-attributed, and kept that way ────────────────────────
        (
            "occurrence fingerprint",
            |a, _b| a.scc_index = 7,
            false,
            "occurrence binding is inconsistent",
            "OrientedSubcontinuationPlanV1::validate",
        ),
        (
            "callee segment site",
            |a, _b| a.callee_segment_site_id = 99,
            true,
            "callee binding is inconsistent",
            "OrientedSubcontinuationPlanV1::validate",
        ),
        (
            "callee frame-template set",
            |a, _b| a.callee_frame_templates = vec![D5_MUTUAL_A_FRAME],
            true,
            "callee binding is inconsistent",
            "OrientedSubcontinuationPlanV1::validate",
        ),
        (
            "result interface composition",
            |a, _b| a.result_interface = oriented_test_interface(5),
            true,
            "checked endpoints do not compose",
            "OrientedSubcontinuationPlanV1::validate",
        ),
        (
            "caller interface composition",
            |a, _b| a.caller_interface = oriented_test_interface(5),
            true,
            "checked endpoints do not compose",
            "OrientedSubcontinuationPlanV1::validate",
        ),
        // ── omission: a planned template with no Runtime marker ───────────
        (
            "omission (marker location)",
            |a, _b| a.runtime_marker_locations[0].runtime_path = vec![3, 0],
            true,
            "Runtime occurrences differ",
            "planning::validate_oriented_subcontinuation_transport",
        ),
    ];

    for (label, edit, refingerprint, fragment, plane) in rows {
        let plan = d5_mutual_plan_with(edit, refingerprint);
        let (outcome, emitted) = d5_mutual_compile(plan);
        let refusal = outcome.unwrap_err_or_panic(label);
        assert!(
            refusal.contains(fragment),
            "D5 control 4 [{label}]: the refusal must be the one {plane} \
             owns. A different one means this row measures some other \
             mechanism, and the axis it names stays unpinned: {refusal}"
        );
        // ⚠ **Zero is the wrong floor, and measuring said so.** The
        // entry's own call into `mutual_a`'s unit is UNCHECKED and lawful,
        // and the root unit is emitted before any declaration body — so a
        // refusal inside a body legitimately leaves it behind. Rows whose
        // authority runs before lowering leave nothing at all.
        //
        // ⇒ The fact being asserted is that **no checked cross-call was
        // emitted**: the unmutated fixture emits 3, so anything above 1
        // means a checked call this row was supposed to stop got through.
        assert!(
            emitted.len() <= 1,
            "D5 control 4 [{label}]: refused, but {} declaration-unit \
             call(s) were written — more than the entry's own unchecked \
             one, so a checked cross-call reached emission. Refusing after \
             emission is a different guarantee from refusing before it: \
             {emitted:?}",
            emitted.len()
        );
    }

    // The positive control on the harness, in the same shape as every row
    // above: unmutated, this fixture compiles and emits its three calls.
    let (outcome, emitted) = d5_mutual_compile(d5_mutual_plan_with(|_, _| {}, true));
    assert!(
        outcome.is_ok() && emitted.len() == 3,
        "D5 control 4: without a mutation the fixture must reach emission. \
         Every refusal above is otherwise consistent with a fixture that \
         never got there: {outcome:?} {emitted:?}"
    );
}

#[cfg(test)]
trait D5UnwrapErr {
    fn unwrap_err_or_panic(self, label: &str) -> String;
}

#[cfg(test)]
impl D5UnwrapErr for Result<(), String> {
    fn unwrap_err_or_panic(self, label: &str) -> String {
        match self {
            Ok(()) => panic!(
                "D5 control 4 [{label}]: the mutation compiled. An accepted \
                 mutation means no plane is reading that field"
            ),
            Err(reason) => reason,
        }
    }
}

/// **The mutual fixture is LOAD-BEARING, and this is the proof rather than the
/// claim.**
///
/// ⭐⭐ The `recursion_group` row above is the one the ruling's "no
/// self-call-only shortcut" clause exists for. Its comment says a self-call
/// fixture cannot discriminate that axis; a comment is
/// [structurally exempt from execution][[a-mechanism-claim-in-a-comment-is-structurally-exempt-from-execution]],
/// so the same mutation is run on BOTH fixtures here and the difference is
/// asserted.
///
/// ⛔ If this ever goes green in both directions, the `recursion_group` row is
/// no longer measuring anything and the mutual fixture has stopped earning its
/// keep.
#[test]
fn d5_the_recursion_group_axis_is_inert_on_a_self_call_and_causal_on_the_mutual_pair() {
    let rename = |group: &mut String| *group = "scc:fixture::d5::elsewhere".to_string();

    // Self-call: the template is its OWN group witness, so renaming the group
    // moves the template and its witness together and nothing disagrees.
    let (self_outcome, self_emitted) = d5_compile(d5_plan_with(|call| rename(&mut call.recursion_group)), None);
    assert!(
        self_outcome.is_ok() && self_emitted.len() == 2,
        "the self-call fixture must be INERT under the rename — that is the \
         gap the mutual fixture closes, and if this fixture caught it the \
         mutual one would be redundant: {self_outcome:?} {self_emitted:?}"
    );

    // Mutual: the witness is the OTHER template, and the rename separates them.
    let (mutual_outcome, mutual_emitted) = d5_mutual_compile(d5_mutual_plan_with(
        |a, _b| rename(&mut a.recursion_group),
        true,
    ));
    let reason = mutual_outcome.expect_err(
        "the mutual fixture must CATCH the rename the self-call fixture misses",
    );
    assert!(
        reason.contains("callee is not a recursive member of its own recursion group"),
        "the mutual fixture must catch it for the same-SCC reason, not some \
         incidental one: {reason}"
    );
    assert!(
        mutual_emitted.len() <= 1,
        "no checked cross-call may be emitted: {mutual_emitted:?}"
    );
}

/// **Duplicate — one checked template consumed by two occurrences.**
///
/// ⚠ Unlike every other row, this one legitimately emits the FIRST call before
/// refusing. The first occurrence is lawful; only its repeat is not. Asserting
/// `emitted.is_empty()` here would be asserting the wrong property, so the
/// count is measured and stated.
#[test]
fn d5_c4_a_duplicated_checked_occurrence_is_refused_after_its_lawful_first() {
    let a = RuntimeDeclaration {
        symbol: D5_MUTUAL_A.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: vec![RuntimeExpr::Value(RuntimeValue::Int((7).into()))],
                params: vec!["n".to_string()],
                // Two occurrences of the SAME template id, sequenced by a
                // `Let` so neither nests inside the other — nesting has its own
                // separate refusal and would mask this one.
                body: Box::new(RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::CheckedRecursiveInvocation {
                        call_template_id: D5_MUTUAL_A_TEMPLATE,
                        checked_occurrence_path: vec![5],
                        body: Box::new(RuntimeExpr::Call {
                            callee: Box::new(RuntimeExpr::DeclarationRef {
                                symbol: D5_MUTUAL_B.to_string(),
                            }),
                            args: vec![RuntimeExpr::Var(0)],
                        }),
                    }),
                    body: Box::new(RuntimeExpr::CheckedRecursiveInvocation {
                        call_template_id: D5_MUTUAL_A_TEMPLATE,
                        checked_occurrence_path: vec![5],
                        body: Box::new(RuntimeExpr::Call {
                            callee: Box::new(RuntimeExpr::DeclarationRef {
                                symbol: D5_MUTUAL_B.to_string(),
                            }),
                            args: vec![RuntimeExpr::Var(1)],
                        }),
                    }),
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };
    let b = d5_mutual_declaration(D5_MUTUAL_B, D5_MUTUAL_A, D5_MUTUAL_B_TEMPLATE);
    let carrier = d5_mutual_frame_carrier();
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D5_MUTUAL_A.to_string(),
        }),
        args: vec![RuntimeExpr::Value(RuntimeValue::Int((5).into()))],
    };
    // ⛔ Both structural paths are declared in the plan, so the marker-location
    // reconciliation upstream is SATISFIED and this row reaches the affine
    // occurrence check it names rather than stopping at transport.
    let plan = d5_mutual_plan_with(
        |a, _b| {
            a.runtime_marker_locations = vec![
                crate::CheckedRuntimeMarkerLocationV1 {
                    declaration: D5_MUTUAL_A.to_string(),
                    runtime_path: vec![3, 0],
                },
                crate::CheckedRuntimeMarkerLocationV1 {
                    declaration: D5_MUTUAL_A.to_string(),
                    runtime_path: vec![3, 1],
                },
            ];
        },
        true,
    );
    reset_d5_emitted_declaration_calls();
    let outcome = compile_expr_into_module(
        new_jit_module().expect("JIT module"),
        "d5_duplicate",
        Linkage::Local,
        &entry,
        &NativeSeedEnvironment::empty(),
        BTreeMap::from([
            (D5_MUTUAL_A, &a),
            (D5_MUTUAL_B, &b),
            (D5_FRAME_CARRIER, &carrier),
        ]),
        None,
        false,
        None,
        None,
        Some(plan),
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"));
    let reason = outcome.expect_err(
        "D5: one checked template consumed by two occurrences must be refused",
    );
    assert!(
        reason.contains("consumed twice") || reason.contains("consumed more than once"),
        "D5: the refusal must be the affine occurrence check. Any other \
         one leaves the duplicate class unpinned: {reason}"
    );
    let emitted = d5_emitted_declaration_calls();
    assert!(
        emitted.len() <= 2,
        "D5: at most the entry's unchecked call and the first, LAWFUL \
         checked occurrence may be emitted before the repeat is refused: \
         {emitted:?}"
    );
}

// ── The D5 checked-call CLOSEOUT ──────────────────────────────────────────
//
// ⭐⭐ **The closeout is the only check that can see a call nobody accounted
// for.** Every other D5 control is local to one call site: it can prove that
// site refused, or emitted the right target. None of them can see a lawful
// emission whose record went missing, a template that recorded twice, or a
// record whose callee is not what the instruction calls — because each of those
// is consistent with every per-site check passing.
//
// ⛔ Each row defeats exactly ONE of the closeout's three claims, so a green row
// names a specific property rather than "the closeout is on".

#[test]
fn d5_the_checked_call_closeout_rejects_omission_duplication_and_a_substituted_callee() {
    let rows: [(&str, D5CloseoutMutation, &str); 4] = [
        (
            "a lawful call whose ledger entry is suppressed",
            D5CloseoutMutation::SuppressLedgerEntry,
            "does not equal the planned one",
        ),
        (
            "one template recording two entries",
            D5CloseoutMutation::DuplicateLedgerEntry,
            "emitted more than one declaration-unit call",
        ),
        (
            "an entry under a template the plan never issued",
            D5CloseoutMutation::ExtraLedgerEntry,
            "does not equal the planned one",
        ),
        (
            "a recorded callee that is not the emitted one",
            D5CloseoutMutation::SubstituteEmittedCallee,
            "emitted a call to",
        ),
    ];
    for (label, mutation, fragment) in rows {
        with_d5_closeout_mutation(mutation, || {
            let (outcome, _emitted) =
                d5_mutual_compile(d5_mutual_plan_with(|_, _| {}, true));
            let refusal = outcome.unwrap_err_or_panic(label);
            assert!(
                refusal.contains(fragment),
                "D5 closeout [{label}]: the refusal must be the closeout's \
                 own. Any other one means this row never reached it and the \
                 claim it names stays unpinned: {refusal}"
            );
        });
    }

    // ⛔ The positive, in the same shape. Without it every row above is equally
    // consistent with the mutual fixture failing for an unrelated reason
    // ([[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]).
    let (outcome, emitted) = d5_mutual_compile(d5_mutual_plan_with(|_, _| {}, true));
    assert!(
        outcome.is_ok() && emitted.len() == 3,
        "D5 closeout: unmutated, planned = consumed = emitted holds and the \
         fixture compiles: {outcome:?} {emitted:?}"
    );
}

/// **The closeout's `planned` set is the plan's own, and this proves it is not
/// derived from what happened to be emitted.**
///
/// ⚠ A set-equality gate whose "expected" side is computed from the observed
/// side is an identity. Here `planned` comes from `plan.recursive_calls` and
/// nothing else, so adding a template the program cannot possibly emit must
/// red — and does.
#[test]
fn d5_the_closeout_planned_set_comes_from_the_plan_not_from_the_emissions() {
    let mut plan = d5_mutual_plan_with(|_, _| {}, true);
    // A third template, well-formed and bound to a real frame, for a
    // declaration whose body carries no marker for it.
    let mut orphan = d5_mutual_template(
        912,
        D5_MUTUAL_B,
        D5_MUTUAL_A,
        D5_MUTUAL_A_FRAME,
        10,
    );
    orphan.checked_occurrence_path = vec![6];
    orphan.runtime_marker_locations = vec![crate::CheckedRuntimeMarkerLocationV1 {
        declaration: D5_MUTUAL_B.to_string(),
        runtime_path: vec![3],
    }];
    orphan.occurrence_binding_fingerprint =
        crate::compiler_private_recursive_call_binding_fingerprint(&orphan);
    plan.recursive_calls.push(orphan);
    let (outcome, _emitted) = d5_mutual_compile(plan);
    let refusal = outcome.expect_err(
        "a planned template the program never emits must be caught — if this \
         compiles, `planned` is being read off the emissions and the whole \
         set equality is an identity",
    );
    // ⚠ Attribution stated, not assumed: the marker-location reconciliation
    // upstream sees this first, because the extra template declares an
    // occurrence `mutual_b`'s body does not have. That is the correct owner
    // for THIS shape; the closeout owns the shapes above, where the plan and
    // the IR agree and only the ledger diverges.
    assert!(
        refusal.contains("Runtime occurrences differ")
            || refusal.contains("does not equal the planned one"),
        "the refusal must come from the marker reconciliation or the \
         closeout, not from somewhere incidental: {refusal}"
    );
}

// ── The generic closure-valued-constructor-field NEGATIVE ────────────────
//
// ⭐⭐ **This row must keep rejecting, and it is not waiting on a capability.**
//
// ⚠ **I first wrote it as a D6 activation sentinel and that framing was wrong**
// (Architect `evt_44k5h9z49nf9b`). The fixture returns `Wrap(LexicalClosure)`
// with **no consuming computational eliminator**, so its closure genuinely is
// stored constructor data — an escape, permanently forbidden by
// `CallableCapsuleEscape -> EscapeForbidden`. The landed object fixture that
// reddened D6 looks similar and is a different thing: there the closure sits at
// a ruled recursive position of a checked `ComputationalMatch` and is invoked
// through the checked IH route, which makes it an existing
// `ContinuationSpecialization` edge rather than observable data. `D5a`
// eliminates that one; nothing eliminates this one.
//
// ⛔ **So this control has no sentinel trigger and names no future capability.**
// A comment saying it "should turn green" would invite exactly the carrier lane
// the prohibition exists to prevent.
//
// ⚠ The pair is still what makes it a measurement: both fixtures are
// closure-bodied transparent declarations reaching the SAME lane under the
// witness, differing in one field. The plain-field half proves the lane, the
// witness and the declaration shape are all fine; the closure-field half proves
// the escape prohibition.

