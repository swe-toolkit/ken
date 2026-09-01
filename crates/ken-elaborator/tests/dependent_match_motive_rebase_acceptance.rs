//! `LANG-DEPENDENT-MATCH-MOTIVE-REBASE` acceptance.
//!
//! A dependent `match` in a theorem/fn on an index-REFINING recursive family
//! (`Fin nn`, constructor at `Fin (Suc n)`) with an INDEX-AND-VALUE-dependent
//! motive — a goal that couples the scrutinee to its OWN index, e.g.
//! `flt (fin_to_nat nn i) nn` — previously reddened in motive construction
//! (`kernel_infer` on the abstracted motive: the scrutinee binder had type
//! `Fin idx` while the goal kept the outer actual index `nn`, a `Fin idx` vs
//! `Fin nn` mismatch). The three `subst_term_generalize` producer sites (motive,
//! constructor `expected_here`, direct-recursive IH) now rebase the scrutinee's
//! actual indices to their LOCAL indices TOGETHER with the scrutinee, gated on
//! the scrutinee actually occurring in the goal so an uncoupled goal (a
//! result-type index, `Vec`'s `map`/`zip_with`) is left exactly as before.
//!
//! Generic over the recursive family — NOT a `Fin`/`FokFin`/`FokDerivation`
//! special case: the same producers serve `Nat`-indexed and record-indexed
//! families, `fn`/`theorem`, `Type`/`Omega` goals.

use ken_elaborator::ElabEnv;

/// A base env carrying a fresh index-refining recursive family plus the two
/// helper functions the coupling goal is built from.
fn fin_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(
        "data Fin : Nat -> Type where { \
           FZero : (n : Nat) -> Fin (Suc n); \
           FSuc : (n : Nat) -> Fin n -> Fin (Suc n) }",
    )
    .expect("Fin family");
    env.elaborate_decl(
        "fn fin_to_nat (n : Nat) (i : Fin n) : Nat = \
         match i { FZero m ↦ Zero; FSuc m j ↦ Suc (fin_to_nat m j) }",
    )
    .expect("fin_to_nat");
    env.elaborate_decl(
        "fn flt (a : Nat) (b : Nat) : Bool = \
         match b { Zero ↦ False; Suc b2 ↦ \
           match a { Zero ↦ True; Suc a2 ↦ flt a2 b2 } }",
    )
    .expect("flt");
    env
}

#[test]
fn regression_index_and_value_dependent_omega_motive_over_a_recursive_family() {
    // THE REGRESSION. The goal `flt (fin_to_nat nn i) nn` couples the scrutinee
    // `i` with its own index `nn` (both feed `fin_to_nat`, and `nn` also feeds
    // `flt`). Recursion on `i` exercises all three producers (motive,
    // constructor expected goal, direct-recursive IH slot for `j : Fin p`). Red
    // before the rebase (motive-construction kernel mismatch `Fin idx` vs
    // `Fin nn`); green now, and it kernel-checks.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem fin_lt (nn : Nat) (i : Fin nn) \
           : Equal Bool (flt (fin_to_nat nn i) nn) True = \
         match i { FZero p ↦ Proved; FSuc p j ↦ fin_lt p j }",
    )
    .expect("the coupled index-and-value-dependent Omega motive must elaborate");
}

#[test]
fn control_no_match_reflexive_self_equality_still_elaborates() {
    // Positive control: the SAME coupled goal shape proved WITHOUT a match is a
    // reflexive self-equality — unaffected by the match producers.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem fin_id (nn : Nat) (i : Fin nn) \
           : Equal Nat (fin_to_nat nn i) (fin_to_nat nn i) = Refl",
    )
    .expect("the no-match reflexive control must elaborate");
}

#[test]
fn control_constant_motive_match_still_elaborates() {
    // Positive control: a match on the SAME refining family whose goal does NOT
    // mention the scrutinee (constant motive) never needed rebasing and must
    // stay green — the occurrence gate leaves it as the original scrutinee-only
    // pass.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem fin_const (nn : Nat) (i : Fin nn) : Equal Nat Zero Zero = \
         match i { FZero p ↦ Proved; FSuc p j ↦ Proved }",
    )
    .expect("the constant-motive control must elaborate");
}

#[test]
fn control_uncoupled_result_index_goal_is_left_unchanged() {
    // Positive control / the gate's whole point: a Vec-style operation whose
    // goal is a RESULT-type index (`Vec b n`) does NOT mention the scrutinee, so
    // the actual index must NOT be rebased — a blanket rebasing over-generalizes
    // a coincidentally-equal index occurrence. `map` and the NESTED-match
    // `zip_with` (whose inner `ys` length coincides with the result length) are
    // the exact shapes a blanket rebasing corrupted.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(
        "data Vec (a : Type) : Nat -> Type where { \
           VNil : Vec a Zero; \
           VCons : (n : Nat) -> a -> Vec a n -> Vec a (Suc n) }",
    )
    .expect("Vec family");
    env.elaborate_decl(
        "fn vmap (a : Type) (b : Type) (n : Nat) (f : a -> b) (xs : Vec a n) : Vec b n = \
         match xs { \
           VNil ↦ VNil b; \
           VCons m x tail_xs ↦ VCons b m (f x) (vmap a b m f tail_xs) }",
    )
    .expect("uncoupled map must stay green");
    env.elaborate_decl(
        "fn vzip (a : Type) (b : Type) (c : Type) (n : Nat) (f : a -> b -> c) \
           (xs : Vec a n) (ys : Vec b n) : Vec c n = \
         match xs { \
           VNil ↦ VNil c; \
           VCons m x tail_xs ↦ \
             match ys { VCons k y tail_ys ↦ \
               VCons c m (f x y) (vzip a b c m f tail_xs tail_ys) } }",
    )
    .expect("uncoupled nested zip_with must stay green");
}
