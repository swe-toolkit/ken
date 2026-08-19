//! Factored, pre-emission native transition planner.
//!
//! Node code identity is `(transition kind, static node id)` and edge code
//! identity is `(edge kind, static edge id)`. Dynamic environment,
//! continuation, cleanup, source, and affine state travels as constant-width
//! IDs into hash-consed persistent stores.

mod abi;
mod aggregates;
mod closure;
mod construction;
mod continuations;
mod effects;
mod joins_traps;
mod occurrences;
mod semantic_ir;
mod units;

use std::collections::BTreeMap;
#[cfg_attr(not(test), allow(unused_imports))]
use std::collections::BTreeSet;

use super::{
    backend, unsupported, BackendFailure, CraneliftBackendError, RuntimeDeclaration,
    RuntimeDeclarationKind,
};
#[cfg_attr(not(test), allow(unused_imports))]
use crate::boundary_value::{BoundaryClass, BoundaryReferentOwner, BoundaryTag};
use crate::{RuntimeExpr, RuntimeTrap};
use abi::AbiPlane;
#[cfg_attr(not(test), allow(unused_imports))]
use abi::install_continuation_specialization_abi;
use semantic_ir::{SemanticMaterialArena, SemanticPlane, SemanticSourceKind, SemanticSourceSeed};
#[cfg_attr(not(test), allow(unused_imports))]
use semantic_ir::build_synthesized_constructor_inventory;
#[cfg_attr(not(test), allow(unused_imports))]
use semantic_ir::RuntimeExprShape;
// `RT-CONTSRC-PRODUCER-LOCAL` `D2` — the shape vocabulary, so a producer-local
// contract asks the existing `abi::result_carrier` authority rather than
// restating a carrier.
use occurrences::{
    occurrence_authority, occurrence_subtree_contains, PlannedOccurrence, PlannedOccurrenceAuthority,
    PlannedOccurrenceChildAuthority,
};
#[cfg_attr(not(test), allow(unused_imports))]
use occurrences::{origin_of, validate_occurrence_authority_plan};

// ⭐ `D1`'s capability surface. The two identity types cross into
// `crate::cranelift_backend` so `lowering` can hold and compare them; ⛔
// `SemanticPlane`, `SemanticMaterialArena` and the `names` arena stay on the
// `use` above, visible only inside this planner. Widening either of those to
// serve a consumer is the move `§2d` forbids.
pub(in crate::cranelift_backend) use abi::{
    AbiCaptureProvenance, AbiCarrier, AbiFrameHeader, AbiOwnership, AbiProcessParameter,
    AbiRootIngress, AbiSchedulingIngress, AbiSlot, AbiSlotKind, AbiStorageOwner, AbiUnitDefinition,
    expected_capture_slot,
};
#[cfg(test)]
pub(in crate::cranelift_backend) use semantic_ir::with_last_io_error_role_omitted;
#[cfg(test)]
pub(in crate::cranelift_backend) use semantic_ir::{
    with_d2a_population_mutation, D2aPopulationMutation,
};
pub(in crate::cranelift_backend) use semantic_ir::{
    ConstructorIdentity, FieldIdentity, SynthesizedConstructorRole, SynthesizedFixedConstructorRole,
};
pub(in crate::cranelift_backend) use occurrences::StaticOriginId;
pub(in crate::cranelift_backend) use units::{
    EmittableCallKind, PredeclaredFunctionId,
};

#[allow(unused_imports)]
pub(in crate::cranelift_backend) use continuations::{
    ContinuationSpecializationId, ContinuationEmissionOwner, ContinuationContextId, PlannedContinuationContext, ContinuationContextView, ContinuationInputSource, ProducerLocalBinding, ProducerLocalLocator, ContinuationSourceCoordinate, ContinuationEnvironmentClaimOver, ContinuationEnvironmentClaim, ContinuationEnvironmentDraft, ContinuationFrameRequirement, ContinuationFrameIdentity, ContinuationAvailabilityOver, ContinuationAvailabilityViews, ContinuationAvailabilityDraft, ContinuationSourceSlotAuthority, ContinuationWorkerCaptureSource, ContinuationWorkerCaptureProvenance, ContinuationConsumingOccurrence, RequiredConsumerProjection, ContinuationCallIdentity, ContinuationUnitView, ContinuationOrdinaryEnvelopeRole, ComposedWorkerRouteEligibility, ComposedWorkerView, ComposedCallTarget, ContinuationInputView, ContinuationCallView, ContinuationResultEdge, verify_current_lexical_availability, verify_predeclared_entry_frame_membership, FusionComposedEdge, FusionOwnedOuterRealization, FusionCompositionLayer, AdmittedContinuationDiscovery, CheckedCaseBinderRole, CheckedCaseBinderLayout, CheckedIhBinding, CheckedTransportCoordinate, StaticContinuationFusionId, StaticContinuationFusionKey, StaticContinuationFusionDescriptor, StaticContinuationFusionPlan, StaticContinuationFusionView, fusion_redirect_target, BodyEmissionDisposition, FusionOwnedBody, FusionRegionClaim, FusionClaimRefusal, FusionRegionClaimLedger, build_static_continuation_fusion_plan, StaticContinuationFusionCandidate,
};
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::cranelift_backend) use continuations::{
    D3bFinalizationPerturbation, d3b_refinalize, d3b_publish_without_finalization, ComposedCallTargetDefect, set_composed_call_target_defect, RequiredConsumerProjectionDisposition, ContinuationRequiredConsumerObservation, RequiredConsumerProjectionMutation, with_required_consumer_projection_mutation, take_continuation_required_consumer_observations, with_continuation_consuming_occurrence_seed_mutated, with_continuation_consuming_eliminator_seed_mutated, EnvelopeDefect, set_envelope_defect, set_primary_fusion_key_derivation_mutated, set_binder_body_resolution_suppressed, set_static_body_triple_duplicated, set_post_specialization_descent_suppressed, set_continuation_descent_owner_duplication, FusionClaimParameterMutation, FusionProducerCaptureMutation, with_fusion_producer_capture_mutation, with_fusion_claim_parameter_mutation, reset_r3_fusion_claim_consumptions, r3_fusion_claim_consumptions,
};
use continuations::{PlannedContinuationSpecialization, PlannedContinuationSpecializationCall};
#[cfg(test)]
use continuations::{
    validate_continuation_specialization_closure,
};

#[cfg(test)]
#[allow(unused_imports)]
use continuations::{
    CONTINUATION_INTERN_MUTATION, CONTINUATION_PRODUCTION_MUTATION, COMPOSED_CALL_TARGET_DEFECT, WEAKEN_CONTINUATION_DECREASING_MEASURE, SUPPRESS_POST_SPECIALIZATION_DESCENT, DUPLICATE_STATIC_BODY_TRIPLE, ENVELOPE_DEFECT,
};
#[cfg(test)]
use continuations::{
    ContinuationProjectionOmission, ContinuationInternMutation, ContinuationProductionMutation,
};

// `RT-PLANNER-AGGREGATES-SPLIT` `D1` — the aggregates domain's cross-boundary
// surface: `lowering` and `planning`'s own re-export both reach these through
// this module, unchanged from before the move.
#[allow(unused_imports)]
pub(in crate::cranelift_backend) use aggregates::{
    AggregateOccurrenceId, AggregateOccurrenceProducer, PlannedAggregateAllocation,
    PlannedAggregateOwnership, PlannedAggregateShape, SynthesizedAggregateNode,
    SynthesizedAggregatePath, SynthesizedAggregateRole, SynthesizedAggregateRoot,
    SynthesizedDynamicSet,
};
use aggregates::lifetime_referent_affinity;
#[cfg(test)]
use aggregates::{
    aggregate_child_referent_owners, fixed_node_selected_owner,
    flatten_allocation_reachable_uses, host_effect_recipe_tree, node_referent_owners,
    validate_aggregate_producers_are_unique, SynthesizedAggregateStep,
};

// `RT-PLANNER-EFFECTS-SPLIT` `D1` — the host-effect seat authority's
// cross-boundary surface: `lowering` and `planning`'s own re-export both
// reach these through this module, unchanged from before the move.
#[allow(unused_imports)]
pub(in crate::cranelift_backend) use effects::{
    host_effect_seat_contract_of, EffectSeatAvail, EffectSeatNeed, EffectSeatOperation,
    EffectSeatPhase, EffectSeatSlot, PlannedEffectSeat, CRANELIFT_HOST_EFFECT_CONSUMERS_V1,
};
#[cfg(test)]
pub(in crate::cranelift_backend) use effects::{set_effect_seat_plan_mutation, EffectSeatPlanMutation};

// `RT-PLANNER-JOINS-TRAPS-SPLIT` `D1` — the joins-traps domain's
// cross-boundary surface: `lowering` and `planning`'s own re-export both
// reach these through this module, unchanged from before the move.
#[allow(unused_imports)]
pub(in crate::cranelift_backend) use joins_traps::{
    planned_partiality_trap, JoinPlanToken, JoinResultRepresentation,
};
use joins_traps::PlannedJoinResult;

// `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — the closure lifecycle's
// cross-boundary surface: `lowering` and `planning`'s own re-export both
// reach these through this module, unchanged from before the split.
pub(in crate::cranelift_backend) use closure::CaseEmissionStatus;
#[cfg(test)]
pub(in crate::cranelift_backend) use closure::{PlannedResultFieldKindForTest, ScaleBPlanCensus};
use closure::PlannedCaseEmission;
use construction::Planner;
// `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — root's own `mod tests` (still
// resident here pending `D2`) reaches these purely-internal construction/
// closure items through this module's own `use super::*` glob, exactly as
// items 4-9's own moved-domain tests did.
#[cfg(test)]
use closure::{
    d4b_arm_admission, d4b_take_admission, with_static_worker_member_mutation,
    validate_static_worker_member_population, validate_case_emission_plan,
    validate_substrate_preallocation_closure, BoundaryACensus, BoundaryB1Census, CaseProducerSet,
    D4bVerdict, StaticWorkerMemberMutation, MAX_HELPERS_PER_STATIC_SOURCE,
};
#[cfg(test)]
use construction::{
    reset_recursive_lowering_frame_count, D4DeclarationTargetMutation,
    D4_DECLARATION_TARGET_MUTATION,
};


/// One planned entry paired with the body occurrence its own planning visit
/// returned.
///
/// **This is the SOLE entry/body pairing authority, for EVERY seed class.** Its
/// exact key population is `plan.entries` UNION every `StaticBody` target. Both
/// fields come from one [`PlannedExpr`], so the pair is *issued* at the moment
/// the two identities exist together and is never recovered afterwards.
///
/// **One authority extended, not a parallel ledger.** The first attempt scoped
/// this to scheduling-entry registration and let `StaticBodyTarget` seeds keep
/// a child-0 relation, on the ground that a body node's own origin is already
/// grounded. That holds for an ordinary body and **diverges for a body that
/// schedules something before itself** — the identical two-axis split, one seat
/// class over. Measured: owner 2 seeded on `n58` was issued `SOI(58)` while its
/// real body was `SOI(26)`, and its four planned joins were never entered.
///
/// Recovering the body by asking which resume is "outermost" — by graph
/// traversal, completion-edge shape, an owner scan, expression shape, origin
/// arithmetic, or a first-match rule — is the inference this record exists to
/// retire. A nested match supplies several resumes under one entry and **no
/// property of the graph distinguishes the unit boundary among them**; only the
/// returning visit knows, and only at the instant it returns.
///
/// `root_occurrence` and `declaration_occurrences` are projections of this
/// table, equality-checked against it by
/// [`StaticTransitionPlan::validate_planned_entry_bodies`].
/// `declaration_origins` stays a membership projection and is never a pairing
/// authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlannedEntryBody {
    entry: StaticNodeId,
    body_occurrence: StaticOriginId,
}


#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
struct StaticNodeId(u32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
struct StaticEdgeId(u32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
struct StaticSourceId(u32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
struct PersistentNodeId(u32);

#[derive(Clone, Copy)]
struct PlanContext {
    environment: PersistentNodeId,
    continuation: PersistentNodeId,
    path: PersistentNodeId,
    cleanup: PersistentNodeId,
    affine: PersistentNodeId,
    source_return: PersistentNodeId,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum TransitionKind {
    Terminal,
    TrapTerminal,
    Evaluate,
    Sequence,
    Branch,
    CaseTest,
    ClosureBody,
    ProducerWrapper,
    SourceReturnResume,
    ProducerTail,
    CompletedTail,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum EdgeKind {
    Continue,
    Select,
    Reject,
    InvokeProducerWrapper,
    SourceReturnOwnedResume,
    InvokeProducerTail,
    CompleteProducerTail,
    StaticBody,
    DeclarationCall,
    Trap,
}

/// **`RT-DECL-CLOSURE-PORT` `D4` — the two lawful targets of a
/// `EdgeKind::DeclarationCall` edge, kept apart by name.**
///
/// A transparent declaration is planned as one scheduling entry. Whether that
/// entry *is* the callable unit depends on the declaration's own body:
///
/// - a body that is not a closure seed schedules and returns a value, so the
///   entry is its unit and the call takes **no inputs**;
/// - a `Closure` / `LexicalClosure` seed body owns a second unit — the
///   declaration-owned [`abi::AbiUnitDefinition::CallableDeclaration`] reached
///   by the entry's one forward `StaticBody` edge — which declares the
///   declaration's parameters and captures and must be called **with them**.
///
/// ⛔ The two are indistinguishable downstream once flattened: both are reached
/// by the same edge kind, both resolve to a `FuncId`, and `&[]` type-checks
/// against either. ⇒ The class is carried, not re-inferred.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum DeclarationCallTargetClass {
    /// The declaration's own scheduling entry, called with no inputs.
    SchedulingEntry,
    /// The declaration-owned callable unit, called with the declaration's
    /// parameters followed by its captures.
    CallableDeclaration,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum StoreKind {
    Syntax,
    Environment,
    Continuation,
    Path,
    Cleanup,
    Affine,
    SourceReturn,
}

/// The complete fixed-width helper identity. It contains no activation or
/// occurrence path.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
enum PlannedHelperKey {
    Node(TransitionKind, StaticNodeId),
    Edge(EdgeKind, StaticEdgeId),
}

impl PlannedHelperKey {
    const fn node(transition: TransitionKind, node: StaticNodeId) -> Self {
        Self::Node(transition, node)
    }

    const fn edge(kind: EdgeKind, edge: StaticEdgeId) -> Self {
        Self::Edge(kind, edge)
    }
}

/// Fixed ABI shape carried between helpers. Every field is one dense ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
struct DynamicActivationFrame {
    syntax: PersistentNodeId,
    environment: PersistentNodeId,
    normal: PersistentNodeId,
    abrupt: PersistentNodeId,
    path: PersistentNodeId,
    cleanup: PersistentNodeId,
    affine: PersistentNodeId,
    source_return: PersistentNodeId,
}

/// The sole persistent-node schema. `local` and `aux` are dense IDs/tags, and
/// `child` is the shared suffix. No vector or recursive payload is inline.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
struct PersistentStoreNode {
    kind: StoreKind,
    local: u32,
    aux: u32,
    child: PersistentNodeId,
}

#[derive(Clone, Copy, Debug)]
struct StaticNode {
    id: StaticNodeId,
    transition: TransitionKind,
    owner: StaticSourceId,
    frame: DynamicActivationFrame,
}

#[derive(Clone, Copy, Debug)]
struct StaticEdge {
    id: StaticEdgeId,
    from: StaticNodeId,
    to: StaticNodeId,
    kind: EdgeKind,
}

/// Exact graph evidence is deliberately out of line and keyed by one edge ID.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct EdgeEvidence {
    edge: u32,
    owner: StaticSourceId,
    from: StaticNodeId,
    to: StaticNodeId,
    kind: EdgeKind,
}

/// The shortest referent lifetime a later planner slice may rely on.
///
/// Unknown or dynamic result forms are conservatively activation-owned.
/// `Persistent` is issued only when the complete source result is closed over
/// persistent children. There is deliberately no promotion operation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum PlannedReferentLifetime {
    Persistent,
    ActivationOwned,
}



/// Bounds-checked dense-range slice, failing closed rather than truncating.
fn dense_slice<T>(arena: &[T], range: semantic_ir::DenseRange) -> Option<&[T]> {
    let start = range.start as usize;
    let end = start.checked_add(range.len as usize)?;
    arena.get(start..end)
}


#[derive(Clone)]
pub(in crate::cranelift_backend) struct StaticTransitionPlan<'src> {
    entries: Vec<StaticNodeId>,
    /// The exact `entry -> body_occurrence` pairing, one row per `entries` row
    /// and in the same order.
    ///
    /// Kept **beside** `entries` rather than replacing its element type: the
    /// scheduling-entry population is frozen topology consumed by the owner
    /// partition, the ABI plane and their validators, and reshaping it would
    /// churn all of them for no gain. Exactness comes from the constructor
    /// instead — [`Self::register_scheduling_entry`] is the only writer of
    /// either vector and pushes to both, so an entry without a pair is
    /// unconstructible rather than merely rejected.
    planned_entry_bodies: Vec<PlannedEntryBody>,
    nodes: Vec<StaticNode>,
    edges: Vec<StaticEdge>,
    stores: Vec<PersistentStoreNode>,
    store_depths: Vec<u32>,
    evidence: Vec<EdgeEvidence>,
    planned_helpers: Vec<PlannedHelperKey>,
    semantic_sources: Vec<SemanticSourceSeed>,
    semantic_material: SemanticMaterialArena,
    semantic: SemanticPlane,
    /// `RT-FNSPLIT-B2R` — one representation/call-ABI descriptor per function
    /// unit in `semantic`'s validated owner partition.
    ///
    /// ⛔ **Inert.** This plane is *declared and validated*, never emitted from.
    /// It carries no `FunctionBuilder`, no `define_function`, no call edge and no
    /// encoder; `RT-FNSPLIT-B2F` performs the atomic switch-over that makes it
    /// live.
    abi: AbiPlane,
    /// The scheduling entry returned by the root visit. Kept separately from
    /// the root occurrence because computational matches make them differ.
    root_entry: Option<StaticNodeId>,
    root_ingress: AbiRootIngress,
    /// The **occurrence** origin of the whole program's root, stored at planning
    /// time.
    ///
    /// ⛔ It is not recoverable from `entries`, which holds scheduling entries: a
    /// root whose body is a `ComputationalMatch` schedules its scrutinee while
    /// its occurrence lives on the resume (D9/AC-15). Deriving one from the other
    /// afterwards is the conflation this field exists to prevent.
    root_occurrence: Option<StaticOriginId>,
    /// The **occurrence** origin of each transparent declaration, keyed by its
    /// symbol — likewise stored at planning time, not recovered from an entry.
    ///
    /// A declaration is planned as its own source occurrence, so its body's
    /// static name is reachable **by name** and needs no origin threaded into it.
    /// This is what makes `Lowered::DeclarationClosure`'s construction site
    /// asymmetric with the two `lower_expr` closure arms
    ///.
    declaration_occurrences: BTreeMap<String, StaticOriginId>,
    /// Each transparent declaration's SCHEDULING ENTRY, by the same symbol key.
    ///
    /// Retained so `declaration_occurrences` can be checked against the pairing
    /// authority **per symbol**. Without the association, a validator can only
    /// ask whether a recorded occurrence was issued to some entry, and a swap
    /// between two declarations survives that question untouched.
    declaration_entries: BTreeMap<String, StaticNodeId>,
    /// Which of the two lawful target classes each `DeclarationRef`
    /// occurrence's `DeclarationCall` edge resolved to, keyed by the
    /// **reference** occurrence.
    ///
    /// ⭐ `RT-DECL-CLOSURE-PORT` `D4`: the planner decides this once, in
    /// `connect_declaration_calls`, and records the decision beside the edge it
    /// made. ⛔ It is deliberately **not** erased into a single "declared unit"
    /// notion: a zero-input scheduling entry and a declaration-owned callable
    /// unit are both reached by `EdgeKind::DeclarationCall`, and an empty input
    /// slice is type-correct-looking against either. Recording the class is
    /// what lets the consumer refuse the empty call at a callable target
    /// instead of emitting a wrong-arity one.
    declaration_call_targets: BTreeMap<StaticOriginId, DeclarationCallTargetClass>,
    /// Exact trap values interned during the same occurrence visit that records
    /// their source semantics. Identity zero is reserved for "no trap".
    trap_catalog: Vec<RuntimeTrap>,
    /// Every planned source occurrence, **dense by origin ordinal**.
    ///
    /// `origin_of` is `StaticOriginId(node.0)`, so a node's origin *is* its index
    /// here. The table is written in the same visit that allocates the node's
    /// semantic seed — `expression_seed`, the one function every occurrence's
    /// seed passes through — so there is no second walk that could disagree with
    /// the first, and totality is a property of the construction rather than of
    /// an enumeration someone has to keep current.
    ///
    /// ⛔ `None` is a real answer, not a gap to paper over: a control node is a
    /// planned node with no source term, so its slot stays empty and a lookup on
    /// one is a **loud planner failure** rather than a substituted body.
    source_occurrences: Vec<Option<PlannedOccurrence<'src>>>,
    /// The closed result contract for every source occurrence that can create a
    /// lowering join.  Absence is meaningful for non-join occurrences.
    join_results: Vec<Option<PlannedJoinResult>>,
    /// `RT-CONTSPEC-SUBSTRATE` `D1`. Computed and validated before ABI planning;
    /// no lowering accessor exists in this slice.
    case_emissions: Vec<PlannedCaseEmission>,
    /// `RT-CONTSPEC-SUBSTRATE` `D2`. Likewise dormant until a later planner
    /// slice explicitly widens a capability over this private population.
    occurrence_authorities: Vec<PlannedOccurrenceAuthority>,
    /// `RT-CONTSPEC-PLANNER` Slice 1. The planner computes and closes these
    /// facts, but no lowering accessor exists until the activation slice.
    continuation_specializations: Vec<PlannedContinuationSpecialization>,
    continuation_specialization_calls: Vec<PlannedContinuationSpecializationCall>,
    /// One independently validated consumer-level occurrence per continuation
    /// call whose discovery established the relation. Keyed by the whole opaque
    /// call identity, never by specialization identity or function provenance.
    required_consumer_projections:
        BTreeMap<ContinuationCallIdentity, RequiredConsumerProjection>,
    /// `RT-DECL-CLOSURE-PORT` `D5a`. The generated producer execution contexts,
    /// derived after the specialization fixed point closes.
    continuation_contexts: Vec<PlannedContinuationContext>,
    /// `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f`. The interned fusion identity
    /// plane, **installed after planning rather than during it**.
    ///
    /// **Empty on every plan the planner returns, and that is not a defect**: a
    /// fusion's identity is a function of this plan *and* the oriented plan, and
    /// the planner holds only the first. [`Self::install_static_continuation_`]
    /// [`fusions`] is the one writer, and it writes this field and the ABI
    /// arena together so the two-sided identity join below cannot see one
    /// without the other.
    static_continuation_fusions: StaticContinuationFusionPlan,
    /// `D2f` — producer bodies lowered inside a fused definition rather than by
    /// a standalone unit. Empty until `install_fusion_owned_bodies` moves a
    /// fully validated scratch map in; there is deliberately no other writer.
    fusion_owned_bodies: BTreeMap<StaticOriginId, FusionOwnedBody>,
    /// **`D3` — the exact call edges an installed fusion COMPOSES, one record
    /// per edge, keyed by the call's whole opaque identity.**
    ///
    /// Ruled at `evt_1t3f4e8100rb5`. A composed edge lowers its target's
    /// selected body in the caller and hands the result straight to the
    /// caller's already-active computational eliminator; it emits no call and
    /// returns no SSA word.
    ///
    /// **Keyed by identity, never by target, body, origin, owner or spelling.**
    /// The injective call-target law makes each target's liveness the outcome of
    /// its own unique identity, so this map is exactly "which edges compose" and
    /// nothing has to scan an incoming population to find out.
    fusion_composed_calls: BTreeMap<ContinuationCallIdentity, FusionComposedEdge>,
    /// **`D3` — `R`.** The fusion-owned outer realizations, keyed by the exact
    /// planned identity each owned body realized. Disjoint from
    /// `fusion_composed_calls` by a live preflight refusal, not by construction:
    /// the two maps are built by two selectors and could name one identity.
    fusion_outer_realizations: BTreeMap<ContinuationCallIdentity, FusionOwnedOuterRealization>,
    /// Whether body ownership has been installed. **Not derivable from the map's
    /// emptiness:** a plan with no fused regions installs an empty map, and a
    /// second install against it must still refuse.
    fusion_bodies_installed: bool,
    /// `RT-DECL-CLOSURE-PORT` `D7`. One ownership record per aggregate producer
    /// occurrence. ⭐ Unlike its two dormant siblings above, this population
    /// HAS a lowering accessor — the allocation lane is unreadable at the
    /// producer without it.
    aggregate_ownership: Vec<PlannedAggregateOwnership>,
    /// `RT-DECL-CLOSURE-PORT` `D7`. One record per capability/argument seat of
    /// every admitted host effect occurrence. Read by lowering, which claims
    /// exactly one of these per seat it consumes.
    host_effect_seats: Vec<PlannedEffectSeat>,
}

fn planner_error(detail: impl Into<String>) -> CraneliftBackendError {
    backend(BackendFailure::PlannerInvariant(detail.into()))
}

fn planner_capacity_error(detail: impl Into<String>) -> CraneliftBackendError {
    unsupported("NativeStaticTransitionPlanner", detail)
}

fn runtime_value_lifetime(value: &crate::RuntimeValue) -> PlannedReferentLifetime {
    use crate::RuntimeValue;
    match value {
        RuntimeValue::Bool(_)
        | RuntimeValue::Int(_)
        | RuntimeValue::Bytes(_)
        | RuntimeValue::String(_) => PlannedReferentLifetime::Persistent,
        RuntimeValue::Constructor { args, .. } => args
            .iter()
            .fold(PlannedReferentLifetime::Persistent, |lifetime, value| {
                lifetime.max(runtime_value_lifetime(value))
            }),
        RuntimeValue::Record { fields } => fields.iter().fold(
            PlannedReferentLifetime::Persistent,
            |lifetime, (_, value)| lifetime.max(runtime_value_lifetime(value)),
        ),
        RuntimeValue::ClosureRef { .. } | RuntimeValue::Unknown => {
            PlannedReferentLifetime::ActivationOwned
        }
    }
}






























/// Every emission owner under which one effect seat's body may be lowered.
///
/// A seat is always emitted by its own predeclared unit. It is ALSO emitted
/// inside every generated specialization context whose selected worker body
/// contains it — that is the `D5a` case, and those two emissions are different
/// occurrences of the same static seat.
///
/// Both halves are enumerated so neither needs a default. A seat reached under
/// an owner this misses has no record and refuses loudly at its allocation,
/// which is the fail-closed direction.
fn synthesized_seat_emission_owners(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
) -> Result<Vec<ContinuationEmissionOwner>, CraneliftBackendError> {
    let mut owners = Vec::new();
    if let Some(predeclared) = plan.semantic.function_owner(seat)? {
        owners.push(ContinuationEmissionOwner::Predeclared(predeclared));
    }
    for context in &plan.continuation_contexts {
        if occurrence_subtree_contains(plan, context.worker_body_origin, seat)? {
            owners.push(ContinuationEmissionOwner::Specialization(
                context.enclosing_specialization,
            ));
        }
    }
    owners.sort();
    owners.dedup();
    Ok(owners)
}

// **`RT-FNSPLIT-B2A-S` `AC-4` — the route counters.**
//
// ⛔⛔ **These exist because the instrument that used to carry `AC-4` cannot
// carry it through `B2F`.** That instrument reads this file's source text and
// asserts a list of exported signatures; it constrains the *identifier*
// `source_occurrence` and says nothing about **who calls the route** — and
// `B2F` `S6` widens `Lowering::retained_body_occurrence` from private-to-`core`
// to all of `lowering` so a unit body can resolve its own origin. ⚠ A
// source-text oracle also reddens on a reflow that changes nothing about how
// any program behaves, which is why the replacement is a behavioural one.
//
// ⭐ **The property is a RATIO, not a count, and that is what makes it durable.**
// `retained_body_occurrence` calls [`StaticTransitionPlan::source_occurrence`]
// exactly once, so the two counters move together **for as long as that route
// is the only caller**. Any second call site — a convenience resolver, a
// "just this once" direct call from an emission site — makes resolutions
// exceed route invocations, and nothing else can.
//
// ⚠ **Deliberately NOT a bound on how many times the route is used.** Seven
// consumption sites call it today and more may; `AC-4` holds the number of
// **routes** at one, never the number of resolutions. ⛔ A pin that froze the
// call count would go red on legitimate work and would be a snapshot wearing an
// invariant's name.
#[cfg(test)]
thread_local! {
    /// Resolutions performed by `source_occurrence`, since the last window open.
    static AC4_RESOLUTIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Invocations of the single route, since the last window open.
    static AC4_ROUTE_INVOCATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Zero both counters. ⛔ Call this immediately before the compile under
/// measurement: without a per-window reset a reading cannot distinguish this
/// compile's resolutions from an earlier one's, and a stale equal pair reads
/// exactly like the outcome the pin wants.
#[cfg(test)]
pub(in crate::cranelift_backend) fn ac4_open_route_window() {
    AC4_RESOLUTIONS.with(|cell| cell.set(0));
    AC4_ROUTE_INVOCATIONS.with(|cell| cell.set(0));
}

/// Record one invocation of the single `origin -> expression` route.
///
/// ⚠ Called by `Lowering::retained_body_occurrence` and by nothing else — that
/// is the whole point. A second route that recorded itself here would be
/// *claiming* to be the single route, which is a visible lie rather than a
/// silent one.
#[cfg(test)]
pub(in crate::cranelift_backend) fn ac4_note_route_invocation() {
    AC4_ROUTE_INVOCATIONS.with(|cell| cell.set(cell.get() + 1));
}

/// `(resolutions, route invocations)` since the window opened.
#[cfg(test)]
pub(in crate::cranelift_backend) fn ac4_route_counts() -> (usize, usize) {
    (
        AC4_RESOLUTIONS.with(std::cell::Cell::get),
        AC4_ROUTE_INVOCATIONS.with(std::cell::Cell::get),
    )
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn plan_static_transition_graph<'src>(
    entry: &'src RuntimeExpr,
    declarations: &BTreeMap<&str, &'src RuntimeDeclaration>,
) -> Result<StaticTransitionPlan<'src>, CraneliftBackendError> {
    // The legacy direct-lowering fixtures exercise the retained authority and
    // do not install a UnitBundle. Production passes its selected authority at
    // the call site; D8's functionized controls do the same explicitly.
    plan_static_transition_graph_with_symbols(
        entry,
        declarations,
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        false,
    )
}

pub(in crate::cranelift_backend) fn plan_static_transition_graph_with_symbols<'src>(
    entry: &'src RuntimeExpr,
    declarations: &BTreeMap<&str, &'src RuntimeDeclaration>,
    symbols: &crate::NativeProcessSymbols,
    root_ingress: AbiRootIngress,
    functionized_units: bool,
) -> Result<StaticTransitionPlan<'src>, CraneliftBackendError> {
    #[cfg(test)]
    reset_recursive_lowering_frame_count();
    let mut planner = Planner::new()?;
    let empty = PersistentNodeId(0);
    let context = PlanContext {
        environment: empty,
        continuation: empty,
        path: empty,
        cleanup: empty,
        affine: empty,
        source_return: empty,
    };
    // D9/AC-15: `entries` keeps the SCHEDULING entry; the occurrence is stored
    // separately, from the same visit. For a root or declaration body that is a
    // `ComputationalMatch` these are different nodes, and that case is the
    // required discriminator.
    let root = planner.plan_expr(entry, context, planner.terminal, EdgeKind::Continue, 0)?;
    planner.plan.register_scheduling_entry(root);
    planner.plan.root_entry = Some(root.entry);
    planner.plan.root_occurrence = Some(root.occurrence);
    let mut declaration_entries = BTreeMap::new();
    for (symbol, declaration) in declarations {
        if let RuntimeDeclarationKind::Transparent { body } = &declaration.kind {
            let planned =
                planner.plan_expr(body, context, planner.terminal, EdgeKind::Continue, 0)?;
            planner.plan.register_scheduling_entry(planned);
            // A declaration body is its own planned source occurrence, so its
            // occurrence origin is reachable by name. Two occurrences under one
            // symbol would make that lookup ambiguous, which is a planner bug
            // rather than an input condition.
            if planner
                .plan
                .declaration_occurrences
                .insert((*symbol).to_owned(), planned.occurrence)
                .is_some()
            {
                return Err(planner_error(
                    "transparent declaration planned more than one occurrence origin",
                ));
            }
            if declaration_entries
                .insert((*symbol).to_owned(), planned.entry)
                .is_some()
            {
                return Err(planner_error(
                    "transparent declaration planned more than one scheduling entry",
                ));
            }
            // Retained on the plan so the occurrence projection can be checked
            // against the pairing authority under this symbol, rather than
            // merely shown to be issued to somebody.
            planner
                .plan
                .declaration_entries
                .insert((*symbol).to_owned(), planned.entry);
        }
    }
    planner.connect_declaration_calls(&declaration_entries)?;
    planner.finish(symbols, root_ingress, functionized_units)
}

/// The governed nested-bracket source shared by the planning and emission
/// controls. Keeping one constructor prevents the emission gate from silently
/// measuring a trap-free or non-recursive surrogate.
#[cfg(test)]
pub(in crate::cranelift_backend) fn governed_nested_resource_bracket(depth: usize) -> RuntimeExpr {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum BinderRole {
        AllocatedBuffer,
        ScopeArgument,
        InductionHypothesis,
        RecursiveResult,
    }

    #[derive(Clone, Debug, Default)]
    struct BinderScope(Vec<BinderRole>);

    impl BinderScope {
        fn bind(&self, role: BinderRole) -> Self {
            let mut roles = self.0.clone();
            roles.push(role);
            Self(roles)
        }

        fn var(&self, role: BinderRole) -> RuntimeExpr {
            let index = self
                .0
                .iter()
                .rev()
                .position(|candidate| *candidate == role)
                .unwrap_or_else(|| panic!("governed bracket role {role:?} is not in scope"));
            RuntimeExpr::Var(
                u32::try_from(index).expect("governed bracket binder depth fits RuntimeExpr::Var"),
            )
        }
    }

    pub(in crate::cranelift_backend) fn trap(message: &str) -> crate::RuntimeTrap {
        crate::RuntimeTrap {
            code: crate::RuntimeTrapCode::PatternMatchFailure,
            message: message.to_string(),
        }
    }

    fn unit() -> RuntimeExpr {
        RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }
    }

    if depth == 0 {
        return unit();
    }
    let recursive_body = governed_nested_resource_bracket(depth - 1);
    let closure_scope = BinderScope::default().bind(BinderRole::AllocatedBuffer);
    let release_scope = closure_scope.bind(BinderRole::RecursiveResult);
    let release = RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Effect {
            family: "FS".to_string(),
            operation: ken_host::HostOpV1::BufferFreeze,
            capability: None,
            args: vec![
                release_scope.var(BinderRole::AllocatedBuffer),
                RuntimeExpr::Value(crate::RuntimeValue::Int(0.into())),
                RuntimeExpr::Value(crate::RuntimeValue::Int(1.into())),
                release_scope.var(BinderRole::AllocatedBuffer),
            ],
        }),
        cases: vec![
            crate::RuntimeMatchCase {
                constructor: "ctor:prelude::Result::Err".to_string(),
                binders: 1,
                body: RuntimeExpr::Trap(trap("release failed")),
            },
            crate::RuntimeMatchCase {
                constructor: "ctor:prelude::Result::Ok".to_string(),
                binders: 1,
                body: unit(),
            },
        ],
        default: trap("release result"),
    };
    let closure_body = RuntimeExpr::Let {
        value: Box::new(recursive_body),
        body: Box::new(release),
    };
    let allocation_scope = BinderScope::default().bind(BinderRole::AllocatedBuffer);
    let bracket_case_scope = allocation_scope
        .bind(BinderRole::ScopeArgument)
        .bind(BinderRole::InductionHypothesis);
    let bracket = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::Bracket::Scope".to_string(),
            args: vec![RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["buffer".to_string()],
                body: Box::new(closure_body),
            }],
        }),
        cases: vec![crate::RuntimeComputationalMatchCase {
            constructor: "ctor:fixture::Bracket::Scope".to_string(),
            argument_binders: 1,
            recursive_positions: vec![0],
            body: RuntimeExpr::Call {
                callee: Box::new(bracket_case_scope.var(BinderRole::InductionHypothesis)),
                args: vec![bracket_case_scope.var(BinderRole::AllocatedBuffer)],
            },
        }],
        default: trap("bracket scope"),
    };
    RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Effect {
            family: "FS".to_string(),
            operation: ken_host::HostOpV1::BufferAllocate,
            capability: None,
            args: vec![RuntimeExpr::Value(crate::RuntimeValue::Int(1.into()))],
        }),
        cases: vec![
            crate::RuntimeMatchCase {
                constructor: "ctor:prelude::Result::Err".to_string(),
                binders: 1,
                body: RuntimeExpr::Trap(trap("allocate failed")),
            },
            crate::RuntimeMatchCase {
                constructor: "ctor:prelude::Result::Ok".to_string(),
                binders: 1,
                body: bracket,
            },
        ],
        default: trap("allocate result"),
    }
}

#[cfg(test)]
pub(in crate::cranelift_backend) use tests::contspec_nested_fixture;

/// `D2f` Deliverable 0 — the shared checked-witness fixture, re-exported so the
/// full-compile gate consumes the very constructor the planner controls do.
#[cfg(test)]
pub(in crate::cranelift_backend) use tests::{
    d2j_checked_fixture_under, d2j_installed_plan_under, D2jCause, D2J_DECLARATION,
};

#[cfg(test)]
mod tests {
    use super::abi::{AbiCarrier, AbiSlot, AbiSlotKind};
    /// `D2g` — the `R3` shape, in an unmarked and a checked-transport form.
    ///
    /// One builder for both, so the two differ in **transport and nothing
    /// else**. A separately authored twin could differ in shape as well, and
    /// then a relation that survived would not be evidence about transport.
    ///
    /// This is a DECLARATION body. The landed validator refuses a recursive/IH
    /// marker that escapes its declaration into the entry expression, so a
    /// checked fixture living in the entry can never be positively validated --
    /// the entry only references this.
    #[cfg(test)]
    pub(super) fn d2g_declaration_body(checked: bool) -> RuntimeExpr {
        d2g_declaration_body_relocated(checked, false)
    }

    /// The same body, optionally with the OUTER SLOT WRAPPER MOVED to the
    /// sibling case.
    ///
    /// This is the runtime-only mutation `AC-2` needs. The plan stays
    /// byte-for-byte fixed and the ARTIFACT changes: the outer slot marker
    /// stops wrapping the selected case body and wraps the sibling `OutLeaf`
    /// case instead -- a real case body, so nothing is malformed. The
    /// invocation marker stays on the consuming `Call` where it always was.
    ///
    /// Mutating the plan instead would only show that the validator notices
    /// when its own description is edited. Moving the marker in the Runtime IR
    /// is what shows it detects a change in the thing described.
    #[cfg(test)]
    pub(super) fn d2g_declaration_body_relocated(checked: bool, relocate_outer_slot: bool) -> RuntimeExpr {
        let trap = |what: &str| RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: format!("D2g {what} default"),
        };
        let unit = || RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        };
        // Frame wrappers and slot/invocation markers are applied ONLY in the
        // checked form; the unmarked form is the same tree without them.
        let frame = |id: u64, body: RuntimeExpr| {
            if checked {
                RuntimeExpr::CheckedSubcontinuationFrame {
                    frame_id: id,
                    body: Box::new(body),
                }
            } else {
                body
            }
        };
        let slots = |slot: u64, path: Vec<u64>, body: RuntimeExpr| {
            if checked {
                RuntimeExpr::CheckedComputationalIHSlots {
                    slot_template_ids: vec![slot],
                    checked_occurrence_paths: vec![path],
                    body: Box::new(body),
                }
            } else {
                body
            }
        };
        let invocation = |call: u64, path: Vec<u64>, body: RuntimeExpr| {
            if checked {
                RuntimeExpr::CheckedComputationalIHInvocation {
                    call_template_id: call,
                    checked_occurrence_path: path,
                    body: Box::new(body),
                }
            } else {
                body
            }
        };

        let inner = frame(
            1,
            RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::D2gIn::Node".to_string(),
                    args: vec![RuntimeExpr::LexicalClosure {
                        captures: Vec::new(),
                        params: vec!["unit".to_string()],
                        body: Box::new(RuntimeExpr::Construct {
                            constructor: "ctor:fixture::D2gIn::Leaf".to_string(),
                            args: Vec::new(),
                        }),
                    }],
                }),
                cases: vec![
                    RuntimeComputationalMatchCase {
                        constructor: "ctor:fixture::D2gIn::Node".to_string(),
                        argument_binders: 1,
                        recursive_positions: vec![0],
                        // THE PRODUCER: the hypothesis lands in field 0.
                        body: slots(
                            D2G_INNER_SLOT,
                            vec![20, 1],
                            RuntimeExpr::Construct {
                                constructor: "ctor:fixture::D2gOut::Node".to_string(),
                                args: vec![RuntimeExpr::Var(0)],
                            },
                        ),
                    },
                    RuntimeComputationalMatchCase {
                        constructor: "ctor:fixture::D2gIn::Leaf".to_string(),
                        argument_binders: 0,
                        recursive_positions: Vec::new(),
                        body: RuntimeExpr::Construct {
                            constructor: "ctor:fixture::D2gOut::Leaf".to_string(),
                            args: Vec::new(),
                        },
                    },
                ],
                default: trap("inner"),
            },
        );

        frame(
            0,
            RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::D2gOut::Node".to_string(),
                    args: vec![RuntimeExpr::LexicalClosure {
                        captures: Vec::new(),
                        params: vec!["unit".to_string()],
                        body: Box::new(inner),
                    }],
                }),
                cases: vec![
                    RuntimeComputationalMatchCase {
                        constructor: "ctor:fixture::D2gOut::Node".to_string(),
                        argument_binders: 1,
                        recursive_positions: vec![0],
                        // THE IH-CONSUMING CALL, under its slot and invocation
                        // markers in the checked form.
                        body: {
                            let consuming = invocation(
                                D2G_CALL,
                                vec![30, 0],
                                RuntimeExpr::Call {
                                    callee: Box::new(RuntimeExpr::Var(0)),
                                    args: vec![unit()],
                                },
                            );
                            if relocate_outer_slot {
                                consuming
                            } else {
                                slots(D2G_OUTER_SLOT, vec![20, 0], consuming)
                            }
                        },
                    },
                    RuntimeComputationalMatchCase {
                        constructor: "ctor:fixture::D2gOut::Leaf".to_string(),
                        argument_binders: 0,
                        recursive_positions: Vec::new(),
                        body: {
                            let ok = RuntimeExpr::Construct {
                                constructor: "ctor:prelude::Result::Ok".to_string(),
                                args: vec![unit()],
                            };
                            if relocate_outer_slot {
                                slots(D2G_OUTER_SLOT, vec![20, 0], ok)
                            } else {
                                ok
                            }
                        },
                    },
                ],
                default: trap("outer"),
            },
        )
    }


    pub(super) const D2G_DECLARATION: &str = "decl:fixture::d2g";
    pub(super) const D2G_OUTER_FRAME: u64 = 0;
    pub(super) const D2G_INNER_FRAME: u64 = 1;
    pub(super) const D2G_OUTER_SLOT: u64 = 200;
    pub(super) const D2G_INNER_SLOT: u64 = 201;
    pub(super) const D2G_CALL: u64 = 100;
    /// The constructor each slot's own frame eliminates. Two different facts,
    /// and the validator cannot tell them apart -- so they are pinned.
    pub(super) const D2G_OUTER_SLOT_CONSTRUCTOR: &str = "ctor:fixture::D2gOut::Node";
    pub(super) const D2G_INNER_SLOT_CONSTRUCTOR: &str = "ctor:fixture::D2gIn::Node";

    /// The marker locations, DERIVED BY HAND from the collector's documented
    /// edge convention and this fixture's structure -- never read back out of
    /// the collector.
    ///
    /// Feeding collected locations into the plan would make the positive
    /// validation compare the collector with itself. That is the
    /// manufactured-evidence form the frame forbids, and it is why an earlier
    /// revision of this deliverable was blocked.
    ///
    /// The convention, from `collect_checked_oriented_markers`: a checked
    /// wrapper descends its body at edge `0`; `ComputationalMatch` takes its
    /// scrutinee at `0` and case *i*'s body at `1 + i`; `Construct` takes
    /// argument *i* at `i`; `LexicalClosure` takes its body at `3`.
    ///
    /// ```text
    /// []                    frame 0 wrapper
    /// [0]                     outer ComputationalMatch
    /// [0, 0]                    Construct D2gOut::Node
    /// [0, 0, 0]                   LexicalClosure
    /// [0, 0, 0, 3]                  frame 1 wrapper
    /// [0, 0, 0, 3, 0]                 inner ComputationalMatch
    /// [0, 0, 0, 3, 0, 1]                slot 201   <- inner slot marker
    /// [0, 1]                    slot 200           <- outer slot marker
    /// [0, 1, 0]                   call 100         <- invocation marker
    /// ```
    #[cfg(test)]
    pub(super) fn d2g_outer_slot_location() -> Vec<u64> {
        vec![0, 1]
    }
    #[cfg(test)]
    pub(super) fn d2g_inner_slot_location() -> Vec<u64> {
        vec![0, 0, 0, 3, 0, 1]
    }
    #[cfg(test)]
    pub(super) fn d2g_call_location() -> Vec<u64> {
        vec![0, 1, 0]
    }

    #[cfg(test)]
    pub(super) fn d2g_declaration(checked: bool) -> RuntimeDeclaration {
        RuntimeDeclaration {
            symbol: D2G_DECLARATION.to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: d2g_declaration_body(checked),
            },
            metadata: crate::RuntimeSymbolMetadata {
                obligations: Default::default(),
                obligation_metadata: Default::default(),
                assumptions: Default::default(),
                assumption_trust_metadata: Default::default(),
                trusted_base_delta: Default::default(),
                lowerability: None,
                unsupported: None,
                runtime_checks: Default::default(),
                capabilities: Default::default(),
                effects: Default::default(),
            },
        }
    }

    #[cfg(test)]
    pub(super) fn d2g_entry() -> RuntimeExpr {
        RuntimeExpr::DeclarationRef {
            symbol: D2G_DECLARATION.to_string(),
        }
    }

    #[cfg(test)]
    pub(super) fn d2g_interface(name: u8) -> crate::CheckedAnswerInterfaceV1 {
        let mut bytes = crate::CHECKED_ANSWER_INTERFACE_V1_HEADER.to_vec();
        bytes.push(name);
        crate::CheckedAnswerInterfaceV1::new(bytes).expect("interface")
    }

    /// One checked frame's fingerprint, from the frame's OWN cases and default.
    ///
    /// The fingerprint is definitionally that content's hash and there is no
    /// other way to obtain it, so computing it here is not the circularity the
    /// ruling forbids -- that prohibition is about LOCATIONS, which are
    /// hand-derived above.
    #[cfg(test)]
    pub(super) fn d2g_frame_fingerprint(body: &RuntimeExpr, frame_id: u64) -> u64 {
        fn find(expr: &RuntimeExpr, frame_id: u64) -> Option<u64> {
            match expr {
                RuntimeExpr::CheckedSubcontinuationFrame { frame_id: id, body } => {
                    if *id == frame_id {
                        if let RuntimeExpr::ComputationalMatch { cases, default, .. } =
                            body.as_ref()
                        {
                            return Some(
                                crate::compiler_private_computational_match_frame_fingerprint(
                                    cases, default,
                                ),
                            );
                        }
                    }
                    find(body, frame_id)
                }
                RuntimeExpr::CheckedComputationalIHSlots { body, .. }
                | RuntimeExpr::CheckedComputationalIHInvocation { body, .. } => find(body, frame_id),
                RuntimeExpr::ComputationalMatch {
                    scrutinee, cases, ..
                } => find(scrutinee, frame_id)
                    .or_else(|| cases.iter().find_map(|case| find(&case.body, frame_id))),
                RuntimeExpr::Construct { args, .. } => args.iter().find_map(|arg| find(arg, frame_id)),
                RuntimeExpr::LexicalClosure { body, .. } => find(body, frame_id),
                RuntimeExpr::Call { callee, args } => find(callee, frame_id)
                    .or_else(|| args.iter().find_map(|arg| find(arg, frame_id))),
                _ => None,
            }
        }
        find(body, frame_id).expect("the fixture declares this frame")
    }

    /// A complete `OrientedSubcontinuationPlanV1` for the twin, authored
    /// independently of the collector.
    #[cfg(test)]
    pub(super) fn d2g_oriented_plan() -> crate::OrientedSubcontinuationPlanV1 {
        let body = d2g_declaration_body(true);
        let location = |path: Vec<u64>| crate::CheckedRuntimeMarkerLocationV1 {
            declaration: D2G_DECLARATION.to_string(),
            runtime_path: path,
        };
        let mut frames = Vec::new();
        for (frame_id, semantic_position, parent) in [
            (D2G_OUTER_FRAME, 0u64, None),
            (D2G_INNER_FRAME, 1u64, Some(D2G_OUTER_FRAME)),
        ] {
            let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
                frame_id,
                segment_site_id: 9,
                declaration: D2G_DECLARATION.to_string(),
                checked_occurrence_path: vec![10, frame_id],
                semantic_position,
                input_interface: d2g_interface(frame_id as u8),
                output_interface: d2g_interface(frame_id as u8 + 1),
                runtime_frame_fingerprint: d2g_frame_fingerprint(&body, frame_id),
                occurrence_binding_fingerprint: 0,
                control_witness: parent.map_or(
                    crate::OrientedControlWitnessV1::DistinguishedRoot,
                    crate::OrientedControlWitnessV1::ParentFrame,
                ),
            };
            frame.occurrence_binding_fingerprint =
                crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
            frames.push(frame);
        }

        let mut computational_ih_slots = Vec::new();
        // The constructor is a PER-SLOT fact, authored from each slot's own case.
        // The outer frame eliminates `D2gOut` and the inner one eliminates
        // `D2gIn`; hardcoding one for both is a semantic mismatch the landed
        // validator cannot detect, which is why `d2g_slot_constructors` pins it.
        for (slot_template_id, frame_template_id, checked_path, marker, constructor) in [
            (
                D2G_OUTER_SLOT,
                D2G_OUTER_FRAME,
                vec![20u64, 0],
                d2g_outer_slot_location(),
                D2G_OUTER_SLOT_CONSTRUCTOR,
            ),
            (
                D2G_INNER_SLOT,
                D2G_INNER_FRAME,
                vec![20u64, 1],
                d2g_inner_slot_location(),
                D2G_INNER_SLOT_CONSTRUCTOR,
            ),
        ] {
            let mut slot = crate::CheckedComputationalIHSlotTemplateV1 {
                slot_template_id,
                declaration: D2G_DECLARATION.to_string(),
                checked_match_ordinal: frame_template_id,
                checked_occurrence_path: checked_path,
                frame_template_id,
                constructor: constructor.to_string(),
                recursive_position: 0,
                method_binder_ordinal: 0,
                local_telescope: Vec::new(),
                ih_interface: d2g_interface(frame_template_id as u8),
                segment_site_id: 9,
                frame_templates: vec![frame_template_id],
                input_interface: d2g_interface(frame_template_id as u8),
                output_interface: d2g_interface(frame_template_id as u8 + 1),
                runtime_marker_locations: vec![location(marker)],
                occurrence_binding_fingerprint: 0,
            };
            slot.occurrence_binding_fingerprint =
                crate::compiler_private_computational_ih_slot_binding_fingerprint(&slot);
            computational_ih_slots.push(slot);
        }

        let mut call = crate::CheckedComputationalIHCallTemplateV1 {
            call_template_id: D2G_CALL,
            declaration: D2G_DECLARATION.to_string(),
            checked_occurrence_path: vec![30, 0],
            slot_template_id: D2G_OUTER_SLOT,
            arity: 1,
            local_telescope: Vec::new(),
            result_interface: d2g_interface(D2G_OUTER_FRAME as u8 + 1),
            callee_segment_site_id: 9,
            callee_frame_templates: vec![D2G_OUTER_FRAME],
            composed_frame_templates: Vec::new(),
            parent_frame_template_id: Some(D2G_OUTER_FRAME),
            parent_segment_site_id: Some(9),
            caller_interface: d2g_interface(D2G_OUTER_FRAME as u8 + 1),
            runtime_marker_locations: vec![location(d2g_call_location())],
            occurrence_binding_fingerprint: 0,
        };
        call.occurrence_binding_fingerprint =
            crate::compiler_private_computational_ih_call_binding_fingerprint(&call);

        crate::OrientedSubcontinuationPlanV1 {
            representation_rule_version:
                crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
            frames,
            recursive_calls: Vec::new(),
            computational_ih_slots,
            computational_ih_calls: vec![call],
        }
    }



    #[cfg(test)]
    pub(super) fn d2h_plane_fixture() -> (
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

    // ── `D2j` — the fusion-reaching witness with a NON-EMPTY input projection ──
    //
    // The landed `D2g` twin reaches a fusion candidate but projects zero ordered
    // inputs, because its consumer's owning unit has no parameters to project.
    // This witness is that same body inside a two-parameter `LexicalClosure`, so
    // the consumer sits in a unit with an entry ABI -- one structural
    // difference, and the reason the projection becomes non-empty.
    //
    // Wrapping shifts every checked marker one edge deeper: a `LexicalClosure`
    // descends its body at edge `3`, so each `D2g` path gains a leading `3`.
    // These are hand-derived from that convention, not read back out of the
    // collector.

    pub(in crate::cranelift_backend) const D2J_DECLARATION: &str = "decl:fixture::d2j";

    /// `D2j` — the source-side causes, each a variant of ONE witness family.
    ///
    /// Five are refusal causes. `ReHomed` is the segment-owner category, which
    /// the Architect's disposition makes a provenance and non-aliasing claim
    /// rather than a sixth refusal, and `ProducerArity` is a positive widening
    /// that makes the argument row's inventory non-degenerate.
    ///
    /// A fixture per cause would be a fixture per member; this is one builder
    /// with a selector, so the family stays a single witness and the sizing stop
    /// does not fire.
    #[cfg(test)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(in crate::cranelift_backend) enum D2jCause {
        Exact,
        /// The frame marker's identity no longer matches the plan's frame.
        Frame,
        /// The selected slot template no longer matches the plan's slot.
        SelectedSlot,
        /// The invocation template no longer matches the plan's call.
        Invocation,
        /// The selected case body is no longer the IH-consuming `Call`.
        ExactSuffix,
        /// The consuming `Call` calls the ordinary child instead of the
        /// hypothesis.
        CallIdentity,
        /// The outer parameterisation is removed, which RE-HOMES the whole
        /// fusion into different units. Not a refusal cause -- the producer
        /// still sits behind the inner closure, so the split survives and a
        /// candidate is still formed. It is the segment-owner PROVENANCE and
        /// NON-ALIASING disposition instead.
        ReHomed,
        /// A second, non-recursive argument on the producer construct.
        ///
        /// The exact witness's producer construct has exactly ONE child, so
        /// "the argument is the child at the recursive position" cannot
        /// discriminate the position there -- a single-element inventory is
        /// the degenerate witness `AC-1` names. This widens it to two.
        ProducerArity,
    }

    /// The constructor the producer carries.
    ///
    /// **IT IS NOT A SELECTOR, AND USING IT AS ONE WAS A DEFECT.** Two
    /// constructs in this fixture carry this symbol: the inner case-body
    /// producer, and the OUTER computational match's scrutinee. An earlier
    /// revision of [`D2jCause::ProducerArity`] keyed the widening on the symbol
    /// and so widened both, which made the "producer-only" causal claim beside
    /// it false. The widening is keyed on STRUCTURAL POSITION instead --
    /// see [`d2j_rewrite_body`] -- and the census in the matrix test holds the
    /// other occurrence at arity one.
    #[cfg(test)]
    pub(super) const D2J_PRODUCER_CONSTRUCTOR: &str = "ctor:fixture::D2gOut::Node";

    /// The witness body under one cause. `Exact` is the reviewed witness.
    #[cfg(test)]
    pub(super) fn d2j_witness_body_under(cause: D2jCause) -> RuntimeExpr {
        let inner = d2g_declaration_body(true);
        let mutated = match cause {
            D2jCause::Exact
            | D2jCause::ReHomed
            | D2jCause::ProducerArity
            | D2jCause::ExactSuffix
            | D2jCause::CallIdentity
            | D2jCause::Frame
            | D2jCause::SelectedSlot
            | D2jCause::Invocation => d2j_rewrite_body(inner, cause, false),
        };
        if cause == D2jCause::ReHomed {
            // No wrapper: the producer stops being behind a closure, so the two
            // sides collapse toward one unit and the split the fusion exists to
            // close is gone.
            mutated
        } else {
            RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["a".to_string(), "b".to_string()],
                body: Box::new(mutated),
            }
        }
    }

    /// Apply one source-side cause to the checked body.
    ///
    /// `in_case_body` is the STRUCTURAL POSITION of `expr` relative to its
    /// nearest enclosing [`RuntimeExpr::ComputationalMatch`]: true in a case
    /// body, false in a scrutinee. It is reset at every match rather than
    /// inherited, so descending through the outer match's scrutinee into the
    /// inner match's case body arrives at the producer with it true.
    ///
    /// This exists because [`D2J_PRODUCER_CONSTRUCTOR`] does not identify the
    /// producer: the outer match's scrutinee carries the same symbol, and
    /// keying the widening on the symbol widened both.
    #[cfg(test)]
    pub(super) fn d2j_rewrite_body(expr: RuntimeExpr, cause: D2jCause, in_case_body: bool) -> RuntimeExpr {
        match expr {
            RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body } => {
                let frame_id = if cause == D2jCause::Frame && frame_id == D2G_OUTER_FRAME {
                    frame_id + 90
                } else {
                    frame_id
                };
                RuntimeExpr::CheckedSubcontinuationFrame {
                    frame_id,
                    body: Box::new(d2j_rewrite_body(*body, cause, in_case_body)),
                }
            }
            RuntimeExpr::CheckedComputationalIHSlots {
                slot_template_ids,
                checked_occurrence_paths,
                body,
            } => RuntimeExpr::CheckedComputationalIHSlots {
                slot_template_ids: slot_template_ids
                    .into_iter()
                    .map(|id| {
                        if cause == D2jCause::SelectedSlot && id == D2G_OUTER_SLOT {
                            id + 90
                        } else {
                            id
                        }
                    })
                    .collect(),
                checked_occurrence_paths,
                body: Box::new(d2j_rewrite_body(*body, cause, in_case_body)),
            },
            RuntimeExpr::CheckedComputationalIHInvocation {
                call_template_id,
                checked_occurrence_path,
                body,
            } => RuntimeExpr::CheckedComputationalIHInvocation {
                call_template_id: if cause == D2jCause::Invocation {
                    call_template_id + 90
                } else {
                    call_template_id
                },
                checked_occurrence_path,
                body: Box::new(d2j_rewrite_body(*body, cause, in_case_body)),
            },
            RuntimeExpr::ComputationalMatch {
                scrutinee,
                cases,
                default,
            } => RuntimeExpr::ComputationalMatch {
                // THE POSITION IS RESET HERE, NOT INHERITED.
                scrutinee: Box::new(d2j_rewrite_body(*scrutinee, cause, false)),
                cases: cases
                    .into_iter()
                    .map(|case| crate::RuntimeComputationalMatchCase {
                        body: d2j_rewrite_body(case.body, cause, true),
                        ..case
                    })
                    .collect(),
                default,
            },
            RuntimeExpr::Construct { constructor, args } => {
                let mut args: Vec<RuntimeExpr> = args
                    .into_iter()
                    .map(|arg| d2j_rewrite_body(arg, cause, in_case_body))
                    .collect();
                // A second child on the CASE-BODY producer only. It is a nullary
                // constructor, so it adds no result-position origin and no
                // marker edge -- the one thing it changes is the size of the
                // inventory the argument row selects from.
                //
                // The symbol alone does not select it. The outer match's
                // scrutinee carries the same one and must stay at arity one,
                // which the census in the matrix test asserts.
                if cause == D2jCause::ProducerArity
                    && in_case_body
                    && constructor == D2J_PRODUCER_CONSTRUCTOR
                {
                    args.push(RuntimeExpr::Construct {
                        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                        args: Vec::new(),
                    });
                }
                RuntimeExpr::Construct { constructor, args }
            }
            RuntimeExpr::LexicalClosure {
                captures,
                params,
                body,
            } => RuntimeExpr::LexicalClosure {
                captures,
                params,
                body: Box::new(d2j_rewrite_body(*body, cause, in_case_body)),
            },
            RuntimeExpr::Call { callee, args } => match cause {
                // The selected case body stops being the consuming Call.
                D2jCause::ExactSuffix => RuntimeExpr::Construct {
                    constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                    args: Vec::new(),
                },
                // The Call remains, and calls the ordinary child binder instead
                // of the hypothesis.
                D2jCause::CallIdentity => RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(1)),
                    args,
                },
                _ => RuntimeExpr::Call { callee, args },
            },
            other => other,
        }
    }

    #[cfg(test)]
    pub(super) fn d2j_witness_body(checked: bool) -> RuntimeExpr {
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["a".to_string(), "b".to_string()],
            body: Box::new(d2g_declaration_body(checked)),
        }
    }

    /// The marker locations for one cause's actual shape.
    ///
    /// [`D2jCause::ReHomed`] removes the wrapper, so its markers sit one edge
    /// shallower. Giving it a plan that matches means what it measures is
    /// attributable to the re-home rather than to marker paths having moved --
    /// which is what it refused on before, and would have made the row evidence
    /// about the wrong thing.
    #[cfg(test)]
    pub(super) fn d2j_prefixed_under(cause: D2jCause, path: Vec<u64>) -> Vec<u64> {
        if cause == D2jCause::ReHomed {
            return path;
        }
        let mut prefixed = vec![3];
        prefixed.extend(path);
        prefixed
    }

    #[cfg(test)]
    pub(super) fn d2j_declaration(checked: bool) -> RuntimeDeclaration {
        RuntimeDeclaration {
            symbol: D2J_DECLARATION.to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: d2j_witness_body(checked),
            },
            metadata: crate::RuntimeSymbolMetadata {
                obligations: Default::default(),
                obligation_metadata: Default::default(),
                assumptions: Default::default(),
                assumption_trust_metadata: Default::default(),
                trusted_base_delta: Default::default(),
                lowerability: None,
                unsupported: None,
                runtime_checks: Default::default(),
                capabilities: Default::default(),
                effects: Default::default(),
            },
        }
    }

    #[cfg(test)]
    /// The entry for one cause. **The root family is PER CAUSE** — Architect
    /// `evt_4trsqtkxtghjx`.
    ///
    /// Every cause that keeps the declaration's `params: ["a", "b"]` gets the
    /// **ABI-applied** root, because the governing invariant is one end-to-end
    /// *program*: the complete key carries positional and provenance facts
    /// derived from the planned program, and a bare `DeclarationRef` stops at
    /// `Unsupported(Closure)` in root projection, so it cannot state the
    /// emission contract at all.
    ///
    /// **`ReHomed` is the explicit exception, and it is not a special case
    /// of convenience.** That cause *removes* the outer `LexicalClosure`, so the
    /// re-homed program has ZERO ABI inputs. Applying two `Unit`s to it builds
    /// an **ill-typed program**, and the `Unsupported(Call, "callee is not a
    /// closure")` that follows would be evidence about this harness rather than
    /// about fusion.
    ///
    /// **The branch is on the cause, never on the mutated body.** Inferring the
    /// root by inspecting the source would let the entry contract drift silently
    /// along with a malformed mutation; an explicit arm keeps it reviewable.
    /// Every arm is spelled out so a new cause is a compile error here rather
    /// than an inherited default.
    #[cfg(test)]
    pub(super) fn d2j_entry_under(cause: D2jCause) -> RuntimeExpr {
        let bare = || RuntimeExpr::DeclarationRef {
            symbol: D2J_DECLARATION.to_string(),
        };
        let applied = || RuntimeExpr::Call {
            callee: Box::new(bare()),
            args: vec![
                RuntimeExpr::Construct {
                    constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                    args: Vec::new(),
                },
                RuntimeExpr::Construct {
                    constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                    args: Vec::new(),
                },
            ],
        };
        match cause {
            D2jCause::ReHomed => bare(),
            D2jCause::Exact
            | D2jCause::Frame
            | D2jCause::SelectedSlot
            | D2jCause::Invocation
            | D2jCause::ExactSuffix
            | D2jCause::CallIdentity
            | D2jCause::ProducerArity => applied(),
        }
    }

    #[cfg(test)]
    pub(super) fn d2j_oriented_plan() -> crate::OrientedSubcontinuationPlanV1 {
        d2j_oriented_plan_under(D2jCause::Exact)
    }

    #[cfg(test)]
    pub(super) fn d2j_oriented_plan_under(cause: D2jCause) -> crate::OrientedSubcontinuationPlanV1 {
        // For a REFUSAL cause the fingerprints come from the EXACT body. The
        // plan is the correct description; the cause mutates the SOURCE and the
        // two then disagree. Deriving them from the mutated body would move the
        // description along with the artifact and there would be nothing left to
        // catch.
        //
        // `ProducerArity` is the opposite case: it is a POSITIVE witness, not a
        // refusal control, so its plan must describe its own source or it would
        // refuse at transport and stop being a witness at all.
        //
        // `ReHomed` deliberately stays on the exact body. Its marker paths are
        // matched to its shape just above, and it was MEASURED to reach a
        // candidate that way; re-deriving its fingerprints would change the
        // object the measurement is about.
        let body = match cause {
            D2jCause::ProducerArity => d2j_witness_body_under(cause),
            _ => d2j_witness_body_under(D2jCause::Exact),
        };
        let location = |path: Vec<u64>| crate::CheckedRuntimeMarkerLocationV1 {
            declaration: D2J_DECLARATION.to_string(),
            runtime_path: path,
        };
        let mut frames = Vec::new();
        for (frame_id, semantic_position, parent) in [
            (D2G_OUTER_FRAME, 0u64, None),
            (D2G_INNER_FRAME, 1u64, Some(D2G_OUTER_FRAME)),
        ] {
            let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
                frame_id,
                segment_site_id: 9,
                declaration: D2J_DECLARATION.to_string(),
                checked_occurrence_path: vec![10, frame_id],
                semantic_position,
                input_interface: d2g_interface(frame_id as u8),
                output_interface: d2g_interface(frame_id as u8 + 1),
                runtime_frame_fingerprint: d2g_frame_fingerprint(&body, frame_id),
                occurrence_binding_fingerprint: 0,
                control_witness: parent.map_or(
                    crate::OrientedControlWitnessV1::DistinguishedRoot,
                    crate::OrientedControlWitnessV1::ParentFrame,
                ),
            };
            frame.occurrence_binding_fingerprint =
                crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
            frames.push(frame);
        }

        let mut computational_ih_slots = Vec::new();
        for (slot_template_id, frame_template_id, checked_path, marker, constructor) in [
            (
                D2G_OUTER_SLOT,
                D2G_OUTER_FRAME,
                vec![20u64, 0],
                d2j_prefixed_under(cause, d2g_outer_slot_location()),
                D2G_OUTER_SLOT_CONSTRUCTOR,
            ),
            (
                D2G_INNER_SLOT,
                D2G_INNER_FRAME,
                vec![20u64, 1],
                d2j_prefixed_under(cause, d2g_inner_slot_location()),
                D2G_INNER_SLOT_CONSTRUCTOR,
            ),
        ] {
            let mut slot = crate::CheckedComputationalIHSlotTemplateV1 {
                slot_template_id,
                declaration: D2J_DECLARATION.to_string(),
                checked_match_ordinal: frame_template_id,
                checked_occurrence_path: checked_path,
                frame_template_id,
                constructor: constructor.to_string(),
                recursive_position: 0,
                method_binder_ordinal: 0,
                local_telescope: Vec::new(),
                ih_interface: d2g_interface(frame_template_id as u8),
                segment_site_id: 9,
                frame_templates: vec![frame_template_id],
                input_interface: d2g_interface(frame_template_id as u8),
                output_interface: d2g_interface(frame_template_id as u8 + 1),
                runtime_marker_locations: vec![location(marker)],
                occurrence_binding_fingerprint: 0,
            };
            slot.occurrence_binding_fingerprint =
                crate::compiler_private_computational_ih_slot_binding_fingerprint(&slot);
            computational_ih_slots.push(slot);
        }

        let mut call = crate::CheckedComputationalIHCallTemplateV1 {
            call_template_id: D2G_CALL,
            declaration: D2J_DECLARATION.to_string(),
            checked_occurrence_path: vec![30, 0],
            slot_template_id: D2G_OUTER_SLOT,
            arity: 1,
            local_telescope: Vec::new(),
            result_interface: d2g_interface(D2G_OUTER_FRAME as u8 + 1),
            callee_segment_site_id: 9,
            callee_frame_templates: vec![D2G_OUTER_FRAME],
            // `RT-LEXICAL-R3-FUSION-EMITTER` `DP` — the checked source's
            // composition-time claim: when a fusion splice builds this
            // invocation's segment, the producer frame joins it, qualified by
            // the same single invocation source and affine instance.
            //
            // The ordinary sequence above is UNCHANGED, which is what keeps the
            // uncomposed segments of this same template covering exactly
            // `{outer}`. `89ee005b` widened that one instead and refused them.
            composed_frame_templates: vec![D2G_INNER_FRAME],
            parent_frame_template_id: Some(D2G_OUTER_FRAME),
            parent_segment_site_id: Some(9),
            caller_interface: d2g_interface(D2G_OUTER_FRAME as u8 + 1),
            runtime_marker_locations: vec![location(d2j_prefixed_under(cause, d2g_call_location()))],
            occurrence_binding_fingerprint: 0,
        };
        call.occurrence_binding_fingerprint =
            crate::compiler_private_computational_ih_call_binding_fingerprint(&call);

        crate::OrientedSubcontinuationPlanV1 {
            representation_rule_version:
                crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
            frames,
            recursive_calls: Vec::new(),
            computational_ih_slots,
            computational_ih_calls: vec![call],
        }
    }

    #[cfg(test)]
    pub(super) fn d2j_declaration_under(cause: D2jCause) -> RuntimeDeclaration {
        RuntimeDeclaration {
            symbol: D2J_DECLARATION.to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: d2j_witness_body_under(cause),
            },
            metadata: crate::RuntimeSymbolMetadata {
                obligations: Default::default(),
                obligation_metadata: Default::default(),
                assumptions: Default::default(),
                assumption_trust_metadata: Default::default(),
                trusted_base_delta: Default::default(),
                lowerability: None,
                unsupported: None,
                runtime_checks: Default::default(),
                capabilities: Default::default(),
                effects: Default::default(),
            },
        }
    }




    /// `D2f` Deliverable 0 — THE ONE fixture constructor, shared by the
    /// planner controls in this module and the full-compile gate in
    /// `lowering::core`'s controls.
    ///
    /// **It exists so the two cannot drift.** `D2f`'s gate has to establish
    /// that the checked witness reaches a resolved plane through the *production*
    /// compile, and that claim is only about the same witness the planner
    /// controls measure if both consume one constructor. A duplicated or
    /// re-hand-wrapped fixture would let the planner side stay green while the
    /// production side measured a different program — which is the exact defect
    /// that made the old `px8j` binding unsatisfiable.
    ///
    /// Returns the entry, the transparent declaration, and the independently
    /// authored plan **for one cause**, together, because a caller that took
    /// them from three separate calls could mix causes.
    ///
    /// **The sharing invariant is PER CAUSE** — Architect `evt_4trsqtkxtghjx`.
    /// One fixture means one *cause-aware* constructor, not one identical root
    /// across causes that deliberately change callable arity. Planner,
    /// production-gate and emitter controls for `Exact` all consume the same
    /// applied object; those for `ReHomed` all consume the same bare object.
    /// **Different causes are not required to share an outer root**, and
    /// `d2j_entry_under` is where that family is decided.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn d2j_checked_fixture_under(
        cause: D2jCause,
    ) -> (
        RuntimeExpr,
        crate::RuntimeDeclaration,
        crate::OrientedSubcontinuationPlanV1,
    ) {
        (
            d2j_entry_under(cause),
            d2j_declaration_under(cause),
            d2j_oriented_plan_under(cause),
        )
    }

    /// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the fully installed planner
    /// witness for one cause: fusions interned, region claims preflighted,
    /// fusion-owned bodies installed.**
    ///
    /// One constructor, so a control outside this module measures the SAME
    /// installed plan the partition control inside it does. The interning and
    /// installation steps are module-private, which is what previously forced
    /// any consumer to be written in here beside them; the alternative was a
    /// second inline copy of the sequence, and two copies of an installation
    /// order is exactly how two controls come to disagree about the witness
    /// they share.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn d2j_installed_plan_under<'src>(
        cause: D2jCause,
        entry: &'src RuntimeExpr,
        declarations: &BTreeMap<&'static str, &'src crate::RuntimeDeclaration>,
        oriented: &crate::OrientedSubcontinuationPlanV1,
    ) -> Result<StaticTransitionPlan<'src>, CraneliftBackendError> {
        let _ = cause;
        let mut plan = plan_static_transition_graph(entry, declarations)?;
        let resolved =
            build_static_continuation_fusion_plan(&plan, entry, declarations, Some(oriented))?;
        let mut plane = StaticContinuationFusionPlan::default();
        for key in resolved.installed_keys().to_vec() {
            plane.intern(key)?;
        }
        plan.install_static_continuation_fusions(plane)?;
        let mut claims = FusionRegionClaimLedger::preflight(&plan)?;
        plan.install_fusion_owned_bodies(&mut claims)?;
        Ok(plan)
    }



    /// Install one plane — optionally with the key perturbed — and preflight it.
    ///
    /// The perturbation is applied to the key **production derived**, so a
    /// refusal is attributable to the one moved operand rather than to a
    /// hand-built key that was never a real identity.
    #[cfg(test)]
    pub(super) fn d2f_preflight_exact(
        perturb: impl FnOnce(&mut Vec<StaticContinuationFusionKey>),
    ) -> Result<FusionRegionClaimLedger, CraneliftBackendError> {
        d2f_preflight_exact_owned(perturb, false)
    }

    /// As above, optionally taking the ruled producer-side body ownership as
    /// well — which is the state a production compile is actually in.
    #[cfg(test)]
    pub(super) fn d2f_preflight_exact_owned(
        perturb: impl FnOnce(&mut Vec<StaticContinuationFusionKey>),
        own_bodies: bool,
    ) -> Result<FusionRegionClaimLedger, CraneliftBackendError> {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let mut plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let resolved =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("the witness resolves a plane");
        let mut keys = resolved.installed_keys().to_vec();
        perturb(&mut keys);
        let mut plane = StaticContinuationFusionPlan::default();
        for key in keys {
            plane.intern(key)?;
        }
        plan.install_static_continuation_fusions(plane)?;
        let mut ledger = FusionRegionClaimLedger::preflight(&plan)?;
        if own_bodies {
            plan.install_fusion_owned_bodies(&mut ledger)?;
        }
        Ok(ledger)
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





















    use super::semantic_ir::{
        build_semantic_plane, DenseRange, RuntimeExprShape,
        SemanticAtomKind, SemanticOperandElement, SemanticOwner, SemanticSourceKind,
    };
    use super::*;
    use crate::cranelift_backend::surface::NativeSeedEnvironment;
    use crate::RuntimeGroundValue;
    use crate::{
        RuntimeComputationalMatchCase, RuntimeMatchCase, RuntimeTrap, RuntimeTrapCode, RuntimeValue,
    };

    pub(in crate::cranelift_backend) fn trap(message: &str) -> RuntimeTrap {
        RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: message.to_string(),
        }
    }

    pub(in crate::cranelift_backend) fn unit() -> RuntimeExpr {
        RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }
    }

    pub(in crate::cranelift_backend) fn nested_resource_bracket(depth: usize) -> RuntimeExpr {
        governed_nested_resource_bracket(depth)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum GovernedBracketRole {
        AllocatedBuffer,
        ScopeArgument,
        InductionHypothesis,
        RecursiveResult,
    }

    pub(super) fn role_at(index: u32, outer_to_inner: &[GovernedBracketRole]) -> GovernedBracketRole {
        outer_to_inner[outer_to_inner.len() - 1 - index as usize]
    }

    pub(super) fn assert_governed_bracket_shape(expr: &RuntimeExpr, depth: usize) {
        if depth == 0 {
            assert!(matches!(
                expr,
                RuntimeExpr::Construct { constructor, args }
                    if constructor == "ctor:prelude::Unit::MkUnit" && args.is_empty()
            ));
            return;
        }

        let RuntimeExpr::Match {
            scrutinee,
            cases,
            default,
        } = expr
        else {
            panic!("depth {depth} is not allocation-result match");
        };
        assert!(matches!(
            scrutinee.as_ref(),
            RuntimeExpr::Effect {
                operation: ken_host::HostOpV1::BufferAllocate,
                capability: None,
                args,
                ..
            } if matches!(
                args.as_slice(),
                [RuntimeExpr::Value(RuntimeValue::Int(value))] if *value == 1.into()
            )
        ));
        assert_eq!(
            cases.len(),
            2,
            "allocation match lost a trap or success arm"
        );
        assert!(matches!(
            &cases[0],
            RuntimeMatchCase {
                constructor,
                binders: 1,
                body: RuntimeExpr::Trap(_),
            } if constructor == "ctor:prelude::Result::Err"
        ));
        assert_eq!(default.message, "allocate result");

        let RuntimeMatchCase {
            constructor,
            binders: 1,
            body:
                RuntimeExpr::ComputationalMatch {
                    scrutinee,
                    cases,
                    default,
                },
        } = &cases[1]
        else {
            panic!("allocation success arm lost its recursive computational match");
        };
        assert_eq!(constructor, "ctor:prelude::Result::Ok");
        assert_eq!(default.message, "bracket scope");
        assert_eq!(
            cases.len(),
            1,
            "recursive computational match is not closed"
        );

        let RuntimeExpr::Construct { constructor, args } = scrutinee.as_ref() else {
            panic!("recursive scrutinee is not the governed Scope constructor");
        };
        assert_eq!(constructor, "ctor:fixture::Bracket::Scope");
        let [RuntimeExpr::LexicalClosure {
            captures,
            params,
            body,
        }] = args.as_slice()
        else {
            panic!("Scope does not carry exactly one lexical closure");
        };
        assert!(captures.is_empty());
        assert_eq!(params, &["buffer"]);

        let RuntimeComputationalMatchCase {
            constructor,
            argument_binders: 1,
            recursive_positions,
            body: case_body,
        } = &cases[0]
        else {
            panic!("recursive Scope arm changed binder arity");
        };
        assert_eq!(constructor, "ctor:fixture::Bracket::Scope");
        assert_eq!(recursive_positions, &[0]);
        let RuntimeExpr::Call { callee, args } = case_body else {
            panic!("recursive Scope arm no longer invokes its induction hypothesis");
        };
        let (RuntimeExpr::Var(callee), [RuntimeExpr::Var(argument)]) =
            (callee.as_ref(), args.as_slice())
        else {
            panic!("induction-hypothesis call lost its two semantic binder roles");
        };
        let case_roles = [
            GovernedBracketRole::AllocatedBuffer,
            GovernedBracketRole::ScopeArgument,
            GovernedBracketRole::InductionHypothesis,
        ];
        assert_eq!(
            role_at(*callee, &case_roles),
            GovernedBracketRole::InductionHypothesis
        );
        assert_eq!(
            role_at(*argument, &case_roles),
            GovernedBracketRole::AllocatedBuffer,
            "the induction-hypothesis argument is not the allocation result"
        );

        let RuntimeExpr::Let {
            value: recursive_body,
            body: release,
        } = body.as_ref()
        else {
            panic!("lexical closure lost its recursive-before-release ordering");
        };
        assert_governed_bracket_shape(recursive_body, depth - 1);

        let RuntimeExpr::Match {
            scrutinee,
            cases,
            default,
        } = release.as_ref()
        else {
            panic!("lexical closure release is not a result match");
        };
        assert_eq!(default.message, "release result");
        assert_eq!(cases.len(), 2, "release match lost a trap or success arm");
        assert!(matches!(
            &cases[0],
            RuntimeMatchCase {
                constructor,
                binders: 1,
                body: RuntimeExpr::Trap(_),
            } if constructor == "ctor:prelude::Result::Err"
        ));
        assert!(matches!(
            &cases[1],
            RuntimeMatchCase {
                constructor,
                binders: 1,
                body: RuntimeExpr::Construct {
                    constructor: unit,
                    args,
                },
            } if constructor == "ctor:prelude::Result::Ok"
                && unit == "ctor:prelude::Unit::MkUnit"
                && args.is_empty()
        ));

        let RuntimeExpr::Effect {
            operation: ken_host::HostOpV1::BufferFreeze,
            capability: None,
            args,
            ..
        } = scrutinee.as_ref()
        else {
            panic!("release scrutinee is not BufferFreeze");
        };
        let [RuntimeExpr::Var(buffer), RuntimeExpr::Value(RuntimeValue::Int(start)), RuntimeExpr::Value(RuntimeValue::Int(length)), RuntimeExpr::Var(span_origin)] =
            args.as_slice()
        else {
            panic!("BufferFreeze does not have its canonical four operands");
        };
        let release_roles = [
            GovernedBracketRole::AllocatedBuffer,
            GovernedBracketRole::RecursiveResult,
        ];
        assert_eq!(
            role_at(*buffer, &release_roles),
            GovernedBracketRole::AllocatedBuffer
        );
        assert_eq!(
            role_at(*span_origin, &release_roles),
            GovernedBracketRole::AllocatedBuffer
        );
        assert_eq!(
            buffer, span_origin,
            "resource seats do not name the same closure parameter"
        );
        assert_eq!(*start, 0.into());
        assert_eq!(*length, 1.into());
    }


    pub(super) fn assert_fixed_helper_identity_shape(key: PlannedHelperKey) {
        fn require_copy<T: Copy>() {}
        require_copy::<PlannedHelperKey>();
        match key {
            PlannedHelperKey::Node(_transition, StaticNodeId(_ordinal)) => {}
            PlannedHelperKey::Edge(_kind, StaticEdgeId(_ordinal)) => {}
        }
    }

    pub(super) fn census(depth: usize) -> BoundaryACensus {
        let expr = nested_resource_bracket(depth);
        plan_static_transition_graph(&expr, &BTreeMap::new())
            .map(|plan| {
                for key in &plan.planned_helpers {
                    assert_fixed_helper_identity_shape(*key);
                }
                plan.census()
            })
            .unwrap_or_else(|error| {
                panic!("RT_NATIVE_FNSPLIT_BOUNDARY_A could_not_determine n={depth}: {error}")
            })
    }

    pub(super) fn values(rows: &[BoundaryACensus], field: impl Fn(&BoundaryACensus) -> usize) -> Vec<isize> {
        rows.iter().map(|row| field(row) as isize).collect()
    }

    pub(super) fn differences(values: &[isize]) -> (Vec<isize>, Vec<isize>) {
        let first = values.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        let second = first.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        (first, second)
    }

    pub(super) fn semantic_census(depth: usize) -> (BoundaryACensus, BoundaryB1Census) {
        let expr = nested_resource_bracket(depth);
        plan_static_transition_graph(&expr, &BTreeMap::new())
            .map(|plan| (plan.census(), plan.semantic_census()))
            .unwrap_or_else(|error| {
                panic!("RT_NATIVE_FNSPLIT_B1 could_not_determine n={depth}: {error}")
            })
    }

    pub(super) fn semantic_values(
        rows: &[BoundaryB1Census],
        field: impl Fn(&BoundaryB1Census) -> usize,
    ) -> Vec<isize> {
        rows.iter().map(|row| field(row) as isize).collect()
    }

    pub(super) fn index_of_edge_helper(plan: &StaticTransitionPlan, edge: StaticEdgeId) -> usize {
        plan.planned_helpers
            .iter()
            .position(|helper| matches!(helper, PlannedHelperKey::Edge(_, id) if *id == edge))
            .expect("edge has a planned helper")
    }

    pub(super) fn rewrite_edge(
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

    pub(super) fn append_edge(
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

    /// Two occurrences of the same shape whose material differs. `Var(0)` and
    /// `Var(1)` agree on shape, opcode, atom count and child count, so shape and
    /// count checks cannot separate them: only occurrence-exact material can.
    pub(in crate::cranelift_backend) fn equal_shaped_atom_fixture() -> RuntimeExpr {
        RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Var(0)),
            body: Box::new(RuntimeExpr::Var(1)),
        }
    }






    /// Two `Let` occurrences of identical shape and counts whose positional
    /// children are different occurrences.
    /// ⛔ **Test-local, closure-REFUSING witness for exactly this fixture's
    /// grammar** — `If`, the unit `Construct`, `Let`, and `Var(index)`.
    ///
    /// `D2` removed `RuntimeExpr: PartialEq` because it reached
    /// `RuntimeValue::ClosureRef`. The address-independence control below
    /// genuinely needs **recursive** comparison — `discriminant` cannot express
    /// it — so this is the narrow route the Architect's ruling permits: an input
    /// grammar that **refuses** anything closure-capable before producing a
    /// verdict.
    ///
    /// ⛔ Deliberately NOT a shared `RuntimeExpr` projection. It lives in this
    /// test module, covers four forms, and every other variant — including
    /// `Closure`, `LexicalClosure`, and `Value(ClosureRef)` — returns `None`.
    /// `None` is a **refusal that fails the test**, never a skip.
    #[derive(Debug, PartialEq, Eq)]
    pub(in crate::cranelift_backend) enum FixtureWitness {
        Unit,
        Var(u32),
        Let(Box<FixtureWitness>, Box<FixtureWitness>),
        If(
            Box<FixtureWitness>,
            Box<FixtureWitness>,
            Box<FixtureWitness>,
        ),
    }

    pub(in crate::cranelift_backend) fn fixture_witness(expr: &RuntimeExpr) -> Option<FixtureWitness> {
        Some(match expr {
            RuntimeExpr::Construct { constructor, args }
                if constructor == "ctor:prelude::Unit::MkUnit" && args.is_empty() =>
            {
                FixtureWitness::Unit
            }
            RuntimeExpr::Var(index) => FixtureWitness::Var(*index),
            RuntimeExpr::Let { value, body } => FixtureWitness::Let(
                Box::new(fixture_witness(value)?),
                Box::new(fixture_witness(body)?),
            ),
            RuntimeExpr::If {
                scrutinee,
                then_expr,
                else_expr,
            } => FixtureWitness::If(
                Box::new(fixture_witness(scrutinee)?),
                Box::new(fixture_witness(then_expr)?),
                Box::new(fixture_witness(else_expr)?),
            ),
            // ⛔ Every other form REFUSES. Closure-bearing ones are the reason
            // this arm exists, and `p2_the_fixture_witness_refuses_closures`
            // is the negative control proving they cannot slip through.
            _ => return None,
        })
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

    pub(in crate::cranelift_backend) fn equal_shaped_child_fixture() -> RuntimeExpr {
        RuntimeExpr::If {
            scrutinee: Box::new(unit()),
            then_expr: Box::new(RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::Var(0)),
                body: Box::new(RuntimeExpr::Var(1)),
            }),
            else_expr: Box::new(RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::Var(2)),
                body: Box::new(RuntimeExpr::Var(3)),
            }),
        }
    }

    pub(super) fn nodes_of_shape(plan: &StaticTransitionPlan, shape: RuntimeExprShape) -> Vec<StaticNodeId> {
        plan.semantic_sources
            .iter()
            .filter_map(|source| {
                (source.source == SemanticSourceKind::Expression(shape))
                    .then_some(source.planned_node)
            })
            .collect()
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

    pub(super) fn primitive_call(symbol: &str, partiality: crate::RuntimePartiality) -> RuntimeExpr {
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
    pub(super) fn equal_shaped_primitive_pair(
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
    pub(super) fn descriptor_bytes(plan: &StaticTransitionPlan, node: StaticNodeId) -> Vec<u8> {
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
    pub(super) fn assert_partiality_is_occurrence_exact(
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
    pub(super) fn identity_fixture() -> RuntimeExpr {
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
                super::semantic_ir::unpack_identity(packed).unwrap(),
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
            super::semantic_ir::unpack_identity(0).unwrap_err(),
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
        pub(super) fn terminal_id(&self) -> StaticNodeId {
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
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn b2ac_topology_fixtures() -> Vec<(&'static str, RuntimeExpr)> {
        let leaf = || RuntimeExpr::Value(RuntimeValue::Bool(true));
        let trap = || RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "b2ac topology".to_string(),
        };
        let computational = |body: RuntimeExpr| RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::B2AC::Node".to_string(),
                args: vec![leaf()],
            }),
            cases: vec![crate::RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::B2AC::Node".to_string(),
                argument_binders: 1,
                recursive_positions: Vec::new(),
                body,
            }],
            default: trap(),
        };
        vec![
            ("leaf", leaf()),
            (
                "let-if",
                RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::If {
                        scrutinee: Box::new(leaf()),
                        then_expr: Box::new(leaf()),
                        else_expr: Box::new(leaf()),
                    }),
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            ),
            (
                "match",
                RuntimeExpr::Match {
                    scrutinee: Box::new(leaf()),
                    cases: vec![RuntimeMatchCase {
                        constructor: "ctor:fixture::B2AC::A".to_string(),
                        binders: 0,
                        body: leaf(),
                    }],
                    default: trap(),
                },
            ),
            (
                "lexical-closure-call",
                RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::LexicalClosure {
                        captures: vec![leaf()],
                        params: vec!["x".to_string()],
                        body: Box::new(RuntimeExpr::Var(0)),
                    }),
                    args: vec![leaf()],
                },
            ),
            ("computational", computational(RuntimeExpr::Var(0))),
            (
                "computational-nested",
                computational(computational(RuntimeExpr::Var(0))),
            ),
            (
                "computational-under-let",
                RuntimeExpr::Let {
                    value: Box::new(computational(RuntimeExpr::Var(0))),
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            ),
        ]
    }

    /// A canonical digest of the Boundary-A transfer graph: node transitions in
    /// order, then every edge as `(from, to, kind)` in order.
    #[cfg(test)]
    pub(super) fn b2ac_topology_digest(expr: &RuntimeExpr) -> String {
        let plan = plan_static_transition_graph(expr, &BTreeMap::new()).expect("plannable");
        let mut digest = String::new();
        digest.push_str(&format!(
            "nodes={} edges={}",
            plan.nodes.len(),
            plan.edges.len()
        ));
        for node in &plan.nodes {
            digest.push_str(&format!("|n{}:{:?}", node.id.0, node.transition));
        }
        for edge in &plan.edges {
            digest.push_str(&format!(
                "|e{}:{}->{}:{:?}",
                edge.id.0, edge.from.0, edge.to.0, edge.kind
            ));
        }
        digest.push_str(&format!("|entries={:?}", plan.entries));
        digest
    }

    /// **AC-11 — every transfer edge is unchanged and consumes `.entry`.**
    ///
    /// These digests were captured by running the identical probe against the
    /// WP's base commit `70bd2c74` — before `PlannedExpr` existed — in a scratch
    /// worktree, and are asserted here against the post-D9 planner. Equality is
    /// the mechanical proof that the Boundary-A graph is topologically
    /// identical: same nodes in the same order, same edges with the same
    /// `(from, to, kind)`, same scheduling entries.
    ///
    /// ## ⚠ Reproducing the baseline — the recipe, because equality hides its own
    /// provenance
    ///
    /// ⛔ The asserted property is *equality against committed constants*, so a
    /// re-capture taken **after** the change would have produced byte-identical
    /// values. **Nothing in this file distinguishes a genuine pre-change baseline
    /// from a re-recording**, and the scratch worktree it was taken in is gone. So
    /// the binding is demonstrated here rather than testified to — anyone can
    /// redo it in about two minutes:
    ///
    /// ```text
    /// git worktree add --detach /tmp/b2ac-base 70bd2c74
    /// # port these two functions into that tree's test module verbatim:
    /// #   `b2ac_topology_fixtures`  (the seven fixtures, by name)
    /// #   `b2ac_topology_digest`    (nodes, edges, entries -- it reads nothing
    /// #                              that postdates the base, which is why it
    /// #                              compiles there at all)
    /// cd /tmp/b2ac-base
    /// scripts/ken-cargo test -p ken-runtime --lib -- b2ac_topology
    /// git worktree remove /tmp/b2ac-base
    /// ```
    ///
    /// ⛔ `scripts/ken-cargo`, never raw `cargo` — `COORDINATION §12`, and it binds
    /// inside a copied recipe exactly as it binds anywhere else. A recipe that
    /// spells the raw command teaches the next reader to bypass the build lock.
    ///
    /// Verified this way by the adversary on `2db29abe`: **all seven rows
    /// reproduce byte-for-byte** from `70bd2c74`, including
    /// `computational-under-let`, which is the row carrying the load.
    ///
    /// ⭐ Read `computational-under-let`: the parent `Sequence` (n12) edges to
    /// **n11**, the computational match's scrutinee, and *not* to the
    /// `SourceReturnResume` (n6). That is D9's promise — the occurrence moved to
    /// the resume while the schedule stayed on the scrutinee — and this row is
    /// what would redden if a future change returned the resume as the entry.
    #[cfg(test)]
    const B2AC_BASE_TOPOLOGY: &[(&str, &str)] = &[
        ("leaf", "nodes=3 edges=1|n0:Terminal|n1:TrapTerminal|n2:Evaluate|e0:2->0:Continue|entries=[StaticNodeId(2)]"),
        ("let-if", "nodes=9 edges=8|n0:Terminal|n1:TrapTerminal|n2:Evaluate|n3:Evaluate|n4:Evaluate|n5:Branch|n6:Evaluate|n7:Evaluate|n8:Sequence|e0:2->0:Continue|e1:3->2:Continue|e2:4->2:Continue|e3:5->3:Select|e4:5->4:Reject|e5:6->5:Continue|e6:7->6:Continue|e7:8->7:Continue|entries=[StaticNodeId(8)]"),
        ("match", "nodes=7 edges=6|n0:Terminal|n1:TrapTerminal|n2:Evaluate|n3:Evaluate|n4:CaseTest|n5:Evaluate|n6:Evaluate|e0:2->1:Trap|e1:3->0:Continue|e2:4->3:Select|e3:4->2:Reject|e4:5->4:Continue|e5:6->5:Continue|entries=[StaticNodeId(6)]"),
        ("lexical-closure-call", "nodes=8 edges=7|n0:Terminal|n1:TrapTerminal|n2:Evaluate|n3:ClosureBody|n4:Evaluate|n5:Evaluate|n6:Evaluate|n7:Sequence|e0:2->0:Continue|e1:3->0:Continue|e2:4->3:Continue|e3:5->2:Continue|e4:6->5:Continue|e5:6->4:StaticBody|e6:7->6:Continue|entries=[StaticNodeId(7)]"),
        ("computational", "nodes=11 edges=10|n0:Terminal|n1:TrapTerminal|n2:CompletedTail|n3:ProducerTail|n4:ProducerWrapper|n5:SourceReturnResume|n6:Evaluate|n7:Evaluate|n8:CaseTest|n9:Evaluate|n10:Sequence|e0:5->4:InvokeProducerWrapper|e1:4->3:InvokeProducerTail|e2:3->2:CompleteProducerTail|e3:2->0:Continue|e4:6->1:Trap|e5:7->5:SourceReturnOwnedResume|e6:8->7:Select|e7:8->6:Reject|e8:9->8:Continue|e9:10->9:Continue|entries=[StaticNodeId(10)]"),
        ("computational-nested", "nodes=19 edges=19|n0:Terminal|n1:TrapTerminal|n2:CompletedTail|n3:ProducerTail|n4:ProducerWrapper|n5:SourceReturnResume|n6:Evaluate|n7:CompletedTail|n8:ProducerTail|n9:ProducerWrapper|n10:SourceReturnResume|n11:Evaluate|n12:Evaluate|n13:CaseTest|n14:Evaluate|n15:Sequence|n16:CaseTest|n17:Evaluate|n18:Sequence|e0:5->4:InvokeProducerWrapper|e1:4->3:InvokeProducerTail|e2:3->2:CompleteProducerTail|e3:2->0:Continue|e4:6->1:Trap|e5:10->9:InvokeProducerWrapper|e6:9->8:InvokeProducerTail|e7:8->7:CompleteProducerTail|e8:7->5:SourceReturnOwnedResume|e9:11->1:Trap|e10:12->10:SourceReturnOwnedResume|e11:13->12:Select|e12:13->11:Reject|e13:14->13:Continue|e14:15->14:Continue|e15:16->15:Select|e16:16->6:Reject|e17:17->16:Continue|e18:18->17:Continue|entries=[StaticNodeId(18)]"),
        ("computational-under-let", "nodes=13 edges=12|n0:Terminal|n1:TrapTerminal|n2:Evaluate|n3:CompletedTail|n4:ProducerTail|n5:ProducerWrapper|n6:SourceReturnResume|n7:Evaluate|n8:Evaluate|n9:CaseTest|n10:Evaluate|n11:Sequence|n12:Sequence|e0:2->0:Continue|e1:6->5:InvokeProducerWrapper|e2:5->4:InvokeProducerTail|e3:4->3:CompleteProducerTail|e4:3->2:Continue|e5:7->1:Trap|e6:8->6:SourceReturnOwnedResume|e7:9->8:Select|e8:9->7:Reject|e9:10->9:Continue|e10:11->10:Continue|e11:12->11:Continue|entries=[StaticNodeId(12)]"),
    ];

    #[test]
    fn boundary_a_topology_is_identical_to_the_pre_d9_planner() {
        let expected: BTreeMap<&str, &str> = B2AC_BASE_TOPOLOGY.iter().copied().collect();
        for (name, expr) in b2ac_topology_fixtures() {
            let digest = b2ac_topology_digest(&expr);
            let base = expected
                .get(name)
                .expect("every fixture has a recorded base digest");
            assert_eq!(
                &digest.as_str(),
                base,
                "AC-11: `{name}` changed the Boundary-A transfer graph. D9 must move \
                 only which identity is RECORDED at a source position, never which \
                 node is SCHEDULED."
            );
        }
    }

    /// **AC-13 — the split is exactly one variant.**
    #[test]
    fn computational_match_is_the_sole_entry_occurrence_split() {
        let mut split = Vec::new();
        for (name, expr) in b2ac_topology_fixtures() {
            let mut planner = Planner::new().expect("planner");
            let empty = PersistentNodeId(0);
            let context = PlanContext {
                environment: empty,
                continuation: empty,
                path: empty,
                cleanup: empty,
                affine: empty,
                source_return: empty,
            };
            let planned = planner
                .plan_expr(&expr, context, planner.terminal, EdgeKind::Continue, 0)
                .expect("plannable");
            if planned.occurrence != origin_of(planned.entry) {
                split.push(name);
            }
        }
        assert_eq!(
            split,
            vec!["computational", "computational-nested"],
            "AC-13: only a `ComputationalMatch` result may split entry from \
             occurrence, and every such result must. `computational-under-let` is \
             a `Let` at the root, so its own result does not split."
        );
    }


    /// Emit one fixture end to end, returning the failure text if it refuses.
    #[cfg(test)]
    pub(super) fn ac3_emit(
        root: &RuntimeExpr,
        declarations: &BTreeMap<&str, &RuntimeDeclaration>,
    ) -> Result<(), String> {
        use crate::cranelift_backend::artifact::new_object_module_for_lowering_tests;
        use crate::cranelift_backend::lowering::core::{
            compile_expr_into_object_module, NativeSeedEnvironment,
        };
        let seed_env = NativeSeedEnvironment::empty();
        compile_expr_into_object_module(
            new_object_module_for_lowering_tests("ac3")
                .map_err(|error| format!("{error:?}"))?,
            "ac3_entry",
            cranelift_module::Linkage::Export,
            root,
            &seed_env,
            declarations.clone(),
            None,
            true,
            None,
            Some(crate::cranelift_backend::test_support::test_only_distinguished_root_join_plan()),
            None,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-3` — collapsing the issued body
    /// back to the scheduling entry recreates the traversal/closeout failure.**
    ///
    /// > **PROPERTY:** the issued body occurrence is what makes the source
    /// > traversal reach the unit's join subtree.
    /// > **OPERAND THAT MOVED:** the **population** — the value the planner
    /// > issues for a `SchedulingEntry` seed, mutated at the seat that resolves
    /// > it, back to the pre-correction `StaticOriginId(seed.0)` alias. No
    /// > detector, assertion or validator arm was touched.
    /// > **OBSERVED BOUNDARY:** `finalize_join_disposition` refuses with
    /// > *"function left planned source join … neither emitted nor statically
    /// > unselected"* — a required join reached by neither consumption nor
    /// > disposition, which is the attribution record's failure exactly.
    ///
    /// **Population-side is the whole point.** `AC-3` asserts REACH. A
    /// detector-side mutation would redden this same test name while the
    /// carried value never moved, and would keep reddening for the entire life
    /// of a correction that reached nothing.
    ///
    /// **The `Exact` arm is not a fourth assertion — it is what validates the
    /// other three.** A refusal control only has to reach its own guard; the
    /// success arm has to traverse every guard, so it is the only arm that
    /// establishes the fixture could have lowered at all. Without it, a fixture
    /// broken upstream would refuse under both settings and read as a discharge.
    ///
    /// **Honest scope of the two arms.** The root arm exercises the arm that
    /// previously carried a *workaround* (`define_unit_body`'s `is_root`
    /// substitution), so it demonstrates the mechanism rather than the shipped
    /// defect. The **non-root** arm is the population that was actually broken:
    /// nothing compensated for it, which is why the defect shipped. Both are
    /// asserted; neither is offered as the other.
    #[test]
    fn collapsing_the_body_to_its_scheduling_entry_recreates_the_closeout_failure() {
        use super::semantic_ir::{with_body_occurrence_mutation, BodyOccurrenceMutation};

        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");

        // Arm 1: the ROOT unit.
        let empty = BTreeMap::new();
        assert_eq!(
            ac3_emit(&computational, &empty),
            Ok(()),
            "AC-3 positive control: the root fixture must lower under the exact \
             pairing, or the refusals below prove nothing about the pairing"
        );
        let collapsed_root = with_body_occurrence_mutation(
            BodyOccurrenceMutation::CollapseSchedulingEntryBody,
            || ac3_emit(&computational, &empty),
        );
        assert_eq!(
            collapsed_root,
            Err(
                "Backend(Module(\"function left planned source join StaticOriginId(5) \
                 neither emitted nor statically unselected\"))"
                    .to_string()
            ),
            "AC-3: with the body collapsed to the scheduling entry the traversal \
             enters the entry and never reaches the join subtree, so closeout \
             finds a required join neither consumed nor dispositioned"
        );

        // Arm 2: a NON-ROOT unit — the population that actually shipped broken.
        let declaration = b2o_transparent_declaration(computational.clone());
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        let root = RuntimeExpr::Value(RuntimeValue::Bool(true));
        assert_eq!(
            ac3_emit(&root, &declarations),
            Ok(()),
            "AC-3 positive control: the non-root fixture must lower under the \
             exact pairing"
        );
        let collapsed_declaration = with_body_occurrence_mutation(
            BodyOccurrenceMutation::CollapseSchedulingEntryBody,
            || ac3_emit(&root, &declarations),
        );
        assert!(
            collapsed_declaration
                .as_ref()
                .err()
                .is_some_and(|message| message.contains("planned source join")),
            "AC-3: the non-root unit is the population the removed root-only \
             substitution never covered; collapsing its body must recreate the \
             same closeout failure. got {collapsed_declaration:?}"
        );
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
    pub(super) fn liftrose_synthetic_witness_closes_owner_two_required_joins() {
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



    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-1b` — the `StaticBodyTarget` class
    /// takes its ISSUED pair, not its seed's own ordinal.**
    ///
    /// > **MEASURED:** for a closure whose body is a computational match, the
    /// > unit seeded on the `StaticBody` target carries a body occurrence that
    /// > differs from `origin_of(seed)`, and equals the pair issued when that
    /// > body's `StaticBody` edge was registered.
    /// > **CLAIMED:** the retired `StaticOriginId(edge.to.0)` fallback is gone
    /// > and this class reads the one relation.
    /// > **THE GAP:** the fixture's closure body must genuinely schedule
    /// > something before itself. For an ordinary body the seed's own ordinal
    /// > IS its occurrence, so the fallback and the relation agree and the test
    /// > passes under both -- which is exactly how the carve-out survived
    /// > review the first time.
    ///
    /// This is the class the original bounded contract exempted as
    /// already-grounded. On venue 4 that exemption issued `SOI(58)` to a unit
    /// whose real body was `SOI(26)`, and its four planned joins were never
    /// entered.
    #[test]
    fn a_static_body_target_whose_body_is_computational_takes_its_issued_pair() {
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let expr = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(computational),
            }),
            args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
        };
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plannable");

        let body_edge = plan
            .edges
            .iter()
            .find(|edge| edge.kind == EdgeKind::StaticBody)
            .expect("precondition: the fixture must carry a static body edge");
        let unit = plan
            .semantic
            .functions
            .iter()
            .find(|function| function.planned_node == body_edge.to)
            .expect("the static body target seeded a function unit");

        assert_ne!(
            unit.body_occurrence,
            origin_of(unit.planned_node),
            "AC-1b precondition AND claim: the closure body must schedule \
             something before itself, so the retired fallback \
             `StaticOriginId(edge.to.0)` and the issued pair DISAGREE here. If \
             they agreed, this test would pass under the carve-out too"
        );
        assert_eq!(
            plan.planned_entry_body(body_edge.to),
            Some(unit.body_occurrence),
            "AC-1b: the unit reads the row issued when its static body edge was \
             registered -- one relation, not a per-class rule"
        );
    }

    /// **`AC-3` `StaticBodyTarget` arm — the CLASS-SELECTIVE collapse.**
    ///
    /// > **OPERAND THAT MOVED:** the population, restricted to the
    /// > `StaticBodyTarget` class -- the retired `StaticOriginId(edge.to.0)`
    /// > fallback restored for that class ONLY.
    ///
    /// A global collapse reddens first through the `SchedulingEntry` class and
    /// therefore says nothing about this one. The informative side is the arm
    /// that would still green if this class were left on the fallback, which is
    /// why the mutation has to be class-selective rather than plan-wide.
    #[test]
    fn collapsing_only_the_static_body_target_class_is_refused() {
        use super::semantic_ir::{with_body_occurrence_mutation, BodyOccurrenceMutation};
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let expr = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(computational),
            }),
            args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
        };
        let empty = BTreeMap::new();
        assert_eq!(
            ac3_emit(&expr, &empty),
            Ok(()),
            "AC-3 positive control: the fixture must lower under the exact \
             relation, or the refusal below proves nothing about the relation"
        );
        let collapsed = with_body_occurrence_mutation(
            BodyOccurrenceMutation::CollapseStaticBodyTargetBody,
            || ac3_emit(&expr, &empty),
        );
        assert!(
            collapsed
                .as_ref()
                .err()
                .is_some_and(|message| message.contains("planned source join")),
            "AC-3: restoring the retired fallback for this class alone must \
             recreate the traversal/closeout failure. got {collapsed:?}"
        );
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-4` — call identity is the ENTRY
    /// axis and did not move with the body axis.**
    ///
    /// > **MEASURED:** across every `b2ac` fixture, each call edge's
    /// > `callee_origin` equals `origin_of(callee_unit.planned_node)`.
    /// > **CLAIMED:** the correction changed the BODY axis only; call identity
    /// > is invariant under it.
    /// > **THE GAP:** at least one fixture must have a unit whose two axes
    /// > DIFFER, or the equality holds for both readings and the pin cannot
    /// > tell which axis it measured.
    ///
    /// **This is the invariant most easily broken by accident, and the one a
    /// green suite is least likely to catch.** The old `origin` field was an
    /// alias of `planned_node`, so every consumer read the entry axis whether or
    /// not it meant to. Renaming that field in bulk would have silently moved
    /// call identity onto the body axis for exactly the units where the two
    /// differ — the same units the correction targets — and every fixture whose
    /// axes coincide would have stayed green.
    #[test]
    fn call_identity_stays_on_the_entry_axis_after_the_body_axis_moved() {
        let mut fixtures_with_split_axes = 0usize;
        let mut checked_edges = 0usize;

        for (name, expr) in b2ac_topology_fixtures() {
            let plan = plan_static_transition_graph(&expr, &BTreeMap::new())
                .unwrap_or_else(|error| panic!("{name} must plan: {error:?}"));

            fixtures_with_split_axes += usize::from(
                plan.semantic
                    .functions
                    .iter()
                    .any(|function| function.body_occurrence != origin_of(function.planned_node)),
            );

            for edge in plan.emittable_call_edges().expect("call edges") {
                let callee = plan
                    .semantic
                    .functions
                    .iter()
                    .find(|function| function.id == edge.callee())
                    .expect("a call edge names a planned unit");
                assert_eq!(
                    edge.callee_origin(),
                    origin_of(callee.planned_node),
                    "AC-4 [{name}]: a call names the unit it ENTERS. The body \
                     occurrence is where that unit's traversal begins once \
                     inside, and moving call identity onto it would change which \
                     unit a call resolves to"
                );
                checked_edges += 1;
            }
        }

        // Non-vacuity, both axes.
        assert!(
            fixtures_with_split_axes > 0,
            "AC-4 precondition: at least one fixture must have a unit whose entry \
             and body DIFFER, or this test passes under either reading and \
             measures nothing"
        );
        assert!(
            checked_edges > 0,
            "AC-4 precondition: the fixture set must actually produce call edges"
        );
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-1` — the issued pair is `n18 ->
    /// n5`, and `n10` is not registered as that unit's body.**
    ///
    /// > **MEASURED:** on the frozen `computational-nested` fixture, the sole
    /// > row of the pairing authority is `(n18, origin_of(n5))`, and no
    /// > `PredeclaredFunction` carries `origin_of(n10)` as its body.
    /// > **CLAIMED:** the registration binds the OUTER scheduling entry to the
    /// > outer body occurrence its own visit returned, and excludes the nested
    /// > call's occurrence.
    /// > **THE GAP:** `n10` must actually EXIST and be the inner match's
    /// > occurrence. "`n10` is not registered" is vacuously true of a node the
    /// > fixture never planned, so the exclusion carries no information until
    /// > the excluded thing is shown to be the real, competing candidate.
    ///
    /// **The non-vacuity arms are the test.** Both are asserted here rather
    /// than assumed: `n5 != n10` (two distinct resumes exist under one entry, so
    /// there is a genuine choice to get wrong) and `origin_of(n18) != n5` (the
    /// entry is not the body, so a pin that read the entry would differ). On a
    /// fixture where the axes coincide this test would pass while measuring
    /// nothing.
    ///
    /// The exact node identities are a **normative compatibility vector**:
    /// they are the frozen `B2AC_BASE_TOPOLOGY` row for this fixture, which
    /// pins `nodes=19`, `n5`/`n10` as the two `SourceReturnResume` nodes and
    /// `entries=[StaticNodeId(18)]`. A topology change reddens that row first.
    #[test]
    fn nested_registration_issues_the_outer_pair_and_excludes_the_inner_resume() {
        let (_, nested) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational-nested")
            .expect("the nested computational fixture");
        let plan = plan_static_transition_graph(&nested, &BTreeMap::new()).expect("plannable");

        let n18 = StaticNodeId(18);
        let n5 = origin_of(StaticNodeId(5));
        let n10 = origin_of(StaticNodeId(10));

        // Non-vacuity, before anything is concluded from an absence.
        assert_ne!(
            n5, n10,
            "AC-1 precondition: the fixture must supply TWO distinct resumes \
             under one entry, or there is no wrong answer available to exclude"
        );
        assert_ne!(
            origin_of(n18),
            n5,
            "AC-1 precondition: entry and body must differ on this fixture, or \
             reading the entry would be indistinguishable from reading the body"
        );
        let outer = plan.root_static_origin().expect("root occurrence");
        assert_eq!(outer, n5, "the outer occurrence is the outer resume");
        assert_eq!(
            plan.child_static_origin(outer, 1)
                .expect("the outer match's case body resolves"),
            n10,
            "AC-1 precondition: `n10` is the INNER match's occurrence — a real, \
             competing candidate, not an absent node"
        );

        // The issued pairing itself.
        assert_eq!(
            plan.planned_entry_bodies,
            vec![PlannedEntryBody {
                entry: n18,
                body_occurrence: n5,
            }],
            "AC-1: the sole issued pair binds the outer scheduling entry to the \
             outer body occurrence its own visit returned"
        );

        // And no unit claims the nested call's occurrence as its body.
        assert!(
            plan.semantic
                .functions
                .iter()
                .all(|function| function.body_occurrence != n10),
            "AC-1: `n10` is the occurrence the NESTED call returned to its \
             parent; recovering an `outermost` resume by graph shape is exactly \
             what would select it"
        );
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` supporting discrimination — a NON-ROOT
    /// unit whose body schedules something before itself has entry != body.**
    ///
    /// **This is NOT the node's `AC-2`, and it must not be read as discharging
    /// it.** The node's `AC-2` is the exact `LiftRose` synthetic-venue result:
    /// owner 2's required `{26, 33, 39, 53}` reached and closed. This test is
    /// obligation 2 of the LEADER'S DISPATCH list, which numbers differently
    /// from the node's acceptance table — an earlier revision of this file
    /// labelled it `AC-2` and thereby claimed a gate it does not touch. The
    /// node's table is the authority; a dispatch's ordering is not.
    ///
    /// > **MEASURED:** for a transparent declaration whose body is a
    /// > computational match, the unit's `body_occurrence` differs from
    /// > `origin_of(planned_node)`, and equals the declaration occurrence the
    /// > planner recorded for that symbol.
    /// > **CLAIMED:** the correction reaches NON-ROOT units — the population the
    /// > removed root-only substitution never covered.
    /// > **THE GAP:** the unit must genuinely be non-root. `AC-1`'s fixture has
    /// > exactly one entry and it IS the root, so it cannot discharge this;
    /// > passing it off as coverage would leave the entire defect population
    /// > unmeasured.
    ///
    /// This is the discriminating pair the old code could not produce: before
    /// the correction `body_occurrence` was `StaticOriginId(seed.0)`, so the
    /// first assertion below was an identity and could not fail.
    #[test]
    fn a_non_root_computational_declaration_body_differs_from_its_entry() {
        // Reuse the frozen `computational` fixture shape as the DECLARATION
        // body, so the only thing varying from `AC-1` is root-ness.
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let declaration = b2o_transparent_declaration(computational);
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        let root = RuntimeExpr::Value(RuntimeValue::Bool(true));
        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");

        let root_entry = plan.root_entry.expect("a root entry");
        let declaration_body = plan
            .declaration_occurrence_origin("decl:fixture::b2o")
            .expect("the declaration was planned");

        let pair = plan
            .planned_entry_bodies
            .iter()
            .find(|pair| pair.entry != root_entry)
            .expect("precondition: a non-root scheduling entry exists");

        assert_ne!(
            origin_of(pair.entry),
            pair.body_occurrence,
            "a non-root unit whose body schedules its scrutinee first must \
             not have its entry aliased as its body — this equality is what the \
             correction removed"
        );
        assert_eq!(
            pair.body_occurrence, declaration_body,
            "the issued body is the occurrence the declaration's own visit \
             returned"
        );

        // The unit built from that seed carries the issued value, not the alias.
        let unit = plan
            .semantic
            .functions
            .iter()
            .find(|function| function.planned_node == pair.entry)
            .expect("the non-root seed built a function unit");
        assert_eq!(
            unit.body_occurrence, declaration_body,
            "the carried field is the issued body occurrence"
        );
        assert_ne!(
            unit.body_occurrence,
            origin_of(unit.planned_node),
            "and it is NOT an alias of the scheduling entry"
        );
    }

    /// **`RT-FNSPLIT-B2A-S` AC-5 — keying selection by the scheduling ENTRY
    /// resolves to the WRONG body. Demonstrated, not forbidden by a grep.**
    ///
    /// ⛔ The first candidate discharged AC-5 by scanning for four container
    /// spellings keyed by `StaticNodeId`. The Architect rejected that
    /// (`evt_6sq2tq3v9jcd0`) and was right: a `Vec` indexed by `planned.entry.0`, a
    /// type alias, or a bespoke collection all violate the ruled property while
    /// such a scan stays green. **The property is about which value selects a body,
    /// so the control has to be about that too.**
    #[test]
    fn keying_selection_by_the_scheduling_entry_does_not_resolve_the_body() {
        // Promise class: durable invariant.
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let plan =
            plan_static_transition_graph(&computational, &BTreeMap::new()).expect("plannable");

        let occurrence = plan.root_static_origin().expect("root occurrence");
        let entry = *plan.entries.first().expect("a root entry");
        assert_ne!(
            occurrence,
            origin_of(entry),
            "AC-5: the fixture must actually exhibit the split, or this test is vacuous"
        );

        // What the TAG resolves to: this match.
        let by_tag = plan
            .source_occurrence(occurrence)
            .expect("the occurrence resolves its own body");
        assert!(
            matches!(by_tag, RuntimeExpr::ComputationalMatch { .. }),
            "AC-5: the occurrence must resolve to the match itself"
        );

        // What an ENTRY-keyed lookup would resolve to: anything but this body. It
        // is either a different term or no source occurrence at all -- both are
        // wrong answers for "the body of this match", which is the point.
        let by_entry = plan.source_occurrence(origin_of(entry));
        assert!(
            !matches!(by_entry, Ok(term) if std::ptr::eq(term, by_tag)),
            "AC-5: the scheduling entry must not resolve to the occurrence's body; \
             if it does, entry and occurrence have been conflated again and \
             hard-stop #8 is back"
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
            super::semantic_ir::positioned_sources(&plan.nodes, &plan.semantic_sources)
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


    // ---- RT-FNSPLIT-B2O — static body ownership -----------------------------

    pub(in crate::cranelift_backend) fn b2o_transparent_declaration(body: RuntimeExpr) -> RuntimeDeclaration {
        RuntimeDeclaration {
            symbol: "decl:fixture::b2o".to_string(),
            kind: RuntimeDeclarationKind::Transparent { body },
            metadata: crate::RuntimeSymbolMetadata {
                obligations: Default::default(),
                obligation_metadata: Default::default(),
                assumptions: Default::default(),
                assumption_trust_metadata: Default::default(),
                trusted_base_delta: Default::default(),
                lowerability: None,
                unsupported: None,
                runtime_checks: Default::default(),
                capabilities: Default::default(),
                effects: Default::default(),
            },
        }
    }

    pub(super) fn b2o_retained_closure(body: RuntimeExpr) -> RuntimeExpr {
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["x".to_string()],
            body: Box::new(body),
        }
    }

    pub(super) fn b2o_units(expr: &RuntimeExpr, declarations: &BTreeMap<&str, &RuntimeDeclaration>) -> usize {
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

    /// A fixture with two retained closures **and** a transparent declaration, so
    /// every seed class and both `AC-5` duplicate/overlap shapes are constructible.
    pub(in crate::cranelift_backend) fn b2o_two_closure_fixture() -> RuntimeExpr {
        RuntimeExpr::Let {
            value: Box::new(b2o_retained_closure(unit())),
            body: Box::new(b2o_retained_closure(RuntimeExpr::Var(0))),
        }
    }

    pub(super) fn b2o_err(
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
    pub(super) fn b2o_disposition(plan: &StaticTransitionPlan) -> (usize, usize, usize, usize) {
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

    // ================================================================
    // `RT-FNSPLIT-B2R` — the representation and call-ABI contract.
    //
    // ⛔ Every control below mutates the **graph, the owner partition, or the
    // recorded descriptor** — never a Rust spelling. `AC-8` inverts the usual
    // reflex: a rename, a wrapper, a visibility change, or a `fn` moved between
    // files MUST leave these green, and a pin that reddens on one of those is a
    // defect in the pin, reported as such rather than repaired into greenness.
    // ================================================================



    pub(in crate::cranelift_backend) fn b2r_plan(expr: &RuntimeExpr) -> StaticTransitionPlan<'_> {
        let declarations = BTreeMap::new();
        plan_static_transition_graph(expr, &declarations).expect("plannable")
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
        let abi = include_str!("static_transition/abi.rs");
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
        let abi = include_str!("static_transition/abi.rs");
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
    pub(super) fn b2r_code_identifier_occurrences(source: &str, needle: &str) -> usize {
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

    /// **`RT-DECL-CLOSURE-PORT` `D2` fixture — one program holding BOTH owners.**
    ///
    /// A transparent declaration whose body is a lexical closure seed, and a
    /// separate anonymous lexical closure at the root. ⭐ Both are in the *same*
    /// program on purpose: the property `D2` establishes is a **split**, and a
    /// fixture carrying only the declaration cannot tell "classified by owner"
    /// apart from "classified `CallableDeclaration` unconditionally".
    pub(super) fn d2_declaration_and_anonymous_closure() -> (RuntimeExpr, RuntimeDeclaration) {
        // The declaration's own body: two captures, one parameter.
        let declaration = RuntimeDeclaration {
            symbol: "decl:fixture::d2".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: vec![RuntimeExpr::Var(0), RuntimeExpr::Var(1)],
                    params: vec!["arg0".to_string()],
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        // The anonymous closure: a DIFFERENT arity, so an assertion cannot be
        // satisfied by reading the wrong unit's header and still agreeing.
        let root = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: vec![RuntimeExpr::Var(0)],
                params: Vec::new(),
                body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(7.into()))),
            }),
            args: Vec::new(),
        };
        (root, declaration)
    }

    /// The definition arms of a `D2` plan, paired with each unit's declared
    /// `(parameters, captures)`.
    pub(super) fn d2_units(plan: &StaticTransitionPlan<'_>) -> Vec<(AbiUnitDefinition, (u32, u32))> {
        plan.emittable_units()
            .expect("validated units")
            .into_iter()
            .map(|unit| {
                (
                    unit.definition(),
                    (unit.header().parameters, unit.header().captures),
                )
            })
            .collect()
    }

    /// **`D2` — a transparent closure-seed declaration owns its own callable
    /// unit, and an anonymous closure in the same program does not.**
    #[test]
    fn d2_a_transparent_closure_seed_declaration_owns_a_callable_unit() {
        let (root, declaration) = d2_declaration_and_anonymous_closure();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d2", &declaration);
        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");

        let declaration_origin = plan
            .declaration_occurrence_origin("decl:fixture::d2")
            .expect("the transparent declaration has an occurrence origin");
        let units = d2_units(&plan);

        // The declaration's callable unit: owned by the DECLARATION's occurrence,
        // carrying the closure's own arity. Asserted as an exact member rather
        // than "some unit is a CallableDeclaration", so a unit owned by the
        // wrong occurrence cannot satisfy it.
        assert!(
            units.contains(&(
                AbiUnitDefinition::CallableDeclaration {
                    declaration_origin,
                    provenance: AbiCaptureProvenance::Lexical,
                },
                (1, 2),
            )),
            "D2: the declaration's closure-seed body must be a callable unit \
             owned by the declaration, with the closure's own arity; got {units:?}"
        );

        // The discriminator: the anonymous closure stays an anonymous body. ⛔ Its
        // defining origin is NOT the declaration's, and that is the whole content
        // of "separately owned".
        let anonymous = units
            .iter()
            .filter_map(|(definition, arity)| match definition {
                AbiUnitDefinition::ClosureBody {
                    defining_origin, ..
                } => Some((*defining_origin, *arity)),
                _ => None,
            })
            .collect::<Vec<_>>();
        let [(anonymous_origin, anonymous_arity)] = anonymous[..] else {
            panic!("D2: expected exactly one anonymous ClosureBody unit, got {anonymous:?}");
        };
        assert_eq!(
            anonymous_arity,
            (0, 1),
            "D2: the anonymous closure must keep its own arity, distinct from \
             the declaration's -- reading the wrong unit's header would agree \
             with the declaration's (1, 2) instead"
        );
        assert_ne!(
            anonymous_origin, declaration_origin,
            "D2: the anonymous closure body must not be owned by the declaration"
        );
    }

    /// **`D2` causal control — the derivation, not the fixture, decides.**
    ///
    /// ⭐ Two mutations, and **each catches a defect the other cannot**. Ignoring
    /// ownership reds only the positive arm; claiming universal ownership reds
    /// only the discriminator. A single control here would leave one of the two
    /// wrong derivations green, and both wrong derivations still compile.
    #[test]
    fn d2_the_owner_split_is_causal_in_both_directions() {
        let (root, declaration) = d2_declaration_and_anonymous_closure();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d2", &declaration);

        let arms = |units: &[(AbiUnitDefinition, (u32, u32))]| {
            let callable = units
                .iter()
                .filter(|(definition, _)| {
                    matches!(definition, AbiUnitDefinition::CallableDeclaration { .. })
                })
                .count();
            let bodies = units
                .iter()
                .filter(|(definition, _)| {
                    matches!(definition, AbiUnitDefinition::ClosureBody { .. })
                })
                .count();
            (callable, bodies)
        };

        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");
        assert_eq!(
            arms(&d2_units(&plan)),
            (1, 1),
            "D2: the fixture must hold exactly one unit of each owner"
        );

        // Mutation 1 -- the pre-port derivation. The declaration loses its unit.
        let ignored = super::abi::D2_IGNORE_DECLARATION_OWNERSHIP.with(|flag| {
            flag.set(true);
            let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");
            let observed = arms(&d2_units(&plan));
            flag.set(false);
            observed
        });
        assert_eq!(
            ignored,
            (0, 2),
            "D2: ignoring the owner discriminator must restore the pre-port \
             classification -- if this stays (1, 1) the split is not derived \
             from declaration ownership at all"
        );

        // Mutation 2 -- the opposite defect. The anonymous closure is captured
        // by the new arm, which the positive assertion alone accepts happily.
        let claimed = super::abi::D2_CLAIM_ALL_BODIES_DECLARATION_OWNED.with(|flag| {
            flag.set(true);
            let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");
            let observed = arms(&d2_units(&plan));
            flag.set(false);
            observed
        });
        assert_eq!(
            claimed,
            (2, 0),
            "D2: claiming universal declaration ownership must swallow the \
             anonymous closure -- the discriminator is what rejects this"
        );
    }




    /// **`RT-DECL-CLOSURE-PORT` `D4` fixture — one program holding BOTH target
    /// classes, each actually referenced.**
    ///
    /// ⭐ A closure-seed declaration and a non-closure transparent declaration,
    /// with a `DeclarationRef` to each, because `D4`'s property is a
    /// **partition**. A fixture carrying only the closure seed cannot tell
    /// "retargeted by seed class" apart from "retargeted unconditionally" — and
    /// the unconditional reading is the hazard: a non-closure declaration's
    /// entry *is* its unit, so moving its call breaks every declaration call
    /// the corpus already makes.
    ///
    /// The two declarations carry different arities on purpose, so an assertion
    /// cannot be satisfied by reading the wrong unit's header and still agree.
    pub(super) fn d4_both_target_classes() -> (RuntimeExpr, RuntimeDeclaration, RuntimeDeclaration) {
        let closure_seed = RuntimeDeclaration {
            symbol: "decl:fixture::d4::callable".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: vec![RuntimeExpr::Var(0), RuntimeExpr::Var(1)],
                    params: vec!["arg0".to_string()],
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        let thunk = RuntimeDeclaration {
            symbol: "decl:fixture::d4::thunk".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::Value(RuntimeValue::Int(73.into())),
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        // ⭐ A THIRD closure, anonymous and at the root, carrying an arity that
        // matches neither declaration. It is what makes "the static-body edge
        // leaving THIS declaration's entry" distinguishable from "some
        // static-body edge": with one closure in the program those two
        // derivations agree, and a reverse body search would pass unnoticed.
        let root = RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(7.into()))),
                }),
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::DeclarationRef {
                    symbol: "decl:fixture::d4::callable".to_string(),
                }),
                body: Box::new(RuntimeExpr::DeclarationRef {
                    symbol: "decl:fixture::d4::thunk".to_string(),
                }),
            }),
        };
        (root, closure_seed, thunk)
    }

    /// Every planned declaration call, as
    /// `(recorded class, callee definition, callee (parameters, captures))`.
    ///
    /// ⭐ Joined through the **resolved call edge**, not through the recorded
    /// class alone: the class is what the planner decided, and the descriptor is
    /// where the decision landed. Reading only the class would let a correct
    /// record sit above an edge that went somewhere else entirely.
    pub(super) fn d4_declaration_calls(
        plan: &StaticTransitionPlan<'_>,
    ) -> Vec<(
        DeclarationCallTargetClass,
        AbiUnitDefinition,
        (u32, u32),
    )> {
        let units = plan.emittable_units().expect("validated units");
        let mut calls = plan
            .emittable_call_edges()
            .expect("validated call edges")
            .into_iter()
            .filter(|edge| edge.kind() == EmittableCallKind::Declaration)
            .map(|edge| {
                let unit = units
                    .iter()
                    .find(|unit| unit.function() == edge.callee())
                    .expect("a declaration call edge names an emittable unit");
                (
                    plan.declaration_call_target_class(edge.call_site_origin())
                        .expect("a planned declaration call records its target class"),
                    unit.definition(),
                    (unit.header().parameters, unit.header().captures),
                )
            })
            .collect::<Vec<_>>();
        calls.sort_by_key(|(class, _, _)| *class);
        calls
    }

    /// **`D4` — a closure-seed declaration's call reaches its declaration-owned
    /// callable unit, and a non-closure declaration's call does NOT move.**
    ///
    /// Promise class: durable invariant. It is asserted as an equality over the
    /// whole declaration-call population, so a third class, a lost call, or a
    /// duplicated one all red — none of which a per-call `contains` would see.
    #[test]
    fn d4_the_declaration_call_partition_follows_the_seed_class() {
        let (root, closure_seed, thunk) = d4_both_target_classes();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d4::callable", &closure_seed);
        declarations.insert("decl:fixture::d4::thunk", &thunk);
        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");

        let callable_origin = plan
            .declaration_occurrence_origin("decl:fixture::d4::callable")
            .expect("the closure-seed declaration has an occurrence origin");

        assert_eq!(
            d4_declaration_calls(&plan),
            vec![
                (
                    DeclarationCallTargetClass::SchedulingEntry,
                    AbiUnitDefinition::SchedulingEntry {
                        ingress: abi::AbiSchedulingIngress::Empty,
                    },
                    (0, 0),
                ),
                (
                    DeclarationCallTargetClass::CallableDeclaration,
                    AbiUnitDefinition::CallableDeclaration {
                        declaration_origin: callable_origin,
                        provenance: AbiCaptureProvenance::Lexical,
                    },
                    (1, 2),
                ),
            ],
            "D4: the closure-seed declaration's call must reach the unit that \
             declares its one parameter and two captures, and the non-closure \
             declaration's call must still reach its own zero-input scheduling \
             entry"
        );
    }

    // ─── RT-DECL-CLOSURE-PORT D2a — the function-unit population ───────────
    //
    // ⭐⭐ **One source declaration contributes ONE function.** Before `D2a` a
    // closure-seed transparent declaration contributed two: its `StaticBody`
    // target (the `D2` callable unit) and its own zero-input `SchedulingEntry`
    // at the closure occurrence. The second has no lawful runtime meaning — it
    // cannot call the callable unit without the missing parameters and
    // captures, cannot return the closure, and cannot be a no-op without
    // changing program meaning.

    /// Every class in the ruled partition, in one program.
    ///
    /// ⚠ All four are present **and distinguishable**: the two declaration
    /// closure forms differ in arity, and the anonymous closure differs from
    /// both. A fixture carrying one closure cannot tell "the relation leaving
    /// THIS declaration" from "some relation".
    #[cfg(test)]
    pub(super) fn d2a_every_partition_class() -> (RuntimeExpr, Vec<RuntimeDeclaration>) {
        let lexical = RuntimeDeclaration {
            symbol: "decl:fixture::d2a::lexical".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
                    params: vec!["a".to_string()],
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        let seed = RuntimeDeclaration {
            symbol: "decl:fixture::d2a::seed".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::Closure {
                    captures: Vec::new(),
                    params: vec!["p".to_string(), "q".to_string()],
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        let thunk = RuntimeDeclaration {
            symbol: "decl:fixture::d2a::thunk".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::Value(RuntimeValue::Int(73.into())),
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        // The root, carrying an ANONYMOUS closure that must keep its own
        // `ClosureBody` unit and its own emitted `StaticBody` call.
        let root = RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(7.into()))),
                }),
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::DeclarationRef {
                    symbol: "decl:fixture::d2a::lexical".to_string(),
                }),
                body: Box::new(RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::DeclarationRef {
                        symbol: "decl:fixture::d2a::seed".to_string(),
                    }),
                    body: Box::new(RuntimeExpr::DeclarationRef {
                        symbol: "decl:fixture::d2a::thunk".to_string(),
                    }),
                }),
            }),
        };
        (root, vec![lexical, seed, thunk])
    }

    /// The unit population as a sorted class census, plus the count of emitted
    /// `StaticBody` **calls**.
    #[cfg(test)]
    pub(super) fn d2a_population(plan: &StaticTransitionPlan<'_>) -> (Vec<&'static str>, usize) {
        let mut classes = plan
            .emittable_units()
            .expect("validated units")
            .into_iter()
            .map(|unit| match unit.definition() {
                AbiUnitDefinition::SchedulingEntry { .. } => "SchedulingEntry",
                AbiUnitDefinition::CallableDeclaration { .. } => "CallableDeclaration",
                AbiUnitDefinition::ClosureBody { .. } => "ClosureBody",
                AbiUnitDefinition::ContinuationSpecialization { .. } => {
                    "ContinuationSpecialization"
                }
                // `D2f`: named rather than filtered, so the census stays a
                // TOTAL classification of the planned population. Absorbing the
                // class into another label is how a new unit class becomes
                // invisible to the very control that measures the population.
                AbiUnitDefinition::StaticContinuationFusion { .. } => {
                    "StaticContinuationFusion"
                }
            })
            .collect::<Vec<_>>();
        classes.sort_unstable();
        let static_body_calls = plan
            .emittable_call_edges()
            .expect("validated call edges")
            .into_iter()
            .filter(|edge| edge.kind() == EmittableCallKind::StaticBody)
            .count();
        (classes, static_body_calls)
    }

    /// **`D2a` — the closed partition, stated as a population.**
    #[test]
    fn d2a_one_source_declaration_contributes_exactly_one_function() {
        let (root, declarations) = d2a_every_partition_class();
        let declarations = declarations
            .iter()
            .map(|declaration| (declaration.symbol.as_str(), declaration))
            .collect::<BTreeMap<_, _>>();
        let plan = plan_static_transition_graph_with_symbols(
            &root,
            &declarations,
            &crate::NativeProcessSymbols::legacy_prelude(),
            abi::AbiRootIngress::Value,
            true,
        )
        .expect("the D2a fixture plans");
        let (classes, static_body_calls) = d2a_population(&plan);
        assert_eq!(
            classes,
            vec![
                // the two closure declarations
                "CallableDeclaration",
                "CallableDeclaration",
                // the anonymous closure at the root
                "ClosureBody",
                // the root, and the non-closure thunk declaration
                "SchedulingEntry",
                "SchedulingEntry",
            ],
            "D2a: root + thunk are the ONLY scheduling entries; each closure \
             declaration contributes exactly one callable unit and no separate \
             scheduling entry; the anonymous closure keeps its ClosureBody"
        );
        assert_eq!(
            static_body_calls, 1,
            "D2a: only the ANONYMOUS closure's static-body relation is an \
             emitted call. A declaration-owned pair's relation is a \
             definition/signature relation inside one unit, and emitting it as \
             a call would reintroduce the phantom from the other side"
        );
        // Cross-plane one-for-one: the semantic partition, the ABI descriptors,
        // and the declared function population must state the same result.
        // ⛔ A repair that only skipped `emittable_units` would leave a phantom
        // owner here, which is one of the four explicitly rejected half-measures.
        assert_eq!(
            plan.semantic.functions.len(),
            classes.len(),
            "D2a: the semantic function population must equal the ABI \
             descriptor population exactly"
        );
    }

    /// **`D2a` — the substitution is CAUSAL.**
    #[test]
    fn d2a_retaining_the_obsolete_scheduling_unit_restores_the_phantom() {
        let (root, declarations) = d2a_every_partition_class();
        let declarations = declarations
            .iter()
            .map(|declaration| (declaration.symbol.as_str(), declaration))
            .collect::<BTreeMap<_, _>>();
        let retained = with_d2a_population_mutation(
            D2aPopulationMutation::RetainObsoleteSchedulingUnit,
            || {
                plan_static_transition_graph_with_symbols(
                    &root,
                    &declarations,
                    &crate::NativeProcessSymbols::legacy_prelude(),
                    abi::AbiRootIngress::Value,
                    true,
                )
                .map(|plan| d2a_population(&plan).0)
            },
        );
        let retained = retained.expect("the pre-D2a population still plans");
        assert_eq!(
            retained.iter().filter(|class| **class == "SchedulingEntry").count(),
            4,
            "D2a: with the substitution suppressed, BOTH closure declarations \
             get their obsolete zero-input scheduling entry back — 4 entries \
             where the ruled partition has 2. If this count did not move, the \
             partition assertion above is not caused by D2a: {retained:?}"
        );
    }

    /// **`D4` — the partition is CAUSAL in both directions.**
    ///
    /// ⭐ Two mutations, because one cannot defeat a split. Without
    /// `NeverRetarget` the positive assertion above is consistent with the
    /// retarget never having been installed on a plan that happened to agree;
    /// without `AlwaysRetarget` it is consistent with a blanket retarget that
    /// drags the thunk along and is only accidentally right about the closure
    /// seed.
    #[test]
    fn d4_the_declaration_call_partition_is_causal_in_both_directions() {
        let (root, closure_seed, thunk) = d4_both_target_classes();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d4::callable", &closure_seed);
        declarations.insert("decl:fixture::d4::thunk", &thunk);

        let under = |mutation: D4DeclarationTargetMutation| {
            D4_DECLARATION_TARGET_MUTATION.with(|cell| {
                cell.set(mutation);
                let outcome = plan_static_transition_graph(&root, &declarations)
                    .map(|plan| d4_declaration_calls(&plan));
                cell.set(D4DeclarationTargetMutation::Exact);
                outcome
            })
        };

        // Direction 1 -- the pre-`D4` world, and `D2a` has made it STRICTLY
        // unreachable rather than merely wrong.
        //
        // ⭐ Before `D2a` this mutation planned: both calls landed on a
        // zero-input scheduling entry, so the closure-seed reference called a
        // thunk declaring none of its parameters or captures. That wrong-arity
        // target was the thing `D4` removed. `D2a` removes the target itself —
        // a closure-seed declaration's scheduling entry is no longer a function
        // unit — so the same mutation now cannot be planned at all.
        //
        // ⚠ The assertion was updated because the mechanism got stronger, not
        // because the control was re-fit to whatever the code now does: the
        // direction being measured is unchanged (suppress the retarget, prove
        // the positive partition was caused by the seed discriminator), and it
        // is now measured by a refusal instead of by an arity.
        let never = under(D4DeclarationTargetMutation::NeverRetarget);
        let Err(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason))) = never
        else {
            panic!(
                "D4/D2a: with the retarget suppressed the closure-seed call \
                 targets a scheduling entry that D2a no longer seeds as a unit, \
                 so planning must refuse it: {never:?}"
            );
        };
        assert!(
            reason.contains("declaration call edge target is not its function unit's seed"),
            "D4/D2a: the refusal must name the missing unit seed. Any other \
             invariant would leave the suppressed retarget unpinned: {reason}"
        );

        // Direction 2 -- the ruled-out reverse body search. It lands on the
        // anonymous closure's body, which is a `ClosureBody` unit owned by
        // nobody's declaration. ⛔ It must not merely produce a different
        // partition: a declaration call to an anonymous closure body is not a
        // lawful target at all, so planning refuses it.
        let any = under(D4DeclarationTargetMutation::AnyStaticBody);
        let Err(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason))) = any
        else {
            panic!(
                "D4: a declaration call retargeted by reverse body search reaches \
                 an anonymous closure body, which must be refused rather than \
                 planned: {any:?}"
            );
        };
        assert!(
            reason.contains("neither a scheduling entry nor a callable declaration unit"),
            "D4: the refusal must name the target CLASS -- a refusal for some \
             other invariant would leave the wrong-owner target unpinned: {reason}"
        );
    }

    pub(in crate::cranelift_backend) fn substrate_constructor(name: &str) -> RuntimeExpr {
        RuntimeExpr::Construct {
            constructor: format!("ctor:fixture::Substrate::{name}"),
            args: Vec::new(),
        }
    }

    pub(in crate::cranelift_backend) fn substrate_case(name: &str) -> RuntimeMatchCase {
        RuntimeMatchCase {
            constructor: format!("ctor:fixture::Substrate::{name}"),
            binders: 0,
            body: unit(),
        }
    }

    pub(super) fn substrate_case_fixture() -> RuntimeExpr {
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

    pub(in crate::cranelift_backend) fn contspec_nested_fixture() -> RuntimeExpr {
        let leaf = || RuntimeExpr::Construct {
            constructor: "ctor:fixture::Contspec::Leaf".to_string(),
            args: Vec::new(),
        };
        let inner_worker = RuntimeExpr::LexicalClosure {
            captures: vec![unit()],
            params: vec!["inner".to_string()],
            body: Box::new(leaf()),
        };
        let outer_worker = RuntimeExpr::LexicalClosure {
            captures: vec![unit()],
            params: vec!["outer".to_string()],
            body: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Node".to_string(),
                args: vec![inner_worker],
            }),
        };
        let computational = RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Node".to_string(),
                args: vec![outer_worker],
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
                message: "contspec fixture".to_string(),
            },
        };
        RuntimeExpr::LexicalClosure {
            captures: vec![unit()],
            params: vec!["input".to_string()],
            body: Box::new(computational),
        }
    }

    pub(in crate::cranelift_backend) fn contspec_plan() -> StaticTransitionPlan<'static> {
        let expr = Box::leak(Box::new(contspec_nested_fixture()));
        plan_static_transition_graph(expr, &BTreeMap::new()).expect("contspec fixture plans")
    }

    pub(in crate::cranelift_backend) fn contspec_parameter_match(case_body: RuntimeExpr) -> RuntimeExpr {
        let worker = RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["worker".to_string()],
            body: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                args: Vec::new(),
            }),
        };
        RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Node".to_string(),
                args: vec![worker],
            }),
            cases: vec![
                RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: unit(),
                },
                RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: case_body,
                },
            ],
            default: trap("persistent continuation result"),
        }
    }


    pub(super) fn contspec_complete_environment_fixture() -> RuntimeExpr {
        RuntimeExpr::Let {
            // Rebind process parameter 1 at de Bruijn ordinal 0. The
            // continuation environment is `[source 1, source 0, source 1]`,
            // not the consumer descriptor's declared `[source 0, source 1]`.
            value: Box::new(RuntimeExpr::Var(1)),
            body: Box::new(contspec_parameter_match(RuntimeExpr::Var(4))),
        }
    }

    pub(super) fn contspec_required_tail_fixture(tail: RuntimeExpr) -> RuntimeExpr {
        RuntimeExpr::Let {
            value: Box::new(tail),
            body: Box::new(RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::Var(1)),
                body: Box::new(RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::Var(2)),
                    body: Box::new(contspec_parameter_match(RuntimeExpr::Var(4))),
                }),
            }),
        }
    }

















    /// A `ComputationalMatch` whose single case has **one recursive position
    /// and one ordinary argument binder**, with a persistent scrutinee, and
    /// whose case body is itself a `ComputationalMatch`.
    ///
    /// Walking to that inner occurrence lands exactly on the outer case's own
    /// environment, which is the run this discriminator is about:
    /// `[IH, argument, outer...]`.
    pub(super) fn contsrc_d2_ih_and_argument_case_fixture() -> RuntimeExpr {
        RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Node".to_string(),
                args: vec![unit(), unit()],
            }),
            cases: vec![RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::Contspec::Node".to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: contspec_parameter_match(RuntimeExpr::Var(3)),
            }],
            default: trap("d2 ih and argument"),
        }
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
    pub(super) fn d7_1c_member_and_ordinary_twin_fixture() -> RuntimeExpr {
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
    pub(super) fn d7_1c_member_and_ordinary_body_counts(plan: &StaticTransitionPlan<'_>) -> (usize, usize) {
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
