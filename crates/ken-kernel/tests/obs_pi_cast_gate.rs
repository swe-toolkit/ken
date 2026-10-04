//! Durable invariants: observational casts and J fire only with the side
//! conditions their reducts need (10-kernel/16 §3.2, 15 §4).
use ken_kernel::env::Context;
use ken_kernel::term::{GlobalId, Level, LevelVar, Term};
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

fn family(
    env: &mut GlobalEnv,
    levels: Vec<LevelVar>,
    params: Vec<Term>,
    indices: Vec<Term>,
    ctors: Vec<CtorSpec>,
) -> GlobalId {
    declare_inductive(env, |_| InductiveSpec {
        level_params: levels,
        params,
        indices,
        level: Level::zero(),
        constructors: ctors,
    })
    .unwrap()
}

fn ctor(env: &GlobalEnv, family: GlobalId, index: usize, levels: Vec<Level>) -> Term {
    Term::Constructor {
        id: env.inductive(family).unwrap().constructors[index].id,
        level_args: levels,
    }
}

fn cast(env: &mut GlobalEnv, source: Term, target: Term, value: Term) -> (Context, Term) {
    let ctx = Context::new();
    let Term::Type(level) = infer(env, &ctx, &source).unwrap() else {
        panic!("source must be Type-sorted");
    };
    assert_eq!(infer(env, &ctx, &target), Ok(Term::Type(level.clone())));
    let equality = Term::Eq(
        Box::new(Term::Type(level)),
        Box::new(source.clone()),
        Box::new(target.clone()),
    );
    let e = opaque(env, "endpoint_equality", equality);
    let term = Term::Cast(
        Box::new(source),
        Box::new(target.clone()),
        Box::new(e),
        Box::new(value),
    );
    assert_eq!(infer(env, &ctx, &term), Ok(target));
    (ctx, term)
}

fn assert_stuck(env: &GlobalEnv, ctx: &Context, redex: &Term) {
    let trusted = env.trusted_base();
    assert_eq!(whnf(env, ctx, redex), *redex);
    assert_eq!(env.trusted_base(), trusted);
}

fn assert_reduct_checks(env: &GlobalEnv, ctx: &Context, redex: &Term) -> Term {
    let expected = infer(env, ctx, redex).unwrap();
    let trusted = env.trusted_base();
    let reduct = whnf(env, ctx, redex);
    assert_ne!(reduct, *redex, "a valid rule must still reduce");
    assert_eq!(check(env, ctx, &reduct, &expected), Ok(()));
    assert_eq!(env.trusted_base(), trusted);
    reduct
}

#[test]
fn pi_codomain_level_mismatch_stays_neutral() {
    let mut env = GlobalEnv::new();
    let nat = Term::indformer(
        family(
            &mut env,
            vec![],
            vec![],
            vec![],
            vec![CtorSpec {
                args: vec![],
                target_indices: vec![],
            }],
        ),
        vec![],
    );
    let ty0 = Term::Type(Level::zero());
    let source = Term::pi(ty0.clone(), nat);
    let target = Term::pi(ty0.clone(), ty0.clone());
    let value = opaque(&mut env, "f_level", source.clone());
    let (ctx, redex) = cast(&mut env, source, target, value);
    assert_stuck(&env, &ctx, &redex);
}

#[test]
fn pi_dependent_codomain_level_mismatch_stays_neutral() {
    let mut env = GlobalEnv::new();
    let ty0 = Term::Type(Level::zero());
    let source = Term::pi(ty0.clone(), Term::var(0));
    let target = Term::pi(ty0.clone(), ty0);
    let value = opaque(&mut env, "f_dependent", source.clone());
    let (ctx, redex) = cast(&mut env, source, target, value);
    assert_stuck(&env, &ctx, &redex);
}

#[test]
fn pi_decomposition_still_reduces_and_checks() {
    let mut env = GlobalEnv::new();
    let nat = Term::indformer(
        family(
            &mut env,
            vec![],
            vec![],
            vec![],
            vec![CtorSpec {
                args: vec![],
                target_indices: vec![],
            }],
        ),
        vec![],
    );
    let a = opaque(&mut env, "A", Term::Type(Level::zero()));
    let b = opaque(&mut env, "B", Term::Type(Level::zero()));
    let source = Term::pi(nat.clone(), a);
    let target = Term::pi(nat, b);
    let value = opaque(&mut env, "f_components", source.clone());
    let (ctx, redex) = cast(&mut env, source, target, value);
    assert!(matches!(
        assert_reduct_checks(&env, &ctx, &redex),
        Term::Lam(..)
    ));
}

#[test]
fn pi_omega_to_type_codomain_still_reduces_and_checks() {
    let mut env = GlobalEnv::new();
    let ty0 = Term::Type(Level::zero());
    let source = Term::pi(ty0.clone(), Term::Omega(Level::zero()));
    let target = Term::pi(ty0.clone(), ty0);
    let value = opaque(&mut env, "f_omega", source.clone());
    let (ctx, redex) = cast(&mut env, source, target, value);
    assert!(matches!(
        assert_reduct_checks(&env, &ctx, &redex),
        Term::Lam(..)
    ));
}

/// AC-0's dependent-earlier-index instance exposes a typed Sigma telescope,
/// but its changed first index is Type/opaque, not same-constructor-headed.
/// Thus the old index-inversion guard itself prevents the cast from firing.
#[test]
fn dependent_earlier_index_decomposes_but_nonconstructor_change_stays_neutral() {
    let mut env = GlobalEnv::new();
    let l0 = Level::zero();
    let l1 = l0.clone().suc();
    let l2 = l1.clone().suc();
    let d = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![Term::Type(l1.clone()), Term::var(0)],
        level: l2.clone(),
        constructors: vec![CtorSpec {
            args: vec![Term::Type(l1.clone()), Term::var(0)],
            target_indices: vec![Term::var(1), Term::var(0)],
        }],
    })
    .unwrap();
    let x0 = opaque(&mut env, "x0", Term::Type(l0.clone()));
    let a1 = opaque(&mut env, "A1", Term::Type(l1));
    let x1 = opaque(&mut env, "x1", a1.clone());
    let former = Term::indformer(d, vec![]);
    let source = Term::app(
        Term::app(former.clone(), Term::Type(l0.clone())),
        x0.clone(),
    );
    let target = Term::app(Term::app(former, a1), x1);
    let eq = Term::Eq(
        Box::new(Term::Type(l2)),
        Box::new(source.clone()),
        Box::new(target.clone()),
    );
    let reduct = whnf(&env, &Context::new(), &eq);
    assert!(matches!(reduct, Term::Sigma(..)));
    assert!(infer(&env, &Context::new(), &reduct).is_ok());
    let value = Term::app(Term::app(ctor(&env, d, 0, vec![]), Term::Type(l0)), x0);
    let (ctx, redex) = cast(&mut env, source, target, value);
    assert_stuck(&env, &ctx, &redex);
}

/// A one-index family exposes a single Eq component, not an outer Sigma.
/// An unhandled *constant* target-index template must not be silently carried.
#[test]
fn index_rewrite_refuses_unhandled_template_but_preserves_forced_argument() {
    let mut env = GlobalEnv::new();
    let bool_id = family(
        &mut env,
        vec![],
        vec![],
        vec![],
        vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
        ],
    );
    let bool_ty = Term::indformer(bool_id, vec![]);
    let left = ctor(&env, bool_id, 0, vec![]);
    let right = ctor(&env, bool_id, 1, vec![]);
    let rel = opaque(
        &mut env,
        "R",
        Term::pi(
            bool_ty.clone(),
            Term::pi(bool_ty.clone(), Term::Omega(Level::zero())),
        ),
    );
    let quotient = Term::Quot(Box::new(bool_ty), Box::new(rel));
    let qleft = Term::QuotClass(Box::new(left));
    let qright = Term::QuotClass(Box::new(right));
    let box_id = family(
        &mut env,
        vec![],
        vec![],
        vec![],
        vec![CtorSpec {
            args: vec![quotient.clone()],
            target_indices: vec![],
        }],
    );
    let box_ty = Term::indformer(box_id, vec![]);
    let bx = ctor(&env, box_id, 0, vec![]);
    let indexed = family(
        &mut env,
        vec![],
        vec![],
        vec![box_ty],
        vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![Term::app(bx.clone(), qleft.clone())],
            },
            CtorSpec {
                args: vec![quotient],
                target_indices: vec![Term::app(bx, Term::var(0))],
            },
        ],
    );
    let idx_left = Term::app(ctor(&env, box_id, 0, vec![]), qleft.clone());
    let idx_right = Term::app(ctor(&env, box_id, 0, vec![]), qright.clone());
    let source = Term::app(Term::indformer(indexed, vec![]), idx_left);
    let target = Term::app(Term::indformer(indexed, vec![]), idx_right);
    let eq = Term::Eq(
        Box::new(Term::Type(Level::zero())),
        Box::new(source.clone()),
        Box::new(target.clone()),
    );
    assert!(
        matches!(whnf(&env, &Context::new(), &eq), Term::Eq(..)),
        "single Eq telescope exists"
    );
    let bad = ctor(&env, indexed, 0, vec![]);
    let good = Term::app(ctor(&env, indexed, 1, vec![]), qleft);
    let (ctx, redex) = cast(&mut env, source.clone(), target.clone(), bad);
    assert_stuck(&env, &ctx, &redex);
    let (ctx, redex) = cast(&mut env, source, target, good);
    let reduct = assert_reduct_checks(&env, &ctx, &redex);
    assert_eq!(reduct, Term::app(ctor(&env, indexed, 1, vec![]), qright));
}

/// The second changed index must be checked even after the first exposes a
/// forced argument. A guard that checks only the first template misses this.
#[test]
fn index_rewrite_checks_later_template_after_earlier_forced_position() {
    let mut env = GlobalEnv::new();
    let bool_id = family(
        &mut env,
        vec![],
        vec![],
        vec![],
        vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
        ],
    );
    let bool_ty = Term::indformer(bool_id, vec![]);
    let rel = opaque(
        &mut env,
        "R_later",
        Term::pi(
            bool_ty.clone(),
            Term::pi(bool_ty.clone(), Term::Omega(Level::zero())),
        ),
    );
    let quotient = Term::Quot(Box::new(bool_ty), Box::new(rel));
    let qleft = Term::QuotClass(Box::new(ctor(&env, bool_id, 0, vec![])));
    let qright = Term::QuotClass(Box::new(ctor(&env, bool_id, 1, vec![])));
    let box_id = family(
        &mut env,
        vec![],
        vec![],
        vec![],
        vec![CtorSpec {
            args: vec![quotient.clone()],
            target_indices: vec![],
        }],
    );
    let bx = ctor(&env, box_id, 0, vec![]);
    let box_ty = Term::indformer(box_id, vec![]);
    let indexed = family(
        &mut env,
        vec![],
        vec![],
        vec![box_ty.clone(), box_ty],
        vec![
            CtorSpec {
                args: vec![quotient.clone()],
                target_indices: vec![
                    Term::app(bx.clone(), Term::var(0)),
                    Term::app(bx.clone(), qleft.clone()),
                ],
            },
            CtorSpec {
                args: vec![quotient],
                target_indices: vec![
                    Term::app(bx.clone(), Term::var(0)),
                    Term::app(bx.clone(), Term::var(0)),
                ],
            },
        ],
    );
    let former = Term::indformer(indexed, vec![]);
    let source_index = Term::app(bx.clone(), qleft.clone());
    let target_index = Term::app(bx, qright.clone());
    let source = Term::app(
        Term::app(former.clone(), source_index.clone()),
        source_index,
    );
    let target = Term::app(Term::app(former, target_index.clone()), target_index);
    let incorrect = Term::app(ctor(&env, indexed, 0, vec![]), qleft.clone());
    let (ctx, bad) = cast(&mut env, source.clone(), target.clone(), incorrect);
    assert_stuck(&env, &ctx, &bad);
    let valid = Term::app(ctor(&env, indexed, 1, vec![]), qleft);
    let (ctx, good) = cast(&mut env, source, target, valid);
    assert_eq!(
        assert_reduct_checks(&env, &ctx, &good),
        Term::app(ctor(&env, indexed, 1, vec![]), qright)
    );
}

/// The parameter arm's two constructors differ only in whether their target
/// index follows the parameter or stays at the original fixed quotient class.
fn parameter_rewrite_fixture() -> (GlobalEnv, GlobalId, Term, Term, Term, Term) {
    let mut env = GlobalEnv::new();
    let bool_id = family(
        &mut env,
        vec![],
        vec![],
        vec![],
        vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
        ],
    );
    let bool_ty = Term::indformer(bool_id, vec![]);
    let rel = opaque(
        &mut env,
        "R_param",
        Term::pi(
            bool_ty.clone(),
            Term::pi(bool_ty.clone(), Term::Omega(Level::zero())),
        ),
    );
    let q = Term::Quot(Box::new(bool_ty), Box::new(rel));
    let qleft = Term::QuotClass(Box::new(ctor(&env, bool_id, 0, vec![])));
    let qright = Term::QuotClass(Box::new(ctor(&env, bool_id, 1, vec![])));
    let indexed = family(
        &mut env,
        vec![],
        vec![q.clone()],
        vec![q],
        vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![Term::var(0)],
            },
            CtorSpec {
                args: vec![],
                target_indices: vec![qleft.clone()],
            },
        ],
    );
    let former = Term::indformer(indexed, vec![]);
    let source = Term::app(Term::app(former.clone(), qleft.clone()), qleft.clone());
    let target = Term::app(Term::app(former, qright.clone()), qleft.clone());
    (env, indexed, source, target, qleft, qright)
}

/// Rebuilding `rfl qright` changes the target index away from `qleft`.
#[test]
fn parameter_rewrite_checks_rebuilt_target_index() {
    let (mut env, indexed, source, target, qleft, _) = parameter_rewrite_fixture();
    let value = Term::app(ctor(&env, indexed, 0, vec![]), qleft);
    let (ctx, redex) = cast(&mut env, source, target, value);
    assert_stuck(&env, &ctx, &redex);
}

/// `fixed qright` still targets `qleft`, so the same parameter arm reduces.
#[test]
fn parameter_rewrite_preserves_fixed_target_index() {
    let (mut env, indexed, source, target, qleft, qright) = parameter_rewrite_fixture();
    let value = Term::app(ctor(&env, indexed, 1, vec![]), qleft);
    let (ctx, redex) = cast(&mut env, source, target, value);
    assert_eq!(
        assert_reduct_checks(&env, &ctx, &redex),
        Term::app(ctor(&env, indexed, 1, vec![]), qright)
    );
}

fn level_cast_fixture() -> (GlobalEnv, GlobalId, Term, Term, Term, Term, Term, Level) {
    let mut env = GlobalEnv::new();
    let indexed = family(
        &mut env,
        vec![LevelVar(0)],
        vec![Term::Type(Level::zero())],
        vec![],
        vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    );
    let p = opaque(&mut env, "level_P", Term::Type(Level::zero()));
    let q = opaque(&mut env, "level_Q", Term::Type(Level::zero()));
    let l0 = Level::zero();
    let l1 = l0.clone().suc();
    let source = Term::app(Term::indformer(indexed, vec![l0.clone()]), p.clone());
    let target_mismatch = Term::app(Term::indformer(indexed, vec![l1]), q.clone());
    let value = Term::app(ctor(&env, indexed, 0, vec![l0.clone()]), p);
    let target = Term::app(Term::indformer(indexed, vec![l0.clone()]), q.clone());
    (env, indexed, source, target_mismatch, target, value, q, l0)
}

#[test]
fn level_instantiation_mismatch_stays_neutral() {
    let (mut env, _, source, target_mismatch, _, value, _, _) = level_cast_fixture();
    let (ctx, redex) = cast(&mut env, source, target_mismatch, value);
    assert_stuck(&env, &ctx, &redex);
}

#[test]
fn level_same_instantiation_still_computes() {
    let (mut env, indexed, source, _, target, value, q, l0) = level_cast_fixture();
    let (ctx, redex) = cast(&mut env, source, target, value);
    assert_eq!(
        assert_reduct_checks(&env, &ctx, &redex),
        Term::app(ctor(&env, indexed, 0, vec![l0]), q)
    );
}

fn j_carrier_nat_env() -> (GlobalEnv, Term, Term) {
    let mut env = GlobalEnv::new();
    let nat_id = family(
        &mut env,
        vec![],
        vec![],
        vec![],
        vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    );
    let nat_ty = Term::indformer(nat_id, vec![]);
    let nat_zero = ctor(&env, nat_id, 0, vec![]);
    (env, nat_ty, nat_zero)
}

/// A constant lambda deliberately ignores its ill-typed second argument on
/// beta reduction. The old first-domain-only inference accepted this J.
#[test]
fn j_motive_rejects_wrong_second_domain_but_accepts_equality_domain() {
    let (mut env, nat, zero) = j_carrier_nat_env();
    let a_ty = opaque(&mut env, "J_A", Term::Type(Level::zero()));
    let value = opaque(&mut env, "J_a", a_ty.clone());
    let mut ctx = Context::new();
    let eq = Term::Eq(
        Box::new(a_ty.clone()),
        Box::new(value.clone()),
        Box::new(value.clone()),
    );
    assert_eq!(whnf(&env, &ctx, &eq), eq, "neutral carrier keeps Eq open");
    ctx.push(eq.clone());
    let expected = Term::Eq(
        Box::new(a_ty.clone()),
        Box::new(value),
        Box::new(Term::var(0)),
    );
    let wrong_type = Term::pi(
        a_ty.clone(),
        Term::pi(nat.clone(), Term::Type(Level::zero())),
    );
    let bad_motive = Term::Ascript(
        Box::new(Term::lam(a_ty.clone(), Term::lam(nat.clone(), nat.clone()))),
        Box::new(wrong_type),
    );
    let bad = Term::J(
        Box::new(bad_motive),
        Box::new(zero.clone()),
        Box::new(Term::var(0)),
    );
    let trusted = env.trusted_base();
    assert_eq!(
        infer(&env, &ctx, &bad),
        Err(KernelError::BadEliminator(
            "J motive's second domain ≠ Eq A a b".into()
        ))
    );
    let good_type = Term::pi(
        a_ty.clone(),
        Term::pi(expected.clone(), Term::Type(Level::zero())),
    );
    let good_motive = Term::Ascript(
        Box::new(Term::lam(a_ty, Term::lam(expected, nat.clone()))),
        Box::new(good_type),
    );
    let good = Term::J(
        Box::new(good_motive),
        Box::new(zero),
        Box::new(Term::var(0)),
    );
    let inferred = infer(&env, &ctx, &good).expect("valid J motive");
    assert!(ken_kernel::convert_type(&env, &ctx, &inferred, &nat));
    assert_eq!(infer(&env, &ctx, &inferred), Ok(Term::Type(Level::zero())));
    assert_eq!(env.trusted_base(), trusted);
}
