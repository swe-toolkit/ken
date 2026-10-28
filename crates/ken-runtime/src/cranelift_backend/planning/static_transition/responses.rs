//! Compile-time response-producer to static-continuation feasibility.
//!
//! This module derives the relation from both ends before any response-owner
//! Function exists. The producer half starts at a closed `ITree::Vis` operation
//! and resolves the matching host-response case. The continuation half starts
//! at the planner-issued continuation specialization and retains its worker,
//! context, captures, continuation inputs, and opaque call identity.

use crate::cranelift_backend::surface::LoweringCause;
use std::collections::{BTreeMap, BTreeSet};

use super::abi::{AbiFrameHeader, AbiSlot, AbiSlotKind};
use super::aggregates::{
    checked_ih_post_call_consumer_frames, pair_detached_required_consumer, RequiredConsumerCall,
};
use super::continuations::{
    checked_frame_for_consumer, continuation_call_selected_result_identity_opt,
    continuation_owner_entry_sources, derive_checked_ih_post_call_consumer_chain,
    generated_context_parameters,
    walk_continuation_value_environment, CheckedIhPostCallConsumerStep, ContinuationCallIdentity,
    ContinuationContextId, ContinuationEmissionOwner, ContinuationEnvironmentClaim,
    ContinuationFrameIdentity, ContinuationInputProjection, ContinuationUnitView,
    ContinuationSourceCoordinate,
    ContinuationSpecializationId, ContinuationValueSourceAuthority,
    ContinuationWorkerCaptureSource, ContinuationWorkerProvenance, PlannedContinuationContext,
    RequiredConsumerProjection, SourceReturnContextRole, SourceReturnContextTemplate,
};
use super::occurrences::{occurrence_authority, StaticOriginId};
use super::semantic_ir::ConstructorIdentity;
use super::{
    occurrence_subtree_contains, peel_checked_bracket_operation, planner_capacity_error, planner_error,
    CheckedIhEnvironmentTransport, CraneliftBackendError, StaticTransitionPlan,
};
use crate::cranelift_backend::grafted_spine_control_graph::GraftedSpineFunctionScope;
use crate::cranelift_backend::backend_module;
use crate::{CheckedComputationalIHInvocationKind, HostOpV1, RuntimeExpr, RuntimeSymbol, RuntimeValue};

/// A response site that may carry a `StaticResponseDeferred` placeholder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum StaticResponseSite {
    Effect(StaticOriginId),
    OperationRoot(StaticOriginId),
}

impl StaticResponseSite {
    fn matches(self, effect_origin: StaticOriginId, operation_root_origin: StaticOriginId) -> bool {
        match self {
            Self::Effect(origin) => origin == effect_origin,
            Self::OperationRoot(origin) => origin == operation_root_origin,
        }
    }
}

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
    selected_operation_origin: StaticOriginId,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedIhDetachedConsumedFrame {
    origin: StaticOriginId,
    checked_frame_id: Option<u64>,
}

impl CheckedIhDetachedConsumedFrame {
    pub(in crate::cranelift_backend) fn origin(self) -> StaticOriginId {
        self.origin
    }

    pub(in crate::cranelift_backend) fn checked_frame_id(self) -> Option<u64> {
        self.checked_frame_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedIhDetachedCallerCut {
    selecting_call: ContinuationCallIdentity,
    caller_result_origin: StaticOriginId,
    consumed_continuation_origin: StaticOriginId,
    selected_body_origin: StaticOriginId,
    checked_frame_id: Option<u64>,
    consumed_caller_suffix: Vec<CheckedIhDetachedConsumedFrame>,
    producer_transport: CheckedIhEnvironmentTransport,
    caller_transport: CheckedIhEnvironmentTransport,
}

impl CheckedIhDetachedCallerCut {
    pub(in crate::cranelift_backend) fn selecting_call(&self) -> &ContinuationCallIdentity {
        &self.selecting_call
    }

    pub(in crate::cranelift_backend) fn caller_result_origin(&self) -> StaticOriginId {
        self.caller_result_origin
    }

    pub(in crate::cranelift_backend) fn consumed_continuation_origin(&self) -> StaticOriginId {
        self.consumed_continuation_origin
    }

    pub(in crate::cranelift_backend) fn selected_body_origin(&self) -> StaticOriginId {
        self.selected_body_origin
    }

    pub(in crate::cranelift_backend) fn checked_frame_id(&self) -> Option<u64> {
        self.checked_frame_id
    }

    pub(in crate::cranelift_backend) fn consumed_caller_suffix(
        &self,
    ) -> &[CheckedIhDetachedConsumedFrame] {
        &self.consumed_caller_suffix
    }

    pub(in crate::cranelift_backend) fn producer_transport(
        &self,
    ) -> &CheckedIhEnvironmentTransport {
        &self.producer_transport
    }

    pub(in crate::cranelift_backend) fn caller_transport(&self) -> &CheckedIhEnvironmentTransport {
        &self.caller_transport
    }
}

/// Which exact source-exit suffix remains executable for a required consumer.
/// The arms are explicit planner evidence: lowering never infers one from the
/// presence or absence of a caller cut.
#[derive(Clone, Debug, Eq, PartialEq)]
enum RequiredConsumerIncomingEdgeSelection {
    /// A distinct selected caller completed the prefix before this suffix.
    CallerCompleted {
        caller_cut: CheckedIhDetachedCallerCut,
        caller_exit_index: usize,
        incoming_consumer_edge_index: usize,
    },
    /// The defining call has no caller and completed none of its exit chain.
    SelfDefining {
        incoming_consumer_edge_index: usize,
    },
}

/// The exact before-value transport paired with the selected incoming consumer
/// edge. The defining call is derived from the destination's one transport;
/// lowering has no second call identity it could supply independently.
pub(in crate::cranelift_backend) struct RequiredConsumerIncomingEdge<'plan> {
    destination: &'plan super::aggregates::RequiredConsumerDestination,
    selected_case_exits: &'plan [CheckedIhPostCallConsumerStep],
    selection: &'plan RequiredConsumerIncomingEdgeSelection,
}

impl<'plan> RequiredConsumerIncomingEdge<'plan> {
    pub(in crate::cranelift_backend) fn destination(
        &self,
    ) -> &super::aggregates::RequiredConsumerDestination {
        self.destination
    }

    pub(in crate::cranelift_backend) fn executable_exits(
        &self,
    ) -> &'plan [CheckedIhPostCallConsumerStep] {
        match self.selection {
            RequiredConsumerIncomingEdgeSelection::CallerCompleted {
                caller_exit_index,
                ..
            } => &self.selected_case_exits[*caller_exit_index..],
            RequiredConsumerIncomingEdgeSelection::SelfDefining { .. } => {
                self.selected_case_exits
            }
        }
    }

    pub(in crate::cranelift_backend) fn incoming_consumer_edge_index(&self) -> usize {
        match self.selection {
            RequiredConsumerIncomingEdgeSelection::CallerCompleted {
                incoming_consumer_edge_index,
                ..
            }
            | RequiredConsumerIncomingEdgeSelection::SelfDefining {
                incoming_consumer_edge_index,
            } => *incoming_consumer_edge_index,
        }
    }
}

/// Closed compiler proof joining one detached checked-IH return context to the
/// existing static-response row, owner, complete selected caller and K context.
/// The response owner forwards the same Result word; this record says which
/// already-selected source exit that call completed and which caller suffix
/// remains live. It is not stored in any runtime descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedIhStaticResponseReturnBoundary {
    response: StaticResponseContinuation,
    owner: StaticResponseOwnerSpecialization,
    consumer: CheckedIhPostCallConsumer,
    required_consumer_edge: RequiredConsumerIncomingEdgeSelection,
}

impl CheckedIhStaticResponseReturnBoundary {
    pub(in crate::cranelift_backend) fn response(&self) -> &StaticResponseContinuation {
        &self.response
    }

    pub(in crate::cranelift_backend) fn owner(&self) -> &StaticResponseOwnerSpecialization {
        &self.owner
    }

    pub(in crate::cranelift_backend) fn caller_cut(&self) -> &CheckedIhDetachedCallerCut {
        match &self.required_consumer_edge {
            RequiredConsumerIncomingEdgeSelection::CallerCompleted { caller_cut, .. } => caller_cut,
            RequiredConsumerIncomingEdgeSelection::SelfDefining { .. } => {
                unreachable!("a static-response boundary cannot carry a self-defining edge")
            }
        }
    }

    pub(in crate::cranelift_backend) fn consumer(&self) -> &CheckedIhPostCallConsumer {
        &self.consumer
    }

    pub(in crate::cranelift_backend) fn required_consumer_incoming_edge(
        &self,
    ) -> Result<Option<RequiredConsumerIncomingEdge<'_>>, CraneliftBackendError> {
        let Some(call) = self.consumer.required_consumer() else {
            return Ok(None);
        };
        let RequiredConsumerIncomingEdgeSelection::CallerCompleted {
            caller_cut,
            caller_exit_index,
            incoming_consumer_edge_index,
        } = &self.required_consumer_edge
        else {
            return Err(planner_error(
                "a static-response return boundary carries no caller-completed consumer edge",
            ));
        };
        if caller_cut.producer_transport() != self.consumer.transport()
            || *caller_exit_index
                != checked_ih_post_call_caller_exit_index(&self.consumer, caller_cut)?
            || *incoming_consumer_edge_index
                >= self.consumer.selected_case_exits()[*caller_exit_index..].len()
        {
            return Err(planner_error(
                "a static-response return boundary disagrees with its caller-completed consumer edge",
            ));
        }
        Ok(Some(RequiredConsumerIncomingEdge {
            destination: call.destination(),
            selected_case_exits: self.consumer.selected_case_exits(),
            selection: &self.required_consumer_edge,
        }))
    }

    fn caller_exit_index(&self) -> Result<usize, CraneliftBackendError> {
        let RequiredConsumerIncomingEdgeSelection::CallerCompleted {
            caller_exit_index,
            ..
        } = &self.required_consumer_edge
        else {
            return Err(planner_error(
                "a static-response return boundary carries no caller-completed exit index",
            ));
        };
        Ok(*caller_exit_index)
    }

    pub(in crate::cranelift_backend) fn owner_completed_exits(
        &self,
    ) -> Result<&[CheckedIhPostCallConsumerStep], CraneliftBackendError> {
        Ok(&self.consumer.selected_case_exits()[..self.caller_exit_index()?])
    }

    pub(in crate::cranelift_backend) fn caller_completed_exits(
        &self,
    ) -> Result<&[CheckedIhPostCallConsumerStep], CraneliftBackendError> {
        Ok(&self.consumer.selected_case_exits()[self.caller_exit_index()?..])
    }
}

fn checked_ih_post_call_caller_exit_index(
    consumer: &CheckedIhPostCallConsumer,
    caller_cut: &CheckedIhDetachedCallerCut,
) -> Result<usize, CraneliftBackendError> {
    let mut matching = consumer
        .selected_case_exits()
        .iter()
        .enumerate()
        .filter(|(_, step)| {
            step.occurrence().eliminator_origin()
                == caller_cut.consumed_continuation_origin()
                && step.checked_frame_id() == caller_cut.checked_frame_id()
        });
    let Some((index, _)) = matching.next() else {
        return Err(planner_error(
            "a static-response return boundary has no exact selected-caller exit",
        ));
    };
    if matching.next().is_some() {
        return Err(planner_error(
            "a static-response return boundary repeats its selected-caller exit",
        ));
    }
    Ok(index)
}

fn checked_ih_caller_completed_required_consumer_edge_index(
    plan: &StaticTransitionPlan<'_>,
    consumer: &CheckedIhPostCallConsumer,
    caller_exit_index: usize,
) -> Result<usize, CraneliftBackendError> {
    let required_call = consumer.required_consumer().ok_or_else(|| {
        planner_error("a caller-completed required-consumer edge has no exact destination")
    })?;
    let (consumer_origin, _) = required_call.destination().consumer_occurrence();
    let executable_exits = &consumer.selected_case_exits()[caller_exit_index..];
    let mut matching = Vec::new();
    for (index, step) in executable_exits.iter().enumerate() {
        if occurrence_subtree_contains(
            plan,
            step.occurrence().eliminator_origin(),
            consumer_origin,
        )? {
            matching.push(index);
        }
    }
    match matching.as_slice() {
        [index] => Ok(*index),
        [] => Err(planner_error(
            "a caller-completed required consumer has no exact incoming edge",
        )),
        _ => Err(planner_error(
            "a caller-completed required consumer has more than one incoming edge",
        )),
    }
}

/// One-source transport ownership for a post-call consumer. A required row
/// cannot store a transport beside its call: the call's destination owns that
/// exact transport, so there is no second value that could disagree with it.
#[derive(Clone, Debug, Eq, PartialEq)]
enum CheckedIhPostCallTransport {
    Ordinary(CheckedIhEnvironmentTransport),
    Required(RequiredConsumerCall),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedIhPostCallConsumer {
    transport: CheckedIhPostCallTransport,
    actual_result_identity: ConstructorIdentity,
    demanded_result_identity: ConstructorIdentity,
    consumers: Vec<CheckedIhPostCallConsumerStep>,
    selected_case_exits: Vec<CheckedIhPostCallConsumerStep>,
    detached_return_context: Option<SourceReturnContextTemplate>,
    required_consumer_edge: Option<RequiredConsumerIncomingEdgeSelection>,
}

impl CheckedIhPostCallConsumer {
    pub(in crate::cranelift_backend) fn transport(&self) -> &CheckedIhEnvironmentTransport {
        match &self.transport {
            CheckedIhPostCallTransport::Ordinary(transport) => transport,
            CheckedIhPostCallTransport::Required(call) => {
                call.destination().defining_transport()
            }
        }
    }

    pub(in crate::cranelift_backend) fn actual_result_identity(&self) -> ConstructorIdentity {
        self.actual_result_identity
    }

    pub(in crate::cranelift_backend) fn demanded_result_identity(&self) -> ConstructorIdentity {
        self.demanded_result_identity
    }

    pub(in crate::cranelift_backend) fn consumers(&self) -> &[CheckedIhPostCallConsumerStep] {
        &self.consumers
    }

    pub(in crate::cranelift_backend) fn selected_case_exits(
        &self,
    ) -> &[CheckedIhPostCallConsumerStep] {
        &self.selected_case_exits
    }

    pub(in crate::cranelift_backend) fn detached_return_context(
        &self,
    ) -> Option<&SourceReturnContextTemplate> {
        self.detached_return_context.as_ref()
    }

    pub(in crate::cranelift_backend) fn required_consumer(
        &self,
    ) -> Option<&RequiredConsumerCall> {
        match &self.transport {
            CheckedIhPostCallTransport::Ordinary(_) => None,
            CheckedIhPostCallTransport::Required(call) => Some(call),
        }
    }

    pub(in crate::cranelift_backend) fn required_consumer_incoming_edge(
        &self,
    ) -> Result<Option<RequiredConsumerIncomingEdge<'_>>, CraneliftBackendError> {
        let Some(call) = self.required_consumer() else {
            if self.required_consumer_edge.is_some() {
                return Err(planner_error(
                    "an ordinary post-call consumer carries a required-consumer incoming edge",
                ));
            }
            return Ok(None);
        };
        let selection = self.required_consumer_edge.as_ref().ok_or_else(|| {
            planner_error("an exact required-consumer destination has no explicit incoming edge")
        })?;
        let RequiredConsumerIncomingEdgeSelection::SelfDefining {
            incoming_consumer_edge_index,
        } = selection
        else {
            return Err(planner_error(
                "a defining post-call consumer row carries a fabricated caller-completed edge",
            ));
        };
        if *incoming_consumer_edge_index >= self.selected_case_exits.len() {
            return Err(planner_error(
                "the exact incoming consumer edge is outside its self-defining exit chain",
            ));
        }
        Ok(Some(RequiredConsumerIncomingEdge {
            destination: call.destination(),
            selected_case_exits: &self.selected_case_exits,
            selection,
        }))
    }

    pub(in crate::cranelift_backend) fn required_consumer_executable_suffix(
        &self,
    ) -> Result<Option<&[CheckedIhPostCallConsumerStep]>, CraneliftBackendError> {
        let Some(edge) = self.required_consumer_incoming_edge()? else {
            return Ok(None);
        };
        let index = edge.incoming_consumer_edge_index();
        Ok(Some(&edge.executable_exits()[index..]))
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

    pub(in crate::cranelift_backend) fn operation_root_origin(&self) -> StaticOriginId {
        self.operation_root_origin
    }
}

/// The classify verdict for one response `ITree::Vis` occurrence (recut
/// `evt_5yjjsrhpmt204` + amendment `evt_4ar3rxzrra5v4`). Computed ONCE at
/// planning (R2) and consumed by total matches at every downstream stage, so a
/// stage that fails to reconcile the residual is a Rust compile error
/// (COORDINATION §7), never a CI-red. `Specialized` is the proved path: a
/// continuation unit plus either an ordinary selected caller or an
/// execute-then-resume transport emission that becomes a real owner call.
/// `Deferred` is the complete residual: P1 plus ineligible or suppressed P2.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ResponseDisposition {
    /// Projected from an authenticated region and one exact response key.
    BracketOwned(super::brackets::BracketSettlementId),
    Specialized,
    Deferred,
}

/// Which residual sub-case a `Deferred` response is, kept for congruence
/// evidence (AC-1) and control fixtures — never a routing key (all three
/// sub-cases retain the ordinary Construct lowering unless separately handled).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum DeferredResponseSubCase {
    /// P1 — no continuation unit for this `Vis` (`matching.is_empty()`): the
    /// `1229` absent complement Q1 declined. There is no static continuation to
    /// name and no owner; main already lowers the `Vis` construct.
    NoContinuationUnit,
    /// P2 — a transport-source caller outside the execute-then-resume subset.
    /// An eligible plane contains at least two producer groups whose transport
    /// sources are exclusively predeclared. In a P1-bearing plane only those
    /// groups are promoted; mixed-owner and single-stage groups remain here. The
    /// suppression control also restores this disposition.
    UnconsumedTransportCaller,
    /// The selected producer construct is the emitting continuation worker's
    /// body root. It returns a Vis without an enclosing consumer seat, so no
    /// selected response-owner call can be recorded in that function.
    ContinuationBodyTail,
}

/// One response `Vis` classified `Deferred` (recut amendment
/// `evt_4ar3rxzrra5v4`). It acquires no dedicated response owner and no
/// `StaticResponseDeferred` placeholder. A statically bounded P2 suffix may be
/// dispatched by the same enclosing specialized owner; every other row's
/// operation root and host effect fall through to main's pre-WP lowering (R3).
/// The row is POPULATED (never an
/// absence), so `classify` is congruent (AC-1) and every consumer reconciles the
/// residual by total match rather than reconstructing it from local negative
/// evidence (R2).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct DeferredResponseRow {
    vis_origin: StaticOriginId,
    /// The producer call origin this Deferred residual belongs to (P1: the
    /// route's producer edge; P2: the demand's). Retained for the
    /// closed diagnostic relation and suppression control.
    producer_call_origin: StaticOriginId,
    operation_root_origin: StaticOriginId,
    /// The operation leaf the planner's route selection chose inside the
    /// operation root. A driver descends to it and never past it.
    selected_operation_origin: StaticOriginId,
    effect_origin: StaticOriginId,
    operation: HostOpV1,
    sub_case: DeferredResponseSubCase,
    /// The K's capture and continuation-input counts (P2: from the demand; P1:
    /// zero because no continuation unit exists). Eligible-plane census derives
    /// every has-K count from Specialized rows.
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

    pub(in crate::cranelift_backend) fn selected_operation_origin(&self) -> StaticOriginId {
        self.selected_operation_origin
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

/// One planner-selected transfer of a handler-owned Deferred response into
/// the exact generated context already planned to execute its lexical K.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct HandlerDriveContextRoute {
    pub(in crate::cranelift_backend) vis_origin: StaticOriginId,
    pub(in crate::cranelift_backend) handler: ContinuationEmissionOwner,
    pub(in crate::cranelift_backend) context: ContinuationContextId,
    pub(in crate::cranelift_backend) continuation_origin: StaticOriginId,
    pub(in crate::cranelift_backend) recursive_position: u32,
    pub(in crate::cranelift_backend) worker_body_origin: StaticOriginId,
    /// The context's Capture run, re-planned in its declared order for this
    /// handler's actual defining frame rather than its original caller's.
    pub(in crate::cranelift_backend) capture_claims: Vec<ContinuationEnvironmentClaim>,
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
    /// Phase A's validated S demands retain their K-context authority, but
    /// never enter ordinary Phase-B ownership, Deferred, or candidate rows.
    marked: Vec<StaticResponseContextDemand>,
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

// Test-only decisive applications of the installed phase-B Ret key. A build
// may plan repeatedly; the wrapper gathers every response demand whose route
// changes solely because the pending checked-control Ret disjunct is true.
#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static PENDING_CHECKED_RET_SINK_APPLICATIONS:
        std::cell::RefCell<Option<Vec<(u32, u32, HostOpV1)>>> =
            const { std::cell::RefCell::new(None) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_pending_checked_ret_sink_applications<T>(
    operation: impl FnOnce() -> T,
) -> (T, Vec<(u32, u32, HostOpV1)>) {
    PENDING_CHECKED_RET_SINK_APPLICATIONS.with(|slot| {
        assert!(
            slot.borrow_mut().replace(Vec::new()).is_none(),
            "pending checked Ret sink applications cannot nest"
        );
    });
    let result = operation();
    let applications = PENDING_CHECKED_RET_SINK_APPLICATIONS.with(|slot| {
        slot.borrow_mut().take().expect("pending application window was open")
    });
    (result, applications)
}

// Execute-then-resume materialization control. Production specializes a
// response whose selected caller is a checked-IH environment transport source:
// the existing transport assembly emits a real response-owner call, so the
// owner performs the effect and resumes K before returning. The test-only
// mutation restores the pre-fix P2 deferral in an otherwise closed plane at
// the classification producer. The unchanged parity fixture must then flip
// from exact `InvalidOffset` to the old pre-effect `ResourceBodyResult` trap.
#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static SUPPRESS_EXECUTE_THEN_RESUME_RESPONSE: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_suppressed_execute_then_resume_response<T>(operation: impl FnOnce() -> T) -> T {
    SUPPRESS_EXECUTE_THEN_RESUME_RESPONSE.with(|slot| {
        assert!(
            !slot.replace(true),
            "execute-then-resume response suppression mutations cannot nest"
        );
    });
    let result = operation();
    SUPPRESS_EXECUTE_THEN_RESUME_RESPONSE.with(|slot| slot.set(false));
    result
}

#[cfg(feature = "px8-ds-test-support")]
pub fn suppressed_execute_then_resume_response_is_exact() -> bool {
    SUPPRESS_EXECUTE_THEN_RESUME_RESPONSE.with(|slot| !slot.get())
}

// Separate from execute-then-resume suppression: a relay can require its
// performing owner even where ordinary composed-stage promotion is disabled.
#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static SUPPRESS_RELAY_PROMOTION: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
    static SUPPRESS_RELAY_PROMOTION_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_suppressed_relay_promotion<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    SUPPRESS_RELAY_PROMOTION.with(|slot| {
        assert!(!slot.replace(true), "relay promotion mutations cannot nest");
    });
    SUPPRESS_RELAY_PROMOTION_APPLICATIONS.with(|slot| slot.set(0));
    let result = operation();
    SUPPRESS_RELAY_PROMOTION.with(|slot| slot.set(false));
    let applications = SUPPRESS_RELAY_PROMOTION_APPLICATIONS.with(std::cell::Cell::get);
    (result, applications)
}

/// Typed test-only evidence of a mixed-owner relay refusal at the planner
/// producer; no diagnostic wording or source-number snapshot is its oracle.
#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug)]
pub struct MixedRelayRefusalObservation {
    pub source: u32,
    pub edge_identity: String,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static MIXED_RELAY_REFUSALS:
        std::cell::RefCell<Option<Vec<MixedRelayRefusalObservation>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_mixed_relay_refusal_observations<T>(operation: impl FnOnce() -> T)
    -> (T, Vec<MixedRelayRefusalObservation>)
{
    MIXED_RELAY_REFUSALS.with(|slot| {
        assert!(slot.borrow().is_none(), "mixed relay observations cannot nest");
        *slot.borrow_mut() = Some(Vec::new());
    });
    let result = operation();
    let observations = MIXED_RELAY_REFUSALS.with(|slot| slot.borrow_mut().take().unwrap());
    (result, observations)
}

// AC-1 control: each marked S must stay outside ordinary response ownership.
// At plan close, independently revalidate a scratch plan with one installed
// bracket row illegally reinserted in ordinary ownership. Each S is tested
// separately; a mutation that never reaches a row cannot count as a control.
#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static BRACKET_ORDINARY_OWNER_MUTATION: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
    static BRACKET_ORDINARY_OWNER_MUTATION_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_bracket_ordinary_owner_mutation<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    BRACKET_ORDINARY_OWNER_MUTATION.with(|slot| {
        assert!(!slot.replace(true), "bracket owner mutations cannot nest");
    });
    BRACKET_ORDINARY_OWNER_MUTATION_APPLICATIONS.with(|count| count.set(0));
    let result = operation();
    BRACKET_ORDINARY_OWNER_MUTATION.with(|slot| slot.set(false));
    let applications = BRACKET_ORDINARY_OWNER_MUTATION_APPLICATIONS
        .with(std::cell::Cell::get);
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
pub fn bracket_ordinary_owner_mutation_is_exact() -> bool {
    BRACKET_ORDINARY_OWNER_MUTATION.with(|slot| !slot.get())
}

/// Compile-preserving HS15/HS17 refutations. Each moves one operand of the
/// post-call consumer contract while leaving the ordinary call ABI untouched.
#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum D5bHs17PostCallConsumerMutation {
    Exact,
    DeleteRelation,
    TransplantConsumer,
    RelabelWithoutConsumer,
    SkipDetachedRequiredConsumerSuffix,
    DeleteStaticResponseBoundary,
    TransplantStaticResponseBoundary,
    SubstituteForwardedResultWord,
    ReplayCompletedSelectedExit,
    DropResidualSuffix,
    MintReceiptAtNonEmittingTail,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static D5B_HS17_POST_CALL_CONSUMER_MUTATION:
        std::cell::Cell<D5bHs17PostCallConsumerMutation> =
        const { std::cell::Cell::new(D5bHs17PostCallConsumerMutation::Exact) };
    static D5B_HS17_POST_CALL_CONSUMER_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(feature = "px8-ds-test-support")]
struct D5bHs17PostCallConsumerMutationGuard;

#[cfg(feature = "px8-ds-test-support")]
impl Drop for D5bHs17PostCallConsumerMutationGuard {
    fn drop(&mut self) {
        D5B_HS17_POST_CALL_CONSUMER_MUTATION
            .with(|slot| slot.set(D5bHs17PostCallConsumerMutation::Exact));
    }
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_d5b_hs17_post_call_consumer_mutation<T>(
    mutation: D5bHs17PostCallConsumerMutation,
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    let previous = D5B_HS17_POST_CALL_CONSUMER_MUTATION.with(|slot| slot.replace(mutation));
    assert_eq!(
        previous,
        D5bHs17PostCallConsumerMutation::Exact,
        "HS15/HS17 post-call consumer mutations cannot nest"
    );
    D5B_HS17_POST_CALL_CONSUMER_APPLICATIONS.with(|count| count.set(0));
    let guard = D5bHs17PostCallConsumerMutationGuard;
    let result = operation();
    let applications = D5B_HS17_POST_CALL_CONSUMER_APPLICATIONS.with(std::cell::Cell::get);
    drop(guard);
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
pub fn d5b_hs17_post_call_consumer_mutation_is_exact() -> bool {
    D5B_HS17_POST_CALL_CONSUMER_MUTATION
        .with(|slot| slot.get() == D5bHs17PostCallConsumerMutation::Exact)
}

#[cfg(feature = "px8-ds-test-support")]
pub(in crate::cranelift_backend) fn d5b_hs17_post_call_consumer_mutation(
) -> D5bHs17PostCallConsumerMutation {
    D5B_HS17_POST_CALL_CONSUMER_MUTATION.with(std::cell::Cell::get)
}

#[cfg(feature = "px8-ds-test-support")]
pub(in crate::cranelift_backend) fn record_d5b_hs17_post_call_consumer_application() {
    D5B_HS17_POST_CALL_CONSUMER_APPLICATIONS.with(|count| count.set(count.get().saturating_add(1)));
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
        RuntimeExpr::CheckedBracketFinalizer { .. }
        | RuntimeExpr::CheckedJoinSite { .. }
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

type ResponseParents = BTreeMap<StaticOriginId, (StaticOriginId, usize)>;
type HostResponseRoutes = BTreeMap<StaticOriginId, BTreeMap<RuntimeSymbol, HostResponseRoute>>;

mod relay_settlement;

/// An admission key for a source Vis and enclosing response emission owner.
/// Static producer fusion consumes the forwarded relay at the source, so the
/// admitted owner's returned set omits it. Admitted only with a planned route
/// or planned trap at every planner-issued call edge that consumes the source;
/// each consuming seat branches to that trap when the route is absent.
pub(in crate::cranelift_backend) type RelayPromotionKey =
    (StaticOriginId, ContinuationEmissionOwner, ContinuationCallIdentity);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct RelayDrivePair {
    pub(in crate::cranelift_backend) source: StaticOriginId,
    pub(in crate::cranelift_backend) owner: ContinuationEmissionOwner,
    /// The enclosing response-owner row that minted this pair. A successor's
    /// own base owner is not necessarily the instance that emits it.
    pub(in crate::cranelift_backend) minting_row: StaticResponseContinuationId,
    pub(in crate::cranelift_backend) operation_root_origin: StaticOriginId,
    pub(in crate::cranelift_backend) effect_origin: StaticOriginId,
    pub(in crate::cranelift_backend) operation: HostOpV1,
}

/// An exact emitted clause and the response-owner function entered to reach it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct EmittedRelayMember {
    owner: StaticResponseOwnerId,
    member: StaticResponseContinuationId,
}

/// One performing clause, selected before lowering at an exact pair/edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct RelayRoute {
    pub(in crate::cranelift_backend) relay_route: EmittedRelayMember,
}

impl StaticTransitionPlan<'_> {
    /// The final returned-Vis protocol and checked-IH transports exist here.
    /// A pair's performing member belongs to its selected incoming call edge,
    /// not to a join on the source, owner, or operation spelling.
    pub(in crate::cranelift_backend) fn build_relay_routes(
        &self,
    ) -> Result<BTreeMap<(StaticOriginId, ContinuationEmissionOwner, ContinuationCallIdentity), RelayRoute>, CraneliftBackendError> {
        if self.relay_drive_pairs.is_empty() {
            if !self.pre_schema_relay_promotions.is_empty() {
                return Err(planner_error("pre-schema relay promotion has no installed pair"));
            }
            return Ok(BTreeMap::new());
        }
        let candidates = self.return_protocol_candidate_rows()?;
        // This is the emitter's population, not every planned response row.
        // The protocol cloned into the lowerer is this same plan object; the
        // successor's own base_owner is NOT the function that emits it.
        let rows = self.static_response_feasibility_ledger_all()?.map_err(|infeasible| {
            backend_module(LoweringCause::Fragment, format!(
                "compile-time response specialization is infeasible at {:?}: {}",
                infeasible.vis_origin(), infeasible.reason(),
            ))
        })?;
        let owners = self.static_response_owner_specializations()?.map_err(|infeasible| {
            backend_module(LoweringCause::Fragment, format!(
                "compile-time response specialization is infeasible at {:?}: {}",
                infeasible.vis_origin(), infeasible.reason(),
            ))
        })?;
        if owners.len() != rows.len() {
            return Err(planner_error(
                "the response-owner definition population disagrees with the validated response rows",
            ));
        }
        // Response-owner selection is a function of the exact causal call
        // identity. The owner plan itself rejects a duplicate selected caller.
        let mut by_edge = BTreeMap::new();
        let mut emitted_by_row = BTreeMap::new();
        for (owner, row) in owners.iter().zip(&rows) {
            if owner.response() != row.id()
                || owner.base_owner() != row.base_owner()
                || owner.selected_caller() != row.k_identity()
                || owner.k_context() != row.k_context()
            {
                return Err(planner_error(
                    "a response-owner definition disagrees with its validated response row",
                ));
            }
            if by_edge.insert(owner.selected_caller().clone(), (owner, row)).is_some() {
                return Err(planner_error("two response owners claim one selected incoming caller"));
            }
            if emitted_by_row.insert(row.id(), (owner, row)).is_some() {
                return Err(planner_error("one response row is emitted by two owners"));
            }
        }
        let select = |owner: &StaticResponseOwnerSpecialization,
                      row: &StaticResponseContinuation,
                      pair: &RelayDrivePair|
         -> Result<Option<EmittedRelayMember>, CraneliftBackendError> {
            let mut members = BTreeMap::new();
            members.insert(row.id(), row);
            if let Some(protocol) = self.pending_vis_settlements.protocol.as_ref()
                .and_then(|protocol| protocol.owners.get(&row.id()))
            {
                for context in &protocol.contexts {
                    for member in &context.members {
                        let successor = member.successor.as_ref().ok_or_else(||
                            planner_error("a pending-Vis member has no derived response row"))?;
                        if let Some(previous) = members.insert(successor.id(), successor) {
                            if previous != successor {
                                return Err(planner_error(
                                    "one emitted relay member identity names different response rows",
                                ));
                            }
                        }
                    }
                }
            }
            let matching = members.values()
                .filter(|member| member.effect_origin() == pair.effect_origin
                    && member.operation() == pair.operation)
                .collect::<Vec<_>>();
            match matching.as_slice() {
                [] => Ok(None),
                [member] => Ok(Some(EmittedRelayMember { owner: owner.id(), member: member.id() })),
                _ => Err(planner_error(
                    "relay: forwarded op reaches more than one protocol position of its minting instance; static plan refuses",
                )),
            }
        };
        let calls = self.continuation_calls()?;
        let mut pair_edges = BTreeMap::<
            (StaticOriginId, ContinuationEmissionOwner),
            BTreeSet<ContinuationCallIdentity>,
        >::new();
        for (&pair_key, pair) in &self.relay_drive_pairs {
            for call in &calls {
                if call.producer_construct_origin() == pair.source {
                    pair_edges.entry(pair_key).or_default().insert(call.identity());
                }
            }
        }
        // An active transport consumes its source's checked call identity,
        // even when its destination construct is not that call's producer.
        // Verify that identity against the very four-coordinate binding the
        // computational seats use, rather than guessing from the destination.
        for transport in &self.checked_ih_environment_transports {
            let pair_key = (transport.destination_construct_origin(), transport.destination_owner());
            if !self.relay_drive_pairs.contains_key(&pair_key) {
                continue;
            }
            let identity = transport.source_call_identity();
            let call = calls.iter().find(|call| call.identity() == *identity)
                .ok_or_else(|| planner_error("an admitted active transport has no issued call identity"))?;
            let rebound = self.continuation_call_binding_for(
                call.producer_construct_origin(),
                call.continuation_origin(),
                call.producer_alternative(),
                call.recursive_position(),
            )?;
            if rebound.as_ref() != Some(identity) {
                return Err(planner_error("an admitted active transport call is not bound by its four coordinates"));
            }
            pair_edges.entry(pair_key).or_default().insert(identity.clone());
        }
        if let Some((source, _)) = self.relay_drive_pairs.keys()
            .find(|key| !pair_edges.contains_key(*key))
        {
            return Err(planner_error(format!(
                "an admitted relay pair has no planned call edge (source {source:?})",
            )));
        }
        // This edge set comes from the installed pairs and BOTH actual route
        // enumeration loops, never from the pre-schema fixpoint. In particular
        // an unrepresentable active-transport destination cannot be inferred.
        for (source, owner, identity) in &self.pre_schema_relay_promotions {
            if !pair_edges.get(&(*source, *owner))
                .is_some_and(|edges| edges.contains(identity))
            {
                return Err(planner_error(format!(
                    "promoted relay at {source:?} has no installed pair edge"
                )));
            }
            let Some((selected_owner, row)) = by_edge.get(identity) else {
                return Err(planner_error("promoted relay has no selected response owner at its edge"));
            };
            if row.vis_origin() != *source || selected_owner.selected_caller() != identity {
                return Err(planner_error("promoted relay edge selects another response source"));
            }
        }
        let mut routes = BTreeMap::new();
        for (&key, pair) in &self.relay_drive_pairs {
            let source_rows = candidates.iter()
                .filter(|row| row.vis_origin() == pair.source)
                .collect::<Vec<_>>();
            let [source_row] = source_rows.as_slice() else {
                return Err(planner_error("an admitted relay pair has no unique source response row"));
            };
            if source_row.effect_origin() != pair.effect_origin
                || source_row.operation() != pair.operation
            {
                return Err(planner_error("an admitted relay pair disagrees with its source response operation"));
            }
            let minting_owner = emitted_by_row.get(&pair.minting_row).ok_or_else(||
                planner_error("relay: minting row is not an emitted response owner"))?;
            if minting_owner.1.base_owner() != pair.owner {
                return Err(planner_error("relay: minting row disagrees with its pair owner"));
            }
            for identity in pair_edges.get(&key).into_iter().flat_map(|edges| edges.iter()) {
                let relay_route = if let Some((owner, row)) = by_edge.get(identity) {
                    // The entered owner's own row is the clause at this causal
                    // call edge. Another protocol position can handle the same
                    // operation, but it is not the position this call entered.
                    if row.vis_origin() != pair.source
                        || row.effect_origin() != pair.effect_origin
                        || row.operation() != pair.operation
                    {
                        return Err(planner_error(
                            "relay: entered owner's own row does not record the forwarded source",
                        ));
                    }
                    Some(EmittedRelayMember { owner: owner.id(), member: row.id() })
                } else {
                    // Without an entered owner the minting instance remains
                    // fixed. Its emitted clauses are selected by the checked
                    // operation only when that selector is a function.
                    select(minting_owner.0, minting_owner.1, pair)?
                };
                let member = relay_route.ok_or_else(|| planner_error(format!(
                    "relay: forwarded host op {:?} from source {:?} has no performing route at its exact edge",
                    pair.operation, pair.source,
                )))?;
                if self.pre_schema_relay_promotions.contains(&(
                    pair.source, pair.owner, identity.clone(),
                )) {
                    let (selected_owner, selected_row) = by_edge.get(identity)
                        .ok_or_else(|| planner_error("promoted relay lost its selected response owner"))?;
                    if member.owner != selected_owner.id() || member.member != selected_row.id() {
                        return Err(planner_error("promoted relay edge did not select its own performing owner"));
                    }
                }
                routes.insert((pair.source, pair.owner, identity.clone()),
                    RelayRoute { relay_route: member });
            }
        }
        Ok(routes)
    }

    /// The lowering coverage ledger may discharge an unentered response owner
    /// only for an explicitly promoted source whose Construct was never lowered.
    pub(in crate::cranelift_backend) fn has_pre_schema_relay_promotion(
        &self,
        source: StaticOriginId,
        identity: &ContinuationCallIdentity,
    ) -> bool {
        self.pre_schema_relay_promotions.iter().any(|(paired, _, edge)|
            *paired == source && edge == identity
        )
    }

    pub(in crate::cranelift_backend) fn build_relay_drive_pairs(
        &self,
    ) -> Result<BTreeMap<(StaticOriginId, ContinuationEmissionOwner), RelayDrivePair>, CraneliftBackendError> {
        relay_settlement::build_drive_pairs(self)
    }

    /// The sole authority for relay admission condition (d), not a lowering
    /// drive. Absence never means a key can be invented locally.
    pub(in crate::cranelift_backend) fn relay_drive_pair(
        &self,
        source: StaticOriginId,
        owner: ContinuationEmissionOwner,
    ) -> Result<Option<RelayDrivePair>, CraneliftBackendError> {
        let Some(pair) = self.relay_drive_pairs.get(&(source, owner)).copied() else {
            return Ok(None);
        };
        if pair.source != source || pair.owner != owner
            || self.child_static_origin(source, 0)? != pair.operation_root_origin
            || !self.static_response_continuations.iter().any(|row|
                row.id() == pair.minting_row && row.base_owner() == owner)
        {
            return Err(planner_error("a relay drive pair disagrees with its source and owner"));
        }
        Ok(Some(pair))
    }

    /// An admitted relay operation root stays source material, not a
    /// deferred placeholder. Its pair licenses no in-place response drive;
    /// the installed response owner remains the effect emitter.
    pub(in crate::cranelift_backend) fn relay_drive_operation_root(
        &self,
        root: StaticOriginId,
        owner: Option<ContinuationEmissionOwner>,
    ) -> Result<bool, CraneliftBackendError> {
        let Some(owner) = owner else { return Ok(false); };
        for (&(source, pair_owner), pair) in &self.relay_drive_pairs {
            if pair_owner == owner && pair.operation_root_origin == root {
                return Ok(self.relay_drive_pair(source, owner)?.is_some());
            }
        }
        Ok(false)
    }
}

/// The same classifier supplies pending-Vis projection before emission:
/// admitted relay members are consumed by source fusion, not returned.
pub(in crate::cranelift_backend) fn settled_relay_owners(
    plan: &StaticTransitionPlan<'_>,
) -> BTreeSet<StaticResponseContinuationId> {
    relay_settlement::settled_relay_owners(plan)
}

fn response_parents(plan: &StaticTransitionPlan<'_>) -> Result<ResponseParents, CraneliftBackendError> {
    let mut parents = BTreeMap::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        for (position, child) in plan
            .semantic
            .child_origins(occurrence.static_origin)?
            .iter()
            .copied()
            .enumerate()
        {
            if parents.insert(child, (occurrence.static_origin, position)).is_some() {
                return Err(planner_error("one response source occurrence has two parents"));
            }
        }
    }
    Ok(parents)
}

// A candidate belongs to the Vis whose CM case body encloses it most closely:
// the same ownership relation the dispatch observer checks independently.
fn response_leaf_owner(
    plan: &StaticTransitionPlan<'_>,
    parents: &ResponseParents,
    leaf: StaticOriginId,
) -> Result<Option<(StaticOriginId, usize)>, CraneliftBackendError> {
    let mut cursor = leaf;
    while let Some((parent, position)) = parents.get(&cursor).copied() {
        if let RuntimeExpr::ComputationalMatch { cases, .. } = plan.planned_occurrence_expr(parent)? {
            if position
                .checked_sub(1)
                .and_then(|index| cases.get(index))
                .is_some_and(|case| case.constructor.as_str().ends_with("::ITree::Vis"))
            {
                return Ok(Some((parent, position)));
            }
        }
        cursor = parent;
    }
    Ok(None)
}

// A checked Let->Effect->Call case identifies its own leaf Match. Its
// enclosing Match ancestors need not be generated operation dispatches:
// ordinary source matches use the same runtime-IR constructor and binders.
fn host_response_routes(
    plan: &StaticTransitionPlan<'_>,
) -> Result<HostResponseRoutes, CraneliftBackendError> {
    let mut routes: HostResponseRoutes = BTreeMap::new();
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
            let leaf = occurrence.static_origin;
            if routes.entry(leaf).or_default().insert(case.constructor.clone(), route).is_some() {
                return Err(planner_error(
                    "two host response cases claim one operation constructor",
                ));
            }
        }
    }
    Ok(routes)
}

// construction.rs::plan_expr: Match case bodies (:880-899), Let body
// (:834-852), If branches (:854-879), and ComputationalMatch case bodies
// (:901-951) use their parent's successor. CheckedSubcontinuationFrame[0]
// (:818-831; lowering/source.rs:796-803) enters a checked frame, then
// evaluates the same body and environment. CheckedComputationalIHSlots[0]
// (:818-831; lowering/source.rs:822-827) does likewise. No other child is a
// tail value of its parent: in particular a scrutinee, argument, value, or
// closure/recursive-invocation body cannot carry the Vis operation to a caller.
fn response_tail_edge(parent: &RuntimeExpr, position: usize) -> Option<usize> {
    match parent {
        RuntimeExpr::Match { cases, .. } if position > 0 =>
            cases.get(position - 1).map(|case| case.binders),
        RuntimeExpr::Let { .. } if position == 1 => Some(1),
        RuntimeExpr::If { .. } if position == 1 || position == 2 => Some(0),
        RuntimeExpr::ComputationalMatch { cases, .. } if position > 0 =>
            cases.get(position - 1).and_then(|case| {
                case.argument_binders.checked_add(case.recursive_positions.len())
            }),
        RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
            if position == 0 => Some(0),
        _ => None,
    }
}

// The Vis constructor has operation in constructor field 0 and continuation
// in field 1 (spec/30-surface/36-effects.md §2.1). Lowering/source.rs:
// 2436-2518 installs the IH bindings first, then ordered constructor fields
// (lowering/mod.rs:5644-5690). Compute the operation binder from the same
// layout, not from the Var(1) seen in a particular compiled witness.
fn response_vis_operation_binder(case: &crate::RuntimeComputationalMatchCase) -> Option<u32> {
    if case.argument_binders != 2 || case.recursive_positions.as_slice() != [1] {
        return None;
    }
    u32::try_from(case.recursive_positions.len()).ok()
}

fn response_forwards_vis(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    op_binder: u32,
) -> Result<bool, CraneliftBackendError> {
    match plan.planned_occurrence_expr(origin)? {
        RuntimeExpr::Construct { constructor, args }
            if constructor.as_str().ends_with("::ITree::Vis") && args.len() == 2 =>
        {
            Ok(matches!(&args[0], RuntimeExpr::Var(index) if *index == op_binder))
        }
        RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. } => response_forwards_vis(
            plan,
            plan.semantic.child_origin(origin, 0)?,
            op_binder,
        ),
        RuntimeExpr::If { .. } => Ok(response_forwards_vis(
            plan,
            plan.semantic.child_origin(origin, 1)?,
            op_binder,
        )? && response_forwards_vis(
            plan,
            plan.semantic.child_origin(origin, 2)?,
            op_binder,
        )?),
        RuntimeExpr::Match { cases, .. } if !cases.is_empty() && cases.iter().all(|case| case.binders == 0) => {
            for position in 1..=cases.len() {
                if !response_forwards_vis(plan, plan.semantic.child_origin(origin, position)?, op_binder)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        RuntimeExpr::ComputationalMatch { cases, .. }
            if !cases.is_empty() && cases.iter().all(|case| {
                case.argument_binders == 0 && case.recursive_positions.is_empty()
            }) =>
        {
            for position in 1..=cases.len() {
                if !response_forwards_vis(plan, plan.semantic.child_origin(origin, position)?, op_binder)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

// Follow only zero-binder tail edges from a Vis case to the first Match
// scrutinizing the checked operation binder. A source Match on an unrelated
// scrutinee cannot become the entry just by enclosing a response leaf.
// Return whether EVERY reached tail exit found such an entry; a conditional
// with an unrelated exit is not a proven operation-dispatch flow.
fn response_dispatch_entries(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    binder: u32,
    reached: &mut BTreeSet<StaticOriginId>,
) -> Result<bool, CraneliftBackendError> {
    let mut pending = vec![origin];
    let mut complete = true;
    while let Some(origin) = pending.pop() {
        let expr = plan.planned_occurrence_expr(origin)?;
        if matches!(expr, RuntimeExpr::Match { .. }) {
            let scrutinee = plan.semantic.child_origin(origin, 0)?;
            if matches!(
                plan.planned_occurrence_expr(scrutinee)?,
                RuntimeExpr::Var(index) if *index == binder
            ) {
                reached.insert(origin);
                continue;
            }
        }
        let mut found_tail = false;
        for (position, child) in plan
            .semantic
            .child_origins(origin)?
            .iter()
            .copied()
            .enumerate()
        {
            if response_tail_edge(expr, position) == Some(0) {
                found_tail = true;
                pending.push(child);
            }
        }
        // Even if another branch reaches an entry, this one cannot be
        // silently omitted. Keep traversing to detect competing entries.
        if !found_tail {
            complete = false;
        }
    }
    Ok(complete)
}

// At a dispatch Match, each one-binder case can pass only its freshly bound
// payload, Var(0), to the next Match. Producer keys are checked leaf Matches;
// no generated/source marker or constructor spelling is required.
fn response_dispatch_leaves(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    binder: u32,
    candidates: &BTreeMap<StaticOriginId, HostResponseRoute>,
    reached: &mut BTreeSet<StaticOriginId>,
) -> Result<(), CraneliftBackendError> {
    let RuntimeExpr::Match { cases, .. } = plan.planned_occurrence_expr(origin)? else {
        return Ok(());
    };
    let scrutinee = plan.semantic.child_origin(origin, 0)?;
    if !matches!(
        plan.planned_occurrence_expr(scrutinee)?,
        RuntimeExpr::Var(index) if *index == binder
    ) {
        return Ok(());
    }
    if candidates.contains_key(&origin) {
        reached.insert(origin);
    }
    for (index, case) in cases.iter().enumerate() {
        if case.binders != 1 {
            continue;
        }
        let body = plan.semantic.child_origin(origin, index + 1)?;
        let mut next = BTreeSet::new();
        response_dispatch_entries(plan, body, 0, &mut next)?;
        for child in next {
            response_dispatch_leaves(plan, child, 0, candidates, reached)?;
        }
    }
    Ok(())
}

// Only the multi-candidate population runs this walk. Positional parents
// classify the Vis-to-CM path; at its Vis case, descend by the checked op
// binder, not by the syntax or supposed provenance of Match ancestors.
// A CM scrutinee is an eliminator, never itself a tail edge.
fn response_vis_dispatch_leaf(
    plan: &StaticTransitionPlan<'_>,
    parents: &ResponseParents,
    vis_origin: StaticOriginId,
    candidates: &BTreeMap<StaticOriginId, HostResponseRoute>,
) -> Result<Option<StaticOriginId>, CraneliftBackendError> {
    let mut cursor = vis_origin;
    loop {
        let Some((parent, position)) = parents.get(&cursor).copied() else {
            return Ok(None);
        };
        if let RuntimeExpr::ComputationalMatch { cases, .. } = plan.planned_occurrence_expr(parent)? {
            if position == 0 {
                let mut vis_cases = cases.iter().enumerate().filter(|(_, case)| {
                    case.constructor.as_str().ends_with("::ITree::Vis")
                });
                let Some((case_index, vis_case)) = vis_cases.next() else {
                    return Ok(None);
                };
                if vis_cases.next().is_some() {
                    return Ok(None);
                }
                let Some(op_binder) = response_vis_operation_binder(vis_case) else {
                    return Ok(None);
                };
                let body = plan.semantic.child_origin(parent, case_index + 1)?;
                let mut entries = BTreeSet::new();
                let complete = response_dispatch_entries(plan, body, op_binder, &mut entries)?;
                if !entries.is_empty() {
                    // More than one entry, or a branch without one, represents
                    // alternatives rather than one guaranteed operation flow.
                    if !complete || entries.len() != 1 {
                        return Ok(None);
                    }
                    let mut reached = BTreeSet::new();
                    response_dispatch_leaves(
                        plan,
                        *entries.first().unwrap(),
                        op_binder,
                        candidates,
                        &mut reached,
                    )?;
                    let owner = Some((parent, case_index + 1));
                    let mut owned = BTreeSet::new();
                    for leaf in candidates.keys().copied() {
                        if response_leaf_owner(plan, parents, leaf)? == owner {
                            owned.insert(leaf);
                        }
                    }
                    // An owned candidate the structural descent did not reach
                    // is an unproven alternative for this response.
                    return Ok((reached.len() == 1 && owned == reached)
                        .then(|| *reached.first().unwrap()));
                }
                if !response_forwards_vis(plan, body, op_binder)? {
                    return Ok(None);
                }
                cursor = parent;
                continue;
            }
        }
        if response_tail_edge(plan.planned_occurrence_expr(parent)?, position).is_none() {
            return Ok(None);
        }
        cursor = parent;
    }
}

fn selected_host_response_route(
    plan: &StaticTransitionPlan<'_>,
    vis_origin: StaticOriginId,
    operation_origin: StaticOriginId,
    routes: &HostResponseRoutes,
) -> Result<Option<(HostResponseRoute, StaticOriginId)>, CraneliftBackendError> {
    let mut selected = None;
    let mut pending = vec![operation_origin];
    while let Some(origin) = pending.pop() {
        if let RuntimeExpr::Construct { constructor, .. } = plan.planned_occurrence_expr(origin)? {
            let candidates = routes
                .iter()
                .filter_map(|(leaf, cases)| cases.get(constructor).copied().map(|route| (*leaf, route)))
                .collect::<BTreeMap<_, _>>();
            let route = match candidates.len() {
                0 => None,
                1 => candidates.values().next().copied(),
                _ => {
                    let parents = response_parents(plan)?;
                    let Some(leaf) = response_vis_dispatch_leaf(plan, &parents, vis_origin, &candidates)? else {
                        return Err(planner_error("one response Vis selects a constructor with more than one host response occurrence and no structural path to exactly one of them"));
                    };
                    candidates.get(&leaf).copied()
                }
            };
            if let Some(route) = route {
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

// Observation only: the checked response row already carries the selected
// producer Call. Project its leaf, then independently inspect the selected
// CM's scrutinee subtree; never use the selector's value-flow walk as the
// oracle for containment. This adds no production route or authority.
#[cfg(feature = "px8-ds-test-support")]
impl StaticTransitionPlan<'_> {
    pub(super) fn observed_host_response_dispatch(
        &self,
        vis_origin: StaticOriginId,
        producer_call_origin: StaticOriginId,
    ) -> Result<(StaticOriginId, Option<StaticOriginId>, bool), CraneliftBackendError> {
        let routes = host_response_routes(self)?;
        let mut matching = routes.iter().filter(|(_, cases)| {
            cases.values().any(|route| route.producer_call_origin == producer_call_origin)
        });
        let Some((&leaf, _)) = matching.next() else {
            return Err(planner_error("an observed response call has no dispatch leaf"));
        };
        if matching.next().is_some() {
            return Err(planner_error("an observed response call has multiple dispatch leaves"));
        }
        let parents = response_parents(self)?;
        let mut cursor = leaf;
        while let Some((parent, position)) = parents.get(&cursor).copied() {
            if let RuntimeExpr::ComputationalMatch { cases, .. } =
                self.planned_occurrence_expr(parent)?
            {
                if position.checked_sub(1).and_then(|index| cases.get(index))
                    .is_some_and(|case| case.constructor.as_str().ends_with("::ITree::Vis"))
                {
                    let mut pending = vec![self.semantic.child_origin(parent, 0)?];
                    let mut contains_vis = false;
                    while let Some(origin) = pending.pop() {
                        if origin == vis_origin {
                            contains_vis = true;
                            break;
                        }
                        pending.extend(self.semantic.child_origins(origin)?.iter().copied());
                    }
                    return Ok((leaf, Some(parent), contains_vis));
                }
            }
            cursor = parent;
        }
        Ok((leaf, None, false))
    }
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
        RuntimeExpr::CheckedBracketFinalizer { body, .. }
        | RuntimeExpr::CheckedJoinSite { body, .. }
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
        actual_argument_inputs.push(StaticResponseEffectInput::OperationArgument {
            origin: argument_origin,
            environment,
        });
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

/// Transient proof inputs for recut-A's static single-owner gate.
///
/// `response_uses` is the maximum along one executable branch, not a sum over
/// mutually exclusive case bodies. A handler may mention its response once in
/// every alternative and still consume it exactly once dynamically.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeferredResponseContinuationShape {
    pub(super) k_body_origin: StaticOriginId,
    pub(super) response_uses: usize,
    pub(super) ret_exits: usize,
    pub(super) static_tail_calls: usize,
    pub(super) other_exits: usize,
}

/// Every uniquely resolved declaration tail call in `k_body`'s tail positions
/// targets a body whose own tail exits construct no `ITree` node.
fn deferred_response_tail_callees_answer_directly(
    plan: &StaticTransitionPlan<'_>,
    k_body: StaticOriginId,
) -> Result<bool, CraneliftBackendError> {
    let mut visited = BTreeSet::new();
    tail_answers_directly(plan, k_body, &mut visited, false)
}

fn tail_answers_directly(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    visited: &mut BTreeSet<StaticOriginId>,
    inside_callee: bool,
) -> Result<bool, CraneliftBackendError> {
    let mut each = |positions: &[usize], visited: &mut BTreeSet<StaticOriginId>| {
        for &position in positions {
            let child = plan.semantic.child_origin(origin, position)?;
            if !tail_answers_directly(plan, child, visited, inside_callee)? {
                return Ok(false);
            }
        }
        Ok(true)
    };
    match plan.planned_occurrence_expr(origin)? {
        RuntimeExpr::CheckedBracketFinalizer { .. }
        | RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { .. } => each(&[0], visited),
        RuntimeExpr::Let { .. } => each(&[1], visited),
        RuntimeExpr::If { .. } => each(&[1, 2], visited),
        RuntimeExpr::Match { cases, .. } => each(&(1..=cases.len()).collect::<Vec<_>>(), visited),
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            each(&(1..=cases.len()).collect::<Vec<_>>(), visited)
        }
        RuntimeExpr::Construct { constructor, .. } => Ok(!inside_callee
            || !(constructor.as_str().ends_with("::ITree::Ret")
                || constructor.as_str().ends_with("::ITree::Vis"))),
        RuntimeExpr::Call { .. } => {
            let callee = plan.semantic.child_origin(origin, 0)?;
            let declared = matches!(
                plan.planned_occurrence_expr(callee)?,
                RuntimeExpr::DeclarationRef { .. }
            ) && plan.declaration_call_target_class(callee).is_some();
            let units = plan.emittable_units()?;
            let targets = plan
                .emittable_call_edges()?
                .into_iter()
                .filter(|edge| {
                    edge.kind() == super::units::EmittableCallKind::Declaration
                        && edge.call_site_origin() == callee
                })
                .map(|edge| {
                    units
                        .iter()
                        .find(|unit| unit.function() == edge.callee())
                        .map(|unit| unit.body_occurrence())
                })
                .collect::<Option<Vec<_>>>();
            let (Some(targets), true) = (targets, declared) else {
                return Ok(false);
            };
            let [body] = targets.as_slice() else {
                return Ok(false);
            };
            if !visited.insert(*body) {
                return Ok(true);
            }
            tail_answers_directly(plan, *body, visited, true)
        }
        // A callee tail that forwards a value it received or projected can
        // return an ITree word it never constructed, so inside a callee it
        // does not answer directly. Every other form is named, so a new
        // RuntimeExpr variant has to be classified here.
        RuntimeExpr::Var(_) | RuntimeExpr::Project { .. } => Ok(!inside_callee),
        RuntimeExpr::Value(RuntimeValue::Constructor { constructor, .. }) => Ok(!inside_callee
            || !(constructor.as_str().ends_with("::ITree::Ret")
                || constructor.as_str().ends_with("::ITree::Vis"))),
        RuntimeExpr::Value(_)
        | RuntimeExpr::PrimitiveCall { .. }
        | RuntimeExpr::Record { .. }
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::LexicalClosure { .. }
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Effect { .. }
        | RuntimeExpr::Trap(_) => Ok(true),
    }
}

/// Count uses of lexical parameter zero along the most demanding execution
/// path. Sequential children add; mutually exclusive branches take their max;
/// nested closure bodies have their own coordinate space, so only capture
/// expressions can retain this parameter.
fn count_deferred_response_parameter_uses(
    expr: &RuntimeExpr,
    depth: u32,
) -> Result<usize, CraneliftBackendError> {
    let visit = |expr, depth| count_deferred_response_parameter_uses(expr, depth);
    let sum = |values: Vec<usize>| {
        values.into_iter().try_fold(0usize, |total, value| {
            total
                .checked_add(value)
                .ok_or_else(|| planner_capacity_error("response-use count exhausted"))
        })
    };
    match expr {
        RuntimeExpr::CheckedBracketFinalizer { body, .. }
        | RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { body, .. } => visit(body, depth),
        RuntimeExpr::Value(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_) => Ok(0),
        RuntimeExpr::Var(index) => Ok(usize::from(*index == depth)),
        RuntimeExpr::Let { value, body } => sum(vec![
            visit(value, depth)?,
            visit(
                body,
                depth
                    .checked_add(1)
                    .ok_or_else(|| planner_capacity_error("response-use depth exhausted"))?,
            )?,
        ]),
        RuntimeExpr::If {
            scrutinee,
            then_expr,
            else_expr,
        } => sum(vec![
            visit(scrutinee, depth)?,
            visit(then_expr, depth)?.max(visit(else_expr, depth)?),
        ]),
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => sum(args
            .iter()
            .map(|arg| visit(arg, depth))
            .collect::<Result<Vec<_>, _>>()?),
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            let mut branch_max = 0;
            for case in cases {
                let binders = u32::try_from(case.binders)
                    .map_err(|_| planner_capacity_error("response match depth exhausted"))?;
                branch_max = branch_max.max(visit(
                    &case.body,
                    depth
                        .checked_add(binders)
                        .ok_or_else(|| planner_capacity_error("response match depth exhausted"))?,
                )?);
            }
            sum(vec![visit(scrutinee, depth)?, branch_max])
        }
        RuntimeExpr::ComputationalMatch {
            scrutinee, cases, ..
        } => {
            let mut branch_max = 0;
            for case in cases {
                let binders = case
                    .argument_binders
                    .checked_add(case.recursive_positions.len())
                    .and_then(|count| u32::try_from(count).ok())
                    .ok_or_else(|| {
                        planner_capacity_error("response computational depth exhausted")
                    })?;
                branch_max = branch_max.max(visit(
                    &case.body,
                    depth.checked_add(binders).ok_or_else(|| {
                        planner_capacity_error("response computational depth exhausted")
                    })?,
                )?);
            }
            sum(vec![visit(scrutinee, depth)?, branch_max])
        }
        RuntimeExpr::Record { fields } => sum(fields
            .iter()
            .map(|(_, value)| visit(value, depth))
            .collect::<Result<Vec<_>, _>>()?),
        RuntimeExpr::Project { record, .. } => visit(record, depth),
        // A nested closure body has its own parameter/capture coordinate space.
        // Only its capture expressions can retain this K's response parameter.
        RuntimeExpr::Closure { .. } => Ok(0),
        RuntimeExpr::LexicalClosure { captures, .. } => sum(captures
            .iter()
            .map(|capture| visit(capture, depth))
            .collect::<Result<Vec<_>, _>>()?),
        RuntimeExpr::Call { callee, args } => {
            let mut uses = vec![visit(callee, depth)?];
            uses.extend(
                args.iter()
                    .map(|arg| visit(arg, depth))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            sum(uses)
        }
        RuntimeExpr::Effect {
            capability, args, ..
        } => {
            let mut uses = Vec::new();
            if let Some(capability) = capability {
                uses.push(visit(&capability.value, depth)?);
            }
            uses.extend(
                args.iter()
                    .map(|arg| visit(arg, depth))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            sum(uses)
        }
    }
}

/// Classify every tail exit without following calls. A direct, uniquely
/// resolved declaration call remains a static synchronous tail; a returned
/// closure, effect, indirect call, or other value is outside the bounded class.
fn classify_deferred_response_tail(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    shape: &mut DeferredResponseContinuationShape,
) -> Result<(), CraneliftBackendError> {
    let visit = |origin, shape: &mut DeferredResponseContinuationShape| {
        classify_deferred_response_tail(plan, origin, shape)
    };
    match plan.planned_occurrence_expr(origin)? {
        RuntimeExpr::CheckedBracketFinalizer { .. }
        | RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { .. } => {
            visit(plan.semantic.child_origin(origin, 0)?, shape)?;
        }
        RuntimeExpr::Let { .. } => {
            visit(plan.semantic.child_origin(origin, 1)?, shape)?;
        }
        RuntimeExpr::If { .. } => {
            for position in [1, 2] {
                visit(plan.semantic.child_origin(origin, position)?, shape)?;
            }
        }
        RuntimeExpr::Match { cases, .. } => {
            for alternative in 0..cases.len() {
                visit(plan.semantic.child_origin(origin, 1 + alternative)?, shape)?;
            }
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            for alternative in 0..cases.len() {
                visit(plan.semantic.child_origin(origin, 1 + alternative)?, shape)?;
            }
        }
        RuntimeExpr::Construct { constructor, args }
            if constructor.as_str().ends_with("::ITree::Ret") && args.len() == 1 =>
        {
            shape.ret_exits += 1;
        }
        RuntimeExpr::Trap(_) => {}
        RuntimeExpr::Call { .. } => {
            let callee = plan.semantic.child_origin(origin, 0)?;
            let declared = matches!(
                plan.planned_occurrence_expr(callee)?,
                RuntimeExpr::DeclarationRef { .. }
            ) && plan.declaration_call_target_class(callee).is_some();
            let edges = plan
                .emittable_call_edges()?
                .into_iter()
                .filter(|edge| {
                    edge.kind() == super::units::EmittableCallKind::Declaration
                        && edge.call_site_origin() == callee
                })
                .count();
            if declared && edges == 1 {
                shape.static_tail_calls += 1;
            } else {
                shape.other_exits += 1;
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
        | RuntimeExpr::Effect { .. } => {
            shape.other_exits += 1;
        }
    }
    Ok(())
}

impl StaticTransitionPlan<'_> {
    /// Exact contexts at marked settlement S origins, before any ordinary
    /// response demand or Phase-B owner decision. The marker determines the
    /// population; a host operation is never a membership predicate.
    pub(super) fn marked_bracket_response_contexts(
        &self,
        templates: &[super::brackets::BracketStaticRegion],
    ) -> Result<BTreeMap<StaticOriginId, BTreeSet<super::brackets::BracketResponseContextKey>>,
        CraneliftBackendError>
    {
        use super::brackets::BracketResponseContextKey;
        let units = self.continuation_units()?;
        let mut contexts = BTreeMap::new();
        for template in templates {
            let vis = template.settlement;
            let RuntimeExpr::Construct { args, .. } = self.planned_occurrence_expr(vis)? else {
                return Err(planner_error("a marked settlement is not a response Vis"));
            };
            if args.len() != 2 {
                return Err(planner_error("a marked settlement has no response continuation"));
            }
            let matching = units.iter().filter(|unit| unit.producer_construct_origin() == vis)
                .collect::<Vec<_>>();
            let mut keys = BTreeSet::new();
            if matching.is_empty() {
                if matches!(&args[1], RuntimeExpr::Value(RuntimeValue::Unknown)) {
                    return Err(planner_error("a marked bracket settlement has opaque response continuation"));
                }
                // Inline needs the exact source response coordinate, not an
                // ordinary disposition. Membership was settled by the mark
                // before this source-route projection was consulted.
                let routes = host_response_routes(self)?;
                let root = self.semantic.child_origin(vis, 0)?;
                let (route, _) = selected_host_response_route(self, vis, root, &routes)?
                    .ok_or_else(|| planner_error(
                        "an unowned marked settlement has no exact inline response coordinate"))?;
                keys.insert(BracketResponseContextKey::Inline {
                    producer_call_origin: route.producer_call_origin,
                    response_origin: route.response_origin,
                    vis_origin: vis,
                    operation_root_origin: root,
                });
            } else {
                for unit in matching {
                    let identity = self.continuation_call_binding_for(
                        vis, unit.continuation_origin(), unit.producer_alternative(),
                        unit.recursive_position(),
                    )?.ok_or_else(|| planner_error(
                        "a marked bracket response has no exact continuation call identity"))?;
                    if !keys.insert(BracketResponseContextKey::Continuation(identity)) {
                        return Err(planner_error(
                            "dual ownership: two response rows share one marked settlement context"));
                    }
                }
            }
            if contexts.insert(vis, keys).is_some() {
                return Err(planner_error("dual ownership: a settlement S occurs twice"));
            }
        }
        Ok(contexts)
    }

    /// Derive and fully validate every statically attributable response demand.
    ///
    /// This phase intentionally does not ask whether an old causal caller had
    /// already caused the K context to be interned. The response edge is itself
    /// a lawful context demand, so that question belongs to the union interner
    /// below. The outer `Result` is plan integrity; the inner `Result` retains
    /// only the SSA trichotomy's genuinely dynamic or non-expressible arm.
    fn validated_response_context_demand(
        &self,
        vis_origin: StaticOriginId,
        operation_root_origin: StaticOriginId,
        selected_operation_origin: StaticOriginId,
        route: HostResponseRoute,
        unit: &ContinuationUnitView<'_>,
        id: StaticResponseContinuationId,
    ) -> Result<Result<StaticResponseContextDemand, SsaInfeasible>, CraneliftBackendError> {
        let base_owner = ContinuationEmissionOwner::Specialization(unit.id());
        let continuation_inputs = unit.prefinalization_continuation_inputs()?;
        let infeasible = |reason| {
            SsaInfeasible::at_vis(base_owner, vis_origin, Some(route.producer_call_origin), reason)
                .with_k(route.operation, unit.worker_closure_origin(), unit.worker_body_origin(),
                    unit.worker_capture_count(), continuation_inputs.len())
        };
        let k_identity = self.continuation_call_binding_for(
            vis_origin, unit.continuation_origin(), unit.producer_alternative(),
            unit.recursive_position(),
        )?.ok_or_else(|| planner_error(
            "a static response producer's own edge has no continuation call identity",
        ))?;
        let k_ret_identity = match exact_response_ret_identity(self, unit.continuation_origin())? {
            Ok(identity) => identity,
            Err(reason) => return Ok(Err(infeasible(reason))),
        };
        let envelope = unit.ordinary_envelope()?;
        let mut captures = Vec::new();
        for (position, member) in envelope.iter().enumerate() {
            let super::continuations::ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                ordinal, owner, source, ..
            } = member else { continue };
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
                ordinal: *ordinal, origin: *origin, source,
                producer_abi_slot: u32::try_from(position).map_err(|_| {
                    planner_capacity_error("static response capture slot exhausted")
                })?,
            });
        }
        if captures.iter().enumerate().any(|(ordinal, capture)|
            capture.ordinal as usize != ordinal)
        {
            return Err(planner_error("static response captures are not dense in capture-ordinal order"));
        }
        let raw_owner = self.semantic.function_owner(unit.worker_body_origin())?
            .ok_or_else(|| planner_error(
                "a response K worker body has no predeclared source owner",
            ))?;
        let effect_source_owner = self.semantic.function_owner(route.effect_origin)?
            .ok_or_else(|| planner_error(
                "a static response effect has no predeclared source owner",
            ))?;
        let operation_source_owner = self.semantic.function_owner(selected_operation_origin)?
            .ok_or_else(|| planner_error(
                "a selected response operation has no predeclared source owner",
            ))?;
        let effect_environment = match static_response_effect_environment(
            self, effect_source_owner, route.effect_origin, selected_operation_origin,
            &captures, &continuation_inputs,
        )? {
            Ok(environment) => environment,
            Err(reason) => return Ok(Err(infeasible(reason))),
        };
        Ok(Ok(StaticResponseContextDemand {
            id,
            base_owner,
            producer_call_origin: route.producer_call_origin,
            response_origin: route.response_origin,
            effect_origin: route.effect_origin,
            operation_root_origin,
            selected_operation_origin,
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
        }))
    }

    #[allow(clippy::type_complexity)]
    fn static_response_context_demands_filtered(
        &self,
        operation: Option<HostOpV1>,
        apply_mutation: bool,
    ) -> Result<
        Result<(Vec<StaticResponseContextDemand>, Vec<StaticResponseContextDemand>, Vec<DeferredResponseRow>), SsaInfeasible>,
        CraneliftBackendError,
    > {
        let marked = self.bracket_settlements.regions.iter().map(|region| region.settlement)
            .collect::<BTreeSet<_>>();
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
            // Decide marked membership before selecting any ordinary route.
            let is_marked = marked.contains(&vis_origin);
            if is_marked && operation.is_some() { continue; }
            let operation_origin = self.semantic.child_origin(vis_origin, 0)?;
            let Some((route, selected_operation_origin)) =
                selected_host_response_route(self, vis_origin, operation_origin, &routes)?
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
                is_marked,
            ));
        }
        response_vis.sort_by_key(
            |(vis_origin, operation_origin, selected_operation_origin, route, _k_is_opaque, _is_marked)| {
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
        // and the transport records exist. Phase A builds a context demand for
        // EVERY has-K-unit member and captures ONLY P1 (no continuation unit) as
        // a Deferred residual. Phase B assigns owners to exclusively-predeclared
        // transport groups only when at least two such groups compose; otherwise
        // transport sources remain P2. In a P1-bearing plane, P1 stays
        // main-lowered outside the demand population and a mixed-owner group
        // stays P2 rather than borrowing the ordinary groups' authority. The
        // record-derived set identifies those
        // callers and gives the suppression control a production-side
        // discriminator.
        // Congruence
        // over the full
        // response-Vis population (AC-1) and total-match reconciliation (R2/§7)
        // hold across the two phases: phase A's demand domain is has-K-unit, phase
        // B's disposition domain is the whole population (P1 sealed via its own
        // main-lowered Deferred arm).
        let mut marked_demands = Vec::new();
        let mut deferred: Vec<DeferredResponseRow> = Vec::new();
        for (vis_origin, operation_root_origin, selected_operation_origin, route, k_is_opaque, is_marked) in
            response_vis
        {
            let matching = units
                .iter()
                .filter(|unit| unit.producer_construct_origin() == vis_origin)
                .collect::<Vec<_>>();
            if matching.is_empty() {
                if is_marked { continue; } // Inline marked instances have no K context.
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
                    selected_operation_origin,
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
                let destination = if is_marked { &mut marked_demands } else { &mut demands };
                let id = StaticResponseContinuationId::from_position(destination.len())?;
                let demand = match self.validated_response_context_demand(
                    vis_origin, operation_root_origin, selected_operation_origin, route,
                    &unit, id,
                )? {
                    Ok(demand) => demand,
                    Err(reason) => return Ok(Err(reason)),
                };
                destination.push(demand);
            }
        }
        marked_demands.sort_by_key(|demand| {
            (demand.producer_call_origin, demand.vis_origin, demand.k_identity.clone())
        });
        for (position, demand) in marked_demands.iter_mut().enumerate() {
            demand.id = StaticResponseContinuationId::from_position(position)?;
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
        let expected_marked_demands = marked_demands.clone();
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
                        // The shared producer's two release K keys are marked
                        // S rows. AC-1 removed them from ordinary demands;
                        // corrupt the plane-bound demands themselves instead.
                        let pair = marked_demands
                            .iter()
                            .enumerate()
                            .find_map(|(left, first)| {
                                marked_demands.iter().enumerate().find_map(|(right, second)| {
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
                                    "the bracket two-K merge mutation found no shared producer",
                                )
                            })?;
                        marked_demands[pair.1].k_specialization =
                            marked_demands[pair.0].k_specialization;
                        marked_demands[pair.1].k_body_origin =
                            marked_demands[pair.0].k_body_origin;
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
                if mutation == StaticResponseContextDemandMutation::MergeTwoKKeys {
                    validate_static_response_demand_closure(
                        &expected_marked_demands, &marked_demands,
                    )?;
                }
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
        Ok(Ok((demands, marked_demands, deferred)))
    }

    fn validate_marked_bracket_demands(
        &self,
        marked: &[StaticResponseContextDemand],
    ) -> Result<(), CraneliftBackendError> {
        use super::brackets::BracketResponseContextKey;
        let expected = self.bracket_settlements.regions.iter().filter_map(|region| {
            match &region.context {
                BracketResponseContextKey::Continuation(identity) =>
                    Some((region.settlement, identity.clone())),
                BracketResponseContextKey::Inline { .. } => None,
            }
        }).collect::<Vec<_>>();
        let actual = marked.iter().map(|demand| (demand.vis_origin, demand.k_identity.clone()))
            .collect::<Vec<_>>();
        if expected.len() != actual.len()
            || expected.iter().collect::<BTreeSet<_>>() != actual.iter().collect::<BTreeSet<_>>()
            || expected.iter().collect::<BTreeSet<_>>().len() != expected.len()
        {
            return Err(planner_error("marked demand and plane instance disagree"));
        }
        Ok(())
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

    /// Resolve marked S against the already-interned K-context population.
    /// Its identity is a region-local label, never an ordinary successor id or
    /// a pending-Vis discriminant. Bind both directions: no two rows can claim
    /// one region, and no Continuation-keyed region can remain unbound.
    fn resolve_marked_bracket_rows(
        &self,
        mut marked: Vec<StaticResponseContextDemand>,
        contexts: &[PlannedContinuationContext],
        preexisting_count: usize,
    ) -> Result<BTreeMap<super::brackets::BracketSettlementId, StaticResponseContinuation>,
        CraneliftBackendError> {
        use super::brackets::BracketResponseContextKey;
        self.validate_marked_bracket_demands(&marked)?;
        let mut ids = Vec::with_capacity(marked.len());
        for demand in &mut marked {
            let matches = self.bracket_settlements.regions.iter().filter(|region| {
                region.settlement == demand.vis_origin && matches!(
                    &region.context, BracketResponseContextKey::Continuation(identity)
                        if identity == &demand.k_identity
                )
            }).collect::<Vec<_>>();
            let [region] = matches.as_slice() else {
                return Err(planner_error("marked demand and plane instance disagree"));
            };
            demand.id = StaticResponseContinuationId::from_position(
                region.id.ordinal() as usize)?;
            ids.push(region.id);
        }
        let rows = self.resolve_static_response_context_demands(
            marked, contexts, preexisting_count)?;
        let mut bound = BTreeMap::new();
        for (id, row) in ids.into_iter().zip(rows) {
            if bound.insert(id, row).is_some() {
                return Err(planner_error("two marked rows claim one bracket region"));
            }
        }
        if bound.len() != self.bracket_settlements.regions.iter()
            .filter(|region| matches!(region.context,
                BracketResponseContextKey::Continuation(_))).count() {
            return Err(planner_error("a Continuation bracket region has no marked row"));
        }
        Ok(bound)
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
        symbols: &crate::NativeProcessSymbols,
    ) -> Result<(), CraneliftBackendError> {
        if self.static_response_plan_installed
            || self.static_response_phase_a.is_some()
            || self.static_response_infeasible.is_some()
        {
            return Err(planner_error(
                "the static response context plan phase A may run exactly once",
            ));
        }
        let roles = super::brackets::BracketRoles::from(symbols);
        let static_regions = super::brackets::build_bracket_static_regions(self, &roles)?;
        let marked = self.marked_bracket_response_contexts(&static_regions)?;
        self.bracket_settlements = super::brackets::bind_bracket_contexts(
            roles, &static_regions, &marked,
        )?;
        let (demands, marked_demands, deferred) = match self.static_response_context_demands_filtered(None, true)? {
            Ok(classified) => classified,
            Err(infeasible) => {
                self.static_response_infeasible = Some(infeasible);
                return Ok(());
            }
        };
        self.validate_marked_bracket_demands(&marked_demands)?;
        let causal_contexts = self.continuation_contexts.clone();
        let (contexts, preexisting_count) = self.response_context_union(
            &causal_contexts, &[demands.as_slice(), marked_demands.as_slice()].concat(),
        )?;
        self.continuation_contexts = contexts;
        self.static_response_phase_a = Some(StaticResponsePhaseA {
            demands,
            marked: marked_demands,
            preexisting_count,
            deferred,
        });
        Ok(())
    }

    /// PHASE B of the two-phase response context install. Runs after the first
    /// aggregate/transport derivation identifies transport-source callers.
    /// Execute-then-resume assigns owners only to exclusively-predeclared
    /// transport groups when at least two such groups compose. A P1 response
    /// remains on main lowering and is not replaced; in that P1-bearing plane a
    /// mixed-owner group remains P2 rather than borrowing an ordinary group's
    /// call authority. P1-free composed planes keep their existing promotion.
    /// Single-stage groups retain the inc1 forward-Ret path. The test suppression
    /// restores P2 in an eligible plane. Phase A entries are never retracted.
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
        let has_unitless_response = !phase_a.deferred.is_empty();
        let (specialized, mut deferred) = self.static_response_phase_b_split_over(
            phase_a.demands,
            has_unitless_response,
            &self.pre_schema_transport_sources,
            &self.pre_schema_relay_promotions,
            true,
        )?;
        deferred.extend(phase_a.deferred);
        deferred.sort_by_key(|row| (row.vis_origin, row.operation_root_origin, row.operation));
        let contexts = self.continuation_contexts.clone();
        let rows = self.resolve_static_response_context_demands(
            specialized,
            &contexts,
            phase_a.preexisting_count,
        )?;
        let bracket_rows = self.resolve_marked_bracket_rows(
            phase_a.marked, &contexts, phase_a.preexisting_count)?;
        self.static_response_continuations = rows;
        self.bracket_rows = bracket_rows;
        self.static_response_deferred = deferred;
        self.static_response_plan_installed = true;
        Ok(())
    }

    /// Close relay promotion over would-be owners before any actual owner is
    /// installed. Each iteration recomputes the phase-B split on the final
    /// phase-A demands and contexts, then admits pairs against those candidate
    /// rows and the already-installed host-effect-seat authority. Neither the
    /// scratch plan nor its non-counting split writes into the live plan.
    pub(in crate::cranelift_backend) fn derive_pre_schema_relay_promotions(
        &self,
    ) -> Result<BTreeSet<RelayPromotionKey>, CraneliftBackendError> {
        if self.static_response_infeasible.is_some() {
            return Ok(BTreeSet::new());
        }
        let phase_a = self.static_response_phase_a.as_ref().ok_or_else(|| {
            planner_error("relay preselection ran before response phase A")
        })?;
        let calls = self.continuation_calls()?;
        let mut promoted = BTreeSet::new();
        loop {
            let mut candidate = self.clone();
            let (specialized, mut deferred) = self.static_response_phase_b_split_over(
                phase_a.demands.clone(),
                !phase_a.deferred.is_empty(),
                &self.pre_schema_transport_sources,
                &promoted,
                false,
            )?;
            deferred.extend(phase_a.deferred.iter().cloned());
            candidate.static_response_continuations = self.resolve_static_response_context_demands(
                specialized, &self.continuation_contexts, phase_a.preexisting_count,
            )?;
            candidate.static_response_deferred = deferred;
            candidate.static_response_plan_installed = true;
            let pairs = candidate.build_relay_drive_pairs()?;
            let mut next = promoted.clone();
            for (&(source, owner), pair) in &pairs {
                let minting = candidate.static_response_continuations.iter()
                    .find(|row| row.id() == pair.minting_row)
                    .ok_or_else(|| planner_error("relay pair has no hypothetical minting owner"))?;
                let protocol = candidate.returned_vis_protocol(minting.vis_origin())?;
                let already_performed =
                    (minting.effect_origin() == pair.effect_origin
                        && minting.operation() == pair.operation)
                        || protocol.contexts.iter().flat_map(|context| &context.members)
                            .filter_map(|member| member.successor.as_ref())
                            .any(|successor| successor.effect_origin() == pair.effect_origin
                                && successor.operation() == pair.operation);
                if already_performed {
                    continue;
                }
                for call in calls.iter().filter(|call| call.producer_construct_origin() == source) {
                    let identity = call.identity();
                    let Some(demand) = phase_a.demands.iter().find(|demand|
                        demand.vis_origin == source && demand.k_identity == identity
                    ) else { continue };
                    if !self.pre_schema_transport_sources.contains(&identity) {
                        continue;
                    }
                    let Some(deferred) = candidate.deferred_response_at_vis(source)? else {
                        continue;
                    };
                    if deferred.sub_case() != DeferredResponseSubCase::UnconsumedTransportCaller
                        || candidate.bounded_deferred_response_handler_owner(&deferred)?.is_some()
                        || candidate.admitted_deferred_handler_owner(&deferred)?.is_some()
                    {
                        continue;
                    }
                    let exclusively_predeclared = matches!(
                        identity.emission_owner(), ContinuationEmissionOwner::Predeclared(_)
                    ) && !phase_a.demands.iter().any(|other|
                        other.producer_call_origin == demand.producer_call_origin
                            && self.pre_schema_transport_sources.contains(&other.k_identity)
                            && !matches!(other.k_identity.emission_owner(),
                                ContinuationEmissionOwner::Predeclared(_))
                    );
                    if !exclusively_predeclared {
                        #[cfg(feature = "px8-ds-test-support")]
                        MIXED_RELAY_REFUSALS.with(|slot| {
                            if let Some(rows) = slot.borrow_mut().as_mut() {
                                rows.push(MixedRelayRefusalObservation {
                                    source: source.0,
                                    edge_identity: format!("{identity:?}"),
                                });
                            }
                        });
                        return Err(planner_error(format!(
                            "relay source {source:?} has a mixed-owner transport stage"
                        )));
                    }
                    #[cfg(feature = "px8-ds-test-support")]
                    if SUPPRESS_RELAY_PROMOTION.with(std::cell::Cell::get) {
                        SUPPRESS_RELAY_PROMOTION_APPLICATIONS.with(|slot| slot.set(slot.get() + 1));
                        continue;
                    }
                    if next.iter().any(|(previous_source, previous_owner, previous_identity)|
                        *previous_source == source && *previous_identity == identity
                            && *previous_owner != owner
                    ) {
                        return Err(planner_error(format!(
                            "relay source {source:?} and one incoming edge name two minting owners"
                        )));
                    }
                    next.insert((source, owner, identity));
                }
            }
            if next == promoted {
                return Ok(promoted);
            }
            if next.len() > phase_a.demands.len() {
                return Err(planner_error("relay promotion exceeded the finite response demand population"));
            }
            promoted = next;
        }
    }

    /// Preselect phase-B callers from phase-A demands and the ownership-free
    /// transport-source stratum. This does not install owners or count test
    /// instrumentation for the later installation path.
    pub(in crate::cranelift_backend) fn preselect_static_response_callers(
        &self,
        transport_sources: &BTreeSet<ContinuationCallIdentity>,
    ) -> Result<BTreeSet<ContinuationCallIdentity>, CraneliftBackendError> {
        if self.static_response_infeasible.is_some() {
            return Ok(BTreeSet::new());
        }
        let phase_a = self.static_response_phase_a.as_ref().ok_or_else(|| {
            planner_error("response caller preselection ran before phase A installed the demands")
        })?;
        let (specialized, _) = self.static_response_phase_b_split_over(
            phase_a.demands.clone(),
            !phase_a.deferred.is_empty(),
            transport_sources,
            &self.pre_schema_relay_promotions,
            false,
        )?;
        Ok(specialized.into_iter().map(|demand| demand.k_identity).collect())
    }

    fn static_response_phase_b_split_over(
        &self,
        demands: Vec<StaticResponseContextDemand>,
        has_unitless_response: bool,
        transport_sources: &BTreeSet<ContinuationCallIdentity>,
        relay_promotions: &BTreeSet<RelayPromotionKey>,
        count_install_applications: bool,
    ) -> Result<(Vec<StaticResponseContextDemand>, Vec<DeferredResponseRow>), CraneliftBackendError>
    {
        #[cfg(not(feature = "px8-ds-test-support"))]
        let _ = count_install_applications;
        // Execute-then-resume serves a composed response plane: at least two
        // producer groups have exclusively predeclared transport sources.
        // A producer that also has a specialization/fusion-owned source is a
        // mixed-owner fan-out and does not increase the composition count. When
        // P1 is present it also cannot borrow the ordinary groups' promotion
        // authority; P1-free composed planes preserve their existing promotion.
        // This keeps single-stage and P1-bearing mixed-owner residuals on their
        // proven paths while admitting the ordinary read-then-write stages.
        let mut transport_producer_owners = BTreeMap::new();
        for demand in &demands {
            if !transport_sources.contains(&demand.k_identity) {
                continue;
            }
            let owners = transport_producer_owners
                .entry(demand.producer_call_origin)
                .or_insert((false, false));
            match demand.k_identity.emission_owner() {
                ContinuationEmissionOwner::Predeclared(_) => owners.0 = true,
                ContinuationEmissionOwner::Specialization(_)
                | ContinuationEmissionOwner::Fusion(_) => owners.1 = true,
            }
        }
        let ordinary_stage_count = transport_producer_owners
            .values()
            .filter(|(predeclared, specialization)| *predeclared && !*specialization)
            .count();
        let pending_checked_ret_sink =
            super::aggregates::plane_has_pending_checked_control_ret_sink(self)?;
        #[cfg(feature = "px8-ds-test-support")]
        if count_install_applications {
            // The plane key's presence may be inert if no transport-source
            // demand reaches the decision below. The application recorder
            // inside the demand loop is the population acceptance oracle.
            if std::env::var_os("KEN_RT_RET_PLANE_CENSUS").is_some() {
                eprintln!(
                    "RT_RET_PLANE thread={:?} pending={} stages={} demands={}",
                    std::thread::current().name(),
                    pending_checked_ret_sink,
                    ordinary_stage_count,
                    demands.len(),
                );
            }
        }
        let requires_execute_then_resume = ordinary_stage_count >= 2 || pending_checked_ret_sink;
        let units = self.continuation_units()?;
        let mut specialized = Vec::new();
        let mut deferred = Vec::new();
        for demand in demands {
            // A construct returned as the emitting worker's body root has no
            // enclosing eliminator in that Function: neither the direct claim
            // nor the bypassed-candidate bridge can settle its selected call.
            // Predeclared and fusion owners do not name a continuation worker.
            let continuation_body_tail = match demand.k_identity.emission_owner() {
                ContinuationEmissionOwner::Specialization(owner) => {
                    let worker = units
                        .iter()
                        .find(|unit| unit.id() == owner)
                        .ok_or_else(|| {
                            planner_error(
                                "a response caller's emission specialization has no worker unit",
                            )
                        })?;
                    demand.k_identity.producer_construct_origin() == worker.worker_body_origin()
                }
                ContinuationEmissionOwner::Predeclared(_)
                | ContinuationEmissionOwner::Fusion(_) => false,
            };
            if continuation_body_tail {
                deferred.push(DeferredResponseRow {
                    vis_origin: demand.vis_origin,
                    producer_call_origin: demand.producer_call_origin,
                    operation_root_origin: demand.operation_root_origin,
                    selected_operation_origin: demand.selected_operation_origin,
                    effect_origin: demand.effect_origin,
                    operation: demand.operation,
                    sub_case: DeferredResponseSubCase::ContinuationBodyTail,
                    capture_count: demand.captures.len(),
                    continuation_input_count: demand.continuation_inputs.len(),
                });
                continue;
            }
            #[cfg(feature = "px8-ds-test-support")]
            let suppress_execute = SUPPRESS_EXECUTE_THEN_RESUME_RESPONSE.with(std::cell::Cell::get);
            #[cfg(not(feature = "px8-ds-test-support"))]
            let suppress_execute = false;
            let transport_source = transport_sources.contains(&demand.k_identity);
            let exclusively_predeclared_stage = transport_producer_owners
                .get(&demand.producer_call_origin)
                .is_some_and(|owners| *owners == (true, false));
            let relay_promote = relay_promotions.iter().any(|(source, _, edge)|
                *source == demand.vis_origin && *edge == demand.k_identity
            );
            if relay_promote && (!transport_source || !exclusively_predeclared_stage) {
                return Err(planner_error(format!(
                    "relay promotion at {:?} has no exclusively predeclared transport source",
                    demand.vis_origin,
                )));
            }
            // Execute-then-resume is admitted only for the composed ordinary
            // has-K groups identified above. Unit-less P1 members are absent
            // from `demands` and remain on main lowering, so this loop cannot
            // replace their path. In a P1-bearing plane, mixed-owner groups may
            // not borrow another group's authority; P1-free composed planes keep
            // their prior promotion. Single-stage groups retain their existing
            // route.
            if transport_source
                && !relay_promote
                && (suppress_execute
                    || !requires_execute_then_resume
                    || (has_unitless_response
                        && !exclusively_predeclared_stage))
            {
                // Population-side mutation restores P2 for an otherwise eligible
                // plane; open and single-stage planes remain lawful residuals.
                deferred.push(DeferredResponseRow {
                    vis_origin: demand.vis_origin,
                    producer_call_origin: demand.producer_call_origin,
                    operation_root_origin: demand.operation_root_origin,
                    selected_operation_origin: demand.selected_operation_origin,
                    effect_origin: demand.effect_origin,
                    operation: demand.operation,
                    sub_case: DeferredResponseSubCase::UnconsumedTransportCaller,
                    capture_count: demand.captures.len(),
                    continuation_input_count: demand.continuation_inputs.len(),
                });
            } else {
                // Execute-then-resume makes a transport-source caller a real
                // synchronous response-owner call. Its existing transport
                // morphism supplies the owner's Parameter/Capture frame; the
                // owner performs the host effect, calls the exact K context once,
                // and returns its existing Result word before the caller resumes.
                specialized.push(demand);
                #[cfg(feature = "px8-ds-test-support")]
                if count_install_applications
                    && transport_source
                    && pending_checked_ret_sink
                    && ordinary_stage_count < 2
                {
                    // This demand was actually placed in Specialized. For a
                    // transport source below two ordinary stages, reaching
                    // here also proves the suppress and mixed-owner deferrals
                    // did not fire; only the pending-checked-Ret disjunct
                    // admitted it.
                    let placed = specialized
                        .last()
                        .expect("just specialized a response demand");
                    let row = (
                        placed.vis_origin.0,
                        placed.producer_call_origin.0,
                        placed.operation,
                    );
                    PENDING_CHECKED_RET_SINK_APPLICATIONS.with(|slot| {
                        if let Some(rows) = slot.borrow_mut().as_mut() {
                            rows.push(row);
                        }
                    });
                    if std::env::var_os("KEN_RT_RET_PLANE_CENSUS").is_some() {
                        eprintln!(
                            "RT_RET_APP thread={:?} vis={} producer={} operation={:?}",
                            std::thread::current().name(),
                            row.0,
                            row.1,
                            row.2,
                        );
                    }
                }
            }
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
    /// pre-schema transport and relay identities the install read, so both
    /// halves are closed by same-state re-derivation and a phase-unstable or
    /// over-admitting input can no longer pass.
    pub(super) fn validate_static_response_context_plan(
        &self,
        causal_contexts: &[PlannedContinuationContext],
    ) -> Result<(), CraneliftBackendError> {
        if !self.static_response_plan_installed {
            return Err(planner_error(
                "the final plan carries no installed static response context plan",
            ));
        }
        #[cfg(feature = "px8-ds-test-support")]
        if BRACKET_ORDINARY_OWNER_MUTATION.with(|slot| slot.replace(false)) {
            let result = (|| {
                let mut first_refusal = None;
                for row in self.bracket_rows.values() {
                    let mut mutant = self.clone();
                    mutant.static_response_continuations.push(row.clone());
                    let Err(error) = mutant.validate_static_response_context_plan(causal_contexts)
                    else {
                        return Err(planner_error(
                            "a bracket row reinserted as an ordinary owner passed plan close",
                        ));
                    };
                    if !format!("{error:?}").contains(
                        "the installed response context/continuation plane is not its exact closed derivation"
                    ) {
                        return Err(planner_error(
                            "bracket ordinary-owner mutation reached a different refusal",
                        ));
                    }
                    BRACKET_ORDINARY_OWNER_MUTATION_APPLICATIONS
                        .with(|count| count.set(count.get() + 1));
                    first_refusal.get_or_insert(error);
                }
                first_refusal.ok_or_else(|| planner_error(
                    "bracket ordinary-owner mutation found no installed marked row",
                ))
            })();
            BRACKET_ORDINARY_OWNER_MUTATION.with(|slot| slot.set(true));
            return Err(result?);
        }
        let (demands, marked_demands, p1_deferred) =
            match self.static_response_context_demands_filtered(None, false)? {
                Ok(classified) => classified,
                Err(infeasible) => {
                    let mut landed_contexts = self.continuation_contexts.clone();
                    for context in &mut landed_contexts {
                        context.finalized_availability.clear();
                    }
                    if self.static_response_infeasible.as_ref() != Some(&infeasible)
                        || !self.static_response_continuations.is_empty()
                        || !self.bracket_rows.is_empty()
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
        self.validate_marked_bracket_demands(&marked_demands)?;
        // Phase A re-derivation: the owner-less context-entry plane over has-K-unit.
        let (mut expected_contexts, preexisting_count) = self.response_context_union(
            causal_contexts, &[demands.as_slice(), marked_demands.as_slice()].concat(),
        )?;
        // Phase B re-derivation uses the same pre-schema transport and relay
        // identities that preselection and the real phase-B install consumed.
        // Phase B re-derivation: a P1-bearing eligible plane gives each
        // exclusively-predeclared group an owner while mixed-owner groups retain
        // P2; P1-free composed planes preserve their existing promotion. A
        // single-stage plane or the suppression control retains P2. P1 stays on
        // main lowering outside the demand population.
        let (specialized, mut expected_deferred) = self.static_response_phase_b_split_over(
            demands, !p1_deferred.is_empty(), &self.pre_schema_transport_sources,
            &self.pre_schema_relay_promotions, false,
        )?;
        expected_deferred.extend(p1_deferred);
        expected_deferred
            .sort_by_key(|row| (row.vis_origin, row.operation_root_origin, row.operation));
        let expected_rows = self.resolve_static_response_context_demands(
            specialized,
            &expected_contexts,
            preexisting_count,
        )?;
        let expected_bracket_rows = self.resolve_marked_bracket_rows(
            marked_demands, &expected_contexts, preexisting_count)?;
        for context in &mut expected_contexts {
            context.finalized_availability.clear();
        }
        let mut landed_contexts = self.continuation_contexts.clone();
        for context in &mut landed_contexts {
            context.finalized_availability.clear();
        }
        if landed_contexts != expected_contexts
            || self.static_response_continuations != expected_rows
            || self.bracket_rows != expected_bracket_rows
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

    /// Only the marked plane can mint `BracketOwned`; no Phase-B classifier
    /// predicate can choose it. The response key is mandatory: an origin-only
    /// query would collapse distinct instances at one shared S.
    pub(in crate::cranelift_backend) fn bracket_response_disposition_at_vis(
        &self,
        vis_origin: StaticOriginId,
        context: &super::brackets::BracketResponseContextKey,
    ) -> Option<ResponseDisposition> {
        self.bracket_settlements.regions.iter()
            .find(|region| region.settlement == vis_origin && &region.context == context)
            .map(|region| ResponseDisposition::BracketOwned(region.id))
    }

    /// The bracket settlement whose marked response context is exactly this
    /// continuation identity (`marked_bracket_response_contexts`). Keyed on the
    /// full identity, never on S alone, a selected caller, or emission
    /// position. Two owners refuse.
    pub(in crate::cranelift_backend) fn bracket_settlement_owning_continuation(
        &self,
        identity: &ContinuationCallIdentity,
    ) -> Result<Option<super::brackets::BracketSettlementId>, CraneliftBackendError> {
        let mut owners = self.bracket_settlements.regions.iter().filter(|region| {
            matches!(
                &region.context,
                super::brackets::BracketResponseContextKey::Continuation(owned)
                    if owned == identity
            )
        });
        let owner = owners.next().map(|region| region.id);
        if owners.next().is_some() {
            return Err(planner_error(
                "dual ownership: two bracket regions own one continuation identity",
            ));
        }
        Ok(owner)
    }

    /// Derive the lexical K's single-shot and tail-exit facts from its source
    /// occurrence. Nothing here consults `RecursiveBackedge` or runtime tags.
    pub(super) fn deferred_response_continuation_shape(
        &self,
        row: &DeferredResponseRow,
    ) -> Result<Option<DeferredResponseContinuationShape>, CraneliftBackendError> {
        let RuntimeExpr::Construct { args, .. } = self.planned_occurrence_expr(row.vis_origin)?
        else {
            return Ok(None);
        };
        let [_, RuntimeExpr::LexicalClosure { params, body, .. }] = args.as_slice() else {
            return Ok(None);
        };
        if params.len() != 1 {
            return Ok(None);
        }
        let k_body_origin = self
            .deferred_response_k_body(row)?
            .ok_or_else(|| planner_error("a Deferred response lost its lexical K body"))?;
        if self.semantic.function_owner(row.vis_origin)?.is_none() {
            return Err(planner_error("a Deferred response Vis has no source owner"));
        }
        let mut shape = DeferredResponseContinuationShape {
            k_body_origin,
            response_uses: count_deferred_response_parameter_uses(body, 0)?,
            ret_exits: 0,
            static_tail_calls: 0,
            other_exits: 0,
        };
        classify_deferred_response_tail(self, k_body_origin, &mut shape)?;
        Ok(Some(shape))
    }

    /// Return the possible first response `Vis` nodes evaluated by `origin`.
    ///
    /// Sequential operands stop at their first nonempty response frontier;
    /// mutually exclusive branches contribute a union. A computational match
    /// therefore examines its scrutinee before its cases: a resource body's
    /// pending access is dispatched before the Ret case can release the
    /// resource. Closure bodies are dormant. A call or checked recursive
    /// invocation beyond the visible source tree is not statically bounded and
    /// returns `None`, which leaves the response unowned and fail-closed.
    fn static_response_frontier_sequence(
        &self,
        origins: &[StaticOriginId],
        active: &mut BTreeSet<StaticOriginId>,
    ) -> Result<Option<Vec<StaticOriginId>>, CraneliftBackendError> {
        for origin in origins {
            let Some(frontier) = self.static_response_frontier(*origin, active)? else {
                return Ok(None);
            };
            if !frontier.is_empty() {
                return Ok(Some(frontier));
            }
        }
        Ok(Some(Vec::new()))
    }

    /// A bracket ledger settles this marked S under its checked region row.
    /// The S is an existing owner boundary, not a Deferred suffix member and
    /// not an opaque Vis that invalidates the enclosing owner's suffix.
    fn is_marked_bracket_settlement(&self, vis: StaticOriginId) -> bool {
        self.bracket_settlements
            .regions
            .iter()
            .any(|region| region.settlement == vis)
    }

    fn static_response_frontier(
        &self,
        origin: StaticOriginId,
        active: &mut BTreeSet<StaticOriginId>,
    ) -> Result<Option<Vec<StaticOriginId>>, CraneliftBackendError> {
        if !active.insert(origin) {
            return Ok(None);
        }
        let children = self.semantic.child_origins(origin)?.to_vec();
        let result = match self.planned_occurrence_expr(origin)? {
            RuntimeExpr::CheckedBracketFinalizer { .. }
            | RuntimeExpr::CheckedJoinSite { .. }
            | RuntimeExpr::CheckedSubcontinuationFrame { .. }
            | RuntimeExpr::CheckedComputationalIHSlots { .. } => {
                self.static_response_frontier_sequence(&children, active)
            }
            RuntimeExpr::CheckedRecursiveInvocation { .. }
            | RuntimeExpr::CheckedComputationalIHInvocation { .. } => Ok(None),
            RuntimeExpr::Let { .. } => self.static_response_frontier_sequence(&children, active),
            RuntimeExpr::If { .. } => {
                let Some((scrutinee_origin, branches)) = children.split_first() else {
                    return Err(planner_error(
                        "a response-frontier If has no scrutinee child",
                    ));
                };
                let Some(scrutinee) = self.static_response_frontier(*scrutinee_origin, active)?
                else {
                    active.remove(&origin);
                    return Ok(None);
                };
                if !scrutinee.is_empty() {
                    Ok(Some(scrutinee))
                } else {
                    let mut frontier = Vec::new();
                    for branch in branches {
                        let Some(mut branch) = self.static_response_frontier(*branch, active)?
                        else {
                            active.remove(&origin);
                            return Ok(None);
                        };
                        frontier.append(&mut branch);
                    }
                    frontier.sort();
                    frontier.dedup();
                    Ok(Some(frontier))
                }
            }
            RuntimeExpr::Match { .. } | RuntimeExpr::ComputationalMatch { .. } => {
                let Some((scrutinee_origin, branches)) = children.split_first() else {
                    return Err(planner_error(
                        "a response-frontier Match has no scrutinee child",
                    ));
                };
                let Some(scrutinee) = self.static_response_frontier(*scrutinee_origin, active)?
                else {
                    active.remove(&origin);
                    return Ok(None);
                };
                if !scrutinee.is_empty() {
                    Ok(Some(scrutinee))
                } else {
                    let mut frontier = Vec::new();
                    for branch in branches {
                        let Some(mut branch) = self.static_response_frontier(*branch, active)?
                        else {
                            active.remove(&origin);
                            return Ok(None);
                        };
                        frontier.append(&mut branch);
                    }
                    frontier.sort();
                    frontier.dedup();
                    Ok(Some(frontier))
                }
            }
            RuntimeExpr::Construct { constructor, args }
                if constructor.as_str().ends_with("::ITree::Vis") && args.len() == 2 =>
            {
                let known = self
                    .static_response_continuations
                    .iter()
                    .any(|row| row.vis_origin() == origin)
                    || self
                        .static_response_deferred
                        .iter()
                        .any(|row| row.vis_origin() == origin)
                    || self.is_marked_bracket_settlement(origin);
                if known {
                    Ok(Some(vec![origin]))
                } else {
                    Ok(None)
                }
            }
            RuntimeExpr::Construct { constructor, args }
                if constructor.as_str().ends_with("::ITree::Ret") && args.len() == 1 =>
            {
                Ok(Some(Vec::new()))
            }
            RuntimeExpr::PrimitiveCall { .. }
            | RuntimeExpr::Construct { .. }
            | RuntimeExpr::Record { .. }
            | RuntimeExpr::Project { .. } => {
                self.static_response_frontier_sequence(&children, active)
            }
            RuntimeExpr::LexicalClosure { .. } => {
                let Some((_body, captures)) = children.split_first() else {
                    return Err(planner_error(
                        "a response-frontier lexical closure has no body child",
                    ));
                };
                self.static_response_frontier_sequence(captures, active)
            }
            RuntimeExpr::Closure { .. }
            | RuntimeExpr::Value(_)
            | RuntimeExpr::Var(_)
            | RuntimeExpr::DeclarationRef { .. }
            | RuntimeExpr::ImportedDeclarationRef { .. }
            | RuntimeExpr::Trap(_) => Ok(Some(Vec::new())),
            RuntimeExpr::Call { .. } | RuntimeExpr::Effect { .. } => Ok(None),
        };
        active.remove(&origin);
        result
    }

    /// The statically bounded P2 suffix reached from one specialized owner.
    ///
    /// Every Deferred P2 node on the frontier calls its exact lexical K once
    /// with the current HostResult, then contributes that K's frontier. How many
    /// times the typed K body reads its ordinary argument is source semantics,
    /// not owner multiplicity. P1 and already-
    /// Specialized nodes are existing owner boundaries. The installed finite
    /// response population is the bound; a cycle or opaque frontier leaves the
    /// whole new suffix unowned rather than weakening exact-Ret. S7 admits only
    /// the two evidenced new classes: one repeated producer call, or the frozen
    /// Mapping read/write access family. A terminal `ResourceRelease` belongs
    /// to the qualified bracket suffix but does not decide its operation class.
    /// A heterogeneous sibling outside those classes retains its existing owner
    /// and forward-edge route byte-for-behavior.
    fn bounded_deferred_response_suffix(
        &self,
        response: &StaticResponseContinuation,
    ) -> Result<Vec<DeferredResponseRow>, CraneliftBackendError> {
        let frontier =
            self.static_response_frontier(response.k_body_origin(), &mut BTreeSet::new())?;
        let Some(mut pending) = frontier else {
            return Ok(Vec::new());
        };
        let mut suffix = Vec::new();
        let mut visited = BTreeSet::new();
        while let Some(vis) = pending.pop() {
            if self
                .static_response_continuations
                .iter()
                .any(|candidate| candidate.vis_origin() == vis)
                || self.is_marked_bracket_settlement(vis)
            {
                continue;
            }
            let Some(row) = self.deferred_response_at_vis(vis)? else {
                return Ok(Vec::new());
            };
            match row.sub_case {
                DeferredResponseSubCase::NoContinuationUnit
                | DeferredResponseSubCase::ContinuationBodyTail => continue,
                DeferredResponseSubCase::UnconsumedTransportCaller => {}
            }
            if !visited.insert(row.vis_origin()) {
                return Ok(Vec::new());
            }
            let Some(shape) = self.deferred_response_continuation_shape(&row)? else {
                return Ok(Vec::new());
            };
            let next_frontier =
                self.static_response_frontier(shape.k_body_origin, &mut BTreeSet::new())?;
            let Some(mut next) = next_frontier else {
                return Ok(Vec::new());
            };
            suffix.push(row);
            pending.append(&mut next);
            if suffix.len() > self.static_response_deferred.len() {
                return Err(planner_error(
                    "a static response owner's bounded Deferred suffix exceeded its finite population",
                ));
            }
        }
        let substantive = suffix
            .iter()
            .filter(|candidate| candidate.operation() != HostOpV1::ResourceRelease)
            .collect::<Vec<_>>();
        let repeated_producer = !substantive.is_empty()
            && substantive.iter().all(|candidate| {
                candidate.producer_call_origin() == response.producer_call_origin()
            });
        let mapping_access = |operation| {
            matches!(
                operation,
                HostOpV1::MappingReadView | HostOpV1::MappingWriteView
            )
        };
        let mapping_access_chain = mapping_access(response.operation())
            && !substantive.is_empty()
            && substantive
                .iter()
                .all(|candidate| mapping_access(candidate.operation()));
        if repeated_producer || mapping_access_chain {
            Ok(suffix)
        } else {
            // S7: a heterogeneous sequence outside the two newly evidenced
            // classes keeps its existing owner/forward-edge route unchanged.
            Ok(Vec::new())
        }
    }

    /// The unique specialized owner of a statically bounded P2 suffix.
    pub(in crate::cranelift_backend) fn bounded_deferred_response_handler_owner(
        &self,
        row: &DeferredResponseRow,
    ) -> Result<Option<ContinuationEmissionOwner>, CraneliftBackendError> {
        match row.sub_case {
            DeferredResponseSubCase::UnconsumedTransportCaller => {}
            DeferredResponseSubCase::NoContinuationUnit
            | DeferredResponseSubCase::ContinuationBodyTail => return Ok(None),
        }
        let mut owners = Vec::new();
        for response in &self.static_response_continuations {
            let suffix = self.bounded_deferred_response_suffix(response)?;
            if suffix
                .iter()
                .any(|candidate| candidate.vis_origin() == row.vis_origin())
                && !owners.contains(&response.base_owner())
            {
                owners.push(response.base_owner());
            }
        }
        match owners.as_slice() {
            [] => Ok(None),
            [owner] => Ok(Some(*owner)),
            _ => Err(planner_error(
                "one bounded Deferred response suffix has more than one static handler owner",
            )),
        }
    }

    /// The same admitted Deferred handler authority used by lowering and
    /// relay settlement. A selected pending response fixes its owner once;
    /// otherwise the existing response-plan lookup determines the owner.
    pub(in crate::cranelift_backend) fn admitted_deferred_handler_owner(
        &self,
        row: &DeferredResponseRow,
    ) -> Result<Option<ContinuationEmissionOwner>, CraneliftBackendError> {
        if let Some(witness) = self.selected_pending_response_at_vis(row.vis_origin())? {
            if witness.disposition() != Some(ResponseDisposition::Deferred) {
                return Err(planner_error(
                    "an admitted pending response disagrees with its Deferred disposition",
                ));
            }
            return Ok(witness.owner());
        }
        self.deferred_response_handler_owner(row)
    }

    /// The specialized handler that consumes one Deferred response.
    ///
    /// The first arm is the statically bounded P2 interpreter suffix above. The
    /// fallback is the established single-owner decomposition for a Deferred
    /// response whose K uses the current HostResult exactly once on its most
    /// demanding path and whose exits are only `ITree::Ret`, traps, or uniquely
    /// resolved static tail calls. Several enclosing response bodies may contain
    /// the same lexical subtree; the nearest one is the unique candidate whose K
    /// body is contained by every other candidate K body. That fallback handler
    /// must also contain a unit-less P1. Selection is by structural ownership,
    /// never by specialization ordinal, origin proximity, or runtime state.
    pub(in crate::cranelift_backend) fn deferred_response_handler_owner(
        &self,
        row: &DeferredResponseRow,
    ) -> Result<Option<ContinuationEmissionOwner>, CraneliftBackendError> {
        // A continuation-body-root Vis has no selected incoming owner call to
        // borrow from an enclosing handler: keep its ordinary Construct route.
        if row.sub_case == DeferredResponseSubCase::ContinuationBodyTail {
            return Ok(None);
        }
        if let Some(owner) = self.bounded_deferred_response_handler_owner(row)? {
            return Ok(Some(owner));
        }
        let Some(shape) = self.deferred_response_continuation_shape(row)? else {
            return Ok(None);
        };
        // A static tail call completes like a Ret exit when its callee answers
        // directly: its effects run in place and its result reaches the drive
        // as the CHECKED answer, never as an ITree word.
        let completing_exits = shape.ret_exits.saturating_add(shape.static_tail_calls);
        if shape.response_uses != 1 || completing_exits == 0 || shape.other_exits != 0 {
            return Ok(None);
        }
        if shape.static_tail_calls != 0
            && !deferred_response_tail_callees_answer_directly(self, shape.k_body_origin)?
        {
            return Ok(None);
        }
        let mut candidates = Vec::new();
        for response in &self.static_response_continuations {
            if occurrence_subtree_contains(self, response.k_body_origin(), row.vis_origin)? {
                candidates.push(response);
            }
        }
        let mut nearest = Vec::new();
        for candidate in &candidates {
            let mut inside_every_other = true;
            for other in &candidates {
                if other.k_body_origin() != candidate.k_body_origin()
                    && !occurrence_subtree_contains(
                        self,
                        other.k_body_origin(),
                        candidate.k_body_origin(),
                    )?
                {
                    inside_every_other = false;
                    break;
                }
            }
            if inside_every_other {
                nearest.push(*candidate);
            }
        }
        match nearest.as_slice() {
            [] => Ok(None),
            [handler] => {
                let mut contains_unitless = false;
                for candidate in &self.static_response_deferred {
                    if candidate.sub_case == DeferredResponseSubCase::NoContinuationUnit
                        && occurrence_subtree_contains(
                            self,
                            handler.k_body_origin(),
                            candidate.vis_origin,
                        )?
                    {
                        contains_unitless = true;
                        break;
                    }
                }
                Ok(contains_unitless.then(|| handler.base_owner()))
            }
            _ => Err(planner_error(
                "one Deferred response has more than one nearest static handler owner",
            )),
        }
    }

    /// Claim one exact member of the frame being defined by a handler-owned
    /// response. Ambiguous membership refuses; absent membership is `None`,
    /// since that frame cannot supply the coordinate. Neither falls back to a
    /// root ABI position: the consumer will independently re-check this frame.
    pub(in crate::cranelift_backend) fn entry_frame_claim_in(
        &self,
        handler: ContinuationEmissionOwner,
        coordinate: ContinuationSourceCoordinate,
    ) -> Result<Option<ContinuationEnvironmentClaim>, CraneliftBackendError> {
        let mut found = None;
        let mut offer = |member: ContinuationSourceCoordinate, slot: u32| {
            if member == coordinate {
                if found.replace(slot).is_some() {
                    return Err(planner_error(
                        "a handler drive frame declares two members for one continuation coordinate",
                    ));
                }
            }
            Ok(())
        };
        let frame = match handler {
            ContinuationEmissionOwner::Predeclared(owner) => {
                for (position, member) in continuation_owner_entry_sources(self, owner)?
                    .into_iter()
                    .enumerate()
                {
                    let position = u32::try_from(position).map_err(|_| {
                        planner_capacity_error("handler drive entry slot exhausted")
                    })?;
                    offer(member.coordinate, position)?;
                }
                ContinuationFrameIdentity::Predeclared(owner)
            }
            ContinuationEmissionOwner::Specialization(specialization) => {
                let units = self.continuation_units()?;
                let unit = units.get(specialization.0 as usize).ok_or_else(|| {
                    planner_error("a handler drive names a specialization that was never interned")
                })?;
                let body = unit.worker_body_origin();
                let context = self.continuation_context_for(specialization, body)?
                    .ok_or_else(|| planner_error(
                        "a handler drive's specialization has no defining generated context",
                    ))?;
                for (position, member) in self
                    .continuation_context_parameter_members(context.id())?
                    .into_iter()
                    .enumerate()
                {
                    let position = u32::try_from(position).map_err(|_| {
                        planner_capacity_error("handler drive context Parameter slot exhausted")
                    })?;
                    offer(member, position)?;
                }
                for (position, capture) in context.captures()?.iter().enumerate() {
                    let slot = u32::try_from(position).ok()
                        .and_then(|position| context.parameters().checked_add(position))
                        .ok_or_else(|| planner_capacity_error(
                            "handler drive context Capture slot exhausted",
                        ))?;
                    offer(capture.coordinate, slot)?;
                }
                ContinuationFrameIdentity::GeneratedContext {
                    context: context.id(),
                    specialization,
                    worker_body_origin: body,
                }
            }
            ContinuationEmissionOwner::Fusion(_) => return Err(planner_error(
                "a handler drive cannot claim an ordinary frame for an installed fusion",
            )),
        };
        Ok(found.map(|declared_slot| {
            ContinuationEnvironmentClaim::EntryFrame { frame, declared_slot }
        }))
    }

    /// The planned context transfer for one handler-owned Deferred response.
    pub(in crate::cranelift_backend) fn handler_drive_context_route(
        &self,
        row: &DeferredResponseRow,
        handler: ContinuationEmissionOwner,
    ) -> Result<Option<HandlerDriveContextRoute>, CraneliftBackendError> {
        if self.admitted_deferred_handler_owner(row)? != Some(handler) {
            return Ok(None);
        }
        // A unit-less P1 has no context to transfer into; its handler lowers K.
        if row.sub_case == DeferredResponseSubCase::NoContinuationUnit {
            return Ok(None);
        }
        let Some(body) = self.deferred_response_k_body(row)? else {
            return Ok(None);
        };
        let units = self.continuation_units()?;
        let selecting = units.iter()
            .filter(|unit| unit.worker_body_origin() == body
                && unit.producer_construct_origin() == row.vis_origin())
            .collect::<Vec<_>>();
        let [unit] = selecting.as_slice() else {
            return Err(planner_error(
                "a handler-owned drive's K body has no unique selecting specialization at its Vis",
            ));
        };
        let context = self
            .carried_invocation_context(unit.continuation_origin(), unit.recursive_position(), body)?
            .ok_or_else(|| planner_error(
                "a handler-owned drive's selecting specialization binds no carried context",
            ))?;
        let view = self.continuation_contexts()?.into_iter()
            .find(|candidate| candidate.id() == context)
            .ok_or_else(|| planner_error(
                "a handler-owned drive's context has no projected view",
            ))?;
        let k_owner = occurrence_authority(self, body)?.owner;
        if view.raw_owner() != k_owner {
            return Err(planner_error(
                "a handler-owned drive's context raw owner is not the K body's checked owner",
            ));
        }
        // The real K closure supplies the full Parameter run. Certify that
        // this context declares exactly its checked raw owner's entry members.
        self.continuation_context_parameter_members(context)?;
        let mut capture_claims = Vec::new();
        for capture in view.captures()? {
            // K closes over a coordinate the handler's frame does not hold, so
            // no transfer leaves this handler. No route is planned; a drive
            // that still reaches `drive_response_at_source` refuses there.
            let Some(claim) = self.entry_frame_claim_in(handler, capture.coordinate)? else {
                return Ok(None);
            };
            capture_claims.push(claim);
        }
        Ok(Some(HandlerDriveContextRoute {
            vis_origin: row.vis_origin(), handler, context,
            continuation_origin: unit.continuation_origin(),
            recursive_position: unit.recursive_position(),
            worker_body_origin: body, capture_claims,
        }))
    }

    /// The prefix ending at the nearest enclosing continuation frame.
    ///
    /// `frame_origins` is ordered inner-to-outer by the existing eliminator
    /// stack. Every later frame that also contains the response must contain the
    /// first such frame, so a crossing or reordered stack refuses rather than
    /// turning a position into ownership authority.
    pub(in crate::cranelift_backend) fn deferred_response_handler_prefix_len(
        &self,
        row: &DeferredResponseRow,
        frame_origins: &[StaticOriginId],
    ) -> Result<Option<usize>, CraneliftBackendError> {
        if self.deferred_response_handler_owner(row)?.is_none() {
            return Ok(None);
        }
        let mut containing = Vec::new();
        for (position, frame) in frame_origins.iter().copied().enumerate() {
            if occurrence_subtree_contains(self, frame, row.vis_origin)? {
                containing.push((position, frame));
            }
        }
        let Some((position, nearest)) = containing.first().copied() else {
            return Ok(None);
        };
        for (_, outer) in containing.iter().copied().skip(1) {
            if !occurrence_subtree_contains(self, outer, nearest)? {
                return Err(planner_error(
                    "Deferred response continuation frames are not nested inner-to-outer",
                ));
            }
        }
        Ok(Some(position + 1))
    }

    /// The exact lexical continuation body of one Deferred response Vis.
    pub(in crate::cranelift_backend) fn deferred_response_k_body(
        &self,
        row: &DeferredResponseRow,
    ) -> Result<Option<StaticOriginId>, CraneliftBackendError> {
        let RuntimeExpr::Construct { args, .. } = self.planned_occurrence_expr(row.vis_origin)?
        else {
            return Ok(None);
        };
        let [_, RuntimeExpr::LexicalClosure { params, .. }] = args.as_slice() else {
            return Ok(None);
        };
        if params.len() != 1 {
            return Ok(None);
        }
        let k_origin = self.semantic.child_origin(row.vis_origin, 1)?;
        Ok(Some(self.semantic.child_origin(k_origin, 0)?))
    }

    /// The one Deferred response row at an exact source `ITree::Vis`.
    pub(in crate::cranelift_backend) fn deferred_response_at_vis(
        &self,
        vis_origin: StaticOriginId,
    ) -> Result<Option<DeferredResponseRow>, CraneliftBackendError> {
        let mut matching = self
            .static_response_deferred
            .iter()
            .filter(|row| row.vis_origin == vis_origin);
        let first = matching.next().cloned();
        if matching.next().is_some() {
            return Err(planner_error(
                "one source Vis has more than one Deferred response row",
            ));
        }
        Ok(first)
    }

    /// The one unit-less Deferred response Vis owned by a retained call body.
    /// Absence is ordinary; multiplicity refuses because one call result cannot
    /// select two distinct in-frame response executions.
    pub(in crate::cranelift_backend) fn deferred_no_unit_response_in_body(
        &self,
        body_origin: StaticOriginId,
    ) -> Result<Option<DeferredResponseRow>, CraneliftBackendError> {
        let mut matching = Vec::new();
        for row in &self.static_response_deferred {
            if row.sub_case == DeferredResponseSubCase::NoContinuationUnit
                && occurrence_subtree_contains(self, body_origin, row.vis_origin)?
            {
                matching.push(row.clone());
            }
        }
        match matching.as_slice() {
            [] => Ok(None),
            [row] => Ok(Some(row.clone())),
            _ => Err(planner_error(
                "one retained call body owns more than one unit-less Deferred response Vis",
            )),
        }
    }

    /// Whether a `StaticResponseDeferred` placeholder is sound at `site` in the
    /// function `scope` defines. A placeholder resumes without dispatch, on the
    /// premise that this function's caller was retargeted to a response owner.
    /// A source Effect is shared by every Vis the handler arm serves, so row
    /// existence at that site alone says nothing about this function's caller.
    ///
    /// Licensed iff this is a continuation specialization targeted by an owner
    /// of a row at `site`, and every ordinary call targeting the specialization
    /// is retargeted by some owner. Every other scope fails closed to ordinary
    /// lowering, which dispatches or refuses rather than silently resuming.
    pub(in crate::cranelift_backend) fn static_response_placeholder_licensed(
        &self,
        site: StaticResponseSite,
        scope: GraftedSpineFunctionScope,
    ) -> Result<bool, CraneliftBackendError> {
        let GraftedSpineFunctionScope::Continuation(target) = scope else {
            return Ok(false);
        };
        let owners = self.static_response_owner_specializations()?.map_err(|infeasible| {
            planner_error(format!(
                "compile-time response specialization is infeasible at {:?}: {}",
                infeasible.vis_origin(), infeasible.reason(),
            ))
        })?;
        let row_targets_scope = self.static_response_continuations.iter()
            .filter(|row| site.matches(row.effect_origin, row.operation_root_origin))
            .any(|row| owners.iter().any(|owner| {
                owner.response() == row.id && owner.selected_caller().target() == target
            }));
        if !row_targets_scope {
            return Ok(false);
        }
        Ok(self.ordinary_continuation_call_identities()?.iter()
            .filter(|identity| identity.target() == target)
            .all(|identity| owners.iter().any(|owner| owner.selected_caller() == identity)))
    }

    /// Check the SOLE placeholder-to-word mint against the exact licensed
    /// production site, the target's declared owner specialization, and the
    /// target slot's own ABI ordinal. The source site is retained on the
    /// compiler-only deferred operand; a bare word could not carry it here.
    pub(in crate::cranelift_backend) fn static_response_placeholder_mint_licensed(
        &self,
        site: StaticResponseSite,
        scope: GraftedSpineFunctionScope,
        target_origin: StaticOriginId,
        slot: &AbiSlot,
    ) -> Result<bool, CraneliftBackendError> {
        if slot.kind != AbiSlotKind::Parameter || slot.ordinal != 0
            || !self.static_response_placeholder_licensed(site, scope)? {
            return Ok(false);
        }
        let GraftedSpineFunctionScope::Continuation(scope_target) = scope else {
            return Ok(false);
        };
        let owners = self.static_response_owner_specializations()?
            .map_err(|failure| planner_error(format!(
                "placeholder owner is infeasible at {:?}: {}",
                failure.vis_origin(), failure.reason(),
            )))?;
        let units = self.continuation_units()?;
        let emittable = self.emittable_units()?;
        let mut matches = 0;
        for owner in &owners {
            let selected = owner.selected_caller().target();
            if selected != scope_target { continue; }
            let Some(row) = self.static_response_continuations.iter()
                .find(|row| row.id() == owner.response()) else {
                return Err(planner_error("placeholder owner has no installed response row"));
            };
            if !site.matches(row.effect_origin(), row.operation_root_origin) { continue; }
            let unit = units.iter().find(|unit| unit.id() == selected)
                .ok_or_else(|| planner_error("placeholder owner has no K unit"))?;
            if emittable.iter().any(|candidate| {
                candidate.entry_origin() == target_origin
                    && candidate.body_occurrence() == unit.worker_body_origin()
            }) {
                matches += 1;
            }
        }
        Ok(matches == 1)
    }

    /// A response-bearing `Vis` whose operation has been assigned a response
    /// owner. Used when a non-owner function tries to transfer its raw lexical
    /// continuation: that is an owner-boundary violation, not a generic closure.
    pub(in crate::cranelift_backend) fn specialized_response_at_vis(
        &self,
        vis_origin: StaticOriginId,
    ) -> bool {
        self.static_response_continuations.iter()
            .any(|row| row.vis_origin == vis_origin)
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

    /// Return the exact installed rows and all response demands that can be
    /// resolved into existing K contexts without installing a new owner. A
    /// returned-Vis analysis may cite a derived row from this population, but
    /// this projection does not change the owner or emission populations.
    pub(super) fn return_protocol_candidate_rows(
        &self,
    ) -> Result<Vec<StaticResponseContinuation>, CraneliftBackendError> {
        let (demands, _marked, _deferred) = self
            .static_response_context_demands_filtered(None, false)?
            .map_err(|failure| planner_error(format!(
                "returned-Vis response demand is infeasible at {:?}: {}",
                failure.vis_origin(), failure.reason(),
            )))?;
        let mut resolved = self.resolve_static_response_context_demands(
            demands,
            &self.continuation_contexts,
            self.continuation_contexts.len(),
        )?;
        let mut rows = self.static_response_continuations.clone();
        for mut row in resolved.drain(..) {
            if rows.iter().any(|existing| existing.vis_origin() == row.vis_origin()) {
                continue;
            }
            row.id = StaticResponseContinuationId::from_position(rows.len())?;
            rows.push(row);
        }
        Ok(rows)
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
    pub(in crate::cranelift_backend) fn checked_ih_post_call_consumer(
        &self,
        transport: &CheckedIhEnvironmentTransport,
    ) -> Result<Option<&CheckedIhPostCallConsumer>, CraneliftBackendError> {
        let mut matching = self
            .checked_ih_post_call_consumers
            .iter()
            .filter(|row| row.transport() == transport);
        let Some(row) = matching.next() else {
            return Ok(None);
        };
        if matching.next().is_some() {
            return Err(planner_error(
                "one checked-IH transport endpoint resolves more than one post-call consumer",
            ));
        }
        Ok(Some(row))
    }

    pub(in crate::cranelift_backend) fn static_response_forwarded_result_identity(
        &self,
        selected_caller: &ContinuationCallIdentity,
    ) -> Result<Option<ConstructorIdentity>, CraneliftBackendError> {
        let owners = self
            .static_response_owner_specializations()?
            .map_err(|infeasible| {
                planner_error(format!(
                    "a static-response Result forwarding boundary is infeasible at {:?}: {}",
                    infeasible.vis_origin(),
                    infeasible.reason(),
                ))
            })?;
        let matching_owners = owners
            .iter()
            .filter(|owner| owner.selected_caller() == selected_caller)
            .collect::<Vec<_>>();
        let owner = match matching_owners.as_slice() {
            [] => return Ok(None),
            [owner] => *owner,
            _ => {
                return Err(planner_error(
                    "one selected caller resolves more than one static response owner",
                ))
            }
        };
        let matching_rows = self
            .static_response_continuations
            .iter()
            .filter(|row| row.id() == owner.response())
            .collect::<Vec<_>>();
        let row = match matching_rows.as_slice() {
            [row] => *row,
            [] => {
                return Err(planner_error(
                    "a static response owner has no exact response row",
                ))
            }
            _ => {
                return Err(planner_error(
                    "a static response owner has more than one exact response row",
                ))
            }
        };
        let context = self
            .continuation_contexts
            .iter()
            .find(|context| context.id() == owner.k_context())
            .ok_or_else(|| {
                planner_error("a static-response Result forwarding boundary has no exact K context")
            })?;
        if owner.base_owner() != row.base_owner()
            || owner.selected_caller() != row.k_identity()
            || owner.k_context() != row.k_context()
            || row.k_identity() != selected_caller
            || row.k_specialization() != selected_caller.target()
            || context.enclosing_specialization() != selected_caller.target()
            || context.worker_body_origin() != row.k_body_origin()
        {
            return Err(planner_error(
                "a static-response Result forwarding boundary does not join its response row, owner, complete selected caller and K context",
            ));
        }
        Ok(Some(row.k_ret_identity()))
    }

    pub(in crate::cranelift_backend) fn checked_ih_static_response_return_boundary(
        &self,
        selected_caller: &ContinuationCallIdentity,
    ) -> Result<Option<CheckedIhStaticResponseReturnBoundary>, CraneliftBackendError> {
        let owners = self
            .static_response_owner_specializations()?
            .map_err(|infeasible| {
                planner_error(format!(
                    "a static-response return boundary is infeasible at {:?}: {}",
                    infeasible.vis_origin(),
                    infeasible.reason(),
                ))
            })?;
        let matching_owners = owners
            .iter()
            .filter(|owner| owner.selected_caller() == selected_caller)
            .cloned()
            .collect::<Vec<_>>();
        let mut owner = match matching_owners.as_slice() {
            [] => return Ok(None),
            [owner] => owner.clone(),
            _ => {
                return Err(planner_error(
                    "one selected caller resolves more than one static response owner",
                ))
            }
        };
        let matching_rows = self
            .static_response_continuations
            .iter()
            .filter(|row| row.id() == owner.response())
            .collect::<Vec<_>>();
        let row = match matching_rows.as_slice() {
            [row] => (*row).clone(),
            [] => {
                return Err(planner_error(
                    "a static response owner has no exact response row",
                ))
            }
            _ => {
                return Err(planner_error(
                    "a static response owner has more than one exact response row",
                ))
            }
        };
        #[cfg(feature = "px8-ds-test-support")]
        if d5b_hs17_post_call_consumer_mutation()
            == D5bHs17PostCallConsumerMutation::TransplantStaticResponseBoundary
        {
            let substitute = owners
                .iter()
                .filter(|candidate| candidate.id() != owner.id())
                .find(|candidate| {
                    self.static_response_continuations.iter().any(|candidate_row| {
                        candidate_row.id() == candidate.response()
                            && candidate_row.k_ret_identity() == row.k_ret_identity()
                    })
                })
                .cloned()
                .ok_or_else(|| {
                    planner_error(
                        "the HS17 static-response transplant control found no equal-Ret foreign owner",
                    )
                })?;
            owner = substitute;
            record_d5b_hs17_post_call_consumer_application();
        }
        if owner.base_owner() != row.base_owner()
            || owner.selected_caller() != row.k_identity()
            || owner.k_context() != row.k_context()
            || row.k_identity() != selected_caller
            || row.k_specialization() != selected_caller.target()
        {
            return Err(planner_error(
                "a static-response return boundary does not join its response row, owner and complete selected caller",
            ));
        }
        let context = self
            .continuation_contexts
            .iter()
            .find(|context| context.id() == owner.k_context())
            .ok_or_else(|| {
                planner_error("a static-response return boundary has no exact K context")
            })?;
        if context.enclosing_specialization() != selected_caller.target()
            || context.worker_body_origin() != row.k_body_origin()
        {
            return Err(planner_error(
                "a static-response return boundary does not join its complete selected caller to the exact K context",
            ));
        }
        // A K whose returned-result route returns a Vis member does not
        // complete its caller's detached return itself: its Protocol owner
        // drives that Vis until the owner's own tag guard sees Ret, so the
        // Ret obligation is the owner's, not a publication contract of K.
        let k_returns_vis = match self.pending_vis_settlements.owners.get(&row.id()) {
            Some(super::returned_vis::ResponseOwnerSettlement::Protocol) => self
                .pending_vis_settlements
                .protocol
                .as_ref()
                .and_then(|protocol| protocol.owners.get(&row.id()))
                .ok_or_else(|| {
                    planner_error(
                        "a Protocol response owner has no returned-Vis protocol record",
                    )
                })?
                .contexts
                .iter()
                .any(|returned| {
                    returned.context == owner.k_context() && !returned.members.is_empty()
                }),
            Some(
                super::returned_vis::ResponseOwnerSettlement::RetOnly
                | super::returned_vis::ResponseOwnerSettlement::Excluded { .. },
            ) => false,
            None => {
                return Err(planner_error(
                    "a static-response return boundary names a response row with no owner settlement",
                ))
            }
        };
        if k_returns_vis {
            return Ok(None);
        }

        let mut boundaries = Vec::new();
        for caller_transport in self
            .checked_ih_environment_transports
            .iter()
            .filter(|transport| transport.source_call_identity() == selected_caller)
        {
            let Some(caller_cut) = self.checked_ih_detached_caller_cut(caller_transport)? else {
                continue;
            };
            let matching_consumers = self
                .checked_ih_post_call_consumers
                .iter()
                .filter(|consumer| {
                    consumer.transport() == caller_cut.producer_transport()
                        && consumer.detached_return_context().is_some_and(|template| {
                            find_worker_return_boundary(template, selected_caller).is_some()
                        })
                })
                .collect::<Vec<_>>();
            let consumer = match matching_consumers.as_slice() {
                [consumer] => (*consumer).clone(),
                [] => continue,
                _ => {
                    return Err(planner_error(
                        "one static-response return boundary resolves more than one detached checked-IH consumer",
                    ))
                }
            };
            if consumer.demanded_result_identity() != row.k_ret_identity()
                || caller_cut.selecting_call() != selected_caller
                || caller_cut.caller_transport() != caller_transport
            {
                return Err(planner_error(
                    "a static-response return boundary disagrees with its Result demand or retained checked-IH endpoints",
                ));
            }
            let caller_exit_index =
                checked_ih_post_call_caller_exit_index(&consumer, &caller_cut)?;
            let incoming_consumer_edge_index =
                checked_ih_caller_completed_required_consumer_edge_index(
                    self,
                    &consumer,
                    caller_exit_index,
                )?;
            let boundary = CheckedIhStaticResponseReturnBoundary {
                response: row.clone(),
                owner: owner.clone(),
                consumer,
                required_consumer_edge:
                    RequiredConsumerIncomingEdgeSelection::CallerCompleted {
                        caller_cut,
                        caller_exit_index,
                        incoming_consumer_edge_index,
                    },
            };
            boundary.required_consumer_incoming_edge()?;
            boundaries.push(boundary);
        }
        #[cfg(feature = "px8-ds-test-support")]
        if !boundaries.is_empty()
            && d5b_hs17_post_call_consumer_mutation()
                == D5bHs17PostCallConsumerMutation::DeleteStaticResponseBoundary
        {
            record_d5b_hs17_post_call_consumer_application();
            return Ok(None);
        }
        match boundaries.as_slice() {
            [] => Ok(None),
            [boundary] => Ok(Some(boundary.clone())),
            _ => Err(planner_error(
                "one selected static-response caller has more than one complete detached return boundary",
            )),
        }
    }

    pub(in crate::cranelift_backend) fn checked_ih_detached_caller_cut(
        &self,
        caller_transport: &CheckedIhEnvironmentTransport,
    ) -> Result<Option<CheckedIhDetachedCallerCut>, CraneliftBackendError> {
        let mut matches = Vec::new();
        let mut saw_boundary = false;
        for row in &self.checked_ih_post_call_consumers {
            let (row_saw_boundary, caller_cut) =
                checked_ih_detached_caller_cut_for_consumer(self, row, caller_transport)?;
            saw_boundary |= row_saw_boundary;
            if let Some(caller_cut) = caller_cut {
                matches.push(caller_cut);
            }
        }
        match matches.as_slice() {
            [] if saw_boundary => Err(planner_error(
                "a detached checked-IH caller cut has no exact producer-to-caller transport endpoint join",
            )),
            [] => Ok(None),
            [cut] => Ok(Some(cut.clone())),
            _ => Err(planner_error(
                "one checked-IH caller transport has more than one detached source return cut",
            )),
        }
    }

    pub(in crate::cranelift_backend) fn checked_ih_detached_caller_construct_binds(
        &self,
        cut: &CheckedIhDetachedCallerCut,
        construct_origin: StaticOriginId,
    ) -> Result<bool, CraneliftBackendError> {
        Ok(occurrence_subtree_contains(
            self,
            cut.selected_body_origin,
            construct_origin,
        )?)
    }

    pub(in crate::cranelift_backend) fn checked_ih_generated_context_result_contract(
        &self,
        context: ContinuationContextId,
    ) -> Result<Option<ConstructorIdentity>, CraneliftBackendError> {
        let planned_context = self
            .continuation_contexts
            .iter()
            .find(|candidate| candidate.id == context)
            .ok_or_else(|| {
                planner_error("a generated-context Result contract names no planned context")
            })?;
        let selected_callers = self
            .static_response_continuations
            .iter()
            .filter(|row| row.k_context() == context)
            .map(StaticResponseContinuation::k_identity)
            .collect::<Vec<_>>();
        let mut has_completed_boundary = false;
        for selected_caller in selected_callers {
            has_completed_boundary |= self
                .checked_ih_static_response_return_boundary(selected_caller)?
                .is_some();
        }
        let contract = if has_completed_boundary {
            let mut identities = Vec::new();
            for identity in self
                .static_response_continuations
                .iter()
                .filter(|row| row.k_context == context)
                .map(|row| row.k_ret_identity)
            {
                if !identities.contains(&identity) {
                    identities.push(identity);
                }
            }
            match identities.as_slice() {
                [] => None,
                [identity] => Some(*identity),
                _ => {
                    return Err(planner_error(
                        "one generated context has disagreeing response Result contracts",
                    ));
                }
            }
        } else {
            None
        };
        #[cfg(feature = "px8-ds-test-support")]
        super::record_generated_context_result_contract_diagnostic(context, contract);
        Ok(contract)
    }

    pub(in crate::cranelift_backend) fn checked_ih_post_call_actual_identity_for_target(
        &self,
        target: ContinuationSpecializationId,
    ) -> Result<Option<ConstructorIdentity>, CraneliftBackendError> {
        let mut identities = Vec::new();
        for identity in self
            .checked_ih_post_call_consumers
            .iter()
            .filter(|row| row.transport().source_specialization() == target)
            .map(|row| row.actual_result_identity)
        {
            if !identities.contains(&identity) {
                identities.push(identity);
            }
        }
        match identities.as_slice() {
            [] => Ok(None),
            [identity] => Ok(Some(*identity)),
            _ => Err(planner_error(
                "one continuation target has disagreeing checked-IH post-call actual Result identities",
            )),
        }
    }

}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{
        CheckedComputationalIHBinderMorphism, RuntimeComputationalMatchCase, RuntimeDeclaration,
        RuntimeDeclarationKind, RuntimeMatchCase, RuntimeSymbolMetadata, RuntimeTrap,
        RuntimeTrapCode, RuntimeValue,
    };

    fn trap() -> RuntimeTrap {
        RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "static response fixture is total".to_string(),
        }
    }

    fn host_response_case(constructor: &str) -> RuntimeMatchCase {
        RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 1,
            body: RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::Effect {
                    bracket_acquire: None,
                    family: "effect:FS".to_string(),
                    operation: HostOpV1::BufferAllocate,
                    capability: None,
                    args: vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
                }),
                body: Box::new(RuntimeExpr::CheckedComputationalIHInvocation {
                    call_template_id: 1,
                    checked_occurrence_path: Vec::new(),
                    kind: CheckedComputationalIHInvocationKind::CheckedHostVisContinuation,
                    binder_morphism: CheckedComputationalIHBinderMorphism::identity_for_test(0),
                    body: Box::new(RuntimeExpr::Call {
                        callee: Box::new(RuntimeExpr::Var(0)),
                        args: vec![RuntimeExpr::Var(0)],
                    }),
                }),
            },
        }
    }

    fn host_response_root(constructor: &str) -> RuntimeExpr {
        RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(1)),
            cases: vec![host_response_case(constructor)],
            default: trap(),
        }
    }

    #[test]
    fn same_leaf_cannot_claim_one_constructor_twice() {
        let constructor = "ctor:fixture::FSOp::Allocate";
        let root = RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(1)),
            cases: vec![host_response_case(constructor), host_response_case(constructor)],
            default: trap(),
        };
        let error = match super::super::plan_static_transition_graph(&root, &BTreeMap::new()) {
            Err(error) => error,
            Ok(_) => panic!("one producer leaf must reject a duplicate case before lookup"),
        };
        assert!(
            format!("{error:?}").contains("two host response cases claim one operation constructor"),
            "unexpected response-route refusal: {error:?}"
        );
    }

    #[test]
    fn source_inl_match_is_not_the_generated_dispatch_root() {
        let root = RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::Coproduct::InL".to_string(),
                binders: 1,
                body: host_response_root("ctor:fixture::FSOp::Allocate"),
            }],
            default: trap(),
        };
        let plan = super::super::plan_static_transition_graph(&root, &BTreeMap::new())
            .expect("the source-match wrapper is a representable planned expression");
        let outer = plan.source_occurrences.iter().flatten()
            .find(|entry| matches!(entry.expr, RuntimeExpr::Match { cases, .. }
                if cases.iter().any(|case| case.constructor.ends_with("::Coproduct::InL"))))
            .expect("source match is in the plan")
            .static_origin;
        let routes = host_response_routes(&plan).expect("one response case has one route");
        assert_eq!(routes.len(), 1);
        assert!(
            !routes.contains_key(&outer),
            "a source InL Match surrounding the producer is not itself a producer leaf"
        );
        let (&leaf, cases) = routes.iter().next().unwrap();
        assert_ne!(leaf, outer);
        assert!(cases.contains_key("ctor:fixture::FSOp::Allocate"));
    }

    fn two_leaf_response_fixture(nested: bool) -> RuntimeExpr {
        let constructor = "ctor:fixture::FSOp::Allocate";
        let response_match = |binder| RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(binder)),
            cases: vec![host_response_case(constructor)],
            default: trap(),
        };
        let dispatch = RuntimeExpr::If {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            then_expr: Box::new(response_match(if nested { 0 } else { 1 })),
            else_expr: Box::new(response_match(if nested { 3 } else { 2 })),
        };
        let body = if nested {
            RuntimeExpr::Match {
                scrutinee: Box::new(RuntimeExpr::Var(1)),
                cases: vec![RuntimeMatchCase {
                    constructor: "ctor:fixture::Coproduct::InL".to_string(),
                    binders: 1,
                    body: dispatch,
                }],
                default: trap(),
            }
        } else {
            dispatch
        };
        RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::ITree::Vis".to_string(),
                args: vec![
                    RuntimeExpr::Construct {
                        constructor: constructor.to_string(),
                        args: Vec::new(),
                    },
                    RuntimeExpr::Value(RuntimeValue::Unknown),
                ],
            }),
            cases: vec![RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::ITree::Vis".to_string(),
                argument_binders: 2,
                recursive_positions: vec![1],
                body,
            }],
            default: trap(),
        }
    }

    // Promise class: durable invariant. MEASURED: two candidates owned by
    // the same synthetic Vis include one reached and one unpaired in a nested
    // dispatch; planning refuses at the multi-candidate structural path.
    // CLAIMED: an owned but unreached candidate cannot be silently omitted.
    // THE GAP: source reachability is established separately by the checked
    // two-bracket witness, not by this hand-built runtime-IR fixture.
    #[test]
    fn nested_unpaired_response_candidate_owned_by_same_vis_refuses() {
        let root = two_leaf_response_fixture(true);
        let error = match super::super::plan_static_transition_graph(&root, &BTreeMap::new()) {
            Err(error) => error,
            Ok(_) => panic!("an unpaired owned response candidate must refuse"),
        };
        assert!(
            format!("{error:?}").contains("one response Vis selects a constructor with more than one host response occurrence and no structural path to exactly one of them"),
            "unexpected nested response-route refusal: {error:?}"
        );
    }

    // Promise class: durable invariant. MEASURED: an unpaired top-level
    // response exit refuses with the same multi-candidate path diagnostic.
    // CLAIMED: the existing top-level completeness guard remains active.
    // THE GAP: this is synthetic IR, not a checked Ken source observation.
    #[test]
    fn top_level_unpaired_response_candidate_still_refuses() {
        let root = two_leaf_response_fixture(false);
        let error = match super::super::plan_static_transition_graph(&root, &BTreeMap::new()) {
            Err(error) => error,
            Ok(_) => panic!("an unpaired top-level response candidate must refuse"),
        };
        assert!(
            format!("{error:?}").contains("one response Vis selects a constructor with more than one host response occurrence and no structural path to exactly one of them"),
            "unexpected top-level response-route refusal: {error:?}"
        );
    }

    #[test]
    fn source_match_on_op_binder_selects_its_effect_leaf() {
        let constructor = "ctor:fixture::FSOp::Allocate";
        let vis = RuntimeExpr::Construct {
            constructor: "ctor:fixture::ITree::Vis".to_string(),
            args: vec![
                RuntimeExpr::Construct {
                    constructor: constructor.to_string(),
                    args: Vec::new(),
                },
                RuntimeExpr::Value(RuntimeValue::Unknown),
            ],
        };
        // The source Match itself scrutinizes the CM's checked operation
        // binder, performs an Effect, and calls its checked continuation.
        // Another leaf outside the CM has the identical constructor: the
        // single-candidate shortcut cannot select the desired producer.
        let root = RuntimeExpr::If {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            then_expr: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(vis),
                cases: vec![RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::ITree::Vis".to_string(),
                    argument_binders: 2,
                    recursive_positions: vec![1],
                    body: host_response_root(constructor),
                }],
                default: trap(),
            }),
            else_expr: Box::new(host_response_root(constructor)),
        };
        let plan = super::super::plan_static_transition_graph(&root, &BTreeMap::new())
            .expect("both source response producers are representable");
        let cm = plan.source_occurrences.iter().flatten()
            .find(|entry| matches!(entry.expr, RuntimeExpr::ComputationalMatch { .. }))
            .expect("source computational match has an origin")
            .static_origin;
        let vis_origin = plan.semantic.child_origin(cm, 0).unwrap();
        let operation_origin = plan.semantic.child_origin(vis_origin, 0).unwrap();
        let source_match = plan.semantic.child_origin(cm, 1).unwrap();
        let routes = host_response_routes(&plan).unwrap();
        assert_eq!(routes.len(), 2, "the second producer requires the descent");
        let selected = selected_host_response_route(&plan, vis_origin, operation_origin, &routes)
            .unwrap()
            .expect("the op binder flows into the source Match Effect arm");
        assert_eq!(
            selected.0.producer_call_origin,
            routes[&source_match][constructor].producer_call_origin,
            "the checked op binder selects the source Match producer, not its sibling"
        );
    }

    #[test]
    fn ambiguous_vis_in_call_argument_has_no_structural_dispatch_path() {
        let constructor = "ctor:fixture::FSOp::Allocate";
        let vis = RuntimeExpr::Construct {
            constructor: "ctor:fixture::ITree::Vis".to_string(),
            args: vec![
                RuntimeExpr::Construct {
                    constructor: constructor.to_string(),
                    args: Vec::new(),
                },
                RuntimeExpr::Value(RuntimeValue::Unknown),
            ],
        };
        let root = RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Value(RuntimeValue::Unknown)),
                args: vec![vis],
            }),
            cases: vec![RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::ITree::Vis".to_string(),
                argument_binders: 2,
                recursive_positions: vec![1],
                body: RuntimeExpr::If {
                    scrutinee: Box::new(RuntimeExpr::Var(0)),
                    then_expr: Box::new(host_response_root(constructor)),
                    else_expr: Box::new(host_response_root(constructor)),
                },
            }],
            default: trap(),
        };
        let error = match super::super::plan_static_transition_graph(&root, &BTreeMap::new()) {
            Err(error) => error,
            Ok(_) => panic!("Call argument cannot convey the Vis to the dispatch"),
        };
        assert!(
            format!("{error:?}").contains("one response Vis selects a constructor with more than one host response occurrence and no structural path to exactly one of them"),
            "unexpected response-route refusal: {error:?}"
        );
    }

    #[test]
    fn zero_binder_computational_case_can_forward_a_vis_operation() {
        let root = RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            cases: vec![RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::Tag::Only".to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::Construct {
                    constructor: "ctor:fixture::ITree::Vis".to_string(),
                    args: vec![
                        RuntimeExpr::Var(1),
                        RuntimeExpr::Value(RuntimeValue::Unknown),
                    ],
                },
            }],
            default: trap(),
        };
        let plan = super::super::plan_static_transition_graph(&root, &BTreeMap::new())
            .expect("the forward-case fixture has planned source occurrences");
        let cm = plan.source_occurrences.iter().flatten()
            .find(|entry| matches!(entry.expr, RuntimeExpr::ComputationalMatch { .. }))
            .expect("the computational match has a source origin")
            .static_origin;
        assert!(response_forwards_vis(&plan, cm, 1).unwrap());
        assert!(!response_forwards_vis(&plan, cm, 0).unwrap());
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
                        bracket_acquire: None,
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

    /// Promise class: durable invariant. MEASURED: a K whose only tail exits
    /// call a uniquely resolved countdown declaration is direct-answering;
    /// invoking the same walk *inside* a declaration whose tail builds an
    /// ITree::Ret or ITree::Vis is not. CLAIMED: P1 admits only direct-answering
    /// static tail callees. THE GAP: the negative arm uses a RuntimeExpr fixture
    /// shaped like the px8ds ITree-form copy, not the checked occurrence 659.
    /// AC-1 and its guard mutation pin the positive arm on checked px8ds.
    #[test]
    fn rt_p1_tail_walk_direct_countdown_and_itree_copy_discriminate() {
        let call_countdown = || RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::DeclarationRef {
                symbol: "decl:fixture::countdown".to_string(),
            }),
            args: vec![RuntimeExpr::Var(0)],
        };
        let k_body = RuntimeExpr::If {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            then_expr: Box::new(call_countdown()),
            else_expr: Box::new(call_countdown()),
        };
        let countdown = RuntimeDeclaration {
            symbol: "decl:fixture::countdown".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["fuel".to_string()],
                    body: Box::new(RuntimeExpr::If {
                        scrutinee: Box::new(RuntimeExpr::Var(0)),
                        then_expr: Box::new(RuntimeExpr::Construct {
                            constructor: "ctor:fixture::Result::Ok".to_string(),
                            args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
                        }),
                        else_expr: Box::new(call_countdown()),
                    }),
                },
            },
            metadata: RuntimeSymbolMetadata::empty(),
        };
        for role in ["Ret", "Vis"] {
            let itree_copy = RuntimeDeclaration {
                symbol: "decl:fixture::itree_copy".to_string(),
                kind: RuntimeDeclarationKind::Transparent {
                    body: RuntimeExpr::LexicalClosure {
                        captures: Vec::new(),
                        params: vec!["answer".to_string()],
                        body: Box::new(RuntimeExpr::Construct {
                            constructor: format!("ctor:fixture::ITree::{role}"),
                            args: vec![RuntimeExpr::Var(0)],
                        }),
                    },
                },
                metadata: RuntimeSymbolMetadata::empty(),
            };
            let declarations = BTreeMap::from([
                (countdown.symbol.as_str(), &countdown),
                (itree_copy.symbol.as_str(), &itree_copy),
            ]);
            let plan = super::super::plan_static_transition_graph(&k_body, &declarations)
                .expect("the direct-countdown and ITree-form copy fixtures plan");
            let k_origin = plan
                .source_occurrences
                .iter()
                .flatten()
                .find(|entry| matches!(entry.expr, RuntimeExpr::If { .. }))
                .expect("K has a checked branch occurrence")
                .static_origin;
            let copy_decl = plan
                .declaration_occurrence_origin(&itree_copy.symbol)
                .expect("ITree-form declaration has an origin");
            let copy_body = plan
                .semantic
                .child_origin(copy_decl, 0)
                .expect("ITree-form closure has a body");
            assert!(
                deferred_response_tail_callees_answer_directly(&plan, k_origin)
                    .expect("the K's static call targets resolve")
            );
            assert!(
                !tail_answers_directly(&plan, copy_body, &mut BTreeSet::new(), true)
                    .expect("the ITree-form copy's tail is classified"),
                "a Construct ITree::{role} tail must not answer directly"
            );
        }
    }

    /// Promise class: durable invariant. MEASURED: a K that calls a declaration
    /// with a ComputationalMatch body ending in ITree::Ret is refused by the
    /// production tail walk. The distinct edge entry alone answers directly,
    /// whereas the body does not. CLAIMED: declaration tail admission walks
    /// the callee body,
    /// not its scheduling entry. THE GAP: this RuntimeExpr fixture is not a
    /// checked-source occurrence; it pins the specific entry/body split and
    /// the planner's classification, not every computational-match body.
    #[test]
    fn rt_p1_tail_walk_computational_match_callee_uses_body_axis() {
        let k_body = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::DeclarationRef {
                symbol: "decl:fixture::probe".to_string(),
            }),
            args: vec![RuntimeExpr::Var(0)],
        };
        let probe = RuntimeDeclaration {
            symbol: "decl:fixture::probe".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["x".to_string()],
                    body: Box::new(RuntimeExpr::ComputationalMatch {
                        scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Constructor {
                            constructor: "ctor:fixture::N::Z".to_string(),
                            args: Vec::new(),
                        })),
                        cases: vec![RuntimeComputationalMatchCase {
                            constructor: "ctor:fixture::N::Z".to_string(),
                            argument_binders: 0,
                            recursive_positions: Vec::new(),
                            body: RuntimeExpr::Construct {
                                constructor: "ctor:fixture::ITree::Ret".to_string(),
                                args: vec![RuntimeExpr::Var(0)],
                            },
                        }],
                        default: trap(),
                    }),
                },
            },
            metadata: RuntimeSymbolMetadata::empty(),
        };
        let declarations = BTreeMap::from([(probe.symbol.as_str(), &probe)]);
        let plan = super::super::plan_static_transition_graph(&k_body, &declarations)
            .expect("the computational-match callee fixture plans");
        let k_origin = plan
            .source_occurrences
            .iter()
            .flatten()
            .find(|entry| matches!(entry.expr, RuntimeExpr::Call { .. }))
            .expect("K has a call occurrence")
            .static_origin;
        let callee = plan
            .semantic
            .child_origin(k_origin, 0)
            .expect("K names a declaration call site");
        let edges = plan
            .emittable_call_edges()
            .expect("the declaration call edges resolve")
            .into_iter()
            .filter(|edge| {
                edge.kind() == super::super::units::EmittableCallKind::Declaration
                    && edge.call_site_origin() == callee
            })
            .collect::<Vec<_>>();
        let [edge] = edges.as_slice() else {
            panic!("K must have exactly one declaration call edge");
        };
        let unit = plan
            .emittable_units()
            .expect("the declaration callee has a unit")
            .into_iter()
            .find(|unit| unit.function() == edge.callee())
            .expect("the call edge resolves to the declaration callee's unit");
        assert_eq!(edge.callee_origin(), unit.entry_origin());
        assert_ne!(
            edge.callee_origin(),
            unit.body_occurrence(),
            "the callee entry must be distinct from its ComputationalMatch body"
        );
        assert!(
            tail_answers_directly(&plan, edge.callee_origin(), &mut BTreeSet::new(), true)
                .expect("the callee's scheduling entry can be classified"),
            "the wrong entry axis alone misses the ITree-form tail"
        );
        assert!(matches!(
            plan.planned_occurrence_expr(unit.body_occurrence())
                .expect("the callee body has a planned occurrence"),
            RuntimeExpr::ComputationalMatch { .. }
        ));
        assert!(
            !tail_answers_directly(&plan, unit.body_occurrence(), &mut BTreeSet::new(), true)
                .expect("the ITree-form callee body is not direct-answering")
        );
        assert!(
            !deferred_response_tail_callees_answer_directly(&plan, k_origin)
                .expect("K resolves its declaration callee"),
            "the K call must be refused when its callee body constructs ITree::Ret"
        );
    }

    /// Promise class: durable invariant. MEASURED: a K whose only tail exit
    /// calls a declaration that forwards its argument is not direct-answering.
    /// CLAIMED: P1 never admits a static tail callee that can return an ITree
    /// word it did not construct. THE GAP: RuntimeExpr fixture, not a checked
    /// occurrence; a direct-style callee whose tail is a Var is refused too,
    /// conservatively.
    #[test]
    fn rt_p1_tail_walk_forwarding_callee_is_not_direct() {
        let k_body = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::DeclarationRef {
                symbol: "decl:fixture::pass".to_string(),
            }),
            args: vec![RuntimeExpr::Var(0)],
        };
        let pass = RuntimeDeclaration {
            symbol: "decl:fixture::pass".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["tree".to_string()],
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            },
            metadata: RuntimeSymbolMetadata::empty(),
        };
        let declarations = BTreeMap::from([(pass.symbol.as_str(), &pass)]);
        let plan = super::super::plan_static_transition_graph(&k_body, &declarations)
            .expect("the forwarding fixture plans");
        let k_origin = plan
            .source_occurrences
            .iter()
            .flatten()
            .find(|entry| matches!(entry.expr, RuntimeExpr::Call { .. }))
            .expect("K has a call occurrence")
            .static_origin;
        assert!(
            !deferred_response_tail_callees_answer_directly(&plan, k_origin)
                .expect("the K's static call target resolves"),
            "a forwarding callee must not count as direct-answering"
        );
    }

    /// Promise class: durable invariant. MEASURED: a declaration tail that
    /// projects a field from its input is refused by the production planner's
    /// direct-answer walk. CLAIMED: a projected ITree word cannot be admitted
    /// as an answer constructed by the callee. THE GAP: this RuntimeExpr
    /// fixture is not a checked source occurrence, and the conservative
    /// refusal also includes projections of direct-style answers.
    #[test]
    fn rt_p1_tail_walk_projected_callee_is_not_direct() {
        let k_body = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::DeclarationRef {
                symbol: "decl:fixture::project_tree".to_string(),
            }),
            args: vec![RuntimeExpr::Var(0)],
        };
        let project_tree = RuntimeDeclaration {
            symbol: "decl:fixture::project_tree".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["record".to_string()],
                    body: Box::new(RuntimeExpr::Project {
                        record: Box::new(RuntimeExpr::Var(0)),
                        field: "tree".to_string(),
                    }),
                },
            },
            metadata: RuntimeSymbolMetadata::empty(),
        };
        let declarations = BTreeMap::from([(project_tree.symbol.as_str(), &project_tree)]);
        let plan = super::super::plan_static_transition_graph(&k_body, &declarations)
            .expect("the projected-callee fixture plans");
        let k_origin = plan
            .source_occurrences
            .iter()
            .flatten()
            .find(|entry| matches!(entry.expr, RuntimeExpr::Call { .. }))
            .expect("K has a call occurrence")
            .static_origin;
        assert!(
            !deferred_response_tail_callees_answer_directly(&plan, k_origin)
                .expect("the K's static call target resolves"),
            "a callee projecting an input field must not count as direct-answering"
        );
    }

    /// Promise class: durable invariant. MEASURED: a declaration tail with
    /// literal ITree::Ret and ITree::Vis RuntimeValues are refused by the
    /// production planner's direct-answer walk. CLAIMED: an ITree-form value,
    /// whether held as a Value or built as a Construct, is not a direct answer.
    /// THE GAP: this RuntimeExpr fixture is not px8ds's checked ITree-form
    /// occurrence; the identity convention still keys on constructor suffix.
    #[test]
    fn rt_p1_tail_walk_itree_value_callee_is_not_direct() {
        for role in ["Ret", "Vis"] {
            let k_body = RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::DeclarationRef {
                    symbol: "decl:fixture::value_tree".to_string(),
                }),
                args: vec![RuntimeExpr::Var(0)],
            };
            let value_tree = RuntimeDeclaration {
                symbol: "decl:fixture::value_tree".to_string(),
                kind: RuntimeDeclarationKind::Transparent {
                    body: RuntimeExpr::LexicalClosure {
                        captures: Vec::new(),
                        params: vec!["answer".to_string()],
                        body: Box::new(RuntimeExpr::Value(RuntimeValue::Constructor {
                            constructor: format!("ctor:fixture::ITree::{role}"),
                            args: vec![RuntimeValue::Bool(true)],
                        })),
                    },
                },
                metadata: RuntimeSymbolMetadata::empty(),
            };
            let declarations = BTreeMap::from([(value_tree.symbol.as_str(), &value_tree)]);
            let plan = super::super::plan_static_transition_graph(&k_body, &declarations)
                .expect("the ITree-value callee fixture plans");
            let k_origin = plan
                .source_occurrences
                .iter()
                .flatten()
                .find(|entry| matches!(entry.expr, RuntimeExpr::Call { .. }))
                .expect("K has a call occurrence")
                .static_origin;
            assert!(
                !deferred_response_tail_callees_answer_directly(&plan, k_origin)
                    .expect("the K's static call target resolves"),
                "an ITree::{role} Value callee must not count as direct-answering"
            );
        }
    }
}


fn find_worker_return_boundary<'a>(
    context: &'a SourceReturnContextTemplate,
    identity: &ContinuationCallIdentity,
) -> Option<&'a super::continuations::SourceWorkerReturnBoundary> {
    let boundary = context.worker_return()?;
    if boundary.selecting_call() == identity {
        Some(boundary)
    } else {
        find_worker_return_boundary(boundary.caller_context(), identity)
    }
}

fn checked_ih_detached_caller_cut_for_consumer(
    plan: &StaticTransitionPlan<'_>,
    row: &CheckedIhPostCallConsumer,
    caller_transport: &CheckedIhEnvironmentTransport,
) -> Result<(bool, Option<CheckedIhDetachedCallerCut>), CraneliftBackendError> {
    let Some(context) = row.detached_return_context() else {
        return Ok((false, None));
    };
    let identity = caller_transport.source_call_identity();
    let Some(boundary) = find_worker_return_boundary(context, identity) else {
        return Ok((false, None));
    };
    if boundary.caller_context().result_origin()
        != caller_transport.destination_construct_origin()
        || row.transport().destination_owner() != caller_transport.source_owner()
        || row.transport().destination_body_origin() != context.root_origin()
    {
        return Ok((true, None));
    }
    let target = plan
        .continuation_specializations
        .get(identity.target().0 as usize)
        .ok_or_else(|| {
            planner_error("a detached checked-IH caller cut names an uninstalled selecting target")
        })?;
    if target.key.worker.body_origin != context.root_origin()
        || target.key.producer_construct_origin != boundary.caller_context().result_origin()
    {
        return Err(planner_error(
            "a detached checked-IH caller cut disagrees with its selecting specialization",
        ));
    }
    let selected_body_origin = plan.semantic.child_origin(
        target.key.continuation_origin,
        1 + target.key.producer_alternative as usize,
    )?;
    let consumer_index = row
        .selected_case_exits
        .iter()
        .position(|step| {
            step.occurrence().eliminator_origin() == target.key.continuation_origin
        })
        .ok_or_else(|| {
            planner_error(
                "a detached checked-IH caller cut did not consume its selecting continuation",
            )
        })?;
    let consumer_step = row
        .selected_case_exits
        .get(consumer_index)
        .ok_or_else(|| planner_error("a detached checked-IH caller cut lost its consumer"))?;
    if !row.selected_case_exits[..consumer_index]
        .iter()
        .all(|step| step.occurrence().eliminator_origin() != target.key.continuation_origin)
    {
        return Err(planner_error(
            "a detached checked-IH caller cut exits its selecting continuation more than once",
        ));
    }
    let consumed_caller_suffix = boundary
        .caller_context()
        .caller_suffix()
        .iter()
        .map(|origin| {
            Ok(CheckedIhDetachedConsumedFrame {
                origin: *origin,
                checked_frame_id: checked_frame_for_consumer(plan, *origin)?,
            })
        })
        .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
    if consumed_caller_suffix
        .iter()
        .any(|frame| frame.origin == target.key.continuation_origin)
    {
        return Err(planner_error(
            "a detached checked-IH caller suffix replays its selecting continuation",
        ));
    }
    Ok((
        true,
        Some(CheckedIhDetachedCallerCut {
            selecting_call: identity.clone(),
            caller_result_origin: boundary.caller_context().result_origin(),
            consumed_continuation_origin: target.key.continuation_origin,
            selected_body_origin,
            checked_frame_id: consumer_step.checked_frame_id(),
            consumed_caller_suffix,
            producer_transport: row.transport().clone(),
            caller_transport: caller_transport.clone(),
        }),
    ))
}


pub(super) fn build_checked_ih_post_call_consumers(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<CheckedIhPostCallConsumer>, CraneliftBackendError> {
    let mut result = Vec::new();
    for transport in &plan.checked_ih_environment_transports {
        let ContinuationEmissionOwner::Specialization(enclosing) = transport.destination_owner()
        else {
            continue;
        };
        let Some(context) =
            plan.continuation_context_for(enclosing, transport.destination_body_origin())?
        else {
            continue;
        };
        let mut demanded = Vec::new();
        for identity in plan
            .static_response_continuations
            .iter()
            .filter(|row| row.k_context == context.id())
            .map(|row| row.k_ret_identity)
        {
            if !demanded.contains(&identity) {
                demanded.push(identity);
            }
        }
        let demanded = match demanded.as_slice() {
            [] => continue,
            [identity] => *identity,
            _ => return Err(planner_error(
                "one checked-IH transport identity has disagreeing response-context Result demands",
            )),
        };
        // FORK 2 (Architect, evt_xew275ff898w): a shape this relation cannot model
        // yields NO ROW, never a refusal. The builder runs on every plan build, so
        // propagating here would give an observability derivation veto power over
        // programs that compile on `main` -- which is what red-lit px8f_write_partition.
        #[cfg(feature = "px8-ds-test-support")]
        super::continuations::note_transport_examined();
        let Some(actual) = continuation_call_selected_result_identity_opt(
            plan,
            transport.source_call_identity(),
        )?
        else {
            continue;
        };
        if actual == demanded {
            continue;
        }
        let detached_return_context = plan
            .detached_return_context_for(transport.source_call_identity())
            .cloned();
        let frame_origins = if let Some(context) = &detached_return_context {
            let mut frames = Vec::new();
            detached_post_call_consumer_frames(plan, context, &mut frames)?;
            if frames.is_empty() {
                return Err(planner_error(
                    "a detached checked-IH return context has no exact source consumer exit",
                ));
            }
            frames
        } else {
            let Some(frames) = checked_ih_post_call_consumer_frames(plan, transport)? else {
                continue;
            };
            frames
        };
        let Some(derived_steps) = derive_checked_ih_post_call_consumer_chain(
            plan,
            transport.source_call_identity(),
            &frame_origins,
            actual,
            demanded,
        )?
        else {
            continue;
        };
        if derived_steps.is_empty() {
            return Err(planner_error(
                "a mismatched checked-IH transport Result produced no source return steps",
            ));
        }
        let exact_transport = match detached_return_context.as_ref() {
            Some(context) => CheckedIhPostCallTransport::Required(
                pair_detached_required_consumer(plan, transport, context)?,
            ),
            None => CheckedIhPostCallTransport::Ordinary(transport.clone()),
        };
        let (consumers, selected_case_exits) = if detached_return_context.is_some() {
            (Vec::new(), derived_steps)
        } else {
            (derived_steps, Vec::new())
        };
        result.push(CheckedIhPostCallConsumer {
            transport: exact_transport,
            actual_result_identity: actual,
            demanded_result_identity: demanded,
            consumers,
            selected_case_exits,
            detached_return_context,
            required_consumer_edge: None,
        });
    }
    result.sort_by(|left, right| left.transport().cmp(right.transport()));
    attach_required_consumer_incoming_edges(plan, &mut result)?;
    Ok(result)
}

pub(super) fn publish_checked_ih_post_call_consumers(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<CheckedIhPostCallConsumer>, CraneliftBackendError> {
    let mut rows = build_checked_ih_post_call_consumers(plan)?;
    // Count this invocation, then emit the record from a call site the run
    // REACHES -- not from a destructor. The counters are monotone process-global
    // atomics, so this emission already carries the running total; there is nothing
    // an end-of-run hook would add but one more sample.
    #[cfg(feature = "px8-ds-test-support")]
    {
        super::continuations::note_publisher_invocation();
        super::continuations::emit_unmodelled_post_call_record();
    }
    #[cfg(feature = "px8-ds-test-support")]
    if d5b_hs17_post_call_consumer_mutation() == D5bHs17PostCallConsumerMutation::TransplantConsumer
    {
        let mut pair = None;
        'outer: for left in 0..rows.len() {
            for right in left + 1..rows.len() {
                if rows[left].transport().source_call_identity()
                    != rows[right].transport().source_call_identity()
                    && rows[left].consumers.len() == rows[right].consumers.len()
                {
                    pair = Some((left, right));
                    break 'outer;
                }
            }
        }
        let (left, right) = pair.ok_or_else(|| {
            planner_error(
                "the HS15 transplant control found no distinct equal-length consumer pair",
            )
        })?;
        let left_consumers = rows[left].consumers.clone();
        rows[left].consumers = rows[right].consumers.clone();
        rows[right].consumers = left_consumers;
        record_d5b_hs17_post_call_consumer_application();
    }
    Ok(rows)
}

/// The post-call consumer relation's MUTATION ORACLE, whose driver is absent.
///
/// **What this detects, stated because a required check with no stated failure
/// mode is satisfied by a tautology.** The stored value comes from
/// `publish_checked_ih_post_call_consumers`, which is
/// `build_checked_ih_post_call_consumers` PLUS the `px8-ds-test-support`
/// transplant. This compares the stored value against a fresh `build_`, so it
/// refuses exactly when that transplant has moved a consumer, and is inert by
/// design when it has not. It is not `f(x) == f(x)`.
///
/// **Its driver is absent on this tree, so it cannot fire.** Firing needs the feature on, the mutation set
/// to `TransplantConsumer`, and at least two rows to transplant between. The
/// population is empty at every plan build here (measured: 0 non-empty of 17
/// publisher invocations), and the harness that would set the mutation has no
/// caller on this tree. So it currently detects post-publication drift and
/// nothing else, and is additionally vacuous while the population is empty.
pub(super) fn validate_checked_ih_post_call_consumers(
    plan: &StaticTransitionPlan<'_>,
    consumers: &[CheckedIhPostCallConsumer],
) -> Result<(), CraneliftBackendError> {
    if consumers != build_checked_ih_post_call_consumers(plan)? {
        return Err(planner_error(
            "the checked-IH post-call consumer relation is not its exact identity-and-endpoint derivation",
        ));
    }
    Ok(())
}


fn attach_required_consumer_incoming_edges(
    _plan: &StaticTransitionPlan<'_>,
    rows: &mut [CheckedIhPostCallConsumer],
) -> Result<(), CraneliftBackendError> {
    for row in rows {
        let Some(required_call) = row.required_consumer() else {
            continue;
        };
        let consumer_occurrence = required_call.destination().consumer_occurrence();
        let matches = row
            .selected_case_exits
            .iter()
            .enumerate()
            .filter(|(_, step)| {
                (
                    step.occurrence().eliminator_origin(),
                    step.checked_frame_id(),
                ) == consumer_occurrence
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let incoming_consumer_edge_index = match matches.as_slice() {
            [index] => *index,
            [] => {
                return Err(planner_error(
                    "a self-defining required consumer has no exact incoming edge",
                ))
            }
            _ => {
                return Err(planner_error(
                    "a self-defining required consumer has more than one exact incoming edge",
                ))
            }
        };
        row.required_consumer_edge = Some(
            RequiredConsumerIncomingEdgeSelection::SelfDefining {
                incoming_consumer_edge_index,
            },
        );
    }
    Ok(())
}

fn detached_post_call_consumer_frames(
    plan: &StaticTransitionPlan<'_>,
    context: &SourceReturnContextTemplate,
    frames: &mut Vec<StaticOriginId>,
) -> Result<(), CraneliftBackendError> {
    for step in context.steps().iter().rev() {
        if matches!(
            step.role(),
            SourceReturnContextRole::ComputationalMatchCase(_)
        ) {
            frames.push(step.parent_origin());
        }
    }
    if let Some(boundary) = context.worker_return() {
        let selecting = boundary.selecting_call();
        let target = plan
            .continuation_specializations
            .get(selecting.target().0 as usize)
            .ok_or_else(|| {
                planner_error(
                    "a detached return context names an uninstalled selecting specialization",
                )
            })?;
        if target.key.worker.body_origin != context.root_origin()
            || target.key.producer_construct_origin != boundary.caller_context().result_origin()
        {
            return Err(planner_error(
                "a detached return context does not join its worker root to the selecting call's exact caller result position",
            ));
        }
        frames.push(target.key.continuation_origin);
        if let Some(source) = target.key.consuming_occurrence {
            if frames.last().copied() != Some(source.eliminator_origin()) {
                frames.push(source.eliminator_origin());
            }
            if let Some(RequiredConsumerProjection::DirectOuter { required, .. }) =
                plan.required_consumer_projections.get(selecting)
            {
                if frames.last().copied() != Some(required.eliminator_origin()) {
                    frames.push(required.eliminator_origin());
                }
            }
        }
        detached_post_call_consumer_frames(plan, boundary.caller_context(), frames)?;
    }
    Ok(())
}
