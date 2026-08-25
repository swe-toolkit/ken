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
    validate_checked_ih_environment_transports, AggregateOccurrenceId, AggregateOccurrenceProducer,
    PlannedAggregateAllocation, PlannedAggregateOwnership, PlannedAggregateShape,
    SynthesizedAggregateNode, SynthesizedAggregatePath, SynthesizedAggregateRole,
    SynthesizedAggregateRoot, SynthesizedDynamicSet,
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
    self, BoolMatchCaseOrdinals, ConstructorIdentity, FieldIdentity, SemanticSourceKind,
    SynthesizedConstructorRole, SynthesizedIoErrorRole,
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

    /// Classify one `Match` occurrence as the exact canonical Bool family.
    /// Lowering receives only the False/True ordinals, never the private role
    /// vocabulary or its symbol spellings.
    pub(in crate::cranelift_backend) fn bool_match_case_ordinals(
        &self,
        origin: StaticOriginId,
    ) -> Result<Option<BoolMatchCaseOrdinals>, CraneliftBackendError> {
        self.semantic.bool_match_case_ordinals(origin)
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
        // Transport derivation consumes continuation identities, so the
        // continuation plane must close before any transport validator may
        // interpret it. This preserves the originating plane's exact refusal
        // under continuation mutations instead of masking it with a derivative
        // transport error.
        validate_continuation_specialization_plan(self)?;
        validate_aggregate_ownership_plan(self, &self.aggregate_ownership)?;
        validate_checked_ih_environment_transports(self, &self.checked_ih_environment_transports)?;
        validate_host_effect_seat_plan(self, &self.host_effect_seats)?;
        validate_substrate_preallocation_closure(
            self,
            &self.case_emissions,
            &self.occurrence_authorities,
        )?;
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

#[cfg(test)]
mod tests {

    use super::super::*;
    use crate::{RuntimeMatchCase, RuntimeTrap, RuntimeTrapCode, RuntimeValue};
    use super::super::tests::{
        D2G_DECLARATION, b2o_retained_closure, b2o_transparent_declaration, b2o_two_closure_fixture,
        census, contspec_complete_environment_fixture, contspec_required_tail_fixture,
        contsrc_d2_ih_and_argument_case_fixture, d2g_declaration, d2g_entry, d2g_oriented_plan,
        equal_shaped_atom_fixture, equal_shaped_child_fixture, fixture_witness,
        nested_resource_bracket, nodes_of_shape, substrate_case, substrate_constructor, trap, unit,
    };
    use super::super::semantic_ir::{
        build_semantic_plane, DenseRange, RuntimeExprShape, SemanticAtomKind,
        SemanticOperandElement, SemanticOwner, SemanticSourceKind,
    };


    #[cfg(test)]
    pub(in crate::cranelift_backend::planning::static_transition) fn d2h_plane_fixture() -> (
        RuntimeDeclaration,
        RuntimeExpr,
        crate::OrientedSubcontinuationPlanV1,
    ) {
        (d2g_declaration(true), d2g_entry(), d2g_oriented_plan())
    }

    /// `D2h` — the bijection round-trips on production planner state.
    ///
    /// key -> ID -> key returns the same key, ID -> descriptor resolves, and the
    /// descriptor's members are checked against the key rather than merely being
    /// present, so a descriptor paired with the wrong id would fail.
    #[test]
    fn d2h_the_key_id_descriptor_bijection_round_trips() {
        let (declaration, entry, oriented) = d2h_plane_fixture();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");

        let fusion =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("the plane builds");
        assert_eq!(fusion.len(), 1, "the gate measured exactly one candidate");

        let id = StaticContinuationFusionId(0);
        let key = fusion.key_for(id).expect("ID -> key").clone();
        assert_eq!(fusion.id_for(&key), Some(id), "key -> ID -> key round-trips");

        let descriptor = fusion.descriptor_for(id).expect("ID -> descriptor");
        assert_eq!(descriptor.id, id);
        assert_eq!(descriptor.recursive_position, key.recursive_position);
        assert_eq!(descriptor.consumer_owner, key.consumer_owner);
        assert_eq!(descriptor.producer_owner, key.producer_owner);
        assert_eq!(
            descriptor.continuation_inputs,
            key.continuation_inputs.len(),
            "the descriptor's shape is the key's, not an independent number"
        );
        assert_ne!(
            key.producer_owner, key.consumer_owner,
            "and the interned identity is the cross-unit one the fusion exists for"
        );
    }

    /// `D2h` `AC-1` — the INTERNER-UNIT matrix: every one-member mutation is
    /// submitted to the production interner and receives its own ID.
    ///
    /// This is the interner's property, not a lookup table's. An earlier
    /// revision only asked `id_for` whether a perturbed key still resolved --
    /// a read, which cannot show that an unequal key would be GIVEN an identity
    /// of its own. Here each mutation is interned, so the plane actually mints a
    /// second id, and both keys then round-trip.
    ///
    /// Same-key reuse is proved alongside it: re-submitting an equal key returns
    /// the id it already has rather than minting a second one, which is the
    /// other half of the identity relation being a function.
    ///
    /// These are SYNTHETIC keys, deliberately. Whether the planner can produce
    /// two valid keys differing in each member is the derivation/provenance
    /// question, and it lives in `D2j`; what is established here is that the
    /// interner keys on the complete structure.
    #[test]
    fn d2h_ac1_every_one_member_mutation_interns_to_its_own_id() {
        let (declaration, entry, oriented) = d2h_plane_fixture();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let mut fusion =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("the plane builds");
        assert_eq!(fusion.len(), 1);

        let base = fusion
            .key_for(StaticContinuationFusionId(0))
            .expect("key")
            .clone();
        let base_id = fusion.intern(base.clone()).expect("intern");
        assert_eq!(
            base_id,
            StaticContinuationFusionId(0),
            "same-key reuse: an equal key returns the id it already has"
        );
        assert_eq!(fusion.len(), 1, "and mints nothing new");

        let bump = |origin: StaticOriginId| StaticOriginId(origin.0 + 1);
        let mut variants: Vec<(&'static str, StaticContinuationFusionKey)> = Vec::new();
        let mut push = |label: &'static str, mutate: &dyn Fn(&mut StaticContinuationFusionKey)| {
            let mut key = base.clone();
            mutate(&mut key);
            variants.push((label, key));
        };

        push("admitted.continuation_origin", &|k| {
            k.admitted.continuation_origin = bump(k.admitted.continuation_origin)
        });
        push("admitted.result_root", &|k| {
            k.admitted.result_root = bump(k.admitted.result_root)
        });
        push("admitted.enclosing_specialization", &|k| {
            k.admitted.enclosing_specialization = None
        });
        push("producer_construct_origin", &|k| {
            k.producer_construct_origin = bump(k.producer_construct_origin)
        });
        push("producer_owner", &|k| {
            k.producer_owner = PredeclaredFunctionId(k.producer_owner.0 + 1)
        });
        push("producer_alternative", &|k| k.producer_alternative += 1);
        push("recursive_position", &|k| k.recursive_position += 1);
        push("producer_argument_origin", &|k| {
            k.producer_argument_origin = bump(k.producer_argument_origin)
        });
        push("producer_argument_binding", &|k| {
            k.producer_argument_binding.recursive_position += 1
        });
        push("selected_case_body", &|k| {
            k.selected_case_body = bump(k.selected_case_body)
        });
        push("consuming_call", &|k| k.consuming_call = bump(k.consuming_call));
        push("consuming_callee", &|k| {
            k.consuming_callee = bump(k.consuming_callee)
        });
        push("consumer_binding", &|k| {
            k.consumer_binding.frame_origin = bump(k.consumer_binding.frame_origin)
        });
        push("transport.frame_id", &|k| k.checked_transport.frame_id += 1);
        push("transport.slot_template_id", &|k| {
            k.checked_transport.slot_template_id += 1
        });
        push("transport.slot_occurrence_path", &|k| {
            k.checked_transport.slot_occurrence_path.push(99)
        });
        push("transport.call_template_id", &|k| {
            k.checked_transport.call_template_id += 1
        });
        push("transport.call_occurrence_path", &|k| {
            k.checked_transport.call_occurrence_path.push(99)
        });
        push("invocation_caller", &|k| {
            k.invocation_caller = PredeclaredFunctionId(k.invocation_caller.0 + 1)
        });
        push("invocation_callee", &|k| {
            k.invocation_callee = PredeclaredFunctionId(k.invocation_callee.0 + 1)
        });
        push("invocation_callee_entry", &|k| {
            k.invocation_callee_entry = bump(k.invocation_callee_entry)
        });
        push("consumer_owner", &|k| {
            k.consumer_owner = PredeclaredFunctionId(k.consumer_owner.0 + 1)
        });
        // `continuation_inputs` is NOT varied here: the projection is empty on
        // this witness, so every mutation of it would be a no-op. A non-empty
        // projection is `D2j`'s fixture obligation, and asserting a no-op would
        // read as coverage.
        assert!(
            base.continuation_inputs.is_empty(),
            "if this witness gains a projected input, vary that class too rather \
             than leaving the exclusion: {:?}",
            base.continuation_inputs
        );

        let mut seen = BTreeSet::new();
        seen.insert(base_id);
        for (label, variant) in &variants {
            assert_ne!(
                variant, &base,
                "the {label} mutation must actually change the key, or it interns nothing new"
            );
            let id = fusion.intern(variant.clone()).expect("intern");
            assert!(
                seen.insert(id),
                "the {label} mutation must mint its OWN id, not reuse one already issued"
            );
            assert_eq!(
                fusion.key_for(id),
                Some(variant),
                "and {label}'s id must round-trip to the key it was minted from"
            );
            assert_eq!(
                fusion.id_for(variant),
                Some(id),
                "and back again"
            );
            assert!(
                fusion.descriptor_for(id).is_some(),
                "and carry a descriptor of its own"
            );
        }
        assert_eq!(
            fusion.len(),
            1 + variants.len(),
            "one identity per distinct key, and no collisions"
        );
        assert_eq!(
            fusion.id_for(&base),
            Some(base_id),
            "and the original key still resolves to its original id"
        );
    }


    /// `D2h` — the independent re-derivation CATCHES a mutation of the primary
    /// derivation.
    ///
    /// The two routes are genuinely different: the primary builds the key from
    /// the candidate, and the re-derivation rebuilds every member from planner
    /// facts -- the case declaration, the semantic child inventory, the
    /// checked-IH authority, the transport map, the `StaticBody` edges and the
    /// producer environment -- without reading the candidate.
    ///
    /// Perturbing only the primary must therefore be caught. An earlier revision
    /// compared two identical enumerator runs, which agree by construction and
    /// proved only determinism; this control is what distinguishes the two
    /// designs.
    #[test]
    fn d2h_the_independent_rederivation_catches_a_mutated_primary_derivation() {
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                set_primary_fusion_key_derivation_mutated(false);
            }
        }

        let (declaration, entry, oriented) = d2h_plane_fixture();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");

        let baseline =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("the unmutated plane builds");
        assert_eq!(baseline.len(), 1, "the baseline mints one identity");

        let _restore = Restore;
        set_primary_fusion_key_derivation_mutated(true);
        let caught =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect_err("a mutated primary derivation must be caught");
        // MEASURED, and sharper than the whole-key comparison I expected. The
        // mutation perturbs the admitted root, which the re-derivation now
        // ESTABLISHES against the production ledger before deriving anything
        // from it -- so it is refused at establishment rather than surviving to
        // the final equality. That is the stronger place to be caught: a locator
        // that cannot be justified never gets to select the members that hang
        // off it.
        assert!(
            format!("{caught:?}")
                .contains("admitted discovery is not in the production ledger"),
            "and caught where the locator is established, not merely at the closing \
             comparison: {caught:?}"
        );
    }

    /// `D2h` `AC-2` — the three expressible refusals, each before any ID or
    /// descriptor exists.
    ///
    /// Checked at the PLANE, where an id and a descriptor would exist if
    /// anything had been minted: an empty plane is the operational meaning of
    /// "before interning", because there is no id to inspect.
    ///
    /// The baseline mints one identity first, so each zero is a change and not
    /// the fixture's resting state. The other six `AC-2` causes are not here:
    /// `ContinuationProductionMutation` cannot express any of them, which was
    /// measured, and they relocate rather than being silently dropped.
    #[test]
    fn d2h_ac2_the_three_expressible_refusals_mint_nothing() {
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                set_post_specialization_descent_suppressed(false);
                set_static_body_triple_duplicated(false);
            }
        }

        let (declaration, entry, oriented) = d2h_plane_fixture();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");

        let baseline =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("the plane builds");
        assert_eq!(baseline.len(), 1);
        assert!(baseline
            .descriptor_for(StaticContinuationFusionId(0))
            .is_some());

        // 1. Stripped transport.
        let stripped = build_static_continuation_fusion_plan(&plan, &entry, &declarations, None)
            .expect_err("markers with no plan must refuse");
        assert!(
            format!("{stripped:?}")
                .contains("checked subcontinuation markers have no checked plan"),
            "at the TRANSPORT gate: {stripped:?}"
        );

        let _restore = Restore;

        // 2. Suppressed post-specialization descent.
        set_post_specialization_descent_suppressed(true);
        let no_descent =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("builds with nothing to intern");
        assert!(
            no_descent.is_empty(),
            "no key, id or descriptor without the descent root: {no_descent:?}"
        );
        assert_eq!(
            no_descent.descriptor_for(StaticContinuationFusionId(0)),
            None
        );
        set_post_specialization_descent_suppressed(false);

        // 3. A duplicated actual StaticBody edge.
        set_static_body_triple_duplicated(true);
        let ambiguous =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("builds with nothing to intern");
        assert!(
            ambiguous.is_empty(),
            "a second matching StaticBody edge must mint nothing: {ambiguous:?}"
        );
        assert_eq!(
            ambiguous.descriptor_for(StaticContinuationFusionId(0)),
            None
        );
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn values(rows: &[BoundaryACensus], field: impl Fn(&BoundaryACensus) -> usize) -> Vec<isize> {
        rows.iter().map(|row| field(row) as isize).collect()
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn differences(values: &[isize]) -> (Vec<isize>, Vec<isize>) {
        let first = values.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        let second = first.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        (first, second)
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn semantic_census(depth: usize) -> (BoundaryACensus, BoundaryB1Census) {
        let expr = nested_resource_bracket(depth);
        plan_static_transition_graph(&expr, &BTreeMap::new())
            .map(|plan| (plan.census(), plan.semantic_census()))
            .unwrap_or_else(|error| {
                panic!("RT_NATIVE_FNSPLIT_B1 could_not_determine n={depth}: {error}")
            })
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn semantic_values(
        rows: &[BoundaryB1Census],
        field: impl Fn(&BoundaryB1Census) -> usize,
    ) -> Vec<isize> {
        rows.iter().map(|row| field(row) as isize).collect()
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn index_of_edge_helper(plan: &StaticTransitionPlan, edge: StaticEdgeId) -> usize {
        plan.planned_helpers
            .iter()
            .position(|helper| matches!(helper, PlannedHelperKey::Edge(_, id) if *id == edge))
            .expect("edge has a planned helper")
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn rewrite_edge(
        plan: &mut StaticTransitionPlan,
        edge: StaticEdgeId,
        from: StaticNodeId,
        to: StaticNodeId,
        kind: EdgeKind,
    ) {
        let index = edge.0 as usize;
        plan.edges[index] = StaticEdge {
            id: edge,
            from,
            to,
            kind,
        };
        plan.evidence[index] = EdgeEvidence {
            edge: edge.0,
            owner: plan.nodes[from.0 as usize].owner,
            from,
            to,
            kind,
        };
        let helper = index_of_edge_helper(plan, edge);
        plan.planned_helpers[helper] = PlannedHelperKey::edge(kind, edge);
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn append_edge(
        plan: &mut StaticTransitionPlan,
        from: StaticNodeId,
        to: StaticNodeId,
        kind: EdgeKind,
    ) {
        let id = StaticEdgeId(plan.edges.len() as u32);
        plan.edges.push(StaticEdge { id, from, to, kind });
        plan.evidence.push(EdgeEvidence {
            edge: id.0,
            owner: plan.nodes[from.0 as usize].owner,
            from,
            to,
            kind,
        });
        plan.planned_helpers.push(PlannedHelperKey::edge(kind, id));
    }

    #[test]
    fn boundary_b1_nested_resource_brackets_n3_through_n7_are_closed_and_affine() {
        // Promise class: durable invariant. The finite differences corroborate
        // the builder's structural one-node/one-edge/one-flattening traversal;
        // they are not the asymptotic proof.
        let rows = (3..=7).map(semantic_census).collect::<Vec<_>>();
        let outer = rows
            .iter()
            .map(|(outer, _)| outer.clone())
            .collect::<Vec<_>>();
        let semantic = rows
            .iter()
            .map(|(_, semantic)| semantic.clone())
            .collect::<Vec<_>>();

        for (depth, (outer, row)) in (3..=7).zip(&rows) {
            eprintln!(
                "RT_NATIVE_FNSPLIT_B1 n={depth} opcode_vocabulary={} origins={} \
                 ir_records={} semantic_edges={} function_units={} \
                 definitions_per_origin={} operand_elements={} duplicate_origins={} \
                 clone_count={} max_definitions_per_origin={} fixed_k={} \
                 descriptor_bytes={} program_bytes={} record_bytes={} \
                 operand_element_bytes={} capture_layout_bytes={} capture_slot_bytes={} \
                 ruled_child_bytes={} function_bytes={}",
                row.opcode_vocabulary,
                row.distinct_origins,
                row.ir_records,
                row.semantic_edges,
                row.function_units,
                row.definitions_per_origin,
                row.all_out_of_line_operand_elements,
                row.duplicate_origin_definitions,
                row.post_origin_clones,
                row.max_definitions_per_origin,
                outer.max_helpers_per_static_source,
                row.descriptor_bytes,
                row.program_bytes,
                row.record_bytes,
                row.operand_element_bytes,
                row.capture_layout_bytes,
                row.capture_slot_bytes,
                row.ruled_child_bytes,
                row.function_bytes,
            );
        }

        for (name, metric) in [
            (
                "distinct_origins",
                semantic_values(&semantic, |row| row.distinct_origins),
            ),
            (
                "ir_records",
                semantic_values(&semantic, |row| row.ir_records),
            ),
            (
                "semantic_edges",
                semantic_values(&semantic, |row| row.semantic_edges),
            ),
            (
                "function_units",
                semantic_values(&semantic, |row| row.function_units),
            ),
            (
                "all_out_of_line_operand_elements",
                semantic_values(&semantic, |row| row.all_out_of_line_operand_elements),
            ),
        ] {
            let (first, second) = differences(&metric);
            eprintln!(
                "RT_NATIVE_FNSPLIT_B1_DIFF metric={name} values={metric:?} \
                 first={first:?} second={second:?}"
            );
            assert!(
                second.iter().all(|difference| *difference == 0),
                "{name} is not affine across n=3..7"
            );
        }

        for (name, metric) in [
            (
                "opcode_vocabulary",
                semantic_values(&semantic, |row| row.opcode_vocabulary),
            ),
            (
                "definitions_per_origin",
                semantic_values(&semantic, |row| row.definitions_per_origin),
            ),
            (
                "max_definitions_per_origin",
                semantic_values(&semantic, |row| row.max_definitions_per_origin),
            ),
            (
                "duplicate_origin_definitions",
                semantic_values(&semantic, |row| row.duplicate_origin_definitions),
            ),
            (
                "post_origin_clones",
                semantic_values(&semantic, |row| row.post_origin_clones),
            ),
            (
                "descriptor_bytes",
                semantic_values(&semantic, |row| row.descriptor_bytes),
            ),
            (
                "program_bytes",
                semantic_values(&semantic, |row| row.program_bytes),
            ),
            (
                "record_bytes",
                semantic_values(&semantic, |row| row.record_bytes),
            ),
            (
                "operand_element_bytes",
                semantic_values(&semantic, |row| row.operand_element_bytes),
            ),
            (
                "capture_layout_bytes",
                semantic_values(&semantic, |row| row.capture_layout_bytes),
            ),
            (
                "capture_slot_bytes",
                semantic_values(&semantic, |row| row.capture_slot_bytes),
            ),
            (
                "ruled_child_bytes",
                semantic_values(&semantic, |row| row.ruled_child_bytes),
            ),
            (
                "function_bytes",
                semantic_values(&semantic, |row| row.function_bytes),
            ),
        ] {
            let (first, second) = differences(&metric);
            eprintln!(
                "RT_NATIVE_FNSPLIT_B1_DIFF metric={name} values={metric:?} \
                 first={first:?} second={second:?}"
            );
            assert!(
                metric.windows(2).all(|pair| pair[0] == pair[1]),
                "{name} is not pairwise constant across n=3..7"
            );
        }

        let fixed_k = outer
            .iter()
            .map(|row| row.max_helpers_per_static_source as isize)
            .collect::<Vec<_>>();
        let (fixed_k_first, fixed_k_second) = differences(&fixed_k);
        eprintln!(
            "RT_NATIVE_FNSPLIT_B1_DIFF metric=fixed_k values={fixed_k:?} \
             first={fixed_k_first:?} second={fixed_k_second:?}"
        );
        assert_eq!(
            fixed_k,
            vec![8, 8, 8, 8, 8],
            "B1 grew or obscured the already-full outer helper inventory"
        );
        assert!(semantic.iter().all(|row| {
            row.opcode_vocabulary == 6
                && row.definitions_per_origin == 1
                && row.max_definitions_per_origin == 1
                && row.duplicate_origin_definitions == 0
                && row.post_origin_clones == 0
        }));
    }

    #[test]
    fn boundary_b1_preserves_equal_occurrences_and_reuses_one_activation_program() {
        // Promise class: durable invariant. Equal source text is the
        // discriminating counterexample to semantic hash-consing.
        let equal_occurrences = RuntimeExpr::If {
            scrutinee: Box::new(unit()),
            then_expr: Box::new(unit()),
            else_expr: Box::new(unit()),
        };
        let plan = plan_static_transition_graph(&equal_occurrences, &BTreeMap::new()).unwrap();
        let equal_nodes = plan
            .semantic_sources
            .iter()
            .filter_map(|source| {
                (source.source == SemanticSourceKind::Expression(RuntimeExprShape::Construct))
                    .then_some(source.planned_node)
            })
            .collect::<Vec<_>>();
        assert_eq!(equal_nodes.len(), 3);
        let descriptors = equal_nodes
            .iter()
            .map(|node| plan.semantic.descriptors[node.0 as usize])
            .collect::<Vec<_>>();
        assert_eq!(
            descriptors
                .iter()
                .map(|descriptor| descriptor.origin)
                .collect::<BTreeSet<_>>()
                .len(),
            3,
            "equal source occurrences were semantic-hash-consed"
        );
        assert_eq!(
            descriptors
                .iter()
                .map(|descriptor| descriptor.program)
                .collect::<BTreeSet<_>>()
                .len(),
            3,
            "equal source occurrences lost positional programs"
        );
        let records = descriptors
            .iter()
            .map(|descriptor| plan.semantic.records[descriptor.program.0 as usize])
            .collect::<Vec<_>>();
        assert!(records.windows(2).all(|pair| {
            pair[0].opcode == pair[1].opcode
                && pair[0].operands.len == pair[1].operands.len
                && pair[0].origin != pair[1].origin
        }));

        let node = equal_nodes[0];
        let static_node = plan.nodes[node.0 as usize];
        let other_activation = plan
            .nodes
            .iter()
            .map(|candidate| candidate.frame)
            .find(|frame| *frame != static_node.frame)
            .expect("fixture has another closed activation frame");
        let descriptor_before = plan.semantic.descriptors[node.0 as usize];
        assert_eq!(
            plan.helper_key_for_activation(node, static_node.frame)
                .unwrap(),
            plan.helper_key_for_activation(node, other_activation)
                .unwrap()
        );
        assert_eq!(
            plan.semantic.descriptors[node.0 as usize], descriptor_before,
            "another activation minted a program or origin"
        );
    }

    #[test]
    fn boundary_b1_semantics_are_discovery_order_and_dynamic_state_independent() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let mut reversed_sources = plan.semantic_sources.clone();
        reversed_sources.reverse();
        let mut reordered = build_semantic_plane(
            &plan.nodes,
            &plan.edges,
            &plan.entries,
            &|entry| plan.planned_entry_body(entry),
            plan.root_entry,
            &reversed_sources,
            &plan.semantic_material,
        )
        .unwrap();
        let mut reordered_material = plan.semantic_material.clone();
        let (reordered_roles, reordered_io_roles) = build_synthesized_constructor_inventory(
            &mut reordered_material,
            &crate::NativeProcessSymbols::legacy_prelude(),
        )
        .unwrap();
        reordered.install_synthesized_constructor_inventory(reordered_roles, reordered_io_roles);
        let reordered_bool_roles = build_bool_constructor_inventory(
            &mut reordered_material,
            &crate::NativeProcessSymbols::legacy_prelude(),
        )
        .unwrap();
        reordered.install_bool_constructor_inventory(reordered_bool_roles);
        assert_eq!(reordered, plan.semantic);

        let mut changed_frames = plan.nodes.clone();
        let frames = plan.nodes.iter().map(|node| node.frame).collect::<Vec<_>>();
        assert!(
            frames.iter().any(|frame| *frame != frames[0]),
            "frame rotation is a no-op: all frames are equal, so this control proves nothing"
        );
        for (index, node) in changed_frames.iter_mut().enumerate() {
            node.frame = frames[(index + 1) % frames.len()];
        }
        let mut changed = build_semantic_plane(
            &changed_frames,
            &plan.edges,
            &plan.entries,
            &|entry| plan.planned_entry_body(entry),
            plan.root_entry,
            &reversed_sources,
            &plan.semantic_material,
        )
        .unwrap();
        let mut changed_material = plan.semantic_material.clone();
        let (changed_roles, changed_io_roles) = build_synthesized_constructor_inventory(
            &mut changed_material,
            &crate::NativeProcessSymbols::legacy_prelude(),
        )
        .unwrap();
        changed.install_synthesized_constructor_inventory(changed_roles, changed_io_roles);
        let changed_bool_roles = build_bool_constructor_inventory(
            &mut changed_material,
            &crate::NativeProcessSymbols::legacy_prelude(),
        )
        .unwrap();
        changed.install_bool_constructor_inventory(changed_bool_roles);
        assert_eq!(
            changed, plan.semantic,
            "dynamic activation state changed semantic programs or bodies"
        );
        assert_eq!(plan.semantic.descriptors.len(), plan.nodes.len());
        // RT-FNSPLIT-B2O, re-baselined 2026-07-25 from `plan.nodes.len()`.
        //
        // PREDICTED FROM THE DESIGN BEFORE MEASURING, and this is the reason:
        // the function table is no longer a positional alias of the node table,
        // so it is seed-exact rather than node-exact. The unit set is
        // `plan.entries` (root plus each transparent declaration) union every
        // `EdgeKind::StaticBody` target (each retained closure-body entry), and
        // those two classes are disjoint, so the count is their sum.
        //
        // ⛔ Asserted RELATIONALLY against the two seed classes, never against
        // the observed number. A count re-fit to whatever the code now emits
        // measures nothing; this form goes red if either seed class stops being
        // enumerated, which is the property `D1` actually claims.
        assert_eq!(
            plan.semantic.functions.len(),
            plan.entries.len()
                + plan
                    .edges
                    .iter()
                    .filter(|edge| edge.kind == EdgeKind::StaticBody)
                    .count()
        );
        assert_eq!(plan.semantic.ruled_children.len(), plan.edges.len());
    }

    #[test]
    fn boundary_b1_negative_controls_fail_at_named_semantic_artifacts() {
        // Promise class: durable mutation proof.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let pointer_origins = plan
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                (
                    std::ptr::from_ref(node) as usize,
                    StaticOriginId(((index + 1) % plan.nodes.len()) as u32),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut pointer_recovery = plan.semantic.clone();
        pointer_recovery.descriptors[0].origin =
            pointer_origins[&(std::ptr::from_ref(&plan.nodes[0]) as usize)];
        assert_eq!(
            pointer_recovery
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("descriptor origin is not its preallocated positional identity")
        );

        let equal_occurrences = RuntimeExpr::If {
            scrutinee: Box::new(unit()),
            then_expr: Box::new(unit()),
            else_expr: Box::new(unit()),
        };
        let equal_plan =
            plan_static_transition_graph(&equal_occurrences, &BTreeMap::new()).unwrap();
        let equal_nodes = equal_plan
            .semantic_sources
            .iter()
            .filter_map(|source| {
                (source.source == SemanticSourceKind::Expression(RuntimeExprShape::Construct))
                    .then_some(source.planned_node)
            })
            .collect::<Vec<_>>();
        let mut hash_cons = equal_plan.semantic.clone();
        hash_cons.descriptors[equal_nodes[1].0 as usize].origin =
            hash_cons.descriptors[equal_nodes[0].0 as usize].origin;
        assert_eq!(
            hash_cons
                .validate(
                    &equal_plan.nodes,
                    &equal_plan.edges,
                    &equal_plan.entries,
                    &|entry| equal_plan.planned_entry_body(entry),
                    equal_plan.root_entry,
                    &equal_plan.semantic_sources,
                    &equal_plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("semantic hash-consing merged distinct static origins")
        );

        let mut second_definition = plan.semantic.clone();
        second_definition
            .descriptors
            .push(second_definition.descriptors[0]);
        assert_eq!(
            second_definition
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("planned node has more than one semantic definition")
        );

        let mut post_origin_clone = plan.semantic.clone();
        post_origin_clone
            .programs
            .push(post_origin_clone.programs[0]);
        assert_eq!(
            post_origin_clone
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("semantic program arena contains a post-origin clone")
        );

        let mut superlinear_material = plan.semantic.clone();
        let deliberate_square = plan.nodes.len().checked_mul(plan.nodes.len()).unwrap();
        superlinear_material
            .operands
            .extend(
                (0..deliberate_square).map(|ordinal| SemanticOperandElement {
                    kind: SemanticAtomKind::LocalIndex,
                    content: DenseRange { start: 0, len: 0 },
                    payload: ordinal as u64,
                }),
            );
        superlinear_material.records[0].operands.len = superlinear_material.records[0]
            .operands
            .len
            .checked_add(deliberate_square as u32)
            .unwrap();
        assert_eq!(
            superlinear_material
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("semantic operand arena exceeds the one-visit source-material budget")
        );
    }

    /// ⚠ **NEGATIVE CONTROL for [`fixture_witness`]** — without it, "the
    /// witnesses compared equal" and "the witness silently admitted a closure"
    /// are indistinguishable.
    #[test]
    fn p2_the_fixture_witness_refuses_closures() {
        // Promise class: durable invariant.
        assert!(
            fixture_witness(&RuntimeExpr::Closure {
                captures: vec![],
                params: vec!["x".to_string()],
                body: Box::new(RuntimeExpr::Var(0)),
            })
            .is_none(),
            "a Closure must not produce a witness"
        );
        assert!(
            fixture_witness(&RuntimeExpr::Value(RuntimeValue::ClosureRef {
                symbol: "decl:fixture::f".to_string(),
                captured: vec![],
            }))
            .is_none(),
            "a ClosureRef value must not produce a witness"
        );
        // ⛔ And transitively: a closure NESTED inside admitted grammar refuses
        // the whole tree, rather than the parent succeeding around it.
        assert!(
            fixture_witness(&RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::Closure {
                    captures: vec![],
                    params: vec!["x".to_string()],
                    body: Box::new(RuntimeExpr::Var(0)),
                }),
                body: Box::new(RuntimeExpr::Var(0)),
            })
            .is_none(),
            "refusal is transitive through admitted parents"
        );

        // ⚠ POSITIVE CONTROL — the fixture grammar itself DOES produce a
        // witness, so the three refusals above are not a witness that refuses
        // everything.
        assert!(
            fixture_witness(&equal_shaped_child_fixture()).is_some(),
            "the fixture grammar produces a witness"
        );
    }

    #[test]
    fn boundary_b1r_control_1_swapping_equal_shaped_occurrence_material_is_rejected() {
        // Promise class: durable mutation proof. This is the load-bearing
        // control: the swapped pair agrees on shape, opcode and every count, so
        // it is exactly the case B1's counted placeholders could not see.
        let expr = equal_shaped_atom_fixture();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let vars = nodes_of_shape(&plan, RuntimeExprShape::Var);
        assert_eq!(
            vars.len(),
            2,
            "fixture must hold two equal-shaped occurrences"
        );

        let first = plan.semantic.records[vars[0].0 as usize];
        let second = plan.semantic.records[vars[1].0 as usize];
        assert_eq!(
            (first.opcode, first.operands.len, first.child_origins.len),
            (second.opcode, second.operands.len, second.child_origins.len),
            "the pair is not equal-shaped, so this control would prove nothing"
        );
        let before = (
            plan.semantic.operands[first.operands.start as usize],
            plan.semantic.operands[second.operands.start as usize],
        );
        assert_ne!(
            before.0, before.1,
            "the pair's material is identical, so a swap is a no-op and this \
             control would prove nothing"
        );

        let mut swapped = plan.semantic.clone();
        for offset in 0..first.operands.len as usize {
            swapped.operands.swap(
                first.operands.start as usize + offset,
                second.operands.start as usize + offset,
            );
        }
        assert_eq!(
            swapped
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("semantic material record is not occurrence-exact for its origin")
        );

        // The same swap on positional children of an equal-shaped pair.
        let child_expr = equal_shaped_child_fixture();
        let child_plan = plan_static_transition_graph(&child_expr, &BTreeMap::new()).unwrap();
        let lets = nodes_of_shape(&child_plan, RuntimeExprShape::Let);
        assert_eq!(lets.len(), 2);
        let first = child_plan.semantic.records[lets[0].0 as usize];
        let second = child_plan.semantic.records[lets[1].0 as usize];
        assert_eq!(
            (first.opcode, first.operands.len, first.child_origins.len),
            (second.opcode, second.operands.len, second.child_origins.len),
            "the pair is not equal-shaped, so this control would prove nothing"
        );
        assert_eq!(first.child_origins.len, 2, "a Let owns value and body");
        let mut swapped_children = child_plan.semantic.clone();
        for offset in 0..first.child_origins.len as usize {
            swapped_children.child_origins.swap(
                first.child_origins.start as usize + offset,
                second.child_origins.start as usize + offset,
            );
        }
        assert_eq!(
            swapped_children
                .validate(
                    &child_plan.nodes,
                    &child_plan.edges,
                    &child_plan.entries,
                    &|entry| child_plan.planned_entry_body(entry),
                    child_plan.root_entry,
                    &child_plan.semantic_sources,
                    &child_plan.semantic_material,
                )
                .unwrap_err(),
            planner_error(
                "semantic child origins are not occurrence-exact for their source positions"
            )
        );
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn primitive_call(symbol: &str, partiality: crate::RuntimePartiality) -> RuntimeExpr {
        RuntimeExpr::PrimitiveCall {
            primitive: crate::RuntimePrimitive {
                symbol: symbol.to_string(),
                partiality,
            },
            args: Vec::new(),
        }
    }

    /// Two `PrimitiveCall` occurrences sharing one symbol and one (empty)
    /// argument shape, differing only in the partiality that lowering branches on.
    pub(in crate::cranelift_backend::planning::static_transition) fn equal_shaped_primitive_pair(
        left: crate::RuntimePartiality,
        right: crate::RuntimePartiality,
    ) -> RuntimeExpr {
        RuntimeExpr::Let {
            value: Box::new(primitive_call("ken.bytes.at", left)),
            body: Box::new(primitive_call("ken.bytes.at", right)),
        }
    }

    /// Decodes one record's single descriptor atom back out of the closed name
    /// arena, so a control asserts on the material's CONTENT and not on the
    /// incidental fact that two occurrences interned at different offsets.
    pub(in crate::cranelift_backend::planning::static_transition) fn descriptor_bytes(plan: &StaticTransitionPlan, node: StaticNodeId) -> Vec<u8> {
        let record = plan.semantic.records[node.0 as usize];
        assert_eq!(record.operands.len, 1, "a primitive owns one atom");
        let atom = plan.semantic.operands[record.operands.start as usize];
        assert_eq!(atom.kind, SemanticAtomKind::PrimitiveDescriptor);
        let start = atom.content.start as usize;
        plan.semantic.names[start..start + atom.content.len as usize].to_vec()
    }

    /// Asserts that an equal-shaped primitive pair differing only in partiality
    /// has genuinely different material, and that cross-wiring one occurrence's
    /// descriptor onto the other reddens at occurrence-exactness.
    pub(in crate::cranelift_backend::planning::static_transition) fn assert_partiality_is_occurrence_exact(
        left: crate::RuntimePartiality,
        right: crate::RuntimePartiality,
        case: &str,
    ) {
        let expr = equal_shaped_primitive_pair(left, right);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let calls = nodes_of_shape(&plan, RuntimeExprShape::PrimitiveCall);
        assert_eq!(calls.len(), 2, "{case}: fixture must hold two occurrences");

        let first = plan.semantic.records[calls[0].0 as usize];
        let second = plan.semantic.records[calls[1].0 as usize];
        assert_eq!(
            (first.opcode, first.operands.len, first.child_origins.len),
            (second.opcode, second.operands.len, second.child_origins.len),
            "{case}: the pair is not equal-shaped, so this control proves nothing"
        );
        assert_ne!(
            descriptor_bytes(&plan, calls[0]),
            descriptor_bytes(&plan, calls[1]),
            "{case}: the two primitives encode identical material, so the plane \
             cannot tell them apart and B2a would emit the wrong behaviour"
        );

        // Cross-wire: point the first occurrence's descriptor at the second's
        // encoded content. Shape, opcode, counts and atom kind all still agree.
        let mut cross_wired = plan.semantic.clone();
        let victim = first.operands.start as usize;
        cross_wired.operands[victim].content =
            cross_wired.operands[second.operands.start as usize].content;
        assert_eq!(
            cross_wired
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("semantic material record is not occurrence-exact for its origin"),
            "{case}: a cross-wired primitive descriptor was not caught"
        );
    }

    #[test]
    fn boundary_b1r_primitive_partiality_is_occurrence_exact_material() {
        // Promise class: durable mutation proof. Partiality changes what
        // lowering emits (immediate trap versus continue, plus distinct
        // constructor/obligation/assumption material), so a symbol-only atom
        // would let these occurrences share one body while lowering differently.
        assert_partiality_is_occurrence_exact(
            crate::RuntimePartiality::Total,
            crate::RuntimePartiality::CheckedTrap {
                obligation: "ken.bytes.at.inBounds".to_string(),
            },
            "distinct partiality variants",
        );

        // A variant-tag-only encoding would pass the case above and fail here:
        // same variant, one differing field.
        assert_partiality_is_occurrence_exact(
            crate::RuntimePartiality::SafeOption {
                none: "None".to_string(),
                some: "Some".to_string(),
                obligation: None,
            },
            crate::RuntimePartiality::SafeOption {
                none: "Nothing".to_string(),
                some: "Some".to_string(),
                obligation: None,
            },
            "same variant, one differing field",
        );

        // The optional field must also discriminate, so its presence byte is
        // load-bearing rather than decorative.
        assert_partiality_is_occurrence_exact(
            crate::RuntimePartiality::SafeOption {
                none: "None".to_string(),
                some: "Some".to_string(),
                obligation: None,
            },
            crate::RuntimePartiality::SafeOption {
                none: "None".to_string(),
                some: "Some".to_string(),
                obligation: Some("ken.bytes.at.inBounds".to_string()),
            },
            "same variant, optional field present versus absent",
        );
    }

    const IDENTITY_CTOR: &str = "ctor:prelude::Pair::MkPair";
    const IDENTITY_OTHER_CTOR: &str = "ctor:prelude::Triple::MkTriple";
    const IDENTITY_FIELD: &str = "field:fst";

    /// A `Match` whose scrutinee **constructs** the same constructor one of its
    /// cases **eliminates**, and whose case body projects a field the record
    /// beneath it **declares**.
    ///
    /// ⭐ The point of the shape is that each spelling appears at **two distinct
    /// occurrences** with different atom kinds — `ConstructorSymbol` vs
    /// `CaseConstructor`, `RecordFieldName` vs `ProjectField`. That is what makes
    /// the equality assertions below non-trivial: they are comparing a
    /// *producer's* identity against a *consumer's*, which is exactly `D2`.
    ///
    /// Positional child layout, verified against the planner's own construction
    /// rather than assumed: `Match` pushes the scrutinee then the case bodies
    /// (`children.push(scrutinee); children.extend(case_bodies)`), and `Project`
    /// plans its record at position 0.
    pub(in crate::cranelift_backend::planning::static_transition) fn identity_fixture() -> RuntimeExpr {
        RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: IDENTITY_CTOR.to_string(),
                args: Vec::new(),
            }),
            cases: vec![
                RuntimeMatchCase {
                    constructor: IDENTITY_CTOR.to_string(),
                    binders: 0,
                    body: RuntimeExpr::Project {
                        record: Box::new(RuntimeExpr::Record {
                            fields: vec![(IDENTITY_FIELD.to_string(), unit())],
                        }),
                        field: IDENTITY_FIELD.to_string(),
                    },
                },
                RuntimeMatchCase {
                    constructor: IDENTITY_OTHER_CTOR.to_string(),
                    binders: 0,
                    body: unit(),
                },
            ],
            default: trap("identity fixture default"),
        }
    }

    /// `RT-FNSPLIT-C1` `D2` — the producer and the eliminator derive **one**
    /// constructor identity, and different spellings stay distinct.
    ///
    /// **MEASURED:** `constructor_symbol_identity` at the `Construct` occurrence
    /// equals `case_constructor_identity` at the `Match` occurrence, and differs
    /// from the identity of a differently-spelled case.
    /// **CLAIMED:** producer and consumer share one authority (`D2`).
    /// **THE GAP:** the two readings must come from **different occurrences** —
    /// otherwise equality is trivially true of any scheme at all, including the
    /// per-occurrence spans this node replaced. That is asserted, not assumed.
    #[test]
    fn boundary_c1_producer_and_eliminator_share_one_constructor_identity() {
        // Promise class: durable invariant.
        let expr = identity_fixture();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let match_origin = plan.root_static_origin().unwrap();
        let construct_origin = plan.child_static_origin(match_origin, 0).unwrap();

        assert_ne!(
            match_origin, construct_origin,
            "NON-VACUITY: the produced and eliminated identities must be read at \
             two different occurrences, or their equality says nothing about \
             sharing an authority."
        );

        let produced = plan.constructor_symbol_identity(construct_origin).unwrap();
        let eliminated = plan.case_constructor_identity(match_origin, 0).unwrap();
        assert_eq!(
            produced, eliminated,
            "the constructor built at one occurrence and matched at another have \
             different artifact-static identities, so producer and consumer are \
             not sharing one authority"
        );

        // Discriminator: a different spelling must not collide.
        let other = plan.case_constructor_identity(match_origin, 1).unwrap();
        assert_ne!(
            produced, other,
            "two differently-spelled constructors share an identity"
        );
    }

    /// `RT-FNSPLIT-C1` `D2` — the record's declared field and the projection's
    /// selected field are one identity.
    #[test]
    fn boundary_c1_declared_and_projected_field_share_one_identity() {
        // Promise class: durable invariant.
        let expr = identity_fixture();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let match_origin = plan.root_static_origin().unwrap();
        let project_origin = plan.child_static_origin(match_origin, 1).unwrap();
        let record_origin = plan.child_static_origin(project_origin, 0).unwrap();

        assert_ne!(
            project_origin, record_origin,
            "NON-VACUITY: the declared and selected field identities must be read \
             at two different occurrences."
        );

        let selected = plan.project_field_identity(project_origin).unwrap();
        let declared = plan.record_field_identity(record_origin, 0).unwrap();
        assert_eq!(
            selected, declared,
            "the field declared by a record and the field selected by a projection \
             over it have different artifact-static identities"
        );
    }

    /// `RT-FNSPLIT-C1` `D1` — the capability refuses a wrong-kind or
    /// out-of-cardinality access rather than returning a plausible identity.
    #[test]
    fn boundary_c1_identity_capability_refuses_wrong_kind_and_cardinality() {
        // Promise class: durable invariant.
        let expr = identity_fixture();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let match_origin = plan.root_static_origin().unwrap();
        let construct_origin = plan.child_static_origin(match_origin, 0).unwrap();

        // Cardinality: the fixture has two cases, so index 2 does not exist.
        assert!(plan.case_constructor_identity(match_origin, 2).is_err());

        // Wrong kind: a `Construct` occurrence has a `ConstructorSymbol` atom
        // and no `ProjectField` atom, so asking it for a field identity must
        // fail rather than fall back to whatever named atom it does hold.
        assert!(plan.project_field_identity(construct_origin).is_err());

        // The positive direction, so the two refusals above are attributable to
        // the kind/cardinality checks and not to an origin that resolves nothing.
        assert!(plan.constructor_symbol_identity(construct_origin).is_ok());
    }

    /// `RT-FNSPLIT-C1` `D2` — equal name bytes have exactly one canonical span.
    ///
    /// **MEASURED:** across every atom of a real plan, atoms whose interned
    /// bytes are equal have equal `content` spans.
    /// **CLAIMED:** a producer and an eliminator at *different occurrences*
    /// derive the *same* artifact-static identity for the same spelling.
    /// **THE GAP:** the identity must be a function of the span alone — which
    /// is why `pack_identity` is the sole encoding and why the newtypes wrap the
    /// span rather than carrying any second field.
    ///
    /// ⛔ **The non-vacuity guard is the load-bearing half of this test.** A
    /// fixture in which no spelling repeats satisfies the canonicalization
    /// assertion trivially and would stay green against an interner that never
    /// deduplicates at all — which is precisely the pre-`C1` behaviour this
    /// test exists to detect. So the repeat count is asserted, not assumed.
    #[test]
    fn boundary_c1_equal_name_bytes_have_one_canonical_span() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let mut spans_by_bytes: BTreeMap<Vec<u8>, Vec<DenseRange>> = BTreeMap::new();
        for atom in &plan.semantic.operands {
            let start = atom.content.start as usize;
            let bytes = plan.semantic.names[start..start + atom.content.len as usize].to_vec();
            spans_by_bytes.entry(bytes).or_default().push(atom.content);
        }

        let repeated = spans_by_bytes
            .iter()
            .filter(|(bytes, spans)| !bytes.is_empty() && spans.len() > 1)
            .count();
        assert!(
            repeated > 0,
            "NON-VACUITY: no non-empty spelling occurs twice in this fixture, so the \
             canonicalization assertion below is trivially satisfied and would not \
             detect an interner that never deduplicates."
        );

        for (bytes, spans) in &spans_by_bytes {
            let first = spans[0];
            let deviant = spans.iter().find(|span| **span != first);
            assert!(
                deviant.is_none(),
                "spelling {:?} is interned at both {:?} and {:?}, so one symbol has more \
                 than one artifact-static identity",
                String::from_utf8_lossy(bytes),
                first,
                deviant.unwrap()
            );
        }
    }

    /// `RT-FNSPLIT-C1` `D2` — the plane refuses a two-identity symbol.
    ///
    /// ⭐ This is the control for the *validator*, not for `intern`. An `intern`
    /// that regressed to an unconditional append leaves every span in bounds and
    /// every budget exact, so every pre-existing check stays green while
    /// producer and consumer quietly stop sharing an identity. The plane has to
    /// assert canonicality itself rather than trusting the function that is
    /// supposed to maintain it.
    #[test]
    fn boundary_c1_validate_rejects_equal_bytes_interned_at_two_spans() {
        // Promise class: durable mutation proof.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        // The unmutated plane is green — the inverse half, so a red result below
        // is attributable to the mutation and not to a fixture that never validated.
        plan.semantic
            .validate(
                &plan.nodes,
                &plan.edges,
                &plan.entries,
                &|entry| plan.planned_entry_body(entry),
                plan.root_entry,
                &plan.semantic_sources,
                &plan.semantic_material,
            )
            .expect("the unmutated plane validates");

        // ⛔ The mutation must NOT grow `names`.
        //
        // `validate` already requires `plane.names == arena.names`, and that
        // check runs first. Appending a duplicate copy of a spelling therefore
        // trips the arena-equality check and never reaches the canonicality
        // one — the first draft of this control did exactly that and proved
        // nothing about the property it names.
        //
        // ⭐ So the two equal-byte spans are manufactured *inside* the existing
        // arena: whole symbols are unique after canonicalization, but their
        // BYTES are not — two distinct symbols routinely share a first byte.
        // Pointing two atoms at one-byte spans over the same byte value at
        // different offsets yields equal content at unequal spans with `names`
        // byte-for-byte untouched.
        let mut duplicated = plan.semantic.clone();
        let candidates = duplicated
            .operands
            .iter()
            .filter(|atom| atom.content.len > 0)
            .map(|atom| atom.content)
            .collect::<Vec<_>>();
        let (first, second) = candidates
            .iter()
            .enumerate()
            .find_map(|(i, a)| {
                candidates[i + 1..]
                    .iter()
                    .find(|b| {
                        b.start != a.start
                            && duplicated.names[a.start as usize]
                                == duplicated.names[b.start as usize]
                    })
                    .map(|b| (*a, *b))
            })
            .expect(
                "NON-VACUITY: the fixture has no two out-of-line atoms starting at \
                 different offsets with the same first byte, so this mutation cannot \
                 manufacture equal bytes at unequal spans and the control is vacuous.",
            );

        for atom in duplicated.operands.iter_mut() {
            if atom.content == first {
                atom.content = DenseRange {
                    start: first.start,
                    len: 1,
                };
            } else if atom.content == second {
                atom.content = DenseRange {
                    start: second.start,
                    len: 1,
                };
            }
        }
        assert_eq!(
            duplicated.names, plan.semantic.names,
            "the mutation must leave the name arena byte-identical, or the \
             arena-equality check fires before the canonicality check and this \
             control measures the wrong rejection"
        );

        assert_eq!(
            duplicated
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error(
                "equal semantic name bytes are interned at two different spans, \
                 so one symbol has two identities"
            )
        );
    }

    /// `RT-FNSPLIT-C1` `D1` — the one identity ABI encoding is injective and
    /// reserves zero.
    ///
    /// ⚠ `start = 0, len = 0` is a **legitimate** identity (the empty name at
    /// offset zero), which is the entire reason the encoding adds one. Without
    /// the `+1` that identity would encode as `0` and be indistinguishable from
    /// uninitialized ABI memory.
    #[test]
    fn boundary_c1_identity_abi_word_round_trips_and_reserves_zero() {
        // Promise class: normative compatibility vector — the encoding is the
        // contract between the planner and the carrier's emitted ABI.
        for (start, len) in [
            (0u32, 0u32),
            (0, 1),
            (1, 0),
            (7, 3),
            (u32::MAX, u32::MAX - 1),
        ] {
            let span = DenseRange { start, len };
            let packed = ConstructorIdentity(span).tag_abi_word().unwrap();
            assert_ne!(packed, 0, "({start},{len}) encoded as the invalid sentinel");
            assert_eq!(
                super::super::semantic_ir::unpack_identity(packed).unwrap(),
                span,
                "({start},{len}) did not round trip"
            );
        }

        // Both namespaces share the one encoding, so a field identity and a
        // constructor identity over the same span agree numerically. That is
        // intended: the separation is carried by the *type*, not by the number.
        let span = DenseRange { start: 9, len: 4 };
        assert_eq!(
            ConstructorIdentity(span).tag_abi_word().unwrap(),
            FieldIdentity(span).name_abi_word().unwrap()
        );

        assert_eq!(
            super::super::semantic_ir::unpack_identity(0).unwrap_err(),
            planner_error("semantic identity is the reserved invalid sentinel")
        );

        // ⭐ Capacity loudness. The `+1` that reserves zero costs exactly one
        // encodable span at the very top of the range, and the refusal must be
        // a loud capacity error rather than a wrap to the sentinel — a silent
        // wrap would hand emitted code the "invalid" word for a valid symbol.
        assert_eq!(
            ConstructorIdentity(DenseRange {
                start: u32::MAX,
                len: u32::MAX
            })
            .tag_abi_word()
            .unwrap_err(),
            planner_capacity_error("semantic identity encoding exhausted")
        );
    }

    #[test]
    fn boundary_b1r_atom_content_must_stay_inside_the_closed_name_arena() {
        // Promise class: durable mutation proof. B2a decodes atom content, so a
        // structurally well-formed atom whose span escapes the arena, or whose
        // bytes are not the ones the walk interned, is undecodable material.
        let expr = equal_shaped_primitive_pair(
            crate::RuntimePartiality::Total,
            crate::RuntimePartiality::CheckedTrap {
                obligation: "ken.bytes.at.inBounds".to_string(),
            },
        );
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let mut escaped = plan.semantic.clone();
        let atom = escaped
            .operands
            .iter_mut()
            .find(|atom| atom.content.len > 0)
            .expect("fixture has an atom with out-of-line content");
        atom.content.start = u32::try_from(plan.semantic.names.len()).unwrap();
        assert_eq!(
            escaped
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("semantic atom content range is outside its closed name arena")
        );

        let mut retagged = plan.semantic.clone();
        retagged.names.push(0xff);
        assert_eq!(
            retagged
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error(
                "semantic atom content arena is not the material the source walk interned"
            )
        );
    }

    #[test]
    fn boundary_b1r_control_2_dropping_one_origins_material_record_is_rejected() {
        // Promise class: durable mutation proof.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let carrier = plan
            .semantic
            .records
            .iter()
            .position(|record| record.operands.len > 0)
            .expect("fixture has an occurrence with non-child material");

        // Drop this origin's ownership of its material while leaving the atom
        // arena intact, so the global one-visit budget still balances and only
        // the per-record artifact can catch it. Removing the atoms instead would
        // redden at the arena-budget artifact the superlinear control already
        // owns, which would not discriminate this fault.
        let mut dropped = plan.semantic.clone();
        assert!(dropped.records[carrier].operands.len > 0);
        dropped.records[carrier].operands.len = 0;
        assert_eq!(
            dropped.operands.len(),
            plan.semantic.operands.len(),
            "the atom arena must be untouched, or a different artifact fires"
        );
        assert_eq!(
            dropped
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("semantic record does not own its exact source-material range")
        );
    }

    #[test]
    fn boundary_b1r_control_3_duplicating_a_material_record_origin_is_rejected() {
        // Promise class: durable mutation proof.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let mut duplicated = plan.semantic.clone();
        duplicated.records[1].origin = duplicated.records[0].origin;
        assert_eq!(
            duplicated
                .validate(
                    &plan.nodes,
                    &plan.edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("semantic program is not the exhaustive lowering of its source")
        );
    }

    #[test]
    fn boundary_a_nested_resource_brackets_n3_through_n7_are_closed_and_affine() {
        const WORKER_ENV: &str = "KEN_RT_SCALE_A_CENSUS_WORKER";
        const FORCE_INDETERMINATE_ENV: &str = "KEN_RT_SCALE_A_FORCE_INDETERMINATE";
        const OMIT_RESULT_ENV: &str = "KEN_RT_SCALE_A_OMIT_RESULT";
        const COMPLETE_RESULT: &str = "RT_NATIVE_FNSPLIT_BOUNDARY_A_RESULT \
             status=measured_complete rows=5 stack_bytes=8388608";
        if std::env::var_os(WORKER_ENV).is_none() {
            let run_worker = |force_indeterminate: bool, omit_result: bool| {
                let executable = std::env::current_exe().unwrap_or_else(|error| {
                    panic!("RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: {error}")
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
                    // This isolated process's incidental libtest thread only
                    // dispatches the deliberately-created 8 MiB planner
                    // thread below. `prlimit` bounds the process and catches
                    // aborts; no recursive planning runs on libtest's stack.
                    // Do not inherit the repository's 256 MiB convention.
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
                        "RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: \
                         prlimit worker could not start: {error}"
                    )
                });
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(45);
                loop {
                    match child.try_wait() {
                        Ok(Some(_)) => {
                            break child.wait_with_output().unwrap_or_else(|error| {
                                panic!(
                                    "RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: \
                                     worker result could not be collected: {error}"
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
                                    "RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: \
                                     timed-out worker could not be reaped: {error}"
                                )
                            });
                        }
                        Err(error) => {
                            let _ = child.kill();
                            panic!(
                                "RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: \
                                 worker status could not be observed: {error}"
                            );
                        }
                    }
                }
            };

            // AC-A1 positive control: the third outcome must be observable and
            // must fail. This is not merely a successful-worker smoke test.
            let forced = run_worker(true, false);
            let forced_report = format!(
                "{}{}",
                String::from_utf8_lossy(&forced.stdout),
                String::from_utf8_lossy(&forced.stderr)
            );
            assert!(
                !forced.status.success() && forced_report.contains("could_not_determine"),
                "AC-A1: forced indeterminacy must fail with the stable third-outcome spelling; \
                 status={:?}, report={forced_report}",
                forced.status
            );

            // A zero exit is not enough: missing/malformed result data is the
            // same third outcome, not a silent pass.
            let omitted = run_worker(false, true);
            let omitted_report = format!(
                "{}{}",
                String::from_utf8_lossy(&omitted.stdout),
                String::from_utf8_lossy(&omitted.stderr)
            );
            assert!(
                omitted.status.success() && !omitted_report.contains(COMPLETE_RESULT),
                "AC-A1: the missing-result control must reach a zero exit without \
                 accidentally emitting a complete census"
            );

            let measured = run_worker(false, false);
            let measured_report = format!(
                "{}{}",
                String::from_utf8_lossy(&measured.stdout),
                String::from_utf8_lossy(&measured.stderr)
            );
            eprint!("{measured_report}");
            assert!(
                measured.status.success() && measured_report.contains(COMPLETE_RESULT),
                "RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: bounded worker \
                 stack_bytes=8388608 status={:?}, complete result sentinel missing or malformed",
                measured.status,
            );
            return;
        }

        if std::env::var_os(FORCE_INDETERMINATE_ENV).is_some() {
            panic!(
                "RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: \
                 stack_bytes=8388608 forced fail-closed positive control"
            );
        }
        if std::env::var_os(OMIT_RESULT_ENV).is_some() {
            return;
        }

        let planner_worker = std::thread::Builder::new()
            .name("rt-scale-a-planner-8-mib".to_string())
            .stack_size(8 * 1024 * 1024)
            .spawn(|| {
        eprintln!(
            "RT_NATIVE_FNSPLIT_BOUNDARY_A_STACK \
             worker=rt-scale-a-planner-8-mib stack=nominal_8_MiB stack_bytes=8388608 \
             process_main_stack_limit=8_MiB cpu_limit=30_s address_space_limit=4_GiB \
             claim=explicit_product_stack_measurement"
        );

        // Promise class: durable invariant. Counts remain relational; the
        // historic literals below are comparison data, never a re-baseline or
        // an exponent inferred from five points.
        let rows = (3..=7).map(census).collect::<Vec<_>>();
        for (depth, row) in (3..=7).zip(&rows) {
            eprintln!(
                "RT_NATIVE_FNSPLIT_BOUNDARY_A n={depth} static_nodes={} edges={} \
                 planned_helpers={} persistent_store_nodes={} evidence_records={} \
                 fixed_k={} observed_max_helpers_per_source={} key_bytes={} \
                 key_schemas={} frame_schemas={} store_schemas={} \
                 static_node_id_bytes={} persistent_node_id_bytes={} \
                 max_logical_chain_depth={} env_depth={} continuation_depth={} \
                 path_depth={} cleanup_depth={} affine_depth={} source_return_depth={} \
                 source_return_resume_nodes={} source_return_owned_resume_edges={} \
                 terminal_outgoing_edges={} recursive_lowering_frames={} \
                 stack_bytes=8388608 \
                 node_payload_width=\"DEFERRED — NEEDS B2V/B2F\" \
                 frame_schema_width=\"DEFERRED — NEEDS B2V/B2F\" \
                 store_node_schema_width=\"DEFERRED — NEEDS B2V/B2F\"",
                row.static_nodes,
                row.edges,
                row.planned_helpers,
                row.persistent_store_nodes,
                row.out_of_line_evidence_records,
                MAX_HELPERS_PER_STATIC_SOURCE,
                row.max_helpers_per_static_source,
                row.helper_key_bytes,
                row.helper_key_schemas,
                row.frame_schemas,
                row.store_node_schemas,
                row.static_node_id_bytes,
                row.persistent_node_id_bytes,
                row.max_logical_chain_depth,
                row.max_environment_depth,
                row.max_continuation_depth,
                row.max_path_depth,
                row.max_cleanup_depth,
                row.max_affine_depth,
                row.max_source_return_depth,
                row.source_return_resume_nodes,
                row.source_return_owned_resume_edges,
                row.terminal_outgoing_edges,
                row.recursive_lowering_frames,
            );
        }
        for (name, values) in [
            ("static_nodes", values(&rows, |r| r.static_nodes)),
            ("edges", values(&rows, |r| r.edges)),
            ("planned_helpers", values(&rows, |r| r.planned_helpers)),
            (
                "persistent_store_nodes",
                values(&rows, |r| r.persistent_store_nodes),
            ),
            (
                "evidence_records",
                values(&rows, |r| r.out_of_line_evidence_records),
            ),
            (
                "fixed_k",
                values(&rows, |_| MAX_HELPERS_PER_STATIC_SOURCE),
            ),
            (
                "observed_max_helpers_per_source",
                values(&rows, |r| r.max_helpers_per_static_source),
            ),
            (
                "source_return_resume_nodes",
                values(&rows, |r| r.source_return_resume_nodes),
            ),
            (
                "source_return_owned_resume_edges",
                values(&rows, |r| r.source_return_owned_resume_edges),
            ),
            (
                "terminal_outgoing_edges",
                values(&rows, |r| r.terminal_outgoing_edges),
            ),
            (
                "recursive_lowering_frames",
                values(&rows, |r| r.recursive_lowering_frames),
            ),
            ("helper_key_bytes", values(&rows, |r| r.helper_key_bytes)),
            (
                "static_node_id_bytes",
                values(&rows, |r| r.static_node_id_bytes),
            ),
            (
                "persistent_node_id_bytes",
                values(&rows, |r| r.persistent_node_id_bytes),
            ),
            (
                "helper_key_schemas",
                values(&rows, |r| r.helper_key_schemas),
            ),
            ("frame_schemas", values(&rows, |r| r.frame_schemas)),
            (
                "store_node_schemas",
                values(&rows, |r| r.store_node_schemas),
            ),
            (
                "max_logical_chain_depth",
                values(&rows, |r| r.max_logical_chain_depth as usize),
            ),
            (
                "environment_depth",
                values(&rows, |r| r.max_environment_depth as usize),
            ),
            (
                "continuation_depth",
                values(&rows, |r| r.max_continuation_depth as usize),
            ),
            ("path_depth", values(&rows, |r| r.max_path_depth as usize)),
            (
                "cleanup_depth",
                values(&rows, |r| r.max_cleanup_depth as usize),
            ),
            (
                "affine_depth",
                values(&rows, |r| r.max_affine_depth as usize),
            ),
            (
                "source_return_depth",
                values(&rows, |r| r.max_source_return_depth as usize),
            ),
        ] {
            let (first, second) = differences(&values);
            eprintln!(
                "RT_NATIVE_FNSPLIT_BOUNDARY_A_DIFF metric={name} first={first:?} second={second:?}"
            );
            assert!(
                second.iter().all(|difference| *difference == 0),
                "{name} is not affine across n=3..7"
            );
        }
        for (name, field) in [
            (
                "helper_key_bytes",
                (|r: &BoundaryACensus| r.helper_key_bytes) as fn(&BoundaryACensus) -> usize,
            ),
            ("static_node_id_bytes", |r: &BoundaryACensus| {
                r.static_node_id_bytes
            }),
            ("persistent_node_id_bytes", |r: &BoundaryACensus| {
                r.persistent_node_id_bytes
            }),
            ("helper_key_schemas", |r: &BoundaryACensus| {
                r.helper_key_schemas
            }),
            ("frame_schemas", |r: &BoundaryACensus| r.frame_schemas),
            ("store_node_schemas", |r: &BoundaryACensus| {
                r.store_node_schemas
            }),
        ] {
            let values = values(&rows, field);
            assert!(
                values.windows(2).all(|pair| pair[0] == pair[1]),
                "{name} is not constant across n=3..7"
            );
        }
        assert!(rows
            .iter()
            .all(|row| row.max_helpers_per_static_source <= MAX_HELPERS_PER_STATIC_SOURCE));
        assert!(rows
            .iter()
            .zip(3..=7)
            .all(|(row, depth)| {
                row.source_return_resume_nodes == depth
                    && row.source_return_owned_resume_edges == depth
                    && row.terminal_outgoing_edges == 0
                    && row.recursive_lowering_frames > depth
            }));
        assert!(rows.iter().all(|row| {
            row.planned_helpers == row.static_nodes + row.edges
                && row.out_of_line_evidence_records == row.edges
                && row.max_environment_depth <= row.persistent_store_nodes as u32
                && row.max_continuation_depth <= row.persistent_store_nodes as u32
                && row.max_path_depth <= row.persistent_store_nodes as u32
                && row.max_logical_chain_depth <= row.persistent_store_nodes as u32
        }));

        let measured_static_nodes = values(&rows, |row| row.static_nodes);
        let provisional_static_nodes = [87, 115, 143, 171, 199];
        let static_nodes_agree = measured_static_nodes
            .iter()
            .copied()
            .eq(provisional_static_nodes);
        let measured_k_is_eight = rows
            .iter()
            .all(|row| row.max_helpers_per_static_source == 8);
        let measured_key_bytes_are_twelve = rows.iter().all(|row| row.helper_key_bytes == 12);
        eprintln!(
            "RT_NATIVE_FNSPLIT_BOUNDARY_A_PROVISIONAL relation_static_nodes={} \
             relation_observed_k={} relation_key_width={} \
             provisional_frame_store_widths=32/16 \
             current_frame_store_widths=\"DEFERRED — NEEDS B2V/B2F\" \
             stack_bytes=8388608 verdict=agreement_is_a_finding_not_confirmation",
            if static_nodes_agree {
                "agrees_with_87/115/143/171/199"
            } else {
                "differs_from_87/115/143/171/199"
            },
            if measured_k_is_eight {
                "agrees_with_8"
            } else {
                "differs_from_8"
            },
            if measured_key_bytes_are_twelve {
                "agrees_with_12"
            } else {
                "differs_from_12"
            },
        );
        eprintln!(
            "RT_NATIVE_FNSPLIT_BOUNDARY_A_EXPONENT_VERDICT \
             five_points_do_not_prove_an_exponent=true \
             historic_n4_fits=370n,93n²,product_switching_on_at_n5 \
             discriminator=structural_invariants table=corroboration_only \
             stack_bytes=8388608"
        );

        const AC_CONTROLS: [(&str, &str); 8] = [
            (
                "AC-A1",
                "prlimit worker plus forced failure and missing-result positive controls",
            ),
            (
                "AC-A2",
                "one emitted row per n with every due D2 field and three spelled deferrals",
            ),
            (
                "AC-A3",
                "first and second finite differences emitted for every due numeric row",
            ),
            (
                "AC-A4",
                "closed Copy helper-key patterns, constant ID/key/schema checks, affine stores/depth",
            ),
            (
                "AC-A5",
                "explicit five-points-do-not-prove-exponent verdict",
            ),
            (
                "AC-A6",
                "test-only guard measures maximum simultaneous production plan_expr calls",
            ),
            (
                "AC-A7",
                "computed provisional relation with agreement-not-confirmation verdict",
            ),
            (
                "AC-A8",
                "exact eight-row AC control inventory asserted below",
            ),
        ];
        assert_eq!(
            AC_CONTROLS.map(|(criterion, _)| criterion),
            [
                "AC-A1", "AC-A2", "AC-A3", "AC-A4", "AC-A5", "AC-A6", "AC-A7", "AC-A8"
            ]
        );
        for (criterion, control) in AC_CONTROLS {
            let control = if control.is_empty() {
                "NO CONTROL — open residual"
            } else {
                control
            };
            eprintln!(
                "RT_NATIVE_FNSPLIT_BOUNDARY_A_CONTROL criterion={criterion} control={control}"
            );
        }
        eprintln!(
            "RT_NATIVE_FNSPLIT_BOUNDARY_A_RESULT \
             status=measured_complete rows=5 stack_bytes=8388608"
        );
            })
            .unwrap_or_else(|error| {
                panic!(
                    "RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: \
                     exact 8 MiB planner worker could not start: {error}"
                )
            });
        if planner_worker.join().is_err() {
            panic!(
                "RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine: \
                 stack_bytes=8388608 exact 8 MiB planner worker panicked"
            );
        }
    }

    #[test]
    fn planner_invariant_failures_have_compiler_bug_attribution() {
        // Promise class: durable invariant. These distinct planner
        // self-consistency failures are compiler bugs. The former fixed-K capacity arm is
        // not input-reachable because fixed K is a structural planner invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let mut missing_helper = plan.clone();
        missing_helper.planned_helpers.pop();
        let invariant = missing_helper.validate().unwrap_err();
        assert!(matches!(
            &invariant,
            CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(detail))
                if detail == "planned helper inventory is not exact for the closed graph"
        ));
        assert_eq!(
            invariant.to_string(),
            "Cranelift backend failure: native static transition planner invariant failed; \
             please report this compiler bug: planned helper inventory is not exact for the \
             closed graph"
        );
        assert!(!invariant.to_string().contains("unsupported"));

        let mut helpers_per_source = BTreeMap::<StaticSourceId, usize>::new();
        for helper in &plan.planned_helpers {
            let owner = match *helper {
                PlannedHelperKey::Node(_, id) => plan.nodes[id.0 as usize].owner,
                PlannedHelperKey::Edge(_, id) => {
                    let edge = plan.edges[id.0 as usize];
                    plan.nodes[edge.from.0 as usize].owner
                }
            };
            *helpers_per_source.entry(owner).or_default() += 1;
        }
        let owner = helpers_per_source
            .iter()
            .find_map(|(owner, count)| (*count == MAX_HELPERS_PER_STATIC_SOURCE).then_some(*owner))
            .expect("nested bracket plan has a source at the fixed K capacity");
        let frame = plan
            .nodes
            .iter()
            .find(|node| node.owner == owner)
            .expect("capacity owner has a node")
            .frame;
        let mut over_capacity = plan.clone();
        let id = StaticNodeId(over_capacity.nodes.len() as u32);
        over_capacity.nodes.push(StaticNode {
            id,
            transition: TransitionKind::Evaluate,
            owner,
            frame,
        });
        over_capacity
            .planned_helpers
            .push(PlannedHelperKey::node(TransitionKind::Evaluate, id));

        let fixed_k_invariant = over_capacity.validate().unwrap_err();
        assert!(matches!(
            &fixed_k_invariant,
            CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(detail))
                if detail == "fixed K helpers per static source was exceeded"
        ));
        assert_eq!(
            fixed_k_invariant.to_string(),
            "Cranelift backend failure: native static transition planner invariant failed; \
             please report this compiler bug: fixed K helpers per static source was exceeded"
        );
        assert!(!fixed_k_invariant.to_string().contains("unsupported"));
    }

    #[test]
    fn distinct_activations_share_one_helper_key_and_source_return_is_not_terminal() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let wrapper = plan
            .nodes
            .iter()
            .find(|node| node.transition == TransitionKind::ProducerWrapper)
            .unwrap();
        let other_activation = plan
            .nodes
            .iter()
            .map(|node| node.frame)
            .find(|frame| {
                frame.environment != wrapper.frame.environment && frame.path != wrapper.frame.path
            })
            .expect("nested bracket plan has a distinct valid activation");
        assert_ne!(wrapper.frame, other_activation);
        let helpers_before = plan.census().planned_helpers;
        let first = plan
            .helper_key_for_activation(wrapper.id, wrapper.frame)
            .unwrap();
        let second = plan
            .helper_key_for_activation(wrapper.id, other_activation)
            .unwrap();
        assert_eq!(
            BTreeSet::from([first, second]).len(),
            1,
            "distinct dynamic activations multiplied one static helper"
        );
        assert_eq!(
            plan.census().planned_helpers,
            helpers_before,
            "flowing another activation through a static node grew planned code"
        );
        assert!(plan
            .edges
            .iter()
            .filter(|edge| edge.kind == EdgeKind::SourceReturnOwnedResume)
            .all(|edge| {
                plan.nodes[edge.to.0 as usize].transition == TransitionKind::SourceReturnResume
                    && edge.to != plan.terminal_id()
            }));
    }

    #[test]
    fn source_return_ownership_guards_fail_closed_on_exact_cross_wires() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let wrappers = plan
            .nodes
            .iter()
            .filter(|node| node.transition == TransitionKind::ProducerWrapper)
            .collect::<Vec<_>>();
        let first_wrapper = wrappers[0];
        let second_wrapper = wrappers[1];
        let node_for = |owner, transition| {
            plan.nodes
                .iter()
                .find(|node| node.owner == owner && node.transition == transition)
                .unwrap()
                .id
        };
        let first_resume = node_for(first_wrapper.owner, TransitionKind::SourceReturnResume);
        let second_resume = node_for(second_wrapper.owner, TransitionKind::SourceReturnResume);
        let first_tail = node_for(first_wrapper.owner, TransitionKind::ProducerTail);
        let second_tail = node_for(second_wrapper.owner, TransitionKind::ProducerTail);

        let source_return_edge = *plan
            .edges
            .iter()
            .find(|edge| edge.to == first_resume && edge.kind == EdgeKind::SourceReturnOwnedResume)
            .unwrap();
        let mut crossed_resume = plan.clone();
        rewrite_edge(
            &mut crossed_resume,
            source_return_edge.id,
            source_return_edge.from,
            second_resume,
            source_return_edge.kind,
        );
        assert_eq!(
            crossed_resume.validate().unwrap_err(),
            planner_error("source-return-owned edge targets a resume from another descriptor")
        );

        let resume_edge = *plan
            .edges
            .iter()
            .find(|edge| edge.from == first_resume && edge.kind == EdgeKind::InvokeProducerWrapper)
            .unwrap();
        let mut crossed_wrapper = plan.clone();
        rewrite_edge(
            &mut crossed_wrapper,
            resume_edge.id,
            resume_edge.from,
            second_wrapper.id,
            resume_edge.kind,
        );
        assert_eq!(
            crossed_wrapper.validate().unwrap_err(),
            planner_error("source-return resume must have only its exact wrapper invocation")
        );

        let wrapper_edge = *plan
            .edges
            .iter()
            .find(|edge| edge.from == first_wrapper.id && edge.kind == EdgeKind::InvokeProducerTail)
            .unwrap();
        let mut crossed_tail = plan.clone();
        rewrite_edge(
            &mut crossed_tail,
            wrapper_edge.id,
            wrapper_edge.from,
            second_tail,
            wrapper_edge.kind,
        );
        assert_eq!(
            crossed_tail.validate().unwrap_err(),
            planner_error("producer wrapper must have only its exact tail invocation")
        );

        let descriptor = first_wrapper.frame.source_return.0 as usize - 1;
        let mut crossed_descriptor_wrapper = plan.clone();
        crossed_descriptor_wrapper.stores[descriptor].local = second_wrapper.id.0;
        assert_eq!(
            crossed_descriptor_wrapper.validate().unwrap_err(),
            planner_error("source-return descriptor does not name its exact W and T")
        );

        let mut crossed_descriptor_tail = plan.clone();
        crossed_descriptor_tail.stores[descriptor].aux = second_tail.0;
        assert_eq!(
            crossed_descriptor_tail.validate().unwrap_err(),
            planner_error("source-return descriptor does not name its exact W and T")
        );

        let tail_edge = *plan
            .edges
            .iter()
            .find(|edge| edge.from == first_tail && edge.kind == EdgeKind::CompleteProducerTail)
            .unwrap();
        let mut duplicate_wrapper = plan.clone();
        rewrite_edge(
            &mut duplicate_wrapper,
            tail_edge.id,
            first_resume,
            first_wrapper.id,
            EdgeKind::InvokeProducerWrapper,
        );
        assert_eq!(
            duplicate_wrapper.validate().unwrap_err(),
            planner_error("source-return resume must have only its exact wrapper invocation")
        );

        let mut terminal_resume = plan.clone();
        rewrite_edge(
            &mut terminal_resume,
            source_return_edge.id,
            source_return_edge.from,
            plan.terminal_id(),
            source_return_edge.kind,
        );
        assert_eq!(
            terminal_resume.validate().unwrap_err(),
            planner_error("source-return-owned edge targets a resume from another descriptor")
        );

        let mut wrapper_entry = plan.clone();
        wrapper_entry.entries[0] = first_wrapper.id;
        assert_eq!(
            wrapper_entry.validate().unwrap_err(),
            planner_error("producer wrapper cannot be a pre-source graph entry")
        );
    }

    #[test]
    fn quartet_edge_sets_and_completed_successor_reject_alternate_calls() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let wrapper = plan
            .nodes
            .iter()
            .find(|node| node.transition == TransitionKind::ProducerWrapper)
            .unwrap();
        let node_for = |transition| {
            plan.nodes
                .iter()
                .find(|node| node.owner == wrapper.owner && node.transition == transition)
                .unwrap()
                .id
        };
        let resume = node_for(TransitionKind::SourceReturnResume);
        let tail = node_for(TransitionKind::ProducerTail);
        let completed = node_for(TransitionKind::CompletedTail);
        let ordinary = plan
            .nodes
            .iter()
            .find(|node| node.owner != wrapper.owner && node.transition == TransitionKind::Evaluate)
            .unwrap()
            .id;

        let mut alternate_tail_incoming = plan.clone();
        append_edge(
            &mut alternate_tail_incoming,
            ordinary,
            tail,
            EdgeKind::Continue,
        );
        assert_eq!(
            alternate_tail_incoming.validate().unwrap_err(),
            planner_error("producer tail must have only its exact wrapper invocation")
        );

        let mut alternate_completed_incoming = plan.clone();
        append_edge(
            &mut alternate_completed_incoming,
            ordinary,
            completed,
            EdgeKind::Continue,
        );
        assert_eq!(
            alternate_completed_incoming.validate().unwrap_err(),
            planner_error("CompletedTail must have only its exact producer-tail completion")
        );

        for (from, expected) in [
            (
                resume,
                "source-return resume must have only its exact wrapper invocation",
            ),
            (
                wrapper.id,
                "producer wrapper must have only its exact tail invocation",
            ),
            (
                tail,
                "producer tail must have only its exact completion edge",
            ),
        ] {
            let mut alternate_outgoing = plan.clone();
            append_edge(
                &mut alternate_outgoing,
                from,
                plan.terminal_id(),
                EdgeKind::Continue,
            );
            assert_eq!(
                alternate_outgoing.validate().unwrap_err(),
                planner_error(expected)
            );
        }

        let completed_edge = *plan
            .edges
            .iter()
            .find(|edge| edge.from == completed)
            .unwrap();
        let mut wrong_successor = plan.clone();
        rewrite_edge(
            &mut wrong_successor,
            completed_edge.id,
            completed,
            plan.trap_terminal_id(),
            completed_edge.kind,
        );
        assert_eq!(
            wrong_successor.validate().unwrap_err(),
            planner_error("CompletedTail must have only its activation-named successor")
        );

        let mut wrong_resume_kind = plan.clone();
        rewrite_edge(
            &mut wrong_resume_kind,
            completed_edge.id,
            completed,
            completed_edge.to,
            EdgeKind::Trap,
        );
        assert_eq!(
            wrong_resume_kind.validate().unwrap_err(),
            planner_error("CompletedTail successor does not use its normal-resume edge kind")
        );
    }

    #[test]
    fn entry_and_reachability_closure_rejects_balancing_invalid_root() {
        // Promise class: durable invariant.
        let expr = unit();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let mut outside = plan.clone();
        outside.entries[0] = StaticNodeId(u32::MAX);
        assert_eq!(
            outside.validate().unwrap_err(),
            planner_error("graph entry is outside the closed node set")
        );

        let mut duplicate = plan.clone();
        duplicate.entries.push(duplicate.entries[0]);
        assert_eq!(
            duplicate.validate().unwrap_err(),
            planner_error("closed graph contains a duplicate entry")
        );
    }

    #[test]
    fn closed_identity_terminal_and_store_guards_reject_exact_mutations() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let mut wrong_node_identity = plan.clone();
        let evaluate = wrong_node_identity
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.transition == TransitionKind::Evaluate)
            .map(|(index, node)| (index, node.id))
            .take(2)
            .collect::<Vec<_>>();
        wrong_node_identity.nodes[evaluate[0].0].id = evaluate[1].1;
        assert_eq!(
            wrong_node_identity.validate().unwrap_err(),
            planner_error("static node identity does not match its closed position")
        );

        let mut terminal_outgoing = plan.clone();
        let resume = terminal_outgoing
            .nodes
            .iter()
            .find(|node| node.transition == TransitionKind::SourceReturnResume)
            .unwrap()
            .id;
        append_edge(
            &mut terminal_outgoing,
            plan.terminal_id(),
            resume,
            EdgeKind::Continue,
        );
        assert_eq!(
            terminal_outgoing.validate().unwrap_err(),
            planner_error("Terminal and TrapTerminal must have no outgoing edges")
        );

        let mut unclosed_store = plan.clone();
        unclosed_store.stores[0].child = PersistentNodeId(unclosed_store.stores.len() as u32 + 1);
        assert_eq!(
            unclosed_store.validate().unwrap_err(),
            planner_error("persistent store child is not an earlier closed node")
        );

        let mut wrong_depth = plan.clone();
        wrong_depth.store_depths[0] += 1;
        assert_eq!(
            wrong_depth.validate().unwrap_err(),
            planner_error("persistent store depth does not match its child chain")
        );

        let mut duplicate_store = plan.clone();
        duplicate_store.stores[1] = duplicate_store.stores[0];
        assert_eq!(
            duplicate_store.validate().unwrap_err(),
            planner_error("persistent store contains a duplicate node")
        );

        let mut missing_helper = plan.clone();
        missing_helper.planned_helpers.pop();
        assert_eq!(
            missing_helper.validate().unwrap_err(),
            planner_error("planned helper inventory is not exact for the closed graph")
        );
    }

    impl StaticTransitionPlan<'_> {
        pub(in crate::cranelift_backend::planning::static_transition) fn terminal_id(&self) -> StaticNodeId {
            self.nodes
                .iter()
                .find(|node| node.transition == TransitionKind::Terminal)
                .expect("closed graph has Terminal")
                .id
        }

        fn trap_terminal_id(&self) -> StaticNodeId {
            self.nodes
                .iter()
                .find(|node| node.transition == TransitionKind::TrapTerminal)
                .expect("closed graph has TrapTerminal")
                .id
        }
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-5` — the deferred synthetic
    /// exact-witness control, CARRIED here, RUNNABLE later.**
    ///
    /// **This control has never executed and this candidate does not claim it
    /// has.** It is carried in the tree, per `AC-5`, so the obligation is an
    /// artifact rather than a sentence in a handoff that evaporates when the
    /// terminal closes. The **committed runnable form is owned by the first
    /// candidate to run once nested-inductive admission is on `main`** — keyed
    /// to that capability, not to the closure of any node.
    ///
    /// **Release condition: nested-inductive admission is on `main`.**
    ///
    /// Stated as the capability rather than as `KERNEL-NESTED-IND` merged,
    /// because those two came apart once already: a merge event is not the
    /// capability it was expected to deliver, and gating on one invites the next
    /// reader to un-ignore this control and take a red they cannot fix.
    ///
    /// **The release condition is tracked at `KERNEL-NESTED-IND` `AC-K12`.**
    /// Consult it there.
    ///
    /// ⛔ **This comment deliberately gives you NO way to decide the condition
    /// locally.** An earlier wording did, and that is the defect this replaces:
    /// a decision procedure written into a comment is correct at the instant it
    /// is written and silently wrong afterwards, at which point it tells the
    /// reader the capability has arrived. Any snapshot, path list or commit put
    /// here would decay the same way — so if you find yourself adding one,
    /// that is the bug, not the omission.
    ///
    /// Until that capability lands the
    /// `LiftRose` witness exists only on the attribution node's disposable
    /// synthetic venue — an unreferenced composition of this Runtime tree with
    /// Kernel's held `dd3cd050` and its projection snapshot — which is a
    /// pre-merge integration gate, not a suite fixture.
    ///
    /// **What the runnable form must assert**, so the next seat inherits the
    /// obligation rather than re-deriving it:
    ///
    /// 1. `SOI(26)` is a reachable `ComputationalMatch` owned by
    ///    `PredeclaredFunctionId(2)`;
    /// 2. that owner's required join set is exactly `{26, 33, 39, 53}`;
    /// 3. every member is **entered and closed through the real traversal** —
    ///    `consumed ∪ dispositioned` equals the required set, rather than the
    ///    empty sets the attribution measured;
    /// 4. sibling owners still close normally, so the fixture discriminates this
    ///    owner rather than reporting a whole-plan change.
    ///
    /// **Fail-closed by construction.** The body panics rather than returning,
    /// so removing `#[ignore]` without supplying the witness is a RED. An
    /// ignored test whose body would pass vacuously is the shape that lets a
    /// deferred obligation read as a discharged one.
    #[test]
    #[ignore = "carried, not runnable: needs nested-inductive admission on main \
                for the LiftRose witness -- the capability, not the \
                KERNEL-NESTED-IND merge event, which fired at afb38934 without \
                delivering it. The first candidate to run once that capability \
                is on main owns the runnable form."]
    pub(in crate::cranelift_backend::planning::static_transition) fn liftrose_synthetic_witness_closes_owner_two_required_joins() {
        panic!(
            "RT-BODY-OCCURRENCE-PROVENANCE AC-5 is CARRIED, not discharged. \
             Supply the LiftRose witness (requires nested-inductive admission \
             on main -- the capability, not merely KERNEL-NESTED-IND merged) \
             and assert: SOI(26) is a reachable ComputationalMatch owned by \
             PredeclaredFunctionId(2); its required join set is exactly \
             {{26, 33, 39, 53}}; consumed union dispositioned equals that set \
             through the real traversal; and sibling owners still close \
             normally. Do not delete this control to make a suite green."
        );
    }


    /// Hard-stop #18 row 2 — declaration-call validation consumes the canonical
    /// node-indexed source view, never the planner's walk order.
    #[test]
    fn declaration_call_validation_positions_out_of_order_sources_once() {
        // Promise class: durable invariant plus a durable mutation proof.
        //
        // MEASURED: the exact DeclarationCall edge source names a
        // `DeclarationRef` in the canonical positioned view while the raw
        // walk-order slot at the same ordinal names a different source.
        // CLAIMED: validation indexes source semantics by StaticOriginId.
        // THE GAP: a fixture whose two views happen to agree cannot distinguish
        // positioned indexing from the rejected raw indexing, so the mismatch
        // assertions below are load-bearing.
        let symbol = "decl:fixture::b2o".to_string();
        let declaration =
            b2o_transparent_declaration(RuntimeExpr::Value(RuntimeValue::Int((73).into())));
        let declarations = BTreeMap::from([(symbol.as_str(), &declaration)]);
        let expr = RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Row2::Node".to_string(),
                args: vec![unit()],
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
                message: "row-2 fixture is total".to_string(),
            },
        };
        let plan = plan_static_transition_graph(&expr, &declarations)
            .expect("the out-of-order declaration call validates");
        let (edge_index, edge) = plan
            .edges
            .iter()
            .copied()
            .enumerate()
            .find(|(_, edge)| edge.kind == EdgeKind::DeclarationCall)
            .expect("the fixture has one declaration call edge");
        let node_indexed_sources =
            super::super::semantic_ir::positioned_sources(&plan.nodes, &plan.semantic_sources)
                .expect("the source population positions");
        assert_ne!(
            plan.semantic_sources[edge.from.0 as usize].source,
            SemanticSourceKind::Expression(RuntimeExprShape::DeclarationRef),
            "the fixture's raw walk-order slot agrees with node order, so it \
             cannot discriminate the rejected indexing"
        );
        assert_eq!(
            node_indexed_sources[edge.from.0 as usize].source,
            SemanticSourceKind::Expression(RuntimeExprShape::DeclarationRef),
            "the canonical positioned source does not name the call occurrence"
        );

        let call = plan
            .emittable_call_edges()
            .expect("the validated call edge projects")
            .into_iter()
            .find(|call| call.kind() == EmittableCallKind::Declaration)
            .expect("the declaration call remains separately typed");
        assert_eq!(call.call_site_origin(), origin_of(edge.from));
        assert_eq!(
            call.callee_origin(),
            plan.declaration_occurrence_origin(symbol.as_str())
                .expect("the transparent declaration owns one exact origin")
        );

        // Ordinary in-order control: positioning is not a special case for
        // ComputationalMatch and leaves an already positional call unchanged.
        let ordinary = RuntimeExpr::DeclarationRef {
            symbol: symbol.clone(),
        };
        let ordinary_plan = plan_static_transition_graph(&ordinary, &declarations)
            .expect("an ordinary declaration call remains valid");
        let ordinary_edge = ordinary_plan
            .edges
            .iter()
            .find(|edge| edge.kind == EdgeKind::DeclarationCall)
            .expect("the ordinary fixture has a declaration call edge");
        assert_eq!(
            ordinary_plan.semantic_sources[ordinary_edge.from.0 as usize].source,
            SemanticSourceKind::Expression(RuntimeExprShape::DeclarationRef)
        );

        // Redirect only the call-site source to a non-DeclarationRef occurrence
        // under the same owner. The source-shape invariant must still be the
        // exact detector; positioning repairs indexing, not validation.
        let caller_owner = plan.semantic.descriptors[edge.from.0 as usize].owner;
        let non_declaration_source = plan
            .nodes
            .iter()
            .map(|node| node.id)
            .find(|node| {
                *node != edge.from
                    && plan.semantic.descriptors[node.0 as usize].owner == caller_owner
                    && node_indexed_sources[node.0 as usize].source
                        != SemanticSourceKind::Expression(RuntimeExprShape::DeclarationRef)
            })
            .expect("the caller owns a non-DeclarationRef occurrence");
        let mut redirected_edges = plan.edges.clone();
        redirected_edges[edge_index].from = non_declaration_source;
        assert_eq!(
            plan.semantic
                .validate(
                    &plan.nodes,
                    &redirected_edges,
                    &plan.entries,
                    &|entry| plan.planned_entry_body(entry),
                    plan.root_entry,
                    &plan.semantic_sources,
                    &plan.semantic_material,
                )
                .unwrap_err(),
            planner_error("declaration call edge source is not a DeclarationRef occurrence")
        );
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn b2o_units(expr: &RuntimeExpr, declarations: &BTreeMap<&str, &RuntimeDeclaration>) -> usize {
        plan_static_transition_graph(expr, declarations)
            .expect("plannable")
            .semantic
            .functions
            .len()
    }

    /// `AC-4` — the unit set is exactly `plan.entries` ∪ `StaticBody` targets,
    /// with **three positive** controls.
    ///
    /// ⚠ A negative check ("no extra units") passes for any reason, including
    /// because nothing reached the checker. Each control here asserts a **delta**
    /// against a base fixture differing in exactly one way, so the count is
    /// attributable and no absolute number is frozen.
    #[test]
    fn b2o_ac4_each_seed_class_adds_exactly_one_function_unit() {
        // Promise class: durable invariant — a relation between fixtures.
        let base = unit();
        let none = BTreeMap::new();
        let base_units = b2o_units(&base, &none);
        // Non-vacuity: the base must already carry the root unit, or every "+1"
        // below could be measuring the root's arrival instead of the new seed.
        assert_eq!(base_units, 1, "the base fixture is the root unit alone");

        // Control 1 — one retained closure adds exactly one unit.
        assert_eq!(
            b2o_units(&b2o_retained_closure(unit()), &none),
            base_units + 1,
            "AC-4: one retained closure must add exactly one function unit"
        );

        // Control 2 — one transparent declaration adds exactly one unit.
        //
        // ⚠ This is the row an obvious test set omits. A closure/non-closure pair
        // does not exercise the **second top-level seed class** at all, so a
        // declaration-entry bug would pass every other control here. Two seed
        // classes require two positive controls.
        let declaration = b2o_transparent_declaration(unit());
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        assert_eq!(
            b2o_units(&base, &declarations),
            base_units + 1,
            "AC-4: one transparent declaration must add exactly one function unit"
        );

        // Control 3 — a non-closure expression inside an existing unit adds zero.
        let interior = RuntimeExpr::Let {
            value: Box::new(unit()),
            body: Box::new(unit()),
        };
        assert_eq!(
            b2o_units(&interior, &none),
            base_units,
            "AC-4: an expression inside an existing unit must add no unit"
        );
        // ...and it genuinely added planned nodes, so control 3 is not vacuous.
        let interior_nodes = plan_static_transition_graph(&interior, &none)
            .expect("plannable")
            .nodes
            .len();
        let base_nodes = plan_static_transition_graph(&base, &none)
            .expect("plannable")
            .nodes
            .len();
        assert!(
            interior_nodes > base_nodes,
            "control 3 proves nothing unless the interior expression added nodes"
        );
    }

    /// `AC-2` — totality and exclusivity are **pinned**, not merely structural.
    ///
    /// ⭐ "It is total by construction" is exactly the claim hard-stop #5 was
    /// defeated on: the carrier existed and the property did not follow. A
    /// structural guarantee still needs a check that *fires* if the construction
    /// changes.
    #[test]
    fn b2o_ac2_every_non_sentinel_node_has_exactly_one_in_range_function_owner() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plannable");

        let mut owned = 0usize;
        let mut terminals = 0usize;
        let mut trap_terminals = 0usize;
        for descriptor in &plan.semantic.descriptors {
            match descriptor.owner {
                SemanticOwner::Function(id) => {
                    assert!(
                        (id.0 as usize) < plan.semantic.functions.len(),
                        "an owner names a function unit outside the closed table"
                    );
                    owned += 1;
                }
                SemanticOwner::Terminal => terminals += 1,
                SemanticOwner::TrapTerminal => trap_terminals += 1,
            }
        }
        // The shared-exit population is EXACTLY the two sentinels — not "at
        // least", and not "whichever nodes ended up unowned".
        assert_eq!(
            (terminals, trap_terminals),
            (1, 1),
            "AC-2: the shared-exit population must be exactly one Terminal and \
             one TrapTerminal"
        );
        assert_eq!(
            owned,
            plan.nodes.len() - 2,
            "AC-2: every non-sentinel node must resolve to one Function owner"
        );
        // Non-vacuity: a single-unit fixture satisfies every line above while
        // proving nothing about exclusivity.
        assert!(
            plan.semantic.functions.len() >= 2,
            "this fixture has one unit, so exclusivity is untested here"
        );
    }

    /// `AC-3` — composition, **bidirectionally**, per `SemanticOpcode` variant.
    ///
    /// ⛔ Hard-stop #8 was predictable from the question its frame asked: the
    /// census answered `TOTAL` and was *true*, but the mechanism needed **closure
    /// under parent→child reachability**, a different property.
    /// `ComputationalMatch` files its occurrence on a different node from the
    /// entry its parent points at, so totality held while composition failed.
    /// This composes the child accessor with the owner map instead of measuring
    /// totality a second time.
    #[test]
    fn b2o_ac3_ownership_composes_down_and_up_for_every_opcode_variant() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plannable");
        let owner_of = |origin: StaticOriginId| plan.semantic.descriptors[origin.0 as usize].owner;

        // The retained-body boundaries, read off the graph rather than assumed.
        let static_body = plan
            .edges
            .iter()
            .filter(|edge| edge.kind == EdgeKind::StaticBody)
            .map(|edge| (edge.from, edge.to))
            .collect::<BTreeMap<_, _>>();

        let mut variants = BTreeSet::new();
        let mut boundary_children = 0usize;
        let mut interior_children = 0usize;
        for (position, record) in plan.semantic.records.iter().enumerate() {
            let parent = StaticOriginId(position as u32);
            variants.insert(record.opcode);
            let parent_owner = owner_of(parent);
            let crossing = static_body.get(&StaticNodeId(parent.0)).copied();
            for index in 0..record.child_origins.len as usize {
                let child = plan
                    .child_static_origin(parent, index)
                    .expect("a positional child origin");
                let child_owner = owner_of(child);
                match crossing {
                    // Child 0 of a closure occurrence IS its retained body. ⚠ Its
                    // *occurrence* may sit on a different node from the entry the
                    // StaticBody edge targets — that is the #8 shape — so this
                    // asserts the OWNER agrees, not that the nodes are equal.
                    Some(callee_seed) if index == 0 => {
                        let callee = match owner_of(StaticOriginId(callee_seed.0)) {
                            SemanticOwner::Function(id) => id,
                            other => panic!("a callee seed cannot be a shared exit: {other:?}"),
                        };
                        assert_eq!(
                            child_owner,
                            SemanticOwner::Function(callee),
                            "the retained body child must be owned by the callee unit"
                        );
                        assert_ne!(
                            child_owner, parent_owner,
                            "the retained body child must not stay in the caller's unit"
                        );
                        boundary_children += 1;
                    }
                    // Every other child — a capture, or any child of a
                    // non-closure — stays inside the parent's own unit.
                    _ => {
                        assert_eq!(
                            child_owner, parent_owner,
                            "descending to a non-boundary child left the parent's unit"
                        );
                        interior_children += 1;
                    }
                }
            }
        }

        // ⭐ The "up" half. The boundary crossed on descent is represented by the
        // **callee seed**, and the body's return node stays inside the
        // **callee's** owner rather than being handed back to the caller. This is
        // AC-5 control 8's property stated positively.
        //
        // ⛔ `B2O` invents no static edge back to the caller — `B2R` carries the
        // dynamic return continuation — so "up" is checked as *the return node is
        // callee-owned and exits only through a shared exit*, never as a
        // cross-owner edge this node manufactured.
        let mut returns = 0usize;
        for node in &plan.nodes {
            if node.transition != TransitionKind::ClosureBody {
                continue;
            }
            returns += 1;
            let SemanticOwner::Function(unit) = plan.semantic.descriptors[node.id.0 as usize].owner
            else {
                panic!("a ClosureBody return successor must be owned by a function unit");
            };
            let seed = plan.semantic.functions[unit.0 as usize].planned_node;
            assert!(
                static_body.values().any(|target| *target == seed),
                "the ClosureBody return successor is owned by a unit that is not a callee"
            );
            let exits = plan
                .edges
                .iter()
                .filter(|edge| edge.from == node.id)
                .collect::<Vec<_>>();
            assert!(
                !exits.is_empty(),
                "a return successor with no exit proves nothing"
            );
            for edge in exits {
                assert!(
                    matches!(
                        plan.semantic.descriptors[edge.to.0 as usize].owner,
                        SemanticOwner::Terminal | SemanticOwner::TrapTerminal
                    ),
                    "a ClosureBody return successor must exit only through a shared exit"
                );
            }
        }

        // ⛔ No silent caps. Say what was exercised and fail if a class never
        // appeared — an assertion nothing reached is green for the wrong reason.
        assert_eq!(
            variants.len(),
            6,
            "AC-3 requires EVERY SemanticOpcode variant, not a sampled few; this \
             fixture exercised {variants:?}"
        );
        assert!(boundary_children > 0, "no boundary child was exercised");
        assert!(interior_children > 0, "no interior child was exercised");
        assert!(returns > 0, "no ClosureBody return successor was exercised");
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn b2o_err(
        plane: &SemanticPlane,
        nodes: &[StaticNode],
        edges: &[StaticEdge],
        entries: &[StaticNodeId],
        plan: &StaticTransitionPlan,
    ) -> CraneliftBackendError {
        plane
            .validate(
                nodes,
                edges,
                entries,
                &|entry| plan.planned_entry_body(entry),
                plan.root_entry,
                &plan.semantic_sources,
                &plan.semantic_material,
            )
            .expect_err("the control must redden")
    }

    /// `AC-5` — every ownership law is enforced, each with its own **independent**
    /// redden control, constructed and confirmed to error **before emission**.
    ///
    /// ⛔ A pin that enumerates spellings is not a proof of the property. Each
    /// control below mutates the *graph, the seeds, or the recorded owner* — never
    /// a string — and every mutation keeps the code compiling.
    ///
    /// ⚠ **Honest residual, and it is a finding about the AC rather than a gap in
    /// the mechanism.** Rows 5 and 6 of `AC-5` land on the **same** detector, and
    /// they must: because ownership is *derived* by traversal from seeds over
    /// non-`StaticBody` edges and then compared against the record, any ordinary
    /// cross-owner edge necessarily makes some node reachable from two seeds. So
    /// "a non-`StaticBody` cross-owner edge" **is** an overlap, and no data
    /// mutation can produce one without producing the other. Both are constructed
    /// below and both redden; what cannot be claimed is that they exercise two
    /// independent checks. The `D3` edge laws are still checked, because they
    /// constrain the **algorithm** — but as **defense in depth behind overlap,
    /// not as the primary detector.** Measured: a traversal edited to cross
    /// `StaticBody` reddens at **overlap** (mutation M1), because the callee's
    /// seed is claimed by the caller; the "crosses to a *distinct* unit" law is
    /// the sole detector only once overlap is **also** disabled (mutation M2).
    /// The genuinely independent edge-law control is the sentinel one (5b).
    ///
    /// ⭐ Note the shape of my own error here, since it is the reusable part: I
    /// identified this exact detector-collapse for the *data*-mutation route in
    /// the paragraph above, then asserted the opposite for the *code*-mutation
    /// route one sentence later. Having found one collapse, sweep every route to
    /// the property before writing prose about any of them.
    #[test]
    fn b2o_ac5_each_ownership_law_reddens_on_its_own() {
        // Promise class: durable mutation proof.
        let declaration = b2o_transparent_declaration(unit());
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        let expr = b2o_two_closure_fixture();
        let plan = plan_static_transition_graph(&expr, &declarations).expect("plannable");

        // Non-vacuity of the fixture itself, before any control runs.
        assert!(
            plan.entries.len() >= 2,
            "controls 1 and 2 need a root AND a declaration entry"
        );
        let static_body = plan
            .edges
            .iter()
            .filter(|edge| edge.kind == EdgeKind::StaticBody)
            .map(|edge| (edge.from, edge.to))
            .collect::<Vec<_>>();
        assert!(
            static_body.len() >= 2,
            "control 4 needs two static body edges to alias"
        );
        plan.semantic
            .validate(
                &plan.nodes,
                &plan.edges,
                &plan.entries,
                &|entry| plan.planned_entry_body(entry),
                plan.root_entry,
                &plan.semantic_sources,
                &plan.semantic_material,
            )
            .expect("the unmutated plane must validate, or every control below is vacuous");

        let unowned = planner_error("planned node has no function unit owner");
        let population = planner_error(
            "function unit population is not the scheduling entries and static body targets",
        );

        // 1 — a missing ROOT entry. The root's subgraph is then reachable from no
        //     seed at all.
        assert_eq!(
            b2o_err(
                &plan.semantic,
                &plan.nodes,
                &plan.edges,
                &plan.entries[1..],
                &plan
            ),
            unowned,
            "AC-5.1: dropping the root entry must redden"
        );

        // 2 — a missing TRANSPARENT DECLARATION entry. ⚠ Independent of control 1:
        //     a checker that only knew about the root would pass 1 and fail here.
        assert_eq!(
            b2o_err(
                &plan.semantic,
                &plan.nodes,
                &plan.edges,
                &plan.entries[..1],
                &plan
            ),
            unowned,
            "AC-5.2: dropping a transparent declaration entry must redden"
        );

        // 3 — a missing StaticBody TARGET: demote one StaticBody edge to an
        //     ordinary transfer, so its body stops being a seed.
        let mut demoted = plan.clone();
        let victim = demoted
            .edges
            .iter()
            .find(|edge| edge.kind == EdgeKind::StaticBody)
            .copied()
            .expect("a static body edge");
        rewrite_edge(
            &mut demoted,
            victim.id,
            victim.from,
            victim.to,
            EdgeKind::Continue,
        );
        assert_eq!(
            b2o_err(
                &plan.semantic,
                &plan.nodes,
                &demoted.edges,
                &plan.entries,
                &plan
            ),
            population,
            "AC-5.3: dropping a static body target must redden"
        );

        // 4 — a DUPLICATE StaticBody target: point the second boundary edge at
        //     the first one's body.
        let mut aliased = plan.clone();
        let second = aliased
            .edges
            .iter()
            .filter(|edge| edge.kind == EdgeKind::StaticBody)
            .nth(1)
            .copied()
            .expect("a second static body edge");
        rewrite_edge(
            &mut aliased,
            second.id,
            second.from,
            static_body[0].1,
            EdgeKind::StaticBody,
        );
        assert_eq!(
            b2o_err(
                &plan.semantic,
                &plan.nodes,
                &aliased.edges,
                &plan.entries,
                &plan
            ),
            planner_error("static body target has more than one incoming static body edge"),
            "AC-5.4: a duplicate static body target must redden, not be deduplicated"
        );

        // 5 — a non-StaticBody CROSS-OWNER edge: reach from the root's unit
        //     straight into a body-owned node. See the honest residual above —
        //     this reddens at the overlap detector, by construction.
        let mut crossed = plan.clone();
        let root_entry = plan.entries[0];
        append_edge(
            &mut crossed,
            root_entry,
            static_body[0].1,
            EdgeKind::Continue,
        );
        assert_eq!(
            b2o_err(
                &plan.semantic,
                &plan.nodes,
                &crossed.edges,
                &plan.entries,
                &plan
            ),
            planner_error("planned node is owned by more than one function unit"),
            "AC-5.5: a non-static-body cross-owner edge must redden"
        );

        // 5b — the genuinely independent EDGE-LAW control: an outgoing edge from
        //      a shared exit. A sentinel is never traversed from, so this creates
        //      no overlap and reaches the edge law itself.
        let mut exiting = plan.clone();
        let terminal = plan
            .nodes
            .iter()
            .find(|node| node.transition == TransitionKind::Terminal)
            .expect("the shared terminal")
            .id;
        append_edge(&mut exiting, terminal, root_entry, EdgeKind::Continue);
        assert_eq!(
            b2o_err(
                &plan.semantic,
                &plan.nodes,
                &exiting.edges,
                &plan.entries,
                &plan
            ),
            planner_error("shared exit has an outgoing transfer edge"),
            "AC-5.5b: the edge law must reject an edge leaving a shared exit"
        );

        // 6 — OVERLAP by a spurious extra seed: name an already-owned interior
        //     node as a scheduling entry. ⚠ A different construction from control
        //     5 — a bad seed rather than a bad edge.
        //
        // ⚠ Select it by its OWNER, not by excluding the node kinds I happen to
        // think of. Picking "not the terminal, not an entry, not a static body
        // target" selected the **trap** terminal, which is never traversed from,
        // so the control created no overlap and reddened at the population check
        // instead — green-for-the-wrong-reason, caught only because this control
        // asserts the exact error rather than merely `is_err`.
        let root_unit = SemanticOwner::Function(PredeclaredFunctionId(0));
        let interior = plan
            .nodes
            .iter()
            .map(|node| node.id)
            .find(|id| {
                *id != root_entry && plan.semantic.descriptors[id.0 as usize].owner == root_unit
            })
            .expect("an interior node inside the root unit");
        assert!(
            !plan.entries.contains(&interior)
                && !static_body.iter().any(|(_, target)| *target == interior),
            "control 6 needs a node that is not already a seed"
        );
        let mut extra_entries = plan.entries.clone();
        extra_entries.push(interior);
        assert_eq!(
            b2o_err(
                &plan.semantic,
                &plan.nodes,
                &plan.edges,
                &extra_entries,
                &plan
            ),
            planner_error("planned node is owned by more than one function unit"),
            "AC-5.6: an ordinary node owned by two seeds must redden"
        );

        // 7 — a SENTINEL misclassified as a Function.
        let mut misclassified = plan.semantic.clone();
        misclassified.descriptors[terminal.0 as usize].owner =
            SemanticOwner::Function(PredeclaredFunctionId(0));
        assert_eq!(
            b2o_err(
                &misclassified,
                &plan.nodes,
                &plan.edges,
                &plan.entries,
                &plan
            ),
            planner_error("semantic descriptor owner is not the node's derived function unit"),
            "AC-5.7: a shared exit recorded as a function unit must redden"
        );

        // 8 — a `ClosureBody` return successor assigned to the CALLER.
        //
        // ⚠ This is the one that would otherwise ship green: assigning the return
        // node to the caller is the *intuitive* reading of "the caller resumes
        // here", it produces a coherent-looking partition, and only the down/up
        // invariant catches it.
        let return_node = plan
            .nodes
            .iter()
            .find(|node| node.transition == TransitionKind::ClosureBody)
            .expect("a ClosureBody return successor")
            .id;
        let caller_owner = plan.semantic.descriptors[static_body[0].0 .0 as usize].owner;
        let callee_owner = plan.semantic.descriptors[return_node.0 as usize].owner;
        assert_ne!(
            caller_owner, callee_owner,
            "control 8 proves nothing unless the caller and callee units differ"
        );
        let mut handed_back = plan.semantic.clone();
        handed_back.descriptors[return_node.0 as usize].owner = caller_owner;
        assert_eq!(
            b2o_err(&handed_back, &plan.nodes, &plan.edges, &plan.entries, &plan),
            planner_error("semantic descriptor owner is not the node's derived function unit"),
            "AC-5.8: a return successor assigned to the caller must redden"
        );
    }

    /// The semantic disposition of a plan, as the ruling's four classification
    /// laws project it. **This is the authority** — an occurrence's owner and the
    /// planned edge kind — and it is deliberately computed from nothing else.
    ///
    /// ⚠ There is no Rust identifier, file name, method name or source offset
    /// anywhere in this function, and that absence is the point: it is why a Rust
    /// wrapper cannot move the result.
    pub(in crate::cranelift_backend::planning::static_transition) fn b2o_disposition(plan: &StaticTransitionPlan) -> (usize, usize, usize, usize) {
        let owner_of = |node: StaticNodeId| plan.semantic.descriptors[node.0 as usize].owner;
        let (mut cross_owner, mut intra_owner, mut shared_exit, mut other) = (0, 0, 0, 0);
        for edge in &plan.edges {
            match (owner_of(edge.from), owner_of(edge.to), edge.kind) {
                // Law 1 — a `StaticBody` edge between DISTINCT function owners is
                // a cross-owner call boundary.
                (SemanticOwner::Function(a), SemanticOwner::Function(b), EdgeKind::StaticBody)
                    if a != b =>
                {
                    cross_owner += 1
                }
                // Law 3 — a function edge to either shared exit is the validated
                // return/trap, never a call.
                (
                    SemanticOwner::Function(_),
                    SemanticOwner::Terminal | SemanticOwner::TrapTerminal,
                    _,
                ) => shared_exit += 1,
                // Law 2 — an ordinary edge inside one owner is local traversal.
                (SemanticOwner::Function(a), SemanticOwner::Function(b), _) if a == b => {
                    intra_owner += 1
                }
                // Law 4 — everything else is a graph planning refuses to build.
                _ => other += 1,
            }
        }
        (
            plan.semantic.functions.len(),
            cross_owner,
            intra_owner,
            shared_exit + other,
        )
    }

    /// `AC-10a` / `AC-10b` — **the harness that must stay GREEN under a Rust
    /// refactor.** Architect ruling `evt_5yxjd1zqnyvcq`.
    ///
    /// ⛔ **The verdict here is INVERTED from the four withdrawn folds.** Those
    /// spent four candidate SHAs making a relocation redden. A Rust wrapper, a
    /// nested `fn`, or a same-named method in a second `impl` creates **no Ken
    /// function-unit boundary**, so a pin that reddens on one is measuring
    /// implementation topology and reporting success.
    ///
    /// - **MEASURED:** the unit count and the three edge-classification counts,
    ///   derived from owners and edge kinds alone.
    /// - **CLAIMED:** that semantic disposition is a function of the plan graph,
    ///   so no source-level reorganisation can move it.
    /// - **THE GAP:** ⚠ a green here proves invariance only for mutations that
    ///   were actually *applied*. That is why `AC-10a`/`10b` are recorded
    ///   **mutation proofs against this pin**, and why `AC-10c` exists at all —
    ///   without it, deleting the assertion below would leave this green forever.
    ///
    /// Promise class: **transition sentinel** — the four numbers are a frozen
    /// snapshot of *this fixture*; the durable claim is their **invariance under
    /// source refactoring**, which only the recorded mutation proofs discharge.
    #[test]
    fn b2o_ac10_semantic_disposition_is_a_function_of_the_plan_graph_alone() {
        let declaration = b2o_transparent_declaration(unit());
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        let expr = b2o_two_closure_fixture();
        let plan = plan_static_transition_graph(&expr, &declarations).expect("plannable");

        let (units, cross_owner, intra_owner, exits) = b2o_disposition(&plan);

        // Non-vacuity before the snapshot: a fixture with no boundary would make
        // every claim below true for the wrong reason.
        assert!(
            cross_owner > 0,
            "the fixture has no cross-owner boundary, so 10a/10b would be green \
             on a harness that observes nothing"
        );
        assert_eq!(
            units,
            plan.entries.len()
                + plan
                    .edges
                    .iter()
                    .filter(|edge| edge.kind == EdgeKind::StaticBody)
                    .count(),
            "the unit population is the ruled seed set"
        );
        // ⚠ PREDICTED (2, 6, 3) before running; MEASURED (2, 4, 4). The
        // cross-owner count was right and the intra/exit split was not — I had
        // both retained-closure bodies reaching the terminal through one more
        // ordinary hop than they do. Recorded as a miss rather than silently
        // re-fitted, because a number edited to match an observation measures
        // nothing (`AC-11`, and the `D5` predictions before it).
        assert_eq!(
            (cross_owner, intra_owner, exits),
            (2, 4, 4),
            "AC-10: the semantic disposition of this fixture moved.\n\
             ⚠ If you reached this by RELOCATING A RUST CALL, adding a wrapper, \
             or adding a same-named method in another `impl`, the pin is not the \
             thing that is wrong -- the ruling is explicit that such a refactor \
             creates no Ken function-unit boundary and this MUST stay green. \
             Investigate why the plan graph moved.\n\
             If you reached it by changing the planner's edges or seeds, that IS \
             a semantic change and belongs in review."
        );
    }

    /// `AC-10c` — **the RED twin that makes `AC-10a`/`10b` mean something.**
    ///
    /// ⭐ Without this, `b2o_ac10_...` is green on a harness that observes
    /// nothing at all: delete its assertion body and the relocation proofs stay
    /// green forever. This control mutates the **one axis that IS authority** —
    /// the planned edge's owner endpoints — and requires both that the projection
    /// moves and that validation refuses the graph.
    #[test]
    fn b2o_ac10c_repointing_a_static_body_edge_changes_the_disposition() {
        let declaration = b2o_transparent_declaration(unit());
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        let expr = b2o_two_closure_fixture();
        let plan = plan_static_transition_graph(&expr, &declarations).expect("plannable");

        let before = b2o_disposition(&plan);

        // Repoint one `StaticBody` edge at a node owned by the SAME unit as its
        // source. Compile-preserving, and it is exactly the "moved boundary" a
        // source-text oracle could never see.
        let (index, edge) = plan
            .edges
            .iter()
            .enumerate()
            .find(|(_, edge)| edge.kind == EdgeKind::StaticBody)
            .map(|(i, e)| (i, *e))
            .expect("the fixture must carry a static body edge");
        let source_owner = plan.semantic.descriptors[edge.from.0 as usize].owner;
        let same_unit_target = plan
            .nodes
            .iter()
            .map(|node| node.id)
            .find(|id| {
                plan.semantic.descriptors[id.0 as usize].owner == source_owner
                    && *id != edge.to
                    // Must carry a source term. A control node would redden the
                    // pairing relation's non-source law instead, moving this
                    // control onto a different axis from the one it names.
                    && plan
                        .source_occurrences
                        .get(id.0 as usize)
                        .and_then(Option::as_ref)
                        .is_some()
            })
            .expect("the caller unit must hold a second source node to repoint at");

        let mut edges = plan.edges.clone();
        edges[index].to = same_unit_target;

        // `RT-BODY-OCCURRENCE-PROVENANCE`: a `StaticBody` edge and its issued
        // `entry -> body_occurrence` row are written together by one operation,
        // so repointing the edge alone builds a state the planner cannot
        // produce -- and the relation's own fail-closed law reddens FIRST,
        // stealing this control's witness and teaching that registration rather
        // than overlap is the primary detector.
        //
        // Repoint the row with the edge, giving the new target a well-formed
        // row of its own. The plan is then internally coherent and the ONLY
        // remaining defect is the ownership overlap this control names -- the
        // axis it claims is varied and its neighbours are held fixed.
        let mut relation = plan.planned_entry_bodies.clone();
        for row in &mut relation {
            if row.entry == edge.to {
                row.entry = same_unit_target;
                row.body_occurrence = origin_of(same_unit_target);
            }
        }
        let mut coherent = plan.clone();
        coherent.edges = edges.clone();
        coherent.planned_entry_bodies = relation;

        let after = {
            let mut repointed = plan.clone();
            repointed.edges = edges.clone();
            b2o_disposition(&repointed)
        };
        assert_ne!(
            before, after,
            "AC-10c: repointing a static body edge left the projection unchanged, \
             so the harness does not observe semantic disposition and 10a/10b are \
             vacuous"
        );

        // ⭐ I named the WRONG detector here and the exact-error assertion caught
        // it. I predicted `"static body edge does not cross a function unit
        // boundary"` -- the edge law. It reddens at **overlap** instead, and that
        // is correct and already documented: repointing the edge inside one unit
        // makes the target reachable from the caller's seed *while still being a
        // seed itself*, so the partition sees two owners before any edge law is
        // consulted. This independently re-confirms the corrected `D3` note that
        // overlap is the primary detector and the edge law is defense in depth.
        //
        // ⚠ `expect_err` would have been GREEN here and would have taught the
        // next reader that the edge law is load-bearing. Asserting the exact
        // error is the only reason this was visible.
        assert_eq!(
            b2o_err(
                &plan.semantic,
                &plan.nodes,
                &edges,
                &plan.entries,
                &coherent,
            ),
            planner_error("planned node is owned by more than one function unit"),
            "AC-10c: a static body edge repointed inside one unit must be REFUSED \
             by planning, not merely reclassified"
        );
    }


    /// `AC-6` — **inert.** The ABI plane declares and validates; it never emits.
    ///
    /// ⚠ MEASURED: the production region of `abi.rs` contains no emission
    /// construct. CLAIMED: exactly that. THE GAP: a source census cannot see an
    /// executable edge, and inertness is pinned BEHAVIOURALLY by
    /// `correspondence_adds_no_emitted_unit_to_the_production_census`. This is a
    /// declaration inventory that makes a new emission construct loud.
    ///
    /// Promise class: **durable invariant.**
    #[test]
    fn b2r_ac6_the_abi_plane_declares_no_emission_construct() {
        let abi = include_str!("abi.rs");
        let production = abi
            .split_once("\n#[cfg(test)]\nmod tests {")
            .map_or(abi, |(before, _)| before);

        // ⚠ POSITIVE CONTROL FIRST. Every assertion below is a NEGATIVE check,
        // and a negative check passes for any reason -- including a broken
        // comment-stripper that returns 0 for everything. So prove the
        // instrument can SEE before trusting what it does not see.
        assert!(
            b2r_code_identifier_occurrences(production, "AbiCarrier") > 0,
            "AC-6: the instrument reports zero occurrences of a token that is \
             certainly present in the production region, so its zeros below mean \
             nothing"
        );
        // And prove it reads CODE rather than comments: `FunctionBuilder` appears
        // in this module's doc comments (denying that it emits one), so a
        // stripper that failed to strip would report a non-zero count for it and
        // the real assertion below would redden for the wrong reason.
        assert!(
            abi.contains("FunctionBuilder"),
            "AC-6: the module no longer MENTIONS the construct it disclaims, so \
             the comment-stripping half of this instrument is untested"
        );

        // Comment-stripped and tokenized, so the doc comments that DENY emitting
        // (and must keep saying so) do not fire the oracle that checks it.
        for forbidden in [
            "FunctionBuilder",
            "define_function",
            "declare_function",
            "ins",
            "Signature",
        ] {
            assert_eq!(
                b2r_code_identifier_occurrences(production, forbidden),
                0,
                "AC-6: `{forbidden}` appears in the ABI plane's production code. \
                 This node is INERT: no new callable target unit, call edge, \
                 dispatch edge, callback, flag, alternate entry, encoder or \
                 decoder lands here -- `RT-FNSPLIT-B2F` performs the atomic \
                 switch-over."
            );
        }
    }

    /// `AC-7` — no oracle, no dependency. The ABI plane parses no source text.
    ///
    /// Promise class: **durable invariant.**
    #[test]
    fn b2r_ac7_the_abi_plane_adds_no_parser_and_no_dependency_edge() {
        let abi = include_str!("abi.rs");
        let production = abi
            .split_once("\n#[cfg(test)]\nmod tests {")
            .map_or(abi, |(before, _)| before);

        // ⚠ POSITIVE CONTROL. Same reasoning as `AC-6`: without it, a broken
        // instrument reports a clean bill of health it never measured.
        assert!(
            b2r_code_identifier_occurrences(production, "AbiPlane") > 0,
            "AC-7: the instrument reports zero occurrences of a token that is \
             certainly present, so its zeros below mean nothing"
        );

        for forbidden in ["syn", "proc_macro2", "quote", "include_str"] {
            assert_eq!(
                b2r_code_identifier_occurrences(production, forbidden),
                0,
                "AC-7: `{forbidden}` appears in the ABI plane. The population is \
                 the owner partition consumed as DATA; a source-parsing oracle \
                 is exactly the mechanism `B2O` spent four candidate SHAs ruling \
                 out."
            );
        }
    }


    /// Whole-token occurrences of `needle` in `source`'s **code**, with line and
    /// block comments stripped.
    ///
    /// ⛔ Tokenized rather than substring-matched: `line.contains("ins")` is a
    /// claim about formatting and fires on `instruction`, `against`, and every
    /// other word containing those letters.
    pub(in crate::cranelift_backend::planning::static_transition) fn b2r_code_identifier_occurrences(source: &str, needle: &str) -> usize {
        let mut code = String::with_capacity(source.len());
        let mut rest = source;
        let mut depth = 0usize;
        while !rest.is_empty() {
            if depth > 0 {
                if let Some(open) = rest.find("/*") {
                    if rest.find("*/").is_none_or(|close| open < close) {
                        depth += 1;
                        rest = &rest[open + 2..];
                        continue;
                    }
                }
                match rest.find("*/") {
                    Some(close) => {
                        depth -= 1;
                        rest = &rest[close + 2..];
                    }
                    None => break,
                }
                continue;
            }
            let block = rest.find("/*");
            let line = rest.find("//");
            match (block, line) {
                (Some(b), None) => {
                    code.push_str(&rest[..b]);
                    code.push(' ');
                    depth = 1;
                    rest = &rest[b + 2..];
                }
                (Some(b), l) if l.is_none_or(|l| b < l) => {
                    code.push_str(&rest[..b]);
                    code.push(' ');
                    depth = 1;
                    rest = &rest[b + 2..];
                }
                (_, Some(l)) => {
                    code.push_str(&rest[..l]);
                    code.push(' ');
                    rest = match rest[l..].find('\n') {
                        Some(nl) => &rest[l + nl..],
                        None => "",
                    };
                }
                (None, None) => {
                    code.push_str(rest);
                    rest = "";
                }
            }
        }
        code.split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|token| *token == needle)
            .count()
    }


    /// Promise class: durable invariant — process mode changes only the
    /// explicitly recorded root scheduling entry's declared source ingress.
    #[test]
    fn process_ingress_is_role_keyed_and_absent_from_value_roots() {
        let expr = RuntimeExpr::Value(RuntimeValue::Bool(true));
        let symbols = crate::NativeProcessSymbols::legacy_prelude();
        let transparent = RuntimeDeclaration {
            symbol: "decl:fixture::process_ingress::transparent".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::Value(RuntimeValue::Bool(false)),
            },
            metadata: crate::RuntimeSymbolMetadata {
                lowerability: Some(crate::RuntimeLowerabilityStatus::Supported),
                ..crate::RuntimeSymbolMetadata::empty()
            },
        };
        let declarations = BTreeMap::from([(transparent.symbol.as_str(), &transparent)]);
        let process = plan_static_transition_graph_with_symbols(
            &expr,
            &declarations,
            &symbols,
            AbiRootIngress::Process,
            true,
        )
        .expect("process root plans");
        let input = process
            .process_parameter_slot(AbiProcessParameter::ProcessInput)
            .expect("lookup validates")
            .expect("process input slot exists");
        let capability = process
            .process_parameter_slot(AbiProcessParameter::Capability)
            .expect("lookup validates")
            .expect("capability slot exists");
        assert_eq!(input.0.kind, AbiSlotKind::Parameter);
        assert_eq!(input.0.ordinal, 0);
        assert_eq!(capability.0.kind, AbiSlotKind::Parameter);
        assert_eq!(capability.0.ordinal, 1);
        assert_ne!(input.1, capability.1);
        let scheduling = process
            .emittable_units()
            .expect("validated units")
            .into_iter()
            .filter_map(|unit| match unit.definition() {
                AbiUnitDefinition::SchedulingEntry { ingress } => {
                    Some((ingress, unit.header().parameters))
                }
                AbiUnitDefinition::ClosureBody { .. }
                | AbiUnitDefinition::CallableDeclaration { .. }
                | AbiUnitDefinition::ContinuationSpecialization { .. }
                // `D2f`: a fusion region is not a scheduling entry, so it is
                // outside the population this census measures.
                | AbiUnitDefinition::StaticContinuationFusion { .. } => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            scheduling,
            vec![
                (AbiSchedulingIngress::ProcessPair, 2),
                (AbiSchedulingIngress::Empty, 0),
            ],
            "only the explicitly recorded process root acquires parameters"
        );

        let value = plan_static_transition_graph_with_symbols(
            &expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Value,
            true,
        )
        .expect("value root plans");
        assert_eq!(
            value
                .process_parameter_slot(AbiProcessParameter::ProcessInput)
                .expect("lookup validates"),
            None
        );
        assert_eq!(
            value
                .process_parameter_slot(AbiProcessParameter::Capability)
                .expect("lookup validates"),
            None
        );

        let closure = |captures| RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures,
                params: Vec::new(),
                body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
            }),
            args: Vec::new(),
        };
        let captured_expr = closure(vec![RuntimeExpr::Var(0), RuntimeExpr::Var(1)]);
        let captured = plan_static_transition_graph_with_symbols(
            &captured_expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            true,
        )
        .expect("capturing process closure plans");
        let capture_counts = captured
            .emittable_units()
            .expect("validated units")
            .into_iter()
            .filter_map(|unit| match unit.definition() {
                // `D2`: grouped WITH `ClosureBody`, not with the `None` arm.
                // These fixtures plan no declarations, so this changes nothing
                // today -- but a declaration added later would otherwise have
                // its captures silently dropped from the measured population.
                AbiUnitDefinition::ClosureBody { .. }
                | AbiUnitDefinition::CallableDeclaration { .. } => {
                    Some(unit.header().captures)
                }
                AbiUnitDefinition::SchedulingEntry { .. }
                | AbiUnitDefinition::ContinuationSpecialization { .. }
                // `D2f`: grouped with the `None` arm because a fusion region
                // declares no captures -- a property of the class, so no later
                // fusion unit has captures to be silently dropped here.
                | AbiUnitDefinition::StaticContinuationFusion { .. } => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(capture_counts, vec![2]);

        let uncaptured_expr = closure(Vec::new());
        let uncaptured = plan_static_transition_graph_with_symbols(
            &uncaptured_expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            true,
        )
        .expect("non-capturing process closure plans");
        let capture_counts = uncaptured
            .emittable_units()
            .expect("validated units")
            .into_iter()
            .filter_map(|unit| match unit.definition() {
                // `D2`: grouped WITH `ClosureBody`, not with the `None` arm.
                // These fixtures plan no declarations, so this changes nothing
                // today -- but a declaration added later would otherwise have
                // its captures silently dropped from the measured population.
                AbiUnitDefinition::ClosureBody { .. }
                | AbiUnitDefinition::CallableDeclaration { .. } => {
                    Some(unit.header().captures)
                }
                AbiUnitDefinition::SchedulingEntry { .. }
                | AbiUnitDefinition::ContinuationSpecialization { .. }
                // `D2f`: grouped with the `None` arm because a fusion region
                // declares no captures -- a property of the class, so no later
                // fusion unit has captures to be silently dropped here.
                | AbiUnitDefinition::StaticContinuationFusion { .. } => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            capture_counts,
            vec![0],
            "an otherwise identical body without a free binding acquired a slot"
        );
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn substrate_case_fixture() -> RuntimeExpr {
        RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::If {
                scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                then_expr: Box::new(substrate_constructor("Left")),
                else_expr: Box::new(substrate_constructor("Right")),
            }),
            cases: ["Left", "Right", "Absent"]
                .into_iter()
                .map(substrate_case)
                .collect(),
            default: trap("substrate default"),
        }
    }

    /// D1 pin.
    ///
    /// MEASURED: the exact root Match records the union of its two Construct
    /// result alternatives; Left and Right are reachable and Absent is
    /// eliminated. A copied record attributed to a different occurrence is
    /// rejected by fresh re-derivation.
    ///
    /// CLAIMED: elimination authority comes from closed result flow keyed by
    /// exact occurrence, never from syntax containment or a constructor
    /// catalog.
    ///
    /// GAP: opaque calls/effects/outer variables deliberately yield Open and
    /// are pinned separately by the fail-closed control below.
    #[test]
    fn substrate_case_emission_closes_the_exact_alternative_union() {
        let expr = substrate_case_fixture();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plans");
        let root = plan.root_static_origin().expect("root occurrence");
        let records = plan
            .case_emissions
            .iter()
            .filter(|record| record.match_origin == root)
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 3);
        assert_eq!(
            records
                .iter()
                .map(|record| record.status)
                .collect::<Vec<_>>(),
            vec![
                CaseEmissionStatus::Reachable,
                CaseEmissionStatus::Reachable,
                CaseEmissionStatus::Eliminated,
            ]
        );
        let CaseProducerSet::Closed(producers) = &records[0].authority.producers else {
            panic!("finite constructor alternatives became open");
        };
        assert_eq!(producers.len(), 2);

        let mut wrong_occurrence = plan.case_emissions.clone();
        wrong_occurrence[0].match_origin = wrong_occurrence[0].scrutinee_origin;
        assert_eq!(
            validate_case_emission_plan(&plan, &wrong_occurrence).unwrap_err(),
            planner_error(
                "dormant case-emission facts are not the exact closed producer derivation"
            )
        );
    }

    /// MEASURED: an opaque effect scrutinee gives positive Open authority and
    /// every case remains reachable.
    ///
    /// CLAIMED: failure to close producer flow can never become case-pruning
    /// authority.
    ///
    /// GAP: this does not assert a host operation's concrete result catalog;
    /// that would be the rejected catalog-as-reachability shortcut.
    #[test]
    fn substrate_case_emission_open_ingress_prunes_nothing() {
        let expr = RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Effect {
                family: "FS".to_string(),
                operation: ken_host::HostOpV1::BufferAllocate,
                capability: None,
                args: vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
            }),
            cases: ["Left", "Right"].into_iter().map(substrate_case).collect(),
            default: trap("opaque substrate default"),
        };
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plans");
        assert!(plan.case_emissions.iter().all(|record| {
            matches!(record.authority.producers, CaseProducerSet::Open)
                && record.status == CaseEmissionStatus::Reachable
        }));
    }


    /// D3 pin.
    ///
    /// MEASURED: D3 rejects a compile-valid mutation that changes the Absent
    /// case from Eliminated to Reachable while leaving its closed producer
    /// authority untouched.
    ///
    /// CLAIMED: D1 and D2 close jointly before ABI planning, so an unreachable
    /// case, foreign origin or owner transplant cannot survive as plausible
    /// dormant authority.
    ///
    /// GAP: descriptor/allocation zero-counters belong to Slice 2; this slice
    /// runs earlier than descriptor construction and activates no emitter.
    #[test]
    fn substrate_preallocation_closure_rejects_an_admitted_unreachable_case() {
        let expr = substrate_case_fixture();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plans");
        let mut admitted = plan.case_emissions.clone();
        let record = admitted
            .iter_mut()
            .find(|record| record.status == CaseEmissionStatus::Eliminated)
            .expect("fixture has an unreachable case");
        record.status = CaseEmissionStatus::Reachable;
        assert_eq!(
            validate_substrate_preallocation_closure(
                &plan,
                &admitted,
                &plan.occurrence_authorities,
            )
            .unwrap_err(),
            planner_error("pre-allocation closure admits an unreachable case")
        );
    }


    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D4b` — the admission partition is EXACTLY
    /// `interned = V` / `declined = R`, with no third modality.**
    ///
    /// ⭐ `D4a` admitted the producer-local domain by **deleting** a filter, and
    /// the claim it left behind is that `R` is refused upstream by the
    /// take-loop's own two clauses — an `Open` value, or a position carrying
    /// more than one exact source — and by nothing else.
    ///
    /// ⛔ **The proposition is an equivalence, and both directions are asserted
    /// per record**: a candidate is admitted **iff** every required position is
    /// closed and unambiguous. An extra route modality, a special case, a corpus
    /// lookup, a closure-identity test or a first-`Open` classification would all
    /// appear as a record where the two sides disagree.
    ///
    /// ⛔ **Both sides must be witnessed**, or the equivalence is half-vacuous.
    /// The declining fixtures are named rather than hoped for: the required-tail
    /// fixture supplies an `Open` and an `Ambiguous(2)` position, and the
    /// IH/argument fixture supplies the `Open[ih-binder]` edge — which are the
    /// census's three non-closed positions, and the only ones in the corpus.
    ///
    /// ⚠ **MEASURED**: admission agrees with the closed-vector predicate on
    /// every record, both sides non-empty, and every declined record carries an
    /// `Open` or ambiguous position. **CLAIMED**: nothing outside the required
    /// vector participates in the decision. **THE GAP**: this measures the
    /// decision, not the vector — that each position's verdict is itself right
    /// is the source walk's authority, not this row's.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn d4b_admission_is_exactly_the_closed_required_vector() {
        let symbols = crate::NativeProcessSymbols::legacy_prelude();
        d4b_arm_admission(true);
        // Admitting fixtures.
        let complete = Box::leak(Box::new(contspec_complete_environment_fixture()));
        let _ = plan_static_transition_graph_with_symbols(
            complete,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        );
        // ⛔ The THREE declining shapes, named rather than hoped for. Between
        // them they carry exactly the census's three non-closed positions.
        //
        // `Open[let-value:Construct]` — a required tail with open provenance.
        let tail = Box::leak(Box::new(contspec_required_tail_fixture(unit())));
        let _ = plan_static_transition_graph_with_symbols(
            &*tail,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        );
        // `AMBIG2[let-value:If]` — an `If` whose branches name two DISTINCT
        // exact sources, which the walk refuses to collapse to one ordinal.
        let ambiguous_tail = RuntimeExpr::If {
            scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
            then_expr: Box::new(RuntimeExpr::Var(0)),
            else_expr: Box::new(RuntimeExpr::Var(1)),
        };
        let ambiguous = Box::leak(Box::new(contspec_required_tail_fixture(ambiguous_tail)));
        let _ = plan_static_transition_graph_with_symbols(
            &*ambiguous,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        );
        let ih = Box::leak(Box::new(contsrc_d2_ih_and_argument_case_fixture()));
        let _ = plan_static_transition_graph(ih, &BTreeMap::new());
        d4b_arm_admission(false);
        let records = d4b_take_admission();

        assert!(
            !records.is_empty(),
            "the witness corpus must produce candidate edges at all"
        );
        for (vector, admitted) in &records {
            let all_closed = vector.iter().all(|verdict| *verdict == D4bVerdict::Closed);
            assert_eq!(
                *admitted, all_closed,
                "admission must be exactly 'every required position closed and unambiguous'; \
                 this record disagrees, which is an extra route modality: vector={vector:?}"
            );
        }

        let admitted = records.iter().filter(|(_, a)| *a).count();
        let declined = records.len() - admitted;
        assert!(
            admitted > 0,
            "no admitted edge was witnessed, so V is unmeasured and the equivalence is \
             half-vacuous"
        );
        assert!(
            declined > 0,
            "no DECLINED edge was witnessed ({admitted} of {}), so R is unmeasured -- the row \
             would pass on a corpus where admission is unconditionally true",
            records.len()
        );

        // ⛔ Every declined edge is refused by one of the take-loop's two
        // clauses. A decline with an all-closed vector would mean some other
        // predicate is running.
        for (vector, admitted) in &records {
            if !admitted {
                assert!(
                    vector.iter().any(|verdict| matches!(
                        verdict,
                        D4bVerdict::Open | D4bVerdict::Ambiguous(_)
                    )),
                    "a declined edge must carry an Open or ambiguous position: vector={vector:?}"
                );
            }
        }

        // ⛔ Both decline CLAUSES are witnessed, not just one. A corpus carrying
        // only `Open` declines would leave the ambiguity clause unmeasured.
        assert!(
            records.iter().any(|(v, _)| v.contains(&D4bVerdict::Open)),
            "the Open decline clause is unwitnessed"
        );
        assert!(
            records
                .iter()
                .any(|(v, _)| v.iter().any(|x| matches!(x, D4bVerdict::Ambiguous(_)))),
            "the ambiguity decline clause is unwitnessed"
        );
    }


    // -- `D7` checkpoint `1c`: the matrix-omission law, in PLANNING ----------
    //
    // ⭐⭐ The law is that a real planned static-worker member which is omitted
    // or misclassified must fail HERE, and may not fall through to the late
    // generic `Closure` arm. Its two halves are measured separately: an exact
    // member's absence or reclassification must red in planning, and an
    // ORDINARY closure -- one outside any planner-proved edge -- must be
    // untouched by it.

    /// One closure shape, used twice: once in the recursive-position seat where
    /// the planner proves an exact edge, and once as an ordinary `Let` binder
    /// where it proves nothing.
    ///
    /// ⭐⭐ **Same shape on purpose.** The frame's scoping sentence is about *the
    /// same closure* outside an exact planner-proved edge, so a fixture whose
    /// two closures differed in arity or capture count could satisfy the law by
    /// telling them apart on shape rather than on membership -- and would say
    /// nothing about the property.
    pub(in crate::cranelift_backend::planning::static_transition) fn d7_1c_member_and_ordinary_twin_fixture() -> RuntimeExpr {
        let twin = || RuntimeExpr::LexicalClosure {
            captures: vec![unit()],
            params: vec!["twin".to_string()],
            body: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                args: Vec::new(),
            }),
        };
        RuntimeExpr::Let {
            // The ordinary twin. Nothing proves an edge into it, so no
            // specialization names it and the law must not demand one.
            value: Box::new(twin()),
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    args: vec![twin()],
                }),
                cases: vec![
                    crate::RuntimeComputationalMatchCase {
                        constructor: "ctor:fixture::Contspec::Node".to_string(),
                        argument_binders: 1,
                        recursive_positions: vec![0],
                        body: RuntimeExpr::Var(0),
                    },
                    crate::RuntimeComputationalMatchCase {
                        constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                        argument_binders: 0,
                        recursive_positions: Vec::new(),
                        body: unit(),
                    },
                ],
                default: RuntimeTrap {
                    code: RuntimeTrapCode::PatternMatchFailure,
                    message: "d7 1c twin fixture".to_string(),
                },
            }),
        }
    }

    /// The members a plan's specialization population names, and the
    /// `ClosureBody` units it does not.
    pub(in crate::cranelift_backend::planning::static_transition) fn d7_1c_member_and_ordinary_body_counts(plan: &StaticTransitionPlan<'_>) -> (usize, usize) {
        let members = plan
            .continuation_specializations
            .iter()
            .map(|specialization| specialization.key.worker.body_origin)
            .collect::<BTreeSet<_>>();
        let ordinary = plan
            .abi
            .descriptors
            .iter()
            .filter(|descriptor| {
                matches!(descriptor.definition, AbiUnitDefinition::ClosureBody { .. })
                    && !members.contains(&descriptor.body_occurrence)
            })
            .count();
        (members.len(), ordinary)
    }

    /// **The positive control and the scoping half in one measurement.**
    ///
    /// ⭐ The scoping assertion is what stops this law from being "every closure
    /// body needs a specialization". That reading would also make every mutation
    /// row below red, so the rows alone cannot distinguish the two -- only a
    /// plan that carries an ordinary `ClosureBody` and still closes can.
    #[test]
    fn d7_1c_the_member_population_closes_and_ordinary_closures_are_outside_it() {
        let expr = d7_1c_member_and_ordinary_twin_fixture();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new())
            .expect("the twin fixture plans, with the member law in force");
        let (members, ordinary) = d7_1c_member_and_ordinary_body_counts(&plan);
        assert!(
            members >= 1,
            "the fixture must prove at least one exact planner edge, or every row below is vacuous"
        );
        assert!(
            ordinary >= 1,
            "the fixture must also carry a closure body OUTSIDE every proved edge, or the law is \
             indistinguishable from `every closure body needs a specialization`"
        );
        validate_static_worker_member_population(&plan)
            .expect("the derived member population is closed");
    }

    /// **Every compile-valid corruption of the exact member reds IN PLANNING.**
    ///
    /// ⭐ Four settings and not one, because a member can be wrong by not being
    /// there, by being there as a different unit kind, by being attributed to a
    /// different closure, and by declaring a different contract in the same
    /// place. `Reclassify` and `Misdeclare` are the two a positive assertion is
    /// most likely to agree with by accident: the unit is still present, still
    /// declared and still has a function.
    ///
    /// ⛔ **The first three share one refusal, and that is the honest reading
    /// rather than a collapsed check.** This law joins the two populations on
    /// the defining closure occurrence, so "the unit is gone", "the unit is not
    /// a closure body" and "the unit belongs to another closure" are one fact to
    /// it: no emittable unit defines this worker's closure. The contract row is
    /// what proves the law is not merely a presence test -- it fails with the
    /// member present, correctly classified, and correctly attributed.
    #[test]
    fn d7_1c_an_omitted_or_misclassified_member_reds_in_planning() {
        let expr = d7_1c_member_and_ordinary_twin_fixture();
        for (mutation, expected) in [
            (
                StaticWorkerMemberMutation::OmitMember,
                "no emittable unit defines",
            ),
            (
                StaticWorkerMemberMutation::ReclassifyMember,
                "no emittable unit defines",
            ),
            (
                StaticWorkerMemberMutation::RedirectDefiningOccurrence,
                "no emittable unit defines",
            ),
            (
                StaticWorkerMemberMutation::MisdeclareMemberContract,
                "different parameter count",
            ),
        ] {
            let Err(error) = with_static_worker_member_mutation(mutation, || {
                plan_static_transition_graph(&expr, &BTreeMap::new())
            }) else {
                panic!("{mutation:?}: a corrupted member population must not close")
            };
            let CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason)) = &error
            else {
                panic!("{mutation:?} must refuse in PLANNING, got {error:?}")
            };
            assert!(
                reason.contains(expected),
                "{mutation:?} must be attributed to the member law, got {reason:?}"
            );
        }
    }

    /// **The refusal reaches the whole lowering entry, and is never the generic
    /// `Closure` diagnostic.**
    ///
    /// ⭐⭐ The planning-entry rows above prove the checker fires; this proves
    /// the checker is ON THE PATH a program takes. Planning runs before any
    /// Cranelift function is defined, so a `PlannerInvariant` here is a refusal
    /// before definition or object emission by construction -- and no object is
    /// returned to say otherwise.
    ///
    /// ⛔ The unmutated row is the positive control: the same fixture through
    /// the same entry must not produce this refusal, which is what makes the
    /// mutated rows evidence rather than a fixture that never compiled.
    #[test]
    fn d7_1c_the_planning_refusal_is_what_the_whole_lowering_entry_reports() {
        let example = crate::RuntimeExample {
            name: "d7-1c-member-law".to_string(),
            checked_core_shape: "LexicalClosure".to_string(),
            ir: d7_1c_member_and_ordinary_twin_fixture(),
            observation: crate::RuntimeObservation::Returned(crate::RuntimeGroundValue::Bool(true)),
        };
        let compile = || {
            crate::run_example_with_seed_observation(&example, &crate::NativeSeedEnvironment::empty())
                .err()
        };
        let member_law = |error: &CraneliftBackendError| {
            matches!(
                error,
                CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason))
                    if reason.contains("no emittable unit defines")
            )
        };
        if let Some(error) = compile() {
            assert!(
                !member_law(&error),
                "the unmutated fixture must not trip the member law: {error:?}"
            );
        }
        let error = with_static_worker_member_mutation(
            StaticWorkerMemberMutation::OmitMember,
            compile,
        )
        .expect("an omitted member must refuse the whole entry, not produce a run report");
        assert!(
            member_law(&error),
            "an omitted member must refuse in planning, never at the generic closure \
             diagnostic: {error:?}"
        );
    }
}
