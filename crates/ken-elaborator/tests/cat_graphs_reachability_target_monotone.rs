//! Checked target monotonicity for graph reachability: spec 62 §6.
//! Source: conformance/stdlib/graphs/seed-reachability-target-monotone.md.
//! Promise class: durable invariant. Generic checked law and finite strict
//! widening distinguish the valid implication from its false converse.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-reachability-target-monotone.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§6 graph package roots-loads");
    env
}

// A theorem identity selects the entire accepted client or isolated refusal,
// regardless of how many other valid source fences the seed contains.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§6 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§6 source fence parses")
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
    selected.unwrap_or_else(|| panic!("missing §6 seed theorem {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Data.Finite.Finite (Finite)
import Algorithm.Graphs.Dependency (Graph, graph_reachable, graph_reachable_target_monotone)

theorem wider_target_client
      (q : Type) (fq : Finite q) (g : Graph q) (s : q)
      (target : q → Bool) (wider : q → Bool)
      (included : (v : q) → Equal Bool (target v) True → Equal Bool (wider v) True)
    : Equal Bool (graph_reachable q fq g s target) True
      → Equal Bool (graph_reachable q fq g s wider) True =
  graph_reachable_target_monotone q fq g s target wider included

theorem equal_target_client
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool)
    : Equal Bool (graph_reachable q fq g s target) True
      → Equal Bool (graph_reachable q fq g s target) True =
  graph_reachable_target_monotone q fq g s target target (λv. λhit. hit)
"#;

/// MEASURED: provider-loaded and Dependency-loaded trusted GlobalId sets
/// agree exactly; the new law is graph-package-owned and Transparent; its
/// independent arbitrary-carrier client and equal-target boundary check;
/// three existing proof helpers refuse selective import. CLAIMED: target
/// monotonicity is a public checked law with no new trust or public helper.
/// THE GAP: the provider's inherited assumptions remain; the closed finite
/// seed cannot itself prove the generic law or a full public export inventory.
#[test]
fn generic_target_monotonicity_is_owned_checked_private_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Data.Finite.Finite",
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Collections.Derived",
        "Core.Logic.Or",
        "Data.Collections.List",
        "Data.Numeric.Nat.Order",
        "Data.Sums.Combinators",
        "Algorithm.FormalLanguages.Dfa",
        "Algorithm.FormalLanguages.Reachability",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§6 checked graph law roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§6 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§6 adds no trusted GlobalId identities");
    let id = env.globals[&format!("{GRAPH}.graph_reachable_target_monotone")];
    assert!(owned.contains(&id), "§6 law belongs to Dependency");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic client consumes §6 type and equal-target boundary");
    for private in ["require_some", "walk_reachable", "find_walk_selected"] {
        match env.elaborate_file(&format!("import {GRAPH} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{private}"));
            }
            other => panic!("{private} must stay unimportable: {other:?}"),
        }
    }
}

// Assert Ken proposition structure, not a line/position in a Markdown file.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected §6 closed proposition: {other:?}"),
    }
}

fn expected_shape(proposition: &str) -> String {
    let source = format!("theorem expected : {proposition} = Proved");
    let parsed = parse_decls(&source).expect("independent §6 proposition parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = parsed.as_slice() else {
        panic!("expected one theorem proposition");
    };
    type_shape(theorem)
}

fn checked_proved_goal(source: &str, theorem_name: &str) -> String {
    let parsed = parse_decls(source).expect("§6 accepted source parses");
    let goals = parsed
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name,
                theorem,
                body: SourceExpr::ECon(proof, _),
                ..
            } if name == theorem_name && proof == "Proved" => Some(theorem),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(goals.len(), 1, "one checked same-form {theorem_name}");
    type_shape(goals[0])
}

/// MEASURED: the full named accepted seed checks strict widening on one
/// three-vertex chain, an independent True result for both targets at V0,
/// the checked public-law application, an independent True result for the
/// wider target at V2, and False for the narrower target there. The isolated
/// same-form narrower-target `Proved` is structurally pinned then kernel-
/// rejected with TypeMismatch at its own theorem. CLAIMED: reaching a wider
/// target does not imply reaching a narrower target. THE GAP: one finite
/// graph is a discriminator for the false converse, not generic coverage.
#[test]
fn complete_strict_widening_seed_accepts_and_false_converse_refuses() {
    let accepted = seed_case("v0_reaches_not_v0_by_law");
    let refused = seed_case("seed_neg");
    let true_goal = "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 not_v0) True";
    let false_control = "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v1) False";
    let false_converse = "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v1) True";
    assert_eq!(
        checked_proved_goal(accepted, "v2_reaches_not_v0"),
        expected_shape(true_goal),
        "same-form wider-target V2 claim must accept"
    );
    assert_eq!(
        checked_proved_goal(accepted, "v2_misses_v1"),
        expected_shape(false_control),
        "narrower target independently evaluates False at the same V2"
    );
    let parsed = parse_decls(refused).expect("isolated §6 converse parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: SourceExpr::ECon(proof, _),
        span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated converse must be one theorem with direct proof");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "converse claim is closed");
    assert_eq!(proof, "Proved", "same form as accepted true V2 control");
    assert_eq!(
        type_shape(theorem),
        expected_shape(false_converse),
        "seed_neg must claim false narrower-target reachability at V2"
    );
    let mut env = graph_env();
    env.elaborate_file(accepted)
        .expect("complete accepted strict-widening seed checks first");
    for name in [
        "v1_inside_not_v0",
        "v0_reaches_v1",
        "v0_reaches_not_v0_value",
        "v0_reaches_not_v0_by_law",
        "v2_reaches_not_v0",
        "v2_misses_v1",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span: failure,
        }) if failure.start >= span.start
            && failure.end <= span.end
            && failure.start < failure.end => {}
        other => panic!("false converse must fail at seed_neg's Proved: {other:?}"),
    }
}
