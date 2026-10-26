//! Handler-owned Deferred drives use their own checked resumption domain.
//! They are neither causal specialization calls nor a second context interner.

use super::continuations::{
    ContinuationEmissionOwner, ContinuationSourceCoordinate,
};
use super::responses::{DeferredResponseRow, HandlerDriveContextRoute};
use super::{planner_capacity_error, planner_error, CraneliftBackendError,
    StaticOriginId, StaticTransitionPlan};
use crate::RuntimeExpr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct PlannedHandlerResumptionId(u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum PlannedResumptionParameter {
    Response { effect_origin: StaticOriginId },
    KCapture { source_origin: StaticOriginId, member: ContinuationSourceCoordinate },
}

/// Where K's body executes. Chosen by which owner lowers the handler-owned
/// responses K's body performs; the frame run is never recorded here, since
/// the lowering already threads K's continuation across named calls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum PlannedResumptionExecution {
    /// The target context's specialization owns every handler-owned response
    /// K's body performs, so K runs there and returns its raw result.
    Transfer,
    /// K's body performs a response its handler owns and the target context
    /// does not, so K runs in the handler's lowering under its live stack.
    AtHandler,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PlannedHandlerResumptionCall {
    pub(in crate::cranelift_backend) id: PlannedHandlerResumptionId,
    pub(in crate::cranelift_backend) row: (StaticOriginId, StaticOriginId, StaticOriginId),
    pub(in crate::cranelift_backend) emission_owner: ContinuationEmissionOwner,
    pub(in crate::cranelift_backend) route: HandlerDriveContextRoute,
    pub(in crate::cranelift_backend) parameters: Vec<PlannedResumptionParameter>,
    pub(in crate::cranelift_backend) execution: PlannedResumptionExecution,
}

impl StaticTransitionPlan<'_> {
    /// Publish only after response phase B fixes owners, transports and rows.
    pub(super) fn publish_handler_resumption_plan(&mut self)
        -> Result<(), CraneliftBackendError>
    {
        if !self.handler_resumptions.is_empty() {
            return Err(planner_error("handler resumptions were published twice"));
        }
        let mut published = Vec::new();
        for row in &self.static_response_deferred {
            let Some(owner) = self.admitted_deferred_handler_owner(row)? else {
                continue;
            };
            let Some(route) = self.handler_drive_context_route(row, owner)? else {
                continue;
            };
            let id = PlannedHandlerResumptionId(u32::try_from(published.len())
                .map_err(|_| planner_capacity_error("handler resumption ids exhausted"))?);
            published.push(self.derive_handler_resumption(row, owner, route, id)?);
        }
        self.handler_resumptions = published;
        Ok(())
    }

    fn derive_handler_resumption(
        &self, row: &DeferredResponseRow, owner: ContinuationEmissionOwner,
        route: HandlerDriveContextRoute, id: PlannedHandlerResumptionId,
    ) -> Result<PlannedHandlerResumptionCall, CraneliftBackendError> {
        let k_origin = self.semantic.child_origin(row.vis_origin(), 1)?;
        let RuntimeExpr::LexicalClosure { captures, .. } =
            self.planned_occurrence_expr(k_origin)? else {
                return Err(planner_error("a planned resumption does not have a lexical K"));
            };
        let members = self.continuation_context_parameter_members(route.context)?;
        if members.len() != captures.len() + 1 {
            return Err(planner_error("a planned resumption's K Parameter run disagrees with its checked context"));
        }
        let mut parameters = vec![PlannedResumptionParameter::Response {
            effect_origin: row.effect_origin(),
        }];
        for (position, member) in members.into_iter().enumerate().skip(1) {
            parameters.push(PlannedResumptionParameter::KCapture {
                source_origin: self.semantic.child_origin(k_origin, position)?, member,
            });
        }
        let mut parents = std::collections::BTreeMap::new();
        for occurrence in self.source_occurrences.iter().flatten() {
            for (position, child) in self.semantic.child_origins(occurrence.static_origin)?
                .iter().copied().enumerate()
            {
                if parents.insert(child, (occurrence.static_origin, position)).is_some() {
                    return Err(planner_error("resumption source occurrence has two parents"));
                }
            }
        }
        let target = ContinuationEmissionOwner::Specialization(
            self.continuation_contexts()?.into_iter()
                .find(|candidate| candidate.id() == route.context)
                .ok_or_else(|| planner_error("a planned resumption's context has no projected view"))?
                .enclosing_specialization(),
        );
        let (mut covered_by_target, mut covered_by_handler) = (true, true);
        for nested in &self.static_response_deferred {
            if nested.vis_origin() == row.vis_origin() {
                continue;
            }
            // The target lowers a response iff its Vis is reached from K's
            // body without crossing a closure, which another unit lowers.
            let mut cursor = nested.vis_origin();
            let mut lowered_in_body = cursor == route.worker_body_origin;
            while !lowered_in_body {
                let Some((parent, _)) = parents.get(&cursor).copied() else { break };
                if parent == route.worker_body_origin {
                    lowered_in_body = true;
                } else if matches!(self.planned_occurrence_expr(parent)?,
                    RuntimeExpr::LexicalClosure { .. })
                {
                    break;
                }
                cursor = parent;
            }
            if !lowered_in_body {
                continue;
            }
            let Some(nested_owner) = self.admitted_deferred_handler_owner(nested)? else {
                continue;
            };
            covered_by_target &= nested_owner == target;
            covered_by_handler &= nested_owner == owner;
        }
        let execution = if covered_by_target {
            PlannedResumptionExecution::Transfer
        } else if covered_by_handler {
            PlannedResumptionExecution::AtHandler
        } else {
            return Err(planner_error("a planned resumption's K performs a handler-owned response that neither its target context nor its handler owns"));
        };
        Ok(PlannedHandlerResumptionCall {
            id, row: (row.vis_origin(), row.operation_root_origin(), row.effect_origin()),
            emission_owner: owner, route, parameters, execution,
        })
    }

    pub(super) fn validate_handler_resumption_plan(&self)
        -> Result<(), CraneliftBackendError>
    {
        let owners = match self.static_response_owner_specializations()? {
            Ok(owners) => owners,
            // Phase A's installed typed refusal stays the plane's outcome; it
            // lowers nothing, so it publishes no resumption to validate.
            Err(infeasible) if self.static_response_infeasible.as_ref() == Some(&infeasible)
                && self.handler_resumptions.is_empty() => return Ok(()),
            Err(infeasible) => return Err(planner_error(infeasible.reason())),
        };
        let mut expected_rows = Vec::new();
        for row in &self.static_response_deferred {
            let Some(owner) = self.admitted_deferred_handler_owner(row)? else { continue };
            let Some(route) = self.handler_drive_context_route(row, owner)? else { continue };
            expected_rows.push((row, owner, route));
        }
        if self.handler_resumptions.len() != expected_rows.len() {
            return Err(planner_error("a handler-owned Deferred resumption row is missing"));
        }
        for (index, (record, (row, owner, route))) in
            self.handler_resumptions.iter().zip(expected_rows).enumerate()
        {
            if record.id != PlannedHandlerResumptionId(u32::try_from(index)
                    .map_err(|_| planner_capacity_error("handler resumption ids exhausted"))?)
                || record.row != (row.vis_origin(), row.operation_root_origin(), row.effect_origin())
                || record.emission_owner != owner || record.route != route
                || !owners.iter().any(|candidate| candidate.base_owner() == owner)
            {
                return Err(planner_error("handler resumption row, owner or context route disagrees with the checked response"));
            }
            let expected = self.derive_handler_resumption(row, owner, route,
                record.id)?;
            if record.parameters != expected.parameters {
                return Err(planner_error("resumption response or K-capture parameter is not certified by the checked context"));
            }
            if record.execution != expected.execution {
                return Err(planner_error("resumption execution site disagrees with its checked response ownership"));
            }
        }
        Ok(())
    }

    pub(in crate::cranelift_backend) fn handler_resumptions(
        &self,
    ) -> &[PlannedHandlerResumptionCall] {
        &self.handler_resumptions
    }

    pub(in crate::cranelift_backend) fn handler_resumption_at_vis(
        &self, vis_origin: StaticOriginId, owner: ContinuationEmissionOwner,
    ) -> Result<Option<&PlannedHandlerResumptionCall>, CraneliftBackendError> {
        let mut matching = self.handler_resumptions.iter()
            .filter(|record| record.row.0 == vis_origin && record.emission_owner == owner);
        let row = matching.next();
        if matching.next().is_some() {
            return Err(planner_error("two handler resumptions claim one Vis and owner"));
        }
        Ok(row)
    }
}
