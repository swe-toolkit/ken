//! Oriented control, PX8J/PX8DS recursor-consumer, root-authority and
//! source-install lowering tests (RT-SPLIT §10.2: `oriented_*`, `px8j_*`,
//! root-authority, join-site, source-install and recursor tests -> `control`).

use super::*;
use crate::cranelift_backend::lowering::units::{
    continuation_case_binder_run, ContinuationCaseBinderSource,
};
use crate::RuntimeSymbolMetadata;
// `RT-SEED-CALL-PORT` `D1` — the class's population is measured on the real
// seed corpus, not on hand-built witnesses. `values.rs` and `constructors.rs`
// reach it the same way.
use crate::nc5_seed_examples;
// `RT-SEED-CALL-PORT` `D2` — the `AC-6` controls report the run, not just its
// success, so they name the report type.
use crate::CraneliftRunReport;
// `RT-SRCBODY-BIND-ORDER` `D3` — the whole-process run harness lives beside
// the effect controls that first needed it; the binding-order controls below
// run the same shape.
use super::effects::{BorrowedFixtureValue, RootIngressFixture};
use crate::cranelift_backend::lowering::units::{
    srcbody_bind_order_take, SrcbodyBindHost, SrcbodyBindOrderObservation,
};


#[derive(Clone, Copy, Debug)]
pub(in crate::cranelift_backend::lowering) enum Px8dsEdgeMutation {
    Delete,
    Duplicate,
    StaleParent,
    CrossSibling,
    WrongStaticParent,
}
/// ⚠ The plan here is a minimal inert one: every test that uses this builder
/// exercises a ledger, authority, or frame validator and never lowers an
/// expression through it, so no child origin is ever derived. A test that DOES
/// lower a fixture builds its own `Lowering` with that fixture's plan.
pub(in crate::cranelift_backend::lowering) fn root_authority_test_lowering<'a>(seed_env: &'a NativeSeedEnvironment) -> Lowering<'a> {
    Lowering {
        seed_env,
        declarations: BTreeMap::new(),
        static_transition_plan: inert_test_plan(),
        result_table: BTreeMap::new(),
        next_token: 0,
        next_recursor_frame_provenance: 0,
        next_recursor_producer_origin: 0,
        next_continuation_activation: 0,
        next_continuation_cursor: 0,
        next_source_join: 0,
        next_source_predecessor: 0,
        live_source_continuations: 0,
        carried_suffix_reentries: 0,
        source_control_root: None,
        active_oriented_semantic_regions: 0,
        active_carried_computational_eliminations: Vec::new(),
        native_join_plan: Some(crate::NativeJoinPlanV1 {
            representation_rule_version: crate::NativeJoinPlanV1::REPRESENTATION_RULE_VERSION,
            sites: vec![self_consistent_root_join_site(0)],
        }),
        consumed_join_sites: BTreeSet::new(),
        root_terminal_authority: None,
        active_join_site: None,
        oriented_subcontinuation_plan: None,
        consumed_subcontinuation_frames: BTreeSet::new(),
        active_subcontinuation_frame: None,
        consumed_recursive_call_templates: BTreeSet::new(),
        pending_recursive_call: None,
        pending_computational_ih_call: None,
        active_recursive_invocations: Vec::new(),
        next_recursive_invocation_instance: 1,
        dynamic_splice_edges: BTreeMap::new(),
        next_dynamic_splice_edge: 1,
        assumptions: BTreeSet::new(),
        unsupported: Vec::new(),
        continuation_claims: None,
        fusion_compositions: None,
        static_worker_fields: Default::default(),
        fusion_claims: None,
        fused_consumer_authority: None,
        continuation_candidates: None,
        checked_call_ledger: None,
        defining_unit: None,
        defining_emission_owner: None,
        defining_function_id: None,
        aggregate_allocations: None,
        host_effect_seats: None,
        process_object: true,
        process_symbols: crate::NativeProcessSymbols::legacy_prelude(),
        // ⛔ `None` — a bare `Lowering` fixture emits into no module, so it has
        // no callable carrier refs. The `Carried` routes fail closed on this
        // rather than silently taking the `Specialized` path.
        native_int_mutation: NativeIntLoweringMutation::Exact,
        bounded_nat_mutation: BoundedNatLoweringMutation::Exact,
        function_local: FunctionLocalRefs {
            defining_abi_operands: Vec::new(),
            defining_abi_slot_kinds: Vec::new(),
            context_calls: BTreeMap::new(),
            worker_templates: BTreeMap::new(),
            generated_context_captures: None,
            constructed_context_frame: None,
            seed_material: crate::cranelift_backend::lowering::seed_material::SeedMaterialRefs::none_for_tests(),
            host_dispatch: None,
            host_dispatch_context: None,
            services_pointer: None,
            native_int_arena: None,
            boundary_arena: None,
            native_int_binop: None,
            native_int_compare: None,
            native_int_intern: None,
            native_int_narrow: None,
            native_int_export: None,
            native_int_export_parts: None,
            native_int_resolve: None,
            native_int_tags: BTreeMap::new(),
            unit_calls: BTreeMap::new(),
            worker_calls: BTreeMap::new(),
            raw_worker_calls: BTreeMap::new(),
            continuation_calls: BTreeMap::new(),
            continuation_emissions: BTreeMap::new(),
            checked_ih_transport_emissions: Vec::new(),
            pending_composed_discharges: Vec::new(),
            composed_discharges: BTreeMap::new(),
            declaration_calls: BTreeMap::new(),
            trap_exit: None,
            terminal_result_origins: BTreeSet::new(),
            consumed_join_origins: BTreeSet::new(),
            dispositioned_join_origins: BTreeSet::new(),
            join_disposition_finalized: false,
            final_reachable_join_origins: BTreeSet::new(),
            materialized_join_blocks: BTreeMap::new(),
            emission_reachable_match_cases: BTreeMap::new(),
            boundary_carrier: None,
        },
    }
}

fn assert_exact_frame_scope(witness: FrameScopeHarnessWitness) {
    assert!(witness.first_consume_succeeds);
    assert!(witness.same_successor_duplicate_rejected);
    assert!(witness.second_successor_first_consume_succeeds);
    assert!(witness.post_join_duplicate_rejected);
}

#[test]
fn checked_frame_branch_scope_harness_uses_live_lowering_ledger() {
    let seed_env = NativeSeedEnvironment::empty();
    let mut lowering = root_authority_test_lowering(&seed_env);

    assert_exact_frame_scope(CheckedFrameBranchScope::harness(
        &mut lowering.consumed_subcontinuation_frames,
        FrameScopeHarnessMutation::Exact,
    ));

    let mut lowering = root_authority_test_lowering(&seed_env);
    let shared = CheckedFrameBranchScope::harness(
        &mut lowering.consumed_subcontinuation_frames,
        FrameScopeHarnessMutation::SharedLedger,
    );
    assert!(shared.first_consume_succeeds);
    assert!(shared.same_successor_duplicate_rejected);
    assert!(!shared.second_successor_first_consume_succeeds);
    assert!(shared.post_join_duplicate_rejected);

    let mut lowering = root_authority_test_lowering(&seed_env);
    let dropped = CheckedFrameBranchScope::harness(
        &mut lowering.consumed_subcontinuation_frames,
        FrameScopeHarnessMutation::DropUnion,
    );
    assert!(dropped.first_consume_succeeds);
    assert!(dropped.same_successor_duplicate_rejected);
    assert!(dropped.second_successor_first_consume_succeeds);
    assert!(!dropped.post_join_duplicate_rejected);
}

#[cfg(test)]
fn run_px8j_malformed_recursor_consumer(
    consumer: Px8jDirectRecursorConsumer,
    malformation: Px8jRecursorMalformation,
) -> Result<LoweringOperand, CraneliftBackendError> {
    let mut module = new_jit_module()?;
    let mut signature = module.make_signature();
    signature.returns.push(AbiParam::new(types::I64));
    let func_id = module
        .declare_function("px8j_malformed_recursor", Linkage::Local, &signature)
        .map_err(|error| backend_module(error.to_string()))?;
    let mut context = module.make_context();
    context.func =
        Function::with_name_signature(UserFuncName::user(0, func_id.as_u32()), signature);
    let seed_env = NativeSeedEnvironment::empty();
    // The consumer under test lowers exactly one of these two fixtures, so the
    // plan is that fixture's own: every origin the lowering derives below is a
    // real positional child of a really-planned occurrence.
    let call = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::Var(0)),
        args: Vec::new(),
    };
    let pending_let = RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        body: Box::new(RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Var(1)),
            args: Vec::new(),
        }),
    };
    let lowered_fixture = match consumer {
        Px8jDirectRecursorConsumer::PendingLetProducer => &pending_let,
        Px8jDirectRecursorConsumer::ProducerCall | Px8jDirectRecursorConsumer::OrdinaryCall => {
            &call
        }
    };
    let (static_transition_plan, fixture_origin) = planned_root_occurrence(lowered_fixture);
    let mut compiler = Lowering {
        seed_env: &seed_env,
        declarations: BTreeMap::new(),
        static_transition_plan,
        result_table: BTreeMap::new(),
        next_token: 0,
        next_recursor_frame_provenance: 0,
        next_recursor_producer_origin: 0,
        next_continuation_activation: 0,
        next_continuation_cursor: 0,
        next_source_join: 0,
        next_source_predecessor: 0,
        live_source_continuations: 0,
        carried_suffix_reentries: 0,
        source_control_root: None,
        active_oriented_semantic_regions: 0,
        active_carried_computational_eliminations: Vec::new(),
        native_join_plan: None,
        consumed_join_sites: BTreeSet::new(),
        root_terminal_authority: None,
        active_join_site: None,
        oriented_subcontinuation_plan: None,
        consumed_subcontinuation_frames: BTreeSet::new(),
        active_subcontinuation_frame: None,
        consumed_recursive_call_templates: BTreeSet::new(),
        pending_recursive_call: None,
        pending_computational_ih_call: None,
        active_recursive_invocations: Vec::new(),
        next_recursive_invocation_instance: 1,
        dynamic_splice_edges: BTreeMap::new(),
        next_dynamic_splice_edge: 1,
        assumptions: BTreeSet::new(),
        unsupported: Vec::new(),
        continuation_claims: None,
        fusion_compositions: None,
        static_worker_fields: Default::default(),
        fusion_claims: None,
        fused_consumer_authority: None,
        continuation_candidates: None,
        checked_call_ledger: None,
        defining_unit: None,
        defining_emission_owner: None,
        defining_function_id: None,
        aggregate_allocations: None,
        host_effect_seats: None,
        process_object: false,
        process_symbols: crate::NativeProcessSymbols::legacy_prelude(),
        // ⛔ `None` — a bare `Lowering` fixture emits into no module, so it has
        // no callable carrier refs. The `Carried` routes fail closed on this
        // rather than silently taking the `Specialized` path.
        native_int_mutation: NativeIntLoweringMutation::Exact,
        bounded_nat_mutation: BoundedNatLoweringMutation::Exact,
        function_local: FunctionLocalRefs {
            defining_abi_operands: Vec::new(),
            defining_abi_slot_kinds: Vec::new(),
            context_calls: BTreeMap::new(),
            worker_templates: BTreeMap::new(),
            generated_context_captures: None,
            constructed_context_frame: None,
            seed_material: crate::cranelift_backend::lowering::seed_material::SeedMaterialRefs::none_for_tests(),
            host_dispatch: None,
            host_dispatch_context: None,
            services_pointer: None,
            native_int_arena: None,
            boundary_arena: None,
            native_int_binop: None,
            native_int_compare: None,
            native_int_intern: None,
            native_int_narrow: None,
            native_int_export: None,
            native_int_export_parts: None,
            native_int_resolve: None,
            native_int_tags: BTreeMap::new(),
            unit_calls: BTreeMap::new(),
            worker_calls: BTreeMap::new(),
            raw_worker_calls: BTreeMap::new(),
            continuation_calls: BTreeMap::new(),
            continuation_emissions: BTreeMap::new(),
            checked_ih_transport_emissions: Vec::new(),
            pending_composed_discharges: Vec::new(),
            composed_discharges: BTreeMap::new(),
            declaration_calls: BTreeMap::new(),
            trap_exit: None,
            terminal_result_origins: BTreeSet::new(),
            consumed_join_origins: BTreeSet::new(),
            dispositioned_join_origins: BTreeSet::new(),
            join_disposition_finalized: false,
            final_reachable_join_origins: BTreeSet::new(),
            materialized_join_blocks: BTreeMap::new(),
            emission_reachable_match_cases: BTreeMap::new(),
            boundary_carrier: None,
        },
    };
    let origin = RecursorProducerOriginId(7);
    let cursor = ContinuationCursorId(9);
    let layer = |role| ComputationalRecursorLayer {
        cases: Vec::new(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::ExplicitTrap,
            message: "px8j malformed recursor role".to_string(),
        },
        outer_env: Vec::new(),
        static_origin: inert_test_static_origin(),
        provenance: RecursorFrameProvenance(6),
        role,
        checked_frame_id: None,
        checked_invocation_id: None,
        checked_invocation_source: None,
        checked_invocation_depth: 0,
        semantic_pending: matches!(role, RecursorLayerRole::SelectsOccurrence { .. }),
    };
    let selection = layer(match malformation {
        Px8jRecursorMalformation::SelectionRole => RecursorLayerRole::ExitsScope {
            origin,
            scope_origin: origin,
            parent_scope: None,
        },
        Px8jRecursorMalformation::RepeatedScopeIdentity
        | Px8jRecursorMalformation::BrokenScopeParent => {
            RecursorLayerRole::SelectsOccurrence { origin }
        }
    });
    let unwind = match malformation {
        Px8jRecursorMalformation::SelectionRole => Vec::new(),
        Px8jRecursorMalformation::RepeatedScopeIdentity => vec![
            layer(RecursorLayerRole::ExitsScope {
                origin,
                scope_origin: RecursorProducerOriginId(11),
                parent_scope: None,
            }),
            layer(RecursorLayerRole::ExitsScope {
                origin,
                scope_origin: RecursorProducerOriginId(11),
                parent_scope: Some(RecursorProducerOriginId(11)),
            }),
        ],
        Px8jRecursorMalformation::BrokenScopeParent => vec![
            layer(RecursorLayerRole::ExitsScope {
                origin,
                scope_origin: RecursorProducerOriginId(11),
                parent_scope: None,
            }),
            layer(RecursorLayerRole::ExitsScope {
                origin,
                scope_origin: RecursorProducerOriginId(12),
                parent_scope: Some(RecursorProducerOriginId(99)),
            }),
        ],
    };
    let recursor = Lowered::ComputationalRecursorClosure {
        residual: Box::new(LoweringOperand::Specialized(Lowered::Closure {
            captures: Vec::new(),
            params: Vec::new(),
            // An inert residual body. This test drives the recursor-malformation
            // validator and never lowers the body, so the inert planned origin is
            // the whole of it — and since B2A-S the carrier *is* the origin, the
            // fixture can no longer pair an arbitrary term with an unrelated tag.
            body: inert_test_static_origin(),
        })),
        activation: ContinuationActivationId(8),
        invocation: RecursorInvocationSegment::new(
            origin,
            0,
            selection,
            RecursorUnwindStack {
                later_wrappers_in_construction_order: unwind,
            },
            cursor,
            None,
            None,
        ),
    };
    let active = ActiveContinuationFrame {
        activation: ContinuationActivationId(8),
        cursor,
        parent: None,
        pending: &[],
        selected_ancestry: &[],
        source_lineage: &[],
        source_selected_cursor: None,
        selected_scope: None,
    };
    let active_frames = [EliminatorFrame::Active(active)];
    let env = [LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(
        recursor,
    ))];
    let mut function_context = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut context.func, &mut function_context);
    let entry = builder.create_block();
    builder.switch_to_block(entry);
    let occurrence = SourceOccurrence {
        expr: lowered_fixture,
        static_origin: fixture_origin,
    };
    match consumer {
        Px8jDirectRecursorConsumer::PendingLetProducer
        | Px8jDirectRecursorConsumer::ProducerCall => compiler.lower_computational_producer_expr(
            &mut builder,
            occurrence,
            &env,
            &active_frames,
        ),
        Px8jDirectRecursorConsumer::OrdinaryCall => {
            compiler.lower_expr(&mut builder, occurrence, &env)
        }
    }
}

pub(in crate::cranelift_backend::lowering) fn oriented_dynamic_sibling_fixture() -> (
    crate::OrientedSubcontinuationPlanV1,
    RecursorInvocationSegment,
    Vec<DynamicSpliceEdge>,
) {
    let plan = oriented_test_ih_plan();
    let origin = RecursorProducerOriginId(60);
    let mut segment = RecursorInvocationSegment::new(
        origin,
        0,
        oriented_test_instance_layer(
            2,
            11,
            1,
            true,
            RecursorLayerRole::SelectsOccurrence { origin },
        ),
        RecursorUnwindStack {
            later_wrappers_in_construction_order: vec![oriented_test_instance_layer(
                0,
                12,
                1,
                true,
                RecursorLayerRole::ExitsScope {
                    origin,
                    scope_origin: RecursorProducerOriginId(61),
                    parent_scope: None,
                },
            )],
        },
        ContinuationCursorId(13),
        None,
        None,
    );
    segment.dynamic_splice_edges = vec![DynamicSpliceEdgeId(71), DynamicSpliceEdgeId(72)];
    let edges = vec![
        DynamicSpliceEdge {
            edge_id: DynamicSpliceEdgeId(71),
            child_invocation_instance_id: 11,
            parent_invocation_instance_id: 0,
            checked_call_template_id: 102,
            parent_frame_template_id: 2,
            segment_site_id: 9,
        },
        DynamicSpliceEdge {
            edge_id: DynamicSpliceEdgeId(72),
            child_invocation_instance_id: 12,
            parent_invocation_instance_id: 0,
            checked_call_template_id: 100,
            parent_frame_template_id: 0,
            segment_site_id: 9,
        },
    ];
    (plan, segment, edges)
}

#[test]
fn oriented_same_depth_siblings_require_exact_dynamic_edges() {
    let (plan, segment, edges) = oriented_dynamic_sibling_fixture();

    let mut old_flat = std::iter::once(&segment.selection)
        .chain(segment.unwind.later_wrappers_in_construction_order.iter())
        .filter(|layer| layer.semantic_pending)
        .collect::<Vec<_>>();
    old_flat.sort_by_key(|layer| {
        (
            std::cmp::Reverse(layer.checked_invocation_depth),
            plan.frame(layer.checked_frame_id.unwrap())
                .unwrap()
                .semantic_position,
        )
    });
    let [left, right] = old_flat.as_slice() else {
        panic!("the discriminator must carry exactly two same-depth siblings")
    };
    assert_eq!(left.checked_invocation_depth, 1);
    assert_eq!(right.checked_invocation_depth, 1);
    let left = plan.frame(left.checked_frame_id.unwrap()).unwrap();
    let right = plan.frame(right.checked_frame_id.unwrap()).unwrap();
    assert_ne!(
        left.output_interface, right.input_interface,
        "the retired flat ordering must invent the non-composable sibling adjacency"
    );

    let installed = compose_oriented_subcontinuation(
        Some(&plan),
        None,
        ContinuationActivationId(14),
        segment,
        edges,
    )
    .expect("exact child-to-parent edges keep same-depth siblings separate");
    assert_eq!(
        installed
            .semantic_frames
            .iter()
            .map(|frame| (
                frame.checked_invocation_id.unwrap(),
                frame.checked_frame_id.unwrap(),
            ))
            .collect::<Vec<_>>(),
        vec![(11, 2), (12, 0)],
    );
}

#[test]
fn oriented_dynamic_edge_mutations_reject_through_named_lanes() {
    let reject =
        |segment: RecursorInvocationSegment, edges: Vec<DynamicSpliceEdge>, expected: &str| {
            let plan = oriented_test_ih_plan();
            let error = match compose_oriented_subcontinuation(
                Some(&plan),
                None,
                ContinuationActivationId(14),
                segment,
                edges,
            ) {
                Ok(_) => panic!("a malformed dynamic splice graph must reject before CFG"),
                Err(error) => error,
            };
            assert!(
                matches!(
                    error,
                    CraneliftBackendError::Unsupported(UnsupportedLowering {
                        construct: "OrientedSubcontinuationPlanV1",
                        ref reason,
                    }) if reason.contains(expected)
                ),
                "expected {expected:?}, got {error:?}"
            );
        };

    let (_, segment, mut edges) = oriented_dynamic_sibling_fixture();
    edges.pop();
    reject(segment, edges, "deletion leaves an unparented");

    let (_, segment, mut edges) = oriented_dynamic_sibling_fixture();
    edges.push(DynamicSpliceEdge {
        edge_id: DynamicSpliceEdgeId(73),
        child_invocation_instance_id: 11,
        parent_invocation_instance_id: 0,
        checked_call_template_id: 102,
        parent_frame_template_id: 2,
        segment_site_id: 9,
    });
    reject(segment, edges, "duplicate affine splice edges");

    let (_, segment, mut edges) = oriented_dynamic_sibling_fixture();
    edges[0].parent_invocation_instance_id = 99;
    reject(segment, edges, "stale parent invocation");

    let (_, segment, mut edges) = oriented_dynamic_sibling_fixture();
    edges[0].parent_frame_template_id = 1;
    reject(segment, edges, "disagrees with its checked static parent");
}

#[test]
fn oriented_dynamic_edge_ledger_is_affine_and_sibling_isolated() {
    let seed_env = NativeSeedEnvironment::empty();
    let mut lowering = root_authority_test_lowering(&seed_env);
    let (_, mut segment, mut edges) = oriented_dynamic_sibling_fixture();
    let edge = edges.remove(0);
    segment.dynamic_splice_edges = vec![edge.edge_id];
    lowering.dynamic_splice_edges.insert(edge.edge_id, edge);

    let consumed = lowering
        .take_dynamic_splice_edges(&segment)
        .expect("the owning invocation consumes its edge exactly once");
    assert_eq!(consumed.len(), 1);
    let stolen = match lowering.take_dynamic_splice_edges(&segment) {
        Ok(_) => panic!("a sibling cannot steal an already-consumed edge"),
        Err(error) => error,
    };
    assert!(matches!(
        stolen,
        CraneliftBackendError::Unsupported(UnsupportedLowering { reason, .. })
            if reason.contains("consumed by a sibling")
    ));

    let (_, mut duplicated, mut edges) = oriented_dynamic_sibling_fixture();
    let edge = edges.remove(0);
    duplicated.dynamic_splice_edges = vec![edge.edge_id, edge.edge_id];
    lowering.dynamic_splice_edges.insert(edge.edge_id, edge);
    let duplicate = match lowering.take_dynamic_splice_edges(&duplicated) {
        Ok(_) => panic!("one carrier cannot duplicate an affine edge handle"),
        Err(error) => error,
    };
    assert!(matches!(
        duplicate,
        CraneliftBackendError::Unsupported(UnsupportedLowering { reason, .. })
            if reason.contains("handle is duplicated")
    ));
}

fn run_px8ds_edge_consumer(
    consumer: Px8jDirectRecursorConsumer,
    mutation: Px8dsEdgeMutation,
) -> Result<LoweringOperand, CraneliftBackendError> {
    let seed_env = NativeSeedEnvironment::empty();
    let mut compiler = root_authority_test_lowering(&seed_env);
    compiler.native_join_plan = None;
    compiler.root_terminal_authority = None;
    compiler.process_object = false;
    let (plan, mut segment, mut edges) = oriented_dynamic_sibling_fixture();
    compiler.oriented_subcontinuation_plan = Some(plan);

    match mutation {
        Px8dsEdgeMutation::Delete => {
            edges.remove(0);
        }
        Px8dsEdgeMutation::Duplicate => {
            segment
                .dynamic_splice_edges
                .push(segment.dynamic_splice_edges[0]);
        }
        Px8dsEdgeMutation::StaleParent => {
            edges[0].parent_invocation_instance_id = 99;
        }
        Px8dsEdgeMutation::CrossSibling => {
            let stolen = RecursorInvocationSegment {
                dynamic_splice_edges: vec![segment.dynamic_splice_edges[0]],
                ..segment.clone()
            };
            for edge in edges.drain(..) {
                compiler.dynamic_splice_edges.insert(edge.edge_id, edge);
            }
            compiler.take_dynamic_splice_edges(&stolen)?;
        }
        Px8dsEdgeMutation::WrongStaticParent => {
            edges[0].parent_frame_template_id = 1;
        }
    }
    for edge in edges {
        compiler.dynamic_splice_edges.insert(edge.edge_id, edge);
    }

    let cursor = segment.resume_cursor;
    let activation = ContinuationActivationId(90);
    let recursor = Lowered::ComputationalRecursorClosure {
        residual: Box::new(LoweringOperand::Specialized(Lowered::Closure {
            captures: Vec::new(),
            params: Vec::new(),
            // An inert residual body, as in the PX8J fixture above: the carrier is
            // the origin, and this test never lowers the body.
            body: inert_test_static_origin(),
        })),
        activation,
        invocation: segment,
    };
    let active = ActiveContinuationFrame {
        activation,
        cursor,
        parent: None,
        pending: &[],
        selected_ancestry: &[],
        source_lineage: &[],
        source_selected_cursor: None,
        selected_scope: None,
    };
    let active_frames = [EliminatorFrame::Active(active)];
    let env = [LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(
        recursor,
    ))];
    let call = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::Var(0)),
        args: Vec::new(),
    };
    let pending_let = RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        body: Box::new(RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Var(1)),
            args: Vec::new(),
        }),
    };
    // Plan the fixture this consumer actually lowers, and install that plan on
    // the compiler under test.
    let lowered_fixture = match consumer {
        Px8jDirectRecursorConsumer::PendingLetProducer => &pending_let,
        Px8jDirectRecursorConsumer::ProducerCall | Px8jDirectRecursorConsumer::OrdinaryCall => {
            &call
        }
    };
    let (static_transition_plan, fixture_origin) = planned_root_occurrence(lowered_fixture);
    compiler.static_transition_plan = static_transition_plan;

    let mut module = new_jit_module()?;
    let mut signature = module.make_signature();
    signature.returns.push(AbiParam::new(types::I64));
    let func_id = module
        .declare_function("px8ds_edge_consumer", Linkage::Local, &signature)
        .map_err(|error| backend_module(error.to_string()))?;
    let mut context = module.make_context();
    context.func =
        Function::with_name_signature(UserFuncName::user(0, func_id.as_u32()), signature);
    let mut function_context = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut context.func, &mut function_context);
    let entry = builder.create_block();
    builder.switch_to_block(entry);
    let occurrence = SourceOccurrence {
        expr: lowered_fixture,
        static_origin: fixture_origin,
    };
    match consumer {
        Px8jDirectRecursorConsumer::PendingLetProducer
        | Px8jDirectRecursorConsumer::ProducerCall => compiler.lower_computational_producer_expr(
            &mut builder,
            occurrence,
            &env,
            &active_frames,
        ),
        Px8jDirectRecursorConsumer::OrdinaryCall => {
            compiler.lower_expr(&mut builder, occurrence, &env)
        }
    }
}

#[test]
fn oriented_edge_mutations_reject_in_all_three_direct_consumers() {
    for consumer in [
        Px8jDirectRecursorConsumer::PendingLetProducer,
        Px8jDirectRecursorConsumer::ProducerCall,
        Px8jDirectRecursorConsumer::OrdinaryCall,
    ] {
        for (mutation, expected) in [
            (Px8dsEdgeMutation::Delete, "deleted, replayed"),
            (Px8dsEdgeMutation::Duplicate, "handle is duplicated"),
            (Px8dsEdgeMutation::StaleParent, "stale parent invocation"),
            (Px8dsEdgeMutation::CrossSibling, "consumed by a sibling"),
            (
                Px8dsEdgeMutation::WrongStaticParent,
                "disagrees with its checked static parent",
            ),
        ] {
            let error = match run_px8ds_edge_consumer(consumer, mutation) {
                Ok(_) => panic!("{consumer:?}/{mutation:?} must reject before CFG"),
                Err(error) => error,
            };
            assert!(
                matches!(
                    error,
                    CraneliftBackendError::Unsupported(UnsupportedLowering {
                        construct: "OrientedSubcontinuationPlanV1",
                        ref reason,
                    }) if reason.contains(expected)
                ),
                "{consumer:?}/{mutation:?}: expected {expected:?}, got {error:?}"
            );
        }
    }
}



#[test]
fn rt_escape_within_path_duplicate_frame_consume_still_rejects() {
    // RT-ESCAPE: forking `consumed_subcontinuation_frames` per mutually-exclusive
    // arm must not weaken the same-path affine guard. On a straight-line path
    // (no branch, so `lower_forked_branch`'s per-arm reset never applies),
    // consuming one checked frame twice must still reject before CFG. Direct-API
    // PX8DS-fixture style; exercises the frame consume the dynamic-splice-edge
    // mutation suite does not reach.
    let seed_env = NativeSeedEnvironment::empty();
    let (_expr, decl, plan) = occurrence_exact_marker_fixture(false, false);
    let RuntimeDeclarationKind::Transparent { body } = decl.kind else {
        panic!("fixture declaration is transparent");
    };
    let RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body } = body else {
        panic!("declaration body is a checked subcontinuation frame");
    };
    let RuntimeExpr::ComputationalMatch { cases, default, .. } = *body else {
        panic!("checked frame wraps a computational match");
    };
    let mut compiler = root_authority_test_lowering(&seed_env);
    compiler.native_join_plan = None;
    compiler.root_terminal_authority = None;
    compiler.process_object = false;
    compiler.oriented_subcontinuation_plan = Some(plan);

    // First consume on the path succeeds.
    compiler
        .enter_checked_subcontinuation_frame(frame_id)
        .expect("first enter of the checked frame");
    assert_eq!(
        compiler
            .consume_checked_subcontinuation_frame(&cases, &default)
            .expect("first consume of the checked frame succeeds"),
        Some(frame_id)
    );

    // A second enter + consume of the same frame on the same path rejects.
    compiler
        .enter_checked_subcontinuation_frame(frame_id)
        .expect("second enter re-marks the active frame");
    let err = compiler
        .consume_checked_subcontinuation_frame(&cases, &default)
        .expect_err("a same-path duplicate consume must reject before CFG");
    assert!(
        matches!(
            err,
            CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "OrientedSubcontinuationPlanV1",
                ref reason,
            }) if reason.contains("consumed more than once")
        ),
        "expected 'consumed more than once', got {err:?}"
    );
}

#[test]
fn oriented_source_open_occurrence_cross_checks_the_closure_selected_parent() {
    let seed_env = NativeSeedEnvironment::empty();
    let mut compiler = root_authority_test_lowering(&seed_env);
    let (_, _, mut edges) = oriented_dynamic_sibling_fixture();
    let edge = edges.remove(0);
    let edge_id = edge.edge_id;
    compiler.dynamic_splice_edges.insert(edge_id, edge);
    let instance = CheckedRecursiveInvocationInstance {
        source: InvocationTemplateRef::ComputationalIHCall(102),
        invocation_instance_id: 11,
        semantic_depth: 1,
        dynamic_splice_edge: Some(edge_id),
    };
    let mut open = OwnedSelectedScope {
        scope_origin: RecursorProducerOriginId(70),
        parent_scope: None,
        frame: ComputationalRecursorFramePayload {
            cases: Vec::new(),
            default: RuntimeTrap {
                code: RuntimeTrapCode::ExplicitTrap,
                message: "PX8-DS source parent".to_string(),
            },
            outer_env: Vec::new(),
            static_origin: inert_test_static_origin(),
            provenance: RecursorFrameProvenance(71),
            checked_frame_id: Some(2),
            checked_invocation_id: Some(0),
            checked_invocation_source: None,
            checked_invocation_depth: 0,
        },
    };
    compiler
        .validate_source_dynamic_splice_parent(instance, &open)
        .expect("the source open occurrence agrees with closure selection");
    open.frame.checked_frame_id = Some(0);
    let mismatch = compiler
        .validate_source_dynamic_splice_parent(instance, &open)
        .expect_err("source and closure parent identities must agree before CFG");
    assert!(matches!(
        mismatch,
        CraneliftBackendError::Unsupported(UnsupportedLowering { reason, .. })
            if reason.contains("source open occurrence disagrees")
    ));
}

#[test]
fn distinguished_root_authority_is_checked_affine_and_cursor_bound() {
    let seed_env = NativeSeedEnvironment::empty();
    let mut lowering = root_authority_test_lowering(&seed_env);
    let authority = lowering
        .take_distinguished_root_answer_authority()
        .expect("the exact checked root site validates")
        .expect("process lowering carries root authority");
    lowering.root_terminal_authority = Some(authority);
    lowering
        .mint_terminal_answer_authority()
        .expect("the first exhausted-root mint consumes the authority");
    let repeated = match lowering.mint_terminal_answer_authority() {
        Ok(_) => panic!("the affine root authority cannot mint twice"),
        Err(error) => error,
    };
    assert!(matches!(
        repeated,
        CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason))
            if reason == "terminal answer has no affine checked-root authority"
    ));

    let mut lowering = root_authority_test_lowering(&seed_env);
    let mut authority = lowering
        .take_distinguished_root_answer_authority()
        .unwrap()
        .unwrap();
    authority.outer_cursor = Some(ContinuationCursorId(7));
    let transplanted = lowering
        .restore_root_terminal_authority(Some(authority), ContinuationCursorId(8))
        .expect_err("a root token cannot cross the wrong source cursor");
    assert!(matches!(
        transplanted,
        CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason))
            if reason == "checked root answer authority returned through the wrong outer cursor"
    ));

    let mut lowering = root_authority_test_lowering(&seed_env);
    let mut authority = lowering
        .take_distinguished_root_answer_authority()
        .unwrap()
        .unwrap();
    let duplicate = RootTerminalAnswerAuthority {
        site_id: authority.site_id,
        checked_result_type_fingerprint: authority.checked_result_type_fingerprint,
        occurrence_binding_fingerprint: authority.occurrence_binding_fingerprint,
        outer_cursor: None,
    };
    lowering.root_terminal_authority = Some(duplicate);
    authority.outer_cursor = Some(ContinuationCursorId(9));
    let duplicated = lowering
        .restore_root_terminal_authority(Some(authority), ContinuationCursorId(9))
        .expect_err("a root token cannot duplicate across source control");
    assert!(matches!(
        duplicated,
        CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason))
            if reason == "checked root answer authority was duplicated across source control"
    ));
}

#[test]
fn px8j_all_three_direct_consumers_propagate_the_role_validator() {
    for consumer in [
        Px8jDirectRecursorConsumer::PendingLetProducer,
        Px8jDirectRecursorConsumer::ProducerCall,
        Px8jDirectRecursorConsumer::OrdinaryCall,
    ] {
        let error = match run_px8j_malformed_recursor_consumer(
            consumer,
            Px8jRecursorMalformation::SelectionRole,
        ) {
            Ok(_) => panic!("each live recursor consumer must reject the malformed selection"),
            Err(error) => error,
        };
        assert!(
            matches!(
                error,
                CraneliftBackendError::Unsupported(UnsupportedLowering {
                    construct: "ComputationalRecursor",
                    ref reason,
                }) if reason == "recursor selection role does not select the invocation origin"
            ),
            "{consumer:?}: {error:?}"
        );
    }
}

#[test]
fn px8j_release_validator_rejects_repeated_and_broken_scope_lineage() {
    for (malformation, expected_reason) in [
        (
            Px8jRecursorMalformation::RepeatedScopeIdentity,
            "recursor unwind repeats a selected scope identity",
        ),
        (
            Px8jRecursorMalformation::BrokenScopeParent,
            "recursor unwind has a broken selected-scope parent link",
        ),
    ] {
        let error = match run_px8j_malformed_recursor_consumer(
            Px8jDirectRecursorConsumer::OrdinaryCall,
            malformation,
        ) {
            Ok(_) => panic!("the real direct consumer must propagate release validation"),
            Err(error) => error,
        };
        assert!(
            matches!(
                error,
                CraneliftBackendError::Unsupported(UnsupportedLowering {
                    construct: "ComputationalRecursor",
                    ref reason,
                }) if reason == expected_reason
            ),
            "{malformation:?}: {error:?}"
        );
    }
}





#[test]
fn oriented_open_control_obligations_are_affine_and_mint_exact() {
    let plan = oriented_test_ih_plan();
    let mut deleted = oriented_five_control_invocation();
    deleted
        .unwind
        .later_wrappers_in_construction_order
        .remove(0);
    let deleted = match compose_oriented_subcontinuation(
        Some(&plan),
        None,
        ContinuationActivationId(8),
        deleted,
        Vec::new(),
    ) {
        Ok(_) => panic!("deleting only an inherited exit obligation must reject"),
        Err(error) => error,
    };
    assert!(matches!(
        deleted,
        CraneliftBackendError::Unsupported(UnsupportedLowering { reason, .. })
            if reason == "open control obligation set changed after affine mint"
    ));

    let mut duplicated = oriented_five_control_invocation();
    let duplicate = duplicated.unwind.later_wrappers_in_construction_order[0].clone();
    duplicated
        .unwind
        .later_wrappers_in_construction_order
        .push(duplicate);
    let duplicated = match compose_oriented_subcontinuation(
        Some(&plan),
        None,
        ContinuationActivationId(8),
        duplicated,
        Vec::new(),
    ) {
        Ok(_) => panic!("duplicating an inherited exit obligation must reject"),
        Err(error) => error,
    };
    assert!(matches!(
        duplicated,
        CraneliftBackendError::Unsupported(UnsupportedLowering { reason, .. })
            if reason == "open control obligation set changed after affine mint"
    ));
}
#[test]
fn oriented_endpoint_corruption_and_affine_reuse_fail_closed() {
    let mut plan = oriented_test_plan();
    plan.frames[2].output_interface = oriented_test_interface(9);
    plan.frames[2].occurrence_binding_fingerprint =
        crate::compiler_private_oriented_occurrence_binding_fingerprint(&plan.frames[2]);
    let error = match compose_oriented_subcontinuation(
        Some(&plan),
        None,
        ContinuationActivationId(8),
        oriented_test_invocation(),
        Vec::new(),
    ) {
        Ok(_) => panic!("endpoint corruption must reject before installation"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "OrientedSubcontinuationPlanV1",
            reason,
        }) if reason.contains("endpoints do not compose")
    ));

    let mut capability = AffineSpliceCapability {
        state: AffineSpliceState::Open,
    };
    capability.consume().unwrap();
    assert!(capability.consume().is_err());
}
fn oriented_five_control_invocation() -> RecursorInvocationSegment {
    let origin = RecursorProducerOriginId(40);
    let mut invocation = RecursorInvocationSegment::new(
        origin,
        0,
        oriented_test_instance_layer(
            2,
            0,
            0,
            true,
            RecursorLayerRole::SelectsOccurrence { origin },
        ),
        RecursorUnwindStack {
            later_wrappers_in_construction_order: vec![
                oriented_test_instance_layer(
                    2,
                    1,
                    0,
                    false,
                    RecursorLayerRole::ExitsScope {
                        origin,
                        scope_origin: RecursorProducerOriginId(41),
                        parent_scope: None,
                    },
                ),
                oriented_test_instance_layer(
                    0,
                    1,
                    0,
                    false,
                    RecursorLayerRole::ExitsScope {
                        origin,
                        scope_origin: RecursorProducerOriginId(42),
                        parent_scope: Some(RecursorProducerOriginId(41)),
                    },
                ),
                oriented_test_instance_layer(
                    0,
                    0,
                    0,
                    true,
                    RecursorLayerRole::ExitsScope {
                        origin,
                        scope_origin: RecursorProducerOriginId(43),
                        parent_scope: Some(RecursorProducerOriginId(42)),
                    },
                ),
                oriented_test_instance_layer(
                    1,
                    0,
                    0,
                    true,
                    RecursorLayerRole::ExitsScope {
                        origin,
                        scope_origin: RecursorProducerOriginId(44),
                        parent_scope: Some(RecursorProducerOriginId(43)),
                    },
                ),
            ],
        },
        ContinuationCursorId(7),
        None,
        None,
    );
    for layer in &mut invocation.unwind.later_wrappers_in_construction_order[..2] {
        layer.checked_invocation_source = Some(InvocationTemplateRef::SameSccCall(999));
    }
    invocation.selection.checked_invocation_source = None;
    for layer in &mut invocation.unwind.later_wrappers_in_construction_order {
        if layer.semantic_pending {
            layer.checked_invocation_source = None;
        }
    }
    invocation
}

#[test]
fn px8j_owned_scope_deletion_fails_closed_before_another_frame_is_emitted() {
    // RT-DESCENT-RETIRE D8 re-describes this internal-emitter control from the
    // measured first outcome in RT-RECURSOR-TRANSPORT.  The fixture is
    // unobserved by construction on the surviving lane: lowering stops at the
    // checked-root planner invariant before the old owned-scope mutation can
    // be observed.  This is an expectation change, not a repair or a claim
    // that every future frontend shape must refuse here.
    let expression = host_result_closure_match(px8j_layered_recursive_result(1, 1));
    let (exact_result, _exact_trace) =
        px8j_capture_source_trace(&expression, false, "ken_px8j_scope_exact");
    assert!(matches!(
        exact_result,
        Err(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason)))
            if reason == "terminal answer has no affine checked-root authority"
    ), "the owned-scope fixture must retain its measured checked-root refusal");
}
#[test]
fn px8j_all_three_producer_paths_reach_real_consumers() {
    // RT-DESCENT-RETIRE D8 re-describes the obsolete all-path expectation from
    // RT-RECURSOR-TRANSPORT's measured first outcome.  On the surviving lane
    // this fixture has no SourceMachine mint; the composed lifecycle remains
    // the positive.  This pins today's internal-emitter behavior as
    // unobserved-by-construction, not as a claim that SourceMachine can never
    // be reachable for another frontend-produced occurrence.
    let aggregate = RuntimeExpr::Construct {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let expression = host_result_closure_match(recursive_computational_result_depth(2, aggregate));
    let (result, trace) =
        px8j_capture_source_trace(&expression, false, "ken_px8j_live_source_paths");
    result.expect("the composed and source-machine producer paths lower");
    let (origin, cursor) = trace
        .iter()
        .find_map(|event| match event {
            Px8jSourceTraceEvent::Mint {
                path: Px8jProducerPath::Composed,
                origin,
                cursor,
                siblings,
                ..
            } if *siblings > 0 => Some((*origin, *cursor)),
            _ => None,
        })
        .expect("the composed path must mint its recursive IH");
    assert!(trace.iter().any(|event| matches!(
        event,
        Px8jSourceTraceEvent::Install {
            origin: actual_origin,
            selection_cursor,
            ..
        } if *actual_origin == origin && *selection_cursor == cursor
    )));
    assert!(trace.iter().any(|event| matches!(
        event,
        Px8jSourceTraceEvent::Selection { origin: actual } if *actual == origin
    )));
    assert!(!trace.iter().any(|event| matches!(
        event,
        Px8jSourceTraceEvent::Mint {
            path: Px8jProducerPath::SourceMachine,
            ..
        }
    )), "the corrected row-2 outcome has no SourceMachine mint: {trace:#?}");

    let deferred = RuntimeExpr::Match {
        scrutinee: Box::new(px8j_deferred_recursive_field_fixture()),
        cases: [
            "ctor:prelude::Result::Err",
            "ctor:prelude::Result::Ok",
        ]
        .into_iter()
        .map(|constructor| RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 1,
            body: RuntimeExpr::Construct {
                constructor: crate::EXIT_SUCCESS_CONSTRUCTOR.to_string(),
                args: Vec::new(),
            },
        })
        .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "direct deferred HostResult default".to_string(),
        },
    };
    let (result, trace) =
        px8j_capture_source_trace(&deferred, false, "ken_px8j_live_deferred_path");
    result.expect("the deferred-constructor producer path lowers");
    let (origin, cursor) = trace
        .iter()
        .find_map(|event| match event {
            Px8jSourceTraceEvent::Mint {
                path: Px8jProducerPath::DeferredConstructor,
                origin,
                cursor,
                siblings: 1,
                ..
            } => Some((*origin, *cursor)),
            _ => None,
        })
        .expect("the deferred constructor mints its recursive IH");
    assert!(trace.iter().any(|event| matches!(
        event,
        Px8jSourceTraceEvent::DirectConsume {
            origin: actual_origin,
            selection_cursor,
            ..
        } if *actual_origin == origin && *selection_cursor == cursor
    )));
}
/// **`RT-LEXICAL-ROW2-MISSING-MINT` `D0` — the attribution, pinned.**
///
/// Row 2 fails under `B`-only exclusion because
/// `px8j_all_three_producer_paths_reach_real_consumers` finds no `Mint` with
/// `siblings > 0` for one of its two producer paths. The frame gives two
/// candidate causes and asks which: **(i)** the emission point is never reached
/// on the `FunctionizedUnits` path, or **(ii)** it is reached with
/// `case.recursive_positions` empty.
///
/// MEASURED, at this base, under `B`-only exclusion: **cause (i), on the
/// `SourceMachine` path.**
///
/// **The activation denominator is asserted, not assumed.**
/// `b2f_units_declared_in_attempt` is attempt-scoped, so `Some(n)` means *this*
/// compile reached the functionized emission seam and declared `n` units. An
/// absent `Mint` on a compile that never took the lane would prove nothing, and
/// this is what excludes that reading. The unexcluded run answers `None` at the
/// same call, which is the lane actually differing rather than a constant.
///
/// **The discriminator's ground was WRONG, and this block is its replacement.**
/// An earlier version licensed cause (i) on the unexcluded baseline's
/// zero-sibling `SourceMachine` `Mint`: cause (ii) would leave such an event,
/// the baseline has one on this very fixture, so the absence under exclusion
/// must mean *never reached*. **That witness is on the wrong lane.** The same
/// assertion below requires `baseline_declared.is_some() == false` — the
/// baseline is the run that did **not** reach the functionized seam — so it
/// cannot testify about minting behaviour on the lane the excluded run takes.
/// It is not asserted here any more, and it is not a second reason standing
/// beside the right one.
///
/// **What licenses cause (i), part one: the emission site's own structure.**
/// Measured at this base, from the source rather than from the trace. There is
/// exactly one production `SourceMachine` `Mint`, in
/// `lower_source_machine_with_continuation_inner`; the other three carry
/// `Composed` and `DeferredConstructor`. No `case.recursive_positions.is_empty()`
/// guard stands between that arm's case selection and its emission, and the
/// event carries `siblings: case.recursive_positions.len()`.
///
/// One correction to the frame's version of this argument, since verifying it
/// rather than crediting it was the point: the frame offers the guarded
/// `Composed` mint in `lower_carried_computational_match_inner` as *the*
/// contrast, which reads as though guarding were the norm. It is not — the
/// other `Composed` mint is unguarded too. **The argument does not rest on the
/// contrast and must not be stated as though it did.** It rests on the
/// absolute structure at the one `SourceMachine` site: nothing there is
/// conditional on the case being non-empty. ⇒ **An empty case there mints
/// `siblings == 0` rather than minting nothing.** That is cause (ii)'s
/// signature, and the assertion below measures its total absence.
///
/// Between the case selection and that emission are **three** ways out, not the
/// one the frame's licence named: the arity refusal, the malformed-recursive-
/// position refusal, and `computational_ih_slots_for_case(..)?`. All three are
/// `Err`-valued, and the row's defining property is that its compile returns
/// `Ok` — asserted below. So none of them fired, and control reaching that arm
/// would have reached the emission.
///
/// **Part two, and it is the one the baseline could not give: a POSITIVE
/// control on the excluded lane itself.**
/// `D6aRouteEvent::ConsumerRoute { seat: D6aConsumerSeat::SourceMachine, .. }`
/// has exactly one production emission site — inside the **carried** arm of the
/// same computational-scrutinee seat, which breaks out of the block before the
/// specialized selection. Observing it under exclusion says two things at once
/// that no absence can: the seat **was** reached on the `FunctionizedUnits`
/// lane, and the branch it took there is the one that returns before the
/// selection the mint sits behind. **That is the entire claim, and nothing is
/// joined to it.**
///
/// **A second operand stood here and was wrong — recorded because deleting it
/// silently is how it would come back.** It asserted
/// `any(.., CarriedEliminationEntered { .. })`, described as *"the eliminator
/// this seat handed its frame to actually ran."* It measured no such thing.
/// The excluded trace, read in order, is: `ConsumerRoute { seat: Composed }`,
/// then `CarriedEliminationEntered`, then `ConsumerRoute { seat: SourceMachine }`.
/// **The elimination event belongs to the `Composed` seat**, and an unordered
/// `any` over the whole trace credited it to this one. Tightening the match
/// on `static_origin` would **not** have caught it: both seats carry the *same*
/// origin here, so the operand would have stayed green and stayed wrong.
///
/// ⇒ **Two independent `any(..)` observations do not compose into a sequenced
/// claim about one seat.** Saying "reached, and then its eliminator ran" needs
/// the second event to be seat-specific *and* ordered against the first;
/// neither held. The route observation alone is sufficient and honest for what
/// this sentinel needs, so it stands alone. A downstream claim may be added
/// only by something that carries the seat in the event itself.
///
/// The same predicate is read on the baseline too, and required to answer
/// **false** there. That is not a witness and not a licence — it is what keeps
/// the operand from being a constant that would satisfy the tuple while
/// measuring nothing.
///
/// ⇒ Reached, took the carried arm, and no mint of any arity anywhere. **Cause
/// (i), argued on the lane in question and needing no witness from the other
/// one.**
///
/// **WHY THE ATTRIBUTED CAUSE IS NOT REPAIRABLE AT THIS SEAT — `D1`'s
/// measurement, recorded here because it is what the next reader needs.**
///
/// Cause (i) is confirmed above and it is *explained*, which is a different
/// finding from the one `D1` was framed to expect. Measured under `B`-only
/// exclusion on this fixture: the seat is entered **once**, and its scrutinee
/// arrives as `LoweringOperand::Carried` — a runtime word — because the
/// residual the exclusion functionizes is the very thing that produced the
/// compile-time constructor on the descent lane, where the same seat is
/// entered **three** times with `Specialized(Lowered::Constructor)`. The
/// carried arm is therefore not a branch taken *instead of* an installation
/// step; it is the arm whose own contract says a carried value must not be
/// asked for a compile-time constructor template.
///
/// ⇒ **The number of `SourceMachine` installations this lane requires for this
/// occurrence is zero**, and the "one mint versus four" gap is the descent
/// lane's compile-time unrolling, not three absent steps. Two alternatives
/// were measured and closed rather than argued: the eliminator's origin is
/// **not** among the units this compile plans, so no missing route-to-a-unit
/// explains it; and the `Composed` mint that does survive exclusion is emitted
/// from a **different site** than the baseline's, so even the passing half of
/// the row passes by a substituted mechanism.
///
/// **Promise class: TRANSITION SENTINEL.** These assertions describe a defect,
/// so they are written to go **red when `D1` repairs it** — that red is the
/// point and must not be satisfied by relaxing the bound. **Retiring event:**
/// whatever settles row 2 — which the paragraph above means is no longer
/// certainly a repair at the attributed cause, and may instead be a ruling that
/// relocates the requirement. Either way this test's job passes to
/// `px8j_all_three_producer_paths_reach_real_consumers` itself, which is the
/// real acceptance control, and this sentinel is deleted rather than adjusted.
#[test]
fn px8j_siblings_share_an_origin_and_nested_ih_gets_a_child_origin() {
    // RT-DESCENT-RETIRE D8 pins RT-RECURSOR-TRANSPORT's measured structural
    // absence.  The Module rendering below is the concrete form of the table's
    // "recursive position 1 has no projected continuation worker" outcome:
    // the missing worker is exactly why the IH prefix cannot be built.
    let expression =
        host_result_closure_match(px8j_recursive_sibling_result(1, 2, px8j_aggregate_result()));
    let (result, _trace) =
        px8j_capture_source_trace(&expression, false, "ken_px8j_live_sibling_origins");
    assert!(matches!(
        result,
        Err(CraneliftBackendError::Backend(BackendFailure::Module(reason)))
            if reason == "the selected case has a recursive position 1 that the continuation \
                specialization projects no worker for, so its induction-hypothesis prefix \
                cannot be built"
    ), "the two-sibling fixture must retain its measured missing-worker outcome");
}
/// **`RT-LEXICAL-ROW2-MISSING-MINT` successor measurement — is the recursive IH
/// installed and consumed on the functionized lane, or absent?**
///
/// **The subject, and it is the whole of it:** under `B`-only exclusion, is a
/// recursive induction hypothesis minted, installed and consumed on the lane
/// the fixture then takes — or is it absent?
///
/// Directed by the Steward ruling at `evt_26cb49zckgq4f`, merged as PR #1953,
/// which closed `D1` with no production repair **for row 2's occurrence on the
/// `FunctionizedUnits` lane**. The frame's `D1` section carries the closure and
/// its measurement; read it there.
///
/// **Cited, not restated, and the qualifier is the whole of the care.** That
/// result is about *this occurrence on this lane*. It is **not** a statement
/// about the `SourceMachine` producer path, and the question of whether it
/// generalizes is live — a durable doc comment travels further than the
/// handback it came from and travels without its bounds, so the bound is
/// written into the sentence rather than left to the reader.
///
/// ⇒ **It is deliberately not used to prefigure what follows.** The question
/// below is open and either answer is a real outcome: absent is as admissible
/// a result as installed-and-consumed, and nothing here should make one of
/// them read as expected.
///
/// MEASURED, at this base: on the excluded lane the `Composed` (carried) route
/// **mints, installs and consumes** one recursive IH; the `SourceMachine` route
/// does none of the three. On the baseline both routes do all three.
///
/// **This artifact reports that and stops.** It takes no position on why the
/// lane differs, on what any repair would owe, or on what the row's observation
/// site ought to assert. Those are live questions elsewhere and an evidence
/// candidate is the wrong place to settle them — a conclusion parked in a test's
/// doc block is durable, unreviewed by the ring that owns it, and inherited by
/// whoever reads the test next.
///
/// **Minted, installed, and consumed are kept as three observations and are
/// never collapsed.** A lane that minted and never installed would satisfy a
/// merged predicate while the semantics it protects were gone, and this row is
/// exactly where that would hide. `Install` and `DirectConsume` are separate
/// production sites — `install_recursor_invocation` and the direct-consumption
/// seat — so both are read, and neither substitutes for the other.
///
/// **The predicate is proven non-constant by its own fourth cell.** The join is
/// a `Mint` with `siblings > 0`, then an `Install` and a `DirectConsume`
/// matched on **that mint's own `origin` and `cursor`** — those two events and
/// no others. It is run over both producer paths on both lanes, and answers
/// **true** three times and **false** once, so it is demonstrably capable of
/// answering either way rather than being a shape any trace satisfies.
///
/// `Carrier` and `Selection` are read too, but **separately and at different
/// strength**, because the variants are not shaped alike: `Carrier` carries an
/// origin and a cursor, `Selection` carries **no cursor at all**. So the
/// `Selection` check is origin-only, and is named as the weaker join it is
/// rather than folded into a sentence claiming all four match on both keys.
///
/// **THE IDENTITY GAP, STATED RATHER THAN PAPERED OVER.** The question names an
/// *occurrence*, and occurrences are named by `StaticOriginId`. **No
/// `Px8jSourceTraceEvent` variant carries one** — every field is a
/// `RecursorProducerOriginId` or a `ContinuationCursorId`. So this test cannot
/// key its answer to row 2's occurrence, and does not claim to. What ties them
/// is weaker and is written down here so nobody upgrades it by reading:
///
/// - The excluded compile contains **exactly one** mint that is both installed
///   and consumed, so there is no second candidate it could be about. That is
///   **counted across every producer path and asserted** below, not inferred
///   from the per-path matrix — a per-path existential answers "did this path
///   have one" and is structurally blind to a second lifecycle on another
///   path, which is precisely the population a singleton claim ranges over.
/// - The `D6a` route trace, which *does* carry `static_origin`, shows exactly
///   one `SourceMachine` seat and exactly one carried elimination on that lane,
///   at the **same** origin. That is asserted below rather than left here,
///   because a claim in a comment is exempt from execution.
///
/// Note what the second bullet does **not** say. The carried elimination is the
/// `Composed` seat's, not the `SourceMachine` seat's, and reading it as the
/// latter is the easy mistake here because the two carry the same origin.
/// **Same origin means same occurrence, not same seat**, and same occurrence is
/// all that is claimed.
///
/// ⇒ The join is **by uniqueness across two traces, not by a shared key**.
/// Adding a key would mean changing the trace machinery, which this branch is
/// forbidden to do; the honest move is to name the limit.
///
/// **This measures ONE occurrence in ONE fixture and generalizes to neither the
/// producer path nor the lane.** The fixture builds its subject with
/// `recursive_computational_result_depth(2, ..)`, so a recursive occurrence is
/// the only kind it ever constructs. Whether the `SourceMachine` recursive-IH
/// producer has any input after retirement is a different question on different
/// evidence, and this test is not evidence for it.
///
/// One datum worth keeping in view: the baseline's fourth `SourceMachine` mint
/// carries `siblings == 0` and has **no** `Carrier`, `Install` or
/// `DirectConsume` after it. That is the shape of a mint with nothing to
/// install, and it is why the join filters on `siblings > 0` rather than
/// treating every mint as an obligation.
///
/// **Promise class: durable invariant.** The surviving lane must mint, install,
/// carry, select, and consume
/// the recursive IH through the composed producer path.
#[test]
fn row2_functionized_lane_installs_and_consumes_the_recursive_ih() {
    /// Every `(origin, cursor)` a mint with `siblings > 0` was issued at, with
    /// the producer path that issued it, in trace order. **Across all paths**
    /// -- this is what a per-path `find_map` cannot give, and the singleton
    /// claim below needs a population rather than a first match.
    fn mints_with_siblings(
        trace: &[Px8jSourceTraceEvent],
    ) -> Vec<(
        Px8jProducerPath,
        RecursorProducerOriginId,
        ContinuationCursorId,
    )> {
        trace
            .iter()
            .filter_map(|event| match event {
                Px8jSourceTraceEvent::Mint {
                    path,
                    origin,
                    cursor,
                    siblings,
                    ..
                } if *siblings > 0 => Some((*path, *origin, *cursor)),
                _ => None,
            })
            .collect()
    }
    fn installed(
        trace: &[Px8jSourceTraceEvent],
        origin: RecursorProducerOriginId,
        cursor: ContinuationCursorId,
    ) -> bool {
        trace.iter().any(|event| {
            matches!(
                event,
                Px8jSourceTraceEvent::Install {
                    origin: actual,
                    selection_cursor,
                    ..
                } if *actual == origin && *selection_cursor == cursor
            )
        })
    }
    fn consumed(
        trace: &[Px8jSourceTraceEvent],
        origin: RecursorProducerOriginId,
        cursor: ContinuationCursorId,
    ) -> bool {
        trace.iter().any(|event| {
            matches!(
                event,
                Px8jSourceTraceEvent::DirectConsume {
                    origin: actual,
                    selection_cursor,
                    ..
                } if *actual == origin && *selection_cursor == cursor
            )
        })
    }
    /// Which producer paths issued a mint that was **both** installed and
    /// consumed, in trace order. The counted population the singleton claim
    /// rests on, so it ranges over every path rather than one.
    fn installed_and_consumed_paths(trace: &[Px8jSourceTraceEvent]) -> Vec<Px8jProducerPath> {
        mints_with_siblings(trace)
            .into_iter()
            .filter(|(_, origin, cursor)| {
                installed(trace, *origin, *cursor) && consumed(trace, *origin, *cursor)
            })
            .map(|(path, _, _)| path)
            .collect()
    }
    /// `(minted, installed, consumed)` for `path`, joined on the mint's own
    /// `origin` and `cursor`. Deliberately three fields: a lane that minted
    /// and never installed must be distinguishable from one that did both.
    fn lifecycle(trace: &[Px8jSourceTraceEvent], path: Px8jProducerPath) -> (bool, bool, bool) {
        let Some((_, origin, cursor)) = mints_with_siblings(trace)
            .into_iter()
            .find(|(actual, _, _)| *actual == path)
        else {
            return (false, false, false);
        };
        (
            true,
            installed(trace, origin, cursor),
            consumed(trace, origin, cursor),
        )
    }
    let aggregate = RuntimeExpr::Construct {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let expression = host_result_closure_match(recursive_computational_result_depth(2, aggregate));

    // `Exact` is the identity perturbation, used for its other effect: it
    // clears the route trace on the way in, so what is read back is this
    // compile's events and not the baseline's residue.
    let (result, trace, routes) =
        with_d6a_route_mutation(D6aRouteMutation::Exact, || {
            let (result, trace) =
                px8j_capture_source_trace(&expression, false, "ken_row2_ih_functionized");
            (result, trace, d6a_route_trace())
        });
    result.expect("the surviving functionized lane must compile row 2");

    assert_eq!(
        (
            lifecycle(&trace, Px8jProducerPath::Composed),
            lifecycle(&trace, Px8jProducerPath::SourceMachine),
        ),
        (
            (true, true, true),
            (false, false, false),
        ),
        "the surviving lane must mint, install, and consume the recursive IH \
         through Composed, never through SourceMachine: {trace:#?}"
    );

    // ── The SINGLETON, counted rather than asserted in prose ──
    //
    // The four-cell matrix above is per-path and existential: it answers "did
    // THIS path have an installed-and-consumed mint", and a `find` over one
    // path cannot see a second lifecycle on another. The bridge below needs
    // something stronger -- that there is no OTHER lifecycle the excluded
    // lane's could be confused with -- so the population is counted here,
    // across every producer path, before anything is built on it.
    assert_eq!(
        installed_and_consumed_paths(&trace),
        vec![Px8jProducerPath::Composed],
        "every mint with siblings > 0 that was both installed and consumed, in trace order \
         and across all producer paths. The excluded lane's list must be a SINGLETON for the \
         identity bridge below to mean anything -- if a second lifecycle existed, the bridge \
         would not say which one it was about. trace={trace:#?}"
    );

    // ── Carrier and Selection, each on the keys ITS OWN variant exposes ──
    //
    // These two are read separately from the matrix, and with different
    // strength, because the events are not shaped alike:
    //
    //   Carrier   { origin, cursor, .. }  -- joins on BOTH
    //   Selection { origin }              -- carries NO cursor at all
    //
    // So the `Selection` check is origin-only and is weaker evidence than the
    // others by construction. Saying "matched on origin and cursor" of all
    // four would have been false of this one and unfalsifiable to a reader who
    // did not go read the enum.
    // Selected by the SAME predicate the singleton was counted with, not by
    // trace position. `.next()` on the raw mint list would take the first mint
    // with siblings, which is only the same event if no un-installed mint
    // precedes it -- a fact this test does not assert and should not depend on.
    let (_, singleton_origin, singleton_cursor) = mints_with_siblings(&trace)
        .into_iter()
        .find(|(_, origin, cursor)| {
            installed(&trace, *origin, *cursor) && consumed(&trace, *origin, *cursor)
        })
        .expect("the singleton counted above");
    let carrier_matched = trace.iter().any(|event| {
        matches!(
            event,
            Px8jSourceTraceEvent::Carrier { origin, cursor, .. }
                if *origin == singleton_origin && *cursor == singleton_cursor
        )
    });
    let selection_matched_on_origin_only = trace.iter().any(|event| {
        matches!(
            event,
            Px8jSourceTraceEvent::Selection { origin } if *origin == singleton_origin
        )
    });
    assert_eq!(
        (carrier_matched, selection_matched_on_origin_only),
        (true, true),
        "the excluded lane's single installed-and-consumed mint also carries and is selected. \
         Carrier is joined on origin AND cursor; Selection on origin ONLY, because that \
         variant exposes no cursor -- a weaker join, named as one. trace={trace:#?}"
    );

    // ── The identity tie, asserted rather than left in the doc block ──
    //
    // The lifecycle above is keyed on producer origins and cursors, which name
    // no source occurrence. This is what connects it to row 2's occurrence,
    // and it is deliberately the WEAKER of the two available statements.
    //
    // It says: the SourceMachine seat and the carried elimination that actually
    // runs are at the SAME `static_origin`, and there is exactly one of each,
    // so there is no second elimination the sole IH lifecycle could belong to.
    //
    // It does NOT say the elimination is the SourceMachine seat's -- it is the
    // Composed seat's. Reading it as the former is the easy mistake, because
    // the two carry the same origin. Same origin means same occurrence, not
    // same seat.
    let source_machine_seat_origins = routes
        .iter()
        .filter_map(|event| match event {
            D6aRouteEvent::ConsumerRoute {
                seat: D6aConsumerSeat::SourceMachine,
                static_origin,
                ..
            } => Some(*static_origin),
            _ => None,
        })
        .collect::<Vec<_>>();
    let carried_elimination_origins = routes
        .iter()
        .filter_map(|event| match event {
            D6aRouteEvent::CarriedEliminationEntered { static_origin, .. } => Some(*static_origin),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        (
            source_machine_seat_origins.len(),
            carried_elimination_origins.len(),
            source_machine_seat_origins.first() == carried_elimination_origins.first(),
        ),
        (1, 1, true),
        "the identity tie: on the excluded lane exactly one SourceMachine seat and exactly \
         one carried elimination must run, and they must be at the same static origin -- \
         otherwise the sole IH lifecycle above cannot be attributed to row 2's occurrence. \
         source_machine_seats={source_machine_seat_origins:?} \
         carried_eliminations={carried_elimination_origins:?}"
    );
}
pub(in crate::cranelift_backend::lowering) fn px8j_capture_source_trace(
    expression: &RuntimeExpr,
    delete_owned_scope: bool,
    symbol: &str,
) -> (
    Result<CraneliftObjectArtifact, CraneliftBackendError>,
    Vec<Px8jSourceTraceEvent>,
) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            PX8J_DELETE_OWNED_SELECTED_SCOPE.set(false);
            PX8J_SOURCE_TRACE.with(|trace| trace.borrow_mut().clear());
        }
    }
    PX8J_SOURCE_TRACE.with(|trace| trace.borrow_mut().clear());
    PX8J_DELETE_OWNED_SELECTED_SCOPE.set(delete_owned_scope);
    let _reset = Reset;
    let result = emit_process_entrypoint_object_with_cranelift(expression, symbol);
    let trace = PX8J_SOURCE_TRACE.with(|trace| trace.borrow().clone());
    (result, trace)
}
#[test]
fn oriented_phase_misclassification_recovers_endpoint_and_missing_semantic_rejections() {
    let plan = oriented_test_ih_plan();
    let mut replayed = oriented_five_control_invocation();
    replayed.unwind.later_wrappers_in_construction_order[0].semantic_pending = true;
    replayed.open_control_obligations = open_control_obligations(&replayed.unwind);
    let replayed = match compose_oriented_subcontinuation(
        Some(&plan),
        None,
        ContinuationActivationId(8),
        replayed,
        Vec::new(),
    ) {
        Ok(_) => panic!("an inherited open scope cannot replay its semantic transformer"),
        Err(error) => error,
    };
    assert!(matches!(
        replayed,
        CraneliftBackendError::Unsupported(UnsupportedLowering { reason, .. })
            if reason.contains("exact invocation-local tree")
    ));

    let mut omitted = oriented_five_control_invocation();
    omitted.selection.semantic_pending = false;
    let omitted = match compose_oriented_subcontinuation(
        Some(&plan),
        None,
        ContinuationActivationId(8),
        omitted,
        Vec::new(),
    ) {
        Ok(_) => panic!("a pending selection cannot be omitted from semantic work"),
        Err(error) => error,
    };
    assert!(matches!(
        omitted,
        CraneliftBackendError::Unsupported(UnsupportedLowering { reason, .. })
            if reason == "pending selection was misclassified as control-only"
    ));
}
#[test]
fn nested_computational_inner_missing_selects_exact_inner_default() {
    let inner_cases = vec![crate::RuntimeComputationalMatchCase {
        constructor: "ctor:fixture::Inner::Hit".to_string(),
        argument_binders: 0,
        recursive_positions: Vec::new(),
        body: RuntimeExpr::Value(RuntimeValue::Int((1).into())),
    }];
    let outer_cases = vec![crate::RuntimeComputationalMatchCase {
        constructor: "ctor:fixture::Outer::Hit".to_string(),
        argument_binders: 0,
        recursive_positions: Vec::new(),
        body: RuntimeExpr::Value(RuntimeValue::Int((2).into())),
    }];
    let inner_default = RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "px7n exact inner default".to_string(),
    };
    let outer_default = RuntimeTrap {
        code: RuntimeTrapCode::ExplicitTrap,
        message: "px7n exact outer default".to_string(),
    };
    let frames = [
        ComputationalEliminatorFrame {
            cases: &inner_cases,
            default: &inner_default,
            env: &[],
            static_origin: inert_test_static_origin(),
            retained_scrutinee_index: None,
            deferred_constructor_case: None,
            provenance: RecursorFrameProvenance(1),
            checked_frame_id: None,
            checked_invocation_id: None,
            checked_invocation_source: None,
            checked_invocation_depth: 0,
            answer_route: SourceComputationalAnswerRoute::DirectScrutinee,
        },
        ComputationalEliminatorFrame {
            cases: &outer_cases,
            default: &outer_default,
            env: &[],
            static_origin: inert_test_static_origin(),
            retained_scrutinee_index: None,
            deferred_constructor_case: None,
            provenance: RecursorFrameProvenance(0),
            checked_frame_id: None,
            checked_invocation_id: None,
            checked_invocation_source: None,
            checked_invocation_depth: 0,
            answer_route: SourceComputationalAnswerRoute::DirectScrutinee,
        },
    ];

    let trap = match select_computational_case(&frames, "ctor:fixture::Inner::Missing") {
        Err(trap) => trap,
        Ok(_) => panic!("a missing inner case must select the inner frame default"),
    };
    assert_eq!(trap.code, RuntimeTrapCode::PatternMatchFailure);
    assert_eq!(trap.message, "px7n exact inner default");
    assert_ne!(trap.code, outer_default.code);
    assert_ne!(trap.message, outer_default.message);
}
#[test]
fn unmarked_equal_shape_frame_cannot_consume_retained_join_site() {
    let cases = vec![RuntimeMatchCase {
        constructor: "ctor:fixture::PX8H::Only".to_string(),
        binders: 0,
        body: RuntimeExpr::Value(RuntimeValue::Int((7).into())),
    }];
    let default = RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "px8h unmarked equal-shape default".to_string(),
    };
    let fingerprint = crate::compiler_private_ordinary_match_frame_fingerprint(&cases, &default);
    let expression = RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::PX8H::Only".to_string(),
            args: Vec::new(),
        }),
        cases,
        default,
    };
    let result = compile_expr_into_module(
        new_object_module("px8h-unmarked-equal-shape").unwrap(),
        "ken_px8h_unmarked_equal_shape",
        Linkage::Export,
        &expression,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        false,
        None,
        Some(crate::NativeJoinPlanV1 {
            representation_rule_version: crate::NativeJoinPlanV1::REPRESENTATION_RULE_VERSION,
            sites: vec![self_consistent_join_site(51, fingerprint)],
        }),
        None,
    );
    let error = match result {
        Ok(_) => panic!("an unmarked equal-shape frame must not consume a plan row"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "NativeJoinPlanV1",
            reason,
        }) if reason.contains("unconsumed or orphan site")
    ));
}
pub(in crate::cranelift_backend::lowering) fn px8j_scope_chain_observation_result(transform_layers: usize, input_depth: usize) -> RuntimeExpr {
    let tree_constructor =
        |_layer: usize, constructor: &str| format!("ctor:fixture::PX8JScopeTree::{constructor}");
    fn child(depth: usize, node: &str, leaf: &str) -> RuntimeExpr {
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["unit".to_string()],
            body: Box::new(if depth == 0 {
                RuntimeExpr::Construct {
                    constructor: leaf.to_string(),
                    args: Vec::new(),
                }
            } else {
                RuntimeExpr::Construct {
                    constructor: node.to_string(),
                    args: vec![child(depth - 1, node, leaf)],
                }
            }),
        }
    }
    let input_node = tree_constructor(0, "Node");
    let input_leaf = tree_constructor(0, "Leaf");
    let mut producer = RuntimeExpr::Construct {
        constructor: input_node.clone(),
        args: vec![child(input_depth, &input_node, &input_leaf)],
    };
    for layer in 0..transform_layers {
        producer = RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(producer),
            cases: vec![
                crate::RuntimeComputationalMatchCase {
                    constructor: tree_constructor(layer, "Node"),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: RuntimeExpr::Construct {
                        constructor: tree_constructor(layer + 1, "Node"),
                        args: vec![RuntimeExpr::Var(0)],
                    },
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: tree_constructor(layer, "Leaf"),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: RuntimeExpr::Construct {
                        constructor: tree_constructor(layer + 1, "Leaf"),
                        args: Vec::new(),
                    },
                },
            ],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: format!("PX8-J transform {layer} default"),
            },
        };
    }
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(producer),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: tree_constructor(transform_layers, "Node"),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::Call {
                        callee: Box::new(RuntimeExpr::Var(0)),
                        args: vec![RuntimeExpr::Construct {
                            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                            args: Vec::new(),
                        }],
                    }),
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: tree_constructor(transform_layers, "Node"),
                        args: vec![child(
                            0,
                            &tree_constructor(transform_layers, "Node"),
                            &tree_constructor(transform_layers, "Leaf"),
                        )],
                    }),
                },
            },
            crate::RuntimeComputationalMatchCase {
                constructor: tree_constructor(transform_layers, "Leaf"),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::Construct {
                    constructor: tree_constructor(transform_layers, "Leaf"),
                    args: Vec::new(),
                },
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "PX8-J terminal transform default".to_string(),
        },
    }
}
pub(in crate::cranelift_backend::lowering) fn px8j_recursive_sibling_result(
    depth: usize,
    siblings: usize,
    leaf_body: RuntimeExpr,
) -> RuntimeExpr {
    assert!(siblings > 0);
    let node = "ctor:fixture::PX8JSiblingTree::Node";
    let leaf = "ctor:fixture::PX8JSiblingTree::Leaf";
    fn child(depth: usize, siblings: usize, node: &str, leaf: &str) -> RuntimeExpr {
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["unit".to_string()],
            body: Box::new(if depth == 0 {
                RuntimeExpr::Construct {
                    constructor: leaf.to_string(),
                    args: Vec::new(),
                }
            } else {
                RuntimeExpr::Construct {
                    constructor: node.to_string(),
                    args: (0..siblings)
                        .map(|_| child(depth - 1, siblings, node, leaf))
                        .collect(),
                }
            }),
        }
    }
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: node.to_string(),
            args: (0..siblings)
                .map(|_| child(depth, siblings, node, leaf))
                .collect(),
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: node.to_string(),
                argument_binders: siblings,
                recursive_positions: (0..siblings).collect(),
                body: if siblings == 1 {
                    RuntimeExpr::Call {
                        callee: Box::new(RuntimeExpr::Var(0)),
                        args: vec![RuntimeExpr::Construct {
                            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                            args: Vec::new(),
                        }],
                    }
                } else {
                    RuntimeExpr::Let {
                        value: Box::new(RuntimeExpr::Call {
                            callee: Box::new(RuntimeExpr::Var(0)),
                            args: vec![RuntimeExpr::Construct {
                                constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                                args: Vec::new(),
                            }],
                        }),
                        body: Box::new(RuntimeExpr::Call {
                            callee: Box::new(RuntimeExpr::Var(2)),
                            args: vec![RuntimeExpr::Construct {
                                constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                                args: Vec::new(),
                            }],
                        }),
                    }
                },
            },
            crate::RuntimeComputationalMatchCase {
                constructor: leaf.to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: leaf_body,
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "PX8-J sibling tree default".to_string(),
        },
    }
}
fn oriented_test_invocation() -> RecursorInvocationSegment {
    let origin = RecursorProducerOriginId(40);
    RecursorInvocationSegment::new(
        origin,
        0,
        oriented_test_layer(0, RecursorLayerRole::SelectsOccurrence { origin }),
        RecursorUnwindStack {
            later_wrappers_in_construction_order: vec![
                oriented_test_layer(
                    1,
                    RecursorLayerRole::ExitsScope {
                        origin,
                        scope_origin: RecursorProducerOriginId(41),
                        parent_scope: None,
                    },
                ),
                oriented_test_layer(
                    2,
                    RecursorLayerRole::ExitsScope {
                        origin,
                        scope_origin: RecursorProducerOriginId(42),
                        parent_scope: Some(RecursorProducerOriginId(41)),
                    },
                ),
            ],
        },
        ContinuationCursorId(7),
        None,
        None,
    )
}
#[test]
fn px8j_one_two_three_scope_segments_reach_selection_hole_and_unwind() {
    // RT-DESCENT-RETIRE D8 changes this internal-emitter expectation to the
    // first FunctionizedUnits outcome measured in RT-RECURSOR-TRANSPORT.  Each
    // fixture is unobserved beyond the StaticWorkerBinding conservation wall;
    // this is an expectation change, not a relaxation of that wall.
    for depth in 1..=3 {
        let expression = host_result_closure_match(px8j_scope_chain_observation_result(depth, 0));
        let (result, _trace) = px8j_capture_source_trace(
            &expression,
            false,
            &format!("ken_px8j_live_scope_depth_{depth}"),
        );
        assert!(matches!(
            result,
            Err(CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "StaticWorkerBinding",
                reason,
            })) if reason.contains("this recognition's own transport never reaches a consumer at an exact-Var call")
                && reason.contains("has no runtime representation")
        ), "scope depth {depth} must retain its measured conservation refusal");
    }
}
#[test]
fn px8j_selected_scope_partitions_differ_across_the_real_return_hole() {
    // RT-DESCENT-RETIRE D8 re-describes both fixtures by the measured first
    // outcomes in RT-RECURSOR-TRANSPORT.  Before-hole stops at the in-flight
    // activation rule; after-hole stops at static-worker conservation.  The
    // old partition is therefore unobserved by construction on this lane.
    let before = host_result_closure_match(px8j_equal_payload_hole_placement(
        Px8jSelectedScopePlacement::BeforeReturnHole,
    ));
    let after = host_result_closure_match(px8j_equal_payload_hole_placement(
        Px8jSelectedScopePlacement::AfterReturnHole,
    ));
    let (before_result, _before_trace) =
        px8j_capture_source_trace(&before, false, "ken_px8j_scope_before_hole");
    let (after_result, _after_trace) =
        px8j_capture_source_trace(&after, false, "ken_px8j_scope_after_hole");
    assert!(matches!(
        before_result,
        Err(CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "ComputationalMatch",
            reason,
        })) if reason == "a computational recursor closure names an in-flight activation, not a transferable value"
    ));
    assert!(matches!(
        after_result,
        Err(CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "StaticWorkerBinding",
            reason,
        })) if reason.contains("this recognition's own transport never reaches a consumer at an exact-Var call")
            && reason.contains("has no runtime representation")
    ));
}
#[test]
fn nested_computational_outer_missing_selects_exact_outer_default() {
    let inner_cases = vec![crate::RuntimeComputationalMatchCase {
        constructor: "ctor:fixture::Inner::Hit".to_string(),
        argument_binders: 0,
        recursive_positions: Vec::new(),
        body: RuntimeExpr::Value(RuntimeValue::Int((1).into())),
    }];
    let outer_cases = vec![crate::RuntimeComputationalMatchCase {
        constructor: "ctor:fixture::Outer::Hit".to_string(),
        argument_binders: 0,
        recursive_positions: Vec::new(),
        body: RuntimeExpr::Value(RuntimeValue::Int((2).into())),
    }];
    let inner_default = RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "px7n exact inner default".to_string(),
    };
    let outer_default = RuntimeTrap {
        code: RuntimeTrapCode::ExplicitTrap,
        message: "px7n exact outer default".to_string(),
    };
    let frames = [
        ComputationalEliminatorFrame {
            cases: &inner_cases,
            default: &inner_default,
            env: &[],
            static_origin: inert_test_static_origin(),
            retained_scrutinee_index: None,
            deferred_constructor_case: None,
            provenance: RecursorFrameProvenance(1),
            checked_frame_id: None,
            checked_invocation_id: None,
            checked_invocation_source: None,
            checked_invocation_depth: 0,
            answer_route: SourceComputationalAnswerRoute::DirectScrutinee,
        },
        ComputationalEliminatorFrame {
            cases: &outer_cases,
            default: &outer_default,
            env: &[],
            static_origin: inert_test_static_origin(),
            retained_scrutinee_index: None,
            deferred_constructor_case: None,
            provenance: RecursorFrameProvenance(0),
            checked_frame_id: None,
            checked_invocation_id: None,
            checked_invocation_source: None,
            checked_invocation_depth: 0,
            answer_route: SourceComputationalAnswerRoute::DirectScrutinee,
        },
    ];

    let (_, _, outer_frames) = select_computational_case(&frames, "ctor:fixture::Inner::Hit")
        .expect("the inner case succeeds before the outer miss");
    let trap = match select_computational_case(outer_frames, "ctor:fixture::Outer::Missing") {
        Err(trap) => trap,
        Ok(_) => panic!("a missing outer case must select the outer frame default"),
    };
    assert_eq!(trap.code, RuntimeTrapCode::ExplicitTrap);
    assert_eq!(trap.message, "px7n exact outer default");
    assert_ne!(trap.code, inner_default.code);
    assert_ne!(trap.message, inner_default.message);
}
#[test]
fn distinguished_root_cannot_discharge_missing_match_site_marker() {
    let seed_env = NativeSeedEnvironment::empty();
    let mut lowering = Lowering {
        seed_env: &seed_env,
        declarations: BTreeMap::new(),
        static_transition_plan: inert_test_plan(),
        result_table: BTreeMap::new(),
        next_token: 0,
        next_recursor_frame_provenance: 0,
        next_recursor_producer_origin: 0,
        next_continuation_activation: 0,
        next_continuation_cursor: 0,
        next_source_join: 0,
        next_source_predecessor: 0,
        live_source_continuations: 0,
        carried_suffix_reentries: 0,
        source_control_root: None,
        active_oriented_semantic_regions: 0,
        active_carried_computational_eliminations: Vec::new(),
        native_join_plan: Some(crate::NativeJoinPlanV1 {
            representation_rule_version: crate::NativeJoinPlanV1::REPRESENTATION_RULE_VERSION,
            sites: vec![self_consistent_root_join_site(0)],
        }),
        consumed_join_sites: BTreeSet::new(),
        root_terminal_authority: None,
        active_join_site: Some(41),
        oriented_subcontinuation_plan: None,
        consumed_subcontinuation_frames: BTreeSet::new(),
        active_subcontinuation_frame: None,
        consumed_recursive_call_templates: BTreeSet::new(),
        pending_recursive_call: None,
        pending_computational_ih_call: None,
        active_recursive_invocations: Vec::new(),
        next_recursive_invocation_instance: 1,
        dynamic_splice_edges: BTreeMap::new(),
        next_dynamic_splice_edge: 1,
        assumptions: BTreeSet::new(),
        unsupported: Vec::new(),
        continuation_claims: None,
        fusion_compositions: None,
        static_worker_fields: Default::default(),
        fusion_claims: None,
        fused_consumer_authority: None,
        continuation_candidates: None,
        checked_call_ledger: None,
        defining_unit: None,
        defining_emission_owner: None,
        defining_function_id: None,
        aggregate_allocations: None,
        host_effect_seats: None,
        process_object: false,
        process_symbols: crate::NativeProcessSymbols::legacy_prelude(),
        // ⛔ `None` — a bare `Lowering` fixture emits into no module, so it has
        // no callable carrier refs. The `Carried` routes fail closed on this
        // rather than silently taking the `Specialized` path.
        native_int_mutation: NativeIntLoweringMutation::Exact,
        bounded_nat_mutation: BoundedNatLoweringMutation::Exact,
        function_local: FunctionLocalRefs {
            defining_abi_operands: Vec::new(),
            defining_abi_slot_kinds: Vec::new(),
            context_calls: BTreeMap::new(),
            worker_templates: BTreeMap::new(),
            generated_context_captures: None,
            constructed_context_frame: None,
            seed_material: crate::cranelift_backend::lowering::seed_material::SeedMaterialRefs::none_for_tests(),
            host_dispatch: None,
            host_dispatch_context: None,
            services_pointer: None,
            native_int_arena: None,
            boundary_arena: None,
            native_int_binop: None,
            native_int_compare: None,
            native_int_intern: None,
            native_int_narrow: None,
            native_int_export: None,
            native_int_export_parts: None,
            native_int_resolve: None,
            native_int_tags: BTreeMap::new(),
            unit_calls: BTreeMap::new(),
            worker_calls: BTreeMap::new(),
            raw_worker_calls: BTreeMap::new(),
            continuation_calls: BTreeMap::new(),
            continuation_emissions: BTreeMap::new(),
            checked_ih_transport_emissions: Vec::new(),
            pending_composed_discharges: Vec::new(),
            composed_discharges: BTreeMap::new(),
            declaration_calls: BTreeMap::new(),
            trap_exit: None,
            terminal_result_origins: BTreeSet::new(),
            consumed_join_origins: BTreeSet::new(),
            dispositioned_join_origins: BTreeSet::new(),
            join_disposition_finalized: false,
            final_reachable_join_origins: BTreeSet::new(),
            materialized_join_blocks: BTreeMap::new(),
            emission_reachable_match_cases: BTreeMap::new(),
            boundary_carrier: None,
        },
    };
    let error = lowering
        .planned_join_site_for_frame(EliminatorFrame::InvocationReturn)
        .expect_err("the distinguished root must not discharge an unrelated live marker");
    assert!(
        matches!(
            error,
            CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "NativeJoinPlanV1",
                ref reason,
            }) if reason.contains("root cannot consume an active match occurrence marker")
        ),
        "{error:?}"
    );
    assert_eq!(lowering.active_join_site, Some(41));
    assert!(lowering.consumed_join_sites.is_empty());
}
#[test]
fn oriented_segment_keeps_semantic_and_control_axes_independent() {
    let installed = compose_oriented_subcontinuation(
        Some(&oriented_test_plan()),
        None,
        ContinuationActivationId(8),
        oriented_test_invocation(),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(
        installed
            .semantic_frames
            .iter()
            .map(|frame| frame.checked_frame_id.unwrap())
            .collect::<Vec<_>>(),
        vec![2, 1, 0],
        "checked composition order is p2, p1, p0"
    );
    assert_eq!(
        installed
            .control_ledger
            .iter()
            .map(|entry| entry.frame_id.unwrap())
            .collect::<Vec<_>>(),
        vec![0, 2, 1],
        "delimiter order remains independently o0, o4, o3"
    );
}
pub(in crate::cranelift_backend::lowering) fn px8j_aggregate_result() -> RuntimeExpr {
    RuntimeExpr::Construct {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    }
}
#[test]
fn oriented_fresh_ih_semantics_retain_all_inherited_control_obligations() {
    let installed = compose_oriented_subcontinuation(
        Some(&oriented_test_ih_plan()),
        None,
        ContinuationActivationId(8),
        oriented_five_control_invocation(),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(
        installed
            .semantic_frames
            .iter()
            .map(|frame| {
                (
                    frame.checked_invocation_id.unwrap(),
                    frame.checked_frame_id.unwrap(),
                )
            })
            .collect::<Vec<_>>(),
        vec![(0, 2), (0, 1), (0, 0)],
    );
    assert_eq!(installed.control_ledger.len(), 5);
    assert_eq!(
        installed
            .control_ledger
            .iter()
            .filter(|entry| matches!(entry.role, RecursorLayerRole::ExitsScope { .. }))
            .count(),
        4,
    );
}
#[test]
fn checked_join_marker_without_exact_plan_site_rejects_before_emission() {
    let expression = RuntimeExpr::CheckedJoinSite {
        site_id: 41,
        body: Box::new(RuntimeExpr::Value(RuntimeValue::Int((7).into()))),
    };
    let result = compile_expr_into_module(
        new_object_module("px8h-missing-join-site").unwrap(),
        "ken_px8h_missing_join_site",
        Linkage::Export,
        &expression,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        false,
        None,
        None,
        None,
    );
    let error = match result {
        Ok(_) => panic!("a live checked occurrence without its plan site must reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "NativeJoinPlanV1",
            reason,
        }) if reason.contains("marker was not consumed")
    ));
}
#[test]
fn process_lowering_without_checked_root_authority_rejects_before_cfg() {
    let result = compile_expr_into_module(
        new_object_module("px8ta-missing-root-authority").unwrap(),
        "ken_px8ta_missing_root_authority",
        Linkage::Export,
        &RuntimeExpr::Construct {
            constructor: crate::EXIT_SUCCESS_CONSTRUCTOR.to_string(),
            args: Vec::new(),
        },
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        true,
        None,
        None,
        None,
    );
    let error = match result {
        Ok(_) => panic!("process lowering must not invent root authority from process mode"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "NativeJoinPlanV1",
            reason,
        }) if reason == "process-object lowering has no checked distinguished-root answer authority"
    ));
}
#[test]
fn checked_marker_census_rejects_duplicate_call_and_slot_occurrences_before_cfg() {
    let (entry, declaration, plan) = occurrence_exact_marker_fixture(false, false);
    let declarations = BTreeMap::from([(declaration.symbol.as_str(), &declaration)]);
    validate_oriented_subcontinuation_transport(&entry, &declarations, Some(&plan))
        .expect("the exact checked Runtime marker occurrence ledger closes");

    for (duplicate_call, duplicate_slot, expected) in [
        (
            true,
            false,
            "computational-IH call Runtime occurrences differ",
        ),
        (
            false,
            true,
            "computational-IH slot Runtime occurrences differ",
        ),
    ] {
        let (entry, declaration, plan) =
            occurrence_exact_marker_fixture(duplicate_call, duplicate_slot);
        let declarations = BTreeMap::from([(declaration.symbol.as_str(), &declaration)]);
        let error = validate_oriented_subcontinuation_transport(&entry, &declarations, Some(&plan))
            .expect_err("an extra static marker occurrence must reject before CFG emission");
        assert!(
            matches!(
                error,
                CraneliftBackendError::Unsupported(UnsupportedLowering {
                    construct: "OrientedSubcontinuationPlanV1",
                    ref reason,
                }) if reason.contains(expected)
            ),
            "{error:?}"
        );
    }
}
#[test]
fn valid_root_plus_missing_marked_scalar_cut_rejects_before_emission() {
    let expression = RuntimeExpr::CheckedJoinSite {
        site_id: 41,
        body: Box::new(host_result_computational_fixture(1, true, false)),
    };
    let symbols = crate::NativeProcessSymbols::legacy_prelude();
    let result = compile_expr_into_module(
        new_object_module("px8h-root-marker-class-separation").unwrap(),
        "ken_px8h_root_marker_class_separation",
        Linkage::Export,
        &expression,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        true,
        Some(&symbols),
        Some(crate::NativeJoinPlanV1 {
            representation_rule_version: crate::NativeJoinPlanV1::REPRESENTATION_RULE_VERSION,
            sites: vec![self_consistent_root_join_site(0)],
        }),
        None,
    );
    let error = match result {
        Ok(_) => panic!("the root must not discharge a missing marked scalar-cut site"),
        Err(error) => error,
    };
    assert!(
        matches!(
            error,
            CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "NativeJoinPlanV1",
                ref reason,
            }) if reason.contains("marker was not consumed")
        ),
        "{error:?}"
    );
}
#[test]
fn self_consistent_appended_orphan_join_site_rejects_before_emission() {
    let result = compile_expr_into_module(
        new_object_module("px8h-orphan-join-site").unwrap(),
        "ken_px8h_orphan_join_site",
        Linkage::Export,
        &RuntimeExpr::Value(RuntimeValue::Int((7).into())),
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        false,
        None,
        Some(crate::NativeJoinPlanV1 {
            representation_rule_version: crate::NativeJoinPlanV1::REPRESENTATION_RULE_VERSION,
            sites: vec![
                self_consistent_root_join_site(0),
                self_consistent_join_site(52, 23),
            ],
        }),
        None,
    );
    let error = match result {
        Ok(_) => panic!("a self-consistent orphan plan row must reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "NativeJoinPlanV1",
            reason,
        }) if reason.contains("unconsumed or orphan site")
    ));
}
#[test]
fn an_out_of_range_child_position_is_a_loud_planner_invariant() {
    let record = one_child_record();
    let (plan, root) = planned_root_occurrence(&record);
    let error = plan
        .child_static_origin(root, 7)
        .expect_err("a record with one field has no child at position 7");
    // AC-6: an invariant violation is a compiler bug, never a capacity limit --
    // so the specific variant is asserted, not `is_err()`.
    match error {
        CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(ref reason)) => {
            assert_eq!(reason, "static origin has no child at that source position");
        }
        other => panic!("expected a loud PlannerInvariant, got {other:?}"),
    }
}

// ─── RT-FNSPLIT-B2A-C N1/N2 — the emission census, pinned mechanically ────
//
// AC-7 wants each negative-boundary pin discharged by a committed check rather
// than by review reading. N1 and N2 are counting properties over the PRODUCTION
// lowering and planning sources, so they are pinned by counting call
// expressions in those exact files. The test sources live in a sibling
// directory, so no `#[cfg(test)]` region has to be parsed out: the partition is
// at file level.

/// **`RT-FNSPLIT-B2F` `AC-2` — WHICH POPULATION THIS CENSUS COVERS, and why the
/// rest is excluded.**
///
/// ⛔ **The population is the production LOWERING AND PLANNING sources** — the
/// seven rows below. It is deliberately **not** "every Cranelift emitter in
/// `ken-runtime`", and stating that boundary is `AC-2`'s second clause: a census
/// whose scope is implicit reads as covering everything.
///
/// **Excluded, measured at base `6534e4a6`, each with its reason:**
///
/// | emitter | measured | why it is out of scope here |
/// |---|---|---|
/// | `native_int_clif.rs` | 5 / 1 / 3 | Θ(1) per native module. Its emitted population is already pinned behaviourally as `LOCAL_HELPER_COUNT = 6` (`artifact/tests.rs:56`) — ⛔ cite that, do not duplicate it |
/// | `boundary_value_clif.rs` | 23 / 3 / 3 | ⭐ a live production emitter that was in **neither** this census nor `BACKEND_PRODUCTION_SOURCES`; same Θ(1)-per-module shape |
///
/// ⭐ **Why they are recorded as reasoned exclusions rather than pinned rows,
/// which is a judgement and is stated as one:** freezing `23` and `5` here would
/// redden this file whenever a *sibling* node legitimately changes an emitter it
/// owns — landing the failure on whoever is unlucky, in a test they have never
/// read, rather than on whoever changed the thing. Their growth is `AC-G0`/`D8`'s
/// obligation and is discharged there **behaviourally**, against emitted counts,
/// not against source spellings.
///
/// ⚠ **MEASURED:** how many times five spellings occur in eight files.
/// **CLAIMED:** exactly that. **THE GAP:** ⛔ this is a source-TEXT oracle and
/// it is retained as a **tripwire, not as the evidence**. A call split across
/// lines evades every needle; a mention inside a string or a block comment
/// inflates them; and nothing here observes what a compiled module actually
/// contains.
///
/// # ⛔⛔ WHICH INSTRUMENT CARRIES THE CLAIM — and they are NOT corroboration
///
/// **`AC-2` requires this division of labour to be stated in-source, because two
/// counts sitting side by side read as corroboration and these two are not: one
/// of them is fail-open by construction.**
///
/// | instrument | what it does | what it carries |
/// |---|---|---|
/// | ⭐ the behavioural counters — `units::b2f_last_unit_emission`, `seed_material::b2f_last_seed_material_emission` | count what the compiled module **actually contains**, at the point of emission | ⭐ **the population claim, entirely** |
/// | ⚠ this census | searches source text for spellings someone enumerated | ⛔ **nothing.** A tripwire only |
///
/// ⛔ **This census's default branch is *"needle not found ⇒ nothing
/// emitted"*, so it fails OPEN for every emission spelling nobody thought of.**
/// It was repaired three times on this node — missing rows, then missing
/// sibling emitters, then a missing needle class — and each repair found the
/// next thing it was not looking for, because a needle-list census can only
/// ever be one discovery behind the code. ⛔ **Adding `.declare_data(` /
/// `.define_data(` did not make it sound and nothing here claims it did.** It
/// is retained, unweakened, because a defeat count never licenses removing a
/// gate — not because it is evidence.
#[test]
fn correspondence_adds_no_emitted_unit_to_the_production_census() {
    struct Census {
        file: &'static str,
        source: &'static str,
        builders: usize,
        definitions: usize,
        declarations: usize,
        /// ⭐ **`RT-FNSPLIT-B2F` `AC-2`, third population defect.** Data objects
        /// are declared and defined by `.declare_data(` / `.define_data(`, and
        /// the three needles above cannot see either. That is a strictly worse
        /// shape than a missing row: a missing *row* leaves one file unmeasured
        /// and the gap is visible, while a missing *needle class* leaves the
        /// census reading **complete across every row** while `n` data objects
        /// sit in the artifact — `D3`'s entire deliverable, invisible, with
        /// nothing looking wrong.
        data_declarations: usize,
        data_definitions: usize,
    }
    let census = [
        Census {
            file: "lowering/core.rs",
            source: include_str!("../../core.rs"),
            // The recursive-descent root builder and definition retired with
            // their lane. The functionized root adapter and unit body remain
            // in `units.rs`; this textual tripwire therefore expects no root
            // builder or definition in this file.
            builders: 0,
            definitions: 0,
            declarations: 2,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-BACKEND-PRIMITIVE-LOWERING-SPLIT` moves the primitive dispatcher
        // and its exclusive helpers into this nested production module. The
        // explicit zero row keeps the whole-roster relation closed while
        // asserting that the move creates no second emission authority.
        Census {
            file: "lowering/core/primitive.rs",
            source: include_str!("../primitive.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "lowering/mod.rs",
            source: include_str!("../../mod.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-LOWERING-VALUES-BOUNDARY-SPLIT` `D1` — the values-boundary
        // disposition/classification/lifecycle-phase vocabulary. A pure
        // classification module: no `FunctionBuilder`, no declared or
        // defined function or data object.
        Census {
            file: "lowering/boundary.rs",
            source: include_str!("../../boundary.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-SOURCE-MACHINE-TYPES-SPLIT` `D1` — the source machine's own
        // dispatch, moved verbatim from `core.rs`/`mod.rs`. It emits IR into
        // the `FunctionBuilder` its caller already owns; it never mints a new
        // defined function or data object.
        Census {
            file: "lowering/source.rs",
            source: include_str!("../../source.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-EMITTER-CALLS-RETURNS-SPLIT` `D1` — the calls and returns
        // emitter, moved verbatim from `core.rs`/`mod.rs`. It emits IR into
        // the `FunctionBuilder` its caller already owns; it never mints a new
        // defined function or data object.
        Census {
            file: "lowering/calls.rs",
            source: include_str!("../../calls.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-EMITTER-CONTROL-JOINS-SPLIT` `D1` — the control and joins
        // emitter, moved verbatim from `core.rs`/`mod.rs`. It emits IR into
        // the `FunctionBuilder` its caller already owns; it never mints a
        // new defined function or data object.
        Census {
            file: "lowering/joins.rs",
            source: include_str!("../../joins.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-EMITTER-AGGREGATES-SPLIT` `D1` — the aggregates emitter's
        // PRODUCTION code, moved verbatim from `mod.rs`, emits IR into the
        // `FunctionBuilder` its caller already owns and records into the
        // already-open `AggregateAllocationLedger`; it never mints a new
        // defined function or data object -- zero on all five needles.
        //
        // `D2` moved the `D7` cluster's TEST code into this same file's own
        // `#[cfg(test)] mod tests`, and this needle scan is a naive
        // full-text `str::matches`, not `cfg`-aware (`control.rs`'s own
        // `correspondence_adds_no_emitted_unit_to_the_production_census`
        // scans `include_str!` of the whole file). Several `D7` direct-API
        // tests build a bare rig `FunctionBuilder`/declare a probe function
        // to test `emit_carrier_alloc`/`source_aggregate_preflight` below
        // the full compile pipeline -- 4 `builders` + 2 `declarations`,
        // confirmed by reading each site directly, all inside `mod tests`.
        // `calls.rs`'s own `D2` test module happens to route entirely
        // through the shared `compile_expr_into_module`/`new_jit_module`
        // harness instead, so it never tripped this same needle -- not a
        // rule that test code must avoid these calls, just a fact about
        // which fixtures each item's own moved tests happened to use.
        Census {
            file: "lowering/aggregates.rs",
            source: include_str!("../../aggregates.rs"),
            builders: 4,
            definitions: 0,
            declarations: 2,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-EMITTER-EFFECTS-SPLIT` `D1` — a pure lowering-emission module:
        // it emits into a `FunctionBuilder` passed in by its caller, never
        // creates its own. No `mod tests` block exists here at `D1` (unlike
        // `aggregates.rs` above, whose non-zero row is entirely its own
        // `D2`-landed test rig) — every needle is confirmed zero by direct
        // grep, not assumed from the row shape.
        Census {
            file: "lowering/effects.rs",
            source: include_str!("../../effects.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "planning.rs",
            source: include_str!("../../../planning.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "planning/static_transition.rs",
            source: include_str!("../../../planning/static_transition.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-OCCURRENCES-SPLIT` `D1` — the occurrence owner. A
        // planning module with no emission, so every count is zero.
        Census {
            file: "planning/static_transition/occurrences.rs",
            source: include_str!("../../../planning/static_transition/occurrences.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "planning/static_transition/semantic_ir.rs",
            source: include_str!("../../../planning/static_transition/semantic_ir.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-UNITS-ABI-SPLIT` `D1` — the Emittable* vocabulary and
        // the StaticTransitionPlan projections that derive it. A planning
        // module with no emission, so every count is zero.
        Census {
            file: "planning/static_transition/units.rs",
            source: include_str!("../../../planning/static_transition/units.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // ⭐ `RT-FNSPLIT-B2F` `AC-2` — THE PREDICTED ROW, and it is predicted
        // rather than fitted.
        //
        // Recorded in `docs/program/rt-fnsplit-b2f-predictions.md` (`P1`) at
        // base `6534e4a6`, committed BEFORE the module was written, and then
        // measured: 1 / 1 / 1, exactly as predicted. A census re-fitted to
        // whatever the output happened to be measures nothing, so the order is
        // the evidence.
        //
        // ⛔ ONE of each spelling, for a population of Θ(n) emitted units. The
        // needles count SPELLINGS, never units: `declare_unit_bundle` holds one
        // `declare_function` inside a loop over every unit, and
        // `define_unit_body` is called once per unit from one site. That gap is
        // the whole content of `AC-G0`'s narrative — `native_int_clif` emits 6
        // definitions from 5 builder source sites — and it is why this row
        // cannot be read as an emitted-unit count. `D8`'s growth verdict is
        // about `UnitBundle::len`, which this pin cannot see.
        Census {
            file: "lowering/units.rs",
            source: include_str!("../../units.rs"),
            // One builder/definition for the public root adapter, one for the
            // loop-defined internal units, one for `RT-CONTSPEC-ACTIVATE`
            // `D2`'s continuation bodies, and one for `RT-DECL-CLOSURE-PORT`
            // `D5a`'s generated producer execution contexts.
            //
            // ⭐ The `D5a` row moved 3 -> 4 deliberately. That is the sentinel
            // working: a new *emitting* function class in this file is exactly
            // the event this row exists to force a reader to look at, and it is
            // the fourth such class rather than a fourth copy of an existing
            // one.
            //
            // And 4 -> 5 for `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f`'s static
            // continuation fusion bodies, for the same reason and on the same
            // terms: a **fifth** emitting function class, not a fifth copy of an
            // existing one. A fused region is a third owner beside the producer
            // and the consumer, so its body is built by its own pass rather than
            // by widening one of the four above.
            //
            // The row moved while the emitter itself is **un-wired**
            // (`D2F_EMITTER_ARMED`): the pass is compiled and reachable but
            // installs no plane, so it defines zero functions on every current
            // compile. That is exactly why this row cannot be read as an
            // emitted-unit count — it counts builder SITES in this file, and
            // `UnitBundle::len` is the number it cannot see.
            builders: 5,
            definitions: 5,
            // Three declaration sites: the emittable unit bundle,
            // `RT-CONTSPEC-ACTIVATE` `D2`'s forward declaration of one target
            // per planned continuation specialization, and `D5a`'s forward
            // declaration of one target per planned generated context. Each is a
            // deliberate addition and this row is the record of them. `D2f`'s
            // forward declaration of one target per installed fused region is
            // the fourth, in the same up-front bundle pass and for the same
            // reason: a target called from a body defined below must exist
            // before that body is built.
            declarations: 4,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-FNSPLIT-B2R`'s ABI plane, added as an explicit ZERO row because
        // the frame flagged its absence: it is in `BACKEND_PRODUCTION_SOURCES`
        // and was not in this census, and an absent row and a zero row read
        // identically to a reader while only one of them is a claim.
        //
        // ⭐ The zero is the load-bearing part: `abi.rs` DECLARES the
        // representation contract and must never emit against it. If this row
        // ever moves, the planner has started emitting, which is the one thing
        // the ownership/representation split exists to prevent.
        Census {
            file: "planning/static_transition/abi.rs",
            source: include_str!("../../../planning/static_transition/abi.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-AGGREGATES-SPLIT` `D1` — the aggregates owner. A
        // planning module with no emission, so every count is zero. The zero
        // is load-bearing for the same reason as `abi.rs`: the planner mints
        // aggregate occurrence identities and ownership records and must
        // never emit against them; the lowering-owned half
        // (`AggregateAllocationEvent`/`AggregateAllocationLedger`/
        // `AggregateRelationClosure`) stays in `lowering/mod.rs`.
        Census {
            file: "planning/static_transition/aggregates.rs",
            source: include_str!("../../../planning/static_transition/aggregates.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-CONTINUATIONS-SPLIT` `D1` — the continuation owner. A
        // planning module with no emission, so every count is zero. The zero
        // is load-bearing for the same reason as `abi.rs`: the planner mints
        // keys, seats and evidence and must never emit against them.
        Census {
            file: "planning/static_transition/continuations.rs",
            source: include_str!("../../../planning/static_transition/continuations.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-CONTINUATIONS-SPLIT` `D1` sub-split — the fusion identity
        // plane. A planning module with no emission, so every count is zero.
        Census {
            file: "planning/static_transition/continuations/fusion.rs",
            source: include_str!("../../../planning/static_transition/continuations/fusion.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-EFFECTS-SPLIT` `D1` — the host-effect seat authority. A
        // planning module with no emission, so every count is zero. The zero
        // is load-bearing for the same reason as `abi.rs`: the planner mints
        // seat identities and validates them and must never emit against
        // them; the emitter-owned half (`EffectSeatGroupId`/
        // `EffectSeatLedger`/`EffectSeatClosure`) stays in `lowering/mod.rs`.
        Census {
            file: "planning/static_transition/effects.rs",
            source: include_str!("../../../planning/static_transition/effects.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-JOINS-TRAPS-SPLIT` `D1` — join disposition and trap
        // identity. A planning module with no emission, so every count is
        // zero. The zero is load-bearing for the same reason as `abi.rs`:
        // the planner derives join representations and dedups trap values
        // and must never emit against them; the emitter-owned half
        // (`Px8trTrapProvenanceEvent`/`PlannedTrapSeat`) stays in
        // `lowering/mod.rs`.
        Census {
            file: "planning/static_transition/joins_traps.rs",
            source: include_str!("../../../planning/static_transition/joins_traps.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — the construction lifecycle
        // (minting, relation and seat construction: `Planner`'s own impl).
        // A planning module with no emission, so every count is zero, for
        // the same reason as every other planner-owned sibling.
        Census {
            file: "planning/static_transition/construction.rs",
            source: include_str!("../../../planning/static_transition/construction.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — the closure lifecycle
        // (validation and closure, and read-only projections: most of
        // `StaticTransitionPlan`'s own impl). A planning module with no
        // emission, so every count is zero, for the same reason as every
        // other planner-owned sibling.
        Census {
            file: "planning/static_transition/closure.rs",
            source: include_str!("../../../planning/static_transition/closure.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        // ⭐ `RT-FNSPLIT-B2F` `D3`/`AC-2` — THE SECOND PREDICTED ROW, and the
        // prediction was recorded before the module existed for the same reason
        // the first one was.
        //
        // Recorded in `docs/program/rt-fnsplit-b2f-predictions.md` (`P6`) at
        // base `6534e4a6`: **1 `declare_data` / 1 `define_data`, every other row
        // 0/0.** Measured: exactly that.
        //
        // ⚠ **AND `P6` WAS WRONG ABOUT WHERE, WHICH IS RECORDED RATHER THAN
        // QUIETLY CORRECTED.** It named `lowering/units.rs` as the file carrying
        // the two needles; the material is minted in `lowering/seed_material.rs`
        // instead, because units and seed material are two populations on two
        // growth axes (Θ(n) in the program vs Θ(|seed environment|), which the
        // program does not affect) and one census row cannot carry both. ⇒ The
        // *counts* held; the *row* moved. A prediction file that only ever
        // agrees with the outcome is a transcription, and `P4` said in advance
        // that the row placement was the likeliest thing to move.
        //
        // ⛔ ONE of each spelling for a population of Θ(|seed environment|)
        // objects: `mint_seed_material` holds one `declare_data` and one
        // `define_data` inside a loop over every entry. Same spellings-not-units
        // gap as the row above, and the same consequence — ⛔ **this row is not
        // an object count and must never be read as one.**
        Census {
            file: "lowering/seed_material.rs",
            source: include_str!("../../seed_material.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 1,
            data_definitions: 1,
        },
        // ⭐⭐ `RT-FNSPLIT-B2F` `AC-2`, SECOND CLAUSE — THE REMAINING SEVEN
        // ROSTER FILES, as explicit zero rows.
        //
        // ⛔ **`abi.rs` was not the only absence.** The frame flagged it by
        // name, it was added, and that read as the clause being discharged.
        // Re-derived here against the roster rather than against the frame's
        // sentence: the frame said thirteen files, `BACKEND_PRODUCTION_
        // SOURCES` had already grown past that by the time this was written,
        // and it keeps growing with every later split (`D2`, `RT-EMITTER-
        // EFFECTS-SPLIT`: dropped the literal count here rather than
        // re-bumping it a second time — the enforced invariant is the
        // `.len()`-based assertion below, not this sentence, and a literal
        // count here only drifts stale again at the next split). The census
        // carried eight of the roster at the time, and **seven** were still
        // absent with no recorded exclusion. All seven measure `0/0/0/0/0`,
        // which is why they are rows and not judgements.
        //
        // ⭐ **A zero row and an absent row read identically and only one of
        // them is a claim** — `AC-2`'s own words, and the reason a file that
        // genuinely emits nothing still needs a line here. ⚠ It is also the
        // reason these seven cost nothing to carry: the sibling-churn objection
        // that keeps `native_int_clif.rs`'s `23` out of this table does not
        // apply to a zero, which moves only when one of these files **starts**
        // emitting.
        //
        // ⚠ What each zero is actually saying, because they are not all the
        // same claim:
        //
        // - `cranelift_backend.rs`, `surface.rs` — a facade and an error
        //   vocabulary. ⛔ `cranelift_backend.rs` is ATTESTED and is read here,
        //   never edited; a row over it is a read, not a modification.
        // - `artifact/api.rs`, `artifact/mod.rs` — module CONSTRUCTION. They
        //   build `JITModule`/`ObjectModule` and hand them on; ⭐ a nonzero here
        //   would mean artifact construction had started declaring or defining
        //   functions on its own, which is a second emission authority in the
        //   one place nobody looks for it.
        // - `compiled.rs` — the ARTIFACT and its runner. ⭐ **The most
        //   load-bearing zero of the seven**: `S6`'s activation-services
        //   launcher lands here, and this row is what forces that landing to be
        //   a deliberate re-baseline rather than a silent one. ⚠ Predicted to
        //   stay `0` through that change — the launcher constructs a Rust
        //   record and calls compiled code; it declares and defines nothing.
        //   ⛔ If it moves, the launcher started emitting and that is the
        //   finding, not the test being stale.
        // - `test_objects.rs`, `test_support.rs` — ⚠ **named "test" and they
        //   are PRODUCTION files**, which is exactly why they need rows: a
        //   reader skipping them by name would leave two production files
        //   unmeasured and believe the roster was covered.
        //
        // ⛔ Still not evidence. These rows inherit every limit stated above —
        // the census is a source-TEXT tripwire that fails OPEN on any spelling
        // nobody enumerated, and adding seven rows widens its coverage without
        // changing what it can carry.
        Census {
            file: "cranelift_backend.rs",
            source: include_str!("../../../../cranelift_backend.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "artifact/api.rs",
            source: include_str!("../../../artifact/api.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "artifact/mod.rs",
            source: include_str!("../../../artifact/mod.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "compiled.rs",
            source: include_str!("../../../compiled.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "surface.rs",
            source: include_str!("../../../surface.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "test_objects.rs",
            source: include_str!("../../../test_objects.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
        Census {
            file: "test_support.rs",
            source: include_str!("../../../test_support.rs"),
            builders: 0,
            definitions: 0,
            declarations: 0,
            data_declarations: 0,
            data_definitions: 0,
        },
    ];
    // ⭐⭐ `AC-2`, SECOND CLAUSE — THE CENSUS COVERS THE WHOLE ROSTER, and this
    // is what keeps the coverage claim true after this commit rather than at it.
    //
    // ⛔ **Without this, "every roster file has a row" is a fact about today,
    // not a property.** A file added to `BACKEND_PRODUCTION_SOURCES` by any
    // future node would be invisible to this census while the census still read
    // as complete — which is precisely how `abi.rs` and then these seven came to
    // be missing in the first place. ⇒ The relation is asserted, so the next
    // absence reddens instead of accumulating.
    //
    // ⚠ It is a relation between two rosters, ⛔ **not** a count: adding a file
    // to either list is fine, and adding it to only one is the failure.
    let censused = census.iter().map(|row| row.file).collect::<BTreeSet<_>>();
    let roster = BACKEND_PRODUCTION_SOURCES
        .iter()
        .map(|(file, _)| *file)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        roster.difference(&censused).copied().collect::<Vec<_>>(),
        Vec::<&str>::new(),
        "AC-2: a production roster file has no census row, so the census reads \
         as complete while that file is unmeasured"
    );
    assert_eq!(
        censused.difference(&roster).copied().collect::<Vec<_>>(),
        Vec::<&str>::new(),
        "AC-2: a census row names a file outside the production roster, so one \
         of the two lists is wrong about what production is"
    );
    for row in census {
        assert_eq!(
            row.source.matches("FunctionBuilder::new(").count(),
            row.builders,
            "{}: N1 -- the production root builder census moved",
            row.file
        );
        assert_eq!(
            row.source.matches(".define_function(").count(),
            row.definitions,
            "{}: N1/N2 -- a definition was added or removed",
            row.file
        );
        assert_eq!(
            row.source.matches(".declare_function(").count(),
            row.declarations,
            "{}: N2 -- a function declaration was added or removed",
            row.file
        );
        assert_eq!(
            row.source.matches(".declare_data(").count(),
            row.data_declarations,
            "{}: N3 -- an artifact-static data declaration was added or removed",
            row.file
        );
        assert_eq!(
            row.source.matches(".define_data(").count(),
            row.data_definitions,
            "{}: N3 -- an artifact-static data definition was added or removed",
            row.file
        );
    }
}

/// **`RT-FNSPLIT-B2A-S` D4/AC-4 — the `origin -> expression` lookup count is
/// EXACTLY ONE.** This pin *replaces* `RT-FNSPLIT-B2A-C`'s N3, and the transition
/// is stated here rather than in a commit message so it stays auditable:
///
/// | | B2A-C's N3 (retired) | this pin |
/// |---|---|---|
/// | lookups that exist | **0** — `!source.contains("-> Result<&'src RuntimeExpr")` | **1** |
/// | consumers that call one | 0 (none existed to call) | **1** |
///
/// ⛔ N3 was **not** violated — it was **retired by design**. B2A-C asserted zero
/// because at that point the origin was provenance, and any lookup would have been
/// an unaudited second authority. B2A-S's whole job is to introduce the lookup so
/// that a retained body is selected by its static name. ⭐ The count therefore goes
/// `0 -> 1`, never `0 -> unbounded`: one producer, one consumer.
///
/// A reviewer reading the new lookup against B2A-C's AC list without this table
/// would reject a correct diff.
// RETIRED by the RT-FNSPLIT-RECUR-PORT successor repair: this reads repository
// text and inventories exported spellings, so it is not a runtime-behavior
// test. `every_origin_to_expression_resolution_goes_through_the_single_route`
// carries the behavioral route property.

/// Counts whole-identifier occurrences in production source, comments removed.
///
/// ⛔ **Neither a substring scan nor a line scan is sound, and the Architect proved
/// both against me** (`evt_6sq2tq3v9jcd0`, `evt_1p11krxny4wny`). The first census
/// tested `line.contains(".source_occurrence(")`, which a second lookup evaded by
/// formatting alone:
///
/// ```text
/// let _second = self
///     .static_transition_plan
///     .source_occurrence
///     (static_origin)?;
/// ```
///
/// No line contains `.source_occurrence(`, so the pin passed with two lookups
/// present. ⇒ **A text pattern is a claim about layout; the property is about
/// code.** Tokenizing is the fix, not a longer list of spellings:
///
/// - splitting on every non-identifier character makes newlines, dots and spaces
///   all separators, so **no formatting can hide a mention**;
/// - matching a **whole token** distinguishes `source_occurrence` from the
///   `source_occurrences` field, which a substring scan would conflate;
/// - counting the **identifier** rather than a call shape also catches a path-form
///   or aliased call (`StaticTransitionPlan::source_occurrence(plan, o)`), because
///   a method cannot be called without naming it;
/// - comments are stripped per line **before** tokenizing, because the resolver's
///   own doc comment and these very notes name it — an oracle that greps a name
///   otherwise fires on the prose describing it.
///
/// ⚠ Residual: a call synthesized by a macro would not name the identifier in this
/// source. There is no such macro in the backend, and one would be visible in the
/// same review; this is a stated limit, not a silent one.
#[cfg(test)]
/// **Every production source carrying an `impl Lowering` block.**
///
/// ⛔ `core.rs` alone is NOT the routing surface, and assuming it was is the
/// defect this constant exists to prevent: `lowering/mod.rs:2473` carries a
/// **second** `impl<'a> Lowering<'a>` block. A retained-body route added there
/// would have sat entirely outside a `core.rs`-scoped inventory.
///
/// ⛔ **An earlier revision of this comment continued: *"today `mod.rs` cannot
/// reach `retained_body_occurrence` … that privacy is therefore load-bearing …
/// this list is what makes the inventory still correct after a deliberate
/// widening."* That is the REACHABILITY entailment the Architect ruling
/// (`evt_5yxjd1zqnyvcq`) struck, and it is withdrawn here too.**
///
/// The list is now a **declaration inventory only**: it names the files that
/// carry an `impl Lowering` block, so a declaration appearing in a second one is
/// *visible*. It supports no claim about who can **call** anything — that is the
/// plan graph's to answer, via an occurrence's `SemanticOwner` and the planned
/// edge kind.
const LOWERING_IMPL_SOURCES: &[(&str, &str)] = &[
    ("lowering/core.rs", include_str!("../../core.rs")),
    ("lowering/mod.rs", include_str!("../../mod.rs")),
];

/// Is the retained-body helper exposed only to the `lowering` parent and its
/// children?
///
/// `B2F` deliberately moved unit emission into sibling `units.rs`, so the
/// narrow `pub(super)` qualifier is now required. Any wider qualifier remains
/// a review-visible change.
fn retained_body_helper_has_lowering_only_visibility(core: &str) -> bool {
    core.lines()
        .any(|line| line.trim() == "pub(super) fn retained_body_occurrence(")
}

/// **`RT-FNSPLIT-B2O` `AC-12` split row — the DECLARATION survives, the
/// REACHABILITY entailment does not.** Architect ruling `evt_5yxjd1zqnyvcq`.
///
/// This pin is what remains of the withdrawn route oracle, and the boundary is
/// the point of it:
///
/// - **MEASURED:** `retained_body_occurrence` is declared in `lowering/core.rs`
///   with the narrow `pub(super)` visibility needed by sibling `units.rs`.
/// - **CLAIMED:** exactly that, and nothing further.
/// - **THE GAP:** ⛔ this does **not** establish which functions can *reach* the
///   helper. The withdrawn oracle made that inference — *"`mod.rs` therefore
///   cannot reach it, so the route inventory is still correct"* — and
///   reachability is not a property of declaration text. Name resolution, macro
///   expansion, and indirect calls all sit outside what any source scan sees.
///
/// ⇒ **The authority for boundaries is the plan graph** — an occurrence's
/// `StaticOriginId`, its validated `SemanticOwner`, and the planned edge kind —
/// **not this file's text.** A Rust wrapper or a same-named method in another
/// `impl` creates no Ken function-unit boundary, so no pin here should redden
/// when one is added; see `b2o_ac10c_repointing_a_static_body_edge_changes_the_
/// disposition` for the axis that *is* authority.
///
/// Promise class: **normative compatibility vector** — `pub(super)` is the
/// contract, and widening it further is a deliberate review event.
#[test]
fn the_retained_body_helper_is_visible_only_inside_lowering() {
    let core = LOWERING_IMPL_SOURCES
        .iter()
        .find(|(file, _)| *file == "lowering/core.rs")
        .map(|(_, source)| *source)
        .expect("the impl-source list must carry core.rs");
    assert!(
        retained_body_helper_has_lowering_only_visibility(core),
        "`retained_body_occurrence` no longer declares with the exact narrow \
         `pub(super)` visibility in `lowering/core.rs`.\n\
         A wider qualifier is a DELIBERATE widening and belongs in review.\n\
         ⚠ This pin makes NO claim about who can reach the helper; that is the \
         plan graph's to answer, not this file's."
    );

    // The helper is declared in exactly one of the `impl Lowering` sources. This
    // is a DECLARATION inventory over both files -- it says where the helper is
    // written, never who can call it.
    let declaring = LOWERING_IMPL_SOURCES
        .iter()
        .filter(|(_, source)| retained_body_helper_has_lowering_only_visibility(source))
        .map(|(file, _)| *file)
        .collect::<Vec<_>>();
    assert_eq!(
        declaring,
        vec!["lowering/core.rs"],
        "the retained-body helper's DECLARING file set changed. ⚠ A declaration \
         in a second `impl Lowering` source is a review event; this pin reports \
         it and draws no conclusion about reachability."
    );

    // Non-vacuity: the needles must be real files, or both assertions above are
    // satisfied by an empty read.
    for (file, source) in LOWERING_IMPL_SOURCES {
        assert!(
            source.len() > 10_000,
            "`{file}` did not load; the assertions above would pass vacuously"
        );
    }
}

pub(super) fn identifier_occurrences(source: &str, identifier: &str) -> usize {
    source
        .lines()
        .map(|line| line.split_once("//").map_or(line, |(code, _)| code))
        .flat_map(|code| code.split(|c: char| !c.is_alphanumeric() && c != '_'))
        .filter(|token| *token == identifier)
        .count()
}

#[test]
fn the_identifier_census_survives_the_evasions_that_defeated_the_text_scan() {
    // ⭐ Positive control built from the Architect's own two mutations. If the
    // census cannot see these, the count above asserts nothing.
    let split_across_lines = "let _second = self\n    .static_transition_plan\n    .source_occurrence\n    (static_origin)?;\n";
    assert_eq!(
        identifier_occurrences(split_across_lines, "source_occurrence"),
        1,
        "a mention split across lines must still be counted"
    );
    let path_form = "let _ = StaticTransitionPlan::source_occurrence(plan, origin)?;\n";
    assert_eq!(identifier_occurrences(path_form, "source_occurrence"), 1);
    // The plural FIELD must not be conflated with the resolver.
    assert_eq!(
        identifier_occurrences("self.plan.source_occurrences.len()\n", "source_occurrence"),
        0,
        "`source_occurrences` is a different identifier"
    );
    // Prose must not satisfy or inflate the census.
    assert_eq!(
        identifier_occurrences("// calls source_occurrence here\n", "source_occurrence"),
        0
    );
    assert_eq!(
        identifier_occurrences(
            "/// `source_occurrence` is the sole route\n",
            "source_occurrence"
        ),
        0
    );
}

/// The backend's complete production source surface — the census's **closure
/// proof**, not a convenience list.
///
/// ⭐ Why this is a proof rather than an enumeration someone must remember: a Rust
/// file is compiled only if an ancestor module declares it with `mod`. So pinning
/// every production `mod` declaration across the backend pins the *file set*, and
/// a thirteenth backend file cannot be compiled without reddening
/// `the_backend_production_surface_inventory_is_closed` below — which is what
/// forces whoever adds it to extend this list.
#[cfg(test)]
const BACKEND_PRODUCTION_SOURCES: &[(&str, &str)] = &[
    (
        "cranelift_backend.rs",
        include_str!("../../../../cranelift_backend.rs"),
    ),
    ("artifact/api.rs", include_str!("../../../artifact/api.rs")),
    ("artifact/mod.rs", include_str!("../../../artifact/mod.rs")),
    ("compiled.rs", include_str!("../../../compiled.rs")),
    ("lowering/core.rs", include_str!("../../core.rs")),
    (
        "lowering/core/primitive.rs",
        include_str!("../primitive.rs"),
    ),
    ("lowering/mod.rs", include_str!("../../mod.rs")),
    // `RT-LOWERING-VALUES-BOUNDARY-SPLIT` `D1` — registered the moment the
    // module exists, for the same reason as every sibling below: a production
    // source absent from this roster is invisible to every pin that iterates
    // it.
    ("lowering/boundary.rs", include_str!("../../boundary.rs")),
    // `RT-SOURCE-MACHINE-TYPES-SPLIT` `D1` — registered the moment the module
    // exists, for the same reason as `boundary.rs` above: a production source
    // absent from this roster is invisible to every pin that iterates it.
    ("lowering/source.rs", include_str!("../../source.rs")),
    // `RT-EMITTER-CALLS-RETURNS-SPLIT` `D1` — the calls and returns emitter.
    // Registered here the moment the module exists, for the same reason as
    // `boundary.rs`/`source.rs` above: a production source absent from this
    // roster is invisible to every pin that iterates it.
    ("lowering/calls.rs", include_str!("../../calls.rs")),
    // `RT-EMITTER-CONTROL-JOINS-SPLIT` `D1` — the control and joins emitter.
    // Registered here the moment the module exists, for the same reason as
    // `boundary.rs`/`source.rs`/`calls.rs` above: a production source absent
    // from this roster is invisible to every pin that iterates it.
    ("lowering/joins.rs", include_str!("../../joins.rs")),
    // `RT-EMITTER-AGGREGATES-SPLIT` `D1` — the aggregates emitter. Registered
    // here the moment the module exists, for the same reason as
    // `boundary.rs`/`source.rs`/`calls.rs`/`joins.rs` above: a production
    // source absent from this roster is invisible to every pin that
    // iterates it.
    ("lowering/aggregates.rs", include_str!("../../aggregates.rs")),
    // `RT-EMITTER-EFFECTS-SPLIT` `D1` — the effects emitter. Registered here
    // the moment the module exists, for the same reason as
    // `boundary.rs`/`source.rs`/`calls.rs`/`joins.rs`/`aggregates.rs` above:
    // a production source absent from this roster is invisible to every pin
    // that iterates it.
    ("lowering/effects.rs", include_str!("../../effects.rs")),
    // `RT-FNSPLIT-B2F` `D1`/`D2` — the target code-unit population. Registered
    // here the moment the module exists, because every pin that iterates this
    // roster is closed only over the files it lists: a production emitter absent
    // from it is invisible to all of them at once, which is precisely how
    // `boundary_value_clif.rs` and `native_int_clif.rs` came to sit outside
    // both this roster and the emitted-unit census.
    ("lowering/units.rs", include_str!("../../units.rs")),
    // `RT-FNSPLIT-B2F` `D3` — the artifact-static seed material. Registered for
    // the same reason as `units.rs` above, and ⭐ **it is the file that made the
    // reason concrete**: this module mints DATA objects, and until `AC-2` was
    // amended no needle in the census could see a data object at all. A file
    // outside this roster is invisible to every pin that iterates it; a file
    // inside it whose emission spelling nobody enumerated is invisible to the
    // census while looking fully measured.
    (
        "lowering/seed_material.rs",
        include_str!("../../seed_material.rs"),
    ),
    ("planning.rs", include_str!("../../../planning.rs")),
    (
        "planning/static_transition.rs",
        include_str!("../../../planning/static_transition.rs"),
    ),
    (
        "planning/static_transition/abi.rs",
        include_str!("../../../planning/static_transition/abi.rs"),
    ),
    // `RT-PLANNER-AGGREGATES-SPLIT` `D1` — the aggregates owner. Registered
    // here the moment the module exists, for the same reason as every
    // sibling: a production module absent from this roster is invisible to
    // every pin that iterates it.
    (
        "planning/static_transition/aggregates.rs",
        include_str!("../../../planning/static_transition/aggregates.rs"),
    ),
    // `RT-PLANNER-CONTINUATIONS-SPLIT` `D1` — the continuation owner. Registered
    // here the moment the module exists, for the same reason as every sibling:
    // a production module absent from this roster is invisible to every pin
    // that iterates it.
    (
        "planning/static_transition/continuations.rs",
        include_str!("../../../planning/static_transition/continuations.rs"),
    ),
    // `RT-PLANNER-CONTINUATIONS-SPLIT` `D1` sub-split — the fusion identity
    // plane (a child of continuations, declared by continuations.rs via
    // `mod fusion;`). Registered for the same reason as every sibling.
    (
        "planning/static_transition/continuations/fusion.rs",
        include_str!("../../../planning/static_transition/continuations/fusion.rs"),
    ),
    // `RT-PLANNER-EFFECTS-SPLIT` `D1` — the host-effect seat authority.
    // Registered here the moment the module exists, for the same reason as
    // every sibling: a production module absent from this roster is
    // invisible to every pin that iterates it.
    (
        "planning/static_transition/effects.rs",
        include_str!("../../../planning/static_transition/effects.rs"),
    ),
    // `RT-PLANNER-JOINS-TRAPS-SPLIT` `D1` — join disposition and trap
    // identity. Registered here the moment the module exists, for the same
    // reason as every sibling: a production module absent from this roster
    // is invisible to every pin that iterates it.
    (
        "planning/static_transition/joins_traps.rs",
        include_str!("../../../planning/static_transition/joins_traps.rs"),
    ),
    // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — the construction lifecycle.
    // Registered here the moment the module exists, for the same reason as
    // every sibling: a production module absent from this roster is
    // invisible to every pin that iterates it.
    (
        "planning/static_transition/construction.rs",
        include_str!("../../../planning/static_transition/construction.rs"),
    ),
    // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — the closure lifecycle.
    // Registered here the moment the module exists, for the same reason as
    // every sibling: a production module absent from this roster is
    // invisible to every pin that iterates it.
    (
        "planning/static_transition/closure.rs",
        include_str!("../../../planning/static_transition/closure.rs"),
    ),
    // `RT-PLANNER-OCCURRENCES-SPLIT` `D1` — the occurrence owner. Registered
    // here the moment the module exists, for the same reason as every sibling:
    // a production module absent from this roster is invisible to every pin
    // that iterates it.
    (
        "planning/static_transition/occurrences.rs",
        include_str!("../../../planning/static_transition/occurrences.rs"),
    ),
    (
        "planning/static_transition/semantic_ir.rs",
        include_str!("../../../planning/static_transition/semantic_ir.rs"),
    ),
    // `RT-PLANNER-UNITS-ABI-SPLIT` `D1` — the emitter's read-only view of one
    // validated function unit. Registered here the moment the module exists,
    // for the same reason as every sibling: a production module absent from
    // this roster is invisible to every pin that iterates it.
    (
        "planning/static_transition/units.rs",
        include_str!("../../../planning/static_transition/units.rs"),
    ),
    ("surface.rs", include_str!("../../../surface.rs")),
    ("test_objects.rs", include_str!("../../../test_objects.rs")),
    ("test_support.rs", include_str!("../../../test_support.rs")),
];

#[test]
fn the_backend_production_surface_inventory_is_closed() {
    // Every production `mod` declaration reachable in the backend, paired with the
    // file that declares it. `mod tests;` is excluded: a sibling test module is not
    // production surface, and its absence from the census is the point.
    let mut declared = Vec::new();
    for (file, source) in BACKEND_PRODUCTION_SOURCES {
        let production = source
            .split_once("\n#[cfg(test)]\nmod tests {")
            .map_or(*source, |(before, _)| before);
        for line in production.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || !trimmed.ends_with(';') {
                continue;
            }
            let Some(rest) = trimmed.strip_suffix(';') else {
                continue;
            };
            let Some(name) = rest.rsplit_once("mod ").map(|(_, name)| name) else {
                continue;
            };
            if name == "tests" || name.contains(' ') {
                continue;
            }
            declared.push((*file, name));
        }
    }
    assert_eq!(
        declared,
        vec![
            ("cranelift_backend.rs", "artifact"),
            ("cranelift_backend.rs", "compiled"),
            ("cranelift_backend.rs", "lowering"),
            ("cranelift_backend.rs", "planning"),
            ("cranelift_backend.rs", "surface"),
            ("cranelift_backend.rs", "test_objects"),
            ("cranelift_backend.rs", "test_support"),
            ("artifact/mod.rs", "api"),
            ("lowering/core.rs", "primitive"),
            ("lowering/mod.rs", "core"),
            // `RT-FNSPLIT-B2F` `D1`/`D2`. A sibling of `core` rather than a
            // region inside it: `core.rs` is the module whose recursive
            // whole-configuration authority `D6` removes, and putting the
            // replacement population in the same file would leave the census
            // that measures the removal unable to tell the two apart.
            ("lowering/mod.rs", "units"),
            // `RT-FNSPLIT-B2F` `D3`. A sibling of `units` rather than a region
            // inside it, because the two mint DIFFERENT POPULATIONS on
            // different growth axes: `units` mints code, Θ(n) in the program;
            // this mints data, Θ(|seed environment|) and independent of the
            // program. Folding them into one file would put two growth axes
            // behind one census row.
            ("lowering/mod.rs", "seed_material"),
            // `RT-LOWERING-VALUES-BOUNDARY-SPLIT` `D1` — the values-boundary
            // disposition/classification/lifecycle-phase vocabulary. A sibling
            // of `core`/`units`/`seed_material`: `Lowered`/`LoweringOperand`
            // and the carrier-emission machinery that consumes this
            // vocabulary stay SCC-pinned in `mod.rs`/`core.rs`.
            ("lowering/mod.rs", "boundary"),
            // `RT-SOURCE-MACHINE-TYPES-SPLIT` `D1` — the source machine's own
            // state types and dispatch control. A sibling of
            // `core`/`units`/`seed_material`/`boundary`; the types the moving
            // methods merely manipulate stay SCC-pinned in `mod.rs`.
            ("lowering/mod.rs", "source"),
            // `RT-EMITTER-CALLS-RETURNS-SPLIT` `D1` — the calls and returns
            // emitter (declared-call emission, residual and recursor call
            // lowering, return emission, callee-side checks). A sibling of
            // `core`/`units`/`seed_material`/`boundary`/`source`; the types
            // the moving methods merely manipulate stay SCC-pinned in
            // `mod.rs`.
            ("lowering/mod.rs", "calls"),
            // `RT-EMITTER-CONTROL-JOINS-SPLIT` `D1` — the control and joins
            // emitter (branch/match emission, join emission, block/
            // terminator construction). A sibling of
            // `core`/`units`/`seed_material`/`boundary`/`source`/`calls`;
            // the types the moving methods merely manipulate
            // (`ScalarMergeKind` and siblings) stay SCC-pinned in `mod.rs`.
            ("lowering/mod.rs", "joins"),
            // `RT-EMITTER-AGGREGATES-SPLIT` `D1` — the aggregates emitter
            // (aggregate construction and projection emission, allocation
            // emission, governed-allocation surfaces). A sibling of
            // `core`/`units`/`seed_material`/`boundary`/`source`/`calls`/
            // `joins`; `AggregateAllocationLedger`/`AggregateAllocationEvent`/
            // `AggregateRelationClosure` move with it (already
            // `pub(in crate::cranelift_backend)`, zero widen); the planner
            // types it merely references (`PlannedAggregateShape` and
            // siblings) stay item 7's.
            ("lowering/mod.rs", "aggregates"),
            // `RT-EMITTER-EFFECTS-SPLIT` `D1` — the effects emitter
            // (effect-seat emission, host-call emission, and the
            // effect-side operand construction). A sibling of
            // `core`/`units`/`seed_material`/`boundary`/`source`/`calls`/
            // `joins`/`aggregates`; `EffectSeatLedger`/`EffectSeatClosure`
            // move with it (already `pub(in crate::cranelift_backend)`,
            // zero widen); `ClaimedEffectSeats`/`SiteOperandWitness` (the
            // Architect's D0 corrections) and the types the moving methods
            // merely manipulate stay at the `mod.rs` hub.
            ("lowering/mod.rs", "effects"),
            ("planning.rs", "static_transition"),
            ("planning/static_transition.rs", "abi"),
            // `RT-PLANNER-AGGREGATES-SPLIT` `D1` — aggregate allocation
            // events, ownership records, and the planner-side aggregate
            // lifecycle, factored into its own domain module. The
            // lowering-owned half (`AggregateAllocationEvent`,
            // `AggregateAllocationLedger`, `AggregateRelationClosure`) stays
            // in `lowering/mod.rs` for item 15.
            ("planning/static_transition.rs", "aggregates"),
            // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — the closure lifecycle
            // (validation and closure, and read-only projections: most of
            // `StaticTransitionPlan`'s own impl), factored into its own
            // domain module. Alphabetically before `construction` in the
            // `mod` declaration order this list follows.
            ("planning/static_transition.rs", "closure"),
            // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — the construction
            // lifecycle (minting, relation and seat construction: `Planner`'s
            // own impl), factored into its own domain module.
            ("planning/static_transition.rs", "construction"),
            // `RT-PLANNER-CONTINUATIONS-SPLIT` `D1` — the continuation owner
            // (keys + seats + evidence surfaces + the fusion identity plane),
            // factored into its own domain module.
            ("planning/static_transition.rs", "continuations"),
            // `RT-PLANNER-EFFECTS-SPLIT` `D1` — the host-effect seat
            // authority (seat derivation, contract lookup, closed-form
            // rebuild-equality and uniqueness validation), factored into its
            // own domain module. The emitter-owned half (`EffectSeatGroupId`,
            // `EffectSeatLedger`, `EffectSeatClosure`,
            // `EffectSeatVisitMutation`, `EffectSeatDispatchMutation`) stays
            // in `lowering/mod.rs` for item 16.
            ("planning/static_transition.rs", "effects"),
            // `RT-PLANNER-JOINS-TRAPS-SPLIT` `D1` — join disposition (which
            // representation a source join's result takes) and trap
            // identity (a value-keyed dedup catalog), factored into its own
            // domain module. The emitter-owned half
            // (`Px8trTrapProvenanceEvent`, `PlannedTrapSeat`) already lives
            // in `lowering/mod.rs` for item 14.
            ("planning/static_transition.rs", "joins_traps"),
            // `RT-PLANNER-OCCURRENCES-SPLIT` `D1` — the occurrence owner
            // (StaticOriginId + records + validations + read views), factored
            // into its own domain module.
            ("planning/static_transition.rs", "occurrences"),
            ("planning/static_transition.rs", "semantic_ir"),
            // `RT-PLANNER-UNITS-ABI-SPLIT` `D1` — the Emittable* vocabulary and
            // the StaticTransitionPlan projections that derive it, factored into
            // their own domain module.
            ("planning/static_transition.rs", "units"),
            // `RT-PLANNER-CONTINUATIONS-SPLIT` `D1` sub-split — the fusion
            // identity plane, a child of continuations (declared by
            // continuations.rs, which the roster scans after static_transition.rs).
            ("planning/static_transition/continuations.rs", "fusion"),
        ],
        "AC-4 -- the backend's module inventory changed, so \
         BACKEND_PRODUCTION_SOURCES is no longer the whole production surface and \
         the sole-consumer census above has stopped being closed. Add the new \
         file to that list."
    );
    assert_eq!(
        declared.len() + 1,
        BACKEND_PRODUCTION_SOURCES.len(),
        "AC-4 -- every declared module must appear in the census list exactly once \
         (+1 for `cranelift_backend.rs`, the root, which no `mod` line declares)"
    );
}

// ─── RT-FNSPLIT-B2A-C AC-1 — uniform threading, shown not asserted ────────
//
// ⛔ A prose claim that "the fallback is covered" does not discharge AC-1. This
// reads the DECLARATIONS of the three source-term carriers and pins two
// properties structurally:
//
//  1. no field in any of them is a bare `RuntimeExpr` / `Vec<RuntimeExpr>` --
//     every carried term is an occurrence pair, so a frame cannot hold a term
//     whose origin was dropped;
//  2. every variant that carries a `cases` list also declares the parent
//     `static_origin` its case bodies are derived from.
//
// Both are declaration-level, not substring-level: a mention of `RuntimeExpr` in
// a comment or in this test's own message cannot satisfy or break them.

#[cfg(test)]
pub(super) fn declaration_span(source: &'static str, header: &str) -> Vec<&'static str> {
    let start = source
        .find(header)
        .unwrap_or_else(|| panic!("{header} is declared in the lowering facade"));
    let mut depth = 0usize;
    let mut span = Vec::new();
    for line in source[start..].lines() {
        span.push(line);
        depth += line.matches('{').count();
        depth -= line.matches('}').count();
        if depth == 0 && span.len() > 1 {
            break;
        }
    }
    span
}

/// The bare-source-term predicate, factored out so it can be given a positive
/// control. ⚠ Without one, this whole pin would pass for the trivial reason that
/// it finds nothing — a negative check passes for any reason.
#[cfg(test)]
pub(super) fn is_bare_source_term_field(line: &str) -> bool {
    let field = line.trim();
    field == "expr: RuntimeExpr,"
        || field == "body: RuntimeExpr,"
        || field == "then_expr: RuntimeExpr,"
        || field == "else_expr: RuntimeExpr,"
        || field == "remaining: Vec<RuntimeExpr>,"
        || field == "args: Vec<RuntimeExpr>,"
}

#[test]
fn the_bare_source_term_detector_catches_the_shape_it_is_looking_for() {
    // The pre-B2A-C declarations, verbatim. If the detector cannot see these it
    // is asserting nothing about the post-B2A-C ones.
    for pre_amendment in [
        "        expr: RuntimeExpr,",
        "        body: RuntimeExpr,",
        "        then_expr: RuntimeExpr,",
        "        remaining: Vec<RuntimeExpr>,",
        "        args: Vec<RuntimeExpr>,",
    ] {
        assert!(
            is_bare_source_term_field(pre_amendment),
            "the AC-1 detector must catch {pre_amendment:?}"
        );
    }
    assert!(!is_bare_source_term_field(
        "        expr: OwnedSourceOccurrence,"
    ));
    assert!(!is_bare_source_term_field(
        "    // a comment naming RuntimeExpr"
    ));
}


// ─── RT-FNSPLIT-B2A-S AC-1/AC-6 — the retained-body carrier holds a NAME ──────
//
// ⛔ AC-1 asks for this structurally, not asserted. It reads the DECLARATIONS of
// the retained-closure variants, so a mention of `OwnedSourceOccurrence` in a
// comment can neither satisfy nor break it, and it states the covered population
// (AC-6) per variant in the assertions themselves.

/// Every field a variant declares, in order.
///
/// ⛔ The **whole inventory**, not a search for known-bad spellings. The first
/// candidate matched three exact `body:` spellings, and the Architect rejected it
/// (`evt_6sq2tq3v9jcd0`): a compile-preserving `cached_body: RuntimeExpr` or
/// `retained: Box<RuntimeExpr>` beside `body: StaticOriginId` evaded it entirely
/// once the construction and pattern sites were updated. A detector enumerating
/// what it forbids can only ever be as complete as the enumeration; pinning what
/// is **allowed** rejects every added field regardless of name or type.
#[cfg(test)]
pub(super) fn declared_fields(source: &'static str, header: &str) -> Vec<&'static str> {
    declaration_span(source, header)
        .into_iter()
        .skip(1)
        .take_while(|line| !line.trim_start().starts_with('}'))
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//") && !line.starts_with("///"))
        .collect()
}

#[test]
fn the_field_inventory_extractor_sees_an_added_term_field() {
    // Positive control on the extractor, using the exact evasion the Architect
    // named. If the extractor cannot see this field, the equality assertion below
    // is not actually closed over "no additional term carrier".
    let synthetic =
        "    Evasion {\n        body: StaticOriginId,\n        cached_body: RuntimeExpr,\n    },\n";
    let fields = declared_fields(
        Box::leak(synthetic.to_string().into_boxed_str()),
        "    Evasion {",
    );
    assert_eq!(
        fields,
        vec!["body: StaticOriginId,", "cached_body: RuntimeExpr,"],
        "the extractor must report EVERY declared field, so an added one breaks \
         the inventory equality"
    );
}


/// **`RT-FNSPLIT-B2A-S` D2 — the plan cannot escape into the compiled artifact,
/// shown by the type system rather than asserted in prose.**
///
/// The plan now **borrows** the source trees, so non-escape stopped being
/// incidental and became load-bearing. `CompiledModule<M>` has no lifetime
/// parameter, so it cannot store a borrow of them; requiring `'static` is exactly
/// that claim, checked by the compiler.
///
/// ⭐ Falsifiable by mutation in the ordinary D6 way: give `CompiledModule` a
/// `&'src RuntimeExpr` field and this test **stops compiling**. That is a stronger
/// failure mode than a red assertion — the escape cannot be introduced and then
/// argued about.
#[test]
fn escaping_a_source_borrow_into_the_compiled_artifact_does_not_typecheck() {
    fn holds_no_borrowed_state<T: 'static>() {}
    holds_no_borrowed_state::<CompiledModule<cranelift_jit::JITModule>>();
}

// ─── RT-FNSPLIT-B2A-S AC-5 — nothing is KEYED by a scheduling entry ───────────
//
// ⭐ Why this needs a pin even though the resolver takes a `StaticOriginId`:
// hard-stop #8 was a category error in which a scheduling entry stood in for a
// source occurrence, and a `ComputationalMatch` SHARES its scheduling entry with
// its scrutinee chain. So a collection keyed by an entry looks perfectly injective
// on every fixture without one — the wrong key still looks unique — and then
// silently merges two occurrences on the fixture that has one.
//
// ⛔ **This scan is a tripwire, not a discharge — and neither are the behavioural
// controls, on their own.** Two Architect blocks established that, and the second
// (`evt_1p11krxny4wny`) is the one that settles it: a real
// `Vec<Option<&RuntimeExpr>>` indexed by `usize::try_from(scrutinee.entry.0)` at
// the `ComputationalMatch` seam **compiles and passes all three nets**.
//
// ⇒ The framed property, *"no collection is keyed by `.entry`, and a mutation
// introducing one reddens,"* is a **global negative over arbitrary code shapes**.
// No test enforces that: detecting it needs dataflow, not a scan, and a scan can
// always be spelled around. So the honest split is recorded here rather than
// papered over with a longer list:
//
// The authoritative AC-5 is the Architect's four clauses (`origin/main`
// `d0b6e064`, transcribed verbatim there); this is what discharges each:
//
//   (a) concrete entry-carrying types stay module-private
//         -> `the_entry_carrying_types_are_module_private`
//   (b) a non-vacuous split fixture proves entry-keying selects the wrong body
//         -> `keying_selection_by_the_scheduling_entry_does_not_resolve_the_body`
//   (c) a compile-preserving re-key of the sanctioned table reddens at the
//       collision/invariant controls
//         -> `filing_two_occurrences_under_one_origin_is_refused`
//   (d) Architect review of the closed two-file planner surface and its exports
//       confirms the stated residual -- review, not a test.
//
// ⛔ **BOTH residual arms, because recording one reads as if the other were
// covered:**
//
//   RESIDUAL 1 — an independently maintained entry-keyed collection INSIDE the
//     two planner files. Inside the planner, entry-keying is the planner's own
//     job and is NOT prohibited; what is unenforceable is detecting a *second*
//     selection authority built from it.
//   RESIDUAL 2 — exported / inferred / ordinal entry exposure. A future method
//     could hand out an entry as `impl Ord` (`StaticNodeId` already derives
//     `Ord`) or as a derived `u32` ordinal, **naming neither private type**, so
//     (a) would still hold while an outside consumer keyed on an entry anyway.
//
// ⛔ **Do not claim that an arbitrary independently maintained entry-keyed
// collection is mechanically detected.** No test enforces that: detecting it
// needs dataflow, not a scan, and a scan can always be spelled around.

/// ⚠ Positive control for the AC-5 detector: it must actually recognise the shape
/// it claims nothing matches, or "no matches" means nothing.
#[cfg(test)]
fn declares_collection_keyed_by_node_id(line: &str) -> bool {
    [
        "BTreeMap<StaticNodeId",
        "BTreeSet<StaticNodeId",
        "HashMap<StaticNodeId",
        "HashSet<StaticNodeId",
    ]
    .iter()
    .any(|shape| line.contains(shape))
}

#[test]
fn the_entry_keyed_collection_detector_catches_the_shape_it_is_looking_for() {
    assert!(declares_collection_keyed_by_node_id(
        "    scheduled: BTreeMap<StaticNodeId, RuntimeExpr>,"
    ));
    assert!(declares_collection_keyed_by_node_id(
        "    seen: BTreeSet<StaticNodeId>,"
    ));
    // The admissible neighbour: keyed by the OCCURRENCE, which B1R's
    // `origin.0 == planned_node.0` bijection makes safe.
    assert!(!declares_collection_keyed_by_node_id(
        "    occurrences: BTreeMap<StaticOriginId, RuntimeExpr>,"
    ));
}

/// **AC-5(a) — the concrete entry-carrying types are module-private.**
///
/// `PlannedExpr` and `StaticNodeId` are declared with **no `pub` modifier**, so
/// they are private to `planning::static_transition` (`StaticNodeId` reaching its
/// own `semantic_ir` child through `use super::`). The set of production files
/// that can *name* either type is therefore exactly those two.
///
/// ## ⚠ What is measured, and what is NOT claimed
///
/// **Measured:** the privacy of two concrete types, i.e. which files can name
/// them. **Not claimed:** that selection authority is confined, or that no
/// outside code can key on a scheduling entry.
///
/// ⛔ **The implication between those two is invalid, and asserting it was my
/// defect** (struck by Steward ruling `evt_4dh098a49cbze`; the earlier version of
/// this test said privacy meant *"none can key on one"* and encoded that claim in
/// its own name). Privacy of a *name* does not confine a *value*: a future method
/// could hand an entry out as `impl Ord` — `StaticNodeId` already derives `Ord` —
/// or as a derived `u32` ordinal, **naming neither private type**, and this test
/// would still pass while an outside consumer keyed on an entry.
///
/// ⇒ This pin is clause **(a)** of four. (b) and (c) are the behavioural and
/// collision controls in the planner; **(d) is Architect review**, and the two
/// residual arms above are what that review covers.
#[test]
fn the_entry_carrying_types_are_module_private() {
    let mut naming = Vec::new();
    for (file, source) in BACKEND_PRODUCTION_SOURCES {
        let production = source
            .split_once("\n#[cfg(test)]\nmod tests {")
            .map_or(*source, |(before, _)| before);
        // Tokenized with comments stripped, so a doc comment MENTIONING the type
        // (as `lowering/core.rs` does, twice, while being unable to name it) does
        // not count.
        let mentions = identifier_occurrences(production, "PlannedExpr")
            + identifier_occurrences(production, "StaticNodeId");
        if mentions > 0 {
            naming.push(*file);
        }
    }
    assert_eq!(
        naming,
        vec![
            "planning/static_transition.rs",
            "planning/static_transition/abi.rs",
            // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — `construction.rs` names
            // `PlannedExpr` because the type's own declaration, and every
            // construction-phase reader of it, moved here with `Planner`'s
            // impl. It is `pub(super)` (root itself reads `.entry`/
            // `.occurrence` off values `plan_static_transition_graph_with_
            // symbols` receives back from `Planner`), never wider -- the
            // reach stays inside `static_transition`'s own module family,
            // the same discipline items 4-9 used for every cross-child
            // surface. It does not cross into `lowering` or beyond.
            "planning/static_transition/construction.rs",
            // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — `closure.rs` names
            // `StaticNodeId` because `planned_entry_body` and the closure/
            // validation family's own occurrence-to-node lookups moved here
            // with most of `StaticTransitionPlan`'s impl. The type remains
            // module-private; only its naming site moved.
            "planning/static_transition/closure.rs",
            // `RT-PLANNER-OCCURRENCES-SPLIT` `D1` — `occurrences.rs` names
            // `StaticNodeId` because `origin_of` maps a node to its occurrence
            // origin (the sole mint site). The type remains module-private;
            // only its naming site moved.
            "planning/static_transition/occurrences.rs",
            "planning/static_transition/semantic_ir.rs",
            // `RT-PLANNER-UNITS-ABI-SPLIT` `D1` — `units.rs` names `StaticNodeId`
            // because `EmittableUnit`'s `planned_node` field moved here from the
            // parent. The type remains module-private; only its naming site
            // moved.
            "planning/static_transition/units.rs",
        ],
        "AC-5(a): another backend file now NAMES an entry-carrying type. That is \
         the measured fact only -- it does not by itself decide whether anything \
         keys on an entry, which is residual arm 2 and Architect review.\n\
         `abi.rs` joined this inventory in `RT-FNSPLIT-B2R`: the ABI plane names \
         `StaticNodeId` because a function unit's frame entry IS its seed node, \
         and the descriptor records which node that is. It remains module-private \
         and is not widened."
    );

    // The naming set is what it is because the declarations stay inside
    // `static_transition`'s own module family. A `pub` reaching `lowering` or
    // beyond would widen it without changing any call; `pub(super)` reaching
    // only root and root's other descendants (exactly `construction.rs`'s
    // case, below) is the standing discipline, not a violation of it.
    let construction = include_str!("../../../planning/static_transition/construction.rs");
    assert!(
        construction.contains("\npub(super) struct PlannedExpr {"),
        "AC-5: `PlannedExpr` must stay confined to `static_transition`'s own \
         module family (`pub(super)` at most), never reach `lowering`"
    );
    let planner = include_str!("../../../planning/static_transition.rs");
    assert!(
        planner.contains("\nstruct StaticNodeId(u32);"),
        "AC-5: `StaticNodeId` must stay module-private"
    );
}

#[test]
fn no_collection_is_keyed_by_a_scheduling_entry() {
    // Over the CLOSED backend surface, not a hand-picked four files: the resolver
    // and the plan are reachable from every backend sibling, so a tripwire scoped
    // to `lowering/` and `planning/` would miss `artifact/**` and `compiled.rs`.
    for (file, source) in BACKEND_PRODUCTION_SOURCES {
        let production = source
            .split_once("\n#[cfg(test)]\nmod tests {")
            .map_or(*source, |(before, _)| before);
        let keyed: Vec<&str> = production
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .filter(|line| {
                declares_collection_keyed_by_node_id(line)
                    // The index form the Architect named: a positional table
                    // subscripted by a scheduling entry rather than an occurrence.
                    || line.contains(".entry.0 as usize")
                    || line.contains("[entry.0 as usize]")
            })
            .collect();
        assert!(
            keyed.is_empty(),
            "{file} keys or indexes by a scheduling entry {keyed:?}; a \
             ComputationalMatch shares its entry with its scrutinee chain, so this \
             merges two occurrences on exactly the fixture that has one. \
             ⚠ NOT an AC-5 clause: this is an early tripwire over enumerated \
             forms, and AC-5 is discharged by (a)-(d) above"
        );
    }
}

// ─── RT-FNSPLIT-B2O D6/D7 — the call population, and inertness ─────────────

/// **`RT-FNSPLIT-B2O` `D6` — the `lower_expr` call population, and why its
/// disposition is now BY OWNER rather than by source site.**
///
/// ⛔ **This is a report, not the authority, and this pin is FROZEN DECLARATION
/// EVIDENCE.** The authority is the ownership mapping in the semantic plane —
/// an occurrence's `StaticOriginId`, its validated `SemanticOwner`, and the
/// planned edge kind.
///
/// ⚠ An earlier revision said this pin existed so the population *"cannot drift
/// silently."* **It does not establish that**, and the claim is withdrawn: the
/// census counts textual occurrences of an identifier, which is a declaration
/// fact. It observes nothing about which Rust functions can reach a retained
/// body. See the `D6` report's UNMECHANIZED section for the four residuals.
///
/// ⚠ **The census is TOKENIZED, not `self.`-spelled.** `grep -c
/// 'self\.lower_expr('` returns **58** and silently loses the program's entry
/// point: the root call is spelled `compiler.lower_expr(` (`core.rs:188`) and
/// takes `root_static_origin`, so it *seeds* the descent rather than traversing.
/// A receiver spelling is a census of the RECEIVER, and the call it misses is the
/// one that matters most.
///
/// ⭐ **Why the count is asserted as two measurements and the 59 is DERIVED.**
/// Freezing "59" directly would be a snapshot. Instead this pins the token total
/// and the definition count, and subtracts — so the pin states the *relation*
/// `calls = tokens - definitions`, and a call added or removed reddens with an
/// arithmetic explanation rather than a bare number mismatch.
///
/// ### The disposition, derived from the ownership mapping
///
/// `B2O` makes a `StaticBody` edge the **one and only** owner boundary. So a call
/// into `lower_expr` crosses an owner boundary **iff the occurrence it lowers is
/// a `StaticBody` target — that is, iff it lowers a retained body.** ⇒ The test
/// is on the **occurrence's owner and the planned edge kind**, and on nothing
/// else.
///
/// ⛔ **Withdrawn here:** that retained bodies are *"reachable only through the
/// single `origin -> expression` route"* and that the population is
/// *"characterised structurally, by one pinned route."*
/// `exactly_one_plan_origin_to_expression_lookup_exists` constrains the
/// identifier `source_occurrence` **only** — it says nothing about who may call
/// `retained_body_occurrence`, so it never supported either sentence.
///
/// ⇒ **The boundary-crossing population is derived from the validated owner
/// partition**, instead of enumerated as a table of source sites. That is the
/// repair for the withdrawn `AC-5`: its two-way site classification had no cell
/// for "depends on the reaching path", so it could have been filled in completely
/// and still been wrong. For the 14 caller-dependent sites the answer genuinely
/// *is* a function of the reaching path — the same parameter carries both a
/// retained body and ordinary sub-expressions — and no per-site row can say that.
/// The **validated owner partition** can: an occurrence's `StaticOriginId`, its
/// `SemanticOwner`, and the planned edge kind answer it per occurrence, which is
/// the only authority here.
// RETIRED by the RT-FNSPLIT-RECUR-PORT successor repair: token counts over
// repository text do not establish occurrence ownership. The semantic-plane
// owner/edge controls above carry that behavioral property.
#[cfg(any())]
fn the_lower_expr_call_population_is_dispositioned_by_owner_not_by_site() {
    // Promise class: durable invariant — a relation over the production source,
    // not a frozen count. `tokens` and `definitions` each move for a stated
    // reason; `calls` is their difference.
    let core = include_str!("../../core.rs");
    let units = include_str!("../../units.rs");
    let tokens = identifier_occurrences(core, "lower_expr")
        + identifier_occurrences(units, "lower_expr");
    let definitions = core
        .lines()
        .chain(units.lines())
        .filter(|line| line.trim_end().ends_with("fn lower_expr("))
        .count();
    assert_eq!(
        definitions, 1,
        "D6: there must be exactly one `lower_expr` definition for the call \
         count to be `tokens - definitions`"
    );
    let calls = tokens - definitions;
    // ⭐ **59 -> 61 on `RT-FNSPLIT-C1` `D3`, then 61 -> 62 on
    // `RT-FNSPLIT-C2-SYNTH-ID`, and the arithmetic is the whole
    // report the pin asks for.** The two added calls are the case-body descents
    // of the two *carried* elimination routes — `lower_carried_match` and
    // `lower_carried_computational_match` — each lowering a case body under a
    // `case_env` whose binders are runtime projections rather than compile-time
    // constructor arguments. C2 adds the HostResult-specific carried case-body
    // descent: the runtime success bit chooses the Result case, while the
    // selected payload remains a carried operand in that case's environment.
    //
    // ⭐ **Neither is a new owner boundary**, which is the disposition this pin
    // actually reports. A carried case body is reached by ordinary descent from
    // the eliminator's own occurrence — `case_body_occurrence(static_origin,
    // index, ..)`, the identical accessor the specialized routes use — so its
    // occurrence's `SemanticOwner` and planned edge kind are unchanged. ⛔ No
    // `StaticBody` edge is introduced, and no retained body is reached by a new
    // path.
    assert_eq!(
        calls, 65,
        "D6: the tokenized production call population into `lower_expr` moved. \
         ⚠ If you reached this by counting `self.lower_expr(` you will have got \
         one fewer -- the root call at `core.rs:188` is spelled \
         `compiler.lower_expr(`"
    );

    // Non-vacuity: the tokenizer must actually see the root call's receiver
    // spelling, or the paragraph above is describing something the pin cannot
    // measure.
    assert!(
        units.contains("compiler.lower_expr("),
        "D6: the functionized root call's spelling is gone, so this census no longer \
         distinguishes the entry point from traversal"
    );

    // ⭐ The DISCRIMINATOR, on a shared input: a non-degenerate pair where the
    // tokenizer and the receiver-spelled scan give different answers. Without
    // this, "use the tokenizer" is advice rather than a checked property — and a
    // positive control that only exercises `self.` would be spelling-scoped in
    // exactly the way that produced 58.
    let both_receivers =
        "let a = self.lower_expr(b, o, e)?;\nlet c = compiler.lower_expr(b, o, e)?;\n";
    assert_eq!(
        identifier_occurrences(both_receivers, "lower_expr"),
        2,
        "the census must count a call regardless of its receiver"
    );
    assert_eq!(
        both_receivers.matches("self.lower_expr(").count(),
        1,
        "if the receiver-spelled scan agreed with the tokenizer here, this pair \
         would not discriminate and would prove nothing about the 58/59 gap"
    );
}

/// **`RT-FNSPLIT-B2O` `D7`/`AC-1` — inertness, as reach rather than as a builder
/// count.**
///
/// The emitted-unit census (`correspondence_adds_no_emitted_unit_to_the_production_census`)
/// already pins `1` builder / `1` definition / `2` declarations in `core.rs` and
/// zero everywhere else, and it counts **source text**, which is why it discharges
/// `AC-1`'s "in BOTH configurations" rather than needing a per-`cfg` variant:
///
/// > **MEASURED:** text occurrences of the builder/definition/declaration forms
/// > across each whole production file, `#[cfg(test)]` regions included.
/// > **CLAIMED:** production emits no new unit under `cfg(test)` or without it.
/// > **THE GAP:** none in the strict direction — any unit emitted in *either*
/// > configuration must appear in the text, so a text census is a superset of
/// > both. It is stricter than the AC, not weaker.
///
/// ⛔ **But a builder census cannot see an executable edge, and it was already
/// zero before this node**, so on its own it is a check that would pass whether
/// or not `B2O` stayed inert.
///
/// ⛔ **Withdrawn:** an earlier revision presented what follows as *"two
/// mechanisms"* proving *"no emission edge is representable."* Neither
/// establishes that, and the pin does not claim it. What this pin is:
///
/// 1. **A visibility inventory (declaration).** `SemanticOwner` is
///    `pub(super)`, and this pin asserts the **allowed inventory** of widened
///    items rather than a forbidden list, so *any* new widening reddens —
///    including one nobody imagined. ⚠ The hatch is not hypothetical:
///    `StaticOriginId` went through it deliberately. ⚠ But visibility bounds
///    **naming**, not reaching: a type is reachable through a method that
///    returns it, an `impl Trait`, or a re-export without ever being named.
/// 2. **A naming inventory (declaration).** `SemanticOwner` appears **zero**
///    times in the production region of every backend source except the file
///    that defines it. This makes a new mention **visible to review**; it is not
///    a proof of unreachability.
///
/// ⇒ **Inertness itself is pinned BEHAVIORALLY**, by
/// `correspondence_adds_no_emitted_unit_to_the_production_census` — that is the
/// mechanism that would actually observe an emission edge. These two are
/// declaration inventories that make a change loud, and that is their whole
/// claim.
#[test]
fn the_owner_classification_has_a_closed_production_naming_inventory() {
    // Promise class: durable invariant — a DECLARATION inventory.
    //
    // ⚠ RENAMED AGAIN by `RT-FNSPLIT-B2R`, and the rename is the honest part.
    // The previous name was `..._is_named_in_production_only_by_the_module_that_
    // defines_it`, and `B2R` **falsified that claim legitimately**: the ABI plane
    // consumes the validated owner partition, which is precisely what the `B2R`
    // frame mandates ("the population is `B2O`'s owner partition, consumed as
    // data"). A pin whose name asserts sole-consumership cannot survive the node
    // that adds the second consumer, and quietly widening the expected list while
    // keeping that name would leave a corrected body under an uncorrected name.
    //
    // ⇒ What is pinned now is the **closed allowed inventory** of production
    // files naming the classification. It still reddens on a *third* consumer —
    // including one nobody imagined — which is the property worth guarding. What
    // it no longer claims is that there is only one.
    //
    // ⚠ RENAMED under the Architect ruling (`evt_5yxjd1zqnyvcq`). This pin was
    // called `..._has_no_reach_into_any_emission_path`, and that name asserted an
    // inference the mechanism cannot make: a type can be *reached* without being
    // *named* — through a method that returns it, an `impl Trait`, a re-export,
    // or a derived ordinal. Naming is not capability. The name now states what
    // is actually measured, because the name is the part future readers quote.
    let mut naming = Vec::new();
    for (file, source) in BACKEND_PRODUCTION_SOURCES {
        // `static_transition.rs` carries its tests inline, and those tests
        // legitimately name the owner classification to exercise it.
        let production = source
            .split_once("\n#[cfg(test)]\nmod tests {")
            .map_or(*source, |(before, _)| before);
        let n = identifier_occurrences(production, "SemanticOwner");
        if n > 0 {
            naming.push(*file);
        }
    }
    assert_eq!(
        naming,
        vec![
            "planning/static_transition/abi.rs",
            "planning/static_transition/semantic_ir.rs",
        ],
        "D7: the owner classification's production naming inventory changed.\n\
         The two permitted members are the module that DEFINES it \
         (`semantic_ir`) and the `B2R` ABI plane (`abi`), which names it to \
         resolve a static-body boundary's CALLEE unit when deriving that \
         boundary's caller-side signature. A third file is a review event: say \
         why that consumer must name the classification rather than take a \
         descriptor.\n\
         ⚠ This membership moved twice inside `RT-FNSPLIT-B2R` and the history \
         is worth one line, because the second move is the load-bearing one. \
         `abi.rs` first named the type in a redundant edge-agreement check that \
         `AC-11` measured as unreachable and deleted -- at which point it left \
         this inventory. The Architect then established that the deleted \
         composition proved target IDENTITY and never layout AGREEMENT, so a \
         real per-boundary signature replaced it, and that mechanism genuinely \
         needs the classification. The name is here now for a live reason, not \
         a vestigial one.\n\
         ⚠ MEASURED: which production files mention the identifier. CLAIMED: \
         exactly that. THE GAP: a mention is not an executable edge and the \
         absence of one is not proof there is none -- a type can be reached \
         without being named. Inertness itself is pinned behaviorally by \
         `correspondence_adds_no_emitted_unit_to_the_production_census`; this \
         pin is a declaration inventory that makes a new mention VISIBLE to \
         review, not a proof of unreachability."
    );

    // The allowed inventory of widened visibility in the plane. ⛔ Asserted as
    // the exact permitted set, not as a scan for a forbidden spelling, so that
    // ANY new widening reddens -- including one nobody imagined.
    //
    // ⚠ This is a DECLARATION inventory. It records which items are widened; it
    // does not entail anything about what is representable or reachable, because
    // visibility bounds NAMING, not reaching.
    let plane = include_str!("../../../planning/static_transition/semantic_ir.rs");
    let widened = plane
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("//"))
        .filter(|line| line.contains("pub(in crate"))
        .collect::<Vec<_>>();
    assert_eq!(
        widened,
        vec![
            // ⭐ `RT-PLANNER-OCCURRENCES-SPLIT` `D1` moved `StaticOriginId` out
            // of `semantic_ir.rs` into the new `occurrences.rs` child, so its
            // widened-visibility row is no longer in this plane's inventory.
            // The type's visibility is unchanged — it is still
            // `pub(in crate::cranelift_backend)` with a `pub(super)` field —
            // only its definition file moved (same shape as the
            // `PredeclaredFunctionId` note below).
            "pub(in crate::cranelift_backend) struct ConstructorIdentity(pub(super) DenseRange);",
            "pub(in crate::cranelift_backend) enum SynthesizedFixedConstructorRole {",
            "pub(in crate::cranelift_backend) struct SynthesizedIoErrorRole(pub(super) u32);",
            "pub(in crate::cranelift_backend) enum SynthesizedConstructorRole {",
            "pub(in crate::cranelift_backend) struct FieldIdentity(pub(super) DenseRange);",
            "pub(in crate::cranelift_backend) fn tag_abi_word(self) -> Result<u64, CraneliftBackendError> {",
            "pub(in crate::cranelift_backend) fn name_abi_word(self) -> Result<u64, CraneliftBackendError> {",
            // ⭐ `RT-FNSPLIT-B2F` `D1` added `PredeclaredFunctionId` here; it
            // moved to `units.rs` under `RT-PLANNER-UNITS-ABI-SPLIT` `D1`, so
            // the widened-visibility row for it is no longer in this plane's
            // inventory. The type's visibility is unchanged — it is still
            // `pub(in crate::cranelift_backend)` with a `pub(super)` field —
            // only its definition file moved.
            "pub(in crate::cranelift_backend) fn with_last_io_error_role_omitted<T>(",
            // ⭐ `RT-DECL-CLOSURE-PORT` `D2a` adds two, and they are the same
            // shape as `with_last_io_error_role_omitted` above: a `cfg(test)`
            // scoped-mutation seam and its closed mode sum. ⛔ Neither widens
            // the plane. `declaration_owned_pairs`,
            // `partition_function_units` and every population derivation stay
            // `pub(super)`, so a consumer can ASK for the pre-`D2a` population
            // inside a scoped control and cannot compute, mint or install one.
            // ⚠ The source tripwire cannot distinguish `cfg(test)`, so it
            // records the seam without claiming production reachability.
            "pub(in crate::cranelift_backend) enum D2aPopulationMutation {",
            "pub(in crate::cranelift_backend) fn with_d2a_population_mutation<T>(",
            // `RT-BODY-OCCURRENCE-PROVENANCE` `AC-3` adds a second scoped
            // mutation seam of exactly the `D2a` shape, and the argument is the
            // same one: it is a `cfg(test)` SCOPE that restores a prior
            // population choice for the duration of one control, never a
            // capability a consumer gains. The enum carries no payload and the
            // scope restores `Exact` on the way out including on panic, so a
            // consumer cannot mint, compute or install a body occurrence
            // through it — it can only ask the planner to re-issue the
            // pre-correction alias and observe the refusal that follows.
            // Same `cfg(test)` caveat as `D2a`: the source tripwire cannot
            // distinguish it, so this records the seam without claiming
            // production reachability.
            "pub(in crate::cranelift_backend) enum BodyOccurrenceMutation {",
            "pub(in crate::cranelift_backend) fn with_body_occurrence_mutation<T>(",
        ],
        "D7: the plane's widened-visibility inventory changed. `StaticOriginId` \
         is widened deliberately so the lowering can carry an occurrence's \
         static name.\n\
         ⭐ `RT-FNSPLIT-C1` `D1`/`D2` adds four members, and the argument for \
         each is the same one that justifies `StaticOriginId`: the widened item \
         is a NAME the lowering may hold, never a CONSTRUCTOR it may use. Both \
         identity newtypes wrap a `pub(super)` field, so a consumer can hold, \
         compare and pass an identity but CANNOT MINT one -- which is what \
         makes `D2`'s single-authority property a fact about the type system \
         rather than about reviewer vigilance. `tag_abi_word`/`name_abi_word` \
         are widened because the carrier's emitted ABI takes a word; they are \
         METHODS ON THE TYPED IDENTITY rather than a shared `u64` conversion, \
         so neither namespace can be erased before the tag-vs-name ABI \
         operation is chosen.\n\
         ⭐ `RT-FNSPLIT-C2-SYNTH-ID` adds the closed fixed-role sum, the opaque \
         dynamic-role token, their closed key sum, and a cfg(test) omission \
         seam. The source tripwire cannot distinguish cfg(test), so it records \
         that seam without claiming production reachability. The IO token's \
         field remains parent-private and lowering can only receive one from \
         the plan.\n\
         ⛔ What is NOT widened, and is the thing this pin most needs to keep \
         catching: `SemanticPlane` and its `names` arena stay `pub(super)`. The \
         Architect's ruling forbids resolving a consumer's need by widening the \
         plane, and `D1` is deliberately a capability export instead. A future \
         `SemanticPlane` or `names` line appearing in this list is the \
         violation, not a fifth capability.\n\
         ⚠ This is a DECLARATION inventory, not a proof of inertness: a \
         widening of the OWNER surface is a DELIBERATE REVIEW EVENT that must \
         be argued here, not absorbed. It entails nothing by itself about what \
         is representable or reachable -- inertness is pinned behaviorally by \
         `correspondence_adds_no_emitted_unit_to_the_production_census`"
    );

    // Non-vacuity: the needle must occur somewhere, or both assertions above are
    // satisfied by a typo.
    assert!(
        identifier_occurrences(plane, "SemanticOwner") > 0,
        "the owner classification is not in the plane at all, so this pin is \
         measuring nothing"
    );
}

// ── RT-MATCH-FRAME-FP: the identity selector and its permutation net ───────
//
// `dec_s30rdnb1dvgk`. `AC-F1` makes two frames that differ only in a
// closure-bearing body share one header fingerprint, so a fingerprint can no
// longer say *which* occurrence is being checked. Identity is transported;
// the fingerprint is a compatibility check only.
//
// ⚠ The fixture below is the reachable shape, not a contrived one: `erasure.rs`
// derives the case headers from the eliminated family and builds the default as
// `format!("no runtime match case selected for {family_symbol}")`. Two
// eliminations of one family in one declaration therefore agree on every field
// a header fingerprint can see.

/// Hard-stop #18 row 2 — an out-of-node-order declaration call reaches the
/// declaration-owned unit after semantic-source positioning.
#[test]
fn computational_match_declaration_ref_emits_and_runs_the_declaration_owned_unit() {
    // Promise class: durable invariant.
    //
    // MEASURED: the real FunctionizedUnits compiler emits two complete units,
    // resolves one typed declaration-call edge, and running the artifact returns
    // the declaration body's unique value.
    // CLAIMED: a transparent non-closure DeclarationRef nested in the exposing
    // ComputationalMatch shape calls its already-owned unit.
    // THE GAP: counts alone cannot prove which unit ran, so the returned `73`
    // is load-bearing and differs from every value in the match scrutinee.
    let symbol = "decl:fixture::row2::value".to_string();
    let declaration = RuntimeDeclaration {
        symbol: symbol.clone(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::Value(RuntimeValue::Int((73).into())),
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };
    let expr = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::Row2::Node".to_string(),
            args: vec![RuntimeExpr::Value(RuntimeValue::Bool(false))],
        }),
        cases: vec![crate::RuntimeComputationalMatchCase {
            constructor: "ctor:fixture::Row2::Node".to_string(),
            argument_binders: 1,
            recursive_positions: Vec::new(),
            body: RuntimeExpr::DeclarationRef {
                symbol: symbol.clone(),
            },
        }],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "row-2 declaration fixture is total".to_string(),
        },
    };
    let compiled = compile_expr_into_module(
        new_jit_module().expect("JIT module"),
        "row2_out_of_order_declaration_call",
        Linkage::Local,
        &expr,
        &NativeSeedEnvironment::empty(),
        BTreeMap::from([(symbol.as_str(), &declaration)]),
        None,
        false,
        None,
        None,
        None,
    )
    .expect("the out-of-order declaration call emits");

    assert_eq!(
        crate::cranelift_backend::lowering::units::b2f_last_unit_emission(),
        (2, 2),
        "the root and transparent declaration must each emit one complete unit"
    );
    assert_eq!(
        crate::cranelift_backend::lowering::units::b2f_last_call_edge_resolution(),
        1,
        "the exact DeclarationRef occurrence must resolve one typed call edge"
    );
    assert_eq!(
        compiled.run(None).expect("the emitted call runs").0,
        RuntimeObservation::Returned(RuntimeGroundValue::Int((73).into())),
        "the caller ran some path other than the declaration-owned unit"
    );
}

/// **`RT-FNSPLIT-B2F` `AC-6` — the removal pin, authored BEFORE the removal so
/// that it can witness it.**
///
/// ⛔⛔ **A pin authored after a removal cannot witness it, and the tests a ban
/// reddens on introduction never contain its witness — they exercise the success
/// path.** So this lands on the green pre-`D6` base, asserting what is true
/// *now*, shaped so that `D6` turns it red and forces the flip to be reviewed.
///
/// ⭐ **The property is symptom-inventory entry 2 itself, measured rather than
/// described:** today a retained body is re-lowered **once per call site**, not
/// once per body. One `LexicalClosure` occurrence, bound once and applied twice,
/// resolves its origin **twice** — the same source term is walked and emitted
/// again for the second application.
///
/// ⛔ **Stated as a RELATION between two programs, never as the literals.** The
/// absolute counts move for reasons that have nothing to do with `D6` — a
/// scheduling change, an extra planned occurrence, a different `Let` shape. What
/// cannot move without `D6` is whether the count **follows the number of call
/// sites**:
///
/// | | today (inliner) | after `D6` (unit + call) |
/// |---|---|---|
/// | applied once | `n` | `n` |
/// | applied twice | `n + 1` | `n` |
///
/// **MEASURED:** the number of `origin -> expression` resolutions a compile
/// performs grows by one when a single retained closure occurrence gains a
/// second application site.
/// **CLAIMED:** `lower_expr`'s recursive descent still emits a retained body per
/// call site — i.e. the inliner `D6` removes is **present**.
/// **THE GAP:** ⚠ a resolution is not an emission. This counts how many times
/// the body's *term* was fetched, which is one-for-one with re-lowering under
/// the current descent but ⛔ is **not** claimed to remain one-for-one under any
/// other. ⇒ When `D6` lands, whoever flips this must re-check that the
/// replacement reading means what they think — the flip is not mechanical.
///
/// **Promise class: TRANSITION SENTINEL — deliberately, and labelled for the
/// boundary rather than the count.** ⭐ **The event that retires it is `D6`:**
/// removal of the recursive-descent emission authority in `lower_expr`, at which
/// point a retained body is emitted **once, into its own unit**, and both rows
/// of the table above read `n`. ⇒ On that day this assertion becomes
/// `assert_eq!(twice, once)` — a **durable invariant**, since no intended
/// extension may reintroduce per-call-site emission.
///
/// ⛔ **Do not "fix" a red here by deleting the test or by widening it to accept
/// both readings.** A sentinel that accepts its own retirement silently is not a
/// sentinel; the red IS the deliverable.
///
/// **The retirement was SIMULATED and the sentinel does redden — run, not
/// reasoned.** Counting one resolution per **distinct** origin instead of per
/// call (which is exactly the post-`D6` reading, since a unit's body is fetched
/// once however often it is called) produces `left: 1, right: 2` and the
/// "D6 HAS LANDED" message above.
///
/// ⚠ **Labelled precisely: that is a simulation of the retirement event, NOT a
/// compile-preserving evasion of this pin.** It mutates the instrument, not the
/// descent. ⛔ What it demonstrates is only that the assertion **discriminates
/// the two worlds** — it is not evidence that `D6` will produce that reading by
/// this mechanism, and the `THE GAP` paragraph above is what governs that.
#[test]
fn a_retained_body_is_defined_once_even_when_called_twice() {
    fn resolutions(expr: &RuntimeExpr) -> usize {
        crate::cranelift_backend::planning::ac4_open_route_window();
        ac11_compiles(expr).expect("fixture compiles");
        crate::cranelift_backend::planning::ac4_route_counts().0
    }

    let closure = || RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: Vec::new(),
        body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
    };

    // ONE closure occurrence, bound once, applied once.
    let applied_once = RuntimeExpr::Let {
        value: Box::new(closure()),
        body: Box::new(RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Var(0)),
            args: Vec::new(),
        }),
    };
    // ⭐ The SAME single closure occurrence, applied twice. ⛔ Not two closure
    // literals: two literals are two distinct origins and would legitimately
    // resolve twice even after `D6`, which would make this green for the wrong
    // reason forever.
    let applied_twice = RuntimeExpr::Let {
        value: Box::new(closure()),
        body: Box::new(RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Var(0)),
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Var(1)),
                args: Vec::new(),
            }),
        }),
    };

    let once = resolutions(&applied_once);
    let twice = resolutions(&applied_twice);

    // ⛔ NON-VACUITY: a harness that never compiled anything reports `0 == 0`
    // and would satisfy the relation below by doing nothing at all.
    assert!(
        once > 0,
        "AC-6 -- NON-VACUITY: a program with a retained closure body must \
         resolve its origin at least once; got {once}. A zero here means this \
         test measures nothing, whatever the relation below reports."
    );
    assert_eq!(
        twice,
        once,
        "AC-6 -- one retained closure occurrence applied twice performed \
         {twice} origin->expression resolutions against {once} when applied \
         once. The selected functionized authority must define that retained \
         body once; a second call may add a call edge, never a second body \
         resolution."
    );
}

// ─── RT-CONTSPEC-ACTIVATE `D4` — the one-token emission-seam controls ────────

#[test]
fn a_closure_stored_as_constructor_data_cannot_cross_a_unit_boundary() {
    let declaration = |arg: RuntimeExpr| RuntimeDeclaration {
        symbol: "decl:fixture::d6::probe".to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["n".to_string()],
                body: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::D6::Wrap".to_string(),
                    args: vec![arg],
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };
    let plain = declaration(RuntimeExpr::Value(RuntimeValue::Int((1).into())));
    let closure_field = declaration(RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: vec!["r".to_string()],
        body: Box::new(RuntimeExpr::Value(RuntimeValue::Int((2).into()))),
    });
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: "decl:fixture::d6::probe".to_string(),
        }),
        args: vec![RuntimeExpr::Value(RuntimeValue::Int((5).into()))],
    };
    let compile = |decl: &RuntimeDeclaration| {
        let declarations = BTreeMap::from([("decl:fixture::d6::probe", decl)]);
        compile_expr_into_module(
            new_jit_module().expect("JIT module"),
            "d6_activation_blocker",
            Linkage::Local,
            &entry,
            &NativeSeedEnvironment::empty(),
            declarations,
            None,
            false,
            None,
            None,
            None,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
    };

    // The positive control on the harness. ⛔ Without it, the refusal below
    // is equally consistent with the lane being broken for every
    // constructor, and the finding would name the wrong thing.
    compile(&plain).expect(
        "a closure-bodied declaration whose constructor field is an ordinary \
         value compiles on the functionized lane — so the lane, the witness \
         and the declaration shape are all fine",
    );

    let refusal = compile(&closure_field).expect_err(
        "a closure stored as observable constructor data must be refused. \
         This is the generic escape prohibition, not a missing capability — \
         if it ever compiles, a carrier lane has appeared that nothing \
         authorized",
    );
    assert!(
        refusal.contains("a closure cannot cross the boundary"),
        "the refusal must be the closure-boundary one. A DIFFERENT refusal \
         would mean this row stopped measuring the escape prohibition: \
         {refusal}"
    );
}

/// `RT-REFUSAL-PINS-REHOMED` D1-D3: a constructed static worker that never
/// enters binding authority is refused by the conservation ledger directly,
/// without selecting either body-emission lane.
///
/// MEASURED: one real recognition followed immediately by `close` returns the
/// `StaticWorkerBinding` refusal and names the unconsumed callable consequence.
/// CLAIMED: construction creates an obligation that only an exact transition
/// and consumption can discharge; lane selection is not part of that law.
/// THE GAP: this pins the construct-level refusal, not the two fixture-only
/// programs that previously reached it through selector exclusion.
///
/// Promise class: durable invariant. Removing the retiring lane leaves this
/// conservation law unchanged.
///
/// This is the sole carrier of a ratified refusal. If it reds, ask whether the
/// refusal changed, not whether the string changed; do not paste a new message
/// into the expected value without answering that question.
#[test]
fn refusal_pins_rehomed_static_worker_without_selector_exclusion() {
    use crate::cranelift_backend::lowering::{FuncId, StaticWorkerFieldLedger};

    let constructor = "ctor:fixture::RefusalPin::Mk";
    let position = 0;
    let owner_expr = RuntimeExpr::Construct {
        constructor: constructor.to_string(),
        args: vec![RuntimeExpr::Var(0)],
    };
    let (plan, owner) = planned_root_occurrence(&owner_expr);
    let field = plan
        .child_static_origin(owner, position)
        .expect("the constructor plans its worker field");
    let mut ledger = StaticWorkerFieldLedger::default();
    let recognition = ledger
        .recognize(
            owner,
            position,
            field,
            constructor,
            Some(FuncId::from_u32(0)),
        )
        .expect("the real issuer mints the recognition");
    let expected_reason = format!(
        "constructor {} at origin {:?} transports a static worker in field {} \
         (field origin {:?}, recognition {recognition:?}) that no static \
         elimination rebinds, so this recognition's own transport never reaches \
         a consumer at an exact-Var call and is not erased; a constructor carrying \
         an unconsumed static worker denotes a value containing the callable and \
         has no runtime representation",
        constructor, owner, position, field
    );

    let refused = ledger
        .close()
        .expect_err("a constructed worker with no transition must be refused");
    assert!(matches!(
        refused,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "StaticWorkerBinding",
            reason,
        }) if reason == expected_reason
    ));
}


// ── `RT-BRANCH-LOCAL-DECLARED-CALLABLE` `D1` — AC-1, the seam property ────────
//
// ⭐⭐ These are PROPERTY-LEVEL IR FIXTURES, and that is not a convenience.
// A capture-free lambda at a nested recursive position is **not authorable from
// surface Ken**: the elaborator gives every expression-position lambda
// `captures: (0..runtime_depth).map(Var)` — the whole enclosing environment,
// with no free-variable analysis (`ken-elaborator/src/erasure.rs:2210`, `2513`,
// `4441`), so a continuation that references nothing still captures its depth.
// Measured on a purpose-built Ken program whose continuation ignores its
// argument and names only a top-level `proc`: still `captures = 5`. The
// unconditionally-admitted `RuntimeExpr::Closure { captures: vec![] }` is only
// ever emitted as a DECLARATION body (`erasure.rs:283`, `2150`, `4068`), never
// at a constructor argument. ⇒ The IR is the only layer at which this property
// has a witness at all.
//
// ⛔ The two tests are a PAIR on a shared producer shape, and the pair is the
// point: they differ in the capture list and nothing else, so a partition that
// widened into the capture-bearing case would flip the second while leaving the
// first green.

/// `Ret`/`Vis` over a shared producer, parameterised only by the recursive
/// position's capture list.
///
/// Arm 0 constructs `Ret` with **one** argument, so position 1 is absent from
/// it — that is precisely the whole-source veto this node was cut to remove.
/// Arm 1 constructs `Vis`, the selected constructor, and carries the position.
#[cfg(test)]
fn d1_ret_vis_producer(captures: Vec<RuntimeExpr>) -> RuntimeExpr {
    let ret = "ctor:fixture::ITree::Ret";
    let vis = "ctor:fixture::ITree::Vis";
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            cases: vec![
                RuntimeMatchCase {
                    constructor: "ctor:fixture::Result::Err".to_string(),
                    binders: 1,
                    body: RuntimeExpr::Construct {
                        constructor: ret.to_string(),
                        args: vec![RuntimeExpr::Value(RuntimeValue::Int(0.into()))],
                    },
                },
                RuntimeMatchCase {
                    constructor: "ctor:fixture::Result::Ok".to_string(),
                    binders: 1,
                    body: RuntimeExpr::Construct {
                        constructor: vis.to_string(),
                        args: vec![
                            RuntimeExpr::Value(RuntimeValue::Int(1.into())),
                            RuntimeExpr::LexicalClosure {
                                captures,
                                params: vec!["arg0".to_string()],
                                body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(7.into()))),
                            },
                        ],
                    },
                },
            ],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "d1 producer default".to_string(),
            },
        }),
        cases: vec![crate::RuntimeComputationalMatchCase {
            constructor: vis.to_string(),
            argument_binders: 2,
            recursive_positions: vec![1],
            body: RuntimeExpr::Var(0),
        }],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d1 eliminator default".to_string(),
        },
    }
}

#[test]
fn the_branch_local_partition_mints_a_declared_body_for_a_capture_free_recursive_position() {
    let fixture = d1_ret_vis_producer(Vec::new());
    let seed_env = NativeSeedEnvironment::empty();
    let (plan, root) = planned_root_occurrence(&fixture);
    let mut lowering = root_authority_test_lowering(&seed_env);
    lowering.static_transition_plan = plan;

    let minted = lowering
        .recursive_position_unit_body(root, 1, &"ctor:fixture::ITree::Vis".to_string())
        .expect("the fixture is plannable and the resolver must not error");

    // ⭐ The MINTING path, not merely a lifted veto. Before the partition this
    // was `None`, because arm 0 constructs `Ret` and lacks position 1.
    let minted = minted.expect(
        "the branch-local partition must mint a declared body for the in-bucket \
         capture-free recursive position",
    );

    // ⛔ It must be the CLOSURE'S BODY, not merely some origin. An assertion of
    // `is_some()` alone would pass on any origin the resolver happened to hold.
    let scrutinee = lowering
        .static_transition_plan
        .child_static_origin(root, 0)
        .expect("the eliminator's scrutinee has a planned origin");
    let ok_arm = lowering
        .static_transition_plan
        .child_static_origin(scrutinee, 2)
        .expect("the producer's second arm has a planned origin");
    let closure = lowering
        .static_transition_plan
        .child_static_origin(ok_arm, 1)
        .expect("the Vis constructor's recursive position has a planned origin");
    let closure_body = lowering
        .static_transition_plan
        .child_static_origin(closure, 0)
        .expect("the closure's body has a planned origin");
    assert_eq!(
        minted, closure_body,
        "the minted unit must be the in-bucket closure's own body"
    );
}

#[test]
fn the_partition_still_refuses_a_capture_bearing_recursive_position() {
    // The discriminating half of the pair: identical in every respect except a
    // non-empty capture list. This is the case all sixteen RT-BRANCH witnesses
    // are in, and RT-CAPTURE-SUPPLY closed as word-only.
    let fixture = d1_ret_vis_producer(vec![RuntimeExpr::Var(0)]);
    let seed_env = NativeSeedEnvironment::empty();
    let (plan, root) = planned_root_occurrence(&fixture);
    let mut lowering = root_authority_test_lowering(&seed_env);
    lowering.static_transition_plan = plan;

    let minted = lowering
        .recursive_position_unit_body(root, 1, &"ctor:fixture::ITree::Vis".to_string())
        .expect("the fixture is plannable and the resolver must not error");

    assert!(
        minted.is_none(),
        "a capture-bearing LexicalClosure must still refuse — the partition is a \
         bucket cut, never a relaxation of the capture condition"
    );
}
