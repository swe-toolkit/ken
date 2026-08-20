use ken_kernel::inductive::check_positivity;
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    declare_def, declare_inductive, declare_postulate, CtorSpec, GlobalEnv, InductiveSpec,
    KernelError,
};

fn type_at(level: Level) -> Term {
    Term::Type(level)
}

fn assert_positivity_violation(result: Result<(), KernelError>) {
    assert!(
        matches!(result, Err(KernelError::PositivityViolation(_))),
        "expected PositivityViolation, got {result:?}"
    );
}

/// Durable invariant: head exposure must retain the polarity flip at Pi domains.
#[test]
fn negative_occurrence_exposed_at_the_head_stays_rejected() {
    let result = declare_inductive(&mut GlobalEnv::new(), |bad| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::pi(
                Term::indformer(bad, vec![]),
                type_at(Level::zero()),
            )],
            target_indices: vec![],
        }],
    });

    assert!(
        matches!(result, Err(KernelError::PositivityViolation(_))),
        "D to the left of an arrow must remain rejected, got {result:?}"
    );
}

/// Durable invariant: a no-occurrence guard must inspect transparent bodies.
/// Replacing `occurs_delta` with syntactic `occurs` makes this test accept.
#[test]
fn negative_occurrence_hidden_in_a_transparent_argument_stays_rejected() {
    let mut env = GlobalEnv::new();
    let opaque_head = declare_postulate(
        &mut env,
        "positivity test head".into(),
        vec![],
        Term::pi(type_at(Level::zero().suc()), type_at(Level::zero())),
    )
    .expect("declare opaque application head");

    let family = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .expect("declare control family");

    // `Wrap : Type 1 := family -> Type 0`. It is intentionally declared after
    // the admitted control family so its body can hide that family's former.
    let wrap = declare_def(
        &mut env,
        vec![],
        type_at(Level::zero().suc()),
        Term::pi(Term::indformer(family, vec![]), type_at(Level::zero())),
    )
    .expect("declare transparent wrapper");

    let wrap_alias = declare_def(
        &mut env,
        vec![],
        type_at(Level::zero().suc()),
        Term::const_(wrap, vec![]),
    )
    .expect("declare a second transparent hop");

    let mut hidden = env.inductive(family).expect("control family").clone();
    hidden.constructors[0].args = vec![Term::app(
        Term::const_(opaque_head, vec![]),
        Term::const_(wrap_alias, vec![]),
    )];

    assert_positivity_violation(check_positivity(&env, &hidden));
}
