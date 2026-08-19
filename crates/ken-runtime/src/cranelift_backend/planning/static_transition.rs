//! Factored, pre-emission native transition planner.
//!
//! Node code identity is `(transition kind, static node id)` and edge code
//! identity is `(edge kind, static edge id)`. Dynamic environment,
//! continuation, cleanup, source, and affine state travels as constant-width
//! IDs into hash-consed persistent stores.

mod abi;
mod aggregates;
mod continuations;
mod occurrences;
mod semantic_ir;
mod units;

#[cfg(test)]
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use super::{
    backend, unsupported, BackendFailure, CraneliftBackendError, RuntimeDeclaration,
    RuntimeDeclarationKind,
};
#[cfg_attr(not(test), allow(unused_imports))]
use crate::boundary_value::{BoundaryClass, BoundaryReferentOwner, BoundaryTag};
use crate::{RuntimeExpr, RuntimePartiality, RuntimeTrap, RuntimeTrapCode};
use abi::{
    build_abi_plane, install_continuation_context_abi, install_continuation_specialization_abi,
    AbiPlane,
};
use semantic_ir::{
    build_semantic_plane, build_synthesized_constructor_inventory, SemanticMaterialArena,
    SemanticPlane, SemanticSourceKind, SemanticSourceSeed,
};
#[cfg_attr(not(test), allow(unused_imports))]
use semantic_ir::RuntimeExprShape;
// `RT-CONTSRC-PRODUCER-LOCAL` `D2` — the shape vocabulary, so a producer-local
// contract asks the existing `abi::result_carrier` authority rather than
// restating a carrier.
use occurrences::{
    build_occurrence_authority_plan, occurrence_authority, occurrence_subtree_contains, origin_of,
    validate_occurrence_authority_plan, PlannedOccurrence, PlannedOccurrenceAuthority,
    PlannedOccurrenceChildAuthority,
};

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
    SynthesizedIoErrorRole,
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
use continuations::{
    finalize_continuation_availability_plan, PlannedContinuationSpecialization, PlannedContinuationSpecializationCall, build_continuation_specialization_plan, validate_continuation_specialization_plan,
};
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
    AggregateOccurrenceId, PlannedAggregateAllocation, PlannedAggregateOwnership,
    PlannedAggregateShape, SynthesizedAggregateNode, SynthesizedAggregatePath,
    SynthesizedAggregateRoot, SynthesizedDynamicSet,
};
use aggregates::{
    build_aggregate_ownership_plan, collect_site_operand_ordinals, host_effect_recipe_tree,
    lifetime_referent_affinity, validate_aggregate_ownership_plan,
};
#[cfg(test)]
use aggregates::{
    aggregate_child_referent_owners, fixed_node_selected_owner,
    flatten_allocation_reachable_uses, node_referent_owners,
    validate_aggregate_producers_are_unique, AggregateOccurrenceProducer,
    SynthesizedAggregateRole, SynthesizedAggregateStep,
};

pub(super) const MAX_HELPERS_PER_STATIC_SOURCE: usize = 8;

#[cfg(test)]
thread_local! {
    static ACTIVE_RECURSIVE_LOWERING_FRAMES: Cell<usize> = const { Cell::new(0) };
    static MAX_RECURSIVE_LOWERING_FRAMES: Cell<usize> = const { Cell::new(0) };
}

/// Test-only observation of the actual `plan_expr` call stack.
///
/// The guard is entered inside `plan_expr`, so `Drop` runs on every `?` return
/// as well as the ordinary path. This measures production recursion rather than
/// deriving a proxy from bracket depth or expression-node counts.
#[cfg(test)]
struct RecursiveLoweringFrameGuard;

#[cfg(test)]
impl RecursiveLoweringFrameGuard {
    fn enter() -> Self {
        ACTIVE_RECURSIVE_LOWERING_FRAMES.with(|active| {
            let depth = active
                .get()
                .checked_add(1)
                .expect("recursive lowering frame count fits usize");
            active.set(depth);
            MAX_RECURSIVE_LOWERING_FRAMES.with(|maximum| {
                maximum.set(maximum.get().max(depth));
            });
        });
        Self
    }
}

#[cfg(test)]
impl Drop for RecursiveLoweringFrameGuard {
    fn drop(&mut self) {
        ACTIVE_RECURSIVE_LOWERING_FRAMES.with(|active| {
            active.set(
                active
                    .get()
                    .checked_sub(1)
                    .expect("recursive lowering frame guard is balanced"),
            );
        });
    }
}

#[cfg(test)]
fn reset_recursive_lowering_frame_count() {
    ACTIVE_RECURSIVE_LOWERING_FRAMES.with(|active| active.set(0));
    MAX_RECURSIVE_LOWERING_FRAMES.with(|maximum| maximum.set(0));
}

#[cfg(test)]
fn max_recursive_lowering_frame_count() -> usize {
    MAX_RECURSIVE_LOWERING_FRAMES.with(Cell::get)
}

/// ⭐ The dual result of planning one expression.
///
/// One `StaticNodeId` was previously made to mean two different things:
///
/// - **`entry`** — the first node the transfer graph *schedules* for the
///   expression;
/// - **`occurrence`** — the node on which `SemanticSourceSeed::expression`
///   registered that `RuntimeExpr`, and from which its positional child-origin
///   record is read.
///
/// They coincide for every ordinary form and **deliberately do not** for
/// `ComputationalMatch`, whose occurrence is registered on its
/// `SourceReturnResume` while the parent must still schedule its scrutinee.
/// Returning one value for both made a parent record the scrutinee's identity as
/// its child's origin — a category error, not an off-by-one.
///
/// ⛔ **The two fields have disjoint consumers, and that is the whole mechanism.**
/// Transfer topology consumes **only `.entry`**; source correspondence consumes
/// **only `.occurrence`**. This adds no node, no origin, no search and no
/// arithmetic: both values are outputs of the same recursive visit, and
/// `occurrence` is the origin already assigned to the already-existing semantic
/// seed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlannedExpr {
    entry: StaticNodeId,
    occurrence: StaticOriginId,
}

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

/// **`RT-DECL-CLOSURE-PORT` `D4` causal control — the two ways the selective
/// retarget can be wrong, both compile-preserving.**
///
/// ⭐ Two settings and not one, because a retarget can be wrong by not moving
/// **and** by moving to the wrong place, and no single mutation shows both.
///
/// `NeverRetarget` is the pre-`D4` world: the closure-seed reference keeps its
/// zero-input scheduling entry, which is the wrong-arity target `D4` removes.
/// `AnyStaticBody` is the ruled-out *reverse body search*: it takes the first
/// static-body target in the graph instead of the one leaving this
/// declaration's own entry. ⛔ The second is the one a positive assertion is
/// most likely to agree with by accident, because a program with one closure
/// gives both spellings the same answer — which is why the fixture carries a
/// second, anonymous closure.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum D4DeclarationTargetMutation {
    Exact,
    /// Never follow the static-body edge: the pre-`D4` behaviour.
    NeverRetarget,
    /// Follow the first static-body edge in the graph, whoever it leaves.
    AnyStaticBody,
}

#[cfg(test)]
thread_local! {
    static D4_DECLARATION_TARGET_MUTATION: std::cell::Cell<D4DeclarationTargetMutation> =
        const { std::cell::Cell::new(D4DeclarationTargetMutation::Exact) };
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

#[derive(Clone, Copy)]
struct PlanContext {
    environment: PersistentNodeId,
    continuation: PersistentNodeId,
    path: PersistentNodeId,
    cleanup: PersistentNodeId,
    affine: PersistentNodeId,
    source_return: PersistentNodeId,
}


/// The complete, pre-emission result representation of one source join.
///
/// This is deliberately a two-way type rather than a phase bit threaded through
/// lowering.  In particular, lowering cannot add a third representation or
/// select one from an emitted predecessor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum JoinResultRepresentation {
    NativeScalarPair,
    CarrierWord,
}

/// Move-only evidence that a particular source join was planned.
///
/// Fields and construction stay in the planner.  Lowering can consume the
/// token and inspect the closed representation, but cannot manufacture a token
/// from an origin or a diagnostic label.
#[derive(Debug)]
pub(in crate::cranelift_backend) struct JoinPlanToken {
    pub(in crate::cranelift_backend) origin: StaticOriginId,
    pub(in crate::cranelift_backend) representation: JoinResultRepresentation,
    pub(in crate::cranelift_backend) has_continuing_predecessor: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlannedJoinResult {
    representation: JoinResultRepresentation,
    has_continuing_predecessor: bool,
}

/// A closed producer result for one exact match scrutinee.
///
/// `Open` is positive fail-closed authority: at least one result route is not
/// statically known. An empty `Closed` set means the expression cannot return
/// normally, not that the analysis forgot to inspect it.
#[derive(Clone, Debug, Eq, PartialEq)]
enum CaseProducerSet {
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
struct CaseProducerAuthority {
    producers: CaseProducerSet,
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
struct PlannedCaseEmission {
    match_origin: StaticOriginId,
    scrutinee_origin: StaticOriginId,
    owner: PredeclaredFunctionId,
    ordinal: u32,
    body_origin: StaticOriginId,
    constructor: ConstructorIdentity,
    authority: CaseProducerAuthority,
    status: CaseEmissionStatus,
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

/// A nonzero identity for one exact trap value interned by the planner.
///
/// The word travels only through [`AbiCarrier::TrapWord`]. It is not a source
/// value and cannot be constructed by lowering.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(in crate::cranelift_backend) struct PlannedTrapIdentity(u32);

impl PlannedTrapIdentity {
    pub(in crate::cranelift_backend) fn abi_word(self) -> i64 {
        i64::from(self.0)
    }
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

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct BoundaryACensus {
    static_nodes: usize,
    edges: usize,
    planned_helpers: usize,
    persistent_store_nodes: usize,
    out_of_line_evidence_records: usize,
    max_helpers_per_static_source: usize,
    helper_key_bytes: usize,
    activation_frame_bytes: usize,
    store_node_bytes: usize,
    helper_key_schemas: usize,
    frame_schemas: usize,
    store_node_schemas: usize,
    static_node_id_bytes: usize,
    persistent_node_id_bytes: usize,
    max_logical_chain_depth: u32,
    max_environment_depth: u32,
    max_continuation_depth: u32,
    max_path_depth: u32,
    max_cleanup_depth: u32,
    max_affine_depth: u32,
    max_source_return_depth: u32,
    source_return_resume_nodes: usize,
    source_return_owned_resume_edges: usize,
    terminal_outgoing_edges: usize,
    recursive_lowering_frames: usize,
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
struct BoundaryB1Census {
    opcode_vocabulary: usize,
    distinct_origins: usize,
    ir_records: usize,
    semantic_edges: usize,
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
    function_units: usize,
    definitions_per_origin: usize,
    all_out_of_line_operand_elements: usize,
    duplicate_origin_definitions: usize,
    post_origin_clones: usize,
    max_definitions_per_origin: usize,
    descriptor_bytes: usize,
    program_bytes: usize,
    record_bytes: usize,
    operand_element_bytes: usize,
    capture_layout_bytes: usize,
    capture_slot_bytes: usize,
    ruled_child_bytes: usize,
    function_bytes: usize,
}

struct Planner<'src> {
    plan: StaticTransitionPlan<'src>,
    store_interner: BTreeMap<PersistentStoreNode, PersistentNodeId>,
    next_source: u32,
    terminal: StaticNodeId,
    trap_terminal: StaticNodeId,
}

fn planner_error(detail: impl Into<String>) -> CraneliftBackendError {
    backend(BackendFailure::PlannerInvariant(detail.into()))
}

fn planner_capacity_error(detail: impl Into<String>) -> CraneliftBackendError {
    unsupported("NativeStaticTransitionPlanner", detail)
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ResultPhase {
    SpecializedOnly,
    CarrierRequired,
}

#[cfg(test)]
thread_local! {
    static D8_FORCE_VARIABLE_SPECIALIZED: Cell<bool> = const { Cell::new(false) };
    static D8_REMOVE_VARIABLE_CALLABLE_SEED: Cell<bool> = const { Cell::new(false) };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ResultPhaseSummary {
    /// Representation of this value itself. This is the earlier bound-value
    /// seed: inserting the summary into an environment preserves a carried
    /// value recovered through `Var`.
    phase: ResultPhase,
    continues: bool,
    /// Representation produced by invoking this value, when it is callable.
    /// Keeping this orthogonal to `phase` closes the bound-lexical-closure form
    /// without weakening the value's specialized closure representation.
    callable_result: Option<ResultPhase>,
}

impl ResultPhaseSummary {
    const TRAP: Self = Self {
        phase: ResultPhase::SpecializedOnly,
        continues: false,
        callable_result: None,
    };

    const SPECIALIZED: Self = Self {
        phase: ResultPhase::SpecializedOnly,
        continues: true,
        callable_result: None,
    };

    fn carrier() -> Self {
        Self {
            phase: ResultPhase::CarrierRequired,
            continues: true,
            callable_result: None,
        }
    }

    fn callable(result: ResultPhase) -> Self {
        Self {
            phase: ResultPhase::SpecializedOnly,
            continues: true,
            callable_result: Some(result),
        }
    }

    fn join(self, other: Self) -> Self {
        Self {
            phase: self.phase.max(other.phase),
            continues: self.continues || other.continues,
            callable_result: self.callable_result.max(other.callable_result),
        }
    }

    fn sequence(self, other: Self) -> Self {
        Self {
            phase: self.phase.max(other.phase),
            continues: self.continues && other.continues,
            // A sequence returns its right-hand value. Callable provenance is
            // therefore not an effect to accumulate from the left.
            callable_result: other.callable_result,
        }
    }
}

fn is_source_join(expr: &RuntimeExpr) -> bool {
    matches!(
        expr,
        RuntimeExpr::CheckedJoinSite { .. }
            | RuntimeExpr::If { .. }
            | RuntimeExpr::Match { .. }
            | RuntimeExpr::ComputationalMatch { .. }
            | RuntimeExpr::Call { .. }
    )
}

pub(in crate::cranelift_backend) fn planned_partiality_trap(
    primitive: &crate::RuntimePrimitive,
) -> Option<RuntimeTrap> {
    match &primitive.partiality {
        RuntimePartiality::CheckedTrap { obligation } => {
            let message = if obligation.ends_with(".bounds") {
                format!("{} bounds obligation failed", primitive.symbol)
            } else {
                format!("{} checked partiality trapped", primitive.symbol)
            };
            Some(RuntimeTrap {
                code: RuntimeTrapCode::ExplicitTrap,
                message,
            })
        }
        RuntimePartiality::TrustedTrap { .. } => Some(RuntimeTrap {
            code: RuntimeTrapCode::ExplicitTrap,
            message: format!("{} trusted partiality trapped", primitive.symbol),
        }),
        RuntimePartiality::Total
        | RuntimePartiality::SafeOption { .. }
        | RuntimePartiality::SafeResult { .. } => None,
    }
}

/// Compute the result phase from semantic result edges, never from arm order or
/// an emitted operand.  The match is intentionally exhaustive: a new source
/// form cannot silently inherit `SpecializedOnly`.
fn summarize_result_phase(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    functionized_units: bool,
    environment: &[ResultPhaseSummary],
    joins: &mut [Option<PlannedJoinResult>],
) -> Result<ResultPhaseSummary, CraneliftBackendError> {
    let occurrence = plan
        .source_occurrences
        .get(origin.0 as usize)
        .and_then(Option::as_ref)
        .ok_or_else(|| planner_error("phase plan names no source occurrence"))?;
    if occurrence.static_origin != origin {
        return Err(planner_error(
            "phase plan occurrence disagrees with its preallocated origin",
        ));
    }
    let expr = occurrence.expr;
    let child = |position| plan.semantic.child_origin(origin, position);
    let summarize_child = |position: usize,
                           environment: &[ResultPhaseSummary],
                           joins: &mut [Option<PlannedJoinResult>]|
     -> Result<ResultPhaseSummary, CraneliftBackendError> {
        let child_origin = child(position)?;
        let crosses_owner = plan.semantic.crosses_function_owner(origin, child_origin)?;
        let child_environment = if crosses_owner {
            result_phase_environment_for_owner(plan, child_origin, functionized_units)?
        } else {
            environment.to_vec()
        };
        let mut summary = summarize_result_phase(
            plan,
            child_origin,
            functionized_units,
            &child_environment,
            joins,
        )?;
        if functionized_units && summary.continues && crosses_owner {
            summary.phase = ResultPhase::CarrierRequired;
        }
        Ok(summary)
    };
    let summary = match expr {
        RuntimeExpr::Trap(_) => ResultPhaseSummary::TRAP,
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. } => {
            summarize_child(0, environment, joins)?
        }
        // These markers are the static call-template seeds consumed by the
        // functionized emitter. Their result is a declared-unit carrier even
        // when the wrapped source spelling itself is specialized.
        RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { .. } => {
            let nested = summarize_child(0, environment, joins)?;
            if functionized_units && nested.continues {
                ResultPhaseSummary::carrier()
            } else {
                nested
            }
        }
        RuntimeExpr::Let { .. } => {
            let value = summarize_child(0, environment, joins)?;
            let mut body_environment = Vec::with_capacity(1 + environment.len());
            body_environment.push(ResultPhaseSummary {
                continues: true,
                ..value
            });
            body_environment.extend_from_slice(environment);
            value.sequence(summarize_child(1, &body_environment, joins)?)
        }
        RuntimeExpr::If { .. } => {
            summarize_child(1, environment, joins)?.join(summarize_child(2, environment, joins)?)
        }
        RuntimeExpr::Match { cases, .. } => {
            let scrutinee = summarize_child(0, environment, joins)?;
            let mut result = ResultPhaseSummary::TRAP;
            for (index, case) in cases.iter().enumerate() {
                // A case projection preserves the scrutinee's representation:
                // fields of a carried constructor remain carried, while native
                // and borrowed specialized scrutinees yield specialized fields.
                let mut case_environment = Vec::with_capacity(case.binders + environment.len());
                case_environment.extend((0..case.binders).map(|_| ResultPhaseSummary {
                    continues: true,
                    ..scrutinee
                }));
                case_environment.extend_from_slice(environment);
                result = result.join(summarize_child(1 + index, &case_environment, joins)?);
            }
            result
        }
        RuntimeExpr::ComputationalMatch {
            scrutinee, cases, ..
        } => {
            let scrutinee_summary = summarize_child(0, environment, joins)?;
            let mut result = ResultPhaseSummary::TRAP;
            for (index, case) in cases.iter().enumerate() {
                let case_binders = case
                    .argument_binders
                    .checked_add(case.recursive_positions.len())
                    .ok_or_else(|| planner_capacity_error("phase-plan case arity exhausted"))?;
                let mut case_environment = Vec::with_capacity(case_binders + environment.len());
                // Lowering installs `[IHs, argument binders, outer env]`.
                // Functionized IHs are declared-unit results; argument binders
                // preserve the scrutinee's representation.
                case_environment.extend(case.recursive_positions.iter().map(|_| {
                    if functionized_units {
                        ResultPhaseSummary::carrier()
                    } else {
                        ResultPhaseSummary::SPECIALIZED
                    }
                }));
                case_environment.extend((0..case.argument_binders).map(|_| ResultPhaseSummary {
                    phase: scrutinee_summary.phase,
                    continues: true,
                    callable_result: scrutinee_summary.callable_result,
                }));
                case_environment.extend_from_slice(environment);
                result = result.join(summarize_child(1 + index, &case_environment, joins)?);
            }
            let scrutinee_origin = child(0)?;
            if let RuntimeExpr::Construct { args, .. } = scrutinee.as_ref() {
                let mut carries_recursive_unit = false;
                'cases: for case in cases {
                    for position in case.recursive_positions.iter().copied() {
                        let Some(RuntimeExpr::LexicalClosure { captures, .. }) = args.get(position)
                        else {
                            continue;
                        };
                        if !captures.is_empty() {
                            continue;
                        }
                        let argument_origin =
                            plan.semantic.child_origin(scrutinee_origin, position)?;
                        let body_origin = plan.semantic.child_origin(argument_origin, 0)?;
                        if plan.semantic.crosses_function_owner(origin, body_origin)? {
                            carries_recursive_unit = true;
                            break 'cases;
                        }
                    }
                }
                if functionized_units && result.continues && carries_recursive_unit {
                    result.phase = ResultPhase::CarrierRequired;
                }
            }
            // Producer-local result joins forward the value after this
            // computational eliminator has run, not the raw producer syntax.
            // Raise only the shared result-position population; argument and
            // let-value joins still carry their own independently summarized
            // representation.
            if functionized_units {
                for join_origin in
                    plan.source_result_origins_in_owner_subtree(scrutinee_origin)?
                {
                    if joins
                        .get(join_origin.0 as usize)
                        .and_then(Option::as_ref)
                        .is_none()
                    {
                        continue;
                    }
                    let join = joins
                        .get_mut(join_origin.0 as usize)
                        .and_then(Option::as_mut)
                        .ok_or_else(|| {
                            planner_error(
                                "computational result flow names an unplanned source join",
                            )
                        })?;
                    join.representation = JoinResultRepresentation::CarrierWord;
                }
            }
            result
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            let mut result = (0..args.len()).try_fold(
                ResultPhaseSummary::SPECIALIZED,
                |summary, position| {
                    Ok(summary.sequence(summarize_child(position, environment, joins)?))
                },
            )?;
            result.callable_result = None;
            result
        }
        RuntimeExpr::Record { fields } => {
            let mut result = (0..fields.len()).try_fold(
                ResultPhaseSummary::SPECIALIZED,
                |summary, position| {
                    Ok(summary.sequence(summarize_child(position, environment, joins)?))
                },
            )?;
            result.callable_result = None;
            result
        }
        RuntimeExpr::Project { .. } => summarize_child(0, environment, joins)?,
        RuntimeExpr::Call { args, .. } => {
            let callee = summarize_child(0, environment, joins)?;
            let mut result = callee;
            for position in 0..args.len() {
                result = result.sequence(summarize_child(1 + position, environment, joins)?);
            }
            if functionized_units && callee.callable_result == Some(ResultPhase::CarrierRequired) {
                result.phase = ResultPhase::CarrierRequired;
            }
            // This planner tracks the representation of the call result, not
            // higher-order provenance of values returned by an opaque call.
            result.callable_result = None;
            result
        }
        RuntimeExpr::Var(index) => {
            let phase = environment
                .get(*index as usize)
                .copied()
                .unwrap_or(ResultPhaseSummary::SPECIALIZED);
            #[cfg(test)]
            if D8_FORCE_VARIABLE_SPECIALIZED.with(Cell::get) {
                ResultPhaseSummary::SPECIALIZED
            } else {
                if D8_REMOVE_VARIABLE_CALLABLE_SEED.with(Cell::get) {
                    ResultPhaseSummary {
                        callable_result: None,
                        ..phase
                    }
                } else {
                    phase
                }
            }
            #[cfg(not(test))]
            phase
        }
        RuntimeExpr::Closure { .. } | RuntimeExpr::LexicalClosure { .. } => {
            let body_origin = child(0)?;
            ResultPhaseSummary::callable(
                if functionized_units
                    && plan.semantic.crosses_function_owner(origin, body_origin)?
                {
                    ResultPhase::CarrierRequired
                } else {
                    ResultPhase::SpecializedOnly
                },
            )
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Effect { .. } => ResultPhaseSummary::SPECIALIZED,
    };
    if is_source_join(expr) {
        let result = PlannedJoinResult {
            representation: match summary.phase {
                ResultPhase::SpecializedOnly => JoinResultRepresentation::NativeScalarPair,
                ResultPhase::CarrierRequired => JoinResultRepresentation::CarrierWord,
            },
            has_continuing_predecessor: summary.continues,
        };
        let entry = joins
            .get_mut(origin.0 as usize)
            .ok_or_else(|| planner_error("phase-plan join origin is outside the plan"))?;
        *entry = Some(match *entry {
            Some(previous) => PlannedJoinResult {
                representation: if previous.representation == JoinResultRepresentation::CarrierWord
                    || result.representation == JoinResultRepresentation::CarrierWord
                {
                    JoinResultRepresentation::CarrierWord
                } else {
                    JoinResultRepresentation::NativeScalarPair
                },
                has_continuing_predecessor: previous.has_continuing_predecessor
                    || result.has_continuing_predecessor,
            },
            None => result,
        });
    }
    Ok(summary)
}

fn result_phase_environment_for_owner(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    functionized_units: bool,
) -> Result<Vec<ResultPhaseSummary>, CraneliftBackendError> {
    let Some(function) = plan.semantic.function_owner(origin)? else {
        return Ok(Vec::new());
    };
    let descriptor = plan
        .abi
        .descriptors
        .iter()
        .find(|descriptor| descriptor.function == function)
        .ok_or_else(|| planner_error("phase plan owner has no ABI descriptor"))?;
    let start = descriptor.slots.start as usize;
    let end = start
        .checked_add(descriptor.slots.len as usize)
        .ok_or_else(|| planner_capacity_error("phase-plan ABI slot range exhausted"))?;
    let slots = plan
        .abi
        .slots
        .get(start..end)
        .ok_or_else(|| planner_error("phase plan ABI slot range is outside the plane"))?;
    Ok(slots
        .iter()
        .filter(|slot| matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture))
        .map(|slot| {
            // The ABI plane remained validated but inert on the retired
            // monolithic route. Its slots therefore could not impose carrier
            // storage on that route.
            if !functionized_units {
                return ResultPhaseSummary::SPECIALIZED;
            }
            // The process pair is the closed exception to generic ValueWord
            // inputs: the root unit recovers these two role-keyed values as a
            // borrowed process input and a capability token. Every other
            // parameter/capture remains an opaque carried word.
            if matches!(
                descriptor.definition,
                AbiUnitDefinition::SchedulingEntry {
                    ingress: AbiSchedulingIngress::ProcessPair,
                }
            ) && slot.kind == AbiSlotKind::Parameter
            {
                ResultPhaseSummary::SPECIALIZED
            } else {
                ResultPhaseSummary::carrier()
            }
        })
        .collect())
}

fn build_join_result_plan(
    plan: &StaticTransitionPlan<'_>,
    functionized_units: bool,
) -> Result<Vec<Option<PlannedJoinResult>>, CraneliftBackendError> {
    let mut joins = vec![None; plan.source_occurrences.len()];
    for descriptor in &plan.abi.descriptors {
        // The carried body occurrence, with NO root special case. This is
        // the same compensation `define_unit_body` used to carry and for the
        // same reason: the field it read was an alias of the scheduling entry,
        // so the root was patched and every other unit silently rooted its join
        // summary at its entry instead of its body.
        let root = descriptor.body_occurrence;
        let environment = result_phase_environment_for_owner(plan, root, functionized_units)?;
        summarize_result_phase(plan, root, functionized_units, &environment, &mut joins)?;
    }
    for occurrence in plan.source_occurrences.iter().flatten() {
        if is_source_join(occurrence.expr) && joins[occurrence.static_origin.0 as usize].is_none() {
            let environment = result_phase_environment_for_owner(
                plan,
                occurrence.static_origin,
                functionized_units,
            )?;
            summarize_result_phase(
                plan,
                occurrence.static_origin,
                functionized_units,
                &environment,
                &mut joins,
            )?;
        }
    }
    Ok(joins)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CaseProducerFact {
    producers: CaseProducerSet,
    producer_origins: Vec<(ConstructorIdentity, BTreeSet<StaticOriginId>)>,
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
fn derive_case_producer_fact(
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

fn build_case_emission_plan(
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

fn validate_case_emission_plan(
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








/// **`RT-DECL-CLOSURE-PORT` `D7` — the phase one host-effect seat's value is
/// actually in.**
///
/// ⛔ Deliberately its own type rather than a reuse of the continuation-input
/// projection's former boundary-use vocabulary. That vocabulary was keyed on
/// ABI slots; this one is keyed on a semantic seat of a host operation. They
/// answer different questions about different populations, and one enum
/// spanning both is what lets an answer derived for one be read as authority
/// for the other.
///
/// ⭐ That separation is why this type survived a deletion that took the other
/// one. `RT-CONTSPEC-LEDGER` (Architect `evt_1v9m7t4m9dmj7`) retired the four
/// continuation-side boundary-use axes as an unowned schema fragment, having
/// established that nothing consumed them. The proposal that preceded it was to
/// populate them by projecting THIS record onto them — refused precisely
/// because the two populations are not one. Keep them apart.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EffectSeatPhase {
    /// A compile-time `Lowered` template the emitter may read directly.
    SpecializedTemplate,
    /// A boundary-carrier word, observable only through emitted helpers.
    CarriedWord,
}

/// What the emitter DOES at one seat.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EffectSeatOperation {
    /// Select one member of a closed constructor set (`Stream`, `CreatePolicy`,
    /// `ResourceOpenMode`) and write its wire tag.
    SelectClosedTag,
    /// Project a byte span to a `(pointer, length)` pair.
    ProjectBytesSpan,
    /// Observe an opaque resource handle as a scalar.
    ObserveResourceHandle,
    /// Observe the opaque invocation capability token as a scalar.
    ObserveCapabilityToken,
    /// Narrow an exact `Int` to a checked `u64`.
    NarrowExactInt,
}

/// **Which slot of one effect occurrence a seat is.**
///
/// ⛔ The conditional capability is NOT argument ordinal 0, and collapsing the
/// two is the exact confusion the post-capability offset exists to prevent.
/// `FsOpen`'s capability and `FsOpen`'s first semantic argument are both real
/// consumed seats with different needs; keyed on the structural position alone
/// they would be positions 0 and 1, and keyed on a bare ordinal they would
/// collide at 0. This carries the distinction in the key itself.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EffectSeatSlot {
    /// The capability at structural position 0, when the occurrence has one.
    Capability,
    /// The semantic argument at this ordinal, AFTER the capability offset.
    Argument(u32),
}

/// **What a seat must be able to OBSERVE — derived FIRST, before any
/// representation is selected.**
///
/// ⭐⭐ **The direction is the whole point.** A `Need` read off a chosen
/// disposition reverses the equation: it makes whatever the representation
/// happens to offer into the definition of what the consumer wanted. Planning
/// derives this from the seat's own semantics — what the wire request requires
/// at that ordinal — and only then selects and validates an `Avail` that
/// satisfies it.
///
/// ⛔ Equality-bearing, together with the operation and the semantic ordinal. A
/// seat's identity is not its structural role: `BufferAllocate.capacity`,
/// `FsChangeMode.mode` and `FsReadAt.length` are all `EffectArgument`s holding
/// an `Int`, and they are three different seats.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EffectSeatNeed {
    /// The member identity of a closed constructor set.
    ConstructorTag,
    /// A byte span's address and length.
    BytesPointerLength,
    /// An opaque resource handle's scalar word.
    ResourceScalar,
    /// The invocation capability token's scalar word.
    ///
    /// ⛔ Deliberately not [`Self::ResourceScalar`]. Both are opaque scalars and
    /// the emitter reads both through `emit_carrier_scalar`, but a capability
    /// token authorizes an operation while a resource handle names an object.
    /// One need spanning both would let a seat proved for one be read as proof
    /// for the other.
    CapabilityTokenScalar,
    /// An exact `Int`'s magnitude as a checked `u64`.
    ExactIntU64,
}

/// **The phases in which a seat's [`EffectSeatNeed`] can actually be
/// satisfied.**
///
/// ⛔ Per-SEAT, never per-need, and that is not redundancy. `BufferFreeze`'s
/// buffer and span-origin seats observe a resource handle in either phase
/// because their route already emits the helper; `ResourceRelease`'s and
/// `FsReadAt`'s observe the same `ResourceScalar` and have no such route. Same
/// need, different availability — a per-need table would have to answer one of
/// them wrongly.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct EffectSeatAvail {
    pub(in crate::cranelift_backend) specialized: bool,
    pub(in crate::cranelift_backend) carried: bool,
}

impl EffectSeatAvail {
    const SPECIALIZED_ONLY: Self = Self {
        specialized: true,
        carried: false,
    };
    const EITHER_PHASE: Self = Self {
        specialized: true,
        carried: true,
    };

    /// Whether a seat consumed in `phase` can satisfy its need.
    ///
    /// ⛔ This IS the `Need ⊆ Avail` test. It is a membership question, and the
    /// seat it is asked about carries its own coordinates — so a seat that
    /// fails it is refused as that exact seat of that exact operation, never as
    /// a generic specialized-only surface.
    pub(in crate::cranelift_backend) fn admits(self, phase: EffectSeatPhase) -> bool {
        match phase {
            EffectSeatPhase::SpecializedTemplate => self.specialized,
            EffectSeatPhase::CarriedWord => self.carried,
        }
    }
}

/// **One FULL semantic seat of one admitted host effect.**
///
/// ⭐ "Full" is the correction this record exists to make. A seat is not a
/// structural position and not a nominal role: it is the position *plus* the
/// operation it belongs to *plus* its post-capability-offset semantic ordinal.
/// Two seats agreeing on the first and differing on either of the others are
/// two records, because the wire request wants different things of them.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct PlannedEffectSeat {
    pub(in crate::cranelift_backend) effect_origin: StaticOriginId,
    /// The exact child occurrence that produces this seat's value.
    pub(in crate::cranelift_backend) child_origin: StaticOriginId,
    /// The child's STRUCTURAL position, capability included.
    pub(in crate::cranelift_backend) position: u32,
    pub(in crate::cranelift_backend) operation: ken_host::HostOpV1,
    /// The slot: the capability, or a semantic argument ordinal AFTER the
    /// conditional capability offset.
    ///
    /// ⛔ Not the structural position. An operation carrying a capability shifts
    /// every argument by one, so a seat keyed on the structural position alone
    /// names a different semantic argument depending on a fact about the
    /// operation's capability that the position does not carry.
    pub(in crate::cranelift_backend) slot: EffectSeatSlot,
    /// The owner of the occurrence that PRODUCES this seat's value.
    pub(in crate::cranelift_backend) producer_owner: PredeclaredFunctionId,
    /// The owner of the body that DISPATCHES the effect.
    ///
    /// ⚠ **No phase accompanies either owner, and that is a measured
    /// correction rather than an omission.** A derived `consumer_phase` was
    /// built first, from the child's planned join-result representation widened
    /// to `CarriedWord` across an owner boundary, and checked against the phase
    /// the emitter actually held. It was WRONG on real programs: `BufferFreeze`
    /// argument 0 and `FsReadFile`'s capability both arrive carried while their
    /// child occurrence has no `CarrierWord` join result and no owner crossing,
    /// because the value reaches the body through a declared ABI slot — a fact
    /// about the enclosing unit's parameters, not about the child. Rather than
    /// keep a prediction that is false, the phase is OBSERVED at the claim and
    /// `Need ⊆ Avail` is asked there, of the operand actually in hand. The
    /// membership question is unchanged; only the thing it is asked about is
    /// now a measurement instead of a guess.
    pub(in crate::cranelift_backend) consumer_owner: PredeclaredFunctionId,
    pub(in crate::cranelift_backend) semantic_operation: EffectSeatOperation,
    pub(in crate::cranelift_backend) need: EffectSeatNeed,
    pub(in crate::cranelift_backend) avail: EffectSeatAvail,
}

/// **Test-only seat construction for the `RT-CARRIER-BYTESPAN-OBSERVE`
/// `D4` observer control.**
///
/// ⛔ Gated on its own, and the gate is not decoration: an earlier draft of
/// this insertion sat between the mutation enum's `#[cfg(test)]` and its
/// `#[derive]`, capturing the attribute and shipping that enum into
/// production builds. The `--lib` test profile cannot observe that, which
/// is why the repair is validated by a production build.
#[cfg(test)]
impl PlannedEffectSeat {
    /// A seat record for a control, with a caller-chosen `need`.
    ///
    /// ⚠ Test-only scaffolding for `RT-CARRIER-BYTESPAN-OBSERVE` `D4`, whose
    /// observer consumes this record. The id newtypes are `pub(super)` here, so
    /// a control in the lowering cannot build one itself.
    ///
    /// ⛔ `avail` is `SPECIALIZED_ONLY` and stays that way: `D4` activates
    /// nothing, and a fixture handing itself `EITHER_PHASE` would be asserting
    /// `D5`'s outcome.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn for_observer_control(
        need: EffectSeatNeed,
    ) -> Self {
        PlannedEffectSeat {
            effect_origin: StaticOriginId(0),
            child_origin: StaticOriginId(1),
            position: 0,
            operation: ken_host::HostOpV1::FsReadFile,
            slot: EffectSeatSlot::Argument(0),
            producer_owner: PredeclaredFunctionId(0),
            consumer_owner: PredeclaredFunctionId(0),
            semantic_operation: EffectSeatOperation::ProjectBytesSpan,
            need,
            avail: EffectSeatAvail::SPECIALIZED_ONLY,
        }
    }
}

/// **Erase one axis of the seat key, or collapse every seat onto one
/// contract.**
///
/// ⛔ Applied ONLY inside [`build_host_effect_seat_plan`], never in the
/// re-derivation the close performs. That asymmetry is the whole mechanism: the
/// rebuild-equality validation mutates on both sides and so cannot see any of
/// these, which is correct — it checks the derivation is a function, not that
/// the function is right. What sees them is the independent recomputation of
/// the contract from the operation and the slot at the ledger close.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum EffectSeatPlanMutation {
    Exact,
    /// The operation stops being part of the key: every seat records the first
    /// admitted operation.
    EraseOperation,
    /// The ordinal stops being part of the key: every argument seat becomes
    /// argument 0.
    EraseOrdinal,
    /// The need stops being part of the authority: every seat records one need.
    EraseNeed,
    /// Every seat takes one contract, which is the "all argument seats are the
    /// same kind of thing" collapse the full-seat key exists to refuse.
    CollapseContract,
}

#[cfg(test)]
thread_local! {
    static EFFECT_SEAT_PLAN_MUTATION: std::cell::Cell<EffectSeatPlanMutation> =
        const { std::cell::Cell::new(EffectSeatPlanMutation::Exact) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_effect_seat_plan_mutation(
    mutation: EffectSeatPlanMutation,
) {
    EFFECT_SEAT_PLAN_MUTATION.with(|cell| cell.set(mutation));
}

/// **The host operations this backend represents as consumers.**
///
/// ⛔ It lives in PLANNING because the seat population is derived here and the
/// emitter's admission check reads the same list. A second copy on the lowering
/// side would be a second authority: the two could disagree about whether an
/// operation is admitted, and the disagreement would show up as a seat with no
/// planned record rather than as a contradiction anyone stated.
pub(in crate::cranelift_backend) const CRANELIFT_HOST_EFFECT_CONSUMERS_V1:
    [ken_host::HostOpV1; 13] = [
    ken_host::HostOpV1::ConsoleWrite,
    ken_host::HostOpV1::ConsoleFlush,
    ken_host::HostOpV1::ConsoleIsTerminal,
    ken_host::HostOpV1::FsReadFile,
    ken_host::HostOpV1::FsWriteFile,
    ken_host::HostOpV1::FsChangeMode,
    ken_host::HostOpV1::FsOpen,
    ken_host::HostOpV1::FsHandleMetadata,
    ken_host::HostOpV1::FsReadAt,
    ken_host::HostOpV1::FsWriteAt,
    ken_host::HostOpV1::ResourceRelease,
    ken_host::HostOpV1::BufferAllocate,
    ken_host::HostOpV1::BufferFreeze,
];

/// The seat contract of one admitted operation at one semantic ordinal.
///
/// ⛔ **Total over the 13 admitted operations, with no `_` arm**, so a new
/// admitted operation is a compile error here rather than an operation whose
/// seats silently have no contract. `None` means the operation has no seat at
/// that ordinal, which is an arity disagreement and is refused by the caller —
/// never a seat that is exempt.
///
/// ⚠ The `Avail` column is where this release's one new capability appears, and
/// nowhere else: `BufferAllocate.capacity` is the single seat whose exact `Int`
/// this release can observe through the carrier ABI. Every other `ExactIntU64`
/// seat stays specialized-only, which is why `Avail` is recorded per seat.
fn host_effect_seat_contract(
    operation: ken_host::HostOpV1,
    slot: EffectSeatSlot,
) -> Option<(EffectSeatOperation, EffectSeatNeed, EffectSeatAvail)> {
    use ken_host::HostOpV1 as Op;
    use EffectSeatAvail as Avail;
    use EffectSeatNeed as Need;
    use EffectSeatOperation as Semantic;
    // ⭐ The CAPABILITY half, kept ahead of the argument table because its
    // population is the exact complement: the four FS-path operations require
    // one, and every other admitted operation refuses one outright. A `None`
    // here is therefore not an arity gap but a capability the operation does
    // not admit, and the caller refuses it with the seat's own coordinates.
    let ordinal = match slot {
        EffectSeatSlot::Capability => {
            return match operation {
                Op::FsReadFile | Op::FsWriteFile | Op::FsChangeMode | Op::FsOpen => Some((
                    Semantic::ObserveCapabilityToken,
                    Need::CapabilityTokenScalar,
                    // Both phases: the emitter reads a specialized
                    // `CapabilityToken` template directly and a carried word
                    // through `emit_carrier_scalar`.
                    Avail::EITHER_PHASE,
                )),
                Op::ConsoleWrite
                | Op::ConsoleFlush
                | Op::ConsoleIsTerminal
                | Op::FsHandleMetadata
                | Op::FsReadAt
                | Op::FsWriteAt
                | Op::ResourceRelease
                | Op::BufferAllocate
                | Op::BufferFreeze
                | Op::ConsoleRead
                | Op::ClockWallNow
                | Op::ClockMonotonicNow
                | Op::ClockSleepUntil
                | Op::FsAppendFile
                | Op::FsMetadata
                | Op::FsReadDirectory
                | Op::FsCreateDirectory
                | Op::FsRemoveFile
                | Op::FsRemoveDirectory
                | Op::FsRename
                | Op::EntropyRandomBytes => None,
            };
        }
        EffectSeatSlot::Argument(ordinal) => ordinal,
    };
    let tag = (
        Semantic::SelectClosedTag,
        Need::ConstructorTag,
        Avail::SPECIALIZED_ONLY,
    );
    let bytes = (
        Semantic::ProjectBytesSpan,
        Need::BytesPointerLength,
        Avail::SPECIALIZED_ONLY,
    );
    // `RT-CARRIER-BYTESPAN-OBSERVE` `D5` — the byte-span seats whose carried
    // route is PROVED, per seat and each against its own measured witness.
    //
    // The tuple is shared by construction, so it is deliberately NOT the
    // discriminator: `bytes` and `carried_bytes` differ only in `Avail`, and a
    // seat moves between them only when a row was observed refusing at that
    // exact `(operation, ordinal)` and observed lowering afterwards. `AC-4`'s
    // disposition table in the node records the evidence per seat, including
    // the proof for each seat left on `bytes`.
    let carried_bytes = (
        Semantic::ProjectBytesSpan,
        Need::BytesPointerLength,
        Avail::EITHER_PHASE,
    );
    let resource = (
        Semantic::ObserveResourceHandle,
        Need::ResourceScalar,
        Avail::SPECIALIZED_ONLY,
    );
    let phase_bearing_resource = (
        Semantic::ObserveResourceHandle,
        Need::ResourceScalar,
        Avail::EITHER_PHASE,
    );
    let exact_int = (
        Semantic::NarrowExactInt,
        Need::ExactIntU64,
        Avail::SPECIALIZED_ONLY,
    );
    let carried_exact_int = (
        Semantic::NarrowExactInt,
        Need::ExactIntU64,
        Avail::EITHER_PHASE,
    );
    match (operation, ordinal) {
        (Op::ConsoleWrite, 0) | (Op::ConsoleFlush, 0) | (Op::ConsoleIsTerminal, 0) => Some(tag),
        // PROVED carried, per seat: `D5` measured a carried word reaching each
        // of these and the observer consuming it. Neither is site-bound.
        (Op::ConsoleWrite, 1) | (Op::FsWriteFile, 2) => Some(carried_bytes),
        // LEFT SPECIALIZED_ONLY for the direct operation consumer, and NOT
        // because the observer fails them — `D5` measured it succeeding at all
        // four. Each operation's synthesized `FileError` separately declares
        // `SiteOperand(0)`. `RT-SITEOP-CARRIED-WITNESS` projects that exact
        // second use through the emitted byte-span helper without widening the
        // seat-wide `Avail` relation.
        (Op::FsReadFile, 0) | (Op::FsWriteFile, 0) | (Op::FsChangeMode, 0) | (Op::FsOpen, 0) => {
            Some(bytes)
        }
        (Op::FsWriteFile, 1) | (Op::FsOpen, 1) => Some(tag),
        (Op::FsChangeMode, 1) => Some(exact_int),
        (Op::FsHandleMetadata, 0) | (Op::ResourceRelease, 0) => Some(resource),
        // ⭐ The one seat this release teaches the carrier to observe.
        (Op::BufferAllocate, 0) => Some(carried_exact_int),
        (Op::BufferFreeze, 0) | (Op::BufferFreeze, 3) => Some(phase_bearing_resource),
        (Op::BufferFreeze, 1) | (Op::BufferFreeze, 2) => Some(exact_int),
        (Op::FsReadAt, 0) | (Op::FsReadAt, 2) | (Op::FsWriteAt, 0) | (Op::FsWriteAt, 2) => {
            Some(resource)
        }
        (Op::FsWriteAt, 5) => Some(resource),
        (Op::FsReadAt, 1)
        | (Op::FsReadAt, 3)
        | (Op::FsReadAt, 4)
        | (Op::FsWriteAt, 1)
        | (Op::FsWriteAt, 3)
        | (Op::FsWriteAt, 4) => Some(exact_int),
        // An ADMITTED operation at an ordinal it does not have. `None` here is
        // an arity disagreement, refused by the caller with the seat's own
        // coordinates -- never a seat that is exempt from having a contract.
        (
            Op::ConsoleWrite
            | Op::ConsoleFlush
            | Op::ConsoleIsTerminal
            | Op::FsReadFile
            | Op::FsWriteFile
            | Op::FsChangeMode
            | Op::FsOpen
            | Op::FsHandleMetadata
            | Op::FsReadAt
            | Op::FsWriteAt
            | Op::ResourceRelease
            | Op::BufferAllocate
            | Op::BufferFreeze,
            _,
        ) => None,
        // ⛔ The represented-UNAVAILABLE lanes, named rather than wildcarded.
        // They are refused before any seat is derived, so they have no seat
        // contract at all -- and naming them is what makes promoting one to the
        // admitted set a compile error here rather than an operation whose
        // seats silently answer `None`.
        (
            Op::ConsoleRead
            | Op::ClockWallNow
            | Op::ClockMonotonicNow
            | Op::ClockSleepUntil
            | Op::FsAppendFile
            | Op::FsMetadata
            | Op::FsReadDirectory
            | Op::FsCreateDirectory
            | Op::FsRemoveFile
            | Op::FsRemoveDirectory
            | Op::FsRename
            | Op::EntropyRandomBytes,
            _,
        ) => None,
    }
}

/// **Derive one record for every capability/argument seat of every admitted
/// host effect occurrence.**
///
/// ⛔ **The population is every `Effect` source occurrence, not the ones some
/// reached trace visited**, and within one occurrence it is every slot the
/// operation actually has — not the slots the arm this compilation took
/// happened to read.
///
/// ⭐ The order of the derivation is the correction this record exists to make.
/// `Need` comes from [`host_effect_seat_contract`], which is keyed on the
/// operation and the slot and knows nothing about how the value will be
/// represented. Only then is the seat's `Avail` checked to admit the phase the
/// consumer will see it in. Reading the need off the representation instead is
/// what makes whatever the emitter happens to offer into the definition of what
/// the wire request wanted.
fn build_host_effect_seat_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<PlannedEffectSeat>, CraneliftBackendError> {
    let mut records = Vec::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Effect {
            operation,
            capability,
            args,
            ..
        } = occurrence.expr
        else {
            continue;
        };
        // A represented-unavailable lane has no seats at all: it is refused
        // whole, before any slot of it is derived.
        if !CRANELIFT_HOST_EFFECT_CONSUMERS_V1.contains(operation) {
            continue;
        }
        let effect_origin = occurrence.static_origin;
        let authority = occurrence_authority(plan, effect_origin)?;
        let consumer_owner = authority.owner;
        let argument_base = u32::from(capability.is_some());
        let slots = capability
            .iter()
            .map(|_| EffectSeatSlot::Capability)
            .chain((0..args.len()).map(|ordinal| {
                EffectSeatSlot::Argument(
                    u32::try_from(ordinal).expect("an argument list shorter than u32::MAX"),
                )
            }));
        for slot in slots {
            let position = match slot {
                EffectSeatSlot::Capability => 0,
                EffectSeatSlot::Argument(ordinal) => argument_base
                    .checked_add(ordinal)
                    .ok_or_else(|| planner_capacity_error("effect seat position overflows"))?,
            };
            let child = authority
                .children
                .get(position as usize)
                .ok_or_else(|| planner_error("a host effect seat has no child occurrence"))?;
            // ⛔ `Need` FIRST, from the seat's own semantics.
            let Some((semantic_operation, need, avail)) =
                host_effect_seat_contract(*operation, slot)
            else {
                return Err(planner_error(format!(
                    "host operation {:?} has no seat contract at {slot:?}, so the occurrence's \
                     shape and the operation's wire request disagree",
                    operation
                )));
            };
            let record = PlannedEffectSeat {
                effect_origin,
                child_origin: child.origin,
                position,
                operation: *operation,
                slot,
                producer_owner: child.owner,
                consumer_owner,
                semantic_operation,
                need,
                avail,
            };
            #[cfg(test)]
            let record = mutate_planned_effect_seat(record);
            records.push(record);
        }
    }
    records.sort();
    Ok(records)
}

#[cfg(test)]
fn mutate_planned_effect_seat(record: PlannedEffectSeat) -> PlannedEffectSeat {
    let tag = (
        EffectSeatOperation::SelectClosedTag,
        EffectSeatNeed::ConstructorTag,
        EffectSeatAvail::SPECIALIZED_ONLY,
    );
    match EFFECT_SEAT_PLAN_MUTATION.with(std::cell::Cell::get) {
        EffectSeatPlanMutation::Exact => record,
        EffectSeatPlanMutation::EraseOperation => PlannedEffectSeat {
            operation: CRANELIFT_HOST_EFFECT_CONSUMERS_V1[0],
            ..record
        },
        EffectSeatPlanMutation::EraseOrdinal => PlannedEffectSeat {
            slot: match record.slot {
                EffectSeatSlot::Capability => EffectSeatSlot::Capability,
                EffectSeatSlot::Argument(_) => EffectSeatSlot::Argument(0),
            },
            ..record
        },
        EffectSeatPlanMutation::EraseNeed => PlannedEffectSeat {
            need: EffectSeatNeed::ConstructorTag,
            ..record
        },
        EffectSeatPlanMutation::CollapseContract => PlannedEffectSeat {
            semantic_operation: tag.0,
            need: tag.1,
            avail: tag.2,
            ..record
        },
    }
}

/// **The contract one operation/slot pair has, recomputed from nothing but the
/// pair.**
///
/// ⭐ This is the INDEPENDENT side of the seat authority's contract half. The
/// planned population records a semantic operation, a need and an availability;
/// this recomputes them at the close from the two key axes alone. Without it
/// `need` would be diagnostic text — nothing would read it, so erasing it would
/// change no decision and no gate could see the erasure.
pub(in crate::cranelift_backend) fn host_effect_seat_contract_of(
    operation: ken_host::HostOpV1,
    slot: EffectSeatSlot,
) -> Option<(EffectSeatOperation, EffectSeatNeed, EffectSeatAvail)> {
    host_effect_seat_contract(operation, slot)
}

/// Every record names a DISTINCT seat.
///
/// The non-aliasing law of the seat domain, in production rather than in a
/// test, for the same reason the aggregate producers have one: if two records
/// shared `(effect_origin, slot)`, one seat's contract could authorize
/// another's consumption.
fn validate_host_effect_seats_are_unique(
    records: &[PlannedEffectSeat],
) -> Result<(), CraneliftBackendError> {
    let mut seen = BTreeSet::new();
    for record in records {
        if !seen.insert((record.effect_origin, record.slot)) {
            return Err(planner_error(
                "two host effect seat records name the same occurrence slot, so a seat identity \
                 is not unique",
            ));
        }
    }
    Ok(())
}

fn validate_host_effect_seat_plan(
    plan: &StaticTransitionPlan<'_>,
    records: &[PlannedEffectSeat],
) -> Result<(), CraneliftBackendError> {
    if records != build_host_effect_seat_plan(plan)? {
        return Err(planner_error(
            "the host effect seat population is not the exact closed seat-contract derivation",
        ));
    }
    validate_host_effect_seats_are_unique(records)
}





fn validate_substrate_preallocation_closure(
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
pub(in crate::cranelift_backend) enum D4bVerdict {
    /// `Closed([S])` — one exact source. The only admitting verdict.
    Closed,
    /// `Open` — refused by the take-loop's first clause.
    Open,
    /// `Closed([S, T, ..])` — refused by the take-loop's second clause.
    Ambiguous(usize),
}

#[cfg(test)]
thread_local! {
    static D4B_ADMISSION: std::cell::RefCell<Vec<(Vec<D4bVerdict>, bool)>> =
        const { std::cell::RefCell::new(Vec::new()) };
    static D4B_ADMISSION_ARMED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4b_arm_admission(armed: bool) {
    D4B_ADMISSION_ARMED.with(|cell| cell.set(armed));
    if armed {
        D4B_ADMISSION.with(|ledger| ledger.borrow_mut().clear());
    }
}

/// Every candidate edge seen while armed, as `(required verdict vector, admitted)`.
#[cfg(test)]
pub(in crate::cranelift_backend) fn d4b_take_admission() -> Vec<(Vec<D4bVerdict>, bool)> {
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
enum StaticWorkerMemberMutation {
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
fn with_static_worker_member_mutation<R>(
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
fn apply_static_worker_member_mutation(plan: &mut StaticTransitionPlan<'_>) {
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
fn validate_static_worker_member_population(
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

impl<'src> Planner<'src> {
    fn new() -> Result<Self, CraneliftBackendError> {
        let empty = PersistentNodeId(0);
        let frame = DynamicActivationFrame {
            syntax: empty,
            environment: empty,
            normal: empty,
            abrupt: empty,
            path: empty,
            cleanup: empty,
            affine: empty,
            source_return: empty,
        };
        let mut planner = Self {
            plan: StaticTransitionPlan {
                entries: Vec::new(),
                planned_entry_bodies: Vec::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
                stores: Vec::new(),
                store_depths: Vec::new(),
                evidence: Vec::new(),
                planned_helpers: Vec::new(),
                semantic_sources: Vec::new(),
                semantic_material: SemanticMaterialArena::default(),
                abi: AbiPlane::default(),
                root_entry: None,
                root_ingress: AbiRootIngress::Value,
                semantic: SemanticPlane::default(),
                root_occurrence: None,
                declaration_occurrences: BTreeMap::new(),
                declaration_entries: BTreeMap::new(),
                declaration_call_targets: BTreeMap::new(),
                trap_catalog: Vec::new(),
                source_occurrences: Vec::new(),
                join_results: Vec::new(),
                case_emissions: Vec::new(),
                aggregate_ownership: Vec::new(),
                host_effect_seats: Vec::new(),
                occurrence_authorities: Vec::new(),
                continuation_specializations: Vec::new(),
                continuation_specialization_calls: Vec::new(),
                required_consumer_projections: BTreeMap::new(),
                continuation_contexts: Vec::new(),
                // Empty by construction: the planner has no oriented plan, so a
                // fusion identity cannot exist yet. `D2f`'s post-planner
                // installer is the only writer.
                static_continuation_fusions: StaticContinuationFusionPlan::default(),
                // Empty by construction: body ownership is installed only from
                // validated claims, which cannot exist before the plan does.
                fusion_owned_bodies: BTreeMap::new(),
                fusion_composed_calls: BTreeMap::new(),
                fusion_outer_realizations: BTreeMap::new(),
                fusion_bodies_installed: false,
            },
            store_interner: BTreeMap::new(),
            next_source: 0,
            terminal: StaticNodeId(0),
            trap_terminal: StaticNodeId(0),
        };
        let terminal_owner = planner.source()?;
        planner.terminal = planner.control_node(TransitionKind::Terminal, terminal_owner, frame)?;
        let trap_owner = planner.source()?;
        planner.trap_terminal =
            planner.control_node(TransitionKind::TrapTerminal, trap_owner, frame)?;
        Ok(planner)
    }

    fn source(&mut self) -> Result<StaticSourceId, CraneliftBackendError> {
        let id = self.next_source;
        self.next_source = self
            .next_source
            .checked_add(1)
            .ok_or_else(|| planner_capacity_error("static source identity exhausted"))?;
        Ok(StaticSourceId(id))
    }

    fn push_node(
        &mut self,
        kind: TransitionKind,
        owner: StaticSourceId,
        frame: DynamicActivationFrame,
    ) -> Result<StaticNodeId, CraneliftBackendError> {
        let id = u32::try_from(self.plan.nodes.len())
            .map_err(|_| planner_capacity_error("static node identity exhausted"))?;
        let id = StaticNodeId(id);
        self.plan.nodes.push(StaticNode {
            id,
            transition: kind,
            owner,
            frame,
        });
        self.plan
            .planned_helpers
            .push(PlannedHelperKey::node(kind, id));
        Ok(id)
    }

    fn control_node(
        &mut self,
        kind: TransitionKind,
        owner: StaticSourceId,
        frame: DynamicActivationFrame,
    ) -> Result<StaticNodeId, CraneliftBackendError> {
        let node = self.push_node(kind, owner, frame)?;
        self.plan
            .semantic_sources
            .push(SemanticSourceSeed::control(node, kind));
        Ok(node)
    }

    /// Registers an expression occurrence whose syntax children are already
    /// planned. `children` is in source position order, and holds each child's
    /// **occurrence** origin — never its scheduling entry (D9).
    ///
    /// The returned `PlannedExpr` has `entry == occurrence.node`: an ordinary
    /// form is scheduled at the very node its occurrence is registered on.
    /// `ComputationalMatch` is the sole variant that does not go through here for
    /// that reason.
    fn expression_node(
        &mut self,
        kind: TransitionKind,
        owner: StaticSourceId,
        frame: DynamicActivationFrame,
        expr: &'src RuntimeExpr,
        children: &[StaticOriginId],
    ) -> Result<PlannedExpr, CraneliftBackendError> {
        let node = self.push_node(kind, owner, frame)?;
        self.expression_seed(node, expr, children)?;
        Ok(PlannedExpr {
            entry: node,
            occurrence: origin_of(node),
        })
    }

    /// Emits an already-pushed node's semantic material. Split out for the one
    /// occurrence whose node must exist before its children are planned (a
    /// computational match's source-return resume owns the outer edges).
    fn expression_seed(
        &mut self,
        node: StaticNodeId,
        expr: &'src RuntimeExpr,
        children: &[StaticOriginId],
    ) -> Result<(), CraneliftBackendError> {
        match expr {
            RuntimeExpr::Trap(trap)
            | RuntimeExpr::Match { default: trap, .. }
            | RuntimeExpr::ComputationalMatch { default: trap, .. } => {
                self.intern_trap(trap)?;
            }
            RuntimeExpr::PrimitiveCall { primitive, .. } => {
                if let Some(trap) = planned_partiality_trap(primitive) {
                    self.intern_trap(&trap)?;
                }
            }
            _ => {}
        }
        let seed =
            SemanticSourceSeed::expression(node, expr, children, &mut self.plan.semantic_material)?;
        self.plan.semantic_sources.push(seed);
        self.record_source_occurrence(node, expr)
    }

    fn intern_trap(
        &mut self,
        trap: &RuntimeTrap,
    ) -> Result<PlannedTrapIdentity, CraneliftBackendError> {
        if let Some(index) = self
            .plan
            .trap_catalog
            .iter()
            .position(|candidate| candidate == trap)
        {
            return u32::try_from(index + 1)
                .map(PlannedTrapIdentity)
                .map_err(|_| planner_capacity_error("trap identity exhausted"));
        }
        self.plan.trap_catalog.push(trap.clone());
        u32::try_from(self.plan.trap_catalog.len())
            .map(PlannedTrapIdentity)
            .map_err(|_| planner_capacity_error("trap identity exhausted"))
    }

    /// Add the cross-owner edges represented by source `DeclarationRef`
    /// occurrences after all transparent declaration entries exist.
    ///
    /// These are deliberately not `StaticBody` edges: that edge kind is the
    /// closure-body owner boundary and also seeds a function unit. A transparent
    /// declaration entry is already a scheduling-entry seed.
    fn connect_declaration_calls(
        &mut self,
        declaration_entries: &BTreeMap<String, StaticNodeId>,
    ) -> Result<(), CraneliftBackendError> {
        // ⭐ `D4`: resolve each declaration's target ONCE, before any edge is
        // added. The class is a property of the **declaration**, not of a
        // reference to it, so two references to one symbol cannot disagree —
        // and a symbol with no reference at all is still resolved, so a
        // malformed closure seed fails in planning rather than only when
        // somebody happens to call it.
        let mut targets: BTreeMap<&str, (StaticNodeId, DeclarationCallTargetClass)> =
            BTreeMap::new();
        for (symbol, entry) in declaration_entries {
            let resolved = self.declaration_call_target(symbol, *entry)?;
            targets.insert(symbol.as_str(), resolved);
        }
        let calls = self
            .plan
            .source_occurrences
            .iter()
            .flatten()
            .filter_map(|occurrence| match occurrence.expr {
                RuntimeExpr::DeclarationRef { symbol } => targets
                    .get(symbol.as_str())
                    .copied()
                    .map(|(target, class)| {
                        (StaticNodeId(occurrence.static_origin.0), target, class)
                    }),
                _ => None,
            })
            .collect::<Vec<_>>();
        for (caller, callee, class) in calls {
            self.edge(caller, callee, EdgeKind::DeclarationCall)?;
            // Keyed by the REFERENCE occurrence, which is the same key the
            // resolved call record is looked up by in lowering. One reference
            // cannot acquire two classes: `source_occurrences` is dense by
            // origin, so each reference is visited once.
            if self
                .plan
                .declaration_call_targets
                .insert(StaticOriginId(caller.0), class)
                .is_some()
            {
                return Err(planner_error(
                    "one declaration-reference occurrence resolved two call target classes",
                ));
            }
        }
        Ok(())
    }

    /// **`RT-DECL-CLOSURE-PORT` `D4` — the selective retarget, and the only
    /// place it is decided.**
    ///
    /// ⛔ **This is not a blanket retarget, and the difference is load-bearing.**
    /// A non-closure transparent declaration's scheduling entry *is* its unit —
    /// a legitimately zero-input thunk — so moving its `DeclarationCall` edge
    /// would break every declaration call the corpus already makes. Only a
    /// `Closure` / `LexicalClosure` seed owns a second, declaration-owned unit
    /// to move to.
    ///
    /// The discriminator is read from the declaration's **planned occurrence**,
    /// not from the entry's shape or from a reverse search for a body: those
    /// two `RuntimeExpr` arms are exactly the pair that emits an
    /// `EdgeKind::StaticBody` edge, which is what makes "is a closure seed" and
    /// "has one forward static-body edge" the same fact stated twice. ⇒ Both are
    /// asserted, so a plan where they disagree fails in planning.
    fn declaration_call_target(
        &self,
        symbol: &str,
        entry: StaticNodeId,
    ) -> Result<(StaticNodeId, DeclarationCallTargetClass), CraneliftBackendError> {
        let occurrence = self
            .plan
            .declaration_occurrences
            .get(symbol)
            .copied()
            .ok_or_else(|| {
                planner_error("a planned transparent declaration has no occurrence origin")
            })?;
        let seed = self
            .plan
            .source_occurrences
            .get(occurrence.0 as usize)
            .and_then(Option::as_ref)
            .map(|planned| {
                matches!(
                    planned.expr,
                    RuntimeExpr::Closure { .. } | RuntimeExpr::LexicalClosure { .. }
                )
            })
            .ok_or_else(|| {
                planner_error("a planned transparent declaration has no source occurrence")
            })?;

        let mut body = None;
        for edge in &self.plan.edges {
            if edge.kind != EdgeKind::StaticBody || edge.from != entry {
                continue;
            }
            if body.is_some() {
                return Err(planner_error(
                    "a transparent declaration entry has two forward static body edges",
                ));
            }
            body = Some(edge.to);
        }
        match (seed, body) {
            (true, Some(body)) => {
                // The mutations act HERE, on a decision the consistency law
                // above has already accepted -- a mutation applied to the
                // discriminator itself would only ever trip that law, and would
                // measure the law instead of the retarget.
                #[cfg(test)]
                match D4_DECLARATION_TARGET_MUTATION.with(std::cell::Cell::get) {
                    D4DeclarationTargetMutation::Exact => {}
                    D4DeclarationTargetMutation::NeverRetarget => {
                        return Ok((entry, DeclarationCallTargetClass::SchedulingEntry));
                    }
                    D4DeclarationTargetMutation::AnyStaticBody => {
                        let any = self
                            .plan
                            .edges
                            .iter()
                            .find(|edge| edge.kind == EdgeKind::StaticBody)
                            .map(|edge| edge.to)
                            .expect("this arm has at least one static body edge");
                        return Ok((any, DeclarationCallTargetClass::CallableDeclaration));
                    }
                }
                Ok((body, DeclarationCallTargetClass::CallableDeclaration))
            }
            (false, None) => Ok((entry, DeclarationCallTargetClass::SchedulingEntry)),
            (true, None) => Err(planner_error(
                "a closure-seed transparent declaration has no forward static body edge to its \
                 callable unit",
            )),
            (false, Some(_)) => Err(planner_error(
                "a non-closure transparent declaration entry has a forward static body edge",
            )),
        }
    }


    fn edge(
        &mut self,
        from: StaticNodeId,
        to: StaticNodeId,
        kind: EdgeKind,
    ) -> Result<(), CraneliftBackendError> {
        let edge = u32::try_from(self.plan.edges.len())
            .map(StaticEdgeId)
            .map_err(|_| planner_capacity_error("static edge identity exhausted"))?;
        let owner = self.plan.nodes[from.0 as usize].owner;
        self.plan.edges.push(StaticEdge {
            id: edge,
            from,
            to,
            kind,
        });
        self.plan.evidence.push(EdgeEvidence {
            edge: edge.0,
            owner,
            from,
            to,
            kind,
        });
        self.plan
            .planned_helpers
            .push(PlannedHelperKey::edge(kind, edge));
        Ok(())
    }

    fn store(
        &mut self,
        kind: StoreKind,
        local: u32,
        aux: u32,
        child: PersistentNodeId,
    ) -> Result<PersistentNodeId, CraneliftBackendError> {
        let node = PersistentStoreNode {
            kind,
            local,
            aux,
            child,
        };
        if let Some(id) = self.store_interner.get(&node) {
            return Ok(*id);
        }
        let id = u32::try_from(self.plan.stores.len() + 1)
            .map(PersistentNodeId)
            .map_err(|_| planner_capacity_error("persistent store identity exhausted"))?;
        let child_depth = if child.0 == 0 {
            0
        } else {
            *self
                .plan
                .store_depths
                .get(child.0 as usize - 1)
                .ok_or_else(|| planner_error("persistent store child is not closed"))?
        };
        self.plan.stores.push(node);
        self.plan.store_depths.push(
            child_depth
                .checked_add(1)
                .ok_or_else(|| planner_capacity_error("persistent chain depth exhausted"))?,
        );
        self.store_interner.insert(node, id);
        Ok(id)
    }

    fn frame(
        &mut self,
        tag: u32,
        ordinal: u32,
        ctx: PlanContext,
        successor: StaticNodeId,
    ) -> Result<DynamicActivationFrame, CraneliftBackendError> {
        let syntax = self.store(StoreKind::Syntax, tag, ordinal, PersistentNodeId(0))?;
        let path = self.store(StoreKind::Path, ordinal, 0, ctx.path)?;
        let normal = self.store(StoreKind::Continuation, successor.0, 0, ctx.continuation)?;
        Ok(DynamicActivationFrame {
            syntax,
            environment: ctx.environment,
            normal,
            abrupt: PersistentNodeId(0),
            path,
            cleanup: ctx.cleanup,
            affine: ctx.affine,
            source_return: ctx.source_return,
        })
    }

    /// Plans a positional operand sequence. Returns the chain **entry** — what the
    /// parent schedules — and each element's **occurrence** origin indexed by its
    /// source position, which is what the parent records as its positional child
    ///. The two are different values for a
    /// `ComputationalMatch` element, and mixing them is a category error.
    fn plan_sequence(
        &mut self,
        expressions: &[&'src RuntimeExpr],
        ctx: PlanContext,
        successor: StaticNodeId,
        exit_kind: EdgeKind,
    ) -> Result<(StaticNodeId, Vec<StaticOriginId>), CraneliftBackendError> {
        let mut next = successor;
        let mut next_kind = exit_kind;
        let mut occurrences = vec![None; expressions.len()];
        for (ordinal, expression) in expressions.iter().enumerate().rev() {
            let planned = self.plan_expr(expression, ctx, next, next_kind, ordinal as u32)?;
            next = planned.entry;
            // Fusion-claim parameter order depends on this original-ordinal
            // write-back: reverse planning changes the execution chain, never
            // the positional child slice that reaches the semantic arena.
            occurrences[ordinal] = Some(planned.occurrence);
            next_kind = EdgeKind::Continue;
        }
        let occurrences = occurrences
            .into_iter()
            .map(|occurrence| {
                occurrence.ok_or_else(|| {
                    planner_error("operand sequence position has no planned occurrence")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok((next, occurrences))
    }

    /// Plans one eliminator's case bodies. Returns the dispatch **entry** and each
    /// body's **occurrence** origin by source position (D9): the case test edges
    /// to the body's `entry`, while the parent records the body's `occurrence`.
    fn plan_cases(
        &mut self,
        bodies: &[(&'src RuntimeExpr, usize)],
        ctx: PlanContext,
        successor: StaticNodeId,
        exit_kind: EdgeKind,
        default: StaticNodeId,
    ) -> Result<(StaticNodeId, Vec<StaticOriginId>), CraneliftBackendError> {
        let mut reject = default;
        let mut occurrences = vec![None; bodies.len()];
        for (ordinal, (body, binders)) in bodies.iter().enumerate().rev() {
            let mut body_ctx = ctx;
            for binder in 0..*binders {
                body_ctx.environment = self.store(
                    StoreKind::Environment,
                    binder as u32,
                    0,
                    body_ctx.environment,
                )?;
            }
            let planned = self.plan_expr(body, body_ctx, successor, exit_kind, ordinal as u32)?;
            occurrences[ordinal] = Some(planned.occurrence);
            let owner = self.source()?;
            let frame = self.frame(0x80, ordinal as u32, ctx, reject)?;
            let test = self.control_node(TransitionKind::CaseTest, owner, frame)?;
            // Topology: the test selects the body's SCHEDULING entry.
            self.edge(test, planned.entry, EdgeKind::Select)?;
            self.edge(test, reject, EdgeKind::Reject)?;
            reject = test;
        }
        let occurrences = occurrences
            .into_iter()
            .map(|occurrence| {
                occurrence.ok_or_else(|| planner_error("case position has no planned occurrence"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok((reject, occurrences))
    }

    /// Plans one expression occurrence and returns **both** of its identities
    /// (D9): the `entry` the parent schedules and the `occurrence` the parent
    /// records at its source position. Every arm but `ComputationalMatch` returns
    /// them equal, by going through `expression_node`.
    /// **Issuance seat 2 — register a static body.**
    ///
    /// Atomically emits `source -> body.entry` as [`EdgeKind::StaticBody`] and
    /// records `body.entry -> body.occurrence` in the pairing relation. It
    /// replaces every bare `StaticBody` edge write, so a body edge without its
    /// issued pair is unconstructible rather than merely rejected — the same
    /// property `register_scheduling_entry` gives seat 1.
    ///
    /// **Generic in the planned form.** It takes the body's returned
    /// [`PlannedExpr`] and names no expression shape. At both call sites the
    /// planner already holds `body.entry` and `body.occurrence` together, so
    /// recording the pair here is issued identity; selecting child 0 afterwards,
    /// scanning the occurrence graph, or using `origin_of(edge.to)` all discard
    /// an identity that was in hand and reconstruct it.
    fn register_static_body(
        &mut self,
        source: StaticNodeId,
        body: PlannedExpr,
    ) -> Result<(), CraneliftBackendError> {
        self.edge(source, body.entry, EdgeKind::StaticBody)?;
        self.plan.record_planned_entry_body(body);
        Ok(())
    }

    fn plan_expr(
        &mut self,
        expr: &'src RuntimeExpr,
        ctx: PlanContext,
        successor: StaticNodeId,
        exit_kind: EdgeKind,
        ordinal: u32,
    ) -> Result<PlannedExpr, CraneliftBackendError> {
        #[cfg(test)]
        let _recursive_lowering_frame = RecursiveLoweringFrameGuard::enter();
        let owner = self.source()?;
        let tag = runtime_expr_tag(expr);
        let frame = self.frame(tag, ordinal, ctx, successor)?;
        let ctx = PlanContext {
            continuation: frame.normal,
            path: frame.path,
            ..ctx
        };
        match expr {
            RuntimeExpr::Trap(_) => {
                let node =
                    self.expression_node(TransitionKind::Evaluate, owner, frame, expr, &[])?;
                self.edge(node.entry, self.trap_terminal, EdgeKind::Trap)?;
                Ok(node)
            }
            RuntimeExpr::Value(_)
            | RuntimeExpr::Var(_)
            | RuntimeExpr::DeclarationRef { .. }
            | RuntimeExpr::ImportedDeclarationRef { .. } => {
                let node =
                    self.expression_node(TransitionKind::Evaluate, owner, frame, expr, &[])?;
                self.edge(node.entry, successor, exit_kind)?;
                Ok(node)
            }
            RuntimeExpr::CheckedJoinSite { body, .. }
            | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
            | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
            | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
            | RuntimeExpr::CheckedComputationalIHInvocation { body, .. }
            | RuntimeExpr::Project { record: body, .. } => {
                let body = self.plan_expr(body, ctx, successor, exit_kind, 0)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &[body.occurrence],
                )?;
                self.edge(node.entry, body.entry, EdgeKind::Continue)?;
                Ok(node)
            }
            RuntimeExpr::Let { value, body } => {
                let environment = self.store(StoreKind::Environment, 0, 0, ctx.environment)?;
                let body = self.plan_expr(
                    body,
                    PlanContext { environment, ..ctx },
                    successor,
                    exit_kind,
                    1,
                )?;
                let value = self.plan_expr(value, ctx, body.entry, EdgeKind::Continue, 0)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &[value.occurrence, body.occurrence],
                )?;
                self.edge(node.entry, value.entry, EdgeKind::Continue)?;
                Ok(node)
            }
            RuntimeExpr::If {
                scrutinee,
                then_expr,
                else_expr,
            } => {
                let then_entry = self.plan_expr(then_expr, ctx, successor, exit_kind, 1)?;
                let else_entry = self.plan_expr(else_expr, ctx, successor, exit_kind, 2)?;
                let branch_owner = self.source()?;
                let branch = self.control_node(TransitionKind::Branch, branch_owner, frame)?;
                self.edge(branch, then_entry.entry, EdgeKind::Select)?;
                self.edge(branch, else_entry.entry, EdgeKind::Reject)?;
                let scrutinee = self.plan_expr(scrutinee, ctx, branch, EdgeKind::Continue, 0)?;
                let node = self.expression_node(
                    TransitionKind::Evaluate,
                    owner,
                    frame,
                    expr,
                    &[
                        scrutinee.occurrence,
                        then_entry.occurrence,
                        else_entry.occurrence,
                    ],
                )?;
                self.edge(node.entry, scrutinee.entry, EdgeKind::Continue)?;
                Ok(node)
            }
            RuntimeExpr::Match {
                scrutinee, cases, ..
            } => {
                let default_owner = self.source()?;
                let default = self.control_node(TransitionKind::Evaluate, default_owner, frame)?;
                self.edge(default, self.trap_terminal, EdgeKind::Trap)?;
                let bodies = cases
                    .iter()
                    .map(|case| (&case.body, case.binders))
                    .collect::<Vec<_>>();
                let (dispatch, case_bodies) =
                    self.plan_cases(&bodies, ctx, successor, exit_kind, default)?;
                let scrutinee = self.plan_expr(scrutinee, ctx, dispatch, EdgeKind::Continue, 0)?;
                let mut children = Vec::with_capacity(1 + case_bodies.len());
                children.push(scrutinee.occurrence);
                children.extend(case_bodies);
                let node =
                    self.expression_node(TransitionKind::Evaluate, owner, frame, expr, &children)?;
                self.edge(node.entry, scrutinee.entry, EdgeKind::Continue)?;
                Ok(node)
            }
            RuntimeExpr::ComputationalMatch {
                scrutinee, cases, ..
            } => {
                let cleanup = self.store(StoreKind::Cleanup, owner.0, 0, ctx.cleanup)?;
                let affine = self.store(StoreKind::Affine, owner.0, 0, ctx.affine)?;
                let control_ctx = PlanContext {
                    cleanup,
                    affine,
                    ..ctx
                };
                let completed = self.control_node(TransitionKind::CompletedTail, owner, frame)?;
                let tail = self.control_node(TransitionKind::ProducerTail, owner, frame)?;
                let wrapper = self.control_node(TransitionKind::ProducerWrapper, owner, frame)?;
                let resume = self.push_node(TransitionKind::SourceReturnResume, owner, frame)?;
                self.edge(resume, wrapper, EdgeKind::InvokeProducerWrapper)?;
                self.edge(wrapper, tail, EdgeKind::InvokeProducerTail)?;
                self.edge(tail, completed, EdgeKind::CompleteProducerTail)?;
                self.edge(completed, successor, exit_kind)?;
                let source_return = self.store(
                    StoreKind::SourceReturn,
                    wrapper.0,
                    tail.0,
                    ctx.source_return,
                )?;
                let control_ctx = PlanContext {
                    source_return,
                    ..control_ctx
                };
                for id in [completed, tail, wrapper, resume] {
                    self.plan.nodes[id.0 as usize].frame.source_return = source_return;
                    self.plan.nodes[id.0 as usize].frame.cleanup = cleanup;
                    self.plan.nodes[id.0 as usize].frame.affine = affine;
                }
                let default_owner = self.source()?;
                let default = self.control_node(TransitionKind::Evaluate, default_owner, frame)?;
                self.edge(default, self.trap_terminal, EdgeKind::Trap)?;
                let bodies = cases
                    .iter()
                    .map(|case| {
                        (
                            &case.body,
                            case.argument_binders + case.recursive_positions.len(),
                        )
                    })
                    .collect::<Vec<_>>();
                let (dispatch, case_bodies) = self.plan_cases(
                    &bodies,
                    control_ctx,
                    resume,
                    EdgeKind::SourceReturnOwnedResume,
                    default,
                )?;
                let scrutinee =
                    self.plan_expr(scrutinee, control_ctx, dispatch, EdgeKind::Continue, 0)?;
                let mut children = Vec::with_capacity(1 + case_bodies.len());
                children.push(scrutinee.occurrence);
                children.extend(case_bodies);
                self.expression_seed(resume, expr, &children)?;
                // ⭐ THE SOLE SPLIT. This occurrence's record
                // lives on `resume`, because the resume owns the outer edges and
                // must exist before the cases are planned — but the transfer
                // graph still schedules the SCRUTINEE, exactly as before. So the
                // two identities genuinely differ here, and returning one value
                // for both is what made a parent record the scrutinee's identity
                // as this match's. ⛔ Do not "fix" this by returning `resume` as
                // the entry: that would change the approved Boundary-A topology.
                Ok(PlannedExpr {
                    entry: scrutinee.entry,
                    occurrence: origin_of(resume),
                })
            }
            RuntimeExpr::Closure { body, .. } => {
                let body_return_owner = self.source()?;
                let body_return =
                    self.control_node(TransitionKind::ClosureBody, body_return_owner, frame)?;
                self.edge(body_return, self.terminal, EdgeKind::Continue)?;
                let body = self.plan_expr(body, ctx, body_return, EdgeKind::Continue, 0)?;
                let node = self.expression_node(
                    TransitionKind::Evaluate,
                    owner,
                    frame,
                    expr,
                    &[body.occurrence],
                )?;
                self.edge(node.entry, successor, exit_kind)?;
                self.register_static_body(node.entry, body)?;
                Ok(node)
            }
            RuntimeExpr::LexicalClosure { captures, body, .. } => {
                let body_return_owner = self.source()?;
                let body_return =
                    self.control_node(TransitionKind::ClosureBody, body_return_owner, frame)?;
                self.edge(body_return, self.terminal, EdgeKind::Continue)?;
                let body = self.plan_expr(body, ctx, body_return, EdgeKind::Continue, 0)?;
                let captures = captures.iter().collect::<Vec<_>>();
                let (capture_entry, capture_occurrences) =
                    self.plan_sequence(&captures, ctx, successor, exit_kind)?;
                let mut children = Vec::with_capacity(1 + capture_occurrences.len());
                children.push(body.occurrence);
                children.extend(capture_occurrences);
                let node =
                    self.expression_node(TransitionKind::Evaluate, owner, frame, expr, &children)?;
                self.edge(
                    node.entry,
                    capture_entry,
                    if captures.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                self.register_static_body(node.entry, body)?;
                Ok(node)
            }
            RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
                let expressions = args.iter().collect::<Vec<_>>();
                let (first, operand_occurrences) =
                    self.plan_sequence(&expressions, ctx, successor, exit_kind)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &operand_occurrences,
                )?;
                self.edge(
                    node.entry,
                    first,
                    if expressions.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                Ok(node)
            }
            RuntimeExpr::Record { fields } => {
                let expressions = fields.iter().map(|(_, value)| value).collect::<Vec<_>>();
                let (first, operand_occurrences) =
                    self.plan_sequence(&expressions, ctx, successor, exit_kind)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &operand_occurrences,
                )?;
                self.edge(
                    node.entry,
                    first,
                    if expressions.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                Ok(node)
            }
            RuntimeExpr::Call { callee, args } => {
                let mut expressions = Vec::with_capacity(args.len() + 1);
                expressions.push(callee.as_ref());
                expressions.extend(args);
                let (first, operand_occurrences) =
                    self.plan_sequence(&expressions, ctx, successor, exit_kind)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &operand_occurrences,
                )?;
                self.edge(
                    node.entry,
                    first,
                    if expressions.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                Ok(node)
            }
            RuntimeExpr::Effect {
                capability, args, ..
            } => {
                let mut expressions =
                    Vec::with_capacity(args.len() + usize::from(capability.is_some()));
                if let Some(capability) = capability {
                    expressions.push(capability.value.as_ref());
                }
                expressions.extend(args);
                let (first, operand_occurrences) =
                    self.plan_sequence(&expressions, ctx, successor, exit_kind)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &operand_occurrences,
                )?;
                self.edge(
                    node.entry,
                    first,
                    if expressions.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                Ok(node)
            }
        }
    }

    fn finish(
        mut self,
        symbols: &crate::NativeProcessSymbols,
        root_ingress: AbiRootIngress,
        functionized_units: bool,
    ) -> Result<StaticTransitionPlan<'src>, CraneliftBackendError> {
        let (synthesized_identities, synthesized_io_roles) =
            build_synthesized_constructor_inventory(&mut self.plan.semantic_material, symbols)?;
        let planned_entry_bodies = self.plan.planned_entry_bodies.clone();
        let entry_bodies = |entry: StaticNodeId| {
            planned_entry_bodies
                .iter()
                .find(|pair| pair.entry == entry)
                .map(|pair| pair.body_occurrence)
        };
        self.plan.semantic = build_semantic_plane(
            &self.plan.nodes,
            &self.plan.edges,
            &self.plan.entries,
            &entry_bodies,
            self.plan.root_entry,
            &self.plan.semantic_sources,
            &self.plan.semantic_material,
        )?;
        self.plan
            .semantic
            .install_synthesized_constructor_inventory(
                synthesized_identities,
                synthesized_io_roles,
            );
        self.plan
            .semantic
            .validate_synthesized_constructor_inventory()?;
        // Slice 0's substrate closes before ABI descriptors or any emitted
        // allocation can exist. The two populations are independently
        // re-derived and checked; their cross-population closure is a third,
        // separately named gate.
        self.plan.case_emissions = build_case_emission_plan(&self.plan)?;
        validate_case_emission_plan(&self.plan, &self.plan.case_emissions)?;
        self.plan.occurrence_authorities = build_occurrence_authority_plan(&self.plan)?;
        validate_occurrence_authority_plan(&self.plan, &self.plan.occurrence_authorities)?;
        validate_substrate_preallocation_closure(
            &self.plan,
            &self.plan.case_emissions,
            &self.plan.occurrence_authorities,
        )?;
        // `B2R` — the representation contract is built from the owner partition
        // the line above just validated, and it fails **before** anything is
        // emitted. It is deliberately not deferred to lowering: a contract that
        // is only checked at emission time cannot be a *pre*-emission gate.
        let root_entry = self
            .plan
            .root_entry
            .ok_or_else(|| planner_error("plan has no root scheduling entry"))?;
        self.plan.root_ingress = root_ingress;
        // `RT-DECL-CLOSURE-PORT` `D2` — the declaration occurrences are the
        // discriminator that splits a `StaticBody` target into a callable
        // declaration unit or an anonymous closure body. They come from the one
        // loop that plans transparent declarations, so the ABI plane cannot
        // classify a unit as declaration-owned unless the planner actually
        // planned that declaration.
        let declaration_origins: BTreeSet<StaticOriginId> =
            self.plan.declaration_occurrences.values().copied().collect();
        self.plan.abi = build_abi_plane(
            &self.plan.semantic,
            &self.plan.nodes,
            &self.plan.semantic_sources,
            &self.plan.edges,
            &self.plan.entries,
            &declaration_origins,
            root_entry,
            root_ingress,
        )?;
        let (
            continuation_specializations,
            continuation_specialization_calls,
            required_consumer_projections,
            continuation_contexts,
            _admitted_discoveries,
        ) = build_continuation_specialization_plan(&self.plan)?;
        self.plan.continuation_specializations = continuation_specializations;
        self.plan.continuation_specialization_calls = continuation_specialization_calls;
        self.plan.required_consumer_projections = required_consumer_projections;
        self.plan.continuation_contexts = continuation_contexts;
        validate_continuation_specialization_plan(&self.plan)?;
        install_continuation_specialization_abi(
            &mut self.plan.abi,
            &self.plan.continuation_specializations,
        )?;
        // `D5a`: the generated contexts' own ABI, in its own arenas. ⛔ Installed
        // AFTER the specialization ABI and into separate vectors, never appended
        // to `continuation_descriptors` -- that population is exactly the
        // continuation-callee partition, and admitting a caller-side context
        // there would make one identity domain readable as the other.
        install_continuation_context_abi(&mut self.plan.abi, &self.plan.continuation_contexts)?;
        // `D3b` STAGE 2 — every context id now exists, so every structural frame
        // requirement can be resolved to exactly one identity.
        //
        // ⛔ **Placed AFTER validation deliberately.** The validator re-derives
        // the whole continuation plan and compares it for exact equality; a
        // finalized sibling field stamped before that runs is state the
        // re-derivation cannot produce, so the comparison would fail on the
        // stamping rather than on any real disagreement. Finalization is a
        // post-derivation publication step, not part of the derivation.
        //
        // ⭐ Still before anything can publish a view: `continuation_units` and
        // `continuation_contexts` both require the ABI installed above, so the
        // earliest possible view is built after this line.
        finalize_continuation_availability_plan(&mut self.plan)?;
        self.plan.join_results = build_join_result_plan(&self.plan, functionized_units)?;
        // `D7` — the aggregate occurrence population is built HERE, last, and
        // deliberately not beside the occurrence authorities it also reads.
        //
        // Two inputs make the position load-bearing, and only one of them was
        // obvious. The meet is taken over the per-child lifetimes in
        // `occurrence_authorities`, so it cannot precede those. It is ALSO
        // taken over each child's planned result representation, which is
        // `join_results` and is not built until this line — a child the
        // emitter will materialize as a native scalar pair has no referent at
        // all, and reading the lifetime without the representation makes every
        // call-shaped child look arena-owned.
        self.plan.aggregate_ownership = build_aggregate_ownership_plan(&self.plan)?;
        validate_aggregate_ownership_plan(&self.plan, &self.plan.aggregate_ownership)?;
        // ⛔ After `join_results` for the same reason the ownership plan is: a
        // seat's consumer phase is a fact about the child's planned result
        // representation, which does not exist until that line.
        self.plan.host_effect_seats = build_host_effect_seat_plan(&self.plan)?;
        validate_host_effect_seat_plan(&self.plan, &self.plan.host_effect_seats)?;
        #[cfg(test)]
        apply_static_worker_member_mutation(&mut self.plan);
        self.plan.validate()?;
        Ok(self.plan)
    }
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
                    Some(RuntimeExpr::Closure { .. }) => {
                        PlannedResultFieldKindForTest::Closure
                    }
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

    /// Consume the planner-owned result contract for one source join.
    ///
    /// The token is keyed only by the opaque origin. Diagnostic labels and
    /// lowered values do not participate in selection.
    pub(in crate::cranelift_backend) fn join_plan_token(
        &self,
        origin: StaticOriginId,
    ) -> Result<JoinPlanToken, CraneliftBackendError> {
        self.join_plan_token_if_planned(origin)?
            .ok_or_else(|| planner_error("static origin has no planned source join"))
    }

    /// Project the authoritative join population onto one traversal entry.
    ///
    /// `None` means this validated source occurrence is not a join. Lowering
    /// therefore never maintains a second spelling inventory of join forms.
    pub(in crate::cranelift_backend) fn join_plan_token_if_planned(
        &self,
        origin: StaticOriginId,
    ) -> Result<Option<JoinPlanToken>, CraneliftBackendError> {
        let planned = self
            .join_results
            .get(origin.0 as usize)
            .ok_or_else(|| planner_error("static origin is outside the join result plan"))?;
        Ok(planned.map(|planned| JoinPlanToken {
            origin,
            representation: planned.representation,
            has_continuing_predecessor: planned.has_continuing_predecessor,
        }))
    }

    /// Every source-join contract owned by one generated function.
    ///
    /// This is a projection of the already-validated occurrence population and
    /// semantic owner partition. Lowering uses it only as the closed expected
    /// set for its end-of-function consumption check; it cannot add or omit a
    /// join by maintaining a second caller inventory.
    pub(in crate::cranelift_backend) fn required_join_origins(
        &self,
        function: PredeclaredFunctionId,
    ) -> Result<BTreeSet<StaticOriginId>, CraneliftBackendError> {
        let mut required = BTreeSet::new();
        for (index, (occurrence, join)) in self
            .source_occurrences
            .iter()
            .zip(&self.join_results)
            .enumerate()
        {
            let (Some(occurrence), Some(_)) = (occurrence, join) else {
                continue;
            };
            if occurrence.static_origin.0 as usize != index {
                return Err(planner_error(
                    "join consumption population is not keyed by source origin",
                ));
            }
            if self.semantic.function_owner(occurrence.static_origin)? == Some(function) {
                required.insert(occurrence.static_origin);
            }
        }
        Ok(required)
    }

    /// Planned joins in one source subtree that remain in its function owner.
    ///
    /// This is the structural population used when lowering proves that a
    /// branch is statically unselected. The traversal follows the semantic
    /// plane's validated positional-child inventory, never a second
    /// `RuntimeExpr` spelling list, and stops at declared-unit owner
    /// boundaries. A closure body in a dead outer branch is still validated
    /// when its own generated function is emitted.
    pub(in crate::cranelift_backend) fn source_join_origins_in_owner_subtree(
        &self,
        root: StaticOriginId,
    ) -> Result<BTreeSet<StaticOriginId>, CraneliftBackendError> {
        let owner = self
            .semantic
            .function_owner(root)?
            .ok_or_else(|| planner_error("source subtree root has no function owner"))?;
        let mut pending = vec![root];
        let mut visited = BTreeSet::new();
        let mut joins = BTreeSet::new();
        while let Some(origin) = pending.pop() {
            if !visited.insert(origin) {
                continue;
            }
            if self.semantic.function_owner(origin)? != Some(owner) {
                continue;
            }
            let index = origin.0 as usize;
            let occurrence = self
                .source_occurrences
                .get(index)
                .and_then(Option::as_ref)
                .ok_or_else(|| planner_error("source subtree names no planned occurrence"))?;
            if occurrence.static_origin != origin {
                return Err(planner_error(
                    "source subtree occurrence disagrees with its positional origin",
                ));
            }
            if is_source_join(occurrence.expr) {
                joins.insert(origin);
            }
            pending.extend(self.semantic.child_origins(origin)?.iter().copied());
        }
        Ok(joins)
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











    /// The closed planned seat population, for the whole-pass seat closeout.
    pub(in crate::cranelift_backend) fn host_effect_seat_records(&self) -> &[PlannedEffectSeat] {
        &self.host_effect_seats
    }

    /// **The planned slot population of ONE effect occurrence.**
    ///
    /// ⛔ This is what a visit's completeness is measured against, so it is
    /// derived from the population rather than from the occurrence's argument
    /// list: a visit that read every argument it happened to lower would be
    /// complete by construction.
    pub(in crate::cranelift_backend) fn host_effect_seat_slots(
        &self,
        effect_origin: StaticOriginId,
    ) -> BTreeSet<EffectSeatSlot> {
        self.host_effect_seats
            .iter()
            .filter(|record| record.effect_origin == effect_origin)
            .map(|record| record.slot)
            .collect()
    }

    /// The exact argument slots that the operation's planned synthesized
    /// result tree consumes as site-bound operands.
    ///
    /// This is deliberately occurrence-keyed even though the recipe is chosen
    /// by operation: the emitter asks about the effect occurrence it is
    /// lowering, and a non-effect coordinate must refuse rather than borrow an
    /// operation from elsewhere.
    pub(in crate::cranelift_backend) fn host_effect_site_operand_slots(
        &self,
        effect_origin: StaticOriginId,
    ) -> Result<BTreeSet<EffectSeatSlot>, CraneliftBackendError> {
        let operation = self.host_effect_operation(effect_origin)?;
        let tree = host_effect_recipe_tree(operation);
        let mut ordinals = BTreeSet::new();
        collect_site_operand_ordinals(tree.error, &mut ordinals);
        collect_site_operand_ordinals(tree.ok, &mut ordinals);
        Ok(ordinals.into_iter().map(EffectSeatSlot::Argument).collect())
    }

    /// **Claim the ONE planned record for an exact seat.**
    ///
    /// ⛔ Keyed on the occurrence and the slot, never on the operation alone: a
    /// lookup by operation would answer for whichever occurrence of that
    /// operation came first, so one effect's proof would authorize another's
    /// consumption. A seat with no record is a loud refusal, not a fallback —
    /// it means the emitter reached a seat planning never derived.
    pub(in crate::cranelift_backend) fn host_effect_seat(
        &self,
        effect_origin: StaticOriginId,
        slot: EffectSeatSlot,
    ) -> Result<PlannedEffectSeat, CraneliftBackendError> {
        self.host_effect_seats
            .iter()
            .find(|record| record.effect_origin == effect_origin && record.slot == slot)
            .copied()
            .ok_or_else(|| {
                planner_error(format!(
                    "host effect occurrence {effect_origin:?} has no planned seat at {slot:?}"
                ))
            })
    }




    /// The host operation of one `Effect` seat.
    fn host_effect_operation(
        &self,
        seat: StaticOriginId,
    ) -> Result<ken_host::HostOpV1, CraneliftBackendError> {
        let occurrence = self
            .source_occurrences
            .get(seat.0 as usize)
            .and_then(|slot| slot.as_ref())
            .ok_or_else(|| planner_error("synthesized aggregate seat is not an occurrence"))?;
        match occurrence.expr {
            RuntimeExpr::Effect { operation, .. } => Ok(*operation),
            _ => Err(planner_error(
                "synthesized aggregate seat is not a host effect",
            )),
        }
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
                return Err(planner_error(
                    "planned entry has no issued body occurrence",
                ));
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
    fn register_scheduling_entry(&mut self, planned: PlannedExpr) {
        self.entries.push(planned.entry);
        self.record_planned_entry_body(planned);
    }

    /// Record one issued `entry -> body_occurrence` row.
    ///
    /// **The single writer of the relation**, shared by both issuance seats so
    /// the two cannot drift into two ledgers with two shapes. It takes a
    /// [`PlannedExpr`] and stores the two fields it was handed; it does not ask
    /// what produced them and must never learn.
    fn record_planned_entry_body(&mut self, planned: PlannedExpr) {
        self.planned_entry_bodies.push(PlannedEntryBody {
            entry: planned.entry,
            body_occurrence: planned.occurrence,
        });
    }

    /// The body occurrence issued for one scheduling entry.
    ///
    /// Reads the pairing authority. `None` means this node is not a
    /// scheduling entry at all — it is never a licence to substitute the entry's
    /// own origin, which is precisely the alias this table replaced.
    fn planned_entry_body(&self, entry: StaticNodeId) -> Option<StaticOriginId> {
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

    pub(in crate::cranelift_backend) fn trap_identity(
        &self,
        trap: &RuntimeTrap,
    ) -> Result<PlannedTrapIdentity, CraneliftBackendError> {
        self.trap_catalog
            .iter()
            .position(|candidate| candidate == trap)
            .ok_or_else(|| {
                planner_error(format!(
                    "trap outcome has no planner-bound identity: {trap:?}"
                ))
            })
            .and_then(|index| {
                u32::try_from(index + 1)
                    .map(PlannedTrapIdentity)
                    .map_err(|_| planner_capacity_error("trap identity exhausted"))
            })
    }

    pub(in crate::cranelift_backend) fn trap_catalog(&self) -> Vec<RuntimeTrap> {
        self.trap_catalog.clone()
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
    ) -> Result<
        Vec<(PredeclaredFunctionId, StaticOriginId, StaticOriginId)>,
        CraneliftBackendError,
    > {
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
        for (caller, _callee, callee_origin) in
            self.semantic.static_body_call_edges(&self.edges)?
        {
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

    fn helper_key_for_activation(
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

    fn validate(&self) -> Result<(), CraneliftBackendError> {
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

    fn validate_join_result_plan(&self) -> Result<(), CraneliftBackendError> {
        if self.join_results.len() != self.source_occurrences.len() {
            return Err(planner_error(
                "join result plan is not dense over the occurrence table",
            ));
        }
        for (index, (occurrence, join)) in self
            .source_occurrences
            .iter()
            .zip(&self.join_results)
            .enumerate()
        {
            match (occurrence, join) {
                (Some(occurrence), Some(_)) if is_source_join(occurrence.expr) => {
                    if occurrence.static_origin.0 as usize != index {
                        return Err(planner_error(
                            "join result entry is not keyed by its source origin",
                        ));
                    }
                }
                (Some(occurrence), None) if !is_source_join(occurrence.expr) => {}
                (Some(_), None) => {
                    return Err(planner_error(
                        "source join occurrence has no result representation",
                    ));
                }
                (Some(_), Some(_)) => {
                    return Err(planner_error(
                        "join result entry names a non-join source occurrence",
                    ));
                }
                (None, Some(_)) => {
                    return Err(planner_error(
                        "join result entry names no source occurrence",
                    ));
                }
                (None, None) => {}
            }
        }
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
    fn census(&self) -> BoundaryACensus {
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
    fn semantic_census(&self) -> BoundaryB1Census {
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

fn runtime_expr_tag(expr: &RuntimeExpr) -> u32 {
    match expr {
        RuntimeExpr::CheckedJoinSite { .. } => 0,
        RuntimeExpr::CheckedSubcontinuationFrame { .. } => 1,
        RuntimeExpr::CheckedRecursiveInvocation { .. } => 2,
        RuntimeExpr::CheckedComputationalIHSlots { .. } => 3,
        RuntimeExpr::CheckedComputationalIHInvocation { .. } => 4,
        RuntimeExpr::Value(_) => 5,
        RuntimeExpr::Var(_) => 6,
        RuntimeExpr::Let { .. } => 7,
        RuntimeExpr::If { .. } => 8,
        RuntimeExpr::PrimitiveCall { .. } => 9,
        RuntimeExpr::Construct { .. } => 10,
        RuntimeExpr::Match { .. } => 11,
        RuntimeExpr::ComputationalMatch { .. } => 12,
        RuntimeExpr::Record { .. } => 13,
        RuntimeExpr::Project { .. } => 14,
        RuntimeExpr::Closure { .. } => 15,
        RuntimeExpr::LexicalClosure { .. } => 16,
        RuntimeExpr::DeclarationRef { .. } => 17,
        RuntimeExpr::ImportedDeclarationRef { .. } => 18,
        RuntimeExpr::Call { .. } => 19,
        RuntimeExpr::Effect { .. } => 20,
        RuntimeExpr::Trap(_) => 21,
    }
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

    pub(super) fn d8_mixed_join(swapped: bool) -> RuntimeExpr {
        let carried = RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Carried".to_string(),
            binders: 0,
            body: RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(11.into()))),
                }),
                args: Vec::new(),
            },
        };
        let specialized = RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Specialized".to_string(),
            binders: 0,
            body: RuntimeExpr::Value(RuntimeValue::Int(7.into())),
        };
        RuntimeExpr::Match {
            // Deliberately specialized: the scrutinee phase is not the result
            // representation selector.
            scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Constructor {
                constructor: "ctor:fixture::D8::Carried".to_string(),
                args: Vec::new(),
            })),
            cases: if swapped {
                vec![specialized, carried]
            } else {
                vec![carried, specialized]
            },
            default: trap("D8 mixed join default"),
        }
    }

    pub(super) fn d8_functionized_plan(
        expr: &RuntimeExpr,
    ) -> Result<StaticTransitionPlan<'_>, CraneliftBackendError> {
        plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &crate::NativeProcessSymbols::legacy_prelude(),
            AbiRootIngress::Value,
            true,
        )
    }

    pub(super) fn d8_environment_join(swapped: bool) -> RuntimeExpr {
        let carried = RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Carried".to_string(),
            binders: 0,
            body: RuntimeExpr::Var(0),
        };
        let specialized = RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Specialized".to_string(),
            binders: 0,
            body: RuntimeExpr::Value(RuntimeValue::Int(7.into())),
        };
        RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(11.into()))),
                }),
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Match {
                scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Constructor {
                    constructor: "ctor:fixture::D8::Carried".to_string(),
                    args: Vec::new(),
                })),
                cases: if swapped {
                    vec![specialized, carried]
                } else {
                    vec![carried, specialized]
                },
                default: trap("D8 environment join default"),
            }),
        }
    }

    pub(super) fn assert_d8_environment_join_is_carrier(swapped: bool) {
        let expr = d8_environment_join(swapped);
        let plan = d8_functionized_plan(&expr).expect("environment join plans");
        let root = plan.root_static_origin().expect("root origin");
        let join = plan
            .semantic
            .child_origin(root, 1)
            .expect("let body origin");
        let token = plan
            .join_plan_token(join)
            .expect("nested environment join has one plan entry");
        assert_eq!(
            token.representation,
            JoinResultRepresentation::CarrierWord,
            "the exact nested join lost its let-bound declared-unit result"
        );
    }

    pub(super) fn d8_bound_callable_join(swapped: bool) -> RuntimeExpr {
        let carried_call = RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Call".to_string(),
            binders: 0,
            body: RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Var(0)),
                args: Vec::new(),
            },
        };
        let specialized = RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Specialized".to_string(),
            binders: 0,
            body: RuntimeExpr::Value(RuntimeValue::Int(7.into())),
        };
        RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: Vec::new(),
                body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(11.into()))),
            }),
            body: Box::new(RuntimeExpr::Match {
                scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Constructor {
                    constructor: "ctor:fixture::D8::Call".to_string(),
                    args: Vec::new(),
                })),
                cases: if swapped {
                    vec![specialized, carried_call]
                } else {
                    vec![carried_call, specialized]
                },
                default: trap("D8 bound callable join default"),
            }),
        }
    }

    pub(super) fn assert_d8_bound_callable_join_is_carrier(swapped: bool) {
        let expr = d8_bound_callable_join(swapped);
        let plan = d8_functionized_plan(&expr).expect("bound callable join plans");
        let root = plan.root_static_origin().expect("root origin");
        let join = plan
            .semantic
            .child_origin(root, 1)
            .expect("let body origin");
        let token = plan
            .join_plan_token(join)
            .expect("nested bound-callable join has one plan entry");
        assert_eq!(
            token.representation,
            JoinResultRepresentation::CarrierWord,
            "the exact nested join lost the bound closure's callable result"
        );
    }

    pub(super) fn d8_abi_parameter_join(swapped: bool) -> RuntimeExpr {
        let carried = RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Carried".to_string(),
            binders: 0,
            body: RuntimeExpr::Var(0),
        };
        let specialized = RuntimeMatchCase {
            constructor: "ctor:fixture::D8::Specialized".to_string(),
            binders: 0,
            body: RuntimeExpr::Value(RuntimeValue::Int(7.into())),
        };
        RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["carried".to_string()],
                body: Box::new(RuntimeExpr::Match {
                    scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Constructor {
                        constructor: "ctor:fixture::D8::Carried".to_string(),
                        args: Vec::new(),
                    })),
                    cases: if swapped {
                        vec![specialized, carried]
                    } else {
                        vec![carried, specialized]
                    },
                    default: trap("D8 ABI parameter join default"),
                }),
            }),
            args: vec![RuntimeExpr::Value(RuntimeValue::Int(11.into()))],
        }
    }

    pub(super) fn d8_abi_parameter_join_origin(
        plan: &StaticTransitionPlan<'_>,
    ) -> Result<StaticOriginId, CraneliftBackendError> {
        let root = plan.root_static_origin()?;
        let callee = plan.semantic.child_origin(root, 0)?;
        plan.semantic.child_origin(callee, 0)
    }

    #[test]
    fn d8_mixed_join_plan_is_carrier_and_arm_order_independent() {
        for swapped in [false, true] {
            let expr = d8_mixed_join(swapped);
            let plan = d8_functionized_plan(&expr).expect("mixed join plans");
            let token = plan
                .join_plan_token(plan.root_static_origin().expect("root origin"))
                .expect("root join has one plan entry");
            assert_eq!(
                token.representation,
                JoinResultRepresentation::CarrierWord,
                "specialized scrutinee or first-arm order selected a native merge"
            );
            assert!(
                token.has_continuing_predecessor,
                "mixed join lost both continuing predecessors"
            );
        }
    }

    /// MEASURED: the exact nested source join receives `CarrierWord` when one
    /// arm forwards a declared-unit result through a de Bruijn environment
    /// insertion, independently of arm order.
    ///
    /// CLAIMED: D8 phase propagation is monotone through `Let`; a join cannot
    /// be planned as native merely because its carrier reaches it through a
    /// variable.
    ///
    /// GAP: this fixture does not exercise ABI parameters; the adjacent control
    /// pins that independent environment seed.
    #[test]
    fn d8_let_environment_provenance_reaches_the_exact_nested_join() {
        for swapped in [false, true] {
            assert_d8_environment_join_is_carrier(swapped);
        }
    }

    /// MEASURED: a lexical closure bound by `Let`, recovered as `Var(0)`, and
    /// invoked in one mixed-join arm makes that exact join `CarrierWord`
    /// independently of arm order.
    ///
    /// CLAIMED: D8's binder environment retains both a bound value's own phase
    /// and the phase produced when that value is invoked.
    ///
    /// GAP: this is the lexical-binder route. ABI parameters remain governed by
    /// the adjacent closed-slot control.
    #[test]
    fn d8_bound_lexical_callable_provenance_reaches_the_exact_nested_join() {
        for swapped in [false, true] {
            assert_d8_bound_callable_join_is_carrier(swapped);
        }
    }

    /// MEASURED: a functionized unit parameter is seeded from the unit ABI and
    /// reaches the exact join occurrence in the unit body.
    ///
    /// CLAIMED: phase planning uses the closed ABI slot inventory for
    /// parameter/capture environment provenance, rather than classifying every
    /// `Var` as specialized.
    ///
    /// GAP: captures share the same ABI-slot seed path and are not duplicated
    /// in this control.
    #[test]
    fn d8_abi_parameter_provenance_reaches_the_exact_nested_join() {
        for swapped in [false, true] {
            let expr = d8_abi_parameter_join(swapped);
            let plan = d8_functionized_plan(&expr).expect("ABI parameter join plans");
            let join =
                d8_abi_parameter_join_origin(&plan).expect("closure body join has an origin");
            let token = plan
                .join_plan_token(join)
                .expect("nested parameter join has one plan entry");
            assert_eq!(
                token.representation,
                JoinResultRepresentation::CarrierWord,
                "the exact nested join lost its function-unit parameter"
            );
        }
    }

    /// MEASURED before retirement: the same closed ABI-parameter fixture was
    /// `CarrierWord` under functionized emission and `NativeScalarPair` under
    /// the monolithic route, independently of arm order.
    ///
    /// CLAIMED: the validated but inert ABI plane cannot impose carrier
    /// storage on the then-retained monolithic lowering authority.
    ///
    /// GAP: this pins the planner boundary and the native/interpreter parity
    /// suite pins the resulting public observations; it does not compare every
    /// emitted block in the two authorities.
    #[test]
    fn d8_inert_abi_slots_do_not_change_recursive_descent_join_storage() {
        for swapped in [false, true] {
            let expr = d8_abi_parameter_join(swapped);
            let functionized =
                d8_functionized_plan(&expr).expect("functionized ABI parameter join plans");
            let retained = plan_static_transition_graph_with_symbols(
                &expr,
                &BTreeMap::new(),
                &crate::NativeProcessSymbols::legacy_prelude(),
                AbiRootIngress::Value,
                false,
            )
            .expect("retained ABI parameter join plans");
            for (plan, expected) in [
                (&functionized, JoinResultRepresentation::CarrierWord),
                (&retained, JoinResultRepresentation::NativeScalarPair),
            ] {
                let join = d8_abi_parameter_join_origin(plan)
                    .expect("closure body join has an origin");
                assert_eq!(
                    plan.join_plan_token(join)
                        .expect("nested parameter join has one plan entry")
                        .representation,
                    expected,
                    "inert and live ABI slots selected the same join storage"
                );
            }
        }
    }

    /// Reversible mutation: forcing all variable seeds back to the rejected
    /// `SpecializedOnly` behavior must red at the plan assertion, before any
    /// lowering or emitted block can influence the result.
    #[test]
    fn d8_variable_seed_mutation_reds_at_the_plan_boundary() {
        D8_FORCE_VARIABLE_SPECIALIZED.with(|forced| forced.set(true));
        let result = std::panic::catch_unwind(|| {
            assert_d8_environment_join_is_carrier(false);
        });
        D8_FORCE_VARIABLE_SPECIALIZED.with(|forced| forced.set(false));
        assert!(
            result.is_err(),
            "forcing the rejected variable seed did not red the plan assertion"
        );
    }

    /// Reversible population-side mutation: removing only the callable-result
    /// seed from `Var` must red at the exact plan assertion before lowering.
    #[test]
    fn d8_callable_seed_removal_reds_at_the_plan_boundary() {
        D8_REMOVE_VARIABLE_CALLABLE_SEED.with(|forced| forced.set(true));
        let result = std::panic::catch_unwind(|| {
            assert_d8_bound_callable_join_is_carrier(false);
        });
        D8_REMOVE_VARIABLE_CALLABLE_SEED.with(|forced| forced.set(false));
        assert!(
            result.is_err(),
            "removing the bound callable seed did not red the plan assertion"
        );
    }

    #[test]
    fn d8_trap_predecessors_do_not_create_a_result_edge() {
        let mixed = d8_mixed_join(false);
        let mixed_plan = d8_functionized_plan(&mixed).expect("mixed join plans");
        let mixed_token = mixed_plan
            .join_plan_token(mixed_plan.root_static_origin().expect("mixed root"))
            .expect("mixed join token");
        assert!(mixed_token.has_continuing_predecessor);

        let all_trap = RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Constructor {
                constructor: "ctor:fixture::D8::Left".to_string(),
                args: Vec::new(),
            })),
            cases: ["Left", "Right"]
                .into_iter()
                .map(|name| RuntimeMatchCase {
                    constructor: format!("ctor:fixture::D8::{name}"),
                    binders: 0,
                    body: RuntimeExpr::Trap(trap("D8 terminal arm")),
                })
                .collect(),
            default: trap("D8 all-trap default"),
        };
        let all_trap_plan = d8_functionized_plan(&all_trap).expect("all-trap plans");
        let all_trap_token = all_trap_plan
            .join_plan_token(all_trap_plan.root_static_origin().expect("all-trap root"))
            .expect("all-trap join token");
        assert!(!all_trap_token.has_continuing_predecessor);
    }

    #[test]
    fn d8_join_plan_is_a_bijection_with_source_join_occurrences() {
        let expr = governed_nested_resource_bracket(3);
        let plan = d8_functionized_plan(&expr).expect("bracket plans");
        for (occurrence, join) in plan.source_occurrences.iter().zip(&plan.join_results) {
            assert_eq!(
                occurrence
                    .as_ref()
                    .is_some_and(|occurrence| is_source_join(occurrence.expr)),
                join.is_some(),
                "join-plan population differs from the source-join population"
            );
        }

        let mut missing = plan.clone();
        let index = missing
            .join_results
            .iter()
            .position(Option::is_some)
            .expect("governed bracket has a source join");
        missing.join_results[index] = None;
        assert_eq!(
            missing.validate_join_result_plan().unwrap_err(),
            planner_error("source join occurrence has no result representation")
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





    /// A seat-bearing fixture and its first emission owner.
    ///
    /// Every `D7` row below needs a real `Effect` occurrence, because the
    /// corrected population resolves site-bound children against the seat's own
    /// operand authority. A hand-built key cannot stand in for that.
    pub(super) fn d7_seat_fixture(
        operation: ken_host::HostOpV1,
        args: Vec<RuntimeExpr>,
    ) -> (StaticTransitionPlan<'static>, StaticOriginId, ContinuationEmissionOwner) {
        let expr = Box::leak(Box::new(RuntimeExpr::Effect {
            family: "FS".to_string(),
            operation,
            capability: None,
            args,
        }));
        let plan = plan_static_transition_graph(expr, &BTreeMap::new()).expect("plans");
        let seat = plan
            .source_occurrences
            .iter()
            .flatten()
            .find(|occurrence| matches!(occurrence.expr, RuntimeExpr::Effect { .. }))
            .expect("the fixture has an effect seat")
            .static_origin;
        let owner = *synthesized_seat_emission_owners(&plan, seat)
            .expect("the seat has emission owners")
            .first()
            .expect("a seat is emitted by at least its own predeclared unit");
        (plan, seat, owner)
    }

    /// Three `Int` operands — enough for `FsReadAt`'s buffer at position 2.
    pub(super) fn d7_three_operands() -> Vec<RuntimeExpr> {
        (1..=3)
            .map(|value| RuntimeExpr::Value(RuntimeValue::Int(value.into())))
            .collect()
    }

    /// `D7` — a scalar node's owner set is derived from its EXACT disposition.
    ///
    /// MEASURED: a `spill: None` scalar yields exactly `{NoReferent}`; a
    /// `spill: Some(_)` scalar yields `{NoReferent, PersistentStore}`; and
    /// every scalar in the production tree agrees with the disposition its own
    /// `Lowered` variant declares.
    ///
    /// CLAIMED: the recorded owner set is the child's exact evidence, read off
    /// the spill field of the sole disposition authority — not a family-wide
    /// answer that happens to reach the right lane.
    ///
    /// THE GAP: neither set contains `InvocationArena`, so **this distinction
    /// changes no lane verdict today**. It is asserted because the sets are the
    /// record's stated evidence and a merely-sufficient set is a false one.
    #[test]
    fn a_scalar_nodes_owner_set_comes_from_its_exact_spill_disposition() {
        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());

        let spill_free = SynthesizedAggregateNode::Scalar {
            tag: BoundaryTag::ImmediateBool,
            spill: None,
        };
        assert_eq!(
            node_referent_owners(&plan, seat, spill_free).expect("a spill-free scalar resolves"),
            vec![BoundaryReferentOwner::NoReferent],
            "a `Bool` never becomes a node at any magnitude, so `PersistentStore` \
             is not among its possible owners"
        );

        for spilling in [
            SynthesizedAggregateNode::native_int(),
            SynthesizedAggregateNode::bounded_nat(),
        ] {
            assert_eq!(
                node_referent_owners(&plan, seat, spilling)
                    .expect("a spill-capable scalar resolves"),
                vec![
                    BoundaryReferentOwner::NoReferent,
                    BoundaryReferentOwner::PersistentStore,
                ],
                "a wide {spilling:?} spills to a handle of its spill class, which \
                 is a persistent-store referent"
            );
        }

        // The two sets must actually differ, or the row is vacuous and would
        // pass against the single family-wide answer it exists to reject.
        assert_ne!(
            node_referent_owners(&plan, seat, spill_free).expect("resolves"),
            node_referent_owners(&plan, seat, SynthesizedAggregateNode::native_int())
                .expect("resolves"),
            "spill presence must change the recorded owner set, or recording the \
             disposition bought nothing over the broad immediate family"
        );

        // Every scalar the production tree declares must name a disposition an
        // emitter variant actually produces. This is the anti-drift half: the
        // tree is a second statement of a shape the disposition authority owns.
        for operation in measured_tree_operations() {
            for node in every_tree_node(operation) {
                if let SynthesizedAggregateNode::Scalar { tag, spill } = node {
                    assert!(
                        matches!(
                            (tag, spill),
                            (BoundaryTag::ImmediateBoundedNat, Some(BoundaryClass::Int))
                                | (BoundaryTag::ImmediateInt, Some(BoundaryClass::Int))
                        ),
                        "{operation:?} declares a scalar {tag:?}/{spill:?} that no \
                         emitter variant produces"
                    );
                }
            }
        }
    }

    /// **A site-bound child is RESOLVED against the seat, never pruned.**
    ///
    /// MEASURED: at a real `FsReadFile` seat, `OptionSome` — whose only child
    /// is the seat's path operand — has a record, and so does `FileError`,
    /// which contains it. At `FsReadAt`, `PrivateBufferSpan` and `ReadSome`
    /// have records. The owner set recorded for the site-bound child equals the
    /// one the seat's own operand occurrence yields.
    ///
    /// CLAIMED: site-dependence bounds the *role-invariant* meet, not the meet.
    /// Every one of these is a real allocation production emits, so each gets
    /// an exact site-bound record.
    ///
    /// THE GAP — this is the correction's whole subject, so it is worth saying
    /// plainly what the previous spelling did. It pruned these four
    /// constructors from `P` on the grounds that no role-invariant answer
    /// existed. That is not the fail-closed direction: it left four
    /// allocations that production emits with **no record at all**, which reads
    /// as "refuses at allocation" and was in fact "allocates with the unproven
    /// persistent lane, via the value-shape disposition".
    #[test]
    fn a_site_bound_child_is_resolved_against_the_seat_not_pruned() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        // `OptionSome` and its parent `FileError`, at a file-operation seat.
        let (plan, seat, owner) = d7_seat_fixture(
            ken_host::HostOpV1::FsReadFile,
            vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
        );
        let err = SynthesizedAggregatePath::root(ERR);
        for (path, role) in [
            (err.clone(), R::FileError),
            (err.field(0), R::FileOperationRead),
            (err.field(1), R::OptionSome),
        ] {
            plan.synthesized_aggregate_occurrence(owner, seat, &path, Fixed(role))
                .unwrap_or_else(|error| {
                    panic!("{role:?} at {path:?} must have a record, not be pruned: {error:?}")
                });
        }

        // The site-bound child's recorded owners ARE the seat's operand's.
        let record = plan
            .synthesized_aggregate_record(
                owner,
                seat,
                &err.field(1),
                SynthesizedAggregateRole::Constructor(Fixed(R::OptionSome)),
            )
            .expect("`OptionSome` has a record");
        let authority = occurrence_authority(&plan, seat).expect("the seat has an authority");
        let operand = authority
            .children
            .first()
            .expect("the seat has its path operand");
        assert_eq!(
            record.children[0].owners,
            aggregate_child_referent_owners(&plan, operand).expect("the operand resolves"),
            "the site-bound child's evidence must BE the seat operand's, not a \
             conservative stand-in for it"
        );

        // `PrivateBufferSpan` and `ReadSome`, at a positioned-read seat.
        let (plan, seat, owner) =
            d7_seat_fixture(ken_host::HostOpV1::FsReadAt, d7_three_operands());
        let ok = SynthesizedAggregatePath::root(OK);
        for (path, role) in [
            (ok.alternative(1), R::ReadSome),
            (ok.alternative(1).field(0), R::PrivateBufferSpan),
            (ok.alternative(1).field(1), R::PrivateTransferCount),
            (ok.alternative(0), R::ReadEof),
        ] {
            plan.synthesized_aggregate_occurrence(owner, seat, &path, Fixed(role))
                .unwrap_or_else(|error| {
                    panic!("{role:?} at {path:?} must have a record: {error:?}")
                });
        }
    }

    /// **An `Absent` node is never a child.**
    ///
    /// MEASURED: deriving owners for `Absent` in a child position is a planner
    /// refusal, while the derivable node in the same position resolves.
    ///
    /// CLAIMED: `Absent` marks a host-result arm that builds no aggregate, and
    /// it is a distinct arm from `SiteOperand` precisely so "nothing is built
    /// here" cannot be read as "a child the site supplies".
    ///
    /// THE GAP: this drives the derivation directly; that the production tree
    /// puts `Absent` only at roots is the tree's own shape.
    #[test]
    fn an_absent_node_is_never_a_child() {
        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        assert!(
            node_referent_owners(&plan, seat, SynthesizedAggregateNode::Absent).is_err(),
            "an absent child means the tree claims an allocation whose operand \
             is not built"
        );
        // The positive control: the same position with a real node resolves,
        // so the refusal above is not "children never resolve".
        assert!(
            node_referent_owners(&plan, seat, SynthesizedAggregateNode::native_int()).is_ok(),
            "a scalar in the same position must resolve, or the refusal is vacuous"
        );
    }

    /// **A dynamic child's owners are the UNION of its alternatives'.**
    ///
    /// MEASURED: a two-alternative set whose members are both persistent yields
    /// `{PersistentStore}`. The SAME set with one arena-capable alternative
    /// yields a set containing `InvocationArena`, and a parent holding it is
    /// `InvocationAggregate` rather than `PersistentGround`.
    ///
    /// CLAIMED: the value at a dynamic position is whichever alternative the
    /// discriminator selects, so the parent must survive every one of them —
    /// one invocation-capable alternative makes the parent invocation-owned.
    ///
    /// THE GAP: the arena-capable alternative is reached through a seat operand
    /// that is a closure, because **no alternative in the production tree is
    /// arena-capable today**. So this proves the UNION rule and the lane it
    /// selects, not that any production parent takes the invocation lane
    /// through a dynamic child. Without the escaping half the row would pass
    /// equally for the flat `{PersistentStore}` answer it replaced, which is
    /// why both halves are asserted against the same set shape.
    #[test]
    fn a_dynamic_childs_owners_are_the_union_of_its_alternatives() {
        use SynthesizedAggregateNode as N;
        use SynthesizedFixedConstructorRole as R;

        // Operand 0 is a closure, which `derive_occurrence_lifetime` answers
        // `ActivationOwned` for -- so `SiteOperand(0)` at this seat is the
        // arena-capable leaf the production tree does not have.
        let (plan, seat, _) = d7_seat_fixture(
            ken_host::HostOpV1::FsWriteAt,
            vec![
                RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["x".to_string()],
                    body: Box::new(unit()),
                },
                RuntimeExpr::Value(RuntimeValue::Int(2.into())),
                RuntimeExpr::Value(RuntimeValue::Int(3.into())),
            ],
        );
        assert!(
            node_referent_owners(&plan, seat, N::SiteOperand(0))
                .expect("the closure operand resolves")
                .contains(&BoundaryReferentOwner::InvocationArena),
            "the fixture's operand 0 must be arena-capable, or the escaping half              below is not testing anything"
        );

        // Both alternatives persistent.
        const PERSISTENT: &[SynthesizedAggregateNode] = &[
            N::nullary(R::ReadEof),
            N::Fixed {
                role: R::PrivateTransferCount,
                children: &[N::bounded_nat(), N::bounded_nat()],
            },
        ];
        assert_eq!(
            node_referent_owners(
                &plan,
                seat,
                N::Dynamic(SynthesizedDynamicSet::Alternatives(PERSISTENT))
            )
            .expect("an all-persistent set resolves"),
            vec![BoundaryReferentOwner::PersistentStore],
            "no alternative can be arena-owned, so the union is persistent"
        );

        // The SAME shape with one arena-capable alternative.
        const ESCAPING: &[SynthesizedAggregateNode] = &[
            N::nullary(R::ReadEof),
            N::Fixed {
                role: R::PrivateTransferCount,
                children: &[N::SiteOperand(0)],
            },
        ];
        let union = node_referent_owners(
            &plan,
            seat,
            N::Dynamic(SynthesizedDynamicSet::Alternatives(ESCAPING)),
        )
        .expect("a set with an escaping alternative resolves");
        assert!(
            union.contains(&BoundaryReferentOwner::InvocationArena),
            "one arena-capable alternative must reach the union, got {union:?}"
        );

        // And it must change the PARENT's lane, not merely its owner set.
        const PARENT_PERSISTENT: &[SynthesizedAggregateNode] =
            &[N::Dynamic(SynthesizedDynamicSet::Alternatives(PERSISTENT))];
        const PARENT_ESCAPING: &[SynthesizedAggregateNode] =
            &[N::Dynamic(SynthesizedDynamicSet::Alternatives(ESCAPING))];
        assert_eq!(
            fixed_node_selected_owner(&plan, seat, PARENT_PERSISTENT)
                .expect("the persistent parent resolves"),
            BoundaryReferentOwner::PersistentStore
        );
        assert_eq!(
            fixed_node_selected_owner(&plan, seat, PARENT_ESCAPING)
                .expect("the escaping parent resolves"),
            BoundaryReferentOwner::InvocationArena,
            "a parent whose dynamic child has ONE invocation-capable alternative              must take the invocation lane; answering persistent because the set              is shaped persistently would allocate it over a child that can be              shorter-lived than it"
        );

        // An empty alternative list is a refusal, not a vacuous persistent
        // answer -- an empty owner set satisfies the escape test trivially.
        assert!(
            node_referent_owners(
                &plan,
                seat,
                N::Dynamic(SynthesizedDynamicSet::Alternatives(&[]))
            )
            .is_err(),
            "an alternative-less dynamic child must refuse rather than resolve              persistent by having nothing to check"
        );
    }

    /// **`D7` — the flattening reproduces the MEASURED tree, alternatives
    /// included.**
    ///
    /// MEASURED: instrumenting every `synthesized_constructor` and
    /// `synthesized_dynamic_alternative` call with the kinds of its own
    /// children, single-threaded, produced the edge set restated below. The
    /// corrected flattening interns every constructor-valued node: fixed nodes,
    /// every dynamic alternative at its ordered position, and every
    /// planner-issued `IOError` role.
    ///
    /// CLAIMED: the tree in `host_effect_recipe_tree` is the structure the
    /// emitter actually builds, the paths it flattens to are the positions
    /// those uses occupy, and `P` now covers every allocation.
    ///
    /// THE GAP: this pins the flattening against **the measurement**, not
    /// against the emitter. What keeps the emitter honest is the
    /// per-construction reconciliation. `FsWriteFile` and `FsChangeMode` have
    /// no fixture, so their rows are derived from the emitter's own operation
    /// match rather than observed, and are marked below.
    #[test]
    fn the_flattening_reproduces_the_measured_tree() {
        use ken_host::HostOpV1 as Op;
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, _, _) = d7_seat_fixture(Op::FsWriteAt, d7_three_operands());
        let io_errors = plan.semantic.synthesized_io_error_roles().len();
        assert!(
            io_errors > 1,
            "the closed IOError inventory must be non-trivial, or the alternative \
             rows below are vacuous"
        );

        let path = |root, steps: &[SynthesizedAggregateStep]| SynthesizedAggregatePath {
            root,
            steps: steps.to_vec(),
        };
        let field = SynthesizedAggregateStep::Field;
        let alt = SynthesizedAggregateStep::Alternative;

        // The eleven resource-surface alternatives, in the emitter's order.
        let surface: Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)> = [
            R::ResourceHostIo,
            R::ResourceClosed,
            R::ResourceMalformed,
            R::ResourceRightNotHeld,
            R::ResourceReleaseFailed,
            R::ResourceKindMismatch,
            R::ResourceBufferLimit,
            R::ResourceAllocationFailed,
            R::ResourceInvalidOffset,
            R::ResourceInvalidBounds,
            R::ResourceNoProgress,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, role)| (path(ERR, &[alt(index as u32)]), Fixed(role)))
        .chain(
            // `ResourceKind` at its THREE distinct parent paths, each with its
            // own two alternatives. These are the repeated-role sites.
            [
                (4_u32, 0_u32),
                (5, 0),
                (5, 1),
            ]
            .into_iter()
            .flat_map(|(alternative, position)| {
                [R::ResourceKindFsHandle, R::ResourceKindBuffer]
                    .into_iter()
                    .enumerate()
                    .map(move |(index, role)| {
                        (
                            path(
                                ERR,
                                &[alt(alternative), field(position), alt(index as u32)],
                            ),
                            Fixed(role),
                        )
                    })
            }),
        )
        .chain(std::iter::once((
            path(ERR, &[alt(4), field(1)]),
            Fixed(R::ResourceTraceIdentity),
        )))
        .collect();

        // The two `IOError` sets the resource surface reaches.
        let surface_io: Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)> =
            [(0_u32, 0_u32), (4, 2)]
                .into_iter()
                .flat_map(|(alternative, position)| {
                    plan.semantic
                        .synthesized_io_error_roles()
                        .iter()
                        .enumerate()
                        .map(move |(index, role)| {
                            (
                                path(
                                    ERR,
                                    &[alt(alternative), field(position), alt(index as u32)],
                                ),
                                SynthesizedConstructorRole::IoError(*role),
                            )
                        })
                })
                .collect();

        let file_error = |operation: R| -> Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)> {
            let mut rows = vec![
                (path(ERR, &[]), Fixed(R::FileError)),
                (path(ERR, &[field(0)]), Fixed(operation)),
                (path(ERR, &[field(1)]), Fixed(R::OptionSome)),
            ];
            rows.extend(
                plan.semantic
                    .synthesized_io_error_roles()
                    .iter()
                    .enumerate()
                    .map(|(index, role)| {
                        (
                            path(ERR, &[field(2), alt(index as u32)]),
                            SynthesizedConstructorRole::IoError(*role),
                        )
                    }),
            );
            rows
        };
        let console_error = || -> Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)> {
            plan.semantic
                .synthesized_io_error_roles()
                .iter()
                .enumerate()
                .map(|(index, role)| {
                    (
                        path(ERR, &[alt(index as u32)]),
                        SynthesizedConstructorRole::IoError(*role),
                    )
                })
                .collect()
        };
        let unit = || vec![(path(OK, &[]), Fixed(R::Unit))];

        let expected: Vec<(Op, Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)>)> = vec![
            // Returns a `Bool` above the synthesis entirely.
            (Op::ConsoleIsTerminal, vec![]),
            (
                Op::ConsoleWrite,
                console_error().into_iter().chain(unit()).collect(),
            ),
            (
                Op::ConsoleFlush,
                console_error().into_iter().chain(unit()).collect(),
            ),
            (Op::FsReadFile, file_error(R::FileOperationRead)),
            (Op::FsOpen, file_error(R::FileOperationRead)),
            // ⚠ DERIVED, not observed — no fixture exercises these two. The
            // `Unit` row is the emitter's `else` branch, which the flat use
            // table this tree replaced had MISSED.
            (
                Op::FsWriteFile,
                file_error(R::FileOperationWrite)
                    .into_iter()
                    .chain(unit())
                    .collect(),
            ),
            (
                Op::FsChangeMode,
                file_error(R::FileOperationChangeMode)
                    .into_iter()
                    .chain(unit())
                    .collect(),
            ),
            (
                Op::BufferAllocate,
                surface.iter().cloned().chain(surface_io.clone()).collect(),
            ),
            (
                Op::BufferFreeze,
                surface.iter().cloned().chain(surface_io.clone()).collect(),
            ),
            (
                Op::FsHandleMetadata,
                surface.iter().cloned().chain(surface_io.clone()).collect(),
            ),
            (
                Op::ResourceRelease,
                surface
                    .iter()
                    .cloned()
                    .chain(surface_io.clone())
                    .chain(unit())
                    .collect(),
            ),
            (
                Op::FsReadAt,
                surface
                    .iter()
                    .cloned()
                    .chain(surface_io.clone())
                    .chain([
                        (path(OK, &[alt(0)]), Fixed(R::ReadEof)),
                        (path(OK, &[alt(1)]), Fixed(R::ReadSome)),
                        (path(OK, &[alt(1), field(0)]), Fixed(R::PrivateBufferSpan)),
                        (
                            path(OK, &[alt(1), field(1)]),
                            Fixed(R::PrivateTransferCount),
                        ),
                    ])
                    .collect(),
            ),
            (
                Op::FsWriteAt,
                surface
                    .iter()
                    .cloned()
                    .chain(surface_io.clone())
                    .chain([
                        (path(OK, &[]), Fixed(R::Wrote)),
                        (path(OK, &[field(0)]), Fixed(R::PrivateTransferCount)),
                    ])
                    .collect(),
            ),
        ];

        for (operation, rows) in &expected {
            let flattened = flatten_allocation_reachable_uses(&plan, *operation)
                .into_iter()
                .map(|semantic_use| (semantic_use.path, semantic_use.role))
                .collect::<BTreeSet<_>>();
            let wanted = rows.iter().cloned().collect::<BTreeSet<_>>();
            assert_eq!(
                flattened, wanted,
                "{operation:?}'s allocation-reachable population disagrees with \
                 the measured tree"
            );
        }
    }

    /// **A repeated role at ONE seat gets DISTINCT REAL RECORDS.**
    ///
    /// MEASURED: at one `FsWriteAt` seat, `ResourceKindFsHandle` has three
    /// records — under `ResourceReleaseFailed` field 0 and `ResourceKindMismatch`
    /// fields 0 and 1 — with three distinct occurrence identities. The `IOError`
    /// set likewise has two reachable positions on that tree, each with a full
    /// set of per-role records.
    ///
    /// CLAIMED: `(owner, seat, path, role)` is injective where
    /// `(owner, seat, role)` is not, and the separation now produces **records**
    /// rather than only distinguishable keys.
    ///
    /// THE GAP: the previous spelling of this row asserted only that the six
    /// sites had distinct *paths*, and said so — because under the fixed-only
    /// population none of them had a record. That is exactly what the Architect
    /// rejected. This row now reads the real population.
    #[test]
    fn a_repeated_role_at_one_seat_gets_distinct_real_records() {
        use SynthesizedAggregateRoot::HostResultError as ERR;
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, seat, owner) =
            d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);

        // The three `ResourceKind` parent paths, each interning its own two
        // alternatives.
        let kind_parents = [
            err.alternative(4).field(0),
            err.alternative(5).field(0),
            err.alternative(5).field(1),
        ];
        let mut identities = BTreeSet::new();
        for parent in &kind_parents {
            for (position, role) in
                [R::ResourceKindFsHandle, R::ResourceKindBuffer].iter().enumerate()
            {
                let occurrence = plan
                    .synthesized_aggregate_occurrence(
                        owner,
                        seat,
                        &parent.alternative(position as u32),
                        Fixed(*role),
                    )
                    .unwrap_or_else(|error| {
                        panic!("{role:?} under {parent:?} must have a REAL record: {error:?}")
                    });
                assert!(
                    identities.insert(occurrence),
                    "{role:?} under {parent:?} reused an identity, so one record \
                     is authorizing two allocations"
                );
            }
        }
        assert_eq!(
            identities.len(),
            6,
            "three parent paths x two alternatives is six non-aliasing records"
        );

        // Each reachable `IOError` set produces path-keyed `IoError(role)`
        // records, and the two sets do not share one.
        let roles = plan.semantic.synthesized_io_error_roles().to_vec();
        let mut io_identities = BTreeSet::new();
        for parent in [err.alternative(0).field(0), err.alternative(4).field(2)] {
            for (position, role) in roles.iter().enumerate() {
                let occurrence = plan
                    .synthesized_aggregate_occurrence(
                        owner,
                        seat,
                        &parent.alternative(position as u32),
                        SynthesizedConstructorRole::IoError(*role),
                    )
                    .unwrap_or_else(|error| {
                        panic!("IOError {position} under {parent:?} must have a record: {error:?}")
                    });
                assert!(
                    io_identities.insert(occurrence),
                    "IOError {position} under {parent:?} reused an identity"
                );
            }
        }
        assert_eq!(
            io_identities.len(),
            roles.len() * 2,
            "two reachable IOError sets x the closed inventory is that many records"
        );
    }

    /// **A path is not an index: the STEP KINDS carry information.**
    ///
    /// MEASURED: taking a `Field` step where the tree has a dynamic set, or an
    /// `Alternative` step where it has a fixed constructor, is refused — even
    /// when the position is in range. Collapsing, dropping or swapping a step
    /// resolves to a different node or to nothing. A path continuing past an
    /// `IOError` alternative is refused.
    ///
    /// CLAIMED: `SynthesizedAggregatePath` names at most one node, and the
    /// mutations a hand-written path in `core.rs` could plausibly contain are
    /// each caught rather than silently landing on a neighbour.
    ///
    /// THE GAP: this drives the resolver directly. That the *emitter's* paths
    /// are right is a different claim, held by the construction-time
    /// reconciliation.
    #[test]
    fn a_path_step_kind_is_load_bearing_not_an_index() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);
        let ok = SynthesizedAggregatePath::root(OK);

        // The truth: `ResourceTraceIdentity` at alternative 4, field 1.
        assert_eq!(
            plan.synthesized_tree_node(seat, &err.alternative(4).field(1))
                .expect("the measured path resolves")
                .0,
            Fixed(R::ResourceTraceIdentity)
        );

        // COLLAPSE — the same two positions with the alternative step dropped.
        // Field 4 of a dynamic set is not a step that set can take.
        plan.synthesized_tree_node(seat, &err.field(4).field(1))
            .expect_err("a field step into a dynamic set must refuse");

        // REMOVE — one step short lands on the alternative itself, which is a
        // constructor, so this resolves to a DIFFERENT role rather than
        // failing. That is exactly why the role is compared too.
        assert_eq!(
            plan.synthesized_tree_node(seat, &err.alternative(4))
                .expect("the alternative itself is a node")
                .0,
            Fixed(R::ResourceReleaseFailed),
            "a dropped step must not silently keep the same role"
        );

        // SWAP — the two step kinds exchanged.
        plan.synthesized_tree_node(seat, &err.field(4).alternative(1))
            .expect_err("an alternative step into a constructor must refuse");

        // A sibling FIELD is a different node: field 0 of that alternative is
        // the `ResourceKind` dynamic set, which is not a constructor itself.
        plan.synthesized_tree_node(seat, &err.alternative(4).field(0))
            .expect_err("a dynamic set is not a constructor node");

        // UNPLANNED — a position past the end of the alternative list.
        plan.synthesized_tree_node(
            seat,
            &err.alternative(
                u32::try_from(
                    plan.synthesized_dynamic_alternatives(seat, &err)
                        .expect("the error root is the resource surface")
                        .len(),
                )
                .expect("the inventory fits"),
            ),
        )
        .expect_err("a position past the closed resource-error inventory must refuse");

        // An `IOError` alternative is terminal: nothing below it is a
        // constructor, so a path continuing past one names no node.
        let io = err.alternative(0).field(0);
        plan.synthesized_tree_node(seat, &io.alternative(0))
            .expect("an IOError alternative is a node");
        plan.synthesized_tree_node(seat, &io.alternative(0).field(0))
            .expect_err("a path may not continue past an IOError alternative");
        plan.synthesized_tree_node(
            seat,
            &io.alternative(u32::try_from(plan.semantic.synthesized_io_error_roles().len())
                .expect("the inventory fits")),
        )
        .expect_err("a position past the closed IOError inventory must refuse");

        // The two ROOTS are not interchangeable.
        assert_eq!(
            plan.synthesized_tree_node(seat, &ok)
                .expect("the ok root of `FsWriteAt` is `Wrote`")
                .0,
            Fixed(R::Wrote)
        );
        assert_ne!(
            plan.synthesized_tree_node(seat, &ok).map(|node| node.0).ok(),
            plan.synthesized_tree_node(seat, &err).map(|node| node.0).ok(),
            "the two arms must not resolve to the same node"
        );
    }

    /// **The ABANDONED eager `IOError` template contributes neither `P` nor an
    /// allocation — because it is no longer built.**
    ///
    /// MEASURED: no operation's tree names an `IOError` set at the bare error
    /// root except the console operations, whose whole error value IS that set.
    /// For the six resource-surface operations, `P` contains no record at the
    /// error root, and `lower_process_host_effect` constructs the generic
    /// template only inside the two branches that use it.
    ///
    /// CLAIMED: the template contributes no record and no event, and does so by
    /// **not existing** rather than by being planned and proven unreachable.
    ///
    /// THE GAP: this is a statement about the tree and about `P`. That the
    /// emitter no longer builds it eagerly is a source fact the reconciliation
    /// enforces indirectly — an eagerly-built template would have to be given
    /// some path, and the resource operations' trees have no `IOError` node at
    /// the root to give it.
    #[test]
    fn an_abandoned_eager_template_contributes_neither_records_nor_allocation() {
        use ken_host::HostOpV1 as Op;
        use SynthesizedAggregateNode as N;
        use SynthesizedAggregateRoot::HostResultError as ERR;

        let (plan, _, _) = d7_seat_fixture(Op::FsWriteAt, d7_three_operands());
        for operation in [
            Op::FsHandleMetadata,
            Op::ResourceRelease,
            Op::BufferAllocate,
            Op::BufferFreeze,
            Op::FsReadAt,
            Op::FsWriteAt,
        ] {
            let root = host_effect_recipe_tree(operation).node(ERR);
            assert!(
                !matches!(root, N::Dynamic(SynthesizedDynamicSet::IoErrors)),
                "{operation:?} builds its own surface error, so the eager \
                 template is abandoned and must not be at its error root"
            );
            assert!(
                flatten_allocation_reachable_uses(&plan, operation)
                    .iter()
                    .all(|semantic_use| {
                        semantic_use.path != SynthesizedAggregatePath::root(ERR)
                    }),
                "{operation:?} plans a record at the error root, which is the \
                 abandoned template's position"
            );
        }

        // The positive control: the operations that DO use the generic template
        // have it exactly where the measurement put it. Without this, the rows
        // above would pass for an implementation that never emits an `IOError`
        // set at all.
        assert!(
            matches!(
                host_effect_recipe_tree(Op::ConsoleWrite).node(ERR),
                N::Dynamic(SynthesizedDynamicSet::IoErrors)
            ),
            "`ConsoleWrite`'s whole error value IS the generic template"
        );
        let file_children = match host_effect_recipe_tree(Op::FsReadFile).node(ERR) {
            N::Fixed { children, .. } => children,
            other => panic!("`FsReadFile`'s error root is `FileError`, got {other:?}"),
        };
        assert!(
            matches!(
                file_children[2],
                N::Dynamic(SynthesizedDynamicSet::IoErrors)
            ),
            "`FileError` field 2 IS the generic template"
        );
    }

    /// **A record's lookup key includes its PATH, not only its role.**
    ///
    /// MEASURED: at a real `FsWriteAt` seat, `ResourceKindFsHandle` resolves at
    /// `error.alternative(4).field(0).alternative(0)` and refuses at every
    /// other path — with the owner, seat and role held identical. The
    /// population now genuinely COLLIDES on `(owner, seat, role)`: that role
    /// has three records at one seat.
    ///
    /// CLAIMED: `synthesized_aggregate_record` matches on all four key parts,
    /// so a path-keyed population cannot silently degrade to a role-keyed one.
    ///
    /// THE GAP — worth recording because it changed under the correction. Under
    /// the earlier fixed-only population this row had to be driven by a
    /// hand-built key, because no two records shared `(owner, seat, role)` and
    /// dropping the path from the lookup left the suite green. With every
    /// alternative interned, the collision is real and the production
    /// population itself discriminates.
    #[test]
    fn a_records_lookup_key_includes_its_path_not_only_its_role() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, seat, owner) =
            d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);

        // The real collision: one role, one seat, three records.
        let colliding = [
            err.alternative(4).field(0).alternative(0),
            err.alternative(5).field(0).alternative(0),
            err.alternative(5).field(1).alternative(0),
        ];
        let resolved = colliding
            .iter()
            .map(|path| {
                plan.synthesized_aggregate_occurrence(
                    owner,
                    seat,
                    path,
                    Fixed(R::ResourceKindFsHandle),
                )
                .expect("each of the three uses has its own record")
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(
            resolved.len(),
            3,
            "one role at one seat resolves to three DIFFERENT occurrences, which \
             is only possible if the path is in the key"
        );

        // And paths that name no node refuse rather than falling back.
        for wrong in [
            err.alternative(4).field(0).alternative(7),
            err.clone(),
            SynthesizedAggregatePath::root(OK),
        ] {
            assert!(
                plan.synthesized_aggregate_occurrence(
                    owner,
                    seat,
                    &wrong,
                    Fixed(R::ResourceKindFsHandle)
                )
                .is_err(),
                "{wrong:?} must not resolve; owner, seat and role are identical, \
                 so only the path can refuse it"
            );
        }

        // The role is still part of the key too: the right path with the wrong
        // role refuses. Without this, the rows above would pass for a lookup
        // that matched on the path alone.
        assert!(
            plan.synthesized_aggregate_occurrence(owner, seat, &colliding[0], Fixed(R::Wrote))
                .is_err(),
            "the right path with the wrong role must refuse"
        );
    }

    /// **The PLANNER owns the alternative population, count included.**
    ///
    /// MEASURED: at a real `FsWriteAt` seat the resource surface reports eleven
    /// ordered roles, `ResourceKind` reports its two, and a reachable `IOError`
    /// set reports the whole closed inventory. A path that names a constructor,
    /// an `IOError` alternative, or nothing at all is refused rather than
    /// answering an empty population.
    ///
    /// CLAIMED: an emitter's alternative vector can be compared for EQUALITY
    /// against this, so a truncated set is caught at construction.
    ///
    /// THE GAP — this is the correction's subject, so it is worth stating what
    /// the previous spelling did. It iterated the emitter's own vector and
    /// resolved each position, which rejects an EXTRA alternative (its path
    /// does not exist) but accepts a MISSING final one, because a prefix agrees
    /// with every position it has. The empty vector agreed with everything.
    ///
    /// ⛔ The gap statement that stood here added that a whole-pass
    /// `image(R) = P` closeout *"would eventually surface the unused records"*.
    /// It would not, and there is no such closeout: `P` is an authorization
    /// population, the whole-pass close states `image(R) ⊆ P`, and a planned
    /// record no event related is lawful. A truncated emitter and a lawfully
    /// unused record are the same shape at the ledger, so this boundary has to
    /// establish the equality itself rather than borrow it from a later pass.
    ///
    /// ⚠ The refusal on an empty population is what makes the count usable: a
    /// zero-length expectation would make the emitter's own emptiness agree.
    #[test]
    fn the_planner_owns_the_ordered_alternative_population() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);

        assert_eq!(
            plan.synthesized_dynamic_alternatives(seat, &err)
                .expect("the error root is the resource surface"),
            vec![
                Fixed(R::ResourceHostIo),
                Fixed(R::ResourceClosed),
                Fixed(R::ResourceMalformed),
                Fixed(R::ResourceRightNotHeld),
                Fixed(R::ResourceReleaseFailed),
                Fixed(R::ResourceKindMismatch),
                Fixed(R::ResourceBufferLimit),
                Fixed(R::ResourceAllocationFailed),
                Fixed(R::ResourceInvalidOffset),
                Fixed(R::ResourceInvalidBounds),
                Fixed(R::ResourceNoProgress),
            ],
            "the surface population is ordered and closed, and its COUNT is the \
             planner's rather than whatever the emitter built"
        );

        assert_eq!(
            plan.synthesized_dynamic_alternatives(seat, &err.alternative(4).field(0))
                .expect("`ResourceReleaseFailed` field 0 is the `ResourceKind` set"),
            vec![Fixed(R::ResourceKindFsHandle), Fixed(R::ResourceKindBuffer)],
        );

        let roles = plan.semantic.synthesized_io_error_roles().to_vec();
        assert!(roles.len() > 1, "the closed inventory must be non-trivial");
        assert_eq!(
            plan.synthesized_dynamic_alternatives(seat, &err.alternative(0).field(0))
                .expect("`ResourceHostIo` field 0 is a reachable IOError set"),
            roles
                .iter()
                .map(|role| SynthesizedConstructorRole::IoError(*role))
                .collect::<Vec<_>>(),
            "a reachable IOError set reports its whole closed inventory"
        );

        // ⛔ Every non-set path REFUSES rather than reporting an empty
        // population. An empty expectation would make an emitter's own
        // emptiness agree, which is the failure this row exists to exclude.
        for wrong in [
            err.alternative(4),                          // a constructor
            err.alternative(4).field(1),                 // a constructor
            err.alternative(0).field(0).alternative(0),  // an IOError ALTERNATIVE
            err.alternative(12),                         // no node at all
            SynthesizedAggregatePath::root(OK),          // `Wrote`, a constructor
        ] {
            assert!(
                plan.synthesized_dynamic_alternatives(seat, &wrong).is_err(),
                "{wrong:?} does not name an alternative set and must refuse, not \
                 report an empty population"
            );
        }
    }

    /// **A lawful non-dynamic root and a FAILED lookup are different answers.**
    ///
    /// MEASURED: at a real `FsWriteAt` seat the error root reports a population
    /// and the `ok` root — `Wrote`, a constructor — reports `Ok(None)`. At an
    /// `FsReadFile` seat the `ok` root is `Absent` and also reports `Ok(None)`.
    /// A seat that is not an `Effect` occurrence, and a path that leaves the
    /// tree or names an `IOError` position outside the closed inventory, each
    /// report `Err`.
    ///
    /// CLAIMED: absence is typed apart from failure, so a caller cannot act on
    /// "the planner plans no set here" when what actually happened is that the
    /// question could not be answered.
    ///
    /// THE GAP — and it is the reason this row exists. The root reconciliation
    /// wrote `.ok()` on this query, which merged every failure into absence: a
    /// non-dynamic emitted root then matched the absent case and was accepted,
    /// a missing-authority default inside a function whose stated contract is
    /// that neither direction may be defaulted. **No shape or truncation
    /// mutation can find that**, because both keep the lookup working — which
    /// is why the five root mutations were all green against it. The
    /// distinguishing evidence is exactly the two halves below, and the
    /// positive half matters as much as the negative: without it, the
    /// correction would be satisfied by making every non-dynamic root fail.
    #[test]
    fn a_lawful_non_dynamic_root_is_not_a_failed_lookup() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};

        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);
        let ok = SynthesizedAggregatePath::root(OK);

        // Dynamic root: a population.
        assert_eq!(
            plan.synthesized_root_alternative_population(seat, &err)
                .expect("the error root resolves")
                .expect("the error root is the resource surface")
                .len(),
            11
        );

        // ⭐ LAWFULLY non-dynamic: `Wrote` is a constructor, so the answer is a
        // resolved absence rather than a failure.
        assert_eq!(
            plan.synthesized_root_alternative_population(seat, &ok)
                .expect("the ok root resolves lawfully"),
            None,
            "a constructor root is a resolved non-set, not a failed lookup"
        );

        // The same for an arm that builds no aggregate at all.
        let (absent_plan, absent_seat, _) = d7_seat_fixture(
            ken_host::HostOpV1::FsReadFile,
            vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
        );
        assert_eq!(
            absent_plan
                .synthesized_root_alternative_population(
                    absent_seat,
                    &SynthesizedAggregatePath::root(OK)
                )
                .expect("an absent ok arm resolves lawfully"),
            None,
            "`FsReadFile`'s `ok` builds no aggregate, which is still a RESOLVED \
             answer"
        );

        // ⛔ And every way the question can fail to be answerable stays `Err`.
        let non_effect = plan
            .source_occurrences
            .iter()
            .flatten()
            .find(|occurrence| !matches!(occurrence.expr, RuntimeExpr::Effect { .. }))
            .expect("the fixture has a non-effect occurrence")
            .static_origin;
        assert!(
            plan.synthesized_root_alternative_population(non_effect, &err)
                .is_err(),
            "a seat that is not a host effect cannot answer, and must not read \
             as a root with no planned set"
        );
        assert!(
            plan.synthesized_root_alternative_population(
                StaticOriginId(u32::MAX),
                &err
            )
            .is_err(),
            "an origin outside the occurrence population must fail, not resolve \
             to an absence"
        );
        assert!(
            plan.synthesized_root_alternative_population(seat, &err.alternative(12))
                .is_err(),
            "a path that leaves the tree must fail, not resolve to an absence"
        );
        let inventory = plan.semantic.synthesized_io_error_roles().len();
        assert!(
            plan.synthesized_root_alternative_population(
                seat,
                &err.alternative(0).field(0).alternative(
                    u32::try_from(inventory).expect("the inventory fits")
                )
            )
            .is_err(),
            "an IOError position outside the closed inventory must fail, not \
             resolve to an absence"
        );
    }

    /// Every operation whose tree this module states.
    pub(super) fn measured_tree_operations() -> Vec<ken_host::HostOpV1> {
        use ken_host::HostOpV1 as Op;
        vec![
            Op::ConsoleIsTerminal,
            Op::ConsoleWrite,
            Op::ConsoleFlush,
            Op::FsReadFile,
            Op::FsOpen,
            Op::FsWriteFile,
            Op::FsChangeMode,
            Op::BufferAllocate,
            Op::BufferFreeze,
            Op::FsHandleMetadata,
            Op::ResourceRelease,
            Op::FsReadAt,
            Op::FsWriteAt,
        ]
    }

    /// Every node in both of an operation's trees, in no particular order.
    pub(super) fn every_tree_node(operation: ken_host::HostOpV1) -> Vec<SynthesizedAggregateNode> {
        let tree = host_effect_recipe_tree(operation);
        let mut nodes = Vec::new();
        for root in [
            SynthesizedAggregateRoot::HostResultError,
            SynthesizedAggregateRoot::HostResultOk,
        ] {
            walk_tree_with_paths(
                tree.node(root),
                &SynthesizedAggregatePath::root(root),
                &mut |node, _| nodes.push(node),
            );
        }
        nodes
    }

    /// Visit every node of a tree with the path that reaches it.
    pub(super) fn walk_tree_with_paths(
        node: SynthesizedAggregateNode,
        path: &SynthesizedAggregatePath,
        visit: &mut dyn FnMut(SynthesizedAggregateNode, &SynthesizedAggregatePath),
    ) {
        visit(node, path);
        match node {
            SynthesizedAggregateNode::Fixed { children, .. } => {
                for (position, child) in children.iter().enumerate() {
                    walk_tree_with_paths(*child, &path.field(position as u32), visit);
                }
            }
            SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(
                alternatives,
            )) => {
                for (position, alternative) in alternatives.iter().enumerate() {
                    walk_tree_with_paths(
                        *alternative,
                        &path.alternative(position as u32),
                        visit,
                    );
                }
            }
            SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors)
            | SynthesizedAggregateNode::Scalar { .. }
            | SynthesizedAggregateNode::SiteOperand(_)
            | SynthesizedAggregateNode::Absent => {}
        }
    }

    #[test]
    fn substrate_same_shape_aggregates_keep_distinct_lifetimes() {
        let wrapper = |child| RuntimeExpr::Construct {
            constructor: "ctor:fixture::Substrate::Wrapper".to_string(),
            args: vec![child],
        };
        let expr = RuntimeExpr::Record {
            fields: vec![
                ("persistent".to_string(), wrapper(unit())),
                (
                    "activation".to_string(),
                    wrapper(RuntimeExpr::LexicalClosure {
                        captures: Vec::new(),
                        params: vec!["x".to_string()],
                        body: Box::new(unit()),
                    }),
                ),
            ],
        };
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plans");
        let wrappers = plan
            .occurrence_authorities
            .iter()
            .filter(|record| {
                matches!(
                    plan.planned_occurrence_expr(record.origin),
                    Ok(RuntimeExpr::Construct { constructor, .. })
                        if constructor == "ctor:fixture::Substrate::Wrapper"
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(wrappers.len(), 2);
        assert_ne!(wrappers[0].origin, wrappers[1].origin);
        assert_eq!(
            plan.constructor_symbol_identity(wrappers[0].origin)
                .unwrap(),
            plan.constructor_symbol_identity(wrappers[1].origin)
                .unwrap()
        );
        assert_eq!(
            wrappers
                .iter()
                .map(|record| record.lifetime)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([
                PlannedReferentLifetime::Persistent,
                PlannedReferentLifetime::ActivationOwned,
            ])
        );
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
