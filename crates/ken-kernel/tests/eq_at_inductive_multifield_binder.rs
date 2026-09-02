//! Direct kernel control for the `obs.rs::eq_at_inductive` multi-field
//! binder-hygiene repair (LANG-RECORD-INDEX-REFINEMENT-KERNEL).
//!
//! A ≥2-field single-constructor inductive's equality, taken between OPEN
//! endpoints (context vars) under one unrelated trailing context binder,
//! reduces (via the reverse fold `for j in (0..n).rev()`) to a right-nested
//! Sigma whose CODOMAIN is the later field's equality. That codomain is
//! assembled in the outer context and must be WEAKENED past the outer
//! conjunct's proof binder (exactly as `eq_at_sigma` does with
//! `weaken(&second, 1)`). The pre-repair `acc = Term::sigma(conjunct, acc)`
//! (no weaken) let the outer proof binder capture the codomain's field
//! reference — so inferring the reduct failed with a field-type mismatch.
//!
//! - `two_field_...infers`: reflexive equality of `MkPair2 x y` under a
//!   trailing binder — green only with the weakening; captured (rejected) on
//!   the buggy line.
//! - `one_field_...either_way`: a single conjunct has no nested codomain and
//!   stays green in both versions — proving the control reaches the
//!   nested-codomain case, not generic reflexivity, and that a bare/closed
//!   endpoint never enters this reduction.
//! - `two_field_unequal_later_field_...discriminates`: NO-OVERACCEPT. The
//!   repair is a completeness fix, not a soundness relaxation. The REAL
//!   nested-Sigma reduct witness `pair(refl x, tt)` accepts the equal record
//!   and REJECTS the record obtained by varying ONLY the later (codomain)
//!   field — the exact suffix the binder hygiene governs. A witness that
//!   accepted both would be an over-accept.

use ken_kernel::env::Context;
use ken_kernel::obs::tt_term;
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    check, declare_inductive, infer, whnf, CtorSpec, GlobalEnv, GlobalId, InductiveSpec,
};

struct Env {
    nat: GlobalId,
    zero: GlobalId,
    suc: GlobalId,
    unit: GlobalId,
    pair2: GlobalId,
    mk_pair2: GlobalId,
    one: GlobalId,
    mk_one: GlobalId,
}

fn build_env() -> (GlobalEnv, Env) {
    let mut env = GlobalEnv::new();
    let nat = declare_inductive(&mut env, |nat| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![
            CtorSpec { args: vec![], target_indices: vec![] },
            CtorSpec { args: vec![Term::indformer(nat, vec![])], target_indices: vec![] },
        ],
    })
    .expect("Nat");
    let (zero, suc) = {
        let cs = &env.inductive(nat).unwrap().constructors;
        (cs[0].id, cs[1].id)
    };
    let unit = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec { args: vec![], target_indices: vec![] }],
    })
    .expect("Unit");
    let nat_t = Term::indformer(nat, vec![]);
    // Pair2 = MkPair2 (Nat) (Nat) — its reflexive-equality reduct is the nested
    // `Σ (Eq Nat f0 f0) (Eq Nat f1 f1)`: the FIRST field is the outer conjunct,
    // the LATER field the binder-sensitive codomain.
    let pair2 = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![nat_t.clone(), nat_t.clone()],
            target_indices: vec![],
        }],
    })
    .expect("Pair2");
    let mk_pair2 = env.inductive(pair2).unwrap().constructors[0].id;
    let one = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec { args: vec![nat_t.clone()], target_indices: vec![] }],
    })
    .expect("One");
    let mk_one = env.inductive(one).unwrap().constructors[0].id;
    (env, Env { nat, zero, suc, unit, pair2, mk_pair2, one, mk_one })
}

fn nat_t(e: &Env) -> Term {
    Term::indformer(e.nat, vec![])
}
fn zero_t(e: &Env) -> Term {
    Term::constructor(e.zero, vec![])
}
fn suc_t(e: &Env, n: Term) -> Term {
    Term::app(Term::constructor(e.suc, vec![]), n)
}
fn mk_pair2(e: &Env, x: Term, y: Term) -> Term {
    Term::app(Term::app(Term::constructor(e.mk_pair2, vec![]), x), y)
}

#[test]
fn two_field_reflexive_eq_under_trailing_binder_infers_the_nested_proof() {
    let (env, e) = build_env();
    // Context [x : Nat, y : Nat, d : Unit] — x = Var(2), y = Var(1); the
    // unrelated trailing binder d = Var(0) is a DIFFERENT type, so a captured
    // codomain reference (the `y` field) surfaces as a clean type mismatch.
    let mut ctx = Context::new();
    ctx.push(nat_t(&e));
    ctx.push(nat_t(&e));
    ctx.push(Term::indformer(e.unit, vec![]));
    let pair = mk_pair2(&e, Term::var(2), Term::var(1));
    let eq_ty = Term::Eq(
        Box::new(Term::indformer(e.pair2, vec![])),
        Box::new(pair.clone()),
        Box::new(pair),
    );
    let reduced = whnf(&env, &ctx, &eq_ty);
    infer(&env, &ctx, &reduced)
        .expect("the nested per-field equality proof must be well typed under the fix");
}

#[test]
fn one_field_reflexive_eq_stays_well_typed_either_way() {
    let (env, e) = build_env();
    let mut ctx = Context::new();
    ctx.push(nat_t(&e));
    ctx.push(nat_t(&e));
    // x = Var(1); trailing binder d = Var(0).
    let one = Term::app(Term::constructor(e.mk_one, vec![]), Term::var(1));
    let eq_ty = Term::Eq(
        Box::new(Term::indformer(e.one, vec![])),
        Box::new(one.clone()),
        Box::new(one),
    );
    let reduced = whnf(&env, &ctx, &eq_ty);
    infer(&env, &ctx, &reduced)
        .expect("a single-conjunct reflexive equality is well typed in both versions");
}

#[test]
fn two_field_unequal_later_field_witness_discriminates() {
    // NO-OVERACCEPT (discriminating on the CODOMAIN field). The reflexive-equality
    // reduct of `MkPair2 x zero` is `Σ (Eq Nat x x) (Eq Nat zero zero ⇝ Top)`, so
    // `pair(refl x, tt)` inhabits it. The SAME witness must ACCEPT the equal
    // record and REJECT the record obtained by varying ONLY the LATER field
    // (`zero` vs `suc zero`) — the codomain suffix `tt` is asserted against.
    // Equalizing field 1 makes the reject arm accept and reds this test, so the
    // discrimination is on field 1, not a first-field coincidence.
    let (env, e) = build_env();
    let mut ctx = Context::new();
    ctx.push(nat_t(&e)); // x : Nat = Var(1)
    ctx.push(Term::indformer(e.unit, vec![])); // trailing d : Unit = Var(0)
    let pair2_t = Term::indformer(e.pair2, vec![]);
    let lhs = mk_pair2(&e, Term::var(1), zero_t(&e));
    let rhs = mk_pair2(&e, Term::var(1), suc_t(&e, zero_t(&e)));
    let witness = Term::pair(Term::Refl(Box::new(Term::var(1))), tt_term(&env));
    check(
        &env,
        &ctx,
        &witness,
        &Term::Eq(Box::new(pair2_t.clone()), Box::new(lhs.clone()), Box::new(lhs.clone())),
    )
    .expect("the nested-Sigma reduct witness must accept the equal record");
    assert!(
        check(
            &env,
            &ctx,
            &witness,
            &Term::Eq(Box::new(pair2_t), Box::new(lhs), Box::new(rhs)),
        )
        .is_err(),
        "the same witness must reject the record whose later field differs"
    );
}
