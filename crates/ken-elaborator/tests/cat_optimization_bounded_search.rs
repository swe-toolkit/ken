//! Finite least-witness search: spec/50-stdlib/72-optimization.md §2.
//! Source: conformance/stdlib/optimization/seed-bounded-search.md.
//! Promise class: durable invariant. Independent least-hit, missing-hit and
//! strict-bound fixtures distinguish false search results; generic clients
//! restate all four public law interfaces.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.Optimization.BoundedSearch";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-bounded-search.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("BoundedSearch and its Nat-order provider roots-load");
    env
}

// These fenced Ken programs are compiler inputs, not textual source assertions.
// Each negative is compiled independently after the complete positive client.
fn seed_programs() -> Vec<&'static str> {
    SEED.split("\n```ken\n")
        .skip(1)
        .map(|fence| fence.split_once("\n```").expect("closed Ken fence").0)
        .collect()
}

/// SOURCE: spec/50-stdlib/72-optimization.md §2.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the package's five public globals have transparent kernel-checked
/// bodies, the exact trusted GlobalId set is unchanged after its imports load,
/// and arbitrary-argument clients check all four theorem types.
/// Every private operation and proof helper refuses selective import.
/// CLAIMED: bounded selection's four general contracts add no trust.
/// THE GAP: the existing provider assumptions are inherited; concrete search
/// results are distinguished separately by the closed conformance fixtures.
#[test]
fn generic_laws_check_without_added_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in ["Core.Logic.Transport", "Data.Numeric.Nat.Order"] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let package_owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("bounded-search proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "BoundedSearch adds no trusted GlobalId");
    for name in [
        "find_below",
        "find_below_sound",
        "find_below_least",
        "find_below_complete",
        "find_below_stable",
    ] {
        let id = env.globals[&format!("{PACKAGE}.{name}")];
        assert!(package_owned.contains(&id), "{name} must be package-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all four §2 generic law interfaces check from a client");
    for private in [
        "find_next",
        "found_at",
        "none_below",
        "found_spec",
        "found_some_sound",
        "found_some_least",
        "find_below_correct",
        "find_next_correct",
        "found_at_correct",
        "lt_suc_not_lt_same",
        "leq_suc_not_leq_same",
    ] {
        match env.elaborate_file(&format!("import {PACKAGE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{private}"));
            }
            other => panic!("{private} must be private: {other:?}"),
        }
    }
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.BoundedSearch
  (find_below, find_below_sound, find_below_least, find_below_complete, find_below_stable)
import Data.Numeric.Nat.Order (leq_nat, lt_nat)

theorem client_sound (p : Nat → Bool) (bound : Nat) (n : Nat) :
  Equal (Option Nat) (find_below p bound) (Some Nat n) →
  And (Equal Bool (lt_nat n bound) True) (Equal Bool (p n) True) =
  find_below_sound p bound n

theorem client_least (p : Nat → Bool) (bound : Nat) (n : Nat) :
  Equal (Option Nat) (find_below p bound) (Some Nat n) →
  (m : Nat) → Equal Bool (lt_nat m n) True → Equal Bool (p m) False =
  find_below_least p bound n

theorem client_complete (p : Nat → Bool) (bound : Nat) :
  Equal (Option Nat) (find_below p bound) (None Nat) →
  (m : Nat) → Equal Bool (lt_nat m bound) True → Equal Bool (p m) False =
  find_below_complete p bound

theorem client_stable (p : Nat → Bool) (bound : Nat) (larger : Nat) (n : Nat) :
  Equal Bool (leq_nat bound larger) True →
  Equal (Option Nat) (find_below p bound) (Some Nat n) →
  Equal (Option Nat) (find_below p larger) (Some Nat n) =
  find_below_stable p bound larger n
"#;

/// SOURCE: conformance/stdlib/optimization/seed-bounded-search.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete positive seed compiles through a client import and
/// checked named theorems establish least hit, a later-hit control, no-hit at
/// bound three, empty range, and instances of every public law.
/// CLAIMED: the least in-range hit is reported and `None` means no in-range hit.
/// THE GAP: the finite fixtures do not replace the generic checked law bodies;
/// no iteration direction, cost, or predicate-call-count assertion is made.
#[test]
fn seed_positive_client_checks_least_hit_and_absence() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 4, "positive plus three refusal programs");
    let mut env = loaded_package();
    env.elaborate_file(programs[0])
        .expect("complete BoundedSearch seed positive client checks");
    for name in [
        "found_least_witness",
        "found_least_witness_below_six",
        "no_witness_below_three",
        "empty_bound_has_no_witness",
        "sound_instance",
        "least_instance",
        "complete_instance",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-bounded-search.md refusals.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after the full positive client checks, each independently false
/// result is kernel-refused with TypeMismatch inside its own theorem. Each
/// positive fixture on the same input passed before the refusal.
/// CLAIMED: a later hit, a missing existing hit, or a non-hit cannot be
/// misrepresented as `find_below`'s returned result.
/// THE GAP: these three counterexamples do not exhaust all predicates/bounds.
#[test]
fn three_false_results_are_refused_at_their_own_theorems() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 4, "positive plus three refusal programs");
    for (negative, name) in [
        (programs[1], "reject_later_witness"),
        (programs[2], "reject_none_when_witness_exists"),
        (programs[3], "reject_non_hit"),
    ] {
        let mut env = loaded_package();
        env.elaborate_file(programs[0])
            .expect("full positive seed must check before a negative");
        match env.elaborate_file(negative) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) => {
                let start = negative.find("theorem ").expect("negative theorem");
                assert!(
                    negative[start..].starts_with(&format!("theorem {name}"))
                        && span.start >= start
                        && span.end <= negative.len()
                        && span.start < span.end,
                    "{name}: refusal must target its own false theorem: {span:?}"
                );
            }
            other => panic!("{name}: expected typed false-result refusal, got {other:?}"),
        }
    }
}
