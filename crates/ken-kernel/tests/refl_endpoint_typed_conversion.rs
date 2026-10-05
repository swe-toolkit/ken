//! Typed endpoint congruence for observational equality (`15 §2`, `16 §8.2`,
//! `17 §2–3`). Promise class: durable invariants. All positive cases enter
//! the public `check` path, not an isolated structural-comparison helper.

use ken_kernel::env::Context;
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    check, convert, convert_type, declare_inductive, declare_postulate, infer, CtorSpec, GlobalEnv,
    InductiveSpec, KernelError,
};

fn eq(carrier: Term, left: Term, right: Term) -> Term {
    Term::Eq(Box::new(carrier), Box::new(left), Box::new(right))
}

fn nat(env: &mut GlobalEnv) -> Term {
    let id = declare_inductive(env, |self_id| InductiveSpec {
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
                args: vec![Term::indformer(self_id, vec![])],
                target_indices: vec![],
            },
        ],
    });
    Term::indformer(id.expect("Nat"), vec![])
}

/// `Eq P x y` with `P : Ω` forms at the same level (`16 §2.1`).
/// Declaration admission, rather than raw Refl checking, witnesses formation.
#[test]
fn omega_carrier_eq_forms_at_declaration_classification() {
    let mut env = GlobalEnv::new();
    let prop = Term::const_(
        declare_postulate(&mut env, "P".into(), vec![], Term::Omega(Level::zero())).expect("P : Ω"),
        vec![],
    );
    let x = Term::const_(
        declare_postulate(&mut env, "x".into(), vec![], prop.clone()).expect("x : P"),
        vec![],
    );
    let y = Term::const_(
        declare_postulate(&mut env, "y".into(), vec![], prop.clone()).expect("y : P"),
        vec![],
    );
    let carrier_eq = eq(prop, x, y);
    let before = env.trusted_base();
    assert_eq!(
        infer(&env, &Context::new(), &carrier_eq),
        Ok(Term::Omega(Level::zero()))
    );
    let admitted = declare_postulate(&mut env, "carrier_eq".into(), vec![], carrier_eq)
        .expect("Ω-carrier Eq must pass declaration classification");
    let mut expected = before;
    expected.push(admitted);
    assert_eq!(
        env.trusted_base(),
        expected,
        "only this test's admitted postulate adds trust"
    );
}

#[test]
fn refl_at_dependent_sigma_eta_with_distinct_proofs_checks() {
    let mut env = GlobalEnv::new();
    let nat = nat(&mut env);
    let family = Term::const_(
        declare_postulate(
            &mut env,
            "P".into(),
            vec![],
            Term::pi(nat.clone(), Term::Omega(Level::zero())),
        )
        .expect("P : Nat → Ω"),
        vec![],
    );
    let sigma = Term::sigma(nat.clone(), Term::app(family.clone(), Term::var(0)));
    let mut ctx = Context::new();
    ctx.push(sigma.clone()); // v : Σ n:Nat. P n
    let v = Term::var(0);
    ctx.push(Term::app(family, Term::proj1(v))); // k : P v.1
    let v = Term::var(1);
    let other = Term::pair(Term::proj1(v.clone()), Term::var(0));
    let target = eq(sigma.clone(), v.clone(), other.clone());
    let trust = env.trusted_base();
    assert!(trust.len() > 0, "trust comparison includes postulated P");
    assert!(
        infer(&env, &ctx, &target).is_ok(),
        "well-formed Eq at Sigma"
    );
    check(&env, &ctx, &other, &sigma).expect("alternate pair is typed");
    assert!(
        convert(&env, &ctx, &sigma, &v, &other),
        "Σ-η and Ω-PI premise"
    );
    check(&env, &ctx, &Term::Refl(Box::new(v)), &target)
        .expect("refl v checks at Eq Sigma v (v.1,k)");
    assert_eq!(env.trusted_base(), trust);
}

/// A *well-formed* outer Eq at a Π/Σ carrier reaches a nested proof equality
/// by `eq_at_pi` then `eq_at_sigma`. This is separate from the direct Σ row
/// and the source `Pos` row's inductive reduction (`15 §2`, `16 §8.2`).
#[test]
fn refl_at_pi_route_sigma_fields_with_distinct_proofs_checks() {
    let mut env = GlobalEnv::new();
    let nat = nat(&mut env);
    let family = Term::const_(
        declare_postulate(
            &mut env,
            "P".into(),
            vec![],
            Term::pi(nat.clone(), Term::Omega(Level::zero())),
        )
        .expect("P : Nat → Ω"),
        vec![],
    );
    let proof_family = Term::pi(nat.clone(), Term::app(family.clone(), Term::var(0)));
    let mut ctx = Context::new();
    ctx.push(proof_family.clone()); // p : (n:Nat) → P n
    ctx.push(proof_family); // q : (n:Nat) → P n
    let carrier = Term::pi(
        nat.clone(),
        Term::sigma(nat.clone(), Term::app(family, Term::var(0))),
    );
    // Under the new n binder, p is Var(2), q is Var(1), n is Var(0).
    let f = Term::lam(
        nat.clone(),
        Term::pair(Term::var(0), Term::app(Term::var(2), Term::var(0))),
    );
    let g = Term::lam(
        nat,
        Term::pair(Term::var(0), Term::app(Term::var(1), Term::var(0))),
    );
    let target = eq(carrier.clone(), f.clone(), g.clone());
    let trust = env.trusted_base();
    check(&env, &ctx, &f, &carrier).expect("f : C");
    check(&env, &ctx, &g, &carrier).expect("g : C");
    infer(&env, &ctx, &target).expect("outer Eq C f g is well formed");
    let f_inferable = Term::Ascript(Box::new(f), Box::new(carrier.clone()));
    assert_eq!(infer(&env, &ctx, &f_inferable), Ok(carrier));
    check(&env, &ctx, &Term::Refl(Box::new(f_inferable)), &target)
        .expect("Refl f checks at Eq C f g through Π and Σ reductions");
    assert_eq!(env.trusted_base(), trust);
}

#[test]
fn refl_at_open_sigma_eta_without_proofs_stays_accepted() {
    let mut env = GlobalEnv::new();
    let nat = nat(&mut env);
    let sigma = Term::sigma(nat.clone(), nat);
    let mut ctx = Context::new();
    ctx.push(sigma.clone());
    let pair = Term::var(0);
    let eta = Term::pair(Term::proj1(pair.clone()), Term::proj2(pair.clone()));
    let target = eq(sigma, pair.clone(), eta);
    check(&env, &ctx, &Term::Refl(Box::new(pair)), &target)
        .expect("ordinary Σ-η Refl remains accepted without Ω components");
}

#[test]
fn refl_at_open_pi_eta_checks() {
    let mut env = GlobalEnv::new();
    let nat = nat(&mut env);
    let function_type = Term::pi(nat.clone(), nat.clone());
    let mut ctx = Context::new();
    ctx.push(function_type.clone()); // f : Nat → Nat
    let f = Term::var(0);
    let eta = Term::lam(nat, Term::app(Term::var(1), Term::var(0)));
    let target = eq(function_type.clone(), f.clone(), eta.clone());
    assert!(convert(&env, &ctx, &function_type, &f, &eta), "Π-η premise");
    check(&env, &ctx, &Term::Refl(Box::new(f)), &target)
        .expect("Refl f checks at Eq (Nat → Nat) f (λx. f x)");
}

#[test]
fn eq_congruence_stays_positional_at_proof_relevant_endpoints() {
    let mut env = GlobalEnv::new();
    let nat = nat(&mut env);
    let mut ctx = Context::new();
    ctx.push(nat.clone()); // x : Nat
    ctx.push(nat.clone()); // y : Nat, distinct from x
    let x = Term::var(1);
    let y = Term::var(0);
    let forward = eq(nat.clone(), x.clone(), y.clone());
    let reversed = eq(nat.clone(), y.clone(), x.clone());
    assert!(
        infer(&env, &ctx, &forward).is_ok(),
        "well-formed first equality"
    );
    assert!(
        infer(&env, &ctx, &reversed).is_ok(),
        "well-formed second equality"
    );
    assert!(
        convert_type(&env, &ctx, &forward, &forward),
        "same Eq is convertible"
    );
    assert!(
        !convert_type(&env, &ctx, &forward, &reversed),
        "Eq is not definitionally symmetric"
    );
    assert_eq!(
        check(&env, &ctx, &Term::Refl(Box::new(x.clone())), &forward),
        Err(KernelError::TypeMismatch {
            expected: Box::new(forward),
            found: Box::new(eq(nat.clone(), x.clone(), x.clone())),
        }),
        "distinct Nat binders cannot be equated by Eq/Eq congruence",
    );
    check(
        &env,
        &ctx,
        &Term::Refl(Box::new(x.clone())),
        &eq(nat, x.clone(), x),
    )
    .expect("same endpoint remains a positive control");
}
