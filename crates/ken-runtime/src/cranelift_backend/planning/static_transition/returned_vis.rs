//! Behavior-inert returned-Vis provenance for generated response owners.
//!
//! This analysis reads planned source occurrences and generated call edges; it
//! does not install a response row or change any emitted function. In particular,
//! a constructor tag is not an occurrence identity and a locally emitted Vis is
//! not necessarily a returned Vis.

use std::collections::BTreeSet;

use super::continuations::ContinuationContextId;
use super::occurrences::StaticOriginId;
use super::responses::StaticResponseContinuation;
use super::units::{EmittableCallEdge, EmittableCallKind, EmittableUnit};
use super::{planner_error, CraneliftBackendError, StaticTransitionPlan};
use crate::RuntimeExpr;

#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct ReturnedVisMember {
    pub(in crate::cranelift_backend) origin: StaticOriginId,
    pub(in crate::cranelift_backend) successor: Option<StaticResponseContinuation>,
    pub(in crate::cranelift_backend) installed: bool,
    pub(in crate::cranelift_backend) relay: bool,
}

#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct ReturnedVisContext {
    pub(in crate::cranelift_backend) context: ContinuationContextId,
    pub(in crate::cranelift_backend) members: Vec<ReturnedVisMember>,
}

#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct ReturnedVisProtocol {
    pub(in crate::cranelift_backend) owner: StaticOriginId,
    pub(in crate::cranelift_backend) contexts: Vec<ReturnedVisContext>,
    /// No owner may use a partial protocol when any returned member is a relay.
    pub(in crate::cranelift_backend) excluded_by_relay: bool,
}

impl StaticTransitionPlan<'_> {
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
                RuntimeExpr::CheckedJoinSite { .. }
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
    ) -> Result<BTreeSet<StaticOriginId>, CraneliftBackendError> {
        let mut pending = vec![body];
        let mut seen = BTreeSet::new();
        let mut returned = BTreeSet::new();
        while let Some(origin) = pending.pop() {
            if !seen.insert(origin) {
                continue;
            }
            let child = |position| self.child_static_origin(origin, position);
            match self.source_occurrence(origin)? {
                RuntimeExpr::CheckedJoinSite { .. }
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
                    if constructor.as_str().ends_with("::ITree::Vis") =>
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
        Ok(returned)
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
        let candidate_rows = self.return_protocol_candidate_rows()?;
        let contexts = self.continuation_contexts()?;
        let units = self.emittable_units()?;
        let edges = self.emittable_call_edges()?;
        let mut pending = vec![owner.k_context()];
        let mut seen = BTreeSet::new();
        let mut results = Vec::new();
        let mut excluded_by_relay = false;
        while let Some(context_id) = pending.pop() {
            if !seen.insert(context_id) {
                continue;
            }
            let context = contexts
                .iter()
                .find(|context| context.id() == context_id)
                .ok_or_else(|| {
                    planner_error("returned-Vis successor has no generated K context")
                })?;
            let origins =
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
                if !relay {
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
                    pending.push(row.k_context());
                }
                members.push(ReturnedVisMember {
                    origin,
                    successor,
                    installed,
                    relay,
                });
            }
            results.push(ReturnedVisContext {
                context: context_id,
                members,
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
        let plan = plan_static_transition_graph_with_symbols(
            &entry,
            &BTreeMap::new(),
            &NativeProcessSymbols::legacy_prelude(),
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
        assert_eq!(observed, BTreeSet::from([callee_body]));
        assert!(plan
            .returned_vis_in_context(root, &units, &[])
            .expect_err("a missing generated edge must refuse")
            .to_string()
            .contains("unknown endpoint"));
    }
}
