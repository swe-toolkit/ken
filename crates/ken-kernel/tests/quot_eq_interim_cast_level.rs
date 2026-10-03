//! Kernel-side interim quotient-Eq and cast-level fences (16 §2.2, §3.1, §5).
//! The neutral quotient rule is a transition sentinel: P0 may restore it only
//! after Quot-Form checks that the relation is an equivalence.

use ken_kernel::env::Context;
use ken_kernel::obs::bottom_term;
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    check, convert, declare_inductive, infer, whnf, CtorSpec, GlobalEnv, GlobalId, InductiveSpec,
    KernelError,
};

fn eq(ty: Term, left: Term, right: Term) -> Term {
    Term::Eq(Box::new(ty), Box::new(left), Box::new(right))
}

fn constructor(id: GlobalId) -> Term {
    Term::Constructor {
        id,
        level_args: vec![],
    }
}

fn nullary(env: &mut GlobalEnv, constructor_count: usize) -> (Term, Vec<Term>) {
    let id = declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: (0..constructor_count)
            .map(|_| CtorSpec {
                args: vec![],
                target_indices: vec![],
            })
            .collect(),
    })
    .unwrap();
    (
        Term::indformer(id, vec![]),
        env.inductive(id)
            .unwrap()
            .constructors
            .iter()
            .map(|ctor| constructor(ctor.id))
            .collect(),
    )
}

fn relation(carrier: Term, second_domain: Term, body: Term) -> Term {
    let ty = Term::pi(
        carrier.clone(),
        Term::pi(second_domain.clone(), Term::Omega(Level::zero())),
    );
    Term::Ascript(
        Box::new(Term::lam(carrier, Term::lam(second_domain, body))),
        Box::new(ty),
    )
}

/// Build the constructor-field route that the raw-class `eq_at_quot` arm
/// reaches. A bare `refl [a]` cannot infer its QuotClass operand; ascribing
/// the class instead hides the raw pair behind Ascript at the Eq redex.
fn constructor_field_proof(env: &mut GlobalEnv, quot: Term, member: Term) -> (Term, Term) {
    let bx_id = declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![quot.clone()],
            target_indices: vec![],
        }],
    })
    .unwrap();
    let bx = Term::indformer(bx_id, vec![]);
    let value = Term::app(
        constructor(env.inductive(bx_id).unwrap().constructors[0].id),
        Term::QuotClass(Box::new(member)),
    );
    let equality = eq(bx, value.clone(), value.clone());
    let ctx = Context::new();
    assert_eq!(infer(env, &ctx, &equality), Ok(Term::Omega(Level::zero())));
    // The one-field constructor equality carries a raw QuotClass argument.
    // Do not assert an intermediate reduct here: M1 must reach the final
    // proof-refusal assertion, not fail merely because its shape changed.
    let proof = Term::Ascript(
        Box::new(Term::Refl(Box::new(value))),
        Box::new(equality.clone()),
    );
    assert_eq!(infer(env, &ctx, &proof), Ok(equality.clone()));
    (proof, equality)
}

fn assert_reflexive_constructor_does_not_prove_bottom(
    env: &mut GlobalEnv,
    quot: Term,
    member: Term,
) {
    let (proof, equality) = constructor_field_proof(env, quot, member);
    let ctx = Context::new();
    let bottom = bottom_term(env);
    assert_eq!(
        check(env, &ctx, &proof, &bottom),
        Err(KernelError::TypeMismatch {
            expected: Box::new(bottom),
            found: Box::new(equality),
        }),
        "the checked constructor reflexivity proof cannot close Bottom"
    );
}

/// Transition sentinel, AC-1/S2: even the currently accepted malformed
/// second domain must not turn a checked Bx reflexivity proof into Bottom.
/// P0 may instead reject the malformed quotient at formation.
#[test]
fn constructor_field_with_mismatched_relation_second_domain_cannot_prove_bottom() {
    let mut env = GlobalEnv::new();
    let trusted = env.trusted_base();
    let (bool_, values) = nullary(&mut env, 2);
    let (nat, numbers) = nullary(&mut env, 1);
    let r = relation(
        bool_.clone(),
        nat.clone(),
        eq(nat, Term::var(0), numbers[0].clone()),
    );
    let quot = Term::Quot(Box::new(bool_), Box::new(r));
    assert_eq!(
        infer(&env, &Context::new(), &quot),
        Ok(Term::Type(Level::zero()))
    );
    assert_reflexive_constructor_does_not_prove_bottom(&mut env, quot, values[0].clone());
    assert_eq!(env.trusted_base(), trusted);
}

/// Transition sentinel, AC-1/S2: well-typed R need not be reflexive. Both
/// this pair and the mismatched-domain pair above traverse the same raw-class
/// equality conjunct in the inductive Eq reduction.
#[test]
fn constructor_field_with_nonreflexive_relation_cannot_prove_bottom() {
    let mut env = GlobalEnv::new();
    let trusted = env.trusted_base();
    let (bool_, values) = nullary(&mut env, 2);
    let r = relation(
        bool_.clone(),
        bool_.clone(),
        eq(bool_.clone(), Term::var(1), values[1].clone()),
    );
    let quot = Term::Quot(Box::new(bool_), Box::new(r));
    assert_eq!(
        infer(&env, &Context::new(), &quot),
        Ok(Term::Type(Level::zero()))
    );
    assert_reflexive_constructor_does_not_prove_bottom(&mut env, quot, values[0].clone());
    assert_eq!(env.trusted_base(), trusted);
}

/// Transition sentinel, AC-1/M1: an independent closed Eq Nat zero (suc zero)
/// must not be derivable from the constructor-field reflexivity proof merely
/// because the relation ignores its endpoints and returns that proposition.
#[test]
fn closed_constructor_quotient_refl_cannot_prove_zero_equals_one() {
    let mut env = GlobalEnv::new();
    let trusted = env.trusted_base();
    let nat_id = declare_inductive(&mut env, |id| InductiveSpec {
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
    .unwrap();
    let nat = Term::indformer(nat_id, vec![]);
    let cs = &env.inductive(nat_id).unwrap().constructors;
    let zero = constructor(cs[0].id);
    let one = Term::app(constructor(cs[1].id), zero.clone());
    let false_eq = eq(nat.clone(), zero.clone(), one);
    let r = relation(nat.clone(), nat.clone(), false_eq.clone());
    let quot = Term::Quot(Box::new(nat), Box::new(r));
    let (proof, constructor_eq) = constructor_field_proof(&mut env, quot, zero);
    let ctx = Context::new();
    assert_eq!(whnf(&env, &ctx, &false_eq), bottom_term(&env));
    assert_eq!(
        check(&env, &ctx, &proof, &false_eq),
        Err(KernelError::TypeMismatch {
            expected: Box::new(false_eq),
            found: Box::new(constructor_eq),
        }),
    );
    assert_eq!(env.trusted_base(), trusted);
}

/// Durable invariant, AC-1/AC-2 M2: a conversion-equivalent pair of Type-sorted Σ types can
/// inhabit different levels after the observational Eq reduct drops a level.
/// Cast must compare their classified levels before checking its proof.
#[test]
fn cast_refuses_unequal_endpoint_levels_but_keeps_equal_level_control() {
    let mut env = GlobalEnv::new();
    let trusted = env.trusted_base();
    let (bool_, values) = nullary(&mut env, 2);
    let d_id = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero().suc(),
        constructors: vec![CtorSpec {
            args: vec![bool_.clone()],
            target_indices: vec![],
        }],
    })
    .unwrap();
    let d = Term::indformer(d_id, vec![]);
    let c = constructor(env.inductive(d_id).unwrap().constructors[0].id);
    let c_x = Term::app(c.clone(), Term::var(0));
    let large = Term::sigma(bool_.clone(), eq(d, c_x.clone(), c_x));
    let small = Term::sigma(bool_.clone(), eq(bool_.clone(), Term::var(0), Term::var(0)));
    let ctx = Context::new();
    let level0 = Level::zero();
    let level1 = level0.clone().suc();
    assert_eq!(infer(&env, &ctx, &large), Ok(Term::Type(level1.clone())));
    assert_eq!(infer(&env, &ctx, &small), Ok(Term::Type(level0.clone())));
    assert!(convert(
        &env,
        &ctx,
        &Term::Type(level1.clone()),
        &large,
        &small
    ));
    let value = Term::Ascript(
        Box::new(Term::Pair(
            Box::new(values[0].clone()),
            Box::new(Term::Refl(Box::new(Term::app(c, values[0].clone())))),
        )),
        Box::new(large.clone()),
    );
    assert_eq!(infer(&env, &ctx, &value), Ok(large.clone()));
    let cross_level = Term::Cast(
        Box::new(large.clone()),
        Box::new(small),
        Box::new(Term::Refl(Box::new(large.clone()))),
        Box::new(value.clone()),
    );
    assert_eq!(
        infer(&env, &ctx, &cross_level),
        Err(KernelError::TypeMismatch {
            expected: Box::new(Term::Type(level1)),
            found: Box::new(Term::Type(level0)),
        }),
    );
    let equal_level = Term::Cast(
        Box::new(large.clone()),
        Box::new(large.clone()),
        Box::new(Term::Refl(Box::new(large.clone()))),
        Box::new(value.clone()),
    );
    assert_eq!(infer(&env, &ctx, &equal_level), Ok(large.clone()));
    let reduct = whnf(&env, &ctx, &equal_level);
    assert_eq!(reduct, whnf(&env, &ctx, &value));
    assert_eq!(check(&env, &ctx, &reduct, &large), Ok(()));
    assert_eq!(env.trusted_base(), trusted);
}
