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
/// Fixture re-export for the lowering-side reconcile controls; see
/// `planning.rs`. Test-only, and deliberately re-exporting the one fixture
/// rather than opening the module.
#[cfg(test)]
pub(in crate::cranelift_backend) use continuations::tests::contspec_activation_owned_worker_captures_fixture;
mod effects;
mod joins_traps;
mod occurrences;
mod responses;
mod semantic_ir;
mod units;

// `RT-CAPTURE-PROJECTION-GROW` `D1` — the deferral ledger's cross-crate surface,
// reached as `ken_runtime::{with_worker_prefix_deferrals, WorkerPrefixDeferral}`.
// Same reachability caveat as the sibling observers: only a consumer can observe
// a break in this path, so the `ken-cli` control is what keeps it honest.
#[cfg(feature = "px8-ds-test-support")]
pub use continuations::{with_worker_prefix_deferrals, WorkerPrefixDeferral};

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
use semantic_ir::{
    build_bool_constructor_inventory, build_synthesized_constructor_inventory,
};
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
    BoolMatchCaseOrdinals, ConstructorIdentity, FieldIdentity, SynthesizedConstructorRole,
    SynthesizedFixedConstructorRole,
};
pub(in crate::cranelift_backend) use occurrences::StaticOriginId;
#[allow(unused_imports)]
pub(in crate::cranelift_backend) use responses::{
    DeferredResponseRow, DeferredResponseSubCase, ResponseDisposition, SsaInfeasible,
    StaticResponseCapture, StaticResponseContextDemand, StaticResponseContinuation,
    StaticResponseContinuationId, StaticResponseEffectInput, StaticResponseEnvironmentBinding,
    StaticResponseFrameSource, StaticResponseOwnerId, StaticResponseOwnerSpecialization,
    StaticResponsePhaseA,
};
#[cfg(feature = "px8-ds-test-support")]
pub use responses::{
    mixed_owner_execute_then_resume_overpromotion_is_exact,
    static_response_context_demand_mutation_is_exact,
    suppressed_execute_then_resume_response_is_exact,
    with_mixed_owner_execute_then_resume_overpromotion,
    with_static_response_context_demand_mutation, with_suppressed_execute_then_resume_response,
    StaticResponseContextDemandMutation,
};
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
#[allow(unused_imports)]
use continuations::{
    CONTINUATION_INTERN_MUTATION, CONTINUATION_PRODUCTION_MUTATION, COMPOSED_CALL_TARGET_DEFECT, WEAKEN_CONTINUATION_DECREASING_MEASURE, SUPPRESS_POST_SPECIALIZATION_DESCENT, DUPLICATE_STATIC_BODY_TRIPLE, ENVELOPE_DEFECT,
};
// `RT-BACKEND-SPLIT-CLOSURE` (item 18): `validate_continuation_specialization_
// closure`/`ContinuationProjectionOmission`/`ContinuationInternMutation`
// narrowed away here -- compiler-flagged unused, independently re-verified
// crate-wide (including the `lowering/` tree, not just this file) before
// narrowing: neither is consumed anywhere outside `construction.rs`/
// `closure.rs`'s own direct declaration/use, and `planning.rs` does not
// re-export either. `ContinuationProductionMutation` alone stays.
#[cfg(test)]
use continuations::ContinuationProductionMutation;

#[cfg(feature = "px8-ds-test-support")]
pub use aggregates::{
    checked_ih_continuation_inheritance_mutation_is_exact,
    checked_ih_generated_entry_admission_mutation_is_exact,
    checked_ih_generated_entry_arrival_mutation_is_exact,
    checked_ih_generated_entry_confluence_mutation_is_exact,
    composed_return_forward_ret_authority_mutation_is_exact,
    retained_result_closure_proof_mutation_applied,
    retained_result_closure_proof_mutation_is_exact,
    with_checked_ih_continuation_inheritance_mutation,
    with_checked_ih_continuation_inheritance_observations,
    with_checked_ih_generated_entry_admission_mutation,
    with_checked_ih_generated_entry_admission_observations,
    with_checked_ih_generated_entry_arrival_mutation,
    with_checked_ih_generated_entry_confluence_mutation,
    with_checked_ih_generated_entry_observations,
    with_composed_return_forward_edge_collapsibility_observations,
    with_composed_return_forward_ret_authority_mutation,
    with_composed_return_forward_ret_role_witnesses,
    with_retained_result_closure_proof_mutation, CheckedIhContinuationInheritanceMutation,
    CheckedIhContinuationInheritanceObservation, CheckedIhGeneratedEntryAdmissionMutation,
    CheckedIhGeneratedEntryAdmissionObservation, CheckedIhGeneratedEntryArrivalMutation,
    CheckedIhGeneratedEntryConfluenceMutation, CheckedIhGeneratedEntryObservation,
    ComposedReturnForwardEdgeCollapsibilityObservation,
    ComposedReturnForwardRetAuthorityMutation, ComposedReturnForwardRetAuthorityObservation,
    ComposedReturnForwardRetRoleWitnessObservation,
    ComposedReturnForwardRetCoordinateObservation, RetainedResultClosureProofMutation,
};

#[cfg(feature = "px8-ds-test-support")]
pub(in crate::cranelift_backend) use aggregates::{
    checked_ih_generated_entry_arrival_mutation, composed_return_forward_ret_authority_mutation,
    discharge_forward_edge_sealed_observations,
    record_checked_ih_generated_entry_governed_validation,
    record_checked_ih_generated_entry_installed,
    record_checked_ih_generated_entry_ordinary_continuation,
    record_checked_ih_generated_entry_raw_arrival, record_checked_ih_generated_entry_reached,
    record_composed_return_forward_edge_collapsibility,
    record_composed_return_forward_ret_authority,
    record_composed_return_forward_ret_role_witness,
    take_composed_return_forward_ret_population_mutation,
};

// `RT-PLANNER-AGGREGATES-SPLIT` `D1` — the aggregates domain's cross-boundary
// surface: `lowering` and `planning`'s own re-export both reach these through
// this module, unchanged from before the move.
#[allow(unused_imports)]
pub(in crate::cranelift_backend) use aggregates::{
    AggregateOccurrenceId, AggregateOccurrenceProducer, BoundaryClosureEnvironment,
    CheckedIhCapabilityInheritance, CheckedIhContinuationInheritance,
    CheckedIhContinuationInheritanceView, CheckedIhEnvironmentTransport,
    CheckedIhForwardRetPlanProof, CheckedIhFreshResultDestination,
    CheckedIhFreshResultRoute, CheckedIhGeneratedEntryAccess,
    CheckedIhGeneratedEntryAdmission, CheckedIhGeneratedEntryCallCoordinate,
    CheckedIhGeneratedEntryProjection,
    CheckedIhImmediateKBindingLocator,
    CheckedIhKAvailabilityDomain, CheckedIhTransportInputDestination,
    PlannedAggregateAllocation, PlannedAggregateOwnership,
    PlannedAggregateShape, SynthesizedAggregateNode, SynthesizedAggregatePath,
    SynthesizedAggregateRole, SynthesizedAggregateRoot, SynthesizedDynamicSet,
};
use aggregates::{
    lifetime_referent_affinity, CheckedIhGeneratedEntryConfluence,
    CheckedIhGeneratedEntryCoordinate,
};
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
    host_effect_seat_contract_of, EffectSeatAvail, EffectSeatConstructorPath, EffectSeatNeed,
    EffectSeatOperation, EffectSeatPhase, EffectSeatSlot, PlannedEffectSeat,
    CRANELIFT_HOST_EFFECT_CONSUMERS_V1,
};
#[cfg(test)]
pub(in crate::cranelift_backend) use effects::{set_effect_seat_plan_mutation, EffectSeatPlanMutation};

// `RT-PLANNER-JOINS-TRAPS-SPLIT` `D1` — the joins-traps domain's
// cross-boundary surface: `lowering` and `planning`'s own re-export both
// reach these through this module, unchanged from before the move.
#[allow(unused_imports)]
pub(in crate::cranelift_backend) use joins_traps::{
    dead_arm_effect_trap, malformed_dynamic_constructor_trap, planned_partiality_trap,
    JoinPlanToken, JoinResultRepresentation,
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
    /// `RT-DECL-CLOSURE-PORT` `D5a`. The generated producer execution contexts.
    /// Causal-call demands retain the exact prefix produced by specialization
    /// planning; validated static-response demands append through the same
    /// `(specialization, worker body)` interner before this population's ABI is
    /// installed.
    continuation_contexts: Vec<PlannedContinuationContext>,
    /// Every validated response edge resolved through the installed union
    /// context population. Empty exactly when `static_response_infeasible` is
    /// populated or the complete response population is empty.
    static_response_continuations: Vec<StaticResponseContinuation>,
    /// Distinguishes a lawfully empty installed response population from the
    /// pre-install draft state; neither row count nor infeasibility can do so.
    static_response_plan_installed: bool,
    /// The typed fail-closed result for a genuinely opaque/dynamic response K
    /// or a source that cannot be expressed in the existing typed schema.
    static_response_infeasible: Option<SsaInfeasible>,
    /// The complete Deferred residual. P1 is a response with no continuation
    /// unit. In an open plane, transport-source K callers remain P2 so the plane
    /// is not partially specialized. An eligible closed plane with at least two
    /// exclusively-predeclared producer groups turns those emissions into owner calls;
    /// a single-stage plane retains the forward-Ret path. The suppression
    /// control restores P2. Deferred responses acquire no owner or
    /// placeholder and fall through to ordinary lowering.
    static_response_deferred: Vec<DeferredResponseRow>,
    /// Phase-A carry of the two-phase response context install (RECUT 2, HS5):
    /// the owner-less demand + P1 population minted at install
    /// (construction.rs:1213), consumed by
    /// `install_static_response_context_plan_phase_b` post-:1251. `Some` after
    /// phase A on a feasible plane; `None` before phase A, after phase B (taken),
    /// or on an opaque-K refusal (which sets `static_response_infeasible` instead).
    static_response_phase_a: Option<StaticResponsePhaseA>,
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
    /// The exact two-endpoint transports that carry a force-materialized
    /// checked-IH environment to an escaping closure crossing. These reference
    /// `aggregate_ownership`; they never issue a second record.
    checked_ih_environment_transports: Vec<CheckedIhEnvironmentTransport>,
    /// Planner-only successor projections proving, separately, that an
    /// existing captured continuation capability remains in scope at a
    /// descendant checked invocation and that its conditional fresh result has
    /// one ordinary Ret/capture destination. This plane is inert: lowering has
    /// no consumer in this predecessor.
    checked_ih_continuation_inheritances: Vec<CheckedIhContinuationInheritance>,
    /// Planner-owned confluence certificates proving that all source-specific
    /// inheritances at one generated entry agree on one typed consumer
    /// projection. Source identities remain class members; D2 may move one
    /// exact selected member into a compiler-only proof after membership and
    /// projection equality close, never into a runtime value or carrier.
    checked_ih_generated_entry_confluences:
        BTreeMap<CheckedIhGeneratedEntryCoordinate, CheckedIhGeneratedEntryConfluence>,
    /// Total sanitized admission maps, one per generated context carrying a
    /// governed confluence. Built and validated in the planner before lowering
    /// can clone one into function-local compile-time state.
    checked_ih_generated_entry_accesses:
        BTreeMap<ContinuationContextId, CheckedIhGeneratedEntryAccess>,
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


/// Every emission owner under which an inline synthesized aggregate is built.
///
/// A seat is emitted by its own predeclared unit. It is also emitted under each
/// continuation specialization whose exact selected case body contains it.
/// Those specialization bodies are the population lowering actually enters
/// under `defining_emission_owner = Specialization(unit.id())`; generated
/// continuation contexts are a narrower, post-hoc population and therefore
/// cannot authorize these records.
///
/// This authority applies to synthesized aggregates constructed inline at
/// `seat`: host-result constructors and unit-boundary environments. A checked-IH
/// environment is force-emitted at a different seat and uses its explicit force
/// relation instead.
fn inline_synthesized_seat_emission_owners(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
) -> Result<Vec<ContinuationEmissionOwner>, CraneliftBackendError> {
    let mut owners = Vec::new();
    if let Some(predeclared) = plan.semantic.function_owner(seat)? {
        owners.push(ContinuationEmissionOwner::Predeclared(predeclared));
    }
    for unit in plan.continuation_units()? {
        let frame = plan.planned_occurrence_expr(unit.continuation_origin())?;
        let RuntimeExpr::ComputationalMatch { cases, .. } = frame else {
            return Err(planner_error(
                "a continuation specialization's continuation origin is not a computational \
                 frame, so its emitted body cannot be identified",
            ));
        };
        let alternative = unit.producer_alternative() as usize;
        if cases.get(alternative).is_none() {
            return Err(planner_error(
                "a continuation specialization's selected alternative is outside its \
                 computational frame",
            ));
        }
        let body_position = alternative
            .checked_add(1)
            .ok_or_else(|| planner_capacity_error("continuation case position overflows"))?;
        let body = plan.semantic.child_origin(unit.continuation_origin(), body_position)?;
        if occurrence_subtree_contains(plan, body, seat)? {
            owners.push(ContinuationEmissionOwner::Specialization(unit.id()));
        }
    }
    if plan.static_response_plan_installed {
        let responses = match plan.static_response_feasibility_ledger_all()? {
            Ok(responses) => responses,
            Err(_) => Vec::new(),
        };
        owners.extend(
            responses
                .into_iter()
                .filter(|response| response.effect_origin() == seat)
                .map(|response| response.base_owner()),
        );
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
    let plan = planner.finish(symbols, root_ingress, functionized_units)?;
    #[cfg(feature = "px8-ds-test-support")]
    record_static_response_feasibility_diagnostic(&plan)?;
    Ok(plan)
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticResponseCaptureObservation {
    pub ordinal: u32,
    pub origin: u32,
    pub source: String,
    pub producer_abi_slot: u32,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticResponseFeasibilityObservation {
    pub base_owner: String,
    pub producer_call_origin: u32,
    pub response_origin: u32,
    pub vis_origin: u32,
    pub operation: String,
    pub k_identity: String,
    pub k_specialization: u32,
    pub k_closure_origin: u32,
    pub k_body_origin: u32,
    pub k_context: u32,
    pub context_was_preexisting: bool,
    pub captures: Vec<StaticResponseCaptureObservation>,
    pub continuation_inputs: Vec<(u32, String, u32)>,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticResponseInfeasibleObservation {
    pub base_owner: String,
    pub vis_origin: u32,
    pub producer_call_origin: Option<u32>,
    pub operation: Option<String>,
    pub k_closure_origin: Option<u32>,
    pub k_body_origin: Option<u32>,
    pub k_capture_count: Option<usize>,
    pub continuation_input_count: Option<usize>,
    pub reason: String,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticResponseOwnerObservation {
    pub owner: u32,
    pub base_owner: String,
    pub response: u32,
    pub selected_caller: String,
    pub k_context: u32,
    pub context_was_preexisting: bool,
    pub parameters: u32,
    pub captures: u32,
    pub frame_bytes: u32,
    pub slots: Vec<(String, u32)>,
}

/// One `Deferred` residual row observed for AC-1 congruence and AC-4/AC-5/AC-7
/// controls (recut amendment `evt_4ar3rxzrra5v4`).
#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredResponseObservation {
    pub vis_origin: u32,
    /// The producer call origin this Deferred residual belongs to. Retained for
    /// the closed diagnostic relation and the P2 suppression control.
    pub producer_call_origin: u32,
    pub operation_root_origin: u32,
    pub effect_origin: u32,
    pub operation: String,
    /// "NoContinuationUnit" (P1) or "UnconsumedTransportCaller" (ineligible
    /// or test-suppressed P2).
    pub sub_case: String,
    /// The K's capture / continuation-input counts (P2 from the demand, P1
    /// zero). Eligible-plane has-K census comes from Specialized rows.
    pub capture_count: usize,
    pub continuation_input_count: usize,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticResponseFeasibilityDiagnostic {
    pub static_response_rows: Vec<StaticResponseFeasibilityObservation>,
    pub static_response_infeasible: Option<StaticResponseInfeasibleObservation>,
    pub all_static_response_rows: Vec<StaticResponseFeasibilityObservation>,
    pub all_static_response_infeasible: Option<StaticResponseInfeasibleObservation>,
    pub static_response_owners: Vec<StaticResponseOwnerObservation>,
    /// The complete Deferred residual: P1 plus ineligible or test-suppressed
    /// P2. Together with the Specialized rows this is the full response-Vis
    /// classification.
    pub static_response_deferred: Vec<DeferredResponseObservation>,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static STATIC_RESPONSE_FEASIBILITY_DIAGNOSTICS:
        std::cell::RefCell<Option<Vec<StaticResponseFeasibilityDiagnostic>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_static_response_feasibility_diagnostics<T>(
    operation: impl FnOnce() -> T,
) -> (T, Vec<StaticResponseFeasibilityDiagnostic>) {
    STATIC_RESPONSE_FEASIBILITY_DIAGNOSTICS.with(|slot| {
        assert!(
            slot.borrow().is_none(),
            "static-response feasibility observation windows cannot nest"
        );
        *slot.borrow_mut() = Some(Vec::new());
    });
    let result = operation();
    let observations = STATIC_RESPONSE_FEASIBILITY_DIAGNOSTICS.with(|slot| {
        slot.borrow_mut()
            .take()
            .expect("static-response feasibility observation window")
    });
    (result, observations)
}

#[cfg(feature = "px8-ds-test-support")]
fn record_static_response_feasibility_diagnostic(
    plan: &StaticTransitionPlan<'_>,
) -> Result<(), CraneliftBackendError> {
    let observe = |result: Result<Vec<StaticResponseContinuation>, SsaInfeasible>| match result {
        Ok(rows) => {
            let observations = rows
                .iter()
                .map(|row| StaticResponseFeasibilityObservation {
                    base_owner: format!("{:?}", row.base_owner()),
                    producer_call_origin: row.producer_call_origin().0,
                    response_origin: row.response_origin().0,
                    vis_origin: row.vis_origin().0,
                    operation: format!("{:?}", row.operation()),
                    k_identity: format!("{:?}", row.k_identity()),
                    k_specialization: row.k_specialization().0,
                    k_closure_origin: row.k_closure_origin().0,
                    k_body_origin: row.k_body_origin().0,
                    k_context: row.k_context().0,
                    context_was_preexisting: row.context_was_preexisting(),
                    captures: row
                        .captures()
                        .iter()
                        .map(|capture| StaticResponseCaptureObservation {
                            ordinal: capture.ordinal(),
                            origin: capture.origin().0,
                            source: format!("{:?}", capture.source()),
                            producer_abi_slot: capture.producer_abi_slot(),
                        })
                        .collect(),
                    continuation_inputs: row
                        .continuation_inputs()
                        .iter()
                        .map(|(ordinal, source, slot)| (*ordinal, format!("{source:?}"), *slot))
                        .collect(),
                })
                .collect();
            (observations, None)
        }
        Err(infeasible) => (
            Vec::new(),
            Some(StaticResponseInfeasibleObservation {
                base_owner: format!("{:?}", infeasible.base_owner()),
                vis_origin: infeasible.vis_origin().0,
                producer_call_origin: infeasible.producer_call_origin().map(|origin| origin.0),
                operation: infeasible.operation().map(|operation| format!("{operation:?}")),
                k_closure_origin: infeasible.k_closure_origin().map(|origin| origin.0),
                k_body_origin: infeasible.k_body_origin().map(|origin| origin.0),
                k_capture_count: infeasible.k_capture_count(),
                continuation_input_count: infeasible.continuation_input_count(),
                reason: infeasible.reason().to_string(),
            }),
        ),
    };
    let (static_response_rows, static_response_infeasible) = observe(
        plan.static_response_feasibility_ledger(ken_host::HostOpV1::BufferAllocate)?,
    );
    let (all_static_response_rows, all_static_response_infeasible) =
        observe(plan.static_response_feasibility_ledger_all()?);
    let static_response_owners = match plan.static_response_owner_specializations()? {
        Ok(owners) => owners
            .iter()
            .map(|owner| StaticResponseOwnerObservation {
                owner: owner.id().ordinal(),
                base_owner: format!("{:?}", owner.base_owner()),
                response: owner.response().ordinal(),
                selected_caller: format!("{:?}", owner.selected_caller()),
                k_context: owner.k_context().0,
                context_was_preexisting: owner.context_was_preexisting(),
                parameters: owner.header().parameters,
                captures: owner.header().captures,
                frame_bytes: owner.header().frame_bytes,
                slots: owner
                    .slots()
                    .iter()
                    .map(|slot| (format!("{:?}", slot.kind), slot.ordinal))
                    .collect(),
            })
            .collect(),
        Err(_) => Vec::new(),
    };
    let static_response_deferred = plan
        .static_response_deferred()
        .iter()
        .map(|row| DeferredResponseObservation {
            vis_origin: row.vis_origin().0,
            producer_call_origin: row.producer_call_origin().0,
            operation_root_origin: row.operation_root_origin().0,
            effect_origin: row.effect_origin().0,
            operation: format!("{:?}", row.operation()),
            sub_case: format!("{:?}", row.sub_case()),
            capture_count: row.capture_count(),
            continuation_input_count: row.continuation_input_count(),
        })
        .collect();
    STATIC_RESPONSE_FEASIBILITY_DIAGNOSTICS.with(|slot| {
        if let Some(rows) = slot.borrow_mut().as_mut() {
            rows.push(StaticResponseFeasibilityDiagnostic {
                static_response_rows,
                static_response_infeasible,
                all_static_response_rows,
                all_static_response_infeasible,
                static_response_owners,
                static_response_deferred,
            });
        }
    });
    Ok(())
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
                    kind: crate::CheckedComputationalIHInvocationKind::OrdinaryApplication,
                    binder_morphism:
                        crate::CheckedComputationalIHBinderMorphism::identity_for_test(0),
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

    use super::abi::{AbiCarrier, AbiSlot};


    pub(super) const D2G_DECLARATION: &str = "decl:fixture::d2g";

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
                kind,
                binder_morphism,
                body,
            } => RuntimeExpr::CheckedComputationalIHInvocation {
                call_template_id: if cause == D2jCause::Invocation {
                    call_template_id + 90
                } else {
                    call_template_id
                },
                checked_occurrence_path,
                kind,
                binder_morphism,
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


    use super::semantic_ir::{RuntimeExprShape, SemanticSourceKind};
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

    /// A fixture with two retained closures **and** a transparent declaration, so
    /// every seed class and both `AC-5` duplicate/overlap shapes are constructible.
    pub(in crate::cranelift_backend) fn b2o_two_closure_fixture() -> RuntimeExpr {
        RuntimeExpr::Let {
            value: Box::new(b2o_retained_closure(unit())),
            body: Box::new(b2o_retained_closure(RuntimeExpr::Var(0))),
        }
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
}
