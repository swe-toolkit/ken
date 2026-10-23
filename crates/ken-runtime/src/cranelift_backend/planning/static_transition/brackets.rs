//! Compiler-only, checked-marker bracket settlement authority.
//!
//! A region is a static body set; response contexts distinguish executions at
//! its marked settlement Vis. Host operation identities never discover regions.

use std::collections::{BTreeMap, BTreeSet};

use super::closure::CaseEmissionStatus;
use super::{planner_capacity_error, planner_error, CraneliftBackendError, StaticOriginId,
    StaticTransitionPlan};
use crate::{CheckedBracketMarkRole, CheckedBracketRegionMarker,
    CheckedComputationalIHInvocationKind, NativeProcessSymbols, RuntimeDeclaration,
    RuntimeDeclarationKind, RuntimeExpr, RuntimeSymbol};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct BracketRoles {
    ret: RuntimeSymbol,
    vis: RuntimeSymbol,
    in_l: RuntimeSymbol,
    result_err: RuntimeSymbol,
    result_ok: RuntimeSymbol,
}

impl From<&NativeProcessSymbols> for BracketRoles {
    fn from(symbols: &NativeProcessSymbols) -> Self {
        Self {
            ret: symbols.itree_ret.clone(), vis: symbols.itree_vis.clone(),
            in_l: symbols.coproduct_in_l.clone(), result_err: symbols.result_err.clone(),
            result_ok: symbols.result_ok.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct BracketSettlementId(u32);

impl BracketSettlementId {
    fn next(size: usize) -> Result<Self, CraneliftBackendError> {
        Ok(Self(u32::try_from(size).map_err(|_| {
            planner_capacity_error("bracket settlement identity exhausted")
        })?))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum BracketResponseContextKey {
    Inline {
        producer_call_origin: StaticOriginId,
        response_origin: StaticOriginId,
        vis_origin: StaticOriginId,
        operation_root_origin: StaticOriginId,
    },
    Continuation(super::ContinuationCallIdentity),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum BracketBodyExitClass {
    ReturnedValue,
    ReturnedBodyError,
    ControlledTrap,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct BracketRegionKey {
    owner: RuntimeSymbol,
    ok_resource_binder_path: Vec<usize>,
}

impl From<&CheckedBracketRegionMarker> for BracketRegionKey {
    fn from(marker: &CheckedBracketRegionMarker) -> Self {
        Self {
            owner: marker.owner.clone(),
            ok_resource_binder_path: marker.ok_resource_binder_path.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct BracketSettlementRegion {
    pub(super) id: BracketSettlementId,
    pub(super) parent: Option<BracketSettlementId>,
    pub(super) key: BracketRegionKey,
    pub(super) context: BracketResponseContextKey,
    pub(super) acquire: StaticOriginId,
    pub(super) acquire_failure_resume: StaticOriginId,
    pub(super) body_entry: StaticOriginId,
    pub(super) body: BTreeSet<StaticOriginId>,
    pub(super) body_exit: StaticOriginId,
    pub(super) returned_classes: [BracketBodyExitClass; 2],
    pub(super) settlement: StaticOriginId,
    pub(super) outcome_merge: StaticOriginId,
    pub(super) resume: StaticOriginId,
    pub(super) controlled_trap_edges: Vec<StaticOriginId>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct BracketSettlementPlan {
    pub(super) roles: Option<BracketRoles>,
    pub(super) regions: Vec<BracketSettlementRegion>,
}

#[derive(Clone, Debug)]
pub(super) struct BracketStaticRegion {
    pub(super) key: BracketRegionKey,
    pub(super) acquire: StaticOriginId,
    pub(super) acquire_failure_resume: StaticOriginId,
    pub(super) body_entry: StaticOriginId,
    pub(super) body: BTreeSet<StaticOriginId>,
    pub(super) settlement: StaticOriginId,
    pub(super) outcome_merge: StaticOriginId,
    pub(super) resume: StaticOriginId,
    pub(super) crossed_settlements: BTreeSet<StaticOriginId>,
    pub(super) parent: Option<usize>,
}

#[derive(Default)]
struct MarkGroup {
    acquire: Vec<(StaticOriginId, CheckedBracketRegionMarker)>,
    finalizers: Vec<(StaticOriginId, CheckedBracketRegionMarker)>,
}

type ParentMap = BTreeMap<StaticOriginId, (StaticOriginId, usize)>;

fn parents(plan: &StaticTransitionPlan<'_>) -> Result<ParentMap, CraneliftBackendError> {
    let mut out = ParentMap::new();
    for node in plan.source_occurrences.iter().flatten() {
        for (position, child) in plan.semantic.child_origins(node.static_origin)?.iter().enumerate() {
            if out.insert(*child, (node.static_origin, position)).is_some() {
                return Err(planner_error("a bracket source child has two parents"));
            }
        }
    }
    Ok(out)
}

fn parent(parents: &ParentMap, origin: StaticOriginId, position: usize)
    -> Result<StaticOriginId, CraneliftBackendError>
{
    match parents.get(&origin) {
        Some((parent, actual)) if *actual == position => Ok(*parent),
        _ => Err(planner_error("a checked bracket anchor is not at its ruled child position")),
    }
}

fn expr<'a>(plan: &'a StaticTransitionPlan<'_>, origin: StaticOriginId)
    -> Result<&'a RuntimeExpr, CraneliftBackendError>
{
    plan.planned_occurrence_expr(origin)
}

fn child(plan: &StaticTransitionPlan<'_>, origin: StaticOriginId, position: usize)
    -> Result<StaticOriginId, CraneliftBackendError>
{
    plan.semantic.child_origin(origin, position)
}

fn is_construct(plan: &StaticTransitionPlan<'_>, origin: StaticOriginId,
    symbol: &RuntimeSymbol, arity: usize) -> Result<bool, CraneliftBackendError>
{
    Ok(matches!(expr(plan, origin)?, RuntimeExpr::Construct { constructor, args }
        if constructor == symbol && args.len() == arity))
}

fn lexical_body(plan: &StaticTransitionPlan<'_>, origin: StaticOriginId)
    -> Result<StaticOriginId, CraneliftBackendError>
{
    if !matches!(expr(plan, origin)?, RuntimeExpr::LexicalClosure { params, .. }
        if params.len() == 1) {
        return Err(planner_error("a checked bracket Vis has no one-parameter continuation"));
    }
    child(plan, origin, 0)
}

fn select_cases(plan: &StaticTransitionPlan<'_>, origin: StaticOriginId,
    first: &RuntimeSymbol, second: &RuntimeSymbol) -> Result<(usize, usize), CraneliftBackendError>
{
    let cases = match expr(plan, origin)? {
        RuntimeExpr::Match { cases, .. } => cases.iter().map(|c| &c.constructor).collect::<Vec<_>>(),
        RuntimeExpr::ComputationalMatch { cases, .. } =>
            cases.iter().map(|c| &c.constructor).collect::<Vec<_>>(),
        _ => return Err(planner_error("a checked bracket port is not a source Match")),
    };
    if cases.len() != 2 || first == second {
        return Err(planner_error("a checked bracket dispatch has no exact two checked cases"));
    }
    let a = cases.iter().position(|c| *c == first);
    let b = cases.iter().position(|c| *c == second);
    match (a, b) {
        (Some(a), Some(b)) if a != b => Ok((a, b)),
        _ => Err(planner_error("a checked bracket dispatch has no exact two checked cases")),
    }
}

fn forwarding_vis_arm(expr: &RuntimeExpr, vis: &RuntimeSymbol) -> bool {
    match expr {
        RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. } => forwarding_vis_arm(body, vis),
        RuntimeExpr::Construct { constructor, args } if constructor == vis && args.len() == 2 =>
            matches!(&args[0], RuntimeExpr::Var(1))
            && matches!(&args[1], RuntimeExpr::CheckedComputationalIHInvocation {
                    kind: CheckedComputationalIHInvocationKind::OrdinaryApplication, body, .. }
                if matches!(body.as_ref(), RuntimeExpr::Call { callee, args }
                    if args.is_empty() && matches!(callee.as_ref(), RuntimeExpr::Var(_)))),
        _ => false,
    }
}

fn forwarding_bind(plan: &StaticTransitionPlan<'_>, origin: StaticOriginId,
    roles: &BracketRoles) -> Result<usize, CraneliftBackendError>
{
    let RuntimeExpr::ComputationalMatch { cases, .. } = expr(plan, origin)? else {
        return Err(planner_error("body fold is not a forwarding bind"));
    };
    let (ret, vis) = select_cases(plan, origin, &roles.ret, &roles.vis)?;
    let vis_case = &cases[vis];
    if vis_case.argument_binders != 2 || vis_case.recursive_positions.as_slice() != [1]
        || !forwarding_vis_arm(&vis_case.body, &roles.vis)
    {
        return Err(planner_error("fold is not a forwarding bind"));
    }
    Ok(ret)
}

fn marked_finalizer<'a>(plan: &'a StaticTransitionPlan<'_>,
    vis: StaticOriginId, roles: &BracketRoles)
    -> Result<Option<&'a CheckedBracketRegionMarker>, CraneliftBackendError>
{
    if !is_construct(plan, vis, &roles.vis, 2)? {
        return Ok(None);
    }
    let in_l = child(plan, vis, 0)?;
    if !is_construct(plan, in_l, &roles.in_l, 1)? {
        return Ok(None);
    }
    let mark = child(plan, in_l, 0)?;
    match expr(plan, mark)? {
        RuntimeExpr::CheckedBracketFinalizer { region, .. }
            if region.role == CheckedBracketMarkRole::Finalizer => Ok(Some(region)),
        _ => Ok(None),
    }
}

fn body_walk(plan: &StaticTransitionPlan<'_>, origin: StaticOriginId,
    roles: &BracketRoles, key: &BracketRegionKey, depth: usize,
    visited: &mut BTreeSet<StaticOriginId>, exits: &mut BTreeSet<StaticOriginId>,
    crossed: &mut BTreeSet<StaticOriginId>) -> Result<(), CraneliftBackendError>
{
    if !visited.insert(origin) { return Ok(()); }
    let node = expr(plan, origin)?;
    match node {
        RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. } =>
            body_walk(plan, child(plan, origin, 0)?, roles, key, depth, visited, exits, crossed),
        RuntimeExpr::Let { .. } =>
            body_walk(plan, child(plan, origin, 1)?, roles, key, depth, visited, exits, crossed),
        RuntimeExpr::If { .. } => {
            body_walk(plan, child(plan, origin, 1)?, roles, key, depth, visited, exits, crossed)?;
            body_walk(plan, child(plan, origin, 2)?, roles, key, depth, visited, exits, crossed)
        }
        RuntimeExpr::Match { cases, .. } => {
            for (index, _) in cases.iter().enumerate() {
                let body = child(plan, origin, 1 + index)?;
                let statuses = plan.case_emissions.iter().filter(|row|
                    row.match_origin == origin && row.body_origin == body
                ).collect::<Vec<_>>();
                match statuses.as_slice() {
                    [row] if row.status == CaseEmissionStatus::Reachable =>
                        body_walk(plan, body, roles, key, depth, visited, exits, crossed)?,
                    [row] if row.status == CaseEmissionStatus::Eliminated => {},
                    _ => return Err(planner_error("a bracket Body case has no unique planned verdict")),
                }
            }
            Ok(())
        }
        RuntimeExpr::ComputationalMatch { .. } => {
            let ret = forwarding_bind(plan, origin, roles)?;
            body_walk(plan, child(plan, origin, 0)?, roles, key, depth + 1, visited, exits, crossed)?;
            body_walk(plan, child(plan, origin, 1 + ret)?, roles, key, depth, visited, exits, crossed)
        }
        RuntimeExpr::Construct { constructor, args } if constructor == &roles.vis => {
            if let Some(marker) = marked_finalizer(plan, origin, roles)? {
                if BracketRegionKey::from(marker) == *key {
                    if depth != 0 {
                        return Err(planner_error("bracket settlement lies inside a bind scrutinee"));
                    }
                    exits.insert(origin);
                    return Ok(());
                }
                crossed.insert(origin);
            }
            if args.len() != 2 {
                return Err(planner_error("body tail leaves the region unit"));
            }
            let k = child(plan, origin, 1)?;
            body_walk(plan, lexical_body(plan, k)?, roles, key, depth, visited, exits, crossed)
        }
        RuntimeExpr::Construct { constructor, args } if constructor == &roles.ret => {
            if depth == 0 || args.len() != 1 {
                return Err(planner_error("body bypasses settlement"));
            }
            Ok(())
        }
        RuntimeExpr::Trap(_) => Err(planner_error("body trap has no settlement edge")),
        _ => Err(planner_error("body tail leaves the region unit")),
    }
}

fn source_marks(plan: &StaticTransitionPlan<'_>)
    -> Result<BTreeMap<BracketRegionKey, MarkGroup>, CraneliftBackendError>
{
    let mut groups = BTreeMap::<BracketRegionKey, MarkGroup>::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let origin = occurrence.static_origin;
        let (marker, effect) = match occurrence.expr {
            RuntimeExpr::Effect { bracket_acquire: Some(mark), .. } => (mark.as_ref(), true),
            RuntimeExpr::CheckedBracketFinalizer { region, .. } => (region.as_ref(), false),
            _ => continue,
        };
        let key = BracketRegionKey::from(marker);
        let group = groups.entry(key).or_default();
        match (marker.role, effect) {
            (CheckedBracketMarkRole::Acquire, _) => group.acquire.push((origin, marker.clone())),
            (CheckedBracketMarkRole::Finalizer, false) =>
                group.finalizers.push((origin, marker.clone())),
            (CheckedBracketMarkRole::Finalizer, true) =>
                return Err(planner_error("a bracket Effect has a non-Acquire checked mark")),
        }
    }
    Ok(groups)
}

fn exact_mark_pair(group: &MarkGroup) -> Result<(
    &(StaticOriginId, CheckedBracketRegionMarker),
    &(StaticOriginId, CheckedBracketRegionMarker),
), CraneliftBackendError> {
    let [acquire] = group.acquire.as_slice() else {
        return Err(planner_error("a checked bracket region lacks one Acquire mark"));
    };
    let [finalizer] = group.finalizers.as_slice() else {
        return Err(planner_error(if group.finalizers.is_empty() {
            "an unowned marked settlement has no Finalizer"
        } else { "one region owns two settlement effects" }));
    };
    Ok((acquire, finalizer))
}

pub(super) fn build_bracket_static_regions(plan: &StaticTransitionPlan<'_>,
    roles: &BracketRoles) -> Result<Vec<BracketStaticRegion>, CraneliftBackendError>
{
    let parents = parents(plan)?;
    let mut regions = Vec::new();
    for (key, group) in source_marks(plan)? {
        let ((acquire_mark, acquire), (finalizer_mark, finalizer)) =
            exact_mark_pair(&group)?;
        if acquire.owner != finalizer.owner
            || acquire.acquire_vis_path != finalizer.acquire_vis_path
            || acquire.acquire_response_binder_path != finalizer.acquire_response_binder_path
            || acquire.finalizer_path != finalizer.finalizer_path
        {
            return Err(planner_error("dual bracket ownership disagrees on checked region ports"));
        }
        if !matches!(expr(plan, *finalizer_mark)?, RuntimeExpr::CheckedBracketFinalizer { body, .. }
            if matches!(body.as_ref(), RuntimeExpr::Construct { .. })) {
            return Err(planner_error("a Finalizer mark does not wrap its selected constructor"));
        }
        let in_l = parent(&parents, *finalizer_mark, 0)?;
        if !is_construct(plan, in_l, &roles.in_l, 1)? {
            return Err(planner_error("a Finalizer mark is not in checked InL"));
        }
        let settlement = parent(&parents, in_l, 0)?;
        if !is_construct(plan, settlement, &roles.vis, 2)? {
            return Err(planner_error("a Finalizer mark does not select checked Vis"));
        }
        let (a, mut k) = match expr(plan, *acquire_mark)? {
            RuntimeExpr::Effect { bracket_acquire: Some(_), .. } => {
                let a = parent(&parents, *acquire_mark, 0)?;
                if !matches!(expr(plan, a)?, RuntimeExpr::Let { .. }) {
                    return Err(planner_error("static Acquire is not the executing Let Effect"));
                }
                (a, child(plan, a, 1)?)
            }
            RuntimeExpr::CheckedBracketFinalizer { body, .. } if matches!(body.as_ref(),
                RuntimeExpr::Construct { .. }) => {
                let in_l = parent(&parents, *acquire_mark, 0)?;
                if !is_construct(plan, in_l, &roles.in_l, 1)? {
                    return Err(planner_error("generic Acquire is not under checked InL"));
                }
                let a = parent(&parents, in_l, 0)?;
                if !is_construct(plan, a, &roles.vis, 2)? {
                    return Err(planner_error("generic Acquire is not selected by checked Vis"));
                }
                let lexical = child(plan, a, 1)?;
                (a, lexical_body(plan, lexical)?)
            }
            _ => return Err(planner_error("an Acquire mark has no executing operation")),
        };
        let mut folds = Vec::new();
        loop {
            match expr(plan, k)? {
                RuntimeExpr::CheckedSubcontinuationFrame { .. }
                | RuntimeExpr::CheckedJoinSite { .. } => k = child(plan, k, 0)?,
                RuntimeExpr::ComputationalMatch { .. } => {
                    folds.push(k);
                    k = child(plan, k, 0)?;
                }
                _ => break,
            }
        }
        let Some(&f) = folds.last() else {
            return Err(planner_error("acquire dispatch has no positional return fold"));
        };
        let RuntimeExpr::Match { scrutinee, .. } = expr(plan, k)? else {
            return Err(planner_error("acquire response is not dispatched at its own continuation"));
        };
        if !matches!(scrutinee.as_ref(), RuntimeExpr::Var(0)) {
            return Err(planner_error("acquire response is not its own Var(0)"));
        }
        let (err, ok) = select_cases(plan, k, &roles.result_err, &roles.result_ok)?;
        let acquire_error = child(plan, k, 1 + err)?;
        if !is_construct(plan, acquire_error, &roles.ret, 1)? {
            return Err(planner_error("acquire failure does not return to Resume"));
        }
        let body_entry = child(plan, k, 1 + ok)?;
        // D.Ok must be the positional ancestor of the authenticated Finalizer.
        let mut ancestor = settlement;
        let mut in_body = ancestor == body_entry;
        while let Some(&(p, _)) = parents.get(&ancestor) {
            ancestor = p;
            in_body |= ancestor == body_entry;
        }
        if !in_body {
            return Err(planner_error("a Finalizer is outside its acquire Ok body"));
        }
        // The positional F is the host response dispatch; E5's pure-forwarding
        // predicate applies only to folds actually reached from Body.
        let (ret, _) = select_cases(plan, f, &roles.ret, &roles.vis)?;
        let resume = child(plan, f, 1 + ret)?;
        let outcome_merge = child(plan, settlement, 1)?;
        lexical_body(plan, outcome_merge)?;
        let mut body = BTreeSet::new();
        let mut exits = BTreeSet::new();
        let mut crossed = BTreeSet::new();
        body_walk(plan, body_entry, roles, &key, 0, &mut body, &mut exits, &mut crossed)?;
        if exits != BTreeSet::from([settlement]) {
            return Err(planner_error("bracket Body exits do not equal its authenticated settlement marks"));
        }
        regions.push(BracketStaticRegion {
            key, acquire: a, acquire_failure_resume: acquire_error, body_entry, body,
            settlement, outcome_merge, resume, crossed_settlements: crossed, parent: None,
        });
    }
    assign_parent_forest(&mut regions)?;
    Ok(regions)
}

/// Source containment must form a unique acyclic parent relation. Keeping
/// this gate on the very region vector built from checked marks also lets the
/// negative controls vary one graph axis at a time without manufacturing IR.
fn assign_parent_forest(regions: &mut [BracketStaticRegion])
    -> Result<(), CraneliftBackendError>
{
    let mut owned = BTreeSet::new();
    for region in regions.iter() {
        if !owned.insert(region.settlement) {
            return Err(planner_error("dual bracket ownership of one settlement"));
        }
    }
    for child in 0..regions.len() {
        let candidates = (0..regions.len()).filter(|&p| p != child &&
            regions[p].body.contains(&regions[child].acquire)).collect::<Vec<_>>();
        for &p in &candidates {
            if !regions[p].body.contains(&regions[child].resume) {
                return Err(planner_error("child resume outside the parent body"));
            }
            if !regions[p].crossed_settlements.contains(&regions[child].settlement) {
                return Err(planner_error("crossing regions: child settlement was not crossed"));
            }
        }
        for &p in &candidates {
            for &other in &candidates {
                if p != other && !regions[p].body.is_subset(&regions[other].body)
                    && !regions[other].body.is_subset(&regions[p].body)
                {
                    return Err(planner_error("two parents have incomparable Body sets"));
                }
            }
        }
        let minimal = candidates.into_iter().filter(|&p| (0..regions.len())
            .filter(|&other| other != child && other != p &&
                regions[other].body.contains(&regions[child].acquire))
            .all(|other| regions[p].body.is_subset(&regions[other].body)))
            .collect::<Vec<_>>();
        if minimal.len() > 1 {
            return Err(planner_error("two parents share the minimal Body set"));
        }
        regions[child].parent = minimal.first().copied();
    }
    for p in 0..regions.len() {
        for c in 0..regions.len() {
            if p == c { continue; }
            if regions[p].body.iter().any(|id| regions[c].body.contains(id))
                && !regions[p].body.is_subset(&regions[c].body)
                && !regions[c].body.is_subset(&regions[p].body)
            {
                return Err(planner_error("crossing regions have overlapping Body sets"));
            }
            if regions[p].crossed_settlements.contains(&regions[c].settlement)
                != regions[p].body.contains(&regions[c].acquire)
            {
                return Err(planner_error("crossing regions disagree with child acquire containment"));
            }
        }
    }
    for start in 0..regions.len() {
        let mut seen = BTreeSet::new();
        let mut cursor = Some(start);
        while let Some(position) = cursor {
            if !seen.insert(position) { return Err(planner_error("bracket parent cycle")); }
            cursor = regions[position].parent;
        }
    }
    Ok(())
}

/// Expand source regions only by distinct keys on their own authenticated S.
/// Never select one key with `find_map` at a shared static settlement origin.
pub(super) fn bind_bracket_contexts(
    roles: BracketRoles,
    templates: &[BracketStaticRegion],
    raw: &BTreeMap<StaticOriginId, BTreeSet<BracketResponseContextKey>>,
) -> Result<BracketSettlementPlan, CraneliftBackendError> {
    let settlement_owners = templates.iter().map(|r| r.settlement).collect::<BTreeSet<_>>();
    if raw.keys().any(|s| !settlement_owners.contains(s)) {
        return Err(planner_error("dual ownership: raw marked settlement has no static region"));
    }
    let mut ranges = Vec::new();
    let mut regions = Vec::new();
    for template in templates {
        let keys = raw.get(&template.settlement).ok_or_else(||
            planner_error("unowned marked settlement has no exact response context"))?;
        if keys.is_empty() {
            return Err(planner_error("unowned marked settlement has no exact response context"));
        }
        let mut ids = Vec::new();
        for key in keys {
            let id = BracketSettlementId::next(regions.len())?;
            ids.push(id);
            regions.push(BracketSettlementRegion {
                id, parent: None, key: template.key.clone(), context: key.clone(),
                acquire: template.acquire,
                acquire_failure_resume: template.acquire_failure_resume,
                body_entry: template.body_entry, body: template.body.clone(),
                body_exit: template.settlement,
                returned_classes: [BracketBodyExitClass::ReturnedValue,
                    BracketBodyExitClass::ReturnedBodyError],
                settlement: template.settlement, outcome_merge: template.outcome_merge,
                resume: template.resume,
                controlled_trap_edges: Vec::new(),
            });
        }
        ranges.push(ids);
    }
    for (child, template) in templates.iter().enumerate() {
        if let Some(parent) = template.parent {
            match (ranges[parent].as_slice(), ranges[child].as_slice()) {
                ([p], [c]) => {
                    let row = regions.iter_mut().find(|region| region.id == *c)
                        .ok_or_else(|| planner_error("nested bracket instance has no ID"))?;
                    row.parent = Some(*p);
                }
                _ => return Err(planner_error(format!(
                    "nested bracket instance parent has ambiguous response contexts: parent={:?} child={:?}",
                    ranges[parent], ranges[child],
                ))),
            }
        }
    }
    Ok(BracketSettlementPlan { roles: Some(roles), regions })
}

/// Walk the supplied runtime tree, not the occurrence table populated during
/// planning. This order is the semantic plane's child order; mismatches refuse
/// rather than borrowing the original table's cached expression at an origin.
fn direct_children(expr: &RuntimeExpr) -> Vec<&RuntimeExpr> {
    match expr {
        RuntimeExpr::CheckedBracketFinalizer { body, .. }
        | RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { body, .. }
        | RuntimeExpr::Project { record: body, .. }
        | RuntimeExpr::Closure { body, .. } => vec![body.as_ref()],
        RuntimeExpr::LexicalClosure { body, captures, .. } =>
            std::iter::once(body.as_ref()).chain(captures.iter()).collect(),
        RuntimeExpr::Let { value, body } => vec![value.as_ref(), body.as_ref()],
        RuntimeExpr::If { scrutinee, then_expr, else_expr } =>
            vec![scrutinee.as_ref(), then_expr.as_ref(), else_expr.as_ref()],
        RuntimeExpr::Call { callee, args } =>
            std::iter::once(callee.as_ref()).chain(args.iter()).collect(),
        RuntimeExpr::Match { scrutinee, cases, .. } =>
            std::iter::once(scrutinee.as_ref()).chain(cases.iter().map(|case| &case.body)).collect(),
        RuntimeExpr::ComputationalMatch { scrutinee, cases, .. } =>
            std::iter::once(scrutinee.as_ref()).chain(cases.iter().map(|case| &case.body)).collect(),
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } =>
            args.iter().collect(),
        RuntimeExpr::Record { fields } => fields.iter().map(|(_, value)| value).collect(),
        RuntimeExpr::Effect { capability, args, .. } => capability.iter()
            .map(|capability| capability.value.as_ref()).chain(args.iter()).collect(),
        RuntimeExpr::Value(_) | RuntimeExpr::Var(_) | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. } | RuntimeExpr::Trap(_) => Vec::new(),
    }
}

/// Re-derive the complete plane at plan close from direct RuntimeExpr trees
/// and the FINAL source-return graph. The first build reads the planner's
/// occurrence table; this pass replaces that table temporarily with one
/// walked from the caller's actual root and transparent declarations. The
/// inputs are independent, but the two passes share the region derivation
/// functions; this is not algorithmic independence. The original occurrence
/// table supplies only pointer-to-origin identity, never cached expressions
/// or edges. Source edges come from the direct tree; the original table and
/// semantic child arena are restored even on error.
pub(super) fn validate_direct_rebuild<'src>(
    plan: &mut StaticTransitionPlan<'src>,
    entry: &'src RuntimeExpr,
    declarations: &BTreeMap<&str, &'src RuntimeDeclaration>,
    symbols: &NativeProcessSymbols,
) -> Result<(), CraneliftBackendError> {
    // The empty plane must still be checked against the actual source, but a
    // program with no checked mark needs no new graph-coordinate refusal. The
    // direct tree's absent-marker result and the plane's empty region set must
    // agree; only marked programs require the full independent graph rebuild.
    let mut source = vec![entry];
    source.extend(declarations.values().filter_map(|declaration| match &declaration.kind {
        RuntimeDeclarationKind::Transparent { body } => Some(body),
        _ => None,
    }));
    let mut has_checked_mark = false;
    while let Some(expression) = source.pop() {
        has_checked_mark |= matches!(expression,
            RuntimeExpr::CheckedBracketFinalizer { .. }
            | RuntimeExpr::Effect { bracket_acquire: Some(_), .. });
        source.extend(direct_children(expression));
    }
    if !has_checked_mark && plan.bracket_settlements.regions.is_empty() {
        if plan.bracket_settlements.roles.as_ref() != Some(&BracketRoles::from(symbols)) {
            return Err(planner_error(
                "bracket settlement plane differs from direct RuntimeExpr rebuild",
            ));
        }
        return Ok(());
    }
    // The first pass gave each source expression an origin. Use that identity
    // association only: no cached expression content and no planned child edge
    // participates in locating a direct RuntimeExpr child. The source trees
    // own their expressions, so two distinct occurrences have distinct
    // addresses within this one plan-close invocation.
    let mut origin_by_ptr = BTreeMap::new();
    for (index, occurrence) in plan.source_occurrences.iter().enumerate() {
        let Some(occurrence) = occurrence else { continue };
        let origin = StaticOriginId(u32::try_from(index).map_err(|_| {
            planner_capacity_error("direct bracket rebuild source origin exhausted")
        })?);
        if occurrence.static_origin != origin {
            return Err(planner_error("direct bracket rebuild has a misfiled source origin"));
        }
        let pointer = std::ptr::from_ref(occurrence.expr).addr();
        if origin_by_ptr.insert(pointer, origin).is_some() {
            return Err(planner_error("direct bracket rebuild has one expression at two origins"));
        }
    }
    let origin_of = |expr: &RuntimeExpr| {
        origin_by_ptr.get(&std::ptr::from_ref(expr).addr()).copied().ok_or_else(|| {
            planner_error("direct bracket rebuild expression has no source origin")
        })
    };
    let mut direct = vec![None; plan.source_occurrences.len()];
    let root = origin_of(entry)?;
    if plan.root_occurrence != Some(root) {
        return Err(planner_error("direct bracket rebuild root origin differs"));
    }
    let mut pending = vec![(root, entry)];
    for (name, declaration) in declarations {
        if let RuntimeDeclarationKind::Transparent { body } = &declaration.kind {
            let origin = origin_of(body)?;
            if plan.declaration_occurrences.get(*name) != Some(&origin) {
                return Err(planner_error(
                    "direct bracket rebuild transparent declaration origin differs",
                ));
            }
            pending.push((origin, body));
        }
    }
    let mut direct_graph = BTreeMap::new();
    while let Some((origin, expression)) = pending.pop() {
        let slot = direct.get_mut(origin.0 as usize).ok_or_else(|| planner_error(
            "direct bracket rebuild names an origin outside the source table",
        ))?;
        if slot.replace(super::PlannedOccurrence { static_origin: origin, expr: expression }).is_some() {
            return Err(planner_error("direct bracket rebuild visits a source origin twice"));
        }
        let children = direct_children(expression);
        let child_origins = children.iter().map(|child| origin_of(child))
            .collect::<Result<Vec<_>, _>>()?;
        pending.extend(child_origins.iter().copied().zip(children));
        direct_graph.insert(origin, child_origins);
    }
    // Both the source occurrence population and every parent→child edge are
    // independently recovered. A same-arity substitution in the first graph
    // must refuse even if it happens to leave the bracket region list equal.
    if direct.iter().map(Option::is_some).ne(
        plan.source_occurrences.iter().map(Option::is_some),
    ) {
        return Err(planner_error("direct bracket rebuild source population differs"));
    }
    let direct_children = plan.semantic.direct_source_child_arena(&direct_graph)?;
    let previous = std::mem::replace(&mut plan.source_occurrences, direct);
    let previous_children = std::mem::replace(&mut plan.semantic.child_origins, direct_children);
    let rebuilt = (|| {
        let roles = BracketRoles::from(symbols);
        let templates = build_bracket_static_regions(plan, &roles)?;
        let marked = plan.marked_bracket_response_contexts(&templates)?;
        bind_bracket_contexts(roles, &templates, &marked)
    })();
    plan.semantic.child_origins = previous_children;
    plan.source_occurrences = previous;
    if rebuilt? != plan.bracket_settlements {
        return Err(planner_error(
            "bracket settlement plane differs from direct RuntimeExpr rebuild",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn template(settlement: u32) -> BracketStaticRegion {
        BracketStaticRegion {
            key: BracketRegionKey { owner: "measured-bracket-owner".into(),
                ok_resource_binder_path: vec![0, 1] },
            acquire: StaticOriginId(1), acquire_failure_resume: StaticOriginId(2),
            body_entry: StaticOriginId(3), body: BTreeSet::from([StaticOriginId(3)]),
            settlement: StaticOriginId(settlement), outcome_merge: StaticOriginId(4),
            resume: StaticOriginId(5), crossed_settlements: BTreeSet::new(), parent: None,
        }
    }

    fn roles() -> BracketRoles {
        BracketRoles {
            ret: "checked-Ret".into(), vis: "checked-Vis".into(),
            in_l: "checked-InL".into(), result_err: "checked-Err".into(),
            result_ok: "checked-Ok".into(),
        }
    }

    fn assert_refusal<T: std::fmt::Debug>(
        result: Result<T, CraneliftBackendError>, reason: &str,
    ) {
        let error = result.expect_err(reason);
        let CraneliftBackendError::Backend(crate::BackendFailure::PlannerInvariant(actual)) = error
        else { panic!("wrong refusal class for {reason}: {error}") };
        assert_eq!(actual, reason);
    }

    fn marked(role: CheckedBracketMarkRole) -> CheckedBracketRegionMarker {
        CheckedBracketRegionMarker {
            owner: "measured-bracket-owner".into(), role, constructor_node_id: 1,
            acquire_vis_path: vec![0], acquire_response_binder_path: vec![1],
            ok_resource_binder_path: vec![0, 1], finalizer_path: vec![2],
        }
    }

    fn graph_region(acquire: u32, resume: u32, settlement: u32,
        body: &[u32], crossed: &[u32]) -> BracketStaticRegion {
        let mut region = template(settlement);
        region.acquire = StaticOriginId(acquire);
        region.resume = StaticOriginId(resume);
        region.body = body.iter().copied().map(StaticOriginId).collect();
        region.crossed_settlements = crossed.iter().copied().map(StaticOriginId).collect();
        region
    }

    /// Promise class: durable invariant. Duplicate ownership of the same
    /// settlement origin and an extra raw S are independently refused.
    #[test]
    fn dual_bracket_ownership_refuses_by_exact_origin() {
        let mut duplicates = [template(11), template(11)];
        assert_refusal(assign_parent_forest(&mut duplicates),
            "dual bracket ownership of one settlement");
        let raw = BTreeMap::from([(StaticOriginId(12), BTreeSet::new())]);
        assert_refusal(bind_bracket_contexts(roles(), &[template(11)], &raw),
            "dual ownership: raw marked settlement has no static region");
    }

    /// Promise class: durable invariant. Each marked source region requires a
    /// Finalizer and its exact response context, rather than an inferred key.
    #[test]
    fn unowned_marked_settlement_refuses_without_finalizer_or_context() {
        let group = MarkGroup {
            acquire: vec![(StaticOriginId(1), marked(CheckedBracketMarkRole::Acquire))],
            finalizers: Vec::new(),
        };
        assert_refusal(exact_mark_pair(&group),
            "an unowned marked settlement has no Finalizer");
        assert_refusal(bind_bracket_contexts(roles(), &[template(11)], &BTreeMap::new()),
            "unowned marked settlement has no exact response context");
    }

    /// Promise class: durable invariant. A second Finalizer at one authenticated
    /// region key must refuse even when both would reach the same source Vis.
    #[test]
    fn one_region_cannot_own_two_settlement_effects() {
        let finalizer = marked(CheckedBracketMarkRole::Finalizer);
        let group = MarkGroup {
            acquire: vec![(StaticOriginId(1), marked(CheckedBracketMarkRole::Acquire))],
            finalizers: vec![(StaticOriginId(11), finalizer.clone()),
                (StaticOriginId(12), finalizer)],
        };
        assert_refusal(exact_mark_pair(&group),
            "one region owns two settlement effects");
    }

    /// Promise class: durable invariant. A parent whose Body contains a child
    /// Acquire but not that child's settlement edge cannot form a scope exit.
    #[test]
    fn crossing_region_without_child_settlement_refuses() {
        let mut regions = [graph_region(1, 2, 11, &[21, 22], &[]),
            graph_region(21, 22, 12, &[30], &[])];
        assert_refusal(assign_parent_forest(&mut regions),
            "crossing regions: child settlement was not crossed");
    }

    /// Promise class: durable invariant. A cycle over two otherwise valid
    /// crossed-settlement edges must refuse at the cycle-specific gate.
    #[test]
    fn bracket_parent_cycle_refuses() {
        let mut regions = [graph_region(1, 2, 11, &[1, 2, 3, 4], &[12]),
            graph_region(3, 4, 12, &[1, 2, 3, 4], &[11])];
        assert_refusal(assign_parent_forest(&mut regions), "bracket parent cycle");
    }

    /// Promise class: durable invariant. Two enclosing Bodies neither nested
    /// in the other cannot both claim the same child's Acquire.
    #[test]
    fn two_incomparable_parents_refuse() {
        let mut regions = [graph_region(1, 2, 11, &[21, 22, 31], &[13]),
            graph_region(3, 4, 12, &[21, 22, 32], &[13]),
            graph_region(21, 22, 13, &[41], &[])];
        assert_refusal(assign_parent_forest(&mut regions),
            "two parents have incomparable Body sets");
    }

    /// Promise class: durable invariant. A child's Resume must be inside its
    /// parent's Body, even when its Acquire is inside and settlement is crossed.
    #[test]
    fn child_resume_outside_parent_body_refuses() {
        let mut regions = [graph_region(1, 2, 11, &[21], &[12]),
            graph_region(21, 22, 12, &[30], &[])];
        assert_refusal(assign_parent_forest(&mut regions),
            "child resume outside the parent body");
    }

    /// Promise class: durable invariant. A genuinely nested child with its
    /// resume in the parent Body and a crossed S has exactly that parent.
    #[test]
    fn nested_bracket_parent_is_selected_by_body_containment() {
        let mut regions = [graph_region(1, 2, 11, &[21, 22], &[12]),
            graph_region(21, 22, 12, &[30], &[])];
        assign_parent_forest(&mut regions).expect("one exact nested parent");
        assert_eq!(regions[0].parent, None);
        assert_eq!(regions[1].parent, Some(0));
    }

    /// Promise class: durable invariant. A returned Body trap with no outgoing
    /// S edge cannot masquerade as an ordinary Body exit in the bounded walk.
    #[test]
    fn body_trap_without_settlement_edge_refuses() {
        let symbols = NativeProcessSymbols::legacy_prelude();
        let root = RuntimeExpr::Trap(crate::RuntimeTrap {
            code: crate::RuntimeTrapCode::ExplicitTrap,
            message: "measured body trap".into(),
        });
        let plan = super::super::plan_static_transition_graph_with_symbols(
            &root, &BTreeMap::new(), &symbols, super::super::AbiRootIngress::Value, false,
        ).expect("the source Trap has a planned origin");
        let mut visited = BTreeSet::new();
        let mut exits = BTreeSet::new();
        let mut crossed = BTreeSet::new();
        assert_refusal(body_walk(&plan, plan.root_occurrence.unwrap(),
            &BracketRoles::from(&symbols), &template(11).key, 0,
            &mut visited, &mut exits, &mut crossed),
            "body trap has no settlement edge");
        assert_eq!(visited.len(), 1, "the refusal reached the exact Trap source");
        assert!(exits.is_empty());
    }

    /// Durable invariant: each exact source-return context over one marked S
    /// mints a separate plane instance; the source S alone is not an ID.
    #[test]
    fn two_context_keys_over_one_settlement_mint_distinct_instances() {
        let origin = StaticOriginId(11);
        let inline = |call| BracketResponseContextKey::Inline {
            producer_call_origin: StaticOriginId(call),
            response_origin: StaticOriginId(20), vis_origin: origin,
            operation_root_origin: StaticOriginId(21),
        };
        let first = inline(31);
        let second = inline(32);
        assert_ne!(first, second);
        let raw = BTreeMap::from([(origin, BTreeSet::from([first.clone(), second.clone()]))]);
        let plan = bind_bracket_contexts(roles(), &[template(origin.0)], &raw).unwrap();
        assert_eq!(plan.regions.len(), 2);
        assert_eq!(plan.regions[0].settlement, origin);
        assert_eq!(plan.regions[1].settlement, origin);
        assert_ne!(plan.regions[0].id, plan.regions[1].id);
        assert_eq!(plan.regions.iter().map(|region| region.context.clone())
            .collect::<BTreeSet<_>>(), BTreeSet::from([first, second]));
    }
}
