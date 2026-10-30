//! Vector scaling over indexed natural vectors: linear algebra §9.
//! Promise class: durable invariant. Open clients check all four public laws;
//! closed seed computations and the isolated dropped-scale refusal check AC-2.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::Decl;

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/linear-algebra/seed-vector-scale-action.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix root-loads with four vector-scale laws");
    env
}

// Select whole Ken programs by a declaration identity rather than by a fence
// position or the number of unrelated source fences in the seed.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("vector-scaling seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("vector-scaling source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} identifies one Ken client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed theorem {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, vector_scale_distrib,
   vector_scale_factor_distrib, vector_scale_compose, vector_scale_one)

theorem generic_sum (n : Nat) (x : Nat) (u : Vec Nat n) (v : Vec Nat n)
  : Equal (Vec Nat n) (vector_scale n x (vector_add n u v))
      (vector_add n (vector_scale n x u) (vector_scale n x v)) =
  vector_scale_distrib n x u v

theorem generic_factor_sum (n : Nat) (x : Nat) (y : Nat) (u : Vec Nat n)
  : Equal (Vec Nat n) (vector_scale n (add x y) u)
      (vector_add n (vector_scale n x u) (vector_scale n y u)) =
  vector_scale_factor_distrib n x y u

theorem generic_composition (n : Nat) (x : Nat) (y : Nat) (u : Vec Nat n)
  : Equal (Vec Nat n) (vector_scale n (mul x y) u)
      (vector_scale n x (vector_scale n y u)) =
  vector_scale_compose n x y u

theorem generic_one (n : Nat) (u : Vec Nat n)
  : Equal (Vec Nat n) (vector_scale n (Suc Zero) u) u =
  vector_scale_one n u
"#;

/// MEASURED: NatMatrix and its providers have identical trusted GlobalId sets;
/// four module-owned laws have Transparent bodies, all four open generic
/// clients check, and the cons-equality helper refuses selective import.
/// CLAIMED: the new vector-scaling laws have the public §9 types with no
/// added trusted declarations. GAP: this does not make inherited trust empty.
#[test]
fn generic_vector_scale_laws_are_owned_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Conjunction",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Vector.Vector",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("four scaling laws kernel-check from roots");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§9 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "no new trusted GlobalId identities");
    for law in [
        "vector_scale_distrib",
        "vector_scale_factor_distrib",
        "vector_scale_compose",
        "vector_scale_one",
    ] {
        let id = env.globals[&format!("{MATRIX}.{law}")];
        assert!(owned.contains(&id), "{law} belongs to NatMatrix");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all four independent open types check through public imports");
    match loaded_matrix().elaborate_file(&format!("import {MATRIX} (vector_cons_eq)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{MATRIX}.vector_cons_eq"));
        }
        other => panic!("cons-equality bridge must remain private: {other:?}"),
    }
}

/// MEASURED: the full accepted §9 seed applies four laws and checks eight
/// closed sides plus the [3,6] dropped-scale RHS and same-scope true control.
/// Only the isolated false [5,10] = [3,6] Refl refuses after a fresh accepted
/// prefix. CLAIMED: the fixed AC-2 client distinguishes the second factor's
/// contribution. GAP: finite values alone do not prove the generic laws.
#[test]
fn complete_seed_accepts_and_dropped_scale_refuses() {
    let accepted = seed_case("vector_scale_distrib_at_two");
    let refused = seed_case("seed_neg");
    let mut env = loaded_matrix();
    env.elaborate_file(accepted)
        .expect("complete accepted vector-scale seed checks");
    for name in [
        "vector_scale_distrib_at_two",
        "vector_scale_factor_distrib_at_two",
        "vector_scale_compose_at_two",
        "vector_scale_one_at_two",
        "vector_scale_distrib_left_value",
        "vector_scale_distrib_right_value",
        "vector_scale_factor_distrib_left_value",
        "vector_scale_factor_distrib_right_value",
        "vector_scale_compose_left_value",
        "vector_scale_compose_right_value",
        "vector_scale_one_left_value",
        "vector_scale_one_right_value",
        "dropped_scale_false_rhs_value",
        "dropped_scale_same_scope_true_control",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let parsed = parse_decls(refused).expect("isolated dropped-scale claim parses");
    let [SourceDecl::TheoremDecl { name, span, .. }] = parsed.as_slice() else {
        panic!("isolated claim is one theorem");
    };
    assert_eq!(name, "seed_neg");
    let mut rejecting = loaded_matrix();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh accepted prefix checks before refusal");
    let error = rejecting
        .elaborate_file(refused)
        .expect_err("the sole false dropped-scale theorem must be refused");
    match error {
        ElabError::TypeMismatch {
            span: failure,
            reason,
        } if failure.start >= span.start
            && failure.end <= span.end
            && reason.contains("Refl expects an `Eq`-shaped goal") => {}
        other => panic!("false dropped-scale claim must fail at its Refl: {other:?}"),
    }
}
