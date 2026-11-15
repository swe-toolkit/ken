//! Matrix-vector scaling: spec/50-stdlib/65-linear-algebra.md §17.
//! Promise class: durable invariant. An open-dimension law client and two
//! independently reduced closed sides are paired with a same-form false value.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError, Expr, Type as SourceType};
use ken_kernel::Decl;

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";

// The shared client contains R-GEN and R-VAL. The refused case starts with a
// fresh environment and only this prefix, never an environment contaminated by
// the right-value or law-at-value declarations.
const SHARED: &str = r#"
import Data.Vector.Vector (Vec, VNil, VCons)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_vector, matrix_scale, vector_scale, matrix_vector_scale)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const seven : Nat = add three four
const nine : Nat = mul three three
const twenty_one : Nat = mul three seven

const a : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (VCons Nat one one (VCons Nat zero two (VNil Nat)))
    (VCons (Vec Nat two) zero (VCons Nat one three (VCons Nat zero four (VNil Nat)))
      (VNil (Vec Nat two)))
const x : Vec Nat two = VCons Nat one one (VCons Nat zero one (VNil Nat))
const scaled_result : Vec Nat two = VCons Nat one nine (VCons Nat zero twenty_one (VNil Nat))
const unscaled_result : Vec Nat two = VCons Nat one three (VCons Nat zero seven (VNil Nat))

theorem gen
      (m : Nat) (n : Nat) (factor : Nat) (b : Vec (Vec Nat n) m) (y : Vec Nat n)
    : Equal
        (Vec Nat m)
        (matrix_vector m n (matrix_scale m n factor b) y)
        (vector_scale m factor (matrix_vector m n b y)) =
  matrix_vector_scale m n factor b y

theorem scaled_value
    : Equal (Vec Nat two) (matrix_vector two two (matrix_scale two two three a) x) scaled_result =
  Refl
"#;

const RIGHT_AND_LAW: &str = r#"
theorem right_value
    : Equal (Vec Nat two) (vector_scale two three (matrix_vector two two a x)) scaled_result =
  Refl

theorem law_at_value
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_scale two two three a) x)
        (vector_scale two three (matrix_vector two two a x)) =
  matrix_vector_scale two two three a x
"#;

const UNSCALED: &str = r#"
theorem unscaled_value
    : Equal (Vec Nat two) (matrix_vector two two (matrix_scale two two three a) x) unscaled_result =
  Refl
"#;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix, including matrix_vector_scale, roots-loads");
    env
}

// Compare the checked fixture propositions, not source lines or formatting.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed matrix-vector proposition: {other:?}"),
    }
}

fn equality_shape(source: &str) -> (String, String, String) {
    let decls = parse_decls(source).expect("closed matrix-vector theorem parses");
    let [SourceDecl::TheoremDecl {
        theorem,
        body: Expr::ECon(proof, _),
        ..
    }] = decls.as_slice()
    else {
        panic!("one closed matrix-vector theorem with a constructor proof");
    };
    assert_eq!(proof, "Refl", "both value claims use the same proof form");
    let mut args = Vec::new();
    let mut head = theorem;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    assert!(
        matches!(head, SourceType::TVar(name, _) | SourceType::TCon(name, _) if name == "Equal")
    );
    let [carrier, left, right] = args.as_slice() else {
        panic!("Equal has one carrier and two endpoints");
    };
    (type_shape(carrier), type_shape(left), type_shape(right))
}

/// SOURCE: spec 65 §17, ruled law and closed [9, 21] discriminator.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the provider-first exact trusted GlobalId set is unchanged after
/// NatMatrix loads; its own public law is Transparent and an open-dimension
/// client applies it. The scaled matrix-vector application and the scaled
/// result of the unscaled application separately reduce to [9, 21]; the law
/// applies again on that closed input.
/// CLAIMED: a checked, reusable matrix-side scaling law at zero added trust.
/// THE GAP: provider trust is inherited, and finite values do not establish
/// a generic proof; the package's checked theorem body supplies that proof.
#[test]
fn matrix_vector_scale_is_public_checked_generic_and_has_both_closed_values() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Data.Vector.Vector",
        "Data.Numeric.Nat.Arithmetic",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("matrix-vector scaling proof roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "matrix-vector-scale trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "NatMatrix adds no trusted identity");
    let id = env.globals[&format!("{MATRIX}.matrix_vector_scale")];
    assert!(owned.contains(&id), "NatMatrix owns the new public law");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "matrix_vector_scale has a kernel-checked body"
    );
    env.elaborate_file(SHARED)
        .expect("R-GEN open law and R-VAL explicit [9, 21] accept");
    env.elaborate_file(RIGHT_AND_LAW)
        .expect("R-RIGHT explicit [9, 21] and R-LAW application accept");
}

/// SOURCE: spec 65 §17 refused same-input [3, 7] result.
/// PROMISE CLASS: durable invariant.
/// MEASURED: parsed R-VAL and X-UNSCALED have the same indexed carrier,
/// scaled input and Refl proof form; a fresh accepted prefix checks R-GEN and
/// R-VAL before X-UNSCALED refuses precisely at its own Refl.
/// CLAIMED: scaling the matrix by three is not silently ignored.
/// THE GAP: this finite discriminator is paired with the general checked law.
#[test]
fn matrix_vector_scale_refuses_unscaled_same_form_at_own_refl() {
    let true_theorem = SHARED
        .rsplit_once("theorem scaled_value")
        .map(|(_, rest)| format!("theorem scaled_value{rest}"))
        .expect("R-VAL ends the accepted prefix");
    let positive = equality_shape(&true_theorem);
    let negative = equality_shape(UNSCALED);
    assert_eq!(positive.0, negative.0, "same indexed result carrier");
    assert_eq!(positive.1, negative.1, "same scaled matrix and vector");
    assert_ne!(positive.2, negative.2, "different explicit results");

    let mut env = loaded_matrix();
    env.elaborate_file(SHARED)
        .expect("fresh R-GEN and same-form true R-VAL prefix");
    let start = UNSCALED
        .rfind("Refl")
        .expect("false claim has its own Refl");
    match env.elaborate_file(UNSCALED) {
        Err(ElabError::TypeMismatch { span, .. })
            if span.start == start && span.end == start + "Refl".len() => {}
        other => panic!("X-UNSCALED must refuse at its own Refl: {other:?}"),
    }
}
