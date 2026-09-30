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

/// Conversion's first pass: reduce β/ι/etc. but leave a transparent global
/// application folded at the head, so congruence can inspect its spine first.
/// A nested reduction may commit δ only if its consumer fires; otherwise the
/// stuck component is rebuilt in this same deferred-head mode.
fn whnf_defer_head_delta(env: &GlobalEnv, ctx: &Context, t: &Term) -> (Term, WhnfProgress) {
    whnf_progress_mode(env, ctx, t, true, true)
}

/// Retry full head δ in conversion, retaining deferred heads inside stuck
/// eliminators so a symbolic recursive call reaches its own δ-origin ledger.
fn whnf_progress_for_conversion(env: &GlobalEnv, ctx: &Context, t: &Term) -> (Term, WhnfProgress) {
    whnf_progress_mode(env, ctx, t, false, true)
}

/// Only a transparent head needs a second pass: it may expose a constructor
/// or a reducible observational type to the enclosing consumer. A neutral
/// deferred head already makes that consumer stuck.
fn deferred_head_is_transparent_const(env: &GlobalEnv, term: &Term) -> bool {
    // Borrow the same peeled head as `peel_app(term).0`, without cloning the
    // entire (often deeply nested) component just to inspect its head.
    let mut head = term;
    while let Term::App(f, _) = head {
        head = f;
    }
    matches!(head, Term::Const { id, .. } if env.transparent_body(*id).is_some())
}

/// Reduce a nested component once in deferred mode. If its folded head can
/// discharge the consumer, retry δ on that result rather than reducing the
/// original component again. The caller commits both progress flags only when
/// the consumer fires; otherwise it rebuilds from the deferred term/flag.
/// Public `whnf` puts its eager result in the deferred slot, with no retry.
/// The optional retry avoids cloning a stuck neutral component merely to test
/// its consumer, which matters for large eliminator methods or deep spines.
fn whnf_nested_component(
    env: &GlobalEnv,
    ctx: &Context,
    component: &Term,
    defer_stuck_nested_delta: bool,
) -> (Term, WhnfProgress, Option<(Term, WhnfProgress)>) {
    if defer_stuck_nested_delta {
        let (deferred, progress) = whnf_defer_head_delta(env, ctx, component);
        let eager = deferred_head_is_transparent_const(env, &deferred)
            .then(|| whnf_progress_for_conversion(env, ctx, &deferred));
        (deferred, progress, eager)
    } else {
        let (eager, progress) = whnf_progress(env, ctx, component);
        (eager, progress, None)
    }
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
    whnf_progress_mode(env, ctx, t, false, false)
}

/// `defer_head_delta` applies along the function spine. In conversion-only
/// mode a nested component is rolled back to deferred-head form if its
/// consumer stays stuck; the public reducer still commits eager δ.
#[allow(clippy::only_used_in_recursion)]
fn whnf_progress_mode(
    env: &GlobalEnv,
    ctx: &Context,
    t: &Term,
    defer_head_delta: bool,
    defer_stuck_nested_delta: bool,
) -> (Term, WhnfProgress) {
    probe_reducer_entry();
    let mut cur = t.clone();
    let mut iota = false;
    loop {
        match &cur {
            Term::App(f, a) => {
                let (f_w, fp) =
                    whnf_progress_mode(env, ctx, f, defer_head_delta, defer_stuck_nested_delta);
                iota |= fp.iota;
                // K3: only the registered String -> List Char operation on
                // an immutable checked String literal. No other primitive
                // call, nor a neutral String argument, gains reduction.
                if let Term::Const { id: op, level_args } = &f_w {
                    if level_args.is_empty() {
                        if let Some((char_type, nil, cons)) = env.literal_char_view(*op) {
                            let (argument, progress) = whnf_progress(env, ctx, a);
                            // The original argument remains in the stuck App.
                            // Its reductions count toward the δ-origin ledger
                            // only when the checked-literal result consumes it.
                            if let Term::Const {
                                id: literal,
                                level_args: literal_levels,
                            } = argument
                            {
                                if literal_levels.is_empty() {
                                    if let Some(value) = env.checked_literal(literal) {
                                        iota |= progress.iota;
                                        let char_ty = Term::const_(char_type, vec![]);
                                        let mut result = Term::app(
                                            Term::constructor(nil, vec![]),
                                            char_ty.clone(),
                                        );
                                        for scalar in value.as_str().chars().rev() {
                                            result = Term::app(
                                                Term::app(
                                                    Term::app(
                                                        Term::constructor(cons, vec![]),
                                                        char_ty.clone(),
                                                    ),
                                                    Term::IntLit((scalar as u32).into()),
                                                ),
                                                result,
                                            );
                                        }
                                        cur = result;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                match &f_w {
                    Term::Lam(_, body) => {
                        cur = subst0(body, a);
                        continue;
                    }
                    Term::Const { id, level_args }
                        if !defer_head_delta && env.transparent_body(*id).is_some() =>
                    {
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
                let (p_d, dp, p_e) = whnf_nested_component(env, ctx, p, defer_stuck_nested_delta);
                let p_w = p_e.as_ref().map_or(&p_d, |(e, _)| e);
                match p_w {
                    Term::Pair(a, _) => {
                        iota |= dp.iota || p_e.as_ref().is_some_and(|(_, ep)| ep.iota);
                        cur = (**a).clone();
                        continue;
                    }
                    // The legacy explicit δ retry is public-whnf only. In
                    // conversion, a stuck projectee must take the folded
                    // catch-all below, even when it is a transparent Const.
                    Term::Const { id, level_args }
                        if !defer_stuck_nested_delta && env.transparent_body(*id).is_some() =>
                    {
                        if let Some(body) = unfold_const(env, *id, level_args) {
                            cur = Term::proj1(body);
                            continue;
                        }
                        iota |= dp.iota;
                        return (Term::proj1(p_d), WhnfProgress { iota });
                    }
                    _ => {
                        iota |= dp.iota;
                        return (Term::proj1(p_d), WhnfProgress { iota });
                    }
                }
            }
            Term::Proj2(p) => {
                let (p_d, dp, p_e) = whnf_nested_component(env, ctx, p, defer_stuck_nested_delta);
                let p_w = p_e.as_ref().map_or(&p_d, |(e, _)| e);
                match p_w {
                    Term::Pair(_, b) => {
                        iota |= dp.iota || p_e.as_ref().is_some_and(|(_, ep)| ep.iota);
                        cur = (**b).clone();
                        continue;
                    }
                    Term::Const { id, level_args }
                        if !defer_stuck_nested_delta && env.transparent_body(*id).is_some() =>
                    {
                        if let Some(body) = unfold_const(env, *id, level_args) {
                            cur = Term::proj2(body);
                            continue;
                        }
                        iota |= dp.iota;
                        return (Term::proj2(p_d), WhnfProgress { iota });
                    }
                    _ => {
                        iota |= dp.iota;
                        return (Term::proj2(p_d), WhnfProgress { iota });
                    }
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
                let (s_d, dp, s_e) =
                    whnf_nested_component(env, ctx, scrut, defer_stuck_nested_delta);
                let s_w = s_e.as_ref().map_or(&s_d, |(e, _)| e);
                let (head, all_args) = peel_app(s_w);
                if let Term::Constructor { id, .. } = head {
                    if let Some((ind, k)) = env.constructor(id) {
                        if ind.id == *fam {
                            if let Ok(reduct) = iota_reduct(
                                env, ind, k, level_args, params, motive, methods, &all_args,
                            ) {
                                // This ι step subsumes both nested progress
                                // flags (`14 §7.2`).
                                iota = true;
                                probe_iota();
                                cur = reduct;
                                continue;
                            }
                        }
                    }
                }
                // A stuck scrutinee cannot commit eager nested δ; keep its
                // deferred head for the structural child comparison instead.
                // Indices don't gate ι firing (`14 §7.2`).
                iota |= dp.iota;
                return (
                    Term::Elim {
                        fam: *fam,
                        level_args: level_args.clone(),
                        params: params.clone(),
                        motive: motive.clone(),
                        methods: methods.clone(),
                        indices: indices.clone(),
                        scrut: Box::new(s_d),
                    },
                    WhnfProgress { iota },
                );
            }
            Term::Const { id, level_args }
                if !defer_head_delta && env.transparent_body(*id).is_some() =>
            {
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
                let (ty_d, dp, ty_e) =
                    whnf_nested_component(env, ctx, ty, defer_stuck_nested_delta);
                let ty_w = ty_e.as_ref().map_or(&ty_d, |(e, _)| e);
                if let Some(r) = crate::obs::eq_reduce(env, ctx, ty_w, x, y) {
                    iota |= dp.iota || ty_e.as_ref().is_some_and(|(_, ep)| ep.iota);
                    cur = r;
                    continue;
                }
                iota |= dp.iota;
                return (
                    Term::Eq(Box::new(ty_d), (*x).clone(), (*y).clone()),
                    WhnfProgress { iota },
                );
            }
            Term::Cast(a, b, e, t) => {
                // `cast A B e t` reduces by recursion on `whnf(A)`,`whnf(B)`
                // (`16 §3.2`); mismatched/neutral heads or a neutral proof leave
                // it a neutral cast.
                let (a_d, ad, a_e) = whnf_nested_component(env, ctx, a, defer_stuck_nested_delta);
                let (b_d, bd, b_e) = whnf_nested_component(env, ctx, b, defer_stuck_nested_delta);
                let a_w = a_e.as_ref().map_or(&a_d, |(e, _)| e);
                let b_w = b_e.as_ref().map_or(&b_d, |(e, _)| e);
                if let Some(r) = crate::obs::cast_reduce(env, ctx, a_w, b_w, e, t) {
                    iota |= ad.iota
                        || a_e.as_ref().is_some_and(|(_, ep)| ep.iota)
                        || bd.iota
                        || b_e.as_ref().is_some_and(|(_, ep)| ep.iota);
                    cur = r;
                    continue;
                }
                iota |= ad.iota || bd.iota;
                return (
                    Term::Cast(Box::new(a_d), Box::new(b_d), (*e).clone(), (*t).clone()),
                    WhnfProgress { iota },
                );
            }
            Term::J(motive, base, eq) => {
                // A well-typed `J` always reduces: `infer_j` guarantees that
                // `eq` infers to `Eq`, so `j_reduce` returns the base on `refl`
                // and a `Cast` otherwise (`15 §4`). This rebuild is reachable
                // only for ill-typed raw input; structural conversion keeps
                // that residual fail-closed by providing no `J` arm.
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
                let (s_d, dp, s_e) =
                    whnf_nested_component(env, ctx, scrut, defer_stuck_nested_delta);
                let s_w = s_e.as_ref().map_or(&s_d, |(e, _)| e);
                match s_w {
                    Term::QuotClass(a0) => {
                        iota |= dp.iota || s_e.as_ref().is_some_and(|(_, ep)| ep.iota);
                        cur = Term::app((**method).clone(), (**a0).clone());
                        continue;
                    }
                    Term::TruncProj(a0) => {
                        iota |= dp.iota || s_e.as_ref().is_some_and(|(_, ep)| ep.iota);
                        cur = Term::app((**method).clone(), (**a0).clone());
                        continue;
                    }
                    _ => {}
                }
                iota |= dp.iota;
                return (
                    Term::QuotElim {
                        motive: (*motive).clone(),
                        method: (*method).clone(),
                        respect: (*respect).clone(),
                        scrut: Box::new(s_d),
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
// At each structural edge, first weak-head reduce while deferring δ at the
// head (`17 §3.3`, §3.5). Equal transparent heads can compare their argument
// spines without unfolding. If congruence fails, or the heads differ, full δ
// is retried. An origin is recorded only if BOTH deferred heads are
// transparent and at least one belongs to a cycle in the transparent-body
// graph, including (c, c) for recursive c. A recurring soft origin can be
// discharged only by ι; a recurring hard one refuses when its SCT-decreasing
// arguments mention a fresh binder, regardless of ι. This never concludes
// equality from a cyclic hypothesis. Each edge either descends on a subterm of
// deferred-whnf inputs or retries δ. Pure non-recursive δ terminates down
// the condensation DAG: if a non-recursive g recurred indefinitely by
// regeneration from recursive f, then g reaches f and f reaches g,
// contradicting g's non-recursiveness. Finite constant pairs force some
// origin to recur infinitely. If it stays soft, each lap needs a fresh ι.
// Without an
// intervening structural descent into a stuck eliminator, a recursive call
// can then be reached only at the whnf head or in an executed ι method on a
// real constructor. Infinitely many such soft discharges form an infinite
// executed call trace of an SCT-admitted recursive group; SCT rules that out.
// Nested δ is committed only by an executed consumer, so a symbolic recursive
// call under a stuck eliminator still reaches the ledger at its Const head.
//
// A hard origin may recur when no SCT-decreasing argument mentions a binder
// introduced after that origin. Suppose an infinite chain did so: infinitely
// many meetings of one recursive head give an idempotent SCT loop with a
// strictly decreasing parameter i. Its position belongs to D(g), the union
// of strict diagonals measured at admission. A decrease obtained through a
// stuck destructuring introduces a method binder after the origin and would
// refuse. Thus every decrease must come from executed ι on a finite term;
// infinitely many such decreases are impossible. Requiring every position in
// D(g) to be non-symbolic suffices without guessing which loop supplies i.
//
// Governing spec: `17 §3.3` step (5) and `17 §3.5` require head-δ deferral
// before congruence; the distinct-identity boundary also forbids unbounded
// cross-identity symbolic retries. SCT (`17 §4`) certifies each admitted
// recursive group, not a lock-step comparison of different groups. The
// black-box twin is `conformance/kernel/conversion/seed-conversion.md`,
// `delta-distinct-recursive-heads-stuck` (durable invariant).

/// A canonical unordered pair of transparent-`Const` GlobalIds (possibly
/// equal) at a δ retry; the origin of a structural conversion edge.
type ConstPair = (GlobalId, GlobalId);

/// A recorded δ-origin. Hard entries have passed into a stuck eliminator's
/// components; a later ι cannot witness descent for a symbolic call.
/// The entry depth identifies binders introduced since this particular retry.
#[derive(Clone, Copy)]
struct DeltaPathEntry {
    pair: ConstPair,
    hard: bool,
    depth: usize,
}

/// Whether an argument mentions one of the `fresh` innermost context binders.
/// Variables bound inside the argument are not context variables: traverse
/// each Pi/Sigma/Lam/Let binder with its own de Bruijn depth.
fn mentions_var_below(term: &Term, fresh: usize) -> bool {
    let mut pending = vec![(term, 0)];
    while let Some((term, local)) = pending.pop() {
        match term {
            Term::Var(i) if *i >= local && *i - local < fresh => return true,
            Term::Pi(domain, body) | Term::Lam(domain, body) | Term::Sigma(domain, body) => {
                pending.push((domain, local));
                pending.push((body, local + 1));
            }
            Term::Let { ty, val, body } => {
                pending.push((ty, local));
                pending.push((val, local));
                pending.push((body, local + 1));
            }
            _ => pending.extend(term.children().into_iter().map(|child| (child, local))),
        }
    }
    false
}

/// Only the parameter positions whose strict diagonals SCT actually checked
/// can justify refusal. The heads/spines are already peeled on the deferred
/// plane; a partial application at a decreasing position is fail-closed.
fn symbolic_since(
    env: &GlobalEnv,
    ctx: &Context,
    left: (&Term, &[Term]),
    right: (&Term, &[Term]),
    entry_depth: usize,
) -> bool {
    let Some(fresh) = ctx.len().checked_sub(entry_depth) else {
        return true;
    };
    [left, right].iter().any(|(head, args)| {
        let Term::Const { id, .. } = head else {
            return false;
        };
        if !env.is_recursive_transparent(*id) {
            return false;
        }
        let needs_check = |position: usize| {
            args.get(position)
                .is_none_or(|arg| mentions_var_below(arg, fresh))
        };
        match env.sct_decreasing_positions(*id) {
            Some(positions) if !positions.is_empty() => positions.iter().copied().any(needs_check),
            // Raw transparent installs did not pass SCT. An empty D on a
            // now-cyclic member is likewise no certificate for that cycle.
            // Check every declared position, refusing even the zero-arity
            // case where an unchecked δ-cycle offers no argument to inspect.
            _ => match crate::sct::declared_arity(env, *id) {
                Ok(0) | Err(_) => true,
                Ok(arity) => (0..arity).any(needs_check),
            },
        }
    })
}

/// Canonicalise so `(x, y)` and `(y, x)` denote the same δ-origin.
fn canonical_pair(x: GlobalId, y: GlobalId) -> ConstPair {
    if x <= y {
        (x, y)
    } else {
        (y, x)
    }
}

/// The deferred-whnf δ-origin of a structural retry: `Some((min, max))`
/// iff both sides are applications (possibly nullary) headed by transparent
/// constants and at least one head lies on a transparent-body cycle. Equal
/// recursive heads are included when their spine comparison failed. A pair
/// of non-recursive heads (or any opaque head) has no δ-retry origin.
fn is_transparent(env: &GlobalEnv, id: GlobalId) -> bool {
    matches!(env.lookup(id), Some(crate::env::Decl::Transparent { .. }))
}

fn delta_origin_pair(env: &GlobalEnv, a: &Term, b: &Term) -> Option<ConstPair> {
    let (ha, _) = peel_app(a);
    let (hb, _) = peel_app(b);
    match (&ha, &hb) {
        (Term::Const { id: ia, .. }, Term::Const { id: ib, .. })
            if is_transparent(env, *ia)
                && is_transparent(env, *ib)
                && (env.is_recursive_transparent(*ia) || env.is_recursive_transparent(*ib)) =>
        {
            Some(canonical_pair(*ia, *ib))
        }
        _ => None,
    }
}

/// Test-only observation of the δ-ledger events — a reached δ unfold (the
/// causal count, at the `unfold_const` edge), deferred-head retry capture, a
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
        static HARD_CONTINUES: Cell<u64> = const { Cell::new(0) };
        static REDUCER_ENTRIES: Cell<u64> = const { Cell::new(0) };
    }
    pub(super) fn reset() {
        UNFOLDS.with(|c| c.set(0));
        CAPTURES.with(|c| c.set(0));
        IOTAS.with(|c| c.set(0));
        REFUSALS.with(|c| c.set(0));
        HARD_CONTINUES.with(|c| c.set(0));
        REDUCER_ENTRIES.with(|c| c.set(0));
    }
    pub(super) fn bump_reducer_entry() {
        REDUCER_ENTRIES.with(|c| c.set(c.get() + 1));
    }
    pub(super) fn reducer_entries() -> u64 {
        REDUCER_ENTRIES.with(|c| c.get())
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
    pub(super) fn bump_hard_continue() {
        HARD_CONTINUES.with(|c| c.set(c.get() + 1));
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
    pub(super) fn hard_continues() -> u64 {
        HARD_CONTINUES.with(|c| c.get())
    }
}

#[cfg(test)]
#[inline]
fn probe_reducer_entry() {
    delta_probe::bump_reducer_entry();
}
#[cfg(not(test))]
#[inline(always)]
fn probe_reducer_entry() {}

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

#[cfg(test)]
#[inline]
fn probe_hard_continue() {
    delta_probe::bump_hard_continue();
}
#[cfg(not(test))]
#[inline(always)]
fn probe_hard_continue() {}

/// Definitional equality `Γ ⊢ a ≡ b : A` for the K1 fragment (`13 §6.2`):
/// α (de Bruijn syntactic identity), then type-directed η (Π-η, Σ-η) when the
/// type is a Π/Σ, else structural congruence with whnf. This is the **K2c
/// extension seam** — K2c replaces this body with lazy-WHNF NbE without
/// changing the signature (`13 §6.3`). K2 adds the Ω-PI shortcut (`16 §8.2`).
///
/// The path-local no-progress δ-origin ledger (empty here at the public entry)
/// is threaded through the private recursion to bound no-progress δ retries;
/// see [`conv_struct_path`].
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
    path: &[DeltaPathEntry],
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
/// `path` is the path-local no-progress δ-origin ledger. Reduce both sides
/// with head δ deferred first. Compare same transparent heads by congruence;
/// otherwise retry full δ if a head is transparent, and record the pair of
/// deferred transparent heads only if at least one is recursive. Head
/// ι-progress discharges a recurring soft pair; a recurring hard pair
/// refuses when an SCT-decreasing argument uses a fresh binder. A hard entry
/// otherwise remains hard across its full δ retry; fresh entries start soft.
fn conv_struct_path(
    env: &GlobalEnv,
    ctx: &Context,
    a: &Term,
    b: &Term,
    path: &[DeltaPathEntry],
) -> bool {
    // Syntactic-identity fast path (pre-δ, `13 §6.2` step 1): identical
    // de Bruijn terms are convertible with no reduction and no ledger touch.
    if a == b {
        return true;
    }

    let (a_deferred, ad) = whnf_defer_head_delta(env, ctx, a);
    let (b_deferred, bd) = whnf_defer_head_delta(env, ctx, b);
    if a_deferred == b_deferred {
        return true;
    }

    // Head identity is observed AFTER β/let/ascription reduction. A head
    // hidden under β in the source is just as eligible for spine congruence
    // as one visible before weak-head reduction. A failed spine comparison
    // falls through to δ retry (e.g. a constant ignoring that argument).
    let (ha, args_a) = peel_app(&a_deferred);
    let (hb, args_b) = peel_app(&b_deferred);
    if let (
        Term::Const {
            id: ia,
            level_args: la,
        },
        Term::Const {
            id: ib,
            level_args: lb,
        },
    ) = (&ha, &hb)
    {
        if ia == ib
            && is_transparent(env, *ia)
            && level_args_eq(la, lb)
            && args_a.len() == args_b.len()
            && args_a
                .iter()
                .zip(&args_b)
                .all(|(x, y)| conv_struct_path(env, ctx, x, y, path))
        {
            return true;
        }
    }

    // The ledger's origin belongs to the deferred plane, which reveals a
    // transparent head even if β hid it in the original terms. A recursive
    // same-head retry is recorded too: it can recur one binder deeper.
    let transparent_head = |head: &Term| match head {
        Term::Const { id, .. } => is_transparent(env, *id),
        _ => false,
    };
    let retry = transparent_head(&ha) || transparent_head(&hb);
    let origin = if retry {
        delta_origin_pair(env, &a_deferred, &b_deferred)
    } else {
        None
    };
    if origin.is_some() {
        probe_capture();
    }
    let (a, ap) = if retry {
        whnf_progress_for_conversion(env, ctx, &a_deferred)
    } else {
        (a_deferred, WhnfProgress::default())
    };
    let (b, bp) = if retry {
        whnf_progress_for_conversion(env, ctx, &b_deferred)
    } else {
        (b_deferred, WhnfProgress::default())
    };
    let iota_progress = ad.iota || bd.iota || ap.iota || bp.iota;

    if a == b {
        return true;
    }

    // The ledger the structural descendants inherit. A first sighting is
    // recorded even if the current whnf happened to do ι: only a later ι may
    // discharge a *previously recorded* soft entry. A hard entry is never
    // discharged, and both α checks above precede every refusal.
    let hard_entry = origin.and_then(|p| path.iter().find(|entry| entry.pair == p && entry.hard));
    let child_storage: Vec<DeltaPathEntry>;
    let child_path: &[DeltaPathEntry] = match origin {
        Some(_)
            if hard_entry.is_some_and(|entry| {
                symbolic_since(env, ctx, (&ha, &args_a), (&hb, &args_b), entry.depth)
            }) =>
        {
            probe_refusal();
            return false;
        }
        // Do not enter the soft ι arm: even a closed-argument recurrence
        // remains hard for every descendant of this path.
        Some(_) if hard_entry.is_some() => {
            probe_hard_continue();
            path
        }
        Some(p) if !path.iter().any(|entry| entry.pair == p) => {
            child_storage = {
                let mut v = path.to_vec();
                v.push(DeltaPathEntry {
                    pair: p,
                    hard: false,
                    depth: ctx.len(),
                });
                v
            };
            &child_storage
        }
        Some(p) if iota_progress => {
            child_storage = path
                .iter()
                .copied()
                .filter(|entry| entry.pair != p)
                .collect();
            &child_storage
        }
        Some(_) => {
            probe_refusal();
            return false;
        }
        None => path,
    };

    // Only a structural match on a whnf-stuck eliminator descends into its
    // components. Canonical forms, binders, neutral App spines and Ascript do
    // not harden. J has no structural congruence arm: stuck J already refuses.
    let hard_storage: Vec<DeltaPathEntry>;
    let child_path: &[DeltaPathEntry] = if !child_path.is_empty()
        && matches!(
            (&a, &b),
            (Term::Elim { .. }, Term::Elim { .. })
                | (Term::QuotElim { .. }, Term::QuotElim { .. })
                | (Term::Proj1(_), Term::Proj1(_))
                | (Term::Proj2(_), Term::Proj2(_))
                | (Term::TruncProj(_), Term::TruncProj(_))
                | (Term::Cast(_, _, _, _), Term::Cast(_, _, _, _))
                | (Term::Absurd(_, _), Term::Absurd(_, _))
        ) {
        hard_storage = child_path
            .iter()
            .map(|entry| DeltaPathEntry {
                pair: entry.pair,
                hard: true,
                depth: entry.depth,
            })
            .collect();
        &hard_storage
    } else {
        child_path
    };

    match (&a, &b) {
        (Term::Type(l1), Term::Type(l2)) => level_eq(l1, l2),
        // Proposition-universe congruence (`17 §3.3`, §3.6), deliberately
        // distinct from proof irrelevance: these are two universe terms, not
        // two proofs at one proposition. Their levels must be equivalent.
        (Term::Omega(l1), Term::Omega(l2)) => level_eq(l1, l2),
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
        // Neutral Cast congruence (`16 §3.2`, `17 §3.3`): all four fields are
        // structural. In particular `e` is deliberately not skipped by proof
        // irrelevance because this type-agnostic path has no trusted field type.
        (Term::Cast(a1, b1, e1, t1), Term::Cast(a2, b2, e2, t2)) => {
            conv_struct_path(env, ctx, a1, a2, child_path)
                && conv_struct_path(env, ctx, b1, b2, child_path)
                && conv_struct_path(env, ctx, e1, e2, child_path)
                && conv_struct_path(env, ctx, t1, t2, child_path)
        }
        // Quotient congruence (`16 §5`, `17 §3.3`): quotient types compare
        // their carriers and relations structurally. Class introductions
        // compare only their representatives; relation-respect is an
        // elimination-time obligation, never an extra equality premise here.
        (Term::Quot(a1, r1), Term::Quot(a2, r2)) => {
            conv_struct_path(env, ctx, a1, a2, child_path)
                && conv_struct_path(env, ctx, r1, r2, child_path)
        }
        (Term::QuotClass(t1), Term::QuotClass(t2)) => {
            conv_struct_path(env, ctx, t1, t2, child_path)
        }
        // A neutral quotient eliminator is congruent exactly when its four
        // fields are (`16 §5`, `17 §3.3`). `respect` remains structural rather
        // than proof-irrelevance-skipped for the same reason as Cast's `e`.
        (
            Term::QuotElim {
                motive: m1,
                method: f1,
                respect: r1,
                scrut: s1,
            },
            Term::QuotElim {
                motive: m2,
                method: f2,
                respect: r2,
                scrut: s2,
            },
        ) => {
            conv_struct_path(env, ctx, m1, m2, child_path)
                && conv_struct_path(env, ctx, f1, f2, child_path)
                && conv_struct_path(env, ctx, r1, r2, child_path)
                && conv_struct_path(env, ctx, s1, s2, child_path)
        }
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

    /// Durable reducer-work invariant: a stuck component is reduced once,
    /// even when conversion defers its head δ. MEASURED: reducer entries at
    /// depths 16/32/64 across every nested-consumer arm, against public whnf
    /// on the identical neutral term. CLAIMED: conversion's deferral adds no
    /// exponential work on nested stuck components. GAP: entry counts do not
    /// measure allocation, nor independent computation inside a consumer.
    /// Cast tests *both type components* as deep neutral projection chains;
    /// `Cast^k` itself already repeats conversion within public `whnf`.
    #[test]
    fn stuck_nested_components_take_linear_reducer_entries() {
        let env = GlobalEnv::new();
        let mut ctx = Context::new();
        ctx.push(Term::Type(Level::zero()));
        for shape in ["proj1", "proj2", "elim", "quot", "eq", "cast"] {
            for k in [16, 32, 64] {
                let mut term = Term::var(0);
                for _ in 0..k {
                    term = match shape {
                        "proj1" => Term::proj1(term),
                        "proj2" => Term::proj2(term),
                        "elim" => Term::Elim {
                            fam: GlobalId(9000),
                            level_args: vec![],
                            params: vec![],
                            motive: Box::new(Term::var(1)),
                            methods: vec![],
                            indices: vec![],
                            scrut: Box::new(term),
                        },
                        "quot" => Term::QuotElim {
                            motive: Box::new(Term::var(1)),
                            method: Box::new(Term::var(1)),
                            respect: Box::new(Term::var(1)),
                            scrut: Box::new(term),
                        },
                        "eq" => Term::Eq(
                            Box::new(term),
                            Box::new(Term::var(1)),
                            Box::new(Term::var(2)),
                        ),
                        // Both Cast types carry k nested stuck projectees.
                        "cast" => Term::proj2(term),
                        _ => unreachable!(),
                    };
                }
                if shape == "proj2" {
                    // This deep neutral projectee can arise from a checked
                    // Σ-field chain, not only from a raw reducer probe.
                    let mut field_type = Term::Type(Level::zero());
                    for _ in 0..k {
                        field_type = Term::sigma(Term::Type(Level::zero()), field_type);
                    }
                    let mut typed_ctx = Context::new();
                    typed_ctx.push(field_type);
                    assert_eq!(
                        crate::check::infer(&env, &typed_ctx, &term).expect("nested Σ projection"),
                        Term::Type(Level::zero())
                    );
                }
                if shape == "cast" {
                    let mut other = Term::var(0);
                    for _ in 0..k {
                        other = Term::proj1(other);
                    }
                    term = Term::Cast(
                        Box::new(term),
                        Box::new(other),
                        Box::new(Term::var(1)),
                        Box::new(Term::var(2)),
                    );
                }
                delta_probe::reset();
                let public = whnf(&env, &ctx, &term);
                let public_count = delta_probe::reducer_entries();
                delta_probe::reset();
                let deferred = whnf_defer_head_delta(&env, &ctx, &term).0;
                let deferred_count = delta_probe::reducer_entries();
                delta_probe::reset();
                let full = whnf_progress_for_conversion(&env, &ctx, &term).0;
                let full_count = delta_probe::reducer_entries();
                assert_eq!(public, deferred, "deferred {shape} depth {k}");
                assert_eq!(public, full, "full {shape} depth {k}");
                assert!(
                    public_count > k,
                    "public reducer must reach the nested site"
                );
                for (mode, entries) in [("deferred", deferred_count), ("full", full_count)] {
                    assert!(
                        entries <= public_count * 2 && entries <= k * 6 + 8,
                        "{shape} depth {k}: {mode} {entries} vs public {public_count} entries"
                    );
                }
            }
        }
    }

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

    fn cast_term(a: Term, b: Term, e: Term, t: Term) -> Term {
        Term::Cast(Box::new(a), Box::new(b), Box::new(e), Box::new(t))
    }

    fn quotient_elim_term(motive: Term, method: Term, respect: Term, scrut: Term) -> Term {
        Term::QuotElim {
            motive: Box::new(motive),
            method: Box::new(method),
            respect: Box::new(respect),
            scrut: Box::new(scrut),
        }
    }

    fn assert_structural_verdict(
        env: &GlobalEnv,
        ctx: &Context,
        left: Term,
        right: Term,
        expected: bool,
        field: &str,
    ) {
        assert_ne!(left, right, "{field} control must avoid syntactic equality");
        assert_eq!(
            conv_struct_path(env, ctx, &left, &right, &[]),
            expected,
            "unexpected {field} verdict left-to-right"
        );
        assert_eq!(
            conv_struct_path(env, ctx, &right, &left, &[]),
            expected,
            "unexpected {field} verdict right-to-left"
        );
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

    /// Durable invariant (`17 §3.3`, §3.6): Omega congruence uses semantic
    /// level equality, while non-equivalent levels remain distinct.
    #[test]
    fn omega_congruence_accepts_level_equivalence_and_rejects_distinct_levels() {
        let env = GlobalEnv::new();
        let ctx = Context::new();
        let level = Level::Var(LevelVar(0));

        assert!(convert_type(
            &env,
            &ctx,
            &Term::Omega(level.clone().max(Level::zero())),
            &Term::Omega(level),
        ));
        assert!(!convert_type(
            &env,
            &ctx,
            &Term::Omega(Level::zero()),
            &Term::Omega(Level::suc(Level::zero())),
        ));
    }

    /// Durable invariant (`16 §5`, `17 §3.3`): quotient congruence accepts a
    /// reducible relation and independently rejects a changed carrier or
    /// relation.
    #[test]
    fn quotient_congruence_compares_carrier_and_relation_directionally() {
        let env = GlobalEnv::new();
        let carrier = Term::Type(Level::zero());
        let other_carrier = Term::Type(Level::suc(Level::zero()));
        let relation_type = Term::pi(
            carrier.clone(),
            Term::pi(carrier.clone(), Term::Omega(Level::zero())),
        );
        let mut ctx = Context::new();
        ctx.push(relation_type.clone());
        ctx.push(relation_type);
        let reducible_relation =
            beta_identity(ctx.lookup(0).expect("relation type").clone(), Term::var(0));

        assert!(convert_type(
            &env,
            &ctx,
            &Term::Quot(Box::new(carrier.clone()), Box::new(reducible_relation)),
            &Term::Quot(Box::new(carrier.clone()), Box::new(Term::var(0))),
        ));
        assert!(!convert_type(
            &env,
            &ctx,
            &Term::Quot(Box::new(carrier.clone()), Box::new(Term::var(0))),
            &Term::Quot(Box::new(other_carrier), Box::new(Term::var(0))),
        ));
        assert!(!convert_type(
            &env,
            &ctx,
            &Term::Quot(Box::new(carrier.clone()), Box::new(Term::var(0))),
            &Term::Quot(Box::new(carrier), Box::new(Term::var(1))),
        ));
    }

    /// Durable invariant (`16 §3.2`, `17 §3.3`): neutral Cast congruence
    /// recursively compares A, B, proof, and value. Every field independently
    /// accepts a non-syntactic equality and rejects a distinct term in both
    /// directions; the neutral endpoints keep the outer Cast from reducing.
    #[test]
    fn cast_congruence_compares_all_four_fields_directionally() {
        let env = GlobalEnv::new();
        let mut ctx = Context::new();
        for _ in 0..10 {
            ctx.push(Term::Type(Level::zero()));
        }
        let level = Level::Var(LevelVar(0));
        let omega = Term::Omega(level.clone());
        let omega_max_zero = Term::Omega(level.max(Level::zero()));
        let (a, b, e, t, other) = (
            Term::var(9),
            Term::var(8),
            Term::var(7),
            Term::var(6),
            Term::var(5),
        );

        for (field, left, right) in [
            (
                "Cast source type A",
                cast_term(omega_max_zero.clone(), b.clone(), e.clone(), t.clone()),
                cast_term(omega.clone(), b.clone(), e.clone(), t.clone()),
            ),
            (
                "Cast target type B",
                cast_term(a.clone(), omega_max_zero, e.clone(), t.clone()),
                cast_term(a.clone(), omega, e.clone(), t.clone()),
            ),
            (
                "Cast equality proof e",
                cast_term(
                    a.clone(),
                    b.clone(),
                    beta_identity(Term::Type(Level::zero()), e.clone()),
                    t.clone(),
                ),
                cast_term(a.clone(), b.clone(), e.clone(), t.clone()),
            ),
            (
                "Cast value t",
                cast_term(
                    a.clone(),
                    b.clone(),
                    e.clone(),
                    beta_identity(Term::Type(Level::zero()), t.clone()),
                ),
                cast_term(a.clone(), b.clone(), e.clone(), t.clone()),
            ),
        ] {
            assert_structural_verdict(&env, &ctx, left, right, true, field);
        }

        let baseline = cast_term(a.clone(), b.clone(), e.clone(), t.clone());
        for (field, changed) in [
            (
                "Cast source type A",
                cast_term(other.clone(), b.clone(), e.clone(), t.clone()),
            ),
            (
                "Cast target type B",
                cast_term(a.clone(), other.clone(), e.clone(), t.clone()),
            ),
            (
                "Cast equality proof e",
                cast_term(a.clone(), b.clone(), other.clone(), t.clone()),
            ),
            ("Cast value t", cast_term(a, b, e, other)),
        ] {
            assert_structural_verdict(&env, &ctx, baseline.clone(), changed, false, field);
        }
    }

    /// Durable invariant (`16 §5`): quotient-class congruence compares only
    /// representatives. A reducible representative accepts; distinct open
    /// representatives reject rather than acquiring relatedness implicitly.
    #[test]
    fn quotient_class_congruence_compares_representatives_only() {
        let env = GlobalEnv::new();
        let representative_type = Term::Type(Level::zero());
        let mut ctx = Context::new();
        ctx.push(representative_type.clone());
        ctx.push(representative_type.clone());
        let reducible_representative = beta_identity(representative_type, Term::var(0));

        assert!(conv_struct_path(
            &env,
            &ctx,
            &Term::QuotClass(Box::new(reducible_representative)),
            &Term::QuotClass(Box::new(Term::var(0))),
            &[],
        ));
        assert!(!conv_struct_path(
            &env,
            &ctx,
            &Term::QuotClass(Box::new(Term::var(0))),
            &Term::QuotClass(Box::new(Term::var(1))),
            &[],
        ));
    }

    /// Durable invariant (`16 §5`, `17 §3.3`): a neutral QuotElim compares
    /// motive, method, respect proof, and scrutinee structurally. Every field
    /// has a non-syntactic acceptance and a bidirectional lone-field reject.
    #[test]
    fn quotient_elim_congruence_compares_all_four_fields_directionally() {
        let env = GlobalEnv::new();
        let mut ctx = Context::new();
        for _ in 0..12 {
            ctx.push(Term::Type(Level::zero()));
        }
        let (motive, method, respect, scrut, other) = (
            Term::var(11),
            Term::var(10),
            Term::var(9),
            Term::var(8),
            Term::var(7),
        );
        let beta = |term| beta_identity(Term::Type(Level::zero()), term);
        let level = Level::Var(LevelVar(0));
        let scrut_left = Term::app(Term::var(6), Term::Omega(level.clone().max(Level::zero())));
        let scrut_right = Term::app(Term::var(6), Term::Omega(level));

        for (field, left, right) in [
            (
                "QuotElim motive",
                quotient_elim_term(
                    beta(motive.clone()),
                    method.clone(),
                    respect.clone(),
                    scrut.clone(),
                ),
                quotient_elim_term(
                    motive.clone(),
                    method.clone(),
                    respect.clone(),
                    scrut.clone(),
                ),
            ),
            (
                "QuotElim method",
                quotient_elim_term(
                    motive.clone(),
                    beta(method.clone()),
                    respect.clone(),
                    scrut.clone(),
                ),
                quotient_elim_term(
                    motive.clone(),
                    method.clone(),
                    respect.clone(),
                    scrut.clone(),
                ),
            ),
            (
                "QuotElim respect proof",
                quotient_elim_term(
                    motive.clone(),
                    method.clone(),
                    beta(respect.clone()),
                    scrut.clone(),
                ),
                quotient_elim_term(
                    motive.clone(),
                    method.clone(),
                    respect.clone(),
                    scrut.clone(),
                ),
            ),
            (
                "QuotElim scrutinee",
                quotient_elim_term(motive.clone(), method.clone(), respect.clone(), scrut_left),
                quotient_elim_term(motive.clone(), method.clone(), respect.clone(), scrut_right),
            ),
        ] {
            assert_structural_verdict(&env, &ctx, left, right, true, field);
        }

        let baseline = quotient_elim_term(
            motive.clone(),
            method.clone(),
            respect.clone(),
            scrut.clone(),
        );
        for (field, changed) in [
            (
                "QuotElim motive",
                quotient_elim_term(
                    other.clone(),
                    method.clone(),
                    respect.clone(),
                    scrut.clone(),
                ),
            ),
            (
                "QuotElim method",
                quotient_elim_term(
                    motive.clone(),
                    other.clone(),
                    respect.clone(),
                    scrut.clone(),
                ),
            ),
            (
                "QuotElim respect proof",
                quotient_elim_term(motive.clone(), method.clone(), other.clone(), scrut.clone()),
            ),
            (
                "QuotElim scrutinee",
                quotient_elim_term(motive, method, respect, other),
            ),
        ] {
            assert_structural_verdict(&env, &ctx, baseline.clone(), changed, false, field);
        }
    }

    /// Durable invariant: heterogeneous heads remain fail-closed after adding
    /// the two same-former congruence arms.
    #[test]
    fn cast_and_quotient_elim_heterogeneous_heads_stay_distinct() {
        let env = GlobalEnv::new();
        let mut ctx = Context::new();
        for _ in 0..4 {
            ctx.push(Term::Type(Level::zero()));
        }
        let cast = cast_term(Term::var(3), Term::var(2), Term::var(1), Term::var(0));
        let elim = quotient_elim_term(Term::var(3), Term::var(2), Term::var(1), Term::var(0));
        assert!(!conv_struct_path(&env, &ctx, &cast, &elim, &[]));
        assert!(!conv_struct_path(&env, &ctx, &elim, &cast, &[]));
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

    use crate::check::{
        declare_def, declare_inductive, declare_postulate, declare_recursive_group, CtorSpec,
        InductiveSpec,
    };

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
                Term::app(
                    Term::app(cref_at(rec_target, lu()), Term::var(6)),
                    Term::var(5),
                ),
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
        declare_def(
            env,
            vec![LU],
            ty,
            map_body_gen(list, nil, cons, target, false),
        )
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
    fn declare_nat_for_iota(env: &mut GlobalEnv) -> (GlobalId, GlobalId, GlobalId) {
        let nat = declare_inductive(env, |nat| InductiveSpec {
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
                    args: vec![Term::indformer(nat, vec![])],
                    target_indices: vec![],
                },
            ],
        })
        .expect("Nat admission");
        let ctors = &env.inductive(nat).expect("Nat lookup").constructors;
        (nat, ctors[0].id, ctors[1].id)
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
        let a = map_apply(
            map_f,
            bt.clone(),
            bt.clone(),
            cref0(not),
            nil_val(nil, bt.clone()),
        );
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
            "successful same-Const spine captures no δ-retry origin"
        );
        assert_eq!(delta_probe::iotas(), 0, "case 2 performs no ι reduction");
        assert_eq!(delta_probe::refusals(), 0);
    }

    /// Durable invariant (`17 §3.3`, §3.5): β can reveal the same recursive
    /// head after the old pre-whnf spine check. Compare its arguments before
    /// unfolding δ; this is the closed-Nil instance of the public conversion
    /// path, not a source-shape assertion.
    #[test]
    fn beta_reveals_same_head_and_spine_converts_without_delta() {
        let mut env = GlobalEnv::new();
        let (list, nil, cons) = declare_list(&mut env);
        let map = declare_map(&mut env, list, nil, cons);
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let not = declare_not(&mut env, bool_id, false_id, true_id);
        let bt = bool_ty(bool_id);
        let list_bool = list_at(list, zero(), bt.clone());
        let original = map_apply(map, bt.clone(), bt.clone(), cref0(not), nil_val(nil, bt));
        let beta_hidden = Term::app(
            Term::Ascript(
                Box::new(Term::lam(list_bool.clone(), Term::var(0))),
                Box::new(Term::pi(list_bool.clone(), list_bool.clone())),
            ),
            original.clone(),
        );
        let ascribed = Term::Ascript(Box::new(original), Box::new(list_bool));
        let ctx = Context::new();
        assert_typed(&env, &ctx, &beta_hidden);
        assert_typed(&env, &ctx, &ascribed);
        delta_probe::reset();
        assert!(convert_type(&env, &ctx, &beta_hidden, &ascribed));
        assert_eq!(
            delta_probe::unfolds(),
            0,
            "β-revealed shared head must retain δ deferral"
        );
        assert_eq!(delta_probe::refusals(), 0);
    }

    /// Deferred δ applies only at the outer head: a transparent function
    /// inside an ι scrutinee must still unfold and expose its constructor.
    #[test]
    fn deferred_outer_head_keeps_full_delta_in_eliminator_scrutinee() {
        let mut env = GlobalEnv::new();
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let not = declare_not(&mut env, bool_id, false_id, true_id);
        let bt = bool_ty(bool_id);
        let motive = Term::Ascript(
            Box::new(Term::lam(bt.clone(), bt.clone())),
            Box::new(Term::pi(bt.clone(), Term::Type(Level::zero()))),
        );
        let scrut = Term::app(cref0(not), bool_ctor(true_id));
        let elim = Term::Elim {
            fam: bool_id,
            level_args: vec![],
            params: vec![],
            motive: Box::new(motive),
            methods: vec![bool_ctor(true_id), bool_ctor(false_id)],
            indices: vec![],
            scrut: Box::new(scrut),
        };
        let ctx = Context::new();
        assert_typed(&env, &ctx, &elim);
        delta_probe::reset();
        let (reduced, progress) = whnf_defer_head_delta(&env, &ctx, &elim);
        assert_eq!(reduced, bool_ctor(true_id));
        assert!(progress.iota);
        assert!(
            delta_probe::unfolds() >= 1,
            "nested `not` must still δ-unfold"
        );
        assert_eq!(whnf(&env, &ctx, &elim), bool_ctor(true_id));
    }

    /// SCT-admitted `ignore f z n`: the base is `true`, and the Suc method
    /// passes both otherwise-unused inputs unchanged to the recursive call.
    /// On an open `n`, unequal `f` or `z` must be refused at the SAME-head
    /// origin after spine conversion fails, not equated by a cyclic premise.
    fn declare_unused_recursive_inputs(
        env: &mut GlobalEnv,
        nat: GlobalId,
        true_id: GlobalId,
        bool_id: GlobalId,
    ) -> GlobalId {
        let bt = bool_ty(bool_id);
        let nt = Term::indformer(nat, vec![]);
        let ft = Term::pi(bt.clone(), bt.clone());
        let ty = Term::pi(
            ft.clone(),
            Term::pi(bt.clone(), Term::pi(nt.clone(), bt.clone())),
        );
        declare_recursive_group(env, vec![(vec![], ty)], |ids| {
            let recursive_call = Term::app(
                Term::app(Term::app(cref0(ids[0]), Term::var(4)), Term::var(3)),
                Term::var(1),
            );
            let suc_method = Term::lam(nt.clone(), Term::lam(bt.clone(), recursive_call));
            let motive = Term::Ascript(
                Box::new(Term::lam(nt.clone(), bt.clone())),
                Box::new(Term::pi(nt.clone(), Term::Type(Level::zero()))),
            );
            vec![Term::lam(
                ft.clone(),
                Term::lam(
                    bt.clone(),
                    Term::lam(
                        nt.clone(),
                        Term::Elim {
                            fam: nat,
                            level_args: vec![],
                            params: vec![],
                            motive: Box::new(motive),
                            methods: vec![bool_ctor(true_id), suc_method],
                            indices: vec![],
                            scrut: Box::new(Term::var(0)),
                        },
                    ),
                ),
            )]
        })
        .expect("structural recursion ignoring two carried inputs must be admitted")[0]
    }

    fn unused_recursive_apply(id: GlobalId, f: Term, z: Term, n: Term) -> Term {
        Term::app(Term::app(Term::app(cref0(id), f), z), n)
    }

    #[test]
    fn same_head_open_recursive_retry_refuses_nonconvertible_function_and_accumulator() {
        let mut env = GlobalEnv::new();
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let bt = bool_ty(bool_id);
        let nat = declare_inductive(&mut env, |nat| InductiveSpec {
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
                    args: vec![Term::indformer(nat, vec![])],
                    target_indices: vec![],
                },
            ],
        })
        .expect("Nat admission");
        let id = declare_unused_recursive_inputs(&mut env, nat, true_id, bool_id);
        let not = declare_not(&mut env, bool_id, false_id, true_id);
        let identity = declare_def(
            &mut env,
            vec![],
            Term::pi(bt.clone(), bt.clone()),
            Term::lam(bt.clone(), Term::var(0)),
        )
        .expect("Bool identity admission");
        let nt = Term::indformer(nat, vec![]);
        let mut ctx = Context::new();
        ctx.push(nt);
        let left = unused_recursive_apply(id, cref0(not), bool_ctor(true_id), Term::var(0));
        let changed_function =
            unused_recursive_apply(id, cref0(identity), bool_ctor(true_id), Term::var(0));
        let changed_accumulator =
            unused_recursive_apply(id, cref0(not), bool_ctor(false_id), Term::var(0));
        for term in [&left, &changed_function, &changed_accumulator] {
            assert_typed(&env, &ctx, term);
        }
        for other in [&changed_function, &changed_accumulator] {
            delta_probe::reset();
            assert!(!convert_type(&env, &ctx, &left, other));
            assert_eq!(delta_probe::iotas(), 0, "open Nat cannot ι-reduce");
            assert!(
                delta_probe::captures() >= 1,
                "same transparent head must reach a retry"
            );
            assert_eq!(
                delta_probe::refusals(),
                1,
                "same-head no-progress recurrence must refuse"
            );
        }
    }

    /// Case 3 (finite δ retry, nonzero δ, zero refusal, true): two DISTINCT
    /// transparent non-recursive constants that ARE convertible. They genuinely
    /// δ-unfold (`unfolds() >= 1`) without a ledger origin. This is the
    /// nonzero-δ counterpart to case 2's zero: a finite convergent pair must
    /// not be refused or mistaken for a recursive retry.
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
        assert_eq!(
            delta_probe::captures(),
            0,
            "two acyclic heads never create a recursion-keyed δ-origin"
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
            convert_type(
                &env,
                &ctx,
                &cref_at(map_f, lu()),
                &cref_at(map_g_deleg, lu())
            ),
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

    fn neutral_nat_recursive_elim(
        nat: GlobalId,
        bool_id: GlobalId,
        true_id: GlobalId,
        self_id: GlobalId,
    ) -> Term {
        let bt = bool_ty(bool_id);
        let nt = Term::indformer(nat, vec![]);
        let recursive_call = Term::app(cref0(self_id), Term::var(1));
        Term::Elim {
            fam: nat,
            level_args: vec![],
            params: vec![],
            motive: Box::new(Term::Ascript(
                Box::new(Term::lam(nt.clone(), bt.clone())),
                Box::new(Term::pi(nt.clone(), Term::Type(Level::zero()))),
            )),
            methods: vec![
                bool_ctor(true_id),
                Term::lam(nt.clone(), Term::lam(bt, recursive_call)),
            ],
            indices: vec![],
            scrut: Box::new(Term::var(0)),
        }
    }

    /// Both declarations are SCT-admitted independently. A closed Bool ι-step
    /// selects a neutral Nat eliminator whose step method calls its own head on
    /// the predecessor; the two heads have distinct identities.
    fn closed_iota_recursive_pair(
        env: &mut GlobalEnv,
        nat: GlobalId,
        bool_id: GlobalId,
        true_id: GlobalId,
    ) -> GlobalId {
        let bt = bool_ty(bool_id);
        let nt = Term::indformer(nat, vec![]);
        let ty = Term::pi(nt.clone(), bt.clone());
        declare_recursive_group(env, vec![(vec![], ty)], |ids| {
            let nat_elim = neutral_nat_recursive_elim(nat, bool_id, true_id, ids[0]);
            let closed_bool_elim = Term::Elim {
                fam: bool_id,
                level_args: vec![],
                params: vec![],
                motive: Box::new(Term::Ascript(
                    Box::new(Term::lam(bt.clone(), bt.clone())),
                    Box::new(Term::pi(bt.clone(), Term::Type(Level::zero()))),
                )),
                methods: vec![nat_elim.clone(), nat_elim],
                indices: vec![],
                scrut: Box::new(bool_ctor(true_id)),
            };
            vec![Term::lam(nt.clone(), closed_bool_elim)]
        })
        .expect("closed-Bool wrapper remains SCT-admitted")[0]
    }

    fn closed_nat_recursive_pair(
        env: &mut GlobalEnv,
        nat: GlobalId,
        zero_id: GlobalId,
        bool_id: GlobalId,
        true_id: GlobalId,
    ) -> GlobalId {
        let bt = bool_ty(bool_id);
        let nt = Term::indformer(nat, vec![]);
        let ty = Term::pi(nt.clone(), bt.clone());
        declare_recursive_group(env, vec![(vec![], ty)], |ids| {
            let nat_elim = neutral_nat_recursive_elim(nat, bool_id, true_id, ids[0]);
            let closed_nat_elim = Term::Elim {
                fam: nat,
                level_args: vec![],
                params: vec![],
                motive: Box::new(Term::Ascript(
                    Box::new(Term::lam(nt.clone(), bt.clone())),
                    Box::new(Term::pi(nt.clone(), Term::Type(Level::zero()))),
                )),
                methods: vec![
                    nat_elim,
                    Term::lam(nt.clone(), Term::lam(bt.clone(), bool_ctor(true_id))),
                ],
                indices: vec![],
                scrut: Box::new(Term::constructor(zero_id, vec![])),
            };
            vec![Term::lam(nt.clone(), closed_nat_elim)]
        })
        .expect("closed-Nat wrapper remains SCT-admitted")[0]
    }

    /// The recursive call occurs as a Nat-eliminator scrutinee inside the
    /// predecessor method of another Nat elimination. Both groups must pass
    /// SCT; the closed Bool only selects this neutral computation.
    fn closed_iota_recursive_scrutinee_pair(
        env: &mut GlobalEnv,
        nat: GlobalId,
        zero_id: GlobalId,
        suc_id: GlobalId,
        bool_id: GlobalId,
        true_id: GlobalId,
    ) -> GlobalId {
        let nt = Term::indformer(nat, vec![]);
        let bt = bool_ty(bool_id);
        let motive = || {
            Term::Ascript(
                Box::new(Term::lam(nt.clone(), nt.clone())),
                Box::new(Term::pi(nt.clone(), Term::Type(Level::zero()))),
            )
        };
        let zero = Term::constructor(zero_id, vec![]);
        declare_recursive_group(
            env,
            vec![(vec![], Term::pi(nt.clone(), nt.clone()))],
            |ids| {
                let inner_step = Term::lam(
                    nt.clone(),
                    Term::lam(
                        nt.clone(),
                        Term::app(Term::constructor(suc_id, vec![]), Term::var(0)),
                    ),
                );
                let inner = Term::Elim {
                    fam: nat,
                    level_args: vec![],
                    params: vec![],
                    motive: Box::new(motive()),
                    methods: vec![zero.clone(), inner_step],
                    indices: vec![],
                    scrut: Box::new(Term::app(cref0(ids[0]), Term::var(1))),
                };
                let outer_nat = Term::Elim {
                    fam: nat,
                    level_args: vec![],
                    params: vec![],
                    motive: Box::new(motive()),
                    methods: vec![
                        zero.clone(),
                        Term::lam(nt.clone(), Term::lam(nt.clone(), inner)),
                    ],
                    indices: vec![],
                    scrut: Box::new(Term::var(0)),
                };
                let closed_bool = Term::Elim {
                    fam: bool_id,
                    level_args: vec![],
                    params: vec![],
                    motive: Box::new(Term::Ascript(
                        Box::new(Term::lam(bt.clone(), nt.clone())),
                        Box::new(Term::pi(bt.clone(), Term::Type(Level::zero()))),
                    )),
                    methods: vec![outer_nat.clone(), outer_nat],
                    indices: vec![],
                    scrut: Box::new(bool_ctor(true_id)),
                };
                vec![Term::lam(nt.clone(), closed_bool)]
            },
        )
        .expect("recursive-call-in-scrutinee must be SCT-admitted")[0]
    }

    /// Durable invariant, `17 §3.5`: unrelated closed ι cannot erase the
    /// identity boundary of two recursive heads beneath a stuck eliminator.
    #[test]
    fn closed_bool_iota_cannot_erase_distinct_recursive_heads() {
        let mut env = GlobalEnv::new();
        let (bool_id, _, true_id) = declare_bool(&mut env);
        let (nat, _, _) = declare_nat_for_iota(&mut env);
        let c = closed_iota_recursive_pair(&mut env, nat, bool_id, true_id);
        let d = closed_iota_recursive_pair(&mut env, nat, bool_id, true_id);
        assert_ne!(c, d, "separately admitted heads must remain distinct");
        let mut ctx = Context::new();
        ctx.push(Term::indformer(nat, vec![]));
        let lhs = Term::app(cref0(c), Term::var(0));
        let rhs = Term::app(cref0(d), Term::var(0));
        assert_typed(&env, &ctx, &lhs);
        assert_typed(&env, &ctx, &rhs);
        delta_probe::reset();
        assert!(
            !convert_type(&env, &ctx, &lhs, &rhs),
            "a closed Bool ι must not make distinct neutral recursion equal"
        );
        assert!(
            delta_probe::captures() >= 1,
            "the pair must reach the δ-ledger"
        );
        assert!(
            delta_probe::iotas() >= 1,
            "the closed Bool scrutinee must ι-reduce"
        );
        assert_eq!(
            delta_probe::refusals(),
            1,
            "the recurrent hard pair must refuse"
        );
    }

    /// Durable invariant, `17 §3.5`: changing the unrelated closed ι from
    /// Bool to Nat cannot erase a distinct recursive-identity boundary.
    #[test]
    fn closed_nat_iota_cannot_erase_distinct_recursive_heads() {
        let mut env = GlobalEnv::new();
        let (bool_id, _, true_id) = declare_bool(&mut env);
        let (nat, zero, _) = declare_nat_for_iota(&mut env);
        let c = closed_nat_recursive_pair(&mut env, nat, zero, bool_id, true_id);
        let d = closed_nat_recursive_pair(&mut env, nat, zero, bool_id, true_id);
        assert_ne!(c, d);
        let mut ctx = Context::new();
        ctx.push(Term::indformer(nat, vec![]));
        let lhs = Term::app(cref0(c), Term::var(0));
        let rhs = Term::app(cref0(d), Term::var(0));
        assert_typed(&env, &ctx, &lhs);
        assert_typed(&env, &ctx, &rhs);
        delta_probe::reset();
        assert!(
            !convert_type(&env, &ctx, &lhs, &rhs),
            "a closed Nat ι must not make distinct neutral recursion equal"
        );
        assert!(
            delta_probe::captures() >= 1,
            "the pair must reach the δ-ledger"
        );
        assert!(
            delta_probe::iotas() >= 1,
            "the closed Nat scrutinee must ι-reduce"
        );
        assert_eq!(
            delta_probe::refusals(),
            1,
            "the recurrent hard pair must refuse"
        );
    }

    /// Durable invariant, `17 §3.5`: an unexecuted recursive call in an
    /// eliminator's scrutinee remains a symbolic cross-identity boundary.
    #[test]
    fn recursive_call_in_scrutinee_under_closed_iota_refuses() {
        let mut env = GlobalEnv::new();
        let (bool_id, _, true_id) = declare_bool(&mut env);
        let (nat, zero_id, suc_id) = declare_nat_for_iota(&mut env);
        let c =
            closed_iota_recursive_scrutinee_pair(&mut env, nat, zero_id, suc_id, bool_id, true_id);
        let d =
            closed_iota_recursive_scrutinee_pair(&mut env, nat, zero_id, suc_id, bool_id, true_id);
        assert_ne!(c, d);
        let mut ctx = Context::new();
        ctx.push(Term::indformer(nat, vec![]));
        let lhs = Term::app(cref0(c), Term::var(0));
        let rhs = Term::app(cref0(d), Term::var(0));
        assert_typed(&env, &ctx, &lhs);
        assert_typed(&env, &ctx, &rhs);
        delta_probe::reset();
        assert!(!convert_type(&env, &ctx, &lhs, &rhs));
        assert!(
            delta_probe::captures() >= 1,
            "scrutinee calls must enter the ledger"
        );
        assert!(
            delta_probe::iotas() >= 1,
            "closed Bool selector must execute"
        );
        assert!(
            delta_probe::refusals() >= 1,
            "symbolic recursive calls must refuse"
        );
    }

    fn declare_is_even(
        env: &mut GlobalEnv,
        nat: GlobalId,
        bool_id: GlobalId,
        true_id: GlobalId,
        not_id: GlobalId,
    ) -> GlobalId {
        let nt = Term::indformer(nat, vec![]);
        let bt = bool_ty(bool_id);
        declare_recursive_group(
            env,
            vec![(vec![], Term::pi(nt.clone(), bt.clone()))],
            |ids| {
                let recursive_call = Term::app(cref0(ids[0]), Term::var(1));
                let step = Term::lam(
                    nt.clone(),
                    Term::lam(bt.clone(), Term::app(cref0(not_id), recursive_call)),
                );
                vec![Term::lam(
                    nt.clone(),
                    Term::Elim {
                        fam: nat,
                        level_args: vec![],
                        params: vec![],
                        motive: Box::new(Term::Ascript(
                            Box::new(Term::lam(nt.clone(), bt.clone())),
                            Box::new(Term::pi(nt.clone(), Term::Type(Level::zero()))),
                        )),
                        methods: vec![bool_ctor(true_id), step],
                        indices: vec![],
                        scrut: Box::new(Term::var(0)),
                    },
                )]
            },
        )
        .expect("each isEven twin must be SCT-admitted")[0]
    }

    fn nat_value(zero_id: GlobalId, suc_id: GlobalId, n: usize) -> Term {
        (0..n).fold(Term::constructor(zero_id, vec![]), |prev, _| {
            Term::app(Term::constructor(suc_id, vec![]), prev)
        })
    }

    /// Durable invariant, `17 §3.5`: canonical recursive inputs compute to a
    /// shared Bool, but a neutral input never identifies distinct recursive
    /// GlobalIds through an unexecuted method.
    #[test]
    fn is_even_closed_three_converges_but_neutral_twin_refuses() {
        let mut env = GlobalEnv::new();
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let (nat, zero_id, suc_id) = declare_nat_for_iota(&mut env);
        let not_id = declare_not(&mut env, bool_id, false_id, true_id);
        let c = declare_is_even(&mut env, nat, bool_id, true_id, not_id);
        let d = declare_is_even(&mut env, nat, bool_id, true_id, not_id);
        assert_ne!(c, d);
        for id in [c, d] {
            assert_eq!(
                env.sct_decreasing_positions(id),
                Some(&std::collections::BTreeSet::from([0usize]))
            );
        }

        let three = nat_value(zero_id, suc_id, 3);
        let closed_lhs = Term::app(cref0(c), three.clone());
        let closed_rhs = Term::app(cref0(d), three);
        let empty = Context::new();
        assert_typed(&env, &empty, &closed_lhs);
        assert_typed(&env, &empty, &closed_rhs);
        assert_eq!(whnf(&env, &empty, &closed_lhs), bool_ctor(false_id));
        assert_eq!(whnf(&env, &empty, &closed_rhs), bool_ctor(false_id));
        assert!(
            convert_type(&env, &empty, &closed_lhs, &closed_rhs),
            "honest canonical descent to equal Bool constructors must convert"
        );

        let mut open = Context::new();
        open.push(Term::indformer(nat, vec![]));
        let open_lhs = Term::app(cref0(c), Term::var(0));
        let open_rhs = Term::app(cref0(d), Term::var(0));
        assert_typed(&env, &open, &open_lhs);
        assert_typed(&env, &open, &open_rhs);
        delta_probe::reset();
        assert!(
            !convert_type(&env, &open, &open_lhs, &open_rhs),
            "neutral twin recursion cannot use ι as an equality hypothesis"
        );
        assert!(
            delta_probe::captures() >= 1,
            "neutral twins must reach the ledger"
        );
        assert!(
            delta_probe::refusals() >= 1,
            "neutral recurrence must refuse"
        );
    }

    fn declare_mutual_even_odd(
        env: &mut GlobalEnv,
        nat: GlobalId,
        bool_id: GlobalId,
        false_id: GlobalId,
        true_id: GlobalId,
    ) -> (GlobalId, GlobalId) {
        let nt = Term::indformer(nat, vec![]);
        let bt = bool_ty(bool_id);
        let ty = Term::pi(nt.clone(), bt.clone());
        let ids = declare_recursive_group(env, vec![(vec![], ty.clone()), (vec![], ty)], |ids| {
            let body = |callee: GlobalId, base: GlobalId| {
                let method = Term::lam(
                    nt.clone(),
                    Term::lam(bt.clone(), Term::app(cref0(callee), Term::var(1))),
                );
                Term::lam(
                    nt.clone(),
                    Term::Elim {
                        fam: nat,
                        level_args: vec![],
                        params: vec![],
                        motive: Box::new(Term::Ascript(
                            Box::new(Term::lam(nt.clone(), bt.clone())),
                            Box::new(Term::pi(nt.clone(), Term::Type(Level::zero()))),
                        )),
                        methods: vec![bool_ctor(base), method],
                        indices: vec![],
                        scrut: Box::new(Term::var(0)),
                    },
                )
            };
            vec![body(ids[1], true_id), body(ids[0], false_id)]
        })
        .expect("mutual even/odd must be admitted as one SCT group");
        (ids[0], ids[1])
    }

    /// Durable invariant, `17 §3.5`: distinct mutual SCCs must not acquire a
    /// cyclic cross-group equality through their unexecuted step methods.
    #[test]
    fn mutual_even_odd_neutral_twins_refuse_without_affecting_closed_three() {
        let mut env = GlobalEnv::new();
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let (nat, zero_id, suc_id) = declare_nat_for_iota(&mut env);
        let (even_a, odd_a) = declare_mutual_even_odd(&mut env, nat, bool_id, false_id, true_id);
        let (even_b, odd_b) = declare_mutual_even_odd(&mut env, nat, bool_id, false_id, true_id);
        assert_ne!(even_a, even_b);
        assert_ne!(odd_a, odd_b);
        for id in [even_a, odd_a, even_b, odd_b] {
            assert!(
                env.is_recursive_transparent(id),
                "every mutual member is recursive"
            );
            assert_eq!(
                env.sct_decreasing_positions(id),
                Some(&std::collections::BTreeSet::from([0usize]))
            );
        }
        let empty = Context::new();
        let three = nat_value(zero_id, suc_id, 3);
        let closed_a = Term::app(cref0(even_a), three.clone());
        let closed_b = Term::app(cref0(even_b), three);
        assert_typed(&env, &empty, &closed_a);
        assert_typed(&env, &empty, &closed_b);
        assert_eq!(whnf(&env, &empty, &closed_a), bool_ctor(false_id));
        assert_eq!(whnf(&env, &empty, &closed_b), bool_ctor(false_id));
        assert!(convert_type(&env, &empty, &closed_a, &closed_b));

        let mut open = Context::new();
        open.push(Term::indformer(nat, vec![]));
        let open_a = Term::app(cref0(even_a), Term::var(0));
        let open_b = Term::app(cref0(even_b), Term::var(0));
        assert_typed(&env, &open, &open_a);
        assert_typed(&env, &open, &open_b);
        delta_probe::reset();
        assert!(!convert_type(&env, &open, &open_a, &open_b));
        assert!(
            delta_probe::captures() >= 1,
            "mutual comparison must reach the ledger"
        );
        assert!(
            delta_probe::refusals() >= 1,
            "neutral mutual recurrence must refuse"
        );
    }

    /// A locally bound Var(0) is not a fresh context variable. Each binder
    /// must shift the fresh-window test; the outer Var(0), seen as Var(1)
    /// inside that binder, still counts.
    #[test]
    fn fresh_binder_scan_distinguishes_local_and_context_variables() {
        let nt = Term::Type(Level::zero());
        assert!(!mentions_var_below(&Term::lam(nt.clone(), Term::var(0)), 1));
        assert!(mentions_var_below(&Term::lam(nt.clone(), Term::var(1)), 1));
        assert!(!mentions_var_below(&Term::pi(nt.clone(), Term::var(0)), 1));
        assert!(mentions_var_below(
            &Term::sigma(nt.clone(), Term::var(1)),
            1
        ));
        let local_let = Term::Let {
            ty: Box::new(nt.clone()),
            val: Box::new(nt),
            body: Box::new(Term::var(0)),
        };
        assert!(!mentions_var_below(&local_let, 1));
    }

    /// A recursive List-to-Decoder-shaped function: its result type is a
    /// transparent alias to `Nat → Bool`, so SCT's declared arity is two.
    /// Only its List argument strictly decreases. Its cursor is a fresh
    /// method binder in the neutral Nat elimination, but does not decrease.
    fn declare_list_decoder_twin(
        env: &mut GlobalEnv,
        list: GlobalId,
        nat: GlobalId,
        bool_id: GlobalId,
        true_id: GlobalId,
        alias: GlobalId,
    ) -> GlobalId {
        let nt = Term::indformer(nat, vec![]);
        let bt = bool_ty(bool_id);
        let list_nat = list_at(list, zero(), nt.clone());
        let ty = Term::pi(list_nat.clone(), cref0(alias));
        let id = declare_recursive_group(env, vec![(vec![], ty)], |ids| {
            let nat_motive = Term::Ascript(
                Box::new(Term::lam(nt.clone(), bt.clone())),
                Box::new(Term::pi(nt.clone(), type0())),
            );
            // In the Suc method: ihNat=0, pred=1, cur=2, ihList=3,
            // tail=4, head=5, xs=6. The recursive List argument is tail;
            // the returned function takes the fresh Nat predecessor.
            let call = Term::app(Term::app(cref0(ids[0]), Term::var(4)), Term::var(1));
            let nat_step = Term::lam(nt.clone(), Term::lam(bt.clone(), call));
            let on_cursor = Term::Elim {
                fam: nat,
                level_args: vec![],
                params: vec![],
                motive: Box::new(nat_motive),
                methods: vec![bool_ctor(true_id), nat_step],
                indices: vec![],
                scrut: Box::new(Term::var(0)),
            };
            let cons_method = Term::lam(
                nt.clone(),
                Term::lam(
                    list_nat.clone(),
                    Term::lam(cref0(alias), Term::lam(nt.clone(), on_cursor)),
                ),
            );
            let list_motive = Term::Ascript(
                Box::new(Term::lam(list_nat.clone(), cref0(alias))),
                Box::new(Term::pi(list_nat.clone(), type0())),
            );
            vec![Term::lam(
                list_nat.clone(),
                Term::Elim {
                    fam: list,
                    level_args: vec![zero()],
                    params: vec![nt.clone()],
                    motive: Box::new(list_motive),
                    methods: vec![Term::lam(nt.clone(), bool_ctor(true_id)), cons_method],
                    indices: vec![],
                    scrut: Box::new(Term::var(0)),
                },
            )]
        })
        .expect("checked decoder-shaped recursive definition")[0];
        assert_eq!(
            env.sct_decreasing_positions(id),
            Some(&std::collections::BTreeSet::from([0usize])),
            "SCT must certify descent only through the List argument"
        );
        id
    }

    /// Durable invariant (`17 §3.5`): a fresh argument to the returned
    /// function must not be mistaken for the List parameter that SCT proves
    /// decreases. Reuse the SAME pair of checked definitions for the two
    /// controls: closed List ι accepts, neutral List binder refuses.
    #[test]
    fn closed_list_decoder_accepts_fresh_cursor_but_neutral_tail_refuses() {
        let mut env = GlobalEnv::new();
        let (list, nil, cons) = declare_list(&mut env);
        let (nat, zero_id, _) = declare_nat_for_iota(&mut env);
        let (bool_id, _, true_id) = declare_bool(&mut env);
        let nt = Term::indformer(nat, vec![]);
        let list_nat = list_at(list, zero(), nt.clone());
        let alias = declare_def(
            &mut env,
            vec![],
            type0(),
            Term::pi(nt.clone(), bool_ty(bool_id)),
        )
        .expect("checked alias to returned decoder function");
        let f = declare_list_decoder_twin(&mut env, list, nat, bool_id, true_id, alias);
        let g = declare_list_decoder_twin(&mut env, list, nat, bool_id, true_id, alias);
        let call = |id, xs, cur| Term::app(Term::app(cref0(id), xs), cur);

        let nil = nil_val(nil, nt.clone());
        let closed_tail = cons_val(cons, nt.clone(), Term::constructor(zero_id, vec![]), nil);
        let closed_list = cons_val(
            cons,
            nt.clone(),
            Term::constructor(zero_id, vec![]),
            closed_tail,
        );
        let mut cursor_ctx = Context::new();
        cursor_ctx.push(nt.clone());
        let positive_lhs = call(f, closed_list.clone(), Term::var(0));
        let positive_rhs = call(g, closed_list, Term::var(0));
        assert_typed(&env, &cursor_ctx, &positive_lhs);
        assert_typed(&env, &cursor_ctx, &positive_rhs);
        delta_probe::reset();
        assert!(
            convert_type(&env, &cursor_ctx, &positive_lhs, &positive_rhs),
            "a fresh cursor outside SCT's decreasing List position must convert"
        );
        assert!(delta_probe::iotas() >= 1, "the closed List must execute ι");
        assert!(
            delta_probe::hard_continues() >= 1,
            "SCT non-symbolic hard arm must fire"
        );
        assert_eq!(
            delta_probe::refusals(),
            0,
            "no hard or soft refusal on the positive"
        );

        let mut neutral_ctx = Context::new();
        neutral_ctx.push(list_nat);
        neutral_ctx.push(nt);
        let negative_lhs = call(f, Term::var(1), Term::var(0));
        let negative_rhs = call(g, Term::var(1), Term::var(0));
        assert_typed(&env, &neutral_ctx, &negative_lhs);
        assert_typed(&env, &neutral_ctx, &negative_rhs);
        delta_probe::reset();
        assert!(!convert_type(
            &env,
            &neutral_ctx,
            &negative_lhs,
            &negative_rhs
        ));
        assert_eq!(
            delta_probe::refusals(),
            1,
            "the method's fresh List tail must trip exactly the hard refusal"
        );
    }

    /// The function type's second Π is hidden behind a checked alias, and
    /// the body exposes only one leading λ before a let. SCT eta-expands it,
    /// still recording parameter 1 (the Nat cursor) as the strict diagonal.
    fn declare_eta_decoder_twin(
        env: &mut GlobalEnv,
        nat: GlobalId,
        bool_id: GlobalId,
        true_id: GlobalId,
        alias: GlobalId,
    ) -> GlobalId {
        let nt = Term::indformer(nat, vec![]);
        let bt = bool_ty(bool_id);
        let id = declare_recursive_group(
            env,
            vec![(vec![], Term::pi(nt.clone(), cref0(alias)))],
            |ids| {
                let call = Term::app(Term::app(cref0(ids[0]), Term::var(3)), Term::var(1));
                let step = Term::lam(nt.clone(), Term::lam(bt.clone(), call));
                let cursor_body = Term::Elim {
                    fam: nat,
                    level_args: vec![],
                    params: vec![],
                    motive: Box::new(Term::Ascript(
                        Box::new(Term::lam(nt.clone(), bt.clone())),
                        Box::new(Term::pi(nt.clone(), type0())),
                    )),
                    methods: vec![bool_ctor(true_id), step],
                    indices: vec![],
                    scrut: Box::new(Term::var(0)),
                };
                vec![Term::lam(
                    nt.clone(),
                    Term::Let {
                        ty: Box::new(cref0(alias)),
                        val: Box::new(Term::lam(nt.clone(), cursor_body)),
                        body: Box::new(Term::var(0)),
                    },
                )]
            },
        )
        .expect("SCT admits a one-lambda body at two declared positions")[0];
        assert_eq!(
            env.sct_decreasing_positions(id),
            Some(&std::collections::BTreeSet::from([1usize])),
            "eta-canonical SCT must include the beyond-leading-lambda descent"
        );
        id
    }

    /// Durable invariant: the body-lambda shortcut would miss SCT's second
    /// parameter and let a fresh Nat predecessor pass a hard origin.
    #[test]
    fn eta_expanded_decreasing_cursor_still_refuses_fresh_predecessor() {
        let mut env = GlobalEnv::new();
        let (nat, zero_id, _) = declare_nat_for_iota(&mut env);
        let (bool_id, _, true_id) = declare_bool(&mut env);
        let nt = Term::indformer(nat, vec![]);
        let alias = declare_def(
            &mut env,
            vec![],
            type0(),
            Term::pi(nt.clone(), bool_ty(bool_id)),
        )
        .expect("checked alias to second Nat argument");
        let f = declare_eta_decoder_twin(&mut env, nat, bool_id, true_id, alias);
        let g = declare_eta_decoder_twin(&mut env, nat, bool_id, true_id, alias);
        let zero = Term::constructor(zero_id, vec![]);
        let mut ctx = Context::new();
        ctx.push(nt);
        let lhs = Term::app(Term::app(cref0(f), zero.clone()), Term::var(0));
        let rhs = Term::app(Term::app(cref0(g), zero), Term::var(0));
        assert_typed(&env, &ctx, &lhs);
        assert_typed(&env, &ctx, &rhs);
        delta_probe::reset();
        assert!(!convert_type(&env, &ctx, &lhs, &rhs));
        assert_eq!(
            delta_probe::refusals(),
            1,
            "fresh predecessor is SCT parameter 1"
        );
    }

    /// A rolled-back recursive body loses its SCT diagonal certificate;
    /// reusing the same GlobalId for an acyclic checked definition cannot
    /// inherit that earlier proof. The new body's D is empty, not missing.
    #[test]
    fn sct_decreasing_positions_clear_on_rollback_and_id_reuse() {
        let mut env = GlobalEnv::new();
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let (nat, _, _) = declare_nat_for_iota(&mut env);
        let not_id = declare_not(&mut env, bool_id, false_id, true_id);
        let recursive = declare_is_even(&mut env, nat, bool_id, true_id, not_id);
        assert_eq!(
            env.sct_decreasing_positions(recursive),
            Some(&std::collections::BTreeSet::from([0usize]))
        );
        assert_eq!(env.remove_last().map(|decl| decl.id()), Some(recursive));
        assert_eq!(env.sct_decreasing_positions(recursive), None);

        let nt = Term::indformer(nat, vec![]);
        let nonrecursive = declare_def(
            &mut env,
            vec![],
            Term::pi(nt.clone(), bool_ty(bool_id)),
            Term::lam(nt, bool_ctor(true_id)),
        )
        .expect("checked acyclic definition reuses the released id");
        assert_eq!(nonrecursive, recursive);
        assert_eq!(
            env.sct_decreasing_positions(nonrecursive),
            Some(&std::collections::BTreeSet::new())
        );
        assert!(!env.is_recursive_transparent(nonrecursive));
    }

    fn literal_is_even_body(env: &GlobalEnv, id: GlobalId, input: &Term) -> Term {
        let (_, body) = env.transparent_body(id).expect("checked isEven body");
        let Term::Lam(_, body) = body else {
            panic!("isEven definition must be a lambda");
        };
        subst0(&body, input)
    }

    fn bool_elim_on_scrut(
        bool_id: GlobalId,
        false_id: GlobalId,
        true_id: GlobalId,
        scrut: Term,
    ) -> Term {
        let bt = bool_ty(bool_id);
        Term::Elim {
            fam: bool_id,
            level_args: vec![],
            params: vec![],
            motive: Box::new(Term::Ascript(
                Box::new(Term::lam(bt.clone(), bt.clone())),
                Box::new(Term::pi(bt, Term::Type(Level::zero()))),
            )),
            methods: vec![bool_ctor(false_id), bool_ctor(true_id)],
            indices: vec![],
            scrut: Box::new(scrut),
        }
    }

    /// Durable invariant, `17 §3.3`/§3.5: a deferred scrutinee must still
    /// compare equal to its literal δ-unfolding, not become syntactically rigid.
    /// This pin uses the checked declaration's stored body as the independent
    /// expected constructor, not the conversion reducer's output.
    #[test]
    fn stuck_bool_elim_compares_with_literal_is_even_delta_unfolding() {
        let mut env = GlobalEnv::new();
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let (nat, _, _) = declare_nat_for_iota(&mut env);
        let not_id = declare_not(&mut env, bool_id, false_id, true_id);
        let a = declare_is_even(&mut env, nat, bool_id, true_id, not_id);
        let mut ctx = Context::new();
        ctx.push(Term::indformer(nat, vec![]));
        let folded = Term::app(cref0(a), Term::var(0));
        let literal = literal_is_even_body(&env, a, &Term::var(0));
        assert_ne!(folded, literal, "δ must visibly change this scrutinee");
        assert_typed(&env, &ctx, &folded);
        assert_typed(&env, &ctx, &literal);
        let lhs = bool_elim_on_scrut(bool_id, false_id, true_id, folded.clone());
        let rhs = bool_elim_on_scrut(bool_id, false_id, true_id, literal.clone());
        assert_typed(&env, &ctx, &lhs);
        assert_typed(&env, &ctx, &rhs);
        delta_probe::reset();
        assert!(
            convert_type(&env, &ctx, &lhs, &rhs),
            "a stuck folded recursive scrutinee converts to its literal δ body"
        );
        assert!(
            delta_probe::unfolds() >= 1,
            "comparison must retry δ on demand"
        );

        // Public whnf retains the historical eager stuck-eliminator rebuild;
        // only the private conversion reducer may leave `folded` in place.
        let eager = whnf(&env, &ctx, &lhs);
        let Term::Elim { fam, scrut, .. } = eager else {
            panic!("open Bool elimination must remain stuck");
        };
        assert_eq!(fam, bool_id);
        assert_eq!(scrut.as_ref(), &literal);
        assert_ne!(scrut.as_ref(), &folded);
    }

    /// Durable invariant: an acyclic transparent scrutinee still computes to
    /// a constructor and its elimination performs ι, including in conversion.
    #[test]
    fn nonrecursive_transparent_bool_scrutinee_still_iota_reduces() {
        let mut env = GlobalEnv::new();
        let (bool_id, false_id, true_id) = declare_bool(&mut env);
        let bt = bool_ty(bool_id);
        let constant = declare_def(&mut env, vec![], bt, bool_ctor(true_id))
            .expect("checked nonrecursive Bool constant");
        let source = bool_elim_on_scrut(bool_id, false_id, true_id, cref0(constant));
        let expected = bool_ctor(true_id);
        let ctx = Context::new();
        assert_typed(&env, &ctx, &source);
        assert_eq!(whnf(&env, &ctx, &source), expected);
        delta_probe::reset();
        assert!(convert_type(&env, &ctx, &source, &expected));
        assert!(
            delta_probe::iotas() >= 1,
            "real constructor selection must run"
        );
        assert_eq!(normalize(&env, &ctx, &source), expected);
    }

    /// Durable invariant: when projection-β cannot fire, conversion retains
    /// a checked transparent alias as a deferred projectee, while public whnf
    /// still exposes its opaque target. Both projection arms are exercised.
    #[test]
    fn stuck_projections_defer_nested_delta_but_public_whnf_does_not() {
        let mut env = GlobalEnv::new();
        let (nat, _, _) = declare_nat_for_iota(&mut env);
        let nt = Term::indformer(nat, vec![]);
        let pair_ty = Term::sigma(nt.clone(), nt);
        let opaque = declare_postulate(&mut env, "opaque_pair".into(), vec![], pair_ty.clone())
            .expect("opaque pair declaration");
        let alias =
            declare_def(&mut env, vec![], pair_ty, cref0(opaque)).expect("checked pair alias");
        let ctx = Context::new();
        for projection in [Term::proj1 as fn(Term) -> Term, Term::proj2] {
            let source = projection(cref0(alias));
            let unfolded = projection(cref0(opaque));
            assert_typed(&env, &ctx, &source);
            assert_typed(&env, &ctx, &unfolded);
            assert_ne!(source, unfolded);
            assert_eq!(whnf_progress_for_conversion(&env, &ctx, &source).0, source);
            assert_eq!(whnf(&env, &ctx, &source), unfolded);
            assert!(convert_type(&env, &ctx, &source, &unfolded));
        }
    }

    /// Durable invariant: Eq at an opaque type and Cast between opaque types
    /// cannot commit nested δ unless their observational reduction fires.
    /// This compares the conversion-only stuck rebuild to the eager public one
    /// for both type-observation arms independently.
    #[test]
    fn stuck_eq_and_cast_types_defer_nested_delta() {
        let mut env = GlobalEnv::new();
        let ty = Term::Type(Level::zero());
        let opaque_a =
            declare_postulate(&mut env, "opaque_a".into(), vec![], ty.clone()).expect("opaque A");
        let opaque_b =
            declare_postulate(&mut env, "opaque_b".into(), vec![], ty.clone()).expect("opaque B");
        let alias_a =
            declare_def(&mut env, vec![], ty.clone(), cref0(opaque_a)).expect("checked A alias");
        let alias_b =
            declare_def(&mut env, vec![], ty.clone(), cref0(opaque_b)).expect("checked B alias");
        let x = declare_postulate(&mut env, "x".into(), vec![], cref0(opaque_a))
            .expect("opaque A value");
        let proof_ty = Term::Eq(
            Box::new(ty),
            Box::new(cref0(opaque_a)),
            Box::new(cref0(opaque_b)),
        );
        let proof = declare_postulate(&mut env, "cast_proof".into(), vec![], proof_ty)
            .expect("opaque cast certificate");
        let ctx = Context::new();
        let eq_alias = Term::Eq(
            Box::new(cref0(alias_a)),
            Box::new(cref0(x)),
            Box::new(cref0(x)),
        );
        let eq_opaque = Term::Eq(
            Box::new(cref0(opaque_a)),
            Box::new(cref0(x)),
            Box::new(cref0(x)),
        );
        assert_typed(&env, &ctx, &eq_alias);
        assert_eq!(
            whnf_progress_for_conversion(&env, &ctx, &eq_alias).0,
            eq_alias
        );
        assert_eq!(whnf(&env, &ctx, &eq_alias), eq_opaque);
        assert!(convert_type(&env, &ctx, &eq_alias, &eq_opaque));

        let cast_alias = Term::Cast(
            Box::new(cref0(alias_a)),
            Box::new(cref0(alias_b)),
            Box::new(cref0(proof)),
            Box::new(cref0(x)),
        );
        let cast_opaque = Term::Cast(
            Box::new(cref0(opaque_a)),
            Box::new(cref0(opaque_b)),
            Box::new(cref0(proof)),
            Box::new(cref0(x)),
        );
        assert_typed(&env, &ctx, &cast_alias);
        assert_eq!(
            whnf_progress_for_conversion(&env, &ctx, &cast_alias).0,
            cast_alias
        );
        assert_eq!(whnf(&env, &ctx, &cast_alias), cast_opaque);
        assert!(convert_type(&env, &ctx, &cast_alias, &cast_opaque));
    }

    /// Durable invariant: a checked quotient eliminator with an opaque
    /// scrutinee does not consume its checked alias's δ-unfolding merely by
    /// staying stuck. Public whnf remains eager on that same scrutinee.
    #[test]
    fn stuck_quot_elim_preserves_deferred_alias_and_eager_public_whnf() {
        let mut env = GlobalEnv::new();
        let (nat, zero_id, _) = declare_nat_for_iota(&mut env);
        let nt = Term::indformer(nat, vec![]);
        let rel_ty = Term::pi(nt.clone(), Term::pi(nt.clone(), Term::Omega(Level::zero())));
        let rel = declare_postulate(&mut env, "quot_relation".into(), vec![], rel_ty)
            .expect("opaque quotient relation");
        let quot_ty = Term::Quot(Box::new(nt.clone()), Box::new(cref0(rel)));
        let q = declare_postulate(&mut env, "opaque_quot".into(), vec![], quot_ty.clone())
            .expect("opaque quotient value");
        let alias = declare_def(&mut env, vec![], quot_ty.clone(), cref0(q))
            .expect("checked quotient alias");
        let motive = declare_def(
            &mut env,
            vec![],
            Term::pi(quot_ty.clone(), Term::Type(Level::zero())),
            Term::lam(quot_ty, nt.clone()),
        )
        .expect("checked constant quotient motive");
        let zero = Term::constructor(zero_id, vec![]);
        let method = declare_def(
            &mut env,
            vec![],
            Term::pi(nt.clone(), nt.clone()),
            Term::lam(nt.clone(), zero),
        )
        .expect("checked constant quotient method");
        let h_ty = Term::app(Term::app(cref0(rel), Term::var(1)), Term::var(0));
        let respect = Term::lam(
            nt.clone(),
            Term::lam(nt.clone(), Term::lam(h_ty, cref0(env.tt_id()))),
        );
        let elim = |scrut| Term::QuotElim {
            motive: Box::new(cref0(motive)),
            method: Box::new(cref0(method)),
            respect: Box::new(respect.clone()),
            scrut: Box::new(scrut),
        };
        let source = elim(cref0(alias));
        let unfolded = elim(cref0(q));
        let ctx = Context::new();
        assert_typed(&env, &ctx, &source);
        assert_typed(&env, &ctx, &unfolded);
        assert_eq!(whnf_progress_for_conversion(&env, &ctx, &source).0, source);
        assert_eq!(whnf(&env, &ctx, &source), unfolded);
        assert!(convert_type(&env, &ctx, &source, &unfolded));
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

    /// The K3 view is neutral when a closed ι-redex produces a runtime-only
    /// String operation, even though reducing that *discarded* argument did
    /// perform ι. The no-progress δ-origin ledger must not inherit that event.
    #[test]
    fn k3_closed_iota_to_neutral_argument_does_not_report_iota_progress() {
        use crate::check::{
            declare_checked_string_literal, declare_primitive, register_checked_char_carrier,
            register_checked_string_carrier, register_literal_char_view,
        };
        use crate::env::PrimReduction;
        let mut env = GlobalEnv::new();
        let int = declare_primitive(&mut env, vec![], type0(), PrimReduction::OpaqueType).unwrap();
        env.register_int_lit_type(int);
        let char_id = declare_def(&mut env, vec![], type0(), cref0(int)).unwrap();
        register_checked_char_carrier(&mut env, char_id).unwrap();
        let string_id =
            declare_primitive(&mut env, vec![], type0(), PrimReduction::OpaqueType).unwrap();
        register_checked_string_carrier(&mut env, string_id).unwrap();
        let str_ty = cref0(string_id);
        let char_ty = cref0(char_id);
        let list = declare_inductive(&mut env, |list| InductiveSpec {
            level_params: vec![],
            params: vec![type0()],
            indices: vec![],
            level: zero(),
            constructors: vec![
                CtorSpec {
                    args: vec![],
                    target_indices: vec![],
                },
                CtorSpec {
                    args: vec![
                        Term::var(0),
                        Term::app(Term::indformer(list, vec![]), Term::var(1)),
                    ],
                    target_indices: vec![],
                },
            ],
        })
        .unwrap();
        let nil = env.inductive(list).unwrap().constructors[0].id;
        let cons = env.inductive(list).unwrap().constructors[1].id;
        let list_char = Term::app(Term::indformer(list, vec![]), char_ty.clone());
        let view = declare_primitive(
            &mut env,
            vec![],
            Term::pi(str_ty.clone(), list_char.clone()),
            PrimReduction::Op {
                symbol: "string_to_list_char",
            },
        )
        .unwrap();
        let inverse = declare_primitive(
            &mut env,
            vec![],
            Term::pi(list_char.clone(), str_ty.clone()),
            PrimReduction::Op {
                symbol: "list_char_to_string",
            },
        )
        .unwrap();
        register_literal_char_view(&mut env, string_id, char_id, view, list, nil, cons).unwrap();
        let (bool_id, _, true_id) = declare_bool(&mut env);
        let bool_type = bool_ty(bool_id);
        let neutral_string = Term::app(
            cref0(inverse),
            Term::app(Term::constructor(nil, vec![]), char_ty.clone()),
        );
        let motive = Term::Ascript(
            Box::new(Term::lam(bool_type.clone(), str_ty)),
            Box::new(Term::pi(bool_type, type0())),
        );
        let iota_to_neutral = Term::Elim {
            fam: bool_id,
            level_args: vec![],
            params: vec![],
            motive: Box::new(motive),
            methods: vec![neutral_string.clone(), neutral_string],
            indices: vec![],
            scrut: Box::new(bool_ctor(true_id)),
        };
        let stuck_view = Term::app(cref0(view), iota_to_neutral);
        assert_typed(&env, &Context::new(), &stuck_view);
        let (reduct, progress) = whnf_progress(&env, &Context::new(), &stuck_view);
        assert_eq!(
            reduct, stuck_view,
            "the closed ι argument is discarded on a neutral result"
        );
        assert!(
            !progress.iota,
            "discarded argument ι cannot discharge a δ-origin"
        );

        // A positive through the *same* ι construction: when the selected
        // method returns an admitted literal, the K3 operation consumes it.
        // That ι event legitimately propagates and the result is checked.
        let literal = declare_checked_string_literal(&mut env, "Az").unwrap();
        let checked_iota = Term::Elim {
            fam: bool_id,
            level_args: vec![],
            params: vec![],
            motive: Box::new(Term::Ascript(
                Box::new(Term::lam(bool_ty(bool_id), cref0(string_id))),
                Box::new(Term::pi(bool_ty(bool_id), type0())),
            )),
            methods: vec![cref0(literal), cref0(literal)],
            indices: vec![],
            scrut: Box::new(bool_ctor(true_id)),
        };
        let positive = Term::app(cref0(view), checked_iota);
        assert_typed(&env, &Context::new(), &positive);
        let (list_result, positive_progress) = whnf_progress(&env, &Context::new(), &positive);
        assert!(positive_progress.iota, "consumed argument ι must propagate");
        crate::check::check(&env, &Context::new(), &list_result, &list_char).unwrap();
        let expected = Term::app(
            Term::app(
                Term::app(Term::constructor(cons, vec![]), char_ty.clone()),
                Term::IntLit(65u32.into()),
            ),
            Term::app(
                Term::app(
                    Term::app(Term::constructor(cons, vec![]), char_ty.clone()),
                    Term::IntLit(122u32.into()),
                ),
                Term::app(Term::constructor(nil, vec![]), char_ty.clone()),
            ),
        );
        assert_eq!(list_result, expected, "NFC view keeps source scalar order");

        // Two distinct SCT-admitted recursive definitions on the same List
        // return a recurring δ-origin pair when the scrutinee is neutral.
        // Unlike a synthetic primed ledger, both sides here are closed and
        // independently kernel-typed before conversion.
        let declare_self_map = |env: &mut GlobalEnv| {
            let list_char = list_char.clone();
            let char_ty = char_ty.clone();
            declare_recursive_group(
                env,
                vec![(vec![], Term::pi(list_char.clone(), list_char.clone()))],
                |ids| {
                    let recur = Term::app(cref0(ids[0]), Term::var(1));
                    let cons_result = Term::app(
                        Term::app(
                            Term::app(Term::constructor(cons, vec![]), char_ty.clone()),
                            Term::var(2),
                        ),
                        recur,
                    );
                    let method = Term::lam(
                        char_ty.clone(),
                        Term::lam(list_char.clone(), Term::lam(list_char.clone(), cons_result)),
                    );
                    let motive = Term::Ascript(
                        Box::new(Term::lam(list_char.clone(), list_char.clone())),
                        Box::new(Term::pi(list_char.clone(), type0())),
                    );
                    vec![Term::lam(
                        list_char.clone(),
                        Term::Elim {
                            fam: list,
                            level_args: vec![],
                            params: vec![char_ty.clone()],
                            motive: Box::new(motive),
                            methods: vec![
                                Term::app(Term::constructor(nil, vec![]), char_ty),
                                method,
                            ],
                            indices: vec![],
                            scrut: Box::new(Term::var(0)),
                        },
                    )]
                },
            )
            .expect("self-map must pass SCT")[0]
        };
        let f = declare_self_map(&mut env);
        let g = declare_self_map(&mut env);
        assert_ne!(f, g);
        let lhs = Term::app(cref0(f), stuck_view.clone());
        let rhs = Term::app(cref0(g), stuck_view);
        assert_typed(&env, &Context::new(), &lhs);
        assert_typed(&env, &Context::new(), &rhs);
        delta_probe::reset();
        assert!(!convert_type(&env, &Context::new(), &lhs, &rhs));
        assert!(
            delta_probe::captures() >= 1,
            "distinct recursive δ origins must be reached"
        );
        assert!(
            delta_probe::iotas() >= 1,
            "the discarded String argument really performs ι"
        );
        assert_eq!(
            delta_probe::refusals(),
            1,
            "a discarded argument ι must not erase the recurring no-progress pair"
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
        let lhs = map_apply(
            map_f,
            bt.clone(),
            bt.clone(),
            cref0(not),
            nil_val(nil, bt.clone()),
        );
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
