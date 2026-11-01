//! Returned-Vis provenance and settlement for generated response owners.
//!
//! This analysis reads planned source occurrences and generated call edges. It
//! does not install a response row: an admitted relay consumed by static
//! producer fusion leaves the returned set, while independent static
//! successors retain their rows.
//! A constructor tag is not an occurrence identity, and a locally emitted Vis
//! is not necessarily a returned Vis.

use std::collections::{BTreeMap, BTreeSet};

use super::continuations::ContinuationContextId;
use super::occurrences::StaticOriginId;
use super::continuations::{ContinuationSpecializationId, StaticContinuationFusionId};
use super::responses::{StaticResponseContinuation, StaticResponseContinuationId};
use super::units::{EmittableCallEdge, EmittableCallKind, EmittableUnit, PredeclaredFunctionId};
use super::{planner_capacity_error, planner_error, CraneliftBackendError, StaticTransitionPlan};
use crate::RuntimeExpr;

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static FORCED_UNDERIVED_OWNER_VIS: std::cell::Cell<Option<u32>> =
        const { std::cell::Cell::new(None) };
}

/// Test-only: `returned_vis_protocol` fails for exactly one installed owner
/// Vis origin, at the protocol's single producer, so every reader sees what a
/// genuinely underivable owner produces. The program IR is unchanged.
#[cfg(feature = "px8-ds-test-support")]
pub fn with_forced_underived_owner<T>(owner_vis: u32, operation: impl FnOnce() -> T) -> T {
    FORCED_UNDERIVED_OWNER_VIS.with(|slot| {
        assert!(slot.get().is_none(), "forced Underived owners cannot nest");
        slot.set(Some(owner_vis));
    });
    let result = operation();
    FORCED_UNDERIVED_OWNER_VIS.with(|slot| slot.set(None));
    result
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ReturnedVisMember {
    pub(in crate::cranelift_backend) origin: StaticOriginId,
    pub(in crate::cranelift_backend) successor: Option<StaticResponseContinuation>,
    pub(in crate::cranelift_backend) bracket: Option<super::brackets::BracketSettlementId>,
    pub(in crate::cranelift_backend) installed: bool,
    pub(in crate::cranelift_backend) relay: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ReturnedVisContext {
    pub(in crate::cranelift_backend) context: ContinuationContextId,
    pub(in crate::cranelift_backend) members: Vec<ReturnedVisMember>,
    /// Actual generated-call targets visited on this context's returned-result
    /// route. A raw unit that is not on the route cannot acquire a record tail.
    pub(in crate::cranelift_backend) returning_units: BTreeSet<PredeclaredFunctionId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ReturnedVisProtocol {
    pub(in crate::cranelift_backend) owner: StaticOriginId,
    pub(in crate::cranelift_backend) contexts: Vec<ReturnedVisContext>,
    /// No owner may use a partial protocol when any returned member is a relay.
    pub(in crate::cranelift_backend) excluded_by_relay: bool,
}

/// A checked, program-local pending-return domain. Each member uses the
/// *existing response row* as its only identity/ABI/effect authority. The
/// discriminant is the row id plus one; zero is never a Vis member. No record
/// exists for an unadmitted relay or an unknown returned-result endpoint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PendingVisRecordProtocol {
    pub(in crate::cranelift_backend) owners:
        BTreeMap<StaticResponseContinuationId, ReturnedVisProtocol>,
    pub(in crate::cranelift_backend) members:
        BTreeMap<StaticOriginId, StaticResponseContinuation>,
    /// Each function's maximum over only the owner protocols whose return
    /// paths pass through that exact generated function. Absent means no tail.
    pub(in crate::cranelift_backend) frames: BTreeMap<PendingVisFrameOwner, PendingVisFrameWidth>,
}

/// Every installed response owner's settlement route, built once from the
/// final plan. No owner is absent; no consumer reads it through a `_` arm.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ResponseOwnerSettlement {
    /// A member of the pending-Vis record protocol after admitted relays
    /// are projected out of its returned set.
    Protocol,
    /// The K route returns no Vis member, so a Ret-only body is exact.
    RetOnly,
    /// The K route returns a Vis the record protocol cannot settle.
    Excluded { reason: ExcludedSettlement },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ExcludedSettlement {
    /// At least one unsettled returned member forwards a pattern-bound operation.
    Relay,
    /// The census errored while deriving a returned member (its text).
    Underived(String),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PendingVisSettlements {
    pub(in crate::cranelift_backend) protocol: Option<PendingVisRecordProtocol>,
    pub(in crate::cranelift_backend) owners:
        BTreeMap<StaticResponseContinuationId, ResponseOwnerSettlement>,
    /// Returned T1 settlement exits beside each owner's ordinary settlement.
    pub(in crate::cranelift_backend) bracket_exits:
        BTreeMap<StaticResponseContinuationId, BTreeSet<super::brackets::BracketSettlementId>>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum PendingVisFrameOwner {
    Unit(PredeclaredFunctionId),
    Context(ContinuationContextId),
    ResponseOwner(StaticResponseContinuationId),
    Continuation(ContinuationSpecializationId),
    Fusion(StaticContinuationFusionId),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PendingVisFrameWidth {
    pub(in crate::cranelift_backend) captures: u32,
    pub(in crate::cranelift_backend) continuation_inputs: u32,
}

/// The tail of a caller-owned activation frame. Its shape is projected once
/// from the closed successor population, never from a constructor tag or an
/// emitter-local count. The caller allocates it; the callee writes it before
/// returning and the caller copies it into its own frame before its next call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PendingVisFrameRegion {
    pub(in crate::cranelift_backend) discriminant: i32,
    pub(in crate::cranelift_backend) captures: i32,
    pub(in crate::cranelift_backend) continuation_inputs: i32,
    pub(in crate::cranelift_backend) frame_bytes: u32,
}

impl PendingVisRecordProtocol {
    pub(in crate::cranelift_backend) fn frame_region(
        &self,
        frame: PendingVisFrameOwner,
        base_bytes: u32,
    ) -> Result<Option<PendingVisFrameRegion>, CraneliftBackendError> {
        let Some(width) = self.frames.get(&frame) else {
            return Ok(None);
        };
        let bytes = |words: u32| -> Result<u32, CraneliftBackendError> {
            words.checked_mul(8).ok_or_else(|| planner_capacity_error("pending-Vis frame word count exhausted"))
        };
        let captures = base_bytes.checked_add(bytes(1)?)
            .ok_or_else(|| planner_capacity_error("pending-Vis discriminant offset exhausted"))?;
        let continuation_inputs = captures.checked_add(bytes(width.captures)?)
            .ok_or_else(|| planner_capacity_error("pending-Vis capture offset exhausted"))?;
        let frame_bytes = continuation_inputs.checked_add(bytes(width.continuation_inputs)?)
            .ok_or_else(|| planner_capacity_error("pending-Vis frame size exhausted"))?;
        let addressable = |offset: u32| i32::try_from(offset)
            .map_err(|_| planner_capacity_error("pending-Vis frame offset exceeds addressable range"));
        Ok(Some(PendingVisFrameRegion {
            discriminant: addressable(base_bytes)?,
            captures: addressable(captures)?,
            continuation_inputs: addressable(continuation_inputs)?,
            frame_bytes,
        }))
    }
}

impl StaticTransitionPlan<'_> {
    /// An admitted owner's forwarded relay is consumed by source fusion. Its
    /// K must never construct that relay in the same emission owner.
    pub(in crate::cranelift_backend) fn reject_reached_admitted_relay(
        &self,
        origin: StaticOriginId,
        constructor: &str,
        args: &[RuntimeExpr],
        emission_owner: Option<super::continuations::ContinuationEmissionOwner>,
    ) -> Result<(), CraneliftBackendError> {
        let Some(emission_owner) = emission_owner else { return Ok(()); };
        // The reached Construct and its arguments are already the lowering
        // operand. Do not re-resolve its source occurrence behind the single
        // origin-to-expression route merely to inspect this shape.
        if constructor != self.process_symbols.itree_vis.as_str()
            || args.len() != 2
            || !matches!(args.first(), Some(RuntimeExpr::Var(_)))
        {
            return Ok(());
        }
        for owner in &self.static_response_continuations {
            if owner.base_owner() != emission_owner
                || !matches!(self.pending_vis_settlements.owners.get(&owner.id()),
                    Some(ResponseOwnerSettlement::Protocol | ResponseOwnerSettlement::RetOnly))
            {
                continue;
            }
            let protocol = self.returned_vis_protocol(owner.vis_origin())?;
            if protocol.contexts.iter().flat_map(|context| &context.members)
                .any(|member| member.relay && member.origin == origin)
            {
                return Err(planner_error(format!(
                    "an admitted response owner {:?} reached relay construct {origin:?} despite source fusion",
                    owner.id(),
                )));
            }
        }
        Ok(())
    }

    /// Classify every installed response owner. A complete relay-bearing
    /// fixpoint may activate the record only after the shared source-pair
    /// authority admits it; a missing or unsettled relay stays excluded.
    pub(in crate::cranelift_backend) fn build_pending_vis_settlements(
        &self,
    ) -> Result<PendingVisSettlements, CraneliftBackendError> {
        let admitted_relays = super::responses::settled_relay_owners(self);
        let mut settlements = BTreeMap::new();
        let mut bracket_exits = BTreeMap::new();
        let mut owners = BTreeMap::new();
        let mut members: BTreeMap<StaticOriginId, StaticResponseContinuation> = BTreeMap::new();
        let mut frames: BTreeMap<PendingVisFrameOwner, PendingVisFrameWidth> = BTreeMap::new();
        for owner in &self.static_response_continuations {
            let protocol = match self.returned_vis_protocol(owner.vis_origin()) {
                Ok(protocol) => protocol,
                Err(error) => {
                    settlements.insert(owner.id(), ResponseOwnerSettlement::Excluded {
                        reason: ExcludedSettlement::Underived(error.to_string()),
                    });
                    continue;
                }
            };
            let mut protocol = protocol;
            let exits = protocol.contexts.iter().flat_map(|context| &context.members)
                .filter_map(|member| member.bracket)
                .collect::<BTreeSet<_>>();
            if !exits.is_empty() {
                for context in &mut protocol.contexts {
                    context.members.retain(|member| member.bracket.is_none());
                }
                bracket_exits.insert(owner.id(), exits);
            }
            // Source fusion consumes an admitted relay before this K returns.
            // Retain every non-relay successor and its existing exact row;
            // only an admitted owner's relay members leave the returned set.
            if protocol.excluded_by_relay && admitted_relays.contains(&owner.id()) {
                for context in &mut protocol.contexts {
                    context.members.retain(|member| !member.relay);
                }
                protocol.excluded_by_relay = false;
            }
            if protocol.contexts.iter().all(|ctx| ctx.members.is_empty()) {
                settlements.insert(owner.id(), ResponseOwnerSettlement::RetOnly);
                continue;
            }
            if protocol.excluded_by_relay {
                settlements.insert(owner.id(), ResponseOwnerSettlement::Excluded {
                    reason: ExcludedSettlement::Relay,
                });
                continue;
            }
            let mut width = PendingVisFrameWidth {
                captures: 0,
                continuation_inputs: 0,
            };
            for member in protocol.contexts.iter().flat_map(|context| &context.members) {
                let row = member.successor.as_ref().ok_or_else(|| {
                    planner_error("a pending-Vis member has no exact successor response row")
                })?;
                if member.relay {
                    return Err(planner_error("a relay entered a non-relay pending-Vis protocol"));
                }
                if row.id().ordinal() == u32::MAX {
                    return Err(planner_capacity_error("pending-Vis discriminant exhausted"));
                }
                width.captures = width.captures.max(u32::try_from(row.captures().len())
                    .map_err(|_| planner_capacity_error("pending-Vis capture run exhausted"))?);
                width.continuation_inputs = width.continuation_inputs.max(
                    u32::try_from(row.continuation_inputs().len())
                        .map_err(|_| planner_capacity_error("pending-Vis continuation input run exhausted"))?
                );
                if let Some(previous) = members.insert(member.origin, row.clone()) {
                    if previous.id() != row.id() {
                        return Err(planner_error("a pending-Vis occurrence has conflicting response-row identities"));
                    }
                }
            }
            // The owner calls its K contexts. Source-result Call edges mark
            // exact raw units; detached continuation grafts need their own
            // continuation frame to carry a member minted below the worker
            // back to the owner. At this construction phase fusion is not yet
            // installed, so this pre-fusion population deliberately marks a
            // superset. It never selects or emits a response member.
            let member_origins = protocol.contexts.iter()
                .flat_map(|context| &context.members)
                .map(|member| member.origin)
                .collect::<BTreeSet<_>>();
            let graft_frames = self.prefusion_continuation_result_edges()?
                .into_iter()
                .filter(|edge| member_origins.contains(&edge.producer_construct_origin))
                .map(|edge| PendingVisFrameOwner::Continuation(edge.identity.target()))
                .collect::<BTreeSet<_>>();
            let chain = std::iter::once(PendingVisFrameOwner::ResponseOwner(owner.id()))
                .chain(protocol.contexts.iter().map(|context|
                    PendingVisFrameOwner::Context(context.context)))
                .chain(protocol.contexts.iter().flat_map(|context|
                    context.returning_units.iter().copied().map(PendingVisFrameOwner::Unit)))
                .chain(graft_frames);
            for frame in chain {
                let entry = frames.entry(frame).or_insert(width);
                entry.captures = entry.captures.max(width.captures);
                entry.continuation_inputs = entry.continuation_inputs.max(width.continuation_inputs);
            }
            settlements.insert(owner.id(), ResponseOwnerSettlement::Protocol);
            owners.insert(owner.id(), protocol);
        }
        Ok(PendingVisSettlements {
            protocol: (!owners.is_empty()).then_some(PendingVisRecordProtocol {
                owners,
                members,
                frames,
            }),
            owners: settlements,
            bracket_exits,
        })
    }

    /// The only permitted case exclusion: the *same* planned-result traversal
    /// proves the scrutinee's COMPLETE terminal inventory to be constructors.
    /// Unknown results keep all cases live rather than inventing deadness.
    fn returned_vis_scrutinee_constructors(
        &self,
        origin: StaticOriginId,
    ) -> Result<Option<BTreeSet<String>>, CraneliftBackendError> {
        let mut constructors = BTreeSet::new();
        for result in self.source_result_origins_in_owner_subtree(origin)? {
            match self.source_occurrence(result)? {
                RuntimeExpr::Construct { constructor, .. } => {
                    constructors.insert(constructor.as_str().to_owned());
                }
                RuntimeExpr::Call { .. }
                | RuntimeExpr::Value(_)
                | RuntimeExpr::Var(_)
                | RuntimeExpr::Effect { .. }
                | RuntimeExpr::Project { .. }
                | RuntimeExpr::PrimitiveCall { .. }
                | RuntimeExpr::DeclarationRef { .. }
                | RuntimeExpr::ImportedDeclarationRef { .. }
                | RuntimeExpr::Closure { .. }
                | RuntimeExpr::LexicalClosure { .. }
                | RuntimeExpr::Record { .. } => return Ok(None),
                RuntimeExpr::Trap(_) => {}
                RuntimeExpr::CheckedBracketFinalizer { .. }
                | RuntimeExpr::CheckedJoinSite { .. }
                | RuntimeExpr::CheckedSubcontinuationFrame { .. }
                | RuntimeExpr::CheckedRecursiveInvocation { .. }
                | RuntimeExpr::CheckedComputationalIHSlots { .. }
                | RuntimeExpr::CheckedComputationalIHInvocation { .. }
                | RuntimeExpr::Let { .. }
                | RuntimeExpr::If { .. }
                | RuntimeExpr::Match { .. }
                | RuntimeExpr::ComputationalMatch { .. } => {}
            }
        }
        Ok((!constructors.is_empty()).then_some(constructors))
    }

    /// A monotone walk of terminal source occurrences and forwarded generated
    /// results. Vis origins are minted at their actual Construct occurrence,
    /// never recovered from a merged ResultWord or its constructor tag.
    fn returned_vis_in_context(
        &self,
        body: StaticOriginId,
        units: &[EmittableUnit<'_>],
        edges: &[EmittableCallEdge],
    ) -> Result<(BTreeSet<StaticOriginId>, BTreeSet<PredeclaredFunctionId>), CraneliftBackendError> {
        let mut pending = vec![body];
        let mut seen = BTreeSet::new();
        let mut returned = BTreeSet::new();
        let mut returning_units = BTreeSet::new();
        while let Some(origin) = pending.pop() {
            if !seen.insert(origin) {
                continue;
            }
            let child = |position| self.child_static_origin(origin, position);
            match self.source_occurrence(origin)? {
                RuntimeExpr::CheckedBracketFinalizer { .. }
                | RuntimeExpr::CheckedJoinSite { .. }
                | RuntimeExpr::CheckedSubcontinuationFrame { .. }
                | RuntimeExpr::CheckedRecursiveInvocation { .. }
                | RuntimeExpr::CheckedComputationalIHSlots { .. }
                | RuntimeExpr::CheckedComputationalIHInvocation { .. } => pending.push(child(0)?),
                RuntimeExpr::Let { .. } => pending.push(child(1)?),
                RuntimeExpr::If { .. } => {
                    pending.push(child(1)?);
                    pending.push(child(2)?);
                }
                RuntimeExpr::Match { cases, .. } => {
                    for index in 0..cases.len() {
                        pending.push(child(1 + index)?);
                    }
                }
                RuntimeExpr::ComputationalMatch { cases, .. } => {
                    let possible = self.returned_vis_scrutinee_constructors(child(0)?)?;
                    for (index, case) in cases.iter().enumerate() {
                        if possible
                            .as_ref()
                            .is_none_or(|names| names.contains(case.constructor.as_str()))
                        {
                            pending.push(child(1 + index)?);
                        }
                    }
                }
                RuntimeExpr::Construct { constructor, .. }
                    if constructor == &self.process_symbols.itree_vis =>
                {
                    returned.insert(origin);
                }
                RuntimeExpr::Call { .. } => {
                    let callee = child(0)?;
                    // A direct lexical-closure call is keyed on its BODY
                    // occurrence by the emitted StaticBody edge, not on the
                    // enclosing closure-expression occurrence. A declaration
                    // call is keyed on its DeclarationRef occurrence instead.
                    let call_site = match self.source_occurrence(callee)? {
                        RuntimeExpr::Closure { .. } | RuntimeExpr::LexicalClosure { .. } => {
                            self.child_static_origin(callee, 0)?
                        }
                        _ => callee,
                    };
                    let targets = edges
                        .iter()
                        .filter(|edge| {
                            edge.call_site_origin() == call_site
                                && (edge.callee_origin() == call_site
                                    || edge.kind() == EmittableCallKind::Declaration)
                        })
                        .map(|edge| edge.callee())
                        .collect::<BTreeSet<_>>();
                    if targets.is_empty() {
                        return Err(planner_error(format!(
                            "returned-Vis generated Call at {origin:?} has an unknown endpoint",
                        )));
                    }
                    for target in targets {
                        returning_units.insert(target);
                        let body = units.iter().find(|unit| unit.function() == target)
                            .ok_or_else(|| planner_error(format!(
                                "returned-Vis Call at {origin:?} has no generated target {target:?}",
                            )))?.body_occurrence();
                        pending.push(body);
                    }
                }
                RuntimeExpr::Construct { .. } | RuntimeExpr::Trap(_) => {}
                RuntimeExpr::Value(_)
                | RuntimeExpr::Var(_)
                | RuntimeExpr::Effect { .. }
                | RuntimeExpr::PrimitiveCall { .. }
                | RuntimeExpr::Record { .. }
                | RuntimeExpr::Project { .. }
                | RuntimeExpr::Closure { .. }
                | RuntimeExpr::LexicalClosure { .. }
                | RuntimeExpr::DeclarationRef { .. }
                | RuntimeExpr::ImportedDeclarationRef { .. } => {
                    return Err(planner_error(format!(
                        "returned-Vis context has an unclassified result at {origin:?}",
                    )));
                }
            }
        }
        Ok((returned, returning_units))
    }

    /// A returned Vis with no ordinary successor row is the plane's T1 edge
    /// (BodyExit -> Settlement) iff exactly one bracket instance settles at
    /// it and the returning owner is that instance's acquire or lies in its
    /// Body. Anything else stays underived; no ordinary row is consulted.
    fn returned_bracket_settlement(
        &self,
        from: StaticOriginId,
        origin: StaticOriginId,
    ) -> Result<Option<super::brackets::BracketSettlementId>, CraneliftBackendError> {
        let mut instances = self.bracket_settlements.regions.iter()
            .filter(|region| region.settlement == origin);
        let Some(region) = instances.next() else { return Ok(None) };
        if instances.next().is_some() {
            return Err(planner_error(format!(
                "returned Vis {origin:?} names two bracket settlement instances",
            )));
        }
        if from != region.acquire && !region.body.contains(&from) {
            return Err(planner_error(format!(
                "returned bracket settlement {origin:?} is reached from outside its region",
            )));
        }
        #[cfg(feature = "px8-ds-test-support")]
        super::brackets::record_region_exit_owner(
            super::brackets::RegionExitPlane::ReturnedVisSettlement, origin, from,
        );
        Ok(Some(region.id))
    }

    /// Resolve a response owner's complete returned-Vis K-context fixpoint.
    /// A relay makes the WHOLE owner ineligible; there is no partial route.
    /// Derived successors have the existing response-row type and are not
    /// installed into the planner's live row population in this increment.
    pub(in crate::cranelift_backend) fn returned_vis_protocol(
        &self,
        owner_vis: StaticOriginId,
    ) -> Result<ReturnedVisProtocol, CraneliftBackendError> {
        let owner = self
            .static_response_continuations
            .iter()
            .find(|row| row.vis_origin() == owner_vis)
            .ok_or_else(|| planner_error("returned-Vis protocol names no installed owner row"))?;
        #[cfg(feature = "px8-ds-test-support")]
        if FORCED_UNDERIVED_OWNER_VIS.with(std::cell::Cell::get) == Some(owner_vis.0) {
            return Err(planner_error(
                "test-forced underivable returned-Vis protocol",
            ));
        }
        let candidate_rows = self.return_protocol_candidate_rows()?;
        let contexts = self.continuation_contexts()?;
        let units = self.emittable_units()?;
        let edges = self.emittable_call_edges()?;
        // Each pending context carries the Vis whose response continuation
        // it is: that Vis, not the protocol's root owner, returns a member.
        let mut pending = vec![(owner.k_context(), owner.vis_origin())];
        let mut seen = BTreeSet::new();
        let mut results = Vec::new();
        let mut excluded_by_relay = false;
        while let Some((context_id, returning)) = pending.pop() {
            if !seen.insert(context_id) {
                continue;
            }
            let context = contexts
                .iter()
                .find(|context| context.id() == context_id)
                .ok_or_else(|| {
                    planner_error("returned-Vis successor has no generated K context")
                })?;
            let (origins, returning_units) =
                self.returned_vis_in_context(context.worker_body_origin(), &units, &edges)?;
            let mut members = Vec::new();
            for origin in origins {
                let RuntimeExpr::Construct { args, .. } = self.source_occurrence(origin)? else {
                    return Err(planner_error("returned-Vis origin is not a constructor"));
                };
                if args.len() != 2 {
                    return Err(planner_error(
                        "returned-Vis construct does not have two fields",
                    ));
                }
                let operation = self.child_static_origin(origin, 0)?;
                let relay = matches!(self.source_occurrence(operation)?, RuntimeExpr::Var(_));
                if relay {
                    excluded_by_relay = true;
                }
                let installed = self
                    .static_response_continuations
                    .iter()
                    .any(|row| row.vis_origin() == origin);
                let matches = candidate_rows
                    .iter()
                    .filter(|row| row.vis_origin() == origin)
                    .collect::<Vec<_>>();
                if matches.len() > 1 {
                    return Err(planner_error(
                        "returned Vis has ambiguous successor response rows",
                    ));
                }
                let successor = matches.first().map(|row| (*row).clone());
                let bracket = if relay || successor.is_some() { None } else {
                    self.returned_bracket_settlement(returning, origin)?
                };
                if !relay && bracket.is_none() {
                    if !matches!(
                        self.source_occurrence(operation)?,
                        RuntimeExpr::Construct { .. }
                    ) {
                        return Err(planner_error(format!(
                            "returned Vis {origin:?} has no static operation constructor",
                        )));
                    }
                    let row = successor.as_ref().ok_or_else(|| {
                        planner_error(format!(
                            "returned Vis {origin:?} has no derivable successor response row",
                        ))
                    })?;
                    if !self.host_effect_seat_records().iter().any(|seat| {
                        seat.effect_origin == row.effect_origin()
                            && seat.operation == row.operation()
                    }) {
                        return Err(planner_error(format!(
                            "returned Vis {origin:?} has no planned host effect seat",
                        )));
                    }
                    pending.push((row.k_context(), row.vis_origin()));
                }
                members.push(ReturnedVisMember {
                    origin,
                    successor,
                    bracket,
                    installed,
                    relay,
                });
            }
            results.push(ReturnedVisContext {
                context: context_id,
                members,
                returning_units,
            });
        }
        results.sort_by_key(|context| context.context);
        Ok(ReturnedVisProtocol {
            owner: owner_vis,
            contexts: results,
            excluded_by_relay,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cranelift_backend::planning::{
        plan_static_transition_graph_with_symbols, AbiRootIngress,
    };
    use crate::{NativeProcessSymbols, RuntimeValue};
    use std::collections::BTreeMap;

    /// Promise class: normative compatibility vector for the N0002 capacity
    /// family. Each reachable arithmetic guard sees its immediate predecessor,
    /// limit, first excess, empty input, and a farther excess. Every frame is
    /// present. At the four u32 arithmetic ceilings, an input that passes that
    /// guard may still fail a later addition or the i32 addressability ceiling;
    /// labels explain that ordering without pinning unstable error prose.
    #[test]
    fn pending_vis_frame_region_reports_capacity_at_each_width_guard() {
        let frame = PendingVisFrameOwner::Unit(PredeclaredFunctionId::for_test(0));
        let region = |base_bytes, captures, continuation_inputs| {
            let protocol = PendingVisRecordProtocol {
                owners: BTreeMap::new(),
                members: BTreeMap::new(),
                frames: BTreeMap::from([(
                    frame,
                    PendingVisFrameWidth {
                        captures,
                        continuation_inputs,
                    },
                )]),
            };
            protocol.frame_region(frame, base_bytes)
        };
        let fits = |label, base_bytes, captures, continuation_inputs| {
            assert!(
                region(base_bytes, captures, continuation_inputs)
                    .unwrap_or_else(|error| panic!("{label} must fit: {error:?}"))
                    .is_some(),
                "{label}: the frame must exist"
            );
        };
        let refuses_at = |label, base_bytes, captures, continuation_inputs, guard: &str| {
            let error = region(base_bytes, captures, continuation_inputs)
                .expect_err("the planned width must refuse");
            assert_eq!(error.code().as_str(), "N0002", "{label}, expected {guard}: {error:?}");
            let CraneliftBackendError::Unsupported(error) = error else {
                panic!("{label}: capacity must retain the Unsupported carrier");
            };
            assert_eq!(
                error.cause,
                crate::cranelift_backend::surface::LoweringCause::Capacity,
                "{label}: the carrier's cause must be capacity"
            );
            // The guard label above names the arithmetic expectation, not an
            // assertion about unstable refusal text. An individual cause
            // revert establishes which site's first-excess case is reached.
        };

        fits("empty frame", 0, 0, 0);
        let max_words = u32::MAX / 8;
        // The first neighbor below the checked-multiply ceiling fits the
        // whole frame; the ceiling itself passes multiplication but exceeds
        // the later frame-size addition.
        fits("word limit - 1", 0, 0, max_words - 1);
        refuses_at("word limit", 0, 0, max_words,
            "pending-Vis frame size exhausted");
        refuses_at("word limit + 1", 0, 0, max_words + 1,
            "pending-Vis frame word count exhausted");
        refuses_at("word oversized", 0, 0, u32::MAX,
            "pending-Vis frame word count exhausted");

        let max_discriminant_base = u32::MAX - 8;
        refuses_at("discriminant limit - 1", max_discriminant_base - 1, 0, 0,
            "pending-Vis frame offset exceeds addressable range");
        refuses_at("discriminant limit", max_discriminant_base, 0, 0,
            "pending-Vis frame offset exceeds addressable range");
        refuses_at("discriminant limit + 1", max_discriminant_base + 1, 0, 0,
            "pending-Vis discriminant offset exhausted");
        refuses_at("discriminant oversized", u32::MAX, 0, 0,
            "pending-Vis discriminant offset exhausted");

        fits("two captured words", 0, 2, 0);
        let max_capture_base = u32::MAX - 8 - 2 * 8;
        refuses_at("capture limit - 1", max_capture_base - 1, 2, 0,
            "pending-Vis frame offset exceeds addressable range");
        refuses_at("capture limit", max_capture_base, 2, 0,
            "pending-Vis frame offset exceeds addressable range");
        refuses_at("capture limit + 1", max_capture_base + 1, 2, 0,
            "pending-Vis capture offset exhausted");
        refuses_at("capture oversized", max_capture_base + 8, 2, 0,
            "pending-Vis capture offset exhausted");

        fits("empty continuation run", 0, 1, 0);
        let max_continuation_words = (u32::MAX - 16) / 8;
        fits("frame size limit - 1", 0, 1, max_continuation_words - 1);
        fits("frame size limit", 0, 1, max_continuation_words);
        refuses_at("frame size limit + 1", 0, 1, max_continuation_words + 1,
            "pending-Vis frame size exhausted");
        refuses_at("frame size oversized", 0, 1, max_words,
            "pending-Vis frame size exhausted");

        let max_addressable_base = (i32::MAX as u32) - 8;
        fits("addressable limit - 1", max_addressable_base - 1, 0, 0);
        fits("addressable limit", max_addressable_base, 0, 0);
        refuses_at("addressable limit + 1", max_addressable_base + 1, 0, 0,
            "pending-Vis frame offset exceeds addressable range");
        refuses_at("addressable oversized", (i32::MAX as u32) + 1, 0, 0,
            "pending-Vis frame offset exceeds addressable range");
    }

    /// Promise class: durable invariant. The outer result is a generated
    /// call, not a Vis site. Its callee's returned construct is the only Vis
    /// source. Omitting forwarded call edges must make this pin red; omitting
    /// the edge from the plan altogether must refuse an unknown endpoint.
    #[test]
    fn returned_vis_from_generated_call_is_the_callees_occurrence() {
        let vis = RuntimeExpr::Construct {
            constructor: "ctor:fixture::ITree::Vis".to_string(),
            args: vec![
                RuntimeExpr::Construct {
                    constructor: "ctor:fixture::Op::Request".to_string(),
                    args: vec![],
                },
                RuntimeExpr::Value(RuntimeValue::Unknown),
            ],
        };
        let entry = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: vec![],
                params: vec![],
                body: Box::new(vis),
            }),
            args: vec![],
        };
        let mut symbols = NativeProcessSymbols::legacy_prelude();
        symbols.itree_vis = "ctor:fixture::ITree::Vis".to_string();
        let plan = plan_static_transition_graph_with_symbols(
            &entry,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Value,
            true,
        )
        .expect("closed generated-call fixture plans");
        let units = plan.emittable_units().expect("generated unit population");
        let edges = plan
            .emittable_call_edges()
            .expect("generated call-edge population");
        let root = plan.root_static_origin().expect("call occurrence");
        let callee_body = units
            .iter()
            .find(|unit| unit.body_occurrence() != root)
            .expect("the callee has a distinct source body")
            .body_occurrence();
        let observed = plan
            .returned_vis_in_context(root, &units, &edges)
            .expect("the generated result is forwarded");
        assert_eq!(observed.0, BTreeSet::from([callee_body]));
        assert_eq!(observed.1.len(), 1, "the returned unit call is on the chain");
        assert!(plan
            .returned_vis_in_context(root, &units, &[])
            .expect_err("a missing generated edge must refuse")
            .to_string()
            .contains("unknown endpoint"));
    }
}
