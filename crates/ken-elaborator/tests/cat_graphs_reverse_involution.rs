//! Graph and walk reversal involution: spec/50-stdlib/62-graphs.md §10.
//! Promise class: durable invariant. The generic clients use the two public
//! theorems without extra premises; a directed chain separates one reversal
//! from two reversals and exercises the two-edge walk.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::Decl;

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-reverse-involution.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§10 Graphs Dependency roots-loads");
    env
}

fn seed_source_containing(theorem: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§10 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§10 source fence parses")
            .iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem))
        {
            assert!(
                selected.replace(source).is_none(),
                "one source fence declares {theorem}"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("§10 seed declares {theorem}"))
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, graph_reverse, walk_reverse,
   graph_reverse_involutive, walk_reverse_involutive)

theorem double_graph_generic (q : Type) (g : Graph q) :
  Equal (Graph q) (graph_reverse q (graph_reverse q g)) g =
  graph_reverse_involutive q g

theorem double_walk_generic
    (q : Type) (step : q → q → Bool) (s : q) (t : q)
    (walk : Walk q (MkGraph q step) s t) :
  Equal
    (Walk q (MkGraph q step) s t)
    (walk_reverse q (graph_reverse q (MkGraph q step)) t s
      (walk_reverse q (MkGraph q step) s t walk))
    walk =
  walk_reverse_involutive q step s t walk
"#;

/// MEASURED: both module-owned public proof IDs are Transparent, and their
/// exact universal signatures are consumed by independent open clients. The
/// module adds no trusted GlobalId relative to its preloaded providers; three
/// proof helpers remain unimportable. CLAIMED: §10's two law exports are
/// premise-free checked laws, not new sources of trust or public scaffolding.
/// THE GAP: the trust set does not certify inherited providers, and this test
/// does not run the official conformance seed or cover all future clients.
#[test]
fn generic_involution_laws_are_owned_checked_private_and_trust_free() {
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
        .expect("§10 Graphs package checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§10 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§10 adds no trusted GlobalIds");
    for name in ["graph_reverse_involutive", "walk_reverse_involutive"] {
        let id = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to Dependency");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic clients check both public law signatures");
    for private in [
        "graph_reverse_involutive_at",
        "walk_reverse_involutive_here",
        "walk_reverse_prepend",
    ] {
        match env.elaborate_file(&format!("import {GRAPH} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{private}"));
            }
            other => panic!("{private} stays private: {other:?}"),
        }
    }
}

const CHAIN_CLIENT: &str = r#"
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, WalkHere, WalkStep, graph_reverse, walk_reverse,
   graph_reverse_involutive, walk_reverse_involutive)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

fn chain_step (u : Vertex3) (v : Vertex3) : Bool =
  match u {
    V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ False };
    V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ True };
    V2 ↦ False
  }

const chain : Graph Vertex3 = MkGraph Vertex3 chain_step

const chain_walk : Walk Vertex3 chain V0 V2 =
  WalkStep Vertex3 chain V0 V1 V2 Proved
    (WalkStep Vertex3 chain V0 V0 V1 Proved (WalkHere Vertex3 chain V0))

theorem double_graph_by_law :
  Equal (Graph Vertex3) (graph_reverse Vertex3 (graph_reverse Vertex3 chain)) chain =
  graph_reverse_involutive Vertex3 chain

theorem double_graph_same_form :
  Equal (Graph Vertex3) (graph_reverse Vertex3 (graph_reverse Vertex3 chain)) chain =
  Refl

theorem double_walk_by_law :
  Equal
    (Walk Vertex3 chain V0 V2)
    (walk_reverse Vertex3 (graph_reverse Vertex3 chain) V2 V0
      (walk_reverse Vertex3 chain V0 V2 chain_walk))
    chain_walk =
  walk_reverse_involutive Vertex3 chain_step V0 V2 chain_walk

theorem double_walk_same_form :
  Equal
    (Walk Vertex3 chain V0 V2)
    (walk_reverse Vertex3 (graph_reverse Vertex3 chain) V2 V0
      (walk_reverse Vertex3 chain V0 V2 chain_walk))
    chain_walk =
  Refl
"#;

/// MEASURED: the CV-owned seed's complete accepted source block elaborates
/// through both public laws and both double-Refl controls. After a fresh
/// accepted prefix, its isolated one-reversal claim refuses at its own Refl.
/// CLAIMED: the seed's accepted/refused Ken blocks are buildable against this
/// candidate, and its discriminator selects graph direction rather than a
/// malformed prelude. THE GAP: this is package-test elaboration of selected
/// seed blocks, not execution through the official conformance runner.
#[test]
fn seed_accepted_client_checks_and_one_reversal_refuses_at_refl() {
    let accepted_source = seed_source_containing("graph_reverse_law_client");
    let refused_source = seed_source_containing("seed_neg");
    let mut accepted = graph_env();
    accepted
        .elaborate_file(accepted_source)
        .expect("CV seed's laws and double-Refl controls check");
    let parsed = parse_decls(refused_source).expect("isolated seed refusal parses");
    let [SourceDecl::TheoremDecl { name, body, .. }] = parsed.as_slice() else {
        panic!("seed refusal is one theorem");
    };
    assert_eq!(name, "seed_neg");
    let claim_span = body.span();
    let mut rejecting = graph_env();
    rejecting
        .elaborate_file(accepted_source)
        .expect("fresh accepted CV seed prefix before refusal");
    match rejecting.elaborate_file(refused_source) {
        Err(ElabError::TypeMismatch {
            span: failure,
            reason,
        }) if reason == "Refl: the two sides of the goal are not convertible"
            && claim_span.start <= failure.start
            && failure.start < failure.end
            && failure.end <= claim_span.end => {}
        other => panic!("seed's one-reversal Refl must refuse: {other:?}"),
    }
}

const FALSE_ONCE: &str = r#"
theorem false_one_reverse :
  Equal (Graph Vertex3) (graph_reverse Vertex3 chain) chain = Refl
"#;

/// MEASURED: a separately written three-vertex chain has only forward edges
/// V0→V1→V2. Double reversal of this graph and of its two-edge walk checks
/// through the public theorems; the same-form closed double-Refl controls
/// check; one graph reversal refuses at its own Refl on a fresh accepted
/// prefix. CLAIMED: reversing twice returns the original directed graph and
/// the original walk data; one reversal is not an involution witness.
/// THE GAP: closed normalization cannot replace the generic clients above,
/// and candidate-local elaboration is not official conformance execution.
#[test]
fn chain_double_reversal_accepts_and_single_reversal_refuses_at_refl() {
    let mut accepted = graph_env();
    accepted
        .elaborate_file(CHAIN_CLIENT)
        .expect("two public law uses and same-form double-Refl controls check");
    let parsed = parse_decls(FALSE_ONCE).expect("isolated false claim parses");
    let [SourceDecl::TheoremDecl { name, body, .. }] = parsed.as_slice() else {
        panic!("isolated claim is one theorem");
    };
    assert_eq!(name, "false_one_reverse");
    let claim_span = body.span();
    let mut rejecting = graph_env();
    rejecting
        .elaborate_file(CHAIN_CLIENT)
        .expect("fresh accepted chain client before refusal");
    match rejecting.elaborate_file(FALSE_ONCE) {
        Err(ElabError::TypeMismatch {
            span: failure,
            reason,
        }) if reason == "Refl: the two sides of the goal are not convertible"
            && claim_span.start <= failure.start
            && failure.start < failure.end
            && failure.end <= claim_span.end => {}
        other => panic!("false one-reversal claim must refuse at Refl: {other:?}"),
    }
}
