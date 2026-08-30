//! K1 conversion — weak-head reduction, normalization, and definitional
//! equality (`13-pi-sigma.md §6`).
//!
//! K1 builds only the conversion its own rules require: α (de Bruijn syntactic
//! identity), β/Σ-β/ι/δ reduction, and **type-directed η** for Π and Σ. The
//! full decidable conversion (lazy-WHNF NbE, `Eq`/`cast` equations, Ω proof
//! irrelevance, SCT-gated δ) is **K2c** (`17`). [`convert`] is the standalone
//! entry point the rest of K1 calls and that K2c replaces, body-only, without
//! changing the signature (`13 §6.3`).
//!
//! Termination on the K1 fragment (`14 §9.2`): β strictly decreases size; η
//! descends on the (finite) type; ι descends on structurally smaller
//! scrutinees; δ is **cyclic** post-K2c (recursive transparent defs are the
//! cycles) — its termination is **not** structural here but guaranteed by the
//! SCT gate at admission time (`sct_check`, `17 §4`): every transparent def's
//! δ-unfolding terminates because `whnf`'s δ step only ever unfolds a
//! definition the gate has already certified.

use crate::env::{Context, GlobalEnv};
use crate::inductive::{iota_reduct, peel_app};
use crate::subst::{subst0, subst_levels, weaken};
use crate::term::{GlobalId, Level, Term};

/// Decidable level equality (`12 §1`, §6.1) — the semilattice normal form.
pub fn level_eq(a: &Level, b: &Level) -> bool {
    a.equiv(b)
}

/// Equality of level-argument lists (polymorphic uses agree on instantiation).
fn level_args_eq(a: &[Level], b: &[Level]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| level_eq(x, y))
}

/// Unfold a transparent constant `c` to its body with `level_args`
/// instantiated (δ-reduction, `11 §4`). Returns `None` if `c` is not
/// transparent (opaque/primitive/inductive — no δ).
fn unfold_const(env: &GlobalEnv, id: crate::term::GlobalId, level_args: &[Level]) -> Option<Term> {
    let (params, body) = env.transparent_body(id)?;
    // The sole δ-expansion site: every reached δ unfold is observed here (a
    // causal count, not the distinct-head-capture proxy). Test-only.
    probe_unfold();
    Some(subst_levels(&body, &params, level_args))
}

/// Whether a weak-head reduction made **ι-progress**: a successfully taken
/// [`iota_reduct`] somewhere in the reduction (directly at the head, or in a
/// nested head reduction of a sub-term). This is the ONLY event the recursive-
/// head totality guard ([`conv_struct_path`]) counts as progress — never β, δ,
/// `let`, ascription removal, stuck-`Elim` rebuilding, an observational
/// (`Eq`/`cast`/`J`/`QuotElim`) reduction, or merely peeling to a constructor.
#[derive(Clone, Copy, Default)]
struct WhnfProgress {
    iota: bool,
}

/// Weak-head normal form: reduce head redexes (β, δ, Σ-β, ι, let, ascription)
/// until the head is not a redex. Infallible — an ι arity mismatch leaves the
/// eliminator stuck (neutral), which is sound (`14 §7.6`).
///
/// This is the eager public entry the rest of the kernel calls. It is a thin
/// wrapper over [`whnf_progress`] that discards the ι-progress flag, so its
/// result is byte-for-byte identical to the historical `whnf`.
pub fn whnf(env: &GlobalEnv, ctx: &Context, t: &Term) -> Term {
    whnf_progress(env, ctx, t).0
}

/// The sole weak-head reducer, additionally reporting [`WhnfProgress`]. The
/// reduced `Term` is exactly what the historical `whnf` produced; the only
/// addition is the `iota` flag — set iff an `iota_reduct` was successfully
/// taken during this reduction (at the head or in any nested head reduction).
/// `whnf` drops the flag, so every existing caller is unaffected.
///
/// `ctx` is threaded for the K2c NbE replacement (which evaluates against a
/// context); K1's head reduction does not consult it, hence the allow.
#[allow(clippy::only_used_in_recursion)]
fn whnf_progress(env: &GlobalEnv, ctx: &Context, t: &Term) -> (Term, WhnfProgress) {
    let mut cur = t.clone();
    let mut iota = false;
    loop {
        match &cur {
            Term::App(f, a) => {
                let (f_w, fp) = whnf_progress(env, ctx, f);
                iota |= fp.iota;
                match &f_w {
                    Term::Lam(_, body) => {
                        cur = subst0(body, a);
                        continue;
                    }
                    Term::Const { id, level_args } if env.transparent_body(*id).is_some() => {
                        if let Some(body) = unfold_const(env, *id, level_args) {
                            cur = Term::app(body, (**a).clone());
                            continue;
                        }
                        return (Term::app(f_w, (**a).clone()), WhnfProgress { iota });
                    }
                    // stuck neutral application
                    _ => return (Term::app(f_w, (**a).clone()), WhnfProgress { iota }),
                }
            }
            Term::Proj1(p) => {
                let (p_w, pp) = whnf_progress(env, ctx, p);
                iota |= pp.iota;
                match &p_w {
                    Term::Pair(a, _) => {
                        cur = (**a).clone();
                        continue;
                    }
                    Term::Const { id, level_args } if env.transparent_body(*id).is_some() => {
                        if let Some(body) = unfold_const(env, *id, level_args) {
                            cur = Term::proj1(body);
                            continue;
                        }
                        return (Term::proj1(p_w), WhnfProgress { iota });
                    }
                    _ => return (Term::proj1(p_w), WhnfProgress { iota }),
                }
            }
            Term::Proj2(p) => {
                let (p_w, pp) = whnf_progress(env, ctx, p);
                iota |= pp.iota;
                match &p_w {
                    Term::Pair(_, b) => {
                        cur = (**b).clone();
                        continue;
                    }
                    Term::Const { id, level_args } if env.transparent_body(*id).is_some() => {
                        if let Some(body) = unfold_const(env, *id, level_args) {
                            cur = Term::proj2(body);
                            continue;
                        }
                        return (Term::proj2(p_w), WhnfProgress { iota });
                    }
                    _ => return (Term::proj2(p_w), WhnfProgress { iota }),
                }
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
                let (s_w, sp) = whnf_progress(env, ctx, scrut);
                iota |= sp.iota;
                let (head, all_args) = peel_app(&s_w);
                if let Term::Constructor { id, .. } = head {
                    if let Some((ind, k)) = env.constructor(id) {
                        if ind.id == *fam {
                            if let Ok(reduct) =
                                iota_reduct(env, ind, k, level_args, params, motive, methods, &all_args)
                            {
                                // The sole ι-progress site (`14 §7.2`).
                                iota = true;
                                probe_iota();
                                cur = reduct;
                                continue;
                            }
                        }
                    }
                }
                // Stuck eliminator (neutral): rebuild with the whnf'd scrutinee
                // (`14 §7.6`). Indices don't gate ι firing (`14 §7.2`).
                return (
                    Term::Elim {
                        fam: *fam,
                        level_args: level_args.clone(),
                        params: params.clone(),
                        motive: motive.clone(),
                        methods: methods.clone(),
                        indices: indices.clone(),
                        scrut: Box::new(s_w),
                    },
                    WhnfProgress { iota },
                );
            }
            Term::Const { id, level_args } if env.transparent_body(*id).is_some() => {
                if let Some(body) = unfold_const(env, *id, level_args) {
                    cur = body;
                    continue;
                }
                return (cur, WhnfProgress { iota });
            }
            Term::Let { body, val, .. } => {
                cur = subst0(body, val);
                continue;
            }
            Term::Ascript(t, _) => {
                cur = (**t).clone();
                continue;
            }
            // --- K2 observational reductions (`16 §8.1`) ---
            Term::Eq(ty, x, y) => {
                // `Eq A a b` reduces by recursion on `whnf(A)` (`15 §2`, `16
                // §2.2`); a neutral `A` leaves it a neutral proposition.
                let (ty_w, tp) = whnf_progress(env, ctx, ty);
                iota |= tp.iota;
                if let Some(r) = crate::obs::eq_reduce(env, ctx, &ty_w, x, y) {
                    cur = r;
                    continue;
                }
                return (
                    Term::Eq(Box::new(ty_w), (*x).clone(), (*y).clone()),
                    WhnfProgress { iota },
                );
            }
            Term::Cast(a, b, e, t) => {
                // `cast A B e t` reduces by recursion on `whnf(A)`,`whnf(B)`
                // (`16 §3.2`); mismatched/neutral heads or a neutral proof leave
                // it a neutral cast.
                let (a_w, ap) = whnf_progress(env, ctx, a);
                iota |= ap.iota;
                let (b_w, bp) = whnf_progress(env, ctx, b);
                iota |= bp.iota;
                if let Some(r) = crate::obs::cast_reduce(env, ctx, &a_w, &b_w, e, t) {
                    cur = r;
                    continue;
                }
                return (
                    Term::Cast(Box::new(a_w), Box::new(b_w), (*e).clone(), (*t).clone()),
                    WhnfProgress { iota },
                );
            }
            Term::J(motive, base, eq) => {
                // Derived `J` (`15 §4`): `J-β` on `refl`, and reduction on
                // non-`refl` via `cast`. A neutral `eq` (or non-constant motive)
                // leaves `J` neutral.
                if let Some(r) = crate::obs::j_reduce(env, ctx, motive, base, eq) {
                    cur = r;
                    continue;
                }
                return (
                    Term::J((*motive).clone(), (*base).clone(), (*eq).clone()),
                    WhnfProgress { iota },
                );
            }
            Term::QuotElim {
                motive,
                method,
                respect,
                scrut,
            } => {
                // Quotient/truncation i-reduction: `elim_/ M f r [a] ⇝ f a`
                // (`16 §5`); `elim_trunc P f |a| ⇝ f a` (truncation elim encoded
                // as `QuotElim` on a `TruncProj` scrut, `16 §6`). A neutral
                // scrutinee leaves the eliminator neutral.
                let (s_w, sp) = whnf_progress(env, ctx, scrut);
                iota |= sp.iota;
                match &s_w {
                    Term::QuotClass(a0) => {
                        cur = Term::app((**method).clone(), (**a0).clone());
                        continue;
                    }
                    Term::TruncProj(a0) => {
                        cur = Term::app((**method).clone(), (**a0).clone());
                        continue;
                    }
                    _ => {}
                }
                return (
                    Term::QuotElim {
                        motive: (*motive).clone(),
                        method: (*method).clone(),
                        respect: (*respect).clone(),
                        scrut: Box::new(s_w),
                    },
                    WhnfProgress { iota },
                );
            }
            // already in weak-head normal form
            _ => return (cur, WhnfProgress { iota }),
        }
    }
}

/// Full normal form: whnf, then normalize the sub-terms (recursing under
/// binders). Used by the API surface and by tests; K1 conversion uses
/// [`convert`] (whnf + type-directed η), but `normalize` realises the
/// "reduce to normal form" half of `13 §6.2` for inspection.
pub fn normalize(env: &GlobalEnv, ctx: &Context, t: &Term) -> Term {
    let h = whnf(env, ctx, t);
    match &h {
        Term::Pi(a, b) => {
            let a_n = normalize(env, ctx, a);
            let mut ctx2 = ctx.clone();
            ctx2.push((**a).clone());
            Term::pi(a_n, normalize(env, &ctx2, b))
        }
        Term::Lam(a, body) => {
            let a_n = normalize(env, ctx, a);
            let mut ctx2 = ctx.clone();
            ctx2.push((**a).clone());
            Term::lam(a_n, normalize(env, &ctx2, body))
        }
        Term::Sigma(a, b) => {
            let a_n = normalize(env, ctx, a);
            let mut ctx2 = ctx.clone();
            ctx2.push((**a).clone());
            Term::sigma(a_n, normalize(env, &ctx2, b))
        }
        Term::Pair(a, b) => Term::pair(normalize(env, ctx, a), normalize(env, ctx, b)),
        Term::App(f, a) => Term::app(normalize(env, ctx, f), normalize(env, ctx, a)),
        Term::Proj1(p) => Term::proj1(normalize(env, ctx, p)),
        Term::Proj2(p) => Term::proj2(normalize(env, ctx, p)),
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => Term::Elim {
            fam: *fam,
            level_args: level_args.clone(),
            params: params.iter().map(|p| normalize(env, ctx, p)).collect(),
            motive: Box::new(normalize(env, ctx, motive)),
            methods: methods.iter().map(|m| normalize(env, ctx, m)).collect(),
            indices: indices.iter().map(|i| normalize(env, ctx, i)).collect(),
            scrut: Box::new(normalize(env, ctx, scrut)),
        },
        Term::Eq(a, t, u) => Term::Eq(
            Box::new(normalize(env, ctx, a)),
            Box::new(normalize(env, ctx, t)),
            Box::new(normalize(env, ctx, u)),
        ),
        Term::Cast(a, b, e, t) => Term::Cast(
            Box::new(normalize(env, ctx, a)),
            Box::new(normalize(env, ctx, b)),
            Box::new(normalize(env, ctx, e)),
            Box::new(normalize(env, ctx, t)),
        ),
        Term::J(m, d, e) => Term::J(
            Box::new(normalize(env, ctx, m)),
            Box::new(normalize(env, ctx, d)),
            Box::new(normalize(env, ctx, e)),
        ),
        Term::Quot(a, r) => Term::Quot(
            Box::new(normalize(env, ctx, a)),
            Box::new(normalize(env, ctx, r)),
        ),
        Term::QuotClass(t) => Term::QuotClass(Box::new(normalize(env, ctx, t))),
        Term::Trunc(a) => Term::Trunc(Box::new(normalize(env, ctx, a))),
        Term::TruncProj(t) => Term::TruncProj(Box::new(normalize(env, ctx, t))),
        Term::Refl(t) => Term::Refl(Box::new(normalize(env, ctx, t))),
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => Term::QuotElim {
            motive: Box::new(normalize(env, ctx, motive)),
            method: Box::new(normalize(env, ctx, method)),
            respect: Box::new(normalize(env, ctx, respect)),
            scrut: Box::new(normalize(env, ctx, scrut)),
        },
        Term::Let { ty: _, val, body } => {
            // let reduces to body[val/x] before normalizing (it is a redex).
            normalize(env, ctx, &subst0(body, val))
        }
        Term::Ascript(t, _) => normalize(env, ctx, t),
        Term::Absurd(motive, proof) => Term::Absurd(
            Box::new(normalize(env, ctx, motive)),
            Box::new(normalize(env, ctx, proof)),
        ),
        // Leaves and closed-ish nodes: no sub-terms to normalize (levels aside).
        Term::Type(_)
        | Term::Omega(_)
        | Term::Var(_)
        | Term::Const { .. }
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => h,
    }
}

/// Is `ty` a proposition — `Γ ⊢ ty : Ω_ℓ` for some `ℓ` (`16 §1.1`)? This is the
/// guard for the Ω proof-irrelevance shortcut (`16 §8.2`): any two terms at a
/// proposition type are definitionally equal. Infallible — an ill-typed `ty` is
/// treated as "not a proposition" (conversion never crashes).
fn is_omega_type(env: &GlobalEnv, ctx: &Context, ty: &Term) -> bool {
    crate::check::infer(env, ctx, ty)
        .map(|t| matches!(whnf(env, ctx, &t), Term::Omega(_)))
        .unwrap_or(false)
}

// ===== Path-local no-progress δ-origin ledger (recursive-head totality) =====
//
// `convert`/`convert_type` on two DISTINCT transparent recursive definitions
// can δ-unfold forever: each side unfolds to a body still headed by its own
// recursive `Const`, and when the recursion's eliminator sits on a neutral
// scrutinee no ι ever fires to bottom the unfolding out. The SCT gate certifies
// each definition's OWN δ-termination, but not the lock-step structural
// comparison of two distinct ones — that is the gap this ledger closes
// (`d0_distinct_recursive_map_*`).
//
// The guard, applied at each structural conversion edge: BEFORE whnf, note the
// canonical unordered pair of the two sides' DISTINCT transparent-`Const`
// application heads, and thread that observation down the private recursion. If
// a side's whnf makes real ι-progress the pair is discharged for descendants
// (genuine reduction — SCT bounds it); otherwise the first sighting of a pair
// is recorded and one structural lap is allowed, and a SECOND sighting of the
// SAME pair with still no ι-progress returns `false` — the two heads are
// looping without converging, so they are not definitionally equal. Refusing is
// fail-closed: it can only under-accept (a completeness boundary), never admit
// a false equality, so it cannot weaken the trust root. Public signatures are
// unchanged and start from an empty ledger.
//
// Governing spec: `17 §3.5` "Distinct recursive-identity boundary" (a `GlobalId`
// is a declaration's identity; conversion MUST halt with `false` where two
// distinct self `GlobalId`s meet beneath a stuck eliminator, and MUST NOT keep
// unfolding to recreate the comparison on a fresh neutral argument) and `17 §5`
// obligation 3 (cross-identity symbolic retry is NOT an SCT-certified call
// sequence — SCT (§4) bounds re-entry within ONE admitted group, not the
// lock-step comparison of two). This mechanism realises that boundary. The
// black-box conformance twin is
// `conformance/kernel/conversion/seed-conversion.md`,
// `delta-distinct-recursive-heads-stuck` (promise class: durable invariant).

/// A canonical unordered pair of two DISTINCT transparent-`Const` GlobalIds —
/// the δ-origin of a structural conversion edge.
type ConstPair = (GlobalId, GlobalId);

/// Canonicalise so `(x, y)` and `(y, x)` denote the same δ-origin.
fn canonical_pair(x: GlobalId, y: GlobalId) -> ConstPair {
    if x <= y {
        (x, y)
    } else {
        (y, x)
    }
}

/// The pre-whnf δ-origin of a structural edge: `Some((min, max))` iff both
/// sides are (possibly nullary) applications whose heads are DISTINCT
/// transparent `Const`s. Same-`Const` heads are closed by the spine fast path
/// and are never a divergence origin, so they are excluded here; an opaque /
/// primitive / inductive head (no δ) cannot drive the unfold loop and is
/// likewise excluded.
fn delta_origin_pair(env: &GlobalEnv, a: &Term, b: &Term) -> Option<ConstPair> {
    let (ha, _) = peel_app(a);
    let (hb, _) = peel_app(b);
    match (&ha, &hb) {
        (Term::Const { id: ia, .. }, Term::Const { id: ib, .. })
            if ia != ib
                && env.transparent_body(*ia).is_some()
                && env.transparent_body(*ib).is_some() =>
        {
            Some(canonical_pair(*ia, *ib))
        }
        _ => None,
    }
}

/// Test-only observation of the δ-ledger events — a reached δ unfold (the
/// causal count, at the `unfold_const` edge), pre-whnf distinct-head capture, a
/// successful ι-reduction, and a refusal. Compiled to nothing outside
/// `cfg(test)`, so the production conversion path carries zero instrumentation.
#[cfg(test)]
mod delta_probe {
    use std::cell::Cell;
    thread_local! {
        static UNFOLDS: Cell<u64> = const { Cell::new(0) };
        static CAPTURES: Cell<u64> = const { Cell::new(0) };
        static IOTAS: Cell<u64> = const { Cell::new(0) };
        static REFUSALS: Cell<u64> = const { Cell::new(0) };
    }
    pub(super) fn reset() {
        UNFOLDS.with(|c| c.set(0));
        CAPTURES.with(|c| c.set(0));
        IOTAS.with(|c| c.set(0));
        REFUSALS.with(|c| c.set(0));
    }
    pub(super) fn bump_unfold() {
        UNFOLDS.with(|c| c.set(c.get() + 1));
    }
    pub(super) fn bump_capture() {
        CAPTURES.with(|c| c.set(c.get() + 1));
    }
    pub(super) fn bump_iota() {
        IOTAS.with(|c| c.set(c.get() + 1));
    }
    pub(super) fn bump_refusal() {
        REFUSALS.with(|c| c.set(c.get() + 1));
    }
    pub(super) fn unfolds() -> u64 {
        UNFOLDS.with(|c| c.get())
    }
    pub(super) fn captures() -> u64 {
        CAPTURES.with(|c| c.get())
    }
    pub(super) fn iotas() -> u64 {
        IOTAS.with(|c| c.get())
    }
    pub(super) fn refusals() -> u64 {
        REFUSALS.with(|c| c.get())
    }
}

#[cfg(test)]
#[inline]
fn probe_unfold() {
    delta_probe::bump_unfold();
}
#[cfg(not(test))]
#[inline(always)]
fn probe_unfold() {}

#[cfg(test)]
#[inline]
fn probe_capture() {
    delta_probe::bump_capture();
}
#[cfg(not(test))]
#[inline(always)]
fn probe_capture() {}

#[cfg(test)]
#[inline]
fn probe_iota() {
    delta_probe::bump_iota();
}
#[cfg(not(test))]
#[inline(always)]
fn probe_iota() {}

#[cfg(test)]
#[inline]
fn probe_refusal() {
    delta_probe::bump_refusal();
}
#[cfg(not(test))]
#[inline(always)]
fn probe_refusal() {}

/// Definitional equality `Γ ⊢ a ≡ b : A` for the K1 fragment (`13 §6.2`):
/// α (de Bruijn syntactic identity), then type-directed η (Π-η, Σ-η) when the
/// type is a Π/Σ, else structural congruence with whnf. This is the **K2c
/// extension seam** — K2c replaces this body with lazy-WHNF NbE without
/// changing the signature (`13 §6.3`). K2 adds the Ω-PI shortcut (`16 §8.2`).
///
/// The path-local no-progress δ-origin ledger (empty here at the public entry)
/// is threaded through the private recursion to keep conversion total on
/// distinct recursive transparent heads; see [`conv_struct_path`].
pub fn convert(env: &GlobalEnv, ctx: &Context, ty: &Term, a: &Term, b: &Term) -> bool {
    convert_path(env, ctx, ty, a, b, &[])
}

/// [`convert`] with the δ-origin ledger threaded in. Only the private recursion
/// carries `path`; the public entry starts it empty.
fn convert_path(
    env: &GlobalEnv,
    ctx: &Context,
    ty: &Term,
    a: &Term,
    b: &Term,
    path: &[ConstPair],
) -> bool {
    if a == b {
        return true; // α: syntactic identity under de Bruijn (`13 §6.2` step 1)
    }
    // Ω proof-irrelevance shortcut (`16 §8.2`): if `ty : Ω`, any two terms are
    // definitionally equal — a constant-time "yes" without inspecting contents.
    // This is what makes `Eq : Ω` (and the whole logic) proof-irrelevant, and
    // lets conversion skip propositional arguments.
    if is_omega_type(env, ctx, ty) {
        return true;
    }
    let ty_w = whnf(env, ctx, ty);
    match &ty_w {
        Term::Pi(dom, cod) => {
            // Π-η (`13 §6.2` step 3): compare `f x` and `g x` at the codomain,
            // for a fresh `x : dom` (`f ≡ λx. f x`). The ledger threads through
            // η so a pair seen above η is still remembered below it.
            let a_w = whnf(env, ctx, a);
            let b_w = whnf(env, ctx, b);
            let a_ext = weaken(&a_w, 1);
            let b_ext = weaken(&b_w, 1);
            let lhs = Term::app(a_ext, Term::var(0));
            let rhs = Term::app(b_ext, Term::var(0));
            let mut ctx2 = ctx.clone();
            ctx2.push((**dom).clone());
            convert_path(env, &ctx2, cod, &lhs, &rhs, path)
        }
        Term::Sigma(dom, cod) => {
            // Σ-η (`13 §6.2` step 3): compare both projections.
            let a_w = whnf(env, ctx, a);
            let b_w = whnf(env, ctx, b);
            let a1 = whnf(env, ctx, &Term::proj1(a_w.clone()));
            let b1 = whnf(env, ctx, &Term::proj1(b_w.clone()));
            if !convert_path(env, ctx, dom, &a1, &b1, path) {
                return false;
            }
            let cod_a1 = subst0(cod, &a1); // B[a1/x]
            let a2 = whnf(env, ctx, &Term::proj2(a_w.clone()));
            let b2 = whnf(env, ctx, &Term::proj2(b_w.clone()));
            convert_path(env, ctx, &cod_a1, &a2, &b2, path)
        }
        _ => {
            // (4) Unit-η / single-constructor-no-field inductive (`17 §2`):
            // any two values of a no-field single-constructor type are equal.
            let (ty_head, _ty_args) = crate::inductive::peel_app(&ty_w);
            if let Term::IndFormer { id, .. } = &ty_head {
                if let Some(ind) = env.inductive(*id) {
                    if ind.constructors.len() == 1 && ind.constructors[0].args.is_empty() {
                        return true;
                    }
                }
            }
            conv_struct_path(env, ctx, a, b, path)
        }
    }
}

/// Definitional equality of two **types** `Γ ⊢ A ≡ B type` (`13 §6.2` for
/// type expressions). Types do not take η (η is for values at Π/Σ types), so
/// this is whnf + structural congruence. Used for domain matching, ascription,
/// and the mode-switch `A ≡ A'` between the expected and inferred types.
pub fn convert_type(env: &GlobalEnv, ctx: &Context, a: &Term, b: &Term) -> bool {
    conv_struct_path(env, ctx, a, b, &[])
}

/// Structural congruence (no type-directed η): whnf both sides, then compare
/// structurally, recursing. Used when the type is not Π/Σ (`13 §6.2` step 4
/// and the congruence closure).
///
/// `path` is the path-local no-progress δ-origin ledger. At this edge we
/// capture the pre-whnf δ-origin pair (distinct transparent-`Const` heads),
/// whnf both sides tracking ι-progress, and either discharge the pair (real
/// ι-progress), record a first sighting (one lap allowed), or refuse a
/// recurring no-progress pair — before recursing structurally with the ledger
/// the descendants inherit.
fn conv_struct_path(env: &GlobalEnv, ctx: &Context, a: &Term, b: &Term, path: &[ConstPair]) -> bool {
    // Syntactic-identity fast path (pre-δ, `13 §6.2` step 1): identical
    // de Bruijn terms are convertible with no reduction and no ledger touch.
    if a == b {
        return true;
    }

    // Congruence-first / lazy-δ fast path (`obs-eq-termination`): `whnf`
    // below unconditionally δ-unfolds a transparent head `Const` before any
    // congruence dispatch runs. When both sides are ALREADY (pre-whnf) an
    // application of the SAME constant to the SAME number of arguments,
    // try congruence on the argument spine FIRST, without ever unfolding
    // the head — application congruence for a deterministic function is
    // always sound regardless of whether its body would normalize, and
    // this avoids manufacturing an ever-deeper unfolded form when the
    // constant is itself recursive and its scrutinee is neutral (so ι never
    // fires to bottom out the δ-unfold). Falls through to the existing
    // whnf-based path, completely unchanged, whenever this doesn't apply or
    // any argument fails to convert (fallback preserves completeness: a
    // constant that ignores an argument, or two constants that only agree
    // after unfolding, still get the full treatment below). Same-`Const`
    // heads are never a divergence origin, so this path does not capture.
    if let (Term::Const { id: id1, level_args: la1 }, args1) = peel_app(a) {
        if let (Term::Const { id: id2, level_args: la2 }, args2) = peel_app(b) {
            if id1 == id2
                && level_args_eq(&la1, &la2)
                && args1.len() == args2.len()
                && args1
                    .iter()
                    .zip(args2.iter())
                    .all(|(x, y)| conv_struct_path(env, ctx, x, y, path))
            {
                return true;
            }
        }
    }

    // δ-origin capture (BEFORE whnf): the canonical pair of DISTINCT
    // transparent-`Const` application heads, if the edge has one. This is the
    // pre-whnf observation the no-progress guard is keyed on.
    let origin = delta_origin_pair(env, a, b);
    if origin.is_some() {
        probe_capture();
    }

    let (a, ap) = whnf_progress(env, ctx, a);
    let (b, bp) = whnf_progress(env, ctx, b);
    let iota_progress = ap.iota || bp.iota;

    if a == b {
        return true;
    }

    // The ledger the structural descendants inherit:
    //   - real ι-progress on either side DISCHARGES this pair (genuine
    //     reduction, whose depth SCT already bounds);
    //   - otherwise a FIRST no-progress sighting is recorded and one structural
    //     lap is allowed;
    //   - a RECURRING no-progress sighting REFUSES before another structural
    //     copy — the heads loop without converging, so they are not
    //     definitionally equal (fail-closed: sound, at worst under-accepting).
    let child_storage: Vec<ConstPair>;
    let child_path: &[ConstPair] = match origin {
        Some(p) if iota_progress => {
            child_storage = path.iter().copied().filter(|q| *q != p).collect();
            &child_storage
        }
        Some(p) if path.contains(&p) => {
            probe_refusal();
            return false;
        }
        Some(p) => {
            child_storage = {
                let mut v = path.to_vec();
                v.push(p);
                v
            };
            &child_storage
        }
        None => path,
    };

    match (&a, &b) {
        (Term::Type(l1), Term::Type(l2)) => level_eq(l1, l2),
        (Term::Var(i), Term::Var(j)) => i == j,
        (
            Term::Const {
                id: id1,
                level_args: la1,
            },
            Term::Const {
                id: id2,
                level_args: la2,
            },
        ) => id1 == id2 && level_args_eq(la1, la2),
        (
            Term::IndFormer {
                id: id1,
                level_args: la1,
            },
            Term::IndFormer {
                id: id2,
                level_args: la2,
            },
        ) => id1 == id2 && level_args_eq(la1, la2),
        (
            Term::Constructor {
                id: id1,
                level_args: la1,
            },
            Term::Constructor {
                id: id2,
                level_args: la2,
            },
        ) => id1 == id2 && level_args_eq(la1, la2),
        (Term::Pi(a1, b1), Term::Pi(a2, b2)) => {
            conv_struct_path(env, ctx, a1, a2, child_path) && {
                let mut c = ctx.clone();
                c.push((**a1).clone());
                conv_struct_path(env, &c, b1, b2, child_path)
            }
        }
        (Term::Lam(a1, t1), Term::Lam(a2, t2)) => {
            conv_struct_path(env, ctx, a1, a2, child_path) && {
                let mut c = ctx.clone();
                c.push((**a1).clone());
                conv_struct_path(env, &c, t1, t2, child_path)
            }
        }
        (Term::Sigma(a1, b1), Term::Sigma(a2, b2)) => {
            conv_struct_path(env, ctx, a1, a2, child_path) && {
                let mut c = ctx.clone();
                c.push((**a1).clone());
                conv_struct_path(env, &c, b1, b2, child_path)
            }
        }
        (Term::Pair(a1, b1), Term::Pair(a2, b2)) => {
            conv_struct_path(env, ctx, a1, a2, child_path)
                && conv_struct_path(env, ctx, b1, b2, child_path)
        }
        (Term::App(f1, a1), Term::App(f2, a2)) => {
            if !conv_struct_path(env, ctx, f1, f2, child_path) {
                return false;
            }
            // Propositional-argument skip (`16 §8.2`): compare the argument at
            // the function's domain type via [`convert`], so an Ω-typed
            // argument is skipped (Ω-PI) and a Π/Σ-typed argument gets η. Falls
            // back to structural congruence if the function's type can't be
            // inferred (then this matches the K1 behaviour exactly).
            if let Ok(tf) = crate::check::infer(env, ctx, f1) {
                let tf_w = whnf(env, ctx, &tf);
                if let Term::Pi(dom, _cod) = &tf_w {
                    return convert_path(env, ctx, dom, a1, a2, child_path);
                }
            }
            conv_struct_path(env, ctx, a1, a2, child_path)
        }
        (Term::Proj1(p1), Term::Proj1(p2)) => conv_struct_path(env, ctx, p1, p2, child_path),
        (Term::Proj2(p1), Term::Proj2(p2)) => conv_struct_path(env, ctx, p1, p2, child_path),
        // Truncation congruence (`16 §6`): the former compares its underlying
        // type, and `|a|` compares its sole introduction operand.
        (Term::Trunc(a1), Term::Trunc(a2)) => conv_struct_path(env, ctx, a1, a2, child_path),
        (Term::TruncProj(t1), Term::TruncProj(t2)) => {
            conv_struct_path(env, ctx, t1, t2, child_path)
        }
        (
            Term::Elim {
                fam: f1,
                level_args: la1,
                params: p1,
                motive: m1,
                methods: ms1,
                indices: ix1,
                scrut: s1,
            },
            Term::Elim {
                fam: f2,
                level_args: la2,
                params: p2,
                motive: m2,
                methods: ms2,
                indices: ix2,
                scrut: s2,
            },
        ) => {
            f1 == f2
                && level_args_eq(la1, la2)
                && p1.len() == p2.len()
                && p1
                    .iter()
                    .zip(p2)
                    .all(|(x, y)| conv_struct_path(env, ctx, x, y, child_path))
                && conv_struct_path(env, ctx, m1, m2, child_path)
                && ms1.len() == ms2.len()
                && ms1
                    .iter()
                    .zip(ms2)
                    .all(|(x, y)| conv_struct_path(env, ctx, x, y, child_path))
                && ix1.len() == ix2.len()
                && ix1
                    .iter()
                    .zip(ix2)
                    .all(|(x, y)| conv_struct_path(env, ctx, x, y, child_path))
                && conv_struct_path(env, ctx, s1, s2, child_path)
        }
        (Term::Ascript(t1, _), x) => conv_struct_path(env, ctx, t1, x, child_path),
        (x, Term::Ascript(t2, _)) => conv_struct_path(env, ctx, x, t2, child_path),
        // `absurd` congruence. For Ω motives this is usually bypassed by the
        // proof-irrelevance shortcut; for Type motives it keeps `Absurd`
        // structurally comparable without adding any reduction rule.
        (Term::Absurd(m1, p1), Term::Absurd(m2, p2)) => {
            conv_struct_path(env, ctx, m1, m2, child_path)
                && conv_struct_path(env, ctx, p1, p2, child_path)
        }
        // `Eq` congruence (Gap-conv, `conv-eq-congruence`, re-landing here per
        // `obs-eq-termination`) — the missing congruence closure for the `Eq`
        // type-former: two `Eq` *types* convert iff their three components do,
        // recursively. Restores the invariant every other former above
        // already carries; not a loosening (fail-closed direction only —
        // recognises strictly more true equalities, never a false one).
        (Term::Eq(ty1, a1, b1), Term::Eq(ty2, a2, b2)) => {
            conv_struct_path(env, ctx, ty1, ty2, child_path)
                && conv_struct_path(env, ctx, a1, a2, child_path)
                && conv_struct_path(env, ctx, b1, b2, child_path)
        }
        // `IntLit` definitional equality: by `BigInt` value, matching the
        // observational `Eq`-at-registered-literal reduction (`obs.rs`).
        // Redundant with the `a == b` fast path above (canonical `BigInt`
        // representation makes derived `PartialEq` already correct here) —
        // kept explicit for auditability and defense-in-depth rather than
        // relying solely on the fast path.
        (Term::IntLit(m), Term::IntLit(n)) => m == n,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::{Level, LevelVar};

    #[test]
    fn level_semilattice_eq() {
        assert!(level_eq(&Level::zero(), &Level::zero()));
        assert!(level_eq(
            &Level::zero().max(Level::suc(Level::zero())),
            &Level::suc(Level::zero())
        ));
        assert!(level_eq(
            &Level::suc(Level::zero()).max(Level::Zero),
            &Level::suc(Level::zero())
        ));
        assert!(level_eq(&Level::zero().max(Level::zero()), &Level::zero())); // idempotent
        assert!(!level_eq(&Level::zero(), &Level::suc(Level::zero())));
    }

    // --- BLOCKER 1 regression: distinct level variables must not collapse ---
    // (Architect review on dec_2hnhhdb7mrxze.) The old domination test dropped
    // `max`-atoms by offset ignoring atom identity, so `max (suc u) v`
    // normalized to `suc u`. Distinct variables are incomparable.

    #[test]
    fn level_max_two_distinct_vars_do_not_collapse() {
        let u = Level::Var(LevelVar(0));
        let v = Level::Var(LevelVar(1));
        // max u v  must NOT equal u or v (they are distinct, incomparable vars).
        assert!(!level_eq(&u.clone().max(v.clone()), &u));
        assert!(!level_eq(&u.clone().max(v.clone()), &v));
        // max (suc u) v  must NOT equal suc u (v may exceed u).
        assert!(!level_eq(&u.clone().suc().max(v.clone()), &u.clone().suc()));
    }

    #[test]
    fn level_max_same_var_higher_offset_dominates() {
        // max (suc u) u = suc u  — same variable, higher offset absorbs lower.
        let u = Level::Var(LevelVar(0));
        assert!(level_eq(&u.clone().suc().max(u.clone()), &u.clone().suc()));
        // max u u = u  (idempotent, same variable).
        assert!(level_eq(&u.clone().max(u.clone()), &u));
    }

    #[test]
    fn level_max_zero_absorbed_by_var_at_same_offset() {
        // max (suc^n v) (suc^n 0) = suc^n v  — Zero absorbed by a Var at the
        // same offset (the `max ℓ 0 = ℓ` law at a non-zero offset).
        let v = Level::Var(LevelVar(2));
        assert!(level_eq(
            &v.clone().suc().max(Level::zero().suc()),
            &v.clone().suc()
        ));
    }

    #[test]
    fn level_equiv_reproduction_max_suc_u_v() {
        // The Architect's exact reproduction: equiv(max (suc u) v, suc u) must
        // be FALSE. (At u:=0, v:=5, `max 1 5 = 5 != 1`.)
        let u = Level::Var(LevelVar(0));
        let v = Level::Var(LevelVar(1));
        assert!(!u.clone().suc().max(v).equiv(&u.clone().suc()));
    }

    #[test]
    fn beta_whnf() {
        let env = GlobalEnv::new();
        let ctx = Context::new();
        // (λ x. x) y  ⇝  y   (x at index 0, y a free var 0 in empty ctx)
        let redex = Term::app(
            Term::lam(Term::Type(Level::zero()), Term::var(0)),
            Term::Type(Level::zero()),
        );
        assert_eq!(whnf(&env, &ctx, &redex), Term::Type(Level::zero()));
    }

    #[test]
    fn sigma_beta_whnf() {
        let env = GlobalEnv::new();
        let ctx = Context::new();
        let pair = Term::pair(Term::Type(Level::zero()), Term::Omega(Level::zero()));
        assert_eq!(
            whnf(&env, &ctx, &Term::proj1(pair.clone())),
            Term::Type(Level::zero())
        );
        assert_eq!(
            whnf(&env, &ctx, &Term::proj2(pair)),
            Term::Omega(Level::zero())
        );
    }

    #[test]
    fn pi_eta_convert() {
        let env = GlobalEnv::new();
        let ctx = Context::new();
        // f : (x:A)→B  in context; f ≡ λx. f x  at the Π-type.
        let a = Term::Type(Level::zero());
        let b = Term::Type(Level::suc(Level::zero()));
        let pi_ty = Term::pi(a.clone(), b.clone());
        // context: f at index 0 with type (x:A)→B
        let mut c = ctx.clone();
        c.push(pi_ty.clone());
        let f = Term::var(0);
        let eta = Term::lam(a.clone(), Term::app(Term::var(1), Term::var(0))); // λx. f x (f at 1, x at 0)
        assert!(convert(&env, &c, &pi_ty, &f, &eta));
        assert!(convert(&env, &c, &pi_ty, &eta, &f));
    }

    #[test]
    fn sigma_eta_convert() {
        let env = GlobalEnv::new();
        let ctx = Context::new();
        let a = Term::Type(Level::zero());
        let b = Term::Type(Level::suc(Level::zero()));
        let sig_ty = Term::sigma(a.clone(), b.clone());
        let mut c = ctx.clone();
        c.push(sig_ty.clone());
        let p = Term::var(0);
        let eta = Term::pair(Term::proj1(p.clone()), Term::proj2(p.clone()));
        assert!(convert(&env, &c, &sig_ty, &p, &eta));
    }

    fn beta_identity(domain: Term, argument: Term) -> Term {
        Term::app(Term::lam(domain, Term::var(0)), argument)
    }

    /// Durable invariant (`16 §6`): `Trunc` is congruent exactly when its
    /// interior is; a distinct universe remains distinct.
    #[test]
    fn trunc_congruence_accepts_convertible_interior_and_rejects_distinct_interior() {
        let env = GlobalEnv::new();
        let ctx = Context::new();
        let type_zero = Term::Type(Level::zero());
        let type_one = Term::Type(Level::suc(Level::zero()));
        let beta_type_zero = beta_identity(type_one.clone(), type_zero.clone());

        assert!(convert_type(
            &env,
            &ctx,
            &Term::Trunc(Box::new(beta_type_zero)),
            &Term::Trunc(Box::new(type_zero.clone())),
        ));
        assert!(!convert_type(
            &env,
            &ctx,
            &Term::Trunc(Box::new(type_zero)),
            &Term::Trunc(Box::new(type_one)),
        ));
    }

    /// Durable invariant (`16 §6`): structural `TruncProj` congruence recurses
    /// through its sole introduction operand; distinct open operands reject.
    #[test]
    fn trunc_proj_congruence_accepts_convertible_interior_and_rejects_distinct_interior() {
        let env = GlobalEnv::new();
        let mut ctx = Context::new();
        let type_zero = Term::Type(Level::zero());
        ctx.push(type_zero.clone());
        ctx.push(type_zero.clone());
        let beta_var_zero = beta_identity(type_zero, Term::var(0));

        assert!(conv_struct_path(
            &env,
            &ctx,
            &Term::TruncProj(Box::new(beta_var_zero)),
            &Term::TruncProj(Box::new(Term::var(0))),
            &[],
        ));
        assert!(!conv_struct_path(
            &env,
            &ctx,
            &Term::TruncProj(Box::new(Term::var(0))),
            &Term::TruncProj(Box::new(Term::var(1))),
            &[],
        ));
    }

    // ===== Recursive-head totality: δ-origin ledger observation matrix =====
    //
    // The path-local no-progress δ-origin ledger is exercised through its
    // observable events — reached δ unfold, pre-whnf capture, successful ι,
    // refusal — on the canonical distinct-recursive-`map` fixture that motivated
    // it. The `delta_probe` counters are compiled only under `cfg(test)`; the
    // production path carries zero instrumentation. These are the counter-level
    // twin of the integration file's 2 MiB normal-exit black-box contract.
    //
    // Traceability. Durable black-box twin:
    // `conformance/kernel/conversion/seed-conversion.md`,
    // `delta-distinct-recursive-heads-stuck` (promise class: durable invariant).
    // Governing spec: `17 §3.5` (distinct recursive-identity boundary) and
    // `17 §5` obligation 3 (cross-identity symbolic retry is not SCT-certified).
    // SCT itself is `17 §4` and is explicitly NOT this rule. Carriers/values are
    // the seed's well-typed `Bool`/`not`/`true`/`false` (`Type 0` inhabitants),
    // each checked well-typed by `assert_typed` before conversion. This is
    // authored traceability, not an executable scan of repository text.

    use crate::check::{declare_def, declare_inductive, declare_recursive_group, CtorSpec, InductiveSpec};

    const LU: LevelVar = LevelVar(0);

    fn lu() -> Level {
        Level::Var(LU)
    }
    fn zero() -> Level {
        Level::zero()
    }
    fn type0() -> Term {
        Term::Type(Level::zero())
    }

    fn cref_at(id: GlobalId, level: Level) -> Term {
        Term::Const {
            id,
            level_args: vec![level],
        }
    }
    fn cref0(id: GlobalId) -> Term {
        Term::Const {
            id,
            level_args: vec![],
        }
    }
    fn list_at(list: GlobalId, level: Level, element: Term) -> Term {
        Term::app(Term::indformer(list, vec![level]), element)
    }

    fn declare_list(env: &mut GlobalEnv) -> (GlobalId, GlobalId, GlobalId) {
        let list = declare_inductive(env, |list| InductiveSpec {
            level_params: vec![LU],
            params: vec![Term::Type(lu())],
            indices: vec![],
            level: lu(),
            constructors: vec![
                CtorSpec {
                    args: vec![],
                    target_indices: vec![],
                },
                CtorSpec {
                    args: vec![Term::var(0), list_at(list, lu(), Term::var(1))],
                    target_indices: vec![],
                },
            ],
        })
        .expect("List admission");
        let constructors = &env.inductive(list).expect("List lookup").constructors;
        (list, constructors[0].id, constructors[1].id)
    }

    fn map_type(list: GlobalId) -> Term {
        let type_u = Term::Type(lu());
        Term::pi(
            type_u.clone(),
            Term::pi(
                type_u,
                Term::pi(
                    Term::pi(Term::var(1), Term::var(1)),
                    Term::pi(
                        list_at(list, lu(), Term::var(2)),
                        list_at(list, lu(), Term::var(2)),
                    ),
                ),
            ),
        )
    }

    /// `map` body whose recursive edge targets `rec_target` (its own id for the
    /// self-recursive forms, or an already-declared const for the delegating
    /// control). When `dup` is set the cons method emits the mapped head TWICE
    /// (`Cons (f head) (Cons (f head) rec)`) — a genuinely different, still
    /// well-typed constructor equation, i.e. a different function from `map`.
    fn map_body_gen(
        list: GlobalId,
        nil: GlobalId,
        cons: GlobalId,
        rec_target: GlobalId,
        dup: bool,
    ) -> Term {
        let type_u = Term::Type(lu());
        let list_a = list_at(list, lu(), Term::var(3));
        let list_b_under_motive = list_at(list, lu(), Term::var(3));
        let motive = Term::Ascript(
            Box::new(Term::lam(list_a.clone(), list_b_under_motive)),
            Box::new(Term::pi(list_a, type_u.clone())),
        );
        let nil_b = Term::app(Term::constructor(nil, vec![lu()]), Term::var(2));
        // `f head` (identical, well-typed) in both variants.
        let head_image = Term::app(Term::var(4), Term::var(2));
        // The self/delegated recursion on the tail (`rec_target A B f xs`).
        let rec_call = Term::app(
            Term::app(
                Term::app(Term::app(cref_at(rec_target, lu()), Term::var(6)), Term::var(5)),
                Term::var(4),
            ),
            Term::var(1),
        );
        // `Cons var5 (f head) <tail>`.
        let one_cons = |tail: Term| {
            Term::app(
                Term::app(
                    Term::app(Term::constructor(cons, vec![lu()]), Term::var(5)),
                    head_image.clone(),
                ),
                tail,
            )
        };
        let cons_body = if dup {
            one_cons(one_cons(rec_call))
        } else {
            one_cons(rec_call)
        };
        let cons_method = Term::lam(
            Term::var(3),
            Term::lam(
                list_at(list, lu(), Term::var(4)),
                Term::lam(list_at(list, lu(), Term::var(4)), cons_body),
            ),
        );
        let elim = Term::Elim {
            fam: list,
            level_args: vec![lu()],
            params: vec![Term::var(3)],
            motive: Box::new(motive),
            methods: vec![nil_b, cons_method],
            indices: vec![],
            scrut: Box::new(Term::var(0)),
        };
        Term::lam(
            type_u.clone(),
            Term::lam(
                type_u,
                Term::lam(
                    Term::pi(Term::var(1), Term::var(1)),
                    Term::lam(list_at(list, lu(), Term::var(2)), elim),
                ),
            ),
        )
    }

    /// The plain self-recursive `map` (`f head`).
    fn declare_map(env: &mut GlobalEnv, list: GlobalId, nil: GlobalId, cons: GlobalId) -> GlobalId {
        let ty = map_type(list);
        declare_recursive_group(env, vec![(vec![LU], ty)], |ids| {
            vec![map_body_gen(list, nil, cons, ids[0], false)]
        })
        .expect("recursive map must be SCT-admitted")[0]
    }

    /// Control 1: a separately SCT-admitted self-recursive map whose constructor
    /// equation genuinely differs (emits each mapped element twice), so it is a
    /// different function from `map`, not a source-isomorphic copy.
    fn declare_map_twice(
        env: &mut GlobalEnv,
        list: GlobalId,
        nil: GlobalId,
        cons: GlobalId,
    ) -> GlobalId {
        let ty = map_type(list);
        declare_recursive_group(env, vec![(vec![LU], ty)], |ids| {
            vec![map_body_gen(list, nil, cons, ids[0], true)]
        })
        .expect("double-image map must be SCT-admitted")[0]
    }

    /// Control 2: a separately declared map whose recursive edge references
    /// `target` (an already-declared map) rather than itself, so its unfolded
    /// body is identical to `target`'s and the open comparison must be `true`.
    fn declare_map_delegating(
        env: &mut GlobalEnv,
        list: GlobalId,
        nil: GlobalId,
        cons: GlobalId,
        target: GlobalId,
    ) -> GlobalId {
        let ty = map_type(list);
        declare_def(env, vec![LU], ty, map_body_gen(list, nil, cons, target, false))
            .expect("delegating map admission")
    }

    /// `Nil` at element-type `ty`, monomorphic at level 0.
    fn nil_val(nil: GlobalId, ty: Term) -> Term {
        Term::app(Term::constructor(nil, vec![zero()]), ty)
    }
    /// `Cons ty head tail`, monomorphic at level 0.
    fn cons_val(cons: GlobalId, ty: Term, head: Term, tail: Term) -> Term {
        Term::app(
            Term::app(Term::app(Term::constructor(cons, vec![zero()]), ty), head),
            tail,
        )
    }
    /// `map<self> A B f xs`, monomorphic at level 0.
    fn map_apply(self_id: GlobalId, a: Term, b: Term, f: Term, xs: Term) -> Term {
        Term::app(
            Term::app(Term::app(Term::app(cref_at(self_id, zero()), a), b), f),
            xs,
        )
    }
    // ---- Well-typed Type-0 carriers/values (the seed's `Bool`/`not`) ----
    // The level-zero `map` requires its `A`/`B` arguments to inhabit `Type 0`.
    // `Term::Type(Level::zero())` does NOT (it inhabits `Type (suc 0)`), so the
    // normative carriers are a declared `Bool : Type 0` with `true`/`false`
    // values and `not : Bool -> Bool` — matching
    // `conformance/kernel/conversion/seed-conversion.md`
    // (`delta-distinct-recursive-heads-stuck`).

    /// `Bool : Type 0` with constructors `false` (0) and `true` (1).
    fn declare_bool(env: &mut GlobalEnv) -> (GlobalId, GlobalId, GlobalId) {
        let b = declare_inductive(env, |_b| InductiveSpec {
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
        .expect("Bool admission");
        let ctors = &env.inductive(b).expect("Bool lookup").constructors;
        (b, ctors[0].id, ctors[1].id)
    }
    /// `Bool` as a type term (`Type 0` inhabitant).
    fn bool_ty(bool_id: GlobalId) -> Term {
        Term::indformer(bool_id, vec![])
    }
    /// A nullary `Bool` constructor as a value.
    fn bool_ctor(id: GlobalId) -> Term {
        Term::constructor(id, vec![])
    }
    /// `not : Bool -> Bool` (`not false = true`, `not true = false`).
    fn declare_not(
        env: &mut GlobalEnv,
        bool_id: GlobalId,
        false_id: GlobalId,
        true_id: GlobalId,
    ) -> GlobalId {
        let bt = bool_ty(bool_id);
        let motive = Term::Ascript(
            Box::new(Term::lam(bt.clone(), bt.clone())),
            Box::new(Term::pi(bt.clone(), Term::Type(Level::zero()))),
        );
        let body = Term::lam(
            bt.clone(),
            Term::Elim {
                fam: bool_id,
                level_args: vec![],
                params: vec![],
                motive: Box::new(motive),
                methods: vec![bool_ctor(true_id), bool_ctor(false_id)],
                indices: vec![],
                scrut: Box::new(Term::var(0)),
            },
        );
        declare_def(env, vec![], Term::pi(bt.clone(), bt), body).expect("not admission")
    }

    /// Positive typing/admission observation: the fixture is well-typed BEFORE
    /// it is fed to `convert`/`convert_type` (which accept raw terms and do not
    /// establish typing), so a green verdict is over a normative input.
    fn assert_typed(env: &GlobalEnv, ctx: &Context, t: &Term) {
        assert!(
            crate::check::infer(env, ctx, t).is_ok(),
            "fixture must be well-typed before conversion; infer error: {:?}",
            crate::check::infer(env, ctx, t).err()
        );
    }

    /// Case 2 (zero δ unfold): two applications of the SAME recursive `map`
    /// close via the same-`Const`/spine fast path. The load-bearing assertion is
    /// `unfolds() == 0` — a CAUSAL count at the `unfold_const` edge, so it
    /// genuinely establishes that the recursive head was never δ-expanded (the
    /// distinct-head `captures()` proxy could not: it is blind to a same-head
    /// unfold). A β-redex third argument (vs its contractum) keeps it off the
    /// raw syntactic-identity path, so the spine path is what closes it.
    #[test]
    fn framed_case2_same_const_spine_does_zero_delta_unfold() {
        let mut env = GlobalEnv::new();
        let (list, nil, cons) = declare_list(&mut env);
        let map_f = declare_map(&mut env, list, nil, cons);
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let not = declare_not(&mut env, bool_id, false_id, true_id);
        let bt = bool_ty(bool_id);
        let ctx = Context::new();
        let a = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), nil_val(nil, bt.clone()));
        // Same spine (same `map_f`, same `not`), differing only in the list
        // argument by a type ASCRIPTION — convertible via the ascription arm
        // (α/strip, no δ), and both well-typed. This keeps the pair off the raw
        // syntactic-identity path while performing zero δ unfold.
        let ascribed_nil = Term::Ascript(
            Box::new(nil_val(nil, bt.clone())),
            Box::new(list_at(list, zero(), bt.clone())),
        );
        let b = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), ascribed_nil);
        assert_ne!(a, b, "must not be caught by the raw syntactic fast path");
        assert_typed(&env, &ctx, &a);
        assert_typed(&env, &ctx, &b);
        delta_probe::reset();
        assert!(convert_type(&env, &ctx, &a, &b));
        assert_eq!(
            delta_probe::unfolds(),
            0,
            "same-Const spine must perform zero δ unfolds (causal, not the capture proxy)"
        );
        assert_eq!(
            delta_probe::captures(),
            0,
            "same-Const spine captures no distinct-head δ-origin"
        );
        assert_eq!(delta_probe::iotas(), 0, "case 2 performs no ι reduction");
        assert_eq!(delta_probe::refusals(), 0);
    }

    /// Case 3 (finite δ retry, nonzero δ, zero refusal, true): two DISTINCT
    /// transparent constants that ARE convertible — the heads genuinely δ-unfold
    /// (`unfolds() >= 1`), a δ-origin is captured, but they converge, so the
    /// ledger records the pair and never refuses. This is the completeness guard
    /// AND the nonzero-δ counterpart to case 2's zero: a distinct-const pair that
    /// converges must not be over-rejected, and it must actually have unfolded.
    #[test]
    fn framed_case3_distinct_convertible_consts_unfold_but_never_refuse() {
        let mut env = GlobalEnv::new();
        let type1 = Term::Type(Level::suc(Level::zero()));
        // f := Type0 ; h := f — distinct constants; h δ-unfolds through f to
        // Type0 (finite δ retries on the h side), so they converge.
        let f = declare_def(&mut env, vec![], type1.clone(), type0()).expect("f admission");
        let h = declare_def(&mut env, vec![], type1, cref0(f)).expect("h admission");
        let ctx = Context::new();
        assert_typed(&env, &ctx, &cref0(f));
        assert_typed(&env, &ctx, &cref0(h));
        delta_probe::reset();
        assert!(convert_type(&env, &ctx, &cref0(f), &cref0(h)));
        assert!(
            delta_probe::unfolds() >= 1,
            "distinct convertible heads must actually δ-unfold (nonzero, vs case 2's zero)"
        );
        assert!(
            delta_probe::captures() >= 1,
            "distinct transparent heads must capture a δ-origin"
        );
        assert_eq!(delta_probe::iotas(), 0, "aliases converge by δ/β, not ι");
        assert_eq!(
            delta_probe::refusals(),
            0,
            "a convergent distinct-const pair must never be refused"
        );
    }

    /// Control 1 (frame AC-MATRIX case 4): a separately SCT-admitted recursive
    /// function whose constructor equation genuinely DIFFERS from `map` (emits
    /// each element twice). The open comparison is `false` for a SEMANTIC reason
    /// — the cons-method bodies diverge structurally — and this MUST be
    /// distinguished from the source-isomorphic false case, which is false via
    /// the divergence refusal. The discriminator is `refusals() == 0`: the
    /// structural body difference is caught before any recurring-pair refusal,
    /// so `false` here is NOT produced by the ledger guard (while `captures()`
    /// confirms the ledger did engage on the distinct heads).
    #[test]
    fn control_genuinely_different_recursive_function_is_false_without_refusal() {
        let mut env = GlobalEnv::new();
        let (list, nil, cons) = declare_list(&mut env);
        let map_f = declare_map(&mut env, list, nil, cons);
        let map_twice = declare_map_twice(&mut env, list, nil, cons);
        assert_ne!(map_f, map_twice);
        let ctx = Context::new();
        assert_typed(&env, &ctx, &cref_at(map_f, lu()));
        assert_typed(&env, &ctx, &cref_at(map_twice, lu()));
        delta_probe::reset();
        assert!(
            !convert_type(&env, &ctx, &cref_at(map_f, lu()), &cref_at(map_twice, lu())),
            "a genuinely different recursive function must not convert with map"
        );
        assert!(
            delta_probe::captures() >= 1,
            "the distinct (map_f, map_twice) heads are captured (ledger engaged)"
        );
        assert_eq!(
            delta_probe::refusals(),
            0,
            "false must come from the structural body difference, NOT a ledger refusal"
        );
    }

    /// Control 2 (seed one-axis positive): a separately declared `map_g` whose
    /// recursive edge references `map_f` (not itself). Its unfolded body is
    /// syntactically identical to `map_f`'s, so the open comparison FLIPS to
    /// `true` — the ledger captures the pair, then converges at the post-whnf
    /// equality without refusing. This is the positive twin of the
    /// source-isomorphic (self-referencing) `map_g`, which stays false.
    #[test]
    fn control_delegating_map_g_flips_open_comparison_to_true() {
        let mut env = GlobalEnv::new();
        let (list, nil, cons) = declare_list(&mut env);
        let map_f = declare_map(&mut env, list, nil, cons);
        let map_g_deleg = declare_map_delegating(&mut env, list, nil, cons, map_f);
        assert_ne!(map_f, map_g_deleg);
        let ctx = Context::new();
        assert_typed(&env, &ctx, &cref_at(map_f, lu()));
        assert_typed(&env, &ctx, &cref_at(map_g_deleg, lu()));
        delta_probe::reset();
        assert!(
            convert_type(&env, &ctx, &cref_at(map_f, lu()), &cref_at(map_g_deleg, lu())),
            "a map delegating its recursive edge to map_f must convert with map_f"
        );
        assert!(
            delta_probe::captures() >= 1,
            "the distinct (map_f, map_g_deleg) heads are captured"
        );
        assert_eq!(
            delta_probe::refusals(),
            0,
            "the delegating map converges (identical unfolded body), so never refuses"
        );
    }

    /// Open-recursive case (recurring pair, no ι, one refusal, false): two
    /// DISTINCT recursive `map`s whose recursion sits on a neutral (bound)
    /// scrutinee — no ι ever fires, so the (map_f, map_g) δ-origin recurs and
    /// the guard refuses exactly once, returning `false` in finite time (the
    /// D0 divergence, observed at the counter level rather than by stack
    /// exhaustion).
    #[test]
    fn open_recursive_distinct_maps_refuse_once_and_return_false() {
        let mut env = GlobalEnv::new();
        let (list, nil, cons) = declare_list(&mut env);
        let map_f = declare_map(&mut env, list, nil, cons);
        let map_g = declare_map(&mut env, list, nil, cons);
        assert_ne!(map_f, map_g, "the two maps must be distinct constants");
        let ctx = Context::new();
        assert_typed(&env, &ctx, &cref_at(map_f, lu()));
        assert_typed(&env, &ctx, &cref_at(map_g, lu()));
        delta_probe::reset();
        assert!(
            !convert_type(&env, &ctx, &cref_at(map_f, lu()), &cref_at(map_g, lu())),
            "distinct recursive maps are not definitionally equal"
        );
        assert!(
            delta_probe::captures() >= 1,
            "the (map_f, map_g) δ-origin must be captured"
        );
        assert_eq!(
            delta_probe::iotas(),
            0,
            "the neutral scrutinee means no ι ever fires"
        );
        assert_eq!(
            delta_probe::refusals(),
            1,
            "the recurring no-progress pair refuses exactly once"
        );
    }

    /// Closed positive (Nil): `map_f Nil ≡ map_g Nil` — both ι-reduce to the
    /// same closed `Nil B`, so ι fires and the verdict is `true` with zero
    /// refusals. This positive converges at the post-whnf equality, so it does
    /// NOT depend on ι-discharge (contrast the two-Cons positive).
    #[test]
    fn closed_nil_positive_reduces_by_iota_without_refusal() {
        let mut env = GlobalEnv::new();
        let (list, nil, cons) = declare_list(&mut env);
        let map_f = declare_map(&mut env, list, nil, cons);
        let map_g = declare_map(&mut env, list, nil, cons);
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let not = declare_not(&mut env, bool_id, false_id, true_id);
        let _ = cons;
        let bt = bool_ty(bool_id);
        let ctx = Context::new();
        let lhs = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), nil_val(nil, bt.clone()));
        let rhs = map_apply(map_g, bt.clone(), bt.clone(), cref0(not), nil_val(nil, bt));
        assert_typed(&env, &ctx, &lhs);
        assert_typed(&env, &ctx, &rhs);
        delta_probe::reset();
        assert!(convert_type(&env, &ctx, &lhs, &rhs));
        assert!(delta_probe::iotas() >= 1, "the Nil scrutinee must fire ι");
        assert_eq!(delta_probe::refusals(), 0);
    }

    /// Closed positive (two-Cons): `map_f L ≡ map_g L` for a closed 2-element
    /// list. Both distinct maps ι-reduce in lock-step; the ledger DISCHARGES
    /// the (map_f, map_g) pair on each ι step, letting the structural descent
    /// reach equality. This is the ι-discharge-load-bearing positive — the one
    /// that reddens under the "suppress real-ι discharge" mutation.
    #[test]
    fn closed_two_cons_positive_reduces_by_iota_without_refusal() {
        let mut env = GlobalEnv::new();
        let (list, nil, cons) = declare_list(&mut env);
        let map_f = declare_map(&mut env, list, nil, cons);
        let map_g = declare_map(&mut env, list, nil, cons);
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let not = declare_not(&mut env, bool_id, false_id, true_id);
        let bt = bool_ty(bool_id);
        let ctx = Context::new();
        // L = Cons Bool true (Cons Bool false (Nil Bool)) — distinct elements.
        let l = cons_val(
            cons,
            bt.clone(),
            bool_ctor(true_id),
            cons_val(
                cons,
                bt.clone(),
                bool_ctor(false_id),
                nil_val(nil, bt.clone()),
            ),
        );
        let lhs = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), l.clone());
        let rhs = map_apply(map_g, bt.clone(), bt.clone(), cref0(not), l);
        assert_typed(&env, &ctx, &lhs);
        assert_typed(&env, &ctx, &rhs);
        delta_probe::reset();
        assert!(convert_type(&env, &ctx, &lhs, &rhs));
        assert!(
            delta_probe::iotas() >= 1,
            "the Cons scrutinees must fire ι at each level"
        );
        assert_eq!(
            delta_probe::refusals(),
            0,
            "ι-discharge must keep the lock-step descent refusal-free"
        );
    }
}
