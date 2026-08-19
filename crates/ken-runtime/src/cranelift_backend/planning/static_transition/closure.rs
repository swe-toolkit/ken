//! The planner-owned closure lifecycle -- validation and closure, and
//! read-only projections.
//!
//! `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` -- this module owns most of
//! `StaticTransitionPlan`'s own impl: `validate()` and everything it calls
//! (`validate_source_return_topology`, `activation_successor`,
//! `require_only_outgoing_edge`/`require_only_incoming_edge`, the
//! case-producer-authority / case-emission family, the static-worker-member
//! mutation family, `validate_substrate_preallocation_closure`), plus the
//! read-only projection surface lowering and the sibling domains read after
//! the plan is closed (`census`, `semantic_census`,
//! `process_parameter_slot`, and the rest). `StaticTransitionPlan` itself
//! stays declared in the parent (it is the LCA of this module and
//! `construction.rs`); this module's own `impl StaticTransitionPlan`
//! fragments hold every method here under the standing child-module pattern
//! (item 4's `units.rs` precedent).
//!
//! The construction half -- `Planner<'src>`'s own graph-construction machine
//! and its two registration-write-path `StaticTransitionPlan` methods -- is a
//! DIFFERENT lifecycle entirely and lives in `construction.rs`; see the `D0`
//! ledger's outcome-2 determination in
//! `docs/program/issues/RT-PLANNER-ROOT-CLOSURE-SPLIT.md`.

#[cfg(test)]
use super::construction::max_recursive_lowering_frame_count;
use std::collections::{BTreeMap, BTreeSet};

use super::abi::{
    AbiProcessParameter, AbiSchedulingIngress, AbiSlot, AbiSlotKind, AbiUnitDefinition,
};
#[cfg(test)]
#[allow(unused_imports)]
use super::aggregates::{
    aggregate_child_referent_owners, fixed_node_selected_owner, flatten_allocation_reachable_uses,
    host_effect_recipe_tree, node_referent_owners, validate_aggregate_producers_are_unique,
    SynthesizedAggregateStep,
};
#[allow(unused_imports)]
use super::aggregates::{
    build_aggregate_ownership_plan, lifetime_referent_affinity, validate_aggregate_ownership_plan,
    AggregateOccurrenceId, AggregateOccurrenceProducer, PlannedAggregateAllocation,
    PlannedAggregateOwnership, PlannedAggregateShape, SynthesizedAggregateNode,
    SynthesizedAggregatePath, SynthesizedAggregateRole, SynthesizedAggregateRoot,
    SynthesizedDynamicSet,
};
use super::continuations::validate_continuation_specialization_plan;
#[allow(unused_imports)]
use super::continuations::{
    build_static_continuation_fusion_plan, fusion_redirect_target,
    verify_current_lexical_availability, verify_predeclared_entry_frame_membership,
    AdmittedContinuationDiscovery, BodyEmissionDisposition, CheckedCaseBinderLayout,
    CheckedCaseBinderRole, CheckedIhBinding, CheckedTransportCoordinate, ComposedCallTarget,
    ComposedWorkerRouteEligibility, ComposedWorkerView, ContinuationAvailabilityDraft,
    ContinuationAvailabilityOver, ContinuationAvailabilityViews, ContinuationCallIdentity,
    ContinuationCallView, ContinuationConsumingOccurrence, ContinuationContextId,
    ContinuationContextView, ContinuationEmissionOwner, ContinuationEnvironmentClaim,
    ContinuationEnvironmentClaimOver, ContinuationEnvironmentDraft, ContinuationFrameIdentity,
    ContinuationFrameRequirement, ContinuationInputSource, ContinuationInputView,
    ContinuationOrdinaryEnvelopeRole, ContinuationResultEdge, ContinuationSourceCoordinate,
    ContinuationSourceSlotAuthority, ContinuationSpecializationId, ContinuationUnitView,
    ContinuationWorkerCaptureProvenance, ContinuationWorkerCaptureSource, FusionClaimRefusal,
    FusionComposedEdge, FusionCompositionLayer, FusionOwnedBody, FusionOwnedOuterRealization,
    FusionRegionClaim, FusionRegionClaimLedger, PlannedContinuationContext, ProducerLocalBinding,
    ProducerLocalLocator, RequiredConsumerProjection, StaticContinuationFusionCandidate,
    StaticContinuationFusionDescriptor, StaticContinuationFusionId, StaticContinuationFusionKey,
    StaticContinuationFusionPlan, StaticContinuationFusionView,
};
#[cfg(test)]
#[allow(unused_imports)]
use super::continuations::{
    validate_continuation_specialization_closure, ContinuationInternMutation,
    ContinuationProductionMutation, ContinuationProjectionOmission, COMPOSED_CALL_TARGET_DEFECT,
    CONTINUATION_INTERN_MUTATION, CONTINUATION_PRODUCTION_MUTATION, DUPLICATE_STATIC_BODY_TRIPLE,
    ENVELOPE_DEFECT, SUPPRESS_POST_SPECIALIZATION_DESCENT, WEAKEN_CONTINUATION_DECREASING_MEASURE,
};
#[allow(unused_imports)]
use super::effects::{
    build_host_effect_seat_plan, host_effect_seat_contract_of, validate_host_effect_seat_plan,
    EffectSeatAvail, EffectSeatNeed, EffectSeatOperation, EffectSeatPhase, EffectSeatSlot,
    PlannedEffectSeat, CRANELIFT_HOST_EFFECT_CONSUMERS_V1,
};
#[allow(unused_imports)]
use super::joins_traps::{
    build_join_result_plan, planned_partiality_trap, JoinPlanToken, JoinResultRepresentation,
    PlannedJoinResult,
};
use super::occurrences::{
    validate_occurrence_authority_plan, PlannedOccurrenceAuthority, StaticOriginId,
};
use super::semantic_ir::{
    self, ConstructorIdentity, FieldIdentity, SemanticSourceKind, SynthesizedConstructorRole,
    SynthesizedIoErrorRole,
};
use super::units::{EmittableCallKind, PredeclaredFunctionId};
#[cfg_attr(not(test), allow(unused_imports))]
use super::PersistentStoreNode;
use super::{
    planner_capacity_error, planner_error, CraneliftBackendError, DeclarationCallTargetClass,
    DynamicActivationFrame, EdgeKind, PersistentNodeId, PlannedHelperKey, StaticNode, StaticNodeId,
    StaticSourceId, StaticTransitionPlan, StoreKind, TransitionKind,
};
use crate::RuntimeExpr;

pub(super) const MAX_HELPERS_PER_STATIC_SOURCE: usize = 8;

/// A closed producer result for one exact match scrutinee.
///
/// `Open` is positive fail-closed authority: at least one result route is not
/// statically known. An empty `Closed` set means the expression cannot return
/// normally, not that the analysis forgot to inspect it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum CaseProducerSet {
    Open,
    Closed(Vec<ConstructorIdentity>),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CaseProducerFlowKind {
    Construct,
    Forward,
    Alternative,
    Environment,
    OpaqueIngress,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CaseProducerFlowEdge {
    from: StaticOriginId,
    to: StaticOriginId,
    kind: CaseProducerFlowKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CaseProducerAuthority {
    pub(super) producers: CaseProducerSet,
    producer_origins: Vec<(ConstructorIdentity, BTreeSet<StaticOriginId>)>,
    flow: BTreeSet<CaseProducerFlowEdge>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum CaseEmissionStatus {
    Reachable,
    Eliminated,
}

/// `D1`: one exact case-emission verdict, retained as dormant planner data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlannedCaseEmission {
    pub(super) match_origin: StaticOriginId,
    pub(super) scrutinee_origin: StaticOriginId,
    owner: PredeclaredFunctionId,
    ordinal: u32,
    pub(super) body_origin: StaticOriginId,
    constructor: ConstructorIdentity,
    pub(super) authority: CaseProducerAuthority,
    pub(super) status: CaseEmissionStatus,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct BoundaryACensus {
    pub(super) static_nodes: usize,
    pub(super) edges: usize,
    pub(super) planned_helpers: usize,
    pub(super) persistent_store_nodes: usize,
    pub(super) out_of_line_evidence_records: usize,
    pub(super) max_helpers_per_static_source: usize,
    pub(super) helper_key_bytes: usize,
    pub(super) activation_frame_bytes: usize,
    pub(super) store_node_bytes: usize,
    pub(super) helper_key_schemas: usize,
    pub(super) frame_schemas: usize,
    pub(super) store_node_schemas: usize,
    pub(super) static_node_id_bytes: usize,
    pub(super) persistent_node_id_bytes: usize,
    pub(super) max_logical_chain_depth: u32,
    pub(super) max_environment_depth: u32,
    pub(super) max_continuation_depth: u32,
    pub(super) max_path_depth: u32,
    pub(super) max_cleanup_depth: u32,
    pub(super) max_affine_depth: u32,
    pub(super) max_source_return_depth: u32,
    pub(super) source_return_resume_nodes: usize,
    pub(super) source_return_owned_resume_edges: usize,
    pub(super) terminal_outgoing_edges: usize,
    pub(super) recursive_lowering_frames: usize,
}

/// The planner-side material retained until one completed FunctionizedUnits
/// emission can be measured at Boundary B.
///
/// Unlike [`BoundaryACensus`], this is not itself a result row.  The lowering
/// collector takes this snapshot from the exact plan it subsequently emits and
/// publishes it only after every production CLIF body has been defined.
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ScaleBPlanCensus {
    pub(in crate::cranelift_backend) static_nodes: usize,
    pub(in crate::cranelift_backend) edges: usize,
    pub(in crate::cranelift_backend) planned_helpers: usize,
    pub(in crate::cranelift_backend) persistent_store_nodes: usize,
    pub(in crate::cranelift_backend) out_of_line_evidence_records: usize,
    pub(in crate::cranelift_backend) max_helpers_per_static_source: usize,
    pub(in crate::cranelift_backend) helper_key_bytes: usize,
    pub(in crate::cranelift_backend) activation_frame_bytes: usize,
    pub(in crate::cranelift_backend) store_node_bytes: usize,
    pub(in crate::cranelift_backend) helper_key_schemas: usize,
    pub(in crate::cranelift_backend) frame_schemas: usize,
    pub(in crate::cranelift_backend) store_node_schemas: usize,
    pub(in crate::cranelift_backend) static_node_id_bytes: usize,
    pub(in crate::cranelift_backend) persistent_node_id_bytes: usize,
    pub(in crate::cranelift_backend) max_logical_chain_depth: u32,
    pub(in crate::cranelift_backend) max_environment_depth: u32,
    pub(in crate::cranelift_backend) max_continuation_depth: u32,
    pub(in crate::cranelift_backend) max_path_depth: u32,
    pub(in crate::cranelift_backend) max_cleanup_depth: u32,
    pub(in crate::cranelift_backend) max_affine_depth: u32,
    pub(in crate::cranelift_backend) max_source_return_depth: u32,
    pub(in crate::cranelift_backend) source_return_resume_nodes: usize,
    pub(in crate::cranelift_backend) source_return_owned_resume_edges: usize,
    pub(in crate::cranelift_backend) terminal_outgoing_edges: usize,
    pub(in crate::cranelift_backend) recursive_lowering_frames: usize,
    pub(in crate::cranelift_backend) distinct_interned_semantic_states: usize,
    pub(in crate::cranelift_backend) defined_helpers: usize,
    pub(in crate::cranelift_backend) descriptor_construction_work: usize,
    pub(in crate::cranelift_backend) descriptor_comparison_work: usize,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct BoundaryB1Census {
    pub(super) opcode_vocabulary: usize,
    pub(super) distinct_origins: usize,
    pub(super) ir_records: usize,
    pub(super) semantic_edges: usize,
    /// The number of **function units** — `entries.len() + count(StaticBody
    /// edges) - count(declaration-owned pairs)` since `RT-DECL-CLOSURE-PORT`
    /// `D2a`. ⛔ The subtraction is not a rounding detail: without it a
    /// closure-seed transparent declaration contributes a second, unreachable
    /// zero-input function whose body is a closure that cannot cross a unit
    /// boundary. Old form, for readers of earlier evidence:
    /// `entries.len() + count(StaticBody edges)`. Renamed from
    /// `helper_definitions` by `RT-FNSPLIT-B2O` `AC-6`.
    ///
    /// ⚠ **This field is the one re-baselined quantity that cannot fail
    /// loudly, which is why the rename is an acceptance criterion and not
    /// tidiness.** It is a *reported metric*: its only consumer asserts that the
    /// second finite difference across `n = 3..7` is zero — affine scaling — and
    /// asserts **no absolute value**. `RT-FNSPLIT-B2O` changed this quantity from
    /// "one definition per planned node" to "one per function unit"; both are
    /// affine in `n`, so that assertion passed before and after. Nothing failed
    /// and nothing warned. A name still reporting `helper_definitions` for a
    /// number whose meaning had changed underneath it would be worse than a
    /// rename precisely because there is no red to notice.
    pub(super) function_units: usize,
    pub(super) definitions_per_origin: usize,
    pub(super) all_out_of_line_operand_elements: usize,
    pub(super) duplicate_origin_definitions: usize,
    pub(super) post_origin_clones: usize,
    pub(super) max_definitions_per_origin: usize,
    pub(super) descriptor_bytes: usize,
    pub(super) program_bytes: usize,
    pub(super) record_bytes: usize,
    pub(super) operand_element_bytes: usize,
    pub(super) capture_layout_bytes: usize,
    pub(super) capture_slot_bytes: usize,
    pub(super) ruled_child_bytes: usize,
    pub(super) function_bytes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CaseProducerFact {
    pub(super) producers: CaseProducerSet,
    pub(super) producer_origins: Vec<(ConstructorIdentity, BTreeSet<StaticOriginId>)>,
    flow: BTreeSet<CaseProducerFlowEdge>,
    frontier: BTreeSet<StaticOriginId>,
}

/// Test-only plan-side classification of one field on a closed constructor
/// result. This names the source expression kind which the planner's existing
/// producer-flow analysis selected; it does not inspect a lowered value.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum PlannedResultFieldKindForTest {
    Closure,
    LexicalClosure,
    Other,
    Absent,
}

impl CaseProducerFact {
    fn open(origin: StaticOriginId) -> Self {
        Self {
            producers: CaseProducerSet::Open,
            producer_origins: Vec::new(),
            flow: BTreeSet::from([CaseProducerFlowEdge {
                from: origin,
                to: origin,
                kind: CaseProducerFlowKind::OpaqueIngress,
            }]),
            frontier: BTreeSet::from([origin]),
        }
    }

    fn empty() -> Self {
        Self {
            producers: CaseProducerSet::Closed(Vec::new()),
            producer_origins: Vec::new(),
            flow: BTreeSet::new(),
            frontier: BTreeSet::new(),
        }
    }

    fn constructor(origin: StaticOriginId, identity: ConstructorIdentity) -> Self {
        Self {
            producers: CaseProducerSet::Closed(vec![identity]),
            producer_origins: vec![(identity, BTreeSet::from([origin]))],
            flow: BTreeSet::from([CaseProducerFlowEdge {
                from: origin,
                to: origin,
                kind: CaseProducerFlowKind::Construct,
            }]),
            frontier: BTreeSet::from([origin]),
        }
    }

    fn join(&self, other: &Self) -> Self {
        let producers = match (&self.producers, &other.producers) {
            (CaseProducerSet::Open, _) | (_, CaseProducerSet::Open) => CaseProducerSet::Open,
            (CaseProducerSet::Closed(left), CaseProducerSet::Closed(right)) => {
                let mut union = left.clone();
                for identity in right {
                    if !union.contains(identity) {
                        union.push(*identity);
                    }
                }
                CaseProducerSet::Closed(union)
            }
        };
        let mut producer_origins = self.producer_origins.clone();
        for (identity, origins) in &other.producer_origins {
            if let Some((_, known)) = producer_origins
                .iter_mut()
                .find(|(known, _)| known == identity)
            {
                known.extend(origins.iter().copied());
            } else {
                producer_origins.push((*identity, origins.clone()));
            }
        }
        Self {
            producers,
            producer_origins,
            flow: self.flow.union(&other.flow).copied().collect(),
            frontier: self.frontier.union(&other.frontier).copied().collect(),
        }
    }

    fn forwarded(mut self, to: StaticOriginId, kind: CaseProducerFlowKind) -> Self {
        for from in self.frontier.iter().copied() {
            self.flow.insert(CaseProducerFlowEdge { from, to, kind });
        }
        self.frontier.clear();
        self.frontier.insert(to);
        self
    }

    fn authority(&self) -> CaseProducerAuthority {
        CaseProducerAuthority {
            producers: self.producers.clone(),
            producer_origins: self.producer_origins.clone(),
            flow: self.flow.clone(),
        }
    }
}

/// Re-derive the constructor result of one exact source occurrence.
///
/// This is deliberately conservative at opaque calls, effects, projections,
/// values and outer lexical variables. `Open` keeps every case reachable; it
/// never converts lack of knowledge into elimination authority.
pub(super) fn derive_case_producer_fact(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    environment: &[CaseProducerFact],
    match_scrutinees: &mut BTreeMap<StaticOriginId, CaseProducerFact>,
) -> Result<CaseProducerFact, CraneliftBackendError> {
    let expr = plan.planned_occurrence_expr(origin)?;
    let child = |position| plan.semantic.child_origin(origin, position);
    let result = match expr {
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { .. } => {
            derive_case_producer_fact(plan, child(0)?, environment, match_scrutinees)?
                .forwarded(origin, CaseProducerFlowKind::Forward)
        }
        RuntimeExpr::Construct { args, .. } => {
            for position in 0..args.len() {
                derive_case_producer_fact(plan, child(position)?, environment, match_scrutinees)?;
            }
            CaseProducerFact::constructor(origin, plan.constructor_symbol_identity(origin)?)
        }
        RuntimeExpr::Let { .. } => {
            let value = derive_case_producer_fact(plan, child(0)?, environment, match_scrutinees)?;
            let mut nested = Vec::with_capacity(environment.len() + 1);
            nested.push(value);
            nested.extend_from_slice(environment);
            derive_case_producer_fact(plan, child(1)?, &nested, match_scrutinees)?
                .forwarded(origin, CaseProducerFlowKind::Forward)
        }
        RuntimeExpr::Var(index) => environment
            .get(*index as usize)
            .cloned()
            .unwrap_or_else(|| CaseProducerFact::open(origin))
            .forwarded(origin, CaseProducerFlowKind::Environment),
        RuntimeExpr::If { .. } => {
            derive_case_producer_fact(plan, child(0)?, environment, match_scrutinees)?;
            let then_fact =
                derive_case_producer_fact(plan, child(1)?, environment, match_scrutinees)?;
            let else_fact =
                derive_case_producer_fact(plan, child(2)?, environment, match_scrutinees)?;
            then_fact
                .join(&else_fact)
                .forwarded(origin, CaseProducerFlowKind::Alternative)
        }
        RuntimeExpr::Match { cases, .. } => {
            let scrutinee =
                derive_case_producer_fact(plan, child(0)?, environment, match_scrutinees)?;
            match_scrutinees.insert(origin, scrutinee.clone());
            let mut result = CaseProducerFact::empty();
            for (index, case) in cases.iter().enumerate() {
                let identity = plan.case_constructor_identity(origin, index)?;
                let reachable = match &scrutinee.producers {
                    CaseProducerSet::Open => true,
                    CaseProducerSet::Closed(constructors) => constructors.contains(&identity),
                };
                let mut nested = Vec::with_capacity(case.binders + environment.len());
                nested.extend((0..case.binders).map(|_| CaseProducerFact::open(origin)));
                nested.extend_from_slice(environment);
                let body =
                    derive_case_producer_fact(plan, child(1 + index)?, &nested, match_scrutinees)?;
                if reachable {
                    result = result.join(&body);
                }
            }
            result.forwarded(origin, CaseProducerFlowKind::Alternative)
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            let scrutinee =
                derive_case_producer_fact(plan, child(0)?, environment, match_scrutinees)?;
            let mut result = CaseProducerFact::empty();
            for (index, case) in cases.iter().enumerate() {
                let identity = plan.case_constructor_identity(origin, index)?;
                let reachable = match &scrutinee.producers {
                    CaseProducerSet::Open => true,
                    CaseProducerSet::Closed(constructors) => constructors.contains(&identity),
                };
                let binders = case
                    .argument_binders
                    .checked_add(case.recursive_positions.len())
                    .ok_or_else(|| {
                        planner_capacity_error("case producer binder count exhausted")
                    })?;
                let mut nested = Vec::with_capacity(binders + environment.len());
                nested.extend((0..binders).map(|_| CaseProducerFact::open(origin)));
                nested.extend_from_slice(environment);
                let body =
                    derive_case_producer_fact(plan, child(1 + index)?, &nested, match_scrutinees)?;
                if reachable {
                    result = result.join(&body);
                }
            }
            result.forwarded(origin, CaseProducerFlowKind::Alternative)
        }
        RuntimeExpr::Trap(_) => CaseProducerFact::empty(),
        RuntimeExpr::PrimitiveCall { args, .. } => {
            for position in 0..args.len() {
                derive_case_producer_fact(plan, child(position)?, environment, match_scrutinees)?;
            }
            CaseProducerFact::open(origin)
        }
        RuntimeExpr::Record { fields } => {
            for position in 0..fields.len() {
                derive_case_producer_fact(plan, child(position)?, environment, match_scrutinees)?;
            }
            CaseProducerFact::open(origin)
        }
        RuntimeExpr::Project { .. } => {
            derive_case_producer_fact(plan, child(0)?, environment, match_scrutinees)?;
            CaseProducerFact::open(origin)
        }
        RuntimeExpr::Closure {
            captures, params, ..
        } => {
            let mut body_environment = Vec::with_capacity(captures.len() + params.len());
            body_environment
                .extend((0..captures.len() + params.len()).map(|_| CaseProducerFact::open(origin)));
            derive_case_producer_fact(plan, child(0)?, &body_environment, match_scrutinees)?;
            CaseProducerFact::open(origin)
        }
        RuntimeExpr::LexicalClosure {
            captures, params, ..
        } => {
            let mut capture_facts = Vec::with_capacity(captures.len());
            for position in 0..captures.len() {
                capture_facts.push(derive_case_producer_fact(
                    plan,
                    child(1 + position)?,
                    environment,
                    match_scrutinees,
                )?);
            }
            let mut body_environment = Vec::with_capacity(captures.len() + params.len());
            body_environment.extend((0..params.len()).map(|_| CaseProducerFact::open(origin)));
            body_environment.extend(capture_facts);
            derive_case_producer_fact(plan, child(0)?, &body_environment, match_scrutinees)?;
            CaseProducerFact::open(origin)
        }
        RuntimeExpr::Call { args, .. } => {
            derive_case_producer_fact(plan, child(0)?, environment, match_scrutinees)?;
            for position in 0..args.len() {
                derive_case_producer_fact(
                    plan,
                    child(1 + position)?,
                    environment,
                    match_scrutinees,
                )?;
            }
            CaseProducerFact::open(origin)
        }
        RuntimeExpr::Effect {
            capability, args, ..
        } => {
            let child_count = args.len() + usize::from(capability.is_some());
            for position in 0..child_count {
                derive_case_producer_fact(plan, child(position)?, environment, match_scrutinees)?;
            }
            CaseProducerFact::open(origin)
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. } => CaseProducerFact::open(origin),
    };
    Ok(result)
}

pub(super) fn build_case_emission_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<PlannedCaseEmission>, CraneliftBackendError> {
    let mut match_scrutinees = BTreeMap::new();
    let root = plan
        .root_occurrence
        .ok_or_else(|| planner_error("case-emission analysis has no root occurrence"))?;
    derive_case_producer_fact(plan, root, &[], &mut match_scrutinees)?;
    for origin in plan.declaration_occurrences.values().copied() {
        derive_case_producer_fact(plan, origin, &[], &mut match_scrutinees)?;
    }
    let mut records = Vec::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Match { cases, .. } = occurrence.expr else {
            continue;
        };
        let match_origin = occurrence.static_origin;
        let owner = plan
            .semantic
            .function_owner(match_origin)?
            .ok_or_else(|| planner_error("case-emission match has no function owner"))?;
        let scrutinee_origin = plan.semantic.child_origin(match_origin, 0)?;
        let authority = match_scrutinees
            .get(&match_origin)
            .ok_or_else(|| planner_error("source Match has no producer-flow observation"))?
            .authority();
        for (ordinal, _) in cases.iter().enumerate() {
            let constructor = plan.case_constructor_identity(match_origin, ordinal)?;
            let status = match &authority.producers {
                CaseProducerSet::Open => CaseEmissionStatus::Reachable,
                CaseProducerSet::Closed(constructors) if constructors.contains(&constructor) => {
                    CaseEmissionStatus::Reachable
                }
                CaseProducerSet::Closed(_) => CaseEmissionStatus::Eliminated,
            };
            records.push(PlannedCaseEmission {
                match_origin,
                scrutinee_origin,
                owner,
                ordinal: u32::try_from(ordinal)
                    .map_err(|_| planner_capacity_error("case-emission ordinal exhausted"))?,
                body_origin: plan.semantic.child_origin(match_origin, 1 + ordinal)?,
                constructor,
                authority: authority.clone(),
                status,
            });
        }
    }
    records.sort_by_key(|record| (record.owner, record.match_origin, record.ordinal));
    Ok(records)
}

pub(super) fn validate_case_emission_plan(
    plan: &StaticTransitionPlan<'_>,
    records: &[PlannedCaseEmission],
) -> Result<(), CraneliftBackendError> {
    if records != build_case_emission_plan(plan)? {
        return Err(planner_error(
            "dormant case-emission facts are not the exact closed producer derivation",
        ));
    }
    Ok(())
}

pub(super) fn validate_substrate_preallocation_closure(
    plan: &StaticTransitionPlan<'_>,
    case_emissions: &[PlannedCaseEmission],
    occurrence_authorities: &[PlannedOccurrenceAuthority],
) -> Result<(), CraneliftBackendError> {
    let by_origin = occurrence_authorities
        .iter()
        .map(|record| (record.origin, record))
        .collect::<BTreeMap<_, _>>();
    if by_origin.len() != occurrence_authorities.len()
        || by_origin.len() != plan.source_occurrences.iter().flatten().count()
    {
        return Err(planner_error(
            "dormant substrate occurrence population is not bijective",
        ));
    }
    for record in case_emissions {
        let match_authority = by_origin
            .get(&record.match_origin)
            .ok_or_else(|| planner_error("case-emission match has no occurrence authority"))?;
        if match_authority.owner != record.owner {
            return Err(planner_error(
                "case-emission owner disagrees with occurrence authority",
            ));
        }
        if !by_origin.contains_key(&record.scrutinee_origin)
            || !by_origin.contains_key(&record.body_origin)
        {
            return Err(planner_error(
                "case-emission edge is outside the occurrence authority population",
            ));
        }
        match (&record.authority.producers, record.status) {
            (CaseProducerSet::Open, CaseEmissionStatus::Eliminated) => {
                return Err(planner_error(
                    "pre-allocation closure eliminated a case from open producer authority",
                ));
            }
            (CaseProducerSet::Closed(constructors), CaseEmissionStatus::Reachable)
                if !constructors.contains(&record.constructor) =>
            {
                return Err(planner_error(
                    "pre-allocation closure admits an unreachable case",
                ));
            }
            (CaseProducerSet::Closed(constructors), CaseEmissionStatus::Eliminated)
                if constructors.contains(&record.constructor) =>
            {
                return Err(planner_error(
                    "pre-allocation closure eliminates a reachable case",
                ));
            }
            (CaseProducerSet::Open, CaseEmissionStatus::Reachable)
            | (CaseProducerSet::Closed(_), CaseEmissionStatus::Reachable)
            | (CaseProducerSet::Closed(_), CaseEmissionStatus::Eliminated) => {}
        }
    }
    Ok(())
}

/// **`D4b`** — one required position's admission verdict, as the take-loop's own
/// two clauses see it. Test-only.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum D4bVerdict {
    /// `Closed([S])` — one exact source. The only admitting verdict.
    Closed,
    /// `Open` — refused by the take-loop's first clause.
    Open,
    /// `Closed([S, T, ..])` — refused by the take-loop's second clause.
    Ambiguous(usize),
}

#[cfg(test)]
thread_local! {
    pub(super) static D4B_ADMISSION: std::cell::RefCell<Vec<(Vec<D4bVerdict>, bool)>> =
        const { std::cell::RefCell::new(Vec::new()) };
    pub(super) static D4B_ADMISSION_ARMED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(super) fn d4b_arm_admission(armed: bool) {
    D4B_ADMISSION_ARMED.with(|cell| cell.set(armed));
    if armed {
        D4B_ADMISSION.with(|ledger| ledger.borrow_mut().clear());
    }
}

/// Every candidate edge seen while armed, as `(required verdict vector, admitted)`.
#[cfg(test)]
pub(super) fn d4b_take_admission() -> Vec<(Vec<D4bVerdict>, bool)> {
    D4B_ADMISSION.with(|ledger| std::mem::take(&mut *ledger.borrow_mut()))
}

/// **`cfg(test)`-only corruption of the planned static-worker MEMBER
/// population** — the two compile-valid ways a real member goes missing.
///
/// ⭐ Two settings and not one, because a member can be wrong by **not being
/// there** and by **being there as something else**, and neither mutation
/// produces the other's plan. `Reclassify` is the one a positive assertion is
/// most likely to agree with by accident: the unit is still present, still
/// declared, still has a function, and only its definition arm says it is not
/// this worker's body.
///
/// ⛔ Applied to the descriptor population **after** it is derived and
/// **before** the plan validates, so what is measured is a planner refusal on a
/// real plan state -- not a checker fed a hand-built argument.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum StaticWorkerMemberMutation {
    Exact,
    /// Drop the emittable unit that defines the planned member's body.
    OmitMember,
    /// Keep the unit, but define it as a scheduling entry instead.
    ReclassifyMember,
    /// Keep the unit and its arm, but attribute it to a different closure.
    RedirectDefiningOccurrence,
    /// Keep the member exactly where it is, and change the contract it declares.
    MisdeclareMemberContract,
}

#[cfg(test)]
thread_local! {
    static STATIC_WORKER_MEMBER_MUTATION: std::cell::Cell<StaticWorkerMemberMutation> =
        const { std::cell::Cell::new(StaticWorkerMemberMutation::Exact) };
}

/// Run `body` with the planned static-worker member population corrupted.
///
/// ⛔ No restore-on-unwind guard: a panic inside the scope is the fixture
/// failing, the cell is thread-local, and Rust runs each test on its own
/// thread, so a panicking row cannot leak the setting into a sibling.
#[cfg(test)]
pub(super) fn with_static_worker_member_mutation<R>(
    mutation: StaticWorkerMemberMutation,
    body: impl FnOnce() -> R,
) -> R {
    STATIC_WORKER_MEMBER_MUTATION.with(|cell| cell.set(mutation));
    let result = body();
    STATIC_WORKER_MEMBER_MUTATION.with(|cell| cell.set(StaticWorkerMemberMutation::Exact));
    result
}

/// Corrupt the emittable unit that defines the FIRST planned specialization's
/// worker body, leaving every other unit -- including every `ClosureBody` that
/// no specialization names -- exactly as the planner derived it.
///
/// ⭐ Scoped to the exact member on purpose. A mutation that swept every
/// `ClosureBody` would red under this law and under nothing else, and would
/// therefore be unable to show that ordinary closures are untouched by it.
#[cfg(test)]
pub(super) fn apply_static_worker_member_mutation(plan: &mut StaticTransitionPlan<'_>) {
    let mutation = STATIC_WORKER_MEMBER_MUTATION.with(std::cell::Cell::get);
    if mutation == StaticWorkerMemberMutation::Exact {
        return;
    }
    let Some(specialization) = plan.continuation_specializations.first() else {
        return;
    };
    let closure_origin = specialization.key.worker.closure_origin;
    let defines_member = |definition: &AbiUnitDefinition| {
        matches!(
            definition,
            AbiUnitDefinition::ClosureBody { defining_origin, .. }
                if *defining_origin == closure_origin
        )
    };
    match mutation {
        StaticWorkerMemberMutation::Exact => {}
        StaticWorkerMemberMutation::OmitMember => {
            plan.abi
                .descriptors
                .retain(|descriptor| !defines_member(&descriptor.definition));
        }
        StaticWorkerMemberMutation::ReclassifyMember => {
            for descriptor in &mut plan.abi.descriptors {
                if defines_member(&descriptor.definition) {
                    descriptor.definition = AbiUnitDefinition::SchedulingEntry {
                        ingress: AbiSchedulingIngress::Empty,
                    };
                }
            }
        }
        StaticWorkerMemberMutation::RedirectDefiningOccurrence => {
            for descriptor in &mut plan.abi.descriptors {
                if defines_member(&descriptor.definition) {
                    if let AbiUnitDefinition::ClosureBody {
                        defining_origin, ..
                    } = &mut descriptor.definition
                    {
                        *defining_origin = StaticOriginId(defining_origin.0.wrapping_add(1));
                    }
                }
            }
        }
        StaticWorkerMemberMutation::MisdeclareMemberContract => {
            for descriptor in &mut plan.abi.descriptors {
                if defines_member(&descriptor.definition) {
                    descriptor.header.parameters = descriptor.header.parameters.wrapping_add(1);
                }
            }
        }
    }
}

/// Whether an emittable unit names this closure occurrence as its defining
/// occurrence -- the planner's own answer to *"is this a real static-worker
/// member?"*.
///
/// ⛔ Keyed on the DEFINING OCCURRENCE, not on the worker's body origin.
/// Measured: those are different coordinates. A specialization's
/// `worker.body_origin` is the closure's child-0 occurrence, while an
/// `AbiDescriptor::origin` is the unit's seed -- the target of the closure's
/// `StaticBody` edge. They coincide only when a body's occurrence is also its
/// scheduling entry, and the `px8j` fixture is a live counter-example (closure
/// `29` names body `18`; its unit's origin is `28`). Joining on them produced a
/// false planning refusal on a green fixture.
fn closure_defines_a_planned_member(
    plan: &StaticTransitionPlan<'_>,
    closure_origin: StaticOriginId,
) -> bool {
    plan.abi.descriptors.iter().any(|descriptor| {
        matches!(
            descriptor.definition,
            AbiUnitDefinition::ClosureBody { defining_origin, .. }
                if defining_origin == closure_origin
        )
    })
}

/// **`RT-DECL-CLOSURE-PORT` `D7` checkpoint `1c` — THE MATRIX-OMISSION LAW, at
/// the planning boundary.**
///
/// ⭐⭐ **A real planned static-worker member that is omitted or misclassified
/// must red HERE.** The frame states the law directly: such a member *"must FAIL
/// IN PLANNING, and may NOT fall through to the late generic `Closure`
/// refusal."* Checkpoint 1's
/// `validate_retained_callable_capture_contract` is a **pre-emission** gate --
/// correct, and later than the law. It also treats `worker_templates`
/// membership as necessary and insufficient, which is exactly right and exactly
/// why it cannot be this check: **every** emittable body is in that map, so
/// membership does not distinguish an exact member from an ordinary closure.
///
/// ⭐ **The discriminator is the specialization population, and it already
/// exists.** Measured on the linked row: four `ClosureBody` units, and exactly
/// one of them named by a `ContinuationSpecialization`'s worker. That one is the
/// exact planner-proved member; the other three are ordinary closures which the
/// frame says must *still* reach the generic arm. So the law's subject is
/// precisely "named by a specialization", and its obligation is that the unit
/// population contain that member, defined by that closure, under that contract.
///
/// ⛔ **Nothing is minted and nothing is widened.** Both sides are populations
/// the planner already closed: `continuation_specializations`, whose worker
/// provenance names `(closure_origin, body_origin, declared_arity, captures)`,
/// and `abi.descriptors`, whose `ClosureBody` arm already records its
/// `defining_origin`. This is a cross-population closure over two existing
/// derivations -- the same shape as the substrate-preallocation closure beside
/// it -- not a new identity, lane, disposition or ABI field.
///
/// ⛔ **The join is on the DEFINING CLOSURE OCCURRENCE, not on the worker's body
/// origin, and that correction was measured rather than reasoned.** A
/// specialization's `worker.body_origin` is the closure's child-`0` occurrence;
/// an `AbiDescriptor::origin` is the unit's seed, the target of the closure's
/// `StaticBody` edge. They coincide only when a body's occurrence is also its
/// scheduling entry. Joining on them refused the green `px8j` fixture, whose
/// closure `29` names body `18` while its unit's origin is `28`.
///
/// The two ways the member can be wrong, named separately:
///
/// | failure | what it means |
/// |---|---|
/// | no emittable unit defines this worker's closure | the member is **omitted or misclassified**: it is gone, it is not a closure body, or it belongs to another closure -- one fact to a join on the defining occurrence |
/// | the unit's declared parameters/captures disagree with the worker | the member is present, correctly classified and correctly attributed, under a **different contract** than the specialization was interned against |
///
/// ⛔⛔ **This is the CONVERSE direction of the frame's law, and it does not
/// close the framed gap.** It says every interned specialization has its member.
/// The framed witness is the other direction -- a real member with *no*
/// specialization at all -- and its omission site is the declined candidate in
/// `build_continuation_specialization_plan`, which carries the hard stop. Read
/// that comment before treating this as the whole of `1c`.
pub(super) fn validate_static_worker_member_population(
    plan: &StaticTransitionPlan<'_>,
) -> Result<(), CraneliftBackendError> {
    for specialization in &plan.continuation_specializations {
        let worker = &specialization.key.worker;
        // Uniqueness is checked HERE rather than over the whole descriptor
        // population, because the law's subject is this member. A global
        // duplicate sweep would be a different, broader claim about the ABI
        // plane, and asserting it from this function would make a failure
        // report the wrong cause.
        let mut members = plan.abi.descriptors.iter().filter(|descriptor| {
            matches!(
                descriptor.definition,
                AbiUnitDefinition::ClosureBody { defining_origin, .. }
                    if defining_origin == worker.closure_origin
            )
        });
        let Some(member) = members.next() else {
            return Err(planner_error(
                "a planned continuation specialization names a static-worker closure that no \
                 emittable unit defines, so the exact member is omitted from the planned \
                 population and that closure could only reach the generic closure arm",
            ));
        };
        if members.next().is_some() {
            return Err(planner_error(
                "two emittable units define one static-worker closure occurrence, so the planned \
                 specialization could not name either member unambiguously",
            ));
        }
        if member.header.parameters != worker.declared_arity {
            return Err(planner_error(
                "a planned static-worker member declares a different parameter count than the \
                 specialization interned against it",
            ));
        }
        if member.header.captures as usize != worker.captures.len() {
            return Err(planner_error(
                "a planned static-worker member declares a different capture count than the \
                 specialization interned against it",
            ));
        }
    }
    Ok(())
}

impl StaticTransitionPlan<'_> {
    /// Resolve one process-root parameter by its closed semantic role.
    ///
    /// The caller cannot restate ordinals: only the scheduling entry whose
    /// validated definition carries `ProcessPair` can answer, and the slot
    /// offset comes from B2R's sole offset walk.
    pub(in crate::cranelift_backend) fn process_parameter_slot(
        &self,
        role: AbiProcessParameter,
    ) -> Result<Option<(AbiSlot, u32)>, CraneliftBackendError> {
        let mut answer = None;
        for unit in self.emittable_units()? {
            if unit.definition()
                != (AbiUnitDefinition::SchedulingEntry {
                    ingress: AbiSchedulingIngress::ProcessPair,
                })
            {
                continue;
            }
            let (offsets, _) = unit.slot_offsets()?;
            let found = unit
                .slots()
                .iter()
                .copied()
                .zip(offsets)
                .find(|(slot, _)| {
                    slot.kind == AbiSlotKind::Parameter && slot.ordinal == role.ordinal()
                })
                .ok_or_else(|| planner_error("process ingress role has no declared root slot"))?;
            if answer.replace(found).is_some() {
                return Err(planner_error(
                    "more than one scheduling entry declares process ingress",
                ));
            }
        }
        Ok(answer)
    }
}

impl<'src> StaticTransitionPlan<'src> {
    /// Test-only projection of the existing closed constructor-result analysis.
    ///
    /// The boolean is whether the producer set is closed. The vector preserves
    /// every source constructor occurrence admitted by that analysis and the
    /// source kind at `position`. No crossing or lowered-value observation
    /// participates.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn planned_result_field_kinds_for_test(
        &self,
        origin: StaticOriginId,
        position: usize,
    ) -> Result<(bool, Vec<(StaticOriginId, PlannedResultFieldKindForTest)>), CraneliftBackendError>
    {
        let mut match_scrutinees = BTreeMap::new();
        let fact = derive_case_producer_fact(self, origin, &[], &mut match_scrutinees)?;
        let closed = matches!(fact.producers, CaseProducerSet::Closed(_));
        let mut fields = Vec::new();
        for (_, origins) in fact.producer_origins {
            for producer in origins {
                let RuntimeExpr::Construct { args, .. } = self.source_occurrence(producer)? else {
                    return Err(planner_error(
                        "planned constructor-result origin is not a source Construct",
                    ));
                };
                let kind = match args.get(position) {
                    Some(RuntimeExpr::Closure { .. }) => PlannedResultFieldKindForTest::Closure,
                    Some(RuntimeExpr::LexicalClosure { .. }) => {
                        PlannedResultFieldKindForTest::LexicalClosure
                    }
                    Some(_) => PlannedResultFieldKindForTest::Other,
                    None => PlannedResultFieldKindForTest::Absent,
                };
                fields.push((producer, kind));
            }
        }
        Ok((closed, fields))
    }

    /// The planner's exact emission verdict for one ordinary `Match` case.
    ///
    /// `Reachable` when the scrutinee's producer set is `Open`, or is
    /// `Closed(S)` and contains this case's canonical constructor identity;
    /// `Eliminated` when it is `Closed(S)` and does not. The derivation lives in
    /// `build_case_emission_plan` and is validated against a re-derivation, so
    /// this is a projection of an already-closed fact.
    ///
    /// Lowering asks an occurrence-and-ordinal keyed QUESTION and receives a
    /// verdict it cannot mint: `case_emissions`, `semantic` and the producer-set
    /// derivation all stay private, so an emitter can obtain this answer and
    /// cannot derive a different one. It returns no source term, so it cannot
    /// widen the single `-> Result<&'src RuntimeExpr` route.
    ///
    /// `None` means the origin is not a planned ordinary `Match` occurrence --
    /// a `ComputationalMatch` has no record here. That is a refusal to answer,
    /// never a default to `Reachable`.
    pub(in crate::cranelift_backend) fn case_emission_status(
        &self,
        match_origin: StaticOriginId,
        ordinal: usize,
    ) -> Result<Option<CaseEmissionStatus>, CraneliftBackendError> {
        let ordinal =
            u32::try_from(ordinal).map_err(|_| planner_error("case-emission ordinal exhausted"))?;
        Ok(self
            .case_emissions
            .iter()
            .find(|record| record.match_origin == match_origin && record.ordinal == ordinal)
            .map(|record| record.status))
    }

    /// Result-position source occurrences below one root in one function owner.
    ///
    /// This is the sole exhaustive inventory for source result flow. Lowering
    /// uses it to recognize terminal process results, while phase planning uses
    /// the same population when an enclosing computational eliminator changes
    /// the representation forwarded by a producer-local join.
    pub(in crate::cranelift_backend) fn source_result_origins_in_owner_subtree(
        &self,
        root: StaticOriginId,
    ) -> Result<BTreeSet<StaticOriginId>, CraneliftBackendError> {
        let owner = self
            .semantic
            .function_owner(root)?
            .ok_or_else(|| planner_error("source result root has no function owner"))?;
        let mut pending = vec![root];
        let mut results = BTreeSet::new();
        while let Some(origin) = pending.pop() {
            if !results.insert(origin) {
                continue;
            }
            if self.semantic.function_owner(origin)? != Some(owner) {
                results.remove(&origin);
                continue;
            }
            let occurrence = self
                .source_occurrences
                .get(origin.0 as usize)
                .and_then(Option::as_ref)
                .ok_or_else(|| {
                    planner_error("source result traversal names no planned occurrence")
                })?;
            if occurrence.static_origin != origin {
                return Err(planner_error(
                    "source result occurrence disagrees with its positional origin",
                ));
            }
            let expr = occurrence.expr;
            let child = |position| self.semantic.child_origin(origin, position);
            match expr {
                RuntimeExpr::CheckedJoinSite { .. }
                | RuntimeExpr::CheckedSubcontinuationFrame { .. }
                | RuntimeExpr::CheckedRecursiveInvocation { .. }
                | RuntimeExpr::CheckedComputationalIHSlots { .. }
                | RuntimeExpr::CheckedComputationalIHInvocation { .. } => {
                    pending.push(child(0)?);
                }
                RuntimeExpr::Let { .. } => pending.push(child(1)?),
                RuntimeExpr::If { .. } => {
                    pending.push(child(1)?);
                    pending.push(child(2)?);
                }
                RuntimeExpr::Match { cases, .. } => {
                    for index in 0..cases.len() {
                        pending.push(child(1 + index)?);
                    }
                }
                RuntimeExpr::ComputationalMatch { cases, .. } => {
                    for index in 0..cases.len() {
                        pending.push(child(1 + index)?);
                    }
                }
                RuntimeExpr::Value(_)
                | RuntimeExpr::Var(_)
                | RuntimeExpr::PrimitiveCall { .. }
                | RuntimeExpr::Construct { .. }
                | RuntimeExpr::Record { .. }
                | RuntimeExpr::Project { .. }
                | RuntimeExpr::Closure { .. }
                | RuntimeExpr::LexicalClosure { .. }
                | RuntimeExpr::DeclarationRef { .. }
                | RuntimeExpr::ImportedDeclarationRef { .. }
                | RuntimeExpr::Call { .. }
                | RuntimeExpr::Effect { .. }
                | RuntimeExpr::Trap(_) => {}
            }
        }
        Ok(results)
    }

    /// The case-body roots of a source `Match` occurrence.
    ///
    /// Both ordinary and computational matches have one scrutinee followed by
    /// their case bodies in the semantic plane. Static selection consumes this
    /// validated positional population directly; lowering does not supply or
    /// recount the source cases.
    pub(in crate::cranelift_backend) fn source_match_case_body_origins(
        &self,
        origin: StaticOriginId,
    ) -> Result<Vec<StaticOriginId>, CraneliftBackendError> {
        let occurrence = self
            .source_occurrences
            .get(origin.0 as usize)
            .and_then(Option::as_ref)
            .ok_or_else(|| planner_error("source match names no planned occurrence"))?;
        if occurrence.static_origin != origin
            || !matches!(
                occurrence.expr,
                RuntimeExpr::Match { .. } | RuntimeExpr::ComputationalMatch { .. }
            )
        {
            return Err(planner_error(
                "source match population was requested for a different occurrence kind",
            ));
        }
        let children = self.semantic.child_origins(origin)?;
        let Some((_scrutinee, case_bodies)) = children.split_first() else {
            return Err(planner_error(
                "source match occurrence has no validated scrutinee child",
            ));
        };
        Ok(case_bodies.to_vec())
    }

    /// The artifact-static constructor identity of one case of the `Match` /
    /// `ComputationalMatch` occurrence at `origin` (`D1`).
    ///
    /// ⭐ **This is the capability export, not the plane.** `SemanticPlane` and
    /// its `names` arena stay `pub(super)`; what crosses into
    /// `crate::cranelift_backend` is an occurrence-keyed *question* and an
    /// unmintable answer. That is `RT-FNSPLIT-B2E`'s surviving `R3` shape —
    /// *"expose the capability, not the plane internals"* — and it is why `D1`
    /// is not discharged by widening a field.
    ///
    /// ⭐ The returned identity **is** occurrence-independent: equal spellings
    /// intern to one canonical span, so a producer's identity for `Cons` and an
    /// eliminator's identity for `Cons` are the same value even at different
    /// occurrences. That is `D2`'s shared-authority property.
    ///
    /// ⚠ **Artifact-local.** The identity is stable within one artifact's plane
    /// and carries no cross-artifact meaning. ⛔ Do not persist or compare it
    /// across artifacts.
    pub(in crate::cranelift_backend) fn case_constructor_identity(
        &self,
        origin: StaticOriginId,
        case_index: usize,
    ) -> Result<ConstructorIdentity, CraneliftBackendError> {
        self.semantic.case_constructor_identity(origin, case_index)
    }

    /// The artifact-static constructor identity of a `Construct` occurrence —
    /// the producer side of [`Self::case_constructor_identity`] (`D2`).
    pub(in crate::cranelift_backend) fn constructor_symbol_identity(
        &self,
        origin: StaticOriginId,
    ) -> Result<ConstructorIdentity, CraneliftBackendError> {
        self.semantic.constructor_symbol_identity(origin)
    }

    /// The existing semantic-plane identity for one compiler-synthesized
    /// constructor role.
    ///
    /// The key is a closed sum.  In particular, dynamic IOError alternatives
    /// can only be named with opaque tokens returned by
    /// [`Self::synthesized_io_error_roles`].
    pub(in crate::cranelift_backend) fn synthesized_constructor_identity(
        &self,
        role: SynthesizedConstructorRole,
    ) -> Result<ConstructorIdentity, CraneliftBackendError> {
        self.semantic.synthesized_constructor_identity(role)
    }

    pub(in crate::cranelift_backend) fn synthesized_io_error_roles(
        &self,
    ) -> &[SynthesizedIoErrorRole] {
        self.semantic.synthesized_io_error_roles()
    }

    /// The artifact-static field identity a `Project` occurrence selects (`D1`).
    pub(in crate::cranelift_backend) fn project_field_identity(
        &self,
        origin: StaticOriginId,
    ) -> Result<FieldIdentity, CraneliftBackendError> {
        self.semantic.project_field_identity(origin)
    }

    /// The artifact-static field identity of one field of a `Record` occurrence
    /// — the producer side of [`Self::project_field_identity`] (`D2`).
    pub(in crate::cranelift_backend) fn record_field_identity(
        &self,
        origin: StaticOriginId,
        position: usize,
    ) -> Result<FieldIdentity, CraneliftBackendError> {
        self.semantic.record_field_identity(origin, position)
    }

    /// The pairing table's own fail-closed laws.
    ///
    /// Every operand here is **independent of the table**: the scheduling
    /// entry population, the source-occurrence table, and the two projections.
    /// A law that re-derived the pair from the same visit that issued it would
    /// be comparing one value with itself and could not fail.
    ///
    /// **No completion edge is walked.** Reconstructing the answer from graph
    /// shape is the inference the table replaced; a validator that did it would
    /// re-admit it under the name of a check, and would agree with a wrong table
    /// exactly when the wrong table was wrong for the reason the walk shares.
    fn validate_planned_entry_bodies(&self) -> Result<(), CraneliftBackendError> {
        // The exact key population: every scheduling entry, UNION every
        // `StaticBody` target. Derived here from the graph the relation is
        // validated against, so a row can be checked for membership without the
        // relation being consulted about its own domain.
        //
        // A declaration-owned scheduling entry stays a KEY even though `D2a`
        // excludes it from the emitted-function population: it was registered at
        // seat 1 and its declaration projection is checked against that row.
        // Dropping it here would make the relation disagree with its own writer.
        let mut population = self.entries.iter().copied().collect::<BTreeSet<_>>();
        let mut static_body_targets = 0usize;
        for edge in &self.edges {
            if edge.kind == EdgeKind::StaticBody {
                if !population.insert(edge.to) {
                    return Err(planner_error(
                        "static body target is also a scheduling entry",
                    ));
                }
                static_body_targets += 1;
            }
        }
        // Exact-total in both directions, over the union.
        if self.planned_entry_bodies.len() != population.len()
            || population.len() != self.entries.len() + static_body_targets
        {
            return Err(planner_error(
                "planned entry bodies are not exact for the planned entry population",
            ));
        }
        let mut keys = BTreeSet::new();
        let mut bodies = BTreeSet::new();
        for pair in &self.planned_entry_bodies {
            if !population.contains(&pair.entry) {
                return Err(planner_error(
                    "planned entry body names a key outside the planned entry population",
                ));
            }
            if !keys.insert(pair.entry) {
                return Err(planner_error(
                    "planned entry has more than one issued body occurrence",
                ));
            }
            // `source_occurrences` is dense-by-ordinal and its slots are
            // `Option`: an in-range `None` is a CONTROL NODE with no source
            // term. `.get(..).is_none()` only rejects an out-of-RANGE ordinal,
            // so `Some(None)` -- a real planned node that carries no source --
            // passed. That is the exact substitution this law exists to refuse,
            // and the owner check does not close it because descriptors exist
            // for control nodes too and a same-owner control node satisfies it.
            if self
                .source_occurrences
                .get(pair.body_occurrence.0 as usize)
                .and_then(Option::as_ref)
                .is_none()
            {
                return Err(planner_error(
                    "scheduling entry body occurrence is not a planned source occurrence",
                ));
            }
            if !bodies.insert(pair.body_occurrence) {
                return Err(planner_error(
                    "two planned entries claim one body occurrence",
                ));
            }
        }
        // Every member of the union has a pair. With the length check above,
        // this is set equality rather than containment in one direction.
        for entry in &population {
            if !keys.contains(entry) {
                return Err(planner_error("planned entry has no issued body occurrence"));
            }
        }
        // The two surviving views are projections of this table, so they are
        // checked AGAINST it rather than trusted beside it. A disagreement means
        // something wrote a pairing that did not come through the registration
        // helper.
        if let Some(root_entry) = self.root_entry {
            if self.planned_entry_body(root_entry) != self.root_occurrence {
                return Err(planner_error(
                    "root occurrence projection disagrees with the issued pairing",
                ));
            }
        }
        // Keyed equality per SYMBOL, not set membership. Membership asks only
        // whether each recorded occurrence is issued to SOMEONE, which two
        // declarations swapping body occurrences preserves exactly -- the set is
        // unchanged and every element is still issued. Only comparing each
        // symbol's own entry against its own recorded occurrence can see a swap,
        // which is what "equality-checked projection" has to mean.
        if self.declaration_entries.len() != self.declaration_occurrences.len() {
            return Err(planner_error(
                "declaration entry and occurrence projections have different populations",
            ));
        }
        for (symbol, entry) in &self.declaration_entries {
            let recorded = self.declaration_occurrences.get(symbol).ok_or_else(|| {
                planner_error("declaration entry projection names an unrecorded symbol")
            })?;
            if self.planned_entry_body(*entry) != Some(*recorded) {
                return Err(planner_error(
                    "declaration occurrence projection disagrees with the issued pairing",
                ));
            }
        }
        Ok(())
    }

    /// Register one scheduling entry together with the body occurrence its
    /// planning visit returned.
    ///
    /// **The only writer of `entries` and of `planned_entry_bodies`**, and
    /// it writes both or neither. That is what makes the pairing exact-total by
    /// construction: there is no ordering of calls that registers an entry
    /// without its pair, so "missing pair" is a shape the constructor cannot
    /// produce rather than a state a checker has to catch after the fact.
    ///
    /// Deliberately **generic in the planned form**. It takes a
    /// [`PlannedExpr`] and stores the two fields it was handed; it does not ask
    /// what shape produced them and must never learn. A registration helper that
    /// branches on the form is the special case the ruling exists to avoid —
    /// and it would be wrong as well as forbidden, because the caller has
    /// already resolved the two axes correctly for **every** form, coincident or
    /// not.

    /// Record one issued `entry -> body_occurrence` row.
    ///
    /// **The single writer of the relation**, shared by both issuance seats so
    /// the two cannot drift into two ledgers with two shapes. It takes a
    /// [`PlannedExpr`] and stores the two fields it was handed; it does not ask
    /// what produced them and must never learn.

    /// The body occurrence issued for one scheduling entry.
    ///
    /// Reads the pairing authority. `None` means this node is not a
    /// scheduling entry at all — it is never a licence to substitute the entry's
    /// own origin, which is precisely the alias this table replaced.
    pub(super) fn planned_entry_body(&self, entry: StaticNodeId) -> Option<StaticOriginId> {
        self.planned_entry_bodies
            .iter()
            .find(|pair| pair.entry == entry)
            .map(|pair| pair.body_occurrence)
    }

    /// The **occurrence** origin of a transparent declaration, by symbol.
    ///
    /// `None` is a real answer, not a failure: a declaration that is not
    /// transparent has no planned body, and the lowering rejects it on its own
    /// terms. The caller must not substitute an origin of its own when this is
    /// `None`.
    pub(in crate::cranelift_backend) fn declaration_occurrence_origin(
        &self,
        symbol: &str,
    ) -> Option<StaticOriginId> {
        self.declaration_occurrences.get(symbol).copied()
    }

    /// **`RT-DECL-CLOSURE-PORT` `D4`** — which class of unit this
    /// `DeclarationRef` occurrence's call edge targets.
    ///
    /// ⛔ `None` is a real answer and must not be defaulted: a reference with no
    /// recorded class had no `DeclarationCall` edge planned for it, so there is
    /// no call to emit and no class to guess. Substituting
    /// [`DeclarationCallTargetClass::SchedulingEntry`] here would restore
    /// exactly the empty-input call this record exists to prevent.
    pub(in crate::cranelift_backend) fn declaration_call_target_class(
        &self,
        reference: StaticOriginId,
    ) -> Option<DeclarationCallTargetClass> {
        self.declaration_call_targets.get(&reference).copied()
    }

    pub(in crate::cranelift_backend) fn carrier_identity_catalog(
        &self,
    ) -> Result<Vec<(String, u64)>, CraneliftBackendError> {
        self.semantic.carrier_identity_catalog()
    }

    /// **`RT-FNSPLIT-B2F` `D4` — the cross-owner call edges, DERIVED.**
    ///
    /// ⛔ **Nothing here decides what a call edge is.** The classification is
    /// `B2O`'s and it is enforced as `return Err` arms inside
    /// `SemanticPlane::validate_function_units`: a `StaticBody` edge crosses to
    /// a **distinct** unit and lands on that unit's **seed**; every other edge
    /// either stays inside one unit or exits to a shared exit; anything else is
    /// refused during planning. ⇒ ⭐ **A plan that reaches this method cannot
    /// carry a violating edge**, so this is a projection of facts already
    /// validated, not a second classification.
    ///
    /// ⚠ **Which is exactly why `B2F` must not re-assert those four laws.** A
    /// control here asserting "a `StaticBody` edge crosses owners" is green on
    /// every input that can reach emission and tests nothing. ⭐ What `B2F` owes
    /// instead is **one-for-one consumption** — that emission is driven by this
    /// view and does not build a second table beside it — which is a property
    /// the inert node could not check about itself.
    ///
    /// ⛔ **Fails closed on a missing descriptor** rather than skipping the
    /// edge: a dropped call edge is a unit that is never called, which is
    /// silent at emission and wrong at run time.
    /// ⛔ **The owner classification is NOT named here**, and that is enforced:
    /// `the_owner_classification_has_a_closed_production_naming_inventory` reds
    /// if this file starts spelling `SemanticOwner`. ⇒ The `StaticBody` walk
    /// lives in `semantic_ir.rs`, beside the validation that makes it sound, and
    /// this method only wraps the resulting id pairs in the emitter's view type.
    ///
    /// ⭐ That pin caught a real defect in this deliverable's first draft, which
    /// destructured `SemanticOwner::Function(..)` right here — a third file
    /// naming the classification is how a second, divergent classification
    /// authority starts.
    /// **`RT-CONTSPEC-ACTIVATE` `D1b` — the source-body binding, beside
    /// `emittable_call_edges` and never widening its filter.**
    ///
    /// An exact-set join over facts that already exist. Authoritative
    /// caller/callee classification stays **solely** with
    /// `static_body_call_edges`; the raw `EdgeKind::StaticBody` walk supplies
    /// **endpoints only**, and every derivation is forward:
    ///
    /// ```text
    /// closure_occurrence     = descriptors[edge.from].origin
    /// source_body_occurrence = child_origin(closure_occurrence, 0)
    /// scheduling_entry       = descriptors[edge.to].origin
    /// ```
    ///
    /// It classifies no edge, creates no unit or call, and moves no `AC-1`
    /// count. Five facts fail closed: a non-closure source, a missing child 0,
    /// a duplicate scheduling entry, an authoritative call with no endpoint
    /// record, and any endpoint record left over.
    pub(in crate::cranelift_backend) fn static_body_source_bindings(
        &self,
    ) -> Result<Vec<(PredeclaredFunctionId, StaticOriginId, StaticOriginId)>, CraneliftBackendError>
    {
        let origin_of = |node: StaticNodeId| -> Result<StaticOriginId, CraneliftBackendError> {
            self.semantic
                .descriptors
                .get(node.0 as usize)
                .map(|descriptor| descriptor.origin)
                .ok_or_else(|| planner_error("a static body edge endpoint has no descriptor"))
        };
        let shape_of = |node: StaticNodeId| -> Option<semantic_ir::RuntimeExprShape> {
            self.semantic_sources
                .iter()
                .find(|seed| seed.planned_node == node)
                .and_then(|seed| match seed.source {
                    SemanticSourceKind::Expression(shape) => Some(shape),
                    _ => None,
                })
        };

        // Endpoints only, from the raw edges.
        let mut endpoints: BTreeMap<StaticOriginId, StaticOriginId> = BTreeMap::new();
        for edge in &self.edges {
            if edge.kind != EdgeKind::StaticBody {
                continue;
            }
            // `D2a`: a declaration-owned pair's relation is a definition, not a
            // call, so it mints no endpoint record here either. ⛔ Asked of the
            // semantic plane rather than decided here — the owner
            // classification has one home, and this file is pinned not to name
            // it.
            if self.semantic.is_declaration_owned_static_body(edge)? {
                continue;
            }
            match shape_of(edge.from) {
                Some(semantic_ir::RuntimeExprShape::Closure)
                | Some(semantic_ir::RuntimeExprShape::LexicalClosure) => {}
                _ => {
                    return Err(planner_error(
                        "a static body edge's source is not exactly a Closure or LexicalClosure \
                         occurrence",
                    ));
                }
            }
            let closure_occurrence = origin_of(edge.from)?;
            let source_body = self.semantic.child_origin(closure_occurrence, 0)?;
            let scheduling_entry = origin_of(edge.to)?;
            if endpoints.insert(scheduling_entry, source_body).is_some() {
                return Err(planner_error(
                    "two static body edges declare the same scheduling entry",
                ));
            }
        }

        // One-for-one drain against the authoritative classification.
        let mut bindings = Vec::new();
        for (caller, _callee, callee_origin) in self.semantic.static_body_call_edges(&self.edges)? {
            let source_body = endpoints.remove(&callee_origin).ok_or_else(|| {
                planner_error(
                    "an authoritative static body call has no raw endpoint record for its \
                     scheduling entry",
                )
            })?;
            bindings.push((caller, source_body, callee_origin));
        }
        if !endpoints.is_empty() {
            return Err(planner_error(
                "a raw static body endpoint record was never claimed by an authoritative call",
            ));
        }
        Ok(bindings)
    }

    /// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 1 — the raw worker bodies that
    /// are TEMPLATE-ONLY.**
    ///
    /// Architect ruling `evt_5a0q3m9tnkh8e`. "Unchanged ordinary `fn2` ABI"
    /// preserves the raw worker's **descriptor and source binding** so a
    /// generated context can validate and lower the same body. It does not
    /// require *defining* that body in an environment known to lack the
    /// continuation inputs. The governing population is the **post-retarget
    /// executable call graph**, and this method derives it.
    ///
    /// A body is template-only exactly when **every** route into it is
    /// superseded by a generated context:
    ///
    /// | route into body `B` | superseded when |
    /// |---|---|
    /// | a specialization `S` whose selected worker body is `B` | a generated context exists for `(S, B)` |
    /// | a `StaticBody` or `Declaration` call edge targeting `B` | ⛔ **not at this checkpoint** — see below |
    ///
    /// ⛔ **"A context exists" is NOT a global suppression predicate**, and the
    /// shape above is what stops it becoming one: a mixed caller population --
    /// one specialization retargeted and another not, or any surviving call
    /// edge -- leaves at least one unsuperseded route and keeps the raw
    /// `Function`. The ruling names that case explicitly.
    ///
    /// ## ⚠ Why a `StaticBody` edge is not superseded HERE — MEASURED, and it
    /// refutes the obvious argument
    ///
    /// The tempting reasoning: a `StaticBody` edge into `B` is `B`'s *seeding*
    /// edge, its closure occurrence `c` satisfies `child_origin(c, 0) == B`, and
    /// `child_origin` is injective in `c` -- so if a specialization selects `B`
    /// it selected *that* occurrence, and the edge and the worker call look like
    /// two records of one route. ⇒ Supersede the edge with the worker call.
    ///
    /// **That conclusion is false, and the witness measures it.** One closure
    /// occurrence can be realized **twice in one caller**: once as a
    /// `StaticWorkerBinding` stored into the producer constructor (which becomes
    /// the worker call, and retargets), and once as a **carried computational
    /// re-entry** --
    /// `lower_carried_computational_match` -> `lower_source_machine` ->
    /// `lower_source_machine_with_continuation` ->
    /// `call_declared_recursive_position_unit`, which calls `B` **directly**
    /// through the graph-derived target. Suppressing the edge left that call
    /// with no target and refused with *"retained body has no graph-derived call
    /// target in this unit"*.
    ///
    /// ⇒ The injectivity is real; the inference from it is not. Same occurrence,
    /// two emitted calls, and only one of them retargets. **A call edge
    /// therefore counts as a live route at this checkpoint**, which is the
    /// conservative direction: the cost of being wrong here is an emitted body
    /// that refuses, not a call to a function that does not exist.
    ///
    /// ## ⭐ Checkpoint 4 step 2 — the census re-run, now that the binding exists
    ///
    /// The paragraph above was written when nothing retargeted the carried
    /// invocation, and it said so: *"no edge is superseded yet"*, not *"no edge
    /// can ever be"*. Step 1 built the binding, so this is the promised re-run.
    ///
    /// **What is permanent is that the source edge and its provenance are never
    /// deleted. What moves is the exact emitted callee.** A `StaticBody` edge
    /// into `B` has exactly two realizations, and they are now both accounted
    /// for:
    ///
    /// | realization | retargets when |
    /// |---|---|
    /// | the specialization's static worker call | a generated context exists for `(S, B)` |
    /// | the carried source-machine invocation resuming `S`'s match at `S`'s position | [`Self::carried_invocation_context`] binds one |
    ///
    /// ⇒ The edge is superseded exactly when **every** specialization selecting
    /// `B` binds a context through that same method — the one the emission seam
    /// itself calls, so the census and the emitter cannot drift into two
    /// derivations that disagree.
    ///
    /// ⛔ Still not a global suppression predicate: a `Declaration` edge, or one
    /// specialization without a context, leaves an unsuperseded route and keeps
    /// `B` in branch one with its permanent raw closure refusal governing that
    /// route.
    pub(in crate::cranelift_backend) fn template_only_worker_bodies(
        &self,
    ) -> Result<BTreeSet<StaticOriginId>, CraneliftBackendError> {
        let contexts = self.continuation_contexts()?;
        if contexts.is_empty() {
            // Total and fast: with no generated context nothing is superseded,
            // which is the entire pre-`D5a` program population.
            return Ok(BTreeSet::new());
        }
        let units = self.continuation_units()?;
        let edges = self.emittable_call_edges()?;
        let mut template_only = BTreeSet::new();
        // Only a body some context actually names can be superseded. Starting
        // from the contexts rather than from every unit keeps the candidate set
        // exactly the population the retarget touches.
        let candidates = contexts
            .iter()
            .map(|context| context.worker_body_origin())
            .collect::<BTreeSet<_>>();
        for body in candidates {
            let selecting = units
                .iter()
                .filter(|unit| unit.worker_body_origin() == body)
                .collect::<Vec<_>>();
            if selecting.is_empty() {
                // A context whose body no specialization selects is a planner
                // contradiction, not a licence to suppress.
                return Err(planner_error(
                    "a generated context names a worker body that no specialization selects",
                ));
            }
            let every_specialization_retargeted = selecting.iter().all(|unit| {
                contexts.iter().any(|context| {
                    context.enclosing_specialization() == unit.id()
                        && context.worker_body_origin() == body
                })
            });
            if !every_specialization_retargeted {
                // The mixed caller population the ruling names: at least one
                // specialization still calls the raw worker.
                continue;
            }
            // Every selecting specialization must also bind a generated context
            // for the CARRIED invocation, through the same planner method the
            // emission seam calls. This is what supersedes the `StaticBody`
            // edge's second realization -- the one that, before step 1 existed,
            // was measured to keep this body executable.
            let mut carried_bound = true;
            for unit in &selecting {
                if self
                    .carried_invocation_context(
                        unit.continuation_origin(),
                        unit.recursive_position(),
                        body,
                    )?
                    .is_none()
                {
                    carried_bound = false;
                }
            }
            if !carried_bound {
                continue;
            }
            // A `Declaration` edge is an ordinary direct call and no context
            // stands in for it, so one surviving here keeps the body in branch
            // one. A `StaticBody` edge is superseded by the two retargets
            // above.
            if edges.iter().any(|edge| {
                edge.callee_origin() == body && edge.kind() == EmittableCallKind::Declaration
            }) {
                continue;
            }
            // ⛔ `D8b` withdrew a third clause here — retain a body some
            // composed-call requirement names. It was correct on the planner's
            // own terms and refused one plane later: defining the retained raw
            // body transfers a constructor carrying a raw closure child across
            // the unit boundary, which is the permanent raw closure refusal this
            // very method names above. A composed-call target makes no claim on
            // this population, so nothing replaces it.
            template_only.insert(body);
        }
        Ok(template_only)
    }

    /// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 3 — the computational-match
    /// origins that have a planner-issued source-machine recursive
    /// predecessor.**
    ///
    /// Architect ruling `evt_5a0q3m9tnkh8e` §2: *"Final match-case reachability
    /// must be the union of the initial selection and every planner-issued
    /// source-machine recursive predecessor. An initial static selection may
    /// disposition a case only when no planned recursive/dynamic predecessor
    /// can select it."*
    ///
    /// A match named as the `continuation_origin` of a planner-issued causal
    /// call has exactly that: the call's **return edge** comes back into the
    /// match, and control resumes there with a value the initial scrutinee did
    /// not determine.
    ///
    /// ⛔ Derived from the planner's own causal-call projection, not from
    /// anything lowering observed. That matters because the defect is precisely
    /// that lowering's *observed* selections are incomplete — the initial
    /// scrutinee selects one case and the recursive return can select another,
    /// so an authority built from observation cannot see the second.
    pub(in crate::cranelift_backend) fn source_machine_recursive_predecessor_origins(
        &self,
    ) -> Result<BTreeSet<StaticOriginId>, CraneliftBackendError> {
        Ok(self
            .continuation_calls()?
            .iter()
            .map(|call| call.continuation_origin())
            .collect())
    }

    /// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 4 step 1 — the generated
    /// context one carried source-machine invocation must call.**
    ///
    /// Selected by the invocation's **retained source coordinates**: the exact
    /// computational-match origin it resumes, the ruled recursive position it
    /// occupies, and the worker body it names. ⛔ **Not** by body origin alone,
    /// not by ABI shape, not by "a context exists", and never by first match —
    /// the resolution below requires the candidate set to be a singleton and
    /// rejects otherwise.
    ///
    /// ## The four outcomes, because two of them look alike
    ///
    /// | candidates | contexts among them | result |
    /// |---|---|---|
    /// | none | — | `None` — an ordinary invocation. This is every pre-`D5a` program, and the raw target is correct |
    /// | some | none | `None` — the mixed population: this worker is still called raw here |
    /// | some | exactly one | `Some(context)` — the ruled retarget |
    /// | some | more than one | ⛔ hard stop |
    ///
    /// ⚠ The two `None`s are deliberately the same answer, because the caller's
    /// action is the same: emit the raw target. They differ in *why*, and the
    /// difference matters only to a reader, so it is documented rather than
    /// encoded as a variant a consumer could branch on and get wrong.
    ///
    /// ⛔ **A source edge, predecessor or provenance is never deleted by this.**
    /// It answers which `Function` one already-planned emitted call transfers
    /// to; the call, its edge and its causal ancestry are untouched.
    pub(in crate::cranelift_backend) fn carried_invocation_context(
        &self,
        continuation_origin: StaticOriginId,
        recursive_position: u32,
        worker_body_origin: StaticOriginId,
    ) -> Result<Option<ContinuationContextId>, CraneliftBackendError> {
        let units = self.continuation_units()?;
        let candidates = units
            .iter()
            .filter(|unit| {
                unit.continuation_origin() == continuation_origin
                    && unit.recursive_position() == recursive_position
                    && unit.worker_body_origin() == worker_body_origin
            })
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            return Ok(None);
        }
        let contexts = self.continuation_contexts()?;
        let mut bound = BTreeSet::new();
        for candidate in &candidates {
            for context in &contexts {
                if context.enclosing_specialization() == candidate.id()
                    && context.worker_body_origin() == worker_body_origin
                {
                    bound.insert(context.id());
                }
            }
        }
        let mut bound = bound.into_iter();
        let first = bound.next();
        if bound.next().is_some() {
            return Err(planner_error(
                "a carried source-machine invocation's retained source coordinates bind more than \
                 one generated context; one emitted call cannot transfer to two functions, and \
                 choosing either would make lowering the authority for a fact planning owns",
            ));
        }
        Ok(first)
    }

    // ── `RT-CONTSPEC-ACTIVATE` `D1` — the activation projection ─────────────
    //
    // Read-only and unmintable. Every fact below is already validated planner
    // or ABI material, re-checked here and failing closed; nothing is derived
    // from source syntax and no id, owner, descriptor or call is invented.

    pub(super) fn helper_key_for_activation(
        &self,
        node: StaticNodeId,
        activation: DynamicActivationFrame,
    ) -> Result<PlannedHelperKey, CraneliftBackendError> {
        let static_node = self
            .nodes
            .get(node.0 as usize)
            .ok_or_else(|| planner_error("activation names an unknown static node"))?;
        let store_is_closed =
            |id: PersistentNodeId| id.0 == 0 || id.0 as usize <= self.stores.len();
        for id in [
            activation.syntax,
            activation.environment,
            activation.normal,
            activation.abrupt,
            activation.path,
            activation.cleanup,
            activation.affine,
            activation.source_return,
        ] {
            if !store_is_closed(id) {
                return Err(planner_error(
                    "activation frame references an unclosed persistent node",
                ));
            }
        }
        Ok(PlannedHelperKey::node(static_node.transition, node))
    }

    pub(super) fn validate(&self) -> Result<(), CraneliftBackendError> {
        // ⭐⭐ **`D7` checkpoint `1c` runs FIRST, and the position is argued.**
        //
        // Measured: dropping the member's descriptor is ALSO caught downstream,
        // by the ABI plane's "descriptor population is not exact for the
        // function unit partition". That refusal is true, is in planning, and
        // names neither the member nor the law -- it reports a population size
        // where the cause is a specific worker the specialization population
        // still points at.
        //
        // ⛔ Running first preempts that check **only in the case this law is
        // about**: a descriptor no specialization names still reaches the
        // partition check unchanged, because this law never looks at it. A
        // narrow, correctly-attributed refusal ahead of a broad one is the
        // whole reason for stating the law separately.
        validate_static_worker_member_population(self)?;
        if self.entries.is_empty() {
            return Err(planner_error("closed graph has no entry"));
        }
        if self.evidence.len() != self.edges.len() {
            return Err(planner_error("edge evidence is incomplete"));
        }
        if self.store_depths.len() != self.stores.len() {
            return Err(planner_error(
                "persistent store depth table does not match the store",
            ));
        }
        let mut unique_stores = BTreeSet::new();
        for (index, node) in self.stores.iter().enumerate() {
            if !unique_stores.insert(*node) {
                return Err(planner_error("persistent store contains a duplicate node"));
            }
            let child_depth = if node.child.0 == 0 {
                0
            } else {
                let child_index = node.child.0 as usize - 1;
                if child_index >= index {
                    return Err(planner_error(
                        "persistent store child is not an earlier closed node",
                    ));
                }
                self.store_depths[child_index]
            };
            let depth = child_depth
                .checked_add(1)
                .ok_or_else(|| planner_capacity_error("persistent chain depth exhausted"))?;
            if self.store_depths[index] != depth {
                return Err(planner_error(
                    "persistent store depth does not match its child chain",
                ));
            }
        }

        let mut expected_helpers = BTreeSet::new();
        for (index, node) in self.nodes.iter().enumerate() {
            if node.id.0 as usize != index {
                return Err(planner_error(
                    "static node identity does not match its closed position",
                ));
            }
            expected_helpers.insert(PlannedHelperKey::node(node.transition, node.id));
        }
        let closed_nodes = self
            .nodes
            .iter()
            .map(|node| node.id)
            .collect::<BTreeSet<_>>();
        if self
            .entries
            .iter()
            .any(|entry| entry.0 as usize >= self.nodes.len())
        {
            return Err(planner_error("graph entry is outside the closed node set"));
        }
        if self.entries.iter().copied().collect::<BTreeSet<_>>().len() != self.entries.len() {
            return Err(planner_error("closed graph contains a duplicate entry"));
        }
        for (index, edge) in self.edges.iter().enumerate() {
            if edge.id.0 as usize != index {
                return Err(planner_error(
                    "static edge identity does not match its closed position",
                ));
            }
            if edge.from.0 as usize >= self.nodes.len() || edge.to.0 as usize >= self.nodes.len() {
                return Err(planner_error("edge endpoint is outside the closed graph"));
            }
            expected_helpers.insert(PlannedHelperKey::edge(edge.kind, edge.id));
        }
        let actual_helpers = self
            .planned_helpers
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if actual_helpers.len() != self.planned_helpers.len() {
            return Err(planner_error(
                "planned helper inventory contains a duplicate identity",
            ));
        }
        if actual_helpers != expected_helpers
            || self.planned_helpers.len() != self.nodes.len() + self.edges.len()
        {
            return Err(planner_error(
                "planned helper inventory is not exact for the closed graph",
            ));
        }
        let mut helpers = BTreeMap::<StaticSourceId, usize>::new();
        for helper in &self.planned_helpers {
            let owner =
                match *helper {
                    PlannedHelperKey::Node(transition, id) => {
                        let node = self.nodes.get(id.0 as usize).ok_or_else(|| {
                            planner_error("planned node helper is outside the graph")
                        })?;
                        if transition != node.transition || id != node.id {
                            return Err(planner_error(
                                "planned node helper does not match its static node",
                            ));
                        }
                        node.owner
                    }
                    PlannedHelperKey::Edge(kind, id) => {
                        let edge = self.edges.get(id.0 as usize).ok_or_else(|| {
                            planner_error("planned edge helper is outside the graph")
                        })?;
                        if kind != edge.kind || id != edge.id {
                            return Err(planner_error(
                                "planned edge helper does not match its static edge",
                            ));
                        }
                        self.nodes[edge.from.0 as usize].owner
                    }
                };
            *helpers.entry(owner).or_default() += 1;
        }

        for (index, (edge, evidence)) in self.edges.iter().zip(&self.evidence).enumerate() {
            if evidence.edge as usize != index
                || evidence.owner != self.nodes[edge.from.0 as usize].owner
                || evidence.from != edge.from
                || evidence.to != edge.to
                || evidence.kind != edge.kind
            {
                return Err(planner_error("out-of-line edge evidence is not exact"));
            }
        }
        for node in &self.nodes {
            if self.helper_key_for_activation(node.id, node.frame)?
                != PlannedHelperKey::node(node.transition, node.id)
            {
                return Err(planner_error(
                    "dynamic activation changed static helper identity",
                ));
            }
        }

        let terminals = self
            .nodes
            .iter()
            .filter(|node| node.transition == TransitionKind::Terminal)
            .map(|node| node.id)
            .collect::<Vec<_>>();
        let trap_terminals = self
            .nodes
            .iter()
            .filter(|node| node.transition == TransitionKind::TrapTerminal)
            .map(|node| node.id)
            .collect::<Vec<_>>();
        if terminals.len() != 1 || trap_terminals.len() != 1 {
            return Err(planner_error(
                "closed graph must have exactly one Terminal and TrapTerminal",
            ));
        }
        let terminal = terminals[0];
        let trap_terminal = trap_terminals[0];
        if self
            .edges
            .iter()
            .any(|edge| edge.from == terminal || edge.from == trap_terminal)
        {
            return Err(planner_error(
                "Terminal and TrapTerminal must have no outgoing edges",
            ));
        }

        self.validate_source_return_topology()?;
        if helpers.values().copied().max().unwrap_or(0) > MAX_HELPERS_PER_STATIC_SOURCE {
            return Err(planner_error(
                "fixed K helpers per static source was exceeded",
            ));
        }

        let mut reachable = self.entries.iter().copied().collect::<BTreeSet<_>>();
        reachable.extend([terminal, trap_terminal]);
        loop {
            let before = reachable.len();
            for edge in &self.edges {
                if reachable.contains(&edge.from) {
                    reachable.insert(edge.to);
                }
            }
            if reachable.len() == before {
                break;
            }
        }
        if reachable != closed_nodes {
            return Err(planner_error(
                "closed graph contains unreachable transitions",
            ));
        }
        self.validate_planned_entry_bodies()?;
        self.semantic.validate(
            &self.nodes,
            &self.edges,
            &self.entries,
            &|entry| self.planned_entry_body(entry),
            self.root_entry,
            &self.semantic_sources,
            &self.semantic_material,
        )?;
        self.abi.validate(
            &self.semantic,
            &self.nodes,
            &self.semantic_sources,
            &self.edges,
            &self.entries,
            &self.declaration_occurrences.values().copied().collect(),
            self.root_entry
                .ok_or_else(|| planner_error("plan has no root scheduling entry"))?,
            self.root_ingress,
        )?;
        self.validate_source_occurrence_table()?;
        validate_case_emission_plan(self, &self.case_emissions)?;
        validate_occurrence_authority_plan(self, &self.occurrence_authorities)?;
        validate_aggregate_ownership_plan(self, &self.aggregate_ownership)?;
        validate_host_effect_seat_plan(self, &self.host_effect_seats)?;
        validate_substrate_preallocation_closure(
            self,
            &self.case_emissions,
            &self.occurrence_authorities,
        )?;
        validate_continuation_specialization_plan(self)?;
        self.abi
            .validate_continuation_specializations(&self.continuation_specializations)?;
        self.validate_join_result_plan()?;
        Ok(())
    }

    fn validate_source_return_topology(&self) -> Result<(), CraneliftBackendError> {
        let special = |transition| {
            matches!(
                transition,
                TransitionKind::SourceReturnResume
                    | TransitionKind::ProducerWrapper
                    | TransitionKind::ProducerTail
                    | TransitionKind::CompletedTail
            )
        };
        let owners = self
            .nodes
            .iter()
            .filter(|node| special(node.transition))
            .map(|node| node.owner)
            .collect::<BTreeSet<_>>();
        for owner in owners {
            let one = |transition| {
                let nodes = self
                    .nodes
                    .iter()
                    .filter(|node| node.owner == owner && node.transition == transition)
                    .collect::<Vec<_>>();
                match nodes.as_slice() {
                    [node] => Ok(*node),
                    _ => Err(planner_error(
                        "computational source owner lacks one R/W/T/CompletedTail quartet",
                    )),
                }
            };
            let resume = one(TransitionKind::SourceReturnResume)?;
            let wrapper = one(TransitionKind::ProducerWrapper)?;
            let tail = one(TransitionKind::ProducerTail)?;
            let completed = one(TransitionKind::CompletedTail)?;
            let descriptor = wrapper.frame.source_return;
            if descriptor.0 == 0
                || [resume, tail, completed]
                    .iter()
                    .any(|node| node.frame.source_return != descriptor)
            {
                return Err(planner_error(
                    "computational quartet does not share one source-return descriptor",
                ));
            }
            let stored = self
                .stores
                .get(descriptor.0 as usize - 1)
                .ok_or_else(|| planner_error("source-return descriptor is not closed"))?;
            if stored.kind != StoreKind::SourceReturn
                || stored.local != wrapper.id.0
                || stored.aux != tail.id.0
            {
                return Err(planner_error(
                    "source-return descriptor does not name its exact W and T",
                ));
            }
            self.require_only_outgoing_edge(
                resume.id,
                wrapper.id,
                EdgeKind::InvokeProducerWrapper,
                "source-return resume must have only its exact wrapper invocation",
            )?;
            self.require_only_incoming_edge(
                wrapper.id,
                resume.id,
                EdgeKind::InvokeProducerWrapper,
                "producer wrapper must have only its exact resume invocation",
            )?;
            self.require_only_outgoing_edge(
                wrapper.id,
                tail.id,
                EdgeKind::InvokeProducerTail,
                "producer wrapper must have only its exact tail invocation",
            )?;
            self.require_only_incoming_edge(
                tail.id,
                wrapper.id,
                EdgeKind::InvokeProducerTail,
                "producer tail must have only its exact wrapper invocation",
            )?;
            self.require_only_outgoing_edge(
                tail.id,
                completed.id,
                EdgeKind::CompleteProducerTail,
                "producer tail must have only its exact completion edge",
            )?;
            self.require_only_incoming_edge(
                completed.id,
                tail.id,
                EdgeKind::CompleteProducerTail,
                "CompletedTail must have only its exact producer-tail completion",
            )?;
            if self.entries.contains(&wrapper.id) {
                return Err(planner_error(
                    "producer wrapper cannot be a pre-source graph entry",
                ));
            }

            let successor = self.activation_successor(completed)?;
            let completed_edges = self
                .edges
                .iter()
                .filter(|edge| edge.from == completed.id)
                .collect::<Vec<_>>();
            if !matches!(completed_edges.as_slice(), [edge] if edge.to == successor) {
                return Err(planner_error(
                    "CompletedTail must have only its activation-named successor",
                ));
            }
            let completed_edge = completed_edges[0];
            let successor_transition = self.nodes[successor.0 as usize].transition;
            let expected_kind = if successor_transition == TransitionKind::SourceReturnResume {
                EdgeKind::SourceReturnOwnedResume
            } else {
                EdgeKind::Continue
            };
            if completed_edge.kind != expected_kind {
                return Err(planner_error(
                    "CompletedTail successor does not use its normal-resume edge kind",
                ));
            }
        }
        for edge in self
            .edges
            .iter()
            .filter(|edge| edge.kind == EdgeKind::SourceReturnOwnedResume)
        {
            let from = &self.nodes[edge.from.0 as usize];
            let to = &self.nodes[edge.to.0 as usize];
            let edge_descriptor = if from.transition == TransitionKind::CompletedTail {
                let descriptor_index =
                    from.frame.source_return.0.checked_sub(1).ok_or_else(|| {
                        planner_error(
                            "CompletedTail source return does not name a closed parent descriptor",
                        )
                    })? as usize;
                self.stores
                    .get(descriptor_index)
                    .filter(|descriptor| descriptor.kind == StoreKind::SourceReturn)
                    .map(|descriptor| descriptor.child)
                    .ok_or_else(|| {
                        planner_error(
                            "CompletedTail source return does not name a closed parent descriptor",
                        )
                    })?
            } else {
                from.frame.source_return
            };
            if to.transition != TransitionKind::SourceReturnResume
                || edge_descriptor.0 == 0
                || edge_descriptor != to.frame.source_return
            {
                return Err(planner_error(
                    "source-return-owned edge targets a resume from another descriptor",
                ));
            }
        }
        for edge in self
            .edges
            .iter()
            .filter(|edge| edge.kind == EdgeKind::CompleteProducerTail)
        {
            let from = &self.nodes[edge.from.0 as usize];
            let to = &self.nodes[edge.to.0 as usize];
            if from.transition != TransitionKind::ProducerTail
                || to.transition != TransitionKind::CompletedTail
                || from.owner != to.owner
            {
                return Err(planner_error(
                    "producer completion is not owned by one computational source",
                ));
            }
        }
        Ok(())
    }

    fn activation_successor(
        &self,
        node: &StaticNode,
    ) -> Result<StaticNodeId, CraneliftBackendError> {
        let continuation_index =
            node.frame.normal.0.checked_sub(1).ok_or_else(|| {
                planner_error("activation does not name a closed normal continuation")
            })? as usize;
        let continuation = self
            .stores
            .get(continuation_index)
            .filter(|store| store.kind == StoreKind::Continuation)
            .ok_or_else(|| {
                planner_error("activation does not name a closed normal continuation")
            })?;
        let successor = StaticNodeId(continuation.local);
        if successor.0 as usize >= self.nodes.len() {
            return Err(planner_error(
                "activation normal continuation is outside the closed graph",
            ));
        }
        Ok(successor)
    }

    fn require_only_outgoing_edge(
        &self,
        from: StaticNodeId,
        to: StaticNodeId,
        kind: EdgeKind,
        error: &'static str,
    ) -> Result<(), CraneliftBackendError> {
        let edges = self
            .edges
            .iter()
            .filter(|edge| edge.from == from)
            .collect::<Vec<_>>();
        if !matches!(edges.as_slice(), [edge] if edge.to == to && edge.kind == kind) {
            return Err(planner_error(error));
        }
        Ok(())
    }

    fn require_only_incoming_edge(
        &self,
        to: StaticNodeId,
        from: StaticNodeId,
        kind: EdgeKind,
        error: &'static str,
    ) -> Result<(), CraneliftBackendError> {
        let edges = self
            .edges
            .iter()
            .filter(|edge| edge.to == to)
            .collect::<Vec<_>>();
        if !matches!(edges.as_slice(), [edge] if edge.from == from && edge.kind == kind) {
            return Err(planner_error(error));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn census(&self) -> BoundaryACensus {
        let max_depth = |kind| {
            self.stores
                .iter()
                .zip(&self.store_depths)
                .filter_map(|(node, depth)| (node.kind == kind).then_some(*depth))
                .max()
                .unwrap_or(0)
        };
        let mut helpers = BTreeMap::<StaticSourceId, usize>::new();
        for helper in &self.planned_helpers {
            let owner = match *helper {
                PlannedHelperKey::Node(_, id) => self.nodes[id.0 as usize].owner,
                PlannedHelperKey::Edge(_, id) => {
                    let edge = self.edges[id.0 as usize];
                    self.nodes[edge.from.0 as usize].owner
                }
            };
            *helpers.entry(owner).or_default() += 1;
        }
        BoundaryACensus {
            static_nodes: self.nodes.len(),
            edges: self.edges.len(),
            planned_helpers: self.planned_helpers.len(),
            persistent_store_nodes: self.stores.len(),
            out_of_line_evidence_records: self.evidence.len(),
            max_helpers_per_static_source: helpers.values().copied().max().unwrap_or(0),
            helper_key_bytes: std::mem::size_of::<PlannedHelperKey>(),
            activation_frame_bytes: std::mem::size_of::<DynamicActivationFrame>(),
            store_node_bytes: std::mem::size_of::<PersistentStoreNode>(),
            helper_key_schemas: 1,
            frame_schemas: 1,
            store_node_schemas: 1,
            static_node_id_bytes: std::mem::size_of::<StaticNodeId>(),
            persistent_node_id_bytes: std::mem::size_of::<PersistentNodeId>(),
            max_logical_chain_depth: self.store_depths.iter().copied().max().unwrap_or(0),
            max_environment_depth: max_depth(StoreKind::Environment),
            max_continuation_depth: max_depth(StoreKind::Continuation),
            max_path_depth: max_depth(StoreKind::Path),
            max_cleanup_depth: max_depth(StoreKind::Cleanup),
            max_affine_depth: max_depth(StoreKind::Affine),
            max_source_return_depth: max_depth(StoreKind::SourceReturn),
            source_return_resume_nodes: self
                .nodes
                .iter()
                .filter(|node| node.transition == TransitionKind::SourceReturnResume)
                .count(),
            source_return_owned_resume_edges: self
                .edges
                .iter()
                .filter(|edge| edge.kind == EdgeKind::SourceReturnOwnedResume)
                .count(),
            terminal_outgoing_edges: self
                .edges
                .iter()
                .filter(|edge| {
                    matches!(
                        self.nodes[edge.from.0 as usize].transition,
                        TransitionKind::Terminal | TransitionKind::TrapTerminal
                    )
                })
                .count(),
            recursive_lowering_frames: max_recursive_lowering_frame_count(),
        }
    }

    /// Capture the complete plan/semantic/ABI material that the emission
    /// collector will bind to its completed-object row.
    ///
    /// Descriptor work is counted in explicit representation work units: one
    /// descriptor header plus each slot it constructs, and the same closed
    /// population compared by `AbiPlane::validate`.  This keeps the metric tied
    /// to the actual descriptor/slot population rather than to wall-clock
    /// sampling or a source-text proxy.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn scale_b_census(&self) -> ScaleBPlanCensus {
        let outer = self.census();
        let descriptor_work = self
            .abi
            .descriptors
            .len()
            .checked_add(self.abi.slots.len())
            .expect("the descriptor work population fits usize");
        ScaleBPlanCensus {
            static_nodes: outer.static_nodes,
            edges: outer.edges,
            planned_helpers: outer.planned_helpers,
            persistent_store_nodes: outer.persistent_store_nodes,
            out_of_line_evidence_records: outer.out_of_line_evidence_records,
            max_helpers_per_static_source: outer.max_helpers_per_static_source,
            helper_key_bytes: outer.helper_key_bytes,
            activation_frame_bytes: outer.activation_frame_bytes,
            store_node_bytes: outer.store_node_bytes,
            helper_key_schemas: outer.helper_key_schemas,
            frame_schemas: outer.frame_schemas,
            store_node_schemas: outer.store_node_schemas,
            static_node_id_bytes: outer.static_node_id_bytes,
            persistent_node_id_bytes: outer.persistent_node_id_bytes,
            max_logical_chain_depth: outer.max_logical_chain_depth,
            max_environment_depth: outer.max_environment_depth,
            max_continuation_depth: outer.max_continuation_depth,
            max_path_depth: outer.max_path_depth,
            max_cleanup_depth: outer.max_cleanup_depth,
            max_affine_depth: outer.max_affine_depth,
            max_source_return_depth: outer.max_source_return_depth,
            source_return_resume_nodes: outer.source_return_resume_nodes,
            source_return_owned_resume_edges: outer.source_return_owned_resume_edges,
            terminal_outgoing_edges: outer.terminal_outgoing_edges,
            recursive_lowering_frames: outer.recursive_lowering_frames,
            distinct_interned_semantic_states: self.semantic.records.len(),
            defined_helpers: self.semantic.functions.len(),
            descriptor_construction_work: descriptor_work,
            descriptor_comparison_work: descriptor_work,
        }
    }

    #[cfg(test)]
    pub(super) fn semantic_census(&self) -> BoundaryB1Census {
        use semantic_ir::{
            CaptureLayout, CaptureSlot, PredeclaredFunction, RuledChild, SemanticDescriptor,
            SemanticOperandElement, SemanticProgram, SemanticRecord,
        };

        let opcode_vocabulary = self
            .semantic
            .records
            .iter()
            .map(|record| record.opcode)
            .collect::<BTreeSet<_>>()
            .len();
        let mut definitions = BTreeMap::new();
        for descriptor in &self.semantic.descriptors {
            *definitions.entry(descriptor.origin).or_insert(0usize) += 1;
        }
        let distinct_origins = definitions.len();
        let duplicate_origin_definitions = definitions
            .values()
            .map(|count| count.saturating_sub(1))
            .sum();
        let max_definitions_per_origin = definitions.values().copied().max().unwrap_or(0);
        let definitions_per_origin = if definitions
            .values()
            .all(|count| *count == max_definitions_per_origin)
        {
            max_definitions_per_origin
        } else {
            0
        };

        BoundaryB1Census {
            opcode_vocabulary,
            distinct_origins,
            ir_records: self.semantic.records.len(),
            semantic_edges: self.semantic.ruled_children.len(),
            function_units: self.semantic.functions.len(),
            definitions_per_origin,
            all_out_of_line_operand_elements: self.semantic.all_out_of_line_operand_elements(),
            duplicate_origin_definitions,
            post_origin_clones: self
                .semantic
                .programs
                .len()
                .saturating_sub(distinct_origins),
            max_definitions_per_origin,
            descriptor_bytes: std::mem::size_of::<SemanticDescriptor>(),
            program_bytes: std::mem::size_of::<SemanticProgram>(),
            record_bytes: std::mem::size_of::<SemanticRecord>(),
            operand_element_bytes: std::mem::size_of::<SemanticOperandElement>(),
            capture_layout_bytes: std::mem::size_of::<CaptureLayout>(),
            capture_slot_bytes: std::mem::size_of::<CaptureSlot>(),
            ruled_child_bytes: std::mem::size_of::<RuledChild>(),
            function_bytes: std::mem::size_of::<PredeclaredFunction>(),
        }
    }
}
