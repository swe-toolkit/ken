//! Compound casts project equality components only when Eq-at-Type decomposes.

use ken_kernel::env::Context;
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    check, declare_inductive, declare_postulate, infer, whnf, CtorSpec, GlobalEnv, InductiveSpec,
    KernelError,
};

fn opaque(env: &mut GlobalEnv, label: &str, ty: Term) -> Term {
    Term::const_(
        declare_postulate(env, label.into(), vec![], ty).unwrap(),
        vec![],
    )
}

fn quotient_assuming_equiv(env: &mut GlobalEnv, carrier: Term, relation: Term) -> Term {
    let proof = opaque(
        env,
        "quotient equivalence",
        ken_kernel::check::quotient_equivalence_type(&carrier, &relation),
    );
    Term::Quot(Box::new(carrier), Box::new(relation), Box::new(proof))
}

fn nat(env: &mut GlobalEnv) -> Term {
    let id = declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .unwrap();
    Term::indformer(id, vec![])
}

fn cast_probe(
    env: &GlobalEnv,
    source: Term,
    target: Term,
    value: Term,
) -> (Context, Term, Term, Term) {
    let mut ctx = Context::new();
    let Term::Type(level) = infer(env, &ctx, &source).expect("source type classifies") else {
        panic!("source must be Type-sorted")
    };
    assert_eq!(
        infer(env, &ctx, &target),
        Ok(Term::Type(level.clone())),
        "endpoints must share the same Type level"
    );
    let equality = Term::Eq(
        Box::new(Term::Type(level)),
        Box::new(source.clone()),
        Box::new(target.clone()),
    );
    infer(env, &ctx, &equality).expect("Eq Type is well formed");
    let eq_reduct = whnf(env, &ctx, &equality);
    ctx.push(equality.clone());
    let redex = Term::Cast(
        Box::new(source),
        Box::new(target.clone()),
        Box::new(Term::var(0)),
        Box::new(value),
    );
    assert_eq!(infer(env, &ctx, &redex), Ok(target));
    (ctx, redex, equality, eq_reduct)
}

fn assert_neutral_cast(env: &GlobalEnv, source: Term, target: Term, value: Term) {
    let (ctx, redex, equality, eq_reduct) = cast_probe(env, source, target, value);
    assert_eq!(eq_reduct, equality, "Eq Type must be neutral");
    let trusted = env.trusted_base();
    assert_eq!(whnf(env, &ctx, &redex), redex, "cast must remain neutral");
    assert_eq!(env.trusted_base(), trusted);
}

fn assert_decomposed_cast(env: &GlobalEnv, source: Term, target: Term, value: Term) {
    let (ctx, redex, _equality, eq_reduct) = cast_probe(env, source, target, value);
    assert!(
        matches!(eq_reduct, Term::Sigma(..)),
        "Eq Type must expose projectable component equalities"
    );
    let before = infer(env, &ctx, &redex).unwrap();
    let trusted = env.trusted_base();
    let reduct = whnf(env, &ctx, &redex);
    assert_ne!(reduct, redex, "decomposing cast must still fire");
    assert_eq!(check(env, &ctx, &reduct, &before), Ok(()));
    assert_eq!(env.trusted_base(), trusted);
}

/// Durable invariant: a subset Σ has a Type-sorted carrier, but its Ω-sorted
/// codomain does not supply a projected Eq Type codomain equality.
#[test]
fn subset_sigma_neutral_equality_stays_stuck() {
    let mut env = GlobalEnv::new();
    let n = nat(&mut env);
    let p = opaque(&mut env, "P", Term::Omega(Level::zero()));
    let q = opaque(&mut env, "Q", Term::Omega(Level::zero()));
    let source = Term::sigma(n.clone(), p);
    let target = Term::sigma(n, q);
    let value = opaque(&mut env, "subset_value", source.clone());
    assert_neutral_cast(&env, source, target, value);
}

/// An upward relation Ω level now fails at Quot-Form, before cast can
/// inspect any apparent equality components.
#[test]
fn quotient_relation_level_mismatch_stays_stuck() {
    let mut env = GlobalEnv::new();
    let n = nat(&mut env);
    let r0 = opaque(
        &mut env,
        "R0",
        Term::pi(n.clone(), Term::pi(n.clone(), Term::Omega(Level::zero()))),
    );
    let r1 = opaque(
        &mut env,
        "R1",
        Term::pi(
            n.clone(),
            Term::pi(n.clone(), Term::Omega(Level::zero().suc())),
        ),
    );
    let source = quotient_assuming_equiv(&mut env, n.clone(), r0);
    assert_eq!(infer(&env, &Context::new(), &source), Ok(Term::Type(Level::zero())));
    let target = Term::Quot(
        Box::new(n),
        Box::new(r1),
        Box::new(Term::const_(env.tt_id(), vec![])),
    );
    assert_eq!(
        infer(&env, &Context::new(), &target),
        Err(KernelError::BadEliminator(
            "quotient relation's Ω level ≠ carrier level".into()
        ))
    );
}

/// Durable invariant: an Ω-sorted first component cannot be projected as an
/// Eq Type domain equality, even when the whole Σ is Type-sorted.
#[test]
fn sigma_proposition_domain_stays_stuck() {
    let mut env = GlobalEnv::new();
    let n = nat(&mut env);
    let p = opaque(&mut env, "P", Term::Omega(Level::zero()));
    let q = opaque(&mut env, "Q", Term::Omega(Level::zero()));
    let source = Term::sigma(p, n.clone());
    let target = Term::sigma(q, n);
    let value = opaque(&mut env, "prop_domain_value", source.clone());
    assert_neutral_cast(&env, source, target, value);
}

/// Durable invariant: equal outer Σ universe levels cannot mask a mismatch
/// between the domain levels needed for component equality.
#[test]
fn sigma_domain_level_mismatch_stays_stuck() {
    let mut env = GlobalEnv::new();
    let u0 = Term::Type(Level::zero());
    let u1 = Term::Type(Level::zero().suc());
    let source = Term::sigma(u0, u1.clone());
    let target = Term::sigma(u1.clone(), u1);
    let value = opaque(&mut env, "level_value", source.clone());
    assert_neutral_cast(&env, source, target, value);
}

/// Durable invariant: a decomposed Σ type equality still permits a cast
/// whose reduct checks at the target type, even with neutral evidence.
#[test]
fn sigma_decomposition_still_fires_with_typed_reduct() {
    let mut env = GlobalEnv::new();
    let n = nat(&mut env);
    let a = opaque(&mut env, "A", Term::Type(Level::zero()));
    let b = opaque(&mut env, "B", Term::Type(Level::zero()));
    let source = Term::sigma(n.clone(), a);
    let target = Term::sigma(n, b);
    let value = opaque(&mut env, "sigma_value", source.clone());
    assert_decomposed_cast(&env, source, target, value);
}

/// Durable invariant: equal relation levels expose the quotient's underlying
/// component equality, so a class-headed cast fires and its reduct checks.
#[test]
fn quotient_decomposition_still_fires_with_typed_reduct() {
    let mut env = GlobalEnv::new();
    let n = nat(&mut env);
    let member = opaque(&mut env, "member", n.clone());
    let relation_ty = Term::pi(n.clone(), Term::pi(n.clone(), Term::Omega(Level::zero())));
    let r = opaque(&mut env, "R", relation_ty.clone());
    let s = opaque(&mut env, "S", relation_ty);
    let source = quotient_assuming_equiv(&mut env, n.clone(), r);
    let target = quotient_assuming_equiv(&mut env, n, s);
    assert_decomposed_cast(&env, source, target, Term::QuotClass(Box::new(member)));
}
