//! The planner-owned Joins-and-traps domain -- join disposition and trap
//! identity.
//!
//! `RT-PLANNER-JOINS-TRAPS-SPLIT` `D1` -- this module owns which result
//! representation (`NativeScalarPair` vs `CarrierWord`) a source join takes,
//! derived by `summarize_result_phase`'s recursive walk and consumed as
//! `JoinPlanToken`; and the value-keyed trap identity catalog
//! (`PlannedTrapIdentity`), a pure dedup index with no source-origin
//! involvement at all. `StaticTransitionPlan` and `Planner` both stay in the
//! parent; the impl fragments here read their ancestor-private root state
//! under the standing child-module pattern (item 4's `units.rs` precedent).
//!
//! The emitter-owned half -- `Px8trTrapProvenanceEvent` and
//! `PlannedTrapSeat` -- is a DIFFERENT thing entirely and already lives in
//! `lowering/mod.rs` (item 14's territory); see the `D0` ledger's frozen-
//! predicate application in `docs/program/issues/RT-PLANNER-JOINS-TRAPS-SPLIT.md`.

#[cfg(test)]
use std::cell::Cell;
use std::collections::BTreeSet;

use super::{
    planner_capacity_error, planner_error, AbiSchedulingIngress, AbiSlotKind, AbiUnitDefinition,
    CraneliftBackendError, PredeclaredFunctionId, StaticOriginId, StaticTransitionPlan,
};
use super::construction::Planner;
use crate::{RuntimeExpr, RuntimePartiality, RuntimeTrap, RuntimeTrapCode};

/// The complete, pre-emission result representation of one source join.
///
/// This is deliberately a two-way type rather than a phase bit threaded through
/// lowering.  In particular, lowering cannot add a third representation or
/// select one from an emitted predecessor.
/// **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- the ONE dead-arm trap, built by the
/// one function both sides call.**
///
/// A `RuntimeTrap` is only emittable if the planner INTERNED it, and identity
/// is by VALUE (`trap_identity` searches the catalog for an equal trap). So the
/// planner and the lowering must produce a byte-identical trap for the same
/// occurrence, and the only way to guarantee that is for neither to spell it.
/// Two copies of this message would compile, intern one value, look up another,
/// and fail as "no planner-bound identity" -- at emission, on a program that
/// had already passed every earlier gate.
pub(in crate::cranelift_backend) fn dead_arm_effect_trap(
    family: &str,
    operation: ken_host::HostOpV1,
) -> RuntimeTrap {
    RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: format!(
            "unreachable {family}.{} arm: this arm's request constructor is neither constructed \
             by the program nor producible by the runtime, so no execution selects it",
            operation as u16
        ),
    }
}

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
pub(super) struct PlannedJoinResult {
    pub(super) representation: JoinResultRepresentation,
    has_continuing_predecessor: bool,
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

pub(super) fn build_join_result_plan(
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

impl<'src> Planner<'src> {
    pub(super) fn intern_trap(
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
}

impl<'src> StaticTransitionPlan<'src> {
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

    pub(super) fn validate_join_result_plan(&self) -> Result<(), CraneliftBackendError> {
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
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::tests::trap;
    use super::super::{
        governed_nested_resource_bracket, plan_static_transition_graph_with_symbols, AbiRootIngress,
    };
    use super::*;
    use crate::{RuntimeMatchCase, RuntimeValue};

    fn d8_mixed_join(swapped: bool) -> RuntimeExpr {
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

    fn d8_functionized_plan(
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

    fn d8_environment_join(swapped: bool) -> RuntimeExpr {
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

    fn assert_d8_environment_join_is_carrier(swapped: bool) {
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

    fn d8_bound_callable_join(swapped: bool) -> RuntimeExpr {
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

    fn assert_d8_bound_callable_join_is_carrier(swapped: bool) {
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

    fn d8_abi_parameter_join(swapped: bool) -> RuntimeExpr {
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

    fn d8_abi_parameter_join_origin(
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
}
