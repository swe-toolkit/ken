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

/// Extends [`fin_env`] with a witness family `Wit a n e` indexed by BOTH the
/// length `n` and the `Env a n` value `e`, so a captured `h : Wit a n xs` depends
/// transitively on the captured `xs : Env a n`.
fn fin_env_wit() -> ElabEnv {
    let mut env = fin_env();
    env.elaborate_decl(
        "data Wit (a : Type) : (n : Nat) -> Env a n -> Type where { \
           MkWit : (n : Nat) -> (e : Env a n) -> Wit a n e }",
    )
    .expect("Wit");
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

#[test]
fn captured_env_index_couples_goal_type_argument() {
    // The goal's TYPE argument names the index directly (`Equal (Env a n) xs
    // xs`): the captured `xs`'s refined type and the goal's own `n` must refine
    // in lockstep. Without the convoy forcing the goal's index rebase (the
    // scrutinee `j` does not itself occur here), `xs`'s binder index and the
    // goal's `Env a n` diverge and the arm reddens on the convoy class.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem env_refl (a : Type) (n : Nat) (xs : Env a n) (j : Fin n) \
           : Equal (Env a n) xs xs = \
         match j { FZ m ↦ Refl; FS m k ↦ Refl }",
    )
    .expect("the captured env's type must follow its own index refinement");
}

#[test]
fn transitive_closure_follows_index_refinement() {
    // THE TRANSITIVE SHAPE: the captured tail is `xs : Env a n`, then
    // `h : Wit a n xs` depending on BOTH the index and the earlier captured
    // `xs`. Both must travel as ONE telescope through the `Fin n` refinement of
    // `match j`, with `h`'s binder type naming `xs`'s binder (not the ambient
    // `xs`). This exercises the inter-convoy threading (c > 1) in the motive,
    // the constructor goal, and the recursive-FS direct IH.
    let mut env = fin_env_wit();
    env.elaborate_decl(
        "theorem wit_refl (a : Type) (n : Nat) (xs : Env a n) (h : Wit a n xs) \
           (j : Fin n) : Equal (Wit a n xs) h h = \
         match j { FZ m ↦ Refl; FS m k ↦ Refl }",
    )
    .expect("the transitive Wit/Env telescope must follow the Fin refinement");
}
