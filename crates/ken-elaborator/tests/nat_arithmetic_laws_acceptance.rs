//! Canonical Nat arithmetic and free-law acceptance.

use std::path::PathBuf;

use ken_elaborator::ElabEnv;

const NAT_ARITH_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md");
const NAT_ARITH_MODULE: &str = "Data.Numeric.Nat.Arithmetic";

const ZERO_SUM_GENERIC_CLIENT: &str = r#"
import Data.Numeric.Nat.Arithmetic (add)

theorem client_eq_zero_l (a : Nat) (b : Nat)
    : Equal Nat (add a b) Zero → Equal Nat a Zero =
  (proof eq_zero_l for add) a b

theorem client_eq_zero_r (a : Nat) (b : Nat)
    : Equal Nat (add a b) Zero → Equal Nat b Zero =
  (proof eq_zero_r for add) a b
"#;

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_arithmetic(env: &mut ElabEnv) {
    env.elaborate_module_from_roots(&[catalog_root()], NAT_ARITH_MODULE)
        .expect("Data/Numeric/Nat/Arithmetic.ken.md must roots-load and kernel-check");
}

#[test]
fn all_laws_kernel_check_before_literate_examples() {
    let mut env = ElabEnv::new().expect("base environment");
    load_arithmetic(&mut env);
}

#[test]
fn entry_elaborates_and_registers_the_free_laws() {
    let mut env = ElabEnv::new().expect("base environment");
    load_arithmetic(&mut env);
    for name in [
        "Data.Numeric.Nat.Arithmetic.add",
        "Data.Numeric.Nat.Arithmetic.mul",
        "Data.Numeric.Nat.Arithmetic.add::assoc",
        "Data.Numeric.Nat.Arithmetic.add::comm",
        "Data.Numeric.Nat.Arithmetic.add::eq_zero_l",
        "Data.Numeric.Nat.Arithmetic.add::eq_zero_r",
        "Data.Numeric.Nat.Arithmetic.mul::assoc",
        "Data.Numeric.Nat.Arithmetic.mul::comm",
        "Data.Numeric.Nat.Arithmetic.mul_add_distrib_l",
        "Data.Numeric.Nat.Arithmetic.mul_add_distrib_r",
        "Data.Numeric.Nat.Arithmetic.add_cancel_r",
    ] {
        assert!(
            env.globals.contains_key(name),
            "{name} must be a checked global"
        );
    }
}

/// MEASURED: selective import of `add` checks both attached zero-sum proofs
/// at their open curried types, including the zero premise.
/// CLAIMED: consumers can derive either summand's zero equality without
/// assuming it, using the public Arithmetic identities.
/// THE GAP: the generic clients test the proof types, not an inhabited
/// premise; the independent seed owns closed accepted and refused cases.
#[test]
fn attached_zero_sum_proofs_check_at_generic_curried_types() {
    let mut env = ElabEnv::new().expect("base environment");
    load_arithmetic(&mut env);
    env.elaborate_file(ZERO_SUM_GENERIC_CLIENT)
        .expect("both attached proofs apply at exact generic types");
}

/// Promise class: durable invariant. A selective client consumes the public
/// additive-cancellation theorem at its unchanged generic Nat type. Its checked
/// body belongs to Arithmetic; the package's trust check covers its closure.
#[test]
fn additive_cancellation_is_public_at_its_generic_type() {
    let mut env = ElabEnv::new().expect("base environment");
    load_arithmetic(&mut env);
    env.elaborate_file(
        "import Data.Numeric.Nat.Arithmetic (add, add_cancel_r)\n\
         theorem cancellation_client (a : Nat) (b : Nat) (c : Nat)\n\
           : Equal Nat (add a c) (add b c) → Equal Nat a b =\n\
             add_cancel_r a b c",
    )
    .expect("promoted Nat additive cancellation has its unchanged generic type");
}

#[test]
fn canonical_operations_compute_on_concrete_naturals() {
    let mut env = ElabEnv::new().expect("base environment");
    load_arithmetic(&mut env);
    env.elaborate_file("import Data.Numeric.Nat.Arithmetic")
        .expect("client imports the qualified arithmetic module");
    env.elaborate_decl(
        "theorem add_two_three_check : Equal Nat (Data.Numeric.Nat.Arithmetic.add (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))) (Suc (Suc (Suc (Suc (Suc Zero))))) = Proved",
    )
    .expect("add 2 3 must compute to 5");
    env.elaborate_decl(
        "theorem mul_two_three_check : Equal Nat (Data.Numeric.Nat.Arithmetic.mul (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))) (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))) = Proved",
    )
    .expect("mul 2 3 must compute to 6");
}

#[test]
fn tangled_source_stays_free_of_numeric_classes_and_trusted_declarations() {
    let extracted = ken_elaborator::literate::extract_ken_md(NAT_ARITH_KEN_MD)
        .expect("Arithmetic.ken.md must extract");
    for forbidden in ["Axiom", "postulate", "class", "instance", "sorry"] {
        assert!(
            !extracted.source.contains(forbidden),
            "NatArith checked source must not contain {forbidden}",
        );
    }
}

#[test]
fn trusted_base_delta_is_empty_across_the_entry() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: std::collections::BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    load_arithmetic(&mut env);
    let after: std::collections::BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "NatArith must add no trusted-base entries");
}
