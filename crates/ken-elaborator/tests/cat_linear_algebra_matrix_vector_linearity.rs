//! Matrix-vector linearity: spec/50-stdlib/65-linear-algebra.md §6.
//! Seed: conformance/stdlib/linear-algebra/seed-matrix-vector-linearity.md.
//! Promise class: durable invariant. The generic laws and independently
//! valued seed clients are separate checks of the checked public surface.

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
    "/../../conformance/stdlib/linear-algebra/seed-matrix-vector-linearity.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix with both §6 proofs root-loads");
    env
}

// Find the two Ken clients by theorem identity, not Markdown fence position.
// Other valid source fences may be added without changing the selected clients.
fn seed_clients() -> (&'static str, &'static str) {
    let mut positive = None;
    let mut negative = None;
    for fence in extract_ken_md(SEED)
        .expect("authored §6 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let decls = parse_decls(source).expect("authored §6 seed source parses");
        let has = |target: &str| {
            decls
                .iter()
                .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == target))
        };
        if has("additive_closed_refl") {
            assert!(positive.replace(source).is_none(), "unique accepted client");
        }
        if has("seed_neg") {
            assert!(negative.replace(source).is_none(), "unique refused client");
        }
    }
    (
        positive.expect("accepted §6 client by theorem identity"),
        negative.expect("refused §6 client by theorem identity"),
    )
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_vector, vector_add, vector_scale,
   matrix_vector_additive, matrix_vector_homogeneous)

theorem generic_additive
    (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (x : Vec Nat n) (y : Vec Nat n)
  : Equal (Vec Nat m)
      (matrix_vector m n a (vector_add n x y))
      (vector_add m (matrix_vector m n a x) (matrix_vector m n a y)) =
  matrix_vector_additive m n a x y

theorem generic_homogeneous
    (m : Nat) (n : Nat) (factor : Nat)
    (a : Vec (Vec Nat n) m) (x : Vec Nat n)
  : Equal (Vec Nat m)
      (matrix_vector m n a (vector_scale n factor x))
      (vector_scale m factor (matrix_vector m n a x)) =
  matrix_vector_homogeneous m n factor a x
"#;

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §§6.1–6.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: both owned, Transparent proofs root-load and satisfy independent
/// arbitrary-index clients, including the zero-index domain of their types;
/// both dot helpers refuse public import. Provider and package trusted-base
/// GlobalId sets are exactly equal, not merely equal in cardinality.
/// CLAIMED: the two generic public laws close without new trust or helpers.
/// THE GAP: inherited provider trust remains, and the existing package test
/// pins the complete public inventory. QA separately mutates a proof step.
#[test]
fn generic_vector_linearity_is_owned_checked_private_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
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
        .expect("§6 NatMatrix proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§6 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§6 adds no trusted GlobalId identities");

    for law in ["matrix_vector_additive", "matrix_vector_homogeneous"] {
        let id = env.globals[&format!("{MATRIX}.{law}")];
        assert!(owned.contains(&id), "{law} belongs to NatMatrix");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic independent clients consume the exact §6 types");
    for helper in ["dot_distrib_right", "dot_scale_right"] {
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MATRIX}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

// This is a Ken proposition AST oracle, not a scan for documentation text.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed §6 proposition: {other:?}"),
    }
}

fn expected_shape(proposition: &str) -> String {
    let decls = parse_decls(&format!("theorem expected : {proposition} = Refl"))
        .expect("independently stated §6 proposition parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = decls.as_slice() else {
        panic!("expected one theorem proposition");
    };
    type_shape(theorem)
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-matrix-vector-linearity.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the whole named accepted client checks against root-loaded laws;
/// its four independently valued Refl images and same-form additive control
/// are present as checked theorems. An unrelated valid fence is immaterial.
/// CLAIMED: the specified non-square matrix and distinct vectors instantiate
/// the exported laws, and their four stated images compute as given.
/// THE GAP: these closed computations do not prove universal laws; the generic
/// arbitrary-index proof clients and trusted-identity check run separately.
#[test]
fn accepted_vector_linearity_seed_checks_independent_images() {
    let (positive, _) = seed_clients();
    let decls = parse_decls(positive).expect("accepted client parses");
    for name in [
        "image_x_value",
        "image_y_value",
        "image_sum_value",
        "image_scaled_value",
        "additive_closed",
        "homogeneous_closed",
        "additive_closed_refl",
    ] {
        assert!(
            decls.iter().any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name: found, .. } if found == name)
            ),
            "accepted §6 client includes {name}"
        );
    }
    let mut env = loaded_matrix();
    env.elaborate_file(positive)
        .expect("complete accepted §6 client checks through public imports");
    for name in [
        "image_x_value",
        "image_y_value",
        "image_sum_value",
        "image_scaled_value",
        "additive_closed",
        "homogeneous_closed",
        "additive_closed_refl",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked seed theorem"
        );
    }
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-matrix-vector-linearity.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the same-input true Refl control and complete accepted client
/// check before the isolated well-typed false proposition. Its exact AST is
/// pinned to image(x+y) = image(x); only its own Refl fails TypeMismatch.
/// CLAIMED: mapping x+y cannot discard this y on the specified 2×3 matrix.
/// THE GAP: a different false equation would also reject, so the AST pin must
/// distinguish it; no one finite refusal proves the generic additive law.
#[test]
fn false_sum_image_equals_x_image_refuses_at_its_own_refl() {
    let (positive, negative) = seed_clients();
    let positive_decls = parse_decls(positive).expect("accepted client parses");
    let true_goal = "Equal (Vec Nat two) (matrix_vector two three a23 (vector_add three x3 y3)) (vector_add two (matrix_vector two three a23 x3) (matrix_vector two three a23 y3))";
    assert!(
        positive_decls.iter().any(|decl| matches!(decl,
            SourceDecl::TheoremDecl {
                name, theorem, body: Expr::ECon(proof, _), ..
            } if name == "additive_closed_refl" && proof == "Refl"
                && type_shape(theorem) == expected_shape(true_goal))),
        "accepted seed contains the same-form true additive control"
    );

    let negative_decls = parse_decls(negative).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: Expr::ECon(proof, proof_span),
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("isolated refusal must have just one Refl theorem");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "the refusal is closed");
    assert_eq!(proof, "Refl", "the false equality attempts Refl");
    let false_goal = "Equal (Vec Nat two) (matrix_vector two three a23 (vector_add three x3 y3)) (matrix_vector two three a23 x3)";
    assert_eq!(
        type_shape(theorem),
        expected_shape(false_goal),
        "the refusal has the specified unequal images"
    );

    let goal_span = theorem.span();
    let goal = &negative[goal_span.start..goal_span.end];
    let typed_control = format!("theorem typed_refusal (p : {goal}) : {goal} = p");
    let mut well_typed = loaded_matrix();
    well_typed
        .elaborate_file(positive)
        .expect("full accepted seed checks before proposition control");
    well_typed
        .elaborate_file(&typed_control)
        .expect("false proposition itself is well typed when assumed");

    let mut env = loaded_matrix();
    env.elaborate_file(positive)
        .expect("complete accepted client checks before isolated refusal");
    match env.elaborate_file(negative) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start >= proof_span.start
                && span.end <= proof_span.end
                && span.start < span.end => {}
        other => panic!("false sum image must reject at its own Refl: {other:?}"),
    }
}
