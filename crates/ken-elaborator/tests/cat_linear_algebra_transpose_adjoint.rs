//! Transpose adjoint: spec/50-stdlib/65-linear-algebra.md §7.
//! SOURCE: conformance/stdlib/linear-algebra/seed-transpose-adjoint.md.
//! PROMISE CLASS: durable invariant. Generic proof ownership and independent
//! closed value controls are checked separately from the false swapped input.

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
    "/../../conformance/stdlib/linear-algebra/seed-transpose-adjoint.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix with transpose_adjoint root-loads");
    env
}

// A theorem identity selects each complete client, not a fence ordinal or
// the number of unrelated valid Ken source fences in the seed.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§7 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§7 source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify exactly one seed client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §7 seed theorem {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix (dot, matrix_vector, transpose, transpose_adjoint)

theorem adjoint_client
    (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (x : Vec Nat n) (y : Vec Nat m)
  : Equal Nat
      (dot m (matrix_vector m n a x) y)
      (dot n x (matrix_vector n m (transpose m n a) y)) =
  transpose_adjoint m n a x y
"#;

/// MEASURED: provider-loaded and NatMatrix trusted GlobalId identity sets
/// are equal; the new law is module-owned and Transparent, has an independent
/// arbitrary-dimension client, and the two new and two reused proof helpers
/// refuse selective import. CLAIMED: §7 has one generic checked public law
/// without adding trust or exporting proof scaffolding. THE GAP: existing
/// provider trust remains inherited; the seed's finite values cannot prove
/// a universally quantified law or an absent-import boundary by themselves.
#[test]
fn generic_transpose_adjoint_is_owned_checked_private_and_trust_free() {
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
        .expect("NatMatrix with §7 law kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§7 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§7 adds no trusted GlobalId identities");

    let id = env.globals[&format!("{MATRIX}.transpose_adjoint")];
    assert!(
        owned.contains(&id),
        "transpose_adjoint belongs to NatMatrix"
    );
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent arbitrary-index client consumes the exact §7 type");
    for helper in [
        "dot_zero_columns",
        "transpose_cons_image",
        "dot_scale_right",
        "dot_distrib_right",
    ] {
        match loaded_matrix().elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MATRIX}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

// Compare Ken proposition trees, not a source line or the seed's spelling.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected §7 closed proposition: {other:?}"),
    }
}

fn expected_shape(proposition: &str) -> String {
    let source = format!("theorem expected : {proposition} = Proved");
    let parsed = parse_decls(&source).expect("independently stated §7 proposition parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = parsed.as_slice() else {
        panic!("expected one theorem proposition");
    };
    type_shape(theorem)
}

/// MEASURED: the complete named accepted seed checks independently valued
/// transpose and image oracles, both 35-value calculations, the swapped
/// 37-value control, and an application of the public law. Its true closed
/// equality uses Proved. The isolated false 35 = 37 proposition is pinned by
/// parsed Ken shape, then refused at its own Proved with TypeMismatch.
/// CLAIMED: this fixed non-square client obeys adjointness only with the
/// same y on both sides. THE GAP: closed calculations do not prove the generic
/// law; changing the false proposition to true must make the shape pin red.
#[test]
fn full_seed_checks_and_swapped_right_input_refuses() {
    let accepted = seed_case("adjoint_closed_value");
    let refused = seed_case("seed_neg");
    let positive_decls = parse_decls(accepted).expect("complete §7 accepted client parses");
    let true_goal = "Equal Nat (dot two (matrix_vector two three a x) y) \
        (dot three x (matrix_vector three two (transpose two three a) y))";
    let true_control = positive_decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name,
                theorem,
                body: SourceExpr::ECon(proof, _),
                ..
            } if name == "adjoint_closed_value" && proof == "Proved" => Some(theorem),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(true_control.len(), 1, "one same-form true Proved control");
    assert_eq!(
        type_shape(true_control[0]),
        expected_shape(true_goal),
        "accepted seed has the independently stated true adjoint equation"
    );

    let parsed = parse_decls(refused).expect("isolated §7 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: SourceExpr::ECon(proof, _),
        span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated §7 refusal must be one theorem with direct proof");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "the swapped-y equation is closed");
    assert_eq!(proof, "Proved", "same form as the accepted true control");
    let false_goal = "Equal Nat (dot two (matrix_vector two three a x) y) \
        (dot three x (matrix_vector three two (transpose two three a) y_swapped))";
    assert_eq!(
        type_shape(theorem),
        expected_shape(false_goal),
        "seed_neg must assert 35 = 37 by swapping only the right y"
    );
    let mut env = loaded_matrix();
    env.elaborate_file(accepted)
        .expect("complete accepted seed checks before the false claim");
    for name in [
        "transpose_value",
        "left_image_value",
        "right_image_value",
        "swapped_image_value",
        "left_value",
        "right_value",
        "swapped_right_value",
        "adjoint_closed",
        "adjoint_closed_value",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    match env.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { span: failure, .. })
            if failure.start >= span.start
                && failure.end <= span.end
                && failure.start < failure.end => {}
        other => panic!("false swapped-y equation must fail at seed_neg's Proved: {other:?}"),
    }
}
