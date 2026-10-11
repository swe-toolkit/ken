//! Checked empirical samples and five conformance/stdlib/statistics/seed-empirical-sample.md
//! durable-invariant cases. Contract: spec/50-stdlib/63-statistics.md §2.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const MODULE: &str = "Algorithm.Statistics.Empirical";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[root()], MODULE)
        .expect("empirical package must roots-load");
    env
}

const FIXTURE: &str = r#"
import Algorithm.Statistics.Empirical
  (sample_size, event_count, score_sum, unit_weights,
   empirical_distribution, empirical_entries, empirical_total_size,
   empirical_event_count, empirical_score_sum,
   sample_size_append, event_count_append, score_sum_append)
import Algorithm.Statistics.Exact
  (entries, total, event_mass, expectation_weight, constant_one)
import Data.Collections.NonEmpty
  (NonEmpty, nonempty_singleton, nonempty_cons, nonempty_append,
   nonempty_to_list)
import Data.Collections.Derived (filter)
import Data.Collections.List (length)
import Data.Numeric.Nat.Arithmetic (add)

const lone : NonEmpty Bool = nonempty_singleton Bool True
const duplicate : NonEmpty Bool =
  nonempty_cons Bool True (Cons Bool True (Nil Bool))
const misses : NonEmpty Bool =
  nonempty_cons Bool False (Cons Bool False (Nil Bool))
const joined : NonEmpty Bool = nonempty_append Bool duplicate misses
const is_true : Bool → Bool = λx. x
const score : Bool → Nat =
  λx. match x { True ↦ Suc (Suc Zero); False ↦ Suc Zero }
"#;

fn fixture() -> ElabEnv {
    let mut env = loaded();
    env.elaborate_file(FIXTURE)
        .expect("five seed fixtures must elaborate through selective imports");
    env
}

fn number(n: usize) -> String {
    let mut result = "Zero".to_owned();
    for _ in 0..n {
        result = format!("Suc ({result})");
    }
    result
}

fn checked(env: &mut ElabEnv, name: &str, expression: &str, expected: usize) {
    env.elaborate_file(&format!(
        "theorem seed_{name} : Equal Nat ({expression}) ({}) = Proved",
        number(expected)
    ))
    .unwrap_or_else(|error| panic!("{name}: closed expected value must kernel-check: {error:?}"));
}

fn checked_equation(env: &mut ElabEnv, name: &str, proposition: &str) {
    env.elaborate_file(&format!("theorem seed_{name} : {proposition} = Proved"))
        .unwrap_or_else(|error| panic!("{name}: closed relation must kernel-check: {error:?}"));
}

fn checked_list_equation(env: &mut ElabEnv, name: &str, proposition: &str) {
    env.elaborate_file(&format!("theorem seed_{name} : {proposition} = Refl"))
        .unwrap_or_else(|error| panic!("{name}: list equality must kernel-check: {error:?}"));
}

/// Promise class: durable invariant. MEASURED: roots-loading with all providers
/// present adds no trusted GlobalId, exactly five functions and seven checked
/// laws can be selectively imported, private helpers cannot, and seven generic
/// clients inhabit the public law signatures. CLAIMED: the public empirical
/// tier consists of those checked definitions alone. THE GAP: this does not
/// establish the seed's closed values; five independent cases below do.
#[test]
fn public_surface_generic_laws_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base environment");
    for provider in [
        "Algorithm.Statistics.Exact",
        "Data.Collections.NonEmpty",
        "Data.Collections.Derived",
        "Data.Collections.List",
        "Data.Numeric.Nat.Arithmetic",
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MODULE)
        .expect("empirical provider must load checked proofs");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(
        after, before,
        "empirical provider adds no trusted GlobalIds"
    );

    let expected = [
        "sample_size",
        "event_count",
        "score_sum",
        "unit_weights",
        "empirical_distribution",
        "empirical_entries",
        "empirical_total_size",
        "empirical_event_count",
        "empirical_score_sum",
        "sample_size_append",
        "event_count_append",
        "score_sum_append",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();
    let owned_names = env
        .globals
        .iter()
        .filter_map(|(qualified, id)| {
            qualified
                .strip_prefix(&format!("{MODULE}."))
                .filter(|_| owned.contains(id))
                .map(str::to_owned)
        })
        .collect::<BTreeSet<_>>();
    assert!(
        expected.is_subset(&owned_names),
        "every public name has an owned ID"
    );
    for name in &expected {
        let id = env.globals[&format!("{MODULE}.{name}")];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let mut visible = BTreeSet::new();
    for (index, name) in owned_names.iter().enumerate() {
        match env.elaborate_file(&format!(
            "import {MODULE} ({name} as empirical_probe_{index})"
        )) {
            Ok(_) => {
                visible.insert(name.clone());
            }
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("{MODULE}.{name}"));
            }
            Err(other) => panic!("{name}: unrelated import error: {other:?}"),
        }
    }
    assert_eq!(
        visible, expected,
        "loader-visible API must be exactly §2's names"
    );

    env.elaborate_file(
        r#"import Algorithm.Statistics.Empirical
          (sample_size, event_count, score_sum, unit_weights,
           empirical_distribution, empirical_entries, empirical_total_size,
           empirical_event_count, empirical_score_sum,
           sample_size_append, event_count_append, score_sum_append)
        import Algorithm.Statistics.Exact
          (entries, total, event_mass, expectation_weight)
        import Data.Collections.NonEmpty (NonEmpty, nonempty_append, nonempty_to_list)
        import Data.Numeric.Nat.Arithmetic (add)
        theorem client_entries (q : Type) (s : NonEmpty q) :
          Equal (List (Pair q Nat))
            (entries q (empirical_distribution q s))
            (unit_weights q (nonempty_to_list q s)) = empirical_entries q s
        theorem client_total (q : Type) (s : NonEmpty q) :
          Equal Nat (total q (empirical_distribution q s))
            (sample_size q s) = empirical_total_size q s
        theorem client_event (q : Type) (s : NonEmpty q) (p : q → Bool) :
          Equal Nat (event_mass q (empirical_distribution q s) p)
            (event_count q s p) = empirical_event_count q s p
        theorem client_score (q : Type) (s : NonEmpty q) (f : q → Nat) :
          Equal Nat (expectation_weight q (empirical_distribution q s) f)
            (score_sum q s f) = empirical_score_sum q s f
        theorem client_size_append (q : Type) (x : NonEmpty q) (y : NonEmpty q) :
          Equal Nat (sample_size q (nonempty_append q x y))
            (add (sample_size q x) (sample_size q y)) = sample_size_append q x y
        theorem client_event_append (q : Type) (x : NonEmpty q) (y : NonEmpty q)
          (p : q → Bool) :
          Equal Nat (event_count q (nonempty_append q x y) p)
            (add (event_count q x p) (event_count q y p)) =
          event_count_append q x y p
        theorem client_score_append (q : Type) (x : NonEmpty q) (y : NonEmpty q)
          (f : q → Nat) :
          Equal Nat (score_sum q (nonempty_append q x y) f)
            (add (score_sum q x f) (score_sum q y f)) =
          score_sum_append q x y f"#,
    )
    .expect("all seven public laws must serve generic independent clients");
}

/// Promise class: durable invariant. Singleton total and event count must be
/// positive, distinguishing an empty-tail off-by-one from one unit of mass.
#[test]
fn seed_singleton_positive_total() {
    let mut env = fixture();
    checked(&mut env, "lone_size", "sample_size Bool lone", 1);
    checked(
        &mut env,
        "lone_total",
        "total Bool (empirical_distribution Bool lone)",
        1,
    );
    checked(&mut env, "lone_count", "event_count Bool lone is_true", 1);
    checked(
        &mut env,
        "lone_mass",
        "event_mass Bool (empirical_distribution Bool lone) is_true",
        1,
    );
    checked_equation(
        &mut env,
        "lone_size_bridge",
        "Equal Nat (total Bool (empirical_distribution Bool lone)) (sample_size Bool lone)",
    );
    checked_equation(&mut env, "lone_event_bridge", "Equal Nat (event_mass Bool (empirical_distribution Bool lone) is_true) (event_count Bool lone is_true)");
}

/// Promise class: durable invariant. Two equal observations remain separate
/// entries and contribute twice the mass; one aggregated pair of weight two
/// could fake a numerator but fails the explicit list observation.
#[test]
fn seed_repeated_true_adds_twice_without_deduplication() {
    let mut env = fixture();
    checked(&mut env, "duplicate_size", "sample_size Bool duplicate", 2);
    checked(
        &mut env,
        "duplicate_count",
        "event_count Bool duplicate is_true",
        2,
    );
    checked(
        &mut env,
        "duplicate_mass",
        "event_mass Bool (empirical_distribution Bool duplicate) is_true",
        2,
    );
    checked_list_equation(
        &mut env,
        "duplicate_entries",
        "Equal (List (Pair Bool Nat))
          (entries Bool (empirical_distribution Bool duplicate))
          (Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
            (Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
              (Nil (Pair Bool Nat))))",
    );
    checked_list_equation(&mut env, "duplicate_representation", "Equal (List (Pair Bool Nat)) (entries Bool (empirical_distribution Bool duplicate)) (unit_weights Bool (nonempty_to_list Bool duplicate))");
}

/// Promise class: durable invariant. Rejected observations do not count for
/// the event but do retain unit weights in the positive denominator.
#[test]
fn seed_no_observation_matches() {
    let mut env = fixture();
    checked(
        &mut env,
        "misses_count",
        "event_count Bool misses is_true",
        0,
    );
    checked(
        &mut env,
        "misses_mass",
        "event_mass Bool (empirical_distribution Bool misses) is_true",
        0,
    );
    checked(&mut env, "misses_size", "sample_size Bool misses", 2);
    checked(
        &mut env,
        "misses_total",
        "total Bool (empirical_distribution Bool misses)",
        2,
    );
    checked_equation(&mut env, "misses_bridge", "Equal Nat (event_mass Bool (empirical_distribution Bool misses) is_true) (event_count Bool misses is_true)");
}

/// Promise class: durable invariant. Asymmetric right input contributes two
/// to size and zero to True count, distinguishing a dropped right sample.
#[test]
fn seed_append_preserves_both_contributions() {
    let mut env = fixture();
    checked(&mut env, "left_size", "sample_size Bool duplicate", 2);
    checked(&mut env, "right_size", "sample_size Bool misses", 2);
    checked(&mut env, "joined_size", "sample_size Bool joined", 4);
    checked(
        &mut env,
        "left_event",
        "event_count Bool duplicate is_true",
        2,
    );
    checked(
        &mut env,
        "right_event",
        "event_count Bool misses is_true",
        0,
    );
    checked(
        &mut env,
        "joined_event",
        "event_count Bool joined is_true",
        2,
    );
    checked(
        &mut env,
        "joined_total",
        "total Bool (empirical_distribution Bool joined)",
        4,
    );
    checked(
        &mut env,
        "joined_mass",
        "event_mass Bool (empirical_distribution Bool joined) is_true",
        2,
    );
    checked_equation(&mut env, "size_append", "Equal Nat (sample_size Bool joined) (add (sample_size Bool duplicate) (sample_size Bool misses))");
    checked_equation(&mut env, "event_append", "Equal Nat (event_count Bool joined is_true) (add (event_count Bool duplicate is_true) (event_count Bool misses is_true))");
}

/// Promise class: durable invariant. Different scores distinguish a natural
/// sum (6) from observation count (4) and any normalized returned mean.
#[test]
fn seed_score_sums_unit_weight_entries() {
    let mut env = fixture();
    checked(&mut env, "score_left", "score_sum Bool duplicate score", 4);
    checked(&mut env, "score_right", "score_sum Bool misses score", 2);
    checked(&mut env, "score_joined", "score_sum Bool joined score", 6);
    checked(
        &mut env,
        "score_exact",
        "expectation_weight Bool (empirical_distribution Bool joined) score",
        6,
    );
    checked(
        &mut env,
        "score_constant",
        "expectation_weight Bool (empirical_distribution Bool joined) (constant_one Bool)",
        4,
    );
    checked_equation(&mut env, "score_bridge", "Equal Nat (expectation_weight Bool (empirical_distribution Bool joined) score) (score_sum Bool joined score)");
    checked_equation(&mut env, "score_append", "Equal Nat (score_sum Bool joined score) (add (score_sum Bool duplicate score) (score_sum Bool misses score))");
}

/// Promise class: durable invariant. The generic private provider proof
/// connects the two computations; on these fixtures the independently
/// imported `filter`/`length` agrees with event count, including zero hits.
#[test]
fn filtered_length_agrees_with_event_count_and_extra_unit_rejects() {
    let mut env = fixture();
    for (sample, hits) in [("lone", 1), ("duplicate", 2), ("misses", 0), ("joined", 2)] {
        checked(
            &mut env,
            &format!("filtered_{sample}"),
            &format!("length Bool (filter Bool is_true (nonempty_to_list Bool {sample}))"),
            hits,
        );
        checked_equation(
            &mut env,
            &format!("count_filter_{sample}"),
            &format!("Equal Nat (event_count Bool {sample} is_true) (length Bool (filter Bool is_true (nonempty_to_list Bool {sample})))"),
        );
    }
    match env.elaborate_file(
        "theorem false_extra_unit : Equal Nat (event_count Bool duplicate is_true)
         (Suc (Suc (Suc Zero))) = Proved",
    ) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => {}
        other => panic!("a wrong extra count must be kernel-rejected: {other:?}"),
    }
}
