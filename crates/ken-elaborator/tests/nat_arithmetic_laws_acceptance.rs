//! Canonical Nat arithmetic and free-law acceptance.

use std::path::PathBuf;

use ken_elaborator::ElabEnv;

const NAT_ARITH_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md");
const NAT_ARITH_MODULE: &str = "Data.Numeric.Nat.Arithmetic";

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
        "Data.Numeric.Nat.Arithmetic.mul::assoc",
        "Data.Numeric.Nat.Arithmetic.mul::comm",
        "Data.Numeric.Nat.Arithmetic.mul_add_distrib_l",
        "Data.Numeric.Nat.Arithmetic.mul_add_distrib_r",
    ] {
        assert!(
            env.globals.contains_key(name),
            "{name} must be a checked global"
        );
    }
}

#[test]
fn canonical_operations_compute_on_concrete_naturals() {
    let mut env = ElabEnv::new().expect("base environment");
    load_arithmetic(&mut env);
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
