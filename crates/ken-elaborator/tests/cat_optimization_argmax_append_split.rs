//! Argmax selection over two appended non-empty parts: §3.6.
//! Promise class: durable invariant. Generic clients pin both new public law
//! types; closed controls distinguish left-tied from strictly-right selection.
//! This package-owned test does not execute the separately owned CV seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.Optimization.Argmin";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("Argmin and its providers root-load with §3.6 proofs");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin, argmax,
    argmin_append_keeps_left, argmin_append_takes_right,
    argmax_append_keeps_left, argmax_append_takes_right)
import Data.Collections.NonEmpty (NonEmpty, nonempty_append)
import Data.Numeric.Nat.Order (lt_nat)

theorem client_min_keep (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs))) False
      → Equal a (argmin a cost (nonempty_append a xs ys)) (argmin a cost xs) =
  argmin_append_keeps_left a cost xs ys

theorem client_min_take (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs))) True
      → Equal a (argmin a cost (nonempty_append a xs ys)) (argmin a cost ys) =
  argmin_append_takes_right a cost xs ys

theorem client_max_keep (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmax a cost xs)) (cost (argmax a cost ys))) False
      → Equal a (argmax a cost (nonempty_append a xs ys)) (argmax a cost xs) =
  argmax_append_keeps_left a cost xs ys

theorem client_max_take (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmax a cost xs)) (cost (argmax a cost ys))) True
      → Equal a (argmax a cost (nonempty_append a xs ys)) (argmax a cost ys) =
  argmax_append_takes_right a cost xs ys
"#;

/// SOURCE: spec/50-stdlib/72-optimization.md §3.6.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the Argmin package owns four checked Transparent append laws;
/// exact inherited trusted GlobalId sets agree. A selective-import client
/// consumes all four generic Boolean-premise contracts. Each of the fourteen
/// private helpers demonstrably exists with a checked body but cannot be
/// selectively imported.
/// CLAIMED: both selectors share checked append proofs without new trust or
/// exposing comparison and induction scaffolding.
/// THE GAP: inherited provider trust remains; closed branch cases below do
/// not replace universally quantified law types.
#[test]
fn append_laws_are_generic_checked_and_private_helpers_stay_private() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Function.Combinators",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Collections.Derived",
        "Data.Collections.NonEmpty",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("all four append proofs root-load with checked bodies");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§3.6 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(
        after, before,
        "append laws add no trusted GlobalId identity"
    );
    for public in [
        "argmin_append_keeps_left",
        "argmin_append_takes_right",
        "argmax_append_keeps_left",
        "argmax_append_takes_right",
    ] {
        let id = env.globals[&format!("{PACKAGE}.{public}")];
        assert!(owned.contains(&id), "{public} is Argmin-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} has a checked proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client checks all four full open append signatures");
    for private in [
        "choose",
        "costlier",
        "select",
        "select_view",
        "select_via_view",
        "holds_at",
        "transitive_at",
        "select_append_choice",
        "select_from_append",
        "choose_better_assoc",
        "lt_nat_true_trans",
        "lt_nat_false_trans",
        "costlier_true_trans",
        "costlier_false_trans",
    ] {
        let id = env.globals[&format!("{PACKAGE}.{private}")];
        assert!(owned.contains(&id), "{private} must exist in Argmin");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{private} must have a checked body"
        );
        match env.elaborate_file(&format!("import {PACKAGE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

const CLOSED_PREFIX: &str = r#"
import Algorithm.Optimization.Argmin
  (argmax, argmax_append_keeps_left, argmax_append_takes_right)
import Data.Collections.NonEmpty (NonEmpty, nonempty_cons, nonempty_append)
import Data.Numeric.Nat.Order (lt_nat)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const seven : Nat = Suc (Suc (Suc (Suc (Suc two))))
const eight : Nat = Suc seven

fn label_cost (n : Nat) : Nat =
  match n {
    Zero ↦ two;
    Suc m ↦ match m { Zero ↦ seven; Suc k ↦ match k { Zero ↦ seven; Suc j ↦ one } }
  }

fn raised_cost (n : Nat) : Nat =
  match n {
    Zero ↦ two;
    Suc m ↦ match m { Zero ↦ seven; Suc k ↦ match k { Zero ↦ eight; Suc j ↦ one } }
  }

const left_part : NonEmpty Nat = nonempty_cons Nat Zero (Cons Nat one (Nil Nat))
const right_part : NonEmpty Nat = nonempty_cons Nat two (Cons Nat three (Nil Nat))

theorem tied_left_cost : Equal Nat (label_cost one) seven = Proved
theorem tied_right_cost : Equal Nat (label_cost two) seven = Proved
theorem raised_right_cost : Equal Nat (raised_cost two) eight = Proved

theorem tie_comparison_false : Equal Bool
    (lt_nat (label_cost (argmax Nat label_cost left_part))
            (label_cost (argmax Nat label_cost right_part))) False = Proved

theorem raised_comparison_true : Equal Bool
    (lt_nat (raised_cost (argmax Nat raised_cost left_part))
            (raised_cost (argmax Nat raised_cost right_part))) True = Proved

theorem tie_keeps_left_by_law
    : Equal Nat
        (argmax Nat label_cost (nonempty_append Nat left_part right_part))
        (argmax Nat label_cost left_part) =
  argmax_append_keeps_left Nat label_cost left_part right_part tie_comparison_false

theorem tie_selects_one : Equal Nat
    (argmax Nat label_cost (nonempty_append Nat left_part right_part)) one = Proved

theorem left_selects_one : Equal Nat (argmax Nat label_cost left_part) one = Proved
theorem right_selects_two : Equal Nat (argmax Nat label_cost right_part) two = Proved

theorem raised_takes_right_by_law
    : Equal Nat
        (argmax Nat raised_cost (nonempty_append Nat left_part right_part))
        (argmax Nat raised_cost right_part) =
  argmax_append_takes_right Nat raised_cost left_part right_part raised_comparison_true

theorem raised_selects_two : Equal Nat
    (argmax Nat raised_cost (nonempty_append Nat left_part right_part)) two = Proved
"#;

const FALSE_LATER_TIE: &str = r#"
theorem reject_later_tie : Equal Nat
    (argmax Nat label_cost (nonempty_append Nat left_part right_part)) two = Proved
"#;

const FALSE_TAKE_ON_TIE: &str = r#"
theorem reject_take_right_on_tie : Equal Nat
    (argmax Nat label_cost (nonempty_append Nat left_part right_part))
    (argmax Nat label_cost right_part) =
  argmax_append_takes_right Nat label_cost left_part right_part Proved
"#;

const FALSE_KEEP_ON_RAISED: &str = r#"
theorem reject_keep_left_on_raised : Equal Nat
    (argmax Nat raised_cost (nonempty_append Nat left_part right_part))
    (argmax Nat raised_cost left_part) =
  argmax_append_keeps_left Nat raised_cost left_part right_part Proved
"#;

fn refuses_at_own_theorem(env: &mut ElabEnv, name: &str, negative: &str) {
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let start = negative
                .find(&format!("theorem {name}"))
                .expect("isolated negative has its own theorem");
            assert!(
                span.start >= start && span.start < span.end && span.end <= negative.len(),
                "{name} must fail at its own proof: {span:?}"
            );
        }
        other => panic!("{name} must refuse with kernel TypeMismatch: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/72-optimization.md §3.6; package-owned fixtures.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a complete accepted prefix independently checks the tied 7/7
/// split's costs, comparison, both part selections, law use, and label-one
/// result, plus the raised 7/8 comparison, law use, and label-two result.
/// Each isolated false claim follows a fresh accepted prefix and must reject
/// at its own proof with a kernel type mismatch.
/// CLAIMED: a tied split retains the earlier left value, a strictly costlier
/// right split wins, and neither law grants a false premise.
/// THE GAP: these finite witnesses supplement checked generic proofs; CV
/// owns the separate conformance seed and its official execution.
#[test]
fn closed_tie_and_raised_cases_accept_and_false_claims_refuse() {
    let mut positive = loaded_package();
    positive
        .elaborate_file(CLOSED_PREFIX)
        .expect("tie-left and strictly-costlier-right checked clients accept");
    for (name, negative) in [
        ("reject_later_tie", FALSE_LATER_TIE),
        ("reject_take_right_on_tie", FALSE_TAKE_ON_TIE),
        ("reject_keep_left_on_raised", FALSE_KEEP_ON_RAISED),
    ] {
        let mut isolated = loaded_package();
        isolated
            .elaborate_file(CLOSED_PREFIX)
            .expect("all positive controls check before each isolated refusal");
        refuses_at_own_theorem(&mut isolated, name, negative);
    }
}
