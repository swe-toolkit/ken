//! Natural matrix scaling: spec/50-stdlib/65-linear-algebra.md §16.
//! Open client applications assert eight checked laws; closed values exercise
//! the exact scalar semantics and reject both wrong implementations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError, Expr, Type as SourceType};
use ken_kernel::Decl;

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const GENERIC: &str = r###"import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_scale, matrix_add, matrix_product, transpose, matrix_zero,
   matrix_scale_distrib, matrix_scale_factor_distrib, matrix_scale_compose,
   matrix_scale_one, matrix_scale_zero, transpose_scale,
   matrix_product_scale, matrix_product_scale_right)
import Data.Numeric.Nat.Arithmetic (add, mul)

theorem d (m : Nat) (n : Nat) (x : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m)
    (matrix_scale m n x (matrix_add m n a b))
    (matrix_add m n (matrix_scale m n x a) (matrix_scale m n x b)) =
  matrix_scale_distrib m n x a b

theorem fd (m : Nat) (n : Nat) (x : Nat) (y : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_scale m n (add x y) a)
    (matrix_add m n (matrix_scale m n x a) (matrix_scale m n y a)) =
  matrix_scale_factor_distrib m n x y a

theorem c (m : Nat) (n : Nat) (x : Nat) (y : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_scale m n (mul x y) a)
    (matrix_scale m n x (matrix_scale m n y a)) =
  matrix_scale_compose m n x y a

theorem o (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_scale m n (Suc Zero) a) a =
  matrix_scale_one m n a

theorem z (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_scale m n Zero a) (matrix_zero m n) =
  matrix_scale_zero m n a

theorem t (m : Nat) (n : Nat) (x : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat m) n)
    (transpose m n (matrix_scale m n x a))
    (matrix_scale n m x (transpose m n a)) =
  transpose_scale m n x a

theorem pl (m : Nat) (n : Nat) (p : Nat) (x : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) :
  Equal (Vec (Vec Nat p) m)
    (matrix_product m n p (matrix_scale m n x a) b)
    (matrix_scale m p x (matrix_product m n p a b)) =
  matrix_product_scale m n p x a b

theorem pr (m : Nat) (n : Nat) (p : Nat) (x : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) :
  Equal (Vec (Vec Nat p) m)
    (matrix_product m n p a (matrix_scale n p x b))
    (matrix_scale m p x (matrix_product m n p a b)) =
  matrix_product_scale_right m n p x a b
"###;

const TRUE: &str = r###"import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_scale, matrix_add, matrix_product, transpose, matrix_zero)
import Data.Numeric.Nat.Arithmetic (add, mul)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five
const seven : Nat = Suc six
const eight : Nat = Suc seven
const nine : Nat = Suc eight
const twelve : Nat = add six six
const fifteen : Nat = add nine six
const eighteen : Nat = mul three six
fn row3 (x : Nat) (y : Nat) (z : Nat) : Vec Nat three =
  VCons Nat two x (VCons Nat one y (VCons Nat Zero z (VNil Nat)))
fn row2 (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
fn matrix2x3 (first : Vec Nat three) (second : Vec Nat three) : Vec (Vec Nat three) two =
  VCons (Vec Nat three) one first (VCons (Vec Nat three) Zero second (VNil (Vec Nat three)))
fn matrix3x2 (first : Vec Nat two) (second : Vec Nat two) (third : Vec Nat two) : Vec (Vec Nat two) three =
  VCons (Vec Nat two) two first (VCons (Vec Nat two) one second (VCons (Vec Nat two) Zero third (VNil (Vec Nat two))))
const a : Vec (Vec Nat three) two = matrix2x3 (row3 one two three) (row3 four five six)
const c : Vec (Vec Nat three) two = matrix2x3 (row3 six five four) (row3 three two one)
const b : Vec (Vec Nat two) three = matrix3x2 (row2 one two) (row2 three four) (row2 five six)
const scaled_a : Vec (Vec Nat three) two = matrix2x3 (row3 three six nine) (row3 twelve fifteen eighteen)
theorem correct_scale : Equal (Vec (Vec Nat three) two) (matrix_scale two three three a) scaled_a = Refl
"###;

const FIRST: &str = r###"theorem first_row_only_is_false : Equal (Vec (Vec Nat three) two) (matrix_scale two three three a) (matrix2x3 (row3 three six nine) (row3 four five six)) = Refl
"###;

const PLUS: &str = r###"theorem plus_each_is_false : Equal (Vec (Vec Nat three) two) (matrix_scale two three three a) (matrix2x3 (row3 four five six) (row3 seven eight nine)) = Refl
"###;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix and its checked proofs roots-load");
    env
}

// Compare parsed propositions, not repository source or incidental formatting.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed scaling proposition: {other:?}"),
    }
}

fn equality_shape(source: &str) -> (String, String, String) {
    let decls = parse_decls(source).expect("closed scaling theorem parses");
    let [SourceDecl::TheoremDecl {
        theorem,
        body: Expr::ECon(proof, _),
        ..
    }] = decls.as_slice()
    else {
        panic!("one closed scaling theorem with a constructor proof");
    };
    assert_eq!(proof, "Refl", "all controls use the same proof form");
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
        panic!("three Equal arguments");
    };
    (type_shape(carrier), type_shape(left), type_shape(right))
}

/// SOURCE: spec 65 §16; PROMISE CLASS: durable invariant.
/// MEASURED: provider-first exact trusted GlobalId set is unchanged after the
/// package loads; nine new public identities have Transparent checked bodies,
/// their exact generic types accept an open-dimension client, and both new
/// private helper imports refuse as UnboundName.
/// CLAIMED: scaling and all eight laws are usable without new trust or leaked
/// proof scaffolding. THE GAP: inherited provider trust remains; the separate
/// closed client distinguishes wrong factor behavior.
#[test]
fn generic_matrix_scaling_laws_are_public_checked_and_zero_trust() {
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
        .expect("all eight public laws check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "NatMatrix adds no trusted identities");
    for name in [
        "matrix_scale",
        "matrix_scale_distrib",
        "matrix_scale_factor_distrib",
        "matrix_scale_compose",
        "matrix_scale_one",
        "matrix_scale_zero",
        "transpose_scale",
        "matrix_product_scale",
        "matrix_product_scale_right",
    ] {
        let id = env.globals[&format!("{MATRIX}.{name}")];
        assert!(owned.contains(&id), "{name} is NatMatrix-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC)
        .expect("all eight law types accept a generic independent client");
    for helper in ["zip_prepend_scale", "columns_scale_right"] {
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MATRIX}.{helper}"))
            }
            other => panic!("{helper} must stay private: {other:?}"),
        }
    }
}

/// SOURCE: spec 65 §16 and its closed 2×3 discriminator.
/// PROMISE CLASS: normative compatibility vector for the exact stated values.
/// MEASURED: a correct closed Refl theorem checks, and first-row-only and
/// plus-factor twins separately refuse as TypeMismatch at their own Refl,
/// after fresh copies of the same accepted prefix and unchanged left operand.
/// CLAIMED: every entry is multiplied by the factor, including later rows.
/// THE GAP: finite clients cannot prove the generic laws; the checked eight-law
/// client above supplies that separate evidence.
#[test]
fn closed_matrix_scaling_refuses_both_wrong_factor_values() {
    let prefix_shape = equality_shape(
        TRUE.rsplit_once("theorem correct_scale")
            .map(|(_, theorem)| format!("theorem correct_scale{theorem}"))
            .as_deref()
            .expect("correct theorem"),
    );
    for (tag, twin) in [("first-row-only", FIRST), ("plus-factor", PLUS)] {
        let shape = equality_shape(twin);
        assert_eq!(
            shape.0, prefix_shape.0,
            "{tag}: same indexed matrix carrier"
        );
        assert_eq!(
            shape.1, prefix_shape.1,
            "{tag}: same scaling operation and inputs"
        );
        assert_ne!(shape.2, prefix_shape.2, "{tag}: distinct false result");
        let mut env = loaded_matrix();
        env.elaborate_file(TRUE)
            .expect("fresh accepted prefix and true closed equality");
        let start = twin.rfind("Refl").expect("twin proof");
        match env.elaborate_file(twin) {
            Err(ElabError::TypeMismatch { span, .. })
                if span.start == start && span.end == start + "Refl".len() => {}
            other => panic!("{tag} must fail at its own Refl: {other:?}"),
        }
    }
}
