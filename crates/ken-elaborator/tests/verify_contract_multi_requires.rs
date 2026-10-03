//! VERIFY-CONTRACT-LOWERING-MULTI-REQUIRES.
//! Promise class: durable invariant (spec 21 §1 and §6.3).

use std::collections::HashSet;

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::Term;

fn pi_domains(mut ty: &Term, count: usize) -> (Vec<&Term>, &Term) {
    let mut domains = Vec::with_capacity(count);
    for _ in 0..count {
        let Term::Pi(domain, codomain) = ty else {
            panic!("expected at least {count} Pi binders")
        };
        domains.push(domain.as_ref());
        ty = codomain.as_ref();
    }
    (domains, ty)
}

fn not_eq_int(env: &ElabEnv, variable: usize, value: i64) -> Term {
    let int = Term::const_(env.globals["Int"], vec![]);
    let eq = Term::app(
        Term::app(
            Term::app(Term::const_(env.globals["Equal"], vec![]), int),
            Term::var(variable),
        ),
        Term::IntLit(value.into()),
    );
    Term::app(Term::const_(env.globals["Not"], vec![]), eq)
}

#[test]
fn multiple_requires_domains_shift_and_divisor_controls_stay_exact() {
    let cases = [
        (
            "M1",
            "fn f (n : Int) (d : Int) : Int requires Not (Equal Int d 0) \
             requires Not (Equal Int n 0) = n / d",
        ),
        (
            "M2",
            "fn f (n : Int) (d : Int) : Int requires Not (Equal Int n 0) \
             requires Not (Equal Int d 0) = n / d",
        ),
        (
            "M3",
            "fn f (n : Int) (d : Int) : Int requires Equal Int n 1 \
             requires Equal Int d 2 = n",
        ),
        (
            "M4",
            "fn f (n : Int) (d : Int) : Int requires Not (Equal Int d 0) \
             requires Not (Equal Int d 0) = n / d",
        ),
    ];

    for (name, source) in cases {
        let mut env = ElabEnv::new().expect("numeric prelude");
        let result = env.elaborate_decl_v1(source).unwrap_or_else(|error| {
            panic!("{name} must elaborate and kernel-check with both requires: {error:?}")
        });
        assert!(
            result.obligations.is_empty(),
            "{name}: exact premises discharge / or no partial operation exists"
        );

        if name == "M1" {
            let (_, ty) = env.env.const_type(result.def_id).expect("f type");
            let (domains, _) = pi_domains(&ty, 4);
            assert_eq!(domains[2], &not_eq_int(&env, 0, 0));
            assert_eq!(
                domains[3],
                &not_eq_int(&env, 2, 0),
                "the second domain names n under (n,d,h1)"
            );

            let (_, body) = env.env.transparent_body(result.def_id).expect("checked f");
            let mut body_ref = &body;
            for _ in 0..4 {
                let Term::Lam(_, inner) = body_ref else {
                    panic!("expected parameter and requires lambdas")
                };
                body_ref = inner;
            }
            assert_eq!(
                body_ref,
                &Term::app(
                    Term::app(Term::const_(env.globals["div_int"], vec![]), Term::var(3)),
                    Term::var(2),
                ),
                "body uses n and d under both proof binders"
            );
        }
    }

    // The twin does not establish d's nonzero premise. Its single remaining
    // divisor obligation is closed under both explicit proof arguments.
    let mut env = ElabEnv::new().expect("numeric prelude");
    let result = env
        .elaborate_decl_v1(
            "fn twin (n : Int) (d : Int) : Int requires Not (Equal Int n 0) \
             requires Equal Int d d = n / d",
        )
        .expect("twin with unrelated and reflexive requirements");
    let [obligation] = result.obligations.as_slice() else {
        panic!("twin must retain exactly one divisor obligation")
    };
    assert!(matches!(obligation.kind, ObligationKind::PartialPrim));
    let int = Term::const_(env.globals["Int"], vec![]);
    let nonzero_id = env.numeric_env.classify_div(&int).unwrap().nonzero_id;
    let (domains, goal) = pi_domains(&obligation.goal_closed, 4);
    assert_eq!(domains.len(), 4);
    assert_eq!(
        goal,
        &Term::app(Term::const_(nonzero_id, vec![]), Term::var(2)),
        "the remaining obligation is for d under (n,d,h1,h2)"
    );
}

/// Spec 21 §1, durable invariant: the requires premise is available while
/// elaborating ensures, and the reported holes exactly explain trust growth.
#[test]
fn ensures_goal_and_obligation_holes_are_closed_under_requires() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = env.env.trusted_base().into_iter().collect::<HashSet<_>>();
    let results = env
        .elaborate_file_v1(
            "fn divide (n : Int) (d : Int) : Int \
             requires Not (Equal Int d 0) \
             ensures Equal Int (result * d + n % d) n = n / d",
        )
        .expect("the spec 21 contract example elaborates");
    let [divide] = results.as_slice() else {
        panic!("file elaboration returns the divide declaration")
    };
    let [obligation] = divide.obligations.as_slice() else {
        panic!("divide reports its ensures hole and no PartialPrim holes")
    };
    assert!(matches!(obligation.kind, ObligationKind::Ensures));

    let int = Term::const_(env.globals["Int"], vec![]);
    let (domains, goal) = pi_domains(&obligation.goal_closed, 3);
    assert_eq!(domains[0], &int);
    assert_eq!(domains[1], &int);
    assert_eq!(domains[2], &not_eq_int(&env, 0, 0));

    let n = Term::var(2);
    let d = Term::var(1);
    let app2 = |name: &str, left: Term, right: Term| {
        Term::app(
            Term::app(Term::const_(env.globals[name], vec![]), left),
            right,
        )
    };
    let quotient = app2("div_int", n.clone(), d.clone());
    let product = app2("mul_int", quotient, d.clone());
    let remainder = app2("mod_int", n, d);
    let sum = app2("add_int", product, remainder);
    let expected_goal = Term::app(
        Term::app(
            Term::app(Term::const_(env.globals["Equal"], vec![]), int),
            sum,
        ),
        Term::var(2),
    );
    assert_eq!(
        goal, &expected_goal,
        "result is replaced by the checked body"
    );

    let reported = divide
        .obligations
        .iter()
        .map(|reported| reported.hole_id)
        .collect::<HashSet<_>>();
    let after = env.env.trusted_base().into_iter().collect::<HashSet<_>>();
    let added = after.difference(&before).copied().collect::<HashSet<_>>();
    assert_eq!(
        added, reported,
        "every trusted-base addition is exactly a reported obligation hole"
    );
}
