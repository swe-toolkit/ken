//! Bidirectional checking/inference and declaration admission (`18 §3`, §4).
//!
//! Two mutually-recursive, syntax-directed modes:
//! - `Γ ⊢ t ⇐ A` — [`check`]: verify `t : A` against a known type.
//! - `Γ ⊢ t ⇒ A` — [`infer`]: produce the unique `A` with `t : A`.
//!
//! The single place conversion ([`crate::conv::convert`]) is called during
//! checking is the **mode switch** (`18 §3`): any term without a
//! type-driven check rule infers its type and is compared to the expected one.
//! `check`/`infer` reject the `[K2]`-reserved formers as unrecognised (`11 §6`).
//! Admission ([`declare_def`] … [`declare_primitive`]) re-checks every input and
//! gates inductives on strict positivity (`14 §8`).

use crate::conv::{convert_type, level_eq, whnf};
use crate::env::{
    telescope_to_pi, AllSupportSort, CheckedStringLiteral, Context, Decl, GlobalEnv, InductiveDecl,
    PrimReduction,
};
use crate::error::{KernelError, KernelResult};
use crate::inductive::{
    build_all_support_decl, check_positivity, check_support_positivity, method_type,
};
use crate::subst::{apply_args, subst0, subst_levels, subst_outer, subst_tel, weaken};
use crate::term::{GlobalId, Level, LevelVar, Term};
use std::collections::HashSet;
use unicode_normalization::UnicodeNormalization;

// --- raw well-formedness (`11 §6`) -----------------------------------------

/// Raw well-formedness: `t` is built by the grammar and every de Bruijn index
/// resolves to an in-scope binding (`11 §6`). This is the parser/elaborator's
/// precondition to typing — it does **not** decide typing. `offset` is the
/// number of binders entered inside `t` (bound vars `i < offset` are t's own).
fn raw_wf(ctx: &Context, t: &Term, offset: usize) -> KernelResult<()> {
    match t {
        Term::Var(i) => {
            if *i < offset {
                Ok(())
            } else {
                ctx.lookup(i - offset)
                    .map(|_| ())
                    .ok_or(KernelError::VarOutOfScope {
                        index: *i,
                        depth: ctx.len() + offset,
                    })
            }
        }
        Term::Pi(a, b) | Term::Lam(a, b) | Term::Sigma(a, b) => {
            raw_wf(ctx, a, offset)?;
            raw_wf(ctx, b, offset + 1)
        }
        Term::Let { ty, val, body } => {
            raw_wf(ctx, ty, offset)?;
            raw_wf(ctx, val, offset)?;
            raw_wf(ctx, body, offset + 1)
        }
        Term::App(f, a) | Term::Pair(f, a) | Term::Ascript(f, a) | Term::Quot(f, a) => {
            raw_wf(ctx, f, offset)?;
            raw_wf(ctx, a, offset)
        }
        Term::Proj1(p)
        | Term::Proj2(p)
        | Term::Refl(p)
        | Term::QuotClass(p)
        | Term::Trunc(p)
        | Term::TruncProj(p) => raw_wf(ctx, p, offset),
        Term::Eq(a, t, u) => {
            raw_wf(ctx, a, offset)?;
            raw_wf(ctx, t, offset)?;
            raw_wf(ctx, u, offset)
        }
        Term::Cast(a, b, e, t) => {
            raw_wf(ctx, a, offset)?;
            raw_wf(ctx, b, offset)?;
            raw_wf(ctx, e, offset)?;
            raw_wf(ctx, t, offset)
        }
        Term::J(m, d, e) => {
            raw_wf(ctx, m, offset)?;
            raw_wf(ctx, d, offset)?;
            raw_wf(ctx, e, offset)
        }
        Term::Elim {
            params,
            motive,
            methods,
            indices,
            scrut,
            ..
        } => {
            for p in params {
                raw_wf(ctx, p, offset)?;
            }
            raw_wf(ctx, motive, offset)?;
            for m in methods {
                raw_wf(ctx, m, offset)?;
            }
            for i in indices {
                raw_wf(ctx, i, offset)?;
            }
            raw_wf(ctx, scrut, offset)
        }
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => {
            raw_wf(ctx, motive, offset)?;
            raw_wf(ctx, method, offset)?;
            raw_wf(ctx, respect, offset)?;
            raw_wf(ctx, scrut, offset)
        }
        Term::Absurd(motive, proof) => {
            raw_wf(ctx, motive, offset)?;
            raw_wf(ctx, proof, offset)
        }
        // Closed in Σ: no free term variables (levels are not de Bruijn).
        Term::Type(_)
        | Term::Omega(_)
        | Term::Const { .. }
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => Ok(()),
    }
}

/// Check that a declaration's level variables are bound by distinct level
/// parameters. This is separate from term-variable raw well-formedness: levels
/// also occur in universe formers and in explicit global/eliminator arguments.
fn check_level_closure<'a>(
    level_params: &[LevelVar],
    terms: impl IntoIterator<Item = &'a Term>,
) -> KernelResult<()> {
    let allowed: HashSet<_> = level_params.iter().copied().collect();
    if allowed.len() != level_params.len() {
        return Err(KernelError::IllFormedDecl(
            "duplicate level parameter".into(),
        ));
    }

    let mut pending: Vec<&Term> = terms.into_iter().collect();
    while let Some(term) = pending.pop() {
        let levels: &[Level] = match term {
            Term::Type(level) | Term::Omega(level) => std::slice::from_ref(level),
            Term::Const { level_args, .. }
            | Term::IndFormer { level_args, .. }
            | Term::Constructor { level_args, .. }
            | Term::Elim { level_args, .. } => level_args,
            // Name every variant with no direct Level field. A new Term
            // carrying levels must be reviewed here; `children()` alone
            // cannot expose a level or a level-argument list.
            Term::Var(_)
            | Term::IntLit(_)
            | Term::Pi(..)
            | Term::Lam(..)
            | Term::App(..)
            | Term::Sigma(..)
            | Term::Pair(..)
            | Term::Proj1(_)
            | Term::Proj2(_)
            | Term::Let { .. }
            | Term::Ascript(..)
            | Term::Eq(..)
            | Term::Refl(_)
            | Term::Cast(..)
            | Term::J(..)
            | Term::Quot(..)
            | Term::QuotClass(_)
            | Term::QuotElim { .. }
            | Term::Trunc(_)
            | Term::TruncProj(_)
            | Term::Absurd(..) => &[],
        };
        for level in levels {
            let mut level_nodes = vec![level];
            while let Some(node) = level_nodes.pop() {
                match node {
                    Level::Zero => {}
                    Level::Var(var) if allowed.contains(var) => {}
                    Level::Var(var) => {
                        return Err(KernelError::IllFormedDecl(format!(
                            "undeclared level variable {var:?}"
                        )));
                    }
                    Level::Suc(inner) => level_nodes.push(inner),
                    Level::Max(left, right) => {
                        level_nodes.push(left);
                        level_nodes.push(right);
                    }
                }
            }
        }
        pending.extend(term.children());
    }
    Ok(())
}

/// Inductive declarations also carry a standalone family level and generated
/// former/constructor types, in addition to their source telescopes.
fn check_inductive_level_closure(ind: &InductiveDecl) -> KernelResult<()> {
    let family_level = Term::Type(ind.level.clone());
    let mut terms = vec![&family_level, &ind.former_type];
    terms.extend(ind.params.iter());
    terms.extend(ind.indices.iter());
    for constructor in &ind.constructors {
        terms.extend(constructor.args.iter());
        terms.extend(constructor.target_indices.iter());
        terms.push(&constructor.type_);
    }
    check_level_closure(&ind.level_params, terms)
}

/// Raw well-formedness check (public, for the elaborator precondition).
pub fn raw_well_formed(ctx: &Context, t: &Term) -> KernelResult<()> {
    raw_wf(ctx, t, 0)
}

// --- type synthesis: `Γ ⊢ A type` ⇒ its level -----------------------------

/// The universe a type inhabits (`11 §3`): `Type ℓ` or the strict-proposition
/// universe `Ω_ℓ` (`16 §1.1`). A binder type, declaration type, or ascription
/// may be either — a proposition is a valid type.
#[derive(Clone, Debug)]
pub(crate) enum Sort {
    Type(Level),
    Omega(Level),
}

impl Sort {
    pub(crate) fn level(&self) -> &Level {
        match self {
            Sort::Type(l) | Sort::Omega(l) => l,
        }
    }
    /// Reify the sort as a term (`Type ℓ` or `Ω_ℓ`).
    fn to_term(&self) -> Term {
        match self {
            Sort::Type(l) => Term::Type(l.clone()),
            Sort::Omega(l) => Term::Omega(l.clone()),
        }
    }
}

/// `Γ ⊢ A type` ⇒ the sort (and level) of `A` (`11 §3`: a type is `Type ℓ` or
/// `Ω_ℓ`). Generalizes [`synth_type`] to admit proposition types (`16 §1.1`).
pub(crate) fn classify(env: &GlobalEnv, ctx: &Context, a: &Term) -> KernelResult<Sort> {
    let ty = infer(env, ctx, a)?;
    match whnf(env, ctx, &ty) {
        Term::Type(l) => Ok(Sort::Type(l)),
        Term::Omega(l) => Ok(Sort::Omega(l)),
        other => Err(KernelError::TypeMismatch {
            expected: Box::new(Term::Type(Level::Var(LevelVar(0)))), // a type
            found: Box::new(other),
        }),
    }
}

/// Check `Γ ⊢ A : Type ℓ` and return its level — the Type-only specialization
/// of [`classify`]. Use [`classify`] where a proposition type is admissible
/// (binders, declarations, ascriptions); use this where a `Type` level is
/// required specifically (e.g. an inductive family's universe).
fn synth_type(env: &GlobalEnv, ctx: &Context, a: &Term) -> KernelResult<Level> {
    match classify(env, ctx, a)? {
        Sort::Type(l) => Ok(l),
        Sort::Omega(l) => Err(KernelError::TypeMismatch {
            expected: Box::new(Term::Type(Level::Var(LevelVar(0)))),
            found: Box::new(Term::Omega(l)),
        }),
    }
}

/// Formation sort of a Π-type (`13 §1`, `13 §4`, `16 §1.1`): level is
/// `max(s1,s2)`; result is Ω exactly when the **codomain** is a proposition
/// (a function into a prop is a prop, regardless of the domain's sort).
fn sort_pi(s1: &Sort, s2: &Sort) -> Term {
    let lvl = s1.level().clone().max(s2.level().clone()).normalize();
    match s2 {
        Sort::Omega(_) => Term::Omega(lvl),
        Sort::Type(_) => Term::Type(lvl),
    }
}

/// Formation sort of a Σ-type (`13 §2`, `13 §4`): level is `max(s1,s2)`;
/// result is Ω only when **both** components are propositions (the conjunction
/// case). A subset with a **relevant** (`Type`-sorted) first component carries
/// content and must stay in `Type` — collapsing it to Ω would trigger Ω-PI
/// proof-irrelevance on the carrier, closing to `Empty` via a transport motive.
fn sort_sigma(s1: &Sort, s2: &Sort) -> Term {
    let lvl = s1.level().clone().max(s2.level().clone()).normalize();
    match (s1, s2) {
        (Sort::Omega(_), Sort::Omega(_)) => Term::Omega(lvl),
        _ => Term::Type(lvl),
    }
}

// --- infer (`18 §3`) -------------------------------------------------------

/// `Γ ⊢ t ⇒ A` — infer the type of `t` (`18 §3`). Fails for `[K2]`-reserved
/// formers and for λ/pair (which need a type to check against).
pub fn infer(env: &GlobalEnv, ctx: &Context, t: &Term) -> KernelResult<Term> {
    match t {
        Term::Var(i) => {
            ctx.lookup(*i)
                .map(|t| weaken(t, (*i + 1) as i64))
                .ok_or(KernelError::VarOutOfScope {
                    index: *i,
                    depth: ctx.len(),
                })
        }
        Term::Const { id, level_args } => {
            let (params, ty) = env
                .const_type(*id)
                .ok_or_else(|| KernelError::Msg(format!("unknown constant {:?}", id)))?;
            check_level_arity(params, level_args)?;
            Ok(subst_levels(&ty, params, level_args))
        }
        Term::IntLit(_) => {
            let id = env
                .int_lit_type()
                .ok_or_else(|| KernelError::Msg("Int-literal type not registered".into()))?;
            Ok(Term::const_(id, Vec::new()))
        }
        Term::IndFormer { id, level_args } => {
            let (params, ty) = env
                .const_type(*id)
                .ok_or_else(|| KernelError::Msg(format!("unknown type former {:?}", id)))?;
            check_level_arity(params, level_args)?;
            Ok(subst_levels(&ty, params, level_args))
        }
        Term::Constructor { id, level_args } => {
            let (ind, k) = env
                .constructor(*id)
                .ok_or_else(|| KernelError::Msg(format!("unknown constructor {:?}", id)))?;
            check_level_arity(&ind.level_params, level_args)?;
            Ok(subst_levels(
                &ind.constructors[k].type_,
                &ind.level_params,
                level_args,
            ))
        }
        Term::App(f, a) => {
            let tf = infer(env, ctx, f)?;
            match whnf(env, ctx, &tf) {
                Term::Pi(dom, cod) => {
                    check(env, ctx, a, &dom)?;
                    Ok(subst0(&cod, a))
                }
                other => Err(KernelError::NotAFunction {
                    head: Box::new(other),
                }),
            }
        }
        Term::Proj1(p) => {
            let tp = infer(env, ctx, p)?;
            match whnf(env, ctx, &tp) {
                Term::Sigma(dom, _) => Ok((*dom).clone()),
                other => Err(KernelError::NotASigma {
                    head: Box::new(other),
                }),
            }
        }
        Term::Proj2(p) => {
            let tp = infer(env, ctx, p)?;
            match whnf(env, ctx, &tp) {
                Term::Sigma(_, cod) => Ok(subst0(&cod, &Term::proj1((**p).clone()))),
                other => Err(KernelError::NotASigma {
                    head: Box::new(other),
                }),
            }
        }
        Term::Pi(a, b) => {
            let s1 = classify(env, ctx, a)?;
            let mut ctx2 = ctx.clone();
            ctx2.push((**a).clone());
            let s2 = classify(env, &ctx2, b)?;
            Ok(sort_pi(&s1, &s2))
        }
        Term::Sigma(a, b) => {
            let s1 = classify(env, ctx, a)?;
            let mut ctx2 = ctx.clone();
            ctx2.push((**a).clone());
            let s2 = classify(env, &ctx2, b)?;
            Ok(sort_sigma(&s1, &s2))
        }
        Term::Type(l) => Ok(Term::Type(l.clone().suc())), // (U-Type): Type ℓ : Type (suc ℓ) (`12 §1`)
        Term::Omega(l) => Ok(Term::Type(l.clone().suc())), // (Ω-Form): Ω_l : Type (suc l) (`16 §1.1`)
        Term::Ascript(t, a) => {
            classify(env, ctx, a)?;
            check(env, ctx, t, a)?;
            Ok((**a).clone())
        }
        Term::Let { ty, val, body } => {
            classify(env, ctx, ty)?;
            check(env, ctx, val, ty)?;
            infer(env, ctx, &subst0(body, val))
        }
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => infer_elim(
            env, ctx, *fam, level_args, params, motive, methods, indices, scrut,
        ),
        // --- K2 formers (`15`, `16`) ---
        Term::Eq(a_ty, x, y) => {
            // `Eq A a b : Ω_l` for `A : Type l` (`16 §2.1`).
            let l = synth_type(env, ctx, a_ty)?;
            check(env, ctx, x, a_ty)?;
            check(env, ctx, y, a_ty)?;
            Ok(Term::Omega(l))
        }
        Term::Cast(a_ty, b_ty, e, t) => {
            // `cast A B e a : B`, `e : Eq Type A B` (`16 §3.1`).
            let l_a = synth_type(env, ctx, a_ty)?;
            let l_b = synth_type(env, ctx, b_ty)?;
            if !level_eq(&l_a, &l_b) {
                return Err(KernelError::TypeMismatch {
                    expected: Box::new(Term::Type(l_a)),
                    found: Box::new(Term::Type(l_b)),
                });
            }
            let eq_ty = Term::Eq(
                Box::new(Term::Type(l_a)),
                Box::new((**a_ty).clone()),
                Box::new((**b_ty).clone()),
            );
            check(env, ctx, e, &eq_ty)?;
            check(env, ctx, t, a_ty)?;
            Ok((**b_ty).clone())
        }
        Term::J(m, d, e) => infer_j(env, ctx, m, d, e),
        Term::Quot(a, r) => {
            // `A / R : Type l` for `R : A → A → Ω` (`16 §5`).
            let l = synth_type(env, ctx, a)?;
            check_quotient_rel(env, ctx, a, r)?;
            Ok(Term::Type(l))
        }
        Term::Trunc(a) => {
            // `‖A‖ : Ω_l` for `A : Type l` (`16 §6`).
            let l = synth_type(env, ctx, a)?;
            Ok(Term::Omega(l))
        }
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => infer_quot_elim(env, ctx, motive, method, respect, scrut),
        Term::Absurd(motive, proof) => infer_absurd(env, ctx, motive, proof),
        Term::Lam { .. }
        | Term::Pair { .. }
        | Term::Refl(_)
        | Term::QuotClass(_)
        | Term::TruncProj(_) => Err(KernelError::Msg(
            "cannot infer an introduction form (λ/pair/refl/quotient class/truncation) \
             without an expected type (use ascription)"
                .into(),
        )),
    }
}

fn check_level_arity(params: &[LevelVar], args: &[Level]) -> KernelResult<()> {
    if params.len() == args.len() {
        Ok(())
    } else {
        Err(KernelError::LevelArityMismatch {
            expected: params.len(),
            found: args.len(),
        })
    }
}

// --- check (`18 §3`) -------------------------------------------------------

/// Eliminate recursive `check(App(f, a)) -> infer(App) -> check(a)` frames on
/// constructor spines. The argument check must precede conversion of the
/// inferred application type; otherwise an unchecked operand could influence
/// the type's normalization. Deferred comparisons preserve that order.
fn check_app_spine(env: &GlobalEnv, ctx: &Context, t: &Term, ty: &Term) -> KernelResult<()> {
    let mut argument = t;
    let mut expected = ty.clone();
    let mut pending = Vec::new();
    while let Term::App(f, a) = argument {
        let tf = infer(env, ctx, f)?;
        let (dom, cod) = match whnf(env, ctx, &tf) {
            Term::Pi(dom, cod) => (dom, cod),
            other => {
                return Err(KernelError::NotAFunction {
                    head: Box::new(other),
                })
            }
        };
        pending.push((expected, subst0(&cod, a)));
        expected = *dom;
        argument = a;
    }
    check(env, ctx, argument, &expected)?;
    for (expected, inferred) in pending.into_iter().rev() {
        if !convert_type(env, ctx, &expected, &inferred) {
            return Err(KernelError::TypeMismatch {
                expected: Box::new(expected),
                found: Box::new(inferred),
            });
        }
    }
    Ok(())
}

/// `Γ ⊢ t ⇐ A` — check `t` against a known type (`18 §3`). Type-driven rules
/// for λ (Π) and pair (Σ) insert η-relevant structure; everything else falls
/// to the mode switch (infer + conversion).
pub fn check(env: &GlobalEnv, ctx: &Context, t: &Term, ty: &Term) -> KernelResult<()> {
    // Application arguments can themselves be constructor applications: a
    // List spine nests three Apps per Cons. Check its argument chain with an
    // explicit worklist rather than retaining one infer/check stack per cell.
    // Each deferred type comparison runs only after its argument was checked,
    // in exactly the same inner-to-outer order as infer(App)'s mode switch.
    if matches!(t, Term::App(..)) {
        return check_app_spine(env, ctx, t, ty);
    }
    match t {
        Term::Lam(a, body) => match whnf(env, ctx, ty) {
            Term::Pi(dom, cod) => {
                classify(env, ctx, a)?;
                if !convert_type(env, ctx, a, &dom) {
                    return Err(KernelError::TypeMismatch {
                        expected: Box::new((*dom).clone()),
                        found: Box::new((**a).clone()),
                    });
                }
                let mut ctx2 = ctx.clone();
                ctx2.push((*dom).clone());
                check(env, &ctx2, body, &cod)
            }
            other => Err(KernelError::NotAFunction {
                head: Box::new(other),
            }),
        },
        Term::Pair(a, b) => match whnf(env, ctx, ty) {
            Term::Sigma(dom, cod) => {
                check(env, ctx, a, &dom)?;
                let cod_a = subst0(&cod, a);
                check(env, ctx, b, &cod_a)
            }
            other => Err(KernelError::NotASigma {
                head: Box::new(other),
            }),
        },
        Term::Ascript(t, a) => {
            classify(env, ctx, a)?;
            check(env, ctx, t, a)?;
            if !convert_type(env, ctx, a, ty) {
                return Err(KernelError::TypeMismatch {
                    expected: Box::new(ty.clone()),
                    found: Box::new((**a).clone()),
                });
            }
            Ok(())
        }
        Term::Let {
            ty: let_ty,
            val,
            body,
        } => {
            classify(env, ctx, let_ty)?;
            check(env, ctx, val, let_ty)?;
            check(env, ctx, &subst0(body, val), ty)
        }
        // --- K2 introduction forms (`15`, `16`) ---
        Term::Refl(a) => {
            // `refl a` checks at any type convertible to `Eq (infer a) a a`
            // (`15 §2`); do not whnf the expected proposition first, since
            // a compound Eq Type may decompose into a Σ/Π proposition.
            let a_infer = infer(env, ctx, a)?;
            let inferred = Term::Eq(
                Box::new(a_infer),
                Box::new((**a).clone()),
                Box::new((**a).clone()),
            );
            if convert_type(env, ctx, ty, &inferred) {
                Ok(())
            } else {
                Err(KernelError::TypeMismatch {
                    expected: Box::new(ty.clone()),
                    found: Box::new(inferred),
                })
            }
        }
        Term::QuotClass(a) => {
            // `[a] : A / R`  iff  `a : A` (`16 §5`).
            let ty_w = whnf(env, ctx, ty);
            match &ty_w {
                Term::Quot(a_ty, _r) => check(env, ctx, a, a_ty),
                _ => Err(KernelError::TypeMismatch {
                    expected: Box::new(ty.clone()),
                    found: Box::new(ty_w.clone()),
                }),
            }
        }
        Term::TruncProj(a) => {
            // `|a| : ‖A‖`  iff  `a : A` (`16 §6`).
            let ty_w = whnf(env, ctx, ty);
            match &ty_w {
                Term::Trunc(a_ty) => check(env, ctx, a, a_ty),
                _ => Err(KernelError::TypeMismatch {
                    expected: Box::new(ty.clone()),
                    found: Box::new(ty_w.clone()),
                }),
            }
        }
        _ => {
            // Mode switch (`18 §3`): infer t's type and convert it to the
            // expected one — the single place conversion is called in check.
            let inferred = infer(env, ctx, t)?;
            if convert_type(env, ctx, ty, &inferred) {
                Ok(())
            } else {
                Err(KernelError::TypeMismatch {
                    expected: Box::new(ty.clone()),
                    found: Box::new(inferred),
                })
            }
        }
    }
}

// --- dependent eliminator inference (`14 §3`, §7) --------------------------

/// Infer the type of `elim_D p̄ M m̄ i̅ s` = `M i̅ s`, after checking the
/// motive, methods, indices, and scrutinee against the family declaration.
#[allow(clippy::too_many_arguments)]
fn infer_elim(
    env: &GlobalEnv,
    ctx: &Context,
    fam: GlobalId,
    level_args: &[Level],
    params: &[Term],
    motive: &Term,
    methods: &[Term],
    indices: &[Term],
    scrut: &Term,
) -> KernelResult<Term> {
    let ind = env
        .inductive(fam)
        .ok_or_else(|| KernelError::Msg(format!("elim of unknown family {:?}", fam)))?;
    check_level_arity(&ind.level_params, level_args)?;
    let m = ind.params.len();
    let n_i = ind.indices.len();

    // 1. Check the family params p̄ against Δ_p (level-instantiated, earlier
    //    params substituted).
    let inst_params: Vec<Term> = ind
        .params
        .iter()
        .map(|p| subst_levels(p, &ind.level_params, level_args))
        .collect();
    if params.len() != m {
        return Err(KernelError::BadEliminator(format!(
            "expected {m} params, got {}",
            params.len()
        )));
    }
    for (j, pa) in params.iter().enumerate() {
        let pty = subst_tel(&inst_params[j], &params[..j]);
        check(env, ctx, pa, &pty)?;
    }

    // 2. Verify the motive M : (Δ_i) → D p̄ Δ_i → Type ℓ' or Ω_ℓ' (extract the
    //    sort, then re-check against the fully-built expected motive type).
    let motive_sort = infer_motive_level(env, ctx, ind, level_args, params, motive)?;
    let expected_motive = motive_expected_type(ind, level_args, params, &motive_sort);
    check(env, ctx, motive, &expected_motive)?;

    // 3. Check one method per constructor against its method type.
    if methods.len() != ind.constructors.len() {
        return Err(KernelError::BadEliminator(format!(
            "expected {} methods, got {}",
            ind.constructors.len(),
            methods.len()
        )));
    }
    for (k, m) in methods.iter().enumerate() {
        let mt = method_type(env, ind, k, motive, params, level_args)?;
        check(env, ctx, m, &mt)?;
    }

    // 4. Check the index arguments i̅ against Δ_i (params substituted).
    let inst_indices: Vec<Term> = ind
        .indices
        .iter()
        .map(|i| subst_levels(i, &ind.level_params, level_args))
        .collect();
    if indices.len() != n_i {
        return Err(KernelError::BadEliminator(format!(
            "expected {n_i} indices, got {}",
            indices.len()
        )));
    }
    for (j, ix) in indices.iter().enumerate() {
        let ity = subst_tel(&subst_outer(&inst_indices[j], m, params, j), &indices[..j]);
        check(env, ctx, ix, &ity)?;
    }

    // 5. Check the scrutinee s ⇐ D p̄ i̅.
    let mut d_app = Term::IndFormer {
        id: fam,
        level_args: level_args.to_vec(),
    };
    for p in params {
        d_app = Term::app(d_app, p.clone());
    }
    for ix in indices {
        d_app = Term::app(d_app, ix.clone());
    }
    check(env, ctx, scrut, &d_app)?;

    // 6. Result type: M i̅ s (`14 §3`). Valid by M's checked type + i̅ + s.
    let mut result = motive.clone();
    for ix in indices {
        result = Term::app(result, ix.clone());
    }
    result = Term::app(result, scrut.clone());
    Ok(result)
}

/// Extract the motive's result sort by peeling `n_i + 1` Π binders from the
/// motive's inferred type (the index binders, then the scrutinee binder),
/// requiring the body to be `Type ℓ'` or `Ω_ℓ'` (`16 §1.1`) — a per-branch
/// proposition may be proved by case-split on a relevant scrutinee, exactly
/// as a per-branch type may be selected. Loosely verifies the motive's shape;
/// [`infer_elim`] re-checks it fully against [`motive_expected_type`].
pub(crate) fn infer_motive_level(
    env: &GlobalEnv,
    ctx: &Context,
    ind: &InductiveDecl,
    _level_args: &[Level],
    _params: &[Term],
    motive: &Term,
) -> KernelResult<Sort> {
    let n_i = ind.indices.len();
    let mty = infer(env, ctx, motive)?;
    let mut cur = whnf(env, ctx, &mty);
    let mut mctx = ctx.clone();
    for _ in 0..n_i {
        match whnf(env, &mctx, &cur) {
            Term::Pi(a, b) => {
                mctx.push((*a).clone());
                cur = (*b).clone();
            }
            _ => {
                return Err(KernelError::BadEliminator(
                    "motive is not a Π over the family indices".into(),
                ))
            }
        }
    }
    match whnf(env, &mctx, &cur) {
        Term::Pi(d_app, ret) => {
            mctx.push((*d_app).clone());
            match whnf(env, &mctx, &ret) {
                Term::Type(l) => Ok(Sort::Type(l.clone())),
                Term::Omega(l) => Ok(Sort::Omega(l.clone())),
                _ => Err(KernelError::BadEliminator(
                    "motive result is not a type or a proposition (Type ℓ' or Ω_ℓ')".into(),
                )),
            }
        }
        _ => Err(KernelError::BadEliminator(
            "motive is not a Π over a D-value".into(),
        )),
    }
}

/// Build the expected motive type `(Δ_i) → D p̄ Δ_i → Type ℓ'` (or `Ω_ℓ'`) in
/// the caller's context Γ (params p̄ fixed, indices abstracted).
fn motive_expected_type(
    ind: &InductiveDecl,
    level_args: &[Level],
    params: &[Term],
    motive_sort: &Sort,
) -> Term {
    let m = ind.params.len();
    let n_i = ind.indices.len();
    let inst_indices: Vec<Term> = ind
        .indices
        .iter()
        .map(|i| subst_levels(i, &ind.level_params, level_args))
        .collect();
    // Index binders in [Γ, idx 0..j-1]: params substituted, earlier indices kept.
    let idx_types: Vec<Term> = (0..n_i)
        .map(|j| subst_outer(&inst_indices[j], m, params, j))
        .collect();
    // D p̄ Δ_i in [Γ, idx 0..n_i-1]: params weakened past the indices, idx vars.
    let mut d_app = Term::IndFormer {
        id: ind.id,
        level_args: level_args.to_vec(),
    };
    for p in params {
        d_app = Term::app(d_app, weaken(p, n_i as i64));
    }
    for j in 0..n_i {
        d_app = Term::app(d_app, Term::var(n_i - 1 - j));
    }
    let ret = Term::pi(d_app, motive_sort.to_term());
    telescope_to_pi(&idx_types, ret)
}

// --- K2 quotient / J inference (`15 §4`, `16 §5`, §6) ---------------------

/// Infer the type of `J motive base eq` = `motive b eq` (`15 §4`). Recover
/// its endpoints from the proof's recorded Eq, verify the motive's first
/// domain, check `base : motive a (refl a)`, and return `motive b eq`.
/// Peel only β/δ/let/ascription/function heads of a recorded Eq formation;
/// never reduce the Eq head itself when reading the recording.
fn eq_formation_head(env: &GlobalEnv, ty: &Term) -> Term {
    let mut current = ty.clone();
    loop {
        current = match current {
            Term::Ascript(inner, _) => *inner,
            Term::Let { val, body, .. } => subst0(&body, &val),
            Term::Const { id, level_args } => {
                if let Some((params, body)) = env.transparent_body(id) {
                    subst_levels(&body, &params, &level_args)
                } else {
                    return Term::Const { id, level_args };
                }
            }
            Term::App(f, a) => match eq_formation_head(env, &f) {
                Term::Lam(_, body) => subst0(&body, &a),
                head => return Term::app(head, *a),
            },
            head => return head,
        };
    }
}

/// Read J's recorded Eq endpoints. A checked ascription is authoritative:
/// reducing its Eq head could instead expose a different Eq or a Sigma.
/// For an unrecorded proof, retain the old whnf-and-Eq demand exactly.
pub(crate) fn j_endpoints(
    env: &GlobalEnv,
    ctx: &Context,
    eq: &Term,
) -> KernelResult<(Term, Term, Term)> {
    if let Term::Ascript(_, recorded_ty) = eq {
        infer(env, ctx, eq)?; // Check the recording before reducing it.
        if let Term::Eq(a, x, y) = eq_formation_head(env, recorded_ty) {
            return Ok((*a, *x, *y));
        }
    }
    let eq_ty = infer(env, ctx, eq)?;
    match whnf(env, ctx, &eq_ty) {
        Term::Eq(a, x, y) => Ok((*a, *x, *y)),
        _ => Err(KernelError::BadEliminator(
            "J's equality argument is not an `Eq`".into(),
        )),
    }
}

fn infer_j(
    env: &GlobalEnv,
    ctx: &Context,
    motive: &Term,
    base: &Term,
    eq: &Term,
) -> KernelResult<Term> {
    // e : Eq A a b ⇒ use its recorded formation, or the old raw fallback.
    let (a_ty, a_idx, b_idx) = j_endpoints(env, ctx, eq)?;
    // motive : (b:A) → (e':Eq A a b) → Type ℓ'. Verify the first domain ≡ A.
    let m_ty = infer(env, ctx, motive)?;
    match &whnf(env, ctx, &m_ty) {
        Term::Pi(m_dom, _) => {
            if !convert_type(env, ctx, m_dom, &a_ty) {
                return Err(KernelError::BadEliminator(
                    "J motive's first domain ≠ the equality's type A".into(),
                ));
            }
        }
        _ => {
            return Err(KernelError::BadEliminator(
                "J motive is not a Π over A".into(),
            ))
        }
    }
    // base : motive a (refl a).
    let base_ty = Term::app(
        Term::app(motive.clone(), a_idx.clone()),
        Term::Refl(Box::new(a_idx.clone())),
    );
    check(env, ctx, base, &base_ty)?;
    // Result: motive b eq.
    Ok(Term::app(
        Term::app(motive.clone(), b_idx.clone()),
        eq.clone(),
    ))
}

/// Check `R : A → A → Ω` (the quotient relation, `16 §5`): infer `R`'s type and
/// verify the Π–Π–Ω shape with the first domain ≡ `A`. (The second domain is
/// `A` under the first binder; a strict check needs a context shift, so only the
/// shape and first domain are verified — sound for well-elaborated input.)
fn check_quotient_rel(env: &GlobalEnv, ctx: &Context, a: &Term, r: &Term) -> KernelResult<()> {
    let r_ty = infer(env, ctx, r)?;
    let cod1 = match &whnf(env, ctx, &r_ty) {
        Term::Pi(dom1, cod1) => {
            if !convert_type(env, ctx, dom1, a) {
                return Err(KernelError::BadEliminator(
                    "quotient relation's first domain ≠ A".into(),
                ));
            }
            (**cod1).clone()
        }
        _ => {
            return Err(KernelError::BadEliminator(
                "quotient relation is not of type A → A → Ω".into(),
            ))
        }
    };
    let cod2 = match &whnf(env, ctx, &cod1) {
        Term::Pi(_, cod2) => (**cod2).clone(),
        _ => {
            return Err(KernelError::BadEliminator(
                "quotient relation is not of type A → A → Ω".into(),
            ))
        }
    };
    match &whnf(env, ctx, &cod2) {
        Term::Omega(_) => Ok(()),
        _ => Err(KernelError::BadEliminator(
            "quotient relation's codomain is not Ω".into(),
        )),
    }
}

/// Infer `elim_/ M f r q : M q` (`16 §5`), also covering `elim_trunc`
/// (encoded as `QuotElim` on a `‖A‖` scrut, `16 §6`). Checks the motive,
/// method, and (for non-Ω targets) the respect proof; Ω targets are
/// respect-free (`16 §5`).
fn infer_quot_elim(
    env: &GlobalEnv,
    ctx: &Context,
    motive: &Term,
    method: &Term,
    respect: &Term,
    scrut: &Term,
) -> KernelResult<Term> {
    // scrut : A/R (or ‖A‖). Recover the underlying `A` and relation (if Quot).
    let scrut_ty = infer(env, ctx, scrut)?;
    let scrut_whnf = whnf(env, ctx, &scrut_ty);
    let (underlying_a, opt_rel) = match scrut_whnf {
        Term::Quot(a, r) => (*a, Some(*r)),
        Term::Trunc(a) => (*a, None),
        _ => {
            return Err(KernelError::BadEliminator(
                "quotient elim scrutinee is not a quotient or truncation".into(),
            ))
        }
    };
    // motive M : (z : scrut_ty) → Type ℓ'. Verify the Π shape and codomain Type.
    let m_ty = infer(env, ctx, motive)?;
    let m_cod = match &whnf(env, ctx, &m_ty) {
        Term::Pi(dom, cod) => {
            if !convert_type(env, ctx, dom, &scrut_ty) {
                return Err(KernelError::BadEliminator(
                    "motive's domain ≠ scrutinee type".into(),
                ));
            }
            (**cod).clone()
        }
        _ => {
            return Err(KernelError::BadEliminator(
                "motive is not a Π over the quotient".into(),
            ))
        }
    };
    // Motive codomain sort ⇒ target kind (§5):
    //   Ω_l ⇒ respect-free (Ω-PI); Type ℓ ⇒ verify cong/cast schema (§5.1).
    let type_target = match whnf(env, ctx, &m_cod) {
        Term::Omega(_) => false,
        Term::Type(_) => true,
        _ => {
            return Err(KernelError::BadEliminator(
                "motive's codomain is not a type (Type ℓ' or Ω_l)".into(),
            ))
        }
    };
    // method f : (x:A) → M [x].
    let expected_method_ty = Term::pi(
        underlying_a.clone(),
        Term::app(weaken(motive, 1), Term::QuotClass(Box::new(Term::var(0)))),
    );
    check(env, ctx, method, &expected_method_ty)?;
    // Respect proof.
    if type_target {
        // §5.1 cong/cast schema: r must have type
        //   (x:A) → (y:A) → (h:R x y) → Eq(M[x])(f x)(cast M[x] M[y] refl(M[x]) (f y))
        // Requires a proper Quot (not Trunc).
        let rel = match opt_rel {
            Some(r) => r,
            None => {
                return Err(KernelError::BadEliminator(
                    "quotient-elim Type target requires a Quot (not Trunc)".into(),
                ))
            }
        };
        // depth 3: x=Var(2), y=Var(1), h=Var(0) under (x:A)(y:A)(h:R x y)
        let x_class = Term::QuotClass(Box::new(Term::var(2)));
        let y_class = Term::QuotClass(Box::new(Term::var(1)));
        let m_x = Term::app(weaken(motive, 3), x_class);
        let m_y = Term::app(weaken(motive, 3), y_class);
        let f_x = Term::app(weaken(method, 3), Term::var(2));
        let f_y = Term::app(weaken(method, 3), Term::var(1));
        // Transport f_y from M[y] (its type) to M[x] (the Eq's required RHS type).
        // Source = M[y], target = M[x]; cast ignores the proof (§3.4).
        let cast_fy = Term::Cast(
            Box::new(m_y.clone()),
            Box::new(m_x.clone()),
            Box::new(Term::Refl(Box::new(m_y.clone()))),
            Box::new(f_y),
        );
        let eq_body = Term::Eq(Box::new(m_x), Box::new(f_x), Box::new(cast_fy));
        // h_ty = R x y at depth 2: x=Var(1), y=Var(0)
        let h_ty = apply_args(weaken(&rel, 2), &[Term::var(1), Term::var(0)]);
        let expected = Term::pi(
            underlying_a.clone(),
            Term::pi(weaken(&underlying_a, 1), Term::pi(h_ty, eq_body)),
        );
        check(env, ctx, respect, &expected)?;
    } else {
        // Ω-target: respect-free by Ω-PI; well-formedness only.
        raw_well_formed(ctx, respect)?;
    }
    // Result: M scrut.
    Ok(Term::app(motive.clone(), scrut.clone()))
}

/// `absurd C p : C` — ex-falso (`16 §1.3`, K5/KM-index-impossible). Sound
/// because `Bottom` is empty: `p` proves the impossible, so `Absurd` never has a
/// canonical scrutinee to compute on and stays neutral forever. The motive may
/// be either a proposition (`Ω`) or a value type (`Type`), but the proof must
/// still check as actual `Bottom`; constructor disjointness alone does not
/// synthesize a closed contradiction. Non-dependent: `Bottom` has no indices to
/// abstract over, so the result is `motive` itself, never substituted.
fn infer_absurd(env: &GlobalEnv, ctx: &Context, motive: &Term, proof: &Term) -> KernelResult<Term> {
    check(env, ctx, proof, &crate::obs::bottom_term(env))?;
    classify(env, ctx, motive)?;
    Ok(motive.clone())
}

// --- declaration admission (`18 §4`) ---------------------------------------

/// A constructor specification for [`declare_inductive`] (no id/type yet —
/// the kernel allocates and generates them).
#[derive(Clone, Debug)]
pub struct CtorSpec {
    /// `Δₖ` — argument telescope, relative to `Δ_p`.
    pub args: Vec<Term>,
    /// `t̄ₖ` — the index instance the constructor targets, relative to
    /// `Δ_p + Δₖ`.
    pub target_indices: Vec<Term>,
}

/// An inductive family specification for [`declare_inductive`].
#[derive(Clone, Debug)]
pub struct InductiveSpec {
    pub level_params: Vec<LevelVar>,
    /// `Δ_p` — parameters, relative to the empty term context.
    pub params: Vec<Term>,
    /// `Δ_i` — indices, relative to `Δ_p`.
    pub indices: Vec<Term>,
    /// `ℓ` — the family's universe level (may mention `level_params`).
    pub level: Level,
    pub constructors: Vec<CtorSpec>,
}

/// `declare_inductive` — admit `data D (Δ_p) : (Δ_i) → Type ℓ where …` after
/// re-checking signatures, strict positivity, and constructor universes
/// (`14 §1`, `14 §8`, `14 §8.4`). W-style
/// (Π-bound) recursive arguments are admitted (K1.5, `14 §2.1`) — the
/// blanket `check_no_pi_bound_recursive` gate is retired. Generates the type
/// former, constructors, and (on use) the dependent eliminator with
/// Π-abstracted IH for W-style args. Returns the family's [`GlobalId`].
///
/// `build` receives the freshly-allocated family id so the spec's constructor
/// signatures can self-reference `D` (e.g. `suc : Nat → Nat`).
pub fn declare_inductive<F>(env: &mut GlobalEnv, build: F) -> KernelResult<GlobalId>
where
    F: FnOnce(GlobalId) -> InductiveSpec,
{
    declare_inductive_try(env, |id| Ok::<_, std::convert::Infallible>(build(id)))?
        .map_err(|never| match never {})
}

/// Fallible inductive specification builder. If `build` fails, release the
/// reserved family id without installing an unchecked placeholder declaration.
pub fn declare_inductive_try<F, E>(
    env: &mut GlobalEnv,
    build: F,
) -> KernelResult<Result<GlobalId, E>>
where
    F: FnOnce(GlobalId) -> Result<InductiveSpec, E>,
{
    let d_id = env.fresh_id();
    let spec = match build(d_id) {
        Ok(spec) => spec,
        Err(error) => {
            env.release_unused_id(d_id);
            return Ok(Err(error));
        }
    };
    let constructors: Vec<_> = spec
        .constructors
        .into_iter()
        .map(|c| crate::env::ConstructorDecl {
            id: env.fresh_id(),
            args: c.args,
            target_indices: c.target_indices,
            type_: Term::Type(Level::zero()), // placeholder; build_types fills it
            recursive_positions: Vec::new(),
        })
        .collect();
    let mut ind = InductiveDecl {
        id: d_id,
        level_params: spec.level_params,
        params: spec.params,
        parameter_polarities: Vec::new(),
        indices: spec.indices,
        level: spec.level,
        constructors,
        former_type: Term::Type(Level::zero()), // placeholder; build_types fills it
    };

    // Generate former + constructor types (`Π Δ_p. Π Δ_i. Type ℓ`, etc.).
    ind.build_types();
    ind.parameter_polarities = crate::inductive::derive_parameter_polarities(env, &ind);
    if let Err(error) = check_inductive_level_closure(&ind) {
        env.release_unused_id(d_id);
        return Err(error);
    }

    // Provisionally admit so every admission clause can roll back both the
    // declaration and its allocated ids on failure.
    env.add_decl(Decl::Inductive(ind.clone()));

    if let Err(error) = validate_inductive_decl(env, &ind) {
        env.remove_last();
        return Err(error);
    }

    let mut parameter_ctx = Context::new();
    let mut positive_parameters = Vec::new();
    for (position, (parameter_type, polarity)) in
        ind.params.iter().zip(&ind.parameter_polarities).enumerate()
    {
        if *polarity == crate::env::ParameterPolarity::StrictlyPositive
            && matches!(whnf(env, &parameter_ctx, parameter_type), Term::Type(_))
        {
            positive_parameters.push(position);
        }
        parameter_ctx.push(parameter_type.clone());
    }
    let mut supports = Vec::with_capacity(positive_parameters.len() * 2);
    let mut published_supports = 0usize;
    for parameter in positive_parameters {
        for sort in [AllSupportSort::Type, AllSupportSort::Omega] {
            let family = env.fresh_id();
            let constructor_ids = ind
                .constructors
                .iter()
                .map(|_| env.fresh_id())
                .collect::<Vec<_>>();
            let support =
                match build_all_support_decl(env, &ind, parameter, sort, family, &constructor_ids)
                    .and_then(|support| {
                        check_inductive_level_closure(&support)?;
                        Ok(support)
                    }) {
                    Ok(support) => support,
                    Err(error) => {
                        for _ in 0..published_supports {
                            env.remove_last();
                        }
                        env.remove_last();
                        return Err(error);
                    }
                };
            env.add_decl(Decl::Inductive(support.clone()));
            if let Err(error) = validate_inductive_decl_inner(env, &support, true) {
                env.remove_last();
                for _ in 0..published_supports {
                    env.remove_last();
                }
                env.remove_last();
                return Err(error);
            }
            published_supports += 1;
            supports.push((parameter, sort, family));
        }
    }
    env.register_all_supports(ind.id, supports);
    Ok(Ok(d_id))
}

fn validate_inductive_decl(env: &GlobalEnv, ind: &InductiveDecl) -> KernelResult<()> {
    validate_inductive_decl_inner(env, ind, false)
}

fn validate_inductive_decl_inner(
    env: &GlobalEnv,
    ind: &InductiveDecl,
    terminal_support: bool,
) -> KernelResult<()> {
    // Admission has three independent clauses (14 §1): ordinary signatures,
    // strict positivity, and constructor universes.
    if terminal_support {
        check_support_positivity(env, ind)?;
    } else {
        check_positivity(env, ind)?;
    }

    // Check only each constructor-local telescope Δₖ. Family parameters Δ_p
    // establish the base context but are not themselves constructor fields.
    // Constructor arguments may be Type- or Ω-sorted, so use `classify` and
    // compare the resulting sort level with the family's level.
    let mut params = Context::new();
    params.extend_tel(&ind.params);
    for constructor in &ind.constructors {
        let mut ctor_ctx = params.clone();
        for argument in &constructor.args {
            let argument_level = match classify(env, &ctor_ctx, argument) {
                Ok(sort) => sort.level().clone(),
                Err(error) => {
                    return Err(KernelError::IllFormedDecl(format!(
                        "constructor argument failed to type-check: {error}"
                    )));
                }
            };
            if !level_eq(&argument_level.clone().max(ind.level.clone()), &ind.level) {
                return Err(KernelError::ConstructorUniverseViolation {
                    argument: argument_level,
                    family: ind.level.clone(),
                });
            }
            ctor_ctx.push(argument.clone());
        }
    }

    let empty = Context::new();
    let sig_ok = synth_type(env, &empty, &ind.former_type).is_ok()
        && ind
            .constructors
            .iter()
            .all(|c| synth_type(env, &empty, &c.type_).is_ok());
    if !sig_ok {
        return Err(KernelError::IllFormedDecl(
            "inductive signature failed to type-check".into(),
        ));
    }
    Ok(())
}

/// Kernel-owned opaque placeholders awaiting one checked group admission.
/// The stored mark bounds rollback to this transaction and its subsequent
/// literal postulates; external callers cannot construct or alter it.
#[must_use]
pub struct PendingAdmission {
    ids: Vec<GlobalId>,
    mark_len: usize,
    mark_next_id: GlobalId,
    env_instance: u64,
    staged_types: Vec<Term>,
}

impl PendingAdmission {
    pub fn ids(&self) -> &[GlobalId] {
        &self.ids
    }
}

/// Verify the exact staged prefix before either installing bodies or removing
/// declarations. Later literal postulates may follow this prefix; they are
/// removed only by a valid rollback on the staging environment itself.
fn pending_tail_intact(env: &GlobalEnv, pending: &PendingAdmission) -> bool {
    if pending.ids.first().copied() != Some(pending.mark_next_id)
        || pending.ids.len() != pending.staged_types.len()
    {
        return false;
    }
    let Some(end) = pending.mark_len.checked_add(pending.ids.len()) else {
        return false;
    };
    env.declarations()
        .get(pending.mark_len..end)
        .is_some_and(|tail| {
            tail.iter()
                .zip(pending.ids.iter().zip(&pending.staged_types))
                .all(|(decl, (id, ty))| {
                    matches!(decl, Decl::Opaque { id: actual_id, ty: actual_ty, .. }
                        if actual_id == id && actual_ty == ty)
                })
        })
}

/// Classify all signatures before staging any opaque declarations. Staged
/// placeholders can be referenced while the elaborator checks recursive
/// bodies; only [`admit_pending`] can make them transparent.
pub fn stage_placeholders(
    env: &mut GlobalEnv,
    specs: Vec<(String, Vec<LevelVar>, Term)>,
) -> KernelResult<PendingAdmission> {
    if specs.is_empty() {
        return Err(KernelError::IllFormedDecl(
            "checked staging needs at least one placeholder".into(),
        ));
    }
    let empty = Context::new();
    for (_, level_params, ty) in &specs {
        check_level_closure(level_params, [ty])?;
        classify(env, &empty, ty)?;
    }
    let mark_len = env.declarations().len();
    let mark_next_id = env.next_global_id();
    let env_instance = env.instance_id();
    let mut ids = Vec::with_capacity(specs.len());
    let mut staged_types = Vec::with_capacity(specs.len());
    for (name, level_params, ty) in specs {
        let id = env.fresh_id();
        staged_types.push(ty.clone());
        env.add_decl(Decl::Opaque {
            id,
            name,
            level_params,
            ty,
        });
        ids.push(id);
    }
    Ok(PendingAdmission {
        ids,
        mark_len,
        mark_next_id,
        env_instance,
        staged_types,
    })
}

/// Remove only this transaction's declarations, newest first, including
/// literal postulates added after staging. Refuse a handle from another env.
pub fn rollback_pending(env: &mut GlobalEnv, pending: PendingAdmission) -> KernelResult<Vec<Decl>> {
    if env.instance_id() != pending.env_instance {
        return Err(KernelError::Msg(
            "admission handle belongs to another environment".into(),
        ));
    }
    if !pending_tail_intact(env, &pending) {
        return Err(KernelError::IllFormedDecl(
            "pending admission staged tail is no longer intact".into(),
        ));
    }
    let mut removed = Vec::new();
    while env.declarations().len() > pending.mark_len {
        removed.push(
            env.remove_last()
                .expect("declarations exceed admission mark"),
        );
    }
    debug_assert_eq!(env.next_global_id(), pending.mark_next_id);
    Ok(removed)
}

/// Check all pending bodies as one SCT group. Failure consumes and rolls back
/// the transaction and returns the declarations for elaborator-side cleanup.
pub fn admit_pending(
    env: &mut GlobalEnv,
    pending: PendingAdmission,
    bodies: Vec<Term>,
) -> Result<Vec<GlobalId>, (KernelError, Vec<Decl>)> {
    if env.instance_id() != pending.env_instance {
        return Err((
            KernelError::Msg("admission handle belongs to another environment".into()),
            Vec::new(),
        ));
    }
    if !pending_tail_intact(env, &pending) {
        return Err((
            KernelError::IllFormedDecl("pending admission staged tail is no longer intact".into()),
            Vec::new(),
        ));
    }
    let result = if bodies.len() == pending.ids.len() {
        let group = pending.ids.iter().copied().zip(bodies).collect::<Vec<_>>();
        admit_bodies(env, &group)
    } else {
        Err(KernelError::IllFormedDecl(
            "pending admission body count does not match placeholder count".into(),
        ))
    };
    match result {
        Ok(()) => Ok(pending.ids),
        Err(error) => match rollback_pending(env, pending) {
            Ok(removed) => Err((error, removed)),
            Err(rollback_error) => Err((rollback_error, Vec::new())),
        },
    }
}

/// Check and install a complete group of pre-admitted opaque bodies.
///
/// Type-check each body while all group members are still opaque, then run
/// SCT over the entire group. An external transparent definition may already
/// refer to one of these opaque placeholders; upgrading a body that reaches
/// that definition and returns to any group member would create a cycle SCT
/// cannot see. Refuse that escape before installing any body.
///
/// An error leaves the environment untouched, including its trusted base.
/// Callers that allocated provisional placeholders own their rollback.
pub fn admit_bodies(env: &mut GlobalEnv, group: &[(GlobalId, Term)]) -> KernelResult<()> {
    let members: HashSet<GlobalId> = group.iter().map(|(id, _)| *id).collect();
    if members.len() != group.len() {
        return Err(KernelError::IllFormedDecl(
            "duplicate checked-upgrade group member".into(),
        ));
    }
    let empty = Context::new();
    for (id, body) in group {
        let Some(Decl::Opaque {
            level_params, ty, ..
        }) = env.lookup(*id)
        else {
            return Err(KernelError::IllFormedDecl(
                "checked upgrade requires a present opaque member".into(),
            ));
        };
        check_level_closure(level_params, [body])?;
        check(env, &empty, body, ty)?;
    }
    let decreasing = crate::sct::sct_check(env, group)?;

    for (_, body) in group {
        let mut pending = Vec::new();
        let mut terms = vec![body];
        while let Some(term) = terms.pop() {
            if let Term::Const { id, .. } = term {
                if !members.contains(id) {
                    pending.push(*id);
                }
            }
            terms.extend(term.children());
        }
        let mut seen = HashSet::new();
        while let Some(id) = pending.pop() {
            if members.contains(&id) {
                return Err(KernelError::NotTerminating(
                    "transparent body escapes the admission group and returns to a member".into(),
                ));
            }
            if seen.insert(id) {
                if let Some(refs) = env.transparent_body_refs(id) {
                    pending.extend(refs.iter().copied());
                }
            }
        }
    }

    for (id, body) in group {
        assert!(
            env.upgrade_to_transparent(*id, body.clone()),
            "prechecked opaque group member must upgrade"
        );
    }
    env.install_sct_decreasing(decreasing);
    Ok(())
}

/// `declare_def` — admit a transparent definition `c : A := t` after checking
/// `· ⊢ A type`, `· ⊢ t ⇐ A`, and the SCT gate (`17 §4`, `18 §4`).
///
/// The definition is **pre-admitted as opaque** before type-checking, so `t`
/// may contain self-recursive `Const(c)` references.  SCT either accepts (→
/// upgrades to transparent) or rejects (→ removes `c` and returns an error).
pub fn declare_def(
    env: &mut GlobalEnv,
    level_params: Vec<LevelVar>,
    ty: Term,
    body: Term,
) -> KernelResult<GlobalId> {
    let pending = stage_placeholders(
        env,
        vec![("provisional definition".into(), level_params, ty)],
    )?;
    admit_pending(env, pending, vec![body])
        .map(|mut ids| ids.remove(0))
        .map_err(|(error, _removed)| error)
}

/// Declare a group of mutually-recursive transparent definitions.
///
/// All members are pre-admitted as opaque before any body is type-checked, so
/// each body may reference any member.  SCT is run on the whole group; on
/// rejection all pre-admitted members are rolled back.
///
/// `specs` — `(level_params, ty)` for each member.  `bodies_fn` receives the
/// freshly-allocated IDs and must return one body per member in the same order.
pub fn declare_recursive_group<F>(
    env: &mut GlobalEnv,
    specs: Vec<(Vec<LevelVar>, Term)>,
    bodies_fn: F,
) -> KernelResult<Vec<GlobalId>>
where
    F: FnOnce(&[GlobalId]) -> Vec<Term>,
{
    if specs.is_empty() {
        return Ok(Vec::new());
    }
    let pending = stage_placeholders(
        env,
        specs
            .into_iter()
            .map(|(level_params, ty)| ("provisional recursive definition".into(), level_params, ty))
            .collect(),
    )?;
    let bodies = bodies_fn(pending.ids());
    assert_eq!(
        bodies.len(),
        pending.ids().len(),
        "bodies_fn must return one body per member"
    );

    // Admission checks SCT once on the entire mutual group.
    admit_pending(env, pending, bodies).map_err(|(error, _removed)| error)
}

/// `declare_postulate` — admit an opaque constant `c : A` after checking
/// `· ⊢ A type` (`11 §4`). Recorded in the trusted base (`18 §5`).
pub fn declare_postulate(
    env: &mut GlobalEnv,
    name: String,
    level_params: Vec<LevelVar>,
    ty: Term,
) -> KernelResult<GlobalId> {
    check_level_closure(&level_params, [&ty])?;
    let empty = Context::new();
    classify(env, &empty, &ty)?;
    let id = env.fresh_id();
    env.add_decl(Decl::Opaque {
        id,
        name,
        level_params,
        ty,
    });
    Ok(id)
}

/// `declare_primitive` — admit a primitive type/operation (opaque + registered
/// reduction) after checking `· ⊢ A type` (`14 §5`). K1 defines the interface
/// only; the value model (K3) and API (K-api) elaborate the computation.
pub fn declare_primitive(
    env: &mut GlobalEnv,
    level_params: Vec<LevelVar>,
    ty: Term,
    reduction: crate::env::PrimReduction,
) -> KernelResult<GlobalId> {
    check_level_closure(&level_params, [&ty])?;
    let empty = Context::new();
    classify(env, &empty, &ty)?;
    let id = env.fresh_id();
    env.add_decl(Decl::Primitive {
        id,
        level_params,
        ty,
        reduction,
    });
    Ok(id)
}

/// Register a literal carrier when its type is admitted. Prelude declarations
/// may contain checked String literals before the List Char view is installed.
pub fn register_checked_string_carrier(env: &mut GlobalEnv, id: GlobalId) -> KernelResult<()> {
    if env.checked_string_type().is_some()
        || !matches!(
            env.lookup(id),
            Some(Decl::Primitive {
                reduction: PrimReduction::OpaqueType,
                ..
            })
        )
    {
        return Err(KernelError::Msg(
            "invalid or duplicate String literal carrier".into(),
        ));
    }
    env.install_checked_string_carrier(id);
    Ok(())
}

/// Char's current core carrier is a checked transparent alias convertible to
/// Int. The source refinement predicate is erased at this boundary; each
/// literal payload is independently validated as a Unicode scalar below.
pub fn register_checked_char_carrier(env: &mut GlobalEnv, id: GlobalId) -> KernelResult<()> {
    let int_type = env
        .int_lit_type()
        .ok_or_else(|| KernelError::Msg("Int literal carrier is not registered".into()))?;
    if env.checked_char_type().is_some()
        || !matches!(env.lookup(id), Some(Decl::Transparent { .. }))
        || !convert_type(
            env,
            &Context::new(),
            &Term::const_(id, vec![]),
            &Term::const_(int_type, vec![]),
        )
    {
        return Err(KernelError::Msg(
            "invalid or duplicate Char literal carrier".into(),
        ));
    }
    env.install_checked_char_carrier(id);
    Ok(())
}

/// Authorize precisely one String-literal view, after checking all carrier,
/// operation and constructor types inside the kernel. The operation's name is
/// irrelevant: only the registered declaration identity may reduce.
pub fn register_literal_char_view(
    env: &mut GlobalEnv,
    string_type: GlobalId,
    char_type: GlobalId,
    operation: GlobalId,
    list_type: GlobalId,
    nil: GlobalId,
    cons: GlobalId,
) -> KernelResult<()> {
    if env.has_literal_char_view()
        || env.checked_string_type() != Some(string_type)
        || env.checked_char_type() != Some(char_type)
    {
        return Err(KernelError::Msg(
            "literal view registration does not match checked carriers or is duplicate".into(),
        ));
    }
    if !matches!(
        env.lookup(string_type),
        Some(Decl::Primitive {
            reduction: PrimReduction::OpaqueType,
            ..
        })
    ) {
        return Err(KernelError::Msg(
            "literal String carrier must be an opaque primitive type".into(),
        ));
    }
    let int_type = env
        .int_lit_type()
        .ok_or_else(|| KernelError::Msg("Int literal carrier is not registered".into()))?;
    let char_ty = Term::const_(char_type, vec![]);
    // Char's checked core alias converts to Int; scalar validity is separately
    // enforced at every checked literal admission (Rust `char::from_u32`).
    if !matches!(env.lookup(char_type), Some(Decl::Transparent { .. }))
        || !convert_type(
            env,
            &Context::new(),
            &char_ty,
            &Term::const_(int_type, vec![]),
        )
    {
        return Err(KernelError::Msg(
            "literal Char carrier must be a checked Int-compatible alias".into(),
        ));
    }
    let list_ty = Term::app(Term::indformer(list_type, vec![]), char_ty.clone());
    let expected_op = Term::pi(Term::const_(string_type, vec![]), list_ty.clone());
    if !matches!(
        env.lookup(operation),
        Some(Decl::Primitive {
            reduction: PrimReduction::Op { .. },
            ..
        })
    ) || !convert_type(
        env,
        &Context::new(),
        &infer(env, &Context::new(), &Term::const_(operation, vec![]))?,
        &expected_op,
    ) {
        return Err(KernelError::Msg(
            "literal view operation must have String -> List Char type".into(),
        ));
    }
    let nil_term = Term::app(Term::constructor(nil, vec![]), char_ty.clone());
    let sample = apply_args(
        Term::constructor(cons, vec![]),
        &[char_ty, Term::IntLit(65u32.into()), nil_term],
    );
    check(env, &Context::new(), &sample, &list_ty)?;
    env.install_literal_char_view(char_type, operation, nil, cons);
    Ok(())
}

/// Register an immutable NFC payload and its kernel-checked String carrier.
/// The same payload is exposed to the interpreter and native compiler.
pub fn declare_checked_string_literal(env: &mut GlobalEnv, raw: &str) -> KernelResult<GlobalId> {
    let ty_id = env
        .checked_string_type()
        .ok_or_else(|| KernelError::Msg("literal String carrier is not registered".into()))?;
    let ty = Term::const_(ty_id, vec![]);
    let normalized: String = raw.nfc().collect();
    let id = declare_primitive(env, vec![], ty, PrimReduction::Literal)?;
    env.install_checked_literal(id, CheckedStringLiteral(normalized));
    Ok(id)
}

/// A surface Char literal is an IntLit in the current core, but only a valid
/// Unicode scalar at the checked, Int-compatible Char carrier may be emitted.
/// The value lives in the Term itself, so derived `charToInt := λc.c` and
/// direct use share one normal form without an extra reduction rule.
pub fn checked_char_literal(env: &GlobalEnv, scalar: u32) -> KernelResult<Term> {
    let c = char::from_u32(scalar)
        .ok_or_else(|| KernelError::Msg(format!("invalid Unicode scalar U+{scalar:X}")))?;
    let char_type = env
        .checked_char_type()
        .ok_or_else(|| KernelError::Msg("literal Char carrier is not registered".into()))?;
    let term = Term::IntLit((c as u32).into());
    check(
        env,
        &Context::new(),
        &term,
        &Term::const_(char_type, vec![]),
    )?;
    Ok(term)
}

/// `declare_deceq_certificate` — register a decidable-equality certificate
/// for an opaque primitive type
/// (`docs/adr/0013-int-decidable-equality-kernel-posture.md` Layer 1): the
/// kernel trusts `eq_op` to decide propositional equality at `prim_ty`,
/// both directions.
/// General, opt-in, per-primitive — `prim_ty` is the *first* registrant of
/// this mechanism, not a special case of it; an unregistered primitive's
/// `Eq` stays neutral exactly as before (`obs.rs`'s fail-safe default is
/// untouched — this adds no reduction rule).
///
/// Builds and admits, via [`declare_postulate`] (each call `classify`s the
/// constructed type before committing it, so registration fails closed on
/// an incoherent registrant — e.g. an `eq_op` not shaped
/// `prim_ty → prim_ty → bool_ty`, or a `bool_true` not of type `bool_ty`):
///
/// - `sound   : (x y : prim_ty) → Eq bool_ty (eq_op x y) bool_true → Eq prim_ty x y`
/// - `complete: (x y : prim_ty) → Eq prim_ty x y → Eq bool_ty (eq_op x y) bool_true`
///
/// and records the pair in [`GlobalEnv::deceq_cert`] under `prim_ty`.
pub fn declare_deceq_certificate(
    env: &mut GlobalEnv,
    prim_ty: GlobalId,
    eq_op: GlobalId,
    bool_ty: GlobalId,
    bool_true: GlobalId,
) -> KernelResult<crate::env::DecEqCert> {
    let ty_const = Term::const_(prim_ty, vec![]);
    let bool_const = Term::indformer(bool_ty, vec![]);
    let true_const = Term::constructor(bool_true, vec![]);
    let eq_op_const = Term::const_(eq_op, vec![]);

    // Context depth 2 (x y : prim_ty bound, y=Var(0), x=Var(1)) — used for
    // the premise, the domain of the third Π.
    let eq_call_d2 = Term::app(Term::app(eq_op_const.clone(), Term::var(1)), Term::var(0));
    let bool_eq_true_d2 = Term::Eq(
        Box::new(bool_const.clone()),
        Box::new(eq_call_d2),
        Box::new(true_const.clone()),
    );
    let prim_eq_d2 = Term::Eq(
        Box::new(ty_const.clone()),
        Box::new(Term::var(1)),
        Box::new(Term::var(0)),
    );

    // Context depth 3 (x y premise bound, x=Var(2), y=Var(1)) — used for the
    // conclusion, the codomain of the third Π.
    let eq_call_d3 = Term::app(Term::app(eq_op_const, Term::var(2)), Term::var(1));
    let bool_eq_true_d3 = Term::Eq(
        Box::new(bool_const),
        Box::new(eq_call_d3),
        Box::new(true_const),
    );
    let prim_eq_d3 = Term::Eq(
        Box::new(ty_const.clone()),
        Box::new(Term::var(2)),
        Box::new(Term::var(1)),
    );

    let sound_ty = Term::pi(
        ty_const.clone(),
        Term::pi(ty_const.clone(), Term::pi(bool_eq_true_d2, prim_eq_d3)),
    );
    let complete_ty = Term::pi(
        ty_const.clone(),
        Term::pi(ty_const, Term::pi(prim_eq_d2, bool_eq_true_d3)),
    );

    let sound = declare_postulate(
        env,
        "decidable equality sound".to_string(),
        vec![],
        sound_ty,
    )?;
    let complete = declare_postulate(
        env,
        "decidable equality complete".to_string(),
        vec![],
        complete_ty,
    )?;

    let cert = crate::env::DecEqCert {
        eq_op,
        sound,
        complete,
    };
    env.register_deceq_cert(prim_ty, cert.clone());
    Ok(cert)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::GlobalEnv;
    use crate::term::Level;

    #[test]
    fn level_closure_reaches_every_explicit_level_slot_and_nested_child() {
        let u = LevelVar(0);
        let v = LevelVar(1);
        let escaped = Level::Var(u).max(Level::Var(v).suc());
        let id = GlobalId(17);
        let terms = [
            Term::Type(escaped.clone()),
            Term::Omega(escaped.clone()),
            Term::Const {
                id,
                level_args: vec![escaped.clone()],
            },
            Term::IndFormer {
                id,
                level_args: vec![escaped.clone()],
            },
            Term::Constructor {
                id,
                level_args: vec![escaped.clone()],
            },
            Term::Elim {
                fam: id,
                level_args: vec![escaped.clone()],
                params: vec![],
                motive: Box::new(Term::ty(Level::zero())),
                methods: vec![],
                indices: vec![],
                scrut: Box::new(Term::var(0)),
            },
            Term::pi(Term::ty(Level::zero()), Term::Type(escaped)),
        ];
        for term in &terms {
            assert_eq!(
                check_level_closure(&[u], [term]),
                Err(KernelError::IllFormedDecl(format!(
                    "undeclared level variable {v:?}"
                ))),
                "missed explicit or nested level slot in {term:?}"
            );
            assert_eq!(check_level_closure(&[u, v], [term]), Ok(()));
        }
        assert_eq!(
            check_level_closure(&[u, u], [&Term::ty(Level::zero())]),
            Err(KernelError::IllFormedDecl(
                "duplicate level parameter".into()
            ))
        );
    }

    struct BoolNat {
        bool_: GlobalId,
        true_: GlobalId,
        nat: GlobalId,
        zero: GlobalId,
    }

    fn bool_nat_env() -> (GlobalEnv, BoolNat) {
        let mut env = GlobalEnv::new();

        let bool_ = declare_inductive(&mut env, |_| InductiveSpec {
            level_params: vec![],
            params: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![
                CtorSpec {
                    args: vec![],
                    target_indices: vec![],
                },
                CtorSpec {
                    args: vec![],
                    target_indices: vec![],
                },
            ],
        })
        .expect("Bool");
        let true_ = env.inductive(bool_).unwrap().constructors[0].id;

        let nat = declare_inductive(&mut env, |_| InductiveSpec {
            level_params: vec![],
            params: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![CtorSpec {
                args: vec![],
                target_indices: vec![],
            }],
        })
        .expect("Nat");
        let zero = env.inductive(nat).unwrap().constructors[0].id;

        (
            env,
            BoolNat {
                bool_,
                true_,
                nat,
                zero,
            },
        )
    }

    fn bool_ty(ids: &BoolNat) -> Term {
        Term::indformer(ids.bool_, vec![])
    }

    fn nat_ty(ids: &BoolNat) -> Term {
        Term::indformer(ids.nat, vec![])
    }

    fn true_term(ids: &BoolNat) -> Term {
        Term::constructor(ids.true_, vec![])
    }

    fn zero_term(ids: &BoolNat) -> Term {
        Term::constructor(ids.zero, vec![])
    }

    fn let_term(let_ty: Term, val: Term, body: Term) -> Term {
        Term::Let {
            ty: Box::new(let_ty),
            val: Box::new(val),
            body: Box::new(body),
        }
    }

    fn assert_same_admission_state(env: &GlobalEnv, before: &GlobalEnv) {
        assert_eq!(
            env, before,
            "declarations and all indices must be unchanged"
        );
        assert_eq!(env.declarations(), before.declarations());
        assert_eq!(env.next_global_id(), before.next_global_id());
        assert_eq!(env.trusted_base(), before.trusted_base());
    }

    fn assert_foreign_admission_error(error: &KernelError) {
        assert!(
            matches!(error, KernelError::Msg(message)
                if message == "admission handle belongs to another environment"),
            "foreign-handle refusal must be exact, got {error:?}"
        );
    }

    /// Durable invariant (`11 §4`, `18 §4`): matching IDs do not establish
    /// transaction ownership. MEASURED: two separately constructed envs have
    /// the same g3 and staged type, but rollback/admit on the foreign env must
    /// refuse without changing any declaration, index, allocator or trust.
    /// CLAIMED: every `PendingAdmission` is bound to its staging env instance.
    /// GAP: this fixture creates two real envs with coincident IDs; it does not
    /// enumerate all possible environments. A no-instance-check mutation
    /// removes B's postulate (rollback) or upgrades it (admit).
    #[test]
    fn foreign_pending_handle_refuses_matching_ids_without_mutation() {
        let ty = Term::Type(Level::zero().suc());
        let mut a = GlobalEnv::new();
        let mut b = GlobalEnv::new();
        let rollback_handle =
            stage_placeholders(&mut a, vec![("staged A".into(), vec![], ty.clone())])
                .expect("checked staged type");
        let unrelated = declare_postulate(&mut b, "unrelated B".into(), vec![], ty.clone())
            .expect("checked same-id postulate");
        assert_eq!(rollback_handle.ids()[0], unrelated);
        assert_ne!(a.instance_id(), b.instance_id());
        let before_b = b.clone();
        let error = rollback_pending(&mut b, rollback_handle)
            .expect_err("A's handle cannot roll back B's unrelated postulate");
        assert_foreign_admission_error(&error);
        assert_same_admission_state(&b, &before_b);
        assert!(b.trusted_base().contains(&unrelated));

        let mut c = GlobalEnv::new();
        let mut d = GlobalEnv::new();
        let admit_handle =
            stage_placeholders(&mut c, vec![("staged C".into(), vec![], ty.clone())])
                .expect("checked staged type");
        let unrelated_d = declare_postulate(&mut d, "unrelated D".into(), vec![], ty)
            .expect("checked same-id postulate");
        assert_eq!(admit_handle.ids()[0], unrelated_d);
        let before_d = d.clone();
        let (error, removed) = admit_pending(&mut d, admit_handle, vec![Term::Type(Level::zero())])
            .expect_err("C's handle cannot upgrade D's unrelated postulate");
        assert_foreign_admission_error(&error);
        assert!(removed.is_empty());
        assert_same_admission_state(&d, &before_d);
        assert!(d.trusted_base().contains(&unrelated_d));
    }

    /// Durable invariant: cloning an env mints a new transaction identity,
    /// while its declarations and allocator are preserved as ordinary data.
    /// Test-only duplication of the opaque handle exercises both the refused
    /// clone and legitimate owner; the public handle remains move-only.
    #[test]
    fn cloned_env_cannot_consume_source_pending_handle() {
        let mut a = GlobalEnv::new();
        let pending = stage_placeholders(
            &mut a,
            vec![("staged owner".into(), vec![], Term::Type(Level::zero()))],
        )
        .expect("checked staged type");
        let staged_id = pending.ids()[0];
        let test_handle_copy = PendingAdmission {
            ids: pending.ids.clone(),
            mark_len: pending.mark_len,
            mark_next_id: pending.mark_next_id,
            env_instance: pending.env_instance,
            staged_types: pending.staged_types.clone(),
        };
        let mut clone = a.clone();
        assert_ne!(a.instance_id(), clone.instance_id());
        assert_eq!(
            a, clone,
            "structural equality excludes transaction identity"
        );
        assert_ne!(a.instance_id(), GlobalEnv::default().instance_id());
        let before_clone = clone.clone();
        let error = rollback_pending(&mut clone, test_handle_copy)
            .expect_err("source handle cannot mutate a cloned env");
        assert_foreign_admission_error(&error);
        assert_same_admission_state(&clone, &before_clone);
        let removed = rollback_pending(&mut a, pending).expect("owner rollback still succeeds");
        assert_eq!(
            removed.iter().map(Decl::id).collect::<Vec<_>>(),
            vec![staged_id]
        );
        assert!(!a.declarations().iter().any(|decl| decl.id() == staged_id));
    }

    /// Durable invariant: an env move preserves its instance and ownership.
    #[test]
    fn moved_env_retains_pending_transaction_ownership() {
        struct Owner {
            env: GlobalEnv,
            pending: PendingAdmission,
        }
        let mut env = GlobalEnv::new();
        let pending = stage_placeholders(
            &mut env,
            vec![("moved owner".into(), vec![], Term::Type(Level::zero()))],
        )
        .expect("checked staged type");
        let id = pending.ids()[0];
        let owner_id = env.instance_id();
        let mut owner = Owner { env, pending };
        assert_eq!(owner.env.instance_id(), owner_id);
        let removed = rollback_pending(&mut owner.env, owner.pending)
            .expect("moving the env preserves ownership");
        assert_eq!(removed.iter().map(Decl::id).collect::<Vec<_>>(), vec![id]);
    }

    /// Durable defense-in-depth (`11 §4`, `18 §4`): both transaction consumers
    /// reject a changed staged type even when the env instance and ID match.
    /// MEASURED: rollback and admission each refuse an in-kernel raw reinstall
    /// with the same ID but another Opaque type, without mutating env state.
    /// CLAIMED: neither consumer may consume a changed staged tail. GAP: the
    /// raw reinstall is crate-private; external clients cannot construct it.
    #[test]
    fn pending_handle_refuses_changed_staged_tail_without_mutation() {
        let changed_tail = || {
            let mut env = GlobalEnv::new();
            let pending = stage_placeholders(
                &mut env,
                vec![("original".into(), vec![], Term::Type(Level::zero()))],
            )
            .expect("checked staged type");
            let original_id = pending.ids()[0];
            assert_eq!(env.remove_last().map(|decl| decl.id()), Some(original_id));
            let replacement_id = env.fresh_id();
            assert_eq!(replacement_id, original_id);
            env.add_decl(Decl::Opaque {
                id: replacement_id,
                name: "changed type".into(),
                level_params: vec![],
                ty: Term::Type(Level::zero().suc()),
            });
            (env, pending)
        };

        let (mut env, pending) = changed_tail();
        let before = env.clone();
        let error = rollback_pending(&mut env, pending)
            .expect_err("rollback must refuse a changed staged type");
        assert!(
            matches!(&error, KernelError::IllFormedDecl(message)
            if message == "pending admission staged tail is no longer intact"),
            "rollback tail refusal: {error:?}"
        );
        assert_same_admission_state(&env, &before);

        let (mut env, pending) = changed_tail();
        let before = env.clone();
        // Type 0 inhabits Type 1, the replacement's type, not the staged Type 0.
        // Without the admission tail guard, this body upgrades the wrong slot.
        let (error, removed) = admit_pending(&mut env, pending, vec![Term::Type(Level::zero())])
            .expect_err("admission must refuse a changed staged type");
        assert!(
            matches!(&error, KernelError::IllFormedDecl(message)
            if message == "pending admission staged tail is no longer intact"),
            "admission tail refusal: {error:?}"
        );
        assert!(
            removed.is_empty(),
            "refusal must not roll back the replacement"
        );
        assert_same_admission_state(&env, &before);

        let mut valid = GlobalEnv::new();
        let valid_handle = stage_placeholders(
            &mut valid,
            vec![("untouched".into(), vec![], Term::Type(Level::zero().suc()))],
        )
        .expect("checked matching staged type");
        let valid_id = valid_handle.ids()[0];
        assert_eq!(
            admit_pending(&mut valid, valid_handle, vec![Term::Type(Level::zero())])
                .expect("untouched staged tail accepts the same body"),
            vec![valid_id]
        );
        assert!(matches!(
            valid.lookup(valid_id),
            Some(Decl::Transparent { .. })
        ));
    }

    /// Durable invariant (`11 §4`, `18 §4`): checked group admission requires
    /// exactly one body per staged ID. MEASURED: empty, short and excess body
    /// vectors each refuse with the body-count error, remove both placeholders
    /// in reverse order and restore declarations, indices, allocator and trust;
    /// exactly two checked bodies install both. CLAIMED: a group cannot partly
    /// upgrade or silently ignore any bodies. GAP: this is one two-ID group,
    /// not an enumeration of every possible arity or body type.
    #[test]
    fn pending_admission_body_count_mismatch_rolls_back_entire_group() {
        let body = Term::Type(Level::zero());
        let staged_type = Term::Type(Level::zero().suc());
        let specs = || {
            vec![
                ("first".into(), vec![], staged_type.clone()),
                ("second".into(), vec![], staged_type.clone()),
            ]
        };
        for body_count in [1, 3, 0] {
            let mut env = GlobalEnv::new();
            let before = env.clone();
            let pending = stage_placeholders(&mut env, specs()).expect("checked group staging");
            let ids = pending.ids().to_vec();
            assert_eq!(ids.len(), 2);
            let (error, removed) = admit_pending(&mut env, pending, vec![body.clone(); body_count])
                .expect_err(&format!(
                    "body count {body_count} must reject the entire group"
                ));
            assert!(
                matches!(&error, KernelError::IllFormedDecl(message)
                    if message == "pending admission body count does not match placeholder count"),
                "body count {body_count}: wrong refusal {error:?}"
            );
            assert_eq!(
                removed.iter().map(Decl::id).collect::<Vec<_>>(),
                vec![ids[1], ids[0]],
                "body count {body_count}: rollback must remove both staged IDs newest first"
            );
            assert_same_admission_state(&env, &before);
        }

        let mut valid = GlobalEnv::new();
        let pending = stage_placeholders(&mut valid, specs()).expect("checked group staging");
        let ids = pending.ids().to_vec();
        assert_eq!(
            admit_pending(&mut valid, pending, vec![body.clone(), body])
                .expect("matching bodies admit the entire group"),
            ids
        );
        for id in ids {
            assert!(matches!(valid.lookup(id), Some(Decl::Transparent { .. })));
            assert!(!valid.trusted_base().contains(&id));
        }
    }

    #[test]
    fn failed_pending_recursive_admission_restores_entire_environment() {
        let (mut env, ids) = bool_nat_env();
        let before = env.clone();
        let nat = nat_ty(&ids);
        let pending = stage_placeholders(
            &mut env,
            vec![(
                "recursive".into(),
                vec![],
                Term::pi(nat.clone(), nat.clone()),
            )],
        )
        .expect("checked recursive signature");
        let recursive_id = pending.ids()[0];
        let literal = declare_postulate(&mut env, "intervening".into(), vec![], nat.clone())
            .expect("typed intervening postulate");
        let looping_body = Term::lam(
            nat,
            Term::app(Term::const_(recursive_id, vec![]), Term::var(0)),
        );
        let (error, removed) = admit_pending(&mut env, pending, vec![looping_body])
            .expect_err("nondecreasing self recursion must fail SCT");
        assert!(matches!(error, KernelError::NotTerminating(_)));
        assert_eq!(
            removed.iter().map(Decl::id).collect::<Vec<_>>(),
            vec![literal, recursive_id]
        );
        assert_eq!(
            env, before,
            "failed staging must restore all env indices and next_id"
        );
        assert_eq!(env.next_global_id(), before.next_global_id());
        assert_eq!(env.trusted_base(), before.trusted_base());
    }

    #[test]
    fn failed_inductive_builder_releases_uninstalled_family_id() {
        let mut env = GlobalEnv::new();
        let before = env.clone();
        let outcome =
            declare_inductive_try(&mut env, |_| Err::<InductiveSpec, _>("builder rejected"))
                .expect("a builder error is not a kernel error");
        assert_eq!(outcome, Err("builder rejected"));
        assert_eq!(env, before, "builder error must leave no reservation");
    }

    #[test]
    fn checked_recursive_barrier_matches_prior_normalization_without_shadowing() {
        let mut env = GlobalEnv::new();
        let ids_nat = declare_inductive(&mut env, |id| InductiveSpec {
            level_params: vec![],
            params: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![
                CtorSpec {
                    args: vec![],
                    target_indices: vec![],
                },
                CtorSpec {
                    args: vec![Term::indformer(id, vec![])],
                    target_indices: vec![],
                },
            ],
        })
        .expect("checked Nat with successor");
        let nat = Term::indformer(ids_nat, vec![]);
        let zero = Term::constructor(env.inductive(ids_nat).unwrap().constructors[0].id, vec![]);
        let rec = declare_recursive_group(
            &mut env,
            vec![(vec![], Term::pi(nat.clone(), nat.clone()))],
            |ids| {
                let step = Term::lam(
                    nat.clone(),
                    Term::lam(
                        nat.clone(),
                        Term::app(Term::const_(ids[0], vec![]), Term::var(1)),
                    ),
                );
                vec![Term::lam(
                    nat.clone(),
                    Term::Elim {
                        fam: ids_nat,
                        level_args: vec![],
                        params: vec![],
                        motive: Box::new(Term::Ascript(
                            Box::new(Term::lam(nat.clone(), nat.clone())),
                            Box::new(Term::pi(nat.clone(), Term::Type(Level::zero()))),
                        )),
                        methods: vec![zero.clone(), step],
                        indices: vec![],
                        scrut: Box::new(Term::var(0)),
                    },
                )]
            },
        )
        .expect("SCT-admitted structural recursion")[0];
        assert!(env.is_recursive_transparent(rec));
        let before = env.clone();
        let barrier = env
            .with_recursion_barriers(&[rec])
            .expect("checked recursive id");
        assert_eq!(env, before, "view must not mutate the checked environment");
        assert_eq!(barrier.declarations().len(), env.declarations().len());
        assert_eq!(barrier.next_global_id(), env.next_global_id());
        assert!(matches!(barrier.lookup(rec), Some(Decl::Opaque { .. })));
        assert!(!barrier.is_recursive_transparent(rec));
        assert!(barrier.transparent_body_refs(rec).is_none());
        assert!(barrier.sct_decreasing_positions(rec).is_none());
        let mut cloned: GlobalEnv = (*barrier).clone();
        let admitted_in_clone = declare_postulate(
            &mut cloned,
            "checked clone-only postulate".into(),
            vec![],
            nat.clone(),
        )
        .expect("a mutable clone admits only through a checked entry point");
        assert!(cloned.trusted_base().contains(&rec));
        assert!(cloned.trusted_base().contains(&admitted_in_clone));
        assert!(!env.trusted_base().contains(&rec));
        assert!(!env.trusted_base().contains(&admitted_in_clone));

        // The prior driver appended a same-id opaque shadow to a private env.
        // Its stale indexes did not alter normalization's lookup result.
        let mut prior_shadow = env.clone();
        prior_shadow.add_decl(Decl::Opaque {
            id: rec,
            name: "prior px8l barrier".into(),
            level_params: vec![],
            ty: Term::pi(nat.clone(), nat.clone()),
        });
        let source = Term::app(Term::const_(rec, vec![]), zero);
        let ctx = Context::new();
        let (_, admitted_body) = env.transparent_body(rec).expect("checked body");
        assert_eq!(
            crate::conv::normalize(&barrier, &ctx, &admitted_body),
            crate::conv::normalize(&prior_shadow, &ctx, &admitted_body),
            "the actual recursive body must normalize as under the prior driver"
        );
        assert_eq!(
            crate::conv::normalize(&barrier, &ctx, &source),
            crate::conv::normalize(&prior_shadow, &ctx, &source),
            "barrier view must reproduce the prior driver's normalized body"
        );
        assert!(env.with_recursion_barriers(&[ids_nat]).is_err());
        let zero_again =
            Term::constructor(env.inductive(ids_nat).unwrap().constructors[0].id, vec![]);
        let acyclic =
            declare_def(&mut env, vec![], nat, zero_again).expect("checked nonrecursive constant");
        assert!(!env.is_recursive_transparent(acyclic));
        assert!(env.with_recursion_barriers(&[acyclic]).is_err());
    }

    #[test]
    fn let_check_rejects_wrong_outer_expected_type() {
        let (env, ids) = bool_nat_env();
        let ctx = Context::new();
        let let_zero_as_bool = let_term(nat_ty(&ids), zero_term(&ids), Term::var(0));

        assert!(matches!(
            check(&env, &ctx, &let_zero_as_bool, &bool_ty(&ids)),
            Err(KernelError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn let_check_accepts_valid_body_at_outer_expected_type() {
        let (env, ids) = bool_nat_env();
        let ctx = Context::new();
        let let_true_at_bool = let_term(nat_ty(&ids), zero_term(&ids), true_term(&ids));

        assert!(check(&env, &ctx, &let_true_at_bool, &bool_ty(&ids)).is_ok());
    }

    #[test]
    fn let_check_preserves_check_mode_for_intro_body() {
        let (env, ids) = bool_nat_env();
        let ctx = Context::new();
        let pi_bool_bool = Term::pi(bool_ty(&ids), bool_ty(&ids));
        let let_lambda = let_term(
            nat_ty(&ids),
            zero_term(&ids),
            Term::lam(bool_ty(&ids), Term::var(0)),
        );

        assert!(check(&env, &ctx, &let_lambda, &pi_bool_bool).is_ok());
    }

    #[test]
    fn universe_no_type_type() {
        // Type 0 : Type 1; but Type 0 is NOT a Type 0 (no Type:Type).
        let env = GlobalEnv::new();
        let ctx = Context::new();
        // infer Type 0 ⇒ Type 1
        assert_eq!(
            infer(&env, &ctx, &Term::Type(Level::zero())),
            Ok(Term::Type(Level::suc(Level::zero())))
        );
        // check Type 0 ⇐ Type 1  → ok
        assert!(check(
            &env,
            &ctx,
            &Term::Type(Level::zero()),
            &Term::Type(Level::suc(Level::zero()))
        )
        .is_ok());
        // check Type 0 ⇐ Type 0  → REJECT (AC-1)
        assert!(check(
            &env,
            &ctx,
            &Term::Type(Level::zero()),
            &Term::Type(Level::zero())
        )
        .is_err());
    }

    #[test]
    fn k2_omega_formation() {
        let env = GlobalEnv::new();
        let ctx = Context::new();
        // Ω_l : Type (suc l) (`16 §1.1`). Ω_0 : Type 1.
        assert_eq!(
            infer(&env, &ctx, &Term::Omega(Level::zero())),
            Ok(Term::Type(Level::suc(Level::zero())))
        );
        // Ω_0 checks against Type 1 (its universe).
        assert!(check(
            &env,
            &ctx,
            &Term::Omega(Level::zero()),
            &Term::Type(Level::suc(Level::zero()))
        )
        .is_ok());
        // Non-cumulative (`12 §3`): Ω_0 : Type 1 does NOT give Ω_0 : Type 0.
        assert!(check(
            &env,
            &ctx,
            &Term::Omega(Level::zero()),
            &Term::Type(Level::zero())
        )
        .is_err());
    }

    #[test]
    fn k2_piproduct_over_omega_lands_in_omega() {
        // 13 §4 / 16 §1.1: a Π whose codomain is a proposition lands in Ω.
        // (P : Ω_0) → Top  with Top : Ω_0  ⇒  (P → Top) : Ω_0.  Using the closed
        // prelude `Top` as the codomain avoids a de Bruijn shift on the body.
        let env = GlobalEnv::new();
        let mut ctx = Context::new();
        ctx.push(Term::Omega(Level::zero())); // P : Ω_0  (var 0)
        let top = Term::Const {
            id: env.top_id(),
            level_args: Vec::new(),
        }; // Top : Ω_0 (closed)
        let pi = Term::pi(Term::var(0), top.clone()); // (x : P) → Top
        assert_eq!(infer(&env, &ctx, &pi), Ok(Term::Omega(Level::zero())));
        // A Σ over a proposition codomain also lands in Ω_0.
        let sig = Term::sigma(Term::var(0), top); // (x : P) × Top
        assert_eq!(infer(&env, &ctx, &sig), Ok(Term::Omega(Level::zero())));
    }
}
