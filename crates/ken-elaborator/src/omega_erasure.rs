//! Kernel-classified runtime erasure of checked core. This is an untrusted
//! compilation plan, not a new kernel admission rule.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use ken_kernel::subst::{subst0, weaken};
use ken_kernel::{infer, whnf, Context, GlobalEnv, InductiveDecl, KernelError, Level, Term};

/// Kernel inference over a normalized body. `ken_kernel::normalize` drops the
/// ascriptions the elaborator places on motives, while the kernel infers
/// motives. Retry on a copy whose bare-λ motives are re-ascribed; the kernel
/// checks each λ against its ascription, so the hint can refuse but never
/// admit. The copy is never stored, visited, or counted.
fn infer_node(env: &GlobalEnv, ctx: &Context, t: &Term) -> Result<Term, KernelError> {
    match infer(env, ctx, t) {
        Err(KernelError::Msg(reason)) if reason.contains("cannot infer an introduction form") => {
            let hinted = reascribe_motives(env, &mut ctx.clone(), t)?;
            infer(env, ctx, &hinted)
        }
        other => other,
    }
}

/// `inductive::method_type` infers the motive when it builds a nested
/// (All-lift) IH type. A normalized body's motive is a bare λ, so retry with a
/// re-ascribed copy. The kernel checks the ascription, so the hint can refuse
/// but never admit; the copy only supplies the expected method type, and the
/// original method is what is visited and counted.
fn method_type_node(
    env: &GlobalEnv,
    ctx: &Context,
    ind: &InductiveDecl,
    k: usize,
    motive: &Term,
    params: &[Term],
    level_args: &[Level],
) -> Result<Term, KernelError> {
    match ken_kernel::inductive::method_type(env, ind, k, motive, params, level_args) {
        Err(KernelError::Msg(reason)) if reason.contains("cannot infer an introduction form") => {
            let inner = reascribe_motives(env, &mut ctx.clone(), motive)?;
            let hinted = ascribe_motive(env, &mut ctx.clone(), inner)?;
            ken_kernel::inductive::method_type(env, ind, k, &hinted, params, level_args)
        }
        other => other,
    }
}

fn ascribe_motive(env: &GlobalEnv, ctx: &mut Context, motive: Term) -> Result<Term, KernelError> {
    if !matches!(motive, Term::Lam(..)) {
        return Ok(motive);
    }
    let mut domains = Vec::new();
    let mut cur = &motive;
    while let Term::Lam(domain, body) = cur {
        domains.push((**domain).clone());
        ctx.push((**domain).clone());
        cur = body;
    }
    let sort = infer(env, ctx, cur).map(|sort| whnf(env, ctx, &sort));
    for _ in &domains {
        ctx.pop();
    }
    let mut ty = sort?;
    for domain in domains.into_iter().rev() {
        ty = Term::pi(domain, ty);
    }
    Ok(Term::Ascript(Box::new(motive), Box::new(ty)))
}

fn reascribe_motives(env: &GlobalEnv, ctx: &mut Context, t: &Term) -> Result<Term, KernelError> {
    let mut rebuild = |child: &Term, ctx: &mut Context| reascribe_motives(env, ctx, child);
    Ok(match t {
        Term::Type(_)
        | Term::Omega(_)
        | Term::Var(_)
        | Term::Const { .. }
        | Term::IntLit(_)
        | Term::IndFormer { .. }
        | Term::Constructor { .. } => t.clone(),
        Term::Lam(domain, body) | Term::Pi(domain, body) | Term::Sigma(domain, body) => {
            let domain2 = rebuild(domain, ctx)?;
            ctx.push((**domain).clone());
            let body2 = rebuild(body, ctx);
            ctx.pop();
            let body2 = body2?;
            match t {
                Term::Lam(..) => Term::lam(domain2, body2),
                Term::Pi(..) => Term::pi(domain2, body2),
                _ => Term::sigma(domain2, body2),
            }
        }
        Term::Let { ty, val, body } => {
            let ty2 = rebuild(ty, ctx)?;
            let val2 = rebuild(val, ctx)?;
            ctx.push((**ty).clone());
            let body2 = rebuild(body, ctx);
            ctx.pop();
            Term::Let {
                ty: Box::new(ty2),
                val: Box::new(val2),
                body: Box::new(body2?),
            }
        }
        Term::App(function, argument) => {
            Term::app(rebuild(function, ctx)?, rebuild(argument, ctx)?)
        }
        Term::Pair(first, second) => Term::pair(rebuild(first, ctx)?, rebuild(second, ctx)?),
        Term::Proj1(pair) => Term::proj1(rebuild(pair, ctx)?),
        Term::Proj2(pair) => Term::proj2(rebuild(pair, ctx)?),
        Term::Ascript(term, ty) => {
            Term::Ascript(Box::new(rebuild(term, ctx)?), Box::new(rebuild(ty, ctx)?))
        }
        Term::Eq(ty, lhs, rhs) => Term::Eq(
            Box::new(rebuild(ty, ctx)?),
            Box::new(rebuild(lhs, ctx)?),
            Box::new(rebuild(rhs, ctx)?),
        ),
        Term::Refl(term) => Term::Refl(Box::new(rebuild(term, ctx)?)),
        Term::Cast(ty, lhs, equality, term) => Term::Cast(
            Box::new(rebuild(ty, ctx)?),
            Box::new(rebuild(lhs, ctx)?),
            Box::new(rebuild(equality, ctx)?),
            Box::new(rebuild(term, ctx)?),
        ),
        Term::J(motive, method, equality) => {
            let motive2 = rebuild(motive, ctx)?;
            Term::J(
                Box::new(ascribe_motive(env, ctx, motive2)?),
                Box::new(rebuild(method, ctx)?),
                Box::new(rebuild(equality, ctx)?),
            )
        }
        Term::Quot(ty, relation, equivalence) => Term::Quot(
            Box::new(rebuild(ty, ctx)?),
            Box::new(rebuild(relation, ctx)?),
            Box::new(rebuild(equivalence, ctx)?),
        ),
        Term::QuotClass(term) => Term::QuotClass(Box::new(rebuild(term, ctx)?)),
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => {
            let motive2 = rebuild(motive, ctx)?;
            Term::QuotElim {
                motive: Box::new(ascribe_motive(env, ctx, motive2)?),
                method: Box::new(rebuild(method, ctx)?),
                respect: Box::new(rebuild(respect, ctx)?),
                scrut: Box::new(rebuild(scrut, ctx)?),
            }
        }
        Term::Trunc(term) => Term::Trunc(Box::new(rebuild(term, ctx)?)),
        Term::TruncProj(term) => Term::TruncProj(Box::new(rebuild(term, ctx)?)),
        Term::Absurd(motive, proof) => Term::Absurd(
            Box::new(rebuild(motive, ctx)?),
            Box::new(rebuild(proof, ctx)?),
        ),
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => {
            let motive2 = rebuild(motive, ctx)?;
            Term::Elim {
                fam: *fam,
                level_args: level_args.clone(),
                params: params
                    .iter()
                    .map(|param| rebuild(param, ctx))
                    .collect::<Result<_, _>>()?,
                motive: Box::new(ascribe_motive(env, ctx, motive2)?),
                methods: methods
                    .iter()
                    .map(|method| rebuild(method, ctx))
                    .collect::<Result<_, _>>()?,
                indices: indices
                    .iter()
                    .map(|index| rebuild(index, ctx))
                    .collect::<Result<_, _>>()?,
                scrut: Box::new(rebuild(scrut, ctx)?),
            }
        }
    })
}

/// Preorder indices include every term in the canonical checked body, including
/// type terms and descendants of a maximal erased subterm.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OmegaErasurePlan {
    pub erased_subterms: BTreeSet<u32>,
    pub erased_binders: BTreeSet<u32>,
    pub collapsed_sigmas: BTreeSet<u32>,
}

/// Interpreter-only: preorder id of a computational J to its checked
/// equation, with endpoints erased in the J node's own context. Never stored
/// in the durable OmegaErasurePlan consumed by checked-core lowering.
type JEquations = BTreeMap<u32, Term>;

/// Memoized kernel sort classifications for exactly one immutable GlobalEnv.
/// Scoped entries are dropped at each context pop, so sibling binders cannot
/// share a result merely because they have the same de Bruijn depth.
#[derive(Default)]
pub(crate) struct ClassifyMemo {
    closed: HashMap<Term, bool>,
    scoped: Vec<HashMap<Term, bool>>,
    application_types: HashMap<usize, (Term, Term)>,
}

fn has_free_var(term: &Term, bound: usize) -> bool {
    match term {
        Term::Var(index) => *index >= bound,
        Term::Lam(domain, body) | Term::Pi(domain, body) | Term::Sigma(domain, body) => {
            has_free_var(domain, bound) || has_free_var(body, bound + 1)
        }
        Term::Let { ty, val, body } => {
            has_free_var(ty, bound) || has_free_var(val, bound) || has_free_var(body, bound + 1)
        }
        _ => term
            .children()
            .into_iter()
            .any(|child| has_free_var(child, bound)),
    }
}

impl ClassifyMemo {
    pub(crate) fn new() -> Self {
        Self {
            scoped: vec![HashMap::new()],
            ..Self::default()
        }
    }

    fn classify(&mut self, env: &GlobalEnv, ctx: &Context, ty: &Term) -> Result<bool, KernelError> {
        let is_closed = !has_free_var(ty, 0);
        let table = if is_closed {
            &mut self.closed
        } else {
            &mut self.scoped[ctx.len()]
        };
        if let Some(&hit) = table.get(ty) {
            return Ok(hit);
        }
        let value = is_omega_classified(env, ctx, ty)?;
        table.insert(ty.clone(), value);
        Ok(value)
    }

    fn under_binding<T>(
        &mut self,
        ctx: &mut Context,
        domain: Term,
        f: impl FnOnce(&mut Context, &mut Self) -> Result<T, KernelError>,
    ) -> Result<T, KernelError> {
        ctx.push(domain);
        self.scoped.push(HashMap::new());
        let result = f(ctx, self);
        self.scoped.pop();
        ctx.pop();
        result
    }
}

/// A type is erasable exactly when the kernel infers an Ω sort for that type.
/// In particular, the spelling of the type and of its inhabitant is irrelevant.
pub fn is_omega_classified(env: &GlobalEnv, ctx: &Context, ty: &Term) -> Result<bool, KernelError> {
    let sort = match infer(env, ctx, ty) {
        Ok(sort) => sort,
        Err(KernelError::Msg(reason)) if reason.contains("cannot infer an introduction form") => {
            // Eliminator method types are constructed by the kernel as Π
            // telescopes whose codomain can contain an unapplied motive λ.
            // Kernel checking reduces that codomain at use, whereas raw
            // inference of the unreduced telescope cannot synthesize λ.
            // Normalize only this already-checked type, then ask the kernel
            // for its sort; this preserves its Ω/Type classification.
            infer_node(env, ctx, &ken_kernel::normalize(env, ctx, ty))?
        }
        Err(error) => return Err(error),
    };
    Ok(matches!(whnf(env, ctx, &sort), Term::Omega(_)))
}

/// Build the plan from a closed, already admitted body and its checked type.
/// Introduction forms are checked bidirectionally: the kernel does not infer
/// the type of a bare lambda, pair or refl. A bare lambda with no expected
/// type remains relevant; a shape that cannot be classified safely refuses
/// rather than guessing that the position is computationally relevant.
pub fn omega_erasure_plan(
    env: &GlobalEnv,
    body: &Term,
    checked_type: &Term,
) -> Result<OmegaErasurePlan, KernelError> {
    omega_erasure_plan_with_count(env, body, checked_type).map(|(plan, _)| plan)
}

/// The visited count includes descendants skipped by maximal erasure, so it
/// can be compared with the separately parsed canonical-body preorder.
pub(crate) fn omega_erasure_plan_with_count(
    env: &GlobalEnv,
    body: &Term,
    checked_type: &Term,
) -> Result<(OmegaErasurePlan, u32), KernelError> {
    omega_erasure_plan_with_memo(env, body, checked_type, &mut ClassifyMemo::new())
}

pub(crate) fn omega_erasure_plan_with_memo(
    env: &GlobalEnv,
    body: &Term,
    checked_type: &Term,
    memo: &mut ClassifyMemo,
) -> Result<(OmegaErasurePlan, u32), KernelError> {
    let mut plan = OmegaErasurePlan::default();
    let mut next = 0u32;
    let mut ctx = Context::new();
    visit(
        env,
        &mut ctx,
        body,
        Some(checked_type),
        &mut next,
        &mut plan,
        memo,
        None,
    )?;
    Ok((plan, next))
}

/// Erase a checked body for the interpreter. Unlike the durable plan, the
/// computational J's erased evidence slot carries its checked equation, so
/// its endpoints can decide J-beta after the proof itself has been erased.
pub fn erase_for_interpreter(
    env: &GlobalEnv,
    body: &Term,
    checked_type: &Term,
) -> Result<Term, KernelError> {
    let mut plan = OmegaErasurePlan::default();
    let mut equations = JEquations::new();
    let mut next = 0u32;
    visit(
        env,
        &mut Context::new(),
        body,
        Some(checked_type),
        &mut next,
        &mut plan,
        &mut ClassifyMemo::new(),
        Some(&mut equations),
    )?;
    let mut next = 0u32;
    rewrite(env, body, &plan, Some(&equations), &mut next)
}

/// The checked equation for J, preferring the elaborator's recorded Eq
/// ascription as in the kernel's endpoint derivation (15 §4). A carrier read
/// by inference at Type is refused: its WHNF may expose other endpoints.
fn erased_j_equation(
    env: &GlobalEnv,
    ctx: &mut Context,
    evidence: &Term,
    memo: &mut ClassifyMemo,
) -> Option<Term> {
    let (carrier, lhs, rhs) = match evidence {
        Term::Ascript(_, recorded) => match recorded.as_ref() {
            Term::Eq(a, x, y) => ((**a).clone(), (**x).clone(), (**y).clone()),
            _ => return None,
        },
        _ => match whnf(env, ctx, &infer_node(env, ctx, evidence).ok()?) {
            Term::Eq(a, x, y) if !matches!(whnf(env, ctx, &a), Term::Type(_)) => (*a, *x, *y),
            _ => return None,
        },
    };
    let lhs = erase_open(env, ctx, &lhs, &carrier, memo)?;
    let rhs = erase_open(env, ctx, &rhs, &carrier, memo)?;
    // Carrier slot is never evaluated by the interpreter's J arm.
    Some(Term::Eq(Box::new(carrier), Box::new(lhs), Box::new(rhs)))
}

fn erase_open(
    env: &GlobalEnv,
    ctx: &mut Context,
    term: &Term,
    expected: &Term,
    memo: &mut ClassifyMemo,
) -> Option<Term> {
    let mut plan = OmegaErasurePlan::default();
    let mut equations = JEquations::new();
    let mut next = 0u32;
    visit(
        env,
        ctx,
        term,
        Some(expected),
        &mut next,
        &mut plan,
        memo,
        Some(&mut equations),
    )
    .ok()?;
    let mut next = 0u32;
    rewrite(env, term, &plan, Some(&equations), &mut next).ok()
}

fn skipped_nodes(node: &Term, next: &mut u32) -> Result<(), KernelError> {
    *next = next
        .checked_add(1)
        .ok_or_else(|| KernelError::Msg("erasure plan node index overflow".into()))?;
    for child in node.children() {
        skipped_nodes(child, next)?;
    }
    Ok(())
}

fn visit(
    env: &GlobalEnv,
    ctx: &mut Context,
    node: &Term,
    expected: Option<&Term>,
    next: &mut u32,
    plan: &mut OmegaErasurePlan,
    memo: &mut ClassifyMemo,
    mut equations: Option<&mut JEquations>,
) -> Result<(), KernelError> {
    let here = *next;
    *next = next
        .checked_add(1)
        .ok_or_else(|| KernelError::Msg("erasure plan node index overflow".into()))?;
    // The checked expectation, when present, is the sort plane for
    // non-inferable introduction forms; otherwise use the kernel's inference.
    let inferred: Option<Cow<'_, Term>> = if let Some(ty) = expected {
        Some(Cow::Borrowed(ty))
    } else {
        match infer_node(env, ctx, node) {
            Ok(ty) => Some(Cow::Owned(ty)),
            Err(_) if matches!(node, Term::Lam(..)) => None,
            Err(error) => {
                return Err(KernelError::Msg(format!(
                    "Ω erasure plan at preorder node {here} ({:?}): {error}",
                    std::mem::discriminant(node)
                )))
            }
        }
    };
    if inferred
        .as_deref()
        .map(|ty| {
            memo.classify(env, ctx, ty).map_err(|error| {
                KernelError::Msg(format!(
                    "Ω erasure sort at node {here} ({:?}): {error}",
                    std::mem::discriminant(ty)
                ))
            })
        })
        .transpose()?
        .unwrap_or(false)
    {
        // Never descend into proof bytes, including open holes and refl.
        plan.erased_subterms.insert(here);
        for child in node.children() {
            skipped_nodes(child, next)?;
        }
        return Ok(());
    }
    match node {
        Term::Lam(dom, body) => {
            visit(
                env,
                ctx,
                dom,
                None,
                next,
                plan,
                memo,
                equations.as_deref_mut(),
            )?;
            if memo.classify(env, ctx, dom)? {
                plan.erased_binders.insert(here);
            }
            let expected_body = inferred.as_deref().and_then(|ty| match whnf(env, ctx, ty) {
                Term::Pi(_, cod) => Some(*cod),
                _ => None,
            });
            memo.under_binding(ctx, (**dom).clone(), |ctx, memo| {
                visit(
                    env,
                    ctx,
                    body,
                    expected_body.as_ref(),
                    next,
                    plan,
                    memo,
                    equations,
                )
            })
        }
        Term::Pi(dom, cod) | Term::Sigma(dom, cod) => {
            visit(
                env,
                ctx,
                dom,
                None,
                next,
                plan,
                memo,
                equations.as_deref_mut(),
            )?;
            memo.under_binding(ctx, (**dom).clone(), |ctx, memo| {
                visit(env, ctx, cod, None, next, plan, memo, equations)
            })
        }
        Term::Let { ty, val, body } => {
            visit(
                env,
                ctx,
                ty,
                None,
                next,
                plan,
                memo,
                equations.as_deref_mut(),
            )?;
            if memo.classify(env, ctx, ty)? {
                plan.erased_binders.insert(here);
            }
            visit(
                env,
                ctx,
                val,
                Some(ty),
                next,
                plan,
                memo,
                equations.as_deref_mut(),
            )?;
            let expected_body = inferred.as_deref().map(|t| weaken(t, 1));
            memo.under_binding(ctx, (**ty).clone(), |ctx, memo| {
                visit(
                    env,
                    ctx,
                    body,
                    expected_body.as_ref(),
                    next,
                    plan,
                    memo,
                    equations,
                )
            })
        }
        Term::App(function, argument) => {
            let key = node as *const Term as usize;
            if !memo.application_types.contains_key(&key) {
                // One head inference and one Π instantiation per argument.
                // Cache each partial application's immediate function type
                // while descending the same checked AST spine in preorder.
                let mut nodes = Vec::new();
                let mut head = node;
                while let Term::App(f, a) = head {
                    nodes.push((head as *const Term as usize, a.as_ref()));
                    head = f;
                }
                let mut ty = infer_node(env, ctx, head).map_err(|error| {
                    KernelError::Msg(format!(
                        "Ω erasure application head at node {here}: {error}"
                    ))
                })?;
                for (app, argument) in nodes.into_iter().rev() {
                    let Term::Pi(domain, codomain) = whnf(env, ctx, &ty) else {
                        return Err(KernelError::Msg(
                            "checked application has no Pi domain for erasure".into(),
                        ));
                    };
                    let next_type = subst0(&codomain, argument);
                    memo.application_types.insert(app, (ty, *domain));
                    ty = next_type;
                }
            }
            let (function_type, domain) = memo
                .application_types
                .remove(&key)
                .expect("a checked application spine has an inferred Π at every node");
            visit(
                env,
                ctx,
                function,
                Some(&function_type),
                next,
                plan,
                memo,
                equations.as_deref_mut(),
            )?;
            visit(
                env,
                ctx,
                argument,
                Some(&domain),
                next,
                plan,
                memo,
                equations,
            )
        }
        Term::Pair(first, second) => {
            let Some(ty) = inferred else {
                return Err(KernelError::Msg(
                    "checked pair has no Sigma type for erasure".into(),
                ));
            };
            let Term::Sigma(domain, codomain) = whnf(env, ctx, &ty) else {
                return Err(KernelError::Msg(
                    "checked pair has no Sigma type for erasure".into(),
                ));
            };
            let collapsed = memo.under_binding(ctx, (*domain).clone(), |ctx, memo| {
                memo.classify(env, ctx, &codomain)
            })?;
            if collapsed {
                plan.collapsed_sigmas.insert(here);
            }
            visit(
                env,
                ctx,
                first,
                Some(&domain),
                next,
                plan,
                memo,
                equations.as_deref_mut(),
            )?;
            let second_type = subst0(&codomain, first);
            visit(
                env,
                ctx,
                second,
                Some(&second_type),
                next,
                plan,
                memo,
                equations,
            )
        }
        Term::Proj1(pair) => {
            let pair_type = infer_node(env, ctx, pair)?;
            let Term::Sigma(domain, codomain) = whnf(env, ctx, &pair_type) else {
                return Err(KernelError::Msg(
                    "checked projection has no Sigma type for erasure".into(),
                ));
            };
            let collapsed = memo.under_binding(ctx, (*domain).clone(), |ctx, memo| {
                memo.classify(env, ctx, &codomain)
            })?;
            if collapsed {
                plan.collapsed_sigmas.insert(here);
            }
            visit(
                env,
                ctx,
                pair,
                Some(&pair_type),
                next,
                plan,
                memo,
                equations,
            )
        }
        Term::Ascript(term, ty) => {
            visit(
                env,
                ctx,
                term,
                Some(ty),
                next,
                plan,
                memo,
                equations.as_deref_mut(),
            )?;
            visit(env, ctx, ty, None, next, plan, memo, equations)
        }
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => {
            // The kernel checks methods against these dependent Π types;
            // bare method lambdas cannot synthesize them by inference.
            let ind = env.inductive(*fam).ok_or_else(|| {
                KernelError::Msg(format!("erasure plan has unknown family {fam:?}"))
            })?;
            for param in params {
                visit(
                    env,
                    ctx,
                    param,
                    None,
                    next,
                    plan,
                    memo,
                    equations.as_deref_mut(),
                )?;
            }
            visit(
                env,
                ctx,
                motive,
                None,
                next,
                plan,
                memo,
                equations.as_deref_mut(),
            )
            .map_err(|error| KernelError::Msg(format!("Ω erasure eliminator motive: {error}")))?;
            for (k, method) in methods.iter().enumerate() {
                let method_ty = method_type_node(env, ctx, ind, k, motive, params, level_args)
                    .map_err(|error| {
                        KernelError::Msg(format!("Ω erasure method type {k}: {error}"))
                    })?;
                visit(
                    env,
                    ctx,
                    method,
                    Some(&method_ty),
                    next,
                    plan,
                    memo,
                    equations.as_deref_mut(),
                )
                .map_err(|error| {
                    KernelError::Msg(format!("Ω erasure eliminator method {k}: {error}"))
                })?;
            }
            for index in indices {
                visit(
                    env,
                    ctx,
                    index,
                    None,
                    next,
                    plan,
                    memo,
                    equations.as_deref_mut(),
                )?;
            }
            visit(env, ctx, scrut, None, next, plan, memo, equations)
        }
        Term::J(_, _, evidence) => {
            for child in node.children() {
                visit(
                    env,
                    ctx,
                    child,
                    None,
                    next,
                    plan,
                    memo,
                    equations.as_deref_mut(),
                )?;
            }
            if let Some(equations) = equations {
                if let Some(equation) = erased_j_equation(env, ctx, evidence, memo) {
                    equations.insert(here, equation);
                }
            }
            Ok(())
        }
        _ => {
            for child in node.children() {
                visit(
                    env,
                    ctx,
                    child,
                    None,
                    next,
                    plan,
                    memo,
                    equations.as_deref_mut(),
                )?;
            }
            Ok(())
        }
    }
}

/// Rewrite the interpreter's already-checked core before value-only CBV eval.
/// Keeping erased binders (and supplying `tt` in their argument slots) means
/// this tree keeps its de Bruijn indexing without a second runtime remapper.
pub fn apply_omega_erasure(
    env: &GlobalEnv,
    term: &Term,
    plan: &OmegaErasurePlan,
) -> Result<Term, KernelError> {
    let mut next = 0u32;
    rewrite(env, term, plan, None, &mut next)
}

fn rewrite(
    env: &GlobalEnv,
    node: &Term,
    plan: &OmegaErasurePlan,
    equations: Option<&JEquations>,
    next: &mut u32,
) -> Result<Term, KernelError> {
    let here = *next;
    *next = next
        .checked_add(1)
        .ok_or_else(|| KernelError::Msg("erasure plan node index overflow".into()))?;
    if plan.erased_subterms.contains(&here) {
        for child in node.children() {
            skipped_nodes(child, next)?;
        }
        return Ok(Term::const_(env.tt_id(), Vec::new()));
    }
    let mut child = |t: &Term| rewrite(env, t, plan, equations, next);
    match node {
        Term::Pair(a, b) if plan.collapsed_sigmas.contains(&here) => {
            let first = child(a)?;
            let _ = child(b)?;
            Ok(first)
        }
        Term::Proj1(p) if plan.collapsed_sigmas.contains(&here) => child(p),
        Term::Pi(a, b) => Ok(Term::pi(child(a)?, child(b)?)),
        Term::Sigma(a, b) => Ok(Term::sigma(child(a)?, child(b)?)),
        Term::Lam(a, b) => Ok(Term::lam(child(a)?, child(b)?)),
        Term::App(f, a) => Ok(Term::app(child(f)?, child(a)?)),
        Term::Pair(a, b) => Ok(Term::pair(child(a)?, child(b)?)),
        Term::Proj1(p) => Ok(Term::proj1(child(p)?)),
        Term::Proj2(p) => Ok(Term::proj2(child(p)?)),
        Term::Ascript(t, ty) => Ok(Term::Ascript(Box::new(child(t)?), Box::new(child(ty)?))),
        Term::Let { ty, val, body } => Ok(Term::Let {
            ty: Box::new(child(ty)?),
            val: Box::new(child(val)?),
            body: Box::new(child(body)?),
        }),
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => Ok(Term::Elim {
            fam: *fam,
            level_args: level_args.clone(),
            params: params.iter().map(&mut child).collect::<Result<_, _>>()?,
            motive: Box::new(child(motive)?),
            methods: methods.iter().map(&mut child).collect::<Result<_, _>>()?,
            indices: indices.iter().map(&mut child).collect::<Result<_, _>>()?,
            scrut: Box::new(child(scrut)?),
        }),
        Term::Eq(a, x, y) => Ok(Term::Eq(
            Box::new(child(a)?),
            Box::new(child(x)?),
            Box::new(child(y)?),
        )),
        Term::Refl(t) => Ok(Term::Refl(Box::new(child(t)?))),
        Term::Cast(a, b, e, t) => Ok(Term::Cast(
            Box::new(child(a)?),
            Box::new(child(b)?),
            Box::new(child(e)?),
            Box::new(child(t)?),
        )),
        Term::J(m, d, e) => {
            let motive = child(m)?;
            let base = child(d)?;
            let erased = child(e)?;
            let evidence = equations
                .and_then(|eqs| eqs.get(&here))
                .cloned()
                .unwrap_or(erased);
            Ok(Term::J(
                Box::new(motive),
                Box::new(base),
                Box::new(evidence),
            ))
        }
        Term::Quot(a, r, e) => Ok(Term::Quot(
            Box::new(child(a)?),
            Box::new(child(r)?),
            Box::new(child(e)?),
        )),
        Term::QuotClass(t) => Ok(Term::QuotClass(Box::new(child(t)?))),
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => Ok(Term::QuotElim {
            motive: Box::new(child(motive)?),
            method: Box::new(child(method)?),
            respect: Box::new(child(respect)?),
            scrut: Box::new(child(scrut)?),
        }),
        Term::Trunc(t) => Ok(Term::Trunc(Box::new(child(t)?))),
        Term::TruncProj(t) => Ok(Term::TruncProj(Box::new(child(t)?))),
        Term::Absurd(motive, proof) => Ok(Term::Absurd(
            Box::new(child(motive)?),
            Box::new(child(proof)?),
        )),
        Term::Type(_)
        | Term::Omega(_)
        | Term::Var(_)
        | Term::Const { .. }
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => Ok(node.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ElabEnv;

    /// Promise class: durable invariant (15 §4, 42 §3.2). MEASURED: the
    /// checked Ω-valued J theorem's erased root is the canonical tt constant,
    /// while the computational J path does not see its original proof body.
    /// CLAIMED: proof J remains erased, not computationally reduced. THE GAP:
    /// this root assertion alone cannot speak for other Ω cast evidence;
    /// existing proof-erasure controls cover those positions.
    #[test]
    fn proof_valued_j_is_erased_before_interpretation() {
        let mut elaborated = ElabEnv::new().expect("checked prelude");
        let id = elaborated
            .elaborate_decl(
                "theorem jProof (x : Bool) (h : Equal Bool x True) : \
                 Equal Bool x True = J (λp _. Equal Bool x True) h h",
            )
            .expect("the proof J must be kernel checked");
        let ken_kernel::Decl::Transparent { body, ty, .. } = elaborated
            .env
            .lookup(id)
            .expect("checked theorem is in the environment")
        else {
            panic!("the theorem must have a checked transparent body");
        };
        let erased = erase_for_interpreter(&elaborated.env, body, ty)
            .expect("interpreter erasure classifies the proof J");
        assert_eq!(erased, Term::const_(elaborated.env.tt_id(), Vec::new()));
    }

    /// Promise class: durable invariant (11 §5). MEASURED: the non-allocating
    /// free-variable walker agrees with kernel weakening on open and closed
    /// terms, including binding and eliminator shapes. CLAIMED: memoizing a
    /// closed classification cannot reuse a context-dependent type result.
    /// THE GAP: a newly added Term variant must be recursed by children().
    #[test]
    fn memo_closedness_matches_kernel_weakening() {
        let int = Term::IntLit(0.into());
        let terms = [
            Term::var(0),
            Term::lam(int.clone(), Term::var(0)),
            Term::lam(int.clone(), Term::var(1)),
            Term::pi(int.clone(), Term::sigma(int.clone(), Term::var(1))),
            Term::Let {
                ty: Box::new(int.clone()),
                val: Box::new(int.clone()),
                body: Box::new(Term::var(0)),
            },
            Term::app(int.clone(), Term::var(0)),
            Term::Elim {
                fam: ken_kernel::GlobalId(0),
                level_args: vec![],
                params: vec![],
                motive: Box::new(int.clone()),
                methods: vec![Term::var(0)],
                indices: vec![],
                scrut: Box::new(int),
            },
        ];
        for term in terms {
            assert_eq!(
                !has_free_var(&term, 0),
                weaken(&term, 1) == term,
                "{term:?}"
            );
        }
    }

    /// Promise class: durable invariant (14 §3.2, 42 §3.2).
    /// MEASURED: a kernel-checked nested-recursion elimination retains a
    /// well-sorted bare-λ motive after normalization, while changing only its
    /// body to a non-type makes the method-type hint and plan both refuse.
    /// CLAIMED: a motive hint may recover an expected method type but may not
    /// admit an ill-sorted motive. THE GAP: this exercises one nested All-lift
    /// family; C2 pins the production admission and byte-count boundary.
    #[test]
    fn nested_method_type_hint_refuses_ill_sorted_motive() {
        let mut elaborated = ElabEnv::new().expect("prelude admits");
        elaborated
            .elaborate_file(
                "data Bag (a : Type) : Type where { Empty : Bag a ; One : a -> Bag a }\n\
                 data Tree = Leaf | Node (Bag Tree)",
            )
            .expect("nested recursive family admits");
        let tree_id = elaborated.globals["Tree"];
        let tree_type = Term::indformer(tree_id, Vec::new());
        let nat_type = Term::indformer(elaborated.globals["Nat"], Vec::new());
        let zero = Term::constructor(elaborated.globals["Zero"], Vec::new());
        let context = Context::new();
        let good_motive = Term::lam(tree_type.clone(), nat_type.clone());
        let bad_motive = Term::lam(tree_type.clone(), Term::var(0));
        let ind = elaborated
            .env
            .inductive(tree_id)
            .expect("Tree family exists");
        assert!(matches!(
            ken_kernel::inductive::method_type(&elaborated.env, ind, 1, &good_motive, &[], &[]),
            Err(KernelError::Msg(reason)) if reason.contains("cannot infer an introduction form")
        ));
        let good_type = method_type_node(&elaborated.env, &context, ind, 1, &good_motive, &[], &[])
            .expect("the kernel checks a well-sorted motive hint");
        assert!(matches!(
            whnf(&elaborated.env, &context, &good_type),
            Term::Pi(..)
        ));
        let bad_type = method_type_node(&elaborated.env, &context, ind, 1, &bad_motive, &[], &[])
            .expect_err("an ill-sorted motive cannot gain a method type from a hint");
        assert!(
            bad_type.to_string().contains("motive result is not a type"),
            "the kernel must reject the motive's result sort: {bad_type}"
        );

        // The checked term is normalized to precisely the bare-λ motive that
        // the package writer sees; the scrutinee is opaque, so no ι fires.
        let opaque = ken_kernel::declare_postulate(
            &mut elaborated.env,
            "tree_probe".into(),
            Vec::new(),
            tree_type.clone(),
        )
        .expect("typed opaque tree admits");
        let ind = elaborated
            .env
            .inductive(tree_id)
            .expect("Tree family exists");
        let hinted_motive = ascribe_motive(&elaborated.env, &mut context.clone(), good_motive)
            .expect("the well-sorted motive can be ascribed");
        let methods = (0..ind.constructors.len())
            .map(|k| {
                let method_type = ken_kernel::inductive::method_type(
                    &elaborated.env,
                    ind,
                    k,
                    &hinted_motive,
                    &[],
                    &[],
                )
                .expect("ascribed method type admits");
                fn constant_zero_method(env: &GlobalEnv, ty: &Term, zero: &Term) -> Term {
                    match whnf(env, &Context::new(), ty) {
                        Term::Pi(domain, codomain) => {
                            Term::lam(*domain, constant_zero_method(env, &codomain, zero))
                        }
                        _ => zero.clone(),
                    }
                }
                constant_zero_method(&elaborated.env, &method_type, &zero)
            })
            .collect::<Vec<_>>();
        let checked = Term::Elim {
            fam: tree_id,
            level_args: Vec::new(),
            params: Vec::new(),
            motive: Box::new(hinted_motive),
            methods,
            indices: Vec::new(),
            scrut: Box::new(Term::const_(opaque, Vec::new())),
        };
        ken_kernel::check(&elaborated.env, &context, &checked, &nat_type)
            .expect("positive eliminator is kernel-checked");
        let normalized = ken_kernel::normalize(&elaborated.env, &context, &checked);
        let Term::Elim { motive, .. } = &normalized else {
            panic!("opaque scrutinee must retain its elimination");
        };
        assert!(matches!(motive.as_ref(), Term::Lam(..)));
        omega_erasure_plan(&elaborated.env, &normalized, &nat_type)
            .expect("well-sorted normalized motive receives a plan");
        let Term::Elim {
            fam,
            level_args,
            params,
            methods,
            indices,
            scrut,
            ..
        } = normalized
        else {
            unreachable!()
        };
        let malformed = Term::Elim {
            fam,
            level_args,
            params,
            motive: Box::new(bad_motive),
            methods,
            indices,
            scrut,
        };
        omega_erasure_plan(&elaborated.env, &malformed, &nat_type)
            .expect_err("an ill-sorted motive must never acquire an erasure plan");
    }

    /// Promise class: transition sentinel for the kernel's current
    /// introduction-inference diagnostic. MEASURED: a reducible checked
    /// type whose unreduced head is a lambda cannot be synthesized, while
    /// its normal form classifies at Ω. CLAIMED: the narrow fallback keeps
    /// eliminator method codomains classifiable. THE GAP: if the kernel's
    /// diagnostic changes, this row must redden until the gate is revisited.
    #[test]
    fn normalized_introduction_type_uses_kernel_omega_fallback() {
        let env = ElabEnv::new().expect("prelude");
        let int = Term::const_(env.globals["Int"], Vec::new());
        let zero = Term::IntLit(0.into());
        let proposition = Term::Eq(
            Box::new(int.clone()),
            Box::new(zero.clone()),
            Box::new(zero.clone()),
        );
        let reduced_type = Term::app(Term::lam(int, proposition), zero);
        let ctx = Context::new();
        assert!(
            matches!(
                infer(&env.env, &ctx, &reduced_type),
                Err(KernelError::Msg(reason)) if reason.contains("cannot infer an introduction form")
            ),
            "fallback must be reached by the expected kernel diagnostic"
        );
        assert!(is_omega_classified(&env.env, &ctx, &reduced_type)
            .expect("kernel normal form of the checked type is Ω"));
    }
}
