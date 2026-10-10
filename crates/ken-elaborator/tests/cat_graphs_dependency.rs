//! Checked graph walks, predicate reachability, and dependency order or cycle.
//!
//! Contract: spec/50-stdlib/62-graphs.md §1. The ten durable-invariant
//! runtime oracles are conformance/stdlib/graphs/seed-dependency.md.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

const GRAPH: &str = "Algorithm.Graphs.Dependency";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// Promise class: durable invariant. MEASURED: all 23 §1 declarations and
/// eight exported constructors load from real roots under their owner IDs;
/// seven distinct checked law clients inhabit the public signatures; private
/// graph-search helpers cannot be imported; and the exact trusted GlobalId
/// set agrees with its preloaded provider closure. CLAIMED: clients can use
/// exactly the graph's checked public evidence without a new trust entry.
/// THE GAP: generic law clients do not by themselves run the ten closed
/// behavioral oracles below.
#[test]
fn public_surface_generic_laws_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base environment");
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
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§1 graph package must roots-load with its declared providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "graph package adds no trusted GlobalIds");
    for name in [
        "Graph",
        "edge",
        "Walk",
        "Cycle",
        "ReachWitness",
        "graph_reachable",
        "graph_find_walk",
        "graph_reachable_complete",
        "graph_find_walk_some",
        "contains",
        "before",
        "no_duplicates",
        "all_vertices",
        "all_edges_forward",
        "TopoOrder",
        "ordered_vertices",
        "ordered_complete",
        "ordered_unique",
        "ordered_forward",
        "OrderOrCycle",
        "dependency_order_or_cycle",
        "contains_sound",
        "contains_complete",
    ] {
        let id = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&id), "{name} must be owned by graph package");
        if !matches!(
            name,
            "Graph" | "Walk" | "Cycle" | "ReachWitness" | "TopoOrder" | "OrderOrCycle"
        ) {
            assert!(
                matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
                "{name} must be a checked transparent declaration"
            );
        }
    }
    for (family, constructors) in [
        ("Graph", &["MkGraph"][..]),
        ("Walk", &["WalkHere", "WalkStep"][..]),
        ("Cycle", &["MkCycle"][..]),
        ("ReachWitness", &["MkReach"][..]),
        ("TopoOrder", &["MkTopoOrder"][..]),
        ("OrderOrCycle", &["HasOrder", "HasCycle"][..]),
    ] {
        let id = env.globals[&format!("{GRAPH}.{family}")];
        let checked = env.env.inductive(id).expect("public type is inductive");
        for constructor in constructors {
            let cid = env.globals[&format!("{GRAPH}.{constructor}")];
            assert!(
                checked.constructors.iter().any(|item| item.id == cid),
                "{constructor} must belong to {family}"
            );
        }
    }
    env.elaborate_file(
        r#"import Algorithm.Graphs.Dependency
             (Graph, MkGraph, edge, Walk, WalkHere, WalkStep, Cycle, MkCycle,
              ReachWitness, MkReach, graph_reachable, graph_find_walk,
              graph_reachable_complete, graph_find_walk_some,
              contains, before, no_duplicates, all_vertices, all_edges_forward,
              TopoOrder, MkTopoOrder, ordered_vertices, ordered_complete,
              ordered_unique, ordered_forward, OrderOrCycle, HasOrder, HasCycle,
              dependency_order_or_cycle, contains_sound, contains_complete)
           import Data.Finite.Finite (Finite)
           import Core.Classes.LawfulClasses (DecEq)
           import Data.Collections.Derived (list_elem)
           import Data.Sums.Combinators (is_some)
           theorem client_reachable_complete
             (q : Type) (fq : Finite q) (g : Graph q) (s : q) (t : q)
             (target : q → Bool) (walk : Walk q g s t) :
             Equal Bool (target t) True →
             Equal Bool (graph_reachable q fq g s target) True =
             graph_reachable_complete q fq g s t target walk
           theorem client_find_walk_some
             (q : Type) (fq : Finite q) (g : Graph q) (s : q)
             (target : q → Bool) :
             Equal Bool (graph_reachable q fq g s target) True →
             Equal Bool (is_some (ReachWitness q g s target)
               (graph_find_walk q fq g s target)) True =
             graph_find_walk_some q fq g s target
           theorem client_ordered_complete
             (q : Type) (d : DecEq q) (g : Graph q)
             (order : TopoOrder q d g) :
             all_vertices q d (ordered_vertices q d g order) =
             ordered_complete q d g order
           theorem client_ordered_unique
             (q : Type) (d : DecEq q) (g : Graph q)
             (order : TopoOrder q d g) :
             Equal Bool (no_duplicates q d (ordered_vertices q d g order)) True =
             ordered_unique q d g order
           theorem client_ordered_forward
             (q : Type) (d : DecEq q) (g : Graph q)
             (order : TopoOrder q d g) :
             all_edges_forward q d g (ordered_vertices q d g order) =
             ordered_forward q d g order
           theorem client_contains_sound
             (q : Type) (d : DecEq q) (x : q) (xs : List q) :
             Equal Bool (contains q d x xs) True → list_elem q x xs =
             contains_sound q d x xs
           theorem client_contains_complete
             (q : Type) (d : DecEq q) (x : q) (xs : List q) :
             list_elem q x xs → Equal Bool (contains q d x xs) True =
             contains_complete q d x xs"#,
    )
    .expect("all 23 selectively imported public names and seven §1 laws must elaborate");
    for private in [
        "graph_dfa",
        "graph_find_word",
        "graph_cycle_decision",
        "rank",
        "ordered_list",
        "dedup",
        "sort_perm_probe",
    ] {
        match env.elaborate_file(&format!("import {GRAPH} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{private}"));
            }
            Err(other) => panic!("{private} rejected for unrelated reason: {other:?}"),
            Ok(_) => panic!("{private} must not be publicly importable"),
        }
    }
}

const FIXTURE: &str = r#"
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, Walk, WalkHere, WalkStep, Cycle, MkCycle,
   ReachWitness, MkReach, graph_reachable, graph_find_walk,
   graph_reachable_complete, graph_find_walk_some,
   contains, before, no_duplicates, all_vertices, all_edges_forward,
   TopoOrder, MkTopoOrder, ordered_vertices, ordered_complete,
   ordered_unique, ordered_forward, OrderOrCycle, HasOrder, HasCycle,
   dependency_order_or_cycle, contains_sound, contains_complete)
import Data.Finite.Finite (Finite, MkFinite, bool_finite)
import Core.Classes.LawfulClasses (DecEq, DecEq_instance_Bool, bool_not, bool_and, bool_eq)
import Data.Collections.Derived (list_elem, list_elem_head, list_elem_later)
import Data.Sums.Combinators (is_some)

const forward : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ match v { True ↦ False; False ↦ True };
    False ↦ False
  })
const backward : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ False;
    False ↦ match v { True ↦ True; False ↦ False }
  })
const two_cycle : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ match v { True ↦ False; False ↦ True };
    False ↦ match v { True ↦ True; False ↦ False }
  })
const is_true : Bool → Bool = λv. v
const is_false : Bool → Bool = λv. bool_not v
const tt : List Bool = Cons Bool True (Cons Bool False (Nil Bool))
const ftf : List Bool = Cons Bool False (Cons Bool True (Cons Bool False (Nil Bool)))
const tff : List Bool = Cons Bool True (Cons Bool False (Cons Bool False (Nil Bool)))
const tft : List Bool = Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool)))
const single_true : List Bool = Cons Bool True (Nil Bool)
const duplicate_bool : Finite Bool =
  MkFinite Bool tft
    (λb. match b {
      True ↦ list_elem_head Bool True (Cons Bool False (Cons Bool True (Nil Bool)));
      False ↦ list_elem_later Bool False True
        (Cons Bool False (Cons Bool True (Nil Bool)))
        (list_elem_head Bool False (Cons Bool True (Nil Bool)))
    })

const forward_result : OrderOrCycle Bool DecEq_instance_Bool forward =
  dependency_order_or_cycle Bool bool_finite DecEq_instance_Bool forward
const backward_result : OrderOrCycle Bool DecEq_instance_Bool backward =
  dependency_order_or_cycle Bool bool_finite DecEq_instance_Bool backward
const cycle_result : OrderOrCycle Bool DecEq_instance_Bool two_cycle =
  dependency_order_or_cycle Bool bool_finite DecEq_instance_Bool two_cycle
const duplicate_result : OrderOrCycle Bool DecEq_instance_Bool forward =
  dependency_order_or_cycle Bool duplicate_bool DecEq_instance_Bool forward

fn order_list (g : Graph Bool)
  (decision : OrderOrCycle Bool DecEq_instance_Bool g) : List Bool =
  match decision {
    HasOrder order ↦ ordered_vertices Bool DecEq_instance_Bool g order;
    HasCycle cycle ↦ Nil Bool
  }
fn is_order (g : Graph Bool)
  (decision : OrderOrCycle Bool DecEq_instance_Bool g) : Bool =
  match decision { HasOrder order ↦ True; HasCycle cycle ↦ False }
fn is_cycle (g : Graph Bool)
  (decision : OrderOrCycle Bool DecEq_instance_Bool g) : Bool =
  match decision { HasOrder order ↦ False; HasCycle cycle ↦ True }
fn cycle_start (cy : Cycle Bool two_cycle) : Bool =
  match cy { MkCycle start next first rest ↦ start }
fn cycle_next (cy : Cycle Bool two_cycle) : Bool =
  match cy { MkCycle start next first rest ↦ next }
fn walk_steps (g : Graph Bool) (s : Bool) (t : Bool)
  (walk : Walk Bool g s t) : Nat =
  match walk {
    WalkHere ↦ Zero;
    WalkStep u v edge_proof prefix ↦ Suc (walk_steps g s u prefix)
  }
fn walk_edges_real (g : Graph Bool) (s : Bool) (t : Bool)
  (walk : Walk Bool g s t) : Bool =
  match walk {
    WalkHere ↦ True;
    WalkStep u v edge_proof prefix ↦
      bool_and (edge Bool g u v) (walk_edges_real g s u prefix)
  }
fn cycle_return_steps (cy : Cycle Bool two_cycle) : Nat =
  match cy { MkCycle start next first rest ↦
    walk_steps two_cycle next start rest }
fn cycle_return_edges_real (cy : Cycle Bool two_cycle) : Bool =
  match cy { MkCycle start next first rest ↦
    walk_edges_real two_cycle next start rest }
fn result_start (decision : OrderOrCycle Bool DecEq_instance_Bool two_cycle) : Bool =
  match decision { HasCycle cy ↦ cycle_start cy; HasOrder order ↦ False }
fn result_next (decision : OrderOrCycle Bool DecEq_instance_Bool two_cycle) : Bool =
  match decision { HasCycle cy ↦ cycle_next cy; HasOrder order ↦ False }
fn result_return_steps (decision : OrderOrCycle Bool DecEq_instance_Bool two_cycle) : Nat =
  match decision { HasCycle cy ↦ cycle_return_steps cy; HasOrder order ↦ Zero }
fn result_return_edges_real
  (decision : OrderOrCycle Bool DecEq_instance_Bool two_cycle) : Bool =
  match decision { HasCycle cy ↦ cycle_return_edges_real cy; HasOrder order ↦ False }
fn reach_endpoint (g : Graph Bool) (s : Bool) (target : Bool → Bool)
  (decision : Option (ReachWitness Bool g s target)) : Option Bool =
  match decision {
    None ↦ None Bool;
    Some witness ↦ match witness {
      MkReach end hit walk ↦ Some Bool end
    }
  }
fn reach_steps (g : Graph Bool) (s : Bool) (target : Bool → Bool)
  (decision : Option (ReachWitness Bool g s target)) : Nat =
  match decision {
    None ↦ Zero;
    Some witness ↦ match witness {
      MkReach end hit walk ↦ walk_steps g s end walk
    }
  }
fn reach_edges_real (g : Graph Bool) (s : Bool) (target : Bool → Bool)
  (decision : Option (ReachWitness Bool g s target)) : Bool =
  match decision {
    None ↦ False;
    Some witness ↦ match witness {
      MkReach end hit walk ↦ walk_edges_real g s end walk
    }
  }

data EmptyVertex : Type where {}
fn empty_eq (x : EmptyVertex) (y : EmptyVertex) : Bool = match x {}
theorem empty_eq_sound (x : EmptyVertex) (y : EmptyVertex) :
  Equal Bool (empty_eq x y) True → Equal EmptyVertex x y = match x {}
theorem empty_eq_complete (x : EmptyVertex) (y : EmptyVertex) :
  Equal EmptyVertex x y → Equal Bool (empty_eq x y) True = match x {}
instance DecEq EmptyVertex {
  eq = empty_eq;
  sound = empty_eq_sound;
  complete = empty_eq_complete
}
const empty_vertices : Finite EmptyVertex =
  MkFinite EmptyVertex (Nil EmptyVertex) (λv. match v {})
const empty_graph : Graph EmptyVertex = MkGraph EmptyVertex (λv. match v {})
const empty_result : OrderOrCycle EmptyVertex DecEq_instance_EmptyVertex empty_graph =
  dependency_order_or_cycle EmptyVertex empty_vertices
    DecEq_instance_EmptyVertex empty_graph
fn empty_result_list
  (decision : OrderOrCycle EmptyVertex DecEq_instance_EmptyVertex empty_graph)
  : List EmptyVertex =
  match decision {
    HasOrder order ↦ ordered_vertices EmptyVertex DecEq_instance_EmptyVertex empty_graph order;
    HasCycle cycle ↦ Nil EmptyVertex
  }
fn empty_is_order
  (decision : OrderOrCycle EmptyVertex DecEq_instance_EmptyVertex empty_graph) : Bool =
  match decision { HasOrder order ↦ True; HasCycle cycle ↦ False }
"#;

fn fixture() -> (ElabEnv, EvalStore) {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("real graph package and its providers must roots-load");
    env.elaborate_file(FIXTURE)
        .expect("all §1 runtime fixtures must elaborate without trust");
    (env, EvalStore::new())
}

fn observe(env: &mut ElabEnv, store: &mut EvalStore, name: &str, ty: &str, expr: &str) -> EvalVal {
    let id = env
        .elaborate_decl(&format!("const {name} : {ty} = {expr}"))
        .unwrap_or_else(|error| panic!("{name} expression must elaborate: {error:?}"));
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        other => panic!("{name} must elaborate transparently, got {other:?}"),
    }
}

fn boolean(env: &ElabEnv, val: &EvalVal) -> bool {
    match val {
        EvalVal::Ctor { id, args, .. }
            if *id == env.numeric_env.bool_true_id && args.is_empty() =>
        {
            true
        }
        EvalVal::Ctor { id, args, .. }
            if *id == env.numeric_env.bool_false_id && args.is_empty() =>
        {
            false
        }
        EvalVal::Bool(b) => *b,
        other => panic!("expected runtime Bool, got {other:?}"),
    }
}

// The reference interpreter strictly evaluates each field of a proof-carrying
// DecEq dictionary and can yield Unknown even when a closed public expression
// is kernel-reducible. For that pure boundary, demand a checked closed equality
// instead of accepting Unknown or treating a test-computed value as an oracle.
fn truth(env: &mut ElabEnv, _store: &mut EvalStore, name: &str, expr: &str, expected: bool) {
    let result = if expected { "True" } else { "False" };
    env.elaborate_file(&format!(
        "theorem seed_{name} : Equal Bool ({expr}) {result} = Proved"
    ))
    .unwrap_or_else(|error| panic!("{name}: kernel must reduce to {result}: {error:?}"));
}

fn runtime_truth(env: &mut ElabEnv, store: &mut EvalStore, name: &str, expr: &str, expected: bool) {
    let val = observe(env, store, name, "Bool", expr);
    assert_eq!(boolean(env, &val), expected, "{name}: reference evaluation");
}

fn list(env: &mut ElabEnv, _store: &mut EvalStore, name: &str, expr: &str, expected: &[bool]) {
    let mut target = "Nil Bool".to_owned();
    for bit in expected.iter().rev() {
        target = format!(
            "Cons Bool {} ({target})",
            if *bit { "True" } else { "False" }
        );
    }
    env.elaborate_file(&format!(
        "theorem seed_{name} : Equal (List Bool) ({expr}) ({target}) = Refl"
    ))
    .unwrap_or_else(|error| panic!("{name}: checked exact order {target}: {error:?}"));
}

fn number(env: &mut ElabEnv, _store: &mut EvalStore, name: &str, expr: &str, expected: usize) {
    let mut target = "Zero".to_owned();
    for _ in 0..expected {
        target = format!("Suc ({target})");
    }
    env.elaborate_file(&format!(
        "theorem seed_{name} : Equal Nat ({expr}) ({target}) = Proved"
    ))
    .unwrap_or_else(|error| panic!("{name}: checked walk length {target}: {error:?}"));
}

fn has_tag(
    env: &mut ElabEnv,
    store: &mut EvalStore,
    name: &str,
    graph: &str,
    expr: &str,
    tag: &str,
) {
    let predicate = match tag {
        "HasOrder" => "is_order",
        "HasCycle" => "is_cycle",
        other => panic!("unhandled graph choice {other}"),
    };
    truth(
        env,
        store,
        name,
        &format!("{predicate} {graph} {expr}"),
        true,
    );
}

fn option_endpoint(
    env: &mut ElabEnv,
    _store: &mut EvalStore,
    name: &str,
    expr: &str,
    expected: Option<bool>,
) {
    let target = match expected {
        Some(true) => "Some Bool True",
        Some(false) => "Some Bool False",
        None => "None Bool",
    };
    env.elaborate_file(&format!(
        "theorem seed_{name} : Equal (Option Bool) ({expr}) ({target}) = Proved"
    ))
    .unwrap_or_else(|error| panic!("{name}: checked endpoint {target}: {error:?}"));
}

/// Promise class: durable invariant. MEASURED: forward graph returns an order
/// with both Bool vertices once; the actual list puts its real edge forward,
/// the reverse edge and reverse order are false. CLAIMED: source precedes
/// destination. THE GAP: checked generic order fields prove arbitrary edges.
#[test]
fn seed_forward_edge_has_complete_order() {
    let (mut env, mut store) = fixture();
    has_tag(
        &mut env,
        &mut store,
        "forward_tag",
        "forward",
        "forward_result",
        "HasOrder",
    );
    list(
        &mut env,
        &mut store,
        "forward_list",
        "order_list forward forward_result",
        &[true, false],
    );
    for (name, expr, want) in [
        (
            "f_contains_true",
            "contains Bool DecEq_instance_Bool True (order_list forward forward_result)",
            true,
        ),
        (
            "f_contains_false",
            "contains Bool DecEq_instance_Bool False (order_list forward forward_result)",
            true,
        ),
        (
            "f_nodup",
            "no_duplicates Bool DecEq_instance_Bool (order_list forward forward_result)",
            true,
        ),
        (
            "f_before",
            "before Bool DecEq_instance_Bool True False (order_list forward forward_result)",
            true,
        ),
        (
            "f_reverse_before",
            "before Bool DecEq_instance_Bool False True (order_list forward forward_result)",
            false,
        ),
        ("f_edge", "edge Bool forward True False", true),
        ("f_reverse_edge", "edge Bool forward False True", false),
    ] {
        truth(&mut env, &mut store, name, expr, want);
    }
}

/// Promise class: durable invariant. MEASURED: only the edge orientation
/// changes on the same Bool carrier/finite certificate; output lists and
/// corresponding `before` outcomes reverse. CLAIMED: order is computed from
/// adjacency, not a hardcoded preference. THE GAP: no tie-order is specified.
#[test]
fn seed_reversing_only_edge_reverses_order() {
    let (mut env, mut store) = fixture();
    has_tag(
        &mut env,
        &mut store,
        "backward_tag",
        "backward",
        "backward_result",
        "HasOrder",
    );
    list(
        &mut env,
        &mut store,
        "forward_again",
        "order_list forward forward_result",
        &[true, false],
    );
    list(
        &mut env,
        &mut store,
        "backward_list",
        "order_list backward backward_result",
        &[false, true],
    );
    for (name, expr, expected) in [
        (
            "b_before_false_true",
            "before Bool DecEq_instance_Bool False True (order_list backward backward_result)",
            true,
        ),
        (
            "b_before_true_false",
            "before Bool DecEq_instance_Bool True False (order_list backward backward_result)",
            false,
        ),
        (
            "b_nodup",
            "no_duplicates Bool DecEq_instance_Bool (order_list backward backward_result)",
            true,
        ),
        ("b_edge", "edge Bool backward False True", true),
        ("b_reverse_edge", "edge Bool backward True False", false),
    ] {
        truth(&mut env, &mut store, name, expr, expected);
    }
}

/// Promise class: durable invariant. MEASURED: the two-edge graph returns a
/// real cycle, with distinct endpoints, true first/return edges, and no
/// self-loop on either vertex. CLAIMED: a cycle does not fabricate an edge or
/// mistake reflexive WalkHere alone for a nonempty cycle. THE GAP: checked
/// Walk/Cycle constructor fields and generic search carry the universal case.
#[test]
fn seed_two_vertex_cycle_has_real_witness() {
    let (mut env, mut store) = fixture();
    has_tag(
        &mut env,
        &mut store,
        "cycle_tag",
        "two_cycle",
        "cycle_result",
        "HasCycle",
    );
    truth(
        &mut env,
        &mut store,
        "cycle_distinct",
        "bool_not (bool_eq (result_start cycle_result) (result_next cycle_result))",
        true,
    );
    truth(
        &mut env,
        &mut store,
        "cycle_first_edge",
        "edge Bool two_cycle (result_start cycle_result) (result_next cycle_result)",
        true,
    );
    truth(
        &mut env,
        &mut store,
        "cycle_return_edges",
        "result_return_edges_real cycle_result",
        true,
    );
    number(
        &mut env,
        &mut store,
        "cycle_return_length",
        "result_return_steps cycle_result",
        1,
    );
    truth(
        &mut env,
        &mut store,
        "no_true_loop",
        "edge Bool two_cycle True True",
        false,
    );
    truth(
        &mut env,
        &mut store,
        "no_false_loop",
        "edge Bool two_cycle False False",
        false,
    );
}

/// Promise class: durable invariant. MEASURED: checked empty-carrier evidence
/// produces HasOrder, no listed vertices and no fabricated cycle. CLAIMED:
/// an empty graph is orderable without assuming a first listed vertex.
/// THE GAP: no start vertex exists here, so reachability is not queried.
#[test]
fn seed_zero_vertex_graph_has_empty_order() {
    let (mut env, mut store) = fixture();
    truth(
        &mut env,
        &mut store,
        "empty_tag",
        "empty_is_order empty_result",
        true,
    );
    env.elaborate_file(
        "theorem seed_empty_list : Equal (List EmptyVertex) \
         (empty_result_list empty_result) (Nil EmptyVertex) = Proved",
    )
    .expect("empty order list must kernel-normalize to Nil");
}

/// Promise class: durable invariant. MEASURED: a coverage certificate listing
/// True twice yields the same distinct forward order as bool_finite.
/// CLAIMED: certificate multiplicity cannot leak into an order or remove
/// vertices. THE GAP: generic dedup and sort proofs carry every carrier.
#[test]
fn seed_duplicate_enumeration_is_not_duplicate_order() {
    let (mut env, mut store) = fixture();
    has_tag(
        &mut env,
        &mut store,
        "duplicate_tag",
        "forward",
        "duplicate_result",
        "HasOrder",
    );
    list(
        &mut env,
        &mut store,
        "duplicate_list",
        "order_list forward duplicate_result",
        &[true, false],
    );
    for (name, expr) in [
        (
            "dup_contains_true",
            "contains Bool DecEq_instance_Bool True (order_list forward duplicate_result)",
        ),
        (
            "dup_contains_false",
            "contains Bool DecEq_instance_Bool False (order_list forward duplicate_result)",
        ),
        (
            "dup_no_duplicates",
            "no_duplicates Bool DecEq_instance_Bool (order_list forward duplicate_result)",
        ),
        (
            "dup_before",
            "before Bool DecEq_instance_Bool True False (order_list forward duplicate_result)",
        ),
    ] {
        truth(&mut env, &mut store, name, expr, true);
    }
}

/// Promise class: durable invariant. MEASURED: two lists with the same
/// multiset and opposite first two positions produce opposite `before` for
/// True/False. CLAIMED: `before` uses the first of either vertex, including on
/// arbitrary lists with duplicates. THE GAP: one Bool pair is not a proof.
#[test]
fn seed_before_first_of_either_on_duplicate_input() {
    let (mut env, mut store) = fixture();
    for (name, expr, want) in [
        (
            "first_y_false",
            "before Bool DecEq_instance_Bool True False ftf",
            false,
        ),
        (
            "first_x_true",
            "before Bool DecEq_instance_Bool True False tff",
            true,
        ),
        (
            "first_false_true",
            "before Bool DecEq_instance_Bool False True ftf",
            true,
        ),
        (
            "same_vertex",
            "before Bool DecEq_instance_Bool True True tff",
            false,
        ),
    ] {
        truth(&mut env, &mut store, name, expr, want);
    }
}

/// Promise class: durable invariant. MEASURED: a repeated input is rejected,
/// while deleting only its final repeated vertex is accepted. CLAIMED: direct
/// `no_duplicates` applies to its input, not merely an order's output.
/// THE GAP: returned orders also carry the checked uniqueness proof field.
#[test]
fn seed_no_duplicates_rejects_duplicate_input() {
    let (mut env, mut store) = fixture();
    truth(
        &mut env,
        &mut store,
        "repeated_input",
        "no_duplicates Bool DecEq_instance_Bool tft",
        false,
    );
    truth(
        &mut env,
        &mut store,
        "distinct_input",
        "no_duplicates Bool DecEq_instance_Bool tt",
        true,
    );
}

/// Promise class: durable invariant. MEASURED: a present and an absent Bool
/// on the SAME nonempty list have opposite contains results. CLAIMED:
/// membership is not always true; the complete-order field remains honest.
/// THE GAP: contains_sound/complete cover arbitrary lists and DecEq values.
#[test]
fn seed_contains_rejects_missing_element() {
    let (mut env, mut store) = fixture();
    truth(
        &mut env,
        &mut store,
        "missing_false",
        "contains Bool DecEq_instance_Bool False single_true",
        false,
    );
    truth(
        &mut env,
        &mut store,
        "present_true",
        "contains Bool DecEq_instance_Bool True single_true",
        true,
    );
}

/// Promise class: durable invariant. MEASURED: the true edge changes an
/// initially false target into True, with a Some endpoint False, a real
/// nonempty walk, and the checked generic completeness and Some laws have
/// a closed nonvacuous client. CLAIMED: DFA stutters are not graph edges.
/// THE GAP: public Walk/MkReach constructors enforce arbitrary witness steps.
#[test]
fn seed_reachable_edge_has_walk() {
    let (mut env, mut store) = fixture();
    runtime_truth(
        &mut env,
        &mut store,
        "edge_reach",
        "graph_reachable Bool bool_finite forward True is_false",
        true,
    );
    truth(
        &mut env,
        &mut store,
        "target_false_at_start",
        "is_false True",
        false,
    );
    truth(
        &mut env,
        &mut store,
        "target_true_at_end",
        "is_false False",
        true,
    );
    truth(
        &mut env,
        &mut store,
        "real_edge",
        "edge Bool forward True False",
        true,
    );
    option_endpoint(&mut env, &mut store, "found_false",
        "reach_endpoint forward True is_false (graph_find_walk Bool bool_finite forward True is_false)", Some(false));
    number(&mut env, &mut store, "walk_nonempty",
        "reach_steps forward True is_false (graph_find_walk Bool bool_finite forward True is_false)", 1);
    truth(&mut env, &mut store, "walk_edges",
        "reach_edges_real forward True is_false (graph_find_walk Bool bool_finite forward True is_false)", true);
    env.elaborate_file(
        r#"
      const real_walk : Walk Bool forward True False =
        WalkStep Bool forward True True False Proved (WalkHere Bool forward True)
      theorem nonvacuous_complete :
        Equal Bool (graph_reachable Bool bool_finite forward True is_false) True =
        graph_reachable_complete Bool bool_finite forward True False is_false
          real_walk Proved
      theorem nonvacuous_some :
        Equal Bool
          (is_some (ReachWitness Bool forward True is_false)
            (graph_find_walk Bool bool_finite forward True is_false)) True =
        graph_find_walk_some Bool bool_finite forward True is_false nonvacuous_complete
      theorem present_member_sound :
        list_elem Bool True single_true =
        contains_sound Bool DecEq_instance_Bool True single_true Proved
      theorem present_member_complete :
        Equal Bool (contains Bool DecEq_instance_Bool False tt) True =
        contains_complete Bool DecEq_instance_Bool False tt
          (list_elem_later Bool False True (Cons Bool False (Nil Bool))
            (list_elem_head Bool False (Nil Bool)))
    "#,
    )
    .expect("seven generic laws must include closed positive reach/contains instances");
}

/// Promise class: durable invariant. MEASURED: changing only target on the
/// same graph/start changes False+None to True+Some(WalkHere), with no true
/// outgoing edge. CLAIMED: reflexive walk acceptance is not a fabricated
/// edge or one-vertex cycle. THE GAP: the generic two laws are checked above.
#[test]
fn seed_unreachable_reverse_and_reflexive_control() {
    let (mut env, mut store) = fixture();
    truth(
        &mut env,
        &mut store,
        "reverse_absent_edge",
        "edge Bool forward False True",
        false,
    );
    runtime_truth(
        &mut env,
        &mut store,
        "reverse_unreachable",
        "graph_reachable Bool bool_finite forward False is_true",
        false,
    );
    option_endpoint(&mut env, &mut store, "reverse_no_witness",
        "reach_endpoint forward False is_true (graph_find_walk Bool bool_finite forward False is_true)", None);
    runtime_truth(
        &mut env,
        &mut store,
        "reflexive_reachable",
        "graph_reachable Bool bool_finite forward False is_false",
        true,
    );
    option_endpoint(&mut env, &mut store, "reflexive_witness",
        "reach_endpoint forward False is_false (graph_find_walk Bool bool_finite forward False is_false)", Some(false));
    number(&mut env, &mut store, "reflexive_zero_step",
        "reach_steps forward False is_false (graph_find_walk Bool bool_finite forward False is_false)", 0);
    truth(&mut env, &mut store, "reflexive_edges_vacuously_real",
        "reach_edges_real forward False is_false (graph_find_walk Bool bool_finite forward False is_false)", true);
}
