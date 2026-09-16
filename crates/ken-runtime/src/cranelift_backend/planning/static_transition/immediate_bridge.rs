//! Plan-owned classification of immediate producer/eliminator bridges.
//!
//! The structural classifier in this module is shared by plan construction and
//! lowering. Planning owns the population and stores only a bounded descriptor;
//! lowering re-runs this pure classifier to borrow the cases/default it needs,
//! then checks every local coordinate against the stored descriptor.
//!
//! **Strata A and B of the `#3676` re-cut.** Stratum A is the plan-INDEPENDENT
//! classifier: it names no plan type and defines no extension `impl`. Stratum B
//! is the plan-coupled realization plane -- the deriving, building, publishing
//! and validating half, its extension `impl`, and its cfg-gated mutation guard.
//! The relation Stratum B builds is stored on `StaticTransitionPlan`; the field
//! and its `BTreeMap::new()` initializer are the only edits the second slice
//! makes outside this file.
//!
//! **NOTHING HERE IS ON A PRODUCTION PATH, and that is the property the slice
//! is defined by.** The live wiring -- the assignment of
//! `publish_immediate_bridge_realization_plan`'s result into the plan before
//! response phase B -- is a successor slice and is deliberately absent, as are
//! the deferred-response sub-case variant for a bridge realized without a
//! physical call and the `owns_seat` rewrite that travels with it. THOSE NAMES
//! ARE NOT SPELLED ANYWHERE IN THIS FILE, deliberately: a zero-hit grep for them
//! is one of this slice's controls, and a mention in a comment turns that
//! control into a count of its own prose. Slice 1's header made the same point;
//! this sentence is where it is easiest to break it.
//!
//! Because nothing calls in from production, a production build reports
//! `never used` here. That is not debris: it is the only live indicator that
//! this module sits on no live path. Do NOT silence it with
//! `#[allow(dead_code)]` -- the warnings clear themselves when the successor
//! wires the plane in. Architect ruling, 2026-09-16.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    planner_capacity_error, planner_error, ContinuationCallIdentity, CraneliftBackendError,
    StaticOriginId, StaticTransitionPlan,
};
use crate::{RuntimeExpr, RuntimeTrap};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ImmediateBridgeConsumerKind {
    Ordinary,
    Computational,
    CheckedComputational { frame_id: u64 },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ImmediateBridgeCause {
    Heterogeneous,
    StaticHostOperation,
}

#[derive(Clone, Copy)]
pub(in crate::cranelift_backend) enum ImmediateBridgeConsumer<'a> {
    Ordinary {
        cases: &'a [crate::RuntimeMatchCase],
        default: &'a RuntimeTrap,
    },
    Computational {
        cases: &'a [crate::RuntimeComputationalMatchCase],
        default: &'a RuntimeTrap,
    },
    CheckedComputational {
        frame_id: u64,
        cases: &'a [crate::RuntimeComputationalMatchCase],
        default: &'a RuntimeTrap,
    },
}

impl ImmediateBridgeConsumer<'_> {
    pub(in crate::cranelift_backend) fn kind(self) -> ImmediateBridgeConsumerKind {
        match self {
            Self::Ordinary { .. } => ImmediateBridgeConsumerKind::Ordinary,
            Self::Computational { .. } => ImmediateBridgeConsumerKind::Computational,
            Self::CheckedComputational { frame_id, .. } => {
                ImmediateBridgeConsumerKind::CheckedComputational { frame_id }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(in crate::cranelift_backend) struct ImmediateBridgeSelection<'a> {
    pub(in crate::cranelift_backend) selected_field: usize,
    pub(in crate::cranelift_backend) consumer: ImmediateBridgeConsumer<'a>,
    pub(in crate::cranelift_backend) checked_ih_slots_wrapper: bool,
    pub(in crate::cranelift_backend) cause: ImmediateBridgeCause,
}

/// The bounded, plan-owned descriptor for one exact causal identity whose
/// producer is realized by an immediate bridge.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ImmediateBridgeRealization {
    identity: ContinuationCallIdentity,
    producer_construct_origin: StaticOriginId,
    computational_frame_origin: StaticOriginId,
    alternative: u32,
    recursive_position: u32,
    case_body_origin: StaticOriginId,
    effective_bridge_body_origin: StaticOriginId,
    selected_field: u32,
    consumer: ImmediateBridgeConsumerKind,
    checked_ih_slots_wrapper: bool,
    cause: ImmediateBridgeCause,
}

impl ImmediateBridgeRealization {
    pub(in crate::cranelift_backend) fn identity(&self) -> &ContinuationCallIdentity {
        &self.identity
    }

    pub(in crate::cranelift_backend) fn producer_construct_origin(&self) -> StaticOriginId {
        self.producer_construct_origin
    }

    pub(in crate::cranelift_backend) fn computational_frame_origin(&self) -> StaticOriginId {
        self.computational_frame_origin
    }

    pub(in crate::cranelift_backend) fn alternative(&self) -> u32 {
        self.alternative
    }

    pub(in crate::cranelift_backend) fn recursive_position(&self) -> u32 {
        self.recursive_position
    }

    pub(in crate::cranelift_backend) fn case_body_origin(&self) -> StaticOriginId {
        self.case_body_origin
    }

    pub(in crate::cranelift_backend) fn effective_bridge_body_origin(&self) -> StaticOriginId {
        self.effective_bridge_body_origin
    }

    pub(in crate::cranelift_backend) fn selected_field(&self) -> u32 {
        self.selected_field
    }

    pub(in crate::cranelift_backend) fn consumer(&self) -> ImmediateBridgeConsumerKind {
        self.consumer
    }

    pub(in crate::cranelift_backend) fn checked_ih_slots_wrapper(&self) -> bool {
        self.checked_ih_slots_wrapper
    }

    pub(in crate::cranelift_backend) fn cause(&self) -> ImmediateBridgeCause {
        self.cause
    }
}

/// Classify only the four ruled immediate-bridge spellings. This function is
/// pure: it reads source structure and returns borrowed source components, but
/// stores no expression and creates no identity.
pub(in crate::cranelift_backend) fn classify_immediate_bridge<'a>(
    body: &'a RuntimeExpr,
    producer_args: &'a [RuntimeExpr],
    argument_binder_offset: usize,
) -> Option<ImmediateBridgeSelection<'a>> {
    let (scrutinee, consumer, checked_ih_slots_wrapper) = match body {
        RuntimeExpr::ComputationalMatch {
            scrutinee,
            cases,
            default,
        } => (
            scrutinee.as_ref(),
            ImmediateBridgeConsumer::Computational { cases, default },
            false,
        ),
        RuntimeExpr::Match {
            scrutinee,
            cases,
            default,
        } => (
            scrutinee.as_ref(),
            ImmediateBridgeConsumer::Ordinary { cases, default },
            false,
        ),
        RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body } => {
            let RuntimeExpr::ComputationalMatch {
                scrutinee,
                cases,
                default,
            } = body.as_ref()
            else {
                return None;
            };
            (
                scrutinee.as_ref(),
                ImmediateBridgeConsumer::CheckedComputational {
                    frame_id: *frame_id,
                    cases,
                    default,
                },
                false,
            )
        }
        RuntimeExpr::CheckedComputationalIHSlots { body, .. } => {
            let RuntimeExpr::Match {
                scrutinee,
                cases,
                default,
            } = body.as_ref()
            else {
                return None;
            };
            (
                scrutinee.as_ref(),
                ImmediateBridgeConsumer::Ordinary { cases, default },
                true,
            )
        }
        _ => return None,
    };
    let RuntimeExpr::Var(index) = scrutinee else {
        return None;
    };
    let index = usize::try_from(*index).ok()?;
    let selected_field = index.checked_sub(argument_binder_offset)?;
    let producer = producer_args.get(selected_field)?;
    let heterogeneous = !checked_ih_slots_wrapper && requires_heterogeneous_deforestation(producer);
    let static_host_operation = statically_selects_host_operation(producer, consumer);
    let cause = if heterogeneous {
        ImmediateBridgeCause::Heterogeneous
    } else if static_host_operation {
        ImmediateBridgeCause::StaticHostOperation
    } else {
        return None;
    };
    Some(ImmediateBridgeSelection {
        selected_field,
        consumer,
        checked_ih_slots_wrapper,
        cause,
    })
}

fn statically_selects_host_operation(
    producer: &RuntimeExpr,
    consumer: ImmediateBridgeConsumer<'_>,
) -> bool {
    let ImmediateBridgeConsumer::Ordinary { cases, .. } = consumer else {
        return false;
    };
    statically_selected_case_reaches_host_effect(producer, cases)
}

fn statically_selected_case_reaches_host_effect(
    producer: &RuntimeExpr,
    cases: &[crate::RuntimeMatchCase],
) -> bool {
    let RuntimeExpr::Construct { constructor, args } = producer else {
        return false;
    };
    let Some(case) = cases.iter().find(|case| case.constructor == *constructor) else {
        return false;
    };
    if case.binders != args.len() {
        return false;
    }
    match &case.body {
        RuntimeExpr::Let { value, .. } if matches!(value.as_ref(), RuntimeExpr::Effect { .. }) => {
            true
        }
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            let RuntimeExpr::Var(index) = scrutinee.as_ref() else {
                return false;
            };
            usize::try_from(*index)
                .ok()
                .filter(|index| *index < case.binders)
                .and_then(|index| args.get(index))
                .is_some_and(|field| statically_selected_case_reaches_host_effect(field, cases))
        }
        _ => false,
    }
}

pub(in crate::cranelift_backend) fn requires_heterogeneous_deforestation(
    expr: &RuntimeExpr,
) -> bool {
    matches!(
        expr,
        RuntimeExpr::Match { .. }
            | RuntimeExpr::ComputationalMatch { .. }
            | RuntimeExpr::If { .. }
            | RuntimeExpr::Call { .. }
    ) && produces_deforestable_aggregate_with_ih(expr, &BTreeSet::new())
}

pub(in crate::cranelift_backend) fn produces_deforestable_aggregate_with_ih(
    expr: &RuntimeExpr,
    aggregate_ihs: &BTreeSet<usize>,
) -> bool {
    match expr {
        RuntimeExpr::CheckedJoinSite { body, .. } => {
            produces_deforestable_aggregate_with_ih(body, aggregate_ihs)
        }
        RuntimeExpr::Construct { .. } => true,
        RuntimeExpr::Let { body, .. } => {
            produces_deforestable_aggregate_with_ih(body, &shifted_aggregate_ihs(aggregate_ihs, 1))
        }
        RuntimeExpr::Match { cases, .. } => {
            !cases.is_empty()
                && cases.iter().all(|case| {
                    produces_deforestable_aggregate_with_ih(
                        &case.body,
                        &shifted_aggregate_ihs(aggregate_ihs, case.binders),
                    )
                })
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            !cases.is_empty()
                && cases.iter().all(|case| {
                    let mut case_ihs = (0..case.recursive_positions.len()).collect::<BTreeSet<_>>();
                    case_ihs.extend(aggregate_ihs.iter().map(|index| {
                        index + case.recursive_positions.len() + case.argument_binders
                    }));
                    produces_deforestable_aggregate_with_ih(&case.body, &case_ihs)
                })
        }
        RuntimeExpr::If {
            then_expr,
            else_expr,
            ..
        } => {
            produces_deforestable_aggregate_with_ih(then_expr, aggregate_ihs)
                && produces_deforestable_aggregate_with_ih(else_expr, aggregate_ihs)
        }
        RuntimeExpr::Call { callee, .. } => {
            if let RuntimeExpr::Var(index) = callee.as_ref() {
                return usize::try_from(*index).is_ok_and(|index| aggregate_ihs.contains(&index));
            }
            match callee.as_ref() {
                RuntimeExpr::Closure {
                    captures,
                    params,
                    body,
                } => produces_deforestable_aggregate_with_ih(
                    body,
                    &shifted_aggregate_ihs(aggregate_ihs, params.len() + captures.len()),
                ),
                RuntimeExpr::LexicalClosure {
                    captures,
                    params,
                    body,
                } => produces_deforestable_aggregate_with_ih(
                    body,
                    &shifted_aggregate_ihs(aggregate_ihs, params.len() + captures.len()),
                ),
                _ => false,
            }
        }
        _ => false,
    }
}

fn shifted_aggregate_ihs(aggregate_ihs: &BTreeSet<usize>, by: usize) -> BTreeSet<usize> {
    aggregate_ihs.iter().map(|index| index + by).collect()
}

/// Derive one realization row per continuation call whose producer is realized
/// by an immediate bridge, in call order.
///
/// Every coordinate a row records is CHECKED against the plan rather than
/// copied from one side of it. A call and its target unit must agree on the
/// producer construct, the producer alternative, the continuation origin and
/// the recursive position; the position must be one the unit declares; the
/// producer occurrence must be a `Construct` and the target frame a
/// `ComputationalMatch`; the selected case's constructor and binder count must
/// match the producer's; and the recursive position must be inside both the
/// producer's arguments and the case's declared recursive positions. A call
/// that classifies as no bridge is skipped, not refused.
fn derive_immediate_bridge_realizations(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<ImmediateBridgeRealization>, CraneliftBackendError> {
    let units = plan.continuation_units()?;
    let mut rows = Vec::new();
    for call in plan.continuation_calls()? {
        let unit = units
            .iter()
            .find(|unit| unit.id() == call.target())
            .ok_or_else(|| planner_error("an immediate-bridge call has no target unit"))?;
        if unit.producer_construct_origin() != call.producer_construct_origin()
            || unit.producer_alternative() != call.producer_alternative()
            || unit.continuation_origin() != call.continuation_origin()
            || unit.recursive_position() != call.recursive_position()
        {
            return Err(planner_error(
                "an immediate-bridge call disagrees with its target unit coordinates",
            ));
        }
        if !unit
            .recursive_positions()
            .contains(&call.recursive_position())
        {
            return Err(planner_error(
                "an immediate-bridge call position is not a declared recursive position",
            ));
        }
        let RuntimeExpr::Construct { constructor, args } =
            plan.planned_occurrence_expr(call.producer_construct_origin())?
        else {
            return Err(planner_error(
                "an immediate-bridge call producer is not a Construct",
            ));
        };
        let RuntimeExpr::ComputationalMatch { cases, .. } =
            plan.planned_occurrence_expr(call.continuation_origin())?
        else {
            return Err(planner_error(
                "an immediate-bridge target frame is not a ComputationalMatch",
            ));
        };
        let alternative = usize::try_from(call.producer_alternative()).map_err(|_| {
            planner_capacity_error("immediate-bridge alternative exceeds addressable range")
        })?;
        let case = cases.get(alternative).ok_or_else(|| {
            planner_error("an immediate-bridge alternative is outside its computational frame")
        })?;
        if case.constructor != *constructor || case.argument_binders != args.len() {
            return Err(planner_error(
                "an immediate-bridge producer disagrees with its selected constructor case",
            ));
        }
        let recursive_position = usize::try_from(call.recursive_position()).map_err(|_| {
            planner_capacity_error("immediate-bridge recursive position exceeds addressable range")
        })?;
        if recursive_position >= args.len()
            || !case.recursive_positions.contains(&recursive_position)
        {
            return Err(planner_error(
                "an immediate-bridge recursive position is outside its selected constructor",
            ));
        }
        let case_body_origin = plan.child_static_origin(
            call.continuation_origin(),
            alternative.checked_add(1).ok_or_else(|| {
                planner_capacity_error("immediate-bridge case-body position overflows")
            })?,
        )?;
        let Some(selection) =
            classify_immediate_bridge(&case.body, args, case.recursive_positions.len())
        else {
            continue;
        };
        let effective_bridge_body_origin = if matches!(
            case.body,
            RuntimeExpr::CheckedSubcontinuationFrame { .. }
                | RuntimeExpr::CheckedComputationalIHSlots { .. }
        ) {
            plan.child_static_origin(case_body_origin, 0)?
        } else {
            case_body_origin
        };
        let identity = call.identity();
        rows.push(ImmediateBridgeRealization {
            identity,
            producer_construct_origin: call.producer_construct_origin(),
            computational_frame_origin: call.continuation_origin(),
            alternative: call.producer_alternative(),
            recursive_position: call.recursive_position(),
            case_body_origin,
            effective_bridge_body_origin,
            selected_field: u32::try_from(selection.selected_field)
                .map_err(|_| planner_capacity_error("immediate-bridge selected field exhausted"))?,
            consumer: selection.consumer.kind(),
            checked_ih_slots_wrapper: selection.checked_ih_slots_wrapper,
            cause: selection.cause,
        });
    }
    Ok(rows)
}

/// Key the derived rows by their complete call identity.
///
/// Two rows for one identity is a refusal rather than a last-writer-wins
/// overwrite: the relation's whole value is that one identity names at most one
/// realization, and silently collapsing a duplicate would make that true by
/// construction instead of by derivation.
fn relation_from_rows(
    rows: Vec<ImmediateBridgeRealization>,
) -> Result<BTreeMap<ContinuationCallIdentity, ImmediateBridgeRealization>, CraneliftBackendError> {
    let mut relation = BTreeMap::new();
    for row in rows {
        if relation.insert(row.identity.clone(), row).is_some() {
            return Err(planner_error(
                "an immediate-bridge relation contains two rows for one complete call identity",
            ));
        }
    }
    Ok(relation)
}

/// The exact structural derivation of the relation, with no mutation applied.
///
/// This is the re-derivation `validate_immediate_bridge_realization_plan`
/// compares the stored plane against, so it must stay free of the test-support
/// perturbations `publish_immediate_bridge_realization_plan` can apply.
pub(super) fn build_immediate_bridge_realization_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeMap<ContinuationCallIdentity, ImmediateBridgeRealization>, CraneliftBackendError> {
    relation_from_rows(derive_immediate_bridge_realizations(plan)?)
}

/// One deliberate perturbation of the published relation, for controls that
/// must show a downstream check actually reads the field it claims to read.
///
/// Each variant names the exact coordinate it moves. They are applied by
/// `publish_immediate_bridge_realization_plan` only, never by
/// `build_immediate_bridge_realization_plan`, so the validator's re-derivation
/// stays an independent reading.
#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum D5bHs10BridgePlanMutation {
    Exact,
    ChangeFullIdentity,
    ChangeSelectedField,
    ChangeConsumerKind,
    ChangeCheckedIhSlotsWrapper,
    ChangeCause,
    ChangeBodyOrigin,
    DropRow,
    DuplicateRow,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static D5B_HS10_BRIDGE_PLAN_MUTATION: std::cell::Cell<D5bHs10BridgePlanMutation> =
        const { std::cell::Cell::new(D5bHs10BridgePlanMutation::Exact) };
    static D5B_HS10_BRIDGE_PLAN_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

/// Restores `Exact` even if the operation panics, so one failing control cannot
/// leak a mutation into every later test on the thread.
#[cfg(feature = "px8-ds-test-support")]
struct D5bHs10BridgePlanMutationGuard(D5bHs10BridgePlanMutation);

#[cfg(feature = "px8-ds-test-support")]
impl Drop for D5bHs10BridgePlanMutationGuard {
    fn drop(&mut self) {
        D5B_HS10_BRIDGE_PLAN_MUTATION.with(|slot| slot.set(self.0));
        D5B_HS10_BRIDGE_PLAN_APPLICATIONS.with(|count| count.set(0));
    }
}

/// Run `operation` with one plan mutation installed, returning its result and
/// the number of times the mutation was actually APPLIED.
///
/// The application count is what separates "the control is green because the
/// check works" from "the control is green because the mutation never fired" --
/// a zero count with a green result is the second, and no verdict.
#[cfg(feature = "px8-ds-test-support")]
pub fn with_d5b_hs10_bridge_plan_mutation<T>(
    mutation: D5bHs10BridgePlanMutation,
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    let previous = D5B_HS10_BRIDGE_PLAN_MUTATION.with(|slot| slot.replace(mutation));
    assert_eq!(
        previous,
        D5bHs10BridgePlanMutation::Exact,
        "HS10 immediate-bridge plan mutations cannot nest"
    );
    D5B_HS10_BRIDGE_PLAN_APPLICATIONS.with(|count| count.set(0));
    let guard = D5bHs10BridgePlanMutationGuard(previous);
    let result = operation();
    let applications = D5B_HS10_BRIDGE_PLAN_APPLICATIONS.with(std::cell::Cell::get);
    drop(guard);
    (result, applications)
}

/// The relation as a plan should store it: the structural derivation, with any
/// installed test-support mutation applied on top.
///
/// Without the `px8-ds-test-support` feature this is exactly
/// `build_immediate_bridge_realization_plan`.
pub(super) fn publish_immediate_bridge_realization_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeMap<ContinuationCallIdentity, ImmediateBridgeRealization>, CraneliftBackendError> {
    let rows = derive_immediate_bridge_realizations(plan)?;
    // Rebound as mutable only where a mutation can actually push a row, so a
    // build without the feature does not carry a needless `mut`. Same shape as
    // the `relation` rebinding below.
    #[cfg(feature = "px8-ds-test-support")]
    let mut rows = rows;
    #[cfg(feature = "px8-ds-test-support")]
    if D5B_HS10_BRIDGE_PLAN_MUTATION
        .with(|slot| slot.get() == D5bHs10BridgePlanMutation::DuplicateRow)
    {
        if let Some(row) = rows.first().cloned() {
            D5B_HS10_BRIDGE_PLAN_APPLICATIONS.with(|count| count.set(count.get() + 1));
            rows.push(row);
        }
    }
    let relation = relation_from_rows(rows)?;
    #[cfg(feature = "px8-ds-test-support")]
    let mut relation = relation;

    #[cfg(feature = "px8-ds-test-support")]
    {
        let mutation = D5B_HS10_BRIDGE_PLAN_MUTATION.with(std::cell::Cell::get);
        if !matches!(
            mutation,
            D5bHs10BridgePlanMutation::Exact | D5bHs10BridgePlanMutation::DuplicateRow
        ) {
            if let Some(key) = relation.keys().next().cloned() {
                let mut row = relation
                    .remove(&key)
                    .expect("the selected immediate-bridge row is present");
                D5B_HS10_BRIDGE_PLAN_APPLICATIONS.with(|count| count.set(count.get() + 1));
                match mutation {
                    D5bHs10BridgePlanMutation::ChangeFullIdentity => {
                        row.identity.recursive_position =
                            row.identity.recursive_position.wrapping_add(1);
                        relation.insert(row.identity.clone(), row);
                    }
                    D5bHs10BridgePlanMutation::ChangeSelectedField => {
                        row.selected_field = row.selected_field.wrapping_add(1);
                        relation.insert(key, row);
                    }
                    D5bHs10BridgePlanMutation::ChangeConsumerKind => {
                        row.consumer = match row.consumer {
                            ImmediateBridgeConsumerKind::Ordinary => {
                                ImmediateBridgeConsumerKind::Computational
                            }
                            ImmediateBridgeConsumerKind::Computational
                            | ImmediateBridgeConsumerKind::CheckedComputational { .. } => {
                                ImmediateBridgeConsumerKind::Ordinary
                            }
                        };
                        relation.insert(key, row);
                    }
                    D5bHs10BridgePlanMutation::ChangeCheckedIhSlotsWrapper => {
                        row.checked_ih_slots_wrapper = !row.checked_ih_slots_wrapper;
                        relation.insert(key, row);
                    }
                    D5bHs10BridgePlanMutation::ChangeCause => {
                        row.cause = match row.cause {
                            ImmediateBridgeCause::Heterogeneous => {
                                ImmediateBridgeCause::StaticHostOperation
                            }
                            ImmediateBridgeCause::StaticHostOperation => {
                                ImmediateBridgeCause::Heterogeneous
                            }
                        };
                        relation.insert(key, row);
                    }
                    D5bHs10BridgePlanMutation::ChangeBodyOrigin => {
                        row.effective_bridge_body_origin.0 =
                            row.effective_bridge_body_origin.0.wrapping_add(1);
                        relation.insert(key, row);
                    }
                    D5bHs10BridgePlanMutation::DropRow => {}
                    D5bHs10BridgePlanMutation::Exact | D5bHs10BridgePlanMutation::DuplicateRow => {
                        unreachable!("non-mutating plan cases were excluded")
                    }
                }
            }
        }
    }

    Ok(relation)
}

/// The stored plane must be its own exact structural re-derivation.
///
/// This is the closeout check the live wiring will call. It compares against
/// `build_`, never against `publish_`, so an installed mutation is caught
/// rather than reproduced.
pub(super) fn validate_immediate_bridge_realization_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<(), CraneliftBackendError> {
    let expected = build_immediate_bridge_realization_plan(plan)?;
    if plan.immediate_bridge_realizations != expected {
        return Err(planner_error(
            "the immediate-bridge relation is not its exact structural re-derivation",
        ));
    }
    Ok(())
}

impl StaticTransitionPlan<'_> {
    /// The realization recorded for one exact call identity, if the producer at
    /// that identity is bridged.
    pub(in crate::cranelift_backend) fn immediate_bridge_realization(
        &self,
        identity: &ContinuationCallIdentity,
    ) -> Option<&ImmediateBridgeRealization> {
        self.immediate_bridge_realizations.get(identity)
    }

    /// Every identity the plane realizes, as a set.
    pub(in crate::cranelift_backend) fn immediate_bridge_realization_identities(
        &self,
    ) -> BTreeSet<ContinuationCallIdentity> {
        self.immediate_bridge_realizations.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RuntimeMatchCase, RuntimeTrapCode};

    // PROVENANCE RULE (`AC-6`). Every case below is one of two kinds and says
    // which in its own doc comment:
    //
    //   LIFTED      its expected verdict is attested by a named site on the
    //               reference branch `0f71ab5b9`, cited as `file:line`.
    //   NEW INTENT  no plan-level ancestor asserts this verdict. It is the
    //               author's reading and is labelled so, by name, in the test.
    //
    // The distinction is not decoration. `AC-2` -- stub the classifier and the
    // tests go red -- measures COUPLING, not FAITHFULNESS: a case whose
    // expectation was read off the implementation also goes red when the
    // implementation is stubbed. Only the citation separates the two.

    fn trap(message: &str) -> RuntimeTrap {
        RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: message.to_string(),
        }
    }

    fn construct(constructor: &str, args: Vec<RuntimeExpr>) -> RuntimeExpr {
        RuntimeExpr::Construct {
            constructor: constructor.to_string(),
            args,
        }
    }

    /// LIFTED -- `core/tests/mod.rs:1518-1522` at `0f71ab5b9`: *"routes its
    /// scrutinee through `lower_computational_producer_expr` when the scrutinee
    /// `requires_heterogeneous_deforestation` -- which a `Call` whose callee is
    /// a closure returning a `Construct` satisfies."*
    ///
    /// THE SPELLING IS THE ANCESTOR'S, NOT A CONVENIENT EQUIVALENT. That
    /// fixture builds the value through `nullary_closure` (`mod.rs:1471-1480`),
    /// which uses **`LexicalClosure`**. `Closure` reaches the same verdict, but
    /// only by a reading of the predicate's `Call` arm -- which is the
    /// implementation, not the ancestor. The `Closure` spelling is carried
    /// separately below and labelled as authored.
    fn heterogeneous_call_producer() -> RuntimeExpr {
        RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: Vec::new(),
                body: Box::new(construct("ctor:fixture::Pair::Mk", Vec::new())),
            }),
            args: Vec::new(),
        }
    }

    /// NEW INTENT. No plan-level ancestor attests the `Closure` spelling; the
    /// attested one is `LexicalClosure` (see above). That the predicate treats
    /// the two identically is read off its `Call` arm, so this case is the
    /// author's reading and is labelled rather than presented as a lift.
    fn heterogeneous_call_producer_ordinary_closure_spelling() -> RuntimeExpr {
        RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(construct("ctor:fixture::Pair::Mk", Vec::new())),
            }),
            args: Vec::new(),
        }
    }

    /// LIFTED -- `core/tests/mod.rs:2691-2693` at `0f71ab5b9`, inside
    /// `seed_call_port_producer_match_example()`: *"a `Match`, so
    /// `requires_heterogeneous_deforestation` holds, on a compile-time
    /// constructor, so exactly one arm is lowered and no join is merged."*
    fn heterogeneous_match_producer() -> RuntimeExpr {
        RuntimeExpr::Match {
            scrutinee: Box::new(construct("ctor:prelude::Bool::True", Vec::new())),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:prelude::Bool::True".to_string(),
                binders: 0,
                body: construct("ctor:fixture::Pair::Mk", Vec::new()),
            }],
            default: trap("heterogeneous match producer selected no case"),
        }
    }

    /// NEW INTENT. No plan-level site asserts a NEGATIVE verdict for
    /// `requires_heterogeneous_deforestation` over a named value; the recorded
    /// rejected shapes at `core/tests/specialization_binding.rs:4301-4302`
    /// constrain the surrounding FIXTURE, not this predicate's input. The
    /// falsity here is structural: `Var` is none of the four spellings the
    /// `matches!` guard admits.
    fn inert_producer() -> RuntimeExpr {
        RuntimeExpr::Var(9)
    }

    fn ordinary_consumer_body(scrutinee: RuntimeExpr) -> RuntimeExpr {
        RuntimeExpr::Match {
            scrutinee: Box::new(scrutinee),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::Pair::Mk".to_string(),
                binders: 0,
                body: RuntimeExpr::Var(0),
            }],
            default: trap("consumer selected no case"),
        }
    }

    #[test]
    fn lifted_call_closure_construct_producer_is_heterogeneous() {
        assert!(requires_heterogeneous_deforestation(
            &heterogeneous_call_producer()
        ));
    }

    #[test]
    fn new_intent_the_ordinary_closure_spelling_is_also_heterogeneous() {
        // NEW INTENT, see the helper: the attested spelling is `LexicalClosure`.
        assert!(requires_heterogeneous_deforestation(
            &heterogeneous_call_producer_ordinary_closure_spelling()
        ));
    }

    #[test]
    fn lifted_compile_time_constructor_match_producer_is_heterogeneous() {
        assert!(requires_heterogeneous_deforestation(
            &heterogeneous_match_producer()
        ));
    }

    #[test]
    fn new_intent_a_var_producer_is_not_heterogeneous() {
        // NEW INTENT, see `inert_producer`.
        assert!(!requires_heterogeneous_deforestation(&inert_producer()));
    }

    /// LIFTED -- `lowering/mod.rs:11739-11740` at `0f71ab5b9`, PRODUCTION code
    /// and therefore attested by every green run of that branch:
    ///
    ///     produces_deforestable_aggregate_with_ih(expr, &recursive_hypotheses)
    ///         && !produces_deforestable_aggregate_with_ih(expr, &BTreeSet::new())
    ///
    /// The shape CROSSES A BINDER on purpose. `shifted_aggregate_ihs` shifts
    /// the IH set by binder depth, and the `Let` arm recurses WITH the shift
    /// while other arms recurse WITHOUT it. A single-point case that never
    /// crosses a binder cannot tell those arms apart, so an off-by-one in the
    /// shift would be invisible to it. The differential over a binder-crossing
    /// shape is what catches it.
    #[test]
    fn lifted_differential_same_expr_opposite_verdicts_across_one_binder() {
        let expr = RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Var(0)),
            body: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Var(1)),
                args: Vec::new(),
            }),
        };
        let ihs = BTreeSet::from([0usize]);
        assert!(
            produces_deforestable_aggregate_with_ih(&expr, &ihs),
            "IH 0 shifted across the Let's binder must reach the Var(1) callee"
        );
        assert!(
            !produces_deforestable_aggregate_with_ih(&expr, &BTreeSet::new()),
            "with no IHs the same expression must not produce a deforestable \
             aggregate -- this is the half that makes it a differential"
        );
    }

    #[test]
    fn lifted_heterogeneous_producer_selected_by_offset_arithmetic() {
        // Structure authored, producer LIFTED. `Var(3)` with offset 1 selects
        // `producer_args[2]`.
        let body = ordinary_consumer_body(RuntimeExpr::Var(3));
        let args = vec![
            inert_producer(),
            inert_producer(),
            heterogeneous_call_producer(),
        ];
        let selection = classify_immediate_bridge(&body, &args, 1)
            .expect("a heterogeneous producer at the selected field must classify");
        assert_eq!(selection.selected_field, 2);
        assert_eq!(selection.cause, ImmediateBridgeCause::Heterogeneous);
        assert!(!selection.checked_ih_slots_wrapper);
    }

    #[test]
    fn checked_ih_slots_wrapper_suppresses_the_heterogeneous_test() {
        // The ONLY place `checked_ih_slots_wrapper` does anything: it gates the
        // heterogeneous test off. Same producer, two wrappers, opposite verdicts.
        let inner = ordinary_consumer_body(RuntimeExpr::Var(0));
        let args = vec![heterogeneous_call_producer()];

        let bare = classify_immediate_bridge(&inner, &args, 0)
            .expect("the bare Match wrapper must classify as heterogeneous");
        assert_eq!(bare.cause, ImmediateBridgeCause::Heterogeneous);
        assert!(!bare.checked_ih_slots_wrapper);

        let wrapped = RuntimeExpr::CheckedComputationalIHSlots {
            slot_template_ids: Vec::new(),
            checked_occurrence_paths: Vec::new(),
            body: Box::new(inner),
        };
        assert!(
            classify_immediate_bridge(&wrapped, &args, 0).is_none(),
            "the IH-slots wrapper must suppress the heterogeneous cause, and \
             with no other cause available the classification must refuse"
        );
    }

    // ─── `AC-7`: one case per refusal point. Six of the seven are structural or
    // arithmetic and are authored freely -- the negative answer has an obvious
    // shape and no semantic verdict is being guessed. Only the last needs a
    // producer whose verdict is attested, and it reuses `inert_producer`.

    #[test]
    fn refuses_a_body_that_is_none_of_the_four_spellings() {
        // `:190` at `0f71ab5b9` -- the `_ => return None` arm of the dispatch.
        let args = vec![heterogeneous_call_producer()];
        assert!(classify_immediate_bridge(&RuntimeExpr::Var(0), &args, 0).is_none());
    }

    #[test]
    fn refuses_a_checked_subcontinuation_frame_whose_inner_form_is_wrong() {
        // `:163` -- the `return None` inside the frame arm. The frame admits
        // ONLY a ComputationalMatch inside; `:156` is the arm head, not the refusal.
        let args = vec![heterogeneous_call_producer()];
        let body = RuntimeExpr::CheckedSubcontinuationFrame {
            frame_id: 7,
            body: Box::new(ordinary_consumer_body(RuntimeExpr::Var(0))),
        };
        assert!(
            classify_immediate_bridge(&body, &args, 0).is_none(),
            "an ordinary Match inside the frame is the wrong inner form"
        );
    }

    #[test]
    fn refuses_an_ih_slots_wrapper_whose_inner_form_is_wrong() {
        // `:182` -- the `return None` inside the IH-slots arm. That wrapper admits
        // ONLY an ordinary Match inside; `:175` is the arm head, not the refusal.
        let args = vec![heterogeneous_call_producer()];
        let body = RuntimeExpr::CheckedComputationalIHSlots {
            slot_template_ids: Vec::new(),
            checked_occurrence_paths: Vec::new(),
            body: Box::new(RuntimeExpr::Var(0)),
        };
        assert!(classify_immediate_bridge(&body, &args, 0).is_none());
    }

    #[test]
    fn refuses_a_scrutinee_that_is_not_a_var() {
        // `:192`.
        let args = vec![heterogeneous_call_producer()];
        let body = ordinary_consumer_body(construct("ctor:prelude::Bool::True", Vec::new()));
        assert!(classify_immediate_bridge(&body, &args, 0).is_none());
    }

    #[test]
    fn refuses_when_the_binder_offset_underflows_the_scrutinee_index() {
        // `:196` -- `index.checked_sub(argument_binder_offset)` returns None.
        let args = vec![heterogeneous_call_producer()];
        let body = ordinary_consumer_body(RuntimeExpr::Var(1));
        assert!(
            classify_immediate_bridge(&body, &args, 2).is_none(),
            "offset 2 against Var(1) must underflow rather than wrap"
        );
    }

    #[test]
    fn refuses_when_the_selected_field_is_past_the_end_of_the_producer_args() {
        // `:197` -- `producer_args.get(selected_field)` returns None.
        let args = vec![heterogeneous_call_producer()];
        let body = ordinary_consumer_body(RuntimeExpr::Var(4));
        assert!(classify_immediate_bridge(&body, &args, 0).is_none());
    }

    #[test]
    fn refuses_when_neither_cause_holds() {
        // `:205`. The producer is structurally inert (see `inert_producer`) and
        // the consumer is an ordinary Match whose cases cannot statically select
        // it, so both causes are false and the classification refuses rather
        // than returning a selection with no cause.
        let args = vec![inert_producer()];
        let body = ordinary_consumer_body(RuntimeExpr::Var(0));
        assert!(classify_immediate_bridge(&body, &args, 0).is_none());
    }

    /// NEW INTENT -- NO PLAN-LEVEL ANCESTOR, AND THIS IS THE FIRST LIVE
    /// APPLICATION OF THAT LABEL.
    ///
    /// `statically_selects_host_operation` is named NOWHERE outside this module
    /// on the reference branch: two hits across `crates/`, the call at `:199`
    /// and the definition at `:215`. No test drives it, so the expected verdict
    /// below is the author's reading of the function and is **not** attested.
    /// It is labelled here rather than left to look like the lifted cases.
    ///
    /// The cause has no consumer until the plan-coupled successor slice lands;
    /// that slice is where it can be attested rather than authored.
    #[test]
    fn new_intent_static_host_operation_cause() {
        let producer = construct("ctor:fixture::Seat::Mk", Vec::new());
        let body = RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::Seat::Mk".to_string(),
                binders: 0,
                body: RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::Effect {
                        family: "eff:fixture::Console".to_string(),
                        operation: ken_host::HostOpV1::ConsoleRead,
                        capability: None,
                        args: Vec::new(),
                    }),
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            }],
            default: trap("static host operation fixture selected no case"),
        };
        let args = [producer];
        let selection = classify_immediate_bridge(&body, &args, 0)
            .expect("a statically selected case reaching a host effect must classify");
        assert_eq!(selection.cause, ImmediateBridgeCause::StaticHostOperation);
    }

    // ─── Stratum B: the realization plane over a real StaticTransitionPlan ───
    //
    // PROVENANCE, per `AC-7`. There are no direct unit tests of these four
    // functions to lift: the reference proved the plane out through the live
    // plan, which is exactly the wiring this slice does not do. So every
    // expectation below is declared, and the two kinds are kept apart:
    //
    //   LIFTED      the VERDICT is attested by a named reference site.
    //   STRUCTURAL  the value is a coordinate of a fixture authored here. It is
    //               read off the fixture, never off the implementation, and
    //               says which part of the fixture fixes it.
    //
    // `AC-2` measures COUPLING, not faithfulness. A structural expectation goes
    // red against a stub exactly as a lifted one does, and proves less; saying
    // which is which is the only thing that keeps the difference visible.

    /// The plan fixture the Stratum B cases run against: one computational
    /// frame whose `Node` case immediately consumes a producer field.
    ///
    /// The producer carries TWO arguments on purpose. Argument 0 is the worker
    /// at the declared recursive position, which is what makes the planner
    /// issue a continuation call at all; argument 1 is the bridged field. A
    /// fixture with one argument cannot separate them -- the bridge would have
    /// to select the worker, and "the selected field is the recursive position"
    /// would be true by construction rather than by derivation.
    ///
    /// The bridged field is `heterogeneous_call_producer()` above, whose
    /// `Heterogeneous` verdict is LIFTED. The rest is STRUCTURAL.
    fn heterogeneous_bridge_plan_fixture() -> RuntimeExpr {
        let unit = || construct("ctor:prelude::Unit::MkUnit", Vec::new());
        let leaf = || construct("ctor:fixture::Bridge::Leaf", Vec::new());
        let worker = RuntimeExpr::LexicalClosure {
            captures: vec![unit()],
            params: vec!["w".to_string()],
            body: Box::new(leaf()),
        };
        let producer = construct(
            "ctor:fixture::Bridge::Node",
            vec![worker, heterogeneous_call_producer()],
        );
        // Scrutinee `Var(2)`: the offset is `recursive_positions.len()` == 1, so
        // this selects producer argument 1, the heterogeneous call.
        let bridge_body = RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Var(2)),
            cases: vec![crate::RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::Pair::Mk".to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: unit(),
            }],
            default: trap("bridge consumer"),
        };
        let frame = RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(producer),
            cases: vec![
                crate::RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::Bridge::Node".to_string(),
                    argument_binders: 2,
                    recursive_positions: vec![0],
                    body: bridge_body,
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::Bridge::Leaf".to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: unit(),
                },
            ],
            default: trap("bridge frame"),
        };
        RuntimeExpr::LexicalClosure {
            captures: vec![unit()],
            params: vec!["input".to_string()],
            body: Box::new(frame),
        }
    }

    /// The derivation reads one row off a real plan, and every recorded
    /// coordinate is checked rather than the count alone.
    ///
    /// LIFTED: `cause == Heterogeneous`. The producer is
    /// `heterogeneous_call_producer()`, whose verdict is attested at
    /// `core/tests/mod.rs:1518-1522` on `0f71ab5b9` (see its own doc comment).
    ///
    /// STRUCTURAL, each fixed by a named part of the fixture above:
    ///   `alternative == 0`             `Node` is case 0 of the frame
    ///   `recursive_position == 0`      the case declares `recursive_positions: [0]`
    ///   `selected_field == 1`          scrutinee `Var(2)`, offset 1
    ///   `consumer == Computational`    the case body is a `ComputationalMatch`
    ///   `checked_ih_slots_wrapper`     false; no checked wrapper in the fixture
    ///
    /// Against a stub returning no rows this reds on the count.
    #[test]
    fn derivation_reads_one_row_with_its_exact_coordinates() {
        let expr = heterogeneous_bridge_plan_fixture();
        let plan = crate::cranelift_backend::planning::static_transition::tests::b2r_plan(&expr);
        let rows = derive_immediate_bridge_realizations(&plan)
            .expect("the bridge fixture derives without refusal");
        assert_eq!(rows.len(), 1, "one bridged call in the fixture: {rows:?}");
        let row = &rows[0];
        assert_eq!(row.alternative(), 0);
        assert_eq!(row.recursive_position(), 0);
        assert_eq!(row.selected_field(), 1);
        assert_eq!(row.consumer(), ImmediateBridgeConsumerKind::Computational);
        assert!(!row.checked_ih_slots_wrapper());
        assert_eq!(row.cause(), ImmediateBridgeCause::Heterogeneous);
        assert_eq!(
            row.computational_frame_origin(),
            plan.continuation_calls().expect("calls")[0].continuation_origin(),
            "the frame origin is the call's own continuation origin"
        );
    }

    /// A continuation call whose case body is not one of the four bridge
    /// spellings is SKIPPED, not refused.
    ///
    /// STRUCTURAL, and it is the discriminator for the case above: the shared
    /// `contspec` fixture's `Node` case body is `Var(0)`, a bare variable. The
    /// plan has continuation calls -- so the loop, and every coordinate check
    /// inside it, runs on real rows -- and yields no realization.
    ///
    /// Without this case, "one row" above is consistent with "every call
    /// becomes a row".
    #[test]
    fn a_non_bridge_case_body_is_skipped_rather_than_refused() {
        let plan = crate::cranelift_backend::planning::static_transition::tests::contspec_plan();
        assert_eq!(
            plan.continuation_calls().expect("calls").len(),
            2,
            "the contspec fixture must actually have calls for the skip to mean anything"
        );
        let rows = derive_immediate_bridge_realizations(&plan)
            .expect("a non-bridge case body is not a refusal");
        assert!(rows.is_empty(), "{rows:?}");
    }

    /// The build keys the derived rows by their complete call identity, and the
    /// relation's single entry is the derived row unchanged.
    ///
    /// STRUCTURAL. The point is the KEYING, which the row vector does not
    /// express: the map's one key must be the row's own identity, not some
    /// coarser coordinate that happens to be unique in a one-row fixture.
    #[test]
    fn the_built_relation_is_keyed_by_the_complete_call_identity() {
        let expr = heterogeneous_bridge_plan_fixture();
        let plan = crate::cranelift_backend::planning::static_transition::tests::b2r_plan(&expr);
        let rows = derive_immediate_bridge_realizations(&plan).expect("derives");
        let relation = build_immediate_bridge_realization_plan(&plan).expect("builds");
        assert_eq!(relation.len(), 1);
        let (key, value) = relation.iter().next().expect("one entry");
        assert_eq!(key, rows[0].identity());
        assert_eq!(value, &rows[0]);
    }

    /// Validation compares the STORED plane against a fresh derivation, so a
    /// plan that never installed the plane is refused the moment a row exists.
    ///
    /// STRUCTURAL, and this is the strongest anti-stub case in the group: the
    /// field is `BTreeMap::new()` at construction and this slice does not wire
    /// the publish, so the bridge fixture's plan holds an EMPTY plane against a
    /// one-row derivation. A `validate_` that returns `Ok(())`, or a `build_`
    /// that returns an empty relation, both make this green.
    #[test]
    fn validation_refuses_a_plan_whose_stored_plane_is_not_its_re_derivation() {
        let expr = heterogeneous_bridge_plan_fixture();
        let plan = crate::cranelift_backend::planning::static_transition::tests::b2r_plan(&expr);
        assert!(
            plan.immediate_bridge_realization_identities().is_empty(),
            "this slice installs no plane; the field must still be empty"
        );
        let error = validate_immediate_bridge_realization_plan(&plan)
            .expect_err("an empty plane against a one-row derivation is not valid");
        assert!(
            format!("{error:?}").contains("not its exact structural re-derivation"),
            "{error:?}"
        );
    }

    /// The same validation ACCEPTS when the derivation is empty, so the refusal
    /// above is about the mismatch and not about the plane being empty.
    ///
    /// STRUCTURAL. Without this pair the previous case cannot tell "validate
    /// compares" from "validate refuses an empty plane".
    #[test]
    fn validation_accepts_an_empty_plane_when_the_derivation_is_empty() {
        let plan = crate::cranelift_backend::planning::static_transition::tests::contspec_plan();
        validate_immediate_bridge_realization_plan(&plan)
            .expect("an empty plane against an empty derivation is valid");
    }

    /// The plan-level accessors read the stored field, which is the only thing
    /// a consumer will ever see.
    ///
    /// STRUCTURAL. On an uninstalled plane both must report absence rather than
    /// fall back to a derivation -- a lookup that silently re-derives would make
    /// the validation above unfalsifiable.
    #[test]
    fn the_plan_accessors_read_the_stored_plane_and_never_re_derive() {
        let expr = heterogeneous_bridge_plan_fixture();
        let plan = crate::cranelift_backend::planning::static_transition::tests::b2r_plan(&expr);
        let derived = derive_immediate_bridge_realizations(&plan).expect("derives");
        assert_eq!(derived.len(), 1);
        assert!(plan.immediate_bridge_realization_identities().is_empty());
        assert!(plan
            .immediate_bridge_realization(derived[0].identity())
            .is_none());
    }

    /// Publishing with no mutation installed is exactly the build, and it
    /// reports zero applications.
    ///
    /// STRUCTURAL. The application count is what separates "the control passed
    /// because the check works" from "the control passed because the mutation
    /// never fired", so the zero is asserted here before any variant relies on
    /// a one.
    #[cfg(feature = "px8-ds-test-support")]
    #[test]
    fn publishing_without_a_mutation_is_the_build_and_applies_nothing() {
        let expr = heterogeneous_bridge_plan_fixture();
        let plan = crate::cranelift_backend::planning::static_transition::tests::b2r_plan(&expr);
        let expected = build_immediate_bridge_realization_plan(&plan).expect("builds");
        let (published, applications) =
            with_d5b_hs10_bridge_plan_mutation(D5bHs10BridgePlanMutation::Exact, || {
                publish_immediate_bridge_realization_plan(&plan)
            });
        assert_eq!(applications, 0);
        assert_eq!(published.expect("publishes"), expected);
    }

    /// Every mutating variant moves the published relation away from the build,
    /// and each reports exactly one application.
    ///
    /// STRUCTURAL, and deliberately written as a loop over the whole variant
    /// set rather than as six cases: the property is that NO variant is inert.
    /// A variant added later with no effect fails here without anyone
    /// remembering to add a case for it.
    ///
    /// `DuplicateRow` is excluded because it does not perturb a relation -- it
    /// makes one impossible to build, which is its own case below.
    #[cfg(feature = "px8-ds-test-support")]
    #[test]
    fn every_plan_mutation_moves_the_published_relation_and_applies_once() {
        let expr = heterogeneous_bridge_plan_fixture();
        let plan = crate::cranelift_backend::planning::static_transition::tests::b2r_plan(&expr);
        let expected = build_immediate_bridge_realization_plan(&plan).expect("builds");
        for mutation in [
            D5bHs10BridgePlanMutation::ChangeFullIdentity,
            D5bHs10BridgePlanMutation::ChangeSelectedField,
            D5bHs10BridgePlanMutation::ChangeConsumerKind,
            D5bHs10BridgePlanMutation::ChangeCheckedIhSlotsWrapper,
            D5bHs10BridgePlanMutation::ChangeCause,
            D5bHs10BridgePlanMutation::ChangeBodyOrigin,
            D5bHs10BridgePlanMutation::DropRow,
        ] {
            let (published, applications) =
                with_d5b_hs10_bridge_plan_mutation(mutation, || {
                    publish_immediate_bridge_realization_plan(&plan)
                });
            let published = published.expect("publishes");
            assert_eq!(applications, 1, "{mutation:?} never fired");
            assert_ne!(published, expected, "{mutation:?} published the exact plan");
            if matches!(mutation, D5bHs10BridgePlanMutation::DropRow) {
                assert!(published.is_empty(), "{mutation:?}: {published:?}");
            } else {
                assert_eq!(published.len(), 1, "{mutation:?}: {published:?}");
            }
        }
    }

    /// Two rows for one identity is a REFUSAL, not a silent overwrite.
    ///
    /// STRUCTURAL. `DuplicateRow` pushes the first derived row a second time,
    /// so both copies carry the same complete identity. A relation builder that
    /// let the later row win would return a one-entry map and look correct.
    #[cfg(feature = "px8-ds-test-support")]
    #[test]
    fn a_duplicate_identity_refuses_rather_than_overwriting() {
        let expr = heterogeneous_bridge_plan_fixture();
        let plan = crate::cranelift_backend::planning::static_transition::tests::b2r_plan(&expr);
        let (published, applications) =
            with_d5b_hs10_bridge_plan_mutation(D5bHs10BridgePlanMutation::DuplicateRow, || {
                publish_immediate_bridge_realization_plan(&plan)
            });
        assert_eq!(applications, 1, "the duplicate was never pushed");
        let error = published.expect_err("two rows for one identity must refuse");
        assert!(
            format!("{error:?}").contains("two rows for one complete call identity"),
            "{error:?}"
        );
    }

    /// A mutation does not leak past its own scope, even though the guard is
    /// thread-local and these tests share a thread with everything else.
    ///
    /// STRUCTURAL. Without this, one failing control above could turn every
    /// later case in the binary into a false red or a false green, and the
    /// cause would be invisible at the failure site.
    #[cfg(feature = "px8-ds-test-support")]
    #[test]
    fn a_plan_mutation_does_not_leak_past_its_scope() {
        let expr = heterogeneous_bridge_plan_fixture();
        let plan = crate::cranelift_backend::planning::static_transition::tests::b2r_plan(&expr);
        let expected = build_immediate_bridge_realization_plan(&plan).expect("builds");
        let (dropped, _) =
            with_d5b_hs10_bridge_plan_mutation(D5bHs10BridgePlanMutation::DropRow, || {
                publish_immediate_bridge_realization_plan(&plan)
            });
        assert!(dropped.expect("publishes").is_empty());
        let after = publish_immediate_bridge_realization_plan(&plan).expect("publishes");
        assert_eq!(after, expected, "the mutation outlived its scope");
    }
}
