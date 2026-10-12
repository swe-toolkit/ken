//! Kernel-checked clients of exact finite-support statistics.
//!
//! Contract: spec/50-stdlib/63-statistics.md §1. The eight durable-invariant
//! oracles are conformance/stdlib/statistics/seed-exact-distribution.md.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const MODULE: &str = "Algorithm.Statistics.Exact";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// Promise class: durable invariant. MEASURED: the 31 §1/§3 declarations
/// and constructor are rooted in the module, all 18 public laws have generic
/// checked client terms, and the exact trusted GlobalId set equals the
/// preloaded provider closure. CLAIMED: the checked public surface adds no
/// local trust. THE GAP: generic clients do not replace independent closed
/// value oracles, nor does an owned name alone establish its precise type.
#[test]
fn public_surface_eighteen_laws_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base environment");
    for provider in [
        "Data.Numeric.Nat.Arithmetic",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Core.Classes.EffectfulClasses",
        "Data.Numeric.Nat.Order",
        "Data.Numeric.Rational",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MODULE)
        .expect("§1 exact-distribution package must roots-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
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
        "probability",
        "expectation",
        "probability_numerator",
        "probability_denominator",
        "expectation_numerator",
        "expectation_denominator",
        "probability_cross_bridge",
        "expectation_cross_bridge",
        "probability_mass_bound",
        "probability_leq_one",
        "probability_complement_one",
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
    .expect("§1 public declarations and nine law clients elaborate");
    env.elaborate_file(
        r#"import Algorithm.Statistics.Exact
             (ExactDistribution, event_mass, expectation_weight, total,
              probability, expectation, probability_numerator,
              probability_denominator, expectation_numerator,
              expectation_denominator, probability_cross_bridge,
              expectation_cross_bridge, probability_mass_bound,
              probability_leq_one, probability_complement_one)
           import Data.Numeric.Rational
             (MkRational, numerator, denominator, rational_equiv,
              rational_leq, rational_add, rational_one)
           import Data.Numeric.Nat.Arithmetic (mul)
           import Core.Classes.LawfulClasses (leq_nat, bool_not)
           import Core.Classes.EffectfulClasses (compose)
           theorem client_probability_numerator (q : Type)
             (d : ExactDistribution q) (p : q → Bool) :
             Equal Nat (numerator (probability q d p)) (event_mass q d p) =
             probability_numerator q d p
           theorem client_probability_denominator (q : Type)
             (d : ExactDistribution q) (p : q → Bool) :
             Equal Nat (denominator (probability q d p)) (total q d) =
             probability_denominator q d p
           theorem client_expectation_numerator (q : Type)
             (d : ExactDistribution q) (f : q → Nat) :
             Equal Nat (numerator (expectation q d f))
               (expectation_weight q d f) = expectation_numerator q d f
           theorem client_expectation_denominator (q : Type)
             (d : ExactDistribution q) (f : q → Nat) :
             Equal Nat (denominator (expectation q d f)) (total q d) =
             expectation_denominator q d f
           theorem client_probability_cross_bridge (q : Type)
             (d : ExactDistribution q) (p : q → Bool) (n : Nat) (k : Nat)
             (same : Equal Nat (mul (event_mass q d p) (Suc k))
               (mul n (total q d))) :
             rational_equiv (probability q d p) (MkRational n k) =
             probability_cross_bridge q d p n k same
           theorem client_expectation_cross_bridge (q : Type)
             (d : ExactDistribution q) (f : q → Nat) (n : Nat) (k : Nat)
             (same : Equal Nat (mul (expectation_weight q d f) (Suc k))
               (mul n (total q d))) :
             rational_equiv (expectation q d f) (MkRational n k) =
             expectation_cross_bridge q d f n k same
           theorem client_probability_mass_bound (q : Type)
             (d : ExactDistribution q) (p : q → Bool) :
             Equal Bool (leq_nat (event_mass q d p) (total q d)) True =
             probability_mass_bound q d p
           theorem client_probability_leq_one (q : Type)
             (d : ExactDistribution q) (p : q → Bool) :
             Equal Bool (rational_leq (probability q d p) rational_one) True =
             probability_leq_one q d p
           theorem client_probability_complement_one (q : Type)
             (d : ExactDistribution q) (p : q → Bool) :
             rational_equiv
               (rational_add (probability q d p)
                 (probability q d (compose q Bool Bool bool_not p)))
               rational_one = probability_complement_one q d p"#,
    )
    .expect("nine §3 laws must inhabit exact generic client propositions");
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
import Core.Classes.LawfulClasses (bool_not, bool_and, leq_nat)
import Core.Classes.EffectfulClasses (compose)
import Algorithm.Statistics.Exact
  (probability, expectation, probability_numerator,
   probability_denominator, expectation_numerator,
   expectation_denominator, probability_cross_bridge,
   expectation_cross_bridge, probability_mass_bound,
   probability_leq_one, probability_complement_one)
import Data.Numeric.Rational
  (Rational, MkRational, numerator, denominator, rational_equiv,
   rational_leq, rational_add, rational_one)

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
const rational_score : Bool → Nat =
  λx. match x { True ↦ Suc (Suc Zero); False ↦ Suc Zero }
const two_sixths : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc (Suc (Suc Zero)))))
const one_half : Rational = MkRational (Suc Zero) (Suc Zero)
const four_thirds : Rational =
  MkRational (Suc (Suc (Suc (Suc Zero)))) (Suc (Suc Zero))
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

/// Promise class: durable invariant. MEASURED: a 1/3 result has numerator one,
/// denominator three, and an independently constructed 2/6 value has the same
/// cross product. CLAIMED: the adapter preserves exact event probability.
/// THE GAP: this fixture alone does not prove the generic cross bridge; the
/// independent generic client consumes that checked law above.
#[test]
fn seed_rational_probability_one_third_equals_two_sixths() {
    let mut env = fixture();
    number(
        &mut env,
        "rational_probability_mass",
        "event_mass Bool skew bool_true",
        1,
    );
    number(&mut env, "rational_probability_total", "total Bool skew", 3);
    number(
        &mut env,
        "rational_probability_numerator",
        "numerator (probability Bool skew bool_true)",
        1,
    );
    number(
        &mut env,
        "rational_probability_denominator",
        "denominator (probability Bool skew bool_true)",
        3,
    );
    env.elaborate_file(
        "theorem seed_rational_probability_equiv : \
         rational_equiv (probability Bool skew bool_true) two_sixths = \
         probability_cross_bridge Bool skew bool_true \
         (Suc (Suc Zero)) (Suc (Suc (Suc (Suc (Suc Zero))))) Proved",
    )
    .expect("independent 1/3 ~ 2/6 cross equation supplies the public bridge");
}

/// Promise class: durable invariant. MEASURED: both weighted scores yield 4/3,
/// whose order against one is False. CLAIMED: expectation is not clamped to an
/// event probability. THE GAP: arbitrary scores rely on the generic bridge.
#[test]
fn seed_rational_expectation_four_thirds_exceeds_one() {
    let mut env = fixture();
    number(
        &mut env,
        "rational_expectation_weight",
        "expectation_weight Bool skew rational_score",
        4,
    );
    number(
        &mut env,
        "rational_expectation_numerator",
        "numerator (expectation Bool skew rational_score)",
        4,
    );
    number(
        &mut env,
        "rational_expectation_denominator",
        "denominator (expectation Bool skew rational_score)",
        3,
    );
    env.elaborate_file(
        "theorem seed_rational_expectation_equiv : \
         rational_equiv (expectation Bool skew rational_score) four_thirds = \
         expectation_cross_bridge Bool skew rational_score \
         (Suc (Suc (Suc (Suc Zero)))) (Suc (Suc Zero)) Proved \
         theorem seed_rational_expectation_above_one : \
         Equal Bool (rational_leq (expectation Bool skew rational_score) rational_one) \
         False = Proved",
    )
    .expect("independent score 4/3 is exactly valued and exceeds one");
}

/// Promise class: durable invariant. MEASURED: 1/3 and 1/2 share numerator
/// one yet the first is equivalent to 2/6 and not to 1/2. CLAIMED: equality
/// compares cross products, not just numerators or constructor fields.
/// THE GAP: the contradiction is closed, not a generic inequivalence theorem.
#[test]
fn seed_rational_probability_one_third_is_not_one_half() {
    let mut env = fixture();
    env.elaborate_file(
        "theorem seed_one_third_not_half \
         (claim : rational_equiv (probability Bool skew bool_true) one_half) \
         : Bottom = claim",
    )
    .expect("different closed cross-products make the false claim impossible");
    match env.elaborate_file(
        "theorem seed_wrong_one_third_is_half : \
         rational_equiv (probability Bool skew bool_true) one_half = Proved",
    ) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => {}
        Err(other) => panic!("false equivalence rejected for wrong reason: {other:?}"),
        Ok(_) => panic!("1/3 must not equal 1/2"),
    }
    env.elaborate_file(
        "theorem seed_positive_control_two_sixths : \
         rational_equiv (probability Bool skew bool_true) two_sixths = Proved",
    )
    .expect("opposite closed value result stays accepted on the same adapter");
}

/// Promise class: durable invariant. MEASURED: both nonzero complementary
/// probabilities share denominator three, remain bounded, and sum to one by
/// value. CLAIMED: all Boolean events partition positive natural mass.
/// THE GAP: the imported generic partition and order proofs cover every event.
#[test]
fn seed_rational_probability_bounded_and_complementary() {
    let mut env = fixture();
    number(
        &mut env,
        "rational_complement_mass",
        "event_mass Bool skew (compose Bool Bool Bool bool_not bool_true)",
        2,
    );
    env.elaborate_file(
        "theorem seed_rational_mass_bound : \
         Equal Bool (leq_nat (event_mass Bool skew bool_true) (total Bool skew)) \
         True = probability_mass_bound Bool skew bool_true \
         theorem seed_rational_leq_one : \
         Equal Bool (rational_leq (probability Bool skew bool_true) rational_one) \
         True = probability_leq_one Bool skew bool_true \
         theorem seed_rational_complement_one : \
         rational_equiv (rational_add (probability Bool skew bool_true) \
           (probability Bool skew (compose Bool Bool Bool bool_not bool_true))) \
         rational_one = probability_complement_one Bool skew bool_true",
    )
    .expect("the partition, Boolean order and Rational sum are kernel checked");
}
