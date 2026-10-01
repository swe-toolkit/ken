//! ADR 0013 Layer 2, spec 16 §2.2: a checked Ken proof reaches kernel
//! conversion, not an elaborator-side fact. Promise class: durable invariant.

use ken_elaborator::ElabEnv;
use ken_kernel::{check, whnf, Context, Term};

#[test]
fn registered_leq_int_closes_a_closed_bool_equality_by_proved() {
    let mut env = ElabEnv::new().expect("prelude");
    let leq = Term::const_(env.globals["leq_int"], vec![]);
    let term = Term::app(Term::app(leq, Term::IntLit(0.into())), Term::IntLit(5.into()));
    assert_eq!(
        whnf(&env.env, &Context::new(), &term),
        Term::constructor(env.numeric_env.bool_true_id, vec![])
    );
}

#[test]
fn scalar_boundary_closes_by_proved() {
    let mut env = ElabEnv::new().expect("prelude");
    let goal = Term::app(
        Term::const_(env.globals["isScalar"], vec![]),
        Term::IntLit(55295.into()),
    );
    let reduct = whnf(&env.env, &Context::new(), &goal);
    eprintln!("isScalar 55295 reduct: {reduct:?}");
    check(&env.env, &Context::new(), &ken_kernel::obs::tt_term(&env.env), &goal)
        .expect("kernel conversion closes isScalar 55295");
}
