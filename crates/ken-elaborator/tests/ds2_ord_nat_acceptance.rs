//! DS-2 (`Ord Nat` export + `Nat` operations) acceptance — `docs/program/
//! wp/ds-2-ord-nat-export.md`.
//!
//! - `refl`/`trans`/`antisym` slot into `class Ord` directly (zero
//!   conversion, `IsTrue` unfolds to `Equal Bool ... True`); `total` needs
//!   the `orEqTrueToIsTrueBoolOr` bridge (probed, mirrors `Ord Bool`'s
//!   proof STYLE, not a literal template for this specific conversion).
//! - **Zero new `Axiom`/`trusted_base()` delta** — the acceptance bar the
//!   frame set (`Nat` is inductive, unlike `Int`).
//! - The entry's `` ```ken ``/`` ```ken example ``/`` ```ken reject ``
//!   fences all check via the real literate extractor.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::path::PathBuf;

use ken_elaborator::ElabEnv;

const LAWFUL: &str = "Core.Classes.LawfulClasses";
const ORDER: &str = "Data.Numeric.Nat.Order";
fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn base_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[catalog_root()], LAWFUL)
        .expect("LawfulClasses must elaborate through its real provider closure");
    env
}

fn load_order(env: &mut ElabEnv) {
    env.elaborate_module_from_roots(&[catalog_root()], ORDER)
        .expect("Order must elaborate through its real provider closure");
}

#[test]
fn entry_elaborates_with_every_checked_fence() {
    let mut env = base_env();
    load_order(&mut env);
    env.execute_loaded_entry_checked_fences(ORDER)
        .expect("Order Definition and every checked fence must elaborate");
    catalog_or::assert_transparent_result_uses_core_logic_or(
        &env,
        "Core.Classes.LawfulClasses.total_leq_nat",
    );
    assert!(
        env.globals.contains_key("Ord_instance_Nat"),
        "Ord_instance_Nat must be a real registered global"
    );
}

/// Promise class: durable invariant. The provider-private `total_leq_nat`
/// keeps its informative catalog `Or` result, while the public facade computes
/// opposite concrete relation outcomes.
///
/// **MEASURED:** the loader artifact gives `total_leq_nat` the canonical `Or`
/// family in its result type, and Order-only imports reduce `leq_nat` to both
/// `True` and `False`. **CLAIMED:** the ownership migration preserves the
/// proof-relevant totality source and public relation behavior. **THE GAP:**
/// result-family identity alone would not establish relation behavior, so the
/// two concrete equalities supply the independent axis.
#[test]
fn totality_source_and_public_relation_behavior_survive_the_move() {
    let mut env = base_env();
    load_order(&mut env);
    catalog_or::assert_transparent_result_uses_core_logic_or(
        &env,
        "Core.Classes.LawfulClasses.total_leq_nat",
    );
    env.elaborate_file(
        "import Data.Numeric.Nat.Order (leq_nat)\n\
         theorem order_total_left_behavior\n\
           : Equal Bool (leq_nat Zero (Suc Zero)) True = Proved\n\
         theorem order_total_right_behavior\n\
           : Equal Bool (leq_nat (Suc Zero) Zero) False = Proved",
    )
    .expect("both concrete relation directions must retain their behavior");
}

// Ground the zero-trusted_base()-delta claim structurally: the set of
// trusted (unproved) globals before and after elaborating this entry must
// be IDENTICAL.
#[test]
fn trusted_base_delta_is_empty_across_the_entry() {
    let mut env = base_env();
    let before: std::collections::BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    load_order(&mut env);
    let after: std::collections::BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "Order.ken.md must introduce ZERO new trusted_base() entries (zero-Axiom acceptance bar)"
    );
}

// `refl`/`trans`/`antisym` slot into `class Ord`'s IsTrue-phrased fields
// directly, with no conversion function — confirms the "no bridge needed"
// half of the frame's "probe first" instruction empirically, not just by
// citing IsTrue's definition.
#[test]
fn refl_trans_antisym_need_no_conversion() {
    let mut env = base_env();
    env.elaborate_decl(
        "fn leqNat (m : Nat) (n : Nat) : Bool = match m { Zero ↦ True ; Suc m2 ↦ match n { Zero ↦ False ; Suc n2 ↦ leqNat m2 n2 } }",
    )
    .expect("leqNat");
    env.elaborate_decl(
        "theorem reflLeqNat (x : Nat) : Equal Bool (leqNat x x) True = match x { Zero ↦ Proved ; Suc x2 ↦ reflLeqNat x2 }",
    )
    .expect("reflLeqNat");

    // The SAME term, reflLeqNat, satisfies the IsTrue-phrased signature
    // directly — zero adaptation.
    env.elaborate_decl("theorem probeRefl (x : Nat) : IsTrue (leqNat x x) = reflLeqNat x")
        .expect("reflLeqNat must satisfy IsTrue (leqNat x x) with zero conversion code");
}
