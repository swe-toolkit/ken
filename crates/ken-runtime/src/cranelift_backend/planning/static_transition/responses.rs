//! Compile-time response-producer to static-continuation feasibility.
//!
//! This module derives the relation from both ends before any response-owner
//! Function exists. The producer half starts at a closed `ITree::Vis` operation
//! and resolves the matching host-response case. The continuation half starts
//! at the planner-issued continuation specialization and retains its worker,
//! context, captures, continuation inputs, and opaque call identity.

use std::collections::BTreeMap;

use super::continuations::{
    continuation_owner_entry_sources, walk_continuation_value_environment,
    ContinuationCallIdentity, ContinuationContextId, ContinuationEmissionOwner,
    ContinuationSourceCoordinate, ContinuationValueSourceAuthority,
    ContinuationWorkerCaptureSource,
};
use super::occurrences::StaticOriginId;
use super::{planner_capacity_error, planner_error, CraneliftBackendError, StaticTransitionPlan};
use crate::{CheckedComputationalIHInvocationKind, HostOpV1, RuntimeExpr, RuntimeSymbol};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct StaticResponseContinuationId(u32);

impl StaticResponseContinuationId {
    fn from_position(position: usize) -> Result<Self, CraneliftBackendError> {
        Ok(Self(u32::try_from(position).map_err(|_| {
            planner_capacity_error("static response continuation identity exhausted")
        })?))
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticResponseContinuation {
    id: StaticResponseContinuationId,
    base_owner: ContinuationEmissionOwner,
    producer_call_origin: StaticOriginId,
    response_origin: StaticOriginId,
    vis_origin: StaticOriginId,
    k_identity: ContinuationCallIdentity,
    k_closure_origin: StaticOriginId,
    k_body_origin: StaticOriginId,
    k_context: ContinuationContextId,
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

    pub(in crate::cranelift_backend) fn vis_origin(&self) -> StaticOriginId {
        self.vis_origin
    }

    pub(in crate::cranelift_backend) fn k_identity(&self) -> &ContinuationCallIdentity {
        &self.k_identity
    }

    pub(in crate::cranelift_backend) fn k_closure_origin(&self) -> StaticOriginId {
        self.k_closure_origin
    }

    pub(in crate::cranelift_backend) fn k_body_origin(&self) -> StaticOriginId {
        self.k_body_origin
    }

    pub(in crate::cranelift_backend) fn k_context(&self) -> ContinuationContextId {
        self.k_context
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
) -> Result<Option<HostResponseRoute>, CraneliftBackendError> {
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
                selected = Some(route);
            }
        }
        pending.extend(plan.semantic.child_origins(origin)?.iter().copied());
    }
    Ok(selected)
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

impl StaticTransitionPlan<'_> {
    /// Derive every statically attributable response edge for one host operation.
    ///
    /// The outer `Result` is plan integrity. The inner `Result` is the SSA
    /// feasibility trichotomy's dynamic/ambiguous arm and carries the exact
    /// producer edge that prevents compile-time specialization.
    fn static_response_feasibility_ledger_filtered(
        &self,
        operation: Option<HostOpV1>,
    ) -> Result<Result<Vec<StaticResponseContinuation>, SsaInfeasible>, CraneliftBackendError> {
        let routes = host_response_routes(self)?;
        let mut rows = Vec::new();
        for unit in self.continuation_units()? {
            let vis_origin = unit.producer_construct_origin();
            let RuntimeExpr::Construct { constructor, args } =
                self.planned_occurrence_expr(vis_origin)?
            else {
                continue;
            };
            if !constructor.as_str().ends_with("::ITree::Vis") || args.len() != 2 {
                continue;
            }
            let operation_origin = self.semantic.child_origin(vis_origin, 0)?;
            let Some(route) = selected_host_response_route(self, operation_origin, &routes)? else {
                continue;
            };
            if operation.is_some_and(|operation| route.operation != operation) {
                continue;
            }
            let base_owner = ContinuationEmissionOwner::Specialization(unit.id());
            let Some(context) =
                self.continuation_context_for(unit.id(), unit.worker_body_origin())?
            else {
                return Ok(Err(SsaInfeasible::at_vis(
                    base_owner,
                    vis_origin,
                    Some(route.producer_call_origin),
                    "the statically selected K has no generated context target",
                )
                .with_k(
                    route.operation,
                    unit.worker_closure_origin(),
                    unit.worker_body_origin(),
                    unit.worker_capture_count(),
                    unit.continuation_inputs()?.len(),
                )));
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
                    return Ok(Err(SsaInfeasible::at_vis(
                        base_owner,
                        vis_origin,
                        Some(route.producer_call_origin),
                        "a seeded K capture has no explicit producer-side source coordinate",
                    )));
                };
                let source = match exact_capture_source(self, *owner, *origin)? {
                    Ok(source) => source,
                    Err(reason) => {
                        return Ok(Err(SsaInfeasible::at_vis(
                            base_owner,
                            vis_origin,
                            Some(route.producer_call_origin),
                            reason,
                        )));
                    }
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
            let continuation_inputs = unit
                .continuation_inputs()?
                .into_iter()
                .map(|input| (input.ordinal, input.coordinate, input.ordinary_abi_position))
                .collect();
            rows.push(StaticResponseContinuation {
                id: StaticResponseContinuationId::from_position(rows.len())?,
                base_owner,
                producer_call_origin: route.producer_call_origin,
                response_origin: route.response_origin,
                vis_origin,
                k_identity,
                k_closure_origin: unit.worker_closure_origin(),
                k_body_origin: unit.worker_body_origin(),
                k_context: context.id(),
                captures,
                continuation_inputs,
            });
        }
        rows.sort_by_key(|row| {
            (
                row.producer_call_origin,
                row.vis_origin,
                row.k_identity.clone(),
            )
        });
        for (position, row) in rows.iter_mut().enumerate() {
            row.id = StaticResponseContinuationId::from_position(position)?;
        }
        Ok(Ok(rows))
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
}
