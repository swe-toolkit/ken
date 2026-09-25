//! Read-only admission for a pending recursive child selected by a source Match.
//!
//! This is a pre-emission decision. The emitter does not read this plane in
//! increment 1: in particular, no ticket is issued and no extra word is moved.
//! A later increment may consume only a `Planned` record, never infer admission
//! from an arm's position or from a partially emitted context frame.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    occurrences::occurrence_authority, planner_error, ContinuationSourceCoordinate,
    CraneliftBackendError, PredeclaredFunctionId, RuntimeExpr, StaticOriginId,
    StaticTransitionPlan,
};
use crate::cranelift_backend::lowering::core::agreeing_recursive_body_unit;
use crate::CheckedComputationalIHInvocationKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PendingRouteFamily {
    SelectedArm,
    Join,
    Binding,
    CarriedResidual,
}

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingCallRoute {
    pub(super) defining_function: PredeclaredFunctionId,
    /// These are the only families increment 2 can transport in one function.
    pub(super) families: Vec<PendingRouteFamily>,
    /// Alternative static call sites are permitted only where each reachable
    /// execution path selects exactly one of them.
    pub(super) gates: Vec<StaticOriginId>,
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

/// The only producer-level admission writer. It starts from validated
/// continuation-unit declarations rather than searching for a CLIF arm or
/// interpreting a `producer_alternative` shared by both source arms.
pub(super) fn plan_selected_pending_calls(
    plan: &StaticTransitionPlan<'_>,
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
                eligible = false;
                break;
            }
            let construct_origin = plan.semantic.child_origin(producer, arm + 1)?;
            let mut matching = units.iter().filter(|unit| {
                unit.producer_result_origin() == producer
                    && unit.producer_construct_origin() == construct_origin
                    && unit.recursive_position() as usize == position
            });
            let Some(unit) = matching.next() else {
                eligible = false;
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
                eligible = false;
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
                    eligible = false;
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
            if members.len() != unit.worker_capture_count() + context.captures()?.len() {
                eligible = false;
                break;
            }
            declared_bodies.push(unit.worker_body_origin());
            candidates.push(PendingCandidate {
                arm,
                body: unit.worker_body_origin(),
                members,
            });
        }
        if !eligible || candidates.len() < 2 {
            by_producer.insert(producer, PendingCallAdmission::NotApplicable);
            continue;
        }
        // The existing L2 disagreement predicate remains the authority. Its
        // error identifies the *population*, not an error to propagate from
        // this read-only preflight. L2 still runs, unchanged, at lowering.
        if agreeing_recursive_body_unit(declared_bodies).is_ok() {
            by_producer.insert(producer, PendingCallAdmission::NotApplicable);
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
        let result = prove_route(plan, producer, case_origin, owner);
        let admission = match result {
            Ok(gates) => PendingCallAdmission::Planned(PendingCallPackagePlan {
                producer,
                candidates,
                width,
                route: PendingCallRoute {
                    defining_function: owner,
                    families: vec![
                        PendingRouteFamily::SelectedArm,
                        PendingRouteFamily::Join,
                        PendingRouteFamily::Binding,
                        PendingRouteFamily::CarriedResidual,
                    ],
                    gates,
                },
            }),
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

fn prove_route(
    plan: &StaticTransitionPlan<'_>,
    producer: StaticOriginId,
    consumer: StaticOriginId,
    owner: PredeclaredFunctionId,
) -> Result<Vec<StaticOriginId>, PendingRefusal> {
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
    // Counts are carried separately on each branch; a sum over the whole
    // case would accept two gates in one arm and none in the sibling arm.
    let exits = walk_to_gate(plan, consumer, owner, 0, &mut gates)?;
    if exits.is_empty() || exits.iter().any(|&count| count != 1) || gates.is_empty() {
        return Err(PendingRefusal::NotLinearOrMustReach);
    }
    Ok(gates.into_iter().collect())
}

/// Follow the checked case body, not a guessed arm-order-to-CLIF mapping.
/// `None` of the returned paths is the compiler's explicit trap/termination;
/// any ordinary exit with zero or two reads refuses. All unrecognized calls,
/// closure boundaries and owner changes refuse rather than inventing an F5/F6
/// transport. This traversal is bounded by the acyclic source occurrence tree.
fn walk_to_gate(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    owner: PredeclaredFunctionId,
    count: u8,
    gates: &mut BTreeSet<StaticOriginId>,
) -> Result<Vec<u8>, PendingRefusal> {
    if occurrence_authority(plan, origin)
        .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?
        .owner
        != owner
    {
        return Err(PendingRefusal::RouteLeavesDefiningFunction);
    }
    let expr = plan
        .planned_occurrence_expr(origin)
        .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?;
    let child = |position| {
        plan.semantic
            .child_origin(origin, position)
            .map_err(|_| PendingRefusal::UnsupportedRouteEdge)
    };
    match expr {
        RuntimeExpr::Trap(_) => Ok(Vec::new()),
        RuntimeExpr::Value(_)
        | RuntimeExpr::Var(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. } => Ok(vec![count]),
        RuntimeExpr::CheckedComputationalIHInvocation { kind, body, .. } => {
            if *kind != CheckedComputationalIHInvocationKind::CheckedHostVisContinuation
                || !matches!(body.as_ref(), RuntimeExpr::Call { callee, args }
                    if matches!(callee.as_ref(), RuntimeExpr::Var(_)) && args.len() == 1)
            {
                return Err(PendingRefusal::UnsupportedRouteEdge);
            }
            if count != 0 {
                return Err(PendingRefusal::NotLinearOrMustReach);
            }
            gates.insert(child(0)?);
            Ok(vec![1])
        }
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. } => {
            walk_to_gate(plan, child(0)?, owner, count, gates)
        }
        RuntimeExpr::Let { .. } => {
            let prefix = walk_to_gate(plan, child(0)?, owner, count, gates)?;
            let mut exits = Vec::new();
            for n in prefix {
                exits.extend(walk_to_gate(plan, child(1)?, owner, n, gates)?);
            }
            Ok(exits)
        }
        RuntimeExpr::If { .. } => {
            let prefix = walk_to_gate(plan, child(0)?, owner, count, gates)?;
            let mut exits = Vec::new();
            for n in prefix {
                exits.extend(walk_to_gate(plan, child(1)?, owner, n, gates)?);
                exits.extend(walk_to_gate(plan, child(2)?, owner, n, gates)?);
            }
            Ok(exits)
        }
        RuntimeExpr::Match { cases, .. } => {
            let prefix = walk_to_gate(plan, child(0)?, owner, count, gates)?;
            let mut exits = Vec::new();
            for n in prefix {
                for index in 0..cases.len() {
                    exits.extend(walk_to_gate(plan, child(1 + index)?, owner, n, gates)?);
                }
                // Its default is a RuntimeTrap, so it terminates rather than
                // dropping an open package on an ordinary return.
            }
            Ok(exits)
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            walk_sequence(plan, origin, owner, count, args.len(), 0, gates)
        }
        RuntimeExpr::Record { fields } => {
            walk_sequence(plan, origin, owner, count, fields.len(), 0, gates)
        }
        RuntimeExpr::Project { .. } => walk_to_gate(plan, child(0)?, owner, count, gates),
        RuntimeExpr::Effect {
            capability, args, ..
        } => walk_sequence(
            plan,
            origin,
            owner,
            count,
            args.len() + usize::from(capability.is_some()),
            0,
            gates,
        ),
        // An arbitrary call or closure crosses a unit boundary, or can copy
        // the pending value before the consuming call. The F5/F6 increment
        // needs its own typed/provenance authority before either is admitted.
        RuntimeExpr::Call { .. }
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::LexicalClosure { .. }
        | RuntimeExpr::ComputationalMatch { .. } => Err(PendingRefusal::UnsupportedRouteEdge),
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
        families: Vec<&'static str>,
        gates: Vec<u32>,
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
                    families: package.route.families.iter().map(|family| match family {
                        PendingRouteFamily::SelectedArm => "F1",
                        PendingRouteFamily::Join => "F2",
                        PendingRouteFamily::Binding => "F3",
                        PendingRouteFamily::CarriedResidual => "F4",
                    }).collect(),
                    gates: package.route.gates.iter().map(|gate| gate.observation_ordinal()).collect(),
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
    count: u8,
    len: usize,
    index: usize,
    gates: &mut BTreeSet<StaticOriginId>,
) -> Result<Vec<u8>, PendingRefusal> {
    if index == len {
        return Ok(vec![count]);
    }
    let child = plan
        .semantic
        .child_origin(parent, index)
        .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?;
    let prefix = walk_to_gate(plan, child, owner, count, gates)?;
    let mut exits = Vec::new();
    for n in prefix {
        exits.extend(walk_sequence(
            plan,
            parent,
            owner,
            n,
            len,
            index + 1,
            gates,
        )?);
    }
    Ok(exits)
}
