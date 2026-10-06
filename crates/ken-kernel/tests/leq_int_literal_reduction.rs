//! ADR 0013 Layer 2, spec 16 §2.2: only the registered `leq_int` on two
//! weak-head Int literals computes at kernel conversion. Promise class:
//! durable behavioral invariant (the positive, rejection, and neutral arms).

use ken_kernel::check::register_checked_int_lit_carrier;
use ken_kernel::env::{Context, PrimReduction};
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    check, declare_def, declare_inductive, declare_primitive, whnf, CtorSpec, GlobalEnv, GlobalId,
    InductiveSpec, KernelError,
};
use num_bigint::BigInt;

struct Fixture {
    env: GlobalEnv,
    int: GlobalId,
    leq: GlobalId,
    eq_op: GlobalId,
    add_op: GlobalId,
    bool_: GlobalId,
    true_: GlobalId,
    false_: GlobalId,
}

fn fixture() -> Fixture {
    let mut env = GlobalEnv::new();
    let bool_ = declare_inductive(&mut env, |_| InductiveSpec {
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
                args: vec![],
                target_indices: vec![],
            },
        ],
    })
    .expect("Bool");
    let true_ = env.inductive(bool_).unwrap().constructors[0].id;
    let false_ = env.inductive(bool_).unwrap().constructors[1].id;
    let int = declare_primitive(
        &mut env,
        vec![],
        Term::Type(Level::zero()),
        PrimReduction::OpaqueType,
    )
    .expect("Int");
    register_checked_int_lit_carrier(&mut env, int).unwrap();
    let int_ty = Term::const_(int, vec![]);
    let cmp_ty = Term::pi(
        int_ty.clone(),
        Term::pi(int_ty.clone(), Term::indformer(bool_, vec![])),
    );
    let leq = declare_primitive(
        &mut env,
        vec![],
        cmp_ty.clone(),
        PrimReduction::Op { symbol: "leq_int" },
    )
    .expect("leq_int");
    let eq_op = declare_primitive(
        &mut env,
        vec![],
        cmp_ty,
        PrimReduction::Op { symbol: "eq_int" },
    )
    .expect("eq_int");
    let add_op = declare_primitive(
        &mut env,
        vec![],
        Term::pi(int_ty.clone(), Term::pi(int_ty.clone(), int_ty)),
        PrimReduction::Op { symbol: "add_int" },
    )
    .expect("add_int");
    Fixture {
        env,
        int,
        leq,
        eq_op,
        add_op,
        bool_,
        true_,
        false_,
    }
}

fn call(op: GlobalId, a: Term, b: Term) -> Term {
    Term::app(Term::app(Term::const_(op, vec![]), a), b)
}

fn lit(n: impl Into<BigInt>) -> Term {
    Term::IntLit(n.into())
}

#[test]
fn leq_int_literals_reduce_to_the_correct_bool_and_close_equality() {
    let f = fixture();
    let ctx = Context::new();
    let got = call(f.leq, lit(0), lit(5));
    let true_term = Term::constructor(f.true_, vec![]);
    assert_eq!(whnf(&f.env, &ctx, &got), true_term);
    let goal = Term::Eq(
        Box::new(Term::indformer(f.bool_, vec![])),
        Box::new(got),
        Box::new(true_term),
    );
    check(&f.env, &ctx, &ken_kernel::obs::tt_term(&f.env), &goal)
        .expect("Proved closes Equal Bool (leq_int 0 5) True");

    for (m, n, expected) in [
        (-5, -3, f.true_),
        (5, 5, f.true_),
        (1, 0, f.false_),
        (3, -5, f.false_),
    ] {
        assert_eq!(
            whnf(&f.env, &ctx, &call(f.leq, lit(m), lit(n))),
            Term::constructor(expected, vec![]),
            "leq_int {m} {n}"
        );
    }
}

#[test]
fn false_comparison_refuses_proved_at_true_goal() {
    let f = fixture();
    let ctx = Context::new();
    let goal = Term::Eq(
        Box::new(Term::indformer(f.bool_, vec![])),
        Box::new(call(f.leq, lit(1), lit(0))),
        Box::new(Term::constructor(f.true_, vec![])),
    );
    assert_eq!(
        whnf(&f.env, &ctx, &goal),
        ken_kernel::obs::bottom_term(&f.env)
    );
    let err = check(&f.env, &ctx, &ken_kernel::obs::tt_term(&f.env), &goal)
        .expect_err("Proved must not close false <= true");
    assert!(
        matches!(&err, KernelError::TypeMismatch { expected, found }
            if expected.as_ref() == &goal
                && found.as_ref() == &ken_kernel::obs::top_term(&f.env)),
        "expected the false Eq goal against Top, got {err:?}"
    );
}

/// A transparent Bool elimination consumes the new kernel result by ι; it
/// cannot be discharged by an elaborator-side assertion or a neutral Op.
#[test]
fn closed_bool_elimination_over_leq_int_reduces_to_true() {
    let f = fixture();
    let ctx = Context::new();
    let high = lit(55295);
    let true_term = Term::constructor(f.true_, vec![]);
    let false_term = Term::constructor(f.false_, vec![]);
    let inner = call(f.leq, high.clone(), high.clone());
    assert_eq!(whnf(&f.env, &ctx, &inner), true_term);
    let bool_ty = Term::indformer(f.bool_, vec![]);
    let motive = Term::Ascript(
        Box::new(Term::lam(bool_ty.clone(), bool_ty.clone())),
        Box::new(Term::pi(bool_ty.clone(), Term::Type(Level::zero()))),
    );
    let match_term = Term::Elim {
        fam: f.bool_,
        level_args: vec![],
        params: vec![],
        motive: Box::new(motive),
        methods: vec![inner, false_term],
        indices: vec![],
        scrut: Box::new(call(f.leq, lit(0), high)),
    };
    let inferred = ken_kernel::infer(&f.env, &ctx, &match_term).expect("checked Bool elimination");
    assert_eq!(whnf(&f.env, &ctx, &inferred), bool_ty);
    assert_eq!(whnf(&f.env, &ctx, &match_term), true_term);
}

#[test]
fn symbolic_operands_and_other_int_ops_stay_neutral() {
    let f = fixture();
    let ctx = Context::new();
    let mut open = Context::new();
    open.push(Term::const_(f.int, vec![]));
    for term in [
        call(f.leq, Term::var(0), lit(0)),
        call(f.leq, lit(0), Term::var(0)),
    ] {
        assert_eq!(whnf(&f.env, &open, &term), term);
    }
    for term in [
        call(f.eq_op, lit(0), lit(0)),
        call(f.add_op, lit(0), lit(5)),
    ] {
        assert_eq!(whnf(&f.env, &ctx, &term), term);
    }
}

#[test]
fn transparent_literal_operands_compute_without_changing_trusted_base() {
    let mut f = fixture();
    let alias = declare_def(&mut f.env, vec![], Term::const_(f.int, vec![]), lit(5))
        .expect("transparent Int literal");
    let before = f.env.trusted_base();
    let got = call(f.leq, Term::const_(alias, vec![]), lit(5));
    assert_eq!(
        whnf(&f.env, &Context::new(), &got),
        Term::constructor(f.true_, vec![])
    );
    assert_eq!(
        f.env.trusted_base(),
        before,
        "computation adds no trusted declarations"
    );
}
