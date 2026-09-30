//! P0 subject-reduction fences for `Eq Type` (`16 §2.2`, §3).
//! A generic checked reflexivity lemma must not yield a closed `Bottom` proof
//! merely because its identical type argument has a compound rigid head.

use ken_kernel::env::Context;
use ken_kernel::obs::{bottom_term, top_term};
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    check, declare_inductive, infer, whnf, CtorSpec, GlobalEnv, InductiveSpec, KernelError,
};

fn eq(ty: Term, left: Term, right: Term) -> Term {
    Term::Eq(Box::new(ty), Box::new(left), Box::new(right))
}

fn generic_refl_at(compound: Term) -> (GlobalEnv, Term, Term) {
    let env = GlobalEnv::new();
    let ctx = Context::new();
    let type1 = Term::Type(Level::zero().suc());
    let type2 = Term::Type(Level::zero().suc().suc());
    // T : Type 2; x : T; refl x : Eq T x x. The equality remains neutral
    // under the generic T binder, so this checks before specialization.
    let lemma_ty = Term::pi(
        type2.clone(),
        Term::pi(Term::var(0), eq(Term::var(1), Term::var(0), Term::var(0))),
    );
    let lemma = Term::Ascript(
        Box::new(Term::lam(
            type2,
            Term::lam(Term::var(0), Term::Refl(Box::new(Term::var(0)))),
        )),
        Box::new(lemma_ty.clone()),
    );
    assert_eq!(infer(&env, &ctx, &lemma), Ok(lemma_ty));
    assert_eq!(infer(&env, &ctx, &compound), Ok(type1.clone()));
    let proof = Term::app(Term::app(lemma, type1.clone()), compound.clone());
    let proposition = eq(type1, compound.clone(), compound);
    assert_eq!(infer(&env, &ctx, &proof), Ok(proposition.clone()));
    assert_ne!(proposition, bottom_term(&env));
    // Both Π and Σ witnesses arise from the same checked generic lemma,
    // not a direct Refl fast path at the specialized type.
    (env, proof, proposition)
}

fn assert_closed_proof_does_not_check_as_bottom(compound: Term) {
    let (env, proof, proposition) = generic_refl_at(compound);
    let ctx = Context::new();
    let bottom = bottom_term(&env);
    assert_eq!(
        check(&env, &ctx, &proof, &bottom),
        Err(KernelError::TypeMismatch {
            expected: Box::new(bottom),
            found: Box::new(proposition.clone()),
        }),
        "a closed, well-typed proof cannot be checked against Bottom"
    );
    assert_eq!(
        whnf(&env, &ctx, &proposition),
        proposition,
        "equal compound type heads must remain neutral, not Bottom"
    );
}

/// Durable invariant, AC-1: the closed Π instance of the generic lemma
/// infers an Eq but cannot prove Bottom. MEASURED: closed check outcome;
/// CLAIMED: equal Π/Π type heads do not reduce to falsity; GAP: other rigid
/// formers and open neutral sides have their own independent rows below.
#[test]
fn generic_reflexivity_at_pi_does_not_prove_bottom() {
    let type0 = Term::Type(Level::zero());
    assert_closed_proof_does_not_check_as_bottom(Term::pi(type0.clone(), type0));
}

/// Durable invariant, AC-1: the same closed exploit at Σ/Σ must also fail.
#[test]
fn generic_reflexivity_at_sigma_does_not_prove_bottom() {
    let type0 = Term::Type(Level::zero());
    assert_closed_proof_does_not_check_as_bottom(Term::sigma(type0.clone(), type0));
}

/// Durable invariant, AC-1: a rigid/neutral pair cannot decide inequality.
/// With X := Π Type0.Type0, the assumed Eq is reflexive; the body must not
/// become a proof of Bottom merely because the other head is a Π.
#[test]
fn compound_against_neutral_variable_does_not_prove_bottom() {
    let env = GlobalEnv::new();
    let ctx = Context::new();
    let type0 = Term::Type(Level::zero());
    let type1 = Term::Type(Level::zero().suc());
    let function = Term::pi(type0.clone(), type0);
    let equality = eq(type1.clone(), function, Term::var(0));
    let bottom = bottom_term(&env);
    let claimed = Term::pi(type1.clone(), Term::pi(equality.clone(), bottom.clone()));
    let forged = Term::lam(type1, Term::lam(equality, Term::var(0)));
    assert!(matches!(infer(&env, &ctx, &claimed), Ok(Term::Omega(_))));
    assert!(matches!(
        check(&env, &ctx, &forged, &claimed),
        Err(KernelError::TypeMismatch { expected, .. }) if *expected == bottom
    ));
}

/// AC-2: a genuine mismatch still decides Bottom, and universe comparisons
/// retain both equal and unequal level outcomes. Raw unequal-level pairs are
/// checked as reducer observations, not claimed to be typable Eq redexes.
#[test]
fn distinct_rigid_formers_and_universe_levels_preserve_verdicts() {
    let mut env = GlobalEnv::new();
    let ctx = Context::new();
    let type0 = Term::Type(Level::zero());
    let type1 = Term::Type(Level::zero().suc());
    let function = Term::pi(type0.clone(), type0.clone());
    let product = Term::sigma(type0.clone(), type0.clone());
    let distinct = eq(type1.clone(), type0.clone(), function.clone());
    assert!(
        infer(&env, &ctx, &distinct).is_ok(),
        "rigid mismatch is typed"
    );
    assert_eq!(whnf(&env, &ctx, &distinct), bottom_term(&env));
    let formers = [function, product, Term::Omega(Level::zero()), type0.clone()];
    for (i, left) in formers.iter().enumerate() {
        for right in &formers[i + 1..] {
            assert_eq!(
                whnf(&env, &ctx, &eq(type1.clone(), left.clone(), right.clone())),
                bottom_term(&env),
                "different rigid formers should remain disjoint"
            );
        }
    }
    for (left, right) in [
        (type0.clone(), type0.clone()),
        (Term::Omega(Level::zero()), Term::Omega(Level::zero())),
    ] {
        assert_eq!(
            whnf(&env, &ctx, &eq(type1.clone(), left.clone(), right.clone())),
            top_term(&env)
        );
    }
    for (left, right) in [
        (type0.clone(), type1.clone()),
        (Term::Omega(Level::zero()), Term::Omega(Level::zero().suc())),
    ] {
        assert_eq!(
            whnf(&env, &ctx, &eq(type1.clone(), left, right)),
            bottom_term(&env)
        );
    }

    let make_inductive = |env: &mut GlobalEnv| {
        declare_inductive(env, |_| InductiveSpec {
            level_params: vec![],
            params: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![CtorSpec {
                args: vec![],
                target_indices: vec![],
            }],
        })
        .expect("well-formed nullary family")
    };
    let d1 = Term::indformer(make_inductive(&mut env), vec![]);
    let d2 = Term::indformer(make_inductive(&mut env), vec![]);
    let different_ids = eq(type0.clone(), d1.clone(), d2.clone());
    assert!(infer(&env, &ctx, &different_ids).is_ok());
    assert_eq!(whnf(&env, &ctx, &different_ids), bottom_term(&env));
    let same_id = eq(type0.clone(), d1.clone(), d1.clone());
    assert!(infer(&env, &ctx, &same_id).is_ok());
    assert_eq!(whnf(&env, &ctx, &same_id), same_id);
    let same_former_different_parts = [
        eq(
            type0.clone(),
            Term::pi(d1.clone(), d1.clone()),
            Term::pi(d2.clone(), d2.clone()),
        ),
        eq(
            type0,
            Term::sigma(d1.clone(), d1),
            Term::sigma(d2.clone(), d2),
        ),
    ];
    for proposition in same_former_different_parts {
        assert!(infer(&env, &ctx, &proposition).is_ok());
        assert_eq!(whnf(&env, &ctx, &proposition), proposition);
    }
}

/// AC-2: an application is rigid only when headed by an inductive former.
/// A variable-headed type application can instantiate to the Π alongside it.
#[test]
fn inductive_headed_application_is_rigid_but_variable_headed_is_neutral() {
    let mut env = GlobalEnv::new();
    let mut ctx = Context::new();
    let type0 = Term::Type(Level::zero());
    let type1 = Term::Type(Level::zero().suc());
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
    .expect("nullary family");
    let nat_ty = Term::indformer(nat, vec![]);
    let zero = Term::constructor(env.inductive(nat).unwrap().constructors[0].id, vec![]);
    let indexed = |env: &mut GlobalEnv| {
        declare_inductive(env, |_| InductiveSpec {
            level_params: vec![],
            params: vec![],
            indices: vec![nat_ty.clone()],
            level: Level::zero(),
            constructors: vec![CtorSpec {
                args: vec![],
                target_indices: vec![zero.clone()],
            }],
        })
        .expect("indexed family")
    };
    let left = Term::app(Term::indformer(indexed(&mut env), vec![]), zero.clone());
    let right = Term::app(Term::indformer(indexed(&mut env), vec![]), zero);
    let rigid_pair = eq(type0.clone(), left.clone(), right);
    assert!(infer(&env, &ctx, &rigid_pair).is_ok());
    assert_eq!(whnf(&env, &ctx, &rigid_pair), bottom_term(&env));
    let identical = eq(type0.clone(), left.clone(), left);
    assert_eq!(whnf(&env, &ctx, &identical), identical);

    // F : Type1 → Type0; F Type0 is an open type, not an inductive head.
    ctx.push(Term::pi(type1.clone(), type0.clone()));
    let open = Term::app(Term::var(0), type0);
    let rigid = Term::pi(nat_ty.clone(), nat_ty);
    for neutral_pair in [
        eq(Term::Type(Level::zero()), rigid.clone(), open.clone()),
        eq(Term::Type(Level::zero()), open, rigid),
    ] {
        assert!(infer(&env, &ctx, &neutral_pair).is_ok());
        assert_eq!(whnf(&env, &ctx, &neutral_pair), neutral_pair);
    }
}

/// Quot and Trunc are classified as rigid formers; this checks only the raw
/// reducer dispatch for these shapes, not formation of an Eq Type redex.
#[test]
fn raw_quotient_and_truncation_heads_are_disjoint_from_other_rigid_formers() {
    let env = GlobalEnv::new();
    let ctx = Context::new();
    let type0 = Term::Type(Level::zero());
    for former in [
        Term::Quot(Box::new(type0.clone()), Box::new(type0.clone())),
        Term::Trunc(Box::new(type0.clone())),
    ] {
        let expr = eq(
            type0.clone(),
            former,
            Term::pi(type0.clone(), type0.clone()),
        );
        assert_eq!(whnf(&env, &ctx, &expr), bottom_term(&env));
    }
}
