//! A match arm reaches the kernel with all declaration levels zonked.
//! Spec: `spec/30-surface/39-elaboration.md §5.7` (bare `Type`).
//! Promise class: durable invariant; explicit `Type 0` is a one-axis control.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, Term};

const VEC: &str = "data Vec (a : Type) : Nat → Type where { \
                   VNil : Vec a Zero; \
                   VCons : (n : Nat) → a → Vec a n → Vec a (Suc n) }";

fn checked_body(env: &ElabEnv, name: &str) -> Term {
    let id = *env.globals.get(name).expect("named definition");
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("definition must carry a checked body")
    };
    body.clone()
}

fn check_identity_twins(arms: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(VEC).expect("Vec family");
    for (name, level) in [("bare", "Type"), ("explicit", "Type 0")] {
        let source = format!(
            "fn {name} (a : {level}) (n : Nat) (xs : Vec a n) : Vec a n = \
             match xs {{ {arms} }}"
        );
        env.elaborate_decl(&source)
            .unwrap_or_else(|error| panic!("{name} should check: {error:?}"));
    }
    assert_eq!(
        checked_body(&env, "bare"),
        checked_body(&env, "explicit"),
        "bare Type must give the same checked core as explicit Type 0"
    );
}

#[test]
fn bare_type_vec_identity_reusing_scrutinee_matches_explicit_core() {
    check_identity_twins("VNil ↦ xs; VCons m x t ↦ xs");
}

#[test]
fn bare_type_vec_identity_reconstructing_fields_matches_explicit_core() {
    check_identity_twins("VNil ↦ VNil a; VCons m x t ↦ VCons a m x t");
}

#[test]
fn bare_type_vec_identity_swapped_arms_matches_explicit_core() {
    check_identity_twins("VCons m x t ↦ VCons a m x t; VNil ↦ VNil a");
}

#[test]
fn ill_typed_bare_type_arm_rejects_at_the_arm() {
    for (name, arms, wrong_arm) in [
        ("wrong_nil", "VNil ↦ Zero; VCons m x t ↦ xs", "VNil ↦ Zero"),
        (
            "wrong_cons",
            "VNil ↦ xs; VCons m x t ↦ Zero",
            "VCons m x t ↦ Zero",
        ),
    ] {
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_decl(VEC).expect("Vec family");
        let source = format!(
            "fn {name} (a : Type) (n : Nat) (xs : Vec a n) : Vec a n = match xs {{ {arms} }}"
        );
        let error = env.elaborate_decl(&source).expect_err("Zero is not a Vec");
        let arm = source.find(wrong_arm).expect("written arm");
        match error {
            ElabError::KernelRejected { span, .. } | ElabError::TypeMismatch { span, .. } => {
                assert!(
                    span.start <= arm && span.end >= arm + wrong_arm.len(),
                    "error must belong to the wrong {name} arm: {span:?}"
                );
            }
            other => panic!("expected {name} arm-local type rejection, got {other:?}"),
        }
    }
}
