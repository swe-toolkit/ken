//! Checked §5 reachability back along a graph walk.
//! Conformance-derived seed promise class: durable invariant. The generic
//! client proves the open type; the closed chain discriminates edge direction.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::Type as SourceType;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, KernelError};

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-reachability-along-walk.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("checked §5 graph package roots-loads");
    env
}

fn seed_clients() -> (&'static str, &'static str) {
    let mut accepted = None;
    let mut refused = None;
    for fence in extract_ken_md(SEED)
        .expect("§5 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let declarations = parse_decls(source).expect("§5 seed source parses");
        let has = |name: &str| {
            declarations.iter().any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name: found, .. } if found == name),
            )
        };
        if has("v0_reaches_v2_along_walk") {
            assert!(has("v0_reaches_v2_along_two_edges"));
            assert!(has("v0_reaches_v0") && has("v2_misses_v0"));
            assert!(
                accepted.replace(source).is_none(),
                "one accepted chain client"
            );
        }
        if has("seed_neg") {
            assert!(refused.replace(source).is_none(), "one isolated refusal");
        }
    }
    (
        accepted.expect("seed names the complete accepted chain"),
        refused.expect("seed names the isolated end-to-start refusal"),
    )
}

const GENERIC_CLIENT: &str = r#"
import Data.Finite.Finite (Finite)

import Algorithm.Graphs.Dependency
  (Graph, Walk, WalkHere, graph_reachable, graph_reachable_along_walk)

theorem client_along
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (s : q)
      (t : q)
      (target : q → Bool)
      (walk : Walk q g s t)
    : Equal Bool (graph_reachable q fq g t target) True
      → Equal Bool (graph_reachable q fq g s target) True =
  graph_reachable_along_walk q fq g s t target walk

theorem client_here
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool)
    : Equal Bool (graph_reachable q fq g s target) True
      → Equal Bool (graph_reachable q fq g s target) True =
  graph_reachable_along_walk q fq g s s target (WalkHere q g s)
"#;

/// SOURCE: spec/50-stdlib/62-graphs.md §5.
/// PROMISE CLASS: durable invariant.
/// MEASURED: public owner and transparent body, exact trusted GlobalId set,
/// independent generic client (including WalkHere), and private import refusal.
/// CLAIMED: every checked walk transports the endpoint reachability premise
/// back to its start with no new local trust. THE GAP: inherited provider trust
/// remains, and the closed chain below tests an instance, not generic coverage.
#[test]
fn generic_reachability_export_is_owned_private_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
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
        .expect("§5 checked graph theorem roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§5 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§5 adds no trusted GlobalId identity");

    let name = "graph_reachable_along_walk";
    let id = env.globals[&format!("{GRAPH}.{name}")];
    assert!(owned.contains(&id), "§5 law belongs to Dependency");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "§5 law has a checked body"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client consumes exact §5 type and WalkHere boundary");
    for private in ["require_some", "walk_reachable", "edge_reach_trans"] {
        match env.elaborate_file(&format!("import {GRAPH} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("expected a closed reachability proposition: {other:?}"),
    }
}

fn theorem_goal_shape(source: &str, theorem_name: &str) -> String {
    let decls = parse_decls(source).expect("Ken client parses");
    let goal = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, theorem, .. } if name == theorem_name => Some(theorem),
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected theorem {theorem_name}"));
    type_shape(goal)
}

const EXPECTED_START_TRUE: &str = r#"
theorem expected_start :
  Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v0) True = Proved
"#;
const EXPECTED_END_FALSE: &str = r#"
theorem expected_end_false :
  Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) False = Proved
"#;
const EXPECTED_END_TRUE: &str = r#"
theorem expected_end_true :
  Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) True = Proved
"#;

/// SOURCE: conformance/stdlib/graphs/seed-reachability-along-walk.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete accepted chain (both nonempty walks, independent
/// value controls and the zero-edge start hit) roots-loads; the isolated false
/// end-to-start claim matches a separate goal and is rejected at its theorem.
/// CLAIMED: §5 transport respects forward edge direction, and the refusal is
/// not a fixture/import/proof-form failure. THE GAP: these three closed vertices
/// do not replace the generic checked law and do not run official conformance.
#[test]
fn accepted_chain_and_false_end_to_start_claim_have_distinct_outcomes() {
    let (accepted, refused) = seed_clients();
    assert_eq!(
        theorem_goal_shape(accepted, "v0_reaches_v0"),
        theorem_goal_shape(EXPECTED_START_TRUE, "expected_start"),
        "same-form accepted reachability starts at V0"
    );
    assert_eq!(
        theorem_goal_shape(accepted, "v2_misses_v0"),
        theorem_goal_shape(EXPECTED_END_FALSE, "expected_end_false"),
        "independent value control observes no return from V2"
    );
    assert_eq!(
        theorem_goal_shape(refused, "seed_neg"),
        theorem_goal_shape(EXPECTED_END_TRUE, "expected_end_true"),
        "refusal must claim the false end-to-start reachability"
    );
    let decls = parse_decls(refused).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        span: claim_span,
        ..
    }] = decls.as_slice()
    else {
        panic!("isolated refusal must contain only seed_neg");
    };
    assert_eq!(name, "seed_neg");
    let mut env = graph_env();
    env.elaborate_file(accepted)
        .expect("complete accepted §5 chain checks against checked export");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false end-to-start claim must reject at its theorem: {other:?}"),
    }
}
