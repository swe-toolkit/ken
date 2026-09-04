use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    for decl in [
        "data MiniEnv : Nat -> Type where { \
           MiniNil : MiniEnv Zero; \
           MiniCons : (m : Nat) -> Nat -> MiniEnv m -> MiniEnv (Suc m) }",
        "data MiniForm : Nat -> Type where { \
           MiniLeaf : (n : Nat) -> MiniForm n; \
           MiniBind : (n : Nat) -> MiniForm (Suc n) -> MiniForm n }",
    ] {
        env.elaborate_decl(decl)
            .unwrap_or_else(|error| panic!("fixture failed for `{decl}`: {error:?}"));
    }
    env
}

/// Durable invariant (`39 §5.7`, HS21): selecting the plain path for a
/// shifting declared index preserves index-independent recursion.
#[test]
fn constant_motive_stays_green() {
    let mut env = env();
    env.elaborate_decl(
        "theorem mini_constant (m : Nat) (form : MiniForm m) : Top = \
         match form { \
           MiniLeaf n ↦ Proved; \
           MiniBind n body ↦ mini_constant (Suc n) body }",
    )
    .expect("constant motive");
}

/// Durable positive control (`39 §5.7`): the fully written constructor method
/// inhabits the standard eliminator's field-indexed IH type.
#[test]
fn concrete_bind_method_is_green() {
    let mut env = env();
    env.elaborate_decl(
        "fn mini_bind_method \
           (m : Nat) (body : MiniForm (Suc m)) \
           (ih : MiniEnv (Suc m) -> Omega) : \
           MiniEnv m -> Omega = \
         λxs. (x : Nat) -> ih (MiniCons m x xs)",
    )
    .expect("the kernel accepts the explicit declared-index Bind method");
}

/// Durable soundness pair (`39 §5.7`, HS21): the motive co-indexes `MiniEnv`
/// at the recursive field's declared index, while an unchanged wrong-index
/// argument reaches and is rejected by the kernel type checker.
#[test]
fn coupled_successor_motive_elaborates_and_wrong_index_rejects() {
    let mut env = ElabEnv::new().expect("base env");
    for decl in [
        "data MiniEnv : Nat -> Type where { \
           MiniNil : MiniEnv Zero; \
           MiniCons : (m : Nat) -> Nat -> MiniEnv m -> MiniEnv (Suc m) }",
        "data MiniForm : Nat -> Type where { \
           MiniLeaf : (n : Nat) -> MiniForm n; \
           MiniBind : (n : Nat) -> MiniForm (Suc n) -> MiniForm n }",
    ] {
        env.elaborate_decl(decl).expect("fixture");
    }
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_decl(
        "fn mini_coupled (n : Nat) (form : MiniForm n) : MiniEnv n -> Omega = \
         match form { \
           MiniLeaf m ↦ λxs. Top; \
           MiniBind m body ↦ λxs. (x : Nat) -> \
             mini_coupled (Suc n) body (MiniCons n x xs) }",
    )
    .expect("declared-index recursive field uses the plain eliminator motive");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "plain elimination adds no trusted authority");

    let error = env
        .elaborate_decl(
            "fn mini_wrong_index (n : Nat) (form : MiniForm n) : MiniEnv n -> Omega = \
             match form { \
               MiniLeaf m ↦ λxs. Top; \
               MiniBind m body ↦ λxs. (x : Nat) -> \
                 mini_coupled (Suc n) body xs }",
        )
        .expect_err("the Bind branch must not pass MiniEnv n where MiniEnv (Suc n) is required");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "the plain path must reach the wrong-index kernel check, found {error:?}"
    );
}
