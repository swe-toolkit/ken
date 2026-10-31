//! Graph-walk reversal across composition: spec/50-stdlib/62-graphs.md §9.
//! Promise class: durable invariant. An independent generic client consumes
//! the premise-free law, and a shortcut supplies distinct same-endpoint walks.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::Decl;

const GRAPH: &str = "Algorithm.Graphs.Dependency";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§9 Graphs Dependency roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Graphs.Dependency
  (Graph, Walk, walk_append, graph_reverse, walk_reverse, walk_reverse_append)

theorem composed_reverse_generic
      (q : Type)
      (g : Graph q)
      (s : q)
      (t : q)
      (u : q)
      (first : Walk q g s t)
      (second : Walk q g t u)
    : Equal
        (Walk q (graph_reverse q g) u s)
        (walk_reverse q g s u (walk_append q g s t u first second))
        (walk_append q (graph_reverse q g) u t s
          (walk_reverse q g t u second)
          (walk_reverse q g s t first)) =
  walk_reverse_append q g s t u first second
"#;

/// MEASURED: a provider-loaded Graphs module owns a Transparent public law;
/// an independent open client consumes its exact full-walk proposition and
/// both prepend helpers refuse outsider imports. The GlobalId trust set is
/// unchanged by loading the Graphs module. CLAIMED: the §9 theorem proves
/// reversal of composition for arbitrary walks without adding assumptions.
/// THE GAP: imported providers keep their existing trust; one generic client
/// cannot measure every later consumer's behavior.
#[test]
fn generic_composition_reverse_is_owned_checked_private_and_trust_free() {
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
        .expect("§9 Graphs package checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§9 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§9 adds no trusted identities");
    let id = env.globals[&format!("{GRAPH}.walk_reverse_append")];
    assert!(owned.contains(&id), "law belongs to Dependency");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic open walk equality consumes the §9 law");
    for private in ["walk_prepend", "walk_prepend_append"] {
        match env.elaborate_file(&format!("import {GRAPH} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{private}"));
            }
            other => panic!("{private} stays private: {other:?}"),
        }
    }
}

const SHORTCUT_CLIENT: &str = r#"
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, WalkHere, WalkStep, walk_append, graph_reverse,
   walk_reverse, walk_reverse_append)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

const shortcut : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ True
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ True
            };
          V2 ↦ False
        })

const walk_01 : Walk Vertex3 shortcut V0 V1 =
  WalkStep Vertex3 shortcut V0 V0 V1 Proved (WalkHere Vertex3 shortcut V0)

const walk_12 : Walk Vertex3 shortcut V1 V2 =
  WalkStep Vertex3 shortcut V1 V1 V2 Proved (WalkHere Vertex3 shortcut V1)

const via_v1 : Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0 =
  WalkStep Vertex3 (graph_reverse Vertex3 shortcut) V2 V1 V0 Proved
    (WalkStep Vertex3 (graph_reverse Vertex3 shortcut) V2 V2 V1 Proved
      (WalkHere Vertex3 (graph_reverse Vertex3 shortcut) V2))

const direct : Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0 =
  WalkStep Vertex3 (graph_reverse Vertex3 shortcut) V2 V2 V0 Proved
    (WalkHere Vertex3 (graph_reverse Vertex3 shortcut) V2)

theorem reversed_from_two_parts :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_reverse Vertex3 shortcut V0 V2
      (walk_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12))
    (walk_append Vertex3 (graph_reverse Vertex3 shortcut) V2 V1 V0
      (walk_reverse Vertex3 shortcut V1 V2 walk_12)
      (walk_reverse Vertex3 shortcut V0 V1 walk_01)) =
  walk_reverse_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12

theorem reversed_is_via_v1 :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_reverse Vertex3 shortcut V0 V2
      (walk_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12))
    via_v1 =
  Refl

theorem composed_is_via_v1 :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_append Vertex3 (graph_reverse Vertex3 shortcut) V2 V1 V0
      (walk_reverse Vertex3 shortcut V1 V2 walk_12)
      (walk_reverse Vertex3 shortcut V0 V1 walk_01))
    via_v1 =
  Refl
"#;

const FALSE_DIRECT: &str = r#"
theorem wrong_direct :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_reverse Vertex3 shortcut V0 V2
      (walk_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12))
    direct =
  Refl
"#;

/// MEASURED: the law and two independent `Refl` equalities accept on a
/// checked two-edge route, while the separately well-typed direct shortcut
/// walk has the same endpoints and its isolated false equality rejects at
/// its own `Refl`. CLAIMED: reversing the joined walk preserves route data,
/// not merely endpoint reachability. THE GAP: this finite graph cannot
/// replace the generic theorem, and no official CV seed is executed here.
#[test]
fn shortcut_separates_distinct_same_endpoint_reversed_walks() {
    let mut accepted = graph_env();
    accepted
        .elaborate_file(SHORTCUT_CLIENT)
        .expect("two-edge and direct walks, law, and same-form true Refl checks");
    let parsed = parse_decls(FALSE_DIRECT).expect("isolated false claim parses");
    let [SourceDecl::TheoremDecl { name, body, .. }] = parsed.as_slice() else {
        panic!("isolated claim must be one theorem");
    };
    assert_eq!(name, "wrong_direct");
    let claim_span = body.span();
    let mut rejecting = graph_env();
    rejecting
        .elaborate_file(SHORTCUT_CLIENT)
        .expect("fresh accepted shortcut scope checks before refusal");
    match rejecting.elaborate_file(FALSE_DIRECT) {
        Err(ElabError::TypeMismatch {
            span: failure,
            reason,
        }) if reason == "Refl: the two sides of the goal are not convertible"
            && claim_span.start <= failure.start
            && failure.start < failure.end
            && failure.end <= claim_span.end => {}
        other => panic!("false direct-walk equality must refuse at Refl: {other:?}"),
    }
}
