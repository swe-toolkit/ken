//! Compile-time response-producer to static-continuation feasibility.
//!
//! This module derives the relation from both ends before any response-owner
//! Function exists. The producer half starts at a closed `ITree::Vis` operation
//! and resolves the matching host-response case. The continuation half starts
//! at the planner-issued continuation specialization and retains its worker,
//! context, captures, continuation inputs, and opaque call identity.

use std::collections::{BTreeMap, BTreeSet};

use super::abi::{AbiFrameHeader, AbiSlot, AbiSlotKind};
use super::continuations::{
    continuation_owner_entry_sources, generated_context_parameters,
    walk_continuation_value_environment, ContinuationCallIdentity,
    ContinuationContextId,
    ContinuationEmissionOwner, ContinuationInputProjection, ContinuationSourceCoordinate,
    ContinuationSpecializationId, ContinuationValueSourceAuthority,
    ContinuationWorkerCaptureSource, ContinuationWorkerProvenance, PlannedContinuationContext,
};
use super::occurrences::StaticOriginId;
use super::semantic_ir::ConstructorIdentity;
use super::{planner_capacity_error, planner_error, CraneliftBackendError, StaticTransitionPlan};
use crate::{CheckedComputationalIHInvocationKind, HostOpV1, RuntimeExpr, RuntimeSymbol, RuntimeValue};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct StaticResponseContinuationId(u32);

impl StaticResponseContinuationId {
    fn from_position(position: usize) -> Result<Self, CraneliftBackendError> {
        Ok(Self(u32::try_from(position).map_err(|_| {
            planner_capacity_error("static response continuation identity exhausted")
        })?))
    }

    pub(in crate::cranelift_backend) fn ordinal(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticResponseCapture {
    ordinal: u32,
    origin: StaticOriginId,
    source: ContinuationSourceCoordinate,
    producer_abi_slot: u32,
}

impl StaticResponseCapture {
    pub(in crate::cranelift_backend) fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(in crate::cranelift_backend) fn origin(&self) -> StaticOriginId {
        self.origin
    }

    pub(in crate::cranelift_backend) fn source(&self) -> ContinuationSourceCoordinate {
        self.source
    }

    pub(in crate::cranelift_backend) fn producer_abi_slot(&self) -> u32 {
        self.producer_abi_slot
    }
}

/// A response edge's request for the ordinary continuation context identified
/// by `(k_specialization, k_body_origin)`.
///
/// This record does not mint a context identity or describe a second ABI. Its
/// worker and continuation-input fields are cloned from the already-validated K
/// specialization solely so union interning can reject a same-key schema
/// disagreement before assigning an ordinary [`ContinuationContextId`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum StaticResponseFrameSource {
    Parameter(u32),
    Capture(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticResponseEnvironmentBinding {
    source: ContinuationSourceCoordinate,
    frame_source: StaticResponseFrameSource,
}

impl StaticResponseEnvironmentBinding {
    pub(in crate::cranelift_backend) fn source(&self) -> ContinuationSourceCoordinate {
        self.source
    }

    pub(in crate::cranelift_backend) fn frame_source(&self) -> StaticResponseFrameSource {
        self.frame_source
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum StaticResponseEffectInput {
    Frame(StaticResponseEnvironmentBinding),
    /// The exact `buffer_nat_to_int` normalization inside a static FsWriteAt
    /// request. The owner reads the already-validated span field directly;
    /// this is not a general admission of carried Nat constructor matching.
    BoundedNatToInt {
        span: StaticResponseEnvironmentBinding,
        span_identity: ConstructorIdentity,
    },
    /// An operation argument lowered from its retained source occurrence using
    /// only the explicitly mapped owner-frame environment.
    OperationArgument {
        origin: StaticOriginId,
        environment: Vec<StaticResponseEnvironmentBinding>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticResponseContextDemand {
    id: StaticResponseContinuationId,
    base_owner: ContinuationEmissionOwner,
    producer_call_origin: StaticOriginId,
    response_origin: StaticOriginId,
    effect_origin: StaticOriginId,
    operation_root_origin: StaticOriginId,
    effect_source_owner: super::units::PredeclaredFunctionId,
    operation_source_owner: super::units::PredeclaredFunctionId,
    effect_environment: Vec<StaticResponseEffectInput>,
    vis_origin: StaticOriginId,
    operation: HostOpV1,
    k_identity: ContinuationCallIdentity,
    k_specialization: ContinuationSpecializationId,
    k_closure_origin: StaticOriginId,
    k_body_origin: StaticOriginId,
    k_ret_identity: ConstructorIdentity,
    raw_owner: super::units::PredeclaredFunctionId,
    worker: ContinuationWorkerProvenance,
    captures: Vec<StaticResponseCapture>,
    continuation_inputs: Vec<(u32, ContinuationSourceCoordinate, u32)>,
    context_inputs: Vec<ContinuationInputProjection>,
}

impl StaticResponseContextDemand {
    pub(in crate::cranelift_backend) fn id(&self) -> StaticResponseContinuationId {
        self.id
    }

    pub(in crate::cranelift_backend) fn base_owner(&self) -> ContinuationEmissionOwner {
        self.base_owner
    }

    pub(in crate::cranelift_backend) fn producer_call_origin(&self) -> StaticOriginId {
        self.producer_call_origin
    }

    pub(in crate::cranelift_backend) fn response_origin(&self) -> StaticOriginId {
        self.response_origin
    }

    pub(in crate::cranelift_backend) fn effect_origin(&self) -> StaticOriginId {
        self.effect_origin
    }

    pub(in crate::cranelift_backend) fn effect_source_owner(
        &self,
    ) -> super::units::PredeclaredFunctionId {
        self.effect_source_owner
    }

    pub(in crate::cranelift_backend) fn operation_source_owner(
        &self,
    ) -> super::units::PredeclaredFunctionId {
        self.operation_source_owner
    }

    pub(in crate::cranelift_backend) fn effect_environment(
        &self,
    ) -> &[StaticResponseEffectInput] {
        &self.effect_environment
    }

    pub(in crate::cranelift_backend) fn vis_origin(&self) -> StaticOriginId {
        self.vis_origin
    }

    pub(in crate::cranelift_backend) fn operation(&self) -> HostOpV1 {
        self.operation
    }

    pub(in crate::cranelift_backend) fn k_identity(&self) -> &ContinuationCallIdentity {
        &self.k_identity
    }

    pub(in crate::cranelift_backend) fn k_specialization(&self) -> ContinuationSpecializationId {
        self.k_specialization
    }

    pub(in crate::cranelift_backend) fn k_closure_origin(&self) -> StaticOriginId {
        self.k_closure_origin
    }

    pub(in crate::cranelift_backend) fn k_body_origin(&self) -> StaticOriginId {
        self.k_body_origin
    }

    pub(in crate::cranelift_backend) fn k_ret_identity(&self) -> ConstructorIdentity {
        self.k_ret_identity
    }

    pub(in crate::cranelift_backend) fn captures(&self) -> &[StaticResponseCapture] {
        &self.captures
    }

    pub(in crate::cranelift_backend) fn continuation_inputs(
        &self,
    ) -> &[(u32, ContinuationSourceCoordinate, u32)] {
        &self.continuation_inputs
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticResponseContinuation {
    id: StaticResponseContinuationId,
    base_owner: ContinuationEmissionOwner,
    producer_call_origin: StaticOriginId,
    response_origin: StaticOriginId,
    effect_origin: StaticOriginId,
    operation_root_origin: StaticOriginId,
    effect_source_owner: super::units::PredeclaredFunctionId,
    operation_source_owner: super::units::PredeclaredFunctionId,
    effect_environment: Vec<StaticResponseEffectInput>,
    vis_origin: StaticOriginId,
    operation: HostOpV1,
    k_identity: ContinuationCallIdentity,
    k_specialization: ContinuationSpecializationId,
    k_closure_origin: StaticOriginId,
    k_body_origin: StaticOriginId,
    k_ret_identity: ConstructorIdentity,
    k_context: ContinuationContextId,
    context_was_preexisting: bool,
    captures: Vec<StaticResponseCapture>,
    continuation_inputs: Vec<(u32, ContinuationSourceCoordinate, u32)>,
}

impl StaticResponseContinuation {
    pub(in crate::cranelift_backend) fn id(&self) -> StaticResponseContinuationId {
        self.id
    }

    pub(in crate::cranelift_backend) fn base_owner(&self) -> ContinuationEmissionOwner {
        self.base_owner
    }

    pub(in crate::cranelift_backend) fn producer_call_origin(&self) -> StaticOriginId {
        self.producer_call_origin
    }

    pub(in crate::cranelift_backend) fn response_origin(&self) -> StaticOriginId {
        self.response_origin
    }

    pub(in crate::cranelift_backend) fn effect_origin(&self) -> StaticOriginId {
        self.effect_origin
    }

    pub(in crate::cranelift_backend) fn effect_source_owner(
        &self,
    ) -> super::units::PredeclaredFunctionId {
        self.effect_source_owner
    }

    pub(in crate::cranelift_backend) fn operation_source_owner(
        &self,
    ) -> super::units::PredeclaredFunctionId {
        self.operation_source_owner
    }

    pub(in crate::cranelift_backend) fn effect_environment(
        &self,
    ) -> &[StaticResponseEffectInput] {
        &self.effect_environment
    }

    pub(in crate::cranelift_backend) fn vis_origin(&self) -> StaticOriginId {
        self.vis_origin
    }

    pub(in crate::cranelift_backend) fn operation(&self) -> HostOpV1 {
        self.operation
    }

    pub(in crate::cranelift_backend) fn k_identity(&self) -> &ContinuationCallIdentity {
        &self.k_identity
    }

    pub(in crate::cranelift_backend) fn k_specialization(&self) -> ContinuationSpecializationId {
        self.k_specialization
    }

    pub(in crate::cranelift_backend) fn k_closure_origin(&self) -> StaticOriginId {
        self.k_closure_origin
    }

    pub(in crate::cranelift_backend) fn k_body_origin(&self) -> StaticOriginId {
        self.k_body_origin
    }

    pub(in crate::cranelift_backend) fn k_ret_identity(&self) -> ConstructorIdentity {
        self.k_ret_identity
    }

    pub(in crate::cranelift_backend) fn k_context(&self) -> ContinuationContextId {
        self.k_context
    }

    pub(in crate::cranelift_backend) fn context_was_preexisting(&self) -> bool {
        self.context_was_preexisting
    }

    pub(in crate::cranelift_backend) fn captures(&self) -> &[StaticResponseCapture] {
        &self.captures
    }

    pub(in crate::cranelift_backend) fn continuation_inputs(
        &self,
    ) -> &[(u32, ContinuationSourceCoordinate, u32)] {
        &self.continuation_inputs
    }
}

/// The classify verdict for one response `ITree::Vis` occurrence (recut
/// `evt_5yjjsrhpmt204` + amendment `evt_4ar3rxzrra5v4`). Computed ONCE at
/// planning (R2) and consumed by total matches at every downstream stage, so a
/// stage that fails to reconcile the residual is a Rust compile error
/// (COORDINATION §7), never a CI-red. `Specialized` is the proved path
/// (P0 = a continuation unit AND a selected caller that will be consumed as a
/// real `DirectCall`/`ComposedCall`); `Deferred` is the complete residual
/// (P1 ∪ P2). D0 holds: only `Deferred` is tagged; the Specialized side is
/// unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ResponseDisposition {
    Specialized,
    Deferred,
}

/// Which residual sub-case a `Deferred` response is, kept for congruence
/// evidence (AC-1) and control fixtures — never a routing key (both sub-cases
/// route identically to main's pre-WP lowering).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum DeferredResponseSubCase {
    /// P1 — no continuation unit for this `Vis` (`matching.is_empty()`): the
    /// `1229` absent complement Q1 declined. There is no static continuation to
    /// name and no owner; main already lowers the `Vis` construct.
    NoContinuationUnit,
    /// P2 — a continuation unit exists, but the selected caller is a checked-IH
    /// environment transport source (settles `TransportDormant`, never
    /// retargeted to a real call): the HS3-a/HS3-b present-but-unconsumed
    /// placeholder, now classified `Deferred` up front so no owner and no
    /// `StaticResponseDeferred` placeholder are ever emitted for it.
    UnconsumedTransportCaller,
}

/// One response `Vis` classified `Deferred` (recut amendment
/// `evt_4ar3rxzrra5v4`). It acquires no response owner and no
/// `StaticResponseDeferred` placeholder; its operation root and host effect fall
/// through to main's pre-WP lowering (R3). The row is POPULATED (never an
/// absence), so `classify` is congruent (AC-1) and every consumer reconciles the
/// residual by total match rather than reconstructing it from local negative
/// evidence (R2).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct DeferredResponseRow {
    vis_origin: StaticOriginId,
    /// The producer call origin this Deferred residual belongs to (P1: the route's
    /// producer edge; P2: the demand's). Carried so the fan-out-accounting control
    /// can group the deferred residual by producer -- the RECUT 2 HS6 (ii)-redesign
    /// re-targets that invariant onto the Deferred population (Architect
    /// evt_2fk574v1cb3b1), where the shared-producer multi-K witness now lives
    /// (the transport-deferred ResourceRelease pairs). Deterministic from the
    /// causal route/demand, so the closed-derivation validator is unaffected.
    producer_call_origin: StaticOriginId,
    operation_root_origin: StaticOriginId,
    effect_origin: StaticOriginId,
    operation: HostOpV1,
    sub_case: DeferredResponseSubCase,
    /// The K's capture and continuation-input counts (P2: from the demand; P1:
    /// zero -- no continuation unit). Carried so the capture/input census control
    /// can cross-check the DropEvery{Capture,Input} mutation's `applications`
    /// against the FULL has-K-unit demand population (Specialized rows expose their
    /// own counts; the P2 Deferred demands' counts live only here). RECUT 2 HS6
    /// (ii)-redesign 2nd extension (Architect evt_bk6vky2pkncy). Deterministic from
    /// the demand -> outcome-neutral to the closed-derivation validator (the sort
    /// key omits it, unique vis_origin keeps the sort total).
    capture_count: usize,
    continuation_input_count: usize,
}

impl DeferredResponseRow {
    pub(in crate::cranelift_backend) fn vis_origin(&self) -> StaticOriginId {
        self.vis_origin
    }

    pub(in crate::cranelift_backend) fn producer_call_origin(&self) -> StaticOriginId {
        self.producer_call_origin
    }

    pub(in crate::cranelift_backend) fn capture_count(&self) -> usize {
        self.capture_count
    }

    pub(in crate::cranelift_backend) fn continuation_input_count(&self) -> usize {
        self.continuation_input_count
    }

    pub(in crate::cranelift_backend) fn operation_root_origin(&self) -> StaticOriginId {
        self.operation_root_origin
    }

    pub(in crate::cranelift_backend) fn effect_origin(&self) -> StaticOriginId {
        self.effect_origin
    }

    pub(in crate::cranelift_backend) fn operation(&self) -> HostOpV1 {
        self.operation
    }

    pub(in crate::cranelift_backend) fn sub_case(&self) -> DeferredResponseSubCase {
        self.sub_case
    }
}

/// Phase-A carry of the two-phase response context install (RECUT 2, HS5). Built
/// by [`StaticTransitionPlan::install_static_response_context_plan`] at
/// construction.rs:1213 and consumed by
/// [`StaticTransitionPlan::install_static_response_context_plan_phase_b`]
/// post-:1251. `demands` is the whole has-K-unit population (owner-additive, no
/// Specialized/Deferred split yet); `deferred` holds P1 only; `preexisting_count`
/// is the causal context prefix length that phase B's owner resolution needs.
#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct StaticResponsePhaseA {
    demands: Vec<StaticResponseContextDemand>,
    preexisting_count: usize,
    deferred: Vec<DeferredResponseRow>,
}

/// Identity of one compile-time response-owner function. This domain is
/// deliberately non-convertible to continuation/context identities: an owner
/// implements one selected incoming edge and later calls a K context; it is not
/// a second spelling for either endpoint.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct StaticResponseOwnerId(u32);

impl StaticResponseOwnerId {
    fn from_position(position: usize) -> Result<Self, CraneliftBackendError> {
        Ok(Self(u32::try_from(position).map_err(|_| {
            planner_capacity_error("static response owner identity exhausted")
        })?))
    }

    pub(in crate::cranelift_backend) fn ordinal(self) -> u32 {
        self.0
    }
}

/// The forward-declaration and selected-caller contract for one response owner.
/// Its activation ABI is exactly the selected K specialization ABI; CP2 adds no
/// closure, selector, environment aggregate, tag, or runtime route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticResponseOwnerSpecialization {
    id: StaticResponseOwnerId,
    base_owner: ContinuationEmissionOwner,
    response: StaticResponseContinuationId,
    selected_caller: ContinuationCallIdentity,
    k_context: ContinuationContextId,
    context_was_preexisting: bool,
    header: AbiFrameHeader,
    slots: Vec<AbiSlot>,
}

impl StaticResponseOwnerSpecialization {
    pub(in crate::cranelift_backend) fn id(&self) -> StaticResponseOwnerId {
        self.id
    }

    pub(in crate::cranelift_backend) fn base_owner(&self) -> ContinuationEmissionOwner {
        self.base_owner
    }

    pub(in crate::cranelift_backend) fn response(&self) -> StaticResponseContinuationId {
        self.response
    }

    pub(in crate::cranelift_backend) fn selected_caller(&self) -> &ContinuationCallIdentity {
        &self.selected_caller
    }

    pub(in crate::cranelift_backend) fn k_context(&self) -> ContinuationContextId {
        self.k_context
    }

    pub(in crate::cranelift_backend) fn context_was_preexisting(&self) -> bool {
        self.context_was_preexisting
    }

    pub(in crate::cranelift_backend) fn header(&self) -> AbiFrameHeader {
        self.header
    }

    pub(in crate::cranelift_backend) fn slots(&self) -> &[AbiSlot] {
        &self.slots
    }

    pub(in crate::cranelift_backend) fn slot_offsets(
        &self,
    ) -> Result<(Vec<u32>, u32), CraneliftBackendError> {
        super::abi::slot_offsets(&self.slots)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct SsaInfeasible {
    base_owner: ContinuationEmissionOwner,
    vis_origin: StaticOriginId,
    producer_call_origin: Option<StaticOriginId>,
    operation: Option<HostOpV1>,
    k_closure_origin: Option<StaticOriginId>,
    k_body_origin: Option<StaticOriginId>,
    k_capture_count: Option<usize>,
    continuation_input_count: Option<usize>,
    reason: &'static str,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticResponseContextDemandMutation {
    DeleteResponseOnlyDemand,
    DuplicateResponseOnlyDemand,
    VaryKSpecialization,
    VaryKBody,
    VaryCaptureSource,
    VaryContinuationInputSource,
    DropProducerKRow,
    DuplicateProducerKRow,
    VaryProducerKRow,
    MergeTwoKKeys,
    SubstituteResponseWithOperation,
    SubstituteResponseWithPriorResponse,
    SubstituteResponseWithApplicationEnvironment,
    DropEveryCapture,
    PermuteEveryCapture,
    VaryEveryCapture,
    DropEveryContinuationInput,
    PermuteEveryContinuationInput,
    VaryEveryContinuationInput,
    VaryCausalContextPrefix,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION:
        std::cell::Cell<Option<StaticResponseContextDemandMutation>> = const { std::cell::Cell::new(None) };
    static STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION_APPLICATIONS:
        std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_static_response_context_demand_mutation<T>(
    mutation: StaticResponseContextDemandMutation,
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION.with(|slot| {
        assert!(
            slot.replace(Some(mutation)).is_none(),
            "static response context-demand mutations cannot nest"
        );
    });
    STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION_APPLICATIONS.with(|count| count.set(0));
    let result = operation();
    let applications =
        STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION_APPLICATIONS.with(std::cell::Cell::get);
    STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION.with(|slot| {
        assert_eq!(
            slot.replace(None),
            Some(mutation),
            "static response context-demand mutation changed during its window"
        );
    });
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
pub fn static_response_context_demand_mutation_is_exact() -> bool {
    STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION.with(|slot| slot.get().is_none())
}

/// AC-7 mutation hook (Architect ruling `evt_37dx1wqamabg`). When set, `classify`
/// FORCE-classifies a P2 residual (a response whose selected caller is a
/// checked-IH environment transport source, which never retargets to a real
/// call) as Specialized instead of Deferred -- injecting the FM1 error at the
/// production/planning site. The forced Specialized owner is forward-declared but
/// its transport caller is never consumed, so the EXISTING pin
/// `validate_response_owner_call_coverage` must redden downstream. This is the
/// mutation-provenance proof that that pin bites a CLASSIFY error (mutate at the
/// production site, watch the guard redden), discharging AC-7 without a redundant
/// second assertion. It is NOT a soundness change: production never sets it.
#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static FORCE_SPECIALIZE_DEFERRED_RESPONSE: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_force_specialize_deferred_response<T>(operation: impl FnOnce() -> T) -> T {
    FORCE_SPECIALIZE_DEFERRED_RESPONSE.with(|slot| {
        assert!(
            !slot.replace(true),
            "force-specialize-deferred-response mutations cannot nest"
        );
    });
    let result = operation();
    FORCE_SPECIALIZE_DEFERRED_RESPONSE.with(|slot| slot.set(false));
    result
}

#[cfg(feature = "px8-ds-test-support")]
pub fn force_specialize_deferred_response_is_exact() -> bool {
    FORCE_SPECIALIZE_DEFERRED_RESPONSE.with(|slot| !slot.get())
}

impl SsaInfeasible {
    fn at_vis(
        base_owner: ContinuationEmissionOwner,
        vis_origin: StaticOriginId,
        producer_call_origin: Option<StaticOriginId>,
        reason: &'static str,
    ) -> Self {
        Self {
            base_owner,
            vis_origin,
            producer_call_origin,
            operation: None,
            k_closure_origin: None,
            k_body_origin: None,
            k_capture_count: None,
            continuation_input_count: None,
            reason,
        }
    }

    fn with_operation(mut self, operation: HostOpV1) -> Self {
        self.operation = Some(operation);
        self
    }

    fn with_k(
        mut self,
        operation: HostOpV1,
        k_closure_origin: StaticOriginId,
        k_body_origin: StaticOriginId,
        k_capture_count: usize,
        continuation_input_count: usize,
    ) -> Self {
        self.operation = Some(operation);
        self.k_closure_origin = Some(k_closure_origin);
        self.k_body_origin = Some(k_body_origin);
        self.k_capture_count = Some(k_capture_count);
        self.continuation_input_count = Some(continuation_input_count);
        self
    }

    pub(in crate::cranelift_backend) fn base_owner(&self) -> ContinuationEmissionOwner {
        self.base_owner
    }

    pub(in crate::cranelift_backend) fn vis_origin(&self) -> StaticOriginId {
        self.vis_origin
    }

    pub(in crate::cranelift_backend) fn producer_call_origin(&self) -> Option<StaticOriginId> {
        self.producer_call_origin
    }

    pub(in crate::cranelift_backend) fn operation(&self) -> Option<HostOpV1> {
        self.operation
    }

    pub(in crate::cranelift_backend) fn k_closure_origin(&self) -> Option<StaticOriginId> {
        self.k_closure_origin
    }

    pub(in crate::cranelift_backend) fn k_body_origin(&self) -> Option<StaticOriginId> {
        self.k_body_origin
    }

    pub(in crate::cranelift_backend) fn k_capture_count(&self) -> Option<usize> {
        self.k_capture_count
    }

    pub(in crate::cranelift_backend) fn continuation_input_count(&self) -> Option<usize> {
        self.continuation_input_count
    }

    pub(in crate::cranelift_backend) fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy)]
struct HostResponseRoute {
    operation: HostOpV1,
    effect_origin: StaticOriginId,
    producer_call_origin: StaticOriginId,
    response_origin: StaticOriginId,
}

fn checked_host_response_call(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
) -> Result<Option<(StaticOriginId, StaticOriginId)>, CraneliftBackendError> {
    let expr = plan.planned_occurrence_expr(origin)?;
    let child = |position| plan.semantic.child_origin(origin, position);
    match expr {
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. } => {
            checked_host_response_call(plan, child(0)?)
        }
        RuntimeExpr::CheckedComputationalIHInvocation { kind, .. } => {
            if *kind != CheckedComputationalIHInvocationKind::CheckedHostVisContinuation {
                return Ok(None);
            }
            let call_origin = child(0)?;
            let RuntimeExpr::Call { args, .. } = plan.planned_occurrence_expr(call_origin)? else {
                return Err(planner_error(
                    "a checked host-Vis continuation does not contain its source Call",
                ));
            };
            if args.len() != 1 {
                return Err(planner_error(
                    "a checked host-Vis continuation Call does not take exactly one response",
                ));
            }
            let response_origin = plan.semantic.child_origin(call_origin, 1)?;
            if !matches!(
                plan.planned_occurrence_expr(response_origin)?,
                RuntimeExpr::Var(0)
            ) {
                return Err(planner_error(
                    "a checked host-Vis continuation Call does not receive its enclosing host response",
                ));
            }
            Ok(Some((call_origin, response_origin)))
        }
        _ => Ok(None),
    }
}

fn host_response_routes(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeMap<RuntimeSymbol, HostResponseRoute>, CraneliftBackendError> {
    let mut routes = BTreeMap::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Match { cases, .. } = occurrence.expr else {
            continue;
        };
        for (alternative, case) in cases.iter().enumerate() {
            let body = plan
                .semantic
                .child_origin(occurrence.static_origin, 1 + alternative)?;
            let RuntimeExpr::Let { .. } = plan.planned_occurrence_expr(body)? else {
                continue;
            };
            let effect_origin = plan.semantic.child_origin(body, 0)?;
            let RuntimeExpr::Effect { operation, .. } =
                plan.planned_occurrence_expr(effect_origin)?
            else {
                continue;
            };
            let continuation_origin = plan.semantic.child_origin(body, 1)?;
            let Some((producer_call_origin, response_origin)) =
                checked_host_response_call(plan, continuation_origin)?
            else {
                continue;
            };
            let route = HostResponseRoute {
                operation: *operation,
                effect_origin,
                producer_call_origin,
                response_origin,
            };
            if routes.insert(case.constructor.clone(), route).is_some() {
                return Err(planner_error(
                    "two host response cases claim one operation constructor",
                ));
            }
        }
    }
    Ok(routes)
}

fn selected_host_response_route(
    plan: &StaticTransitionPlan<'_>,
    operation_origin: StaticOriginId,
    routes: &BTreeMap<RuntimeSymbol, HostResponseRoute>,
) -> Result<Option<(HostResponseRoute, StaticOriginId)>, CraneliftBackendError> {
    let mut selected = None;
    let mut pending = vec![operation_origin];
    while let Some(origin) = pending.pop() {
        if let RuntimeExpr::Construct { constructor, .. } = plan.planned_occurrence_expr(origin)? {
            if let Some(route) = routes.get(constructor).copied() {
                if selected.is_some() {
                    return Err(planner_error(
                        "one Vis operation subtree selects more than one host response producer",
                    ));
                }
                selected = Some((route, origin));
            }
        }
        pending.extend(plan.semantic.child_origins(origin)?.iter().copied());
    }
    Ok(selected)
}

fn validate_static_response_demand_closure(
    expected: &[StaticResponseContextDemand],
    reached: &[StaticResponseContextDemand],
) -> Result<(), CraneliftBackendError> {
    let expected = expected
        .iter()
        .map(|demand| (demand.id, demand))
        .collect::<BTreeMap<_, _>>();
    let mut reached_ids = BTreeMap::new();
    for demand in reached {
        let expected_demand = expected.get(&demand.id).ok_or_else(|| {
            planner_error("a response context demand names no derived response row")
        })?;
        if **expected_demand != *demand {
            return Err(planner_error(
                "a response context demand disagrees with its fully validated response row",
            ));
        }
        if reached_ids
            .insert(demand.id, demand)
            .is_some_and(|prior| prior != demand)
        {
            return Err(planner_error(
                "duplicate response context demands disagree on one response row",
            ));
        }
    }
    if reached_ids.len() != expected.len() {
        return Err(planner_error(
            "the response context demand population does not cover every derived response row",
        ));
    }
    Ok(())
}

fn exact_capture_source(
    plan: &StaticTransitionPlan<'_>,
    owner: super::units::PredeclaredFunctionId,
    capture_origin: StaticOriginId,
) -> Result<Result<ContinuationSourceCoordinate, &'static str>, CraneliftBackendError> {
    let source_root = super::continuations::continuation_owner_source_root(plan, owner)?;
    let entry_environment = continuation_owner_entry_sources(plan, owner)?
        .into_iter()
        .map(ContinuationValueSourceAuthority::source)
        .collect::<Vec<_>>();
    let (_, reached) =
        walk_continuation_value_environment(plan, source_root, capture_origin, &entry_environment)?;
    let reached = reached.ok_or_else(|| {
        planner_error("a static response capture is outside its source owner subtree")
    })?;
    let RuntimeExpr::Var(index) = plan.planned_occurrence_expr(capture_origin)? else {
        return Ok(Err(
            "a K capture is not an explicit source-coordinate alias",
        ));
    };
    let Some(value) = reached.get(*index as usize) else {
        return Ok(Err("a K capture indexes outside its source environment"));
    };
    let ContinuationValueSourceAuthority::Closed(sources) = value else {
        return Ok(Err("a K capture has no closed static source coordinate"));
    };
    if sources.len() != 1 {
        return Ok(Err("a K capture has multiple static source coordinates"));
    }
    Ok(Ok(sources[0].coordinate))
}

fn exact_response_ret_identity(
    plan: &StaticTransitionPlan<'_>,
    continuation_origin: StaticOriginId,
) -> Result<Result<ConstructorIdentity, &'static str>, CraneliftBackendError> {
    let RuntimeExpr::ComputationalMatch { cases, .. } =
        plan.planned_occurrence_expr(continuation_origin)?
    else {
        return Ok(Err(
            "the static response continuation is not an ITree computational eliminator",
        ));
    };
    let matches = cases
        .iter()
        .enumerate()
        .filter(|(_, case)| {
            case.constructor.as_str().ends_with("::ITree::Ret")
                && case.argument_binders == 1
                && case.recursive_positions.is_empty()
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Ok(Err(
            "the static response continuation has no exact one-parameter Ret case",
        ));
    }
    Ok(Ok(
        plan.semantic
            .case_constructor_identity(continuation_origin, matches[0])?,
    ))
}

fn free_environment_indices(
    expr: &RuntimeExpr,
    depth: u32,
    free: &mut BTreeSet<u32>,
) -> Result<(), CraneliftBackendError> {
    let visit = |expr, depth, free: &mut BTreeSet<u32>| {
        free_environment_indices(expr, depth, free)
    };
    match expr {
        RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { body, .. } => {
            visit(body, depth, free)?;
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_) => {}
        RuntimeExpr::Var(index) => {
            if *index >= depth {
                free.insert(index - depth);
            }
        }
        RuntimeExpr::Let { value, body } => {
            visit(value, depth, free)?;
            visit(
                body,
                depth
                    .checked_add(1)
                    .ok_or_else(|| planner_capacity_error("response expression depth exhausted"))?,
                free,
            )?;
        }
        RuntimeExpr::If {
            scrutinee,
            then_expr,
            else_expr,
        } => {
            visit(scrutinee, depth, free)?;
            visit(then_expr, depth, free)?;
            visit(else_expr, depth, free)?;
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            for arg in args {
                visit(arg, depth, free)?;
            }
        }
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            visit(scrutinee, depth, free)?;
            for case in cases {
                let binders = u32::try_from(case.binders).map_err(|_| {
                    planner_capacity_error("response match binder depth exhausted")
                })?;
                visit(
                    &case.body,
                    depth.checked_add(binders).ok_or_else(|| {
                        planner_capacity_error("response match depth exhausted")
                    })?,
                    free,
                )?;
            }
        }
        RuntimeExpr::ComputationalMatch {
            scrutinee, cases, ..
        } => {
            visit(scrutinee, depth, free)?;
            for case in cases {
                let binders = case
                    .argument_binders
                    .checked_add(case.recursive_positions.len())
                    .ok_or_else(|| {
                        planner_capacity_error("response computational binder depth exhausted")
                    })?;
                let binders = u32::try_from(binders).map_err(|_| {
                    planner_capacity_error("response computational binder depth exhausted")
                })?;
                visit(
                    &case.body,
                    depth.checked_add(binders).ok_or_else(|| {
                        planner_capacity_error("response computational depth exhausted")
                    })?,
                    free,
                )?;
            }
        }
        RuntimeExpr::Record { fields } => {
            for (_, value) in fields {
                visit(value, depth, free)?;
            }
        }
        RuntimeExpr::Project { record, .. } => visit(record, depth, free)?,
        RuntimeExpr::Closure { .. } => {}
        RuntimeExpr::LexicalClosure { captures, .. } => {
            for capture in captures {
                visit(capture, depth, free)?;
            }
        }
        RuntimeExpr::Call { callee, args } => {
            visit(callee, depth, free)?;
            for arg in args {
                visit(arg, depth, free)?;
            }
        }
        RuntimeExpr::Effect {
            capability, args, ..
        } => {
            if let Some(capability) = capability {
                visit(&capability.value, depth, free)?;
            }
            for arg in args {
                visit(arg, depth, free)?;
            }
        }
    }
    Ok(())
}

fn frame_binding_for_source(
    source: ContinuationSourceCoordinate,
    available: &[(ContinuationSourceCoordinate, StaticResponseFrameSource)],
) -> Option<StaticResponseEnvironmentBinding> {
    available
        .iter()
        .find(|(candidate, _)| *candidate == source)
        .map(|(_, frame_source)| StaticResponseEnvironmentBinding {
            source,
            frame_source: *frame_source,
        })
}

fn static_response_argument_environment(
    plan: &StaticTransitionPlan<'_>,
    owner: super::units::PredeclaredFunctionId,
    origin: StaticOriginId,
    expr: &RuntimeExpr,
    available: &[(ContinuationSourceCoordinate, StaticResponseFrameSource)],
) -> Result<Result<Vec<StaticResponseEnvironmentBinding>, &'static str>, CraneliftBackendError> {
    let mut free = BTreeSet::new();
    free_environment_indices(expr, 0, &mut free)?;
    let Some(maximum) = free.iter().next_back().copied() else {
        return Ok(Ok(Vec::new()));
    };
    let source_root = super::continuations::continuation_owner_source_root(plan, owner)?;
    let entry_environment = continuation_owner_entry_sources(plan, owner)?
        .into_iter()
        .map(ContinuationValueSourceAuthority::source)
        .collect::<Vec<_>>();
    let (_, reached) =
        walk_continuation_value_environment(plan, source_root, origin, &entry_environment)?;
    let reached = reached.ok_or_else(|| {
        planner_error("a static response argument is outside its source owner subtree")
    })?;
    let mut environment = Vec::new();
    for index in 0..=maximum {
        let authority = reached.get(index as usize).ok_or_else(|| {
            planner_error("a static response argument indexes outside its source environment")
        })?;
        let ContinuationValueSourceAuthority::Closed(sources) = authority else {
            return Ok(Err(
                "a static response argument has an open source authority",
            ));
        };
        if sources.len() != 1 {
            return Ok(Err(
                "a static response argument has an ambiguous source coordinate",
            ));
        }
        let Some(binding) = frame_binding_for_source(sources[0].coordinate, available) else {
            return Ok(Err(
                "a static response argument has no explicit owner-frame input",
            ));
        };
        environment.push(binding);
    }
    Ok(Ok(environment))
}

fn is_exact_bounded_nat_to_int(expr: &RuntimeExpr) -> bool {
    let RuntimeExpr::Call {
        callee,
        args: outer_args,
    } = expr
    else {
        return false;
    };
    let RuntimeExpr::LexicalClosure { params, body, .. } = callee.as_ref() else {
        return false;
    };
    if params.len() != 1 || outer_args.len() != 1 {
        return false;
    }
    let RuntimeExpr::Match {
        scrutinee,
        cases: span_cases,
        ..
    } = &outer_args[0]
    else {
        return false;
    };
    if !matches!(scrutinee.as_ref(), RuntimeExpr::Var(0))
        || span_cases.len() != 1
        || span_cases[0].binders != 3
        || !span_cases[0]
            .constructor
            .as_str()
            .contains("::BufferSpan::")
        || !matches!(span_cases[0].body, RuntimeExpr::Var(2))
    {
        return false;
    }
    let RuntimeExpr::Match { cases, .. } = body.as_ref() else {
        return false;
    };
    let zero = cases.iter().find(|case| {
        case.constructor.as_str().ends_with("::Nat::Zero")
            && case.binders == 0
            && matches!(case.body, RuntimeExpr::Value(crate::RuntimeValue::Int(crate::RuntimeIntV1::Small(0))))
    });
    let suc = cases.iter().find(|case| {
        case.constructor.as_str().ends_with("::Nat::Suc") && case.binders == 1
    });
    if zero.is_none() || suc.is_none() || cases.len() != 2 {
        return false;
    }
    let RuntimeExpr::PrimitiveCall {
        primitive,
        args: suc_args,
    } = &suc.expect("one Suc case").body
    else {
        return false;
    };
    if primitive.symbol.as_str() != "add_int" || suc_args.len() != 2 {
        return false;
    }
    matches!(
        suc_args.as_slice(),
        [
            RuntimeExpr::Call {
                callee,
                args: recursive_args,
            },
            RuntimeExpr::Value(crate::RuntimeValue::Int(crate::RuntimeIntV1::Small(1))),
        ] if matches!(callee.as_ref(), RuntimeExpr::DeclarationRef { symbol } if symbol.as_str().ends_with("::buffer_nat_to_int"))
            && matches!(recursive_args.as_slice(), [RuntimeExpr::Var(0)])
    )
}

fn static_response_effect_environment(
    plan: &StaticTransitionPlan<'_>,
    owner: super::units::PredeclaredFunctionId,
    effect_origin: StaticOriginId,
    actual_operation_origin: StaticOriginId,
    captures: &[StaticResponseCapture],
    continuation_inputs: &[ContinuationInputProjection],
) -> Result<Result<Vec<StaticResponseEffectInput>, &'static str>, CraneliftBackendError> {
    let source_root = super::continuations::continuation_owner_source_root(plan, owner)?;
    let entry_environment = continuation_owner_entry_sources(plan, owner)?
        .into_iter()
        .map(ContinuationValueSourceAuthority::source)
        .collect::<Vec<_>>();
    let (_, reached) = walk_continuation_value_environment(
        plan,
        source_root,
        effect_origin,
        &entry_environment,
    )?;
    let reached = reached.ok_or_else(|| {
        planner_error("a static response effect is outside its source owner subtree")
    })?;
    let mut available = captures
        .iter()
        .map(|capture| {
            (
                capture.source,
                StaticResponseFrameSource::Parameter(capture.producer_abi_slot),
            )
        })
        .chain(continuation_inputs.iter().map(|input| {
            (
                input.coordinate,
                StaticResponseFrameSource::Capture(input.ordinal),
            )
        }))
        .collect::<Vec<_>>();
    available.sort_by_key(|(source, frame_source)| (*source, *frame_source));
    available.dedup();

    let actual_source_owner = plan
        .semantic
        .function_owner(actual_operation_origin)?
        .ok_or_else(|| planner_error("a selected response operation has no source owner"))?;
    let RuntimeExpr::Construct {
        args: actual_arguments,
        ..
    } = plan.planned_occurrence_expr(actual_operation_origin)?
    else {
        return Ok(Err(
            "the selected response operation is not a static constructor",
        ));
    };
    let mut actual_argument_inputs = Vec::with_capacity(actual_arguments.len());
    for (ordinal, argument) in actual_arguments.iter().enumerate() {
        let argument_origin = plan.semantic.child_origin(actual_operation_origin, ordinal)?;
        let environment = match static_response_argument_environment(
            plan,
            actual_source_owner,
            argument_origin,
            argument,
            &available,
        )? {
            Ok(environment) => environment,
            Err(reason) => return Ok(Err(reason)),
        };
        if is_exact_bounded_nat_to_int(argument) {
            let span = environment.first().cloned().ok_or_else(|| {
                planner_error("the exact BoundedNat conversion has no span environment input")
            })?;
            let span_match_origin = plan.semantic.child_origin(argument_origin, 1)?;
            actual_argument_inputs.push(StaticResponseEffectInput::BoundedNatToInt {
                span,
                span_identity: plan
                    .semantic
                    .case_constructor_identity(span_match_origin, 0)?,
            });
        } else {
            actual_argument_inputs.push(StaticResponseEffectInput::OperationArgument {
                origin: argument_origin,
                environment,
            });
        }
    }

    let mut free = BTreeSet::new();
    free_environment_indices(plan.planned_occurrence_expr(effect_origin)?, 0, &mut free)?;
    let Some(maximum) = free.iter().next_back().copied() else {
        return Ok(Ok(Vec::new()));
    };
    let mut bindings = Vec::with_capacity(maximum as usize + 1);
    for index in 0..=maximum {
        let authority = reached.get(index as usize).ok_or_else(|| {
            planner_error("a host response effect indexes outside its source environment")
        })?;
        let ContinuationValueSourceAuthority::Closed(sources) = authority else {
            return Ok(Err(
                "the host response effect environment has an open source authority",
            ));
        };
        if sources.len() != 1 {
            return Ok(Err(
                "the host response effect environment has an ambiguous source coordinate",
            ));
        }
        let source = sources[0].coordinate;
        let input = match source {
            ContinuationSourceCoordinate::ProducerLocal { binding, .. }
                if binding.binding_owner == owner =>
            {
                actual_argument_inputs
                    .get(binding.binding_ordinal as usize)
                    .cloned()
                    .ok_or_else(|| {
                        planner_error(
                            "a host response case binder exceeds the selected operation fields",
                        )
                    })?
            }
            ContinuationSourceCoordinate::EntryAbi { .. }
            | ContinuationSourceCoordinate::ProducerLocal { .. } => {
                let Some(binding) = frame_binding_for_source(source, &available) else {
                    return Ok(Err(
                        "the host response effect environment has no explicit owner-frame input",
                    ));
                };
                StaticResponseEffectInput::Frame(binding)
            }
        };
        bindings.push(input);
    }
    Ok(Ok(bindings))
}

impl StaticTransitionPlan<'_> {
    /// Derive and fully validate every statically attributable response demand.
    ///
    /// This phase intentionally does not ask whether an old causal caller had
    /// already caused the K context to be interned. The response edge is itself
    /// a lawful context demand, so that question belongs to the union interner
    /// below. The outer `Result` is plan integrity; the inner `Result` retains
    /// only the SSA trichotomy's genuinely dynamic or non-expressible arm.
    #[allow(clippy::type_complexity)]
    fn static_response_context_demands_filtered(
        &self,
        operation: Option<HostOpV1>,
        apply_mutation: bool,
    ) -> Result<
        Result<(Vec<StaticResponseContextDemand>, Vec<DeferredResponseRow>), SsaInfeasible>,
        CraneliftBackendError,
    > {
        let routes = host_response_routes(self)?;
        let units = self.continuation_units()?;
        let mut response_vis = Vec::new();
        for occurrence in self.source_occurrences.iter().flatten() {
            let RuntimeExpr::Construct { constructor, args } = occurrence.expr else {
                continue;
            };
            if !constructor.as_str().ends_with("::ITree::Vis") || args.len() != 2 {
                continue;
            }
            let vis_origin = occurrence.static_origin;
            let operation_origin = self.semantic.child_origin(vis_origin, 0)?;
            let Some((route, selected_operation_origin)) =
                selected_host_response_route(self, operation_origin, &routes)?
            else {
                continue;
            };
            if operation.is_some_and(|operation| route.operation != operation) {
                continue;
            }
            // Q1 (Architect re-rule evt_2427xbynt1d2e): classify the incoming K so
            // a genuinely opaque/dynamic edge and a real-but-unspecialized edge no
            // longer share one refusal. The K is the Vis's second argument; an
            // opaque K is `RuntimeValue::Unknown` -- there is no static
            // continuation to name and no existing (main) lowering path, so it is
            // the typed fail-closed hard stop. Any other K is a real static
            // continuation main already lowers.
            let k_is_opaque = matches!(&args[1], RuntimeExpr::Value(RuntimeValue::Unknown));
            response_vis.push((
                vis_origin,
                operation_origin,
                selected_operation_origin,
                route,
                k_is_opaque,
            ));
        }
        response_vis.sort_by_key(
            |(vis_origin, operation_origin, selected_operation_origin, route, _k_is_opaque)| {
                (
                    route.producer_call_origin,
                    *vis_origin,
                    *operation_origin,
                    *selected_operation_origin,
                    route.operation,
                )
            },
        );

        let mut demands = Vec::new();
        // PHASE A (RECUT 2, HS5 two-phase, Architect evt_7eh84c8n6w08e). This
        // pass runs at install (construction.rs:1213), BEFORE aggregate_ownership
        // and the transport records exist, so it CANNOT yet decide P2
        // (transport-caller) membership -- that fact is genuinely post-install
        // (the z1315 cycle). Phase A therefore builds a context demand for EVERY
        // has-K-unit member (owner-additive: the whole P2-union-Specialized
        // context-entry domain) and captures ONLY P1 (no continuation unit) as a
        // Deferred residual here. The Specialized/P2 split and owner assignment
        // happen in phase B (`static_response_phase_b_split`, post-:1251), where
        // the record-derived transport set is final. Congruence over the full
        // response-Vis population (AC-1) and total-match reconciliation (R2/§7)
        // hold across the two phases: phase A's demand domain is has-K-unit, phase
        // B's disposition domain is the whole population (P1 sealed via its own
        // Deferred arm).
        let mut deferred: Vec<DeferredResponseRow> = Vec::new();
        for (vis_origin, operation_root_origin, selected_operation_origin, route, k_is_opaque) in
            response_vis
        {
            let matching = units
                .iter()
                .filter(|unit| unit.producer_construct_origin() == vis_origin)
                .collect::<Vec<_>>();
            if matching.is_empty() {
                if k_is_opaque {
                    // Category (i): a genuinely opaque/dynamic K (RuntimeValue::
                    // Unknown) with no static continuation and no existing main
                    // lowering path. This is the typed fail-closed hard stop and
                    // is unchanged -- a runtime-closure dispatcher would be needed
                    // to lower it, which this WP does not introduce.
                    let owner = self.semantic.function_owner(vis_origin)?.ok_or_else(|| {
                        planner_error("a dynamic response Vis has no predeclared source owner")
                    })?;
                    return Ok(Err(SsaInfeasible::at_vis(
                        ContinuationEmissionOwner::Predeclared(owner),
                        vis_origin,
                        Some(route.producer_call_origin),
                        "an incoming response edge carries an opaque or dynamic K",
                    )
                    .with_operation(route.operation)));
                }
                // P1 (recut amendment evt_4ar3rxzrra5v4): no continuation unit
                // exists for this Vis (the 1229 residual). A real static
                // continuation this WP does not specialize; main already lowers
                // the Vis construct (it compiled and reached its runtime frontier
                // before this WP), so it is Deferred, not an abort. Capture it as
                // a populated residual row (not a bare skip): no demand is built,
                // so it is absent from `static_response_continuations` -- every
                // `is_static_response_*` predicate is false for it and its
                // operation root / effect fall through to main's pre-WP lowering.
                deferred.push(DeferredResponseRow {
                    vis_origin,
                    producer_call_origin: route.producer_call_origin,
                    operation_root_origin,
                    effect_origin: route.effect_origin,
                    operation: route.operation,
                    sub_case: DeferredResponseSubCase::NoContinuationUnit,
                    // P1 has no continuation unit -> no captures/inputs. Excluded
                    // from the census population (no demand), so zero is exact.
                    capture_count: 0,
                    continuation_input_count: 0,
                });
                continue;
            }
            for unit in matching {
                let base_owner = ContinuationEmissionOwner::Specialization(unit.id());
                let continuation_inputs = unit.prefinalization_continuation_inputs()?;
                let infeasible = |reason| {
                    SsaInfeasible::at_vis(
                        base_owner,
                        vis_origin,
                        Some(route.producer_call_origin),
                        reason,
                    )
                    .with_k(
                        route.operation,
                        unit.worker_closure_origin(),
                        unit.worker_body_origin(),
                        unit.worker_capture_count(),
                        continuation_inputs.len(),
                    )
                };
                let k_identity = self
                    .continuation_call_binding_for(
                        vis_origin,
                        unit.continuation_origin(),
                        unit.producer_alternative(),
                        unit.recursive_position(),
                    )?
                    .ok_or_else(|| {
                        planner_error(
                            "a static response producer's own edge has no continuation call identity",
                        )
                    })?;
                // Phase A builds a demand for this has-K-unit member
                // UNCONDITIONALLY (owner-additive). Whether it becomes Specialized
                // (owner assigned) or P2-Deferred (a checked-IH environment
                // transport caller, which never retargets to a real call -- the
                // HS3-b leak shape) is decided in phase B from the post-:1251
                // record-derived transport set, keyed on this demand's `k_identity`.
                // See `static_response_phase_b_split`; AC-7's force-specialize hook
                // also lives there now.
                let k_ret_identity = match exact_response_ret_identity(
                    self,
                    unit.continuation_origin(),
                )? {
                    Ok(identity) => identity,
                    Err(reason) => return Ok(Err(infeasible(reason))),
                };
                let envelope = unit.ordinary_envelope()?;
                let mut captures = Vec::new();
                for (position, member) in envelope.iter().enumerate() {
                    let super::continuations::ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                        ordinal,
                        owner,
                        source,
                        ..
                    } = member
                    else {
                        continue;
                    };
                    let ContinuationWorkerCaptureSource::Lexical(origin) = source else {
                        return Ok(Err(infeasible(
                            "a seeded K capture has no explicit producer-side source coordinate",
                        )));
                    };
                    let source = match exact_capture_source(self, *owner, *origin)? {
                        Ok(source) => source,
                        Err(reason) => return Ok(Err(infeasible(reason))),
                    };
                    captures.push(StaticResponseCapture {
                        ordinal: *ordinal,
                        origin: *origin,
                        source,
                        producer_abi_slot: u32::try_from(position).map_err(|_| {
                            planner_capacity_error("static response capture slot exhausted")
                        })?,
                    });
                }
                if captures
                    .iter()
                    .enumerate()
                    .any(|(ordinal, capture)| capture.ordinal as usize != ordinal)
                {
                    return Err(planner_error(
                        "static response captures are not dense in capture-ordinal order",
                    ));
                }
                let raw_owner = self
                    .semantic
                    .function_owner(unit.worker_body_origin())?
                    .ok_or_else(|| {
                        planner_error("a response K worker body has no predeclared source owner")
                    })?;
                let effect_source_owner = self
                    .semantic
                    .function_owner(route.effect_origin)?
                    .ok_or_else(|| {
                        planner_error("a static response effect has no predeclared source owner")
                    })?;
                let operation_source_owner = self
                    .semantic
                    .function_owner(selected_operation_origin)?
                    .ok_or_else(|| {
                        planner_error(
                            "a selected response operation has no predeclared source owner",
                        )
                    })?;
                let effect_environment = match static_response_effect_environment(
                    self,
                    effect_source_owner,
                    route.effect_origin,
                    selected_operation_origin,
                    &captures,
                    &continuation_inputs,
                )? {
                    Ok(environment) => environment,
                    Err(reason) => return Ok(Err(infeasible(reason))),
                };
                demands.push(StaticResponseContextDemand {
                    id: StaticResponseContinuationId::from_position(demands.len())?,
                    base_owner,
                    producer_call_origin: route.producer_call_origin,
                    response_origin: route.response_origin,
                    effect_origin: route.effect_origin,
                    operation_root_origin,
                    effect_source_owner,
                    operation_source_owner,
                    effect_environment,
                    vis_origin,
                    operation: route.operation,
                    k_identity,
                    k_specialization: unit.id(),
                    k_closure_origin: unit.worker_closure_origin(),
                    k_body_origin: unit.worker_body_origin(),
                    k_ret_identity,
                    raw_owner,
                    worker: unit.key.worker.clone(),
                    captures,
                    continuation_inputs: continuation_inputs
                        .iter()
                        .map(|input| (input.ordinal, input.coordinate, input.ordinary_abi_position))
                        .collect(),
                    context_inputs: unit.key.continuation_inputs.clone(),
                });
            }
        }
        demands.sort_by_key(|demand| {
            (
                demand.producer_call_origin,
                demand.vis_origin,
                demand.k_identity.clone(),
            )
        });
        for (position, demand) in demands.iter_mut().enumerate() {
            demand.id = StaticResponseContinuationId::from_position(position)?;
        }
        let expected_demands = demands.clone();
        #[cfg(feature = "px8-ds-test-support")]
        if operation.is_none() && apply_mutation {
            if let Some(mutation) =
                STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION.with(std::cell::Cell::get)
            {
                let mut target = None;
                for (position, demand) in demands.iter().enumerate() {
                    let context_preexists = self.continuation_contexts.iter().any(|context| {
                        context.enclosing_specialization == demand.k_specialization
                            && context.worker_body_origin == demand.k_body_origin
                    });
                    if !context_preexists {
                        target = Some(position);
                        break;
                    }
                }
                let target = target.ok_or_else(|| {
                    planner_error(
                        "the response-demand mutation found no response-only context demand",
                    )
                })?;
                let mut applications = 1usize;
                match mutation {
                    StaticResponseContextDemandMutation::DeleteResponseOnlyDemand
                    | StaticResponseContextDemandMutation::DropProducerKRow => {
                        demands.remove(target);
                    }
                    StaticResponseContextDemandMutation::DuplicateResponseOnlyDemand => {
                        demands.insert(target + 1, demands[target].clone());
                    }
                    StaticResponseContextDemandMutation::DuplicateProducerKRow => {
                        let mut duplicate = demands[target].clone();
                        duplicate.id = StaticResponseContinuationId::from_position(demands.len())?;
                        demands.insert(target + 1, duplicate);
                    }
                    StaticResponseContextDemandMutation::VaryKSpecialization => {
                        demands[target].k_specialization.0 = demands[target]
                            .k_specialization
                            .0
                            .checked_add(1)
                            .ok_or_else(|| {
                                planner_capacity_error(
                                    "response-demand K mutation identity exhausted",
                                )
                            })?;
                    }
                    StaticResponseContextDemandMutation::VaryKBody => {
                        demands[target].k_body_origin.0 = demands[target]
                            .k_body_origin
                            .0
                            .checked_add(1)
                            .ok_or_else(|| {
                                planner_capacity_error(
                                    "response-demand body mutation identity exhausted",
                                )
                            })?;
                    }
                    StaticResponseContextDemandMutation::VaryProducerKRow => {
                        demands[target].producer_call_origin.0 = demands[target]
                            .producer_call_origin
                            .0
                            .checked_add(1)
                            .ok_or_else(|| {
                                planner_capacity_error(
                                    "response-demand producer mutation identity exhausted",
                                )
                            })?;
                    }
                    StaticResponseContextDemandMutation::MergeTwoKKeys => {
                        let pair = demands
                            .iter()
                            .enumerate()
                            .find_map(|(left, first)| {
                                demands.iter().enumerate().find_map(|(right, second)| {
                                    (left != right
                                        && first.producer_call_origin
                                            == second.producer_call_origin
                                        && (first.k_specialization, first.k_body_origin)
                                            != (second.k_specialization, second.k_body_origin))
                                        .then_some((left, right))
                                })
                            })
                            .ok_or_else(|| {
                                planner_error(
                                    "the two-K merge mutation found no shared producer",
                                )
                            })?;
                        demands[pair.1].k_specialization = demands[pair.0].k_specialization;
                        demands[pair.1].k_body_origin = demands[pair.0].k_body_origin;
                    }
                    StaticResponseContextDemandMutation::SubstituteResponseWithOperation => {
                        demands[target].response_origin = demands[target].operation_root_origin;
                    }
                    StaticResponseContextDemandMutation::SubstituteResponseWithPriorResponse => {
                        let current = demands
                            .iter()
                            .position(|demand| demand.id.ordinal() > 0)
                            .ok_or_else(|| {
                                planner_error(
                                    "the prior-response mutation found no later response row",
                                )
                            })?;
                        demands[current].response_origin =
                            demands[current - 1].response_origin;
                    }
                    StaticResponseContextDemandMutation::SubstituteResponseWithApplicationEnvironment => {
                        let environment = demands[target]
                            .captures
                            .first()
                            .map(|capture| capture.origin)
                            .ok_or_else(|| {
                                planner_error(
                                    "the application-environment mutation found no K capture",
                                )
                            })?;
                        demands[target].response_origin = environment;
                    }
                    StaticResponseContextDemandMutation::VaryCaptureSource => {
                        if demands[target].captures.len() < 2 {
                            return Err(planner_error(
                                "the response-demand capture mutation needs two captures",
                            ));
                        }
                        let first = demands[target].captures[0].source;
                        demands[target].captures[0].source = demands[target].captures[1].source;
                        demands[target].captures[1].source = first;
                    }
                    StaticResponseContextDemandMutation::VaryContinuationInputSource => {}
                    StaticResponseContextDemandMutation::VaryCausalContextPrefix => {
                        applications = 0;
                    }
                    StaticResponseContextDemandMutation::DropEveryCapture
                    | StaticResponseContextDemandMutation::PermuteEveryCapture
                    | StaticResponseContextDemandMutation::VaryEveryCapture => {
                        let mut first_mutated = None;
                        applications = 0;
                        for (row, demand) in expected_demands.iter().enumerate() {
                            if demand.captures.is_empty() {
                                return Err(planner_error(
                                    "the every-capture control reached an empty capture run",
                                ));
                            }
                            for capture in 0..demand.captures.len() {
                                let mut probe = expected_demands.clone();
                                match mutation {
                                    StaticResponseContextDemandMutation::DropEveryCapture => {
                                        probe[row].captures.remove(capture);
                                    }
                                    StaticResponseContextDemandMutation::PermuteEveryCapture => {
                                        let other = (capture + 1) % probe[row].captures.len();
                                        probe[row].captures.swap(capture, other);
                                    }
                                    StaticResponseContextDemandMutation::VaryEveryCapture => {
                                        let other = (capture + 1) % probe[row].captures.len();
                                        probe[row].captures[capture].source =
                                            probe[row].captures[other].source;
                                    }
                                    _ => unreachable!(),
                                }
                                if validate_static_response_demand_closure(
                                    &expected_demands,
                                    &probe,
                                )
                                .is_ok()
                                {
                                    return Err(planner_error(
                                        "an independent capture mutation did not red its row",
                                    ));
                                }
                                first_mutated.get_or_insert(probe);
                                applications += 1;
                            }
                        }
                        demands = first_mutated.ok_or_else(|| {
                            planner_error("the every-capture control reached no capture")
                        })?;
                    }
                    StaticResponseContextDemandMutation::DropEveryContinuationInput
                    | StaticResponseContextDemandMutation::PermuteEveryContinuationInput
                    | StaticResponseContextDemandMutation::VaryEveryContinuationInput => {
                        let mut first_mutated = None;
                        applications = 0;
                        for (row, demand) in expected_demands.iter().enumerate() {
                            if demand.continuation_inputs.is_empty() {
                                return Err(planner_error(
                                    "the every-input control reached an empty input run",
                                ));
                            }
                            for input in 0..demand.continuation_inputs.len() {
                                let mut probe = expected_demands.clone();
                                match mutation {
                                    StaticResponseContextDemandMutation::DropEveryContinuationInput => {
                                        probe[row].continuation_inputs.remove(input);
                                    }
                                    StaticResponseContextDemandMutation::PermuteEveryContinuationInput => {
                                        let other =
                                            (input + 1) % probe[row].continuation_inputs.len();
                                        probe[row].continuation_inputs.swap(input, other);
                                    }
                                    StaticResponseContextDemandMutation::VaryEveryContinuationInput => {
                                        let other =
                                            (input + 1) % probe[row].continuation_inputs.len();
                                        probe[row].continuation_inputs[input].1 =
                                            probe[row].continuation_inputs[other].1;
                                    }
                                    _ => unreachable!(),
                                }
                                if validate_static_response_demand_closure(
                                    &expected_demands,
                                    &probe,
                                )
                                .is_ok()
                                {
                                    return Err(planner_error(
                                        "an independent continuation-input mutation did not red its row",
                                    ));
                                }
                                first_mutated.get_or_insert(probe);
                                applications += 1;
                            }
                        }
                        demands = first_mutated.ok_or_else(|| {
                            planner_error("the every-input control reached no input")
                        })?;
                    }
                }
                STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION_APPLICATIONS
                    .with(|count| count.set(count.get() + applications));
            }
        }
        validate_static_response_demand_closure(&expected_demands, &demands)?;
        demands.sort_by_key(|demand| demand.id);
        demands.dedup();
        #[cfg(feature = "px8-ds-test-support")]
        if operation.is_none()
            && apply_mutation
            && STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION.with(std::cell::Cell::get)
                == Some(StaticResponseContextDemandMutation::VaryContinuationInputSource)
        {
            let target = demands
                .iter_mut()
                .find(|demand| demand.context_inputs.len() >= 2)
                .ok_or_else(|| {
                    planner_error(
                        "the response-demand input mutation needs two continuation inputs",
                    )
                })?;
            let first = target.context_inputs[0].coordinate;
            target.context_inputs[0].coordinate = target.context_inputs[1].coordinate;
            target.context_inputs[1].coordinate = first;
        }
        deferred.sort_by_key(|row| (row.vis_origin, row.operation_root_origin, row.operation));
        Ok(Ok((demands, deferred)))
    }

    /// Intern the union of old causal-call contexts and response demands into a
    /// scratch planner population. Existing planned contexts remain the exact
    /// prefix, so their identities and schemas cannot be renumbered by a
    /// response edge; the installed ABI descriptors on immutable `self` are
    /// untouched. This scratch population is validated before CP2 moves it into
    /// the plan; it is never itself visible to lowering.
    fn response_context_union(
        &self,
        causal_contexts: &[PlannedContinuationContext],
        demands: &[StaticResponseContextDemand],
    ) -> Result<(Vec<PlannedContinuationContext>, usize), CraneliftBackendError> {
        let mut contexts = causal_contexts.to_vec();
        let preexisting_count = contexts.len();
        let mut interned = BTreeMap::new();
        for (position, context) in contexts.iter().enumerate() {
            let key = (context.enclosing_specialization, context.worker_body_origin);
            if interned.insert(key, position).is_some() {
                return Err(planner_error(
                    "two causal generated contexts claim one specialization and worker body",
                ));
            }
        }
        for demand in demands {
            if demand.base_owner
                != ContinuationEmissionOwner::Specialization(demand.k_specialization)
            {
                return Err(planner_error(
                    "a response context demand's owner disagrees with its K specialization",
                ));
            }
            let unit = self
                .continuation_specializations
                .get(demand.k_specialization.0 as usize)
                .ok_or_else(|| {
                    planner_error("a response context demand names an uninstalled specialization")
                })?;
            if unit.id != demand.k_specialization
                || unit.key.worker != demand.worker
                || unit.key.worker.body_origin != demand.k_body_origin
                || unit.key.worker.closure_origin != demand.k_closure_origin
                || self.semantic.function_owner(demand.k_body_origin)? != Some(demand.raw_owner)
                || unit.key.continuation_inputs != demand.context_inputs
            {
                return Err(planner_error(
                    "a response context demand disagrees with its K worker or input schema",
                ));
            }
            let key = (demand.k_specialization, demand.k_body_origin);
            if let Some(position) = interned.get(&key).copied() {
                let context = &contexts[position];
                if context.enclosing_specialization != demand.k_specialization
                    || context.worker_body_origin != demand.k_body_origin
                    || context.raw_owner != demand.raw_owner
                    || context.parameters != generated_context_parameters(&unit.key.worker)?
                    || context.captures != unit.key.continuation_inputs
                {
                    return Err(planner_error(
                        "one continuation context key has disagreeing worker or input schemas",
                    ));
                }
                continue;
            }
            let id = ContinuationContextId::from_position(contexts.len())?;
            contexts.push(PlannedContinuationContext {
                id,
                finalized_availability: Vec::new(),
                enclosing_specialization: demand.k_specialization,
                worker_body_origin: demand.k_body_origin,
                raw_owner: demand.raw_owner,
                parameters: generated_context_parameters(&unit.key.worker)?,
                captures: unit.key.continuation_inputs.clone(),
            });
            interned.insert(key, contexts.len() - 1);
        }
        #[cfg(feature = "px8-ds-test-support")]
        if STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION.with(std::cell::Cell::get)
            == Some(StaticResponseContextDemandMutation::VaryCausalContextPrefix)
            && STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION_APPLICATIONS
                .with(std::cell::Cell::get)
                == 0
        {
            let prefix = contexts.first_mut().ok_or_else(|| {
                planner_error(
                    "the causal-prefix mutation found no pre-existing context",
                )
            })?;
            prefix.worker_body_origin.0 = prefix
                .worker_body_origin
                .0
                .checked_add(1)
                .ok_or_else(|| {
                    planner_capacity_error(
                        "the causal-prefix mutation exhausted the body origin",
                    )
                })?;
            STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION_APPLICATIONS
                .with(|count| count.set(1));
        }
        if contexts.get(..preexisting_count) != Some(causal_contexts) {
            return Err(planner_error(
                "appending response context demands changed a causal context identity or schema",
            ));
        }
        Ok((contexts, preexisting_count))
    }

    fn resolve_static_response_context_demands(
        &self,
        demands: Vec<StaticResponseContextDemand>,
        contexts: &[PlannedContinuationContext],
        preexisting_count: usize,
    ) -> Result<Vec<StaticResponseContinuation>, CraneliftBackendError> {
        let mut resolved = Vec::with_capacity(demands.len());
        for demand in demands {
            let mut found = None;
            for (position, context) in contexts.iter().enumerate() {
                if context.enclosing_specialization == demand.k_specialization
                    && context.worker_body_origin == demand.k_body_origin
                {
                    if found.is_some() {
                        return Err(planner_error(
                            "two union contexts claim one response demand key",
                        ));
                    }
                    found = Some((position, context.id));
                }
            }
            let (position, k_context) = found.ok_or_else(|| {
                planner_error(
                    "a validated response demand did not resolve in the union context population",
                )
            })?;
            resolved.push(StaticResponseContinuation {
                id: demand.id,
                base_owner: demand.base_owner,
                producer_call_origin: demand.producer_call_origin,
                response_origin: demand.response_origin,
                effect_origin: demand.effect_origin,
                operation_root_origin: demand.operation_root_origin,
                effect_source_owner: demand.effect_source_owner,
                operation_source_owner: demand.operation_source_owner,
                effect_environment: demand.effect_environment,
                vis_origin: demand.vis_origin,
                operation: demand.operation,
                k_identity: demand.k_identity,
                k_specialization: demand.k_specialization,
                k_closure_origin: demand.k_closure_origin,
                k_body_origin: demand.k_body_origin,
                k_ret_identity: demand.k_ret_identity,
                k_context,
                context_was_preexisting: position < preexisting_count,
                captures: demand.captures,
                continuation_inputs: demand.continuation_inputs,
            });
        }
        Ok(resolved)
    }

    /// PHASE A of the two-phase response context install (RECUT 2, HS5, Architect
    /// evt_7eh84c8n6w08e). Runs once at construction.rs:1213, after causal
    /// specialization planning closes and BEFORE the context ABI (:1221) and
    /// before `aggregate_ownership`/transports (:1249/:1251) exist. It mints
    /// owner-LESS `PlannedContinuationContext` entries over the whole has-K-unit
    /// population (owner-additive) so the context ABI covers them, and captures
    /// P1 (no continuation unit). It assigns NO owners and installs NO
    /// Specialized/Deferred split -- that is genuinely post-install and is done in
    /// [`Self::install_static_response_context_plan_phase_b`]. A typed SSA refusal
    /// (opaque/dynamic K) publishes no partial rows and leaves the causal context
    /// population intact.
    pub(super) fn install_static_response_context_plan(
        &mut self,
    ) -> Result<(), CraneliftBackendError> {
        if self.static_response_plan_installed
            || self.static_response_phase_a.is_some()
            || self.static_response_infeasible.is_some()
        {
            return Err(planner_error(
                "the static response context plan phase A may run exactly once",
            ));
        }
        let (demands, deferred) = match self.static_response_context_demands_filtered(None, true)? {
            Ok(classified) => classified,
            Err(infeasible) => {
                self.static_response_infeasible = Some(infeasible);
                return Ok(());
            }
        };
        let causal_contexts = self.continuation_contexts.clone();
        let (contexts, preexisting_count) =
            self.response_context_union(&causal_contexts, &demands)?;
        self.continuation_contexts = contexts;
        self.static_response_phase_a = Some(StaticResponsePhaseA {
            demands,
            preexisting_count,
            deferred,
        });
        Ok(())
    }

    /// PHASE B of the two-phase response context install (RECUT 2, HS5). Runs once
    /// after construction.rs:1251, where `aggregate_ownership` and the transport
    /// records are final, so the exact record-derived transport-source set -- the
    /// real Deferred/Specialized discriminator (a coordinate-run source WITH a
    /// transport destination) -- exists. It splits phase A's has-K-unit demands
    /// into Specialized (owner assigned) and P2 (unconsumed transport caller, no
    /// owner), records P1 UNION P2 as the complete Deferred residual, and seals
    /// the install. Owner-ADDITIVE: phase A's context entries are never retracted.
    pub(super) fn install_static_response_context_plan_phase_b(
        &mut self,
    ) -> Result<(), CraneliftBackendError> {
        if self.static_response_plan_installed {
            return Err(planner_error(
                "the static response context plan phase B may run exactly once",
            ));
        }
        if self.static_response_infeasible.is_some() {
            // Phase A refused the whole plane on an opaque/dynamic K; there is no
            // Specialized/Deferred population to split. Seal the install.
            self.static_response_plan_installed = true;
            return Ok(());
        }
        let phase_a = self.static_response_phase_a.take().ok_or_else(|| {
            planner_error(
                "the static response context plan phase B ran before phase A installed the demands",
            )
        })?;
        let (specialized, mut deferred) =
            self.static_response_phase_b_split(phase_a.demands)?;
        deferred.extend(phase_a.deferred);
        deferred.sort_by_key(|row| (row.vis_origin, row.operation_root_origin, row.operation));
        let contexts = self.continuation_contexts.clone();
        let rows = self.resolve_static_response_context_demands(
            specialized,
            &contexts,
            phase_a.preexisting_count,
        )?;
        self.static_response_continuations = rows;
        self.static_response_deferred = deferred;
        self.static_response_plan_installed = true;
        Ok(())
    }

    /// The phase-B Deferred/Specialized split (RECUT 2, HS5). Given phase A's
    /// whole has-K-unit demand population, key each demand's `k_identity` against
    /// the record-derived transport-source set (final post-:1251): a transport
    /// source is P2-Deferred (an unconsumed transport caller, never retargeted to
    /// a real owner call -- the HS3-b leak shape); everything else is Specialized.
    /// The Specialized subset is re-sorted and re-numbered contiguously, so the
    /// owner-row identities are exactly the single-phase result over that subset.
    /// This read is closed for the validator: it depends only on the finalized
    /// transport records, identical at install-phase-B and at validation.
    fn static_response_phase_b_split(
        &self,
        demands: Vec<StaticResponseContextDemand>,
    ) -> Result<(Vec<StaticResponseContextDemand>, Vec<DeferredResponseRow>), CraneliftBackendError>
    {
        let transport_sources = self.checked_ih_environment_transport_source_identities();
        let mut specialized = Vec::new();
        let mut deferred = Vec::new();
        for demand in demands {
            // AC-7 mutation: when the force-specialize hook is set, a transport
            // caller is NOT deferred but built as a Specialized owner instead --
            // injecting the FM1 error (a forward-declared owner whose caller is
            // never consumed). The hook is NOT apply_mutation-gated: it applies
            // identically at install-phase-B and at the validator's re-derivation,
            // so it does NOT trip the closed-derivation validator (which is what
            // distinguishes it from the apply_mutation-gated demand mutations).
            #[cfg(feature = "px8-ds-test-support")]
            let force_specialize = FORCE_SPECIALIZE_DEFERRED_RESPONSE.with(std::cell::Cell::get);
            #[cfg(not(feature = "px8-ds-test-support"))]
            let force_specialize = false;
            if !force_specialize && transport_sources.contains(&demand.k_identity) {
                deferred.push(DeferredResponseRow {
                    vis_origin: demand.vis_origin,
                    producer_call_origin: demand.producer_call_origin,
                    operation_root_origin: demand.operation_root_origin,
                    effect_origin: demand.effect_origin,
                    operation: demand.operation,
                    sub_case: DeferredResponseSubCase::UnconsumedTransportCaller,
                    // P2's demand carries the K's captures/inputs -- the census's
                    // only view of this transport-deferred member's counts.
                    capture_count: demand.captures.len(),
                    continuation_input_count: demand.continuation_inputs.len(),
                });
            } else {
                specialized.push(demand);
            }
        }
        // (ii) HS6 owner-coverage ROUTING (Architect ruling evt_2980vtzybp6bj). A
        // Specialized owner whose selected caller is a checked-IH transport source
        // is, by construction, an owner with NO verified selected incoming call --
        // a transport source never retargets to a real owner call. In a REAL
        // compile this is unreachable: the loop above defers every transport
        // source, so `specialized` never contains one (the gate: owner assignment
        // is `!in transport_sources`, and force_specialize is a test-only hook,
        // default false). It arises ONLY under AC-7's force injection. Catch it
        // HERE, at planning, with the owner-coverage validator's own message, so
        // the force-injected abnormal state reds at its INTENDED validator rather
        // than transitively tripping the internal aggregate_ownership lifetime-meet
        // invariant (closure.rs:2091) first -- failure-mode routing, HS6#1. This
        // does NOT weaken that internal invariant for real compiles (it never
        // fires there, since this red pre-empts the force state and no real
        // compile produces the coexistence). See AC-7
        // (force_specializing_a_deferred_response_reds_the_owner_call_coverage_pin)
        // and lowering's validate_response_owner_call_coverage (the standing guard).
        if let Some(forced) = specialized
            .iter()
            .find(|demand| transport_sources.contains(&demand.k_identity))
        {
            return Err(planner_error(format!(
                "a forward-declared response owner has no verified selected incoming call: its \
                 selected caller {:?} is a checked-IH environment transport source, which never \
                 retargets to a real owner call (RECUT 2 HS6 owner-coverage routing; force-only)",
                forced.k_identity,
            )));
        }
        specialized.sort_by_key(|demand| {
            (
                demand.producer_call_origin,
                demand.vis_origin,
                demand.k_identity.clone(),
            )
        });
        for (position, demand) in specialized.iter_mut().enumerate() {
            demand.id = StaticResponseContinuationId::from_position(position)?;
        }
        Ok((specialized, deferred))
    }

    /// Re-derive the complete post-install response plane from the two phases'
    /// own inputs and assert byte-equality with what was installed -- the
    /// closed-derivation invariant (RECUT 2, HS5, AC-9). Phase A is re-derived
    /// from the causal-prefix context population (the argument); phase B's
    /// Specialized/Deferred split is re-derived from the SAME finalized post-:1251
    /// transport records the install read, so both halves are closed by
    /// same-state re-derivation and a phase-unstable or over-admitting input can
    /// no longer pass.
    pub(super) fn validate_static_response_context_plan(
        &self,
        causal_contexts: &[PlannedContinuationContext],
    ) -> Result<(), CraneliftBackendError> {
        if !self.static_response_plan_installed {
            return Err(planner_error(
                "the final plan carries no installed static response context plan",
            ));
        }
        let (demands, p1_deferred) =
            match self.static_response_context_demands_filtered(None, false)? {
                Ok(classified) => classified,
                Err(infeasible) => {
                    let mut landed_contexts = self.continuation_contexts.clone();
                    for context in &mut landed_contexts {
                        context.finalized_availability.clear();
                    }
                    if self.static_response_infeasible.as_ref() != Some(&infeasible)
                        || !self.static_response_continuations.is_empty()
                        || !self.static_response_deferred.is_empty()
                        || landed_contexts != causal_contexts
                    {
                        return Err(planner_error(
                            "the installed typed SSA refusal disagrees with its complete re-derivation",
                        ));
                    }
                    return Ok(());
                }
            };
        if self.static_response_infeasible.is_some() {
            return Err(planner_error(
                "a feasible response derivation retained a stale typed SSA refusal",
            ));
        }
        // Phase A re-derivation: the owner-less context-entry plane over has-K-unit.
        let (mut expected_contexts, preexisting_count) =
            self.response_context_union(causal_contexts, &demands)?;
        // Phase B re-derivation: split the same demands by the same finalized
        // transport records; Specialized get owners, P1 UNION P2 is the residual.
        let (specialized, mut expected_deferred) = self.static_response_phase_b_split(demands)?;
        expected_deferred.extend(p1_deferred);
        expected_deferred
            .sort_by_key(|row| (row.vis_origin, row.operation_root_origin, row.operation));
        let expected_rows = self.resolve_static_response_context_demands(
            specialized,
            &expected_contexts,
            preexisting_count,
        )?;
        for context in &mut expected_contexts {
            context.finalized_availability.clear();
        }
        let mut landed_contexts = self.continuation_contexts.clone();
        for context in &mut landed_contexts {
            context.finalized_availability.clear();
        }
        if landed_contexts != expected_contexts
            || self.static_response_continuations != expected_rows
            || self.static_response_deferred != expected_deferred
        {
            return Err(planner_error(
                "the installed response context/continuation plane is not its exact closed derivation",
            ));
        }
        Ok(())
    }

    fn static_response_feasibility_ledger_filtered(
        &self,
        operation: Option<HostOpV1>,
    ) -> Result<Result<Vec<StaticResponseContinuation>, SsaInfeasible>, CraneliftBackendError> {
        if !self.static_response_plan_installed {
            return Err(planner_error(
                "the static response feasibility ledger was read before installation",
            ));
        }
        if let Some(infeasible) = &self.static_response_infeasible {
            return Ok(Err(infeasible.clone()));
        }
        Ok(Ok(self
            .static_response_continuations
            .iter()
            .filter(|row| operation.map_or(true, |operation| row.operation == operation))
            .cloned()
            .collect()))
    }

    pub(in crate::cranelift_backend) fn static_response_feasibility_ledger(
        &self,
        operation: HostOpV1,
    ) -> Result<Result<Vec<StaticResponseContinuation>, SsaInfeasible>, CraneliftBackendError> {
        self.static_response_feasibility_ledger_filtered(Some(operation))
    }

    pub(in crate::cranelift_backend) fn static_response_feasibility_ledger_all(
        &self,
    ) -> Result<Result<Vec<StaticResponseContinuation>, SsaInfeasible>, CraneliftBackendError> {
        self.static_response_feasibility_ledger_filtered(None)
    }

    pub(in crate::cranelift_backend) fn is_static_response_effect(
        &self,
        effect_origin: StaticOriginId,
    ) -> bool {
        self.static_response_continuations
            .iter()
            .any(|row| row.effect_origin() == effect_origin)
    }

    pub(in crate::cranelift_backend) fn is_static_response_operation_root(
        &self,
        origin: StaticOriginId,
    ) -> bool {
        self.static_response_continuations
            .iter()
            .any(|row| row.operation_root_origin == origin)
    }

    /// Whether this exact opaque causal edge is selected to enter a compile-time
    /// response owner instead of realizing its required consumer inline.
    pub(in crate::cranelift_backend) fn is_static_response_selected_caller(
        &self,
        identity: &ContinuationCallIdentity,
    ) -> bool {
        self.static_response_continuations
            .iter()
            .any(|row| row.k_identity() == identity)
    }

    /// The complete Deferred residual population (recut amendment
    /// `evt_4ar3rxzrra5v4`), for congruence proofs and control fixtures.
    pub(in crate::cranelift_backend) fn static_response_deferred(&self) -> &[DeferredResponseRow] {
        &self.static_response_deferred
    }

    /// The classify verdict for a response `Vis` keyed by its operation-root
    /// origin (recut amendment `evt_4ar3rxzrra5v4`). `Specialized` when a
    /// specialized continuation row owns the origin, `Deferred` when the residual
    /// population does, `None` when the origin is not a response operation root.
    /// Consumed by the lowering production site as a TOTAL match (§7): adding a
    /// `ResponseDisposition` variant reddens the build there.
    pub(in crate::cranelift_backend) fn response_disposition_at_operation_root(
        &self,
        origin: StaticOriginId,
    ) -> Option<ResponseDisposition> {
        if self
            .static_response_continuations
            .iter()
            .any(|row| row.operation_root_origin == origin)
        {
            Some(ResponseDisposition::Specialized)
        } else if self
            .static_response_deferred
            .iter()
            .any(|row| row.operation_root_origin == origin)
        {
            Some(ResponseDisposition::Deferred)
        } else {
            None
        }
    }

    /// The classify verdict for a response `Vis` keyed by its host-effect origin
    /// (the effects production seat), same contract as
    /// [`Self::response_disposition_at_operation_root`].
    pub(in crate::cranelift_backend) fn response_disposition_at_effect(
        &self,
        effect_origin: StaticOriginId,
    ) -> Option<ResponseDisposition> {
        if self
            .static_response_continuations
            .iter()
            .any(|row| row.effect_origin() == effect_origin)
        {
            Some(ResponseDisposition::Specialized)
        } else if self
            .static_response_deferred
            .iter()
            .any(|row| row.effect_origin() == effect_origin)
        {
            Some(ResponseDisposition::Deferred)
        } else {
            None
        }
    }

    /// Seal every installed response row as one forward-declared response-owner
    /// contract and validate its selected caller against the unchanged K ABI.
    pub(in crate::cranelift_backend) fn static_response_owner_specializations(
        &self,
    ) -> Result<
        Result<Vec<StaticResponseOwnerSpecialization>, SsaInfeasible>,
        CraneliftBackendError,
    > {
        let rows = match self.static_response_feasibility_ledger_all()? {
            Ok(rows) => rows,
            Err(infeasible) => return Ok(Err(infeasible)),
        };
        let ordinary_callers = self.ordinary_continuation_call_identities()?;
        let units = self.continuation_units()?;
        let mut selected_callers = std::collections::BTreeSet::new();
        let mut owners = Vec::with_capacity(rows.len());
        for row in rows {
            if !ordinary_callers.contains(row.k_identity()) {
                return Ok(Err(SsaInfeasible::at_vis(
                    row.base_owner(),
                    row.vis_origin(),
                    Some(row.producer_call_origin()),
                    "the selected incoming caller is not an ordinary callable continuation edge",
                )
                .with_k(
                    row.operation(),
                    row.k_closure_origin(),
                    row.k_body_origin(),
                    row.captures().len(),
                    row.continuation_inputs().len(),
                )));
            }
            if !selected_callers.insert(row.k_identity().clone()) {
                return Ok(Err(SsaInfeasible::at_vis(
                    row.base_owner(),
                    row.vis_origin(),
                    Some(row.producer_call_origin()),
                    "two response owners claim one selected incoming caller",
                )
                .with_k(
                    row.operation(),
                    row.k_closure_origin(),
                    row.k_body_origin(),
                    row.captures().len(),
                    row.continuation_inputs().len(),
                )));
            }
            if row.base_owner()
                != ContinuationEmissionOwner::Specialization(row.k_specialization())
            {
                return Err(planner_error(
                    "a static response row's emission owner disagrees with its K specialization",
                ));
            }
            let unit = units
                .iter()
                .find(|unit| unit.id() == row.k_specialization())
                .ok_or_else(|| {
                    planner_error("a static response row's K has no specialization ABI")
                })?;
            let slots = unit.slots().to_vec();
            let header = unit.header();
            let (offsets, frame_bytes) = super::abi::slot_offsets(&slots)?;
            if offsets.len() != slots.len() || frame_bytes != header.frame_bytes {
                return Err(planner_error(
                    "a response-owner slot walk disagrees with its selected K header",
                ));
            }
            for capture in row.captures() {
                let Some(slot) = slots.get(capture.producer_abi_slot() as usize) else {
                    return Ok(Err(SsaInfeasible::at_vis(
                        row.base_owner(),
                        row.vis_origin(),
                        Some(row.producer_call_origin()),
                        "a K capture has no explicit response-owner Parameter slot",
                    )
                    .with_k(
                        row.operation(),
                        row.k_closure_origin(),
                        row.k_body_origin(),
                        row.captures().len(),
                        row.continuation_inputs().len(),
                    )));
                };
                if slot.kind != AbiSlotKind::Parameter || slot.ordinal != capture.producer_abi_slot()
                {
                    return Ok(Err(SsaInfeasible::at_vis(
                        row.base_owner(),
                        row.vis_origin(),
                        Some(row.producer_call_origin()),
                        "a K capture disagrees with its explicit response-owner Parameter slot",
                    )
                    .with_k(
                        row.operation(),
                        row.k_closure_origin(),
                        row.k_body_origin(),
                        row.captures().len(),
                        row.continuation_inputs().len(),
                    )));
                }
            }
            for (ordinal, _, abi_slot) in row.continuation_inputs() {
                let Some(slot) = slots.get(*abi_slot as usize) else {
                    return Ok(Err(SsaInfeasible::at_vis(
                        row.base_owner(),
                        row.vis_origin(),
                        Some(row.producer_call_origin()),
                        "a continuation input has no explicit response-owner Capture slot",
                    )
                    .with_k(
                        row.operation(),
                        row.k_closure_origin(),
                        row.k_body_origin(),
                        row.captures().len(),
                        row.continuation_inputs().len(),
                    )));
                };
                if slot.kind != AbiSlotKind::Capture || slot.ordinal != *ordinal {
                    return Ok(Err(SsaInfeasible::at_vis(
                        row.base_owner(),
                        row.vis_origin(),
                        Some(row.producer_call_origin()),
                        "a continuation input disagrees with its explicit response-owner Capture slot",
                    )
                    .with_k(
                        row.operation(),
                        row.k_closure_origin(),
                        row.k_body_origin(),
                        row.captures().len(),
                        row.continuation_inputs().len(),
                    )));
                }
            }
            owners.push(StaticResponseOwnerSpecialization {
                id: StaticResponseOwnerId::from_position(owners.len())?,
                base_owner: row.base_owner(),
                response: row.id(),
                selected_caller: row.k_identity().clone(),
                k_context: row.k_context(),
                context_was_preexisting: row.context_was_preexisting(),
                header,
                slots,
            });
        }
        Ok(Ok(owners))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{
        CheckedComputationalIHBinderMorphism, RuntimeMatchCase, RuntimeTrap, RuntimeTrapCode,
        RuntimeValue,
    };

    fn trap() -> RuntimeTrap {
        RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "static response fixture is total".to_string(),
        }
    }

    /// A real opaque recursive field, not a population mutation: the Vis K is
    /// `RuntimeValue::Unknown`, so no continuation specialization can name it.
    #[test]
    fn opaque_k_vis_returns_typed_ssa_infeasible() {
        let response_constructor = "ctor:response::Read".to_string();
        let response_route = RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: response_constructor.clone(),
                args: Vec::new(),
            }),
            cases: vec![RuntimeMatchCase {
                constructor: response_constructor.clone(),
                binders: 1,
                body: RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::Effect {
                        family: "effect:FS".to_string(),
                        operation: HostOpV1::BufferAllocate,
                        capability: None,
                        args: vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
                    }),
                    body: Box::new(RuntimeExpr::CheckedComputationalIHInvocation {
                        call_template_id: 1,
                        checked_occurrence_path: Vec::new(),
                        kind: CheckedComputationalIHInvocationKind::CheckedHostVisContinuation,
                        binder_morphism:
                            CheckedComputationalIHBinderMorphism::identity_for_test(0),
                        body: Box::new(RuntimeExpr::Call {
                            callee: Box::new(RuntimeExpr::Var(0)),
                            args: vec![RuntimeExpr::Var(0)],
                        }),
                    }),
                },
            }],
            default: trap(),
        };
        let declaration =
            super::super::tests::b2o_transparent_declaration(response_route);
        let declarations = BTreeMap::from([(declaration.symbol.as_str(), &declaration)]);
        let vis = RuntimeExpr::Construct {
            constructor: "ctor:fixture::ITree::Vis".to_string(),
            args: vec![
                RuntimeExpr::Construct {
                    constructor: response_constructor,
                    args: Vec::new(),
                },
                RuntimeExpr::Value(RuntimeValue::Unknown),
            ],
        };
        let plan = super::super::plan_static_transition_graph(&vis, &declarations)
            .expect("the opaque-K response fixture plans");
        let infeasible = plan
            .static_response_feasibility_ledger_all()
            .expect("the opaque-K ledger is structurally valid")
            .expect_err("an opaque K must not receive a static context demand");
        assert_eq!(infeasible.operation(), Some(HostOpV1::BufferAllocate));
        assert_eq!(
            infeasible.reason(),
            "an incoming response edge carries an opaque or dynamic K"
        );
        assert_eq!(infeasible.k_closure_origin(), None);
        assert_eq!(infeasible.k_body_origin(), None);
    }
}
