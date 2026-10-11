//! A conflicting index-convoy method retains its dependent goal at Ω l.
//! Promise class: durable invariant. Spec 16 §2.1 and §8.4; spec 34 §3.2.
//! Kernel checking, not a trusted assumption, decides all proof obligations.

use ken_elaborator::ElabEnv;

const DUAL_FIN: &str = "data DualFin : Nat -> Type where { \
    DualFZ : (n : Nat) -> DualFin (Suc n); \
    DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }";
const SPLIT: &str = "match i { DualFZ m ↦ match j0 { \
    DualFZ q ↦ Refl; DualFS q r ↦ Refl }; DualFS m j ↦ Refl }";

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(DUAL_FIN)
        .expect("indexed fixture family");
    env
}

fn checked(source: &str) {
    let mut env = env();
    let trust_before = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    assert_eq!(
        env.env.trusted_base(),
        trust_before,
        "a conflicting index-equation case cannot introduce new trust"
    );
}

#[test]
fn type_universe_equality_goal_checks_without_a_level_zero_method() {
    // MEASURED: the nested indexed match checks Eq Type X X at Ω 1.
    // CLAIMED: an impossible index method need not lower a large proposition.
    // THE GAP: a successful elaboration must recheck its generated selector;
    // the source goes through the production index-convoy kernel check.
    checked(&format!(
        "theorem t1 (X : Type) (n : Nat) \
        (i : DualFin n) (j0 : DualFin n) : Eq Type X X = {SPLIT}"
    ));
}

#[test]
fn large_carrier_theorem_checks_the_conflicting_selector() {
    // MEASURED: the theorem rechecks an Ω 1 goal from a : Type 1.
    // CLAIMED: the impossible index method is typed at its carrier's level.
    // THE GAP: an elaborator-only success would not show kernel admission;
    // this declaration goes through the generated selector's kernel check.
    checked(&format!(
        "theorem t2 (a : Type 1) (x : a) (n : Nat) \
        (i : DualFin n) (j0 : DualFin n) : Eq a x x = {SPLIT}"
    ));
}

#[test]
fn large_carrier_function_let_checks_the_conflicting_selector() {
    // MEASURED: the function's let rechecks an Ω 1 proof from a : Type 1.
    // CLAIMED: this selector remains sound within a value-returning fn.
    // THE GAP: the let's stored proof may be finalized at a different time;
    // a separate test ensures this entry route cannot hide behind the theorem.
    checked(&format!(
        "fn t3 (a : Type 1) (x : a) (n : Nat) \
        (i : DualFin n) (j0 : DualFin n) : a = \
        let p : Eq a x x = {SPLIT} in x"
    ));
}

#[test]
fn small_carrier_sibling_checks_the_same_match() {
    // MEASURED: with only a : Type changed, the matching proof checks.
    // CLAIMED: repairing large goals cannot break their Type-0 twin.
    // THE GAP: this is a same-input level discriminator, not an arbitrary
    // unrelated small proof; the constructor structure is otherwise equal.
    checked(&format!(
        "theorem c2 (a : Type) (x : a) (n : Nat) \
        (i : DualFin n) (j0 : DualFin n) : Eq a x x = {SPLIT}"
    ));
}

#[test]
fn forced_index_return_telescope_stays_at_the_large_goal_level() {
    // MEASURED: a forced Suc index with a return-domain index binder checks
    // an Eq Type X X proof at Ω 1. CLAIMED: replacing the impossible Eq arm
    // does not break the separate forced-telescope path. THE GAP: that path
    // must actually rebase the return binder's n after splitting the index.
    checked(
        "theorem forced_telescope (X : Type) (n : Nat) \
        (i : DualFin (Suc n)) : (j : DualFin n) -> Eq Type X X = \
        match i { DualFZ m ↦ λj. Refl; DualFS m k ↦ λj. Refl }",
    );
}
