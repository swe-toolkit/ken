//! Aggregate allocation events, ownership records, and the planner-side
//! aggregate lifecycle -- the synthesized-tree recipe, the lifetime-meet
//! derivation, and the closed ownership population.
//!
//! `RT-PLANNER-AGGREGATES-SPLIT` `D1` -- this module owns the aggregates
//! domain moved from the parent (`AggregateOccurrenceId`,
//! `AggregateOccurrenceProducer`, the shape/role/path/step/node vocabulary,
//! and `PlannedAggregateOwnership` + its read-only view). `StaticTransitionPlan`
//! stays in the parent; the impl block here reads ancestor-private root state
//! under the standing child-module pattern (item 4's `units.rs` precedent).
//!
//! The lowering-owned half (`AggregateAllocationEvent`,
//! `AggregateAllocationLedger`, `AggregateRelationClosure`) is a DIFFERENT
//! thing entirely and stays in `lowering/mod.rs` for item 15 -- see the D0
//! ledger's boundary proposal in
//! `docs/program/issues/RT-PLANNER-AGGREGATES-SPLIT.md`.

use std::collections::{BTreeMap, BTreeSet};
#[cfg(feature = "px8-ds-test-support")]
use std::cell::{Cell, RefCell};

use super::continuations::{
    build_checked_binder_provenance, CheckedBinderProvenance, CheckedBinderResolution,
    CheckedCaseBinderLayout, CheckedCaseBinderRole, CheckedIhBinding,
    ContinuationOrdinaryEnvelopeRole, ContinuationWorkerCaptureSource,
};
use super::{
    inline_synthesized_seat_emission_owners, occurrence_authority,
    planner_capacity_error, planner_error, AbiCaptureProvenance, AbiUnitDefinition,
    BoundaryReferentOwner, ContinuationCallIdentity, ContinuationEmissionOwner,
    ContinuationEnvironmentClaim, ContinuationFrameIdentity, ContinuationSourceCoordinate,
    ContinuationSpecializationId, CraneliftBackendError, EmittableCallKind, FieldIdentity,
    JoinResultRepresentation, PlannedOccurrenceChildAuthority, PlannedReferentLifetime,
    PredeclaredFunctionId, StaticOriginId, StaticTransitionPlan, SynthesizedConstructorRole,
    SynthesizedFixedConstructorRole,
};
use super::closure::{derive_case_producer_fact, CaseProducerSet};
use crate::boundary_value::{BoundaryClass, BoundaryTag};
use crate::RuntimeExpr;

/// **`RT-DECL-CLOSURE-PORT` `D7` — the opaque identity of one aggregate
/// emission occurrence.**
///
/// Issued by the planner and only by the planner. The field is private to this
/// module, so lowering **cannot construct one** — it can only receive an
/// identity from an accessor that already interned it. That is the mechanical
/// form of "lowering does not construct identities"; a doc comment saying so
/// would be advisory, and this is not.
///
/// ## Why an identity rather than the emission origin
///
/// A `Lowered::Constructor` template outlives the occurrence that produced it.
/// Lowering builds the template at the `Construct` occurrence and may transfer
/// it into the carrier much later, at a `Let`, `Match`, `Call` or `Effect`
/// origin reached through nested producer traversal. The identity the emitter
/// needs is the **producer's**, and by then the emission origin is a different
/// occurrence entirely.
///
/// That is not a hypothesis: the sibling `synthesized_identity` field on the
/// template exists for exactly this reason and says so in its own comment —
/// *"the caller occurrence is not the constructor occurrence and therefore
/// cannot lawfully re-query its atom."* The allocation lane is the second fact
/// with that property, and it travels the same way.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(in crate::cranelift_backend) struct AggregateOccurrenceId(
    pub(in crate::cranelift_backend::planning::static_transition) u32,
);
/// Which producer an aggregate occurrence record is about.
///
/// The two arms are the two ways an aggregate comes to exist, and they are
/// named by different authorities on purpose. A source aggregate is named by
/// its own occurrence in the program. A synthesized one has no occurrence to be
/// named by, so it is named by the closed compiler role that builds it — never
/// by the origin it happens to be emitted at, which belongs to whatever
/// expression the emission was reached through.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum AggregateOccurrenceProducer {
    /// A `Construct`/`Record` written in the program.
    Source(StaticOriginId),
    /// One exact compiler-synthesized producer USE.
    ///
    /// A role is a schema, not an identity, and neither is a role at a seat.
    /// Two uses of `PrivateTransferCount` at two seats are two occurrences even
    /// though their schema is one; two uses of `ResourceKind` at ONE seat --
    /// `ResourceKindMismatch` fields 0 and 1 -- are likewise two occurrences,
    /// and no seat-and-role key can separate them. The path is what does.
    SynthesizedUse {
        /// The exact `D5a` emission owner, `Predeclared` or `Specialization`.
        ///
        /// It is in the KEY, not merely on the record. One seat's body may be
        /// lowered under a predeclared unit and again inside a generated
        /// specialization context; those are different emissions and their
        /// records must not alias. Deriving this from the seat's provenance
        /// owner would collapse exactly the distinction `D5a` exists to keep.
        owner: ContinuationEmissionOwner,
        /// The source occurrence that anchors this synthesized use. Host-result
        /// trees use their `Effect`; a unit-boundary environment uses the exact
        /// source constructor that owns the closure-valued field.
        seat: StaticOriginId,
        /// Where in the seat's synthesized tree this use sits.
        ///
        /// ⛔ Not an ordinal. An ordinal would count emissions in lowering's
        /// control flow, which the planner does not execute; a path is measured
        /// structure that both sides state independently and can be checked
        /// against each other at construction.
        path: SynthesizedAggregatePath,
        /// The closed compiler role that builds this use.
        role: SynthesizedAggregateRole,
    },
}
/// The closed compiler role that builds one synthesized aggregate.
///
/// Constructor roles retain the semantic plane's existing constructor
/// identity. The environment arm names the record introduced when a
/// closure-valued source-constructor field is carried as a generated-unit call
/// input; it has no constructor identity because its shape is
/// [`PlannedAggregateShape::Record`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum SynthesizedAggregateRole {
    /// The full semantic constructor-role sum, fixed and `IOError`. Every
    /// `IOError` alternative is a real allocation with its own path, so this
    /// cannot be narrowed to the fixed half.
    Constructor(SynthesizedConstructorRole),
    /// The captured-environment Record introduced at a generated-unit input.
    UnitBoundaryEnvironment,
    /// **`RT-CHECKED-IH-CAPTURED-ENV-SCHEMA` tier 2 -- the checked-IH's
    /// CAPTURED environment.**
    ///
    /// A SIBLING of [`Self::UnitBoundaryEnvironment`], not a widening of it.
    /// That role is the capture-FREE unit-boundary case, and its sole
    /// production consumer gates on `captures.is_empty()` before emitting an
    /// empty-fields Record -- so a captured environment pushed through the same
    /// role would meet a consumer that both tests emptiness and builds no
    /// fields. The two populations share the owner/escape DERIVATION and
    /// nothing else.
    CheckedIhCapturedEnvironment,
    /// The positional captured environment of a statically known lexical
    /// closure crossing an exact generated-unit result or bind-continuation
    /// edge. The runtime aggregate carries captures only; code identity remains
    /// in the compiler-issued descriptor and never becomes a carrier tag.
    BoundaryClosureEnvironment,
}

/// Compile-time identity and positional environment schema for one lexical
/// closure that may cross a generated-unit boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct BoundaryClosureEnvironment {
    owner: ContinuationEmissionOwner,
    seat: StaticOriginId,
    body_origin: StaticOriginId,
    params: Vec<String>,
    capture_origins: Vec<StaticOriginId>,
    record: AggregateOccurrenceId,
}

impl BoundaryClosureEnvironment {
    pub(in crate::cranelift_backend) fn owner(&self) -> ContinuationEmissionOwner {
        self.owner
    }

    pub(in crate::cranelift_backend) fn seat(&self) -> StaticOriginId {
        self.seat
    }

    pub(in crate::cranelift_backend) fn body_origin(&self) -> StaticOriginId {
        self.body_origin
    }

    pub(in crate::cranelift_backend) fn params(&self) -> &[String] {
        &self.params
    }

    pub(in crate::cranelift_backend) fn capture_origins(&self) -> &[StaticOriginId] {
        &self.capture_origins
    }

    pub(in crate::cranelift_backend) fn record(&self) -> AggregateOccurrenceId {
        self.record
    }
}

/// One authorized transport of a force-materialized checked-IH environment.
///
/// This is not another aggregate emission record. `source_owner + seat` names
/// the sole materialization; `destination_owner + destination_construct_origin
/// + recursive_position` names the exact crossing where the continuation call
/// result substitutes for the raw closure. Both endpoints are planner facts,
/// so lowering neither borrows by owner nor searches by seat.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct CheckedIhEnvironmentTransport {
    source_owner: ContinuationEmissionOwner,
    source_specialization: ContinuationSpecializationId,
    source_call_identity: ContinuationCallIdentity,
    seat: StaticOriginId,
    source_result_origin: StaticOriginId,
    source_worker_body_origin: StaticOriginId,
    source_continuation_origin: StaticOriginId,
    source_recursive_position: u32,
    destination_owner: ContinuationEmissionOwner,
    destination_body_origin: StaticOriginId,
    destination_construct_origin: StaticOriginId,
    recursive_position: u32,
    source_record: AggregateOccurrenceId,
    source_lifetime: PlannedReferentLifetime,
    destination_lifetime: PlannedReferentLifetime,
    continuation_input_morphism: Vec<(
        u32,
        ContinuationSourceCoordinate,
        CheckedIhTransportInputDestination,
    )>,
}

/// Which destination environment one transported continuation input indexes.
/// The domain tag is part of the morphism; the same integer in these two
/// frames is not the same coordinate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum CheckedIhTransportInputDestination {
    LexicalEnvironment(u32),
    EntryFrame(u32),
}

/// Which runtime environment owns an immediate checked-IH K binding.
///
/// Only `ImmediateInvocationEnvironment` is derivable in production. The other
/// closed arms exist under test support so compile-preserving mutations can
/// prove that the domain tag, source-slot substitution, and final-residual
/// substitution are independently rejected rather than accepted by integer
/// coincidence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum CheckedIhKAvailabilityDomain {
    ImmediateInvocationEnvironment,
    #[cfg(feature = "px8-ds-test-support")]
    ForeignRuntimeEnvironment,
    #[cfg(feature = "px8-ds-test-support")]
    SourceRecursiveSlot,
    #[cfg(feature = "px8-ds-test-support")]
    FinalRecursorResidual,
}

/// The exact immediate runtime coordinate at which one governed descendant
/// invocation reads inherited continuation capability K.
///
/// Consumer and environment identity travel with the number. This is neither a
/// semantic [`CheckedIhBinding`] nor any existing source/transport locator.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct CheckedIhImmediateKBindingLocator {
    invocation_origin: StaticOriginId,
    callee_origin: StaticOriginId,
    environment_domain: CheckedIhKAvailabilityDomain,
    environment_index: u32,
}

#[allow(dead_code)]
impl CheckedIhImmediateKBindingLocator {
    pub(in crate::cranelift_backend) fn invocation_origin(&self) -> StaticOriginId {
        self.invocation_origin
    }

    pub(in crate::cranelift_backend) fn callee_origin(&self) -> StaticOriginId {
        self.callee_origin
    }

    pub(in crate::cranelift_backend) fn environment_domain(
        &self,
    ) -> CheckedIhKAvailabilityDomain {
        self.environment_domain
    }

    pub(in crate::cranelift_backend) fn environment_index(&self) -> u32 {
        self.environment_index
    }
}

/// The forward proof that one already-issued captured continuation capability
/// remains in scope at one recursively exposed checked invocation.
///
/// Every field is an existing occurrence, frame, binder, or call fact. The
/// record carries no dynamic result identity: in particular, the transport's
/// earlier source result is not an input to this proof.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedIhSelfResumptionStep {
    construct_origin: StaticOriginId,
    active_frame_origin: StaticOriginId,
    recursive_child_origin: StaticOriginId,
    selected_case_body_origin: StaticOriginId,
    invocation_origin: StaticOriginId,
    call_origin: StaticOriginId,
    callee_origin: StaticOriginId,
    callee_binding: CheckedIhBinding,
    /// The closed locator population for this step. Production derives one;
    /// retaining the population lets validation reject absence or ambiguity
    /// rather than making either state unrepresentable to mutation controls.
    immediate_k_locators: Vec<CheckedIhImmediateKBindingLocator>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedIhCapabilityInheritance {
    destination_owner: ContinuationEmissionOwner,
    destination_body_origin: StaticOriginId,
    self_resumption_steps: Vec<CheckedIhSelfResumptionStep>,
}

/// The ordinary Ret/capture destination of the fresh result conditionally
/// produced by applying an inherited continuation capability.
///
/// This is intentionally a sibling of [`CheckedIhCapabilityInheritance`], not
/// its tail. It names no transport source result and states no value equality.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedIhFreshResultDestination {
    active_frame_origin: StaticOriginId,
    ret_case_body_origin: StaticOriginId,
    constructor_child: CheckedBinderProvenance,
    closure_environment_record: AggregateOccurrenceId,
    closure_origin: StaticOriginId,
    closure_body_origin: StaticOriginId,
    closure_parameter_count: u32,
    capture_ordinal: u32,
    capture_occurrence: StaticOriginId,
    body_capture_reads: Vec<StaticOriginId>,
}

/// One planner-only continuation-inheritance projection of an existing
/// checked-IH environment transport.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedIhContinuationInheritance {
    transport: CheckedIhEnvironmentTransport,
    capability: CheckedIhCapabilityInheritance,
    fresh_result_destination: CheckedIhFreshResultDestination,
}

/// Read-only split view returned by the exact continuation-inheritance
/// accessor. The two proofs cannot be mistaken for one transitive result edge.
pub(in crate::cranelift_backend) struct CheckedIhContinuationInheritanceView<'plan> {
    transport: &'plan CheckedIhEnvironmentTransport,
    capability: &'plan CheckedIhCapabilityInheritance,
    fresh_result_destination: &'plan CheckedIhFreshResultDestination,
}

impl CheckedIhContinuationInheritanceView<'_> {
    pub(in crate::cranelift_backend) fn transport(&self) -> &CheckedIhEnvironmentTransport {
        self.transport
    }

    pub(in crate::cranelift_backend) fn capability(&self) -> &CheckedIhCapabilityInheritance {
        self.capability
    }

    pub(in crate::cranelift_backend) fn fresh_result_destination(
        &self,
    ) -> &CheckedIhFreshResultDestination {
        self.fresh_result_destination
    }
}

impl CheckedIhCapabilityInheritance {
    pub(in crate::cranelift_backend) fn destination_owner(&self) -> ContinuationEmissionOwner {
        self.destination_owner
    }

    pub(in crate::cranelift_backend) fn destination_body_origin(&self) -> StaticOriginId {
        self.destination_body_origin
    }

    pub(in crate::cranelift_backend) fn active_frame_origin(&self) -> StaticOriginId {
        self.self_resumption_steps
            .last()
            .expect("a validated inheritance has at least one self-resumption step")
            .callee_binding
            .frame_origin
    }

    pub(in crate::cranelift_backend) fn recursive_position(&self) -> u32 {
        self.self_resumption_steps
            .last()
            .expect("a validated inheritance has at least one self-resumption step")
            .callee_binding
            .recursive_position
    }

    pub(in crate::cranelift_backend) fn immediate_k_locator(
        &self,
    ) -> Option<&CheckedIhImmediateKBindingLocator> {
        let final_step = self.self_resumption_steps.last()?;
        let [locator] = final_step.immediate_k_locators.as_slice() else {
            return None;
        };
        Some(locator)
    }
}

impl CheckedIhFreshResultDestination {
    pub(in crate::cranelift_backend) fn closure_environment_record(&self) -> AggregateOccurrenceId {
        self.closure_environment_record
    }
}

impl CheckedIhEnvironmentTransport {
    pub(in crate::cranelift_backend) fn source_owner(&self) -> ContinuationEmissionOwner {
        self.source_owner
    }

    pub(in crate::cranelift_backend) fn source_specialization(
        &self,
    ) -> ContinuationSpecializationId {
        self.source_specialization
    }

    pub(in crate::cranelift_backend) fn source_call_identity(&self) -> &ContinuationCallIdentity {
        &self.source_call_identity
    }

    pub(in crate::cranelift_backend) fn seat(&self) -> StaticOriginId {
        self.seat
    }

    pub(in crate::cranelift_backend) fn source_record(&self) -> AggregateOccurrenceId {
        self.source_record
    }

    pub(in crate::cranelift_backend) fn destination_owner(&self) -> ContinuationEmissionOwner {
        self.destination_owner
    }

    pub(in crate::cranelift_backend) fn destination_construct_origin(&self) -> StaticOriginId {
        self.destination_construct_origin
    }

    pub(in crate::cranelift_backend) fn recursive_position(&self) -> u32 {
        self.recursive_position
    }

    pub(in crate::cranelift_backend) fn continuation_input_index(
        &self,
        ordinal: u32,
        coordinate: ContinuationSourceCoordinate,
    ) -> Option<CheckedIhTransportInputDestination> {
        self.continuation_input_morphism.iter().find_map(
            |(held_ordinal, held_coordinate, destination)| {
                (*held_ordinal == ordinal && *held_coordinate == coordinate).then_some(*destination)
            },
        )
    }

    pub(in crate::cranelift_backend) fn continuation_input_count(&self) -> usize {
        self.continuation_input_morphism.len()
    }
}
/// Which aggregate shape one producer occurrence builds.
///
/// ⛔ Deliberately its own two-member enum rather than a reuse of
/// [`crate::boundary_value::BoundaryClass`]. That type is the *node* class and
/// admits five ground shapes; the population here is exactly the shapes that
/// **have children to take a lifetime meet over**, and spelling it as its own
/// type is what makes a `Bytes` occurrence a type error here instead of a
/// record nothing ever consumes.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum PlannedAggregateShape {
    Constructor,
    Record,
}
/// A read-only view of one planned aggregate ownership record.
///
/// ⭐ Every accessor answers from the ONE record the occurrence names. Nothing
/// here searches, and nothing takes a coordinate — a consumer that could pass a
/// coordinate could pass the wrong one, which is the defect this projection
/// exists to make unspellable.
pub(in crate::cranelift_backend) struct PlannedAggregateView<'plan> {
    record: &'plan PlannedAggregateOwnership,
}
impl<'plan> PlannedAggregateView<'plan> {
    pub(in crate::cranelift_backend) fn id(&self) -> AggregateOccurrenceId {
        self.record.id
    }

    pub(in crate::cranelift_backend) fn producer(&self) -> &'plan AggregateOccurrenceProducer {
        &self.record.producer
    }

    /// The producer's own source occurrence, for a source aggregate.
    ///
    /// `None` for a compiler-synthesized use, which has no occurrence in the
    /// program — an absence, not a coordinate to fall back on.
    pub(in crate::cranelift_backend) fn producer_origin(&self) -> Option<StaticOriginId> {
        match &self.record.producer {
            AggregateOccurrenceProducer::Source(origin) => Some(*origin),
            AggregateOccurrenceProducer::SynthesizedUse { .. } => None,
        }
    }

    pub(in crate::cranelift_backend) fn owner(&self) -> Option<PredeclaredFunctionId> {
        self.record.owner
    }

    pub(in crate::cranelift_backend) fn shape(&self) -> PlannedAggregateShape {
        self.record.shape
    }

    /// The ruled allocation lane. ⛔ Read here, never re-derived from the value.
    pub(in crate::cranelift_backend) fn allocation(&self) -> PlannedAggregateAllocation {
        self.record.allocation
    }

    pub(in crate::cranelift_backend) fn meet(&self) -> PlannedReferentLifetime {
        self.record.meet
    }

    /// The ordered children, with each one's position, source occurrence,
    /// lifetime and possible referent owners.
    pub(in crate::cranelift_backend) fn children(&self) -> &'plan [PlannedAggregateChild] {
        &self.record.children
    }

    /// The closed positional child recipe for a compiler-synthesized
    /// aggregate. Source aggregates have no such recipe.
    pub(in crate::cranelift_backend) fn declared_children(
        &self,
    ) -> Option<&'static [SynthesizedAggregateNode]> {
        self.record.declared_children
    }
}
/// Test-only, closure-free observation of one issued continuation-inheritance
/// projection. Numeric origins are reports only; production selection never
/// consumes this type.
#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedIhContinuationInheritanceObservation {
    pub source_specialization: u32,
    pub source_call_identity: String,
    pub source_result_origin: u32,
    pub destination_specialization: u32,
    pub destination_body_origin: u32,
    pub active_frame_lineage: Vec<u32>,
    pub destination_construct_origin: u32,
    pub recursive_child_origin: u32,
    pub selected_case_body_origin: u32,
    pub active_frame_origin: u32,
    pub recursive_position: u32,
    pub invocation_origin: u32,
    pub call_origin: u32,
    pub callee_origin: u32,
    pub immediate_k_locator_count: usize,
    pub immediate_k_locator_invocation_origin: u32,
    pub immediate_k_locator_callee_origin: u32,
    pub immediate_k_locator_domain: String,
    pub immediate_k_environment_index: u32,
    pub immediate_k_preceding_environment_provenance: Option<String>,
    pub immediate_k_lineage_environment_indices: Vec<u32>,
    pub ret_case_body_origin: u32,
    pub closure_origin: u32,
    pub capture_ordinal: u32,
    pub capture_occurrence: u32,
    pub closure_body_origin: u32,
    pub body_capture_reads: Vec<u32>,
    pub closure_parameter_count: usize,
    pub fresh_destination_mentions_source_result: bool,
    pub ordinary_non_governed_exclusion_count: usize,
    pub descriptor_only_exclusion_count: usize,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static CONTINUATION_INHERITANCE_OBSERVATION_ACTIVE: Cell<bool> = const { Cell::new(false) };
    static CONTINUATION_INHERITANCE_OBSERVATIONS:
        RefCell<Vec<CheckedIhContinuationInheritanceObservation>> = const { RefCell::new(Vec::new()) };
    static CONTINUATION_INHERITANCE_DESCRIPTOR_ONLY_EXCLUSIONS: Cell<usize> = const { Cell::new(0) };
}

/// Run one compile observation window. The predecessor remains behaviorally
/// inert; this only exposes planner records to governed cross-crate controls.
#[cfg(feature = "px8-ds-test-support")]
pub fn with_checked_ih_continuation_inheritance_observations<T>(
    f: impl FnOnce() -> T,
) -> (T, Vec<CheckedIhContinuationInheritanceObservation>) {
    CONTINUATION_INHERITANCE_OBSERVATIONS.with(|observations| observations.borrow_mut().clear());
    CONTINUATION_INHERITANCE_DESCRIPTOR_ONLY_EXCLUSIONS.with(|count| count.set(0));
    CONTINUATION_INHERITANCE_OBSERVATION_ACTIVE.with(|active| active.set(true));
    let result = f();
    CONTINUATION_INHERITANCE_OBSERVATION_ACTIVE.with(|active| active.set(false));
    let observations = CONTINUATION_INHERITANCE_OBSERVATIONS
        .with(|observations| std::mem::take(&mut *observations.borrow_mut()));
    (result, observations)
}

#[cfg(feature = "px8-ds-test-support")]
pub(super) fn record_checked_ih_continuation_inheritances(
    plan: &StaticTransitionPlan<'_>,
    inheritances: &[CheckedIhContinuationInheritance],
) {
    if !CONTINUATION_INHERITANCE_OBSERVATION_ACTIVE.with(Cell::get) {
        return;
    }
    let mut ordinary_non_governed = BTreeSet::new();
    let calls = plan
        .continuation_calls()
        .expect("validated plan exposes continuation calls");
    let binder_resolutions = build_checked_binder_provenance(plan)
        .expect("validated plan exposes its forward binder resolutions");
    for inheritance in inheritances {
        let final_step = inheritance
            .capability
            .self_resumption_steps
            .last()
            .expect("validated continuation inheritance has a final step");
        for call in &calls {
            if call.emission_owner() != inheritance.capability.destination_owner
                || (call.continuation_origin() == final_step.callee_binding.frame_origin
                    && call.recursive_position() == final_step.callee_binding.recursive_position)
            {
                continue;
            }
            let excluded = plan
                .checked_ih_continuation_inheritance_for_invocation(
                    &inheritance.transport.source_call_identity,
                    inheritance.capability.destination_owner,
                    None,
                    call.continuation_origin(),
                    call.recursive_position(),
                )
                .expect("validated exclusion accessor query");
            if excluded.is_none() {
                ordinary_non_governed.insert((
                    format!("{:?}", inheritance.transport.source_call_identity),
                    call.continuation_origin(),
                    call.recursive_position(),
                ));
            }
        }
    }
    let ordinary_non_governed_exclusion_count = ordinary_non_governed.len();
    let descriptor_only_exclusion_count =
        CONTINUATION_INHERITANCE_DESCRIPTOR_ONLY_EXCLUSIONS.with(Cell::get);
    CONTINUATION_INHERITANCE_OBSERVATIONS.with(|observations| {
        let mut observations = observations.borrow_mut();
        for inheritance in inheritances {
            let destination_specialization = match inheritance.capability.destination_owner {
                ContinuationEmissionOwner::Specialization(id) => id.0,
                ContinuationEmissionOwner::Predeclared(_)
                | ContinuationEmissionOwner::Fusion(_) => continue,
            };
            let source_result_origin = inheritance.transport.source_result_origin.0;
            let destination = &inheritance.fresh_result_destination;
            let final_step = inheritance
                .capability
                .self_resumption_steps
                .last()
                .expect("validated continuation inheritance has a final step");
            let final_locator = inheritance
                .capability
                .immediate_k_locator()
                .expect("validated continuation inheritance has exactly one final K locator");
            let immediate_k_lineage_environment_indices = inheritance
                .capability
                .self_resumption_steps
                .iter()
                .map(|step| {
                    let [locator] = step.immediate_k_locators.as_slice() else {
                        panic!("validated continuation inheritance step has exactly one K locator");
                    };
                    locator.environment_index
                })
                .collect();
            let mut destination_origins = vec![
                destination.active_frame_origin.0,
                destination.ret_case_body_origin.0,
                destination.closure_origin.0,
                destination.closure_body_origin.0,
                destination.capture_occurrence.0,
            ];
            destination_origins
                .extend(destination.body_capture_reads.iter().map(|origin| origin.0));
            observations.push(CheckedIhContinuationInheritanceObservation {
                source_specialization: inheritance.transport.source_specialization.0,
                source_call_identity: format!("{:?}", inheritance.transport.source_call_identity),
                source_result_origin,
                destination_specialization,
                destination_body_origin: inheritance.capability.destination_body_origin.0,
                active_frame_lineage: inheritance
                    .capability
                    .self_resumption_steps
                    .iter()
                    .map(|step| step.active_frame_origin.0)
                    .collect(),
                destination_construct_origin: final_step.construct_origin.0,
                recursive_child_origin: final_step.recursive_child_origin.0,
                selected_case_body_origin: final_step.selected_case_body_origin.0,
                active_frame_origin: final_step.callee_binding.frame_origin.0,
                recursive_position: final_step.callee_binding.recursive_position,
                invocation_origin: final_step.invocation_origin.0,
                call_origin: final_step.call_origin.0,
                callee_origin: final_step.callee_origin.0,
                immediate_k_locator_count: final_step.immediate_k_locators.len(),
                immediate_k_locator_invocation_origin: final_locator.invocation_origin.0,
                immediate_k_locator_callee_origin: final_locator.callee_origin.0,
                immediate_k_locator_domain: format!("{:?}", final_locator.environment_domain),
                immediate_k_environment_index: final_locator.environment_index,
                immediate_k_preceding_environment_provenance: binder_resolutions
                    .get(&final_step.callee_origin)
                    .and_then(|resolution| resolution.preceding_environment_provenance)
                    .map(|provenance| format!("{provenance:?}")),
                immediate_k_lineage_environment_indices,
                ret_case_body_origin: destination.ret_case_body_origin.0,
                closure_origin: destination.closure_origin.0,
                capture_ordinal: destination.capture_ordinal,
                capture_occurrence: destination.capture_occurrence.0,
                closure_body_origin: destination.closure_body_origin.0,
                body_capture_reads: destination
                    .body_capture_reads
                    .iter()
                    .map(|origin| origin.0)
                    .collect(),
                closure_parameter_count: destination.closure_parameter_count as usize,
                fresh_destination_mentions_source_result: destination_origins
                    .contains(&source_result_origin),
                ordinary_non_governed_exclusion_count,
                descriptor_only_exclusion_count,
            });
        }
    });
}

/// Compile-preserving mutations for the continuation-inheritance validator.
#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckedIhContinuationInheritanceMutation {
    Exact,
    RemoveInheritedCapability,
    DuplicateInheritedCapability,
    SwapInheritedEndpoints,
    BreakSelfResumptionStep,
    RemoveImmediateKLocator,
    DuplicateImmediateKLocator,
    SubstituteWrongKLocatorDomain,
    SubstituteWrongKLocatorConsumer,
    SubstituteWrongKLocatorIndex,
    SubstituteSourceRecursiveSlotLocator,
    SubstituteFinalRecursorResidualLocator,
    /// Clone a real validated plan, insert a `Let` binder on its governed
    /// occurrence path, and rerun the unchanged forward derivation/validator.
    InsertInterveningBinder,
    ReclassifyRetChildAsIh,
    SubstituteDescriptorOnlyAuthority,
    SubstituteEarlierResult,
    SwapReadWriteEndpoints,
    /// Test-only inertness baseline: omit this otherwise validated planner-only
    /// projection so emitted artifacts can be compared byte-for-byte.
    SuppressForInertness,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static CONTINUATION_INHERITANCE_MUTATION:
        Cell<CheckedIhContinuationInheritanceMutation> =
            const { Cell::new(CheckedIhContinuationInheritanceMutation::Exact) };
    static CAPTURED_CONTINUATION_INHERITANCE_ENDPOINT:
        RefCell<Option<ContinuationCallIdentity>> = const { RefCell::new(None) };
}

/// Scope one validator mutation and restore exact production behavior even if
/// the compile under test unwinds.
#[cfg(feature = "px8-ds-test-support")]
pub fn with_checked_ih_continuation_inheritance_mutation<T>(
    mutation: CheckedIhContinuationInheritanceMutation,
    f: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            CONTINUATION_INHERITANCE_MUTATION
                .with(|active| active.set(CheckedIhContinuationInheritanceMutation::Exact));
            CAPTURED_CONTINUATION_INHERITANCE_ENDPOINT
                .with(|endpoint| endpoint.borrow_mut().take());
        }
    }
    CONTINUATION_INHERITANCE_MUTATION.with(|active| active.set(mutation));
    CAPTURED_CONTINUATION_INHERITANCE_ENDPOINT.with(|endpoint| endpoint.borrow_mut().take());
    let _restore = Restore;
    f()
}

#[cfg(feature = "px8-ds-test-support")]
pub fn checked_ih_continuation_inheritance_mutation_is_exact() -> bool {
    CONTINUATION_INHERITANCE_MUTATION.with(Cell::get)
        == CheckedIhContinuationInheritanceMutation::Exact
        && CAPTURED_CONTINUATION_INHERITANCE_ENDPOINT.with(|endpoint| endpoint.borrow().is_none())
}

#[cfg(feature = "px8-ds-test-support")]
pub(super) fn checked_ih_intervening_binder_population_control_is_active() -> bool {
    CONTINUATION_INHERITANCE_MUTATION.with(Cell::get)
        == CheckedIhContinuationInheritanceMutation::InsertInterveningBinder
}

#[cfg(feature = "px8-ds-test-support")]
fn replace_scratch_occurrence_expr<'src>(
    plan: &mut StaticTransitionPlan<'src>,
    origin: StaticOriginId,
    replacement: RuntimeExpr,
) -> Result<(), CraneliftBackendError> {
    let occurrence = plan
        .source_occurrences
        .get_mut(origin.0 as usize)
        .and_then(Option::as_mut)
        .ok_or_else(|| planner_error("intervening-binder control names no source occurrence"))?;
    if occurrence.static_origin != origin {
        return Err(planner_error(
            "intervening-binder control occurrence disagrees with its table position",
        ));
    }
    occurrence.expr = Box::leak(Box::new(replacement));
    Ok(())
}

/// Clone one validated real plan, insert a `Let` binder into the occurrence
/// population on its exact deepest governed path, capture-shift that exact
/// callee `Var`, then run the unchanged production inheritance derivation and
/// validator. The scratch plan never reaches lowering.
#[cfg(feature = "px8-ds-test-support")]
pub(super) fn run_checked_ih_intervening_binder_population_control(
    plan: &StaticTransitionPlan<'_>,
) -> Result<(), CraneliftBackendError> {
    if !checked_ih_intervening_binder_population_control_is_active() {
        return Ok(());
    }

    let mut candidates = Vec::new();
    for inheritance in &plan.checked_ih_continuation_inheritances {
        let Some(final_step) = inheritance.capability.self_resumption_steps.last() else {
            continue;
        };
        let [locator] = final_step.immediate_k_locators.as_slice() else {
            continue;
        };
        candidates.push((
            inheritance.capability.self_resumption_steps.len(),
            inheritance.transport.source_call_identity.clone(),
            inheritance.capability.destination_owner,
            inheritance.capability.destination_body_origin,
            final_step.callee_binding,
            final_step.selected_case_body_origin,
            final_step.callee_origin,
            locator.environment_index,
        ));
    }
    let Some(max_depth) = candidates.iter().map(|candidate| candidate.0).max() else {
        return Err(planner_error(
            "the intervening-binder control found no governed K arrival",
        ));
    };
    let deepest = candidates
        .into_iter()
        .filter(|candidate| candidate.0 == max_depth)
        .collect::<Vec<_>>();

    for (
        _,
        source_call_identity,
        destination_owner,
        destination_body_origin,
        callee_binding,
        insertion_origin,
        callee_origin,
        locator_index,
    ) in deepest
    {
        let mut scratch = plan.clone();
        let body_origin = scratch.semantic.child_origin(insertion_origin, 0)?;
        let value_origin = scratch
            .source_occurrences
            .iter()
            .flatten()
            .find_map(|occurrence| {
                matches!(occurrence.expr, RuntimeExpr::Value(_))
                    .then_some(occurrence.static_origin)
            })
            .ok_or_else(|| planner_error("the binder-bearing plan fixture has no closed value"))?;
        let value_expr = scratch.planned_occurrence_expr(value_origin)?.clone();
        let body_expr = scratch.planned_occurrence_expr(body_origin)?.clone();
        replace_scratch_occurrence_expr(
            &mut scratch,
            insertion_origin,
            RuntimeExpr::Let {
                value: Box::new(value_expr),
                body: Box::new(body_expr),
            },
        )?;

        let child_start = u32::try_from(scratch.semantic.child_origins.len())
            .map_err(|_| planner_capacity_error("intervening-binder child range exhausted"))?;
        scratch.semantic.child_origins.push(value_origin);
        scratch.semantic.child_origins.push(body_origin);
        let record = scratch
            .semantic
            .records
            .iter_mut()
            .find(|record| record.origin == insertion_origin)
            .ok_or_else(|| planner_error("the binder insertion origin has no semantic record"))?;
        record.child_origins = super::semantic_ir::DenseRange {
            start: child_start,
            len: 2,
        };

        let mut callee_expr = scratch.planned_occurrence_expr(callee_origin)?.clone();
        let RuntimeExpr::Var(index) = &mut callee_expr else {
            return Err(planner_error(
                "the governed self-resumption callee is not the exact Var resolved by the binder walk",
            ));
        };
        if *index != locator_index {
            return Err(planner_error(
                "the governed self-resumption callee Var disagrees with its immediate K locator",
            ));
        }
        *index = index
            .checked_add(1)
            .ok_or_else(|| planner_capacity_error("capture-shifted callee index exhausted"))?;
        replace_scratch_occurrence_expr(&mut scratch, callee_origin, callee_expr)?;

        let mutated = build_checked_ih_continuation_inheritances(&scratch)?;
        scratch.checked_ih_continuation_inheritances = mutated;
        validate_checked_ih_continuation_inheritances(
            &scratch,
            &scratch.checked_ih_continuation_inheritances,
        )?;
        let targets = scratch
            .checked_ih_continuation_inheritances
            .iter()
            .enumerate()
            .filter_map(|(index, inheritance)| {
                let final_step = inheritance.capability.self_resumption_steps.last()?;
                (&inheritance.transport.source_call_identity == &source_call_identity
                    && inheritance.capability.destination_owner == destination_owner
                    && inheritance.capability.destination_body_origin == destination_body_origin
                    && final_step.callee_binding == callee_binding)
                    .then_some(index)
            })
            .collect::<Vec<_>>();
        let [target] = targets.as_slice() else {
            return Err(planner_error(
                "the binder-bearing plan fixture does not retain one exact governed identity",
            ));
        };
        record_checked_ih_continuation_inheritances(
            &scratch,
            std::slice::from_ref(&scratch.checked_ih_continuation_inheritances[*target]),
        );
    }
    Ok(())
}

#[cfg(feature = "px8-ds-test-support")]
pub(super) fn apply_checked_ih_continuation_inheritance_mutation(
    inheritances: &mut Vec<CheckedIhContinuationInheritance>,
) {
    use CheckedIhContinuationInheritanceMutation as Mutation;
    match CONTINUATION_INHERITANCE_MUTATION.with(Cell::get) {
        Mutation::Exact | Mutation::InsertInterveningBinder => {}
        Mutation::RemoveInheritedCapability => {
            inheritances.pop();
        }
        Mutation::DuplicateInheritedCapability => {
            if let Some(inheritance) = inheritances.last().cloned() {
                inheritances.push(inheritance);
            }
        }
        Mutation::SwapInheritedEndpoints => {
            if inheritances.len() >= 2 {
                let split = inheritances.len() - 1;
                let (left, right) = inheritances.split_at_mut(split);
                std::mem::swap(
                    &mut left[split - 1].transport.source_call_identity,
                    &mut right[0].transport.source_call_identity,
                );
            }
        }
        Mutation::BreakSelfResumptionStep => {
            if let Some(inheritance) = inheritances
                .iter_mut()
                .rev()
                .find(|inheritance| inheritance.capability.self_resumption_steps.len() >= 2)
            {
                let first = inheritance.capability.self_resumption_steps[0].recursive_child_origin;
                inheritance.capability.self_resumption_steps[1].recursive_child_origin = first;
            }
        }
        Mutation::RemoveImmediateKLocator => {
            if let Some(step) = inheritances
                .last_mut()
                .and_then(|inheritance| inheritance.capability.self_resumption_steps.last_mut())
            {
                step.immediate_k_locators.clear();
            }
        }
        Mutation::DuplicateImmediateKLocator => {
            if let Some(step) = inheritances
                .last_mut()
                .and_then(|inheritance| inheritance.capability.self_resumption_steps.last_mut())
            {
                if let Some(locator) = step.immediate_k_locators.last().cloned() {
                    step.immediate_k_locators.push(locator);
                }
            }
        }
        Mutation::SubstituteWrongKLocatorDomain => {
            if let Some(locator) = inheritances
                .last_mut()
                .and_then(|inheritance| inheritance.capability.self_resumption_steps.last_mut())
                .and_then(|step| step.immediate_k_locators.last_mut())
            {
                locator.environment_domain =
                    CheckedIhKAvailabilityDomain::ForeignRuntimeEnvironment;
            }
        }
        Mutation::SubstituteWrongKLocatorConsumer => {
            if let Some(step) = inheritances
                .last_mut()
                .and_then(|inheritance| inheritance.capability.self_resumption_steps.last_mut())
            {
                if let Some(locator) = step.immediate_k_locators.last_mut() {
                    locator.invocation_origin = step.call_origin;
                    locator.callee_origin = step.call_origin;
                }
            }
        }
        Mutation::SubstituteWrongKLocatorIndex => {
            if let Some(locator) = inheritances
                .last_mut()
                .and_then(|inheritance| inheritance.capability.self_resumption_steps.last_mut())
                .and_then(|step| step.immediate_k_locators.last_mut())
            {
                locator.environment_index = locator.environment_index.wrapping_add(1);
            }
        }
        Mutation::SubstituteSourceRecursiveSlotLocator => {
            if let Some(inheritance) = inheritances.last_mut() {
                let source_slot = inheritance.transport.source_recursive_position;
                if let Some(locator) = inheritance
                    .capability
                    .self_resumption_steps
                    .last_mut()
                    .and_then(|step| step.immediate_k_locators.last_mut())
                {
                    locator.environment_domain = CheckedIhKAvailabilityDomain::SourceRecursiveSlot;
                    locator.environment_index = source_slot;
                }
            }
        }
        Mutation::SubstituteFinalRecursorResidualLocator => {
            if let Some(locator) = inheritances
                .last_mut()
                .and_then(|inheritance| inheritance.capability.self_resumption_steps.last_mut())
                .and_then(|step| step.immediate_k_locators.last_mut())
            {
                locator.environment_domain = CheckedIhKAvailabilityDomain::FinalRecursorResidual;
                locator.environment_index = 0;
            }
        }
        Mutation::ReclassifyRetChildAsIh => {
            if let Some(inheritance) = inheritances.last_mut() {
                let binding = inheritance
                    .capability
                    .self_resumption_steps
                    .last()
                    .expect("mutation requires one inherited step")
                    .callee_binding;
                inheritance.fresh_result_destination.constructor_child =
                    CheckedBinderProvenance::InductionHypothesis(binding);
            }
        }
        Mutation::SubstituteDescriptorOnlyAuthority => {
            if let Some(inheritance) = inheritances.last_mut() {
                inheritance.fresh_result_destination.constructor_child =
                    CheckedBinderProvenance::Ordinary;
            }
        }
        Mutation::SubstituteEarlierResult => {
            if let Some(inheritance) = inheritances.last_mut() {
                inheritance.fresh_result_destination.capture_occurrence =
                    inheritance.transport.source_result_origin;
            }
        }
        Mutation::SwapReadWriteEndpoints => {
            if let Some(inheritance) = inheritances.last_mut() {
                CAPTURED_CONTINUATION_INHERITANCE_ENDPOINT.with(|captured| {
                    let mut captured = captured.borrow_mut();
                    if let Some(read_endpoint) = captured.as_ref() {
                        inheritance.transport.source_call_identity = read_endpoint.clone();
                    } else {
                        *captured = Some(inheritance.transport.source_call_identity.clone());
                    }
                });
            }
        }
        Mutation::SuppressForInertness => inheritances.clear(),
    }
}

/// The allocation lane the ruled lifetime meet selects for one aggregate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum PlannedAggregateAllocation {
    /// Every child's possible-owner set excludes the invocation arena.
    PersistentGround,
    /// At least one child has an invocation-owned alternative, so the
    /// aggregate's own lifetime is the invocation.
    InvocationAggregate,
}
/// One child of an aggregate producer, with the exact facts the meet is taken
/// over.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PlannedAggregateChild {
    pub(in crate::cranelift_backend) position: u32,
    /// The child's own source occurrence.
    ///
    /// `None` for a child of a compiler-synthesized aggregate, which has no
    /// occurrence in the program. Recorded as an absence rather than filled
    /// with the parent's origin -- the aliasing that made a synthesized
    /// subtree indistinguishable from the expression it was emitted under.
    pub(in crate::cranelift_backend) origin: Option<StaticOriginId>,
    /// **`RT-DECL-CLOSURE-PORT` `D7` — the ordered field identity a RECORD
    /// producer plans at this position.**
    ///
    /// ⭐ Issued here, at the producer, and read nowhere else. A record's field
    /// names are a producer fact in exactly the way its ownership record is, so
    /// they travel with the template rather than being re-resolved at whatever
    /// coordinate the record is finally transferred at.
    ///
    /// ⛔ `None` for a constructor child and for a synthesized child — an
    /// absence, never a name to fall back on. A consumer comparing a record's
    /// carried identity against `None` must refuse rather than skip: two
    /// absences agreeing is the shape that let a grafted schema pass.
    pub(in crate::cranelift_backend) field_identity: Option<FieldIdentity>,
    pub(in crate::cranelift_backend) lifetime: PlannedReferentLifetime,
    /// The **possible** referent owners of this child, never a determination.
    ///
    /// ⚠ Read the emptiness rule before the membership rule: a child whose set
    /// is empty is not a child that owns nothing, it is a child whose
    /// representation the planner could not derive, and the builder refuses it
    /// rather than letting an empty set satisfy "contains no invocation owner"
    /// vacuously.
    pub(in crate::cranelift_backend) owners: Vec<BoundaryReferentOwner>,
}
/// **`RT-DECL-CLOSURE-PORT` `D7` — one exact ownership record per aggregate
/// producer occurrence.**
///
/// ⭐ **The lifetime of an aggregate is a MEET over its children, and no
/// per-value shape can compute it.** `Construct` and `Record` are persistable
/// shapes, so the value-shape disposition reaches for a persistent lane for
/// every one of them. That is right exactly when every child outlives the
/// parent, and it is the dangling edge otherwise — which is why the tag may not
/// be chosen at the allocation site from the value in hand.
///
/// ⛔ The consumer reads `allocation` and nothing else. It may not re-derive the
/// meet, inspect a runtime tag, or search lifetimes in lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PlannedAggregateOwnership {
    /// The opaque identity lowering carries on the template and hands back at
    /// emission. Dense, and equal to this record's index in the population.
    pub(in crate::cranelift_backend) id: AggregateOccurrenceId,
    /// Which producer this record is about, and the population's sort key.
    pub(in crate::cranelift_backend) producer: AggregateOccurrenceProducer,
    /// The function unit that emits a source producer.
    ///
    /// `None` for a synthesized role, which has no source occurrence and
    /// therefore no function owner. Spelled as an absence rather than a
    /// borrowed owner so nothing can read a synthesized record as if it were
    /// owned by whichever unit happened to emit it.
    pub(in crate::cranelift_backend) owner: Option<PredeclaredFunctionId>,
    pub(in crate::cranelift_backend) shape: PlannedAggregateShape,
    /// The tree's own child model for a synthesized use, handed to the emitter
    /// so it can check that each operand it holds is the KIND the meet was
    /// taken over.
    ///
    /// `None` for a source producer, whose children are occurrences in the
    /// program rather than nodes in a tree.
    pub(in crate::cranelift_backend) declared_children:
        Option<&'static [SynthesizedAggregateNode]>,
    pub(in crate::cranelift_backend) children: Vec<PlannedAggregateChild>,
    /// The meet itself, retained beside the lane it selects so a reader can
    /// see the derivation rather than only its verdict.
    pub(in crate::cranelift_backend) meet: PlannedReferentLifetime,
    pub(in crate::cranelift_backend) allocation: PlannedAggregateAllocation,
}
/// The **possible** referent owners of one aggregate child.
///
/// Two authorities bound this set and the answer is their intersection:
///
/// - **Lifetime** ([`lifetime_referent_affinity`]) — how long the referent may
///   live. `ActivationOwned` admits the invocation arena; `Persistent` does not.
/// - **Representation** ([`JoinResultRepresentation`]) — whether there is a
///   referent to own at all. A child the emitter materializes as a
///   `NativeScalarPair` is an immediate: it has no heap node, so no owner but
///   [`BoundaryReferentOwner::NoReferent`] is possible for it.
///
/// Reading only the first is what makes every call-shaped child look
/// arena-owned. `derive_occurrence_lifetime` answers `ActivationOwned` for
/// every `Call`, `Effect` and `PrimitiveCall` unconditionally — not because
/// their results are arena-owned, but because it does not look through them.
/// That is a sound floor on the LIFETIME axis and says nothing about the
/// REFERENT axis, and treating it as if it did forces an aggregate over two
/// integer-returning calls into the invocation lane. Such an aggregate is then
/// refused at the process root, which cannot accept an arena-owned answer — so
/// the over-approximation does not merely cost a lane, it rejects a program
/// that is sound and that ran before the lane existed.
///
/// This is a narrowing of "possible", not a relaxation of the escape rule. A
/// child with no referent cannot dangle, so it cannot be the reason a parent
/// must die with the invocation. Where the representation is unknown the
/// lifetime answer stands unnarrowed, which is the conservative direction.
pub(in crate::cranelift_backend::planning::static_transition) fn aggregate_child_referent_owners(
    plan: &StaticTransitionPlan<'_>,
    child: &PlannedOccurrenceChildAuthority,
) -> Result<Vec<BoundaryReferentOwner>, CraneliftBackendError> {
    let by_lifetime = lifetime_referent_affinity(child.lifetime);
    let representation = plan
        .join_results
        .get(child.origin.0 as usize)
        .and_then(|slot| slot.as_ref())
        .map(|result| result.representation);
    match representation {
        // The emitter will produce a native scalar pair here. There is no
        // boundary node, so there is nothing for an arena or a store to own.
        Some(JoinResultRepresentation::NativeScalarPair) => {
            Ok(vec![BoundaryReferentOwner::NoReferent])
        }
        // A carrier word may name a node, and an occurrence with no planned
        // join result tells us nothing. Both keep the lifetime's own answer.
        Some(JoinResultRepresentation::CarrierWord) | None => Ok(by_lifetime),
    }
}
/// Which compiler-built tree one synthesized aggregate path is rooted at.
///
/// A host operation synthesizes two independent values — the `error` arm and
/// the `ok` arm — and they are separate trees, not two halves of one. Rooting a
/// path at one of them is what keeps `FsWriteAt`'s `PrivateTransferCount`
/// (which lives under `ok`, inside `Wrote`) distinct from `FsReadAt`'s
/// error-side machinery, without either arm having to know the other's shape.
/// The unit-boundary root instead names the environment record at one exact
/// source-constructor field.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum SynthesizedAggregateRoot {
    HostResultError,
    HostResultOk,
    /// The environment record nested at one field of a source constructor
    /// whose closed result reaches a generated-unit call input.
    UnitBoundaryEnvironment,
    CheckedIhCapturedEnvironment,
    /// A positional captured environment crossing in place of a lexical
    /// closure whose body is selected statically.
    BoundaryClosureEnvironment,
}
/// One step from a synthesized aggregate to one of its ordered children.
///
/// ⛔ The two constructors are deliberately different steps rather than one
/// integer. A fixed constructor's field 0 and a dynamic constructor's
/// alternative 0 are positions in different structures — one is a child that is
/// always present, the other is a child that exists only when the discriminator
/// selects it. Collapsing them to a bare index would let a path name a node it
/// does not reach and still compare equal to the one that does.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend::planning::static_transition) enum SynthesizedAggregateStep {
    /// The ordered field of a fixed constructor, by position.
    Field(u32),
    /// One alternative of a dynamic constructor, by its ordered position in the
    /// closed alternative list.
    ///
    /// ⚠ The POSITION, not the ABI discriminator tag. `ResourceKind`'s two tags
    /// are wire-schema facts (`wire.resource_kind_fs_handle`), so a path keyed
    /// on them would depend on a value this planner does not own and could not
    /// state without importing the host's wire layout. The position is the same
    /// fact on both sides, and the emitter's own alternative list is checked
    /// against it at construction.
    Alternative(u32),
}
/// The exact position of one synthesized aggregate in its compiler-built tree.
///
/// ⭐ **This is the fact a role alone cannot supply.** Six of the measured
/// construction sites build a repeated role at one seat: `ResourceKind` appears
/// under `ResourceReleaseFailed` field 0 and `ResourceKindMismatch` fields 0
/// and 1, and the `IOError` alternative set appears under `ResourceHostIo`
/// field 0, `ResourceReleaseFailed` field 2 and `FileError` field 2. A
/// role-keyed record cannot tell those apart, so one row would have to serve
/// three allocations.
///
/// ⛔ The separator is **where the node sits**, never an issued ordinal. An
/// ordinal would have to count emissions in lowering's control flow, which the
/// planner does not execute and therefore cannot compute; the path is measured
/// structure and both sides can state it independently.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct SynthesizedAggregatePath {
    pub(in crate::cranelift_backend::planning::static_transition) root: SynthesizedAggregateRoot,
    pub(in crate::cranelift_backend::planning::static_transition)
        steps: Vec<SynthesizedAggregateStep>,
}
impl SynthesizedAggregatePath {
    /// The empty path at one of a host result's two arms.
    pub(in crate::cranelift_backend) fn root(root: SynthesizedAggregateRoot) -> Self {
        Self {
            root,
            steps: Vec::new(),
        }
    }

    pub(in crate::cranelift_backend) fn root_kind(&self) -> SynthesizedAggregateRoot {
        self.root
    }

    /// This path extended by one ordered field of a fixed constructor.
    pub(in crate::cranelift_backend) fn field(&self, position: u32) -> Self {
        self.extend(SynthesizedAggregateStep::Field(position))
    }

    /// This path extended by one alternative of a dynamic constructor.
    pub(in crate::cranelift_backend) fn alternative(&self, position: u32) -> Self {
        self.extend(SynthesizedAggregateStep::Alternative(position))
    }

    fn extend(&self, step: SynthesizedAggregateStep) -> Self {
        let mut steps = self.steps.clone();
        steps.push(step);
        Self {
            root: self.root,
            steps,
        }
    }
}
/// One node of a host operation's closed synthesized aggregate tree.
///
/// ⭐ **The tree is the recipe.** The previous spelling was a flat per-role
/// child list plus a flat per-operation use list, and the two together could
/// not state *where* a use sits — which is exactly the fact that separates the
/// six repeated-role sites above.
///
/// ## Acyclicity is a compile-time property here, not a runtime colouring
///
/// The children are `&'static` slices built from `const` items, and a `const`
/// that transitively references itself is an evaluation cycle rustc rejects. So
/// the walk over this tree terminates because the tree is finite, and there is
/// no back-edge check to get wrong. The previous role-graph spelling needed a
/// visiting/done colouring precisely because a role could name itself; a value
/// tree cannot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum SynthesizedAggregateNode {
    /// A fixed constructor with ordered children, built through
    /// `Lowered::Constructor`.
    Fixed {
        role: SynthesizedFixedConstructorRole,
        children: &'static [SynthesizedAggregateNode],
    },
    /// A dynamic constructor: exactly one alternative is selected at runtime.
    Dynamic(SynthesizedDynamicSet),
    /// A scalar, named by the **exact** closed immediate disposition the
    /// emitter must produce for it.
    ///
    /// `spill` is the whole reason this is not one class. A
    /// `RepresentedImmediate` does NOT mean "no boundary node": when a runtime
    /// magnitude test finds the payload too wide for the immediate field, a
    /// `spill: Some(_)` value becomes a handle of that class, which is a
    /// persistent-store referent. So
    ///
    /// - `spill: None` (`Bool`) is exactly `{NoReferent}`;
    /// - `spill: Some(_)` (`Int`, `BoundedNat`, `StructuralNat`,
    ///   `ProcessExitStatus`) is `{NoReferent, PersistentStore}`.
    ///
    /// These are the disposition authority's own two fields, not a second tag
    /// table. Recording only the broad `RepresentedImmediate` family, or
    /// widening every scalar to the larger set, is a safe LANE answer and a
    /// false statement about the child -- and a record's owner sets are its
    /// stated evidence, not merely a means to a verdict.
    Scalar {
        tag: BoundaryTag,
        spill: Option<BoundaryClass>,
    },
    /// A value the **Effect seat itself** supplies, named by the ordered
    /// position of the operand it comes from.
    ///
    /// ⭐ **Site-dependence is not non-reachability.** `OptionSome` wraps the
    /// seat's path operand and `PrivateBufferSpan` carries the seat's buffer
    /// `ResourceToken`; both are real allocations that production emits. What
    /// is unavailable for them is a *role-invariant* meet — not a meet. The
    /// evidence is exact and it is already in the plan: the operand is a child
    /// occurrence of this very seat, with its own lifetime and join
    /// representation.
    ///
    /// ⛔ So this is resolved against the seat, never defaulted and never
    /// pruned. Omitting the node from `P` because no role-invariant answer
    /// exists is not the fail-closed direction once production can emit the
    /// allocation — it leaves a real allocation with no record. If the seat's
    /// operand evidence cannot be derived, **planning fails**.
    ///
    /// The index is into the seat's `args`, before the capability offset that
    /// `RuntimeExpr::Effect` applies to its semantic children.
    SiteOperand(u32),
    /// **A carried continuation-envelope worker-capture word, by position.**
    ///
    /// The child at position `i` is the `i`-th `WorkerCapture` operand of the
    /// unit's ruled ordered-worker envelope -- the ci<->oi run. This is the
    /// checked-IH captured environment's child model.
    ///
    /// ⛔ **THIS IS DELIBERATELY NOT [`Self::SiteOperand`], AND THE TWO MUST
    /// NEVER SHARE A RESOLUTION PATH.** `SiteOperand` resolves against an
    /// EFFECT SEAT's claimed operand vector (`ClaimedEffectSeats`, keyed
    /// `EffectSeatSlot::Argument(i)`). A captured environment has no effect
    /// seat: its operands are the continuation envelope's carried capture
    /// words, reached by a different path entirely. Resolving this kind through
    /// the effect-seat arm would force a second operand source into
    /// `reconcile_declared_children` and weaken the path-identity check every
    /// host-result aggregate depends on -- which is the reuse that was measured
    /// and refused before this kind existed.
    ///
    /// ⛔ Its arity is PER UNIT while the slice is `&'static`, which works only
    /// because the content at position `i` is fully determined by `i`: the
    /// planner slices a const run to the unit's capture arity. That is sound
    /// exactly as long as an arity ABOVE the const run's length REFUSES.
    /// Silently truncating would hand the emitter a child model shorter than
    /// the record it governs, and every count downstream would agree with
    /// itself while describing fewer captures than exist.
    WorkerCaptureOperand(u32),
    /// This arm of the host result synthesizes no aggregate at all.
    ///
    /// `FsReadFile`'s `ResponseBytes`, `FsOpen`'s `ResourceToken`,
    /// `FsHandleMetadata`'s `Int`. Distinct from [`Self::SiteOperand`] on
    /// purpose: that one is a child whose evidence the seat supplies, this one
    /// is a position where the tree governs nothing because no aggregate is
    /// built. Collapsing them would let "no allocation here" and "an allocation
    /// whose child comes from the site" share an arm.
    Absent,
}
/// The closed alternative set of one dynamic constructor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum SynthesizedDynamicSet {
    /// The `IOError` alternative set, whose population is the process symbol
    /// inventory rather than a list this module can spell.
    ///
    /// Named rather than enumerated because its arity is
    /// `NativeProcessSymbols::io_errors.len()` and its roles are minted by
    /// [`StaticTransitionPlan::synthesized_io_error_roles`]. Its alternatives
    /// carry at most one `Int` payload, on the last alternative only.
    IoErrors,
    /// An alternative list this module states, indexed by position.
    Alternatives(&'static [SynthesizedAggregateNode]),
}
impl SynthesizedAggregateNode {
    /// A `BoundedNat` scalar, as the reply-validation lowering produces it.
    pub(in crate::cranelift_backend::planning::static_transition) const fn bounded_nat() -> Self {
        Self::Scalar {
            tag: BoundaryTag::ImmediateBoundedNat,
            spill: Some(BoundaryClass::Int),
        }
    }

    /// A native `Int` scalar.
    pub(in crate::cranelift_backend::planning::static_transition) const fn native_int() -> Self {
        Self::Scalar {
            tag: BoundaryTag::ImmediateInt,
            spill: Some(BoundaryClass::Int),
        }
    }

    /// A fixed constructor with no children.
    pub(in crate::cranelift_backend::planning::static_transition) const fn nullary(role: SynthesizedFixedConstructorRole) -> Self {
        Self::Fixed {
            role,
            children: &[],
        }
    }
}
/// **`RT-DECL-CLOSURE-PORT` `D7` — the closed synthesized aggregate tree of one
/// host operation, both arms.**
///
/// ## It was MEASURED, not transcribed
///
/// This structure lives in `lower_process_host_effect` as roughly four hundred
/// lines of imperative code interleaved with `builder.ins()` calls. It was
/// derived by instrumenting every `synthesized_constructor` and
/// `synthesized_dynamic_alternative` call to print the KINDS of its own
/// children, running the suite **single-threaded**, and reading the edges off
/// the log.
///
/// The single-threaded run is not incidental: `--nocapture` output from
/// concurrent tests interleaves, and a parallel run manufactured a phantom in
/// which one seat built its error tree twice. That phantom would have forced a
/// planner-issued repetition ordinal into the key — an ordinal the planner
/// cannot compute, because it depends on lowering's control flow.
///
/// ## Two things the measurement found that a transcription would have smoothed
///
/// - `OptionSome -> [Bytes]` and `PrivateBufferSpan -> [ResourceToken, …]` are
///   genuine **site-dependent leaves**. A `ResourceToken` is a handle, not a
///   scalar, so neither can take a role-invariant child model.
/// - `Wrote -> [Bool]` occurs exactly once in the whole suite, in the `c2_ac4`
///   fixture, and disagrees with every production construction of `Wrote`. It
///   is recorded here as the fixture's disagreement, not the tree's.
///
/// ## The eager `IOError` template is ABANDONED for the resource-surface ops
///
/// `lower_process_host_effect` builds one `IOError` dynamic constructor before
/// it knows which branch it is in. The file operations use it as `FileError`
/// field 2 and the console operations use it as the whole error; the six
/// resource-surface operations build their **own** `surface_io_error` and never
/// reference it. So the trees below do not contain it at those roots — an
/// abandoned template is not a semantic use, and giving it a path would plan a
/// record for an allocation that never happens.
pub(in crate::cranelift_backend::planning::static_transition) fn host_effect_recipe_tree(operation: ken_host::HostOpV1) -> SynthesizedHostResultTree {
    use ken_host::HostOpV1 as Op;
    use SynthesizedAggregateNode as N;
    use SynthesizedFixedConstructorRole as R;

    const NAT2: &[SynthesizedAggregateNode] = &[N::bounded_nat(), N::bounded_nat()];
    const INT2: &[SynthesizedAggregateNode] = &[N::native_int(), N::native_int()];

    /// `PrivateTransferCount(BoundedNat, BoundedNat)`.
    const TRANSFER_COUNT: SynthesizedAggregateNode = N::Fixed {
        role: R::PrivateTransferCount,
        children: NAT2,
    };
    /// `ResourceTraceIdentity(Int, Int)`.
    const TRACE_IDENTITY: SynthesizedAggregateNode = N::Fixed {
        role: R::ResourceTraceIdentity,
        children: INT2,
    };
    /// The two-alternative `ResourceKind` set, at wire tags this module does
    /// not spell — reached by POSITION, per [`SynthesizedAggregateStep`].
    const RESOURCE_KIND: SynthesizedAggregateNode =
        N::Dynamic(SynthesizedDynamicSet::Alternatives(&[
            N::nullary(R::ResourceKindFsHandle),
            N::nullary(R::ResourceKindBuffer),
        ]));
    const IO_ERRORS: SynthesizedAggregateNode = N::Dynamic(SynthesizedDynamicSet::IoErrors);

    /// The eleven-alternative resource surface, in the emitter's own order.
    const RESOURCE_SURFACE: SynthesizedAggregateNode =
        N::Dynamic(SynthesizedDynamicSet::Alternatives(&[
            N::Fixed {
                role: R::ResourceHostIo,
                children: &[IO_ERRORS],
            },
            N::nullary(R::ResourceClosed),
            N::nullary(R::ResourceMalformed),
            N::Fixed {
                role: R::ResourceRightNotHeld,
                children: INT2,
            },
            N::Fixed {
                role: R::ResourceReleaseFailed,
                children: &[RESOURCE_KIND, TRACE_IDENTITY, IO_ERRORS],
            },
            N::Fixed {
                role: R::ResourceKindMismatch,
                children: &[RESOURCE_KIND, RESOURCE_KIND],
            },
            N::nullary(R::ResourceBufferLimit),
            N::nullary(R::ResourceAllocationFailed),
            N::nullary(R::ResourceInvalidOffset),
            N::nullary(R::ResourceInvalidBounds),
            N::nullary(R::ResourceNoProgress),
        ]));

    /// `Option::Some(<the site's path operand>)`.
    ///
    /// Its child is site-bound, which bounds the ROLE-INVARIANT meet and not
    /// the meet: this node and every parent of it gets an exact seat-bound
    /// record, derived from the seat's own operand authority.
    const SOME_SITE_PATH: SynthesizedAggregateNode = N::Fixed {
        role: R::OptionSome,
        // The seat's operand 0 — the path the caller passed.
        children: &[N::SiteOperand(0)],
    };
    /// `FileError(FileOperation*, Option::Some(<site path>), IOError)`.
    const READ_FILE_ERROR_CHILDREN: &[SynthesizedAggregateNode] = &[
        N::nullary(R::FileOperationRead),
        SOME_SITE_PATH,
        IO_ERRORS,
    ];
    const WRITE_FILE_ERROR_CHILDREN: &[SynthesizedAggregateNode] = &[
        N::nullary(R::FileOperationWrite),
        SOME_SITE_PATH,
        IO_ERRORS,
    ];
    const CHANGE_MODE_ERROR_CHILDREN: &[SynthesizedAggregateNode] = &[
        N::nullary(R::FileOperationChangeMode),
        SOME_SITE_PATH,
        IO_ERRORS,
    ];
    const READ_FILE_ERROR: SynthesizedAggregateNode = N::Fixed {
        role: R::FileError,
        children: READ_FILE_ERROR_CHILDREN,
    };
    const WRITE_FILE_ERROR: SynthesizedAggregateNode = N::Fixed {
        role: R::FileError,
        children: WRITE_FILE_ERROR_CHILDREN,
    };
    const CHANGE_MODE_ERROR: SynthesizedAggregateNode = N::Fixed {
        role: R::FileError,
        children: CHANGE_MODE_ERROR_CHILDREN,
    };

    /// The `FsReadAt` success value: `ReadEof` or `ReadSome(span, transferred)`.
    const READ_PROGRESS: SynthesizedAggregateNode =
        N::Dynamic(SynthesizedDynamicSet::Alternatives(&[
            N::nullary(R::ReadEof),
            N::Fixed {
                role: R::ReadSome,
                children: &[
                    // `PrivateBufferSpan(ResourceToken, Int, BoundedNat)` — the
                    // token is the site's buffer operand, so this whole node is
                    // site-dependent.
                    N::Fixed {
                        role: R::PrivateBufferSpan,
                        // The seat's operand 2 — the buffer `ResourceToken`
                        // this span is bound to (`PX8-SPAN-PROV`).
                        children: &[N::SiteOperand(2), N::native_int(), N::bounded_nat()],
                    },
                    TRANSFER_COUNT,
                ],
            },
        ]));
    const WROTE: SynthesizedAggregateNode = N::Fixed {
        role: R::Wrote,
        children: &[TRANSFER_COUNT],
    };
    const UNIT: SynthesizedAggregateNode = N::nullary(R::Unit);

    let (error, ok) = match operation {
        // Returns a `Bool` before any synthesized producer runs, so neither arm
        // exists. Not a gap: the early return is above the synthesis entirely.
        Op::ConsoleIsTerminal => (N::Absent, N::Absent),
        Op::ConsoleWrite | Op::ConsoleFlush => (IO_ERRORS, UNIT),
        Op::FsReadFile => (READ_FILE_ERROR, N::Absent),
        Op::FsOpen => (READ_FILE_ERROR, N::Absent),
        // ⚠ The `ok` arm here is the emitter's `else` branch, which is `Unit`.
        // The flat use table this tree replaces derived these two rows from the
        // operation match and MISSED that branch, so it planned no `Unit`
        // record for them. No fixture exercises either operation, which is why
        // the omission was invisible; the tree states both arms from the same
        // match the emitter uses, so an arm cannot be dropped by inattention.
        Op::FsWriteFile => (WRITE_FILE_ERROR, UNIT),
        Op::FsChangeMode => (CHANGE_MODE_ERROR, UNIT),
        Op::BufferAllocate | Op::BufferFreeze => (RESOURCE_SURFACE, N::Absent),
        Op::FsHandleMetadata => (RESOURCE_SURFACE, N::Absent),
        Op::ResourceRelease => (RESOURCE_SURFACE, UNIT),
        Op::FsReadAt => (RESOURCE_SURFACE, READ_PROGRESS),
        Op::FsWriteAt => (RESOURCE_SURFACE, WROTE),
        // Not an admitted consumer; `lower_process_host_effect` refuses it
        // before any synthesized producer runs.
        _ => (N::Absent, N::Absent),
    };
    SynthesizedHostResultTree { error, ok }
}
/// The two synthesized aggregate trees of one host operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend::planning::static_transition) struct SynthesizedHostResultTree {
    pub(in crate::cranelift_backend::planning::static_transition) error: SynthesizedAggregateNode,
    pub(in crate::cranelift_backend::planning::static_transition) ok: SynthesizedAggregateNode,
}
impl SynthesizedHostResultTree {
    pub(in crate::cranelift_backend::planning::static_transition) fn node(&self, root: SynthesizedAggregateRoot) -> SynthesizedAggregateNode {
        match root {
            SynthesizedAggregateRoot::HostResultError => self.error,
            SynthesizedAggregateRoot::HostResultOk => self.ok,
            // Environment records are derived from source call-input results,
            // not from a host operation's synthesized tree. Returning the
            // absent node keeps the host-tree resolver fail-closed if the two
            // domains are ever accidentally mixed.
            SynthesizedAggregateRoot::UnitBoundaryEnvironment
            | SynthesizedAggregateRoot::CheckedIhCapturedEnvironment
            | SynthesizedAggregateRoot::BoundaryClosureEnvironment => {
                SynthesizedAggregateNode::Absent
            }
        }
    }
}
/// Collect the site-bound operand ordinals named anywhere under one
/// compiler-synthesized result node.
///
/// This walks the same closed recipe that plans aggregate children. It is not
/// a second operation table: adding or removing a `SiteOperand` in the recipe
/// changes both the planned child relation and this population together.
pub(in crate::cranelift_backend::planning::static_transition) fn collect_site_operand_ordinals(node: SynthesizedAggregateNode, ordinals: &mut BTreeSet<u32>) {
    match node {
        SynthesizedAggregateNode::Fixed { children, .. } => {
            for child in children {
                collect_site_operand_ordinals(*child, ordinals);
            }
        }
        SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(alternatives)) => {
            for alternative in alternatives {
                collect_site_operand_ordinals(*alternative, ordinals);
            }
        }
        // The closed IOError alternatives contain only scalar payloads. They
        // cannot introduce a site operand behind this dynamic node.
        // ⛔ A capture word is NOT a site operand, and this arm states that
        // rather than inheriting it. The two name different operand vectors --
        // an effect seat's `args` versus the continuation envelope's ruled
        // WorkerCapture run -- so a capture contributes no ordinal to a
        // population that indexes the seat's arguments. Folding it in here
        // would claim the seat supplies an argument it does not have.
        SynthesizedAggregateNode::WorkerCaptureOperand(_)
        | SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors)
        | SynthesizedAggregateNode::Scalar { .. }
        | SynthesizedAggregateNode::Absent => {}
        SynthesizedAggregateNode::SiteOperand(index) => {
            ordinals.insert(index);
        }
    }
}
/// What a path walk arrived at.
///
/// The `IOError` alternatives are not nodes in the static tree — they are
/// minted from the process symbol inventory — so a walk that reaches one
/// reports its position rather than a node it cannot produce.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SynthesizedTreeResolution {
    Node(SynthesizedAggregateNode),
    IoErrorAlternative(u32),
}
/// One semantic use the flattening found: a node, and where it sits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend::planning::static_transition) struct FlattenedSynthesizedUse {
    pub(in crate::cranelift_backend::planning::static_transition) path: SynthesizedAggregatePath,
    pub(in crate::cranelift_backend::planning::static_transition) role: SynthesizedConstructorRole,
    pub(in crate::cranelift_backend::planning::static_transition)
        children: &'static [SynthesizedAggregateNode],
}
/// Flatten one operation's trees to every **allocation-reachable** use.
///
/// ⭐ **A dynamic SET is not an allocation; every selected ALTERNATIVE is.**
/// That is the boundary, ruled by the Architect after an earlier spelling drew
/// it at "a `Fixed` node that is not itself a dynamic alternative". That
/// predicate was wrong in a specific way worth recording: it was about *which
/// authority supplies the lane today* (`emit_carrier_dynamic_constructor` read
/// the value-shape disposition) rather than about *what allocates*. An
/// alternative calls `emit_carrier_alloc` exactly as a fixed constructor does —
/// dynamic allocations are measured live — so reconciling one against the tree
/// without interning an occurrence proves only that lowering named the expected
/// schema. It supplies no lifetime record, cannot enter `R`, and cannot satisfy
/// "every event has exactly one record".
///
/// The traversal is therefore total over constructor-valued nodes:
///
/// ```text
/// Fixed(role, children):
///   intern (owner, seat, path, Fixed(role)) with its ordered child model
///   recurse into every aggregate-valued child at path.Field(position)
///
/// Dynamic(Alternatives(alts)):
///   visit each alternative at path.Alternative(index)
///
/// Dynamic(IoErrors):
///   intern each planner-issued role at path.Alternative(index),
///   keyed IoError(role), with that role's exact children
/// ```
///
/// ⛔ Nothing is pruned for want of a role-invariant meet. A node whose child
/// comes from the emission site gets an exact **site-bound** record derived
/// from the seat's own operand authority, or planning fails.
pub(in crate::cranelift_backend::planning::static_transition) fn flatten_allocation_reachable_uses(
    plan: &StaticTransitionPlan<'_>,
    operation: ken_host::HostOpV1,
) -> Vec<FlattenedSynthesizedUse> {
    let tree = host_effect_recipe_tree(operation);
    let io_errors = plan.semantic.synthesized_io_error_roles().len();
    let mut uses = Vec::new();
    for root in [
        SynthesizedAggregateRoot::HostResultError,
        SynthesizedAggregateRoot::HostResultOk,
    ] {
        collect_reachable_uses(
            tree.node(root),
            &SynthesizedAggregatePath::root(root),
            io_errors,
            plan,
            &mut uses,
        );
    }
    uses
}
/// The ordered children of one `IOError` alternative.
///
/// The set is nullary but for its **last** alternative, which carries the
/// decoded payload as a native `Int`. That is the emitter's own shape in
/// `synthesized_io_error_alternatives`, restated here so the record's child
/// model is the one the allocation actually has.
fn io_error_alternative_children(
    index: usize,
    count: usize,
) -> &'static [SynthesizedAggregateNode] {
    const PAYLOAD: &[SynthesizedAggregateNode] = &[SynthesizedAggregateNode::native_int()];
    if count > 0 && index + 1 == count {
        PAYLOAD
    } else {
        &[]
    }
}
/// The flattening's walk. Terminates because the tree is a finite `&'static`
/// value; see [`SynthesizedAggregateNode`] on why it cannot be cyclic.
fn collect_reachable_uses(
    node: SynthesizedAggregateNode,
    path: &SynthesizedAggregatePath,
    io_errors: usize,
    plan: &StaticTransitionPlan<'_>,
    uses: &mut Vec<FlattenedSynthesizedUse>,
) {
    match node {
        SynthesizedAggregateNode::Fixed { role, children } => {
            uses.push(FlattenedSynthesizedUse {
                path: path.clone(),
                role: SynthesizedConstructorRole::Fixed(role),
                children,
            });
            for (position, child) in children.iter().enumerate() {
                collect_reachable_uses(
                    *child,
                    &path.field(position as u32),
                    io_errors,
                    plan,
                    uses,
                );
            }
        }
        // The set is not an allocation. Each alternative is, and each is
        // visited at its own ordered position -- which is what separates the
        // three `ResourceKind` uses and the repeated `IOError` sets.
        SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(alternatives)) => {
            for (position, alternative) in alternatives.iter().enumerate() {
                collect_reachable_uses(
                    *alternative,
                    &path.alternative(position as u32),
                    io_errors,
                    plan,
                    uses,
                );
            }
        }
        // The closed `IOError` inventory supplies both the alternative token
        // and the role. The ABI tag remains non-authoritative: the path step is
        // the ordered position, as everywhere else.
        SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors) => {
            for (position, role) in plan
                .semantic
                .synthesized_io_error_roles()
                .iter()
                .enumerate()
            {
                uses.push(FlattenedSynthesizedUse {
                    path: path.alternative(position as u32),
                    role: SynthesizedConstructorRole::IoError(*role),
                    children: io_error_alternative_children(position, io_errors),
                });
            }
        }
        SynthesizedAggregateNode::Scalar { .. }
        | SynthesizedAggregateNode::SiteOperand(_)
        // A capture word is a leaf and, more to the point, is never reachable
        // from a host-effect recipe at all: this walk starts at
        // `host_effect_recipe_tree(operation)`, and no recipe names a capture.
        // The checked-IH child model is built by the issuance site from the
        // unit's own ci<->oi run, not flattened out of an operation tree.
        | SynthesizedAggregateNode::WorkerCaptureOperand(_)
        | SynthesizedAggregateNode::Absent => {}
    }
}
/// The possible referent owners of one tree node, at one exact effect seat.
///
/// ⚠ Exactness matters here independently of the verdict. These sets are the
/// record's stated evidence, and a set that is merely *sufficient* to reach the
/// right lane is still a false statement about what the child can be.
///
/// ⛔ **There is no "not derivable" answer.** A node whose child comes from the
/// emission site is resolved against that site — the operand is a child
/// occurrence of this very seat, and its lifetime and join representation are
/// already planned. If that evidence cannot be read, planning FAILS. Returning
/// an absence here is what previously pruned four real constructors out of `P`,
/// leaving allocations production emits with no record at all.
pub(in crate::cranelift_backend::planning::static_transition) fn node_referent_owners(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
    node: SynthesizedAggregateNode,
) -> Result<Vec<BoundaryReferentOwner>, CraneliftBackendError> {
    match node {
        // A scalar is NOT `NoReferent` alone. `Int`, `BoundedNat`,
        // `StructuralNat` and `ProcessExitStatus` each have a declared
        // persistent SPILL arm, so a scalar child may be a persistent-store
        // referent. It can never be arena-owned, which is why the lane verdict
        // is the same either way -- and why recording `{NoReferent}` was a
        // false statement that happened not to cost anything yet.
        SynthesizedAggregateNode::Scalar { spill: None, .. } => {
            Ok(vec![BoundaryReferentOwner::NoReferent])
        }
        SynthesizedAggregateNode::Scalar { spill: Some(_), .. } => Ok(vec![
            BoundaryReferentOwner::NoReferent,
            BoundaryReferentOwner::PersistentStore,
        ]),
        // A nested fixed constructor IS a referent, and its owner is
        // determined -- it is the lane its own children select. It is never
        // `NoReferent`, and listing alternatives it cannot take would describe
        // a different node than the one being allocated.
        SynthesizedAggregateNode::Fixed { children, .. } => {
            Ok(vec![fixed_node_selected_owner(plan, seat, children)?])
        }
        // ⭐ A dynamic child is the UNION of its alternatives' selected owners.
        //
        // ⛔ Not `None`, and not a flat `{PersistentStore}` read off the lane
        // `emit_carrier_dynamic_constructor` happens to take. The value at this
        // position is whichever alternative the discriminator selects, so the
        // parent must survive every one of them: a single invocation-capable
        // alternative makes the parent invocation-owned. Answering persistent
        // because the *set* is shaped persistently would allocate the parent
        // over a child that can be shorter-lived than it.
        SynthesizedAggregateNode::Dynamic(set) => {
            let mut owners = BTreeSet::new();
            for alternative in dynamic_alternative_nodes(plan, set) {
                owners.insert(fixed_node_selected_owner_of(plan, seat, alternative)?);
            }
            if owners.is_empty() {
                return Err(planner_error(
                    "a dynamic aggregate child has no alternatives, so its owner set is empty \
                     and would satisfy the escape test vacuously",
                ));
            }
            Ok(owners.into_iter().collect())
        }
        // The seat's own operand. Its evidence is the child occurrence's
        // lifetime narrowed by its join representation -- the same two
        // authorities a source aggregate's children are read through, applied
        // to the exact operand this node names.
        SynthesizedAggregateNode::SiteOperand(index) => {
            site_operand_referent_owners(plan, seat, index)
        }
        // ⛔ A capture word's owners are NOT derivable from this node, so this
        // REFUSES rather than guessing. `SiteOperand` above can answer because
        // the seat's own child occurrence carries the evidence; a capture's
        // evidence is its capture occurrence in the ruled ci<->oi run, which
        // this shape node does not name and this function is not given. The
        // checked-IH issuance derives each child's owners from that occurrence's
        // authority directly. Reaching here means something walked a captured
        // environment down the host-recipe owner path, where the answer would be
        // an invention.
        SynthesizedAggregateNode::WorkerCaptureOperand(_) => Err(planner_error(
            "a worker-capture operand's referent owners come from its capture occurrence, \
             not from the shape node, so they cannot be derived on the host-recipe path",
        )),
        // ⛔ Never a child. `Absent` marks a host-result arm that builds no
        // aggregate; reaching it as a child means the tree claims an allocation
        // has a child at a position where nothing is built.
        SynthesizedAggregateNode::Absent => Err(planner_error(
            "a synthesized aggregate child is marked absent, so the tree describes an \
             allocation whose operand is not built",
        )),
    }
}
/// The possible owners of the operand one [`SynthesizedAggregateNode::SiteOperand`]
/// names, read from the seat's own child occurrence.
///
/// The index is into the Effect's `args`; its semantic child position is offset
/// by the capability operand, exactly as `lower_process_host_effect` offsets it
/// when it lowers the same operand.
fn site_operand_referent_owners(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
    index: u32,
) -> Result<Vec<BoundaryReferentOwner>, CraneliftBackendError> {
    let occurrence = plan
        .source_occurrences
        .get(seat.0 as usize)
        .and_then(|slot| slot.as_ref())
        .ok_or_else(|| planner_error("synthesized aggregate seat is not an occurrence"))?;
    let RuntimeExpr::Effect { capability, .. } = occurrence.expr else {
        return Err(planner_error(
            "a site-bound synthesized aggregate child names a seat that is not a host effect",
        ));
    };
    let position = usize::from(capability.is_some())
        .checked_add(index as usize)
        .ok_or_else(|| planner_capacity_error("site operand position overflows"))?;
    let authority = occurrence_authority(plan, seat)?;
    let child = authority.children.get(position).ok_or_else(|| {
        planner_error(
            "a site-bound synthesized aggregate child names an operand the seat does not have",
        )
    })?;
    let owners = aggregate_child_referent_owners(plan, child)?;
    if owners.is_empty() {
        return Err(planner_error(
            "a site-bound synthesized aggregate child has no derivable referent owner",
        ));
    }
    Ok(owners)
}
/// The alternative nodes of a dynamic set, as owner derivation must see them.
fn dynamic_alternative_nodes(
    plan: &StaticTransitionPlan<'_>,
    set: SynthesizedDynamicSet,
) -> Vec<SynthesizedAggregateNode> {
    match set {
        SynthesizedDynamicSet::Alternatives(alternatives) => alternatives.to_vec(),
        // Every `IOError` alternative is nullary but the last, which carries an
        // `Int`. Both shapes are enumerated rather than collapsed to the widest,
        // so the union is over the alternatives that actually exist.
        SynthesizedDynamicSet::IoErrors => {
            let count = plan.semantic.synthesized_io_error_roles().len();
            (0..count)
                .map(|index| SynthesizedAggregateNode::Fixed {
                    // The role names an alternative for shape purposes only;
                    // owner derivation never keys on it, only on the children.
                    role: SynthesizedFixedConstructorRole::ResourceHostIo,
                    children: io_error_alternative_children(index, count),
                })
                .collect()
        }
    }
}
/// The exact owner one fixed node's allocation takes, given its children.
///
/// `Wrote` is persistent **because** `PrivateTransferCount` is, which is
/// persistent because neither of its scalar children can be arena-owned. The
/// chain is computed rather than asserted per role, so a verdict cannot
/// disagree with the tree it is supposed to follow.
pub(in crate::cranelift_backend::planning::static_transition) fn fixed_node_selected_owner(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
    children: &'static [SynthesizedAggregateNode],
) -> Result<BoundaryReferentOwner, CraneliftBackendError> {
    let mut escapes = false;
    for child in children {
        if node_referent_owners(plan, seat, *child)?
            .contains(&BoundaryReferentOwner::InvocationArena)
        {
            escapes = true;
        }
    }
    Ok(if escapes {
        BoundaryReferentOwner::InvocationArena
    } else {
        BoundaryReferentOwner::PersistentStore
    })
}
/// [`fixed_node_selected_owner`] for a node rather than a child list.
fn fixed_node_selected_owner_of(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
    node: SynthesizedAggregateNode,
) -> Result<BoundaryReferentOwner, CraneliftBackendError> {
    match node {
        SynthesizedAggregateNode::Fixed { children, .. } => {
            fixed_node_selected_owner(plan, seat, children)
        }
        // A dynamic set nested directly inside a dynamic set is not a shape the
        // measured tree has; it would be an alternative that is itself a
        // choice, with no constructor to allocate.
        other => {
            let _ = other;
            Err(planner_error(
                "a dynamic aggregate alternative is not a constructor, so it allocates nothing",
            ))
        }
    }
}
/// Source-constructor fields whose empty lexical environment is carried into a
/// generated-unit call.
///
/// The key is derived from source structure on both sides: the direct lexical
/// callee fixes the generated-unit boundary, the call argument fixes the result
/// root, and the closed producer analysis fixes each concrete constructor field.
/// No lowering-order ordinal participates.
/// **`RT-CHECKED-IH-CAPTURED-ENV-SCHEMA` tier 2 -- the schema's DOMAIN, stated
/// as its coordinate source rather than as a proxy for it.**
///
/// Membership in the checked-IH captured-env schema IS the existence of the
/// tier-1 ci<->oi coordinate run. This returns that run when it exists and
/// `None` when it does not, so the issuance below admits exactly the units the
/// coordinates admit -- membership equals coordinate-existence by
/// construction, not by a separate filter that could drift from it.
///
/// Two earlier framings of this domain were both wrong the same way, and both
/// cost a hard stop: first "the UnitBoundaryEnvironment population", then
/// "units with captures". Each named a PROXY. A unit may carry captures and
/// still have no ruled ordered-worker envelope -- the continuation can declare
/// fewer ordinary parameters than the worker has captures -- and then no
/// complete run exists, because the run IS the envelope's nonrecursive
/// prefix. Such a unit is OUTSIDE this schema by construction, exactly as it is
/// invisible to tier 1, which also derives its run from that envelope. "Has
/// captures" over-approximates the domain, and that over-approximation was the
/// whole defect.
///
/// `Err` is reserved for a capture that IS in the run but cannot be sourced --
/// a seed capture. That is a refusal, not an absence, and it must not be
/// confused with being out of domain.
impl StaticTransitionPlan<'_> {
    /// The one checked-IH record a fixture plans, for tests that must drive the
    /// lowering-side reconcile arm with real planner coordinates.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn checked_ih_record_for_test(
        &self,
    ) -> Option<(ContinuationEmissionOwner, StaticOriginId, Vec<StaticOriginId>)> {
        self.aggregate_ownership.iter().find_map(|record| {
            let AggregateOccurrenceProducer::SynthesizedUse {
                owner,
                seat,
                role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                ..
            } = record.producer
            else {
                return None;
            };
            Some((
                owner,
                seat,
                record
                    .children
                    .iter()
                    .filter_map(|child| child.origin)
                    .collect(),
            ))
        })
    }

    /// The exact planned aggregate record for a checked-IH captured
    /// environment.
    ///
    /// The full key is the emission owner plus the closure occurrence that
    /// serves as this population's seat. Absence is a refusal: a functional IH
    /// with no issued environment record has no admitted value to carry.
    pub(in crate::cranelift_backend) fn checked_ih_captured_environment_record(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
    ) -> Result<&PlannedAggregateOwnership, CraneliftBackendError> {
        let path = SynthesizedAggregatePath::root(
            SynthesizedAggregateRoot::CheckedIhCapturedEnvironment,
        );
        self.synthesized_aggregate_record(
            owner,
            seat,
            &path,
            SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
        )
        .map_err(|_| {
            let available = self
                .aggregate_ownership
                .iter()
                .filter_map(|record| match &record.producer {
                    AggregateOccurrenceProducer::SynthesizedUse {
                        owner,
                        seat,
                        role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                        ..
                    } => Some((*owner, *seat)),
                    AggregateOccurrenceProducer::Source(_)
                    | AggregateOccurrenceProducer::SynthesizedUse { .. } => None,
                })
                .collect::<Vec<_>>();
            planner_error(format!(
                "no checked-IH captured-environment record exists for owner {owner:?} and \
                 closure seat {seat:?}; available checked-IH records are {available:?}"
            ))
        })
    }

    /// Resolve the compile-time code identity and positional captured
    /// environment for one exact lexical-closure occurrence.
    ///
    /// Absence preserves the ordinary refusal. A present descriptor joins the
    /// source occurrence with the independently issued ownership record before
    /// lowering can replace the closure crossing by an environment crossing.
    pub(in crate::cranelift_backend) fn boundary_closure_environment(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
    ) -> Result<Option<BoundaryClosureEnvironment>, CraneliftBackendError> {
        let mut records = self.aggregate_ownership.iter().filter(|record| {
            matches!(
                &record.producer,
                AggregateOccurrenceProducer::SynthesizedUse {
                    owner: record_owner,
                    seat: record_seat,
                    path,
                    role: SynthesizedAggregateRole::BoundaryClosureEnvironment,
                } if *record_owner == owner
                    && *record_seat == seat
                    && path.root == SynthesizedAggregateRoot::BoundaryClosureEnvironment
                    && path.steps.is_empty()
            )
        });
        let Some(record) = records.next() else {
            return Ok(None);
        };
        if records.next().is_some() {
            return Err(planner_error(
                "two boundary closure environments name one emission owner and source seat",
            ));
        }
        let RuntimeExpr::LexicalClosure {
            captures, params, ..
        } = self.planned_occurrence_expr(seat)?
        else {
            return Err(planner_error(
                "a boundary closure environment record names a non-lexical-closure source seat",
            ));
        };
        let body_origin = self.semantic.child_origin(seat, 0)?;
        let capture_origins = (0..captures.len())
            .map(|ordinal| self.semantic.child_origin(seat, 1 + ordinal))
            .collect::<Result<Vec<_>, _>>()?;
        if record.shape != PlannedAggregateShape::Constructor
            || record.children.len() != capture_origins.len()
            || record
                .children
                .iter()
                .zip(&capture_origins)
                .enumerate()
                .any(|(ordinal, (child, origin))| {
                    child.position as usize != ordinal || child.origin != Some(*origin)
                })
        {
            return Err(planner_error(
                "a boundary closure environment record disagrees with its source capture run",
            ));
        }
        Ok(Some(BoundaryClosureEnvironment {
            owner,
            seat,
            body_origin,
            params: params.clone(),
            capture_origins,
            record: record.id,
        }))
    }

    /// Resolve an environment descriptor only under one of M4's two exact
    /// crossing proofs.
    ///
    /// The original arm is unchanged: the exact owner returns a value graph
    /// structurally containing this closure occurrence. The second arm is the
    /// bind-continuation proof: an exact constructor field declares a recursive
    /// resume, has one static closure-body target, and pairs that field with this
    /// owner's exact positional environment record. Everything else remains on
    /// the ordinary closure refusal route.
    pub(in crate::cranelift_backend) fn boundary_closure_crossing_environment(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
    ) -> Result<Option<BoundaryClosureEnvironment>, CraneliftBackendError> {
        let RuntimeExpr::LexicalClosure { captures, .. } =
            self.planned_occurrence_expr(seat)?
        else {
            return Ok(None);
        };
        // M4's checked continuations carry a real positional environment. A
        // capture-free source closure remains in the generic refusal population
        // guarded by `a_closure_stored_as_constructor_data_cannot_cross_a_unit_boundary`.
        if captures.is_empty() {
            return Ok(None);
        }
        let environment = self.boundary_closure_environment(owner, seat)?;
        // Second, independent arm. It is checked before result containment so a
        // closure at a bind response cannot acquire metadata while bypassing the
        // singleton-target and instance-pairing proof.
        if let Some(environment) = environment.as_ref() {
            if boundary_bind_continuation_is_authorized(self, environment)? {
                return Ok(Some(environment.clone()));
            }
        }
        // Original result-value-containment arm, preserved as its own complete
        // authorization proof.
        if boundary_closure_owner_returns_seat(self, owner, seat)? {
            return Ok(environment);
        }
        Ok(None)
    }

    /// Resolve the descriptor behind one planner-issued positional environment
    /// identity. The record itself supplies owner and seat; no body lookup or
    /// same-shaped search is accepted from lowering.
    pub(in crate::cranelift_backend) fn boundary_closure_environment_by_record(
        &self,
        record: AggregateOccurrenceId,
    ) -> Result<BoundaryClosureEnvironment, CraneliftBackendError> {
        let record_view = self.aggregate_record_view(record)?;
        let AggregateOccurrenceProducer::SynthesizedUse {
            owner,
            seat,
            path,
            role: SynthesizedAggregateRole::BoundaryClosureEnvironment,
        } = record_view.producer()
        else {
            return Err(planner_error(
                "a boundary closure capsule names an aggregate record from another role",
            ));
        };
        if path.root != SynthesizedAggregateRoot::BoundaryClosureEnvironment
            || !path.steps.is_empty()
        {
            return Err(planner_error(
                "a boundary closure capsule names a non-root environment path",
            ));
        }
        let environment = self
            .boundary_closure_environment(*owner, *seat)?
            .ok_or_else(|| {
                planner_error(
                    "a boundary closure capsule's environment record has no descriptor",
                )
            })?;
        if environment.record != record {
            return Err(planner_error(
                "a boundary closure capsule resolved a different environment record",
            ));
        }
        Ok(environment)
    }

    /// Resolve a positional environment only when its exact record also proves
    /// the bind-continuation authorization arm.
    ///
    /// This is consumed at the ordinary one-way carrier producer, where a bind
    /// response crosses outside the generated-unit result route. Returning
    /// `None` preserves the generic closure refusal; there is no record-only or
    /// body-only fallback.
    pub(in crate::cranelift_backend) fn boundary_bind_continuation_environment_by_record(
        &self,
        record: AggregateOccurrenceId,
    ) -> Result<Option<BoundaryClosureEnvironment>, CraneliftBackendError> {
        let environment = self.boundary_closure_environment_by_record(record)?;
        if boundary_bind_continuation_is_authorized(self, &environment)? {
            Ok(Some(environment))
        } else {
            Ok(None)
        }
    }

    /// Resolve a direct predeclared unit result's boundary closure descriptor.
    /// The owner comes from the seat's function membership, never from the
    /// caller or from a body-identity search.
    pub(in crate::cranelift_backend) fn predeclared_boundary_closure_environment(
        &self,
        seat: StaticOriginId,
    ) -> Result<Option<BoundaryClosureEnvironment>, CraneliftBackendError> {
        let Some(owner) = self.semantic.function_owner(seat)? else {
            return Ok(None);
        };
        self.boundary_closure_environment(ContinuationEmissionOwner::Predeclared(owner), seat)
    }

    /// Resolve one exact two-endpoint checked-IH environment transport.
    ///
    /// The destination tuple is the crossing lowering holds; the source
    /// specialization is the opaque continuation-call identity's target. No
    /// seat-only search is expressible through this accessor. `None` means the
    /// claimed continuation result is ordinary and the existing path remains.
    pub(in crate::cranelift_backend) fn checked_ih_environment_transport(
        &self,
        destination_owner: ContinuationEmissionOwner,
        destination_construct_origin: StaticOriginId,
        recursive_position: u32,
        source_specialization: ContinuationSpecializationId,
    ) -> Result<Option<&CheckedIhEnvironmentTransport>, CraneliftBackendError> {
        let source_owner = ContinuationEmissionOwner::Specialization(source_specialization);
        let mut matched = self
            .checked_ih_environment_transports
            .iter()
            .filter(|transport| {
                transport.source_owner == source_owner
                    && transport.source_specialization == source_specialization
                    && transport.source_call_identity.target() == source_specialization
                    && transport.destination_owner == destination_owner
                    && transport.destination_construct_origin == destination_construct_origin
                    && transport.recursive_position == recursive_position
            });
        let Some(transport) = matched.next() else {
            return Ok(None);
        };
        if matched.next().is_some() {
            return Err(planner_error(
                "two checked-IH environment transports name one source and destination edge",
            ));
        }
        let source_record = self
            .aggregate_ownership
            .get(transport.source_record.0 as usize)
            .ok_or_else(|| {
                planner_error("a checked-IH transport names an unknown source record")
            })?;
        if !matches!(
            source_record.producer,
            AggregateOccurrenceProducer::SynthesizedUse {
                owner,
                seat,
                role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                ..
            } if owner == transport.source_owner && seat == transport.seat
        ) || source_record.meet != transport.source_lifetime
            || transport.source_lifetime > transport.destination_lifetime
        {
            return Err(planner_error(
                "a checked-IH transport no longer agrees with its force record or lifetime dominance proof",
            ));
        }
        if !checked_ih_escape_subtree_contains(
            self,
            transport.destination_body_origin,
            transport.source_result_origin,
        )? || !checked_ih_escape_subtree_contains(
            self,
            transport.destination_body_origin,
            transport.destination_construct_origin,
        )? {
            return Err(planner_error(
                "a checked-IH transport destination no longer contains both the source result and crossing constructor",
            ));
        }
        Ok(Some(transport))
    }

    pub(in crate::cranelift_backend) fn checked_ih_environment_transport_source_identities(
        &self,
    ) -> BTreeSet<ContinuationCallIdentity> {
        self.checked_ih_environment_transports
            .iter()
            .map(|transport| transport.source_call_identity.clone())
            .collect()
    }

    pub(in crate::cranelift_backend) fn checked_ih_environment_transport_source(
        &self,
        identity: &ContinuationCallIdentity,
    ) -> Option<&CheckedIhEnvironmentTransport> {
        self.checked_ih_environment_transports
            .iter()
            .find(|transport| &transport.source_call_identity == identity)
    }

    pub(in crate::cranelift_backend) fn checked_ih_environment_transport_for_invocation(
        &self,
        destination_owner: ContinuationEmissionOwner,
        worker_body_origin: Option<StaticOriginId>,
        continuation_origin: StaticOriginId,
        recursive_position: u32,
    ) -> Result<Option<&CheckedIhEnvironmentTransport>, CraneliftBackendError> {
        let mut matched = self
            .checked_ih_environment_transports
            .iter()
            .filter(|transport| {
                transport.destination_owner == destination_owner
                    && worker_body_origin
                        .is_none_or(|body| transport.source_worker_body_origin == body)
                    && transport.source_continuation_origin == continuation_origin
                    && transport.source_recursive_position == recursive_position
            });
        let Some(transport) = matched.next() else {
            return Ok(None);
        };
        if matched.next().is_some() {
            return Err(planner_error(
                "one checked-IH invocation resolves more than one authorized environment transport",
            ));
        }
        Ok(Some(transport))
    }

    /// Resolve one existing captured continuation capability at one exact
    /// descendant checked invocation.
    ///
    /// The source call identity and the full descendant coordinate are the
    /// key. `worker_body_origin` may refine that coordinate, but its absence
    /// never authorizes a candidate or first match: ambiguity still refuses.
    pub(in crate::cranelift_backend) fn checked_ih_continuation_inheritance_for_invocation(
        &self,
        source_call_identity: &ContinuationCallIdentity,
        destination_owner: ContinuationEmissionOwner,
        worker_body_origin: Option<StaticOriginId>,
        continuation_origin: StaticOriginId,
        recursive_position: u32,
    ) -> Result<Option<CheckedIhContinuationInheritanceView<'_>>, CraneliftBackendError> {
        let mut matched = self
            .checked_ih_continuation_inheritances
            .iter()
            .filter(|inheritance| {
                &inheritance.transport.source_call_identity == source_call_identity
                    && inheritance.capability.destination_owner == destination_owner
                    && worker_body_origin
                        .is_none_or(|body| inheritance.capability.destination_body_origin == body)
                    && inheritance
                        .capability
                        .self_resumption_steps
                        .last()
                        .is_some_and(|step| {
                            step.callee_binding
                                == (CheckedIhBinding {
                                    frame_origin: continuation_origin,
                                    recursive_position,
                                })
                        })
            });
        let Some(inheritance) = matched.next() else {
            return Ok(None);
        };
        if matched.next().is_some() {
            return Err(planner_error(
                "one transport/call identity and descendant invocation coordinate resolve more than one continuation inheritance",
            ));
        }
        Ok(Some(CheckedIhContinuationInheritanceView {
            transport: &inheritance.transport,
            capability: &inheritance.capability,
            fresh_result_destination: &inheritance.fresh_result_destination,
        }))
    }

    pub(in crate::cranelift_backend) fn checked_ih_environment_transport_at(
        &self,
        destination_owner: ContinuationEmissionOwner,
        destination_construct_origin: StaticOriginId,
    ) -> Result<Option<&CheckedIhEnvironmentTransport>, CraneliftBackendError> {
        let mut matched = self
            .checked_ih_environment_transports
            .iter()
            .filter(|transport| {
                transport.destination_owner == destination_owner
                    && transport.destination_construct_origin == destination_construct_origin
            });
        let Some(transport) = matched.next() else {
            return Ok(None);
        };
        if matched.next().is_some() {
            return Err(planner_error(
                "one terminal checked-IH crossing has more than one authorized transport; one result cannot substitute two environments",
            ));
        }
        Ok(Some(transport))
    }

    pub(in crate::cranelift_backend) fn checked_ih_environment_transports_owned_by(
        &self,
        destination_owner: ContinuationEmissionOwner,
    ) -> Vec<&CheckedIhEnvironmentTransport> {
        self.checked_ih_environment_transports
            .iter()
            .filter(|transport| transport.destination_owner == destination_owner)
            .collect()
    }

    /// The capture occurrence the ruled run places at `ordinal`, for one
    /// checked-IH captured-environment record.
    ///
    /// ⛔ This is the THIRD party in the reconcile cross-check, and that is the
    /// whole reason it exists. The declared child model says "position i is
    /// capture word i" -- a compile-time positional contract carrying no
    /// occurrence. The emitter states which occurrence it actually put there.
    /// This resolves what the PLAN's ci<->oi run says belongs there. Comparing
    /// the emitter's answer against this one is an actual-versus-declared
    /// check between two independent sources; deriving the expectation from
    /// the emitter's own operand instead would make the comparison free.
    ///
    /// Fails closed: an unknown (owner, seat) or a position the run does not
    /// carry is a refusal, never a default.
    pub(in crate::cranelift_backend) fn checked_ih_capture_origin(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        ordinal: u32,
    ) -> Result<StaticOriginId, CraneliftBackendError> {
        let record = self
            .aggregate_ownership
            .iter()
            .find(|record| {
                matches!(
                    record.producer,
                    AggregateOccurrenceProducer::SynthesizedUse {
                        owner: record_owner,
                        seat: record_seat,
                        role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                        ..
                    } if record_owner == owner && record_seat == seat
                )
            })
            .ok_or_else(|| {
                planner_error(
                    "no checked-IH captured-environment record is planned for this owner and \
                     seat, so a capture operand cannot be reconciled against its ruled run",
                )
            })?;
        let child = record
            .children
            .iter()
            .find(|child| child.position == ordinal)
            .ok_or_else(|| {
                planner_error(format!(
                    "the checked-IH captured environment's ruled run has no capture at \
                     position {ordinal}"
                ))
            })?;
        child.origin.ok_or_else(|| {
            planner_error(
                "a checked-IH capture child carries no source occurrence, so there is nothing \
                 to reconcile the emitter's operand against",
            )
        })
    }

    /// Compare the call assembler's independently projected WorkerCapture
    /// suffix with the force-materialized environment record in their common
    /// `(ordinal, source occurrence)` frame.
    ///
    /// `Ok(false)` is an ordinary continuation outside this population. Once a
    /// record exists, any short, long, moved or reordered suffix refuses.
    pub(in crate::cranelift_backend) fn validate_checked_ih_capture_suffix(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        suffix: &[(u32, StaticOriginId)],
    ) -> Result<bool, CraneliftBackendError> {
        let Some(record) = self.aggregate_ownership.iter().find(|record| {
            matches!(
                record.producer,
                AggregateOccurrenceProducer::SynthesizedUse {
                    owner: record_owner,
                    seat: record_seat,
                    role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                    ..
                } if record_owner == owner && record_seat == seat
            )
        }) else {
            return Ok(false);
        };
        let planned = record
            .children
            .iter()
            .map(|child| {
                child
                    .origin
                    .map(|origin| (child.position, origin))
                    .ok_or_else(|| {
                        planner_error(
                            "a checked-IH environment field has no source occurrence in its positional run",
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        if planned != suffix {
            return Err(planner_error(format!(
                "the checked-IH environment fields are {planned:?}, but the continuation call's independently assembled WorkerCapture suffix is {suffix:?}"
            )));
        }
        Ok(true)
    }
}

/// The largest capture arity the positional child model can state.
///
/// ⛔ This is a REFUSAL bound, not a truncation bound. The const run below is
/// what makes a per-unit arity expressible in a `&'static` slice at all -- the
/// content at position `i` is fully determined by `i`, so one run serves every
/// unit and each takes the prefix its arity names. That trick is sound exactly
/// as long as an arity ABOVE the run REFUSES: silently handing back a shorter
/// prefix would give the emitter a child model describing fewer captures than
/// the record holds, and every count downstream would agree with itself while
/// describing the wrong aggregate.
const CHECKED_IH_CAPTURE_OPERAND_LIMIT: usize = 64;

/// The positional child model, DERIVED rather than hand-listed.
///
/// A hand-written run of 64 entries is a roster that can drift from its own
/// index; this states the rule (`position i names capture word i`) once and
/// lets the compiler produce the members, so the table cannot disagree with
/// itself. `static` rather than `const` on purpose: the slice handed to
/// `declared_children` must be `&'static`, which a `static` item's address
/// gives and a `const`'s per-use temporary does not.
static CHECKED_IH_CAPTURE_OPERANDS: [SynthesizedAggregateNode;
    CHECKED_IH_CAPTURE_OPERAND_LIMIT] = {
    let mut operands =
        [SynthesizedAggregateNode::WorkerCaptureOperand(0); CHECKED_IH_CAPTURE_OPERAND_LIMIT];
    let mut position = 0usize;
    while position < CHECKED_IH_CAPTURE_OPERAND_LIMIT {
        operands[position] = SynthesizedAggregateNode::WorkerCaptureOperand(position as u32);
        position += 1;
    }
    operands
};

/// The declared positional child model for one unit's capture arity.
fn positional_capture_declared_children(
    arity: usize,
) -> Result<&'static [SynthesizedAggregateNode], CraneliftBackendError> {
    if arity > CHECKED_IH_CAPTURE_OPERAND_LIMIT {
        return Err(planner_capacity_error(format!(
            "a captured environment has {arity} captures, more than the \
             {CHECKED_IH_CAPTURE_OPERAND_LIMIT} its positional child model can state -- \
             refusing rather than declaring a shorter run than the record carries"
        )));
    }
    Ok(&CHECKED_IH_CAPTURE_OPERANDS[..arity])
}

fn checked_ih_coordinate_run(
    unit: &super::continuations::ContinuationUnitView<'_>,
) -> Result<Option<(StaticOriginId, Vec<(u32, StaticOriginId)>)>, CraneliftBackendError> {
    // ⛔ `Ok(None)` on this line is EXACTLY ONE condition -- the ruled envelope
    // has no nonrecursive prefix -- and the ENVELOPE AUTHORITY decides it, not a
    // predicate re-derived here. Such a unit has no capture-to-ordinary-parameter
    // correspondence, hence no run, hence no record it could have; it is out of
    // the domain, not skipped.
    //
    // ⛔ What stood here was `let Ok(envelope) = unit.ordinary_envelope() else {
    // return Ok(None) }`, which turned EVERY envelope failure into
    // non-membership -- a malformed envelope on a unit that IS in the domain
    // silently issued nothing. That is fail-open, in exactly the direction this
    // slice exists to close, and it was introduced while implementing the fix
    // FOR fail-open. An armed `EnvelopeDefect::SelectionOutOfRange` produced
    // zero records instead of a refusal.
    let Some(envelope) = unit.ruled_ordinary_envelope()? else {
        return Ok(None);
    };
    let mut closure_origin: Option<StaticOriginId> = None;
    let mut run = Vec::new();
    for role in &envelope {
        let ContinuationOrdinaryEnvelopeRole::WorkerCapture {
            ordinal,
            closure_origin: origin,
            source,
            ..
        } = role
        else {
            continue;
        };
        let ContinuationWorkerCaptureSource::Lexical(sourced) = source else {
            return Err(planner_error(
                "checked-IH captured environment: a capture in the ruled run has no source \
                 occurrence, so its field identity cannot be admitted from the plan",
            ));
        };
        // ⛔ A LAST-WRITE-WINS ASSIGNMENT IS SAFE HERE, AND THE REASON IS
        // UPSTREAM, NOT LOCAL. Every capture in a ruled run carries the same
        // `closure_origin` because `exact_continuation_ordinary_parameters`
        // (`continuations.rs`, the "continuation worker captures are not one
        // exact ordered envelope" refusal) compares EVERY capture's
        // `closure_origin` against the worker's before this run can exist. A
        // disagreeing run is refused there and never reaches this loop.
        //
        // ⛔ A guard was added here for that disagreement and then removed: it
        // was unreachable through the natural producer, so it pinned nothing
        // and read as though this function established a property it merely
        // inherits. Do not re-add it without a production witness that reaches
        // it without going through that validator first -- if you find one, the
        // fix belongs upstream, where the invariant is stated.
        closure_origin = Some(*origin);
        run.push((*ordinal, *sourced));
    }
    // ⛔ No worker captures means no run, hence no membership -- and that is
    // ALL this says. It is NOT a claim that such a unit is served by
    // `UnitBoundaryEnvironment`: that producer additionally requires the
    // concrete `Call` -> empty-capture `LexicalClosure` shape
    // (`unit_boundary_environment_fields` below), which most capture-free units
    // do not have. The two populations are disjoint from this one for different
    // reasons, and conflating them would let "not ours" be read as "theirs".
    match closure_origin {
        Some(origin) if !run.is_empty() => Ok(Some((origin, run))),
        _ => Ok(None),
    }
}

/// The canonical capture run and every exact context that forces its worker.
///
/// A continuation specialization `u` emits its selected case body under
/// `Specialization(u.id())`, and that body binds and may force exactly
/// `u.worker_closure_origin()`. This is the real force edge. It differs from
/// [`inline_synthesized_seat_emission_owners`] is the authority for aggregates
/// emitted inline at their own seat; this environment is emitted at the force
/// seam instead.
///
/// Runs are canonical per closure seat. If two specialization edges select the
/// same closure, their environment records clone that one run; a disagreement
/// refuses rather than letting the owner key hide two definitions of the
/// captured environment.
fn checked_ih_force_emissions(
    plan: &StaticTransitionPlan<'_>,
) -> Result<
    BTreeMap<
        StaticOriginId,
        (
            Vec<(u32, StaticOriginId)>,
            BTreeSet<ContinuationEmissionOwner>,
        ),
    >,
    CraneliftBackendError,
> {
    let mut emissions = BTreeMap::new();
    for unit in plan.continuation_units()? {
        let Some((seat, run)) = checked_ih_coordinate_run(&unit)? else {
            continue;
        };
        let (canonical, owners) = emissions
            .entry(seat)
            .or_insert_with(|| (run.clone(), BTreeSet::new()));
        if *canonical != run {
            return Err(planner_error(
                "two force edges for one checked-IH worker closure disagree on its canonical \
                 capture run",
            ));
        }
        owners.insert(ContinuationEmissionOwner::Specialization(unit.id()));
    }
    Ok(emissions)
}

fn unit_boundary_environment_fields(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeSet<(StaticOriginId, u32)>, CraneliftBackendError> {
    let mut fields = BTreeSet::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Call { args, .. } = occurrence.expr else {
            continue;
        };
        let callee = plan
            .semantic
            .child_origin(occurrence.static_origin, 0)?;
        if !matches!(
            plan.planned_occurrence_expr(callee)?,
            RuntimeExpr::LexicalClosure { .. }
        ) {
            continue;
        }
        for argument_position in 0..args.len() {
            let argument = plan
                .semantic
                .child_origin(occurrence.static_origin, 1 + argument_position)?;
            let mut match_scrutinees = BTreeMap::new();
            let fact = derive_case_producer_fact(
                plan,
                argument,
                &[],
                &mut match_scrutinees,
            )?;
            let CaseProducerSet::Closed(_) = fact.producers else {
                continue;
            };
            for (_, origins) in fact.producer_origins {
                for producer in origins {
                    let RuntimeExpr::Construct { args, .. } =
                        plan.planned_occurrence_expr(producer)?
                    else {
                        return Err(planner_error(
                            "closed constructor-result authority names a \
                             non-Construct producer",
                        ));
                    };
                    for (position, field) in args.iter().enumerate() {
                        if matches!(
                            field,
                            RuntimeExpr::LexicalClosure { captures, .. }
                                if captures.is_empty()
                        ) {
                            fields.insert((
                                producer,
                                u32::try_from(position).map_err(|_| {
                                    planner_capacity_error(
                                        "unit-boundary environment field exceeds the \
                                         position space",
                                    )
                                })?,
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(fields)
}
/// Derive one ownership record for every aggregate producer occurrence.
///
/// ⛔ **The population is every `Construct`/`Record` source occurrence, not the
/// ones some reached trace visited.** A lane chosen from the branch this
/// execution happened to take is exactly the row-driven discovery the frame
/// forbids.
///
/// The synthesized population below adds records for compiler-built trees; it
/// does not remove any source producer from this population.
pub(in crate::cranelift_backend::planning::static_transition) fn build_aggregate_ownership_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<PlannedAggregateOwnership>, CraneliftBackendError> {
    let mut records = Vec::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let shape = match occurrence.expr {
            RuntimeExpr::Construct { .. } => PlannedAggregateShape::Constructor,
            RuntimeExpr::Record { .. } => PlannedAggregateShape::Record,
            _ => continue,
        };
        let origin = occurrence.static_origin;
        let authority = occurrence_authority(plan, origin)?;
        let mut children = Vec::with_capacity(authority.children.len());
        for child in &authority.children {
            let owners = aggregate_child_referent_owners(plan, child)?;
            if owners.is_empty() {
                return Err(planner_error(
                    "aggregate producer child has no derivable referent owner",
                ));
            }
            // ⭐ The RECORD half of the producer schema, issued once beside the
            // ownership record it belongs to. ⛔ Gated on the shape rather than
            // attempted-and-recovered: a `Construct` occurrence has no field
            // names at all, so asking for one and swallowing the failure would
            // make "this producer plans no name here" and "the lookup did not
            // work" the same answer.
            let field_identity = match shape {
                PlannedAggregateShape::Record => Some(
                    plan.record_field_identity(origin, child.position as usize)?,
                ),
                PlannedAggregateShape::Constructor => None,
            };
            children.push(PlannedAggregateChild {
                position: child.position,
                origin: Some(child.origin),
                field_identity,
                lifetime: child.lifetime,
                owners,
            });
        }
        // ⭐ The ruled meet, stated once. "Any invocation-owned ALTERNATIVE"
        // is membership in the possible set, not a proof that the child *is*
        // invocation-owned — an aggregate is only persistable when no child
        // could be shorter-lived than it.
        let escapes = children
            .iter()
            .any(|child| child.owners.contains(&BoundaryReferentOwner::InvocationArena));
        let (meet, allocation) = if escapes {
            (
                PlannedReferentLifetime::ActivationOwned,
                PlannedAggregateAllocation::InvocationAggregate,
            )
        } else {
            (
                PlannedReferentLifetime::Persistent,
                PlannedAggregateAllocation::PersistentGround,
            )
        };
        records.push(PlannedAggregateOwnership {
            // Renumbered below. The identity is the record's index in the
            // sorted population, so it cannot be assigned before the order is
            // final.
            id: AggregateOccurrenceId(0),
            producer: AggregateOccurrenceProducer::Source(origin),
            owner: Some(
                plan.semantic
                    .function_owner(origin)?
                    .ok_or_else(|| planner_error("aggregate producer has no function owner"))?,
            ),
            shape,
            declared_children: None,
            children,
            meet,
            allocation,
        });
    }

    // The synthesized half: ONE record per exact allocation-reachable use in
    // the operation's tree, keyed by WHERE that use sits.
    //
    // The population is (every `Effect` source occurrence) x (its emission
    // owners) x (the allocation-reachable uses its operation's tree flattens
    // to). Two seats using one role get two records, and two uses of one role
    // at one seat -- `ResourceKind` under `ResourceKindMismatch` fields 0 and
    // 1, say -- get two records because their paths differ.
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Effect { operation, .. } = occurrence.expr else {
            continue;
        };
        let seat = occurrence.static_origin;
        for owner in inline_synthesized_seat_emission_owners(plan, seat)? {
            for semantic_use in flatten_allocation_reachable_uses(plan, *operation) {
                let mut children = Vec::with_capacity(semantic_use.children.len());
                for (position, child) in semantic_use.children.iter().enumerate() {
                    // ⛔ No pruning. A child the emission site supplies is
                    // resolved AGAINST that site; a child that cannot be
                    // resolved fails planning. Skipping the use here is what
                    // left four real constructors -- `OptionSome`, `FileError`,
                    // `PrivateBufferSpan`, `ReadSome` -- allocating with no
                    // record.
                    let owners = node_referent_owners(plan, seat, *child)?;
                    children.push(PlannedAggregateChild {
                        position: u32::try_from(position).map_err(|_| {
                            planner_capacity_error(
                                "synthesized aggregate arity exceeds the position space",
                            )
                        })?,
                        // A synthesized child has no source occurrence of its own.
                        origin: None,
                        // Every synthesized aggregate this population reaches is
                        // a constructor node, so there is no field name to plan.
                        field_identity: None,
                        lifetime: if owners.contains(&BoundaryReferentOwner::InvocationArena) {
                            PlannedReferentLifetime::ActivationOwned
                        } else {
                            PlannedReferentLifetime::Persistent
                        },
                        owners,
                    });
                }
                let escapes = children
                    .iter()
                    .any(|child| child.owners.contains(&BoundaryReferentOwner::InvocationArena));
                let (meet, allocation) = if escapes {
                    (
                        PlannedReferentLifetime::ActivationOwned,
                        PlannedAggregateAllocation::InvocationAggregate,
                    )
                } else {
                    (
                        PlannedReferentLifetime::Persistent,
                        PlannedAggregateAllocation::PersistentGround,
                    )
                };
                records.push(PlannedAggregateOwnership {
                    id: AggregateOccurrenceId(0),
                    producer: AggregateOccurrenceProducer::SynthesizedUse {
                        owner,
                        seat,
                        path: semantic_use.path.clone(),
                        role: SynthesizedAggregateRole::Constructor(
                            semantic_use.role,
                        ),
                    },
                    // Provenance only, kept for readers. The emission owner that
                    // confers authority is in the key above.
                    owner: plan.semantic.function_owner(seat)?,
                    shape: PlannedAggregateShape::Constructor,
                    declared_children: Some(semantic_use.children),
                    children,
                    meet,
                    allocation,
                });
            }
        }
    }
    // The unit-boundary environment half. Each record is rooted in one exact
    // source producer and field selected by the closed call-input result
    // analysis above. Empty captures are the bounded first population: the
    // record has no fields, so no compiler-created field-name authority is
    // needed or inferred.
    for (seat, position) in unit_boundary_environment_fields(plan)? {
        for owner in inline_synthesized_seat_emission_owners(plan, seat)? {
            records.push(PlannedAggregateOwnership {
                id: AggregateOccurrenceId(0),
                producer: AggregateOccurrenceProducer::SynthesizedUse {
                    owner,
                    seat,
                    path: SynthesizedAggregatePath::root(
                        SynthesizedAggregateRoot::UnitBoundaryEnvironment,
                    )
                    .field(position),
                    role: SynthesizedAggregateRole::UnitBoundaryEnvironment,
                },
                owner: plan.semantic.function_owner(seat)?,
                shape: PlannedAggregateShape::Record,
                declared_children: Some(&[]),
                children: Vec::new(),
                meet: PlannedReferentLifetime::Persistent,
                allocation: PlannedAggregateAllocation::PersistentGround,
            });
        }
    }
    // The generated-unit boundary closure half. One positional environment
    // record per exact lexical-closure occurrence and emission owner. Issuing
    // the record does not authorize a crossing: lowering consumes it only when
    // that same occurrence is the statically described result or child at a
    // generated-unit edge, and the ordinary Closure refusal remains the
    // fail-closed answer everywhere else.
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::LexicalClosure { captures, .. } = occurrence.expr else {
            continue;
        };
        let seat = occurrence.static_origin;
        let declared = positional_capture_declared_children(captures.len())?;
        let mut children = Vec::with_capacity(captures.len());
        for (ordinal, _) in captures.iter().enumerate() {
            let ordinal = u32::try_from(ordinal).map_err(|_| {
                planner_capacity_error(
                    "a boundary closure capture ordinal exceeds the position space",
                )
            })?;
            let sourced = plan.semantic.child_origin(seat, 1 + ordinal as usize)?;
            let authority = occurrence_authority(plan, sourced)?;
            let child_authority = PlannedOccurrenceChildAuthority {
                origin: sourced,
                position: ordinal,
                owner: authority.owner,
                lifetime: authority.lifetime,
            };
            let owners = aggregate_child_referent_owners(plan, &child_authority)?;
            children.push(PlannedAggregateChild {
                position: ordinal,
                origin: Some(sourced),
                field_identity: None,
                lifetime: if owners.contains(&BoundaryReferentOwner::InvocationArena) {
                    PlannedReferentLifetime::ActivationOwned
                } else {
                    PlannedReferentLifetime::Persistent
                },
                owners,
            });
        }
        let escapes = children
            .iter()
            .any(|child| child.owners.contains(&BoundaryReferentOwner::InvocationArena));
        let (meet, allocation) = if escapes {
            (
                PlannedReferentLifetime::ActivationOwned,
                PlannedAggregateAllocation::InvocationAggregate,
            )
        } else {
            (
                PlannedReferentLifetime::Persistent,
                PlannedAggregateAllocation::PersistentGround,
            )
        };
        for owner in inline_synthesized_seat_emission_owners(plan, seat)? {
            records.push(PlannedAggregateOwnership {
                id: AggregateOccurrenceId(0),
                producer: AggregateOccurrenceProducer::SynthesizedUse {
                    owner,
                    seat,
                    path: SynthesizedAggregatePath::root(
                        SynthesizedAggregateRoot::BoundaryClosureEnvironment,
                    ),
                    role: SynthesizedAggregateRole::BoundaryClosureEnvironment,
                },
                owner: plan.semantic.function_owner(seat)?,
                shape: PlannedAggregateShape::Constructor,
                declared_children: Some(declared),
                children: children.clone(),
                meet,
                allocation,
            });
        }
    }
    // **`RT-CHECKED-IH-CAPTURED-ENV-SCHEMA` tier 2 -- issue for exactly the
    // units the coordinates admit.**
    //
    // Every property is admitted from its authority: the run and its source
    // occurrences from the planner's own `WorkerCapture` roles, each child's
    // lifetime from the occurrence-authority plane, and `meet`/`allocation`
    // from the SAME escape derivation the constructor branch above uses.
    // Nothing is assigned a lifetime here -- a hard-coded answer, even the
    // right one, is the defect this slice exists to remove, and the empty
    // children of the sibling population are why its hard-coded `Persistent`
    // agreed with a derivation run over no evidence.
    for (seat, (run, force_owners)) in checked_ih_force_emissions(plan)? {
        // Refuse an arity the positional model cannot state, BEFORE building
        // anything that would have to agree with it.
        let declared = positional_capture_declared_children(run.len())?;
        let mut children = Vec::with_capacity(run.len());
        for (ordinal, sourced) in &run {
            let authority = occurrence_authority(plan, *sourced)?;
            let child_authority = PlannedOccurrenceChildAuthority {
                origin: *sourced,
                position: *ordinal,
                owner: authority.owner,
                lifetime: authority.lifetime,
            };
            let owners = aggregate_child_referent_owners(plan, &child_authority)?;
            children.push(PlannedAggregateChild {
                position: *ordinal,
                origin: Some(*sourced),
                field_identity: None,
                lifetime: if owners.contains(&BoundaryReferentOwner::InvocationArena) {
                    PlannedReferentLifetime::ActivationOwned
                } else {
                    PlannedReferentLifetime::Persistent
                },
                owners,
            });
        }
        let escapes = children
            .iter()
            .any(|child| child.owners.contains(&BoundaryReferentOwner::InvocationArena));
        let (meet, allocation) = if escapes {
            (
                PlannedReferentLifetime::ActivationOwned,
                PlannedAggregateAllocation::InvocationAggregate,
            )
        } else {
            (
                PlannedReferentLifetime::Persistent,
                PlannedAggregateAllocation::PersistentGround,
            )
        };
        for owner in force_owners {
            records.push(PlannedAggregateOwnership {
                id: AggregateOccurrenceId(0),
                producer: AggregateOccurrenceProducer::SynthesizedUse {
                    owner,
                    seat,
                    path: SynthesizedAggregatePath::root(
                        SynthesizedAggregateRoot::CheckedIhCapturedEnvironment,
                    ),
                    role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                },
                owner: plan.semantic.function_owner(seat)?,
                // ⛔ Constructor, not Record, and the ROLE stays
                // `CheckedIhCapturedEnvironment` -- role is not shape. The
                // shape selects the POSITIONAL downstream path
                // (`Lowered::Constructor`, `record_fields: None`), under which
                // the field-identity preflight does not run and
                // `field_identity: None` is legitimate rather than merely
                // tolerated. A captured environment IS positionally identified:
                // the ci<->oi ordinal is the identity, and M6 consumes it as an
                // ordered projection, not by field name. Keeping the distinct
                // role is what preserves the separation from every other
                // synthesized use.
                shape: PlannedAggregateShape::Constructor,
                declared_children: Some(declared),
                children: children.clone(),
                meet,
                allocation,
            });
        }
    }
    records.sort_by(|left, right| left.producer.cmp(&right.producer));
    for (index, record) in records.iter_mut().enumerate() {
        record.id = AggregateOccurrenceId(u32::try_from(index).map_err(|_| {
            planner_capacity_error("the aggregate occurrence population exceeds the identity space")
        })?);
    }
    Ok(records)
}

/// Whether an occurrence is reachable from one emitted body without entering
/// a nested closure or an ordinary Match arm the closed producer analysis
/// eliminated.
fn checked_ih_escape_subtree_contains(
    plan: &StaticTransitionPlan<'_>,
    root: StaticOriginId,
    needle: StaticOriginId,
) -> Result<bool, CraneliftBackendError> {
    let mut seen = BTreeSet::new();
    let mut pending = vec![root];
    while let Some(origin) = pending.pop() {
        if origin == needle {
            return Ok(true);
        }
        if !seen.insert(origin) {
            continue;
        }
        match plan.planned_occurrence_expr(origin)? {
            RuntimeExpr::Closure { .. } | RuntimeExpr::LexicalClosure { .. } => {}
            RuntimeExpr::Match { cases, .. } => {
                pending.push(plan.semantic.child_origin(origin, 0)?);
                for ordinal in 0..cases.len() {
                    match plan.case_emission_status(origin, ordinal)? {
                        Some(super::CaseEmissionStatus::Reachable) => pending
                            .push(plan.semantic.child_origin(origin, 1 + ordinal)?),
                        Some(super::CaseEmissionStatus::Eliminated) => {}
                        None => {
                            return Err(planner_error(
                                "an ordinary Match in the checked-IH escape certificate has no case-emission verdict",
                            ));
                        }
                    }
                }
            }
            RuntimeExpr::ComputationalMatch { cases, .. } => {
                let scrutinee = plan.semantic.child_origin(origin, 0)?;
                pending.push(scrutinee);
                let mut match_scrutinees = BTreeMap::new();
                let fact = derive_case_producer_fact(
                    plan,
                    scrutinee,
                    &[],
                    &mut match_scrutinees,
                )?;
                for ordinal in 0..cases.len() {
                    let reachable = match &fact.producers {
                        CaseProducerSet::Open => true,
                        CaseProducerSet::Closed(producers) => producers.contains(
                            &plan.case_constructor_identity(origin, ordinal)?,
                        ),
                    };
                    if reachable {
                        pending.push(plan.semantic.child_origin(origin, 1 + ordinal)?);
                    }
                }
            }
            _ => {
                if let Ok(children) = plan.semantic.child_origins(origin) {
                    pending.extend(children.iter().copied());
                }
            }
        }
    }
    Ok(false)
}

/// Derive the exact checked-IH environment transports from the closed
/// continuation-unit occurrence graph.
///
/// A source unit supplies the sole force materialization at
/// `Specialization(source.id())`. A destination exists only when another
/// specialization's worker body contains both that source unit's result and its
/// producer constructor: the result can reach the constructor's recursive
/// position in that emitted body. The two containment checks are the existing
/// closed producer/escape certificate; neither lowering phase nor a reached
/// value participates.
pub(in crate::cranelift_backend::planning::static_transition) fn build_checked_ih_environment_transports(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<CheckedIhEnvironmentTransport>, CraneliftBackendError> {
    let units = plan.continuation_units()?;
    let mut transports = Vec::new();
    for source in &units {
        let Some((seat, _)) = checked_ih_coordinate_run(source)? else {
            continue;
        };
        let source_owner = ContinuationEmissionOwner::Specialization(source.id());
        let source_record = plan
            .aggregate_ownership
            .iter()
            .find(|record| {
                matches!(
                    record.producer,
                    AggregateOccurrenceProducer::SynthesizedUse {
                        owner,
                        seat: record_seat,
                        role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                        ..
                    } if owner == source_owner && record_seat == seat
                )
            })
            .ok_or_else(|| {
                planner_error("a checked-IH transport source has no force-owner environment record")
            })?;
        let parent = source.producer_construct_origin();
        let source_call_identity = plan
            .continuation_call_binding_for(
                parent,
                source.continuation_origin(),
                source.producer_alternative(),
                source.recursive_position(),
            )?
            .ok_or_else(|| {
                planner_error(
                    "a checked-IH transport source has no causal call identity for its own producer edge",
                )
            })?;
        if source_call_identity.target() != source.id() {
            return Err(planner_error(
                "a checked-IH transport's causal identity targets a different specialization than its force materialization",
            ));
        }
        let parent_record = plan
            .aggregate_ownership
            .iter()
            .find(|record| {
                record.producer == AggregateOccurrenceProducer::Source(parent)
                    && record.shape == PlannedAggregateShape::Constructor
            })
            .ok_or_else(|| {
                planner_error(
                    "a checked-IH transport source has no producer-constructor ownership record",
                )
            })?;
        let recursive_position = source.recursive_position();
        let parent_child = parent_record
            .children
            .iter()
            .find(|child| child.position == recursive_position)
            .ok_or_else(|| {
                planner_error(
                    "a checked-IH transport's recursive position is absent from its producer constructor",
                )
            })?;
        if parent_child.origin != Some(seat) {
            return Err(planner_error(
                "a checked-IH transport's stable closure seat is not the source constructor child at its recursive position",
            ));
        }
        if source_record.meet > parent_record.meet {
            return Err(planner_error(
                "a force-materialized checked-IH environment is shorter-lived than the producer crossing it must replace",
            ));
        }

        for destination in &units {
            if destination.id() == source.id() {
                continue;
            }
            let body = destination.worker_body_origin();
            if !checked_ih_escape_subtree_contains(plan, body, parent)?
                || !checked_ih_escape_subtree_contains(
                    plan,
                    body,
                    source.producer_result_origin(),
                )?
            {
                continue;
            }
            let destination_owner = ContinuationEmissionOwner::Specialization(destination.id());
            if plan.aggregate_ownership.iter().any(|record| {
                matches!(
                    record.producer,
                    AggregateOccurrenceProducer::SynthesizedUse {
                        owner,
                        seat: record_seat,
                        role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                        ..
                    } if owner == destination_owner && record_seat == seat
                )
            }) {
                return Err(planner_error(
                    "a checked-IH transport destination also owns an environment record for the transported seat; transport must not become a second emission",
                ));
            }
            let mut continuation_input_morphism = Vec::new();
            let mut seen_indices = BTreeSet::new();
            for (position, input) in source.continuation_inputs()?.into_iter().enumerate() {
                if u32::try_from(position).ok() != Some(input.ordinal) {
                    return Err(planner_error(
                        "a checked-IH transport source's continuation inputs are not in ordinal order",
                    ));
                }
                let destination = match input.availability.direct_emission {
                    Some(ContinuationEnvironmentClaim::CurrentLexical {
                        emission_owner,
                        producer_result_origin,
                        emission_origin,
                        lexical_environment_origin,
                        nearest_alias_index,
                    }) => {
                        if ContinuationEmissionOwner::Predeclared(emission_owner)
                            != source.emission_owner()
                            || producer_result_origin != source.producer_result_origin()
                            || emission_origin != parent
                            || lexical_environment_origin != body
                        {
                            return Err(planner_error(
                                "a checked-IH transport input's current-lexical claim names different source or destination endpoints than the transport edge",
                            ));
                        }
                        CheckedIhTransportInputDestination::LexicalEnvironment(nearest_alias_index)
                    }
                    Some(ContinuationEnvironmentClaim::EntryFrame {
                        frame:
                            ContinuationFrameIdentity::GeneratedContext {
                                specialization,
                                worker_body_origin,
                                ..
                            },
                        declared_slot,
                    }) => {
                        if specialization != destination.id() || worker_body_origin != body {
                            return Err(planner_error(
                                "a checked-IH transport input's entry-frame claim names a different generated destination than the transport edge",
                            ));
                        }
                        CheckedIhTransportInputDestination::EntryFrame(declared_slot)
                    }
                    Some(ContinuationEnvironmentClaim::EntryFrame {
                        frame: ContinuationFrameIdentity::Predeclared(_),
                        ..
                    })
                    | None => {
                        return Err(planner_error(format!(
                            "checked-IH transport source {:?} to destination {:?} input {} at {:?} has no morphism into destination body {body:?}; availability is {:?}",
                            source.id(),
                            destination.id(),
                            input.ordinal,
                            input.coordinate,
                            input.availability,
                        )));
                    }
                };
                if !seen_indices.insert(destination) {
                    return Err(planner_error(
                        "two checked-IH transport inputs map to one destination environment coordinate",
                    ));
                }
                continuation_input_morphism.push((input.ordinal, input.coordinate, destination));
            }
            transports.push(CheckedIhEnvironmentTransport {
                source_owner,
                source_specialization: source.id(),
                source_call_identity: source_call_identity.clone(),
                seat,
                source_result_origin: source.producer_result_origin(),
                source_worker_body_origin: source.worker_body_origin(),
                source_continuation_origin: source.continuation_origin(),
                source_recursive_position: source.recursive_position(),
                destination_owner,
                destination_body_origin: body,
                destination_construct_origin: parent,
                recursive_position,
                source_record: source_record.id,
                source_lifetime: source_record.meet,
                destination_lifetime: parent_record.meet,
                continuation_input_morphism,
            });
        }
    }
    transports.sort();
    if transports.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(planner_error(
            "the checked-IH transport derivation issued one two-endpoint edge twice",
        ));
    }
    Ok(transports)
}

pub(in crate::cranelift_backend::planning::static_transition) fn validate_checked_ih_environment_transports(
    plan: &StaticTransitionPlan<'_>,
    transports: &[CheckedIhEnvironmentTransport],
) -> Result<(), CraneliftBackendError> {
    if transports != build_checked_ih_environment_transports(plan)? {
        return Err(planner_error(
            "checked-IH environment transports are not the exact closed escape derivation",
        ));
    }
    Ok(())
}

fn derive_active_resume_lineage(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    target: StaticOriginId,
    active_lineage: &[StaticOriginId],
    found: &mut Vec<Vec<StaticOriginId>>,
) -> Result<(), CraneliftBackendError> {
    if origin == target {
        found.push(active_lineage.to_vec());
        return Ok(());
    }
    let expr = plan.planned_occurrence_expr(origin)?;
    let children = plan.semantic.child_origins(origin)?.to_vec();
    if let RuntimeExpr::ComputationalMatch { cases, .. } = expr {
        let scrutinee = children
            .first()
            .copied()
            .ok_or_else(|| planner_error("a computational frame has no forward scrutinee child"))?;
        let mut nested = active_lineage.to_vec();
        nested.push(origin);
        derive_active_resume_lineage(plan, scrutinee, target, &nested, found)?;
        if children.len() != cases.len() + 1 {
            return Err(planner_error(
                "a computational frame's forward child run disagrees with its cases",
            ));
        }
        for body in children.iter().skip(1).copied() {
            // The frame itself has been consumed before its selected body runs.
            // A terminal value from that body resumes the already-active outer
            // lineage, not the frame whose case is being evaluated.
            derive_active_resume_lineage(plan, body, target, active_lineage, found)?;
        }
    } else {
        for child in children {
            derive_active_resume_lineage(plan, child, target, active_lineage, found)?;
        }
    }
    Ok(())
}

fn exact_zero_argument_self_resumption(
    plan: &StaticTransitionPlan<'_>,
    case_body: StaticOriginId,
    binding: CheckedIhBinding,
    binder_provenance: &BTreeMap<StaticOriginId, CheckedBinderResolution>,
) -> Result<
    Option<(
        StaticOriginId,
        StaticOriginId,
        StaticOriginId,
        CheckedIhImmediateKBindingLocator,
    )>,
    CraneliftBackendError,
> {
    let mut pending = vec![case_body];
    let mut found = Vec::new();
    while let Some(origin) = pending.pop() {
        if matches!(
            plan.planned_occurrence_expr(origin)?,
            RuntimeExpr::CheckedComputationalIHInvocation { .. }
        ) {
            let call_origin = plan.semantic.child_origin(origin, 0)?;
            if let RuntimeExpr::Call { args, .. } = plan.planned_occurrence_expr(call_origin)? {
                if args.is_empty() {
                    let callee_origin = plan.semantic.child_origin(call_origin, 0)?;
                    if let Some(resolution) = binder_provenance.get(&callee_origin).copied() {
                        if resolution.provenance
                            == CheckedBinderProvenance::InductionHypothesis(binding)
                        {
                            found.push((
                                origin,
                                call_origin,
                                callee_origin,
                                CheckedIhImmediateKBindingLocator {
                                    invocation_origin: origin,
                                    callee_origin,
                                    environment_domain:
                                        CheckedIhKAvailabilityDomain::ImmediateInvocationEnvironment,
                                    environment_index: resolution.immediate_environment_index,
                                },
                            ));
                        }
                    }
                }
            }
        }
        pending.extend(plan.semantic.child_origins(origin)?.iter().rev().copied());
    }
    match found.as_slice() {
        [] => Ok(None),
        [one] => Ok(Some(one.clone())),
        _ => Err(planner_error(
            "one checked recursive case exposes the same zero-argument self-resumption more than once",
        )),
    }
}

fn next_self_resumption_construct(
    plan: &StaticTransitionPlan<'_>,
    case_body: StaticOriginId,
    invocation_origin: StaticOriginId,
    next_active_frame: StaticOriginId,
    recursive_position: usize,
) -> Result<Option<StaticOriginId>, CraneliftBackendError> {
    let RuntimeExpr::ComputationalMatch { cases, .. } =
        plan.planned_occurrence_expr(next_active_frame)?
    else {
        return Err(planner_error(
            "a continuation-inheritance outer frame is not computational",
        ));
    };
    let mut found = Vec::new();
    let mut pending = vec![case_body];
    while let Some(origin) = pending.pop() {
        if let RuntimeExpr::Construct { args, .. } = plan.planned_occurrence_expr(origin)? {
            if args.get(recursive_position).is_some()
                && plan.semantic.child_origin(origin, recursive_position)? == invocation_origin
            {
                let constructor = plan.constructor_symbol_identity(origin)?;
                let selects_recursive_case =
                    cases
                        .iter()
                        .enumerate()
                        .try_fold(false, |selected, (alternative, case)| {
                            Ok::<_, CraneliftBackendError>(
                                selected
                                    || (case.recursive_positions.contains(&recursive_position)
                                        && plan.case_constructor_identity(
                                            next_active_frame,
                                            alternative,
                                        )? == constructor),
                            )
                        })?;
                if selects_recursive_case {
                    found.push(origin);
                }
            }
        }
        pending.extend(plan.semantic.child_origins(origin)?.iter().rev().copied());
    }
    match found.as_slice() {
        [] => Ok(None),
        [origin] => Ok(Some(*origin)),
        _ => Err(planner_error(
            "one self-resumption invocation feeds more than one constructor for the next active frame",
        )),
    }
}

fn fresh_result_destination(
    plan: &StaticTransitionPlan<'_>,
    active_frame: StaticOriginId,
    binder_provenance: &BTreeMap<StaticOriginId, CheckedBinderResolution>,
) -> Result<Option<CheckedIhFreshResultDestination>, CraneliftBackendError> {
    let RuntimeExpr::ComputationalMatch { cases, .. } =
        plan.planned_occurrence_expr(active_frame)?
    else {
        return Err(planner_error(
            "a continuation-inheritance active frame is not computational",
        ));
    };
    let mut destinations = Vec::new();
    for (alternative, case) in cases.iter().enumerate() {
        // A recursive case binds induction hypotheses, not the ordinary fresh
        // result accepted by Ret. We do not identify Ret by spelling or tag;
        // the exact ordinary ConstructorChild -> lexical capture proof below is
        // what distinguishes the destination.
        if !case.recursive_positions.is_empty() {
            continue;
        }
        let layout = CheckedCaseBinderLayout::for_case(case)?;
        let case_body = plan.semantic.child_origin(active_frame, 1 + alternative)?;
        let mut pending = vec![case_body];
        while let Some(origin) = pending.pop() {
            if matches!(
                plan.planned_occurrence_expr(origin)?,
                RuntimeExpr::LexicalClosure { .. }
            ) {
                if let Some(environment) = plan.predeclared_boundary_closure_environment(origin)? {
                    if environment.params().len() != 1 {
                        return Err(planner_error(
                            "a fresh-result destination closure is not the distinct one-parameter ordinary operation",
                        ));
                    }
                    for (capture_ordinal, capture_occurrence) in
                        environment.capture_origins().iter().copied().enumerate()
                    {
                        let Some(
                            provenance @ CheckedBinderProvenance::ConstructorChild {
                                frame_origin,
                                field_position,
                            },
                        ) = binder_provenance
                            .get(&capture_occurrence)
                            .map(|resolution| resolution.provenance)
                        else {
                            continue;
                        };
                        if frame_origin != active_frame {
                            continue;
                        }
                        let binder_index = layout
                            .induction_hypotheses
                            .len()
                            .checked_add(field_position as usize)
                            .ok_or_else(|| {
                                planner_capacity_error(
                                    "fresh-result constructor-child binder index exhausted",
                                )
                            })?;
                        if layout.role_at(binder_index)
                            != (CheckedCaseBinderRole::ConstructorChild { field_position })
                        {
                            return Err(planner_error(
                                "a fresh-result capture's binder is not the exact ordinary ConstructorChild issued by the case layout",
                            ));
                        }
                        let capture_ordinal = u32::try_from(capture_ordinal).map_err(|_| {
                            planner_capacity_error("fresh-result lexical capture ordinal exhausted")
                        })?;
                        let mut body_capture_reads = binder_provenance
                            .iter()
                            .filter_map(|(read_origin, held)| {
                                (held.provenance
                                    == CheckedBinderProvenance::LexicalClosureCapture {
                                        closure_origin: origin,
                                        capture_ordinal,
                                        source_origin: capture_occurrence,
                                    })
                                .then_some(*read_origin)
                            })
                            .collect::<Vec<_>>();
                        body_capture_reads.sort_unstable();
                        if body_capture_reads.is_empty() {
                            // A descriptor whose captured constructor child is
                            // never read is the positive descriptor-only
                            // exclusion, not a destination candidate.
                            #[cfg(feature = "px8-ds-test-support")]
                            if CONTINUATION_INHERITANCE_OBSERVATION_ACTIVE.with(Cell::get) {
                                CONTINUATION_INHERITANCE_DESCRIPTOR_ONLY_EXCLUSIONS
                                    .with(|count| count.set(count.get() + 1));
                            }
                            continue;
                        }
                        destinations.push(CheckedIhFreshResultDestination {
                            active_frame_origin: active_frame,
                            ret_case_body_origin: case_body,
                            constructor_child: provenance,
                            closure_environment_record: environment.record(),
                            closure_origin: environment.seat(),
                            closure_body_origin: environment.body_origin(),
                            closure_parameter_count: u32::try_from(environment.params().len())
                                .map_err(|_| {
                                    planner_capacity_error(
                                        "fresh-result closure parameter count exhausted",
                                    )
                                })?,
                            capture_ordinal,
                            capture_occurrence,
                            body_capture_reads,
                        });
                    }
                }
            }
            pending.extend(plan.semantic.child_origins(origin)?.iter().rev().copied());
        }
    }
    match destinations.as_slice() {
        [] => Ok(None),
        [destination] => Ok(Some(destination.clone())),
        _ => Err(planner_error(
            "one inherited continuation invocation has more than one ordinary fresh-result capture destination",
        )),
    }
}

fn validate_fresh_result_disjointness(
    transport: &CheckedIhEnvironmentTransport,
    destination: &CheckedIhFreshResultDestination,
) -> Result<(), CraneliftBackendError> {
    let earlier_result = transport.source_result_origin;
    let mut destination_origins = vec![
        destination.active_frame_origin,
        destination.ret_case_body_origin,
        destination.closure_origin,
        destination.closure_body_origin,
        destination.capture_occurrence,
    ];
    destination_origins.extend(destination.body_capture_reads.iter().copied());
    if destination_origins.contains(&earlier_result) {
        return Err(planner_error(
            "an earlier transport source result was substituted into the fresh-result destination",
        ));
    }
    Ok(())
}

fn derive_checked_ih_continuation_inheritance(
    plan: &StaticTransitionPlan<'_>,
    transport: &CheckedIhEnvironmentTransport,
    binder_provenance: &BTreeMap<StaticOriginId, CheckedBinderResolution>,
) -> Result<Option<CheckedIhContinuationInheritance>, CraneliftBackendError> {
    let ContinuationEmissionOwner::Specialization(destination_specialization) =
        transport.destination_owner
    else {
        return Ok(None);
    };
    let destination_unit = plan
        .continuation_units()?
        .into_iter()
        .find(|unit| unit.id() == destination_specialization)
        .ok_or_else(|| {
            planner_error(
                "a checked-IH transport destination owner has no continuation specialization",
            )
        })?;
    if destination_unit.worker_body_origin() != transport.destination_body_origin {
        return Err(planner_error(
            "a checked-IH transport destination body disagrees with its specialization owner",
        ));
    }

    let seed = [destination_unit.continuation_origin()];
    let mut lineages = Vec::new();
    derive_active_resume_lineage(
        plan,
        transport.destination_body_origin,
        transport.destination_construct_origin,
        &seed,
        &mut lineages,
    )?;
    let [initial_active_lineage] = lineages.as_slice() else {
        if lineages.is_empty() {
            return Ok(None);
        }
        return Err(planner_error(
            "one transport destination construct has more than one forward active-frame lineage",
        ));
    };
    let recursive_position = usize::try_from(transport.recursive_position)
        .map_err(|_| planner_capacity_error("checked-IH transport recursive position exhausted"))?;
    let mut active_lineage = initial_active_lineage.clone();
    let mut construct_origin = transport.destination_construct_origin;
    let mut steps = Vec::new();
    let bound = plan
        .source_occurrences
        .len()
        .checked_add(1)
        .ok_or_else(|| planner_capacity_error("checked-IH inheritance depth exhausted"))?;

    while steps.len() < bound {
        let Some(active_frame) = active_lineage.last().copied() else {
            return Ok(None);
        };
        let RuntimeExpr::Construct { args, .. } = plan.planned_occurrence_expr(construct_origin)?
        else {
            return Err(planner_error(
                "a checked-IH inheritance step does not begin at a constructor",
            ));
        };
        if args.get(recursive_position).is_none() {
            return Err(planner_error(
                "a checked-IH inheritance step has no declared recursive child",
            ));
        }
        let recursive_child_origin = plan
            .semantic
            .child_origin(construct_origin, recursive_position)?;
        if steps.is_empty() && recursive_child_origin != transport.seat {
            return Err(planner_error(
                "a checked-IH transport destination field no longer contains its exact captured continuation seat",
            ));
        }

        let RuntimeExpr::ComputationalMatch { cases, .. } =
            plan.planned_occurrence_expr(active_frame)?
        else {
            return Err(planner_error(
                "a checked-IH inheritance step resumes a non-computational active frame",
            ));
        };
        let constructor = plan.constructor_symbol_identity(construct_origin)?;
        let mut selected = Vec::new();
        for (alternative, case) in cases.iter().enumerate() {
            if plan.case_constructor_identity(active_frame, alternative)? == constructor
                && case.recursive_positions.contains(&recursive_position)
            {
                selected.push((alternative, case));
            }
        }
        let [(alternative, _selected_case)] = selected.as_slice() else {
            if selected.is_empty() {
                return Ok(None);
            }
            return Err(planner_error(
                "one inherited constructor selects more than one recursive active-frame case",
            ));
        };
        let selected_case_body_origin =
            plan.semantic.child_origin(active_frame, 1 + alternative)?;
        let binding = CheckedIhBinding {
            frame_origin: active_frame,
            recursive_position: transport.recursive_position,
        };
        let Some((invocation_origin, call_origin, callee_origin, immediate_k_locator)) =
            exact_zero_argument_self_resumption(
                plan,
                selected_case_body_origin,
                binding,
                binder_provenance,
            )?
        else {
            return Ok(None);
        };
        steps.push(CheckedIhSelfResumptionStep {
            construct_origin,
            active_frame_origin: active_frame,
            recursive_child_origin,
            selected_case_body_origin,
            invocation_origin,
            call_origin,
            callee_origin,
            callee_binding: binding,
            immediate_k_locators: vec![immediate_k_locator],
        });

        if let Some(fresh_result_destination) =
            fresh_result_destination(plan, active_frame, binder_provenance)?
        {
            validate_fresh_result_disjointness(transport, &fresh_result_destination)?;
            return Ok(Some(CheckedIhContinuationInheritance {
                transport: transport.clone(),
                capability: CheckedIhCapabilityInheritance {
                    destination_owner: transport.destination_owner,
                    destination_body_origin: transport.destination_body_origin,
                    self_resumption_steps: steps,
                },
                fresh_result_destination,
            }));
        }

        active_lineage.pop();
        let Some(next_active_frame) = active_lineage.last().copied() else {
            return Ok(None);
        };
        let Some(next_construct) = next_self_resumption_construct(
            plan,
            selected_case_body_origin,
            invocation_origin,
            next_active_frame,
            recursive_position,
        )?
        else {
            return Ok(None);
        };
        construct_origin = next_construct;
    }
    Err(planner_error(
        "checked-IH continuation inheritance exceeded the finite source-occurrence depth bound",
    ))
}

pub(in crate::cranelift_backend::planning::static_transition) fn build_checked_ih_continuation_inheritances(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<CheckedIhContinuationInheritance>, CraneliftBackendError> {
    let binder_provenance = build_checked_binder_provenance(plan)?;
    let mut inheritances = Vec::new();
    for transport in &plan.checked_ih_environment_transports {
        if let Some(inheritance) =
            derive_checked_ih_continuation_inheritance(plan, transport, &binder_provenance)?
        {
            inheritances.push(inheritance);
        }
    }
    let mut keys = BTreeSet::new();
    for inheritance in &inheritances {
        let final_step = inheritance
            .capability
            .self_resumption_steps
            .last()
            .ok_or_else(|| {
                planner_error("an inherited continuation capability has no self-resumption step")
            })?;
        let [_final_locator] = final_step.immediate_k_locators.as_slice() else {
            return Err(planner_error(
                "one inherited continuation step does not have exactly one immediate K locator",
            ));
        };
        let key = (
            inheritance.transport.source_call_identity.clone(),
            inheritance.capability.destination_owner,
            inheritance.capability.destination_body_origin,
            final_step.callee_binding,
        );
        if !keys.insert(key) {
            return Err(planner_error(
                "one inherited continuation capability was issued twice for the same descendant invocation",
            ));
        }
    }
    Ok(inheritances)
}

pub(in crate::cranelift_backend::planning::static_transition) fn validate_checked_ih_continuation_inheritances(
    plan: &StaticTransitionPlan<'_>,
    inheritances: &[CheckedIhContinuationInheritance],
) -> Result<(), CraneliftBackendError> {
    #[cfg(feature = "px8-ds-test-support")]
    if inheritances.is_empty()
        && CONTINUATION_INHERITANCE_MUTATION.with(Cell::get)
            == CheckedIhContinuationInheritanceMutation::SuppressForInertness
    {
        return Ok(());
    }
    let binder_provenance = build_checked_binder_provenance(plan)?;
    let mut keys = BTreeSet::new();
    for inheritance in inheritances {
        let mut locator_keys = BTreeSet::new();
        if !plan
            .checked_ih_environment_transports
            .contains(&inheritance.transport)
        {
            return Err(planner_error(
                "a continuation-inheritance projection does not reference one exact existing transport endpoint",
            ));
        }
        validate_fresh_result_disjointness(
            &inheritance.transport,
            &inheritance.fresh_result_destination,
        )?;
        let [first_step, ..] = inheritance.capability.self_resumption_steps.as_slice() else {
            return Err(planner_error(
                "an inherited continuation capability has no self-resumption step",
            ));
        };
        if first_step.construct_origin != inheritance.transport.destination_construct_origin
            || first_step.recursive_child_origin != inheritance.transport.seat
        {
            return Err(planner_error(
                "the first continuation-inheritance step is not the transport's exact destination field",
            ));
        }
        for pair in inheritance.capability.self_resumption_steps.windows(2) {
            if pair[1].recursive_child_origin != pair[0].invocation_origin {
                return Err(planner_error(
                    "one continuation-inheritance self-resumption step is disconnected from its predecessor invocation",
                ));
            }
        }
        for step in &inheritance.capability.self_resumption_steps {
            if step.active_frame_origin != step.callee_binding.frame_origin
                || step.callee_binding.recursive_position
                    != inheritance.transport.recursive_position
            {
                return Err(planner_error(
                    "one continuation-inheritance step disagrees with its exact checked invocation binding",
                ));
            }
            let [locator] = step.immediate_k_locators.as_slice() else {
                return Err(planner_error(
                    "one inherited continuation step does not have exactly one immediate K locator",
                ));
            };
            if locator.environment_domain
                != CheckedIhKAvailabilityDomain::ImmediateInvocationEnvironment
            {
                return Err(planner_error(
                    "an immediate K locator names the wrong runtime environment domain",
                ));
            }
            if locator.invocation_origin != step.invocation_origin
                || locator.callee_origin != step.callee_origin
            {
                return Err(planner_error(
                    "an immediate K locator names a different descendant invocation or callee",
                ));
            }
            let Some((derived_invocation, derived_call, derived_callee, derived_locator)) =
                exact_zero_argument_self_resumption(
                    plan,
                    step.selected_case_body_origin,
                    step.callee_binding,
                    &binder_provenance,
                )?
            else {
                return Err(planner_error(
                    "an inherited continuation step has no forward-derived immediate K locator",
                ));
            };
            if (step.invocation_origin, step.call_origin, step.callee_origin)
                != (derived_invocation, derived_call, derived_callee)
                || locator != &derived_locator
            {
                return Err(planner_error(
                    "an immediate K locator does not equal its forward binder re-derivation",
                ));
            }
            if !locator_keys.insert(locator.clone()) {
                return Err(planner_error(
                    "one immediate K locator was issued more than once",
                ));
            }
        }
        let Some(final_step) = inheritance.capability.self_resumption_steps.last() else {
            return Err(planner_error(
                "an inherited continuation capability has no self-resumption step",
            ));
        };
        let [final_locator] = final_step.immediate_k_locators.as_slice() else {
            return Err(planner_error(
                "one inherited continuation step does not have exactly one immediate K locator",
            ));
        };
        if inheritance.capability.immediate_k_locator() != Some(final_locator) {
            return Err(planner_error(
                "the final inherited capability view does not expose its exact immediate K locator",
            ));
        }
        let key = (
            inheritance.transport.source_call_identity.clone(),
            inheritance.capability.destination_owner,
            inheritance.capability.destination_body_origin,
            final_step.callee_binding,
        );
        if !keys.insert(key) {
            return Err(planner_error(
                "one inherited continuation capability was issued twice for the same descendant invocation",
            ));
        }
        let last_frame = final_step.active_frame_origin;
        if last_frame != final_step.callee_binding.frame_origin
            || inheritance.fresh_result_destination.active_frame_origin != last_frame
        {
            return Err(planner_error(
                "the inherited capability and fresh-result destination expose different active frames",
            ));
        }
        for worker_body_origin in [None, Some(inheritance.capability.destination_body_origin)] {
            let view = plan
                .checked_ih_continuation_inheritance_for_invocation(
                    &inheritance.transport.source_call_identity,
                    inheritance.capability.destination_owner,
                    worker_body_origin,
                    final_step.callee_binding.frame_origin,
                    final_step.callee_binding.recursive_position,
                )?
                .ok_or_else(|| {
                    planner_error(
                        "an issued continuation inheritance is absent from its own exact accessor",
                    )
                })?;
            if view.transport() != &inheritance.transport
                || view.capability() != &inheritance.capability
                || view.fresh_result_destination() != &inheritance.fresh_result_destination
            {
                return Err(planner_error(
                    "the continuation-inheritance accessor returned a different split proof",
                ));
            }
        }
        let destination_environment = plan.boundary_closure_environment_by_record(
            inheritance
                .fresh_result_destination
                .closure_environment_record(),
        )?;
        let destination_capture = destination_environment
            .capture_origins()
            .get(inheritance.fresh_result_destination.capture_ordinal as usize)
            .copied();
        if destination_environment.seat() != inheritance.fresh_result_destination.closure_origin
            || destination_environment.body_origin()
                != inheritance.fresh_result_destination.closure_body_origin
            || u32::try_from(destination_environment.params().len()).ok()
                != Some(inheritance.fresh_result_destination.closure_parameter_count)
            || destination_capture != Some(inheritance.fresh_result_destination.capture_occurrence)
        {
            return Err(planner_error(
                "the fresh-result destination does not reference its exact existing boundary-closure environment record",
            ));
        }
        match inheritance.fresh_result_destination.constructor_child {
            CheckedBinderProvenance::ConstructorChild { frame_origin, .. }
                if frame_origin == last_frame => {}
            CheckedBinderProvenance::InductionHypothesis(_) => {
                return Err(planner_error(
                    "the ordinary Ret constructor child was reclassified as an induction hypothesis",
                ));
            }
            CheckedBinderProvenance::ConstructorChild { .. }
            | CheckedBinderProvenance::LexicalClosureParameter { .. }
            | CheckedBinderProvenance::LexicalClosureCapture { .. }
            | CheckedBinderProvenance::Ordinary => {
                return Err(planner_error(
                    "a descriptor-only closure was substituted for the proven ordinary Ret ConstructorChild destination",
                ));
            }
        }
    }
    if inheritances != build_checked_ih_continuation_inheritances(plan)? {
        return Err(planner_error(
            "checked-IH continuation inheritances are not the exact closed forward derivation",
        ));
    }
    Ok(())
}

/// One static target admitted at a bind-resume site.
///
/// `environment_record = None` is intentionally expressible for the exactness
/// validator's non-paired control. Production derives `Some` only from an exact
/// owner-and-seat environment descriptor; it never fills a missing pairing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BoundaryBindTargetProof {
    body_origin: StaticOriginId,
    environment_record: Option<AggregateOccurrenceId>,
}

/// The closed authorization law for a capture-only bind continuation.
///
/// A singleton body is what permits code identity to remain compile-time. A
/// present environment record is what pairs the response with the captures of
/// its own dynamic closure instance. Either missing fact keeps the generic
/// closure refusal live.
fn boundary_bind_targets_are_exact(
    targets: &[BoundaryBindTargetProof],
    expected_body: StaticOriginId,
) -> bool {
    matches!(
        targets,
        [BoundaryBindTargetProof {
            body_origin,
            environment_record: Some(_),
        }] if *body_origin == expected_body
    )
}

/// Prove the second M4 crossing arm for one exact bind-continuation edge.
///
/// The resume site is the constructor field which directly contains `seat`.
/// Its constructor must be consumed as a recursive position by a planned
/// computational eliminator. The static-body plane must then contain exactly
/// one lexical unit and one call edge for the closure body. Finally the source
/// aggregate's exact field must point at this closure occurrence and the
/// owner-specific positional environment record must fit that field's lifetime.
/// Together those facts state both `Targets(resume) = {body}` and that every
/// dynamic construction places its own environment word in the response field.
fn boundary_bind_continuation_is_authorized(
    plan: &StaticTransitionPlan<'_>,
    environment: &BoundaryClosureEnvironment,
) -> Result<bool, CraneliftBackendError> {
    let seat = environment.seat();
    let mut parents = Vec::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        for (position, child) in plan
            .semantic
            .child_origins(occurrence.static_origin)?
            .iter()
            .copied()
            .enumerate()
        {
            if child == seat {
                parents.push((occurrence.static_origin, position));
            }
        }
    }
    let [(resume_site, position)] = parents.as_slice() else {
        return Ok(false);
    };
    let RuntimeExpr::Construct { args, .. } = plan.planned_occurrence_expr(*resume_site)? else {
        return Ok(false);
    };
    if args.get(*position).is_none() {
        return Ok(false);
    }

    // The field is a bind/resume position only when the planner's computational
    // case metadata declares this constructor position recursive. This is a
    // structural identity join, never an `ITree::Vis` spelling check.
    let resume_constructor = plan.constructor_symbol_identity(*resume_site)?;
    let mut recursive_position_declared = false;
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::ComputationalMatch { cases, .. } = occurrence.expr else {
            continue;
        };
        for (case_index, case) in cases.iter().enumerate() {
            if case.recursive_positions.contains(position)
                && plan.case_constructor_identity(occurrence.static_origin, case_index)?
                    == resume_constructor
            {
                recursive_position_declared = true;
            }
        }
    }
    if !recursive_position_declared {
        return Ok(false);
    }

    // The per-response pairing is derived independently from the source
    // constructor record and the environment record. It is not filled merely
    // because the expected body is known.
    let mut parent_records = plan.aggregate_ownership.iter().filter(|record| {
        record.producer == AggregateOccurrenceProducer::Source(*resume_site)
    });
    let Some(parent_record) = parent_records.next() else {
        return Ok(false);
    };
    if parent_records.next().is_some()
        || parent_record.shape != PlannedAggregateShape::Constructor
        || parent_record.children.len() != args.len()
    {
        return Ok(false);
    }
    let mut paired_fields = parent_record
        .children
        .iter()
        .filter(|child| child.position as usize == *position && child.origin == Some(seat));
    let Some(paired_field) = paired_fields.next() else {
        return Ok(false);
    };
    if paired_fields.next().is_some() {
        return Ok(false);
    }
    let Some(environment_record) = plan.aggregate_ownership.get(environment.record().0 as usize)
    else {
        return Ok(false);
    };
    let response_is_paired = matches!(
        &environment_record.producer,
        AggregateOccurrenceProducer::SynthesizedUse {
            owner,
            seat: record_seat,
            role: SynthesizedAggregateRole::BoundaryClosureEnvironment,
            ..
        } if *owner == environment.owner() && *record_seat == seat
    ) && environment_record.meet <= paired_field.lifetime;

    // Targets come from the ABI's independently issued closure-body units for
    // the defining occurrence. The environment descriptor supplies only the
    // expected body used to validate that set; it does not populate the set.
    let emitted_units = plan.emittable_units()?;
    let target_units = emitted_units
        .iter()
        .copied()
        .filter(|unit| {
            matches!(
                unit.definition(),
                AbiUnitDefinition::ClosureBody {
                    defining_origin,
                    provenance: AbiCaptureProvenance::Lexical,
                } if defining_origin == seat
            )
        })
        .collect::<Vec<_>>();
    let targets = target_units
        .iter()
        .map(|unit| BoundaryBindTargetProof {
            body_origin: unit.body_occurrence(),
            environment_record: response_is_paired.then_some(environment.record()),
        })
        .collect::<Vec<_>>();
    if !boundary_bind_targets_are_exact(&targets, environment.body_origin()) {
        return Ok(false);
    }
    let target_unit = target_units[0];
    if emitted_units
        .iter()
        .filter(|unit| unit.body_occurrence() == environment.body_origin())
        .count()
        != 1
    {
        return Ok(false);
    }
    let call_edges = plan.emittable_call_edges()?;
    if call_edges
        .iter()
        .filter(|edge| {
            edge.kind() == EmittableCallKind::StaticBody
                && edge.callee() == target_unit.function()
                && edge.callee_origin() == target_unit.entry_origin()
        })
        .count()
        != 1
    {
        return Ok(false);
    }

    // Both values are emitted in the same owner. Otherwise a source occurrence
    // could borrow another specialization's environment record.
    if !inline_synthesized_seat_emission_owners(plan, seat)?
        .contains(&environment.owner())
        || !inline_synthesized_seat_emission_owners(plan, *resume_site)?
            .contains(&environment.owner())
    {
        return Ok(false);
    }
    Ok(true)
}

/// Every record names a DISTINCT producer.
///
/// This is the non-aliasing law of the occurrence domain, and it is production
/// code rather than a test because it is what makes an identity an identity: if
/// two records shared a producer, one seat's record could authorize another
/// seat's allocation and the lane chosen for one node would govern a different
/// one. Two uses of a role at two seats must be two occurrences; two records for
/// ONE use is the same failure seen from the other side.
/// Whether one lexical closure occurrence is structurally contained in a
/// value returned by the exact emitted owner.
///
/// This is the authorization boundary for `BoundaryClosureEnvironment` records.
/// It follows result flow through control wrappers and descends only through
/// value-container fields. A closure merely present in an emitted body is not a
/// boundary value and receives no record.
fn boundary_result_value_contains_closure(
    plan: &StaticTransitionPlan<'_>,
    root: StaticOriginId,
    closure: StaticOriginId,
) -> Result<bool, CraneliftBackendError> {
    let mut pending = vec![root];
    let mut seen = BTreeSet::new();
    while let Some(origin) = pending.pop() {
        if !seen.insert(origin) {
            continue;
        }
        let expr = plan.planned_occurrence_expr(origin)?;
        let child = |position| plan.semantic.child_origin(origin, position);
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
                for ordinal in 0..cases.len() {
                    pending.push(child(1 + ordinal)?);
                }
            }
            RuntimeExpr::ComputationalMatch { cases, .. } => {
                for ordinal in 0..cases.len() {
                    pending.push(child(1 + ordinal)?);
                }
            }
            RuntimeExpr::Construct { args, .. } => {
                for ordinal in 0..args.len() {
                    pending.push(child(ordinal)?);
                }
            }
            RuntimeExpr::Record { fields } => {
                for ordinal in 0..fields.len() {
                    pending.push(child(ordinal)?);
                }
            }
            RuntimeExpr::LexicalClosure { captures, .. } => {
                if origin == closure {
                    return Ok(true);
                }
                // The body is code, not a value field. Captures begin at child 1.
                for ordinal in 0..captures.len() {
                    pending.push(child(1 + ordinal)?);
                }
            }
            RuntimeExpr::Value(_)
            | RuntimeExpr::Var(_)
            | RuntimeExpr::PrimitiveCall { .. }
            | RuntimeExpr::Project { .. }
            | RuntimeExpr::Closure { .. }
            | RuntimeExpr::DeclarationRef { .. }
            | RuntimeExpr::ImportedDeclarationRef { .. }
            | RuntimeExpr::Call { .. }
            | RuntimeExpr::Effect { .. }
            | RuntimeExpr::Trap(_) => {}
        }
    }
    Ok(false)
}

/// Does this exact emitted owner return a value graph containing `closure`?
///
/// The candidate owner population is independently derived by
/// `inline_synthesized_seat_emission_owners`; this second predicate narrows it
/// from body containment to boundary-result containment.
fn boundary_closure_owner_returns_seat(
    plan: &StaticTransitionPlan<'_>,
    owner: ContinuationEmissionOwner,
    closure: StaticOriginId,
) -> Result<bool, CraneliftBackendError> {
    let mut body_roots = Vec::new();
    match owner {
        ContinuationEmissionOwner::Predeclared(function) => {
            body_roots.extend(
                plan.emittable_units()?
                    .into_iter()
                    .filter(|unit| unit.function() == function)
                    .map(|unit| unit.body_occurrence()),
            );
        }
        ContinuationEmissionOwner::Specialization(specialization) => {
            body_roots.extend(
                plan.continuation_units()?
                    .into_iter()
                    .filter(|unit| unit.id() == specialization)
                    .map(|unit| unit.worker_body_origin()),
            );
        }
        ContinuationEmissionOwner::Fusion(_) => {
            // The inline-owner derivation does not issue Fusion candidates.
            // Keep this explicit so widening that producer is a reviewed choice.
            return Ok(false);
        }
    }
    for body in body_roots {
        for result in plan.source_result_origins_in_owner_subtree(body)? {
            if boundary_result_value_contains_closure(plan, result, closure)? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub(in crate::cranelift_backend::planning::static_transition) fn validate_aggregate_producers_are_unique(
    records: &[PlannedAggregateOwnership],
) -> Result<(), CraneliftBackendError> {
    let mut seen = BTreeSet::new();
    for record in records {
        if !seen.insert(record.producer.clone()) {
            return Err(planner_error(
                "two aggregate ownership records name the same producer, so an occurrence \
                 identity is not unique",
            ));
        }
    }
    Ok(())
}
pub(in crate::cranelift_backend::planning::static_transition) fn validate_aggregate_ownership_plan(
    plan: &StaticTransitionPlan<'_>,
    records: &[PlannedAggregateOwnership],
) -> Result<(), CraneliftBackendError> {
    if records != build_aggregate_ownership_plan(plan)? {
        return Err(planner_error(
            "aggregate ownership is not the exact closed lifetime-meet derivation",
        ));
    }
    validate_aggregate_producers_are_unique(records)?;
    // ⛔ A second, independent check on the same records, because the
    // re-derivation above only proves the builder agrees with itself. This one
    // states the PROPERTY: the persistent lane is issued only where no child
    // has an invocation-owned alternative.
    // The identity is only opaque to its consumers if it is exact here: a
    // record whose id is not its own index would resolve to a *different*
    // record's lane, which is the one failure this domain has to be incapable
    // of. Stated as its own law rather than left to the rebuild comparison
    // above, which would agree with a builder that numbered every record zero.
    for (index, record) in records.iter().enumerate() {
        if record.id.0 as usize != index {
            return Err(planner_error(
                "aggregate occurrence identities are not the dense index of their own population",
            ));
        }
    }
    for record in records {
        let escapes = record
            .children
            .iter()
            .any(|child| child.owners.contains(&BoundaryReferentOwner::InvocationArena));
        let expected = if escapes {
            PlannedAggregateAllocation::InvocationAggregate
        } else {
            PlannedAggregateAllocation::PersistentGround
        };
        if record.allocation != expected {
            return Err(planner_error(
                "aggregate allocation lane disagrees with its own children's owner sets",
            ));
        }
    }
    Ok(())
}
pub(in crate::cranelift_backend::planning::static_transition) fn lifetime_referent_affinity(
    lifetime: PlannedReferentLifetime,
) -> Vec<BoundaryReferentOwner> {
    match lifetime {
        PlannedReferentLifetime::Persistent => vec![
            BoundaryReferentOwner::NoReferent,
            BoundaryReferentOwner::PersistentStore,
        ],
        PlannedReferentLifetime::ActivationOwned => vec![
            BoundaryReferentOwner::NoReferent,
            BoundaryReferentOwner::PersistentStore,
            BoundaryReferentOwner::InvocationArena,
        ],
    }
}

impl<'src> StaticTransitionPlan<'src> {
    /// **`D7` — the ruled allocation lane for one aggregate producer.**
    ///
    /// ⛔ **Absence is a loud failure, never a default.** An aggregate the
    /// planner never issued a record for is one whose lifetime meet was never
    /// taken, and answering `PersistentGround` for it would reinstate exactly
    /// the unproven persistent lane this record exists to replace — silently,
    /// and only for the occurrences the population happened to miss.
    ///
    /// ⚠ The `shape` argument is a cross-check, not a lookup key. The caller
    /// knows which aggregate it is emitting; if that disagrees with the record
    /// at this origin, one of the two is reading the wrong occurrence and the
    /// lane is meaningless either way.
    pub(in crate::cranelift_backend::planning::static_transition) fn aggregate_allocation(
        &self,
        origin: StaticOriginId,
        shape: PlannedAggregateShape,
    ) -> Result<PlannedAggregateAllocation, CraneliftBackendError> {
        let record = self
            .aggregate_ownership
            .iter()
            .find(|record| record.producer == AggregateOccurrenceProducer::Source(origin))
            .ok_or_else(|| {
                planner_error("aggregate producer has no planned ownership record")
            })?;
        if record.shape != shape {
            return Err(planner_error(
                "aggregate producer disagrees with its planned ownership shape",
            ));
        }
        Ok(record.allocation)
    }
    /// The occurrence identity of one **source** aggregate producer.
    ///
    /// This is the only way lowering obtains an identity for a source
    /// `Construct`/`Record`, and it is asked at the producer occurrence — where
    /// the answer is well defined — not at the emission site, where it is not.
    ///
    /// Absence is a loud failure for the same reason the lane's absence is: an
    /// occurrence the planner never interned is one whose meet was never taken.
    pub(in crate::cranelift_backend) fn source_aggregate_occurrence(
        &self,
        origin: StaticOriginId,
        shape: PlannedAggregateShape,
    ) -> Result<AggregateOccurrenceId, CraneliftBackendError> {
        let record = self
            .aggregate_ownership
            .iter()
            .find(|record| record.producer == AggregateOccurrenceProducer::Source(origin))
            .ok_or_else(|| planner_error("aggregate producer has no planned ownership record"))?;
        if record.shape != shape {
            return Err(planner_error(
                "aggregate producer disagrees with its planned ownership shape",
            ));
        }
        Ok(record.id)
    }
    /// The occurrence identity of one **compiler-synthesized** aggregate use.
    ///
    /// ⛔ The key is `owner + seat + path + full role`, never the role alone. A
    /// synthesized aggregate has no occurrence in the program to be keyed by,
    /// and a role repeats within one seat's tree — `ResourceKind` three times —
    /// so the path is what separates the uses.
    ///
    /// Every allocation-reachable use has a record, site-bound ones included.
    /// Absence here is a loud failure, not the ordinary answer for a role whose
    /// children come from the emission site.
    pub(in crate::cranelift_backend) fn synthesized_aggregate_occurrence(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
        role: SynthesizedConstructorRole,
    ) -> Result<AggregateOccurrenceId, CraneliftBackendError> {
        self.synthesized_aggregate_record(
            owner,
            seat,
            path,
            SynthesizedAggregateRole::Constructor(role),
        )
        .map(|record| record.id)
    }
    /// The occurrence of the empty environment record nested in one exact
    /// source-constructor field that the closed result analysis routes to a
    /// generated-unit call input.
    ///
    /// Absence is ordinary for every other closure-valued field. The full key
    /// remains owner + producer seat + structural field path + compiler role;
    /// no lowering-order ordinal is accepted by this interface.
    pub(in crate::cranelift_backend) fn unit_boundary_environment_occurrence(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        position: u32,
    ) -> Option<AggregateOccurrenceId> {
        let path = SynthesizedAggregatePath::root(
            SynthesizedAggregateRoot::UnitBoundaryEnvironment,
        )
        .field(position);
        self.aggregate_ownership
            .iter()
            .find(|record| {
                record.producer
                    == AggregateOccurrenceProducer::SynthesizedUse {
                        owner,
                        seat,
                        path: path.clone(),
                        role: SynthesizedAggregateRole::UnitBoundaryEnvironment,
                    }
            })
            .map(|record| record.id)
    }
    /// The record of one synthesized use, found by the full four-part key.
    ///
    /// ⛔ The path is part of the LOOKUP, not a field checked afterwards. A
    /// lookup that matched on owner/seat/role and then verified the path would
    /// find the first of three `ResourceKind` uses and reject the other two;
    /// matching on all four finds each one's own record.
    pub(in crate::cranelift_backend::planning::static_transition) fn synthesized_aggregate_record(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
        role: SynthesizedAggregateRole,
    ) -> Result<&PlannedAggregateOwnership, CraneliftBackendError> {
        self.aggregate_ownership
            .iter()
            .find(|record| match &record.producer {
                AggregateOccurrenceProducer::SynthesizedUse {
                    owner: record_owner,
                    seat: record_seat,
                    path: record_path,
                    role: record_role,
                } => {
                    *record_owner == owner
                        && *record_seat == seat
                        && record_path == path
                        && *record_role == role
                }
                AggregateOccurrenceProducer::Source(_) => false,
            })
            .ok_or_else(|| {
                planner_error(format!(
                    "synthesized aggregate use has no planned ownership record for owner \
                     {owner:?}, seat {seat:?}, path {path:?}, and role {role:?}"
                ))
            })
    }
    /// The declared child model of one modelled synthesized role.
    ///
    /// The recipe and the lowering code that builds these aggregates are two
    /// statements of one shape. Handing the emitter the model -- rather than
    /// only its length -- is what lets it check that each operand it actually
    /// holds is the KIND the recipe assumed when it took the meet.
    ///
    /// Arity alone is not sufficient and was not claimed to be: a recipe that
    /// says `Immediate` where the emitter passes a referent-bearing child has
    /// the right count and the wrong lane, and the aggregate is allocated
    /// persistent over an operand that can be arena-owned.
    /// The tree node one path names at one effect seat, whether or not it is
    /// allocation-reachable.
    ///
    /// ⭐ This is what lets a **dynamic alternative** be reconciled against the
    /// tree. An alternative HAS its own path-keyed ownership record and takes
    /// its allocation lane from it, exactly as a fixed constructor does; what
    /// it does not have is a parent's declared child model to be reached
    /// through, because a dynamic set's members are not ordered fields of a
    /// constructor. So its ordered fields are read from the tree here, and an
    /// emitter that put `ResourceTraceIdentity` at `ResourceReleaseFailed`
    /// field 2 instead of field 1 would otherwise pass unchallenged while
    /// carrying the wrong occurrence.
    ///
    /// A path that names no node, or names one that is not a fixed
    /// constructor, is a loud failure: the emitter and the tree disagree about
    /// the shape of the thing being built.
    pub(in crate::cranelift_backend) fn synthesized_tree_node(
        &self,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
    ) -> Result<(SynthesizedConstructorRole, &'static [SynthesizedAggregateNode]),
        CraneliftBackendError>
    {
        let operation = self.host_effect_operation(seat)?;
        let roles = self.semantic.synthesized_io_error_roles();
        match self.synthesized_tree_walk(operation, path)? {
            SynthesizedTreeResolution::Node(SynthesizedAggregateNode::Fixed {
                role,
                children,
            }) => Ok((SynthesizedConstructorRole::Fixed(role), children)),
            SynthesizedTreeResolution::IoErrorAlternative(position) => {
                let role = roles.get(position as usize).copied().ok_or_else(|| {
                    planner_error(
                        "synthesized aggregate path names an IOError alternative the closed \
                         inventory does not have",
                    )
                })?;
                Ok((
                    SynthesizedConstructorRole::IoError(role),
                    io_error_alternative_children(position as usize, roles.len()),
                ))
            }
            SynthesizedTreeResolution::Node(_) => Err(planner_error(
                "synthesized aggregate path does not name a constructor node",
            )),
        }
    }
    /// Walk one path from an operation's tree root to the node it names.
    ///
    /// Split out from [`Self::synthesized_tree_node`] because two callers need
    /// different things at the end of the same walk: one wants the constructor
    /// at the path, the other wants the alternative POPULATION at it. Sharing
    /// the walk is what keeps the step-kind law stated once.
    fn synthesized_tree_walk(
        &self,
        operation: ken_host::HostOpV1,
        path: &SynthesizedAggregatePath,
    ) -> Result<SynthesizedTreeResolution, CraneliftBackendError> {
        let mut node = host_effect_recipe_tree(operation).node(path.root);
        for (depth, step) in path.steps.iter().enumerate() {
            // The `IOError` set's alternatives are minted by the planner, so
            // they are resolved from the inventory rather than from a static
            // child list. This is a terminal step: an `IOError` alternative is
            // nullary or carries one scalar, and neither is a node a further
            // step can enter.
            if let SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors) = node {
                let SynthesizedAggregateStep::Alternative(position) = step else {
                    return Err(planner_error(
                        "synthesized aggregate path takes a field step into the IOError set",
                    ));
                };
                if depth + 1 != path.steps.len() {
                    return Err(planner_error(
                        "synthesized aggregate path continues past an IOError alternative, \
                         which has no constructor-valued child",
                    ));
                }
                return Ok(SynthesizedTreeResolution::IoErrorAlternative(*position));
            }
            node = match (node, step) {
                (
                    SynthesizedAggregateNode::Fixed { children, .. },
                    SynthesizedAggregateStep::Field(position),
                ) => *children.get(*position as usize).ok_or_else(|| {
                    planner_error("synthesized aggregate path names a field the tree does not have")
                })?,
                (
                    SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(
                        alternatives,
                    )),
                    SynthesizedAggregateStep::Alternative(position),
                ) => *alternatives.get(*position as usize).ok_or_else(|| {
                    planner_error(
                        "synthesized aggregate path names an alternative the tree does not have",
                    )
                })?,
                // ⛔ A field step into a dynamic set, or an alternative step
                // into a fixed constructor, is not a path this tree has. The
                // step kinds are what make that a refusal rather than an index
                // that happens to be in range.
                _ => {
                    return Err(planner_error(
                        "synthesized aggregate path takes a step the node it is at cannot take",
                    ));
                }
            };
        }
        Ok(SynthesizedTreeResolution::Node(node))
    }
    /// **A DIFFERENT live effect seat running the SAME host operation.**
    ///
    /// ⭐ Same operation means the same synthesized recipe tree, so the sibling
    /// shares this seat's roles, paths and shapes exactly. That is what makes
    /// an A/B out of it: the only coordinate that differs between the two is
    /// which occurrence in the program is being lowered, and every other input
    /// to the record lookup is identical by construction.
    ///
    /// ⛔ Never an invalid or non-`Effect` origin. A refusal driven by one of
    /// those would be a refusal about seat VALIDITY, which is a different and
    /// much weaker claim than the one the discriminator makes.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn sibling_effect_seat(
        &self,
        seat: StaticOriginId,
    ) -> Option<StaticOriginId> {
        let operation = match self.source_occurrence(seat) {
            Ok(RuntimeExpr::Effect { operation, .. }) => operation.clone(),
            _ => return None,
        };
        let mut stack = vec![self.root_static_origin().ok()?];
        let mut seen = 0usize;
        while let Some(origin) = stack.pop() {
            seen += 1;
            if seen > 4096 {
                return None;
            }
            if origin != seat
                && matches!(
                    self.source_occurrence(origin),
                    Ok(RuntimeExpr::Effect { operation: other, .. }) if *other == operation
                )
            {
                return Some(origin);
            }
            let mut position = 0;
            while let Ok(child) = self.child_static_origin(origin, position) {
                stack.push(child);
                position += 1;
            }
        }
        None
    }
    /// **`RT-DECL-CLOSURE-PORT` `D7` — a READ-ONLY projection of one planned
    /// aggregate ownership record, reached by its opaque occurrence identity.**
    ///
    /// ⭐ The key is the occurrence, never a coordinate. That is the whole
    /// point: a consumer that holds a template holds its producer's identity,
    /// and this turns that identity into the planner's own facts without the
    /// consumer knowing where the producer sat or being able to search for it.
    ///
    /// ⛔ Read-only, and deliberately not a `&PlannedAggregateOwnership`. The
    /// record is the planner's; handing out a reference to it would let a
    /// consumer pattern-match its way to facts this projection has not chosen
    /// to publish, and a later field would silently become emitter-visible.
    pub(in crate::cranelift_backend) fn aggregate_record_view(
        &self,
        id: AggregateOccurrenceId,
    ) -> Result<PlannedAggregateView<'_>, CraneliftBackendError> {
        let record = self.aggregate_ownership.get(id.0 as usize).ok_or_else(|| {
            planner_error("aggregate occurrence identity is outside this plan's population")
        })?;
        // ⚠ The identity indexes the arena, so the record found must AGREE
        // that it is the one asked for. A record whose own `id` differs would
        // mean the arena's order and its contents had diverged, which nothing
        // downstream could see.
        if record.id != id {
            return Err(planner_error(
                "planned aggregate record disagrees with the identity it was found by",
            ));
        }
        Ok(PlannedAggregateView { record })
    }
    /// The closed planner population `P`, for the whole-pass relation closeout.
    pub(in crate::cranelift_backend) fn aggregate_ownership_records(
        &self,
    ) -> &[PlannedAggregateOwnership] {
        &self.aggregate_ownership
    }
    /// **The planner's closed, ordered alternative population at one path.**
    ///
    /// ⭐ This exists so the emitter can be checked for **equality** rather than
    /// for prefix agreement. Iterating the emitter's own alternative vector and
    /// resolving each position proves only that the alternatives it *did* build
    /// are the right ones; a vector missing its last alternative — or empty —
    /// agrees with every prefix of the planned population and passes. A planner
    /// tree with two `ResourceKind` alternatives then accepts an emitter
    /// carrying only alternative 0, and the missing allocation is invisible
    /// everywhere.
    ///
    /// ⛔ **Not "invisible until a later closeout" — the earlier text said
    /// that, and it was wrong.** The whole-pass close states `image(R) ⊆ P`,
    /// not equality, because `P` authorizes rather than obliges and an unused
    /// record is lawful. So the ledger cannot distinguish a truncated emitter
    /// from a record this compilation simply had no body for, and the exact
    /// cardinality can never be deferred to it.
    ///
    /// ⛔ So the count comes from HERE and never from the emitter.
    ///
    /// The path must name a dynamic node; anything else is a shape
    /// disagreement rather than a population one and is refused as such.
    pub(in crate::cranelift_backend) fn synthesized_dynamic_alternatives(
        &self,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
    ) -> Result<Vec<SynthesizedConstructorRole>, CraneliftBackendError> {
        self.synthesized_alternative_population(seat, path)?
            .ok_or_else(|| {
                planner_error("synthesized aggregate path does not name a dynamic alternative set")
            })
    }
    /// **The alternative population at a path, with ABSENCE typed apart from
    /// FAILURE.**
    ///
    /// ⭐ `Ok(None)` means the path **lawfully resolved** to a node that is not
    /// a dynamic set — a constructor, a scalar, a site operand, or an absent
    /// arm. `Err` means the question could not be answered at all: the seat is
    /// missing or is not an `Effect`, the walk left the tree, an `IOError`
    /// position is outside the closed inventory, or the population is
    /// malformed.
    ///
    /// ⛔ **Those are not the same answer and a caller may not merge them.** A
    /// root reconciliation that wrote `.ok()` here turned every one of those
    /// failures into "the planner plans no set at this root", so a non-dynamic
    /// emitted root then matched the absent case and was accepted. That is a
    /// missing-authority default in a function whose whole contract is that
    /// neither direction may be defaulted — and no shape or truncation mutation
    /// can find it, because both of those keep the lookup working.
    fn synthesized_alternative_population(
        &self,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
    ) -> Result<Option<Vec<SynthesizedConstructorRole>>, CraneliftBackendError> {
        let operation = self.host_effect_operation(seat)?;
        match self.synthesized_tree_walk(operation, path)? {
            SynthesizedTreeResolution::Node(node) => match node {
                SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(
                    alternatives,
                )) => alternatives
                    .iter()
                    .map(|alternative| match alternative {
                        SynthesizedAggregateNode::Fixed { role, .. } => {
                            Ok(SynthesizedConstructorRole::Fixed(*role))
                        }
                        // ⛔ A malformed population is a FAILURE, not an
                        // absence: an alternative that is not a constructor
                        // allocates nothing and the set cannot be stated.
                        _ => Err(planner_error(
                            "a dynamic aggregate alternative is not a constructor, so it \
                             allocates nothing",
                        )),
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(Some),
                SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors) => Ok(Some(
                    self.semantic
                        .synthesized_io_error_roles()
                        .iter()
                        .map(|role| SynthesizedConstructorRole::IoError(*role))
                        .collect(),
                )),
                // ⭐ A LAWFUL non-set. The path resolved; the node it named
                // simply is not a dynamic set. This is the only absence, and it
                // is the one a caller may act on.
                SynthesizedAggregateNode::Fixed { .. }
                | SynthesizedAggregateNode::Scalar { .. }
                | SynthesizedAggregateNode::SiteOperand(_)
                // A capture word is a leaf, never a dynamic alternative set.
                | SynthesizedAggregateNode::WorkerCaptureOperand(_)
                | SynthesizedAggregateNode::Absent => Ok(None),
            },
            // An alternative is a member of a set, not a set. A path that names
            // one where a set was asked for is a disagreement about the shape
            // of the tree, not a lawful absence.
            SynthesizedTreeResolution::IoErrorAlternative(_) => Err(planner_error(
                "synthesized aggregate path names an IOError alternative, not a set",
            )),
        }
    }
    /// [`Self::synthesized_alternative_population`] at a host-result ROOT.
    ///
    /// Named separately because the two callers want different things from the
    /// same answer: a dynamic CHILD is declared dynamic by its parent's child
    /// model, so `Ok(None)` there is a tree inconsistency and
    /// `synthesized_dynamic_alternatives` turns it into an error. A ROOT has
    /// nothing above it declaring its kind, so `Ok(None)` is the ordinary
    /// answer for the arms that build a constructor or nothing at all.
    pub(in crate::cranelift_backend) fn synthesized_root_alternative_population(
        &self,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
    ) -> Result<Option<Vec<SynthesizedConstructorRole>>, CraneliftBackendError> {
        self.synthesized_alternative_population(seat, path)
    }
    pub(in crate::cranelift_backend) fn synthesized_aggregate_children(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
        role: SynthesizedConstructorRole,
    ) -> Result<&'static [SynthesizedAggregateNode], CraneliftBackendError> {
        self.synthesized_aggregate_record(
            owner,
            seat,
            path,
            SynthesizedAggregateRole::Constructor(role),
        )?
            .declared_children
            .ok_or_else(|| {
                planner_error("synthesized aggregate use has a record but no child model")
            })
    }
    /// The ruled allocation lane of an already-interned aggregate occurrence.
    ///
    /// The identity carries the answer from the producer to the emitter across
    /// a traversal that loses the origin. There is deliberately no fallible
    /// lookup by emission origin here: an identity this plan issued always
    /// resolves, and an identity it did not issue cannot be constructed.
    pub(in crate::cranelift_backend) fn aggregate_allocation_at(
        &self,
        occurrence: AggregateOccurrenceId,
        shape: PlannedAggregateShape,
    ) -> Result<PlannedAggregateAllocation, CraneliftBackendError> {
        let record = self
            .aggregate_ownership
            .get(occurrence.0 as usize)
            .ok_or_else(|| {
                planner_error("aggregate occurrence identity is outside this plan's population")
            })?;
        if record.shape != shape {
            return Err(planner_error(
                "aggregate producer disagrees with its planned ownership shape",
            ));
        }
        Ok(record.allocation)
    }
}


#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::tests::unit;
    use crate::RuntimeValue;

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
        let owner = *inline_synthesized_seat_emission_owners(&plan, seat)
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













    /// A host-result constructor emitted from a specialization's selected body
    /// receives the exact owner the consumer binds while lowering that body.
    ///
    /// Promise class: durable invariant. The fixture is discriminating because
    /// no generated continuation context contains the effect seat: the retired
    /// context-containment proxy omits the specialization owner while the real
    /// lowered-unit body contains it.
    #[test]
    fn non_inline_host_effect_records_equal_their_actual_emission_owner_set() {
        let expression = super::super::tests::contspec_parameter_match(RuntimeExpr::Effect {
            family: "FS".to_string(),
            operation: ken_host::HostOpV1::FsReadFile,
            capability: None,
            args: vec![unit()],
        });
        let plan = plan_static_transition_graph(&expression, &BTreeMap::new())
            .expect("the host-effect specialization fixture plans");
        let seat = plan
            .source_occurrences
            .iter()
            .flatten()
            .find(|occurrence| matches!(occurrence.expr, RuntimeExpr::Effect { .. }))
            .expect("the fixture has one effect seat")
            .static_origin;
        let units = plan.continuation_units().expect("the plan exposes its units");
        assert_eq!(
            units.len(),
            1,
            "the fixture must have one exact specialization owner"
        );
        let specialization = ContinuationEmissionOwner::Specialization(units[0].id());
        let selected_body = plan
            .semantic
            .child_origin(
                units[0].continuation_origin(),
                1 + units[0].producer_alternative() as usize,
            )
            .expect("the selected case body has a planned origin");
        assert!(
            super::super::occurrence_subtree_contains(&plan, selected_body, seat)
                .expect("the selected body subtree is valid"),
            "positive control: the body lowered under the specialization must contain the seat"
        );
        assert!(
            plan.continuation_contexts.iter().all(|context| {
                context.enclosing_specialization != units[0].id()
                    || !super::super::occurrence_subtree_contains(
                        &plan,
                        context.worker_body_origin,
                        seat,
                    )
                    .expect("the context subtree is valid")
            }),
            "negative discriminator: the context-containment proxy must omit this real emission"
        );

        let path = SynthesizedAggregatePath::root(
            SynthesizedAggregateRoot::HostResultError,
        )
        .field(0);
        let actual = plan
            .aggregate_ownership
            .iter()
            .filter_map(|record| match &record.producer {
                AggregateOccurrenceProducer::SynthesizedUse {
                    owner,
                    seat: record_seat,
                    path: record_path,
                    role: SynthesizedAggregateRole::Constructor(
                        SynthesizedConstructorRole::Fixed(
                            SynthesizedFixedConstructorRole::FileOperationRead,
                        ),
                    ),
                } if *record_seat == seat && record_path == &path => Some(*owner),
                AggregateOccurrenceProducer::Source(_)
                | AggregateOccurrenceProducer::SynthesizedUse { .. } => None,
            })
            .collect::<BTreeSet<_>>();
        let predeclared = ContinuationEmissionOwner::Predeclared(
            plan.semantic
                .function_owner(seat)
                .expect("the seat owner resolves")
                .expect("the seat belongs to a predeclared unit"),
        );
        assert_eq!(
            actual,
            BTreeSet::from([predeclared, specialization]),
            "the FileOperationRead record owners must equal the ordinary and specialization \
             bodies that actually emit this seat"
        );
    }

    /// Unit-boundary environments are inline at their source-constructor seat.
    /// Therefore the same selected-body authority used for host-result
    /// constructors is both necessary and sufficient for this sibling role.
    ///
    /// Promise class: durable invariant. Placing the producer inside a selected
    /// specialization body exercises the owner axis that a root-only UBE
    /// fixture cannot distinguish.
    #[test]
    fn unit_boundary_environment_uses_the_same_inline_emission_authority() {
        let body = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["value".to_string()],
                body: Box::new(RuntimeExpr::Var(0)),
            }),
            args: vec![RuntimeExpr::Construct {
                constructor: "ctor:fixture::Environment::Wrap".to_string(),
                args: vec![RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["unit".to_string()],
                    body: Box::new(unit()),
                }],
            }],
        };
        let expression = super::super::tests::contspec_parameter_match(body);
        let plan = plan_static_transition_graph(&expression, &BTreeMap::new())
            .expect("the UBE specialization fixture plans");
        let producer = plan
            .source_occurrences
            .iter()
            .flatten()
            .find_map(|occurrence| match occurrence.expr {
                RuntimeExpr::Construct { args, .. }
                    if matches!(args.as_slice(), [RuntimeExpr::LexicalClosure { .. }]) =>
                {
                    Some(occurrence.static_origin)
                }
                _ => None,
            })
            .expect("the fixture has one UBE producer seat");
        let units = plan.continuation_units().expect("the plan exposes its units");
        assert_eq!(units.len(), 1, "the fixture has one specialization");
        let specialization = ContinuationEmissionOwner::Specialization(units[0].id());
        let path = SynthesizedAggregatePath::root(
            SynthesizedAggregateRoot::UnitBoundaryEnvironment,
        )
        .field(0);
        let actual = plan
            .aggregate_ownership
            .iter()
            .filter_map(|record| match &record.producer {
                AggregateOccurrenceProducer::SynthesizedUse {
                    owner,
                    seat,
                    path: record_path,
                    role: SynthesizedAggregateRole::UnitBoundaryEnvironment,
                } if *seat == producer && record_path == &path => Some(*owner),
                AggregateOccurrenceProducer::Source(_)
                | AggregateOccurrenceProducer::SynthesizedUse { .. } => None,
            })
            .collect::<BTreeSet<_>>();
        let predeclared = ContinuationEmissionOwner::Predeclared(
            plan.semantic
                .function_owner(producer)
                .expect("the producer owner resolves")
                .expect("the producer belongs to a predeclared unit"),
        );
        assert_eq!(
            actual,
            BTreeSet::from([predeclared, specialization]),
            "the UBE record owners must equal the bodies that lower its source producer inline"
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
            // A leaf, and never in a host-effect recipe tree, which is what
            // this walker is pointed at.
            | SynthesizedAggregateNode::WorkerCaptureOperand(_)
            | SynthesizedAggregateNode::Absent => {}
        }
    }
}

#[cfg(test)]
mod checked_ih_captured_env_schema {
    use super::super::continuations::{set_envelope_defect, EnvelopeDefect};
    use super::super::tests::unit;
    use super::*;
    use crate::cranelift_backend::planning::static_transition::plan_static_transition_graph;
    use crate::RuntimeValue;
    use std::collections::{BTreeMap, BTreeSet};

    fn checked_ih_records<'plan>(
        plan: &'plan StaticTransitionPlan<'_>,
    ) -> Vec<&'plan PlannedAggregateOwnership> {
        plan.aggregate_ownership
            .iter()
            .filter(|record| {
                matches!(
                    record.producer,
                    AggregateOccurrenceProducer::SynthesizedUse {
                        role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                        ..
                    }
                )
            })
            .collect()
    }

    fn record_key(
        record: &PlannedAggregateOwnership,
    ) -> (ContinuationEmissionOwner, StaticOriginId) {
        match record.producer {
            AggregateOccurrenceProducer::SynthesizedUse { owner, seat, .. } => (owner, seat),
            _ => unreachable!("checked_ih_records filters on SynthesizedUse"),
        }
    }

    fn record_seat(record: &PlannedAggregateOwnership) -> StaticOriginId {
        record_key(record).1
    }

    /// The retired context-subtree proxy, retained only as the negative oracle
    /// proving the force relation does not accidentally collapse back onto it.
    fn legacy_context_containment_owners(
        plan: &StaticTransitionPlan<'_>,
        seat: StaticOriginId,
    ) -> BTreeSet<ContinuationEmissionOwner> {
        let mut owners = BTreeSet::new();
        if let Some(predeclared) = plan
            .semantic
            .function_owner(seat)
            .expect("the seat owner resolves")
        {
            owners.insert(ContinuationEmissionOwner::Predeclared(predeclared));
        }
        for context in &plan.continuation_contexts {
            if super::super::occurrence_subtree_contains(
                plan,
                context.worker_body_origin,
                seat,
            )
            .expect("the context subtree is valid")
            {
                owners.insert(ContinuationEmissionOwner::Specialization(
                    context.enclosing_specialization,
                ));
            }
        }
        owners
    }

    /// The ruled force-edge population, derived directly from continuation
    /// units rather than through `checked_ih_force_emissions`.
    fn authoritative_force_edges(
        plan: &StaticTransitionPlan<'_>,
    ) -> BTreeSet<(ContinuationEmissionOwner, StaticOriginId)> {
        let mut edges = BTreeSet::new();
        for unit in plan.continuation_units().expect("the plan exposes its units") {
            let Some(envelope) = unit
                .ruled_ordinary_envelope()
                .expect("no fixture here has a malformed envelope")
            else {
                continue;
            };
            let seats = envelope
                .iter()
                .filter_map(|role| match role {
                    ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                        closure_origin,
                        source: ContinuationWorkerCaptureSource::Lexical(_),
                        ..
                    } => Some(*closure_origin),
                    ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { .. }
                    | ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                        source: ContinuationWorkerCaptureSource::Seed,
                        ..
                    } => None,
                })
                .collect::<BTreeSet<_>>();
            if let Some(seat) = seats.iter().next().filter(|_| seats.len() == 1) {
                edges.insert((
                    ContinuationEmissionOwner::Specialization(unit.id()),
                    *seat,
                ));
            }
        }
        edges
    }

    /// **The ORACLE: the planner's own `WorkerCapture` roles, read directly.**
    ///
    /// ⛔ This deliberately does NOT call `checked_ih_coordinate_run`. That
    /// function and the issuance loop are the SUBJECTS here; comparing a record
    /// to the predicate that produced it is circular and stays green under any
    /// change they make together. The authority is the ruled ordered-worker
    /// envelope, so this reads the roles and rebuilds what the record is
    /// supposed to say from them.
    fn authoritative_runs(
        plan: &StaticTransitionPlan<'_>,
    ) -> BTreeMap<StaticOriginId, Vec<(u32, StaticOriginId)>> {
        let mut runs = BTreeMap::new();
        for unit in plan.continuation_units().expect("the plan exposes its units") {
            let Some(envelope) = unit
                .ruled_ordinary_envelope()
                .expect("no fixture here has a malformed envelope")
            else {
                continue;
            };
            let mut seat = None;
            let mut run = Vec::new();
            for role in &envelope {
                let ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                    ordinal,
                    closure_origin,
                    source,
                    ..
                } = role
                else {
                    continue;
                };
                let ContinuationWorkerCaptureSource::Lexical(sourced) = source else {
                    continue;
                };
                seat = Some(*closure_origin);
                run.push((*ordinal, *sourced));
            }
            if let Some(seat) = seat {
                if !run.is_empty() {
                    runs.insert(seat, run);
                }
            }
        }
        runs
    }

    fn plan_of(expr: RuntimeExpr) -> StaticTransitionPlan<'static> {
        let expr = Box::leak(Box::new(expr));
        plan_static_transition_graph(expr, &BTreeMap::new()).expect("the fixture plans")
    }

    /// **The half of the option-(c) control that this instrument can honestly
    /// carry: plan construction still succeeds.**
    ///
    /// The fixture contains a unit that has worker captures and no ruled
    /// ordered-worker envelope. Propagating that precondition failure (option
    /// (a)) fails plan construction for a program that is compile-valid, so this
    /// `expect` is what reds under (a).
    ///
    /// LIMIT, stated rather than implied: the out-of-domain unit is NOT
    /// observable through `continuation_units()` on the FINISHED plan, so a test
    /// cannot assert "this seat received no record" about it from here. The
    /// anti-(b) evidence is carried by the set-equality test below, not by this
    /// fixture. A control that cannot see the case it names is worth less than
    /// saying so.
    #[test]
    fn a_program_with_an_envelope_less_captured_unit_still_plans() {
        let plan = plan_of(
            super::super::continuations::tests::contspec_multiple_worker_captures_fixture(),
        );
        for record in checked_ih_records(&plan) {
            assert!(
                !record.children.is_empty(),
                "a checked-IH record is issued only for a unit with a coordinate run, so it \
                 can never carry an empty child run"
            );
        }
    }

    /// Membership EQUALS coordinate-existence, as SETS of seats.
    ///
    /// ⛔ This asserted `issued.len() == expected` and was green while a seat
    /// was substituted for a different one -- a cardinality check cannot see a
    /// swap, only a count change. Set equality names both directions at once:
    /// an admitted unit with no run appears in `issued` and not `expected`, and
    /// a dropped in-domain unit appears in `expected` and not `issued`.
    #[test]
    fn the_issued_seats_are_exactly_the_seats_with_a_coordinate_run() {
        for fixture in [
            super::super::continuations::tests::contspec_multiple_worker_captures_fixture(),
            super::super::continuations::tests::contspec_activation_owned_worker_captures_fixture(
            ),
            super::super::continuations::tests::contspec_capture_free_worker_fixture(),
        ] {
            let plan = plan_of(fixture);
            let expected: BTreeSet<StaticOriginId> =
                authoritative_runs(&plan).keys().copied().collect();
            let issued: BTreeSet<StaticOriginId> = checked_ih_records(&plan)
                .iter()
                .map(|record| record_seat(record))
                .collect();
            assert_eq!(
                issued, expected,
                "the issued seats must be exactly the seats the authoritative envelope roles \
                 admit -- as a set, not as a count"
            );
        }
    }

    /// The record's ordered `(position, origin)` run equals the authoritative
    /// role sequence, pairs kept together.
    ///
    /// ⛔ The gap this closes: nothing pinned the ci<->oi ASSOCIATION. Reversing
    /// the source column alone left every count, every ordinal and every
    /// membership check satisfied, because each was true of a projection of the
    /// run rather than of the run. Comparing the ordered pair vector refutes a
    /// permutation of either column independently, a drop, and a duplicate.
    #[test]
    fn each_records_ordered_run_matches_the_authoritative_roles() {
        for fixture in [
            super::super::continuations::tests::contspec_multiple_worker_captures_fixture(),
            super::super::continuations::tests::contspec_activation_owned_worker_captures_fixture(
            ),
            super::super::tests::contspec_nested_fixture(),
        ] {
            let plan = plan_of(fixture);
            let runs = authoritative_runs(&plan);
            let records = checked_ih_records(&plan);
            assert!(
                !records.is_empty(),
                "this fixture must issue at least one record or the comparison below is vacuous"
            );
            for record in records {
                let seat = record_seat(record);
                let expected = runs
                    .get(&seat)
                    .expect("every issued seat has an authoritative run");
                let actual: Vec<(u32, StaticOriginId)> = record
                    .children
                    .iter()
                    .map(|child| {
                        (
                            child.position,
                            child
                                .origin
                                .expect("every checked-IH child names its source occurrence"),
                        )
                    })
                    .collect();
                assert_eq!(
                    actual, *expected,
                    "seat {seat:?}: the record's ordered (position, origin) run must equal the \
                     ruled envelope's WorkerCapture sequence"
                );
            }
        }
    }

    /// The producer key is the exact FORCE edge, not the containment proxy.
    ///
    /// The nested fixture supplies the discriminating pair: at least one
    /// generated context contains another worker closure but does not force
    /// that closure. The force pair receives a record; the containment-only
    /// pair must not. Set equality also proves no real force edge was omitted.
    #[test]
    fn records_are_keyed_by_exact_force_edges_not_containment() {
        let plan = plan_of(super::super::tests::contspec_nested_fixture());
        let forced = authoritative_force_edges(&plan);
        let actual = checked_ih_records(&plan)
            .into_iter()
            .map(record_key)
            .collect::<BTreeSet<_>>();
        assert!(!forced.is_empty(), "the fixture must carry real force edges");
        assert_eq!(actual, forced, "record keys must equal the force edges");

        let mut containment_only = BTreeSet::new();
        for (_, seat) in &forced {
            for owner in legacy_context_containment_owners(&plan, *seat) {
                if !forced.contains(&(owner, *seat)) {
                    containment_only.insert((owner, *seat));
                }
            }
        }
        assert!(
            !containment_only.is_empty(),
            "the fixture must contain a worker seat under an owner that does not force it, or \
             force and containment are degenerate and the negative arm proves nothing"
        );
        assert!(
            actual.is_disjoint(&containment_only),
            "a contained-but-not-forced owner must not receive checked-env authority"
        );
    }

    /// A force owner cannot borrow the canonical run of another worker seat.
    #[test]
    fn capture_origin_rejects_a_force_owner_paired_with_the_wrong_seat() {
        let plan = plan_of(super::super::tests::contspec_nested_fixture());
        let edges = authoritative_force_edges(&plan).into_iter().collect::<Vec<_>>();
        let (owner, seat) = *edges.first().expect("the fixture has a force edge");
        let wrong_seat = edges
            .iter()
            .map(|(_, seat)| *seat)
            .find(|candidate| *candidate != seat)
            .expect("the fixture has a second, distinct forced worker seat");
        let refusal = plan
            .checked_ih_capture_origin(owner, wrong_seat, 0)
            .expect_err("a force owner has authority only for the exact worker seat it forces");
        assert!(
            format!("{refusal:?}").contains(
                "no checked-IH captured-environment record is planned for this owner and seat"
            ),
            "the wrong owner-seat pair must reach the exact authority refusal: {refusal:?}"
        );
    }

    /// The escaping arm, on a run that carries BOTH lifetimes at once.
    ///
    /// ⛔ Every earlier control ran on a fixture whose captures were `unit()`,
    /// so every child was `Persistent` and the record was `PersistentGround`.
    /// A hard-coded `Persistent` would have passed all of them. This fixture
    /// alternates `Effect` (activation-owned) and `unit()` captures, so a
    /// constant in EITHER direction reds: the record must say two different
    /// things about two of its own children.
    #[test]
    fn a_mixed_run_carries_the_per_capture_lifetime_and_the_escaping_meet() {
        let plan = plan_of(
            super::super::continuations::tests::contspec_activation_owned_worker_captures_fixture(),
        );
        let records = checked_ih_records(&plan);
        assert_eq!(records.len(), 1, "the fixture has one checked-IH seat");
        let record = records[0];
        assert_eq!(
            record.children.len(),
            9,
            "the fixture declares nine captures"
        );
        for (position, child) in record.children.iter().enumerate() {
            // Independent oracle: the FIXTURE decides which captures are
            // effects, and the occurrence plane rules `Effect` activation-owned
            // and `unit()` persistent. Neither half is read back from the code
            // under test.
            let expected = if position % 2 == 0 {
                PlannedReferentLifetime::ActivationOwned
            } else {
                PlannedReferentLifetime::Persistent
            };
            assert_eq!(
                child.lifetime, expected,
                "capture {position} of the mixed run"
            );
        }
        assert_eq!(
            record.meet,
            PlannedReferentLifetime::ActivationOwned,
            "a run with an escaping member meets at the escaping lifetime"
        );
        assert_eq!(
            record.allocation,
            PlannedAggregateAllocation::InvocationAggregate,
            "an escaping meet allocates in the invocation arena"
        );
    }

    /// The real `UnitBoundaryEnvironment` producer shape: a `Call` whose
    /// argument carries an empty-capture `LexicalClosure`.
    fn unit_boundary_environment_fixture() -> RuntimeExpr {
        RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["value".to_string()],
                body: Box::new(RuntimeExpr::Var(0)),
            }),
            args: vec![RuntimeExpr::Construct {
                constructor: "ctor:fixture::Environment::Wrap".to_string(),
                args: vec![RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["unit".to_string()],
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::Environment::Leaf".to_string(),
                        args: Vec::new(),
                    }),
                }],
            }],
        }
    }

    fn unit_boundary_environment_records(plan: &StaticTransitionPlan<'_>) -> usize {
        plan.aggregate_ownership
            .iter()
            .filter(|record| {
                matches!(
                    record.producer,
                    AggregateOccurrenceProducer::SynthesizedUse {
                        role: SynthesizedAggregateRole::UnitBoundaryEnvironment,
                        ..
                    }
                )
            })
            .count()
    }

    /// **The negative half of the frame's discriminating pair: a program
    /// `UnitBoundaryEnvironment` ACTUALLY SERVES receives no checked-IH
    /// record.**
    ///
    /// ⛔ THE POSITIVE CONTROL IS ASSERTED FIRST, AND IT IS THE WHOLE POINT.
    /// What stood here used a merely capture-free contspec fixture and checked
    /// only that it built continuation units and got no checked-IH record. That
    /// fixture is not in the UBE population at all -- requiring a UBE record on
    /// it reds `0` against `1` -- so the absence it measured was about a program
    /// NEITHER producer serves, and the pair was half-open: it proved
    /// checked-IH fires for the nine-capture family, and never that it declines
    /// where UBE already answers. "No worker captures" is not UBE membership;
    /// UBE additionally requires this `Call` -> empty-capture `LexicalClosure`
    /// shape.
    #[test]
    fn a_unit_boundary_environment_program_receives_no_checked_ih_record() {
        let plan = plan_of(unit_boundary_environment_fixture());
        assert!(
            unit_boundary_environment_records(&plan) > 0,
            "positive control: this fixture must be one UnitBoundaryEnvironment actually \
             serves, or the checked-IH absence below is about a program neither producer \
             claims and discriminates nothing"
        );
        assert!(
            checked_ih_records(&plan).is_empty(),
            "a program UnitBoundaryEnvironment serves has no capture coordinate run, so the \
             checked-IH schema must decline it"
        );
    }

    /// A capture-free continuation unit also receives no record -- a DIFFERENT
    /// out-of-domain population from the UBE one above, and deliberately not
    /// described as that one.
    #[test]
    fn a_capture_free_continuation_unit_receives_no_checked_ih_record() {
        let plan =
            plan_of(super::super::continuations::tests::contspec_capture_free_worker_fixture());
        assert!(
            !plan
                .continuation_units()
                .expect("the plan exposes its units")
                .is_empty(),
            "the negative is vacuous unless this fixture actually builds continuation units"
        );
        assert!(
            checked_ih_records(&plan).is_empty(),
            "a capture-free unit has no coordinate run, so it receives no checked-IH record"
        );
    }

    /// **AC-SCHEMA: the record now DECLARES its positional child model.**
    ///
    /// `declared_children` is the static shape tree and says only "position i
    /// is capture word i" -- it carries no occurrence, which is what keeps it
    /// an independent authority from `children`, the per-unit occurrence
    /// carrier. The reconcile arm cross-checks one against the other; derive
    /// either from the other and that check goes vacuous.
    ///
    /// The shape is `Constructor`, not `Record`, while the ROLE stays
    /// `CheckedIhCapturedEnvironment`: shape selects the positional
    /// (`record_fields: None`) downstream path on which `field_identity: None`
    /// is legitimate, and the distinct role preserves the separation from every
    /// other synthesized use.
    #[test]
    fn the_record_declares_a_positional_child_model_for_its_arity() {
        let plan = plan_of(
            super::super::continuations::tests::contspec_activation_owned_worker_captures_fixture(),
        );
        let records = checked_ih_records(&plan);
        assert_eq!(records.len(), 1, "the fixture has one checked-IH seat");
        let record = records[0];

        assert_eq!(
            record.shape,
            PlannedAggregateShape::Constructor,
            "the captured environment takes the POSITIONAL downstream path"
        );
        let declared = record
            .declared_children
            .expect("a checked-IH record declares its child model");
        assert_eq!(
            declared.len(),
            record.children.len(),
            "the declared model must cover the ruled run exactly -- a shorter one would \
             describe fewer captures than the record carries"
        );
        let expected: Vec<SynthesizedAggregateNode> = (0..record.children.len() as u32)
            .map(SynthesizedAggregateNode::WorkerCaptureOperand)
            .collect();
        assert_eq!(
            declared, expected,
            "position i must declare capture word i, in order"
        );
        for child in &record.children {
            assert!(
                child.field_identity.is_none(),
                "identity is positional here; a field name would be invented"
            );
        }
    }

    /// **The arity bound REFUSES; it never truncates.**
    ///
    /// The const-run-sliced-by-arity trick is what makes a per-unit arity
    /// expressible in a `&'static` slice, and it is sound only while an arity
    /// above the run refuses. Silently returning a shorter prefix would hand
    /// the emitter a child model describing fewer captures than the record
    /// holds, and every length check downstream would agree with itself.
    ///
    /// Both sides of the boundary are asserted: at the limit it must still
    /// serve, one past it must refuse.
    #[test]
    fn an_arity_above_the_positional_run_refuses_rather_than_truncating() {
        let at_limit = positional_capture_declared_children(CHECKED_IH_CAPTURE_OPERAND_LIMIT)
            .expect("an arity exactly at the limit is still expressible");
        assert_eq!(at_limit.len(), CHECKED_IH_CAPTURE_OPERAND_LIMIT);

        let over = positional_capture_declared_children(CHECKED_IH_CAPTURE_OPERAND_LIMIT + 1);
        assert!(
            over.is_err(),
            "one capture past the run must REFUSE; returning a {}-long prefix for a longer \
             run is the silent truncation this bound exists to prevent",
            CHECKED_IH_CAPTURE_OPERAND_LIMIT
        );
    }

    /// A `Seed`-sourced capture is REFUSED, not admitted and not skipped.
    ///
    /// ⛔ The run derivation errors when a capture in the ruled run has no
    /// source occurrence, and nothing exercised that arm: changing the refusal
    /// to a `continue` left the whole module green. `Closure` (symbol captures)
    /// rather than `LexicalClosure` reaches it through the natural producer.
    #[test]
    fn a_seed_sourced_capture_is_refused_rather_than_admitted() {
        let expr = Box::leak(Box::new(
            super::super::continuations::tests::contspec_seed_capture_worker_fixture(),
        ));
        let refusal = match plan_static_transition_graph(expr, &BTreeMap::new()) {
            Ok(_) => panic!(
                "a seed-sourced capture has no source occurrence, so planning must refuse \
                 rather than admit or silently skip it"
            ),
            Err(refusal) => refusal,
        };
        let rendered = format!("{refusal:?}");
        assert!(
            rendered.contains("has no source occurrence"),
            "the refusal must be the checked-IH capture-source one, not an unrelated \
             planner failure that happens to also fail: got {rendered}"
        );
    }

    /// Promise class: durable invariant. A checked-IH transport names both
    /// endpoints and references the sole force-owner record; the destination
    /// remains record-free.
    #[test]
    fn transport_edges_reference_one_force_record_and_issue_no_destination_record() {
        let plan = plan_of(super::super::tests::contspec_nested_fixture());
        let transports = &plan.checked_ih_environment_transports;
        assert!(
            !transports.is_empty(),
            "the nested fixture must contain an escaping checked-IH transport"
        );
        for transport in transports {
            let source = plan
                .aggregate_ownership
                .get(transport.source_record.0 as usize)
                .expect("the transport source record exists");
            assert!(matches!(
                source.producer,
                AggregateOccurrenceProducer::SynthesizedUse {
                    owner,
                    seat,
                    role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                    ..
                } if owner == transport.source_owner && seat == transport.seat
            ));
            assert!(
                source.meet <= transport.destination_lifetime,
                "the materialized environment must outlive its destination"
            );
            assert!(
                plan.aggregate_ownership.iter().all(|record| !matches!(
                    record.producer,
                    AggregateOccurrenceProducer::SynthesizedUse {
                        owner,
                        seat,
                        role: SynthesizedAggregateRole::CheckedIhCapturedEnvironment,
                        ..
                    } if owner == transport.destination_owner && seat == transport.seat
                )),
                "the transport edge, not a second destination record, authorizes the crossing"
            );
        }
    }

    /// Promise class: durable invariant. Transport selection uses the complete
    /// source/destination tuple; changing the source specialization cannot fall
    /// back to the same closure seat.
    #[test]
    fn transport_lookup_is_exact_and_has_no_seat_fallback() {
        let plan = plan_of(super::super::tests::contspec_nested_fixture());
        let transport = plan
            .checked_ih_environment_transports
            .first()
            .expect("the nested fixture has a transport");
        assert_eq!(
            plan.checked_ih_environment_transport(
                transport.destination_owner,
                transport.destination_construct_origin,
                transport.recursive_position,
                transport.source_specialization,
            )
            .expect("the exact transport lookup is valid"),
            Some(transport),
        );
        assert_eq!(
            plan.checked_ih_environment_transport(
                transport.destination_owner,
                transport.destination_construct_origin,
                transport.recursive_position,
                match transport.destination_owner {
                    ContinuationEmissionOwner::Specialization(id) => id,
                    ContinuationEmissionOwner::Predeclared(_)
                    | ContinuationEmissionOwner::Fusion(_) => {
                        unreachable!("transport destinations are specializations")
                    }
                },
            )
            .expect("the wrong-source lookup is still a valid question"),
            None,
            "the destination and seat cannot authorize a transport from another source"
        );
    }

    /// Promise class: durable invariant. One plan may contain a transport
    /// destination and a transport-free producer at the same time. Selection
    /// is therefore per `(owner, producer origin)`, never plan-wide.
    #[test]
    fn transport_lookup_is_per_producer_inside_a_mixed_plan() {
        let plan = plan_of(super::super::tests::contspec_nested_fixture());
        let transport = plan
            .checked_ih_environment_transports
            .first()
            .expect("the nested fixture has a transport");
        let transport_free = plan
            .root_static_origin()
            .expect("the mixed fixture has a root producer");
        assert_ne!(
            transport_free, transport.destination_construct_origin,
            "the negative producer must be distinct from the transport destination"
        );
        assert_eq!(
            plan.checked_ih_environment_transport_at(
                transport.destination_owner,
                transport.destination_construct_origin,
            )
            .expect("the destination query is valid"),
            Some(transport),
        );
        assert_eq!(
            plan.checked_ih_environment_transport_at(
                transport.destination_owner,
                transport_free,
            )
            .expect("the transport-free producer query is valid"),
            None,
            "a plan-wide transport presence must not reroute another producer"
        );
    }

    /// Promise class: durable invariant. The environment record and call
    /// assembler state their ordered WorkerCapture run independently and meet
    /// only in the `(ordinal, source occurrence)` frame.
    #[test]
    fn transport_field_order_and_input_morphism_are_fail_closed() {
        let field_plan = plan_of(
            super::super::continuations::tests::contspec_activation_owned_worker_captures_fixture(),
        );
        let record = checked_ih_records(&field_plan)
            .into_iter()
            .find(|record| record.children.len() > 1)
            .expect("the field-order fixture has a multi-field checked-IH record");
        let (field_owner, field_seat) = record_key(record);
        let exact = record
            .children
            .iter()
            .map(|child| {
                (
                    child.position,
                    child.origin.expect("a checked-IH field names its source"),
                )
            })
            .collect::<Vec<_>>();
        assert!(
            field_plan.validate_checked_ih_capture_suffix(field_owner, field_seat, &exact)
            .expect("the exact suffix validates")
        );
        let mut reordered = exact.clone();
        reordered.reverse();
        assert!(
            field_plan.validate_checked_ih_capture_suffix(field_owner, field_seat, &reordered)
            .is_err(),
            "reordering the independently assembled call suffix must refuse"
        );

        let plan = plan_of(super::super::tests::contspec_nested_fixture());
        let transport = plan
            .checked_ih_environment_transports
            .iter()
            .find(|transport| transport.continuation_input_count() > 0)
            .expect("the nested fixture has a transport with continuation inputs");
        let source = plan
            .continuation_units()
            .expect("the plan exposes its units")
            .into_iter()
            .find(|unit| unit.id() == transport.source_specialization)
            .expect("the transport source unit exists");
        let inputs = source
            .continuation_inputs()
            .expect("the source inputs project");
        assert_eq!(inputs.len(), transport.continuation_input_count());
        for input in &inputs {
            assert!(
                transport
                    .continuation_input_index(input.ordinal, input.coordinate)
                    .is_some(),
                "every declared source input must have one destination coordinate"
            );
        }
        let first = &inputs[0];
        assert!(
            transport
                .continuation_input_index(first.ordinal.wrapping_add(1), first.coordinate)
                .is_none(),
            "a moved ordinal must not resolve by coordinate alone"
        );
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: the production exactness predicate accepts one paired body and
    /// rejects a second body or a missing pairing independently.
    /// CLAIMED: capture-only bind dispatch cannot admit a non-singleton or
    /// unpaired resume target.
    /// THE GAP: this pure law must be reached by real planner facts; the
    /// companion direct/multi fixture below supplies that production control.
    #[test]
    fn bind_target_exactness_rejects_multi_target_and_unpaired() {
        let body = StaticOriginId(10);
        let record = AggregateOccurrenceId(3);
        assert!(boundary_bind_targets_are_exact(
            &[BoundaryBindTargetProof {
                body_origin: body,
                environment_record: Some(record),
            }],
            body,
        ));
        assert!(
            !boundary_bind_targets_are_exact(
                &[
                    BoundaryBindTargetProof {
                        body_origin: body,
                        environment_record: Some(record),
                    },
                    BoundaryBindTargetProof {
                        body_origin: StaticOriginId(11),
                        environment_record: Some(AggregateOccurrenceId(4)),
                    },
                ],
                body,
            ),
            "two statically possible bodies must not acquire capture-only dispatch"
        );
        assert!(
            !boundary_bind_targets_are_exact(
                &[BoundaryBindTargetProof {
                    body_origin: body,
                    environment_record: None,
                }],
                body,
            ),
            "a singleton body without its dynamic-instance environment pairing must refuse"
        );
    }

    fn bind_continuation_fixture_with_case_body(
        field: RuntimeExpr,
        case_body: RuntimeExpr,
    ) -> RuntimeExpr {
        let constructor = "ctor:fixture::Bind::Resume".to_string();
        RuntimeExpr::CheckedSubcontinuationFrame {
            frame_id: 91,
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: constructor.clone(),
                    args: vec![unit(), field],
                }),
                cases: vec![crate::RuntimeComputationalMatchCase {
                    constructor,
                    argument_binders: 2,
                    recursive_positions: vec![1],
                    body: case_body,
                }],
                default: crate::RuntimeTrap {
                    code: crate::RuntimeTrapCode::PatternMatchFailure,
                    message: "bind continuation fixture did not select its case".to_string(),
                },
            }),
        }
    }

    fn bind_continuation_fixture(field: RuntimeExpr) -> RuntimeExpr {
        bind_continuation_fixture_with_case_body(field, RuntimeExpr::Var(0))
    }

    fn captured_bind_closure_with_captures(result: i64, captures: Vec<RuntimeExpr>) -> RuntimeExpr {
        RuntimeExpr::LexicalClosure {
            captures,
            params: vec!["response".to_string()],
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(result.into()))),
        }
    }

    fn captured_bind_closure(result: i64) -> RuntimeExpr {
        captured_bind_closure_with_captures(
            result,
            vec![RuntimeExpr::Value(RuntimeValue::Int(result.into()))],
        )
    }

    fn direct_bind_coordinates(
        plan: &StaticTransitionPlan<'_>,
    ) -> (
        StaticOriginId,
        StaticOriginId,
        usize,
        ContinuationEmissionOwner,
        BoundaryClosureEnvironment,
    ) {
        let mut direct = Vec::new();
        for occurrence in plan.source_occurrences.iter().flatten() {
            if !matches!(occurrence.expr, RuntimeExpr::Construct { .. }) {
                continue;
            }
            for (position, child) in plan
                .semantic
                .child_origins(occurrence.static_origin)
                .expect("the fixture's constructor children resolve")
                .iter()
                .copied()
                .enumerate()
            {
                if matches!(
                    plan.planned_occurrence_expr(child)
                        .expect("the fixture child resolves"),
                    RuntimeExpr::LexicalClosure { .. }
                ) {
                    direct.push((child, occurrence.static_origin, position));
                }
            }
        }
        let [(seat, resume_site, position)] = direct.as_slice() else {
            panic!("the fixture must have exactly one direct lexical-closure response field");
        };
        let owner = ContinuationEmissionOwner::Predeclared(
            plan.semantic
                .function_owner(*seat)
                .expect("the closure owner resolves")
                .expect("the closure belongs to a predeclared unit"),
        );
        let environment = plan
            .boundary_closure_environment(owner, *seat)
            .expect("the direct environment query is valid")
            .expect("the direct closure has an owner-specific environment record");
        (*seat, *resume_site, *position, owner, environment)
    }

    fn assert_direct_bind_is_authorized(
        plan: &StaticTransitionPlan<'_>,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        environment: &BoundaryClosureEnvironment,
    ) {
        assert!(
            !boundary_closure_owner_returns_seat(plan, owner, seat)
                .expect("result containment is a valid query"),
            "the positive fixture must reach the new bind arm, not the older result arm"
        );
        assert_eq!(
            plan.boundary_bind_continuation_environment_by_record(environment.record())
                .expect("the bind proof is valid"),
            Some(environment.clone()),
            "the exact recursive field must prove its singleton target and pairing"
        );
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: a real planned direct recursive response field resolves the
    /// bind record through the production accessor, while result containment is
    /// independently false.
    /// CLAIMED: the new bind authorization arm is production-reaching rather
    /// than an alias for the preserved result-containment arm.
    /// THE GAP: the negative target and pairing decisions are separate arms;
    /// the three production-corruption controls below reach each one directly.
    #[test]
    fn bind_continuation_authorization_is_reaching_and_not_generic() {
        let direct = Box::leak(Box::new(bind_continuation_fixture(captured_bind_closure(
            1,
        ))));
        let direct_plan = plan_static_transition_graph(direct, &BTreeMap::new())
            .expect("the direct bind-continuation fixture plans");
        let (seat, _, _, owner, environment) = direct_bind_coordinates(&direct_plan);
        assert_direct_bind_is_authorized(&direct_plan, owner, seat, &environment);
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: the production bind accessor receives an actual `Construct`
    /// response, an intact owner/pairing proof, and two distinct ABI
    /// `ClosureBody` targets for that response seat, then returns `None`.
    /// CLAIMED: a capture-only bind response cannot select among multiple
    /// compile-time code identities.
    /// THE GAP: the second target is injected into the ABI population after a
    /// valid plan is built; it is a corruption witness, not source syntax that
    /// an earlier parent-shape guard can reject.
    #[test]
    fn bind_continuation_production_rejects_multiple_static_targets() {
        let source = Box::leak(Box::new(bind_continuation_fixture_with_case_body(
            captured_bind_closure(1),
            captured_bind_closure(2),
        )));
        let mut plan = plan_static_transition_graph(source, &BTreeMap::new())
            .expect("the two-body bind-continuation fixture plans");
        let (seat, resume_site, position, owner, environment) = direct_bind_coordinates(&plan);
        assert_direct_bind_is_authorized(&plan, owner, seat, &environment);
        assert!(matches!(
            plan.planned_occurrence_expr(resume_site)
                .expect("the resume site resolves"),
            RuntimeExpr::Construct { .. }
        ));
        let response_record = plan
            .aggregate_ownership
            .iter()
            .find(|record| record.producer == AggregateOccurrenceProducer::Source(resume_site))
            .expect("the response constructor retains its ownership record");
        assert!(response_record
            .children
            .iter()
            .any(|child| { child.position as usize == position && child.origin == Some(seat) }));

        let mut competing = plan
            .abi
            .descriptors
            .iter()
            .copied()
            .find(|descriptor| {
                matches!(
                    descriptor.definition,
                    AbiUnitDefinition::ClosureBody {
                        defining_origin,
                        provenance: AbiCaptureProvenance::Lexical,
                    } if defining_origin != seat
                )
            })
            .expect("the case body supplies a distinct real closure unit");
        competing.definition = AbiUnitDefinition::ClosureBody {
            defining_origin: seat,
            provenance: AbiCaptureProvenance::Lexical,
        };
        plan.abi.descriptors.push(competing);

        let target_bodies = plan
            .emittable_units()
            .expect("the corrupted ABI population still projects")
            .into_iter()
            .filter_map(|unit| {
                matches!(
                    unit.definition(),
                    AbiUnitDefinition::ClosureBody {
                        defining_origin,
                        provenance: AbiCaptureProvenance::Lexical,
                    } if defining_origin == seat
                )
                .then_some(unit.body_occurrence())
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(
            target_bodies.len(),
            2,
            "the negative population must reach target derivation with two distinct bodies"
        );
        assert_eq!(
            plan.boundary_bind_continuation_environment_by_record(environment.record())
                .expect("the multi-target production query is valid"),
            None,
            "two static closure-body targets must restore the generic refusal"
        );
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: an otherwise valid direct response retains its resolvable
    /// environment descriptor and source record, but the record's exact field
    /// origin names a different real child and production authorization returns
    /// `None`.
    /// CLAIMED: position equality cannot substitute for exact response-instance
    /// origin pairing.
    /// THE GAP: removing `child.origin == Some(seat)` must admit this unchanged
    /// population; the mutation proof is recorded in the candidate handoff.
    #[test]
    fn bind_continuation_production_rejects_response_field_origin_mismatch() {
        let source = Box::leak(Box::new(bind_continuation_fixture(captured_bind_closure(
            1,
        ))));
        let mut plan = plan_static_transition_graph(source, &BTreeMap::new())
            .expect("the direct bind-continuation fixture plans");
        let (seat, resume_site, position, owner, environment) = direct_bind_coordinates(&plan);
        assert_direct_bind_is_authorized(&plan, owner, seat, &environment);
        let other_child = plan
            .semantic
            .child_origin(resume_site, 0)
            .expect("the response has a distinct real non-closure child");
        assert_ne!(other_child, seat);
        let response_record = plan
            .aggregate_ownership
            .iter_mut()
            .find(|record| record.producer == AggregateOccurrenceProducer::Source(resume_site))
            .expect("the response constructor retains its ownership record");
        let paired_field = response_record
            .children
            .iter_mut()
            .find(|child| child.position as usize == position)
            .expect("the response record retains the recursive field");
        paired_field.origin = Some(other_child);

        assert_eq!(
            plan.boundary_closure_environment_by_record(environment.record())
                .expect("the environment descriptor remains resolvable"),
            environment
        );
        assert_eq!(
            plan.boundary_bind_continuation_environment_by_record(environment.record())
                .expect("the mismatched-origin production query is valid"),
            None,
            "a response field naming another instance must restore the generic refusal"
        );
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: a direct response retains its exact field origin and resolvable
    /// environment record, but the response field's lifetime authority is
    /// narrowed so it no longer admits the activation-owned environment.
    /// CLAIMED: a response cannot carry an environment whose planned lifetime
    /// is shorter than the field requires.
    /// THE GAP: removing the production lifetime comparison must admit this
    /// unchanged population; the mutation proof is recorded in the handoff.
    #[test]
    fn bind_continuation_production_rejects_response_lifetime_mismatch() {
        let activation_capture = RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["captured".to_string()],
            body: Box::new(RuntimeExpr::Var(0)),
        };
        let source = Box::leak(Box::new(bind_continuation_fixture(
            captured_bind_closure_with_captures(1, vec![activation_capture]),
        )));
        let mut plan = plan_static_transition_graph(source, &BTreeMap::new())
            .expect("the activation-capturing bind fixture plans");
        let (seat, resume_site, position, owner, environment) = direct_bind_coordinates(&plan);
        assert_direct_bind_is_authorized(&plan, owner, seat, &environment);
        let environment_record = plan
            .aggregate_ownership
            .get(environment.record().0 as usize)
            .expect("the environment record remains present");
        assert_eq!(
            environment_record.meet,
            PlannedReferentLifetime::ActivationOwned,
            "the fixture must make the environment lifetime genuinely activation-owned"
        );
        let response_record = plan
            .aggregate_ownership
            .iter_mut()
            .find(|record| record.producer == AggregateOccurrenceProducer::Source(resume_site))
            .expect("the response constructor retains its ownership record");
        let paired_field = response_record
            .children
            .iter_mut()
            .find(|child| child.position as usize == position && child.origin == Some(seat))
            .expect("the response record retains the exact recursive field");
        assert_eq!(
            paired_field.lifetime,
            PlannedReferentLifetime::ActivationOwned
        );
        paired_field.lifetime = PlannedReferentLifetime::Persistent;

        assert_eq!(
            plan.boundary_closure_environment_by_record(environment.record())
                .expect("the environment descriptor remains resolvable"),
            environment
        );
        assert_eq!(
            plan.boundary_bind_continuation_environment_by_record(environment.record())
                .expect("the mismatched-lifetime production query is valid"),
            None,
            "a response field that cannot carry the environment must restore the refusal"
        );
    }

    /// **A MALFORMED ENVELOPE MUST REFUSE, NOT READ AS A NON-MEMBER.**
    ///
    /// ⛔ This is the control for the fail-open defect introduced while
    /// implementing the fix for fail-open. The issuance site matched
    /// `let Ok(envelope) = unit.ordinary_envelope() else { return Ok(None) }`,
    /// which turned EVERY envelope failure into "not in this domain". With the
    /// defect armed, planning silently produced zero records and every other
    /// test here stayed green, because each one asks about records that exist.
    /// Only a test that arms an integrity fault and demands a REFUSAL can see
    /// it.
    #[test]
    fn an_integrity_defect_in_the_envelope_refuses_rather_than_dropping_the_unit() {
        let expr = Box::leak(Box::new(
            super::super::continuations::tests::contspec_activation_owned_worker_captures_fixture(),
        ));
        let baseline = plan_static_transition_graph(expr, &BTreeMap::new())
            .expect("the fixture plans when the envelope is exact");
        assert!(
            !checked_ih_records(&baseline).is_empty(),
            "the armed run below is only meaningful because the unarmed one issues records"
        );

        set_envelope_defect(EnvelopeDefect::SelectionOutOfRange);
        let armed = plan_static_transition_graph(expr, &BTreeMap::new());
        set_envelope_defect(EnvelopeDefect::Exact);

        assert!(
            armed.is_err(),
            "an envelope integrity defect must fail plan construction; swallowing it and \
             issuing no record is the fail-open shape this slice exists to close"
        );
    }
}
