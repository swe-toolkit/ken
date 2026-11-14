//! Core-term acceptance for `KERNEL-CONV-CONGRUENCE-CLOSURE`.
//!
//! Promise class: durable invariant. The expected types and values are checked
//! kernel declarations; each fresh core check requires conversion under one
//! newly covered former. Removing the corresponding `conv_struct_path` arm
//! should make the same check report `TypeMismatch` without a surface name.
//!
//! MEASURED: five core checks succeed; removing the Omega, QuotClass, and
//! Cast arms yields `TypeMismatch`, but removing Quot leaves its check green.
//! QuotElim mutation has not been run. This is WIP, not a complete pin.
//! CLAIMED: same-former conversion handles Omega, Quot, QuotClass, Cast, and
//! QuotElim. THE GAP: Quot's check does not yet prove its named arm is reached;
//! success alone cannot exclude blanket `true`. Kernel directional rejects
//! independently vary components and reject heterogeneity.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::{
    check, declare_def, declare_inductive, declare_postulate, env::Context, CtorSpec, Decl,
    GlobalId, InductiveSpec, Level, LevelVar, Term,
};

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

fn assumed_equivalence(env: &mut ElabEnv, carrier: &Term, relation: &Term) -> Term {
    let expected = ken_kernel::check::quotient_equivalence_type(carrier, relation);
    let id = declare_postulate(
        &mut env.env,
        "test quotient equivalence".into(),
        vec![],
        expected,
    )
    .expect("explicit equivalence evidence for the test relation");
    Term::const_(id, vec![])
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

    let equiv_at_r = ken_kernel::check::quotient_equivalence_type(&bool_type, &Term::var(0));
    install_def(
        env,
        "QuotEquivOf",
        vec![],
        Term::pi(relation_type.clone(), Term::Omega(Level::zero())),
        Term::lam(relation_type.clone(), equiv_at_r.clone()),
    );
    install_def(
        env,
        "QuotExpectedOf",
        vec![],
        Term::pi(
            relation_type.clone(),
            Term::pi(equiv_at_r.clone(), Term::Type(Level::zero())),
        ),
        Term::lam(
            relation_type.clone(),
            Term::lam(
                equiv_at_r.clone(),
                Term::Quot(
                    Box::new(bool_type.clone()),
                    Box::new(Term::var(1)),
                    Box::new(Term::var(0)),
                ),
            ),
        ),
    );

    install_def(
        env,
        "quotient_value",
        vec![],
        Term::pi(
            relation_type.clone(),
            Term::pi(
                equiv_at_r.clone(),
                Term::Quot(
                    Box::new(bool_type),
                    Box::new(Term::app(
                        Term::const_(relation_identity, vec![]),
                        Term::var(1),
                    )),
                    Box::new(Term::var(0)),
                ),
            ),
        ),
        Term::lam(
            relation_type,
            Term::lam(equiv_at_r, Term::QuotClass(Box::new(true_term(env)))),
        ),
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
    let relation_term = Term::const_(relation, vec![]);
    let equivalence = assumed_equivalence(env, &bool_type, &relation_term);
    let quotient = Term::Quot(
        Box::new(bool_type.clone()),
        Box::new(relation_term),
        Box::new(equivalence),
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
    let relation_term = Term::const_(relation, vec![]);
    let equivalence = assumed_equivalence(env, &bool_type, &relation_term);
    let quotient = Term::Quot(
        Box::new(bool_type.clone()),
        Box::new(relation_term),
        Box::new(equivalence),
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

fn assert_core_consumer(
    install: impl FnOnce(&mut ElabEnv),
    expected_name: &str,
    value_name: &str,
    arity: usize,
    consumer: &str,
) {
    let mut env = ElabEnv::new().expect("prelude construction");
    install(&mut env);
    let before = trust(&env);
    let expected = env.globals[expected_name];
    let value = env.globals[value_name];
    let Some(Decl::Transparent { ty: value_ty, .. }) = env.env.lookup(value) else {
        panic!("fixture value must be a checked definition");
    };
    // Match the former surface `fn c (params) : Expected params = value params`:
    // application inference must convert the value's declared result type
    // against Expected, rather than checking its body against a fresh Pi.
    let mut ctx = Context::new();
    let mut ty = value_ty.clone();
    for _ in 0..arity {
        let Term::Pi(domain, codomain) = ty else {
            panic!("fixture value lacks a parameter: {ty:?}");
        };
        ctx.push(*domain);
        ty = *codomain;
    }
    let apply = |head: GlobalId| {
        (0..arity)
            .rev()
            .fold(Term::const_(head, vec![]), |acc, index| {
                Term::app(acc, Term::var(index))
            })
    };
    check(&env.env, &ctx, &apply(value), &apply(expected))
        .unwrap_or_else(|error| panic!("{consumer} must kernel-check: {error:?}"));
    assert_eq!(
        before,
        trust(&env),
        "{consumer} must add no trusted-base entry"
    );
}

/// MEASURED: core value type converts from Omega(0) to Omega(max 0 0).
/// CLAIMED: Omega level conversion reaches the Omega arm. THE GAP: the
/// independent kernel directional rejects forbid a blanket-true conversion.
#[test]
fn omega_congruence_is_reached_by_checked_core_term() {
    let mut env = ElabEnv::new().expect("prelude construction");
    install_omega_fixture_prelude(&mut env);
    let before = trust(&env);
    let mut ctx = Context::new();
    ctx.push(Term::Type(Level::zero()));
    ctx.push(Term::var(0));
    let value = Term::app(
        Term::app(
            Term::const_(env.globals["omega_value"], vec![]),
            Term::var(1),
        ),
        Term::var(0),
    );
    let expected = Term::app(
        Term::const_(env.globals["OmegaExpectedOf"], vec![]),
        Term::var(1),
    );
    check(&env.env, &ctx, &value, &expected)
        .expect("open omega application must check against its expected level");
    assert_eq!(before, trust(&env), "core check changes no trust entries");
}

/// MEASURED: the value applied to its parameters converts to QuotExpectedOf
/// applied to the same parameters, with a reducible relation under Quot.
/// CLAIMED: Quot component conversion is admitted. THE GAP: the core checker
/// alone does not distinguish a blanket-true conversion implementation.
#[test]
fn quotient_congruence_is_reached_by_checked_core_term() {
    assert_core_consumer(
        install_quot_fixture_prelude,
        "QuotExpectedOf",
        "quotient_value",
        2,
        "Quot with a reducible relation",
    );
}

/// MEASURED: class_value applied to its parameter converts to ClassExpectedOf
/// applied to that parameter, at reducibly equal quotient classes.
/// CLAIMED: QuotClass component equality is admitted. THE GAP: heterogeneous
/// classes are rejected in the independent kernel-unit tests.
#[test]
fn quotient_class_congruence_is_reached_by_checked_core_term() {
    assert_core_consumer(
        install_quot_class_fixture_prelude,
        "ClassExpectedOf",
        "class_value",
        1,
        "QuotClass with a reducible representative",
    );
}

/// MEASURED: conv_cast_value applied to its parameters converts to
/// ConvCastExpectedOf applied to those parameters under a neutral Cast.
/// CLAIMED: neutral Cast's component conversion is admitted. THE GAP:
/// independent kernel rejects vary each component.
#[test]
fn cast_congruence_is_reached_by_checked_core_term() {
    assert_core_consumer(
        install_cast_fixture_prelude,
        "ConvCastExpectedOf",
        "conv_cast_value",
        3,
        "neutral Cast with a reducible value",
    );
}

/// MEASURED: conv_quot_elim_value applied to its parameter converts to
/// ConvQuotElimExpectedOf applied to that parameter; methods differ only by
/// transparent identity. CLAIMED: QuotElim components convert. THE GAP: independent
/// kernel rejects cover mismatched motive, method, respect and scrutinee.
#[test]
fn quotient_elim_congruence_is_reached_by_checked_core_term() {
    assert_core_consumer(
        install_quot_elim_fixture_prelude,
        "ConvQuotElimExpectedOf",
        "conv_quot_elim_value",
        1,
        "neutral QuotElim with a reducible method",
    );
}
