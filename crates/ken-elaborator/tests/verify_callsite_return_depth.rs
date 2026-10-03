//! VERIFY-CALL-SITE-PRECONDITION-DISCHARGE item 0.
//! Promise class: durable telescope-depth invariant (spec 21 §6.3).

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::Term;

fn pi_prefix(mut ty: &Term, count: usize) -> (Vec<Term>, Term) {
    let mut domains = Vec::with_capacity(count);
    for _ in 0..count {
        let Term::Pi(domain, codomain) = ty else {
            panic!("expected {count} leading Pi domains")
        };
        domains.push((**domain).clone());
        ty = codomain;
    }
    (domains, ty.clone())
}

fn equal_int(env: &ElabEnv, left: Term, right: Term) -> Term {
    Term::app(
        Term::app(
            Term::app(
                Term::const_(env.globals["Equal"], vec![]),
                Term::const_(env.globals["Int"], vec![]),
            ),
            left,
        ),
        right,
    )
}

fn not(env: &ElabEnv, proposition: Term) -> Term {
    Term::app(Term::const_(env.globals["Not"], vec![]), proposition)
}

/// AR1 and AR5. MEASURED: the full checked type has explicit `n`, then its
/// requires proof, then the returned function argument. CLAIMED: `param_count`
/// is the sole split point even when the carrier result is itself a Pi. THE
/// GAP: exact domains distinguish n from the result-function argument.
#[test]
fn function_return_type_stays_after_requires() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let f = env
        .elaborate_decl_v1(
            "fn f (n : Int) : Int -> Int \
             requires Not (Equal Int n 0) = \\m. m",
        )
        .expect("function-valued return with requires");
    let (_, ty) = env
        .env
        .const_type(f.def_id)
        .expect("checked declaration type");
    let (domains, result) = pi_prefix(&ty, 3);
    let int = Term::const_(env.globals["Int"], vec![]);
    assert_eq!(domains[0], int);
    assert_eq!(
        domains[1],
        not(&env, equal_int(&env, Term::var(0), Term::IntLit(0.into())))
    );
    assert_eq!(domains[2], int);
    assert_eq!(result, int);
}

/// AR1 and H4's multi-clause twin. MEASURED: the second premise names n at
/// its actual depth, after n, d, and the first proof. CLAIMED: the return Pi
/// stays after both requires binders. THE GAP: n and d share a carrier, so
/// the complete domain terms, not just the Pi count, are compared.
#[test]
fn multiple_requires_split_before_a_function_valued_return() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let f = env
        .elaborate_decl_v1(
            "fn f (n : Int) (d : Int) : Int -> Int \
             requires Not (Equal Int d 0) \
             requires Not (Equal Int n 0) = \\m. m",
        )
        .expect("multiple requires before a function-valued return");
    let (_, ty) = env
        .env
        .const_type(f.def_id)
        .expect("checked declaration type");
    let (domains, result) = pi_prefix(&ty, 5);
    let int = Term::const_(env.globals["Int"], vec![]);
    assert_eq!(domains[0], int);
    assert_eq!(domains[1], int);
    assert_eq!(
        domains[2],
        not(&env, equal_int(&env, Term::var(0), Term::IntLit(0.into())))
    );
    assert_eq!(
        domains[3],
        not(&env, equal_int(&env, Term::var(2), Term::IntLit(0.into())))
    );
    assert_eq!(domains[4], int);
    assert_eq!(result, int);
}

/// AR2 before item 4. MEASURED: `f 0 k` is rejected while its contracted
/// function result still exposes the requires Pi. CLAIMED: the call is never
/// admitted as though it had zero obligations. THE GAP: item 4 may later turn
/// this refusal into one call-site Requires obligation.
#[test]
fn function_valued_call_is_not_admitted_without_a_requires_argument() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = env.env.trusted_base();
    let error = env
        .elaborate_file_v1(
            "fn f (n : Int) : Int -> Int requires Not (Equal Int n 0) = \\m. m\n\
             fn h (k : Int) : Int = f 0 k",
        )
        .expect_err("the caller supplies Int where f still requires a proof");
    assert!(format!("{error:?}").contains("TypeMismatch"), "{error:?}");
    assert_eq!(env.env.trusted_base(), before);
}

/// AR3. MEASURED: ensures at the declared return depth closes over n; an
/// applied `result` uses the returned function's type and is substituted by
/// an ascribed body. CLAIMED: ensures resolution uses the parameter split.
/// THE GAP: the expected goal retains the ascription and application redex.
#[test]
fn function_return_ensures_use_parameter_depth_and_ascribed_result() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let simple = env
        .elaborate_decl_v1("fn simple (n : Int) : Int -> Int ensures Equal Int n 5 = \\m. n")
        .expect("ensures can mention a parameter before the result Pi");
    let [simple_ensures] = simple.obligations.as_slice() else {
        panic!("simple has one Ensures obligation")
    };
    assert!(matches!(simple_ensures.kind, ObligationKind::Ensures));
    let int = Term::const_(env.globals["Int"], vec![]);
    assert_eq!(
        simple_ensures.goal_closed,
        Term::pi(
            int.clone(),
            equal_int(&env, Term::var(0), Term::IntLit(5.into())),
        )
    );

    let applied = env
        .elaborate_decl_v1(
            "fn applied (n : Int) : Int -> Int \
             ensures Equal Int (result 0) n = \\m. n",
        )
        .expect("result has the declared function-valued type");
    let [applied_ensures] = applied.obligations.as_slice() else {
        panic!("applied has one Ensures obligation")
    };
    let result_function = Term::Ascript(
        Box::new(Term::lam(int.clone(), Term::var(1))),
        Box::new(Term::pi(int.clone(), int.clone())),
    );
    assert_eq!(
        applied_ensures.goal_closed,
        Term::pi(
            int.clone(),
            equal_int(
                &env,
                Term::app(result_function, Term::IntLit(0.into())),
                Term::var(0),
            ),
        )
    );

    let error = env
        .elaborate_decl_v1(
            "fn bad_result (n : Int) : Int -> Int \
             ensures Equal Int result n = \\m. n",
        )
        .expect_err("the function-valued result is not an Int");
    assert!(format!("{error:?}").contains("TypeMismatch"), "{error:?}");
}

/// AR4. MEASURED: a return refinement below the declared result's own arrow
/// is refused with the ruled diagnostic. CLAIMED: only a refinement at the
/// `param_count` depth becomes an implicit ensures. THE GAP: this type has
/// one explicit parameter and one returned function arrow.
#[test]
fn nested_return_refinement_fails_closed() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let error = env
        .elaborate_decl_v1("fn nested (n : Int) : Int -> { x : Int | Equal Int x n } = \\m. m")
        .expect_err("refinement under the returned function remains out of scope");
    let message = format!("{error:?}");
    assert!(message.contains("TypeMismatch"), "{message}");
    assert!(
        message.contains("a refinement under a function-valued return type is not supported yet"),
        "{message}"
    );
}
