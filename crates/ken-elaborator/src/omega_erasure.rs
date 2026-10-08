//! Kernel-classified runtime erasure of checked core. This is an untrusted
//! compilation plan, not a new kernel admission rule.

use std::collections::BTreeSet;

use ken_kernel::subst::{subst0, weaken};
use ken_kernel::{infer, whnf, Context, GlobalEnv, KernelError, Term};

/// Preorder indices include every term in the canonical checked body, including
/// type terms and descendants of a maximal erased subterm.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OmegaErasurePlan {
    pub erased_subterms: BTreeSet<u32>,
    pub erased_binders: BTreeSet<u32>,
    pub collapsed_sigmas: BTreeSet<u32>,
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
            infer(env, ctx, &ken_kernel::normalize(env, ctx, ty))?
        }
        Err(error) => return Err(error),
    };
    Ok(matches!(whnf(env, ctx, &sort), Term::Omega(_)))
}

/// Build the plan from a closed, already admitted body and its checked type.
/// Introduction forms are checked bidirectionally: the kernel does not infer
/// the type of a bare lambda, pair or refl. An unavailable expected type is a
/// refusal, never a guess that the position is computationally relevant.
pub fn omega_erasure_plan(
    env: &GlobalEnv,
    body: &Term,
    checked_type: &Term,
) -> Result<OmegaErasurePlan, KernelError> {
    let mut plan = OmegaErasurePlan::default();
    let mut next = 0u32;
    visit(
        env,
        &Context::new(),
        body,
        Some(checked_type),
        &mut next,
        &mut plan,
    )?;
    Ok(plan)
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
    ctx: &Context,
    node: &Term,
    expected: Option<&Term>,
    next: &mut u32,
    plan: &mut OmegaErasurePlan,
) -> Result<(), KernelError> {
    let here = *next;
    *next = next
        .checked_add(1)
        .ok_or_else(|| KernelError::Msg("erasure plan node index overflow".into()))?;
    // The checked expectation, when present, is the sort plane for
    // non-inferable introduction forms; otherwise use the kernel's inference.
    let inferred = if let Some(ty) = expected {
        Some(ty.clone())
    } else {
        match infer(env, ctx, node) {
            Ok(ty) => Some(ty),
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
        .as_ref()
        .map(|ty| {
            is_omega_classified(env, ctx, ty).map_err(|error| {
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
            visit(env, ctx, dom, None, next, plan)?;
            if is_omega_classified(env, ctx, dom)? {
                plan.erased_binders.insert(here);
            }
            let expected_body = inferred.as_ref().and_then(|ty| match whnf(env, ctx, ty) {
                Term::Pi(_, cod) => Some(*cod),
                _ => None,
            });
            let mut inside = ctx.clone();
            inside.push((**dom).clone());
            visit(env, &inside, body, expected_body.as_ref(), next, plan)
        }
        Term::Pi(dom, cod) | Term::Sigma(dom, cod) => {
            visit(env, ctx, dom, None, next, plan)?;
            let mut inside = ctx.clone();
            inside.push((**dom).clone());
            visit(env, &inside, cod, None, next, plan)
        }
        Term::Let { ty, val, body } => {
            visit(env, ctx, ty, None, next, plan)?;
            if is_omega_classified(env, ctx, ty)? {
                plan.erased_binders.insert(here);
            }
            visit(env, ctx, val, Some(ty), next, plan)?;
            let mut inside = ctx.clone();
            inside.push((**ty).clone());
            let expected_body = inferred.as_ref().map(|t| weaken(t, 1));
            visit(env, &inside, body, expected_body.as_ref(), next, plan)
        }
        Term::App(function, argument) => {
            let function_type = infer(env, ctx, function).map_err(|error| {
                KernelError::Msg(format!(
                    "Ω erasure application function at node {here}: {error}"
                ))
            })?;
            let Term::Pi(domain, _) = whnf(env, ctx, &function_type) else {
                return Err(KernelError::Msg(
                    "checked application has no Pi domain for erasure".into(),
                ));
            };
            visit(env, ctx, function, Some(&function_type), next, plan)?;
            visit(env, ctx, argument, Some(&domain), next, plan)
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
            let mut inside = ctx.clone();
            inside.push((*domain).clone());
            if is_omega_classified(env, &inside, &codomain)? {
                plan.collapsed_sigmas.insert(here);
            }
            visit(env, ctx, first, Some(&domain), next, plan)?;
            let second_type = subst0(&codomain, first);
            visit(env, ctx, second, Some(&second_type), next, plan)
        }
        Term::Proj1(pair) => {
            let pair_type = infer(env, ctx, pair)?;
            let Term::Sigma(domain, codomain) = whnf(env, ctx, &pair_type) else {
                return Err(KernelError::Msg(
                    "checked projection has no Sigma type for erasure".into(),
                ));
            };
            let mut inside = ctx.clone();
            inside.push((*domain).clone());
            if is_omega_classified(env, &inside, &codomain)? {
                plan.collapsed_sigmas.insert(here);
            }
            visit(env, ctx, pair, Some(&pair_type), next, plan)
        }
        Term::Ascript(term, ty) => {
            visit(env, ctx, term, Some(ty), next, plan)?;
            visit(env, ctx, ty, None, next, plan)
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
                visit(env, ctx, param, None, next, plan)?;
            }
            visit(env, ctx, motive, None, next, plan).map_err(|error| {
                KernelError::Msg(format!("Ω erasure eliminator motive: {error}"))
            })?;
            for (k, method) in methods.iter().enumerate() {
                let method_ty =
                    ken_kernel::inductive::method_type(env, ind, k, motive, params, level_args)
                        .map_err(|error| {
                            KernelError::Msg(format!("Ω erasure method type {k}: {error}"))
                        })?;
                visit(env, ctx, method, Some(&method_ty), next, plan).map_err(|error| {
                    KernelError::Msg(format!("Ω erasure eliminator method {k}: {error}"))
                })?;
            }
            for index in indices {
                visit(env, ctx, index, None, next, plan)?;
            }
            visit(env, ctx, scrut, None, next, plan)
        }
        _ => {
            for child in node.children() {
                visit(env, ctx, child, None, next, plan)?;
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
    rewrite(env, term, plan, &mut next)
}

fn rewrite(
    env: &GlobalEnv,
    node: &Term,
    plan: &OmegaErasurePlan,
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
    let mut child = |t: &Term| rewrite(env, t, plan, next);
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
        Term::J(m, d, e) => Ok(Term::J(
            Box::new(child(m)?),
            Box::new(child(d)?),
            Box::new(child(e)?),
        )),
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
        _ => Ok(node.clone()),
    }
}
