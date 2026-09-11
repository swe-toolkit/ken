//! Plan-owned classification of immediate producer/eliminator bridges.
//!
//! The structural classifier in this module is shared by plan construction and
//! lowering. Planning owns the population and stores only a bounded descriptor;
//! lowering re-runs this pure classifier to borrow the cases/default it needs,
//! then checks every local coordinate against the stored descriptor.

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

pub(super) fn build_immediate_bridge_realization_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeMap<ContinuationCallIdentity, ImmediateBridgeRealization>, CraneliftBackendError> {
    relation_from_rows(derive_immediate_bridge_realizations(plan)?)
}

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
    pub(in crate::cranelift_backend) fn immediate_bridge_realization(
        &self,
        identity: &ContinuationCallIdentity,
    ) -> Option<&ImmediateBridgeRealization> {
        self.immediate_bridge_realizations.get(identity)
    }

    pub(in crate::cranelift_backend) fn immediate_bridge_realization_identities(
        &self,
    ) -> BTreeSet<ContinuationCallIdentity> {
        self.immediate_bridge_realizations.keys().cloned().collect()
    }
}
