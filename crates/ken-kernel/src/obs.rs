//! K2 observational reduction — `Eq`-by-type, `cast`-by-type, and the derived
//! `J` (`15-identity.md`, `16-observational.md` §2–§4). This is the ADR-0005
//! core: equality is a proposition computed by recursion on the type, and
//! `cast` transports along a type-equality computing from the **endpoints**
//! (never the proof `e`), so `J` reduces on non-`refl` equalities.
//!
//! `Eq` and `cast` form a **single mutual reduction system** (`16 §3.3`):
//! `Eq` at a compound type calls `cast` on component types (Σ, inductive), and
//! `cast` at a compound type recovers its sub-equality proofs by *projecting*
//! the proof `e` (`Eq Type (…) (…)` reduces to a Σ/Π of component equalities, so
//! `e.1`, `e.2` are the sub-proofs). Termination is structural on the type tree.
//!
//! Called from [`crate::conv::whnf`]. A neutral head type, mismatched heads, or
//! a sub-proof the kernel cannot build leaves the `Eq`/`cast` **neutral**
//! (stuck) — sound: a stuck `Eq`/`cast` simply does not reduce.

use crate::check::{classify, Sort};
use crate::conv::{convert_type, convert_type_deferred_operands, whnf};
use crate::env::{Context, GlobalEnv};
use crate::inductive::peel_app;
use crate::subst::{apply_args, shift, subst0, subst_levels, subst_outer, subst_tel, weaken};
use crate::term::{Level, Term};

// --- prelude proposition terms (`16 §1.3`) ---

/// `Top : Ω_0` — the truth proposition (the prelude constant).
pub fn top_term(env: &GlobalEnv) -> Term {
    Term::Const {
        id: env.top_id(),
        level_args: Vec::new(),
    }
}

/// `Bottom : Ω_0` — the falsity proposition (the prelude constant).
pub fn bottom_term(env: &GlobalEnv) -> Term {
    Term::Const {
        id: env.bottom_id(),
        level_args: Vec::new(),
    }
}

/// `tt : Top` — `Top`'s sole introduction (the prelude constant, `16 §1.3`,
/// K5).
pub fn tt_term(env: &GlobalEnv) -> Term {
    Term::Const {
        id: env.tt_id(),
        level_args: Vec::new(),
    }
}

/// Is `t` a `refl` proof?
fn is_refl(t: &Term) -> bool {
    matches!(t, Term::Refl(_))
}

/// Typed symmetry of `h : Eq (Type l) A B`, including neutral evidence.
/// Both the Π type-equality arm and Π cast use this identical J construction.
pub(crate) fn type_eq_sym(level: &crate::term::Level, a: &Term, b: &Term, h: Term) -> Term {
    let type_l = Term::Type(level.clone());
    let proof_domain = Term::Eq(
        Box::new(type_l.clone()),
        Box::new(weaken(a, 1)),
        Box::new(Term::var(0)),
    );
    let motive = Term::Ascript(
        Box::new(Term::lam(
            type_l.clone(),
            Term::lam(
                proof_domain.clone(),
                Term::Eq(
                    Box::new(type_l.clone()),
                    Box::new(Term::var(1)),
                    Box::new(weaken(a, 2)),
                ),
            ),
        )),
        Box::new(Term::pi(
            type_l.clone(),
            Term::pi(proof_domain, Term::Omega(level.clone().suc())),
        )),
    );
    Term::J(
        Box::new(motive),
        Box::new(Term::Refl(Box::new(a.clone()))),
        Box::new(Term::Ascript(
            Box::new(h),
            Box::new(Term::Eq(
                Box::new(type_l),
                Box::new(a.clone()),
                Box::new(b.clone()),
            )),
        )),
    )
}

/// Universe level of a type, only when the kernel can infer it.
fn type_level(env: &GlobalEnv, ctx: &Context, ty: &Term) -> Option<crate::term::Level> {
    match crate::check::infer(env, ctx, ty).ok()? {
        Term::Type(level) => Some(level),
        _ => None,
    }
}

fn type_eq(env: &GlobalEnv, ctx: &Context, a: &Term, b: &Term) -> Option<Term> {
    let level = type_level(env, ctx, a)?;
    if !level.equiv(&type_level(env, ctx, b)?) {
        return None;
    }
    Some(Term::Eq(
        Box::new(Term::Type(level)),
        Box::new(a.clone()),
        Box::new(b.clone()),
    ))
}

// ===========================================================================
// Eq-by-type (`16 §2.2`)
// ===========================================================================

/// Whether a type itself inhabits Ω, rather than being an Ω-valued universe.
fn omega_classified(env: &GlobalEnv, ctx: &Context, ty: &Term) -> bool {
    matches!(classify(env, ctx, ty), Ok(Sort::Omega(_)))
}

/// Reduce `Eq ty a b` by recursion on the (already-whnf'd) type `ty`
/// (`16 §2.2`). Returns the reduct, or `None` if `ty` is neutral or Ω-classified.
pub fn eq_reduce(env: &GlobalEnv, ctx: &Context, ty: &Term, a: &Term, b: &Term) -> Option<Term> {
    // An Ω carrier is proof-irrelevant, regardless of its canonical head.
    // In particular, no Π/Σ or Trunc arm may produce an Eq reduct for it.
    if omega_classified(env, ctx, ty) {
        return None;
    }
    match ty {
        Term::Pi(a1, b1) => Some(eq_at_pi(a1, b1, a, b)),
        Term::Sigma(a1, b1) => eq_at_sigma(env, ctx, a1, b1, a, b),
        Term::Omega(_) => Some(eq_at_omega(a, b)),
        Term::Type(_) => eq_at_type(env, ctx, a, b),
        // Formation checked an equivalence proof at the carrier's Ω level.
        // Only two canonical classes expose representatives; open endpoints
        // leave quotient equality neutral (`16 §2.2`, §5).
        Term::Quot(_, r, _) => match (whnf(env, ctx, a), whnf(env, ctx, b)) {
            (Term::QuotClass(x), Term::QuotClass(y)) => Some(apply_args((**r).clone(), &[*x, *y])),
            _ => None,
        },
        Term::App(_, _) | Term::IndFormer { .. } => eq_at_inductive(env, ctx, ty, a, b),
        // A primitive type with a registered decidable-equality certificate
        // (ADR 0013 Layer 2) decides `Eq` between two checked literals by
        // value; general opt-in gate (`GlobalEnv::deceq_cert`), not
        // hardcoded to any specific primitive. An unregistered primitive
        // type falls through to the neutral default below, unchanged.
        Term::Const { id, .. } if env.deceq_cert(*id).is_some() => {
            eq_at_registered_literal(env, ctx, a, b)
        }
        // Primitive types enter as global declarations (`11 §1`); K2 defines no
        // `primEq` reduction yet — `Eq` at a primitive type stays neutral.
        _ => None,
    }
}

/// `Eq ty (IntLit m) (IntLit n) ⇝ Top if m == n else Bottom` for a `ty` with
/// a registered decidable-equality certificate (ADR 0013 Layer 2, `docs/adr/
/// 0013-int-decidable-equality-kernel-posture.md`). Neutral if either
/// operand is not (already, or after `whnf`) a literal — covers abstract
/// variables at any binder depth, and any other non-canonical operand
/// shape, with the same `None` default every other reduction arm uses.
///
/// `m == n` decides `num_bigint::BigInt` value equality — the SAME
/// crate/type/operator the registered primitive's runtime decider computes
/// (e.g. `eq_int`'s interp-side implementation), not a second,
/// independently-written comparison; pinned by a cross-layer test, not left
/// as an inspection-only claim.
fn eq_at_registered_literal(env: &GlobalEnv, ctx: &Context, a: &Term, b: &Term) -> Option<Term> {
    let a_w = whnf(env, ctx, a);
    let b_w = whnf(env, ctx, b);
    match (&a_w, &b_w) {
        (Term::IntLit(m), Term::IntLit(n)) => Some(if m == n {
            top_term(env)
        } else {
            bottom_term(env)
        }),
        _ => None,
    }
}

/// `Eq ((x:A1)→B1) f g ⇝ (x:A1) → Eq (B1 x) (f x) (g x)` — funext definitional
/// (`16 §2.2`). The body's `Eq` reduces lazily by later `whnf` calls.
fn eq_at_pi(a1: &Term, b1: &Term, f: &Term, g: &Term) -> Term {
    // Under the binder `x : A1` (de Bruijn 0): `B1 x`, `f x`, `g x`. `B1` already
    // has `x` at index 0 (it is the Π codomain), so `B1 x` is `B1` itself; `f`,`g`
    // live in the outer context and are weakened by 1 past the new binder.
    let b1_x = b1.clone();
    let f_x = Term::app(weaken(f, 1), Term::var(0));
    let g_x = Term::app(weaken(g, 1), Term::var(0));
    Term::pi(
        a1.clone(),
        Term::Eq(Box::new(b1_x), Box::new(f_x), Box::new(g_x)),
    )
}

/// Introduce a checked reflexive base even when `Eq Type X X` decomposes to
/// a compound proposition. A generic neutral proposition still has no
/// fabricated witness. The caller constructs `Eq Type X X` itself.
fn canonical_type_eq_base(env: &GlobalEnv, ctx: &Context, base_ty: &Term) -> Option<Term> {
    if let Term::Eq(_, x, y) = base_ty {
        if convert_type(env, ctx, x, y) {
            let base = Term::Refl(x.clone());
            crate::check::check(env, ctx, &base, base_ty).ok()?;
            return Some(base);
        }
    }
    match whnf(env, ctx, base_ty) {
        Term::Eq(_, x, _) => Some(Term::Refl(x)),
        head if head == top_term(env) => Some(tt_term(env)),
        _ => None,
    }
}

/// Build `cong F h : Eq Type (F a) (F b)` by `J` over the evidence `h`.
/// `family_at_y` lives under `y` and its equality proof (Var(1) and Var(0)).
/// The base must have a canonical introduction after WHNF; otherwise leave
/// the reduction neutral rather than fabricate a type-equality witness.
fn type_eq_by_j(
    env: &GlobalEnv,
    ctx: &Context,
    domain: &Term,
    start: &Term,
    source: &Term,
    target: &Term,
    family_at_y: Term,
    evidence: Term,
    typed_by_lemma: bool,
) -> Option<Term> {
    let level = match crate::check::infer(env, ctx, source).ok()? {
        Term::Type(level) => level,
        _ => return None,
    };
    let base_ty = Term::Eq(
        Box::new(Term::Type(level.clone())),
        Box::new(source.clone()),
        Box::new(source.clone()),
    );
    let base = canonical_type_eq_base(env, ctx, &base_ty)?;
    type_eq_by_j_with_base(
        env,
        ctx,
        domain,
        start,
        source,
        target,
        family_at_y,
        evidence,
        base,
        typed_by_lemma,
    )
}

/// Chain a J transport onto a previously checked Eq Type witness. The left
/// endpoint remains the original source, while the right endpoint advances
/// through the forced constructor arguments one index at a time.
///
/// J-witness typing lemma: let Γ ⊢ eq : Eq A a b. Let
/// Γ ⊢ P : (y:A) → Eq A a y → Type l and Γ ⊢ d : P a (refl a). Then
/// W := J (λy h. Eq (Type l) (P a (refl a)) (P y h))
///        (refl (P a (refl a))) eq
/// has type Eq (Type l) (P a (refl a)) (P b eq), and
/// cast (P a (refl a)) (P b eq) W d has type P b eq. The motive has sort
/// Ω_(l+1) by Eq-Form; its base type β-reduces to Eq (Type l) X X for
/// X := P a (refl a). The J rule types W; the Cast rule types the result.
/// Only `j_nonrefl` takes `typed_by_lemma = true`: production then relies on
/// this lemma instead of checking W. The other three builders have no typed
/// redex and retain the fail-closed witness check in production.
///
/// Side conditions (research `evt_4xjj6256dapa4`):
/// (i) Motive sort and level. P's codomain is `Type l`, never Ω. The Ω case
/// stays neutral, as the `infer(source)` gate does today. `l` comes from
/// formation, not from a reduct; `infer` is syntactic, so Eq-reduct level
/// gaps cannot perturb it.
/// (ii) One `eq`, one set of endpoints. The `eq` term, and the `a` and `b`
/// that W and `p_b_e` mention, must be the same terms the guard typed under
/// the existing ascription. Reducing endpoints before constructing W breaks
/// this alignment. At a universe carrier `j_nonrefl` instead reads the
/// inferred Eq formation directly, then calls `infer_j_at` at those very
/// endpoints; the other carriers retain `j_endpoints`.
/// (iii) The base typing `d : P a (refl a)`. Cast needs it. It is part of
/// what the once-only `infer_j_at` guard establishes.
/// (iv) de Bruijn hygiene. `motive_at_y` is the motive weakened by 2 and
/// applied to `@1 @0`. The kernel checks this lemma instance in tests, except
/// the production-cost count pins that suppress the test-only assertion.
fn type_eq_by_j_with_base(
    env: &GlobalEnv,
    ctx: &Context,
    domain: &Term,
    start: &Term,
    source: &Term,
    target: &Term,
    family_at_y: Term,
    evidence: Term,
    base: Term,
    typed_by_lemma: bool,
) -> Option<Term> {
    let level = match crate::check::infer(env, ctx, source).ok()? {
        Term::Type(level) => level,
        _ => return None,
    };
    let proof_domain = Term::Eq(
        Box::new(weaken(domain, 1)),
        Box::new(weaken(start, 1)),
        Box::new(Term::var(0)),
    );
    let motive = Term::Ascript(
        Box::new(Term::lam(
            domain.clone(),
            Term::lam(
                proof_domain.clone(),
                Term::Eq(
                    Box::new(Term::Type(level.clone())),
                    Box::new(weaken(source, 2)),
                    Box::new(family_at_y),
                ),
            ),
        )),
        Box::new(Term::pi(
            domain.clone(),
            Term::pi(proof_domain, Term::Omega(level.clone().suc())),
        )),
    );
    let result = Term::J(Box::new(motive), Box::new(base), Box::new(evidence));
    let expected = Term::Eq(
        Box::new(Term::Type(level)),
        Box::new(source.clone()),
        Box::new(target.clone()),
    );
    if typed_by_lemma {
        // Only J reduction has a redex checked by infer_j_at. Production
        // relies on the lemma; tests check each instance except count pins.
        #[cfg(test)]
        if crate::conv::deferred_fixed_point_assertions_enabled() {
            assert_eq!(
                crate::check::check(env, ctx, &result, &expected),
                Ok(()),
                "J-witness lemma instance: source={source:?}, target={target:?}, W={result:?}"
            );
        }
    } else {
        // Σ, inductive Eq and inductive Cast construct their own witnesses;
        // their fail-closed production check cannot rely on the J lemma.
        crate::check::check(env, ctx, &result, &expected).ok()?;
    }
    Some(result)
}

/// `Eq ((x:A1)×B1) p q` compares first projections, then the second
/// components at `B1 q.1` (`16 §2.2`). An Ω codomain discards the source proof
/// and compares `q.2` to itself; a Type codomain transports `p.2` using J.
/// The first-projection equality proof is available inside the Σ codomain.
fn eq_at_sigma(
    env: &GlobalEnv,
    ctx: &Context,
    a1: &Term,
    b1: &Term,
    p: &Term,
    q: &Term,
) -> Option<Term> {
    let p1 = whnf(env, ctx, &Term::proj1(p.clone()));
    let q1 = whnf(env, ctx, &Term::proj1(q.clone()));
    let eq_fst = Term::Eq(
        Box::new(a1.clone()),
        Box::new(p1.clone()),
        Box::new(q1.clone()),
    );
    let b1_p1 = subst0(b1, &p1);
    let b1_q1 = subst0(b1, &q1);
    if omega_classified(env, ctx, &b1_q1) {
        let target = weaken(&whnf(env, ctx, &Term::proj2(q.clone())), 1);
        let second = Term::Eq(
            Box::new(weaken(&b1_q1, 1)),
            Box::new(target.clone()),
            Box::new(target),
        );
        return Some(Term::sigma(eq_fst, second));
    }
    let mut proof_ctx = ctx.clone();
    proof_ctx.push(eq_fst.clone());
    // At the motive's depth Γ, h, y, _ the original B1 is still under its
    // x binder. Shift only its free outer variables by three, then replace
    // x by y (Var(1)); h is the Σ proof's nearest binder in Γ, h.
    let b1_at_y = subst0(&shift(b1, 3, 1), &Term::var(1));
    let proof = type_eq_by_j(
        env,
        &proof_ctx,
        &weaken(a1, 1),
        &weaken(&p1, 1),
        &weaken(&b1_p1, 1),
        &weaken(&b1_q1, 1),
        b1_at_y,
        Term::Ascript(Box::new(Term::var(0)), Box::new(weaken(&eq_fst, 1))),
        false,
    )?;
    let p2_cast = Term::Cast(
        Box::new(weaken(&b1_p1, 1)),
        Box::new(weaken(&b1_q1, 1)),
        Box::new(proof),
        Box::new(weaken(&whnf(env, ctx, &Term::proj2(p.clone())), 1)),
    );
    let second = Term::Eq(
        Box::new(weaken(&b1_q1, 1)),
        Box::new(p2_cast),
        Box::new(weaken(&whnf(env, ctx, &Term::proj2(q.clone())), 1)),
    );
    Some(Term::sigma(eq_fst, second))
}

/// `Eq Ω P Q ⇝ (P → Q) and (Q → P)` — propext definitional (`16 §2.2`).
fn eq_at_omega(p: &Term, q: &Term) -> Term {
    let p_to_q = Term::pi(p.clone(), q.clone()); // (x:P) → Q
    let q_to_p = Term::pi(q.clone(), p.clone()); // (x:Q) → P
    Term::sigma(p_to_q, q_to_p) // (P→Q) and (Q→P)
}

/// Only a rigid type former can establish disjointness from another rigid
/// former. Applications are rigid solely when headed by an inductive family;
/// a neutral application could later instantiate to either side's head.
#[derive(PartialEq, Eq)]
enum RigidTypeFormer {
    Pi,
    Sigma,
    Omega,
    Type,
    Inductive(crate::term::GlobalId),
    Quot,
    Trunc,
}

fn rigid_type_former(ty: &Term) -> Option<RigidTypeFormer> {
    match ty {
        Term::Pi(..) => Some(RigidTypeFormer::Pi),
        Term::Sigma(..) => Some(RigidTypeFormer::Sigma),
        Term::Omega(..) => Some(RigidTypeFormer::Omega),
        Term::Type(..) => Some(RigidTypeFormer::Type),
        Term::IndFormer { id, .. } => Some(RigidTypeFormer::Inductive(*id)),
        Term::App(..) => {
            let mut head = ty;
            while let Term::App(f, _) = head {
                head = f;
            }
            match head {
                Term::IndFormer { id, .. } => Some(RigidTypeFormer::Inductive(*id)),
                _ => None,
            }
        }
        Term::Quot(..) => Some(RigidTypeFormer::Quot),
        Term::Trunc(..) => Some(RigidTypeFormer::Trunc),
        _ => None,
    }
}

/// An open level is not known distinct merely because normalization cannot
/// prove equality. Structural universe inequality is decidable only when both
/// sides are closed, so the open case must remain neutral.
fn closed_level(level: &Level) -> bool {
    let mut pending = vec![level];
    while let Some(node) = pending.pop() {
        match node {
            Level::Zero => {}
            Level::Var(_) => return false,
            Level::Suc(inner) => pending.push(inner),
            Level::Max(left, right) => {
                pending.push(left);
                pending.push(right);
            }
        }
    }
    true
}

/// Structural type equality `Eq Type A B` (`16 §2.2`, §3). Unknown heads
/// and unsupported same-former pairs remain neutral; distinct rigid heads
/// reduce to `Bottom`, not an unproved equality.
fn eq_at_type(env: &GlobalEnv, ctx: &Context, a: &Term, b: &Term) -> Option<Term> {
    let a_w = whnf(env, ctx, a);
    let b_w = whnf(env, ctx, b);
    match (&a_w, &b_w) {
        (Term::Type(l1), Term::Type(l2)) | (Term::Omega(l1), Term::Omega(l2)) => {
            if l1.equiv(l2) {
                Some(top_term(env))
            } else if closed_level(l1) && closed_level(l2) {
                Some(bottom_term(env))
            } else {
                None
            }
        }
        (Term::Pi(a1, b1), Term::Pi(a2, b2)) => {
            let level = type_level(env, ctx, a1)?;
            if !level.equiv(&type_level(env, ctx, a2)?) {
                return None;
            }
            let dom_eq = type_eq(env, ctx, a1, a2)?;
            let mut cod_ctx = ctx.clone();
            cod_ctx.push(dom_eq.clone());
            cod_ctx.push(weaken(a2, 1));
            let back = Term::Cast(
                Box::new(weaken(a2, 2)),
                Box::new(weaken(a1, 2)),
                Box::new(type_eq_sym(
                    &level,
                    &weaken(a1, 2),
                    &weaken(a2, 2),
                    Term::var(1),
                )),
                Box::new(Term::var(0)),
            );
            // `subst0` removes B1's original binder. Shift its free Γ
            // references past d and x *plus* that removal first.
            let left = subst0(&shift(b1, 2, 1), &back);
            let right = shift(b2, 1, 1);
            let cod_eq = type_eq(env, &cod_ctx, &left, &right)?;
            Some(Term::sigma(dom_eq, Term::pi(weaken(a2, 1), cod_eq)))
        }
        (Term::Sigma(a1, b1), Term::Sigma(a2, b2)) => {
            let dom_eq = type_eq(env, ctx, a1, a2)?;
            let mut cod_ctx = ctx.clone();
            cod_ctx.push(dom_eq.clone());
            cod_ctx.push(weaken(a1, 1));
            let forward = Term::Cast(
                Box::new(weaken(a1, 2)),
                Box::new(weaken(a2, 2)),
                Box::new(Term::var(1)),
                Box::new(Term::var(0)),
            );
            let cod_eq = type_eq(
                env,
                &cod_ctx,
                &shift(b1, 1, 1),
                &subst0(&shift(b2, 2, 1), &forward),
            )?;
            Some(Term::sigma(dom_eq, Term::pi(weaken(a1, 1), cod_eq)))
        }
        (Term::Quot(a1, r, _e), Term::Quot(a2, s, _f)) => {
            let dom_eq = type_eq(env, ctx, a1, a2)?;
            let mut rel_ctx = ctx.clone();
            rel_ctx.push(dom_eq.clone());
            rel_ctx.push(weaken(a1, 1));
            rel_ctx.push(weaken(a1, 2));
            let x = Term::var(1);
            let y = Term::var(0);
            let r_xy = apply_args(weaken(r, 3), &[x.clone(), y.clone()]);
            let d = Term::var(2);
            let forward = |v: Term| {
                Term::Cast(
                    Box::new(weaken(a1, 3)),
                    Box::new(weaken(a2, 3)),
                    Box::new(d.clone()),
                    Box::new(v),
                )
            };
            let s_xy = apply_args(weaken(s, 3), &[forward(x), forward(y)]);
            let r_level = match crate::check::infer(env, &rel_ctx, &r_xy).ok()? {
                Term::Omega(level) => level,
                _ => return None,
            };
            let s_level = match crate::check::infer(env, &rel_ctx, &s_xy).ok()? {
                Term::Omega(level) => level,
                _ => return None,
            };
            if !r_level.equiv(&s_level) {
                return None;
            }
            let rel_eq = Term::Eq(
                Box::new(Term::Omega(r_level)),
                Box::new(r_xy),
                Box::new(s_xy),
            );
            Some(Term::sigma(
                dom_eq,
                Term::pi(weaken(a1, 1), Term::pi(weaken(a1, 2), rel_eq)),
            ))
        }
        (Term::App(..) | Term::IndFormer { .. }, Term::App(..) | Term::IndFormer { .. })
            if rigid_type_former(&a_w) == rigid_type_former(&b_w) =>
        {
            eq_type_at_inductive(env, ctx, &a_w, &b_w)
        }
        // A single known head does not decide inequality: the other side
        // might be neutral and later instantiate to that very former.
        _ => match (rigid_type_former(&a_w), rigid_type_former(&b_w)) {
            (Some(left), Some(right)) if left != right => Some(bottom_term(env)),
            _ => None,
        },
    }
}

/// Structural equality of fully applied instances of the same inductive
/// family, using precisely the telescope conjunction used for constructor
/// equality. A partial family or mismatched level application remains neutral.
fn eq_type_at_inductive(env: &GlobalEnv, ctx: &Context, a: &Term, b: &Term) -> Option<Term> {
    let (a_head, a_args) = peel_app(a);
    let (b_head, b_args) = peel_app(b);
    let (
        Term::IndFormer {
            id: a_id,
            level_args: a_levels,
        },
        Term::IndFormer {
            id: b_id,
            level_args: b_levels,
        },
    ) = (a_head, b_head)
    else {
        return None;
    };
    if a_id != b_id
        || a_levels.len() != b_levels.len()
        || !a_levels.iter().zip(&b_levels).all(|(x, y)| x.equiv(y))
    {
        return None;
    }
    let ind = env.inductive(a_id)?;
    let n = ind.params.len() + ind.indices.len();
    if a_args.len() != n || b_args.len() != n {
        return None;
    }
    if n == 0 {
        return Some(top_term(env));
    }
    let (mut a_tpl, mut b_tpl) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for j in 0..n {
        let binder = if j < ind.params.len() {
            &ind.params[j]
        } else {
            &ind.indices[j - ind.params.len()]
        };
        a_tpl.push(subst_levels(binder, &ind.level_params, &a_levels));
        b_tpl.push(subst_levels(binder, &ind.level_params, &b_levels));
    }
    inductive_conjuncts(env, ctx, &a_tpl, &b_tpl, &a_args, &b_args, 0)
}

/// A right-nested constructor prefix `(x1, (x2, ...))`, with the final
/// field unpaired. This is the carrier of the earlier conjunct proofs.
fn telescope_tuple(args: &[Term]) -> Term {
    let mut value = args[args.len() - 1].clone();
    for arg in args[..args.len() - 1].iter().rev() {
        value = Term::pair(arg.clone(), value);
    }
    value
}

/// The kth element of a right-nested tuple `t` of `len` fields. The final
/// projection is `t.2.2...`; every earlier one ends in `.1`.
fn telescope_projection(t: &Term, k: usize, len: usize) -> Term {
    let mut result = t.clone();
    for _ in 0..k {
        result = Term::proj2(result);
    }
    if k + 1 < len {
        Term::proj1(result)
    } else {
        result
    }
}

/// Build one inductive argument equality in the context where all preceding
/// equality conjuncts are bound. An Ω field compares the target proof with
/// itself; a non-Ω dependent cast uses `J` over the right-nested prefix proof
/// tuple. The conjunct is then bound for the remaining suffix.
fn inductive_conjuncts(
    env: &GlobalEnv,
    proof_ctx: &Context,
    a_tpl: &[Term],
    b_tpl: &[Term],
    a_bar: &[Term],
    b_bar: &[Term],
    j: usize,
) -> Option<Term> {
    let a_ty = weaken(&subst_tel(&a_tpl[j], &a_bar[..j]), j as i64);
    let b_ty = weaken(&subst_tel(&b_tpl[j], &b_bar[..j]), j as i64);
    let target = weaken(&b_bar[j], j as i64);
    let conjunct = if omega_classified(env, proof_ctx, &b_ty) {
        // Ω proofs are irrelevant: compare the target proof to itself without
        // transporting the source proof or constructing a J witness.
        Term::Eq(Box::new(b_ty), Box::new(target.clone()), Box::new(target))
    } else {
        let lhs = if convert_type(env, proof_ctx, &a_ty, &b_ty) {
            weaken(&a_bar[j], j as i64)
        } else {
            let prefix = (0..j).rev().fold(None, |tail: Option<Term>, k| {
                Some(match tail {
                    Some(rest) => Term::sigma(a_tpl[k].clone(), rest),
                    None => a_tpl[k].clone(),
                })
            })?;
            let prefix = weaken(&prefix, j as i64);
            let a_values = a_bar[..j]
                .iter()
                .map(|x| weaken(x, j as i64))
                .collect::<Vec<_>>();
            let b_values = b_bar[..j]
                .iter()
                .map(|x| weaken(x, j as i64))
                .collect::<Vec<_>>();
            let (left, right) = if j == 1 {
                (telescope_tuple(&a_values), telescope_tuple(&b_values))
            } else {
                (
                    Term::Ascript(
                        Box::new(telescope_tuple(&a_values)),
                        Box::new(prefix.clone()),
                    ),
                    Term::Ascript(
                        Box::new(telescope_tuple(&b_values)),
                        Box::new(prefix.clone()),
                    ),
                )
            };
            let prefix_eq = Term::Eq(
                Box::new(prefix.clone()),
                Box::new(left.clone()),
                Box::new(right),
            );
            let earlier = (0..j).map(|k| Term::var(j - 1 - k)).collect::<Vec<_>>();
            let evidence = Term::Ascript(
                Box::new(if j == 1 {
                    earlier[0].clone()
                } else {
                    telescope_tuple(&earlier)
                }),
                Box::new(prefix_eq),
            );
            // Shift the outer context past the j constructor positions and the
            // j+2 proof/motive binders; substitute projected tuple components for
            // those constructor positions. The second motive binder is ignored.
            let at_y = (0..j)
                .map(|k| telescope_projection(&Term::var(1), k, j))
                .collect::<Vec<_>>();
            let family_at_y = subst_tel(&shift(&a_tpl[j], j as i64 + 2, j), &at_y);
            let witness = type_eq_by_j(
                env,
                proof_ctx,
                &prefix,
                &left,
                &a_ty,
                &b_ty,
                family_at_y,
                evidence,
                false,
            )?;
            Term::Cast(
                Box::new(a_ty),
                Box::new(b_ty.clone()),
                Box::new(witness),
                Box::new(weaken(&a_bar[j], j as i64)),
            )
        };
        Term::Eq(Box::new(b_ty), Box::new(lhs), Box::new(target))
    };
    if j + 1 == a_tpl.len() {
        Some(conjunct)
    } else {
        let mut next_ctx = proof_ctx.clone();
        next_ctx.push(conjunct.clone());
        Some(Term::sigma(
            conjunct,
            inductive_conjuncts(env, &next_ctx, a_tpl, b_tpl, a_bar, b_bar, j + 1)?,
        ))
    }
}

/// `Eq (D Δp ī) (c_k ā) (c_l b̄)` — equality at an inductive family (`16 §2.2`).
/// Same constructor ⇒ the conjunction of argument-equalities, with later
/// arguments transported along earlier-argument equalities (the dependent
/// telescope `cast`s); different constructors ⇒ `Bottom`; a neutral scrutinee
/// ⇒ neutral.
fn eq_at_inductive(env: &GlobalEnv, ctx: &Context, ty: &Term, a: &Term, b: &Term) -> Option<Term> {
    let (head, _ty_args) = peel_app(ty);
    let d_id = match head {
        Term::IndFormer { id, .. } => id,
        _ => return None,
    };
    let ind = env.inductive(d_id)?;
    let m = ind.params.len();
    let a_w = whnf(env, ctx, a);
    let b_w = whnf(env, ctx, b);
    let (a_head, a_args) = peel_app(&a_w);
    let (b_head, b_args) = peel_app(&b_w);
    let (a_ctor, a_level_args, a_ctor_args) = match a_head {
        Term::Constructor { id, level_args, .. } => (id, level_args, a_args),
        _ => return None, // neutral scrutinee ⇒ neutral Eq
    };
    let (b_ctor, _b_level_args, b_ctor_args) = match b_head {
        Term::Constructor { id, level_args, .. } => (id, level_args, b_args),
        _ => return None,
    };
    if a_ctor_args.len() < m || b_ctor_args.len() < m {
        return None;
    }
    let a_bar = &a_ctor_args[m..];
    let b_bar = &b_ctor_args[m..];
    if a_ctor != b_ctor {
        return Some(bottom_term(env));
    }
    let (ind2, k) = env.constructor(a_ctor)?;
    if ind2.id != d_id {
        return None;
    }
    let c = &ind2.constructors[k];
    if a_bar.len() != c.args.len() || b_bar.len() != c.args.len() {
        return None; // arity guard — yes/no, never crash
    }
    let n = c.args.len();
    let a_param_args = &a_ctor_args[..m];
    let b_param_args = &b_ctor_args[..m];
    if n == 0 {
        return Some(top_term(env));
    }
    let mut a_tpl = Vec::with_capacity(n);
    let mut b_tpl = Vec::with_capacity(n);
    for j in 0..n {
        // `A_j` with the `m` params substituted; the `j` earlier-arg binders
        // (de Bruijn 0..j-1) remain. Instantiate them with the actual earlier
        // args — `a_bar[..j]` for the source, `b_bar[..j]` for the target — via
        // `subst_tel` (the K1-fixed telescope subst). This is what makes the
        // dependent-telescope detection sound: a non-dependent position has
        // `a_ty_j ≡ b_ty_j` (the earlier-arg subst is irrelevant), while a
        // dependent one (whose type mentions a differing earlier arg) does not.
        let a_ty_tpl = subst_levels(
            &subst_outer(&c.args[j], m, a_param_args, j),
            &ind2.level_params,
            &a_level_args,
        );
        let b_ty_tpl = subst_levels(
            &subst_outer(&c.args[j], m, b_param_args, j),
            &ind2.level_params,
            &a_level_args,
        );
        a_tpl.push(a_ty_tpl);
        b_tpl.push(b_ty_tpl);
    }
    inductive_conjuncts(env, ctx, &a_tpl, &b_tpl, a_bar, b_bar, 0)
}

// ===========================================================================
// cast-by-type (`16 §3.2`)
// ===========================================================================

/// A compound cast may project `e.1`/`e.2` only if its Eq-at-Type arm
/// exposes a Σ of component equalities. Neutral Eq Type stays neutral.
fn type_eq_has_components(env: &GlobalEnv, ctx: &Context, a: &Term, b: &Term) -> bool {
    matches!(eq_at_type(env, ctx, a, b), Some(Term::Sigma(..)))
}

/// Reduce `cast a b e t` by recursion on the (whnf'd) types `a`,`b` (`16 §3.2`).
/// Returns the reduct, or `None` if the cast is stuck. `a`,`b` are whnf'd by the
/// caller. The proof `e` is **never inspected** for content — `cast` computes
/// from the endpoints; sub-equality proofs are *projected* from `e` (`16 §3.4`).
pub fn cast_reduce(
    env: &GlobalEnv,
    ctx: &Context,
    a: &Term,
    b: &Term,
    e: &Term,
    t: &Term,
) -> Option<Term> {
    // Regularity (`16 §3.2`): `cast A A refl a ⇝ a`. More generally, if `A ≡ B`
    // the transport is the identity (`e` is proof-irrelevant — `Eq Type A B : Ω`
    // when `A ≡ B`).
    if convert_type_deferred_operands(env, ctx, a, b) {
        return Some(t.clone());
    }
    match (a, b) {
        (Term::Pi(a1, b1), Term::Pi(a2, b2)) if type_eq_has_components(env, ctx, a, b) => {
            cast_at_pi(env, ctx, a1, b1, a2, b2, e, t)
        }
        (Term::Sigma(a1, b1), Term::Sigma(a2, b2)) if type_eq_has_components(env, ctx, a, b) => {
            Some(cast_at_sigma(env, ctx, a1, b1, a2, b2, e, t))
        }
        (Term::Omega(_), Term::Omega(_)) => Some(t.clone()), // cast Ω Ω e P ⇝ P
        (Term::App(_, _) | Term::IndFormer { .. }, Term::App(_, _) | Term::IndFormer { .. }) => {
            cast_at_inductive(env, ctx, a, b, e, t)
        }
        (Term::Quot(_, _, _), Term::Quot(_, _, _)) if type_eq_has_components(env, ctx, a, b) => {
            cast_at_quot(a, b, e, t)
        }
        // `cast Type Type (refl _) A ⇝ A`; non-refl type-equality at a universe
        // is (oracle) neutral (`16 §3.2`).
        (Term::Type(_), Term::Type(_)) if is_refl(e) => Some(t.clone()),
        // Mismatched heads, or a neutral type on either side ⇒ neutral cast.
        _ => None,
    }
}

/// `cast ((x:A1)→B1) ((x:A2)→B2) e f ⇝ λ(x:A2). cast (B1 (back x)) (B2 x)
/// (cod-eq x) (f (back x))` where `back x = cast A2 A1 (sym dom-eq) x`,
/// `dom-eq = e.1`, `cod-eq x = (e.2)x` (`16 §3.2`). Sub-equality proofs
/// are projected from `e`.
#[allow(clippy::too_many_arguments)]
fn cast_at_pi(
    env: &GlobalEnv,
    ctx: &Context,
    a1: &Term,
    b1: &Term,
    a2: &Term,
    b2: &Term,
    e: &Term,
    f: &Term,
) -> Option<Term> {
    let level = type_level(env, ctx, a1)?;
    if !level.equiv(&type_level(env, ctx, a2)?) {
        return None;
    }
    let dom_eq = Term::proj1(e.clone()); // e.1 : Eq Type A1 A2
    let sym_dom = type_eq_sym(&level, a1, a2, dom_eq);
    // back x = cast A2 A1 (sym dom-eq) x,  x:A2 at index 0 under the λ.
    let back_x = Term::Cast(
        Box::new(weaken(a2, 1)),
        Box::new(weaken(a1, 1)),
        Box::new(weaken(&sym_dom, 1)),
        Box::new(Term::var(0)),
    );
    // Keep the output λ binder: `subst0` removes B1's original x, so
    // compensate its free Γ references before replacing x by back x.
    let b1_back = subst0(&shift(b1, 1, 1), &back_x);
    let b2_x = b2.clone(); // B2[x / x] = B2  (B2's var 0 is already the λ's x)
    let cod_eq_x = Term::app(weaken(&Term::proj2(e.clone()), 1), Term::var(0)); // (e.2)x
    let f_back = Term::app(weaken(f, 1), back_x); // f (back x)
    Some(Term::lam(
        a2.clone(),
        Term::Cast(
            Box::new(b1_back),
            Box::new(b2_x),
            Box::new(cod_eq_x),
            Box::new(f_back),
        ),
    ))
}

/// `cast ((x:A1)×B1) ((x:A2)×B2) e p ⇝ (cast A1 A2 dom-eq p.1, cast (B1 p.1)
/// (B2 (cast A1 A2 dom-eq p.1)) cod-eq' p.2)` (`16 §3.2`). `dom-eq = e.1`,
/// `cod-eq' = (e.2)(p.1)`. No back-cast (Σ cast goes forward).
#[allow(clippy::too_many_arguments)]
fn cast_at_sigma(
    env: &GlobalEnv,
    ctx: &Context,
    a1: &Term,
    b1: &Term,
    a2: &Term,
    b2: &Term,
    e: &Term,
    p: &Term,
) -> Term {
    let _ = (env, ctx);
    let dom_eq = Term::proj1(e.clone()); // e.1 : Eq Type A1 A2
    let p1 = Term::proj1(p.clone());
    let p1_cast = Term::Cast(
        Box::new(a1.clone()),
        Box::new(a2.clone()),
        Box::new(dom_eq.clone()),
        Box::new(p1.clone()),
    );
    let b1_p1 = subst0(b1, &p1);
    let b2_p1c = subst0(b2, &p1_cast);
    let cod_eq_prime = Term::app(Term::proj2(e.clone()), p1.clone()); // (e.2)(p.1)
    let p2_cast = Term::Cast(
        Box::new(b1_p1),
        Box::new(b2_p1c),
        Box::new(cod_eq_prime),
        Box::new(Term::proj2(p.clone())),
    );
    Term::pair(p1_cast, p2_cast)
}

/// A family equality with at least one parameter/index exposes a right-nested
/// telescope of exactly that many equality components. A singleton telescope
/// is its Eq component, not a Σ. Unknown or incomplete decompositions cannot
/// justify projections from the original equality witness.
fn inductive_eq_has_telescope(
    env: &GlobalEnv,
    ctx: &Context,
    a: &Term,
    b: &Term,
    fields: usize,
) -> bool {
    let Some(reduct) = eq_at_type(env, ctx, a, b) else {
        return false;
    };
    let mut remainder = &reduct;
    for _ in 1..fields {
        let Term::Sigma(first, rest) = remainder else {
            return false;
        };
        if !matches!(&**first, Term::Eq(..)) {
            return false;
        }
        remainder = rest;
    }
    fields > 0 && matches!(remainder, Term::Eq(..))
}

/// A rebuilt constructor must actually inhabit the target family's indices.
/// This checks each position after substituting the new parameters and all
/// rebuilt constructor arguments, including template positions that did not
/// contribute a forced argument in the index-inversion pass.
#[allow(clippy::too_many_arguments)]
fn constructor_indices_match_target(
    env: &GlobalEnv,
    ctx: &Context,
    ind: &crate::env::InductiveDecl,
    ctor: &crate::env::ConstructorDecl,
    level_args: &[Level],
    params: &[Term],
    args: &[Term],
    target: &[Term],
) -> bool {
    let m = ind.params.len();
    let n = ctor.args.len();
    ctor.target_indices.len() == target.len()
        && ctor
            .target_indices
            .iter()
            .zip(target)
            .all(|(template, expected)| {
                let instantiated = subst_levels(
                    &subst_outer(template, m, params, n),
                    &ind.level_params,
                    level_args,
                );
                convert_type(env, ctx, &subst_tel(&instantiated, args), expected)
            })
}

/// `cast (D Δp ī) (D Δp j̄) e (c_k ā) ⇝ c_k (cast A_1 A_1' eq_1 a_1, …)` — each
/// constructor argument is transported from its `i`-bar type to its `j`-bar type
/// (`16 §3.2`). The sub-equalities come from the `Eq Type (D ī) (D j̄)`
/// decomposition of `e`.
fn cast_at_inductive(
    env: &GlobalEnv,
    ctx: &Context,
    a: &Term,
    b: &Term,
    e: &Term,
    t: &Term,
) -> Option<Term> {
    let (a_head, a_args) = peel_app(a);
    let (b_head, b_args) = peel_app(b);
    let (d_id, source_levels) = match a_head {
        Term::IndFormer { id, level_args } => (id, level_args),
        _ => return None,
    };
    let (b_id, target_levels) = match b_head {
        Term::IndFormer { id, level_args } => (id, level_args),
        _ => return None,
    };
    if d_id != b_id
        || source_levels.len() != target_levels.len()
        || !source_levels
            .iter()
            .zip(target_levels.iter())
            .all(|(source, target)| source.equiv(target))
    {
        return None;
    }
    let ind = env.inductive(d_id)?;
    let m = ind.params.len();
    if a_args.len() < m || b_args.len() < m {
        return None;
    }
    let (t_head, t_args) = peel_app(t);
    let (ctor, level_args, ctor_args) = match t_head {
        Term::Constructor { id, level_args, .. } => (id, level_args, t_args),
        _ => return None, // neutral scrutinee ⇒ neutral cast
    };
    let (ind2, k) = env.constructor(ctor)?;
    if ind2.id != d_id {
        return None;
    }
    let c = &ind2.constructors[k];
    let ctor_arg_vals = &ctor_args[m..];
    if ctor_arg_vals.len() != c.args.len() {
        return None; // arity guard
    }
    let a_param_args = &a_args[..m]; // source family params (from `a = D p̄ ī`)
    let b_param_args = &b_args[..m]; // target family params (from `b = D p̄' j̄`)
    let i_bar = &a_args[m..]; // source result indices
    let j_bar = &b_args[m..]; // target result indices
    let n_ctor = c.args.len();

    // Phase 1: detect whether any index pair differs.
    let mut index_changed = false;
    for (ix, jx) in i_bar.iter().zip(j_bar.iter()) {
        if !convert_type(env, ctx, ix, jx) {
            index_changed = true;
            break;
        }
    }

    if !index_changed {
        // Indices agree — only a param change could remain. Transport each arg
        // from its source-param type to its target-param type; stuck if the
        // type depends on a differing param.
        let mut new_args: Vec<Term> = b_param_args.to_vec();
        for (j, val) in ctor_arg_vals.iter().enumerate() {
            let a_ty_tpl = subst_levels(
                &subst_outer(&c.args[j], m, a_param_args, j),
                &ind2.level_params,
                &level_args,
            );
            let b_ty_tpl = subst_levels(
                &subst_outer(&c.args[j], m, b_param_args, j),
                &ind2.level_params,
                &level_args,
            );
            let a_ty_j = subst_tel(&a_ty_tpl, &ctor_arg_vals[..j]);
            let b_ty_j = subst_tel(&b_ty_tpl, &ctor_arg_vals[..j]);
            if !convert_type(env, ctx, &a_ty_j, &b_ty_j) {
                return None;
            }
            new_args.push(val.clone());
        }
        if !constructor_indices_match_target(
            env,
            ctx,
            ind,
            c,
            &level_args,
            b_param_args,
            &new_args[m..],
            &b_args[m..],
        ) {
            return None;
        }
        return Some(apply_args(
            Term::Constructor {
                id: ctor,
                level_args,
            },
            &new_args,
        ));
    }

    // Index change present. Require params to agree; a mixed param+index
    // change is out of scope — conservative stuck (sound).
    for (ap, bp) in a_param_args.iter().zip(b_param_args.iter()) {
        if !convert_type(env, ctx, ap, bp) {
            return None;
        }
    }

    // Phase 2: for each differing index slot, peel the ctor heads and read
    // off which ctor arg positions are "forced" to target-index values. The
    // guard (§3.2): both sides must be headed by the SAME ctor; a neutral or
    // mismatched-ctor index ⇒ stuck.
    //
    // `c.target_indices[p]` (after param-subst, inner_depth=n_ctor) is a
    // template whose `Var(k)` slots identify ctor arg positions: nat_pos =
    // (n_ctor - 1) - k. The target inner value at that position gives the
    // forced value.
    let mut forced_values: Vec<Option<Term>> = vec![None; n_ctor];
    let mut forced_indices: Vec<Option<usize>> = vec![None; n_ctor];
    for p in 0..i_bar.len() {
        if convert_type(env, ctx, &i_bar[p], &j_bar[p]) {
            continue; // this index slot agrees — skip
        }
        let (i_head, i_inner) = peel_app(&i_bar[p]);
        let (j_head, j_inner) = peel_app(&j_bar[p]);
        let i_ctor = match i_head {
            Term::Constructor { id, .. } => id,
            _ => return None, // neutral index — stuck (§3.2 guard)
        };
        let j_ctor = match j_head {
            Term::Constructor { id, .. } => id,
            _ => return None,
        };
        if i_ctor != j_ctor || i_inner.len() != j_inner.len() {
            return None; // different ctors or arity mismatch — stuck
        }
        let ti = subst_levels(
            &subst_outer(&c.target_indices[p], m, a_param_args, n_ctor),
            &ind2.level_params,
            &level_args,
        );
        let (ti_head, ti_inner) = peel_app(&ti);
        let ti_ctor = match ti_head {
            Term::Constructor { id, .. } => id,
            _ => return None,
        };
        if ti_ctor != i_ctor || ti_inner.len() != i_inner.len() {
            return None;
        }
        for (ti_arg, j_val) in ti_inner.iter().zip(j_inner.iter()) {
            if let Term::Var(vi) = ti_arg {
                let vi = *vi as usize;
                if vi < n_ctor {
                    let pos = (n_ctor - 1) - vi;
                    if forced_values[pos]
                        .as_ref()
                        .is_some_and(|old| !convert_type(env, ctx, old, j_val))
                    {
                        return None;
                    }
                    forced_values[pos] = Some(j_val.clone());
                    forced_indices[pos] = Some(m + p);
                }
            }
        }
    }

    // Phase 3: rebuild the constructor's arg list.
    //   forced position  → target index value
    //   non-dep position → source arg (same type on both sides)
    //   dep position     → sub-cast from a_ty_j to b_ty_j
    //
    // `target_earlier` tracks target-side earlier arg values so that
    // `subst_tel(&b_ty_tpl, &target_earlier)` gives the correct b_ty_j for
    // dependent arg types (those whose type mentions an earlier forced arg).
    // Each dependent sub-cast obtains a typed witness from the projected
    // family-index equality, never from reflexivity at a changed type.
    let mut new_args: Vec<Term> = b_param_args.to_vec();
    let mut target_earlier: Vec<Term> = vec![];
    for j in 0..n_ctor {
        let a_ty_tpl = subst_levels(
            &subst_outer(&c.args[j], m, a_param_args, j),
            &ind2.level_params,
            &level_args,
        );
        let b_ty_tpl = subst_levels(
            &subst_outer(&c.args[j], m, b_param_args, j),
            &ind2.level_params,
            &level_args,
        );
        let a_ty_j = subst_tel(&a_ty_tpl, &ctor_arg_vals[..j]);
        let b_ty_j = subst_tel(&b_ty_tpl, &target_earlier);
        let (new_val, target_val) = if let Some(fv) = &forced_values[j] {
            (fv.clone(), fv.clone())
        } else if convert_type(env, ctx, &a_ty_j, &b_ty_j) {
            (ctor_arg_vals[j].clone(), ctor_arg_vals[j].clone())
        } else {
            // Each changed forced argument corresponds to an index equality
            // projected from e. Change one earlier value at a time and chain
            // J transports over its projected equality. If a dependency
            // cannot be expressed by those checked projections, stay stuck.
            let changed = (0..j)
                .filter(|&k| {
                    forced_values[k]
                        .as_ref()
                        .is_some_and(|fv| !convert_type(env, ctx, &ctor_arg_vals[k], fv))
                })
                .collect::<Vec<_>>();
            if changed.is_empty() {
                return None;
            }
            let level = type_level(env, ctx, &a_ty_j)?;
            let base_ty = Term::Eq(
                Box::new(Term::Type(level.clone())),
                Box::new(a_ty_j.clone()),
                Box::new(a_ty_j.clone()),
            );
            let mut witness = canonical_type_eq_base(env, ctx, &base_ty)?;
            let mut mixed = ctor_arg_vals[..j].to_vec();
            let mut current_ty = a_ty_j.clone();
            for k in changed {
                let index = forced_indices[k]?;
                let target = forced_values[k].as_ref()?;
                mixed[k] = target.clone();
                let next_ty = subst_tel(&a_ty_tpl, &mixed);
                if convert_type(env, ctx, &current_ty, &next_ty) {
                    current_ty = next_ty;
                    continue;
                }
                let domain = crate::check::infer(env, ctx, &ctor_arg_vals[k]).ok()?;
                let index_evidence = telescope_projection(e, index, a_args.len());
                let indexed_eq = Term::Eq(
                    Box::new(domain.clone()),
                    Box::new(ctor_arg_vals[k].clone()),
                    Box::new(target.clone()),
                );
                let family_args = (0..j)
                    .map(|i| {
                        if i == k {
                            Term::var(1)
                        } else {
                            weaken(&mixed[i], 2)
                        }
                    })
                    .collect::<Vec<_>>();
                // Γ gains only the J motive's y and proof binders. The j
                // constructor binders are removed by subst_tel below; unlike
                // inductive_conjuncts, no j equality binders remain in Γ.
                let family_at_y = subst_tel(&shift(&a_ty_tpl, 2, j), &family_args);
                witness = type_eq_by_j_with_base(
                    env,
                    ctx,
                    &domain,
                    &ctor_arg_vals[k],
                    &a_ty_j,
                    &next_ty,
                    family_at_y,
                    Term::Ascript(Box::new(index_evidence), Box::new(indexed_eq)),
                    witness,
                    false,
                )?;
                current_ty = next_ty;
            }
            let expected = Term::Eq(
                Box::new(Term::Type(level)),
                Box::new(a_ty_j.clone()),
                Box::new(b_ty_j.clone()),
            );
            crate::check::check(env, ctx, &witness, &expected).ok()?;
            let cast_val = Term::Cast(
                Box::new(a_ty_j.clone()),
                Box::new(b_ty_j),
                Box::new(witness),
                Box::new(ctor_arg_vals[j].clone()),
            );
            (cast_val.clone(), cast_val)
        };
        new_args.push(new_val);
        target_earlier.push(target_val);
    }
    // One index-path gate covers both the shape of the Eq Type telescope
    // projected above and every rebuilt target index (including unchanged
    // template positions). Neither condition alone establishes the other.
    if !inductive_eq_has_telescope(env, ctx, a, b, a_args.len())
        || !constructor_indices_match_target(
            env,
            ctx,
            ind,
            c,
            &level_args,
            b_param_args,
            &target_earlier,
            j_bar,
        )
    {
        return None;
    }
    Some(apply_args(
        Term::Constructor {
            id: ctor,
            level_args,
        },
        &new_args,
    ))
}

/// `cast (A1/R) (A2/S) e [a] ⇝ [cast A1 A2 e0 a]` where `e0 = e.1` is the
/// underlying type equality from the decomposition of `e` (`16 §3.2`). The class
/// structure is preserved; a non-class representative leaves the cast neutral.
fn cast_at_quot(a: &Term, b: &Term, e: &Term, t: &Term) -> Option<Term> {
    let e0 = Term::proj1(e.clone());
    let a_inner = match a {
        Term::Quot(x, _, _) => (**x).clone(),
        _ => return None,
    };
    let b_inner = match b {
        Term::Quot(y, _, _) => (**y).clone(),
        _ => return None,
    };
    match t {
        Term::QuotClass(a0) => Some(Term::QuotClass(Box::new(Term::Cast(
            Box::new(a_inner),
            Box::new(b_inner),
            Box::new(e0),
            Box::new((**a0).clone()),
        )))),
        _ => None,
    }
}

// ===========================================================================
// derived J (`15 §4`)
// ===========================================================================

/// Reduce `J motive base eq` (`15 §4`). `J-β`: when `eq` whnf's to `refl a`,
/// reduce to `base`; every other well-typed equality reduces via the `cast`
/// construction (`15 §4.3`). `infer_j` guarantees that a well-typed `eq` has an
/// `Eq` type, so `None` is possible only for ill-typed raw input and leaves that
/// input neutral for fail-closed rejection.
pub fn j_reduce(
    env: &GlobalEnv,
    ctx: &Context,
    motive: &Term,
    base: &Term,
    eq: &Term,
) -> Option<Term> {
    let eq_w = whnf(env, ctx, eq);
    if let Term::Refl(_a) = &eq_w {
        // J-β (`15 §4.2`): J A a P d a (refl a) ≡ d.
        return Some(base.clone());
    }
    // Preserve the checked Eq ascription: whnf may erase its formation.
    j_nonrefl(env, ctx, motive, base, eq)
}

/// `J` on a non-`refl` equality (`15 §4.3`): `J ≡ cast (P a (refl a)) (P b e)
/// pair-eq d`. Fires for every non-`refl` `e` — motive constancy is not a gate
/// (`§4.1`). pair-eq is a typing witness only, never inspected by cast (`§3.4`).
/// For a constant motive cast reduces by regularity; for a dependent motive cast
/// descends by type structure (`§3.2`).
fn j_nonrefl(
    env: &GlobalEnv,
    ctx: &Context,
    motive: &Term,
    base: &Term,
    eq: &Term,
) -> Option<Term> {
    #[cfg(test)]
    j_nonrefl_probe::bump();
    // At a universe carrier, prefer the Eq formation inferred for this
    // evidence: WHNF of that Eq type would reduce the source endpoint here, in
    // the guard and again in the Cast.
    let eq_ty = crate::check::infer(env, ctx, eq).ok()?;
    let formation = match crate::check::eq_formation_head(env, &eq_ty) {
        Term::Eq(carrier, x, y) => match whnf(env, ctx, &carrier) {
            Term::Type(level) => Some((Term::Type(level), *x, *y)),
            _ => None,
        },
        _ => None,
    };
    // Establish the raw redex's J typing once, at exactly the endpoints used.
    // The formation read is used only where the guard types the motive at it.
    // Eq-at-Type CAN return an Eq formation: a one-parameter or one-index
    // former's single conjunct is returned bare, so whnf(Eq Type (B X) (B Y))
    // is Eq Type X Y and infer_j types this J at X, Y. Fall back to exactly
    // those whnf endpoints, so every J that infer types reduces as before.
    let endpoints = match formation {
        Some(endpoints)
            if crate::check::infer_j_at(env, ctx, motive, base, eq, endpoints.clone()).is_ok() =>
        {
            endpoints
        }
        _ => {
            let endpoints = crate::check::j_endpoints(env, ctx, eq).ok()?;
            crate::check::infer_j_at(env, ctx, motive, base, eq, endpoints.clone()).ok()?;
            endpoints
        }
    };
    let (a_type, a_idx, b_idx) = endpoints;
    let p_a_refl = apply_args(
        motive.clone(),
        &[a_idx.clone(), Term::Refl(Box::new(a_idx.clone()))],
    );
    let p_b_e = apply_args(motive.clone(), &[b_idx.clone(), eq.clone()]);
    // The singleton's equality e transports the motive's output type from
    // (a, refl a) to (b, e); the cast ignores the proof after typing it.
    let motive_at_y = apply_args(weaken(motive, 2), &[Term::var(1), Term::var(0)]);
    let pair_eq = type_eq_by_j(
        env,
        ctx,
        &a_type,
        &a_idx,
        &p_a_refl,
        &p_b_e,
        motive_at_y,
        Term::Ascript(
            Box::new(eq.clone()),
            Box::new(Term::Eq(
                Box::new(a_type.clone()),
                Box::new(a_idx.clone()),
                Box::new(b_idx),
            )),
        ),
        true,
    )?;
    Some(Term::Cast(
        Box::new(p_a_refl),
        Box::new(p_b_e),
        Box::new(pair_eq),
        Box::new(base.clone()),
    ))
}

#[cfg(test)]
mod j_nonrefl_probe {
    use std::cell::Cell;

    thread_local! {
        static CALLS: Cell<u64> = const { Cell::new(0) };
    }

    pub(super) fn bump() {
        CALLS.with(|c| c.set(c.get() + 1));
    }
    pub(super) fn reset() {
        CALLS.with(|c| c.set(0));
    }
    pub(super) fn calls() -> u64 {
        CALLS.with(Cell::get)
    }
}

#[cfg(test)]
pub(crate) fn reset_j_nonrefl_calls() {
    j_nonrefl_probe::reset();
}

#[cfg(test)]
pub(crate) fn j_nonrefl_calls() -> u64 {
    j_nonrefl_probe::calls()
}

#[cfg(test)]
mod witness_base_tests {
    use super::*;
    use crate::term::Level;

    /// Guard-local control: a raw neutral proposition supplies no witness.
    /// P0 keeps well-typed `Eq Type X X` neutral or Top, so this tests the
    /// fallback branch directly, not its reach from a checked reduct.
    #[test]
    fn neutral_base_does_not_fabricate_a_type_equality_witness() {
        let env = GlobalEnv::new();
        let mut ctx = Context::new();
        ctx.push(Term::Omega(Level::zero()));
        assert_eq!(canonical_type_eq_base(&env, &ctx, &Term::var(0)), None);
        assert_eq!(
            canonical_type_eq_base(&env, &ctx, &top_term(&env)),
            Some(tt_term(&env))
        );
        let ty = Term::pi(Term::Type(Level::zero()), Term::Type(Level::zero()));
        let base = Term::Eq(
            Box::new(Term::Type(Level::zero().suc())),
            Box::new(ty.clone()),
            Box::new(ty.clone()),
        );
        assert_eq!(
            canonical_type_eq_base(&env, &ctx, &base),
            Some(Term::Refl(Box::new(ty)))
        );
    }
}
