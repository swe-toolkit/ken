//! Canonical natural polynomials: spec/50-stdlib/66-symbolic-algebra.md §2.
//! Source: conformance/stdlib/symbolic-algebra/seed-nat-polynomial-canonical-equality.md.
//! Promise class: durable invariant. Independent equal-pair and unequal-pair
//! observations distinguish canonical comparison from ordered-list comparison
//! and a constant answer; generic clients pin all five law interfaces.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-canonical-equality.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial including canonical equality roots-loads");
    env
}

// The seed's fenced Ken programs are compiler inputs, not source-text facts.
// Two positives run in order before the independent false-theorem input.
fn seed_programs() -> Vec<&'static str> {
    SEED.split("\n```ken\n")
        .skip(1)
        .map(|fence| fence.split_once("\n```").expect("closed Ken fence").0)
        .collect()
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §2.1–2.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after five explicit providers load, the exact trusted GlobalId
/// set is unchanged on loading the extended package, each new operation and
/// law has a package-owned transparent body, and arbitrary-expression clients
/// restate all five theorem interfaces. Private helper imports are refused.
/// CLAIMED: §2 provides all its generic proof bodies without additional trust
/// or exposing its canonicity/cancellation proof scaffolding to clients.
/// THE GAP: the providers' assumptions are inherited, and the independent
/// closed semantic discriminator below supplements these generic interfaces.
#[test]
fn section_two_generic_laws_and_public_operations_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.Derived",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial §2 generic proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§2 adds no trusted GlobalId");

    for public in [
        "eval_poly",
        "canonicalize",
        "poly_eq",
        "canonicalize_sound",
        "canonicalize_complete",
        "poly_eq_sound",
        "poly_eq_complete",
        "poly_eq_false_refutes",
    ] {
        let id = env.globals[&format!("{PACKAGE}.{public}")];
        assert!(owned.contains(&id), "{public} must belong to NatPolynomial");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} must have a checked body"
        );
    }
    assert!(owned.contains(&env.globals[&format!("{PACKAGE}.Poly")]));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independently restated §2.2 laws elaborate at arbitrary expressions");

    for private in ["add_cancel_right", "canon", "is_canonical", "poly_complete"] {
        match env.elaborate_file(&format!("import {PACKAGE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, Poly, eval, eval_poly, canonicalize, poly_eq,
   canonicalize_sound, canonicalize_complete, poly_eq_sound,
   poly_eq_complete, poly_eq_false_refutes)

theorem client_canonicalize_sound (env : Nat → Nat) (e : Expr) :
  Equal Nat (eval_poly env (canonicalize e)) (eval env e) =
  canonicalize_sound env e

theorem client_canonicalize_complete (e1 : Expr) (e2 : Expr)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2)) :
  Equal Poly (canonicalize e1) (canonicalize e2) =
  canonicalize_complete e1 e2 h

theorem client_poly_eq_sound (e1 : Expr) (e2 : Expr)
    (h : IsTrue (poly_eq e1 e2)) (env : Nat → Nat) :
  Equal Nat (eval env e1) (eval env e2) =
  poly_eq_sound e1 e2 h env

theorem client_poly_eq_complete (e1 : Expr) (e2 : Expr)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2)) :
  IsTrue (poly_eq e1 e2) =
  poly_eq_complete e1 e2 h

theorem client_poly_eq_false_refutes (e1 : Expr) (e2 : Expr)
    (hf : Equal Bool (poly_eq e1 e2) False)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2)) :
  Bottom =
  poly_eq_false_refutes e1 e2 hf h
"#;

fn run_positive_seed(env: &mut ElabEnv, programs: &[&str]) {
    env.elaborate_file(programs[0])
        .expect("§2 closed operation oracles check through client imports");
    env.elaborate_file(programs[1])
        .expect("§2 closed instances of the five checked laws elaborate");
}

/// SOURCE: conformance/stdlib/symbolic-algebra/
/// seed-nat-polynomial-canonical-equality.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all sixteen source-shaped closed theorems check through the
/// client import, including the nontrivial equal pair x₀+x₀ versus 2·x₀,
/// unequal x₀ versus x₁, the shifted variable and retained zero second child.
/// CLAIMED: canonicalization and Boolean equality observe the §2 Horner
/// semantics, not §1's ordered lists or a constant Boolean answer.
/// THE GAP: the closed cases cannot establish universality; the five generic
/// theorem bodies are checked independently in the previous test.
#[test]
fn seed_operations_and_all_five_law_instances_check_from_client_import() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 3, "two positive and one false seed input");
    let mut env = loaded_package();
    run_positive_seed(&mut env, &programs);
    for name in [
        "equal_x0_sum_and_two_times_x0",
        "eval_x0_sum_oracle",
        "eval_two_times_x0_oracle",
        "eval_horner_x0_double_oracle",
        "eval_horner_x1_oracle",
        "eval_noncanonical_constant_horner_oracle",
        "canonicalize_constant_oracle",
        "canonicalize_x0_sum_oracle",
        "canonicalize_two_times_x0_oracle",
        "poly_eq_equal_pair_oracle",
        "poly_eq_unequal_variables_oracle",
        "canonicalize_sound_instance",
        "canonicalize_complete_instance",
        "poly_eq_sound_instance",
        "poly_eq_complete_instance",
        "poly_eq_false_refutes_instance",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked seed assertion"
        );
    }
}

/// SOURCE: conformance/stdlib/symbolic-algebra/
/// seed-nat-polynomial-canonical-equality.md negative.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after the full positive client including the `False` observation
/// on x₀/x₁, a false `IsTrue` theorem on that identical pair is rejected by
/// the kernel as K0004 TypeMismatch at its own theorem rather than by import,
/// parse, or unrelated fixture errors.
/// CLAIMED: distinct variables cannot be represented as universally equal.
/// THE GAP: this seed refusal does not construct a counterexample environment
/// from `poly_eq_false_refutes`, which only contradicts a universal proof.
#[test]
fn unequal_variable_false_theorem_is_kernel_refused_on_same_positive_pair() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 3, "two positive and one false seed input");
    let mut env = loaded_package();
    run_positive_seed(&mut env, &programs);
    let negative = programs[2];
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let start = negative
                .find("theorem reject_unequal_variables_as_equal")
                .expect("negative theorem declaration");
            assert!(
                span.start >= start && span.end <= negative.len() && span.start < span.end,
                "K0004 must refuse the false theorem's own proof: {span:?}"
            );
        }
        other => panic!("expected kernel TypeMismatch for false IsTrue, got {other:?}"),
    }
}
