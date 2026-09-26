//! Read-only admission for a pending recursive child selected by a source Match.
//!
//! Admission is resolved before emission. A source Match with differing
//! declared pending body units and no admission is a planner invariant error,
//! not `NotApplicable`. A validated response owner guarantees Ret on success;
//! no recursive call package is issued for that selected return.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    occurrences::{occurrence_authority, occurrence_subtree_contains}, planner_error,
    CheckedCaseBinderLayout, CheckedCaseBinderRole, ContinuationEmissionOwner,
    CraneliftBackendError, PredeclaredFunctionId,
    ResolvedContinuationCallee, ResponseDisposition, RuntimeExpr, StaticOriginId,
    StaticTransitionPlan,
};
use crate::cranelift_backend::lowering::core::agreeing_recursive_body_unit;
use crate::{CheckedComputationalIHInvocationKind, OrientedSubcontinuationPlanV1};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PendingCandidate {
    arm: usize,
    /// The actual selected leaf, possibly beneath nested source Match arms.
    construct: StaticOriginId,
    body: StaticOriginId,
    callee: ResolvedContinuationCallee,
}

/// A transport decision for one visited static origin. Local means evaluating
/// within the same function without moving a companion across a binding/edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PendingRouteKind {
    Local,
    SelectedArm,
    Join,
    Binding,
    CarriedResidual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PendingRouteVisit {
    pub(super) origin: StaticOriginId,
    pub(super) kind: PendingRouteKind,
}

// One gate count per execution path, never a sum over the whole match. Every
// marked pending call is its consuming gate; an unmarked occurrence of its IH
// binder is refused separately, at the raw Var leaf.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PendingPathCounts {
    consuming_gates: u8,
}

impl PendingPathCounts {
    const START: Self = Self { consuming_gates: 0 };

    fn consume(self) -> Result<Self, PendingRefusal> {
        if self.consuming_gates != 0 {
            return Err(PendingRefusal::NotLinearOrMustReach);
        }
        Ok(Self { consuming_gates: 1 })
    }
}

struct PendingRouteEvidence<'a> {
    kinds: BTreeMap<StaticOriginId, PendingRouteKind>,
    gate_binder_pairs: BTreeMap<StaticOriginId, (u32, u64)>,
    selected_slot: u64,
    slot_for_call: &'a dyn Fn(u64) -> Option<u64>,
}

impl<'a> PendingRouteEvidence<'a> {
    fn new(selected_slot: u64, slot_for_call: &'a dyn Fn(u64) -> Option<u64>) -> Self {
        Self {
            kinds: BTreeMap::new(),
            gate_binder_pairs: BTreeMap::new(),
            selected_slot,
            slot_for_call,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PendingGateBinderPair {
    pub(super) origin: StaticOriginId,
    pub(super) walker_index: u32,
    pub(super) morphism_index: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PendingCallRoute {
    pub(in crate::cranelift_backend) defining_function: PredeclaredFunctionId,
    /// The set of static origins the checked route walker actually visited.
    /// The emitter must assert membership before carrying a companion there.
    pub(super) visited: Vec<PendingRouteVisit>,
    /// Alternative static call sites are permitted only where each reachable
    /// execution path selects exactly one of them.
    pub(super) gates: Vec<StaticOriginId>,
    /// Observed, not an admission gate: the runtime environment's raw IH slot
    /// and erasure's independently minted coordinate may disagree. Marker
    /// consumption is keyed by its checked call template, not either index.
    pub(super) gate_binder_pairs: Vec<PendingGateBinderPair>,
}

/// Response work reachable from a selected leaf, including the worker body
/// named by its recursive unit. `None` is an actual no-response disposition,
/// never an unknown owner represented by a missing row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct SelectedPendingLeafResponse {
    leaf: StaticOriginId,
    vis: StaticOriginId,
    disposition: Option<ResponseDisposition>,
    owner: Option<ContinuationEmissionOwner>,
    /// Joins in a Specialized operation lowered in a different emission.
    relocated_joins: BTreeSet<StaticOriginId>,
    /// The explicit lowering environments at a Deferred drive, not its frame.
    effect_free: BTreeSet<u32>,
    effect_fields: usize,
    k_free: BTreeSet<u32>,
    k_fields: usize,
}

impl SelectedPendingLeafResponse {
    pub(in crate::cranelift_backend) fn disposition(&self) -> Option<ResponseDisposition> {
        self.disposition
    }

    pub(in crate::cranelift_backend) fn owner(&self) -> Option<ContinuationEmissionOwner> {
        self.owner
    }
}

/// Admission asks "validated owner or refused?" and constructs this witness only from
/// planner facts. Emission asks "is the invariant intact?" and consumes these
/// facts, rather than independently re-deriving them from its current context.
/// No condition dependent on the emission context belongs in this constructor.
///
/// The response facts describe the actual emission paths: a Specialized
/// response may lawfully relocate to a different owner when its operation
/// carries no unaccounted joins; a Deferred drive lowers against its selected
/// operation fields and its reconstructed K environment, not the owner's
/// entire frame. An admitted route cannot rely on a field absent at those calls.
///
/// A successful response-owner call validates Ret before publishing the
/// returned word. The route and response facts retain the admission refusals;
/// neither is a runtime package or an instruction to issue a second call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct SelectedPendingRouteWitness {
    producer: StaticOriginId,
    candidates: Vec<PendingCandidate>,
    route: PendingCallRoute,
    responses: Vec<SelectedPendingLeafResponse>,
}

impl SelectedPendingRouteWitness {
    fn new(
        producer: StaticOriginId,
        candidates: Vec<PendingCandidate>,
        route: PendingCallRoute,
        responses: Vec<SelectedPendingLeafResponse>,
    ) -> Self {
        Self {
            producer,
            candidates,
            route,
            responses,
        }
    }

    pub(in crate::cranelift_backend) fn responses(&self) -> &[SelectedPendingLeafResponse] {
        &self.responses
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PendingRefusal {
    RouteLeavesDefiningFunction,
    NotLinearOrMustReach,
    UnsupportedRouteEdge,
    MissingDeclaredMembers,
    RelocatedWorkMissingLoweringBinding,
    SelectedPendingLeafRelocatesUnaccountedJoins,
    PendingResultNotValidatedByResponseOwner,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum PendingCallAdmission {
    ValidatedResponseOwner(SelectedPendingRouteWitness),
    Refused(PendingRefusal),
    NotApplicable,
}

impl<'src> StaticTransitionPlan<'src> {
    /// Resolve the checked template authority only when both planes are live,
    /// before any emitter or ticket can consume this planner-only decision.
    pub(in crate::cranelift_backend) fn install_selected_pending_calls(
        &mut self,
        oriented: Option<&OrientedSubcontinuationPlanV1>,
    ) -> Result<(), CraneliftBackendError> {
        let admissions = plan_selected_pending_calls(self, oriented)?;
        self.selected_pending_calls = admissions;
        #[cfg(feature = "px8-ds-test-support")]
        record_selected_pending_call_admissions(self);
        Ok(())
    }
}

/// The only producer-level admission writer. It starts from validated
/// continuation-unit declarations rather than searching for a CLIF arm or
/// interpreting a `producer_alternative` shared by both source arms.
fn plan_selected_pending_calls(
    plan: &StaticTransitionPlan<'_>,
    oriented: Option<&OrientedSubcontinuationPlanV1>,
) -> Result<BTreeMap<StaticOriginId, PendingCallAdmission>, CraneliftBackendError> {
    let units = plan.continuation_units()?;
    let contexts = plan.continuation_contexts()?;
    let mut by_producer = BTreeMap::new();
    for occurrence in plan.source_occurrences.iter() {
        let Some(occurrence) = occurrence else {
            continue;
        };
        let RuntimeExpr::ComputationalMatch {
            scrutinee, cases, ..
        } = occurrence.expr
        else {
            continue;
        };
        let RuntimeExpr::Match {
            cases: source_arms, ..
        } = scrutinee.as_ref()
        else {
            continue;
        };
        let eliminator_origin = occurrence.static_origin;
        let producer = plan.semantic.child_origin(eliminator_origin, 0)?;
        let recursive_cases: Vec<_> = cases
            .iter()
            .filter(|case| !case.recursive_positions.is_empty())
            .collect();
        if recursive_cases.len() != 1 || recursive_cases[0].recursive_positions.len() != 1 {
            by_producer.insert(producer, PendingCallAdmission::NotApplicable);
            continue;
        }
        let selected_case = recursive_cases[0];
        let position = selected_case.recursive_positions[0];
        let mut candidates = Vec::with_capacity(source_arms.len());
        let mut declared_bodies = Vec::with_capacity(source_arms.len());
        let mut eligible = source_arms.len() >= 2;
        let mut candidate_refusal = None;
        for (arm, case) in source_arms.iter().enumerate() {
            let arm_origin = plan.semantic.child_origin(producer, arm + 1)?;
            let mut leaves = Vec::new();
            if !pending_source_leaves(plan, arm_origin, &case.body, &mut leaves)? {
                eligible = false;
                break;
            }
            for (construct_origin, leaf) in leaves {
            let RuntimeExpr::Construct { constructor, args } = leaf else {
                unreachable!("the leaf walker returns only constructors")
            };
            // L2's selected-constructor bucket: an arm constructing some
            // other value cannot produce the recursive child in this case.
            if constructor != &selected_case.constructor {
                continue;
            }
            if !matches!(args.get(position), Some(RuntimeExpr::LexicalClosure { .. })) {
                candidate_refusal = Some(PendingRefusal::UnsupportedRouteEdge);
                break;
            }
            let mut matching = units.iter().filter(|unit| {
                unit.producer_result_origin() == producer
                    && unit.producer_construct_origin() == construct_origin
                    && unit.recursive_position() as usize == position
            });
            let Some(unit) = matching.next() else {
                candidate_refusal = Some(PendingRefusal::MissingDeclaredMembers);
                break;
            };
            if matching.next().is_some() {
                return Err(planner_error(
                    "one pending source arm declares two recursive units at one position",
                ));
            }
            let mut owning_contexts = contexts
                .iter()
                .filter(|context| context.worker_body_origin() == unit.worker_body_origin());
            let Some(context) = owning_contexts.next() else {
                candidate_refusal = Some(PendingRefusal::MissingDeclaredMembers);
                break;
            };
            if owning_contexts.next().is_some() {
                // The ordinary capture gate also refuses ambiguous contexts.
                // Treat this as an excluded arm, not a planner-wide ICE: the
                // pending preflight must agree with that existing refusal.
                candidate_refusal = Some(PendingRefusal::MissingDeclaredMembers);
                break;
            }
            if unit.worker_captures().iter().any(|capture| !matches!(
                capture.source(), super::ContinuationWorkerCaptureSource::Lexical(_)
            )) || context.captures()?.is_empty() && unit.worker_capture_count() == 0 {
                candidate_refusal = Some(PendingRefusal::MissingDeclaredMembers);
                break;
            }
            let identity = plan.continuation_call_binding_for(
                unit.producer_construct_origin(), unit.continuation_origin(),
                unit.producer_alternative(), unit.recursive_position(),
            )?.ok_or_else(|| planner_error(
                "a selected pending leaf has no typed direct continuation-call binding",
            ))?;
            let callee = plan.resolved_continuation_callee(&identity)?;
            declared_bodies.push(unit.worker_body_origin());
            candidates.push(PendingCandidate {
                arm,
                construct: construct_origin,
                body: unit.worker_body_origin(),
                callee,
            });
            }
            if candidate_refusal.is_some() {
                break;
            }
        }
        if let Some(reason) = candidate_refusal {
            by_producer.insert(producer, PendingCallAdmission::Refused(reason));
            continue;
        }
        if !eligible || candidates.len() < 2 {
            by_producer.insert(producer, PendingCallAdmission::NotApplicable);
            continue;
        }
        // The existing L2 disagreement predicate remains the authority. Its
        // error identifies the *population*, not an error to propagate from
        // this read-only preflight. L2 still runs, unchanged, at lowering.
        if let Some(admission) = agreed_unit_nonpackage(declared_bodies) {
            by_producer.insert(producer, admission);
            continue;
        }
        let owner = occurrence_authority(plan, producer)?.owner;
        // The checked case layout is the authority for the IH's initial
        // runtime de Bruijn slot. The single recursive position has slot 0;
        // nested Let/Match binders shift it as the walker descends.
        let pending_ih_index = match CheckedCaseBinderLayout::for_case(selected_case)?.role_at(0) {
            CheckedCaseBinderRole::InductionHypothesis { recursive_position }
                if recursive_position as usize == position =>
            {
                0_u32
            }
            _ => {
                by_producer.insert(
                    producer,
                    PendingCallAdmission::Refused(PendingRefusal::UnsupportedRouteEdge),
                );
                continue;
            }
        };
        // The consumer proof below is deliberately conservative. Any edge we
        // cannot identify as a same-function F1-F4 route refuses before the
        // emitter can ever consume this plan.
        let case_origin = plan.semantic.child_origin(
            eliminator_origin,
            1 + cases
                .iter()
                .position(|case| std::ptr::eq(case, selected_case))
                .ok_or_else(|| planner_error("selected recursive case disappeared"))?,
        )?;
        // The slot list is the checked case's authority, in recursive-position
        // order. A marker's call-template id is not itself a slot id: resolve
        // its slot through the checked oriented plan, as native lowering does.
        let selected_slot = match plan.planned_occurrence_expr(case_origin)? {
            RuntimeExpr::CheckedComputationalIHSlots {
                slot_template_ids, ..
            } if slot_template_ids.len() == selected_case.recursive_positions.len() => {
                slot_template_ids[0]
            }
            _ => {
                by_producer.insert(
                    producer,
                    PendingCallAdmission::Refused(PendingRefusal::MissingDeclaredMembers),
                );
                continue;
            }
        };
        let slot_for_call = |call_id| {
            oriented
                .and_then(|plan| plan.computational_ih_call(call_id))
                .map(|template| template.slot_template_id)
        };
        let result = prove_route(
            plan,
            producer,
            case_origin,
            owner,
            pending_ih_index,
            selected_slot,
            &slot_for_call,
        );
        let admission = match result {
            Ok((paths, gates, visited, gate_binder_pairs)) => {
                let route = PendingCallRoute {
                    defining_function: owner,
                    visited,
                    gates,
                    gate_binder_pairs,
                };
                match selected_leaf_response_witness(plan, &candidates, owner)? {
                    Ok(responses) => admit_validated_owner(
                        producer, candidates, route, responses, paths,
                    ),
                    Err(reason) => PendingCallAdmission::Refused(reason),
                }
            }
            Err(reason) => PendingCallAdmission::Refused(reason),
        };
        if by_producer.insert(producer, admission).is_some() {
            return Err(planner_error(
                "two computational eliminators claim one pending Match",
            ));
        }
    }
    Ok(by_producer)
}

impl StaticTransitionPlan<'_> {
    /// The emitter rechecks the typed direct callee at the selected frame.
    /// This is the same classifier the admission consumed, not an inference
    /// from the carrier word or from a nominal specialization target.
    pub(in crate::cranelift_backend) fn pending_result_validated_owner(
        &self,
        frame: StaticOriginId,
    ) -> Result<bool, CraneliftBackendError> {
        let producer = self.semantic.child_origin(frame, 0)?;
        match self.selected_pending_calls.get(&producer) {
            Some(PendingCallAdmission::ValidatedResponseOwner(witness)) => {
                if witness.producer != producer || witness.candidates.is_empty() {
                    return Err(planner_error("a pending owner admission lost its selected leaves"));
                }
                let units = self.continuation_units()?;
                for candidate in &witness.candidates {
                    let unit = units.iter()
                        .find(|unit| unit.producer_construct_origin() == candidate.construct
                            && unit.worker_body_origin() == candidate.body)
                        .ok_or_else(|| planner_error("a pending owner leaf lost its continuation unit"))?;
                    let identity = self.continuation_call_binding_for(
                        unit.producer_construct_origin(), unit.continuation_origin(),
                        unit.producer_alternative(), unit.recursive_position(),
                    )?.ok_or_else(|| planner_error("a pending owner leaf lost its direct call identity"))?;
                    let actual = self.resolved_continuation_callee(&identity)?;
                    if actual != candidate.callee || !matches!(actual, ResolvedContinuationCallee::StaticResponseOwner(_)) {
                        return Err(planner_error("an emitted pending owner leaf has a different direct callee than admission"));
                    }
                }
                Ok(true)
            }
            Some(PendingCallAdmission::Refused(_)) => Err(planner_error(
                "a refused pending call was delivered to an emitting frame",
            )),
            Some(PendingCallAdmission::NotApplicable) | None => Ok(false),
        }
    }

    /// The owner-fed population of one emitted Match instance, keyed by its
    /// emission owner as well as source origin. Only its own return edges may
    /// justify a Ret-only trap and Vis-subtree disposition in that function.
    pub(in crate::cranelift_backend) fn owner_fed_match_population(
        &self,
        origin: StaticOriginId,
        emission_owner: ContinuationEmissionOwner,
    ) -> Result<Option<usize>, CraneliftBackendError> {
        if !self.pending_result_validated_owner(origin)? {
            return Ok(None);
        }
        let producer = self.semantic.child_origin(origin, 0)?;
        let Some(PendingCallAdmission::ValidatedResponseOwner(witness)) =
            self.selected_pending_calls.get(&producer)
        else {
            return Err(planner_error("a validated pending owner lost its witness"));
        };
        let units = self.continuation_units()?;
        let mut incoming = Vec::new();
        for call in self.continuation_calls()? {
            if call.continuation_origin() != origin
                || call.emission_owner() != emission_owner
            {
                continue;
            }
            let unit = units.iter().find(|unit| unit.id() == call.target())
                .ok_or_else(|| planner_error("a re-entering call lost its unit"))?;
            incoming.push((unit.producer_construct_origin(), unit.worker_body_origin()));
        }
        if !owner_reentries_exclusive(
            witness.candidates.iter().map(|candidate| (candidate.construct, candidate.body)),
            incoming,
        )? {
            return Ok(None);
        }
        let occurrence = self.source_occurrences.get(origin.0 as usize)
            .and_then(Option::as_ref)
            .ok_or_else(|| planner_error("an owner-fed Match has no planned occurrence"))?;
        let RuntimeExpr::ComputationalMatch { cases, .. } = occurrence.expr else {
            return Err(planner_error("an owner-fed Match is not computational"));
        };
        let mut returns = cases.iter().enumerate()
            .filter(|(_, case)| case.constructor.ends_with("::ITree::Ret"));
        let (index, ret) = returns.next().ok_or_else(|| {
            planner_error("an owner-fed Match has no ITree::Ret case")
        })?;
        if ret.argument_binders != 1 || returns.next().is_some() {
            return Err(planner_error("an owner-fed Match has no unique unary ITree::Ret case"));
        }
        Ok(Some(index))
    }

    /// Test observation for a specialization whose own calls re-enter this
    /// Match; other specializations may lower the same selected Vis body but
    /// are not members of the pair proving S1's emission-local partition.
    #[cfg(feature = "px8-ds-test-support")]
    pub(in crate::cranelift_backend) fn pending_match_has_reentering_calls(
        &self,
        origin: StaticOriginId,
        owner: ContinuationEmissionOwner,
    ) -> Result<bool, CraneliftBackendError> {
        Ok(self.continuation_calls()?.iter().any(|call| {
            call.continuation_origin() == origin && call.emission_owner() == owner
        }))
    }

    /// A response drive may consult an admitted pending route's immutable owner
    /// evidence, without recomputing the source-to-emission relation locally.
    /// An ordinary response with no pending witness retains its existing plan.
    pub(in crate::cranelift_backend) fn selected_pending_response_at_vis(
        &self,
        vis: StaticOriginId,
    ) -> Result<Option<&SelectedPendingLeafResponse>, CraneliftBackendError> {
        let mut found = None;
        for admission in self.selected_pending_calls.values() {
            let PendingCallAdmission::ValidatedResponseOwner(witness) = admission else {
                continue;
            };
            for response in witness.responses().iter().filter(|row| row.vis == vis) {
                if found.is_some_and(|previous| previous != response) {
                    return Err(planner_error(
                        "two admitted pending leaves disagree on a response emission",
                    ));
                }
                found = Some(response);
            }
        }
        Ok(found)
    }

}

/// Classify the return edges in ONE emission against the planner's candidate
/// coordinates. Mixed edges refuse; zero owner edges keep ordinary lowering.
fn owner_reentries_exclusive(
    candidates: impl IntoIterator<Item = (StaticOriginId, StaticOriginId)>,
    incoming: impl IntoIterator<Item = (StaticOriginId, StaticOriginId)>,
) -> Result<bool, CraneliftBackendError> {
    let admitted = candidates.into_iter().collect::<BTreeSet<_>>();
    let (mut owned, mut ordinary) = (0usize, 0usize);
    for call in incoming {
        if admitted.contains(&call) {
            owned += 1;
        } else {
            ordinary += 1;
        }
    }
    match (owned, ordinary) {
        (0, _) => Ok(false),
        (_, 0) => Ok(true),
        _ => Err(planner_error(
            "owner-fed and ordinary returns re-enter one Match in one emission owner",
        )),
    }
}

/// Leaf-only source producer census: a nested Match chooses exactly one
/// constructor, and no inference from its outer arm number chooses the leaf.
/// Unsupported edges refuse admission rather than guessing a unit body.
fn pending_source_leaves<'a>(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    expr: &'a RuntimeExpr,
    leaves: &mut Vec<(StaticOriginId, &'a RuntimeExpr)>,
) -> Result<bool, CraneliftBackendError> {
    match expr {
        RuntimeExpr::Construct { .. } => {
            leaves.push((origin, expr));
            Ok(true)
        }
        RuntimeExpr::Match { cases, .. } => {
            for (index, case) in cases.iter().enumerate() {
                let child = plan.semantic.child_origin(origin, index + 1)?;
                if !pending_source_leaves(plan, child, &case.body, leaves)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn agreed_unit_nonpackage(
    declared_bodies: impl IntoIterator<Item = StaticOriginId>,
) -> Option<PendingCallAdmission> {
    agreeing_recursive_body_unit(declared_bodies)
        .is_ok()
        .then_some(PendingCallAdmission::NotApplicable)
}

fn marked_pending_read(
    kind: CheckedComputationalIHInvocationKind,
    body: &RuntimeExpr,
    selected_slot: u64,
    call_slot: Option<u64>,
) -> bool {
    kind == CheckedComputationalIHInvocationKind::CheckedHostVisContinuation
        && matches!(body, RuntimeExpr::Call { args, .. } if args.len() == 1)
        && call_slot == Some(selected_slot)
}

fn admit_validated_owner(
    producer: StaticOriginId,
    candidates: Vec<PendingCandidate>,
    route: PendingCallRoute,
    responses: Vec<SelectedPendingLeafResponse>,
    paths: Vec<PendingPathCounts>,
) -> PendingCallAdmission {
    // Keep every route refusal: an unreachable consuming gate does not justify
    // broadening admission or skipping the checked IH identity proof.
    if paths.is_empty()
        || route.gates.is_empty()
        || paths.iter().any(|path| path.consuming_gates != 1)
    {
        PendingCallAdmission::Refused(PendingRefusal::NotLinearOrMustReach)
    } else if candidates.iter().any(|candidate| matches!(
        candidate.callee, ResolvedContinuationCallee::OrdinarySpecialization(_)
    )) {
        PendingCallAdmission::Refused(PendingRefusal::PendingResultNotValidatedByResponseOwner)
    } else {
        PendingCallAdmission::ValidatedResponseOwner(SelectedPendingRouteWitness::new(
            producer, candidates, route, responses,
        ))
    }
}

/// Every Vis in a leaf's constructor and declared worker subtree is classified
/// before the producer is admitted. This is a whole-producer decision, not a
/// per-leaf fallback. (E) reads the explicit lowering environments constructed
/// by the response owner, not its wider frame; (J) accounts for the joins a
/// Specialized operation skips when its emitter returns a placeholder.
fn selected_leaf_response_witness(
    plan: &StaticTransitionPlan<'_>,
    candidates: &[PendingCandidate],
    defining_function: PredeclaredFunctionId,
) -> Result<Result<Vec<SelectedPendingLeafResponse>, PendingRefusal>, CraneliftBackendError> {
    let owner = ContinuationEmissionOwner::Predeclared(defining_function);
    let mut responses = Vec::new();
    let mut environment_closed = true;
    let mut unaccounted_joins = false;
    for candidate in candidates {
        let mut seen = BTreeSet::new();
        for occurrence in plan.source_occurrences.iter().flatten() {
            let RuntimeExpr::Construct { constructor, args } = occurrence.expr else {
                continue;
            };
            if !constructor.as_str().ends_with("::ITree::Vis") || args.len() != 2 {
                continue;
            }
            let vis = occurrence.static_origin;
            if !(occurrence_subtree_contains(plan, candidate.construct, vis)?
                || occurrence_subtree_contains(plan, candidate.body, vis)?)
                || !seen.insert(vis)
            {
                continue;
            }
            let mut specialized = plan.static_response_continuations.iter()
                .filter(|row| row.vis_origin() == vis);
            let specialized_row = specialized.next();
            if specialized.next().is_some() {
                return Err(planner_error("a selected pending Vis has two specialized responses"));
            }
            let deferred = plan.deferred_response_at_vis(vis)?;
            if specialized_row.is_some() && deferred.is_some() {
                return Err(planner_error("a selected pending Vis is both specialized and Deferred"));
            }
            let (disposition, response_owner) = if let Some(row) = specialized_row {
                (Some(ResponseDisposition::Specialized), Some(row.base_owner()))
            } else if let Some(row) = deferred.as_ref() {
                (Some(ResponseDisposition::Deferred), plan.deferred_response_handler_owner(row)?)
            } else {
                (None, None)
            };
            let mut relocated_joins = BTreeSet::new();
            let mut effect_free = BTreeSet::new();
            let mut effect_fields = 0;
            let mut k_free = BTreeSet::new();
            let mut k_fields = 0;
            if let Some(row) = specialized_row {
                // A Specialized response may lawfully have a different owner:
                // px7l does, and emits natively. Its operation arguments use
                // exactly the frame environment already declared by this row.
                for input in row.effect_environment() {
                    if let super::StaticResponseEffectInput::OperationArgument { origin, environment } = input {
                        let mut free = BTreeSet::new();
                        pending_free_indices(plan.planned_occurrence_expr(*origin)?, 0, &mut free)?;
                        environment_closed &= free.iter().all(|index| (*index as usize) < environment.len());
                    }
                }
                if response_owner != Some(owner) {
                    let operation = plan.semantic.child_origin(vis, 0)?;
                    relocated_joins = plan.source_join_origins_in_owner_subtree(operation)?;
                    unaccounted_joins |= !relocated_joins.is_empty();
                }
            }
            // P1 Deferred rows have no handler-owned drive and fall through to
            // ordinary lowering. E applies only where an actual selected owner
            // synthesizes the field and K environments.
            if let Some(row) = deferred.as_ref().filter(|_| response_owner.is_some()) {
                let RuntimeExpr::Construct { args, .. } = plan.planned_occurrence_expr(vis)? else {
                    return Ok(Err(PendingRefusal::UnsupportedRouteEdge));
                };
                let [RuntimeExpr::Construct { args: operation_fields, .. },
                    RuntimeExpr::LexicalClosure { captures, params, .. }] = args.as_slice() else {
                    return Ok(Err(PendingRefusal::UnsupportedRouteEdge));
                };
                let [RuntimeExpr::Construct { args: selected_fields, .. }] = operation_fields.as_slice() else {
                    return Ok(Err(PendingRefusal::UnsupportedRouteEdge));
                };
                if params.len() != 1 {
                    return Ok(Err(PendingRefusal::UnsupportedRouteEdge));
                }
                effect_fields = selected_fields.len();
                let RuntimeExpr::Effect { args, capability, .. } =
                    plan.planned_occurrence_expr(row.effect_origin())? else {
                    return Ok(Err(PendingRefusal::UnsupportedRouteEdge));
                };
                for arg in args {
                    pending_free_indices(arg, 0, &mut effect_free)?;
                }
                if let Some(capability) = capability {
                    pending_free_indices(&capability.value, 0, &mut effect_free)?;
                }
                environment_closed &= effect_free.iter().all(|index| (*index as usize) < effect_fields);
                let Some(k_body) = plan.deferred_response_k_body(row)? else {
                    return Ok(Err(PendingRefusal::UnsupportedRouteEdge));
                };
                pending_free_indices(plan.planned_occurrence_expr(k_body)?, 0, &mut k_free)?;
                k_fields = captures.len() + 1;
                environment_closed &= k_free.iter().all(|index| (*index as usize) < k_fields);
                // This row's handler owner is carried in the witness. The
                // drive compares it with the owner of its actual emission;
                // source-subtree membership is not a proxy for that emission.
            }
            responses.push(SelectedPendingLeafResponse {
                leaf: candidate.construct, vis, disposition, owner: response_owner,
                relocated_joins, effect_free, effect_fields, k_free, k_fields,
            });
        }
    }
    // An E failure must be visible before J for the same producer. J-a's
    // accounting is deferred: no presently admitted row has nonempty R. Its
    // complement refuses rather than making a selected join look unselected.
    if !environment_closed {
        return Ok(Err(PendingRefusal::RelocatedWorkMissingLoweringBinding));
    }
    if unaccounted_joins {
        return Ok(Err(PendingRefusal::SelectedPendingLeafRelocatesUnaccountedJoins));
    }
    Ok(Ok(responses))
}

/// Local binder-aware scan of the inputs to the *explicit* lowering environments
/// above. A nested binder is removed before comparing an external index with
/// the caller's actual run. Lexical closure bodies are separate owner emissions;
/// only their capture expressions evaluate at this call.
fn pending_free_indices(
    expr: &RuntimeExpr,
    depth: u32,
    free: &mut BTreeSet<u32>,
) -> Result<(), CraneliftBackendError> {
    let visit = |expr, depth, free: &mut BTreeSet<u32>| pending_free_indices(expr, depth, free);
    let increase = |depth: u32, by: usize| -> Result<u32, CraneliftBackendError> {
        depth.checked_add(u32::try_from(by).map_err(|_| planner_error("pending binder count exceeds u32"))?)
            .ok_or_else(|| planner_error("pending binder depth exceeds u32"))
    };
    match expr {
        RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { body, .. } => visit(body, depth, free)?,
        RuntimeExpr::Value(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_)
        | RuntimeExpr::Closure { .. } => {},
        RuntimeExpr::Var(index) => {
            if *index >= depth {
                free.insert(index - depth);
            }
        }
        RuntimeExpr::Let { value, body } => {
            visit(value, depth, free)?;
            visit(body, increase(depth, 1)?, free)?;
        }
        RuntimeExpr::If { scrutinee, then_expr, else_expr } => {
            visit(scrutinee, depth, free)?;
            visit(then_expr, depth, free)?;
            visit(else_expr, depth, free)?;
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            for arg in args { visit(arg, depth, free)?; }
        }
        RuntimeExpr::Match { scrutinee, cases, .. } => {
            visit(scrutinee, depth, free)?;
            for case in cases { visit(&case.body, increase(depth, case.binders)?, free)?; }
        }
        RuntimeExpr::ComputationalMatch { scrutinee, cases, .. } => {
            visit(scrutinee, depth, free)?;
            for case in cases {
                let binders = case.argument_binders.checked_add(case.recursive_positions.len())
                    .ok_or_else(|| planner_error("pending computational binder count overflow"))?;
                visit(&case.body, increase(depth, binders)?, free)?;
            }
        }
        RuntimeExpr::Record { fields } => {
            for (_, field) in fields { visit(field, depth, free)?; }
        }
        RuntimeExpr::Project { record, .. } => visit(record, depth, free)?,
        RuntimeExpr::LexicalClosure { captures, .. } => {
            for capture in captures { visit(capture, depth, free)?; }
        }
        RuntimeExpr::Call { callee, args } => {
            visit(callee, depth, free)?;
            for arg in args { visit(arg, depth, free)?; }
        }
        RuntimeExpr::Effect { capability, args, .. } => {
            if let Some(capability) = capability { visit(&capability.value, depth, free)?; }
            for arg in args { visit(arg, depth, free)?; }
        }
    }
    Ok(())
}

fn require_defining_function(
    defining_function: PredeclaredFunctionId,
    observed_function: PredeclaredFunctionId,
) -> Result<(), PendingRefusal> {
    if observed_function == defining_function {
        Ok(())
    } else {
        Err(PendingRefusal::RouteLeavesDefiningFunction)
    }
}

/// The single exhaustive mapping from a walked runtime expression to the
/// transport required at that origin. A new expression variant cannot silently
/// receive a `Local` default; an unsupported route refuses before publication.
fn route_kind(expr: &RuntimeExpr) -> Result<PendingRouteKind, PendingRefusal> {
    use PendingRouteKind as Kind;
    match expr {
        RuntimeExpr::CheckedJoinSite { .. } => Ok(Kind::Join),
        RuntimeExpr::If { .. } | RuntimeExpr::Match { .. } => Ok(Kind::SelectedArm),
        RuntimeExpr::Var(_) | RuntimeExpr::Let { .. } | RuntimeExpr::Project { .. } => {
            Ok(Kind::Binding)
        }
        RuntimeExpr::CheckedComputationalIHInvocation { .. } => Ok(Kind::CarriedResidual),
        RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
        | RuntimeExpr::Value(_)
        | RuntimeExpr::PrimitiveCall { .. }
        | RuntimeExpr::Construct { .. }
        | RuntimeExpr::Record { .. }
        | RuntimeExpr::Effect { .. }
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_) => Ok(Kind::Local),
        RuntimeExpr::Call { .. }
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::LexicalClosure { .. } => Err(PendingRefusal::RouteLeavesDefiningFunction),
        RuntimeExpr::ComputationalMatch { .. } => Err(PendingRefusal::UnsupportedRouteEdge),
    }
}

fn prove_route(
    plan: &StaticTransitionPlan<'_>,
    producer: StaticOriginId,
    consumer: StaticOriginId,
    owner: PredeclaredFunctionId,
    pending_ih_index: u32,
    selected_slot: u64,
    slot_for_call: &dyn Fn(u64) -> Option<u64>,
) -> Result<
    (
        Vec<PendingPathCounts>,
        Vec<StaticOriginId>,
        Vec<PendingRouteVisit>,
        Vec<PendingGateBinderPair>,
    ),
    PendingRefusal,
> {
    let root = plan
        .root_static_origin()
        .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?;
    if occurrence_authority(plan, root)
        .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?
        .owner
        == owner
        || occurrence_authority(plan, producer)
            .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?
            .owner
            != owner
    {
        return Err(PendingRefusal::RouteLeavesDefiningFunction);
    }
    let mut gates = BTreeSet::new();
    let mut visited = PendingRouteEvidence::new(selected_slot, slot_for_call);
    // Counts are carried separately on each branch; a sum over the whole
    // case would accept two gates in one arm and none in the sibling arm.
    let exits = walk_to_gate(
        plan,
        consumer,
        owner,
        pending_ih_index,
        true,
        PendingPathCounts::START,
        &mut gates,
        &mut visited,
    )?;
    Ok((
        exits,
        gates.into_iter().collect(),
        visited
            .kinds
            .into_iter()
            .map(|(origin, kind)| PendingRouteVisit { origin, kind })
            .collect(),
        visited
            .gate_binder_pairs
            .into_iter()
            .map(
                |(origin, (walker_index, morphism_index))| PendingGateBinderPair {
                    origin,
                    walker_index,
                    morphism_index,
                },
            )
            .collect(),
    ))
}

/// Follow the checked case body, not a guessed arm-order-to-CLIF mapping.
/// An empty set of returned paths is an explicit trap/termination; any
/// ordinary exit without a gate refuses. The checked case binder layout gives
/// the pending IH's index, shifted through every binder this walk enters.
/// Calls, closure boundaries and owner changes refuse rather than inventing
/// an F5/F6 transport. The source occurrence tree bounds traversal.
fn shifted_pending_ih(index: u32, binders: usize) -> Result<u32, PendingRefusal> {
    let shift = u32::try_from(binders).map_err(|_| PendingRefusal::UnsupportedRouteEdge)?;
    index
        .checked_add(shift)
        .ok_or(PendingRefusal::UnsupportedRouteEdge)
}

fn walk_to_gate(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    owner: PredeclaredFunctionId,
    pending_ih_index: u32,
    deny_raw_ih: bool,
    path: PendingPathCounts,
    gates: &mut BTreeSet<StaticOriginId>,
    visited: &mut PendingRouteEvidence<'_>,
) -> Result<Vec<PendingPathCounts>, PendingRefusal> {
    require_defining_function(
        owner,
        occurrence_authority(plan, origin)
            .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?
            .owner,
    )?;
    let expr = plan
        .planned_occurrence_expr(origin)
        .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?;
    let kind = route_kind(expr)?;
    if visited
        .kinds
        .insert(origin, kind)
        .is_some_and(|old| old != kind)
    {
        return Err(PendingRefusal::UnsupportedRouteEdge);
    }
    let child = |position| {
        plan.semantic
            .child_origin(origin, position)
            .map_err(|_| PendingRefusal::UnsupportedRouteEdge)
    };
    match expr {
        RuntimeExpr::Trap(_) => Ok(Vec::new()),
        RuntimeExpr::Value(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. } => Ok(vec![path]),
        RuntimeExpr::Var(index) => {
            if deny_raw_ih && *index == pending_ih_index {
                Err(PendingRefusal::NotLinearOrMustReach)
            } else {
                Ok(vec![path])
            }
        }
        RuntimeExpr::CheckedComputationalIHInvocation {
            call_template_id,
            kind,
            binder_morphism,
            body,
            ..
        } => {
            if !marked_pending_read(
                *kind,
                body,
                visited.selected_slot,
                (visited.slot_for_call)(*call_template_id),
            ) {
                return Err(PendingRefusal::UnsupportedRouteEdge);
            }
            let RuntimeExpr::Call { args, .. } = body.as_ref() else {
                return Err(PendingRefusal::UnsupportedRouteEdge);
            };
            let gate = child(0)?;
            require_defining_function(
                owner,
                occurrence_authority(plan, gate)
                    .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?
                    .owner,
            )?;
            // The marked callee is this authorized pending IH; ordinary call
            // lowering is not visited. Its arguments are ordinary occurrences
            // and must not smuggle a second, unmarked IH read through the gate.
            let arguments = walk_sequence(
                plan,
                gate,
                owner,
                pending_ih_index,
                deny_raw_ih,
                path,
                args.len() + 1,
                1,
                gates,
                visited,
            )?;
            let mut exits = Vec::new();
            for argument_path in arguments {
                exits.push(argument_path.consume()?);
                gates.insert(gate);
                let pair = (pending_ih_index, binder_morphism.runtime_binder_index);
                if visited
                    .gate_binder_pairs
                    .insert(origin, pair)
                    .is_some_and(|prior| prior != pair)
                {
                    return Err(PendingRefusal::UnsupportedRouteEdge);
                }
            }
            Ok(exits)
        }
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. } => walk_to_gate(
            plan,
            child(0)?,
            owner,
            pending_ih_index,
            deny_raw_ih,
            path,
            gates,
            visited,
        ),
        RuntimeExpr::Let { .. } => {
            let prefix = walk_to_gate(
                plan,
                child(0)?,
                owner,
                pending_ih_index,
                deny_raw_ih,
                path,
                gates,
                visited,
            )?;
            let body_ih = shifted_pending_ih(pending_ih_index, 1)?;
            let mut exits = Vec::new();
            for step in prefix {
                exits.extend(walk_to_gate(
                    plan,
                    child(1)?,
                    owner,
                    body_ih,
                    deny_raw_ih,
                    step,
                    gates,
                    visited,
                )?);
            }
            Ok(exits)
        }
        RuntimeExpr::If { .. } => {
            let prefix = walk_to_gate(
                plan,
                child(0)?,
                owner,
                pending_ih_index,
                deny_raw_ih,
                path,
                gates,
                visited,
            )?;
            let mut exits = Vec::new();
            for step in prefix {
                exits.extend(walk_to_gate(
                    plan,
                    child(1)?,
                    owner,
                    pending_ih_index,
                    deny_raw_ih,
                    step,
                    gates,
                    visited,
                )?);
                exits.extend(walk_to_gate(
                    plan,
                    child(2)?,
                    owner,
                    pending_ih_index,
                    deny_raw_ih,
                    step,
                    gates,
                    visited,
                )?);
            }
            Ok(exits)
        }
        RuntimeExpr::Match { cases, .. } => {
            let prefix = walk_to_gate(
                plan,
                child(0)?,
                owner,
                pending_ih_index,
                deny_raw_ih,
                path,
                gates,
                visited,
            )?;
            let mut exits = Vec::new();
            for step in prefix {
                for (index, case) in cases.iter().enumerate() {
                    let case_ih = shifted_pending_ih(pending_ih_index, case.binders)?;
                    exits.extend(walk_to_gate(
                        plan,
                        child(1 + index)?,
                        owner,
                        case_ih,
                        deny_raw_ih,
                        step,
                        gates,
                        visited,
                    )?);
                }
                // Its default is a RuntimeTrap, so it terminates rather than
                // dropping an open package on an ordinary return.
            }
            Ok(exits)
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            walk_sequence(
                plan,
                origin,
                owner,
                pending_ih_index,
                deny_raw_ih,
                path,
                args.len(),
                0,
                gates,
                visited,
            )
        }
        RuntimeExpr::Record { fields } => walk_sequence(
            plan,
            origin,
            owner,
            pending_ih_index,
            deny_raw_ih,
            path,
            fields.len(),
            0,
            gates,
            visited,
        ),
        RuntimeExpr::Project { .. } => walk_to_gate(
            plan,
            child(0)?,
            owner,
            pending_ih_index,
            deny_raw_ih,
            path,
            gates,
            visited,
        ),
        RuntimeExpr::Effect {
            capability, args, ..
        } => walk_sequence(
            plan,
            origin,
            owner,
            pending_ih_index,
            deny_raw_ih,
            path,
            args.len() + usize::from(capability.is_some()),
            0,
            gates,
            visited,
        ),
        // An arbitrary call or closure crosses a unit boundary, or can copy
        // the pending value before the consuming call. The F5/F6 increment
        // needs its own typed/provenance authority before either is admitted.
        RuntimeExpr::Call { .. }
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::LexicalClosure { .. } => Err(PendingRefusal::RouteLeavesDefiningFunction),
        RuntimeExpr::ComputationalMatch { .. } => Err(PendingRefusal::UnsupportedRouteEdge),
    }
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedPendingCallCandidateObservation {
    pub arm: usize,
    pub body: u32,
    pub callee: SelectedPendingCalleeObservation,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedPendingCalleeObservation {
    StaticResponseOwner(u32),
    OrdinarySpecialization(u32),
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedPendingCallOutcomeObservation {
    ValidatedResponseOwner {
        candidates: Vec<SelectedPendingCallCandidateObservation>,
        defining_function: u32,
        visited: Vec<(u32, &'static str)>,
        traversed_families: Vec<&'static str>,
        gates: Vec<u32>,
        gate_binder_pairs: Vec<(u32, u32, u64)>,
    },
    Refused(PendingRefusal),
    NotApplicable,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedPendingCallAdmissionObservation {
    pub producer: u32,
    pub outcome: SelectedPendingCallOutcomeObservation,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static FORCE_OWNER_JOIN_ORDINARY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static FORCED_OWNER_JOIN_ORDINARY_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(feature = "px8-ds-test-support")]
pub(in crate::cranelift_backend) fn force_owner_join_ordinary() -> bool {
    FORCE_OWNER_JOIN_ORDINARY.with(|enabled| {
        if !enabled.get() {
            return false;
        }
        FORCED_OWNER_JOIN_ORDINARY_APPLICATIONS.with(|count| count.set(count.get() + 1));
        true
    })
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_owner_fed_join_forced_ordinary<T>(
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            FORCE_OWNER_JOIN_ORDINARY.with(|enabled| enabled.set(false));
        }
    }
    FORCE_OWNER_JOIN_ORDINARY.with(|enabled| assert!(!enabled.replace(true)));
    FORCED_OWNER_JOIN_ORDINARY_APPLICATIONS.with(|count| count.set(0));
    let _restore = Restore;
    let result = operation();
    let applications = FORCED_OWNER_JOIN_ORDINARY_APPLICATIONS.with(std::cell::Cell::get);
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectedPendingMatchEmissionKind {
    ValidatedOwnerVisTrap,
    OrdinaryVisBodyLowered,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedPendingMatchEmissionObservation {
    pub origin: u32,
    pub emission_owner: String,
    pub kind: SelectedPendingMatchEmissionKind,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static SELECTED_PENDING_MATCH_EMISSIONS:
        std::cell::RefCell<Option<Vec<SelectedPendingMatchEmissionObservation>>> =
        const { std::cell::RefCell::new(None) };
    static SELECTED_PENDING_CALL_OBSERVATIONS:
        std::cell::RefCell<Option<Vec<SelectedPendingCallAdmissionObservation>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_selected_pending_call_admissions<T>(
    operation: impl FnOnce() -> T,
) -> (T, Vec<SelectedPendingCallAdmissionObservation>) {
    struct Restore(Option<Vec<SelectedPendingCallAdmissionObservation>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            SELECTED_PENDING_CALL_OBSERVATIONS.with(|cell| *cell.borrow_mut() = self.0.take());
        }
    }
    let previous =
        SELECTED_PENDING_CALL_OBSERVATIONS.with(|cell| cell.borrow_mut().replace(Vec::new()));
    let restore = Restore(previous);
    let result = operation();
    let rows = SELECTED_PENDING_CALL_OBSERVATIONS
        .with(|cell| cell.borrow_mut().take())
        .unwrap_or_default();
    drop(restore);
    (result, rows)
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_selected_pending_match_emissions<T>(
    operation: impl FnOnce() -> T,
) -> (T, Vec<SelectedPendingMatchEmissionObservation>) {
    struct Restore(Option<Vec<SelectedPendingMatchEmissionObservation>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            SELECTED_PENDING_MATCH_EMISSIONS.with(|cell| *cell.borrow_mut() = self.0.take());
        }
    }
    let previous =
        SELECTED_PENDING_MATCH_EMISSIONS.with(|cell| cell.borrow_mut().replace(Vec::new()));
    let restore = Restore(previous);
    let result = operation();
    let rows = SELECTED_PENDING_MATCH_EMISSIONS
        .with(|cell| cell.borrow_mut().take())
        .unwrap_or_default();
    drop(restore);
    (result, rows)
}

#[cfg(feature = "px8-ds-test-support")]
pub(in crate::cranelift_backend) fn record_selected_pending_match_emission(
    origin: StaticOriginId,
    owner: ContinuationEmissionOwner,
    kind: SelectedPendingMatchEmissionKind,
) {
    SELECTED_PENDING_MATCH_EMISSIONS.with(|cell| {
        if let Some(rows) = cell.borrow_mut().as_mut() {
            rows.push(SelectedPendingMatchEmissionObservation {
                origin: origin.observation_ordinal(),
                emission_owner: format!("{owner:?}"),
                kind,
            });
        }
    });
}

#[cfg(feature = "px8-ds-test-support")]
pub(super) fn record_selected_pending_call_admissions(plan: &StaticTransitionPlan<'_>) {
    SELECTED_PENDING_CALL_OBSERVATIONS.with(|cell| {
        let mut cell = cell.borrow_mut();
        let Some(rows) = cell.as_mut() else { return };
        rows.extend(plan.selected_pending_calls.iter().map(|(producer, admission)| {
            let outcome = match admission {
                PendingCallAdmission::NotApplicable => SelectedPendingCallOutcomeObservation::NotApplicable,
                PendingCallAdmission::Refused(reason) => SelectedPendingCallOutcomeObservation::Refused(*reason),
                PendingCallAdmission::ValidatedResponseOwner(witness) => {
                    SelectedPendingCallOutcomeObservation::ValidatedResponseOwner {
                        candidates: witness.candidates.iter().map(|candidate| {
                            SelectedPendingCallCandidateObservation {
                                arm: candidate.arm,
                                body: candidate.body.observation_ordinal(),
                                callee: match candidate.callee {
                                    ResolvedContinuationCallee::StaticResponseOwner(id) =>
                                        SelectedPendingCalleeObservation::StaticResponseOwner(id.ordinal()),
                                    ResolvedContinuationCallee::OrdinarySpecialization(id) =>
                                        SelectedPendingCalleeObservation::OrdinarySpecialization(id.observation_ordinal()),
                                },
                            }
                        }).collect(),
                        defining_function: witness.route.defining_function.observation_ordinal(),
                        visited: witness.route.visited.iter().map(|visit| {
                            (visit.origin.observation_ordinal(), match visit.kind {
                                PendingRouteKind::Local => "Local",
                                PendingRouteKind::SelectedArm => "F1",
                                PendingRouteKind::Join => "F2",
                                PendingRouteKind::Binding => "F3",
                                PendingRouteKind::CarriedResidual => "F4",
                            })
                        }).collect(),
                        traversed_families: witness.route.visited.iter().filter_map(|visit| match visit.kind {
                            PendingRouteKind::Local => None,
                            PendingRouteKind::SelectedArm => Some("F1"),
                            PendingRouteKind::Join => Some("F2"),
                            PendingRouteKind::Binding => Some("F3"),
                            PendingRouteKind::CarriedResidual => Some("F4"),
                        }).collect::<BTreeSet<_>>().into_iter().collect(),
                        gates: witness.route.gates.iter().map(|gate| gate.observation_ordinal()).collect(),
                        gate_binder_pairs: witness.route.gate_binder_pairs.iter().map(|pair| {
                            (pair.origin.observation_ordinal(), pair.walker_index, pair.morphism_index)
                        }).collect(),
                    }
                }
            };
            SelectedPendingCallAdmissionObservation { producer: producer.observation_ordinal(), outcome }
        }));
    });
}

fn walk_sequence(
    plan: &StaticTransitionPlan<'_>,
    parent: StaticOriginId,
    owner: PredeclaredFunctionId,
    pending_ih_index: u32,
    deny_raw_ih: bool,
    path: PendingPathCounts,
    len: usize,
    index: usize,
    gates: &mut BTreeSet<StaticOriginId>,
    visited: &mut PendingRouteEvidence<'_>,
) -> Result<Vec<PendingPathCounts>, PendingRefusal> {
    if index == len {
        return Ok(vec![path]);
    }
    let child = plan
        .semantic
        .child_origin(parent, index)
        .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?;
    let prefix = walk_to_gate(
        plan,
        child,
        owner,
        pending_ih_index,
        deny_raw_ih,
        path,
        gates,
        visited,
    )?;
    let mut exits = Vec::new();
    for step in prefix {
        exits.extend(walk_sequence(
            plan,
            parent,
            owner,
            pending_ih_index,
            deny_raw_ih,
            step,
            len,
            index + 1,
            gates,
            visited,
        )?);
    }
    Ok(exits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CheckedComputationalIHBinderMorphism;

    #[test]
    fn one_emission_owner_refuses_mixed_response_and_ordinary_returns() {
        let owned = (StaticOriginId::for_test(352), StaticOriginId::for_test(343));
        let ordinary = (StaticOriginId::for_test(332), StaticOriginId::for_test(321));
        assert!(!owner_reentries_exclusive([owned], [ordinary])
            .expect("ordinary-only emission is not Ret-only"));
        assert!(owner_reentries_exclusive([owned], [owned])
            .expect("owner-only emission is Ret-only"));
        assert!(!owner_reentries_exclusive([owned], [])
            .expect("no re-entry is not an owner-fed emission"));
        let owner = ContinuationEmissionOwner::Predeclared(PredeclaredFunctionId::for_test(3));
        let different_owner = ContinuationEmissionOwner::Predeclared(
            PredeclaredFunctionId::for_test(4),
        );
        let split_emissions = [(owner, owned), (different_owner, ordinary)];
        assert!(owner_reentries_exclusive(
            [owned], split_emissions.into_iter()
                .filter(|(emission, _)| *emission == owner)
                .map(|(_, coordinates)| coordinates),
        ).expect("an ordinary return in a different emission does not mix"));
        let calls = [(owner, owned), (owner, ordinary), (different_owner, ordinary)];
        let incoming_in_one_emission = calls.into_iter()
            .filter(|(emission, _)| *emission == owner)
            .map(|(_, coordinates)| coordinates);
        let error = owner_reentries_exclusive([owned], incoming_in_one_emission)
            .expect_err("one emitted Match cannot mix owner and ordinary returns");
        assert_eq!(error.to_string(), planner_error(
            "owner-fed and ordinary returns re-enter one Match in one emission owner",
        ).to_string());
    }

    /// Transition sentinel: px8tr's existing gather and worker-call retarget
    /// are not selected pending Match producers at this base.
    #[test]
    fn px8tr_gather_and_retarget_nonproducer_transition_sentinel() {
        let (entry, declarations) =
            crate::cranelift_backend::test_objects::px8tr_nested_post_effect_planning_inputs();
        let declarations = declarations
            .iter()
            .map(|declaration| (declaration.symbol.as_str(), declaration))
            .collect::<BTreeMap<_, _>>();
        let mut plan = super::super::plan_static_transition_graph_with_symbols(
            &entry,
            &declarations,
            &crate::NativeProcessSymbols::legacy_prelude(),
            super::super::AbiRootIngress::Value,
            true,
        )
        .expect("the two-parameter gather witness plans");
        plan.install_selected_pending_calls(None)
            .expect("the read-only admission writer handles a nonproducer");
        assert!(!plan.continuation_units().expect("continuation units").is_empty());
        assert!(plan.selected_pending_calls.is_empty());
    }

    #[test]
    fn ordinary_specialization_cannot_publish_an_unvalidated_pending_result() {
        let origin = StaticOriginId::for_test(355);
        let route = PendingCallRoute {
            defining_function: PredeclaredFunctionId::for_test(3),
            visited: Vec::new(),
            gates: vec![StaticOriginId::for_test(47)],
            gate_binder_pairs: Vec::new(),
        };
        let candidate = PendingCandidate {
            arm: 0,
            construct: StaticOriginId::for_test(352),
            body: StaticOriginId::for_test(343),
            callee: ResolvedContinuationCallee::OrdinarySpecialization(
                super::super::ContinuationSpecializationId(0),
            ),
        };
        assert_eq!(
            admit_validated_owner(
                origin, vec![candidate], route, Vec::new(),
                vec![PendingPathCounts::START.consume().unwrap()],
            ),
            PendingCallAdmission::Refused(
                PendingRefusal::PendingResultNotValidatedByResponseOwner,
            ),
        );
    }

    #[test]
    fn checked_template_slot_identifies_the_marked_read_not_the_raw_var() {
        let call = |callee, args: Vec<RuntimeExpr>| RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Var(callee)),
            args,
        };
        let one_arg = vec![RuntimeExpr::Value(crate::RuntimeValue::Bool(true))];
        let kind = CheckedComputationalIHInvocationKind::CheckedHostVisContinuation;
        assert!(marked_pending_read(kind, &call(3, one_arg.clone()), 202, Some(202)));
        assert!(marked_pending_read(kind, &call(4, one_arg.clone()), 202, Some(202)));
        assert!(!marked_pending_read(kind, &call(3, one_arg.clone()), 202, Some(203)));
        assert!(!marked_pending_read(kind, &call(3, one_arg.clone()), 202, None));
        assert!(!marked_pending_read(kind, &call(3, vec![]), 202, Some(202)));
        assert!(!marked_pending_read(
            CheckedComputationalIHInvocationKind::OrdinaryApplication,
            &call(3, one_arg), 202, Some(202),
        ));
    }

    #[test]
    fn agreeing_unit_preserves_the_existing_l2_nonpending_route() {
        let same = StaticOriginId::for_test(343);
        let other = StaticOriginId::for_test(322);
        assert_eq!(agreed_unit_nonpackage([same, same]),
            Some(PendingCallAdmission::NotApplicable));
        assert_eq!(agreed_unit_nonpackage([same, other]), None);
        assert_eq!(agreed_unit_nonpackage(Vec::new()),
            Some(PendingCallAdmission::NotApplicable));
    }

    // Planner-route input: the real walker still enforces AC-0(d), even
    // though no package is issued for a validated response-owner return.
    fn pending_route_input(
        body_var: u32,
        deny_raw_ih: bool,
        call_template_id: u64,
    ) -> Result<(Vec<PendingPathCounts>, BTreeSet<StaticOriginId>), PendingRefusal> {
        let expr = RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::CheckedComputationalIHInvocation {
                call_template_id,
                checked_occurrence_path: vec![20],
                kind: CheckedComputationalIHInvocationKind::CheckedHostVisContinuation,
                binder_morphism: CheckedComputationalIHBinderMorphism::identity_for_test(0),
                body: Box::new(RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(0)),
                    args: vec![RuntimeExpr::Value(crate::RuntimeValue::Bool(true))],
                }),
            }),
            body: Box::new(RuntimeExpr::Var(body_var)),
        };
        let plan = super::super::plan_static_transition_graph(&expr, &BTreeMap::new())
            .expect("a marked route input plans");
        let root = plan.root_static_origin().expect("root exists");
        let owner = occurrence_authority(&plan, root).expect("root has owner").owner;
        let mut gates = BTreeSet::new();
        let slot_for_call = |id| match id {
            171 => Some(202),
            172 => Some(203),
            _ => None,
        };
        let paths = walk_to_gate(
            &plan, root, owner, 0, deny_raw_ih, PendingPathCounts::START,
            &mut gates, &mut PendingRouteEvidence::new(202, &slot_for_call),
        )?;
        Ok((paths, gates))
    }

    #[test]
    fn second_raw_ih_read_refuses_before_the_checked_route_gate() {
        let (paths, gates) = pending_route_input(0, true, 171)
            .expect("the single marked call can consume the gate");
        assert!(!gates.is_empty());
        assert!(paths.iter().all(|path| path.consuming_gates == 1));
        assert_eq!(pending_route_input(1, true, 171).unwrap_err(),
            PendingRefusal::NotLinearOrMustReach);
        let (paths, gates) = pending_route_input(1, false, 171)
            .expect("disabling only the raw read guard still reaches the gate");
        assert!(!gates.is_empty());
        assert!(paths.iter().all(|path| path.consuming_gates == 1));
    }

    #[test]
    fn wrong_template_slot_refuses_at_the_production_walker() {
        assert!(pending_route_input(0, true, 171).is_ok());
        assert_eq!(pending_route_input(0, true, 172).unwrap_err(),
            PendingRefusal::UnsupportedRouteEdge);
    }

    #[test]
    fn generated_call_crossing_refuses_at_the_production_walker() {
        let expr = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(), params: vec!["x".to_owned()],
                body: Box::new(RuntimeExpr::Var(0)),
            }),
            args: vec![RuntimeExpr::Value(crate::RuntimeValue::Bool(true))],
        };
        let plan = super::super::plan_static_transition_graph(&expr, &BTreeMap::new())
            .expect("generated call route plans");
        let call = plan.root_static_origin().expect("root exists");
        let owner = occurrence_authority(&plan, call).expect("call owner exists").owner;
        assert_eq!(walk_to_gate(
            &plan, call, owner, 0, true, PendingPathCounts::START,
            &mut BTreeSet::new(), &mut PendingRouteEvidence::new(0, &|_| None),
        ).unwrap_err(), PendingRefusal::RouteLeavesDefiningFunction);
    }

    // An ordinary source return is an open exit; the checked marker must be
    // consumed on every path before the owner can be admitted. These are
    // planner-route inputs, not a claim that checked source emits this Ret.
    #[test]
    fn open_return_and_generated_crossings_keep_the_admission_refusals() {
        let ret = RuntimeExpr::Construct {
            constructor: "ctor:fixture::pending::ITree::Ret".to_owned(),
            args: Vec::new(),
        };
        let plan = super::super::plan_static_transition_graph(&ret, &BTreeMap::new())
            .expect("a strict return plans");
        let root = plan.root_static_origin().expect("root exists");
        let owner = occurrence_authority(&plan, root).unwrap().owner;
        let mut gates = BTreeSet::new();
        let exits = walk_to_gate(
            &plan, root, owner, 0, true, PendingPathCounts::START,
            &mut gates, &mut PendingRouteEvidence::new(0, &|_| None),
        ).expect("a local return is an open exit");
        assert_eq!(exits, vec![PendingPathCounts::START]);
        assert!(gates.is_empty());
        assert_eq!(
            admit_validated_owner(root, Vec::new(), PendingCallRoute {
                defining_function: owner,
                visited: Vec::new(),
                gates: Vec::new(),
                gate_binder_pairs: Vec::new(),
            }, Vec::new(), exits),
            PendingCallAdmission::Refused(PendingRefusal::NotLinearOrMustReach),
        );

        let generated = PredeclaredFunctionId::for_test(3);
        assert_ne!(owner, generated);
        assert_eq!(
            walk_to_gate(
                &plan, root, generated, 0, true, PendingPathCounts::START,
                &mut BTreeSet::new(), &mut PendingRouteEvidence::new(0, &|_| None),
            ).unwrap_err(),
            PendingRefusal::RouteLeavesDefiningFunction,
        );
    }
}
