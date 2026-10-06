//! Durable invariants for spec/20-verification/22 §3 and §5: a let binds its
//! equation for obligations in its body, without adding evidence to core.

use ken_elaborator::{ElabEnv, ElabResult, Obligation, ObligationKind};
use ken_kernel::{check::check, Context, Decl, Level, Term};

fn fixture() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let int = int_ty(&env);
    let omega = Term::omega(Level::Zero);
    env.declare_postulate_raw("Q", Term::pi(int.clone(), omega.clone()))
        .expect("opaque Q");
    env.declare_postulate_raw("P2", Term::pi(int.clone(), Term::pi(int, omega)))
        .expect("opaque P2");
    env.elaborate_decl("def QPos = { x : Int | Q x }")
        .expect("checked named refinement");
    env
}

fn int_ty(env: &ElabEnv) -> Term {
    Term::const_(env.globals["Int"], vec![])
}

fn q(env: &ElabEnv, index: usize) -> Term {
    Term::app(Term::const_(env.globals["Q"], vec![]), Term::var(index))
}

fn p2(env: &ElabEnv, left: usize, right: usize) -> Term {
    Term::app(
        Term::app(Term::const_(env.globals["P2"], vec![]), Term::var(left)),
        Term::var(right),
    )
}

fn let_eq(int: &Term, value: usize, rhs: usize) -> Term {
    Term::Eq(
        Box::new(int.clone()),
        Box::new(Term::var(value)),
        Box::new(Term::var(rhs)),
    )
}

fn unary_let_goal(env: &ElabEnv) -> Term {
    let int = int_ty(env);
    Term::pi(
        int.clone(),
        Term::pi(
            q(env, 0),
            Term::pi(int.clone(), Term::pi(let_eq(&int, 0, 2), q(env, 1))),
        ),
    )
}

fn binary_let_goal(env: &ElabEnv) -> Term {
    let int = int_ty(env);
    Term::pi(
        int.clone(),
        Term::pi(
            p2(env, 0, 0),
            Term::pi(int.clone(), Term::pi(let_eq(&int, 0, 2), p2(env, 3, 1))),
        ),
    )
}

fn one<'a>(result: &'a ElabResult, kind: ObligationKind) -> &'a Obligation {
    let [obligation] = result.obligations.as_slice() else {
        panic!("exactly one result obligation: {:?}", result.obligations);
    };
    assert_eq!(
        std::mem::discriminant(&obligation.kind),
        std::mem::discriminant(&kind)
    );
    obligation
}

fn kernel_certificate(env: &ElabEnv, goal: &Term, cert: &Term) {
    check(&env.env, &Context::new(), cert, goal).unwrap_or_else(|error| {
        panic!("kernel certificate rejected: {error:?}; goal={goal:?}; cert={cert:?}")
    });
}

/// In context `[... r, e]`, transport `h : P n` along `e : Eq Int r n`
/// by J with motive `λy proof. P y → P r`. For the True arm, a path proof
/// lies between r and e. The caller supplies the exact binder indices.
fn transport_body(
    int: &Term,
    head: &dyn Fn(usize) -> Term,
    depth: usize,
    r_index: usize,
    h_index: usize,
) -> Term {
    let p = |len: usize, arg: usize| Term::app(head(len), Term::var(arg));
    let motive = Term::lam(
        int.clone(),
        Term::lam(
            let_eq(int, r_index + 1, 0),
            Term::pi(p(depth + 2, 1), p(depth + 3, r_index + 3)),
        ),
    );
    let motive_ty = Term::pi(
        int.clone(),
        Term::pi(let_eq(int, r_index + 1, 0), Term::omega(Level::Zero)),
    );
    let motive = Term::Ascript(Box::new(motive), Box::new(motive_ty));
    let base = Term::lam(p(depth, r_index), Term::var(0));
    Term::app(
        Term::J(Box::new(motive), Box::new(base), Box::new(Term::var(0))),
        Term::var(h_index),
    )
}

fn unary_certificate(env: &ElabEnv) -> Term {
    let int = int_ty(env);
    let q_head = |_: usize| Term::const_(env.globals["Q"], vec![]);
    Term::lam(
        int.clone(),
        Term::lam(
            q(env, 0),
            Term::lam(
                int.clone(),
                Term::lam(let_eq(&int, 0, 2), transport_body(&int, &q_head, 4, 1, 2)),
            ),
        ),
    )
}

fn binary_certificate(env: &ElabEnv) -> Term {
    let int = int_ty(env);
    let p_head = |depth: usize| {
        Term::app(
            Term::const_(env.globals["P2"], vec![]),
            Term::var(depth - 1),
        )
    };
    Term::lam(
        int.clone(),
        Term::lam(
            p2(env, 0, 0),
            Term::lam(
                int.clone(),
                Term::lam(let_eq(&int, 0, 2), transport_body(&int, &p_head, 4, 1, 2)),
            ),
        ),
    )
}

fn checked_body(env: &ElabEnv, result: &ElabResult) -> Term {
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(result.def_id) else {
        panic!("checked declaration must be transparent: {}", result.name);
    };
    body.clone()
}

/// Promise: durable invariant. Measured: the *actual hole type* has the
/// binder and Eq in the right order, and a J certificate kernel-checks in an
/// empty context. Claimed: a leaf under a let can use its RHS equation.
/// Gap: the emitted body must remain the ordinary Let, not acquire a proof.
#[test]
fn ensures_literal_and_named_returns_transport_the_let_equation() {
    let mut env = fixture();
    let int = int_ty(&env);
    let expected_body = Term::lam(
        int.clone(),
        Term::lam(
            q(&env, 0),
            Term::Let {
                ty: Box::new(int.clone()),
                val: Box::new(Term::var(1)),
                body: Box::new(Term::var(0)),
            },
        ),
    );
    for (source, kind) in [
        (
            "fn es_let (n : Int) : Int requires Q n ensures Q result = let r = n in r",
            ObligationKind::Ensures,
        ),
        (
            "fn lit_let (n : Int) : { x : Int | Q x } requires Q n = let r = n in r",
            ObligationKind::Ensures,
        ),
        (
            "fn named_let (n : Int) : QPos requires Q n = let r = n in r",
            ObligationKind::RefinementIntroduction,
        ),
    ] {
        let result = env.elaborate_decl_v1(source).expect(source);
        let obligation = one(&result, kind);
        kernel_certificate(&env, &obligation.goal_closed, &unary_certificate(&env));
        assert_eq!(obligation.goal_closed, unary_let_goal(&env), "{source}");
        assert_eq!(checked_body(&env, &result), expected_body, "{source}");
    }
}

/// Promise: durable invariant. Holding the first argument of opaque P2
/// separate from the result exposes mis-indexed weakening of the let RHS.
#[test]
fn binary_predicate_transports_its_second_argument_only() {
    let mut env = fixture();
    let result = env
        .elaborate_decl_v1(
            "fn binary_let (n : Int) : Int requires P2 n n ensures P2 n result = let r = n in r",
        )
        .expect("binary predicate row");
    let obligation = one(&result, ObligationKind::Ensures);
    kernel_certificate(&env, &obligation.goal_closed, &binary_certificate(&env));
    assert_eq!(obligation.goal_closed, binary_let_goal(&env));

    let without_premise = env
        .elaborate_decl_v1("fn binary_open (n : Int) : Int ensures P2 n result = let r = n in r")
        .expect("binary row without a premise still has an honest hole");
    let no_premise = one(&without_premise, ObligationKind::Ensures);
    let int = int_ty(&env);
    assert_eq!(
        no_premise.goal_closed,
        Term::pi(
            int.clone(),
            Term::pi(int.clone(), Term::pi(let_eq(&int, 0, 1), p2(&env, 2, 1))),
        ),
    );
    assert!(env.is_open_hole(no_premise.hole_id));
}

/// Promise: durable invariant. The True branch retains both the match path
/// Eq and the let Eq, while False remains a let-free single-path obligation.
#[test]
fn match_true_arm_places_let_equation_inside_the_branch_equation() {
    let mut env = fixture();
    let result = env.elaborate_decl_v1(
        "fn match_let (b : Bool) (n : Int) : Int requires Q n ensures Q result = match b { True |-> let r = n in r ; False |-> n }",
    ).expect("match with an inner let");
    let [yes, no] = result.obligations.as_slice() else {
        panic!("one goal per arm")
    };
    assert!(matches!(yes.kind, ObligationKind::Ensures));
    assert!(matches!(no.kind, ObligationKind::Ensures));
    let int = int_ty(&env);
    let bool_ty = Term::indformer(env.globals["Bool"], vec![]);
    let branch_eq = |scrutinee: usize, ctor: &str| {
        Term::Eq(
            Box::new(bool_ty.clone()),
            Box::new(Term::var(scrutinee)),
            Box::new(Term::constructor(env.globals[ctor], vec![])),
        )
    };
    let true_goal = Term::pi(
        bool_ty.clone(),
        Term::pi(
            int.clone(),
            Term::pi(
                q(&env, 0),
                Term::pi(
                    int.clone(),
                    Term::pi(
                        branch_eq(3, "True"),
                        Term::pi(let_eq(&int, 1, 3), q(&env, 2)),
                    ),
                ),
            ),
        ),
    );
    let false_goal = Term::pi(
        bool_ty.clone(),
        Term::pi(
            int.clone(),
            Term::pi(q(&env, 0), Term::pi(branch_eq(2, "False"), q(&env, 2))),
        ),
    );
    let q_head = |_: usize| Term::const_(env.globals["Q"], vec![]);
    let true_cert = Term::lam(
        bool_ty.clone(),
        Term::lam(
            int.clone(),
            Term::lam(
                q(&env, 0),
                Term::lam(
                    int.clone(),
                    Term::lam(
                        branch_eq(3, "True"),
                        Term::lam(let_eq(&int, 1, 3), transport_body(&int, &q_head, 6, 2, 3)),
                    ),
                ),
            ),
        ),
    );
    kernel_certificate(&env, &yes.goal_closed, &true_cert);
    assert_eq!(yes.goal_closed, true_goal);
    assert_eq!(no.goal_closed, false_goal);
    let false_cert = Term::lam(
        Term::indformer(env.globals["Bool"], vec![]),
        Term::lam(
            int.clone(),
            Term::lam(q(&env, 0), Term::lam(branch_eq(2, "False"), Term::var(1))),
        ),
    );
    kernel_certificate(&env, &no.goal_closed, &false_cert);
}

/// Promise: durable invariant. An Ω-classified proof let has no useful value
/// equation; a Type-classified function or type value does. These are the
/// boundary directions of the classifier, not assumptions inferred from a
/// handful of Int-only rows.
#[test]
fn informative_classification_admits_type_and_function_but_not_proof_lets() {
    let mut env = fixture();
    let proof = env
        .elaborate_decl_v1(
            "fn omega (n : Int) : Int requires Q n ensures Q result = let t : Top = Proved in n",
        )
        .expect("Top and Proved are the Ω proof spelling");
    let obligation = one(&proof, ObligationKind::Ensures);
    let int = int_ty(&env);
    let top = Term::const_(env.globals["Top"], vec![]);
    assert_eq!(
        obligation.goal_closed,
        Term::pi(int.clone(), Term::pi(q(&env, 0), Term::pi(top, q(&env, 2))),)
    );
    for (source, domain, rhs) in [
        (
            "fn type_value (n : Int) : Int requires Q n ensures Q result = let T : Type 0 = Int in n",
            Term::ty(Level::Zero),
            int.clone(),
        ),
        (
            "fn function_value (n : Int) : Int requires Q n ensures Q result = let f : Int -> Int = \\x. x in n",
            Term::pi(int.clone(), int.clone()),
            Term::lam(int.clone(), Term::var(0)),
        ),
    ] {
        let result = env.elaborate_decl_v1(source).expect(source);
        let obligation = one(&result, ObligationKind::Ensures);
        assert_eq!(obligation.goal_closed, Term::pi(
            int.clone(),
            Term::pi(
                q(&env, 0),
                Term::pi(
                    domain.clone(),
                    Term::pi(
                        Term::Eq(Box::new(domain), Box::new(Term::var(0)), Box::new(rhs)),
                        q(&env, 3),
                    ),
                ),
            ),
        ), "{source}");
    }
}

/// Promise: durable invariant. Inference of a let is a distinct production
/// site: the named refinement introduced inside its body must see the same
/// scoped equation as a result-position check.
#[test]
fn inferred_let_body_introduction_receives_the_equation() {
    let mut env = fixture();
    let result = env
        .elaborate_decl_v1("const inferred = let r = 5 in (r : QPos)")
        .expect("infer the let and check its body ascription");
    let obligation = one(&result, ObligationKind::RefinementIntroduction);
    let int = int_ty(&env);
    assert_eq!(
        obligation.goal_closed,
        Term::pi(
            int.clone(),
            Term::pi(
                Term::Eq(
                    Box::new(int.clone()),
                    Box::new(Term::var(0)),
                    Box::new(Term::IntLit(5.into()))
                ),
                q(&env, 1),
            ),
        )
    );
    assert!(
        env.is_open_hole(obligation.hole_id),
        "opaque Q has no premise"
    );
    assert_eq!(
        checked_body(&env, &result),
        Term::Let {
            ty: Box::new(int.clone()),
            val: Box::new(Term::IntLit(5.into())),
            body: Box::new(Term::var(0)),
        }
    );
    // A literal predicate mentioning the same RHS is discharged by the Eq
    // hypothesis itself; this is an independent infer-arm certificate.
    let literal = env
        .elaborate_decl_v1("const inferred_literal = let r = 5 in (r : {x : Int | Equal Int x 5})")
        .expect("literal ascription in an inferred let body");
    let literal_obligation = one(&literal, ObligationKind::RefinementIntroduction);
    let certificate = Term::lam(
        int.clone(),
        Term::lam(
            Term::Eq(
                Box::new(int.clone()),
                Box::new(Term::var(0)),
                Box::new(Term::IntLit(5.into())),
            ),
            Term::var(0),
        ),
    );
    kernel_certificate(&env, &literal_obligation.goal_closed, &certificate);
}

/// Promise: durable invariant. A let-free result never acquires an Eq; the
/// matched controls retain their exact preexisting goal and a direct kernel
/// certificate. This guards against pushing outside the let's scope.
#[test]
fn let_free_ensures_literal_named_and_binary_controls_are_unchanged() {
    let mut env = fixture();
    let int = int_ty(&env);
    for (source, kind) in [
        (
            "fn es_plain (n : Int) : Int requires Q n ensures Q result = n",
            ObligationKind::Ensures,
        ),
        (
            "fn lit_plain (n : Int) : {x : Int | Q x} requires Q n = n",
            ObligationKind::Ensures,
        ),
        (
            "fn named_plain (n : Int) : QPos requires Q n = n",
            ObligationKind::RefinementIntroduction,
        ),
    ] {
        let result = env.elaborate_decl_v1(source).expect(source);
        let obligation = one(&result, kind);
        let goal = Term::pi(int.clone(), Term::pi(q(&env, 0), q(&env, 1)));
        assert_eq!(obligation.goal_closed, goal);
        kernel_certificate(
            &env,
            &goal,
            &Term::lam(int.clone(), Term::lam(q(&env, 0), Term::var(0))),
        );
    }
    let binary = env
        .elaborate_decl_v1(
            "fn binary_plain (n : Int) : Int requires P2 n n ensures P2 n result = n",
        )
        .expect("binary let-free control");
    let obligation = one(&binary, ObligationKind::Ensures);
    let goal = Term::pi(int.clone(), Term::pi(p2(&env, 0, 0), p2(&env, 1, 1)));
    assert_eq!(obligation.goal_closed, goal);
    kernel_certificate(
        &env,
        &goal,
        &Term::lam(int, Term::lam(p2(&env, 0, 0), Term::var(0))),
    );
}
