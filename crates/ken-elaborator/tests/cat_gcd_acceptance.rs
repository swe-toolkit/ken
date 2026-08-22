//! CAT-GCD Euclidean-gcd acceptance.
//!
//! Public names are compatibility vectors. Computation, law instantiation,
//! and the zero-trust delta are durable semantic invariants.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

const GCD_KEN_MD: &str = include_str!("../../../catalog/packages/Algorithm/Numeric/Gcd.ken.md");

fn base_env() -> ElabEnv {
    ElabEnv::empty().expect("prelude bootstrap")
}

fn loaded_env() -> ElabEnv {
    let mut env = base_env();
    env.elaborate_ken_md_file(GCD_KEN_MD)
        .expect("Algorithm/Numeric/Gcd.ken.md must elaborate");
    env
}

fn nat(n: u32) -> String {
    let mut result = "Zero".to_string();
    for _ in 0..n {
        result = format!("Suc ({result})");
    }
    result
}

fn nat_count(env: &ElabEnv, value: &EvalVal) -> u64 {
    match value {
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.zero_id && args.is_empty() => 0,
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.suc_id && args.len() == 1 => {
            1 + nat_count(env, &args[0])
        }
        other => panic!("expected a Nat constructor chain, got {other:?}"),
    }
}

fn evaluate_nat(env: &ElabEnv, id: ken_kernel::GlobalId) -> u64 {
    let body = match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("gcd vector must be transparent, got {other:?}"),
    };
    nat_count(env, &eval(&[], body, &env.env, &mut EvalStore::new()))
}

#[test]
fn entry_elaborates_and_registers_algorithm_and_laws() {
    let env = loaded_env();
    for name in [
        "gcd_fuel",
        "gcd",
        "Divides",
        "GcdSpec",
        "gcd_fuel_spec",
        "gcd_spec",
        "gcd_divides_left",
        "gcd_divides_right",
        "divides_gcd",
    ] {
        assert!(
            env.globals.contains_key(name),
            "`{name}` must be a real kernel-checked global"
        );
    }
}

#[test]
fn entry_adds_no_trusted_declarations() {
    let mut env = base_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_ken_md_file(GCD_KEN_MD)
        .expect("Algorithm/Numeric/Gcd.ken.md must elaborate");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "Euclidean gcd must add zero trusted declarations"
    );
}

#[test]
fn gcd_vectors_compute_and_all_laws_instantiate() {
    let mut env = loaded_env();
    env.elaborate_file(
        "fn cat_gcd_ac1_left (a : Nat) (b : Nat) : Divides (gcd a b) a = \
           gcd_divides_left a b\n\
         fn cat_gcd_ac1_right (a : Nat) (b : Nat) : Divides (gcd a b) b = \
           gcd_divides_right a b\n\
         fn cat_gcd_ac2 \
             (d : Nat) (a : Nat) (b : Nat) \
             (da : Divides d a) (db : Divides d b) \
           : Divides d (gcd a b) = \
           divides_gcd d a b da db",
    )
    .expect("both divisibility directions and greatestness must instantiate generically");

    let two = nat(2);
    let four = nat(4);
    let six = nat(6);
    env.elaborate_decl(&format!(
        "const cat_gcd_two_divides_result : Divides ({two}) (gcd ({four}) ({six})) = \
         divides_gcd ({two}) ({four}) ({six}) \
           (MkDivides ({two}) ({four}) ({two}) Proved) \
           (MkDivides ({two}) ({six}) ({nat_three}) Proved)",
        nat_three = nat(3),
    ))
    .expect("greatestness must accept concrete quotient witnesses");

    for (name, a, b, expected) in [
        ("zero_zero", 0, 0, 0),
        ("zero_six", 0, 6, 6),
        ("six_zero", 6, 0, 6),
        ("six_four", 6, 4, 2),
        ("four_six", 4, 6, 2),
        ("twelve_eight", 12, 8, 4),
        ("five_three", 5, 3, 1),
    ] {
        let id = env
            .elaborate_decl(&format!(
                "const cat_gcd_{name} : Nat = gcd ({}) ({})",
                nat(a),
                nat(b)
            ))
            .unwrap_or_else(|error| panic!("gcd vector {name} must elaborate: {error}"));
        assert_eq!(evaluate_nat(&env, id), expected, "gcd vector {name}");
    }
}
