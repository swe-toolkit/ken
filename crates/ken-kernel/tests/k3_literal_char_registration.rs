//! K3 checked String registration and type-preserving Char core literals
//! refuse unregistered/wrong carriers and invalid Unicode scalars.

use ken_kernel::check::{
    checked_char_literal, declare_checked_string_literal, register_checked_char_carrier,
    register_checked_int_lit_carrier, register_checked_string_carrier,
};
use ken_kernel::{declare_primitive, GlobalEnv, KernelError, Level, LevelVar, PrimReduction, Term};

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
fn polymorphic_char_carrier_cannot_register_a_closed_literal_view() {
    let mut env = GlobalEnv::new();
    let int_ty = opaque_type(&mut env);
    register_checked_int_lit_carrier(&mut env, int_ty).unwrap();
    let sigma = Term::sigma(
        Term::const_(int_ty, vec![]),
        Term::const_(env.top_id(), vec![]),
    );
    let polymorphic = ken_kernel::declare_def(
        &mut env,
        vec![LevelVar(0)],
        Term::ty(Level::Zero),
        sigma.clone(),
    )
    .expect("a declared, level-polymorphic subset is valid even if its body ignores that level");
    assert!(matches!(
        ken_kernel::infer(
            &env,
            &ken_kernel::Context::new(),
            &Term::const_(polymorphic, vec![])
        ),
        Err(KernelError::LevelArityMismatch {
            expected: 1,
            found: 0
        })
    ));
    let before = env.next_global_id();
    assert!(
        matches!(
            register_checked_char_carrier(&mut env, polymorphic),
            Err(KernelError::Msg(message)) if message == "invalid or duplicate Char literal carrier"
        ),
        "a literal cannot inhabit a Char reference with omitted level arguments"
    );
    assert_eq!(env.next_global_id(), before);
    assert!(
        checked_char_literal(&env, 65).is_err(),
        "refusal must not install the invalid carrier"
    );

    let monomorphic =
        ken_kernel::declare_def(&mut env, vec![], Term::ty(Level::Zero), sigma).unwrap();
    register_checked_char_carrier(&mut env, monomorphic).unwrap();
    let scalar = checked_char_literal(&env, 65).unwrap();
    ken_kernel::check(
        &env,
        &ken_kernel::Context::new(),
        &scalar,
        &Term::const_(monomorphic, vec![]),
    )
    .expect("the later monomorphic carrier remains available after refusal");
}

#[test]
fn checked_carriers_reject_wrong_shape_and_char_range_without_partial_admission() {
    let mut env = GlobalEnv::new();
    let int_ty = opaque_type(&mut env);
    register_checked_int_lit_carrier(&mut env, int_ty).unwrap();
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
    let true_prop = Term::const_(env.top_id(), vec![]);
    let char_ty = ken_kernel::declare_def(
        &mut env,
        vec![],
        Term::ty(Level::Zero),
        Term::sigma(Term::const_(int_ty, vec![]), true_prop),
    )
    .unwrap();
    register_checked_char_carrier(&mut env, char_ty).unwrap();
    let before = env.next_global_id();
    assert!(checked_char_literal(&env, 0xD800).is_err());
    assert!(checked_char_literal(&env, 0x110000).is_err());
    assert_eq!(env.next_global_id(), before);
    let scalar = checked_char_literal(&env, 0x1F642).unwrap();
    assert_eq!(scalar, Term::Ascript(
        Box::new(Term::pair(
            Term::IntLit(0x1F642u32.into()),
            Term::const_(env.tt_id(), vec![]),
        )),
        Box::new(Term::const_(char_ty, vec![])),
    ));
    assert!(ken_kernel::check(
        &env,
        &ken_kernel::Context::new(),
        &Term::IntLit(0x1F642u32.into()),
        &Term::const_(char_ty, vec![]),
    ).is_err(), "the bare carrier cannot inhabit a checked Char subset");
    ken_kernel::check(
        &env,
        &ken_kernel::Context::new(),
        &scalar,
        &Term::const_(char_ty, vec![]),
    )
    .expect("the checked pair must inhabit the Char subset");

    let mut false_env = GlobalEnv::new();
    let int_ty = opaque_type(&mut false_env);
    register_checked_int_lit_carrier(&mut false_env, int_ty).unwrap();
    let false_prop = Term::const_(false_env.bottom_id(), vec![]);
    let false_char = ken_kernel::declare_def(
        &mut false_env, vec![], Term::ty(Level::Zero),
        Term::sigma(Term::const_(int_ty, vec![]), false_prop),
    ).unwrap();
    register_checked_char_carrier(&mut false_env, false_char).unwrap();
    assert!(checked_char_literal(&false_env, 65).is_err(),
        "a scalar with a false refinement needs a real certificate");
}
