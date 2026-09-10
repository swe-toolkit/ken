//! Source-reaching acceptance for `KERNEL-CONV-CONGRUENCE-CLOSURE` increment 1.
//!
//! Promise class: durable invariant. Each checked `.ken` fixture consumes a
//! test-prelude declaration whose expected and inferred types differ only by a
//! genuine definitional equality under one newly-covered former. Removing the
//! corresponding `conv_struct_path` arm makes the unchanged fixture fail at the
//! kernel mode switch; changing unrelated repository text does not affect it.
//!
//! MEASURED: each source declaration reaches the kernel mode switch and changes
//! from `TypeMismatch` on the arm-less base to acceptance with its arm present.
//! CLAIMED: same-former conversion recognizes exactly the component equalities
//! specified for Omega, Quot, and QuotClass. THE GAP: source acceptance alone
//! cannot exclude a blanket `true`; the kernel-unit directional rejects vary
//! every compared component independently and retain heterogeneous rejection.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::{
    declare_def, declare_inductive, CtorSpec, GlobalId, InductiveSpec, Level, LevelVar, Term,
};

const OMEGA_CONSUMER: &str = include_str!("fixtures/kernel_conv_omega_consumer.ken");
const QUOT_CONSUMER: &str = include_str!("fixtures/kernel_conv_quot_consumer.ken");
const QUOT_CLASS_CONSUMER: &str = include_str!("fixtures/kernel_conv_quot_class_consumer.ken");

fn trust(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn install_def(
    env: &mut ElabEnv,
    name: &str,
    level_params: Vec<LevelVar>,
    ty: Term,
    body: Term,
) -> GlobalId {
    let id = declare_def(&mut env.env, level_params, ty, body)
        .unwrap_or_else(|error| panic!("failed to install {name}: {error}"));
    assert!(env.globals.insert(name.to_owned(), id).is_none());
    id
}

fn bool_term(env: &ElabEnv) -> Term {
    let id = env.globals.get("Bool").copied().expect("Bool prelude id");
    Term::indformer(id, vec![])
}

fn true_term(env: &ElabEnv) -> Term {
    let id = env.globals.get("True").copied().expect("True prelude id");
    Term::constructor(id, vec![])
}

fn relation_type(bool_type: &Term) -> Term {
    Term::pi(
        bool_type.clone(),
        Term::pi(bool_type.clone(), Term::Omega(Level::zero())),
    )
}

fn install_omega_fixture_prelude(env: &mut ElabEnv) {
    let level = Level::zero();
    let expected_level = level.clone().max(Level::zero());

    install_def(
        env,
        "OmegaExpectedOf",
        vec![],
        Term::pi(
            Term::Type(level.clone()),
            Term::Type(expected_level.clone().suc()),
        ),
        Term::lam(Term::Type(level.clone()), Term::Omega(expected_level)),
    );

    install_def(
        env,
        "omega_value",
        vec![],
        Term::pi(
            Term::Type(level.clone()),
            Term::pi(Term::var(0), Term::Omega(level.clone())),
        ),
        Term::lam(
            Term::Type(level),
            Term::lam(
                Term::var(0),
                Term::Eq(
                    Box::new(Term::var(1)),
                    Box::new(Term::var(0)),
                    Box::new(Term::var(0)),
                ),
            ),
        ),
    );
}

fn install_quot_fixture_prelude(env: &mut ElabEnv) {
    let bool_type = bool_term(env);
    let relation_type = relation_type(&bool_type);
    let relation_identity = install_def(
        env,
        "quot_relation_identity",
        vec![],
        Term::pi(relation_type.clone(), relation_type.clone()),
        Term::lam(relation_type.clone(), Term::var(0)),
    );

    install_def(
        env,
        "QuotExpectedOf",
        vec![],
        Term::pi(relation_type.clone(), Term::Type(Level::zero())),
        Term::lam(
            relation_type.clone(),
            Term::Quot(Box::new(bool_type.clone()), Box::new(Term::var(0))),
        ),
    );

    install_def(
        env,
        "quotient_value",
        vec![],
        Term::pi(
            relation_type.clone(),
            Term::Quot(
                Box::new(bool_type),
                Box::new(Term::app(
                    Term::const_(relation_identity, vec![]),
                    Term::var(0),
                )),
            ),
        ),
        Term::lam(relation_type, Term::QuotClass(Box::new(true_term(env)))),
    );
}

fn install_quot_class_fixture_prelude(env: &mut ElabEnv) {
    let bool_type = bool_term(env);
    let relation_type = relation_type(&bool_type);
    let relation = install_def(
        env,
        "quot_class_relation",
        vec![],
        relation_type,
        Term::lam(
            bool_type.clone(),
            Term::lam(
                bool_type.clone(),
                Term::Eq(
                    Box::new(bool_type.clone()),
                    Box::new(Term::var(1)),
                    Box::new(Term::var(0)),
                ),
            ),
        ),
    );
    let quotient = Term::Quot(
        Box::new(bool_type.clone()),
        Box::new(Term::const_(relation, vec![])),
    );
    let family = declare_inductive(&mut env.env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![quotient.clone()],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![quotient],
            target_indices: vec![Term::var(0)],
        }],
    })
    .expect("indexed family over quotient classes");
    let constructor = env
        .env
        .inductive(family)
        .expect("installed indexed family")
        .constructors[0]
        .id;

    install_def(
        env,
        "ClassExpectedOf",
        vec![],
        Term::pi(bool_type.clone(), Term::Type(Level::zero())),
        Term::lam(
            bool_type.clone(),
            Term::app(
                Term::indformer(family, vec![]),
                Term::QuotClass(Box::new(Term::var(0))),
            ),
        ),
    );

    let representative_identity = install_def(
        env,
        "quot_class_representative_identity",
        vec![],
        Term::pi(bool_type.clone(), bool_type.clone()),
        Term::lam(bool_type.clone(), Term::var(0)),
    );
    let reducible_representative =
        Term::app(Term::const_(representative_identity, vec![]), Term::var(0));
    install_def(
        env,
        "class_value",
        vec![],
        Term::pi(
            bool_type.clone(),
            Term::app(
                Term::indformer(family, vec![]),
                Term::QuotClass(Box::new(reducible_representative.clone())),
            ),
        ),
        Term::lam(
            bool_type,
            Term::app(
                Term::constructor(constructor, vec![]),
                Term::QuotClass(Box::new(reducible_representative)),
            ),
        ),
    );
}

fn assert_source_consumer(source: &str, install: impl FnOnce(&mut ElabEnv), consumer: &str) {
    let mut env = ElabEnv::new().expect("prelude construction");
    install(&mut env);
    let before = trust(&env);

    env.elaborate_file(source)
        .unwrap_or_else(|error| panic!("{consumer} must kernel-check: {error}"));

    assert_eq!(
        before,
        trust(&env),
        "{consumer} must add no trusted-base entry"
    );
}

/// On arm-less base `661d988d7`, this fixture reaches the catch-all and reports
/// `TypeMismatch`: expected `OmegaExpectedOf a`, found `Omega0`.
#[test]
fn omega_congruence_is_reached_by_checked_ken_source() {
    assert_source_consumer(
        OMEGA_CONSUMER,
        install_omega_fixture_prelude,
        "Omega(max 0 0) versus Omega(0)",
    );
}

/// On arm-less base `661d988d7`, this fixture reaches the catch-all and reports
/// `TypeMismatch`: expected `QuotExpectedOf r`, found `Bool / identity r`.
#[test]
fn quotient_congruence_is_reached_by_checked_ken_source() {
    assert_source_consumer(
        QUOT_CONSUMER,
        install_quot_fixture_prelude,
        "Quot with a reducible relation",
    );
}

/// On arm-less base `661d988d7`, this fixture reaches the catch-all and reports
/// `TypeMismatch` between the same indexed family at reducibly equal classes.
#[test]
fn quotient_class_congruence_is_reached_by_checked_ken_source() {
    assert_source_consumer(
        QUOT_CLASS_CONSUMER,
        install_quot_class_fixture_prelude,
        "QuotClass with a reducible representative",
    );
}
