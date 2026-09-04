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

/// Durable universe-axis soundness pair (`39 §5.7`, HS21 M5): a polymorphic
/// family stays on the plain path with its level intact, while the neighbouring
/// wrong-index body reaches and is rejected by the kernel type checker.
#[test]
fn universe_polymorphic_declared_index_pair_preserves_levels() {
    let mut env = ElabEnv::new().expect("base env");
    for decl in [
        "data PolyEnv (a : Type) : Nat -> Type where { \
           PolyNil : PolyEnv a Zero; \
           PolyCons : (m : Nat) -> a -> PolyEnv a m -> PolyEnv a (Suc m) }",
        "data PolyForm (a : Type) : Nat -> Type where { \
           PolyLeaf : (n : Nat) -> a -> PolyForm a n; \
           PolyBind : (n : Nat) -> PolyForm a (Suc n) -> PolyForm a n }",
    ] {
        env.elaborate_decl(decl)
            .unwrap_or_else(|error| panic!("polymorphic fixture failed for `{decl}`: {error:?}"));
    }
    env.elaborate_decl(
        "fn poly_coupled (a : Type) (n : Nat) (form : PolyForm a n) : \
           PolyEnv a n -> Omega = \
         match form { \
           PolyLeaf m value ↦ λxs. Top; \
           PolyBind m body ↦ λxs. (x : a) -> \
             poly_coupled a (Suc n) body (PolyCons a n x xs) }",
    )
    .expect("the plain path preserves the polymorphic family's universe");

    let error = env
        .elaborate_decl(
            "fn poly_wrong_index (a : Type) (n : Nat) (form : PolyForm a n) : \
               PolyEnv a n -> Omega = \
             match form { \
               PolyLeaf m value ↦ λxs. Top; \
               PolyBind m body ↦ λxs. (x : a) -> \
                 poly_coupled a (Suc n) body xs }",
        )
        .expect_err("the polymorphic Bind branch must reject the wrong environment index");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "the polymorphic wrong-index case must reach the kernel, found {error:?}"
    );
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
