//! Quot-Form admission and class equality (`16 §2.2`, §5).
//! Formation checks the equivalence proof, both relation domains, and the
//! exact Ω level; only checked quotients can expose relation-as-equality.

use ken_kernel::env::Context;
use ken_kernel::obs::{bottom_term, eq_reduce, top_term, tt_term};
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    convert_type, declare_inductive, infer, whnf, CtorSpec, GlobalEnv, InductiveSpec, KernelError,
};

fn nullary(env: &mut GlobalEnv, count: usize) -> (Term, Vec<Term>) {
    let id = declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: (0..count)
            .map(|_| CtorSpec {
                args: vec![],
                target_indices: vec![],
            })
            .collect(),
    })
    .expect("checked data family");
    (
        Term::indformer(id, vec![]),
        env.inductive(id)
            .unwrap()
            .constructors
            .iter()
            .map(|ctor| Term::constructor(ctor.id, vec![]))
            .collect(),
    )
}

fn nat_with_zero_and_one(env: &mut GlobalEnv) -> (Term, Term, Term) {
    let id = declare_inductive(env, |id| InductiveSpec {
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
    .expect("checked Nat with zero and suc");
    let constructors = &env.inductive(id).unwrap().constructors;
    let zero = Term::constructor(constructors[0].id, vec![]);
    let one = Term::app(Term::constructor(constructors[1].id, vec![]), zero.clone());
    (Term::indformer(id, vec![]), zero, one)
}

fn relation(a: &Term, b: &Term, proposition: Term, level: Level) -> Term {
    Term::Ascript(
        Box::new(Term::lam(a.clone(), Term::lam(b.clone(), proposition))),
        Box::new(Term::pi(a.clone(), Term::pi(b.clone(), Term::Omega(level)))),
    )
}

fn equiv_witness(a: &Term, proposition: Term, proof: Term) -> Term {
    let reflexive = Term::lam(a.clone(), proof.clone());
    let symmetric = Term::lam(
        a.clone(),
        Term::lam(a.clone(), Term::lam(proposition.clone(), proof.clone())),
    );
    let transitive = Term::lam(
        a.clone(),
        Term::lam(
            a.clone(),
            Term::lam(
                a.clone(),
                Term::lam(proposition.clone(), Term::lam(proposition, proof)),
            ),
        ),
    );
    Term::pair(reflexive, Term::pair(symmetric, transitive))
}

fn quot(a: Term, r: Term, e: Term) -> Term {
    Term::Quot(Box::new(a), Box::new(r), Box::new(e))
}

#[test]
fn nonreflexive_false_equality_cannot_form_a_quotient() {
    let mut env = GlobalEnv::new();
    let before = env.trusted_base();
    let (nat, zero, one) = nat_with_zero_and_one(&mut env);
    let false_eq = Term::Eq(Box::new(nat.clone()), Box::new(zero), Box::new(one));
    assert_eq!(whnf(&env, &Context::new(), &false_eq), bottom_term(&env));
    let r = relation(&nat, &nat, false_eq.clone(), Level::zero());
    let q = quot(nat.clone(), r, equiv_witness(&nat, false_eq, tt_term(&env)));
    assert!(matches!(
        infer(&env, &Context::new(), &q),
        Err(KernelError::TypeMismatch { .. })
    ));
    assert_eq!(env.trusted_base(), before);
}

#[test]
fn total_equivalence_forms_and_class_equality_reduces_to_the_relation() {
    let mut env = GlobalEnv::new();
    let before = env.trusted_base();
    let (a, values) = nullary(&mut env, 2);
    let r = relation(&a, &a, top_term(&env), Level::zero());
    let q = quot(
        a.clone(),
        r.clone(),
        equiv_witness(&a, top_term(&env), tt_term(&env)),
    );
    assert_eq!(
        infer(&env, &Context::new(), &q),
        Ok(Term::Type(Level::zero()))
    );
    let lhs = Term::QuotClass(Box::new(values[0].clone()));
    let rhs = Term::QuotClass(Box::new(values[1].clone()));
    let eq = Term::Eq(
        Box::new(q.clone()),
        Box::new(lhs.clone()),
        Box::new(rhs.clone()),
    );
    assert_eq!(
        infer(&env, &Context::new(), &eq),
        Ok(Term::Omega(Level::zero()))
    );
    let related = Term::app(Term::app(r, values[0].clone()), values[1].clone());
    assert_eq!(
        eq_reduce(&env, &Context::new(), &q, &lhs, &rhs),
        Some(related)
    );
    assert_eq!(whnf(&env, &Context::new(), &eq), top_term(&env));
    let mut open = Context::new();
    open.push(q.clone());
    let neutral = Term::Eq(Box::new(q), Box::new(Term::var(0)), Box::new(rhs));
    assert_eq!(whnf(&env, &open, &neutral), neutral);
    assert_eq!(env.trusted_base(), before);
}

#[test]
fn quotient_type_conversion_ignores_distinct_checked_equivalence_proofs() {
    let mut env = GlobalEnv::new();
    let (a, _) = nullary(&mut env, 2);
    let r = relation(&a, &a, top_term(&env), Level::zero());
    let equiv = ken_kernel::check::quotient_equivalence_type(&a, &r);
    let mut ctx = Context::new();
    ctx.push(equiv.clone());
    ctx.push(equiv);
    let first = quot(a.clone(), r.clone(), Term::var(1));
    let second = quot(a, r, Term::var(0));
    assert_eq!(infer(&env, &ctx, &first), Ok(Term::Type(Level::zero())));
    assert_eq!(infer(&env, &ctx, &second), Ok(Term::Type(Level::zero())));
    assert!(convert_type(&env, &ctx, &first, &second));
}

#[test]
fn relation_one_omega_level_above_carrier_is_refused() {
    let mut env = GlobalEnv::new();
    let before = env.trusted_base();
    let (a, _) = nullary(&mut env, 2);
    let proposition = Term::Eq(
        Box::new(Term::Type(Level::zero())),
        Box::new(a.clone()),
        Box::new(a.clone()),
    );
    let r = relation(&a, &a, proposition.clone(), Level::zero().suc());
    let q = quot(
        a.clone(),
        r,
        equiv_witness(&a, proposition, Term::Refl(Box::new(a.clone()))),
    );
    assert_eq!(
        infer(&env, &Context::new(), &q),
        Err(KernelError::BadEliminator(
            "quotient relation's Ω level ≠ carrier level".into()
        ))
    );
    assert_eq!(env.trusted_base(), before);
}

#[test]
fn relation_second_domain_must_match_the_carrier() {
    let mut env = GlobalEnv::new();
    let (a, _) = nullary(&mut env, 2);
    let (b, _, _) = nat_with_zero_and_one(&mut env);
    let r = relation(&a, &b, top_term(&env), Level::zero());
    let q = quot(
        a.clone(),
        r,
        equiv_witness(&a, top_term(&env), tt_term(&env)),
    );
    assert_eq!(
        infer(&env, &Context::new(), &q),
        Err(KernelError::BadEliminator(
            "quotient relation's second domain ≠ A".into()
        ))
    );
}
