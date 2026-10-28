//! Least-cost candidate selection: spec/50-stdlib/72-optimization.md §1.
//! Source: conformance/stdlib/optimization/seed-argmin.md.
//! Promise class: durable invariant. Independent strict-minimum and earliest-
//! tie vectors distinguish the operation; generic clients pin all eight laws.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const ARGMIN: &str = "Algorithm.Optimization.Argmin";
const ORDER: &str = "Data.Numeric.Nat.Order";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-argmin.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_argmin() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin and its provider closure roots-load");
    env
}

// The seed's fenced Ken programs are executable inputs, not repository text
// asserted by the tests. Each negative contains one independent false goal.
fn seed_programs() -> Vec<&'static str> {
    SEED.split("\n```ken\n")
        .skip(1)
        .map(|fence| fence.split_once("\n```").expect("closed Ken fence").0)
        .collect()
}

// The isolated legacy client already has the canonical IsTrue identity in
// its compiler base. Importing it again creates AmbiguousReference even
// though the provider GlobalId is identical; only that redundant import
// is omitted in memory, leaving the seed's declarations and law uses intact.
fn seed_positive(source: &str) -> String {
    source.replace(
        "import Core.Classes.LawfulClasses (IsTrue, leq_nat)",
        "import Core.Classes.LawfulClasses (leq_nat)",
    )
}

/// SOURCE: spec/50-stdlib/72-optimization.md §§1.2–1.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after explicit providers and two exported Order bridges load,
/// the exact trusted-GlobalId set remains unchanged on loading Argmin; eight
/// package-owned law globals carry transparent checked bodies. An independent
/// client restates each generic law at arbitrary values and cost functions.
/// CLAIMED: selecting an input's earliest least-cost element obeys all eight
/// exported laws without an additional trusted assumption.
/// THE GAP: provider assumptions remain inherited; the closed operational
/// vectors below independently distinguish the two cost and tie directions.
#[test]
fn generic_laws_and_nat_bridges_check_without_added_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Collections.Derived",
        "Data.Collections.NonEmpty",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before_order = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let order_owned = env
        .elaborate_module_from_roots(&[root()], ORDER)
        .expect("Order's checked Nat bridges roots-load");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        before_order,
        "Nat Order bridges add no trusted GlobalId"
    );
    for law in ["true_leq", "leq_not_lt"] {
        let id = env.globals[&format!("{ORDER}.lt_nat::{law}")];
        assert!(
            order_owned.contains(&id),
            "{law} must be owned by Nat Order"
        );
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let before_argmin = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let argmin_owned = env
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin's generic proof bodies kernel-check");
    let after_argmin = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before_argmin.len(),
        after_argmin.len()
    );
    assert_eq!(
        after_argmin, before_argmin,
        "Argmin adds no trusted GlobalId"
    );
    for law in [
        "argmin_member",
        "argmin_least",
        "argmin_no_cheaper",
        "argmin_head_on_tie",
        "argmin_singleton",
        "argmin_keeps_head",
        "argmin_takes_tail",
        "argmin_cost_order_invariant",
    ] {
        let id = env.globals[&format!("{ARGMIN}.{law}")];
        assert!(argmin_owned.contains(&id), "{law} must be Argmin-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all seven §1.2 laws, §1.5 invariance, and two order bridges check");
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin, argmin_member, argmin_least, argmin_no_cheaper,
   argmin_head_on_tie, argmin_singleton, argmin_keeps_head, argmin_takes_tail,
   argmin_cost_order_invariant)
import Core.Classes.LawfulClasses (leq_nat)
import Core.Function.Combinators (comp)
import Data.Collections.Derived (list_elem)
import Data.Collections.NonEmpty
  (NonEmpty, nonempty_cons, nonempty_singleton, nonempty_to_list)
import Data.Numeric.Nat.Order (lt_nat)

theorem client_lt_true_leq (m : Nat) (n : Nat) :
  Equal Bool (lt_nat m n) True → IsTrue (leq_nat m n) =
  (proof true_leq for lt_nat) m n

theorem client_leq_not_lt (m : Nat) (n : Nat) :
  IsTrue (leq_nat m n) → Equal Bool (lt_nat n m) False =
  (proof leq_not_lt for lt_nat) m n

theorem client_member (a : Type) (cost : a → Nat) (xs : NonEmpty a) :
  list_elem a (argmin a cost xs) (nonempty_to_list a xs) =
  argmin_member a cost xs

theorem client_least (a : Type) (cost : a → Nat) (xs : NonEmpty a) :
  (y : a) → list_elem a y (nonempty_to_list a xs) →
  IsTrue (leq_nat (cost (argmin a cost xs)) (cost y)) =
  argmin_least a cost xs

theorem client_no_cheaper (a : Type) (cost : a → Nat) (xs : NonEmpty a) (y : a) :
  list_elem a y (nonempty_to_list a xs) →
  Equal Bool (lt_nat (cost y) (cost (argmin a cost xs))) False =
  argmin_no_cheaper a cost xs y

theorem client_head_on_tie (a : Type) (cost : a → Nat) (x : a) (rest : List a) :
  ((y : a) → list_elem a y rest → IsTrue (leq_nat (cost x) (cost y))) →
  Equal a (argmin a cost (nonempty_cons a x rest)) x =
  argmin_head_on_tie a cost x rest

theorem client_singleton (a : Type) (cost : a → Nat) (x : a) :
  Equal a (argmin a cost (nonempty_singleton a x)) x =
  argmin_singleton a cost x

theorem client_keeps_head (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a) :
  Equal Bool (lt_nat (cost (argmin a cost (nonempty_cons a y ys))) (cost x)) False →
  Equal a (argmin a cost (nonempty_cons a x (Cons a y ys))) x =
  argmin_keeps_head a cost x y ys

theorem client_takes_tail (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a) :
  Equal Bool (lt_nat (cost (argmin a cost (nonempty_cons a y ys))) (cost x)) True →
  Equal a (argmin a cost (nonempty_cons a x (Cons a y ys)))
    (argmin a cost (nonempty_cons a y ys)) =
  argmin_takes_tail a cost x y ys

theorem client_cost_order_invariant
    (a : Type) (cost : a → Nat) (f : Nat → Nat)
    (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
    (xs : NonEmpty a) :
  Equal a (argmin a (comp a Nat Nat f cost) xs) (argmin a cost xs) =
  argmin_cost_order_invariant a cost f same_order xs
"#;

/// SOURCE: conformance/stdlib/optimization/seed-argmin.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all three independently spelled closed results and seven law
/// instances check from the package's client import. The earliest-tie and
/// unique-minimum inputs use different cost outcomes.
/// CLAIMED: a later tied candidate cannot replace the earliest minimum;
/// a strictly cheaper tail candidate must replace the head.
/// THE GAP: these concrete vectors do not prove the universal theorem bodies;
/// the generic-client and transparent-global controls above cover those.
#[test]
fn seed_positive_client_checks_earliest_tie_and_strict_minimum() {
    let programs = seed_programs();
    let mut env = loaded_argmin();
    env.elaborate_file(&seed_positive(programs[0]))
        .expect("complete source-shaped Argmin seed positive client checks");
    for name in [
        "seed_singleton_value",
        "seed_strict_value",
        "seed_tie_earliest",
        "tie_two_member",
        "seed_law_member",
        "seed_law_least",
        "seed_law_no_cheaper",
        "seed_law_head_on_tie",
        "seed_law_singleton",
        "seed_law_keeps_head",
        "seed_law_takes_tail",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-argmin.md refusals.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after the same positive client succeeds, each independent false
/// proposition kernel-rejects at its own theorem. The two wrong results differ
/// on the tie versus strict-minimum axis.
/// CLAIMED: a later tied minimum or a head beaten by another member cannot
/// be misrepresented as the selected result.
/// THE GAP: these two counterexamples do not prove all possible outcomes.
#[test]
fn later_tie_and_beaten_head_are_refused_at_their_own_theorems() {
    let programs = seed_programs();
    for (negative, expected) in [(programs[1], "one"), (programs[2], "three")] {
        let mut env = loaded_argmin();
        env.elaborate_file(&seed_positive(programs[0]))
            .expect("full positive seed must check before each refusal");
        match env.elaborate_file(negative) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) => {
                let start = negative.find("theorem seed_neg").expect("negative theorem");
                assert!(
                    negative[start..].contains(&format!("tie_input) {expected}"))
                        && span.start >= start
                        && span.end <= negative.len()
                        && span.start < span.end,
                    "{expected}: rejection must target its own false theorem: {span:?}"
                );
            }
            other => panic!("{expected}: expected typed false-result rejection, got {other:?}"),
        }
    }
}
