//! Dimension-safe natural matrices: spec/50-stdlib/65-linear-algebra.md §1.
//! Every closed assertion and the dimension refusal derive from
//! conformance/stdlib/linear-algebra/seed-nat-matrix.md. Promise class:
//! durable invariant (including the seed's explicitly fixed numeric oracles).

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const VECTOR: &str = "Data.Vector.Vector";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn matrix_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix package roots-loads");
    env
}

/// MEASURED: exact owned, transparent law identities, selective imports of
/// all twelve names and generic clients for five specified propositions.
/// CLAIMED: the public module exports checked, reusable dimension-safe laws.
/// THE GAP: independent closed seed oracles separately catch wrong values.
#[test]
fn public_operations_and_generic_laws_preserve_provider_trust_set() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        VECTOR,
        "Data.Numeric.Nat.Arithmetic",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("matrix package checks against root-loaded providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "matrix package adds no trusted GlobalIds");
    let expected = [
        "vector_add",
        "vector_scale",
        "dot",
        "matrix_vector",
        "transpose",
        "matrix_product",
        "identity",
        "dot_comm",
        "dot_distrib",
        "transpose_invol",
        "vector_identity",
        "matrix_identity_right",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();
    let owned_names = env
        .globals
        .iter()
        .filter_map(|(qualified, id)| {
            qualified
                .strip_prefix(&format!("{MATRIX}."))
                .filter(|_| owned.contains(id))
                .map(str::to_owned)
        })
        .collect::<BTreeSet<_>>();
    assert!(
        expected.is_subset(&owned_names),
        "all twelve public names are owned"
    );
    for name in &expected {
        let id = env.globals[&format!("{MATRIX}.{name}")];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    let mut visible = BTreeSet::new();
    for (index, name) in owned_names.iter().enumerate() {
        match env.elaborate_file(&format!("import {MATRIX} ({name} as matrix_probe_{index})")) {
            Ok(_) => {
                visible.insert(name.clone());
            }
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("{MATRIX}.{name}"));
            }
            other => panic!("{name}: unrelated selective-import result: {other:?}"),
        }
    }
    assert_eq!(visible, expected, "no private helpers escape as public API");
    env.elaborate_file(
        r#"
import Data.Vector.Vector (Vec, VNil, VCons, map, zip_with, vec_zero_vnil)
import Data.Numeric.Nat.Arithmetic (add)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, dot, matrix_vector, transpose, matrix_product,
   identity, dot_comm, dot_distrib, transpose_invol, vector_identity,
   matrix_identity_right)
fn client_add (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Vec Nat n =
  vector_add n xs ys
fn client_scale (n : Nat) (f : Nat) (xs : Vec Nat n) : Vec Nat n =
  vector_scale n f xs
fn client_matrix_vector (m : Nat) (n : Nat)
  (rows : Vec (Vec Nat n) m) (v : Vec Nat n) : Vec Nat m =
  matrix_vector m n rows v
fn client_product (m : Nat) (n : Nat) (p : Nat)
  (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) : Vec (Vec Nat p) m =
  matrix_product m n p a b
theorem client_comm (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) :
  Equal Nat (dot n xs ys) (dot n ys xs) = dot_comm n xs ys
theorem client_distrib (n : Nat) (xs : Vec Nat n)
  (ys : Vec Nat n) (zs : Vec Nat n) :
  Equal Nat (dot n (vector_add n xs ys) zs)
    (add (dot n xs zs) (dot n ys zs)) = dot_distrib n xs ys zs
theorem client_transpose (m : Nat) (n : Nat)
  (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (transpose n m (transpose m n a)) a =
  transpose_invol m n a
theorem client_vector_identity (n : Nat) (v : Vec Nat n) :
  Equal (Vec Nat n) (matrix_vector n n (identity n) v) v =
  vector_identity n v
theorem client_matrix_identity (m : Nat) (n : Nat)
  (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_product m n n a (identity n)) a =
  matrix_identity_right m n a
"#,
    )
    .expect("generic clients independently state five law types");
}

// Ten positive, closed oracles from the nine-case conformance seed. The
// expected vectors are constructed independently of the operations tested.
const SEED: &str = r#"
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, dot, matrix_vector,
   transpose, matrix_product, identity)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five

fn v2 (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
fn v3 (x : Nat) (y : Nat) (z : Nat) : Vec Nat three =
  VCons Nat two x (VCons Nat one y (VCons Nat Zero z (VNil Nat)))

const a23 : Vec (Vec Nat three) two =
  VCons (Vec Nat three) one
    (v3 one two Zero)
    (VCons (Vec Nat three) Zero
      (v3 Zero one three) (VNil (Vec Nat three)))
const b32 : Vec (Vec Nat two) three =
  VCons (Vec Nat two) two (v2 two one)
    (VCons (Vec Nat two) one (v2 Zero two)
      (VCons (Vec Nat two) Zero (v2 one Zero)
        (VNil (Vec Nat two))))
const expected22 : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (v2 two five)
    (VCons (Vec Nat two) Zero (v2 three two)
      (VNil (Vec Nat two)))
const expected32 : Vec (Vec Nat two) three =
  VCons (Vec Nat two) two (v2 one Zero)
    (VCons (Vec Nat two) one (v2 two one)
      (VCons (Vec Nat two) Zero (v2 Zero three)
        (VNil (Vec Nat two))))

theorem dot_oracle : Equal Nat
  (dot three (v3 one two Zero) (v3 three one two)) five = Proved
theorem identity_two_oracle : Equal (Vec (Vec Nat two) two) (identity two)
  (VCons (Vec Nat two) one (v2 one Zero)
    (VCons (Vec Nat two) Zero (v2 Zero one) (VNil (Vec Nat two)))) = Refl
theorem vector_add_oracle : Equal (Vec Nat three)
  (vector_add three (v3 one two Zero) (v3 two Zero one))
  (v3 three two one) = Refl
theorem vector_scale_oracle : Equal (Vec Nat three)
  (vector_scale three two (v3 one two Zero))
  (v3 two four Zero) = Refl
theorem vector_scale_empty_oracle : Equal (Vec Nat Zero)
  (vector_scale Zero three (VNil Nat)) (VNil Nat) = Proved
theorem matrix_vector_oracle : Equal (Vec Nat two)
  (matrix_vector two three a23 (v3 two two one))
  (v2 six five) = Refl
theorem product_oracle : Equal (Vec (Vec Nat two) two)
  (matrix_product two three two a23 b32) expected22 = Refl
theorem right_identity_oracle : Equal (Vec (Vec Nat three) two)
  (matrix_product two three three a23 (identity three)) a23 = Refl
theorem transpose_intermediate_oracle : Equal (Vec (Vec Nat two) three)
  (transpose two three a23) expected32 = Refl
theorem transpose_round_trip_oracle : Equal (Vec (Vec Nat three) two)
  (transpose three two (transpose two three a23)) a23 = Refl
"#;

/// MEASURED: the ten closed propositions kernel-check against independent
/// numeral/constructor expectations. CLAIMED: the nine specified seed cases
/// produce the stated results. THE GAP: finite oracles do not prove generic
/// laws; those are checked above at arbitrary indexed lengths.
#[test]
fn nine_seed_cases_check_ten_independent_closed_oracles() {
    let mut env = matrix_env();
    env.elaborate_file(SEED)
        .expect("all ten seed oracles must kernel-check from public imports");
    for name in [
        "dot_oracle",
        "identity_two_oracle",
        "vector_add_oracle",
        "vector_scale_oracle",
        "vector_scale_empty_oracle",
        "matrix_vector_oracle",
        "product_oracle",
        "right_identity_oracle",
        "transpose_intermediate_oracle",
        "transpose_round_trip_oracle",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

/// MEASURED: the exact matching call above checks, while changing only the
/// right input from three rows to two gets a kernel TypeMismatch. CLAIMED:
/// inner dimensions must agree before a product is evaluated. THE GAP: one
/// fixture does not prove other laws; the product's generic type is checked.
#[test]
fn wrong_inner_dimension_rejects_at_the_index_boundary() {
    let mut env = matrix_env();
    env.elaborate_file(SEED)
        .expect("matching inner dimensions must check first");
    match env.elaborate_file(
        "const bad_product : Vec (Vec Nat two) two =
           matrix_product two three two a23 expected22",
    ) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => {}
        other => panic!("a two-row right operand cannot satisfy a three-row index: {other:?}"),
    }
}
