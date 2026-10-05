//! J reduction agrees with checked whnf endpoints when Eq-at-Type exposes a
//! bare single-conjunct Eq, without losing the formation-head widening.
//!
//! Promise classes: A and Ω-A are durable typed-J reduction invariants.
//! B and Ω-B are transition sentinels for the separately tracked infer_j
//! substitution-stability widening: they intentionally keep the current
//! infer-refusal / whnf-reduction boundary until that rule changes.

use ken_kernel::env::Context;
use ken_kernel::subst::weaken;
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    check, convert_type, declare_inductive, declare_postulate, infer, whnf, CtorSpec, GlobalEnv,
    InductiveSpec, KernelError,
};

fn eq(carrier: Term, left: Term, right: Term) -> Term {
    Term::Eq(Box::new(carrier), Box::new(left), Box::new(right))
}

fn type0() -> Term {
    Term::Type(Level::zero())
}

fn one_parameter_former(env: &mut GlobalEnv, parameter: Term) -> Term {
    let id = declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![parameter],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .expect("one-parameter Type-valued former");
    Term::indformer(id, vec![])
}

fn nat_and_zero(env: &mut GlobalEnv) -> (Term, Term) {
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
    .expect("Nat-like type with zero");
    let nat = Term::indformer(id, vec![]);
    let zero = Term::constructor(env.inductive(id).unwrap().constructors[0].id, vec![]);
    (nat, zero)
}

fn constant_motive(carrier: Term, start_under_one_binder: Term, nat: &Term) -> Term {
    let evidence_domain = eq(carrier.clone(), start_under_one_binder, Term::var(0));
    Term::Ascript(
        Box::new(Term::lam(
            carrier.clone(),
            Term::lam(evidence_domain.clone(), weaken(nat, 2)),
        )),
        Box::new(Term::pi(carrier, Term::pi(evidence_domain, type0()))),
    )
}

fn assert_j_row(
    label: &str,
    env: &GlobalEnv,
    ctx: &Context,
    motive: Term,
    nat: &Term,
    zero: &Term,
    expected_infer_error: Option<KernelError>,
) {
    let term = Term::J(
        Box::new(motive),
        Box::new(zero.clone()),
        Box::new(Term::var(0)),
    );
    match expected_infer_error {
        None => {
            let inferred = infer(env, ctx, &term).expect("this J is well typed");
            assert!(
                convert_type(env, ctx, &inferred, nat),
                "{label}: the J result has type Nat"
            );
        }
        Some(error) => assert_eq!(infer(env, ctx, &term), Err(error), "{label}: inference"),
    }
    assert_eq!(whnf(env, ctx, &term), *zero, "{label}: whnf");
    let goal = eq(nat.clone(), term, zero.clone());
    assert_eq!(
        check(env, ctx, &Term::Refl(Box::new(zero.clone())), &goal),
        Ok(()),
        "{label}: Refl zero proves Eq Nat J zero"
    );
}

#[test]
fn type_parameter_j_uses_the_guarded_formation_or_whnf_endpoints() {
    let mut env = GlobalEnv::new();
    let (nat, zero) = nat_and_zero(&mut env);
    let b = one_parameter_former(&mut env, type0());
    // X, Y : Type0; e : Eq Type0 (B X) (B Y). The single-conjunct
    // Eq-at-Type reduct exposes Eq Type0 X Y, not the inferred formation.
    let mut ctx = Context::new();
    ctx.push(type0());
    ctx.push(type0());
    ctx.push(eq(
        type0(),
        Term::app(b.clone(), Term::var(1)),
        Term::app(b.clone(), Term::var(0)),
    ));
    assert_eq!(
        whnf(
            &env,
            &ctx,
            &eq(
                type0(),
                Term::app(b.clone(), Term::var(2)),
                Term::app(b.clone(), Term::var(1)),
            ),
        ),
        eq(type0(), Term::var(2), Term::var(1)),
        "the single-parameter Eq conjunct is bare"
    );
    let second_domain_error =
        KernelError::BadEliminator("J motive's second domain ≠ Eq A a b".into());
    let trust = env.trusted_base();
    assert_j_row(
        "A: motive at X",
        &env,
        &ctx,
        constant_motive(type0(), Term::var(3), &nat),
        &nat,
        &zero,
        None,
    );
    assert_j_row(
        "B: motive at B X",
        &env,
        &ctx,
        constant_motive(type0(), Term::app(b, Term::var(3)), &nat),
        &nat,
        &zero,
        Some(second_domain_error),
    );
    assert_eq!(
        env.trusted_base(),
        trust,
        "J reduction adds no trusted declaration"
    );
}

#[test]
fn omega_parameter_j_uses_the_guarded_formation_or_whnf_endpoints() {
    let mut env = GlobalEnv::new();
    let (nat, zero) = nat_and_zero(&mut env);
    let p_id = declare_postulate(&mut env, "P".into(), vec![], Term::Omega(Level::zero()))
        .expect("P : Ω0");
    let proposition = Term::const_(p_id, vec![]);
    let c = one_parameter_former(&mut env, proposition.clone());
    // q, q' : P; e : Eq Type0 (C q) (C q'). The sole Ω parameter's
    // conjunct compares the target proof with itself: Eq P q' q'.
    let mut omega_ctx = Context::new();
    omega_ctx.push(proposition.clone());
    omega_ctx.push(proposition.clone());
    omega_ctx.push(eq(
        type0(),
        Term::app(c.clone(), Term::var(1)),
        Term::app(c.clone(), Term::var(0)),
    ));
    assert_eq!(
        whnf(
            &env,
            &omega_ctx,
            &eq(
                type0(),
                Term::app(c.clone(), Term::var(2)),
                Term::app(c.clone(), Term::var(1)),
            ),
        ),
        eq(proposition.clone(), Term::var(1), Term::var(1)),
        "the single-parameter Ω conjunct is bare"
    );
    let trust = env.trusted_base();
    assert!(
        !trust.is_empty(),
        "the P postulate makes the trust comparison nonvacuous"
    );
    assert_j_row(
        "Ω-A: motive at q",
        &env,
        &omega_ctx,
        constant_motive(proposition, Term::var(3), &nat),
        &nat,
        &zero,
        None,
    );
    assert_j_row(
        "Ω-B: motive at C q",
        &env,
        &omega_ctx,
        constant_motive(type0(), Term::app(c, Term::var(3)), &nat),
        &nat,
        &zero,
        Some(KernelError::BadEliminator(
            "J motive's first domain ≠ the equality's type A".into(),
        )),
    );
    assert_eq!(
        env.trusted_base(),
        trust,
        "J reduction adds no trusted declaration"
    );
}
