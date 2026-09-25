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
use crate::{CheckedComputationalIHBinderMorphism, CheckedComputationalIHInvocationKind};

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

// One set of counters per execution path, never a total over the whole match.
// Two *marked* pending reads can reach only one consuming gate. A plain
// source `Var` is not a pending-read identity without its checked binder map.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PendingPathCounts {
    pending_reads: u8,
    consuming_gates: u8,
}

impl PendingPathCounts {
    const START: Self = Self {
        pending_reads: 0,
        consuming_gates: 0,
    };

    fn read_pending(self, enforce_linearity: bool) -> Result<Self, PendingRefusal> {
        if enforce_linearity && self.pending_reads != 0 {
            return Err(PendingRefusal::NotLinearOrMustReach);
        }
        Ok(Self {
            pending_reads: self
                .pending_reads
                .checked_add(1)
                .ok_or(PendingRefusal::NotLinearOrMustReach)?,
            ..self
        })
    }

    fn consume(self) -> Result<Self, PendingRefusal> {
        if self.consuming_gates != 0 || self.pending_reads == 0 {
            return Err(PendingRefusal::NotLinearOrMustReach);
        }
        Ok(Self {
            consuming_gates: 1,
            ..self
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingCallRoute {
    pub(super) defining_function: PredeclaredFunctionId,
    /// Allowed F1-F4 transport families, not a traversal census. Increment 2
    /// must prove which edges the selected package actually takes in emission.
    pub(super) allowed_families: Vec<PendingRouteFamily>,
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
            Ok((paths, gates)) => admit_routed_package(
                PendingCallPackagePlan {
                    producer,
                    candidates,
                    width,
                    route: PendingCallRoute {
                        defining_function: owner,
                        allowed_families: vec![
                            PendingRouteFamily::SelectedArm,
                            PendingRouteFamily::Join,
                            PendingRouteFamily::Binding,
                            PendingRouteFamily::CarriedResidual,
                        ],
                        gates,
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
    binder_morphism: CheckedComputationalIHBinderMorphism,
    body: &RuntimeExpr,
) -> bool {
    kind == CheckedComputationalIHInvocationKind::CheckedHostVisContinuation
        && matches!(body, RuntimeExpr::Call { callee, args }
            if args.len() == 1
                && matches!(callee.as_ref(), RuntimeExpr::Var(index)
                    if binder_morphism.runtime_index(0) == Some(u64::from(*index))))
}

fn admit_routed_package(
    package: PendingCallPackagePlan,
    paths: Vec<PendingPathCounts>,
) -> PendingCallAdmission {
    // The marked pending-read guard rejects at a second marked read; the
    // checked-source direct double-bind is shadowed by response planning at
    // this base. This final check establishes must-reach and
    // one consuming gate, independently of that read count.
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

fn refuse_generated_unit_crossing() -> PendingRefusal {
    PendingRefusal::RouteLeavesDefiningFunction
}

fn prove_route(
    plan: &StaticTransitionPlan<'_>,
    producer: StaticOriginId,
    consumer: StaticOriginId,
    owner: PredeclaredFunctionId,
) -> Result<(Vec<PendingPathCounts>, Vec<StaticOriginId>), PendingRefusal> {
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
    let exits = walk_to_gate(plan, consumer, owner, PendingPathCounts::START, &mut gates)?;
    Ok((exits, gates.into_iter().collect()))
}

/// Follow the checked case body, not a guessed arm-order-to-CLIF mapping.
/// `None` of the returned paths is the compiler's explicit trap/termination;
/// any ordinary exit with zero or two marked gate reads refuses. The checked
/// binder morphism, not a bare `Var` spelling, identifies a marked read.
/// All unrecognized calls,
/// closure boundaries and owner changes refuse rather than inventing an F5/F6
/// transport. This traversal is bounded by the acyclic source occurrence tree.
fn walk_to_gate(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    owner: PredeclaredFunctionId,
    path: PendingPathCounts,
    gates: &mut BTreeSet<StaticOriginId>,
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
        | RuntimeExpr::ImportedDeclarationRef { .. } => Ok(vec![path]),
        RuntimeExpr::CheckedComputationalIHInvocation {
            kind,
            binder_morphism,
            body,
            ..
        } => {
            if !marked_pending_read(*kind, *binder_morphism, body) {
                return Err(PendingRefusal::UnsupportedRouteEdge);
            }
            let path = path.read_pending(true)?.consume()?;
            gates.insert(child(0)?);
            Ok(vec![path])
        }
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. } => {
            walk_to_gate(plan, child(0)?, owner, path, gates)
        }
        RuntimeExpr::Let { .. } => {
            let prefix = walk_to_gate(plan, child(0)?, owner, path, gates)?;
            let mut exits = Vec::new();
            for step in prefix {
                exits.extend(walk_to_gate(plan, child(1)?, owner, step, gates)?);
            }
            Ok(exits)
        }
        RuntimeExpr::If { .. } => {
            let prefix = walk_to_gate(plan, child(0)?, owner, path, gates)?;
            let mut exits = Vec::new();
            for step in prefix {
                exits.extend(walk_to_gate(plan, child(1)?, owner, step, gates)?);
                exits.extend(walk_to_gate(plan, child(2)?, owner, step, gates)?);
            }
            Ok(exits)
        }
        RuntimeExpr::Match { cases, .. } => {
            let prefix = walk_to_gate(plan, child(0)?, owner, path, gates)?;
            let mut exits = Vec::new();
            for step in prefix {
                for index in 0..cases.len() {
                    exits.extend(walk_to_gate(plan, child(1 + index)?, owner, step, gates)?);
                }
                // Its default is a RuntimeTrap, so it terminates rather than
                // dropping an open package on an ordinary return.
            }
            Ok(exits)
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            walk_sequence(plan, origin, owner, path, args.len(), 0, gates)
        }
        RuntimeExpr::Record { fields } => {
            walk_sequence(plan, origin, owner, path, fields.len(), 0, gates)
        }
        RuntimeExpr::Project { .. } => walk_to_gate(plan, child(0)?, owner, path, gates),
        RuntimeExpr::Effect {
            capability, args, ..
        } => walk_sequence(
            plan,
            origin,
            owner,
            path,
            args.len() + usize::from(capability.is_some()),
            0,
            gates,
        ),
        // An arbitrary call or closure crosses a unit boundary, or can copy
        // the pending value before the consuming call. The F5/F6 increment
        // needs its own typed/provenance authority before either is admitted.
        RuntimeExpr::Call { .. }
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::LexicalClosure { .. } => Err(refuse_generated_unit_crossing()),
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
        allowed_families: Vec<&'static str>,
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
                    allowed_families: package.route.allowed_families.iter().map(|family| match family {
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
    path: PendingPathCounts,
    len: usize,
    index: usize,
    gates: &mut BTreeSet<StaticOriginId>,
) -> Result<Vec<PendingPathCounts>, PendingRefusal> {
    if index == len {
        return Ok(vec![path]);
    }
    let child = plan
        .semantic
        .child_origin(parent, index)
        .map_err(|_| PendingRefusal::UnsupportedRouteEdge)?;
    let prefix = walk_to_gate(plan, child, owner, path, gates)?;
    let mut exits = Vec::new();
    for step in prefix {
        exits.extend(walk_sequence(
            plan,
            parent,
            owner,
            step,
            len,
            index + 1,
            gates,
        )?);
    }
    Ok(exits)
}

#[cfg(test)]
mod tests {
    use super::*;

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
                allowed_families: vec![
                    PendingRouteFamily::SelectedArm,
                    PendingRouteFamily::CarriedResidual,
                ],
                gates: vec![StaticOriginId::for_test(19)],
            },
        }
    }

    // From checked source on base 6bdd75394, a direct double bind fails first
    // in response planning; a pure pair's unused second read is erased before
    // runtime IR. This is a *defensive route-input* fixture, not a claim that
    // the source mutation reaches the admission guard. Both routes use the
    // same two-candidate package; only the second pending-binding read varies.
    fn one_route(second_read: bool, enforce_linearity: bool) -> PendingCallAdmission {
        let routed = (|| {
            let path = PendingPathCounts::START.read_pending(enforce_linearity)?;
            let path = if second_read {
                path.read_pending(enforce_linearity)?
            } else {
                path
            };
            path.consume()
        })();
        match routed {
            Ok(path) => admit_routed_package(route_package(), vec![path]),
            Err(reason) => PendingCallAdmission::Refused(reason),
        }
    }

    #[test]
    fn marked_pending_read_is_the_bound_ih_not_just_any_var_call() {
        let morphism = CheckedComputationalIHBinderMorphism::identity_for_test(4);
        let call = |callee, args: Vec<RuntimeExpr>| RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Var(callee)),
            args,
        };
        let one_arg = vec![RuntimeExpr::Value(crate::RuntimeValue::Bool(true))];
        assert!(marked_pending_read(
            CheckedComputationalIHInvocationKind::CheckedHostVisContinuation,
            morphism,
            &call(4, one_arg.clone()),
        ));
        assert!(!marked_pending_read(
            CheckedComputationalIHInvocationKind::CheckedHostVisContinuation,
            morphism,
            &call(5, one_arg),
        ));
        assert!(!marked_pending_read(
            CheckedComputationalIHInvocationKind::CheckedHostVisContinuation,
            morphism,
            &call(4, vec![]),
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

    #[test]
    fn pending_route_input_second_read_is_a_distinct_linearity_refusal() {
        assert!(matches!(
            one_route(false, true),
            PendingCallAdmission::Planned(_)
        ));
        assert_eq!(
            one_route(true, true),
            PendingCallAdmission::Refused(PendingRefusal::NotLinearOrMustReach),
        );
        // Disable only the read-linearity guard; the negative must now plan.
        // The gate count, owner, members and both candidates are unchanged.
        assert!(matches!(
            one_route(true, false),
            PendingCallAdmission::Planned(_)
        ));
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
        let gated = PendingPathCounts::START
            .read_pending(true)
            .unwrap()
            .consume()
            .unwrap();
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
                PendingPathCounts::START,
                &mut BTreeSet::new()
            )
            .expect_err("F6 call cannot carry the pending package"),
            PendingRefusal::RouteLeavesDefiningFunction,
        );
    }

    #[test]
    fn strict_return_and_generated_call_routes_refuse_before_consumption() {
        let package = route_package();
        let selected = PendingPathCounts::START.read_pending(true).unwrap();
        assert!(matches!(
            admit_routed_package(package.clone(), vec![selected.consume().unwrap()]),
            PendingCallAdmission::Planned(_)
        ));
        assert_eq!(
            admit_routed_package(package, vec![selected]),
            PendingCallAdmission::Refused(PendingRefusal::NotLinearOrMustReach),
            "F5 must not return an open pending binding without its gate"
        );
        assert_eq!(
            PendingCallAdmission::Refused(refuse_generated_unit_crossing()),
            PendingCallAdmission::Refused(PendingRefusal::RouteLeavesDefiningFunction),
            "F6 may not carry the package into a generated unit"
        );
    }
}
