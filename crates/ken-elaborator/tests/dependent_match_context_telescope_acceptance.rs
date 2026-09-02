//! `LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE` acceptance.
//!
//! Constructor index refinement in a dependent match must transform the
//! transitive forward-dependency closure of the LOCAL CONTEXT — the ordered
//! dependent tail of captured ambient bindings — as one telescope substitution,
//! together with the motive, constructor expected goal, and direct IH. A
//! captured `xs : Env n` must follow the refinement of the `Fin n` index under
//! `match j`, generalized into the motive codomain and applied back to the
//! original ambient values after the eliminator.

use ken_elaborator::ElabEnv;

/// Base env with a fresh generic index-refining family `Fin`, a length-indexed
/// `Env`, and a total `elookup` — NO Fok names.
fn fin_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(
        "data Fin : Nat -> Type where { \
           FZ : (n : Nat) -> Fin (Suc n); \
           FS : (n : Nat) -> Fin n -> Fin (Suc n) }",
    )
    .expect("Fin");
    env.elaborate_decl(
        "data Env (a : Type) : Nat -> Type where { \
           ENil : Env a Zero; \
           ECons : (n : Nat) -> a -> Env a n -> Env a (Suc n) }",
    )
    .expect("Env");
    env.elaborate_decl(
        "fn elookup (a : Type) (n : Nat) (xs : Env a n) (i : Fin n) : a = \
         match i { \
           FZ m ↦ match xs { ECons k y rest ↦ y }; \
           FS m j ↦ match xs { ECons k y rest ↦ elookup a m rest j } }",
    )
    .expect("elookup");
    env
}

#[test]
fn captured_env_follows_fin_index_refinement_under_match() {
    // THE RED SHAPE (evt_3b9k92cmkn5zh, generic): the captured ambient
    // `xs : Env A n` must follow the `Fin n` refinement of `match j`, so the
    // FS branch's goal `elookup (ECons x xs)(FS n j) = elookup xs j` closes.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem elookup_cons_fs (a : Type) (n : Nat) (x : a) (xs : Env a n) (j : Fin n) \
           : Equal a (elookup a (Suc n) (ECons a n x xs) (FS n j)) (elookup a n xs j) = \
         match j { \
           FZ m ↦ Refl; \
           FS m k ↦ Refl }",
    )
    .expect("the captured-env telescope must follow the Fin refinement");
}
