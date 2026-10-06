//! Checked Int literal-carrier registration (spec 14 §5; ADR 0013 Layer 2).
//! Promise class: durable behavioral invariant. Only a live, monomorphic
//! opaque primitive declared at Type may become the unique literal carrier;
//! unsuccessful registration changes no carrier or trusted-base entry.
//! External API privacy is pinned by the doctest on the checked entry point.

use ken_kernel::check::register_checked_int_lit_carrier;
use ken_kernel::{
    declare_def, declare_inductive, declare_primitive, infer, Context, CtorSpec, GlobalEnv,
    GlobalId, InductiveSpec, KernelError, Level, LevelVar, PrimReduction, Term,
};
use num_bigint::BigInt;

fn opaque_type(env: &mut GlobalEnv) -> GlobalId {
    declare_primitive(
        env,
        vec![],
        Term::ty(Level::Zero),
        PrimReduction::OpaqueType,
    )
    .unwrap()
}

fn assert_refused_unchanged(env: &mut GlobalEnv, id: GlobalId) {
    let previous = env.int_lit_type();
    let trust = env.trusted_base();
    assert_eq!(
        register_checked_int_lit_carrier(env, id),
        Err(KernelError::Msg(
            "invalid or duplicate Int literal carrier".into()
        ))
    );
    assert_eq!(env.int_lit_type(), previous);
    assert_eq!(env.trusted_base(), trust);
}

#[test]
fn transparent_alias_cannot_register_as_int_lit_carrier() {
    let mut env = GlobalEnv::new();
    let int = opaque_type(&mut env);
    let alias = declare_def(
        &mut env,
        vec![],
        Term::ty(Level::Zero),
        Term::const_(int, vec![]),
    )
    .unwrap();
    assert_refused_unchanged(&mut env, alias);
}

#[test]
fn inductive_former_cannot_register_as_int_lit_carrier() {
    let mut env = GlobalEnv::new();
    let family = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::Zero,
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .unwrap();
    assert_refused_unchanged(&mut env, family);
}

#[test]
fn non_opaque_primitive_cannot_register_as_int_lit_carrier() {
    let mut env = GlobalEnv::new();
    let operation = declare_primitive(
        &mut env,
        vec![],
        Term::ty(Level::Zero),
        PrimReduction::Op {
            symbol: "not_an_int_carrier",
        },
    )
    .unwrap();
    assert_refused_unchanged(&mut env, operation);
}

#[test]
fn value_primitive_tagged_opaque_type_cannot_register_as_int_lit_carrier() {
    let mut env = GlobalEnv::new();
    let bool_ = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::Zero,
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .unwrap();
    let value = declare_primitive(
        &mut env,
        vec![],
        Term::indformer(bool_, vec![]),
        PrimReduction::OpaqueType,
    )
    .unwrap();
    assert_refused_unchanged(&mut env, value);
}

#[test]
fn omega_primitive_tagged_opaque_type_cannot_register_as_int_lit_carrier() {
    let mut env = GlobalEnv::new();
    let proposition = declare_primitive(
        &mut env,
        vec![],
        Term::omega(Level::Zero),
        PrimReduction::OpaqueType,
    )
    .unwrap();
    assert_refused_unchanged(&mut env, proposition);
}

#[test]
fn polymorphic_primitive_cannot_type_closed_int_literals() {
    let mut env = GlobalEnv::new();
    let u = LevelVar(0);
    let polymorphic = declare_primitive(
        &mut env,
        vec![u],
        Term::ty(Level::Var(u)),
        PrimReduction::OpaqueType,
    )
    .unwrap();
    assert_refused_unchanged(&mut env, polymorphic);
}

#[test]
fn duplicate_registration_refuses_overwrite() {
    let mut env = GlobalEnv::new();
    let first = opaque_type(&mut env);
    let second = opaque_type(&mut env);
    register_checked_int_lit_carrier(&mut env, first).unwrap();
    assert_refused_unchanged(&mut env, second);
    assert_eq!(env.int_lit_type(), Some(first));
}

#[test]
fn live_opaque_int_carrier_types_literals_without_new_trust() {
    let mut env = GlobalEnv::new();
    let int = opaque_type(&mut env);
    let trust = env.trusted_base();
    register_checked_int_lit_carrier(&mut env, int).unwrap();
    assert_eq!(env.int_lit_type(), Some(int));
    assert_eq!(env.trusted_base(), trust);
    let literal = Term::IntLit(BigInt::from(42));
    let int_ty = Term::const_(int, vec![]);
    assert_eq!(infer(&env, &Context::new(), &literal), Ok(int_ty.clone()));
    assert_eq!(
        ken_kernel::check(&env, &Context::new(), &literal, &int_ty),
        Ok(())
    );
}
