//! Kernel-checked clients of exact finite-support statistics.
//!
//! Contract: spec/50-stdlib/63-statistics.md §1. The eight durable-invariant
//! oracles are conformance/stdlib/statistics/seed-exact-distribution.md.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::Decl;

const MODULE: &str = "Algorithm.Statistics.Exact";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// Promise class: durable invariant. MEASURED: the 20 §1 declarations and
/// constructor are rooted in the module, all nine public law signatures have
/// generic checked client terms, private proof helpers cannot be imported,
/// and the exact trusted GlobalId set agrees with the preloaded provider
/// closure. CLAIMED: clients have the checked public surface with zero local
/// trust increase. THE GAP: generic law clients do not exercise the eight
/// independent closed behavior oracles below.
#[test]
fn public_surface_nine_laws_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base environment");
    for provider in [
        "Data.Numeric.Nat.Arithmetic",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Core.Classes.EffectfulClasses",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MODULE)
        .expect("§1 exact-distribution package must roots-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "package adds no trusted GlobalIds");
    for name in [
        "ExactDistribution",
        "entries",
        "one_less_total",
        "total",
        "event_mass",
        "expectation_weight",
        "always_true",
        "always_false",
        "event_union",
        "constant_one",
        "score_add",
        "total_positive",
        "impossible_event",
        "sure_event",
        "complement_mass",
        "disjoint_mass",
        "expectation_unit",
        "expectation_linearity",
        "event_mass_ext",
        "expectation_weight_ext",
    ] {
        let id = env.globals[&format!("{MODULE}.{name}")];
        assert!(owned.contains(&id), "{name} must be package-owned");
        if name != "ExactDistribution" {
            assert!(
                matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
                "{name} must be checked and transparent"
            );
        }
    }
    let distribution = env.globals[&format!("{MODULE}.ExactDistribution")];
    let constructor = env.globals[&format!("{MODULE}.MkExactDistribution")];
    assert!(
        env.env
            .inductive(distribution)
            .expect("distribution is an inductive type")
            .constructors
            .iter()
            .any(|item| item.id == constructor),
        "the public constructor carries the proof-carrying representation"
    );
    env.elaborate_file(
        r#"import Algorithm.Statistics.Exact
             (ExactDistribution, MkExactDistribution, entries, one_less_total,
              total, event_mass, expectation_weight, always_true, always_false,
              event_union, constant_one, score_add, total_positive,
              impossible_event, sure_event, complement_mass, disjoint_mass,
              expectation_unit, expectation_linearity, event_mass_ext,
              expectation_weight_ext)
           import Data.Numeric.Nat.Arithmetic (add)
           import Core.Classes.LawfulClasses (bool_and, bool_not)
           import Core.Classes.EffectfulClasses (compose)
           theorem client_total_positive (q : Type) (d : ExactDistribution q) :
             Equal Nat (total q d) (Suc (one_less_total q d)) =
             total_positive q d
           theorem client_impossible_event (q : Type) (d : ExactDistribution q) :
             Equal Nat (event_mass q d (always_false q)) Zero =
             impossible_event q d
           theorem client_sure_event (q : Type) (d : ExactDistribution q) :
             Equal Nat (event_mass q d (always_true q)) (total q d) =
             sure_event q d
           theorem client_complement_mass
             (q : Type) (d : ExactDistribution q) (p : q → Bool) :
             Equal Nat (add (event_mass q d p)
               (event_mass q d (compose q Bool Bool bool_not p))) (total q d) =
             complement_mass q d p
           theorem client_disjoint_mass
             (q : Type) (d : ExactDistribution q) (p : q → Bool) (r : q → Bool)
             (disjoint : (x : q) → Equal Bool (bool_and (p x) (r x)) False) :
             Equal Nat (event_mass q d (event_union q p r))
               (add (event_mass q d p) (event_mass q d r)) =
             disjoint_mass q d p r disjoint
           theorem client_expectation_unit (q : Type) (d : ExactDistribution q) :
             Equal Nat (expectation_weight q d (constant_one q)) (total q d) =
             expectation_unit q d
           theorem client_expectation_linearity
             (q : Type) (d : ExactDistribution q) (f : q → Nat) (g : q → Nat) :
             Equal Nat (expectation_weight q d (score_add q f g))
               (add (expectation_weight q d f) (expectation_weight q d g)) =
             expectation_linearity q d f g
           theorem client_event_mass_ext
             (q : Type) (d : ExactDistribution q) (p : q → Bool) (r : q → Bool)
             (same : (x : q) → Equal Bool (p x) (r x)) :
             Equal Nat (event_mass q d p) (event_mass q d r) =
             event_mass_ext q d p r same
           theorem client_expectation_weight_ext
             (q : Type) (d : ExactDistribution q) (f : q → Nat) (g : q → Nat)
             (same : (x : q) → Equal Nat (f x) (g x)) :
             Equal Nat (expectation_weight q d f) (expectation_weight q d g) =
             expectation_weight_ext q d f g same"#,
    )
    .expect("20 public declarations, one constructor, and nine law clients elaborate");
    for private in [
        "total_weight",
        "event_weight",
        "expectation_list",
        "choose_weight",
        "shuffle_add",
        "event_weight_ext_list",
    ] {
        match env.elaborate_file(&format!("import {MODULE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MODULE}.{private}"));
            }
            Err(other) => panic!("{private} rejected for unrelated reason: {other:?}"),
            Ok(_) => panic!("{private} must remain private"),
        }
    }
}

const FIXTURE: &str = r#"
import Algorithm.Statistics.Exact
  (ExactDistribution, MkExactDistribution, entries, one_less_total, total,
   event_mass, expectation_weight, always_true, always_false, event_union,
   constant_one, total_positive, impossible_event, sure_event,
   complement_mass, disjoint_mass, expectation_unit)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Core.Classes.LawfulClasses (bool_not, bool_and)
import Core.Classes.EffectfulClasses (compose)

const uniform_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc Zero))
      (Nil (Pair Bool Nat)))
const uniform : ExactDistribution Bool =
  MkExactDistribution Bool uniform_atoms (Suc Zero) Proved
const skew_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc (Suc Zero)))
      (Nil (Pair Bool Nat)))
const skew : ExactDistribution Bool =
  MkExactDistribution Bool skew_atoms (Suc (Suc Zero)) Proved
const doubled_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc (Suc Zero)))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc (Suc Zero)))
      (Nil (Pair Bool Nat)))
const doubled : ExactDistribution Bool =
  MkExactDistribution Bool doubled_atoms (Suc (Suc (Suc Zero))) Proved
const repeated_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc (Suc Zero)))
      (Nil (Pair Bool Nat)))
const repeated : ExactDistribution Bool =
  MkExactDistribution Bool repeated_atoms (Suc (Suc Zero)) Proved
const zero_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True Zero)
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc Zero))
      (Nil (Pair Bool Nat)))
const zero_weight : ExactDistribution Bool =
  MkExactDistribution Bool zero_atoms Zero Proved
const bool_true : Bool → Bool = λx. x
const bool_false : Bool → Bool = λx. bool_not x
const score_true : Bool → Nat =
  λx. match x { True ↦ Suc Zero; False ↦ Zero }
theorem bool_true_false_disjoint (x : Bool) :
  Equal Bool (bool_and (bool_true x) (bool_false x)) False =
  match x { True ↦ Proved; False ↦ Proved }
"#;

fn fixture() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[root()], MODULE)
        .expect("real exact-distribution package must roots-load");
    env.elaborate_file(FIXTURE)
        .expect("all eight seed fixtures elaborate with checked positive totals");
    env
}

fn numeral(n: usize) -> String {
    let mut target = "Zero".to_owned();
    for _ in 0..n {
        target = format!("Suc ({target})");
    }
    target
}

fn number(env: &mut ElabEnv, name: &str, expr: &str, expected: usize) {
    let target = numeral(expected);
    env.elaborate_file(&format!(
        "theorem seed_{name} : Equal Nat ({expr}) ({target}) = Proved"
    ))
    .unwrap_or_else(|error| panic!("{name}: kernel must reduce to {target}: {error:?}"));
}

fn ratio_equal(env: &mut ElabEnv, name: &str, left: &str, right: &str) {
    env.elaborate_file(&format!(
        "theorem seed_{name} : Equal Nat ({left}) ({right}) = Proved"
    ))
    .unwrap_or_else(|error| panic!("{name}: kernel must equate cross-products: {error:?}"));
}

/// Promise class: durable invariant; the uniform/unequal-skew pair
/// distinguishes actual weight sums from mere list lengths. MEASURED: four
/// closed natural observations; CLAIMED: probabilities use mass over the
/// positive total; THE GAP: the all-zero constructor rejection is QA's AC-4.
#[test]
fn seed_uniform_true_probability() {
    let mut env = fixture();
    number(&mut env, "uniform_total", "total Bool uniform", 2);
    number(
        &mut env,
        "uniform_one_less",
        "one_less_total Bool uniform",
        1,
    );
    number(
        &mut env,
        "uniform_true",
        "event_mass Bool uniform bool_true",
        1,
    );
    number(
        &mut env,
        "uniform_false",
        "event_mass Bool uniform bool_false",
        1,
    );
    env.elaborate_file(
        "theorem seed_uniform_positive : Equal Nat (total Bool uniform) \
         (Suc (one_less_total Bool uniform)) = total_positive Bool uniform",
    )
    .expect("closed positive-total proof must be usable from public API");
}

/// Promise class: durable invariant; scaling both weights preserves the ratio
/// but skewing just the false weight changes it. Both unequal cross-products
/// are independently kernel-checked to unequal natural literals (3 and 2).
#[test]
fn seed_skew_and_cross_multiplied_ratios() {
    let mut env = fixture();
    number(&mut env, "skew_true", "event_mass Bool skew bool_true", 1);
    number(&mut env, "skew_false", "event_mass Bool skew bool_false", 2);
    number(&mut env, "skew_total", "total Bool skew", 3);
    number(
        &mut env,
        "doubled_true",
        "event_mass Bool doubled bool_true",
        2,
    );
    number(
        &mut env,
        "doubled_false",
        "event_mass Bool doubled bool_false",
        2,
    );
    number(&mut env, "doubled_total", "total Bool doubled", 4);
    ratio_equal(
        &mut env,
        "ratio_scaled",
        "mul (event_mass Bool uniform bool_true) (total Bool doubled)",
        "mul (event_mass Bool doubled bool_true) (total Bool uniform)",
    );
    number(
        &mut env,
        "ratio_skew_left",
        "mul (event_mass Bool uniform bool_true) (total Bool skew)",
        3,
    );
    number(
        &mut env,
        "ratio_skew_right",
        "mul (event_mass Bool skew bool_true) (total Bool uniform)",
        2,
    );
}

/// Promise class: durable invariant; same positive denominator, opposite
/// Boolean events. The closed instances use the exported generic laws.
#[test]
fn seed_sure_and_impossible_events() {
    let mut env = fixture();
    number(
        &mut env,
        "sure",
        "event_mass Bool skew (always_true Bool)",
        3,
    );
    number(
        &mut env,
        "impossible",
        "event_mass Bool skew (always_false Bool)",
        0,
    );
    env.elaborate_file(
        "theorem seed_sure_law : Equal Nat \
         (event_mass Bool skew (always_true Bool)) (total Bool skew) = \
         sure_event Bool skew \
         theorem seed_impossible_law : Equal Nat \
         (event_mass Bool skew (always_false Bool)) Zero = \
         impossible_event Bool skew",
    )
    .expect("closed sure/impossible public law instances");
}

/// Promise class: durable invariant; nonsymmetric weights (1 and 2) pin
/// complement through the imported compose, not a second event operation.
#[test]
fn seed_complement_partitions_skew_total() {
    let mut env = fixture();
    let complement = "event_mass Bool skew (compose Bool Bool Bool bool_not bool_true)";
    number(&mut env, "complement", complement, 2);
    number(
        &mut env,
        "complement_false",
        "event_mass Bool skew bool_false",
        2,
    );
    number(
        &mut env,
        "partition",
        &format!("add (event_mass Bool skew bool_true) ({complement})"),
        3,
    );
    env.elaborate_file(
        "theorem seed_complement_law : Equal Nat \
         (add (event_mass Bool skew bool_true) \
         (event_mass Bool skew (compose Bool Bool Bool bool_not bool_true))) \
         (total Bool skew) = complement_mass Bool skew bool_true",
    )
    .expect("closed complement law with shared composer");
}

/// Promise class: durable invariant; the pointwise-disjoint union receives
/// two distinct nonzero weights and agrees with their separately observed sum.
#[test]
fn seed_disjoint_union_counts_each_weight_once() {
    let mut env = fixture();
    number(
        &mut env,
        "union",
        "event_mass Bool skew (event_union Bool bool_true bool_false)",
        3,
    );
    number(&mut env, "union_true", "event_mass Bool skew bool_true", 1);
    number(
        &mut env,
        "union_false",
        "event_mass Bool skew bool_false",
        2,
    );
    env.elaborate_file(
        "theorem seed_disjoint_law : Equal Nat \
         (event_mass Bool skew (event_union Bool bool_true bool_false)) \
         (add (event_mass Bool skew bool_true) (event_mass Bool skew bool_false)) = \
         disjoint_mass Bool skew bool_true bool_false bool_true_false_disjoint",
    )
    .expect("closed instance of genuinely pointwise disjointness");
}

/// Promise class: durable invariant; a zero score on the weight-two atom
/// distinguishes the weighted expectation from its unit-score control.
#[test]
fn seed_skew_natural_score_expectation() {
    let mut env = fixture();
    number(
        &mut env,
        "score_true",
        "expectation_weight Bool skew score_true",
        1,
    );
    number(
        &mut env,
        "score_one",
        "expectation_weight Bool skew (constant_one Bool)",
        3,
    );
    env.elaborate_file(
        "theorem seed_unit_law : Equal Nat \
         (expectation_weight Bool skew (constant_one Bool)) (total Bool skew) = \
         expectation_unit Bool skew",
    )
    .expect("closed unit-expectation law instance");
}

/// Promise class: durable invariant; the equal-total pair moves only the
/// weight-two atom's vertex, so first-wins or last-wins lookup is detected.
#[test]
fn seed_repeated_atom_adds_not_deduplicates() {
    let mut env = fixture();
    number(&mut env, "repeated_total", "total Bool repeated", 3);
    number(
        &mut env,
        "repeated_true",
        "event_mass Bool repeated bool_true",
        3,
    );
    number(
        &mut env,
        "repeated_false",
        "event_mass Bool repeated bool_false",
        0,
    );
    number(&mut env, "skew_same_total", "total Bool skew", 3);
    number(
        &mut env,
        "skew_other_true",
        "event_mass Bool skew bool_true",
        1,
    );
    number(
        &mut env,
        "skew_other_false",
        "event_mass Bool skew bool_false",
        2,
    );
}

/// Promise class: durable invariant; a zero-weight entry is still stored,
/// while its independent positive companion makes the total proof possible.
#[test]
fn seed_zero_weight_atom_remains_valid() {
    let mut env = fixture();
    number(&mut env, "zero_total", "total Bool zero_weight", 1);
    number(
        &mut env,
        "zero_true",
        "event_mass Bool zero_weight bool_true",
        0,
    );
    number(
        &mut env,
        "zero_false",
        "event_mass Bool zero_weight bool_false",
        1,
    );
    number(
        &mut env,
        "unchanged_false",
        "event_mass Bool uniform bool_false",
        1,
    );
    number(&mut env, "uniform_control_total", "total Bool uniform", 2);
    number(
        &mut env,
        "uniform_control_true",
        "event_mass Bool uniform bool_true",
        1,
    );
    env.elaborate_file(
        "theorem seed_zero_entries : Equal (List (Pair Bool Nat)) \
         (entries Bool zero_weight) zero_atoms = Refl",
    )
    .expect("the zero-weight atom remains in the original entry list");
}
