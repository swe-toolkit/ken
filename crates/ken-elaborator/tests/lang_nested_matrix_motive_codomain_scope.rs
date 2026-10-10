//! The nested matrix motive's sort is inferred in the context of its split
//! binder, even when the result does not mention that binder.
//! Spec: spec/30-surface/34-data-match.md §3, §4.4;
//! spec/30-surface/39-elaboration.md §2.6.
//! Promise class: durable checked-admission invariants.

use ken_elaborator::ElabEnv;

fn checks(source: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trusted_before, "no new trust");
}

fn result_at_both_levels(name: &str, pattern: &str) {
    for type_of_a in ["Type 1", "Type"] {
        let source = format!(
            "fn {name} (a : {type_of_a}) (n : Nat) (x : a) : a = \
             match n {{ {pattern} }}"
        );
        checks(&source);
    }
}

/// MEASURED: both higher- and base-universe result types check over Bool's
/// two alternative constructors. CLAIMED: the non-reverting motive uses the
/// split-binder context instead of accidentally assigning its codomain Type 0.
/// THE GAP: admission does not demonstrate the full runtime eliminator value;
/// kernel rechecking does establish the level of the admitted core term.
#[test]
fn m1_bool_or_high_universe_motive_checks() {
    for type_of_a in ["Type 1", "Type"] {
        checks(&format!(
            "fn m1 (a : {type_of_a}) (b : Bool) (x : a) : a = \
             match b {{ True | False ↦ x }}"
        ));
    }
}

/// MEASURED: an exhaustive Nat or-pattern checks with an open result `a` at
/// Type 1 and Type. CLAIMED: the split binder is present even when its value
/// does not occur in the codomain. THE GAP: other nested match forms take
/// distinct pattern-matrix routes and need their own row.
#[test]
fn nat_or_high_universe_motive_checks() {
    result_at_both_levels("nat_or", "Zero | Suc _ ↦ x");
}

/// MEASURED: the first nested Suc split and its deeper sibling check at both
/// levels. CLAIMED: successive non-reverting nested motives retain the correct
/// binder depth. THE GAP: this fixes the sort query, not every method telescope.
#[test]
fn nat_nested_high_universe_motive_checks() {
    result_at_both_levels("nat_nested", "Zero | Suc Zero ↦ x; Suc (Suc k) ↦ x");
}

/// MEASURED: a non-or nested pattern and a Type-valued result at Type 1 both
/// remain checkable. CLAIMED: the fix does not erase reverting or higher-sort
/// control paths. THE GAP: only these bounded shapes are claimed here; the
/// catalog census is checked separately.
#[test]
fn nested_motive_non_or_and_type_valued_controls_check() {
    checks(
        "fn nested_plain (a : Type 1) (n : Nat) (x : a) : a = \
         match n { Zero ↦ x; Suc Zero ↦ x; Suc (Suc k) ↦ x }",
    );
    checks(
        "fn m1 (n : Nat) : Type 1 = \
         match n { Zero | Suc Zero ↦ Type; Suc (Suc k) ↦ Type }",
    );
}
