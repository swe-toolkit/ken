//! Reverse-graph reachability: spec/50-stdlib/62-graphs.md §8.
//! Promise class: durable invariant. Arbitrary-carrier clients consume four
//! checked exports; a fixed two-edge chain separates reversed reachability
//! from the false claim on the original directed graph.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, KernelError};

const GRAPH: &str = "Algorithm.Graphs.Dependency";
// Reuse the §7 seed's checked three-vertex chain, not its law as an oracle.
// The §8 conformance seed has a separate owner; this test exercises the public
// exports directly.
const CHAIN_SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-reachability-graph-monotone.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_graph() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§8 Graphs Dependency roots-loads");
    env
}

fn chain_fixture() -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(CHAIN_SEED)
        .expect("§7 chain source extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &CHAIN_SEED[fence.body_range];
        if parse_decls(source)
            .expect("chain fixture parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::ExplicitDataDecl { name, .. } if name == "Vertex3"))
        {
            assert!(
                selected.replace(source).is_none(),
                "one three-vertex fixture"
            );
        }
    }
    selected.expect("three-vertex chain fixture exists")
}

const GENERIC_CLIENT: &str = r#"
import Data.Finite.Finite (Finite)
import Algorithm.Graphs.Dependency
  (Graph, Walk, edge, graph_reverse, graph_reverse_edge, walk_reverse,
   graph_reachable, graph_reachable_reverse)

fn reversed_graph_client (q : Type) (g : Graph q) : Graph q = graph_reverse q g

theorem flipped_edge_client (q : Type) (g : Graph q) (s : q) (t : q) :
  Equal Bool (edge q (graph_reverse q g) s t) (edge q g t s) =
  graph_reverse_edge q g s t

fn reversed_walk_client (q : Type) (g : Graph q) (s : q) (t : q)
  (walk : Walk q g s t) : Walk q (graph_reverse q g) t s =
  walk_reverse q g s t walk

theorem reversed_reachability_client (q : Type) (fq : Finite q)
  (g : Graph q) (s : q) (t : q) (target : q → Bool)
  (walk : Walk q g s t) :
  Equal Bool (target s) True
    → Equal Bool (graph_reachable q fq (graph_reverse q g) t target) True =
  graph_reachable_reverse q fq g s t target walk
"#;

/// MEASURED: all four public declarations root-load as package-owned
/// Transparent bodies and an independent generic client checks their exact
/// public types; the helper stays private and trusted GlobalId identities do
/// not change after provider loading. CLAIMED: §8 exports the ruled checked
/// forms without adding trust. THE GAP: provider trust is inherited, and a
/// finite value control alone cannot establish the generic open proof.
#[test]
fn generic_reversal_exports_are_owned_checked_and_trust_free() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
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
        .expect("§8 Graphs package checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§8 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§8 adds no trusted identities");
    for name in [
        "graph_reverse",
        "graph_reverse_edge",
        "walk_reverse",
        "graph_reachable_reverse",
    ] {
        let id = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to Dependency");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all four open public types check in an independent client");
    match env.elaborate_file(&format!("import {GRAPH} (walk_prepend)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{GRAPH}.walk_prepend"));
        }
        other => panic!("walk_prepend stays private: {other:?}"),
    }
}

const REVERSED_CHAIN_CLIENT: &str = r#"
import Algorithm.Graphs.Dependency
  (Graph, Walk, WalkHere, WalkStep, edge, graph_reverse,
   graph_reverse_edge, walk_reverse, graph_reachable, graph_reachable_reverse)

const chain_walk : Walk Vertex3 chain V0 V2 =
  WalkStep Vertex3 chain V0 V1 V2 Proved
    (WalkStep Vertex3 chain V0 V0 V1 Proved (WalkHere Vertex3 chain V0))

const reverse_walk : Walk Vertex3 (graph_reverse Vertex3 chain) V2 V0 =
  walk_reverse Vertex3 chain V0 V2 chain_walk

fn reverse_target (v : Vertex3) : Bool = is_v0 v

theorem reversed_edge_v2_v1 :
  Equal Bool (edge Vertex3 (graph_reverse Vertex3 chain) V2 V1) True = Proved

theorem reversed_edge_v1_v0 :
  Equal Bool (edge Vertex3 (graph_reverse Vertex3 chain) V1 V0) True = Proved

theorem forward_edge_v2_v1_absent : Equal Bool (edge Vertex3 chain V2 V1) False = Proved

theorem reversed_chain_value :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite (graph_reverse Vertex3 chain) V2 reverse_target)
    True = Proved

theorem forward_chain_value :
  Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 reverse_target) False = Proved

theorem reversed_chain_by_law :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite (graph_reverse Vertex3 chain) V2 reverse_target)
    True =
  graph_reachable_reverse
    Vertex3 vertex3_finite chain V0 V2 reverse_target chain_walk Proved
"#;

const FALSE_FORWARD_CLAIM: &str = r#"
import Algorithm.Graphs.Dependency (graph_reachable, graph_reverse)
theorem seed_neg :
  Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 reverse_target) True = Proved
"#;

// The forward and reversed edge tables are independently stated controls, not
// computed by the code under test. A swapped edge or a shortcut must redden.
fn chain_edge_controls() -> String {
    let mut source = String::new();
    for (index, from, to, forward, reverse) in [
        (0, "V0", "V0", "False", "False"),
        (1, "V0", "V1", "True", "False"),
        (2, "V0", "V2", "False", "False"),
        (3, "V1", "V0", "False", "True"),
        (4, "V1", "V1", "False", "False"),
        (5, "V1", "V2", "True", "False"),
        (6, "V2", "V0", "False", "False"),
        (7, "V2", "V1", "False", "True"),
        (8, "V2", "V2", "False", "False"),
    ] {
        source.push_str(&format!(
            "theorem forward_edge_{index} : Equal Bool (edge Vertex3 chain {from} {to}) {forward} = Proved\n"
        ));
        source.push_str(&format!(
            "theorem reverse_edge_{index} : Equal Bool (edge Vertex3 (graph_reverse Vertex3 chain) {from} {to}) {reverse} = Proved\n"
        ));
    }
    source
}

/// MEASURED: the §7 chain fixture and §8 client check a real two-edge walk,
/// every forward/reversed edge pair, the public reversal law and independent
/// True/False reachability results on the same finite graph and target. On a
/// fresh accepted prefix the original forward graph's contradictory True
/// claim is rejected at its own Proved. CLAIMED: reversing both graph and
/// walk establishes reverse dependency reachability, not reachability in g.
/// THE GAP: this finite counterexample does not replace the generic proof.
#[test]
fn two_edge_reverse_reaches_start_but_forward_converse_refuses() {
    let mut env = loaded_graph();
    env.elaborate_file(chain_fixture())
        .expect("three-vertex directed chain checks");
    env.elaborate_file(REVERSED_CHAIN_CLIENT)
        .expect("§8 two-edge reversal and reachability client checks");
    env.elaborate_file(&chain_edge_controls())
        .expect("all nine forward and reversed edge pairs check independently");

    let parsed = parse_decls(FALSE_FORWARD_CLAIM).expect("isolated false forward claim parses");
    let theorems = parsed
        .iter()
        .filter(|decl| matches!(decl, SourceDecl::TheoremDecl { .. }))
        .collect::<Vec<_>>();
    let [SourceDecl::TheoremDecl { name, span, .. }] = theorems.as_slice() else {
        panic!("one isolated false theorem");
    };
    assert_eq!(name, "seed_neg");
    let mut rejecting = loaded_graph();
    rejecting
        .elaborate_file(chain_fixture())
        .expect("fresh chain fixture checks before refusal");
    rejecting
        .elaborate_file(REVERSED_CHAIN_CLIENT)
        .expect("fresh accepted reversal client checks before refusal");
    match rejecting.elaborate_file(FALSE_FORWARD_CLAIM) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span: failure,
        }) if failure.start >= span.start && failure.end <= span.end => {}
        other => panic!("forward-chain false reachability must refuse at Proved: {other:?}"),
    }
}
