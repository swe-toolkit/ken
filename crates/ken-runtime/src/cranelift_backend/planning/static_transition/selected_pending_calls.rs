//! Read-only admission for a pending recursive child selected by a source Match.
//!
//! This is a pre-emission decision. The emitter does not read this plane in
//! increment 1: in particular, no ticket is issued and no extra word is moved.
//! A later increment may consume only a `Planned` record, never infer admission
//! from an arm's position or from a partially emitted context frame.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    occurrences::occurrence_authority, planner_error, CheckedCaseBinderLayout,
    CheckedCaseBinderRole, ContinuationSourceCoordinate, CraneliftBackendError,
    PredeclaredFunctionId, RuntimeExpr, StaticOriginId, StaticTransitionPlan,
};
use crate::cranelift_backend::lowering::core::agreeing_recursive_body_unit;
use crate::{CheckedComputationalIHInvocationKind, OrientedSubcontinuationPlanV1};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum PendingMember {
    WorkerCapture {
        ordinal: u32,
        source: StaticOriginId,
    },
    ContextCapture {
        ordinal: u32,
        coordinate: ContinuationSourceCoordinate,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingCandidate {
    pub(super) arm: usize,
    pub(super) body: StaticOriginId,
    pub(super) members: Vec<PendingMember>,
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
pub(super) struct PendingCallRoute {
    pub(super) defining_function: PredeclaredFunctionId,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingCallPackagePlan {
    pub(super) producer: StaticOriginId,
    pub(super) candidates: Vec<PendingCandidate>,
    pub(super) width: usize,
    pub(super) route: PendingCallRoute,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PendingRefusal {
    RouteLeavesDefiningFunction,
    NotLinearOrMustReach,
    UnsupportedRouteEdge,
    MissingDeclaredMembers,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum PendingCallAdmission {
    Planned(PendingCallPackagePlan),
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
            let RuntimeExpr::Construct { constructor, args } = &case.body else {
                eligible = false;
                break;
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
            let construct_origin = plan.semantic.child_origin(producer, arm + 1)?;
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
                return Err(planner_error(
                    "one pending recursive body belongs to multiple generated contexts",
                ));
            }
            let mut members = Vec::new();
            for capture in unit.worker_captures() {
                let super::ContinuationWorkerCaptureSource::Lexical(source) = capture.source()
                else {
                    candidate_refusal = Some(PendingRefusal::MissingDeclaredMembers);
                    break;
                };
                members.push(PendingMember::WorkerCapture {
                    ordinal: capture.ordinal(),
                    source,
                });
            }
            for input in context.captures()? {
                members.push(PendingMember::ContextCapture {
                    ordinal: input.ordinal,
                    coordinate: input.coordinate,
                });
            }
            if candidate_refusal.is_some()
                || members.len() != unit.worker_capture_count() + context.captures()?.len()
            {
                candidate_refusal = Some(PendingRefusal::MissingDeclaredMembers);
                break;
            }
            declared_bodies.push(unit.worker_body_origin());
            candidates.push(PendingCandidate {
                arm,
                body: unit.worker_body_origin(),
                members,
            });
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
        let width = candidates
            .iter()
            .map(|candidate| candidate.members.len())
            .max()
            .unwrap_or(0);
        if width == 0 {
            by_producer.insert(
                producer,
                PendingCallAdmission::Refused(PendingRefusal::MissingDeclaredMembers),
            );
            continue;
        }
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
            Ok((paths, gates, visited, gate_binder_pairs)) => admit_routed_package(
                PendingCallPackagePlan {
                    producer,
                    candidates,
                    width,
                    route: PendingCallRoute {
                        defining_function: owner,
                        visited,
                        gates,
                        gate_binder_pairs,
                    },
                },
                paths,
            ),
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

fn admit_routed_package(
    package: PendingCallPackagePlan,
    paths: Vec<PendingPathCounts>,
) -> PendingCallAdmission {
    // Marked pending calls consume at their own gate; a second gate is
    // refused at that call. A raw IH occurrence is refused by the walker.
    // This final check establishes must-reach on every ordinary exit.
    if paths.is_empty()
        || package.route.gates.is_empty()
        || paths.iter().any(|path| path.consuming_gates != 1)
    {
        PendingCallAdmission::Refused(PendingRefusal::NotLinearOrMustReach)
    } else {
        PendingCallAdmission::Planned(package)
    }
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
    pub worker_captures: usize,
    pub context_sources: Vec<SelectedPendingCallCaptureObservation>,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedPendingCallCaptureObservation {
    EntryAbi { ordinal: u32, slot: u32 },
    ProducerLocal { ordinal: u32 },
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedPendingCallOutcomeObservation {
    Planned {
        candidates: Vec<SelectedPendingCallCandidateObservation>,
        width: usize,
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
pub(super) fn record_selected_pending_call_admissions(plan: &StaticTransitionPlan<'_>) {
    SELECTED_PENDING_CALL_OBSERVATIONS.with(|cell| {
        let mut cell = cell.borrow_mut();
        let Some(rows) = cell.as_mut() else { return };
        rows.extend(plan.selected_pending_calls.iter().map(|(producer, admission)| {
            let outcome = match admission {
                PendingCallAdmission::NotApplicable => SelectedPendingCallOutcomeObservation::NotApplicable,
                PendingCallAdmission::Refused(reason) => SelectedPendingCallOutcomeObservation::Refused(*reason),
                PendingCallAdmission::Planned(package) => SelectedPendingCallOutcomeObservation::Planned {
                    candidates: package.candidates.iter().map(|candidate| {
                        SelectedPendingCallCandidateObservation {
                            arm: candidate.arm,
                            body: candidate.body.observation_ordinal(),
                            worker_captures: candidate.members.iter().filter(|member| matches!(member, PendingMember::WorkerCapture { .. })).count(),
                            context_sources: candidate.members.iter().filter_map(|member| match member {
                                PendingMember::WorkerCapture { .. } => None,
                                PendingMember::ContextCapture { ordinal, coordinate } => Some(match coordinate {
                                    ContinuationSourceCoordinate::EntryAbi { source_abi_position, .. } => SelectedPendingCallCaptureObservation::EntryAbi { ordinal: *ordinal, slot: *source_abi_position },
                                    ContinuationSourceCoordinate::ProducerLocal { .. } => SelectedPendingCallCaptureObservation::ProducerLocal { ordinal: *ordinal },
                                }),
                            }).collect(),
                        }
                    }).collect(),
                    width: package.width,
                    defining_function: package.route.defining_function.observation_ordinal(),
                    visited: package.route.visited.iter().map(|visit| {
                        (visit.origin.observation_ordinal(), match visit.kind {
                            PendingRouteKind::Local => "Local",
                            PendingRouteKind::SelectedArm => "F1",
                            PendingRouteKind::Join => "F2",
                            PendingRouteKind::Binding => "F3",
                            PendingRouteKind::CarriedResidual => "F4",
                        })
                    }).collect(),
                    traversed_families: package.route.visited.iter().filter_map(|visit| match visit.kind {
                        PendingRouteKind::Local => None,
                        PendingRouteKind::SelectedArm => Some("F1"),
                        PendingRouteKind::Join => Some("F2"),
                        PendingRouteKind::Binding => Some("F3"),
                        PendingRouteKind::CarriedResidual => Some("F4"),
                    }).collect::<BTreeSet<_>>().into_iter().collect(),
                    gates: package.route.gates.iter().map(|gate| gate.observation_ordinal()).collect(),
                    gate_binder_pairs: package.route.gate_binder_pairs.iter().map(|pair| {
                        (pair.origin.observation_ordinal(), pair.walker_index, pair.morphism_index)
                    }).collect(),
                },
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

    fn route_package() -> PendingCallPackagePlan {
        let owner = PredeclaredFunctionId::for_test(3);
        let producer = StaticOriginId::for_test(355);
        let candidates = [343, 322]
            .into_iter()
            .enumerate()
            .map(|(arm, body)| PendingCandidate {
                arm,
                body: StaticOriginId::for_test(body),
                members: vec![PendingMember::WorkerCapture {
                    ordinal: 0,
                    source: StaticOriginId::for_test(358),
                }],
            })
            .collect();
        PendingCallPackagePlan {
            producer,
            candidates,
            width: 1,
            route: PendingCallRoute {
                defining_function: owner,
                visited: vec![PendingRouteVisit {
                    origin: StaticOriginId::for_test(19),
                    kind: PendingRouteKind::CarriedResidual,
                }],
                gates: vec![StaticOriginId::for_test(19)],
                gate_binder_pairs: vec![PendingGateBinderPair {
                    origin: StaticOriginId::for_test(19),
                    walker_index: 0,
                    morphism_index: 0,
                }],
            },
        }
    }

    /// Transition sentinel: px8tr's existing gather and worker-call retarget
    /// are not pending-producer Matches at this base. If a later fixture grows
    /// such a Match, review this scope rather than freezing its present count.
    /// MEASURED: real continuation units exist but no source Match sits inside
    /// a computational match, and admission publishes no package rows.
    /// CLAIMED: this fixture stays on its existing gather/retarget path.
    /// THE GAP: actual emission is covered separately by
    /// `rt_seed_direct_and_context_gather_use_the_same_entry_words` and
    /// `d5a_the_retargeted_worker_call_carries_the_raw_run_plus_the_context_capture_suffix`;
    /// this test alone measures only the pre-emission population boundary.
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
        assert!(
            !plan
                .continuation_units()
                .expect("continuation units")
                .is_empty(),
            "the gather/retarget fixture must contain real continuation units"
        );
        let pending_match_producers = plan
            .source_occurrences
            .iter()
            .flatten()
            .filter(|occurrence| {
                matches!(
                    occurrence.expr,
                    RuntimeExpr::ComputationalMatch { scrutinee, .. }
                        if matches!(scrutinee.as_ref(), RuntimeExpr::Match { .. })
                )
            })
            .count();
        assert_eq!(pending_match_producers, 0);
        assert!(
            plan.selected_pending_calls.is_empty(),
            "a nonproducer must not gain a selected pending-call package: {:?}",
            plan.selected_pending_calls
        );
    }

    // From checked source on base 6bdd75394, the direct double bind fails
    // first in response planning and a pure pair's second read is erased.
    // This planner-route input deliberately places the *same* marked IH call
    // before a Let body: Var(0) names the local result; Var(1) additionally
    // reads the still-bound IH. It is not a checked-source second-read claim.
    fn pending_route_input(
        body_var: u32,
        deny_raw_ih: bool,
        call_template_id: u64,
    ) -> PendingCallAdmission {
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
        let root = plan.root_static_origin().expect("route root exists");
        let owner = occurrence_authority(&plan, root)
            .expect("root has an owner")
            .owner;
        let mut gates = BTreeSet::new();
        // Two distinct valid call templates, but only one names this case's
        // slot. Identity and slot must not be inferred from each other.
        let slot_for_call = |id| match id {
            171 => Some(202),
            172 => Some(203),
            _ => None,
        };
        let mut visited = PendingRouteEvidence::new(202, &slot_for_call);
        match walk_to_gate(
            &plan,
            root,
            owner,
            0,
            deny_raw_ih,
            PendingPathCounts::START,
            &mut gates,
            &mut visited,
        ) {
            Ok(paths) => {
                let mut package = route_package();
                package.route.defining_function = owner;
                package.route.gates = gates.into_iter().collect();
                package.route.visited = visited
                    .kinds
                    .into_iter()
                    .map(|(origin, kind)| PendingRouteVisit { origin, kind })
                    .collect();
                package.route.gate_binder_pairs = visited
                    .gate_binder_pairs
                    .into_iter()
                    .map(
                        |(origin, (walker_index, morphism_index))| PendingGateBinderPair {
                            origin,
                            walker_index,
                            morphism_index,
                        },
                    )
                    .collect();
                admit_routed_package(package, paths)
            }
            Err(reason) => PendingCallAdmission::Refused(reason),
        }
    }

    #[test]
    fn pending_route_input_second_raw_ih_read_is_refused_at_the_walker() {
        assert!(matches!(
            pending_route_input(0, true, 171),
            PendingCallAdmission::Planned(_)
        ));
        assert_eq!(
            pending_route_input(1, true, 171),
            PendingCallAdmission::Refused(PendingRefusal::NotLinearOrMustReach)
        );
        // Disable only the raw occurrence check. The same negative then
        // reaches one marked gate and becomes Planned, proving the refusal's
        // provenance rather than relying on the independent gate counter.
        assert!(matches!(
            pending_route_input(1, false, 171),
            PendingCallAdmission::Planned(_)
        ));
    }

    #[test]
    fn wrong_call_template_slot_refuses_at_the_production_walker() {
        assert!(matches!(
            pending_route_input(0, true, 171),
            PendingCallAdmission::Planned(_)
        ));
        assert_eq!(
            pending_route_input(0, true, 172),
            PendingCallAdmission::Refused(PendingRefusal::UnsupportedRouteEdge)
        );
    }

    #[test]
    fn marked_pending_read_is_the_checked_slot_not_a_raw_var_index() {
        let call = |callee, args: Vec<RuntimeExpr>| RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Var(callee)),
            args,
        };
        let one_arg = vec![RuntimeExpr::Value(crate::RuntimeValue::Bool(true))];
        let kind = CheckedComputationalIHInvocationKind::CheckedHostVisContinuation;
        // The callee is never evaluated by native lowering at this checked
        // marker. Even a mismatched raw Var cannot change its template slot.
        assert!(marked_pending_read(
            kind,
            &call(3, one_arg.clone()),
            202,
            Some(202)
        ));
        assert!(marked_pending_read(
            kind,
            &call(4, one_arg.clone()),
            202,
            Some(202)
        ));
        assert!(!marked_pending_read(
            kind,
            &call(3, one_arg.clone()),
            202,
            Some(203)
        ));
        assert!(!marked_pending_read(
            kind,
            &call(3, one_arg.clone()),
            202,
            None
        ));
        assert!(!marked_pending_read(kind, &call(3, vec![]), 202, Some(202)));
        assert!(!marked_pending_read(
            CheckedComputationalIHInvocationKind::OrdinaryApplication,
            &call(3, one_arg),
            202,
            Some(202),
        ));
    }

    #[test]
    fn agreeing_unit_keeps_the_existing_l2_route_out_of_package_admission() {
        let same = StaticOriginId::for_test(343);
        let other = StaticOriginId::for_test(322);
        assert_eq!(
            agreed_unit_nonpackage([same, same]),
            Some(PendingCallAdmission::NotApplicable)
        );
        assert_eq!(agreed_unit_nonpackage([same, other]), None);
        assert_eq!(
            agreed_unit_nonpackage(Vec::new()),
            Some(PendingCallAdmission::NotApplicable)
        );
    }

    // The only single-edit checked-source attempt that returns the pending
    // ITree from main is rejected by checking (Unit versus ExitCode). A
    // generated-root edge is therefore measured as a route-input fixture;
    // unlike the F5/F6 fixture, it changes only the gate's function owner.
    #[test]
    fn generated_root_owner_cannot_receive_the_pending_gate() {
        let package = route_package();
        let local = package.route.defining_function;
        let generated_root = PredeclaredFunctionId::for_test(0);
        let gated = PendingPathCounts::START.consume().unwrap();
        assert!(require_defining_function(local, local).is_ok());
        assert!(matches!(
            admit_routed_package(package.clone(), vec![gated]),
            PendingCallAdmission::Planned(_)
        ));
        let negative = require_defining_function(local, generated_root)
            .expect_err("a gate in the generated root leaves its defining function");
        assert_eq!(
            PendingCallAdmission::Refused(negative),
            PendingCallAdmission::Refused(PendingRefusal::RouteLeavesDefiningFunction)
        );
        // The negative also traverses the real walker, not just its extracted
        // owner predicate. This root is a planned runtime-IR occurrence with
        // its own function; only the package's claimed owner differs.
        let root_expr = RuntimeExpr::Value(crate::RuntimeValue::Bool(true));
        let plan = super::super::plan_static_transition_graph(&root_expr, &BTreeMap::new())
            .expect("a generated-root occurrence plans");
        let root = plan.root_static_origin().expect("root occurrence exists");
        let actual = occurrence_authority(&plan, root).unwrap().owner;
        assert_ne!(
            actual, local,
            "fixture must actually cross the owner boundary"
        );
        assert_eq!(
            walk_to_gate(
                &plan,
                root,
                local,
                0,
                true,
                PendingPathCounts::START,
                &mut BTreeSet::new(),
                &mut PendingRouteEvidence::new(0, &|_| None),
            )
            .expect_err("the generated root cannot consume another owner's package"),
            PendingRefusal::RouteLeavesDefiningFunction,
        );
    }

    // The identity wrapper source edit checks but erases its call before
    // runtime IR (F6 probe), so it is not a crossing witness. The F5 input
    // returns with an open pending read and no consuming gate; the F6 input
    // takes a generated call instead of the local consuming edge. These are
    // separate from the generated-root location control above.
    #[test]
    fn generated_call_crossing_reaches_the_production_route_walker() {
        let expression = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["x".to_owned()],
                body: Box::new(RuntimeExpr::Var(0)),
            }),
            args: vec![RuntimeExpr::Value(crate::RuntimeValue::Bool(true))],
        };
        let plan = super::super::plan_static_transition_graph(&expression, &BTreeMap::new())
            .expect("generated-call route fixture plans");
        let call = plan.root_static_origin().expect("call root exists");
        let owner = occurrence_authority(&plan, call)
            .expect("call owns an occurrence")
            .owner;
        assert_eq!(
            walk_to_gate(
                &plan,
                call,
                owner,
                0,
                true,
                PendingPathCounts::START,
                &mut BTreeSet::new(),
                &mut PendingRouteEvidence::new(0, &|_| None),
            )
            .expect_err("F6 call cannot carry the pending package"),
            PendingRefusal::RouteLeavesDefiningFunction,
        );
    }

    /// A planned runtime-IR `ITree::Ret` constructor exits with zero gates.
    /// This is a route-input F5 representative, not a checked-source strict
    /// `return_body` witness: the one-edit source variant fails typing (Unit
    /// versus ExitCode). The other path's gate in `route_package` keeps the
    /// inventory nonempty, so must-reach alone distinguishes the refusal.
    #[test]
    fn f5_open_return_reaches_production_walker_and_must_reach_guard() {
        let ret = RuntimeExpr::Construct {
            constructor: "ctor:fixture::pending::ITree::Ret".to_owned(),
            args: Vec::new(),
        };
        let plan = super::super::plan_static_transition_graph(&ret, &BTreeMap::new())
            .expect("a strict return is a planned runtime-IR occurrence");
        let root = plan.root_static_origin().expect("return root exists");
        let owner = occurrence_authority(&plan, root).unwrap().owner;
        let mut gates = BTreeSet::new();
        let exits = walk_to_gate(
            &plan,
            root,
            owner,
            0,
            true,
            PendingPathCounts::START,
            &mut gates,
            &mut PendingRouteEvidence::new(0, &|_| None),
        )
        .expect("the local return itself does not cross a function");
        assert_eq!(exits, vec![PendingPathCounts::START]);
        assert!(gates.is_empty(), "the return is not a consuming gate");
        assert!(matches!(
            admit_routed_package(
                route_package(),
                vec![PendingPathCounts::START.consume().unwrap()]
            ),
            PendingCallAdmission::Planned(_)
        ));
        assert_eq!(
            admit_routed_package(route_package(), exits),
            PendingCallAdmission::Refused(PendingRefusal::NotLinearOrMustReach),
        );
    }
}
