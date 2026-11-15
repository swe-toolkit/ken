//! Least-k cost-order invariance and Derived comparator congruence.
//! Source: spec/50-stdlib/72-optimization.md §4.4; the conformance seed is
//! assembled separately from this package test.
//! Promise class: durable invariant. Both generic clients retain their open
//! comparison premise rather than testing only one closed cost transform.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::Decl;

const ARGMIN: &str = "Algorithm.Optimization.Argmin";
const DERIVED: &str = "Data.Collections.Derived";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin (least_k, least_k_cost_order_invariant)
import Core.Function.Combinators (comp)
import Data.Collections.Derived (sort)
import Data.Numeric.Nat.Order (lt_nat)

theorem least_k_generic (a : Type) (cost : a → Nat) (f : Nat → Nat)
    (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
    (k : Nat) (xs : List a)
  : Equal (List a) (least_k a (comp a Nat Nat f cost) k xs) (least_k a cost k xs) =
  least_k_cost_order_invariant a cost f same_order k xs

theorem sort_comparator_generic (a : Type) (le1 : a → a → Bool) (le2 : a → a → Bool)
    (same_le : (x : a) → (y : a) → Equal Bool (le1 x y) (le2 x y)) (xs : List a)
  : Equal (List a) (sort a le1 xs) (sort a le2 xs) =
  (proof comparator_cong for sort) a le1 le2 same_le xs
"#;

/// MEASURED: root-loaded providers establish the exact trusted GlobalId set
/// before Derived and Argmin load; neither adds a trusted identity. All three
/// laws are Transparent declarations returned by their owning
/// module's roots load. The selective-import client applies the public
/// least-k law and sort congruence at arbitrary costs, comparators and lists.
/// CLAIMED: both public generic types are usable without hidden premises and
/// the three checked declarations add no local trust.
/// THE GAP: provider trust is inherited, not erased; the open client checks
/// signatures while the packages' kernel-checked bodies discharge the proofs.
#[test]
fn least_k_order_and_derived_congruence_are_public_checked_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Function.Combinators",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Collections.NonEmpty",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived and both comparator congruence proofs root-load");
    let argmin_owned = env
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("least-k cost-order invariant root-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "least-k cost-order trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(
        after, before,
        "Derived and Argmin add no trusted GlobalId identity"
    );

    for (qualified, owned) in [
        (
            "Algorithm.Optimization.Argmin.least_k_cost_order_invariant",
            &argmin_owned,
        ),
        (
            "Data.Collections.Derived.insert::comparator_cong",
            &derived_owned,
        ),
        (
            "Data.Collections.Derived.sort::comparator_cong",
            &derived_owned,
        ),
    ] {
        let id = env.globals[qualified];
        assert!(owned.contains(&id), "owner module must declare {qualified}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{qualified} must have a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("both open public laws are selectively importable and applicable");
}
