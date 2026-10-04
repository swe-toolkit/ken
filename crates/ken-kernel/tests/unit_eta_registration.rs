//! `14 §4` / `17 §2`: Unit-η belongs to a designated checked family, not
//! every family with one nullary constructor. Durable conversion/refusal pins.

use ken_kernel::check::register_unit_type;
use ken_kernel::env::Context;
use ken_kernel::term::{Level, LevelVar, Term};
use ken_kernel::{
    convert, declare_inductive, infer, CtorSpec, GlobalEnv, GlobalId, InductiveSpec, KernelError,
};

fn nullary() -> CtorSpec {
    CtorSpec {
        args: vec![],
        target_indices: vec![],
    }
}

fn expect_invalid_registration(env: &mut GlobalEnv, id: GlobalId) {
    assert!(matches!(
        register_unit_type(env, id),
        Err(KernelError::Msg(message)) if message == "invalid or duplicate Unit type"
    ));
}

fn nat_with_zero(env: &mut GlobalEnv) -> (GlobalId, GlobalId) {
    let nat = declare_inductive(env, |id| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![
            nullary(),
            CtorSpec {
                args: vec![Term::indformer(id, vec![])],
                target_indices: vec![],
            },
        ],
    })
    .expect("Nat");
    (nat, env.inductive(nat).expect("Nat").constructors[0].id)
}

fn indexed_one_constructor(env: &mut GlobalEnv) -> (GlobalId, Term) {
    let (nat, zero) = nat_with_zero(env);
    let family = declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![Term::indformer(nat, vec![])],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![Term::constructor(zero, vec![])],
        }],
    })
    .expect("indexed family");
    (
        family,
        Term::app(
            Term::indformer(family, vec![]),
            Term::constructor(zero, vec![]),
        ),
    )
}

fn assert_distinct_neutrals_do_not_convert(env: &GlobalEnv, ty: &Term) {
    let mut ctx = Context::new();
    ctx.push(ty.clone());
    ctx.push(ty.clone());
    assert_eq!(infer(env, &ctx, &Term::var(1)), Ok(ty.clone()));
    assert_eq!(infer(env, &ctx, &Term::var(0)), Ok(ty.clone()));
    assert!(
        !convert(env, &ctx, ty, &Term::var(1), &Term::var(0)),
        "two distinct neutral values cannot take unregistered data-eta"
    );
}

#[test]
fn unregistered_solo_nullary_family_has_no_eta() {
    let mut env = GlobalEnv::new();
    let solo = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![nullary()],
    })
    .expect("Solo");
    assert_eq!(env.unit_type(), None);
    assert_distinct_neutrals_do_not_convert(&env, &Term::indformer(solo, vec![]));
}

#[test]
fn unregistered_indexed_nullary_family_has_no_eta_at_a_real_index() {
    let mut env = GlobalEnv::new();
    let (same, same_zero) = indexed_one_constructor(&mut env);
    assert!(env.inductive(same).is_some());
    assert_eq!(env.unit_type(), None);
    assert_distinct_neutrals_do_not_convert(&env, &same_zero);
}

#[test]
fn registering_a_second_unit_is_rejected_without_changing_identity() {
    let mut env = GlobalEnv::new();
    let first = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![nullary()],
    })
    .expect("first Unit");
    let second = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![nullary()],
    })
    .expect("second Unit-shaped family");
    register_unit_type(&mut env, first).expect("first registration");
    assert_eq!(env.unit_type(), Some(first));
    expect_invalid_registration(&mut env, second);
    assert_eq!(env.unit_type(), Some(first));
    expect_invalid_registration(&mut env, first);
    assert_eq!(env.unit_type(), Some(first));
}

#[test]
fn indexed_family_cannot_be_registered_as_unit() {
    let mut env = GlobalEnv::new();
    let (indexed, _) = indexed_one_constructor(&mut env);
    expect_invalid_registration(&mut env, indexed);
    assert_eq!(env.unit_type(), None);
}

#[test]
fn fielded_constructor_cannot_be_registered_as_unit() {
    let mut env = GlobalEnv::new();
    let (nat, _) = nat_with_zero(&mut env);
    let fielded = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::indformer(nat, vec![])],
            target_indices: vec![],
        }],
    })
    .expect("fielded family");
    expect_invalid_registration(&mut env, fielded);
    assert_eq!(env.unit_type(), None);
}

#[test]
fn two_constructors_cannot_be_registered_as_unit() {
    let mut env = GlobalEnv::new();
    let two = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![nullary(), nullary()],
    })
    .expect("two-constructor family");
    expect_invalid_registration(&mut env, two);
    assert_eq!(env.unit_type(), None);
}

#[test]
fn parameterized_family_cannot_be_registered_as_unit() {
    let mut env = GlobalEnv::new();
    let (nat, _) = nat_with_zero(&mut env);
    let parameterized = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![Term::indformer(nat, vec![])],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![nullary()],
    })
    .expect("parameterized family");
    expect_invalid_registration(&mut env, parameterized);
    assert_eq!(env.unit_type(), None);
}

#[test]
fn level_polymorphic_family_cannot_be_registered_as_unit() {
    let mut env = GlobalEnv::new();
    let polymorphic = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![LevelVar(0)],
        params: vec![],
        indices: vec![],
        level: Level::Var(LevelVar(0)),
        constructors: vec![nullary()],
    })
    .expect("level-polymorphic family");
    expect_invalid_registration(&mut env, polymorphic);
    assert_eq!(env.unit_type(), None);
}

#[test]
fn unallocated_id_cannot_be_registered_as_unit() {
    let mut env = GlobalEnv::new();
    expect_invalid_registration(&mut env, GlobalId(u32::MAX));
    assert_eq!(env.unit_type(), None);
}
