//! Row-wise natural matrix sum: spec/50-stdlib/65-linear-algebra.md §4.
//! The package seed is conformance/stdlib/linear-algebra/seed-matrix-sum-distributivity.md.
//! Promise class: durable invariant. Generic laws are independently consumed;
//! closed, typed values and the paired refusal exercise the shipped seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/linear-algebra/seed-matrix-sum-distributivity.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn matrix_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix and provider closure root-load");
    env
}

fn seed_sources() -> Vec<&'static str> {
    extract_ken_md(SEED)
        .expect("§4 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect()
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_add, matrix_product, matrix_vector, transpose, vector_add,
   matrix_product_distrib_left, matrix_product_distrib_right,
   matrix_vector_distrib, transpose_add)

fn client_matrix_add (m : Nat) (n : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m) : Vec (Vec Nat n) m =
  matrix_add m n a b

theorem client_product_distrib_left
    (m : Nat) (n : Nat) (p : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) (c : Vec (Vec Nat p) n) :
  Equal (Vec (Vec Nat p) m)
    (matrix_product m n p a (matrix_add n p b c))
    (matrix_add m p (matrix_product m n p a b) (matrix_product m n p a c)) =
  matrix_product_distrib_left m n p a b c

theorem client_product_distrib_right
    (m : Nat) (n : Nat) (p : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m) (c : Vec (Vec Nat p) n) :
  Equal (Vec (Vec Nat p) m)
    (matrix_product m n p (matrix_add m n a b) c)
    (matrix_add m p (matrix_product m n p a c) (matrix_product m n p b c)) =
  matrix_product_distrib_right m n p a b c

theorem client_matrix_vector_distrib
    (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    (x : Vec Nat n) :
  Equal (Vec Nat m)
    (matrix_vector m n (matrix_add m n a b) x)
    (vector_add m (matrix_vector m n a x) (matrix_vector m n b x)) =
  matrix_vector_distrib m n a b x

theorem client_transpose_add
    (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat m) n)
    (transpose m n (matrix_add m n a b))
    (matrix_add n m (transpose m n a) (transpose m n b)) =
  transpose_add m n a b
"#;

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §§4.1–4.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all five public names are owned, transparent root-loaded globals;
/// their exact generic types check in a separate client. Each of four named
/// proof helpers refuses selective import. The target package adds no new
/// `trusted_base()` GlobalId above its provider closure, compared as sets.
/// CLAIMED: §4 publishes checked generic laws without additional trust.
/// THE GAP: inherited provider trust persists; closed numeric behavior needs
/// the independent seed, and exact API closure is reviewed with the original
/// package-owned inventory test, updated only for the five intended exports.
#[test]
fn generic_matrix_sum_exports_are_checked_private_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Vector.Vector",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("§4 matrix sum and four proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§4 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§4 adds no trusted GlobalId identities");

    for name in [
        "matrix_add",
        "matrix_product_distrib_left",
        "matrix_product_distrib_right",
        "matrix_vector_distrib",
        "transpose_add",
    ] {
        let id = env.globals[&format!("{MATRIX}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to NatMatrix");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent indexed client consumes all five §4 types");
    for helper in [
        "dot_distrib_right",
        "columns_add_right",
        "row_product_add",
        "zip_prepend_add",
    ] {
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MATRIX}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-matrix-sum-distributivity.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each of the four shipped positive source fences checks in order,
/// including the independent 2×2 constructor-built sum oracle, non-square
/// left/right product values, and the correct same-operand control.
/// CLAIMED: fixed indexed matrix sums and product orientations match §4.
/// THE GAP: finite values supplement, not prove, the generic checked laws.
#[test]
fn shipped_sum_seed_four_positive_fences_check() {
    let sources = seed_sources();
    let [sum_oracles, left, right, shared, _negative] = sources.as_slice() else {
        panic!("§4 seed must retain four positive and one refused source fence");
    };
    let mut env = matrix_env();
    for (source, label) in [
        (*sum_oracles, "direct and non-square sum oracles"),
        (*left, "left product distributivity and independent value"),
        (*right, "right product distributivity and independent value"),
        (*shared, "correct same-operand product control"),
    ] {
        env.elaborate_file(source)
            .unwrap_or_else(|error| panic!("{label} checks: {error:?}"));
    }
    let id = env.globals["matrix_add_two_by_two_value"];
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
}

fn applied_type<'a>(ty: &'a SourceType, name: &str) -> Option<Vec<&'a SourceType>> {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    match head {
        SourceType::TVar(found, _) | SourceType::TCon(found, _) if found == name => Some(args),
        _ => None,
    }
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed sum proposition: {other:?}"),
    }
}

fn parsed_goal_shape(proposition: &str) -> String {
    let source = format!("theorem expected_sum_goal : {proposition} = Refl");
    let declarations = parse_decls(&source).expect("expected same-operand goal parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = declarations.as_slice() else {
        panic!("expected same-operand goal is one theorem");
    };
    type_shape(theorem)
}

// Derive the dropped product from the checked accepting control's own
// operands; neither the negative text nor a second reference to the same
// false seed assertion supplies the expected proposition.
fn assert_paired_sum_goals(positive: &str, refused: &str) {
    let positive_decls = parse_decls(positive).expect("same-input control parses");
    let Some(SourceDecl::TheoremDecl {
        theorem: control_goal,
        body: Expr::ECon(proof, _),
        ..
    }) = positive_decls.last()
    else {
        panic!("positive control ends in a checked theorem");
    };
    assert_eq!(proof, "Refl", "control states the actual equality");
    let equal = applied_type(control_goal, "Equal").expect("control is equality");
    let [carrier, lhs, rhs] = equal.as_slice() else {
        panic!("control has one carrier and two endpoints");
    };
    let product = applied_type(lhs, "matrix_product").expect("control multiplies the sum");
    let [m, n, p, a, right_sum] = product.as_slice() else {
        panic!("left product has three dimensions and two operands");
    };
    let sum = applied_type(right_sum, "matrix_add").expect("right operands are summed");
    let [sum_n, sum_p, b, c] = sum.as_slice() else {
        panic!("right sum has two dimensions and two summands");
    };
    assert_eq!(type_shape(n), type_shape(sum_n));
    assert_eq!(type_shape(p), type_shape(sum_p));
    let slice = |ty: &SourceType| &positive[ty.span().start..ty.span().end];
    let ab = format!(
        "matrix_product ({}) ({}) ({}) ({}) ({})",
        slice(m),
        slice(n),
        slice(p),
        slice(a),
        slice(b)
    );
    let ac = format!(
        "matrix_product ({}) ({}) ({}) ({}) ({})",
        slice(m),
        slice(n),
        slice(p),
        slice(a),
        slice(c)
    );
    let relation = |last: &str| {
        format!(
            "Equal ({}) ({}) (matrix_add ({}) ({}) ({}) ({}))",
            slice(carrier),
            slice(lhs),
            slice(m),
            slice(p),
            ab,
            last
        )
    };
    assert_eq!(
        type_shape(control_goal),
        parsed_goal_shape(&relation(&ac)),
        "accepting control has both products on the same operands"
    );
    let refused_decls = parse_decls(refused).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        theorem: negative_goal,
        body: Expr::ECon(negative_proof, _),
        ..
    }] = refused_decls.as_slice()
    else {
        panic!("refusal must consist of exactly one attempted theorem proof");
    };
    assert_eq!(name, "reject_dropped_right_product");
    assert_eq!(negative_proof, "Refl");
    assert_eq!(
        type_shape(negative_goal),
        parsed_goal_shape(&relation(slice(c))),
        "refusal must omit only the same-input right product, not state an unrelated falsehood"
    );
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-matrix-sum-distributivity.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all positive cases and the correct same-input control check
/// before exactly one appended theorem fails with the Refl goal-shape error;
/// the parsed proposition is independently reconstructed from the control's
/// operands, with only its final right product dropped.
/// CLAIMED: this seed's square left factor does not support `a(b+c)=ab+c`.
/// THE GAP: one closed counterexample does not prove the four generic laws.
#[test]
fn shipped_dropped_product_refuses_only_after_same_operand_control() {
    let sources = seed_sources();
    let [first, left, right, shared, refused] = sources.as_slice() else {
        panic!("§4 seed must retain four positive and one refused source fence");
    };
    assert_paired_sum_goals(shared, refused);
    let mut env = matrix_env();
    for positive in [first, left, right, shared] {
        env.elaborate_file(positive)
            .expect("all seed positives, including same-input control, check first");
    }
    match env.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.end <= refused.len()
                && span.start < span.end => {}
        other => panic!("dropped-product proposition must refuse at its own Refl: {other:?}"),
    }
}
