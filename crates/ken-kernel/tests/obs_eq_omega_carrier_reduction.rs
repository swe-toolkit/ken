//! `SPEC-EQ-FORM-OMEGA-CARRIER`, `16 §2.1–2.2, §8.2–8.4`.
//! Each conformance-derived test below is a durable behavioral invariant.
//! The six fixtures exercise checked Eq formation, the public WHNF reducer,
//! and the independent type/checking paths rather than calling `eq_reduce`.

use ken_kernel::env::Context;
use ken_kernel::subst::{subst0, weaken};
use ken_kernel::term::{Level, LevelVar, Term};
use ken_kernel::{
    check, declare_inductive, declare_postulate, infer, whnf, CtorSpec, GlobalEnv, InductiveSpec,
    KernelError,
};

fn post(env: &mut GlobalEnv, label: &str, ty: Term) -> Term {
    Term::const_(
        declare_postulate(env, label.into(), vec![], ty).expect("typed fixture postulate"),
        vec![],
    )
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
    .expect("Nat fixture");
    Term::indformer(id, vec![])
}

fn eq(carrier: Term, left: Term, right: Term) -> Term {
    Term::Eq(Box::new(carrier), Box::new(left), Box::new(right))
}

fn app2(f: Term, a: Term, b: Term) -> Term {
    Term::app(Term::app(f, a), b)
}

fn app3(f: Term, a: Term, b: Term, c: Term) -> Term {
    Term::app(app2(f, a, b), c)
}

#[test]
fn eq_subset_sigma_omega_field() {
    // Conformance: observational/eq-subset-sigma-omega-field. R2 changes a
    // dependent Ω codomain without changing the relevant first projection.
    let mut env = GlobalEnv::new();
    let ctx = Context::new();
    let n_ty = nat(&mut env);
    let p_family = post(
        &mut env,
        "P",
        Term::pi(n_ty.clone(), Term::Omega(Level::zero())),
    );
    let a = post(&mut env, "a", n_ty.clone());
    let b = post(&mut env, "b", n_ty.clone());
    let p = post(&mut env, "p", Term::app(p_family.clone(), a.clone()));
    let q = post(&mut env, "q", Term::app(p_family.clone(), b.clone()));
    let h = post(&mut env, "h", eq(n_ty.clone(), a.clone(), b.clone()));
    let subset = Term::sigma(n_ty.clone(), Term::app(weaken(&p_family, 1), Term::var(0)));
    let equality = eq(
        subset,
        Term::pair(a.clone(), p),
        Term::pair(b.clone(), q.clone()),
    );
    let expected = Term::sigma(
        eq(n_ty, a, b.clone()),
        eq(
            weaken(&Term::app(p_family, b), 1),
            weaken(&q, 1),
            weaken(&q, 1),
        ),
    );
    assert_eq!(infer(&env, &ctx, &equality), Ok(Term::Omega(Level::zero())));
    assert_eq!(whnf(&env, &ctx, &equality), expected);
    assert_eq!(infer(&env, &ctx, &expected), Ok(Term::Omega(Level::zero())));
    assert_eq!(
        check(
            &env,
            &ctx,
            &Term::pair(h, Term::Refl(Box::new(q))),
            &equality
        ),
        Ok(())
    );
}

#[test]
fn eq_inductive_omega_field_dependent_type() {
    // Conformance: observational/eq-inductive-omega-field-dependent-type.
    // Open n≠m force a later Type field through its dependent J path; the
    // two Ω-field proofs p and q are distinct, pinning R3's target-side Eq
    // syntactically, before Ω proof irrelevance could hide the wrong source.
    let mut env = GlobalEnv::new();
    let ctx = Context::new();
    let n_ty = nat(&mut env);
    let p_ty = post(&mut env, "P", Term::Omega(Level::zero()));
    let t = post(
        &mut env,
        "T",
        Term::pi(n_ty.clone(), Term::Type(Level::zero())),
    );
    let d_id = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![
                n_ty.clone(),
                p_ty.clone(),
                Term::app(weaken(&t, 2), Term::var(1)),
            ],
            target_indices: vec![],
        }],
    })
    .expect("D : Type 0 with Ω field and dependent Type field");
    let d = Term::indformer(d_id, vec![]);
    let node = Term::constructor(env.inductive(d_id).unwrap().constructors[0].id, vec![]);
    let n = post(&mut env, "n", n_ty.clone());
    let m = post(&mut env, "m", n_ty.clone());
    let h = post(&mut env, "h", eq(n_ty.clone(), n.clone(), m.clone()));
    let p = post(&mut env, "p", p_ty.clone());
    let q = post(&mut env, "q", p_ty.clone());
    assert_ne!(p, q, "the proof-source axis must not be degenerate");
    let x = post(&mut env, "x", Term::app(t.clone(), n.clone()));
    let y = post(&mut env, "y", Term::app(t.clone(), m.clone()));
    let prefix = Term::sigma(n_ty.clone(), p_ty.clone());
    let prefix_eq = eq(
        prefix,
        Term::pair(n.clone(), p.clone()),
        Term::pair(m.clone(), q.clone()),
    );
    assert_eq!(
        infer(&env, &ctx, &prefix_eq),
        Ok(Term::Omega(Level::zero()))
    );
    let _e_prefix = post(&mut env, "e_prefix", prefix_eq);
    let equality = eq(
        d,
        app3(node.clone(), n.clone(), p, x.clone()),
        app3(node, m.clone(), q.clone(), y.clone()),
    );
    assert_eq!(infer(&env, &ctx, &equality), Ok(Term::Omega(Level::zero())));
    let reduct = whnf(&env, &ctx, &equality);
    let Term::Sigma(first, tail) = &reduct else {
        panic!("same-constructor Eq must expose the first-field equality: {reduct:?}")
    };
    assert_eq!(**first, eq(n_ty, n.clone(), m.clone()));
    let Term::Sigma(proof_conjunct, last) = tail.as_ref() else {
        panic!("the Ω field and the dependent Type field must both be present: {tail:?}")
    };
    assert_eq!(
        **proof_conjunct,
        eq(weaken(&p_ty, 1), weaken(&q, 1), weaken(&q, 1)),
        "R3 must use the target proof twice, syntactically, not only up to Ω-PI"
    );
    let Term::Eq(carrier, left, right) = last.as_ref() else {
        panic!("dependent final field must remain an equality: {last:?}")
    };
    assert_eq!(**carrier, weaken(&Term::app(t.clone(), m.clone()), 2));
    assert_eq!(**right, weaken(&y, 2));
    let Term::Cast(source, destination, _evidence, transported) = left.as_ref() else {
        panic!("non-Ω dependent Type field must still transport: {left:?}")
    };
    assert_eq!(**source, weaken(&Term::app(t.clone(), n), 2));
    assert_eq!(**destination, weaken(&Term::app(t, m), 2));
    assert_eq!(**transported, weaken(&x, 2));
    assert_eq!(infer(&env, &ctx, &reduct), Ok(Term::Omega(Level::zero())));

    // The row's witness is (h, (refl q, r)). Bind r at the independently
    // inspected dependent-field equality after supplying h and refl q.
    let Term::Sigma(_, after_h) = subst0(tail, &h) else {
        panic!("the Ω field must remain a conjunct after the first witness")
    };
    let r_ty = subst0(&after_h, &Term::Refl(Box::new(q.clone())));
    let r = post(&mut env, "r", r_ty);
    assert_eq!(
        check(
            &env,
            &ctx,
            &Term::pair(h, Term::pair(Term::Refl(Box::new(q)), r)),
            &equality,
        ),
        Ok(())
    );
}

#[test]
fn eq_inductive_omega_field_irrelevant_dependency() {
    // Conformance: observational/eq-inductive-omega-field-irrelevant-dependency.
    // U n p and U n q convert only because their Ω arguments are irrelevant.
    let mut env = GlobalEnv::new();
    let ctx = Context::new();
    let n_ty = nat(&mut env);
    let p_ty = post(&mut env, "P", Term::Omega(Level::zero()));
    let u = post(
        &mut env,
        "U",
        Term::pi(
            n_ty.clone(),
            Term::pi(p_ty.clone(), Term::Type(Level::zero())),
        ),
    );
    let e_id = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![
                n_ty.clone(),
                p_ty.clone(),
                app2(weaken(&u, 2), Term::var(1), Term::var(0)),
            ],
            target_indices: vec![],
        }],
    })
    .expect("E : Type 0 with a proof-dependent Type field");
    let mk = Term::constructor(env.inductive(e_id).unwrap().constructors[0].id, vec![]);
    let n = post(&mut env, "n", n_ty.clone());
    let p = post(&mut env, "p", p_ty.clone());
    let q = post(&mut env, "q", p_ty.clone());
    assert_ne!(p, q);
    let x = post(&mut env, "x", app2(u.clone(), n.clone(), p.clone()));
    let y = post(&mut env, "y", app2(u.clone(), n.clone(), q.clone()));
    let r = post(
        &mut env,
        "r",
        eq(app2(u, n.clone(), q.clone()), x.clone(), y.clone()),
    );
    let equality = eq(
        Term::indformer(e_id, vec![]),
        app3(mk.clone(), n.clone(), p, x),
        app3(mk, n.clone(), q.clone(), y),
    );
    assert_eq!(infer(&env, &ctx, &equality), Ok(Term::Omega(Level::zero())));
    let reduct = whnf(&env, &ctx, &equality);
    let Term::Sigma(_, tail) = &reduct else {
        panic!("same-constructor equality must decompose: {reduct:?}")
    };
    let Term::Sigma(proof_conjunct, _) = tail.as_ref() else {
        panic!("the Ω proof field must be retained: {tail:?}")
    };
    assert_eq!(
        **proof_conjunct,
        eq(weaken(&p_ty, 1), weaken(&q, 1), weaken(&q, 1))
    );
    assert_eq!(infer(&env, &ctx, &reduct), Ok(Term::Omega(Level::zero())));
    assert_eq!(
        check(
            &env,
            &ctx,
            &Term::pair(
                Term::Refl(Box::new(n)),
                Term::pair(Term::Refl(Box::new(q)), r)
            ),
            &equality,
        ),
        Ok(())
    );
}

#[test]
fn eq_trunc_omega_level_one_remains_neutral() {
    // Conformance: observational/eq-trunc-omega-level-one. R1 and deletion
    // of the Trunc arm independently protect this non-cumulative Ω₁ case.
    let mut env = GlobalEnv::new();
    let ctx = Context::new();
    let a = post(&mut env, "A", Term::Type(Level::zero().suc()));
    let trunc = Term::Trunc(Box::new(a));
    let u = post(&mut env, "u", trunc.clone());
    let v = post(&mut env, "v", trunc.clone());
    let equality = eq(trunc, u.clone(), v);
    assert_eq!(
        infer(&env, &ctx, &equality),
        Ok(Term::Omega(Level::zero().suc()))
    );
    assert_eq!(whnf(&env, &ctx, &equality), equality);
    assert_eq!(
        check(&env, &ctx, &Term::Refl(Box::new(u)), &equality),
        Ok(())
    );
}

#[test]
fn eq_canonical_omega_carriers_remain_neutral() {
    // Conformance: observational/eq-canonical-omega-carriers-neutral.
    // Canonical Σ and Π heads distinguish the R1 guard from a Trunc-only fix.
    let mut env = GlobalEnv::new();
    let ctx = Context::new();
    let p = post(&mut env, "P", Term::Omega(Level::zero()));
    let q = post(&mut env, "Q", Term::Omega(Level::zero()));
    let a = post(&mut env, "A", Term::Type(Level::zero()));
    let conjunction = Term::sigma(p.clone(), weaken(&q, 1));
    let u = post(&mut env, "u", conjunction.clone());
    let v = post(&mut env, "v", conjunction.clone());
    let function = Term::pi(a, weaken(&q, 1));
    let f = post(&mut env, "f", function.clone());
    let g = post(&mut env, "g", function.clone());
    for (carrier, left, right) in [(conjunction, u, v), (function, f, g)] {
        let equality = eq(carrier, left.clone(), right);
        assert_eq!(infer(&env, &ctx, &equality), Ok(Term::Omega(Level::zero())));
        assert_eq!(whnf(&env, &ctx, &equality), equality);
        assert_eq!(
            check(&env, &ctx, &Term::Refl(Box::new(left)), &equality),
            Ok(())
        );
    }
}

#[test]
fn eq_omega_carrier_admitted_while_cast_and_non_type_eq_refuse() {
    // Conformance: observational/eq-omega-carrier-boundary-pair. Only Eq's
    // carrier classification widens; Cast remains Type-only and a value is
    // still neither a Type nor an Ω-classified carrier.
    let mut env = GlobalEnv::new();
    let ctx = Context::new();
    let p = post(&mut env, "P", Term::Omega(Level::zero()));
    let u = post(&mut env, "u", p.clone());
    let v = post(&mut env, "v", p.clone());
    let equality = eq(p.clone(), u.clone(), v);
    assert_eq!(infer(&env, &ctx, &equality), Ok(Term::Omega(Level::zero())));
    assert_eq!(whnf(&env, &ctx, &equality), equality);
    assert_eq!(
        check(&env, &ctx, &Term::Refl(Box::new(u.clone())), &equality),
        Ok(())
    );
    let e = post(
        &mut env,
        "e",
        eq(Term::Omega(Level::zero()), p.clone(), p.clone()),
    );
    let cast = Term::Cast(Box::new(p.clone()), Box::new(p), Box::new(e), Box::new(u));
    assert_eq!(
        infer(&env, &ctx, &cast),
        Err(KernelError::TypeMismatch {
            expected: Box::new(Term::Type(Level::Var(LevelVar(0)))),
            found: Box::new(Term::Omega(Level::zero())),
        })
    );
    let n_ty = nat(&mut env);
    let n = post(&mut env, "n", n_ty.clone());
    assert_eq!(
        infer(&env, &ctx, &eq(n.clone(), n.clone(), n)),
        Err(KernelError::TypeMismatch {
            expected: Box::new(Term::Type(Level::Var(LevelVar(0)))),
            found: Box::new(n_ty),
        })
    );
}

#[test]
fn eq_type_inductive_omega_parameter_compares_target_proof() {
    // Eq-Type caller of the shared inductive conjunct builder (`16 §2.2`).
    // On the pre-R3 kernel this well-formed Eq reduced to an ill-formed
    // `Eq P p q`; R3 compares the target proof with itself on both callers.
    let mut env = GlobalEnv::new();
    let ctx = Context::new();
    let p_ty = post(&mut env, "P", Term::Omega(Level::zero()));
    let d_id = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![p_ty.clone()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .expect("D : P -> Type 0");
    let d = |arg: Term| Term::app(Term::indformer(d_id, vec![]), arg);
    let p = post(&mut env, "p", p_ty.clone());
    let q = post(&mut env, "q", p_ty.clone());
    assert_ne!(p, q);
    let equality = eq(Term::Type(Level::zero()), d(p), d(q.clone()));
    let reduct = whnf(&env, &ctx, &equality);
    assert_eq!(reduct, eq(p_ty, q.clone(), q.clone()));
    assert_eq!(infer(&env, &ctx, &reduct), Ok(Term::Omega(Level::zero())));
    assert_eq!(check(&env, &ctx, &Term::Refl(Box::new(q)), &reduct), Ok(()));
}

#[test]
fn construction_without_postulates_keeps_trusted_base_identical() {
    // AC-3: this control uses no test-only postulates, so a fixture's own
    // trust-inventory growth cannot mask a kernel/prelude trust-base delta.
    let mut env = GlobalEnv::new();
    let before = env.trusted_base();
    let n = nat(&mut env);
    let id = env
        .inductive(match n {
            Term::IndFormer { id, .. } => id,
            _ => unreachable!(),
        })
        .unwrap()
        .constructors[0]
        .id;
    let z = Term::constructor(id, vec![]);
    let equality = eq(n, z.clone(), z);
    assert_eq!(
        infer(&env, &Context::new(), &equality),
        Ok(Term::Omega(Level::zero()))
    );
    assert_eq!(env.trusted_base(), before);
}
