//! Compile-time response-producer to static-continuation feasibility.
//!
//! This module derives the relation from both ends before any response-owner
//! Function exists. The producer half starts at a closed `ITree::Vis` operation
//! and resolves the matching host-response case. The continuation half starts
//! at the planner-issued continuation specialization and retains its worker,
//! context, captures, continuation inputs, and opaque call identity.

use std::collections::BTreeMap;

use super::continuations::{
    continuation_owner_entry_sources, generated_context_parameters,
    walk_continuation_value_environment, ContinuationCallIdentity, ContinuationContextId,
    ContinuationEmissionOwner, ContinuationInputProjection, ContinuationSourceCoordinate,
    ContinuationSpecializationId, ContinuationValueSourceAuthority,
    ContinuationWorkerCaptureSource, ContinuationWorkerProvenance, PlannedContinuationContext,
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

/// A response edge's request for the ordinary continuation context identified
/// by `(k_specialization, k_body_origin)`.
///
/// This record does not mint a context identity or describe a second ABI. Its
/// worker and continuation-input fields are cloned from the already-validated K
/// specialization solely so union interning can reject a same-key schema
/// disagreement before assigning an ordinary [`ContinuationContextId`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticResponseContextDemand {
    id: StaticResponseContinuationId,
    base_owner: ContinuationEmissionOwner,
    producer_call_origin: StaticOriginId,
    response_origin: StaticOriginId,
    vis_origin: StaticOriginId,
    operation: HostOpV1,
    k_identity: ContinuationCallIdentity,
    k_specialization: ContinuationSpecializationId,
    k_closure_origin: StaticOriginId,
    k_body_origin: StaticOriginId,
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
    vis_origin: StaticOriginId,
    operation: HostOpV1,
    k_identity: ContinuationCallIdentity,
    k_specialization: ContinuationSpecializationId,
    k_closure_origin: StaticOriginId,
    k_body_origin: StaticOriginId,
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

impl StaticTransitionPlan<'_> {
    /// Derive and fully validate every statically attributable response demand.
    ///
    /// This phase intentionally does not ask whether an old causal caller had
    /// already caused the K context to be interned. The response edge is itself
    /// a lawful context demand, so that question belongs to the union interner
    /// below. The outer `Result` is plan integrity; the inner `Result` retains
    /// only the SSA trichotomy's genuinely dynamic or non-expressible arm.
    fn static_response_context_demands_filtered(
        &self,
        operation: Option<HostOpV1>,
    ) -> Result<Result<Vec<StaticResponseContextDemand>, SsaInfeasible>, CraneliftBackendError>
    {
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
            let Some(route) = selected_host_response_route(self, operation_origin, &routes)? else {
                continue;
            };
            if operation.is_some_and(|operation| route.operation != operation) {
                continue;
            }
            response_vis.push((vis_origin, route));
        }
        response_vis.sort_by_key(|(vis_origin, route)| {
            (route.producer_call_origin, *vis_origin, route.operation)
        });

        let mut demands = Vec::new();
        for (vis_origin, route) in response_vis {
            let matching = units
                .iter()
                .filter(|unit| unit.producer_construct_origin() == vis_origin)
                .collect::<Vec<_>>();
            if matching.is_empty() {
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
            for unit in matching {
                let base_owner = ContinuationEmissionOwner::Specialization(unit.id());
                let continuation_inputs = unit.continuation_inputs()?;
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
                demands.push(StaticResponseContextDemand {
                    id: StaticResponseContinuationId::from_position(demands.len())?,
                    base_owner,
                    producer_call_origin: route.producer_call_origin,
                    response_origin: route.response_origin,
                    vis_origin,
                    operation: route.operation,
                    k_identity,
                    k_specialization: unit.id(),
                    k_closure_origin: unit.worker_closure_origin(),
                    k_body_origin: unit.worker_body_origin(),
                    raw_owner,
                    worker: unit.key.worker.clone(),
                    captures,
                    continuation_inputs: continuation_inputs
                        .into_iter()
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
        if operation.is_none() {
            if let Some(mutation) =
                STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION.with(std::cell::Cell::get)
            {
                let mut target = None;
                for (position, demand) in demands.iter().enumerate() {
                    if self
                        .continuation_context_for(demand.k_specialization, demand.k_body_origin)?
                        .is_none()
                    {
                        target = Some(position);
                        break;
                    }
                }
                let target = target.ok_or_else(|| {
                    planner_error(
                        "the response-demand mutation found no response-only context demand",
                    )
                })?;
                match mutation {
                    StaticResponseContextDemandMutation::DeleteResponseOnlyDemand => {
                        demands.remove(target);
                    }
                    StaticResponseContextDemandMutation::DuplicateResponseOnlyDemand => {
                        demands.insert(target + 1, demands[target].clone());
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
                }
                STATIC_RESPONSE_CONTEXT_DEMAND_MUTATION_APPLICATIONS
                    .with(|count| count.set(count.get() + 1));
            }
        }
        validate_static_response_demand_closure(&expected_demands, &demands)?;
        demands.sort_by_key(|demand| demand.id);
        demands.dedup();
        #[cfg(feature = "px8-ds-test-support")]
        if operation.is_none()
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
        Ok(Ok(demands))
    }

    /// Intern the union of old causal-call contexts and response demands into a
    /// scratch planner population. Existing planned contexts remain the exact
    /// prefix, so their identities and schemas cannot be renumbered by a
    /// response edge; the installed ABI descriptors on immutable `self` are
    /// untouched. The returned population is planner evidence only at CP1: it
    /// is not installed, so lowering cannot declare or enter a new Function.
    fn response_context_union(
        &self,
        demands: &[StaticResponseContextDemand],
    ) -> Result<(Vec<PlannedContinuationContext>, usize), CraneliftBackendError> {
        let mut contexts = self.continuation_contexts.clone();
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
        if contexts.get(..preexisting_count) != Some(self.continuation_contexts.as_slice()) {
            return Err(planner_error(
                "appending response context demands changed a causal context identity or schema",
            ));
        }
        Ok((contexts, preexisting_count))
    }

    fn resolve_static_response_context_demands(
        &self,
        demands: Vec<StaticResponseContextDemand>,
    ) -> Result<Vec<StaticResponseContinuation>, CraneliftBackendError> {
        let (contexts, preexisting_count) = self.response_context_union(&demands)?;
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
                vis_origin: demand.vis_origin,
                operation: demand.operation,
                k_identity: demand.k_identity,
                k_specialization: demand.k_specialization,
                k_closure_origin: demand.k_closure_origin,
                k_body_origin: demand.k_body_origin,
                k_context,
                context_was_preexisting: position < preexisting_count,
                captures: demand.captures,
                continuation_inputs: demand.continuation_inputs,
            });
        }
        Ok(resolved)
    }

    fn static_response_feasibility_ledger_filtered(
        &self,
        operation: Option<HostOpV1>,
    ) -> Result<Result<Vec<StaticResponseContinuation>, SsaInfeasible>, CraneliftBackendError> {
        let demands = match self.static_response_context_demands_filtered(operation)? {
            Ok(demands) => demands,
            Err(infeasible) => return Ok(Err(infeasible)),
        };
        Ok(Ok(self.resolve_static_response_context_demands(demands)?))
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
