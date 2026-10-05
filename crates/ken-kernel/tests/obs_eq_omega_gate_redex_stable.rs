//! KERNEL-EQ-OMEGA-GATE-REDEX-STABLE, `16 §2.2, §8.4`.
//! Durable invariants: substitution of a lambda-valued family does not change
//! the three Omega gates. Each redex case has a postulated-family control.

use ken_kernel::env::Context;
use ken_kernel::subst::weaken;
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    declare_inductive, declare_postulate, infer, normalize, whnf, CtorSpec, GlobalEnv,
    InductiveSpec, KernelError,
};

fn post(env: &mut GlobalEnv, label: &str, ty: Term) -> Term {
    Term::const_(
        declare_postulate(env, label.into(), vec![], ty).unwrap(),
        vec![],
    )
}

fn eq(ty: Term, lhs: Term, rhs: Term) -> Term {
    Term::Eq(Box::new(ty), Box::new(lhs), Box::new(rhs))
}

fn app2(f: Term, x: Term, y: Term) -> Term {
    Term::app(Term::app(f, x), y)
}

fn setup() -> (GlobalEnv, Term, Term, Term, Term) {
    let mut env = GlobalEnv::new();
    let n_id = declare_inductive(&mut env, |_| InductiveSpec {
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
    let n = Term::indformer(n_id, vec![]);
    let z = Term::constructor(env.inductive(n_id).unwrap().constructors[0].id, vec![]);
    let p = post(&mut env, "P", Term::Omega(Level::zero()));
    let q = post(&mut env, "Q", Term::Omega(Level::zero()));
    (env, n, z, p, q)
}

/// R1 must not ask the context to type a known inductive-headed carrier.
/// The deliberately open X is absent from the context, as in the catalog
/// trace; xs is a constructor application so R1 can expose its decomposition.
#[test]
fn inductive_headed_carrier_decomposes_without_a_typing_context() {
    let mut env = GlobalEnv::new();
    let list_id = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![Term::Type(Level::zero())],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .expect("a one-parameter Type-valued inductive");
    let x = Term::var(0);
    let list_x = Term::app(Term::indformer(list_id, vec![]), x.clone());
    let nil = Term::constructor(env.inductive(list_id).unwrap().constructors[0].id, vec![]);
    let xs = Term::app(nil, x);
    let ctx = Context::new();
    assert_eq!(
        infer(&env, &ctx, &list_x),
        Err(KernelError::VarOutOfScope { index: 0, depth: 0 }),
        "this raw reduction cannot depend on classification in an empty context"
    );
    assert_eq!(
        ken_kernel::obs::eq_reduce(&env, &ctx, &list_x, &xs, &xs),
        Some(ken_kernel::obs::top_term(&env)),
        "Eq (List X) xs xs decomposes despite X being free"
    );
}

fn family(n: &Term, prop: &Term) -> Term {
    Term::lam(n.clone(), weaken(prop, 1))
}

fn family_type(n: &Term) -> Term {
    Term::pi(n.clone(), Term::Omega(Level::zero()))
}

fn assert_well_formed_reduct(env: &GlobalEnv, equality: &Term, lambda: bool) -> Term {
    let ctx = Context::new();
    assert_eq!(
        infer(env, &ctx, equality),
        Ok(Term::Omega(Level::zero().suc()))
    );
    let reduct = whnf(env, &ctx, equality);
    if lambda {
        let normalized = normalize(env, &ctx, &reduct);
        let inferred = infer(env, &ctx, &normalized);
        assert!(
            inferred.is_ok(),
            "normal reduct must form: {normalized:?}, {inferred:?}"
        );
    }
    reduct
}

fn r3_case(lambda: bool, different: bool) {
    let (mut env, n, z, p_ty, q_ty) = setup();
    let ft = family_type(&n);
    let src_f = if lambda {
        family(&n, &p_ty)
    } else {
        post(&mut env, "F", ft.clone())
    };
    let dst_f = if different {
        if lambda {
            family(&n, &q_ty)
        } else {
            post(&mut env, "G", ft.clone())
        }
    } else {
        src_f.clone()
    };
    let src_ty = if lambda {
        p_ty.clone()
    } else {
        Term::app(src_f.clone(), z.clone())
    };
    let dst_ty = if lambda {
        if different {
            q_ty
        } else {
            p_ty
        }
    } else {
        Term::app(dst_f.clone(), z.clone())
    };
    let src = post(&mut env, "p", src_ty);
    let dst = post(&mut env, "q", dst_ty);
    let d_id = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero().suc(),
        constructors: vec![CtorSpec {
            args: vec![ft.clone(), Term::app(Term::var(0), weaken(&z, 1))],
            target_indices: vec![],
        }],
    })
    .unwrap();
    let d = Term::indformer(d_id, vec![]);
    let mk = Term::constructor(env.inductive(d_id).unwrap().constructors[0].id, vec![]);
    let equality = eq(
        d,
        app2(mk.clone(), src_f, src),
        app2(mk, dst_f.clone(), dst.clone()),
    );
    let reduct = assert_well_formed_reduct(&env, &equality, lambda);
    let Term::Sigma(_, second) = reduct else {
        panic!("R3 must reduce to two conjuncts: {reduct:?}")
    };
    let target_type = Term::app(dst_f, z);
    assert_eq!(
        *second,
        eq(weaken(&target_type, 1), weaken(&dst, 1), weaken(&dst, 1))
    );
}

#[test]
fn r3_same_family_lambda_redex_compares_target_proof() {
    r3_case(true, false);
    r3_case(false, false); // control: postulated family, no lambda redex
}

#[test]
fn r3_distinct_families_lambda_redex_compares_target_proof() {
    r3_case(true, true);
    r3_case(false, true); // control: two postulated families
}

fn r2_case(lambda: bool) {
    let (mut env, n, z, p_ty, _) = setup();
    let ft = family_type(&n);
    let f = if lambda {
        family(&n, &p_ty)
    } else {
        post(&mut env, "F", ft.clone())
    };
    let proof_type = if lambda {
        p_ty
    } else {
        Term::app(f.clone(), z.clone())
    };
    let p = post(&mut env, "p", proof_type.clone());
    let q = post(&mut env, "q", proof_type);
    let subset = Term::sigma(ft.clone(), Term::app(Term::var(0), weaken(&z, 1)));
    let equality = eq(
        subset,
        Term::pair(f.clone(), p),
        Term::pair(f.clone(), q.clone()),
    );
    let reduct = assert_well_formed_reduct(&env, &equality, lambda);
    let Term::Sigma(_, second) = reduct else {
        panic!("R2 must reduce to two conjuncts: {reduct:?}")
    };
    let target_type = Term::app(f, z);
    assert_eq!(
        *second,
        eq(weaken(&target_type, 1), weaken(&q, 1), weaken(&q, 1))
    );
}

#[test]
fn r2_subset_lambda_redex_compares_target_proof() {
    r2_case(true);
    r2_case(false); // control: postulated family
}

fn r1_case(lambda: bool) {
    let (mut env, n, _z, p_ty, _) = setup();
    let ft = family_type(&n);
    let f = if lambda {
        family(&n, &p_ty)
    } else {
        post(&mut env, "F", ft.clone())
    };
    let field_ty = Term::pi(n.clone(), Term::app(weaken(&f, 1), Term::var(0)));
    let post_ty = if lambda {
        Term::pi(n.clone(), p_ty.clone())
    } else {
        field_ty.clone()
    };
    let f1 = post(&mut env, "f1", post_ty.clone());
    let g1 = post(&mut env, "g1", post_ty);
    let d_id = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero().suc(),
        constructors: vec![CtorSpec {
            args: vec![
                ft,
                Term::pi(n.clone(), Term::app(Term::var(1), Term::var(0))),
            ],
            target_indices: vec![],
        }],
    })
    .unwrap();
    let d = Term::indformer(d_id, vec![]);
    let mk = Term::constructor(env.inductive(d_id).unwrap().constructors[0].id, vec![]);
    let equality = eq(d, app2(mk.clone(), f.clone(), f1), app2(mk, f, g1.clone()));
    let reduct = assert_well_formed_reduct(&env, &equality, lambda);
    let Term::Sigma(_, second) = reduct else {
        panic!("R1 reach must expose two conjuncts: {reduct:?}")
    };
    let expected = eq(field_ty, g1.clone(), g1);
    assert_eq!(*second, expected);
    if lambda {
        assert_eq!(whnf(&env, &Context::new(), &second), *second);
    }
}

#[test]
fn r1_reached_through_inductive_lambda_field_stays_neutral() {
    r1_case(true);
    r1_case(false); // control: postulated family
}
