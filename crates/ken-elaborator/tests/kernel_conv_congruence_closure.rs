//! Source-reaching acceptance for `KERNEL-CONV-CONGRUENCE-CLOSURE`.
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
//! specified for Omega, Quot, QuotClass, Cast, and QuotElim. THE GAP: source
//! acceptance alone
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
const CAST_CONSUMER: &str = include_str!("fixtures/kernel_conv_cast_consumer.ken");
const QUOT_ELIM_CONSUMER: &str = include_str!("fixtures/kernel_conv_quot_elim_consumer.ken");

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

fn declare_bool_indexed_family(env: &mut ElabEnv, bool_type: &Term) -> (GlobalId, GlobalId) {
    let family = declare_inductive(&mut env.env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![bool_type.clone()],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![bool_type.clone()],
            target_indices: vec![Term::var(0)],
        }],
    })
    .expect("indexed family over Bool");
    let constructor = env
        .env
        .inductive(family)
        .expect("installed Bool-indexed family")
        .constructors[0]
        .id;
    (family, constructor)
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

fn install_cast_fixture_prelude(env: &mut ElabEnv) {
    let bool_type = bool_term(env);
    let (family, constructor) = declare_bool_indexed_family(env, &bool_type);
    let type_zero = Term::Type(Level::zero());

    let identity_type = Term::pi(type_zero.clone(), Term::pi(Term::var(0), Term::var(1)));
    let identity = install_def(
        env,
        "conv_cast_value_identity",
        vec![],
        identity_type,
        Term::lam(type_zero.clone(), Term::lam(Term::var(0), Term::var(0))),
    );

    // Under A, e, t: A = Var(2), e = Var(1), t = Var(0).
    let plain_cast = Term::Cast(
        Box::new(Term::var(2)),
        Box::new(bool_type.clone()),
        Box::new(Term::var(1)),
        Box::new(Term::var(0)),
    );
    let reducible_value = Term::app(
        Term::app(Term::const_(identity, vec![]), Term::var(2)),
        Term::var(0),
    );
    let reducible_cast = Term::Cast(
        Box::new(Term::var(2)),
        Box::new(bool_type.clone()),
        Box::new(Term::var(1)),
        Box::new(reducible_value),
    );
    let equality_domain = Term::Eq(
        Box::new(type_zero.clone()),
        Box::new(Term::var(0)),
        Box::new(bool_type.clone()),
    );

    install_def(
        env,
        "ConvCastExpectedOf",
        vec![],
        Term::pi(
            type_zero.clone(),
            Term::pi(
                equality_domain.clone(),
                Term::pi(Term::var(1), type_zero.clone()),
            ),
        ),
        Term::lam(
            type_zero.clone(),
            Term::lam(
                equality_domain.clone(),
                Term::lam(
                    Term::var(1),
                    Term::app(Term::indformer(family, vec![]), reducible_cast),
                ),
            ),
        ),
    );

    install_def(
        env,
        "conv_cast_value",
        vec![],
        Term::pi(
            type_zero.clone(),
            Term::pi(
                equality_domain.clone(),
                Term::pi(
                    Term::var(1),
                    Term::app(Term::indformer(family, vec![]), plain_cast.clone()),
                ),
            ),
        ),
        Term::lam(
            type_zero,
            Term::lam(
                equality_domain,
                Term::lam(
                    Term::var(1),
                    Term::app(Term::constructor(constructor, vec![]), plain_cast),
                ),
            ),
        ),
    );
}

fn install_quot_elim_fixture_prelude(env: &mut ElabEnv) {
    let bool_type = bool_term(env);
    let (family, constructor) = declare_bool_indexed_family(env, &bool_type);
    let relation = install_def(
        env,
        "conv_quot_elim_relation",
        vec![],
        relation_type(&bool_type),
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
    let quotient_alias = install_def(
        env,
        "ConvBoolQuotient",
        vec![],
        Term::Type(Level::zero()),
        quotient,
    );
    let quotient_type = Term::const_(quotient_alias, vec![]);

    let motive = install_def(
        env,
        "conv_quot_elim_motive",
        vec![],
        Term::pi(quotient_type.clone(), Term::Type(Level::zero())),
        Term::lam(quotient_type.clone(), bool_type.clone()),
    );
    let method_type = Term::pi(bool_type.clone(), bool_type.clone());
    let method = install_def(
        env,
        "conv_quot_elim_method",
        vec![],
        method_type.clone(),
        Term::lam(bool_type.clone(), Term::var(0)),
    );
    let method_identity = install_def(
        env,
        "conv_quot_elim_method_identity",
        vec![],
        Term::pi(method_type.clone(), method_type.clone()),
        Term::lam(method_type, Term::var(0)),
    );
    let relation_at_xy = Term::app(
        Term::app(Term::const_(relation, vec![]), Term::var(1)),
        Term::var(0),
    );
    let respect_type = Term::pi(
        bool_type.clone(),
        Term::pi(
            bool_type.clone(),
            Term::pi(
                relation_at_xy.clone(),
                Term::Eq(
                    Box::new(bool_type.clone()),
                    Box::new(Term::var(2)),
                    Box::new(Term::var(1)),
                ),
            ),
        ),
    );
    let respect = install_def(
        env,
        "conv_quot_elim_respect",
        vec![],
        respect_type,
        Term::lam(
            bool_type.clone(),
            Term::lam(bool_type.clone(), Term::lam(relation_at_xy, Term::var(0))),
        ),
    );

    // Under q: the scrutinee is Var(0), so both eliminators remain neutral.
    let plain_elim = Term::QuotElim {
        motive: Box::new(Term::const_(motive, vec![])),
        method: Box::new(Term::const_(method, vec![])),
        respect: Box::new(Term::const_(respect, vec![])),
        scrut: Box::new(Term::var(0)),
    };
    let reducible_method = Term::app(
        Term::const_(method_identity, vec![]),
        Term::const_(method, vec![]),
    );
    let reducible_elim = Term::QuotElim {
        motive: Box::new(Term::const_(motive, vec![])),
        method: Box::new(reducible_method),
        respect: Box::new(Term::const_(respect, vec![])),
        scrut: Box::new(Term::var(0)),
    };

    install_def(
        env,
        "ConvQuotElimExpectedOf",
        vec![],
        Term::pi(quotient_type.clone(), Term::Type(Level::zero())),
        Term::lam(
            quotient_type.clone(),
            Term::app(Term::indformer(family, vec![]), reducible_elim),
        ),
    );
    install_def(
        env,
        "conv_quot_elim_value",
        vec![],
        Term::pi(
            quotient_type.clone(),
            Term::app(Term::indformer(family, vec![]), plain_elim.clone()),
        ),
        Term::lam(
            quotient_type,
            Term::app(Term::constructor(constructor, vec![]), plain_elim),
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

/// On arm-less base `f1e7e516c`, the open `A` versus `Bool` endpoints leave a
/// genuine neutral Cast. The fixture then reports `TypeMismatch` between the
/// same indexed family at casts whose values differ only by transparent
/// identity reduction.
#[test]
fn cast_congruence_is_reached_by_checked_ken_source() {
    assert_source_consumer(
        CAST_CONSUMER,
        install_cast_fixture_prelude,
        "neutral Cast with a reducible value",
    );
}

/// On arm-less base `f1e7e516c`, the quotient-variable scrutinee leaves a
/// genuine neutral QuotElim. The fixture then reports `TypeMismatch` between
/// the same indexed family at eliminators whose methods differ only by
/// transparent identity reduction.
#[test]
fn quotient_elim_congruence_is_reached_by_checked_ken_source() {
    assert_source_consumer(
        QUOT_ELIM_CONSUMER,
        install_quot_elim_fixture_prelude,
        "neutral QuotElim with a reducible method",
    );
}
