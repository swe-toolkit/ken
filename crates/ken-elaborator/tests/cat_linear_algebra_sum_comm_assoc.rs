//! Indexed sum laws: spec/50-stdlib/65-linear-algebra.md §5.
//! Seed: conformance/stdlib/linear-algebra/seed-vector-matrix-sum-comm-assoc.md.
//! Promise class: durable invariant. Generic checked laws complement closed
//! value controls, zero dimensions, and four isolated false equalities.

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
    "/../../conformance/stdlib/linear-algebra/seed-vector-matrix-sum-comm-assoc.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix and providers root-load with §5 proofs");
    env
}

fn seed_sources() -> Vec<&'static str> {
    extract_ken_md(SEED)
        .expect("authored §5 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect()
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, matrix_add, vector_add_comm, vector_add_assoc,
   matrix_add_comm, matrix_add_assoc)

theorem client_vector_comm (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) :
  Equal (Vec Nat n) (vector_add n xs ys) (vector_add n ys xs) =
  vector_add_comm n xs ys

theorem client_vector_assoc (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    (zs : Vec Nat n) :
  Equal (Vec Nat n)
    (vector_add n (vector_add n xs ys) zs)
    (vector_add n xs (vector_add n ys zs)) =
  vector_add_assoc n xs ys zs

theorem client_matrix_comm (m : Nat) (n : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_add m n a b) (matrix_add m n b a) =
  matrix_add_comm m n a b

theorem client_matrix_assoc (m : Nat) (n : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    (c : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m)
    (matrix_add m n (matrix_add m n a b) c)
    (matrix_add m n a (matrix_add m n b c)) =
  matrix_add_assoc m n a b c
"#;

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §§5.1–5.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the four root-loaded proofs are transparent and NatMatrix-owned;
/// a generic client uses their exact indexed equality types, and each private
/// step helper refuses selective import. Exact provider and package
/// `trusted_base()` GlobalId identity sets agree, not just their counts.
/// CLAIMED: §5 exports checked generic laws without extending package trust.
/// THE GAP: the inherited provider trust remains and finite seed values do not
/// replace generic proofs. The original package test owns the complete public
/// inventory pin; QA independently mutates the D0 steps for proof soundness.
#[test]
fn generic_sum_laws_are_owned_checked_private_and_add_no_trust() {
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
        .expect("§5 NatMatrix proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§5 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§5 adds no trusted GlobalId identities");
    for law in [
        "vector_add_comm",
        "vector_add_assoc",
        "matrix_add_comm",
        "matrix_add_assoc",
    ] {
        let id = env.globals[&format!("{MATRIX}.{law}")];
        assert!(owned.contains(&id), "{law} must be NatMatrix-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client consumes the four exact public §5 types");
    for helper in [
        "vector_add_cons",
        "vector_add_assoc_cons",
        "matrix_add_cons",
        "matrix_add_assoc_cons",
    ] {
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MATRIX}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-vector-matrix-sum-comm-assoc.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete accepted seed checks against root-loaded public
/// laws, independently constructed component and matrix values, an empty
/// vector, zero-row matrices and positive-row zero-column matrices.
/// CLAIMED: §5's specified closed sums and indexed applications are accepted.
/// THE GAP: finite values supplement, but cannot establish the generic laws.
#[test]
fn accepted_indexed_sum_seed_checks_with_zero_dimensions() {
    let sources = seed_sources();
    assert_eq!(
        sources.len(),
        5,
        "one accepted and four isolated refused fences"
    );
    loaded_matrix()
        .elaborate_file(sources[0])
        .expect("complete accepted §5 seed checks through published imports");
}

// Match a parsed proposition's type/term application tree rather than
// whitespace, seed line numbers, or an unrelated closed false equality.
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

fn expected_shape(proposition: &str) -> String {
    let declarations = parse_decls(&format!("theorem expected : {proposition} = Refl"))
        .expect("independent proposition parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = declarations.as_slice() else {
        panic!("expected one theorem proposition");
    };
    type_shape(theorem)
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-vector-matrix-sum-comm-assoc.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: full accepted seed checks before each isolated refusal; matching
/// same-operand result and changed-component/row controls are proved with
/// Refl. Every refused theorem has an independently specified, well-typed
/// proposition and Refl fails at its own proof with TypeMismatch.
/// CLAIMED: an indexed sum cannot discard an operand or regroup while changing
/// a component or row. Each negative is about its paired values, not any
/// closed false proposition.
/// THE GAP: these finite refusals cannot validate the universal bodies;
/// generic clients and independent QA production mutants supply other gates.
#[test]
fn four_isolated_false_sum_claims_refuse_after_same_operand_controls() {
    let sources = seed_sources();
    let [positive, negative_vector, negative_matrix, negative_component, negative_row] =
        sources.as_slice()
    else {
        panic!("seed has exactly one accepted and four refused fences");
    };
    let accepted = parse_decls(positive).expect("accepted seed parses");
    let cases = [
        (
            *negative_vector,
            "reject_vector_sum_equals_left",
            "Equal (Vec Nat two) (vector_add two xs ys) xs",
            "Equal (Vec Nat two) (vector_add two xs ys) (vec2 four two)",
        ),
        (
            *negative_matrix,
            "reject_matrix_sum_equals_left",
            "Equal (Vec (Vec Nat two) two) (matrix_add two two a b) a",
            "Equal (Vec (Vec Nat two) two) (matrix_add two two a b) expected_ab",
        ),
        (
            *negative_component,
            "reject_vector_regroup_changed_component",
            "Equal (Vec Nat two) (vector_add two (vector_add two xs ys) zs) (vector_add two xs (vector_add two ys changed_zs))",
            "Equal (Vec Nat two) (vector_add two xs (vector_add two ys changed_zs)) (vec2 six four)",
        ),
        (
            *negative_row,
            "reject_matrix_regroup_changed_row",
            "Equal (Vec (Vec Nat two) two) (matrix_add two two (matrix_add two two a b) c) (matrix_add two two a (matrix_add two two b changed_c))",
            "Equal (Vec (Vec Nat two) two) (matrix_add two two a (matrix_add two two b changed_c)) (matrix2 (vec2 six three) (vec2 (Suc six) two))",
        ),
    ];
    for (negative, name, false_goal, paired_control) in cases {
        let positive_shape = expected_shape(paired_control);
        assert!(
            accepted.iter().any(|decl| matches!(decl,
                SourceDecl::TheoremDecl { theorem, body: Expr::ECon(proof, _), .. }
                    if proof == "Refl" && type_shape(theorem) == positive_shape)),
            "{name}: accepted seed must contain an independently valued, paired control"
        );
        let declarations = parse_decls(negative).expect("isolated refusal parses");
        let [SourceDecl::TheoremDecl {
            name: actual,
            params,
            theorem,
            body: Expr::ECon(proof, proof_span),
            ..
        }] = declarations.as_slice()
        else {
            panic!("{name}: one closed attempted theorem, not unrelated failure");
        };
        assert_eq!(actual, name);
        assert!(params.is_empty(), "{name} must be closed");
        assert_eq!(proof, "Refl", "{name}: refuted equality must attempt Refl");
        assert_eq!(
            type_shape(theorem),
            expected_shape(false_goal),
            "{name}: exact false proposition"
        );
        let goal_span = theorem.span();
        let goal_text = &negative[goal_span.start..goal_span.end];
        let typed_control = format!("theorem typed_refusal (h : {goal_text}) : {goal_text} = h");
        let mut well_typed = loaded_matrix();
        well_typed
            .elaborate_file(positive)
            .expect("fresh accepted prefix checks before well-typed control");
        well_typed
            .elaborate_file(&typed_control)
            .expect("the false proposition itself is well-typed when assumed");

        let mut env = loaded_matrix();
        env.elaborate_file(positive)
            .expect("full accepted prefix checks before each fresh refusal");
        match env.elaborate_file(negative) {
            Err(ElabError::TypeMismatch { reason, span })
                if reason == "Refl expects an `Eq`-shaped goal"
                    && span.start >= proof_span.start
                    && span.end <= proof_span.end
                    && span.start < span.end => {}
            other => panic!("{name}: must reject at its own Refl: {other:?}"),
        }
    }
}
