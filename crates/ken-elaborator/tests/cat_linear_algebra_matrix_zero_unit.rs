//! Public zero-matrix laws: spec/50-stdlib/65-linear-algebra.md §12.
//! Source: conformance/stdlib/linear-algebra/seed-matrix-zero-unit.md.
//! Promise class: durable invariant. Finite value oracles supplement, but do
//! not replace, generic checked proofs over open dimensions.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/linear-algebra/seed-matrix-zero-unit.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix package roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_add, matrix_zero, matrix_vector, vector_zero,
   matrix_add_zero_left, matrix_add_zero_right, matrix_vector_zero_matrix)

fn generic_zero_matrix (m : Nat) (n : Nat) : Vec (Vec Nat n) m =
  matrix_zero m n

theorem generic_left_unit (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_add m n (matrix_zero m n) a) a =
  matrix_add_zero_left m n a

theorem generic_right_unit (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_add m n a (matrix_zero m n)) a =
  matrix_add_zero_right m n a

theorem generic_zero_action (m : Nat) (n : Nat) (v : Vec Nat n) :
  Equal (Vec Nat m) (matrix_vector m n (matrix_zero m n) v) (vector_zero m) =
  matrix_vector_zero_matrix m n v
"#;

/// SOURCE: spec 65 §12. PROMISE CLASS: durable invariant.
/// MEASURED: the full provider-first trusted GlobalId set equals the set after
/// NatMatrix loads; all four new names are NatMatrix-owned Transparent bodies,
/// generic open-dimension clients check, and private `repeat` refuses import.
/// CLAIMED: the four public declarations are reusable for arbitrary shapes
/// without adding trusted premises. THE GAP: a checked transparent identity
/// alone cannot pin a theorem's statement; open clients use each exact type,
/// while the independent closed seed below distinguishes incorrect values.
#[test]
fn generic_zero_matrix_laws_are_public_checked_and_zero_trust() {
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
        .expect("all three §12 proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§12 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§12 adds no trusted GlobalId");
    for name in [
        "matrix_zero",
        "matrix_add_zero_left",
        "matrix_add_zero_right",
        "matrix_vector_zero_matrix",
    ] {
        let id = env.globals[&format!("{MATRIX}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to NatMatrix");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must have a checked Transparent body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all public §12 types check at open m, n, a and v");
    match env.elaborate_file(&format!("import {MATRIX} (repeat)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{MATRIX}.repeat"));
        }
        other => panic!("private repeat must refuse selective import: {other:?}"),
    }
}

// Resolve the positive and isolated refusal by their checked theorem names,
// not by fence ordinal, prose positions or any package-source text.
fn seed_case(name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§12 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§12 seed Ken parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name))
        {
            assert!(selected.replace(source).is_none(), "duplicate seed case {name}");
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed case {name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed §12 seed proposition: {other:?}"),
    }
}

fn proposition_shape(proposition: &str) -> String {
    let source = format!("theorem expected_relation : {proposition} = Refl");
    let decls = parse_decls(&source).expect("specified seed proposition parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = decls.as_slice() else {
        panic!("specified seed proposition is exactly one theorem");
    };
    type_shape(theorem)
}

fn seed_theorem<'a>(decls: &'a [SourceDecl], name: &str) -> (&'a SourceType, &'a SourceExpr) {
    let found = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                body,
                ..
            } if actual == name => Some((theorem, body)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [only] = found.as_slice() else {
        panic!("exactly one seed theorem {name} is required");
    };
    *only
}

fn asserted_application(body: &SourceExpr, law: &str, arguments: &[&str]) {
    let mut current = body;
    let mut found = Vec::new();
    while let SourceExpr::EApp(function, argument, _) = current {
        let name = match argument.as_ref() {
            SourceExpr::EVar(name, _) | SourceExpr::ECon(name, _) => name.as_str(),
            other => panic!("{law} argument must be a named seed fixture: {other:?}"),
        };
        found.push(name);
        current = function;
    }
    found.reverse();
    assert!(matches!(current, SourceExpr::EVar(name, _) if name == law));
    assert_eq!(found, arguments, "{law} applies at the specified fixture");
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-matrix-zero-unit.md §12.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete accepted seed checks, its six theorem applications
/// use public names at fixed nonzero/zero-row fixtures, and its zero matrix
/// equals an independently constructed constructor value. A fresh complete
/// accepted prefix precedes the isolated false result; the same sum and Refl
/// control checks, whereas the false RHS refuses inside its own theorem at
/// Refl with TypeMismatch. CLAIMED: the zero matrix is an additive unit,
/// not the result when added to a nonzero matrix. THE GAP: these finite rows
/// do not prove the generic laws; those bodies and open clients check above.
#[test]
fn seed_six_laws_and_zero_value_check_then_false_sum_refuses_at_refl() {
    let accepted = seed_case("matrix_add_zero_left_nonzero");
    let refused = seed_case("reject_zero_as_sum");
    let positives = parse_decls(accepted).expect("entire accepted seed parses");
    for (name, law, arguments) in [
        (
            "matrix_add_zero_left_nonzero",
            "matrix_add_zero_left",
            ["two", "two", "a"],
        ),
        (
            "matrix_add_zero_right_nonzero",
            "matrix_add_zero_right",
            ["two", "two", "a"],
        ),
        (
            "matrix_vector_zero_matrix_nonzero",
            "matrix_vector_zero_matrix",
            ["two", "two", "u"],
        ),
        (
            "matrix_add_zero_left_empty",
            "matrix_add_zero_left",
            ["Zero", "two", "empty"],
        ),
        (
            "matrix_add_zero_right_empty",
            "matrix_add_zero_right",
            ["Zero", "two", "empty"],
        ),
        (
            "matrix_vector_zero_matrix_empty",
            "matrix_vector_zero_matrix",
            ["Zero", "two", "u"],
        ),
    ] {
        let (_, body) = seed_theorem(&positives, name);
        asserted_application(body, law, &arguments);
    }
    let (zero_value, zero_proof) = seed_theorem(&positives, "matrix_zero_two_by_two_value");
    assert!(matches!(zero_proof, SourceExpr::ECon(proof, _) if proof == "Refl"));
    assert_eq!(
        type_shape(zero_value),
        proposition_shape("Equal (Vec (Vec Nat two) two) (matrix_zero two two) (rows2 (pair Zero Zero) (pair Zero Zero))"),
        "independent two-row zero-matrix constructor value stays fixed"
    );
    let (control, control_proof) =
        seed_theorem(&positives, "matrix_add_zero_left_same_form_control");
    assert!(matches!(control_proof, SourceExpr::ECon(proof, _) if proof == "Refl"));
    assert_eq!(
        type_shape(control),
        proposition_shape(
            "Equal (Vec (Vec Nat two) two) (matrix_add two two (matrix_zero two two) a) a"
        ),
        "accepted control must keep the specified sum and nonzero RHS"
    );
    let negative = parse_decls(refused).expect("isolated false theorem parses");
    let [SourceDecl::TheoremDecl {
        name,
        theorem,
        body: SourceExpr::ECon(proof, proof_span),
        span: theorem_span,
        ..
    }] = negative.as_slice()
    else {
        panic!("refusal must contain exactly one Refl theorem");
    };
    assert_eq!(name, "reject_zero_as_sum");
    assert_eq!(proof, "Refl", "refusal has the control's proof form");
    assert_eq!(
        type_shape(theorem),
        proposition_shape("Equal (Vec (Vec Nat two) two) (matrix_add two two (matrix_zero two two) a) (matrix_zero two two)"),
        "false claim keeps the same sum but changes only the RHS"
    );
    let mut positive = loaded_matrix();
    positive
        .elaborate_file(accepted)
        .expect("full accepted seed checks");
    for name in [
        "matrix_add_zero_left_nonzero",
        "matrix_add_zero_right_nonzero",
        "matrix_vector_zero_matrix_nonzero",
        "matrix_add_zero_left_empty",
        "matrix_add_zero_right_empty",
        "matrix_vector_zero_matrix_empty",
        "matrix_zero_two_by_two_value",
        "matrix_add_zero_left_same_form_control",
    ] {
        let id = positive.globals[name];
        assert!(matches!(
            positive.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    let mut rejecting = loaded_matrix();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh complete positive seed and control check before refusal");
    match rejecting.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start >= proof_span.start
                && span.end <= proof_span.end
                && span.start >= theorem_span.start
                && span.end <= theorem_span.end
                && span.start < span.end => {}
        other => panic!("false zero-result theorem must refuse at its own Refl: {other:?}"),
    }
}
