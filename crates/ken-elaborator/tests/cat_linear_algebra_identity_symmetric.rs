//! Identity symmetry: spec/50-stdlib/65-linear-algebra.md §8.
//! SOURCE: conformance/stdlib/linear-algebra/seed-identity-symmetric.md.
//! PROMISE CLASS: durable invariant. The generic checked public law is distinct
//! from closed constructor-valued identity oracles and the isolated false claim.

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
    "/../../conformance/stdlib/linear-algebra/seed-identity-symmetric.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix with public transpose_identity root-loads");
    env
}

// Select whole clients by theorem identity, not Markdown fence ordinal.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§8 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§8 source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            )
        {
            assert!(
                selected.replace(source).is_none(),
                "duplicate {theorem_name} client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §8 seed theorem {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix (identity, transpose, transpose_identity)

theorem client_identity_symmetric (n : Nat) :
  Equal (Vec (Vec Nat n) n) (transpose n n (identity n)) (identity n) =
  transpose_identity n
"#;

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §8.1.
/// PROMISE CLASS: durable invariant.
/// MEASURED: provider and NatMatrix trusted GlobalId identity sets agree;
/// the public law is NatMatrix-owned, Transparent, and consumed by an
/// independent arbitrary-index client. Its four direct helpers refuse import.
/// CLAIMED: the same generic checked law is public without new trust or
/// proof-scaffolding exports. THE GAP: inherited provider trust persists and
/// finite closed clients cannot establish this universal proposition alone.
#[test]
fn generic_identity_symmetry_is_owned_public_private_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Vector.Vector",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("generic NatMatrix §8 law kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§8 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§8 adds no trusted GlobalId identities");
    let id = env.globals[&format!("{MATRIX}.transpose_identity")];
    assert!(owned.contains(&id), "law belongs to NatMatrix");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("arbitrary-index public client consumes exact §8 law type");
    for helper in [
        "transpose_zero_prefix_rows",
        "zip_zero_map",
        "prepend_zero",
        "repeat",
    ] {
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MATRIX}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

// Compare Ken term and proposition ASTs, not source lines or whitespace. The
// constructor-valued identity is an independent fixture, not `identity three`.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected §8 proposition type: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("constructor:{name}"),
        SourceExpr::EVar(name, _) => format!("variable:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected §8 fixture/proof expression: {other:?}"),
    }
}

fn expected_theorem(proposition: &str, proof: &str) -> (String, String) {
    let decls = parse_decls(&format!("theorem expected : {proposition} = {proof}"))
        .expect("independent §8 proposition parses");
    let [SourceDecl::TheoremDecl { theorem, body, .. }] = decls.as_slice() else {
        panic!("expected one theorem");
    };
    (type_shape(theorem), expr_shape(body))
}

const EXPECTED_LITERAL_IDENTITY3: &str = r#"
const expected_identity3 : Vec (Vec Nat three) three =
  VCons
    (Vec Nat three)
    two
    (vec3 one Zero Zero)
    (VCons
      (Vec Nat three)
      one
      (vec3 Zero one Zero)
      (VCons (Vec Nat three) Zero (vec3 Zero Zero one) (VNil (Vec Nat three))))
"#;

/// SOURCE: conformance/stdlib/linear-algebra/seed-identity-symmetric.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the literal constructor-built identity and four accepted Refl
/// relations have independently stated Ken ASTs; the complete accepted client
/// kernel-checks with the public law applied at three. On a fresh accepted
/// prefix the false transpose=original claim has its required AST, is well
/// typed, and fails at its own Refl while the same-form control checks.
/// CLAIMED: transposing the 3×3 identity preserves its entries, but the
/// specified 2×2 upper-triangular matrix is not symmetric.
/// THE GAP: these fixed clients cannot substitute for the generic §8 proof;
/// the generic arbitrary-index public client is checked separately above.
#[test]
fn full_seed_checks_literal_identity_and_refuses_false_square_symmetry() {
    let accepted = seed_case("identity_three_value");
    let refused = seed_case("seed_neg");
    let accepted_decls = parse_decls(accepted).expect("complete accepted §8 client parses");
    let fixture = accepted_decls
        .iter()
        .find(|decl| matches!(decl, SourceDecl::ViewDecl { name, .. } if name == "identity3"))
        .expect("seed independently constructs a literal identity matrix");
    let expected_fixture =
        parse_decls(EXPECTED_LITERAL_IDENTITY3).expect("independent literal identity parses");
    let [SourceDecl::ViewDecl {
        ret_ty: Some(expected_ty),
        body: expected_val,
        ..
    }] = expected_fixture.as_slice()
    else {
        panic!("expected one typed literal matrix");
    };
    let SourceDecl::ViewDecl {
        ret_ty: Some(actual_ty),
        body: actual_val,
        ..
    } = fixture
    else {
        panic!("seed identity3 must be a typed constructor value");
    };
    assert_eq!(type_shape(actual_ty), type_shape(expected_ty));
    assert_eq!(expr_shape(actual_val), expr_shape(expected_val));

    for (name, proposition, proof) in [
        (
            "identity_three_value",
            "Equal (Vec (Vec Nat three) three) (identity three) identity3",
            "Refl",
        ),
        (
            "transpose_identity_three_value",
            "Equal (Vec (Vec Nat three) three) (transpose three three (identity three)) identity3",
            "Refl",
        ),
        (
            "transpose_identity_three_law",
            "Equal (Vec (Vec Nat three) three) (transpose three three (identity three)) (identity three)",
            "transpose_identity three",
        ),
        (
            "upper_transpose_value",
            "Equal (Vec (Vec Nat two) two) (transpose two two upper) upper_transposed",
            "Refl",
        ),
        (
            "upper_same_form_control",
            "Equal (Vec (Vec Nat two) two) upper upper",
            "Refl",
        ),
    ] {
        let matches = accepted_decls
            .iter()
            .filter(|decl| match decl {
                SourceDecl::TheoremDecl {
                    name: actual,
                    params,
                    theorem,
                    body,
                    ..
                } if actual == name && params.is_empty() => {
                    (type_shape(theorem), expr_shape(body)) == expected_theorem(proposition, proof)
                }
                _ => false,
            })
            .count();
        assert_eq!(matches, 1, "accepted §8 control {name} keeps its proof and goal");
    }

    let parsed = parse_decls(refused).expect("isolated §8 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: SourceExpr::ECon(proof, proof_span),
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated false claim is one closed theorem with Refl");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "refused claim is closed");
    assert_eq!(proof, "Refl");
    let false_goal = "Equal (Vec (Vec Nat two) two) (transpose two two upper) upper";
    assert_eq!(
        type_shape(theorem),
        expected_theorem(false_goal, "Refl").0,
        "refusal must be transpose=original on the same upper matrix"
    );
    let mut typing = loaded_matrix();
    typing
        .elaborate_file(accepted)
        .expect("complete accepted §8 client checks before false proposition");
    let typed_refusal = format!("theorem typed_refusal (p : {false_goal}) : {false_goal} = p");
    typing
        .elaborate_file(&typed_refusal)
        .expect("false square equality is well typed when assumed");

    let mut env = loaded_matrix();
    env.elaborate_file(accepted)
        .expect("complete identity symmetry seed and same-form control check");
    for name in [
        "identity_three_value",
        "transpose_identity_three_value",
        "transpose_identity_three_law",
        "upper_transpose_value",
        "upper_same_form_control",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    match env.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start >= proof_span.start
                && span.end <= proof_span.end
                && span.start < span.end => {}
        other => panic!("false transpose=original must reject at its own Refl: {other:?}"),
    }
}
