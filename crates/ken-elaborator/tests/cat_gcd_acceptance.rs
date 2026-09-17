//! CAT-GCD Euclidean-gcd acceptance and catalog-reuse controls.
//!
//! Public names are compatibility vectors. Computation, law artifacts,
//! provider identity, and the zero-trust delta are durable semantic invariants.
//! The controls use the real roots loader and kernel artifacts; repository text
//! and declaration allocation order are not oracles.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{EvalStore, EvalVal, eval};
use ken_kernel::{Decl, GlobalId, Term};

const GCD: &str = "Algorithm.Numeric.Gcd";
const ARITHMETIC: &str = "Data.Numeric.Nat.Arithmetic";
const ORDER: &str = "Data.Numeric.Nat.Order";
const LAWFUL: &str = "Core.Classes.LawfulClasses";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn base_env() -> ElabEnv {
    ElabEnv::new().expect("prelude bootstrap")
}

fn loaded_env() -> ElabEnv {
    let mut env = base_env();
    env.elaborate_module_from_roots(&[catalog_root()], GCD)
        .expect("Algorithm.Numeric.Gcd must elaborate through the real roots loader");
    env
}

fn global(env: &ElabEnv, module: &str, name: &str) -> GlobalId {
    env.globals[&format!("{module}.{name}")]
}

fn transparent(env: &ElabEnv, id: GlobalId) -> (&Term, &Term) {
    match env.env.lookup(id) {
        Some(Decl::Transparent { ty, body, .. }) => (ty, body),
        other => panic!("expected transparent artifact {id:?}, got {other:?}"),
    }
}

fn term_mentions(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| term_mentions(child, target)),
    }
}

fn nat_term(env: &ElabEnv, n: u32) -> Term {
    let mut result = Term::constructor(env.prelude_env.zero_id, vec![]);
    for _ in 0..n {
        result = Term::app(Term::constructor(env.prelude_env.suc_id, vec![]), result);
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

fn evaluate_gcd(env: &ElabEnv, gcd: GlobalId, a: u32, b: u32) -> u64 {
    let term = Term::app(
        Term::app(Term::const_(gcd, vec![]), nat_term(env, a)),
        nat_term(env, b),
    );
    nat_count(env, &eval(&[], &term, &env.env, &mut EvalStore::new()))
}

#[test]
fn roots_loader_registers_gcd_artifacts_with_exact_catalog_provider_identities() {
    let env = loaded_env();
    for name in [
        "Divides",
        "BoolView",
        "GcdSpec",
        "gcd_fuel",
        "gcd",
        "gcd_fuel_spec",
        "gcd_spec",
        "gcd_divides_left",
        "gcd_divides_right",
        "divides_gcd",
    ] {
        assert!(
            env.globals.contains_key(&format!("{GCD}.{name}")),
            "{name} must remain a real Gcd kernel artifact"
        );
    }

    for duplicate in ["add", "mul", "leq_nat", "sub"] {
        assert!(
            !env.globals.contains_key(&format!("{GCD}.{duplicate}")),
            "Gcd must not mint a local replacement for catalog operation {duplicate}"
        );
    }

    let add = global(&env, ARITHMETIC, "add");
    let mul = global(&env, ARITHMETIC, "mul");
    let leq = global(&env, LAWFUL, "leq_nat");
    let sub = global(&env, ORDER, "sub");
    assert!(
        !env.globals.contains_key(&format!("{ORDER}.leq_nat")),
        "the Order facade must carry the LawfulClasses identity without an alias"
    );
    for provider in [add, mul, leq, sub] {
        assert!(env.env.transparent_body(provider).is_some());
    }

    let (_, gcd_body) = transparent(&env, global(&env, GCD, "gcd"));
    assert!(term_mentions(gcd_body, add));
    let (_, fuel_body) = transparent(&env, global(&env, GCD, "gcd_fuel"));
    assert!(term_mentions(fuel_body, leq));
    assert!(term_mentions(fuel_body, sub));
    let (_, mul_add_body) = transparent(&env, global(&env, GCD, "mul_add"));
    assert!(term_mentions(mul_add_body, add));
    assert!(term_mentions(mul_add_body, mul));

    let constructor = global(&env, GCD, "MkDivides");
    let (family, index) = env
        .env
        .constructor(constructor)
        .expect("MkDivides must remain a checked constructor");
    assert!(term_mentions(&family.constructors[index].type_, mul));
}

#[test]
fn entry_adds_no_trust_beyond_the_preexisting_provider_closure() {
    let mut env = base_env();
    env.elaborate_module_from_roots(&[catalog_root()], ARITHMETIC)
        .expect("Arithmetic provider must elaborate");
    env.elaborate_module_from_roots(&[catalog_root()], ORDER)
        .expect("Order facade and provider closure must elaborate");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], GCD)
        .expect("Gcd must elaborate after its exact provider closure");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "Euclidean gcd itself must add zero trusted declarations"
    );
    for name in [
        "add",
        "mul",
        "leq_nat",
        "sub",
        "gcd",
        "gcd_fuel",
        "divides_gcd",
    ] {
        let id = if matches!(name, "add" | "mul") {
            global(&env, ARITHMETIC, name)
        } else if name == "leq_nat" {
            global(&env, LAWFUL, name)
        } else if name == "sub" {
            global(&env, ORDER, name)
        } else {
            global(&env, GCD, name)
        };
        assert!(!after.contains(&id), "{name} must remain outside the TCB");
    }
}

#[test]
fn gcd_vectors_compute_and_divisibility_law_artifacts_remain_checked() {
    let env = loaded_env();
    let divides = global(&env, GCD, "Divides");
    let gcd = global(&env, GCD, "gcd");
    for law in ["gcd_divides_left", "gcd_divides_right", "divides_gcd"] {
        let (ty, _) = transparent(&env, global(&env, GCD, law));
        assert!(
            term_mentions(ty, divides),
            "{law} must retain Divides in its type"
        );
        assert!(term_mentions(ty, gcd), "{law} must retain gcd in its type");
    }

    for (name, a, b, expected) in [
        ("zero_zero", 0, 0, 0),
        ("zero_six", 0, 6, 6),
        ("six_zero", 6, 0, 6),
        ("six_four", 6, 4, 2),
        ("four_six", 4, 6, 2),
        ("twelve_eight", 12, 8, 4),
        ("five_three", 5, 3, 1),
    ] {
        assert_eq!(evaluate_gcd(&env, gcd, a, b), expected, "gcd vector {name}");
    }
}
