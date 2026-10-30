//! Append-selection argmin: spec/50-stdlib/72-optimization.md §1.6.
//! Source: conformance/stdlib/optimization/seed-argmin-append-split.md.
//! Promise class: durable invariant. Tie-left and strictly-cheaper-right
//! selections, generic checked laws, and an isolated false later-tie claim.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const ARGMIN: &str = "Algorithm.Optimization.Argmin";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-argmin-append-split.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_argmin() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("checked Argmin and providers root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin, argmin_append_keeps_left, argmin_append_takes_right)
import Data.Collections.NonEmpty (NonEmpty, nonempty_append)
import Data.Numeric.Nat.Order (lt_nat)

theorem generic_keep
    (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs))) False
      → Equal a (argmin a cost (nonempty_append a xs ys)) (argmin a cost xs) =
  argmin_append_keeps_left a cost xs ys

theorem generic_take
    (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs))) True
      → Equal a (argmin a cost (nonempty_append a xs ys)) (argmin a cost ys) =
  argmin_append_takes_right a cost xs ys
"#;

/// MEASURED: both law identities belong to the roots-loaded Argmin package,
/// are Transparent, and elaborate a client restating their complete open
/// Boolean-premise types. All fourteen live private proof/selector helpers
/// refuse selective import. Exact trusted GlobalId sets agree before and after.
/// CLAIMED: §1.6 exports two checked generic laws with no new trust or helper
/// exposure. THE GAP: inherited provider assumptions remain; closed seed
/// results below test branch behavior but do not prove universal laws.
#[test]
fn append_laws_are_generic_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Function.Combinators",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Collections.Derived",
        "Data.Collections.NonEmpty",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("both §1.6 checked proofs root-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§1.6 Argmin trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "append laws add no trusted GlobalId");
    for name in ["argmin_append_keeps_left", "argmin_append_takes_right"] {
        let id = env.globals[&format!("{ARGMIN}.{name}")];
        assert!(owned.contains(&id), "{name} is owned by Argmin");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a kernel-checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client consumes full §1.6 laws for every type and cost");
    for private in [
        "choose",
        "costlier",
        "select",
        "select_view",
        "select_via_view",
        "holds_at",
        "transitive_at",
        "select_append_choice",
        "select_from_append",
        "choose_better_assoc",
        "lt_nat_true_trans",
        "lt_nat_false_trans",
        "costlier_true_trans",
        "costlier_false_trans",
    ] {
        let id = env.globals[&format!("{ARGMIN}.{private}")];
        assert!(owned.contains(&id), "{private} must exist in Argmin");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{private} must have a checked body"
        );
        match env.elaborate_file(&format!("import {ARGMIN} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{ARGMIN}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

// Select a source program by a contained theorem's identity rather than its
// Markdown position. Assertions below concern the Ken programs' behavior.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§1.6 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("seed source program parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify exactly one seed program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §1.6 theorem {theorem_name}"))
}

const INDEPENDENT_RESULTS: &str = r#"
theorem direct_tied_cost_two : Equal Nat (tied_cost two) two = Proved

theorem direct_tied_cost_three : Equal Nat (tied_cost three) seven = Proved

theorem direct_tied_append_result
    : Equal Nat (argmin Nat tied_cost tied_append) one = Proved

theorem direct_cheaper_append_result
    : Equal Nat (argmin Nat right_cheaper_cost tied_append) two = Proved
"#;

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §1.6 proposition: {other:?}"),
    }
}

/// MEASURED: the complete accepted seed checks cost values, separately
/// reduced part minima, and both law applications. The same append yields
/// label one by direct Proved on a tie and label two by direct Proved when
/// the right part becomes strictly cheaper. On a fresh complete prefix,
/// the sole isolated later-tie theorem must reject; its parsed shape is
/// checked only AFTER the behavioral refusal, so a true-neighbor mutation
/// cannot be masked by the shape assertion. CLAIMED: append selection keeps
/// the earliest tied left choice and switches to a cheaper right choice.
/// THE GAP: finite Nat inputs supplement, not replace, generic proof bodies.
#[test]
fn complete_seed_accepts_and_isolated_later_tie_refuses() {
    let accepted = seed_case("cheaper_append_takes_right");
    let refused = seed_case("seed_neg");
    let mut positive = loaded_argmin();
    positive
        .elaborate_file(accepted)
        .expect("complete §1.6 accepted client checks");
    positive
        .elaborate_file(INDEPENDENT_RESULTS)
        .expect("direct Proved tie-left and strictly-cheaper-right outputs check");
    for name in [
        "direct_tied_cost_two",
        "direct_tied_cost_three",
        "tied_left_selects_one",
        "tied_right_selects_two",
        "tied_append_keeps_left",
        "cheaper_right_selects_two",
        "cheaper_append_takes_right",
        "direct_tied_append_result",
        "direct_cheaper_append_result",
    ] {
        let id = positive.globals[name];
        assert!(matches!(
            positive.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    let parsed = parse_decls(refused).expect("isolated §1.6 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        theorem,
        body: SourceExpr::ECon(proof, _),
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated refusal contains one closed direct-Proved theorem");
    };
    let claim = refused
        .get(claim_span.start..claim_span.end)
        .expect("parsed theorem span is inside the refusal program");
    let mut refusing = loaded_argmin();
    refusing
        .elaborate_file(accepted)
        .expect("fresh complete accepted prefix checks before the refusal");
    refusing
        .elaborate_file(INDEPENDENT_RESULTS)
        .expect("fresh opposite-result controls check before the refusal");
    refusing
        .elaborate_file(claim)
        .expect_err("the sole false later-tie theorem must be refused");
    // Keep fixture-shape pins after behavioral rejection: a true same-input
    // claim must reach expect_err before its AST shape can fail this test.
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "the false claim is closed");
    assert_eq!(proof, "Proved");
    let expected = parse_decls(
        "theorem expected_false : Equal Nat (argmin Nat tied_cost tied_append) two = Proved",
    )
    .expect("independently stated false proposition parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_goal,
        ..
    }] = expected.as_slice()
    else {
        panic!("one independently stated false proposition");
    };
    assert_eq!(
        type_shape(theorem),
        type_shape(expected_goal),
        "isolated refusal must compare tied append to the later label two"
    );
}
