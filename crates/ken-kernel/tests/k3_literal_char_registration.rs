//! K3 checked String registration and type-preserving Char core literals
//! refuse unregistered/wrong carriers and invalid Unicode scalars.

use ken_kernel::check::{
    checked_char_literal, declare_checked_string_literal, register_checked_char_carrier,
    register_checked_string_carrier,
};
use ken_kernel::{declare_primitive, GlobalEnv, Level, PrimReduction, Term};

fn opaque_type(env: &mut GlobalEnv) -> ken_kernel::GlobalId {
    declare_primitive(
        env,
        vec![],
        Term::ty(Level::Zero),
        PrimReduction::OpaqueType,
    )
    .unwrap()
}

#[test]
fn checked_string_literal_normalizes_immutable_payload_and_refuses_absent_carrier() {
    let mut env = GlobalEnv::new();
    let original = env.next_global_id();
    assert!(declare_checked_string_literal(&mut env, "Az").is_err());
    assert_eq!(env.next_global_id(), original);
    let string_ty = opaque_type(&mut env);
    register_checked_string_carrier(&mut env, string_ty).unwrap();
    let id = declare_checked_string_literal(&mut env, "e\u{301}").unwrap();
    assert_eq!(
        env.const_type(id).unwrap().1,
        Term::const_(string_ty, vec![])
    );
    assert_eq!(env.checked_literal(id).unwrap().as_str(), "é");
}

#[test]
fn checked_carriers_reject_wrong_shape_and_char_range_without_partial_admission() {
    let mut env = GlobalEnv::new();
    let int_ty = opaque_type(&mut env);
    env.register_int_lit_type(int_ty);
    let wrong_char_ty = opaque_type(&mut env);
    let before = env.next_global_id();
    assert!(register_checked_char_carrier(&mut env, wrong_char_ty).is_err());
    assert!(checked_char_literal(&env, 65).is_err());
    assert_eq!(env.next_global_id(), before);
    let bad_op = declare_primitive(
        &mut env,
        vec![],
        Term::pi(Term::const_(int_ty, vec![]), Term::const_(int_ty, vec![])),
        PrimReduction::Op {
            symbol: "not_a_carrier",
        },
    )
    .unwrap();
    assert!(register_checked_string_carrier(&mut env, bad_op).is_err());
    let char_ty = ken_kernel::declare_def(
        &mut env,
        vec![],
        Term::ty(Level::Zero),
        Term::const_(int_ty, vec![]),
    )
    .unwrap();
    register_checked_char_carrier(&mut env, char_ty).unwrap();
    let before = env.next_global_id();
    assert!(checked_char_literal(&env, 0xD800).is_err());
    assert!(checked_char_literal(&env, 0x110000).is_err());
    assert_eq!(env.next_global_id(), before);
    let scalar = checked_char_literal(&env, 0x1F642).unwrap();
    assert_eq!(scalar, Term::IntLit(0x1F642u32.into()));
    ken_kernel::check(
        &env,
        &ken_kernel::Context::new(),
        &scalar,
        &Term::const_(char_ty, vec![]),
    )
    .expect("the scalar core value must inhabit the checked Char carrier");
}
