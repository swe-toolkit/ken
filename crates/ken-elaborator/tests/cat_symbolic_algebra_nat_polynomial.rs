//! Checked natural polynomials: spec/50-stdlib/66-symbolic-algebra.md §1.
//! Closed fixtures and refusal controls come from
//! conformance/stdlib/symbolic-algebra/seed-nat-polynomial.md.
//! Promise class: durable invariant; numeric and structural fixture values
//! are fixed by that conformance seed, not a live catalog census.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const POLYNOMIAL: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn polynomial_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], POLYNOMIAL)
        .expect("NatPolynomial package roots-loads");
    env
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §1.3 and
/// conformance/stdlib/symbolic-algebra/seed-nat-polynomial.md, the five
/// generic laws rather than their closed instances.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the public declarations roots-load, all five generic theorem
/// clients elaborate at arbitrary expressions, lists and total environments,
/// and the exact set of trusted GlobalIds does not grow from the providers.
/// CLAIMED: generic laws have checked proof bodies without local trust.
/// THE GAP: independently fixed seed outputs below test the operations'
/// concrete results; a closed client alone cannot prove a universal law.
#[test]
fn public_carriers_operations_and_five_generic_proofs_add_no_trust() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.Derived",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], POLYNOMIAL)
        .expect("polynomial proofs kernel-check from published providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "no new trusted GlobalId from NatPolynomial");

    for law in [
        "normalize_sound",
        "normalize_idempotent",
        "monomials_sound",
        "reify_sound",
        "monomials_reify",
    ] {
        let id = env.globals[&format!("{POLYNOMIAL}.{law}")];
        assert!(owned.contains(&id), "{law} must be package-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} has a checked body"
        );
    }
    for operation in [
        "eval",
        "eval_monomial",
        "eval_monomials",
        "monomials",
        "reify",
        "normalize",
    ] {
        let id = env.globals[&format!("{POLYNOMIAL}.{operation}")];
        assert!(owned.contains(&id), "{operation} must be package-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{operation} is checked"
        );
    }
    for carrier in ["Expr", "Monomial"] {
        assert!(owned.contains(&env.globals[&format!("{POLYNOMIAL}.{carrier}")]));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all five independently restated §1.3 theorem types elaborate");
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, Monomial, eval, eval_monomials, monomials, reify, normalize,
   normalize_sound, normalize_idempotent, monomials_sound,
   reify_sound, monomials_reify)

theorem generic_normalize_sound (env : Nat → Nat) (e : Expr) :
  Equal Nat (eval env (normalize e)) (eval env e) =
  normalize_sound env e
theorem generic_normalize_idempotent (e : Expr) :
  Eq Expr (normalize (normalize e)) (normalize e) =
  normalize_idempotent e
theorem generic_monomials_sound (env : Nat → Nat) (e : Expr) :
  Equal Nat (eval_monomials env (monomials e)) (eval env e) =
  monomials_sound env e
theorem generic_reify_sound (env : Nat → Nat) (ms : List Monomial) :
  Equal Nat (eval env (reify ms)) (eval_monomials env ms) =
  reify_sound env ms
theorem generic_monomials_reify (ms : List Monomial) :
  Eq (List Monomial) (monomials (reify ms)) ms =
  monomials_reify ms
"#;

// Exact positive conformance program, including all nineteen independently
// constructed behavioral assertions and the five closed law applications.
const SEED_POSITIVE: &str = r###"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, Monomial, Mono,
   eval, eval_monomial, eval_monomials, monomials, reify, normalize,
   normalize_sound, normalize_idempotent, monomials_sound,
   reify_sound, monomials_reify)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const six : Nat = Suc (Suc four)
const fifteen : Nat =
  Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc
    (Suc (Suc (Suc (Suc (Suc Zero))))))))))))))

fn env1 (v : Nat) : Nat =
  match v { Zero ↦ two; Suc w ↦ three }

const e1 : Expr =
  EMul (EAdd (EVar Zero) (EConst one))
       (EAdd (EVar one) (EConst two))

const m01 : Monomial = Mono Zero (Cons Nat Zero (Cons Nat one (Nil Nat)))
const m02 : Monomial = Mono one (Cons Nat Zero (Nil Nat))
const m1 : Monomial = Mono Zero (Cons Nat one (Nil Nat))
const m2 : Monomial = Mono one (Nil Nat)
const expected_ms : List Monomial =
  Cons Monomial m01 (Cons Monomial m02
    (Cons Monomial m1 (Cons Monomial m2 (Nil Monomial))))
const expected_expr : Expr =
  EAdd (EMul (EVar Zero) (EMul (EVar one) (EConst one)))
    (EAdd (EMul (EVar Zero) (EConst two))
      (EAdd (EMul (EVar one) (EConst one))
        (EAdd (EConst two) (EConst Zero))))

theorem eval_oracle : Equal Nat (eval env1 e1) fifteen = Proved

theorem eval_monomial_oracle : Equal Nat (eval_monomial env1 m01) six = Proved

theorem eval_monomial_coefficient_oracle :
  Equal Nat (eval_monomial env1 m02) four = Proved

theorem eval_monomials_empty_oracle :
  Equal Nat (eval_monomials env1 (Nil Monomial)) Zero = Proved

theorem eval_monomials_oracle :
  Equal Nat (eval_monomials env1 expected_ms) fifteen = Proved

theorem monomials_oracle :
  Eq (List Monomial) (monomials e1) expected_ms = Refl

theorem monomials_zero_oracle :
  Eq (List Monomial) (monomials (EConst Zero)) (Nil Monomial) = Proved

theorem monomials_var_oracle :
  Eq (List Monomial) (monomials (EVar one))
    (Cons Monomial m1 (Nil Monomial)) = Refl

theorem reify_oracle : Eq Expr (reify expected_ms) expected_expr = Refl

theorem reify_empty_oracle : Eq Expr (reify (Nil Monomial)) (EConst Zero) = Proved

theorem normalize_oracle : Eq Expr (normalize e1) expected_expr = Refl

theorem normalize_eval_oracle :
  Equal Nat (eval env1 (normalize e1)) fifteen = Proved

theorem idempotent_oracle :
  Eq Expr (normalize (normalize e1)) (normalize e1) = Refl

theorem normalize_sound_instance :
  Equal Nat (eval env1 (normalize e1)) (eval env1 e1) =
  normalize_sound env1 e1

theorem normalize_idempotent_instance :
  Eq Expr (normalize (normalize e1)) (normalize e1) =
  normalize_idempotent e1

theorem monomials_sound_instance :
  Equal Nat (eval_monomials env1 (monomials e1)) (eval env1 e1) =
  monomials_sound env1 e1

theorem reify_sound_instance :
  Equal Nat (eval env1 (reify expected_ms))
    (eval_monomials env1 expected_ms) =
  reify_sound env1 expected_ms

theorem monomials_reify_instance :
  Eq (List Monomial) (monomials (reify expected_ms)) expected_ms =
  monomials_reify expected_ms

theorem semantic_pair :
  Equal Nat
    (eval env1 (EAdd (EVar Zero) (EVar Zero)))
    (eval env1 (EMul (EConst two) (EVar Zero))) = Proved
"###;

/// SOURCE: conformance/stdlib/symbolic-algebra/seed-nat-polynomial.md,
/// all nineteen positive assertions from spec §1.1–1.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the full seed client elaborates against the published module,
/// with every assertion belonging to the real checked fixture environment.
/// CLAIMED: each fixed output matches the explicit numeric or constructor
/// expectation, including ordered, unmerged monomials and a trailing zero.
/// THE GAP: finite seed cases do not replace the five generic proof bodies.
#[test]
fn nineteen_seed_assertions_check_from_client_import() {
    let mut env = polynomial_env();
    env.elaborate_file(SEED_POSITIVE)
        .expect("all nineteen positive seed assertions kernel-check");
    for name in [
        "eval_oracle",
        "eval_monomial_oracle",
        "eval_monomial_coefficient_oracle",
        "eval_monomials_empty_oracle",
        "eval_monomials_oracle",
        "monomials_oracle",
        "monomials_zero_oracle",
        "monomials_var_oracle",
        "reify_oracle",
        "reify_empty_oracle",
        "normalize_oracle",
        "normalize_eval_oracle",
        "idempotent_oracle",
        "normalize_sound_instance",
        "normalize_idempotent_instance",
        "monomials_sound_instance",
        "reify_sound_instance",
        "monomials_reify_instance",
        "semantic_pair",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked seed assertion"
        );
    }
}

// Each negative is applied only after the identical positive fixture loaded.
// This avoids weakening the positive import and isolates one false theorem.
const BAD_DROP: &str = r###"
const missing_m1 : List Monomial =
  Cons Monomial m01 (Cons Monomial m02
    (Cons Monomial m2 (Nil Monomial)))
theorem drop_monomial_rejected :
  Eq (List Monomial) (monomials e1) missing_m1 = Refl
"###;
const BAD_CANONICAL: &str = r###"
theorem canonical_converse_rejected :
  Eq (List Monomial)
    (monomials (EAdd (EVar Zero) (EVar Zero)))
    (monomials (EMul (EConst two) (EVar Zero))) = Refl
"###;

fn rejects_false_refl(env: &mut ElabEnv, bad: &str, label: &str) {
    match env.elaborate_file(bad) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => assert_eq!(
            bad.get(span.start..span.end),
            Some("Refl"),
            "{label}: refusal must point to the false proof, not unrelated source",
        ),
        other => panic!("{label}: expected Refl-span kernel TypeMismatch, got {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/symbolic-algebra/seed-nat-polynomial.md,
/// reject-dropped-x1-term and reject-canonical-converse; spec §1.2–1.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: with the same positive client loaded first, dropping the third
/// monomial or asserting structural equality for two semantic equivalents
/// gets a kernel TypeMismatch at the false `Refl` proof span.
/// CLAIMED: the distributed list retains the missing term, and semantic
/// equality does not collapse distinct ordered monomial lists.
/// THE GAP: this guards these two counterexamples only, not arbitrary
/// transformations; generic proof obligations live in the package.
#[test]
fn dropped_monomial_and_false_canonical_converse_are_refused() {
    for (bad, label) in [
        (BAD_DROP, "dropped third monomial"),
        (BAD_CANONICAL, "false canonical converse"),
    ] {
        let mut env = polynomial_env();
        env.elaborate_file(SEED_POSITIVE)
            .expect("positive fixture must check before the negative");
        rejects_false_refl(&mut env, bad, label);
    }
}
