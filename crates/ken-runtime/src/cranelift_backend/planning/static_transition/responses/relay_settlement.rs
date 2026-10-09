//! Closed relay-source settlement shared by the planning refusal and D1 recorder.
//! The D4 lowering arm remains uninstalled; its proposed pair is a separate gate.
use super::*;
use crate::cranelift_backend::planning::static_transition::returned_vis::{
    ExcludedSettlement, ResponseOwnerSettlement,
};

#[derive(Clone, Debug)]
struct Source {
    vis: StaticOriginId,
    tail_edges: bool,
    redex_body: Option<StaticOriginId>,
    via: Vec<StaticOriginId>,
}

// A source result must reach the scrutinized CM through *only* tail edges.
// The CM scrutinee itself is deliberately not a tail edge: its single Vis arm
// is separately checked by response_forwards_vis in source_for_relay.
fn tail_to(
    plan: &StaticTransitionPlan<'_>,
    parents: &ResponseParents,
    mut from: StaticOriginId,
    to: StaticOriginId,
) -> Result<bool, String> {
    let mut seen = BTreeSet::new();
    while from != to {
        if !seen.insert(from) {
            return Err(format!("parent cycle at {from:?}"));
        }
        let (parent, position) = *parents.get(&from)
            .ok_or_else(|| format!("no parent for {from:?} before {to:?}"))?;
        let expr = plan.planned_occurrence_expr(parent).map_err(|e| e.to_string())?;
        if response_tail_edge(expr, position).is_none() {
            return Ok(false);
        }
        from = parent;
    }
    Ok(true)
}

// Trace the operation binder of R back through its enclosing Vis-case CM,
// enumerate ALL result terminals of the scrutinee in its function owner, then
// recurse through Var-Vis feeders. An open terminal or missing parent is a
// false closure, never silently an empty source set.
fn source_for_relay(
    plan: &StaticTransitionPlan<'_>,
    parents: &ResponseParents,
    relay: StaticOriginId,
    root: StaticOriginId,
    seen: &mut BTreeSet<StaticOriginId>,
    omissions: &mut BTreeSet<StaticOriginId>,
) -> Result<Vec<Source>, String> {
    if !seen.insert(relay) {
        // Least fixpoint: another branch already contributed this relay.
        return Ok(Vec::new());
    }
    if seen.len() > plan.source_occurrences.len() {
        return Err("relay traversal exceeded occurrence population".to_owned());
    }
    let operation_origin = plan.child_static_origin(relay, 0).map_err(|e| e.to_string())?;
    let RuntimeExpr::Var(index) = plan.source_occurrence(operation_origin).map_err(|e| e.to_string())?
    else {
        return Err(format!("relay {relay:?} operation is not a Var"));
    };
    let mut cursor = relay;
    let mut crossed_binders = 0usize;
    let mut visited = BTreeSet::new();
    let (_match_origin, scrutinee) = loop {
        if cursor == root || !visited.insert(cursor) {
            return Err(format!("relay {relay:?} has no enclosing Vis-case CM before {root:?}"));
        }
        let (parent, position) = *parents.get(&cursor)
            .ok_or_else(|| format!("relay {relay:?} missing parent at {cursor:?}"))?;
        let expr = plan.planned_occurrence_expr(parent).map_err(|e| e.to_string())?;
        if let RuntimeExpr::ComputationalMatch { cases, .. } = expr {
            if position > 0 {
                let case = cases.get(position - 1)
                    .ok_or_else(|| format!("CM {parent:?} missing case {position}"))?;
                if case.constructor.as_str().ends_with("::ITree::Vis") {
                    let binder = response_vis_operation_binder(case)
                        .ok_or_else(|| format!("CM {parent:?} has no checked Vis operation binder"))?;
                    if usize::try_from(*index).ok().and_then(|i| i.checked_add(crossed_binders))
                        != usize::try_from(binder).ok()
                    {
                        return Err(format!("relay {relay:?} Var({index}) does not name CM {parent:?} op binder {binder} across {crossed_binders} binders"));
                    }
                    let body = plan.child_static_origin(parent, position).map_err(|e| e.to_string())?;
                    if !response_forwards_vis(plan, body, binder).map_err(|e| e.to_string())? {
                        return Err(format!("CM {parent:?} Vis arm is not pure operation forwarding"));
                    }
                    break (parent, plan.child_static_origin(parent, 0).map_err(|e| e.to_string())?);
                }
            }
        }
        let count = response_tail_edge(expr, position)
            .ok_or_else(|| format!("non-tail edge {parent:?}[{position}] between relay {relay:?} and CM"))?;
        crossed_binders = crossed_binders.checked_add(count)
            .ok_or_else(|| "operation binder depth overflow".to_owned())?;
        cursor = parent;
    };
    let sources = source_results_in_scope(plan, parents, relay, scrutinee, None,
        true, seen, &mut BTreeSet::new(), omissions)?;
    // Every result position of the scrutinee was classified (an unclassified
    // terminal, unknown Call, open operation or omitted position is an error
    // above), so an empty set means the scrutinee never yields a Vis: this
    // relay's arm is unreachable and contributes no source.
    Ok(sources)
}

// Only a direct curried application of one-parameter lexical closures is a
// known redex. An arbitrary/generated Call has no source-result provenance.
fn curried_redex_body(
    plan: &StaticTransitionPlan<'_>,
    call: StaticOriginId,
) -> Result<Option<StaticOriginId>, String> {
    let mut depth = 0usize;
    let mut head = call;
    while let RuntimeExpr::Call { .. } = plan.source_occurrence(head).map_err(|e| e.to_string())? {
        depth += 1;
        head = plan.child_static_origin(head, 0).map_err(|e| e.to_string())?;
    }
    if !matches!(plan.source_occurrence(head).map_err(|e| e.to_string())?, RuntimeExpr::LexicalClosure { .. }) {
        return Ok(None);
    }
    let mut body = head;
    for _ in 0..depth {
        match plan.source_occurrence(body).map_err(|e| e.to_string())? {
            RuntimeExpr::LexicalClosure { params, .. } if params.len() == 1 => {
                body = plan.child_static_origin(body, 0).map_err(|e| e.to_string())?;
            }
            _ => return Err(format!("curried closure call {call:?} arity disagrees at {body:?}")),
        }
    }
    Ok(Some(body))
}

fn source_results_in_scope(
    plan: &StaticTransitionPlan<'_>,
    parents: &ResponseParents,
    relay: StaticOriginId,
    scope: StaticOriginId,
    redex_body: Option<StaticOriginId>,
    inherited_tail: bool,
    seen_relays: &mut BTreeSet<StaticOriginId>,
    seen_calls: &mut BTreeSet<StaticOriginId>,
    omissions: &mut BTreeSet<StaticOriginId>,
) -> Result<Vec<Source>, String> {
    let (terminal_origins, omitted) = plan.source_result_origins_with_omissions(scope)
        .map_err(|e| e.to_string())?;
    omissions.extend(&omitted);
    if let Some(position) = omitted.first() {
        return Err(format!("source scope {scope:?} result position {position:?} leaves its function owner"));
    }
    let mut sources = Vec::new();
    for origin in terminal_origins {
        let expr = plan.source_occurrence(origin).map_err(|e| e.to_string())?;
        match expr {
            RuntimeExpr::Call { .. } => {
                if !seen_calls.insert(origin) {
                    return Err(format!("curried source Call repeats at {origin:?}"));
                }
                let body = curried_redex_body(plan, origin)?
                    .ok_or_else(|| format!("generated or unknown Call terminal {origin:?}"))?;
                let _call_side_tail = inherited_tail && tail_to(plan, parents, origin, scope)?;
                // The redex is a bounded source scope, not a generated call
                // edge. But crossing its Call is not itself a response_tail_edge;
                // condition (c) is false for a source found inside that body.
                sources.extend(source_results_in_scope(plan, parents, relay, body,
                    Some(body), false, seen_relays, seen_calls, omissions)?);
            }
            RuntimeExpr::Construct { constructor, args }
                if constructor.as_str().ends_with("::ITree::Vis") && args.len() == 2 => {
                let path_tail = inherited_tail && tail_to(plan, parents, origin, scope)?;
                let operation = plan.child_static_origin(origin, 0).map_err(|e| e.to_string())?;
                match plan.source_occurrence(operation).map_err(|e| e.to_string())? {
                    RuntimeExpr::Construct { .. } => sources.push(Source {
                        vis: origin, tail_edges: path_tail, redex_body, via: vec![origin, relay],
                    }),
                    RuntimeExpr::Var(_) => {
                        let upstream = source_for_relay(plan, parents, origin, scope, seen_relays, omissions)?;
                        for mut source in upstream {
                            source.tail_edges &= path_tail;
                            source.via.push(relay);
                            sources.push(source);
                        }
                    }
                    other => return Err(format!("source {origin:?} operation open: {:?}",
                        std::mem::discriminant(other))),
                }
            }
            RuntimeExpr::Construct { constructor, .. }
                if constructor.as_str().ends_with("::ITree::Ret") => {}
            RuntimeExpr::Trap(_) => {}
            RuntimeExpr::Effect { .. } | RuntimeExpr::Var(_)
            | RuntimeExpr::Value(_) | RuntimeExpr::PrimitiveCall { .. }
            | RuntimeExpr::Record { .. } | RuntimeExpr::Project { .. }
            | RuntimeExpr::Closure { .. } | RuntimeExpr::LexicalClosure { .. }
            | RuntimeExpr::DeclarationRef { .. } | RuntimeExpr::ImportedDeclarationRef { .. }
            | RuntimeExpr::Construct { .. } => {
                return Err(format!("unclassified source terminal {origin:?}: {:?}",
                    std::mem::discriminant(expr)));
            }
            // The result inventory also includes the wrappers it visited.
            RuntimeExpr::CheckedJoinSite { .. }
            | RuntimeExpr::CheckedSubcontinuationFrame { .. }
            | RuntimeExpr::CheckedRecursiveInvocation { .. }
            | RuntimeExpr::CheckedComputationalIHSlots { .. }
            | RuntimeExpr::CheckedComputationalIHInvocation { .. }
            | RuntimeExpr::Let { .. } | RuntimeExpr::If { .. }
            | RuntimeExpr::Match { .. } | RuntimeExpr::ComputationalMatch { .. } => {}
        }
    }
    Ok(sources)
}

fn source_shape(plan: &StaticTransitionPlan<'_>, vis: StaticOriginId) -> Result<bool, String> {
    let expr = plan.source_occurrence(vis).map_err(|e| e.to_string())?;
    let RuntimeExpr::Construct { constructor, args } = expr else { return Ok(false) };
    if !constructor.as_str().ends_with("::ITree::Vis") || args.len() != 2 { return Ok(false); }
    let RuntimeExpr::Construct { args: outer, .. } = &args[0] else { return Ok(false) };
    if !matches!(outer.as_slice(), [RuntimeExpr::Construct { .. }]) { return Ok(false); }
    Ok(matches!(&args[1], RuntimeExpr::LexicalClosure { params, .. } if params.len() == 1))
}

fn classify(plan: &StaticTransitionPlan<'_>, emit_records: bool) -> Vec<String> {
    if !emit_records && !plan.pending_vis_settlements.owners.values()
        .any(|class| matches!(class, ResponseOwnerSettlement::Excluded { .. }))
    {
        return Vec::new();
    }
    let mut hypothetical_refusals = Vec::new();
    let thread = std::thread::current().name().unwrap_or("unnamed").to_owned();
    if emit_records {
        eprintln!("RELAY_D1_BEGIN thread={thread:?} owners={}",
            plan.static_response_continuations.len());
    }
    let parents = response_parents(plan);
    let candidates = plan.return_protocol_candidate_rows();
    let contexts = plan.continuation_contexts();
    if emit_records {
        eprintln!("RELAY_D1_INPUT thread={thread:?} parents={:?} candidates={:?} contexts={:?}",
            parents.as_ref().map(BTreeMap::len), candidates.as_ref().map(Vec::len), contexts.as_ref().map(Vec::len));
    }
    for owner in &plan.static_response_continuations {
        let class = plan.pending_vis_settlements.owners.get(&owner.id());
        let Some(ResponseOwnerSettlement::Excluded { reason }) = class else { continue; };
        let protocol = plan.returned_vis_protocol(owner.vis_origin());
        let mut relay_count = 0usize;
        let mut source_count = 0usize;
        let mut omissions = BTreeSet::new();
        let mut owner_admitted = matches!(reason, ExcludedSettlement::Relay);
        let mut errors = Vec::new();
        match (&protocol, &parents, &candidates, &contexts) {
            (Ok(protocol), Ok(parents), Ok(candidates), Ok(contexts)) => {
                for context in &protocol.contexts {
                    let root = contexts.iter().find(|v| v.id() == context.context)
                        .map(|v| v.worker_body_origin());
                    for member in &context.members {
                        if !member.relay { continue; }
                        relay_count += 1;
                        let traced = root.ok_or_else(|| "missing context root".to_owned())
                            .and_then(|root| source_for_relay(plan, parents, member.origin, root,
                                &mut BTreeSet::new(), &mut omissions));
                        let closed = traced.is_ok();
                        if emit_records {
                            eprintln!("RELAY_D1_RELAY thread={thread:?} owner={:?} relay={:?} context={:?} root={root:?} closed={closed} sources={:?} open={:?}",
                                owner.id(), member.origin, context.context,
                                traced.as_ref().map(|sources| sources.iter().map(|s| s.vis).collect::<Vec<_>>()),
                                traced.as_ref().err());
                        }
                        if !closed { owner_admitted = false; }
                        match traced {
                            Err(error) => errors.push(error),
                            Ok(sources) => for source in sources {
                                source_count += 1;
                                let rows = candidates.iter().filter(|row| row.vis_origin() == source.vis)
                                    .collect::<Vec<_>>();
                                let seats = rows.iter().filter(|row| plan.host_effect_seat_records().iter()
                                    .any(|seat| seat.effect_origin == row.effect_origin()
                                        && seat.operation == row.operation())).count();
                                let a_unique_row_and_seat = rows.len() == 1 && seats == 1;
                                let b_construct_closure = source_shape(plan, source.vis);
                                let c_tail_forward_only = source.tail_edges;
                                // (d) is a proposed dispatch key, NOT an installed gate.
                                // Keep both separately: a pair can be recorded without
                                // making any production assertion about gate reachability.
                                let d_pair_key = (source.vis, owner.base_owner());
                                let d_pair_proposed = true;
                                let d_gate_installed = false;
                                let deferred = plan.deferred_response_at_vis(source.vis);
                                let bounded_handler = deferred.as_ref().ok().and_then(Option::as_ref)
                                    .map(|row| plan.bounded_deferred_response_handler_owner(row));
                                let unitless_row = source.redex_body
                                    .map(|body| plan.deferred_no_unit_response_in_body(body));
                                let admitted_handler = deferred.as_ref().ok().and_then(Option::as_ref)
                                    .map(|row| plan.admitted_deferred_handler_owner(row));
                                let unitless_matches = matches!(
                                    (&unitless_row, &deferred),
                                    (Some(Ok(Some(unitless))), Ok(Some(row))) if unitless == row
                                );
                                let drive_settled = match deferred.as_ref() {
                                    Ok(Some(row)) => match row.sub_case() {
                                        DeferredResponseSubCase::UnconsumedTransportCaller =>
                                            matches!(&bounded_handler, Some(Ok(Some(actual)))
                                                if *actual == owner.base_owner()),
                                        DeferredResponseSubCase::NoContinuationUnit =>
                                            source.redex_body.is_some() && unitless_matches
                                            && matches!(&admitted_handler, Some(Ok(Some(actual)))
                                                if *actual == owner.base_owner()),
                                        DeferredResponseSubCase::ContinuationBodyTail => false,
                                    },
                                    Ok(None) | Err(_) => false,
                                };
                                let relay_drivable_if_keyed = a_unique_row_and_seat
                                    && b_construct_closure.as_ref().is_ok_and(|b| *b)
                                    && c_tail_forward_only && d_pair_proposed;
                                let source_ok = drive_settled || relay_drivable_if_keyed;
                                owner_admitted &= source_ok;
                                if emit_records {
                                    eprintln!("RELAY_D1_SOURCE thread={thread:?} owner={:?} relay={:?} source={:?} via={:?} redex_body={:?} closed={closed} drive_settled={drive_settled} deferred={deferred:?} bounded_handler={bounded_handler:?} unitless_row={unitless_row:?} unitless_matches={unitless_matches} admitted_handler={admitted_handler:?} a_unique_row_and_seat={a_unique_row_and_seat} a_row_count={} a_seat_count={seats} b_construct_closure={b_construct_closure:?} c_tail_forward_only={c_tail_forward_only} d_pair_key={d_pair_key:?} d_pair_proposed={d_pair_proposed} d_gate_installed={d_gate_installed} relay_drivable_if_keyed={relay_drivable_if_keyed} source_ok={source_ok}",
                                        owner.id(), member.origin, source.vis, source.via, source.redex_body, rows.len());
                                }
                                if let Err(error) = b_construct_closure { errors.push(error); }
                                if let Err(error) = deferred { errors.push(error.to_string()); }
                                if let Some(Err(error)) = bounded_handler { errors.push(error.to_string()); }
                                if let Some(Err(error)) = admitted_handler { errors.push(error.to_string()); }
                                if let Some(Err(error)) = unitless_row { errors.push(error.to_string()); }
                            },
                        }
                    }
                }
            }
            _ => {
                owner_admitted = false;
                errors.push(format!("unavailable inputs protocol={:?} parents={:?} candidates={:?} contexts={:?}",
                    protocol.as_ref().err(), parents.as_ref().err(),
                    candidates.as_ref().err(), contexts.as_ref().err()));
            }
        }
        if relay_count == 0 { owner_admitted = false; }
        if !errors.is_empty() { owner_admitted = false; }
        if emit_records {
            eprintln!("RELAY_D1_OWNER thread={thread:?} owner={:?} vis={:?} base={:?} exclusion={reason:?} relays={relay_count} sources={source_count} omissions={omissions:?} would_admit_if_pair_keyed={owner_admitted} would_refuse_if_pair_keyed={} errors={errors:?}",
                owner.id(), owner.vis_origin(), owner.base_owner(), !owner_admitted);
        }
        if !owner_admitted {
            let relays = protocol.as_ref().ok().map(|protocol| protocol.contexts.iter()
                .flat_map(|context| context.members.iter())
                .filter(|member| member.relay)
                .map(|member| format!("{:?}", member.origin))
                .collect::<Vec<_>>()).unwrap_or_default();
            hypothetical_refusals.push(format!(
                "owner {:?} at Vis {:?} excluded {reason:?}: relay [{}] not settled (base {:?})",
                owner.id(), owner.vis_origin(), relays.join(", "), owner.base_owner()));
        }
    }
    if emit_records {
        eprintln!("RELAY_D1_END thread={thread:?}");
    }
    hypothetical_refusals
}

pub(super) fn refusals(plan: &StaticTransitionPlan<'_>) -> Vec<String> {
    classify(plan, false)
}

#[cfg(feature = "px8-ds-test-support")]
pub(super) fn record(plan: &StaticTransitionPlan<'_>) -> Vec<String> {
    classify(plan, true)
}
