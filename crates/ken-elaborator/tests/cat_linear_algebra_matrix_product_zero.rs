//! Zero matrices absorb matrix products: spec/50-stdlib/65-linear-algebra.md §13.
//! Promise class: durable invariant. Open clients consume three checked laws;
//! closed clients separate a zero factor from a nonzero identity factor.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::Decl;

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix and its provider closure root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_product, matrix_zero, transpose,
   matrix_product_zero_left, matrix_product_zero_right, transpose_zero)

theorem left_absorbs (m : Nat) (n : Nat) (p : Nat) (b : Vec (Vec Nat p) n) :
  Equal (Vec (Vec Nat p) m) (matrix_product m n p (matrix_zero m n) b) (matrix_zero m p) =
  matrix_product_zero_left m n p b

theorem right_absorbs (m : Nat) (n : Nat) (p : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat p) m) (matrix_product m n p a (matrix_zero n p)) (matrix_zero m p) =
  matrix_product_zero_right m n p a

theorem transpose_preserves_zero (m : Nat) (n : Nat) :
  Equal (Vec (Vec Nat m) n) (transpose m n (matrix_zero m n)) (matrix_zero n m) =
  transpose_zero m n
"#;

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §13, public law types.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after loading the three existing providers, NatMatrix preserves
/// the entire trusted GlobalId set. All three new public law identities are
/// owned and Transparent, and generic selective-import clients check their
/// exact types with open dimensions. Two new proof helpers refuse import.
/// CLAIMED: both product orientations and the transposition law are reusable
/// without a new assumption or an exported proof scaffold.
/// THE GAP: existing provider trust is inherited; a transparent declaration
/// alone cannot pin its statement, so the open clients use each exact type.
#[test]
fn three_generic_zero_laws_are_public_checked_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Data.Vector.Vector",
        "Data.Numeric.Nat.Arithmetic",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("all three product-zero proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "NatMatrix adds no trusted GlobalId");

    for name in [
        "matrix_product_zero_left",
        "matrix_product_zero_right",
        "transpose_zero",
    ] {
        let id = env.globals[&format!("{MATRIX}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to NatMatrix");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all three public §13 types check at open dimensions");
    for helper in ["map_repeat", "zero_columns_row"] {
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MATRIX}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

// The literal 2×2 factor has nonzero entries; an independent identity
// product checks that the fixture did not collapse to the zero matrix.
const CLOSED_CLIENT: &str = r#"
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_product, matrix_zero, transpose, identity,
   matrix_product_zero_left, matrix_product_zero_right, transpose_zero)

const one : Nat = Suc Zero
const two : Nat = Suc one
fn pair (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
fn rows2 (r0 : Vec Nat two) (r1 : Vec Nat two) : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one r0 (VCons (Vec Nat two) Zero r1 (VNil (Vec Nat two)))
const a : Vec (Vec Nat two) two = rows2 (pair one two) (pair two one)
const empty : Vec (Vec Nat two) Zero = VNil (Vec Nat two)

theorem left_a : Equal (Vec (Vec Nat two) two) (matrix_product two two two (matrix_zero two two) a) (matrix_zero two two) =
  matrix_product_zero_left two two two a
theorem right_a : Equal (Vec (Vec Nat two) two) (matrix_product two two two a (matrix_zero two two)) (matrix_zero two two) =
  matrix_product_zero_right two two two a
theorem left_empty : Equal (Vec (Vec Nat two) Zero) (matrix_product Zero two two (matrix_zero Zero two) a) (matrix_zero Zero two) =
  matrix_product_zero_left Zero two two a
theorem right_empty : Equal (Vec (Vec Nat two) Zero) (matrix_product Zero two two empty (matrix_zero two two)) (matrix_zero Zero two) =
  matrix_product_zero_right Zero two two empty
theorem transpose_a : Equal (Vec (Vec Nat two) two) (transpose two two (matrix_zero two two)) (matrix_zero two two) =
  transpose_zero two two
theorem transpose_empty : Equal (Vec (Vec Nat Zero) two) (transpose Zero two empty) (matrix_zero two Zero) =
  transpose_zero Zero two
theorem identity_product_is_a : Equal (Vec (Vec Nat two) two) (matrix_product two two two a (identity two)) a = Refl
theorem zero_factor_control : Equal (Vec (Vec Nat two) two) (matrix_product two two two a (matrix_zero two two)) (matrix_zero two two) = Refl
"#;

const FALSE_IDENTITY_FACTOR: &str = r#"
theorem identity_factor_is_zero : Equal (Vec (Vec Nat two) two) (matrix_product two two two a (identity two)) (matrix_zero two two) = Refl
"#;

/// SOURCE: spec 65 §13; AC-2's nonzero factor and zero-row examples.
/// PROMISE CLASS: durable invariant.
/// MEASURED: six law applications check at a nonzero 2×2 factor and a
/// zero-row input; a direct identity product equals that nonzero factor.
/// The same-form zero-factor Refl checks, while an isolated identity-factor
/// claim is refused at its own Refl in a fresh roots-loaded context.
/// CLAIMED: the zero matrix absorbs a product on either side, but the
/// identity factor does not turn a nonzero matrix into zero.
/// THE GAP: finite applications and this discriminating value pair do not
/// prove the three universal laws; the generic checked bodies do so above.
#[test]
fn six_closed_law_uses_and_zero_factor_control_separate_false_identity() {
    let mut accepted = loaded_matrix();
    accepted
        .elaborate_file(CLOSED_CLIENT)
        .expect("six law uses, nonzero identity value, and zero-factor control check");
    for name in [
        "left_a",
        "right_a",
        "left_empty",
        "right_empty",
        "transpose_a",
        "transpose_empty",
        "identity_product_is_a",
        "zero_factor_control",
    ] {
        let id = accepted.globals[name];
        assert!(matches!(
            accepted.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }

    let parsed = parse_decls(FALSE_IDENTITY_FACTOR).expect("isolated false theorem parses");
    let [SourceDecl::TheoremDecl {
        name: claim,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("the refused claim must be exactly one theorem");
    };
    assert_eq!(claim, "identity_factor_is_zero");
    let mut rejecting = loaded_matrix();
    rejecting
        .elaborate_file(CLOSED_CLIENT)
        .expect("fresh accepted prefix and Refl control check before refusal");
    match rejecting.elaborate_file(FALSE_IDENTITY_FACTOR) {
        Err(ElabError::TypeMismatch { span, reason })
            if span.start >= claim_span.start
                && span.start < span.end
                && span.end <= claim_span.end
                && reason == "Refl expects an `Eq`-shaped goal" => {}
        other => panic!("nonzero identity product must refuse at its own Refl: {other:?}"),
    }
}
