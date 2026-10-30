//! Canonical polynomial equality under both expression constructors: §10.
//! Promise class: durable invariant. Generic clients pin the two checked law
//! interfaces; closed witnesses distinguish a constructor mismatch from false
//! operand premises. This package-owned test does not execute the CV seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial with both §10 laws roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EAdd, EMul, poly_eq, poly_eq_add, poly_eq_mul)

theorem client_add
    (a1 : Expr) (a2 : Expr) (b1 : Expr) (b2 : Expr)
    (ha : IsTrue (poly_eq a1 a2))
    (hb : IsTrue (poly_eq b1 b2))
  : IsTrue (poly_eq (EAdd a1 b1) (EAdd a2 b2)) =
  poly_eq_add a1 a2 b1 b2 ha hb

theorem client_mul
    (a1 : Expr) (a2 : Expr) (b1 : Expr) (b2 : Expr)
    (ha : IsTrue (poly_eq a1 a2))
    (hb : IsTrue (poly_eq b1 b2))
  : IsTrue (poly_eq (EMul a1 b1) (EMul a2 b2)) =
  poly_eq_mul a1 a2 b1 b2 ha hb
"#;

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §10.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the package owns two Transparent law bodies; exact trusted-base
/// GlobalId sets agree before and after roots-loading. A selective-import
/// client consumes the generic two-premise contracts at arbitrary Expr inputs,
/// while the four existing congruence helpers refuse selective import.
/// CLAIMED: both public constructor-preservation laws are checked without
/// adding trust or exposing unrelated private structural/Nat proof scaffolding.
/// THE GAP: inherited provider trust is not erased by a zero package delta;
/// the generic clients do not themselves distinguish a false closed claim.
#[test]
fn generic_constructor_laws_are_owned_checked_and_trust_free() {
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
        .expect("§10 generic proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§10 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§10 must add no trusted GlobalId identity");
    for public in ["poly_eq_add", "poly_eq_mul"] {
        let id = env.globals[&format!("{PACKAGE}.{public}")];
        assert!(owned.contains(&id), "{public} must belong to NatPolynomial");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} must have a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client consumes exactly both two-premise signatures");
    for private in ["add_cong", "mul_cong", "expr_add_cong", "expr_mul_cong"] {
        match env.elaborate_file(&format!("import {PACKAGE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

const CLOSED_PREFIX: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, poly_eq,
   poly_eq_add, poly_eq_mul)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const x0_sum : Expr = EAdd x0 x0
const two_times_x0 : Expr = EMul (EConst two) x0

fn all_ones (v : Nat) : Nat = one

theorem sum_premise : IsTrue (poly_eq x0_sum two_times_x0) = Proved
theorem x1_premise : IsTrue (poly_eq x1 x1) = Proved

theorem add_congruence_instance :
  IsTrue (poly_eq (EAdd x0_sum x1) (EAdd two_times_x0 x1)) =
  poly_eq_add x0_sum two_times_x0 x1 x1 sum_premise x1_premise

theorem mul_congruence_instance :
  IsTrue (poly_eq (EMul x0_sum x1) (EMul two_times_x0 x1)) =
  poly_eq_mul x0_sum two_times_x0 x1 x1 sum_premise x1_premise

theorem add_decision_value :
  IsTrue (poly_eq (EAdd x0_sum x1) (EAdd two_times_x0 x1)) = Proved

theorem mul_decision_value :
  IsTrue (poly_eq (EMul x0_sum x1) (EMul two_times_x0 x1)) = Proved

theorem left_outer_value :
  Equal Nat (eval all_ones (EMul x0_sum x1)) two = Proved

theorem right_outer_value :
  Equal Nat (eval all_ones (EAdd two_times_x0 x1)) three = Proved

theorem false_second_premise :
  Equal Bool (poly_eq x0 x1) False = Proved
"#;

const FALSE_OUTER: &str = r#"
theorem reject_outer_constructor :
  IsTrue (poly_eq (EMul x0_sum x1) (EAdd two_times_x0 x1)) = Proved
"#;

const WRONG_LAW_RESULT: &str = r#"
theorem reject_wrong_constructor_result :
  IsTrue (poly_eq (EMul x0_sum x1) (EAdd two_times_x0 x1)) =
  poly_eq_mul x0_sum two_times_x0 x1 x1 sum_premise x1_premise
"#;

const FALSE_PREMISE: &str = r#"
theorem reject_false_second_premise :
  IsTrue (poly_eq (EAdd x0_sum x0) (EAdd two_times_x0 x1)) =
  poly_eq_add x0_sum two_times_x0 x0 x1 sum_premise Proved
"#;

fn refuses_at_own_theorem(env: &mut ElabEnv, name: &str, negative: &str) {
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let theorem_start = negative
                .find(&format!("theorem {name}"))
                .expect("isolated negative names its theorem");
            assert!(
                span.start >= theorem_start && span.start < span.end && span.end <= negative.len(),
                "{name} must refuse at its own proof: {span:?}"
            );
        }
        other => panic!("{name} must refuse with kernel TypeMismatch: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §10, package-owned witnesses.
/// PROMISE CLASS: durable invariant.
/// MEASURED: both operand-pair premises check, both law applications and
/// direct IsTrue value controls accept; all-ones evaluation yields distinct 2
/// and 3 at the deliberately mismatched outer constructor. Separate inputs
/// after the same accepted prefix refuse the false outer claim, a congruence
/// proof misapplied to the wrong constructor, and a false second premise.
/// CLAIMED: canonical polynomial equality respects either constructor under
/// two true premises without equating different outer constructors or using a
/// false premise as proof.
/// THE GAP: these closed witnesses supplement, not replace, the generic
/// proof-body checks; CV owns the separate seed.
#[test]
fn closed_constructor_cases_accept_and_controls_refuse() {
    let mut env = loaded_package();
    env.elaborate_file(CLOSED_PREFIX)
        .expect("both premises, constructor laws, decisions and values accept");
    for (name, negative) in [
        ("reject_outer_constructor", FALSE_OUTER),
        ("reject_wrong_constructor_result", WRONG_LAW_RESULT),
        ("reject_false_second_premise", FALSE_PREMISE),
    ] {
        let mut isolated = loaded_package();
        isolated
            .elaborate_file(CLOSED_PREFIX)
            .expect("same accepted controls load before isolated refusal");
        refuses_at_own_theorem(&mut isolated, name, negative);
    }
}
