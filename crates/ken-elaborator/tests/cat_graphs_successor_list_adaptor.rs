//! Successor-list graph adaptor: spec/50-stdlib/62-graphs.md §14.
//! Promise class: durable invariant. A generic public client checks the two
//! proof directions; a closed three-vertex client separates listed edges from
//! absent edges at different sources without treating a list as a walk.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const GRAPH: &str = "Algorithm.Graphs.Dependency";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_graph() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§14 Graphs Dependency roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (DecEq)
import Data.Collections.Derived (list_elem)
import Algorithm.Graphs.Dependency
  (Graph, edge, graph_from_successors,
   graph_from_successors_sound, graph_from_successors_complete)

fn open_graph (q : Type) (d : DecEq q) (successors : q → List q) : Graph q =
  graph_from_successors q d successors

theorem open_sound
    (q : Type) (d : DecEq q) (successors : q → List q) (s : q) (t : q) :
    Equal Bool (edge q (graph_from_successors q d successors) s t) True
      → list_elem q t (successors s) =
  graph_from_successors_sound q d successors s t

theorem open_complete
    (q : Type) (d : DecEq q) (successors : q → List q) (s : q) (t : q) :
    list_elem q t (successors s)
      → Equal Bool (edge q (graph_from_successors q d successors) s t) True =
  graph_from_successors_complete q d successors s t
"#;

/// MEASURED: an independently elaborated client selectively imports all three
/// public names, consumes both generic checked laws at their full types, and
/// the package owns the transparent declarations without a trusted GlobalId
/// delta relative to its preloaded providers. CLAIMED: the adaptor is a
/// public, generic, trust-free checked operation rather than a closed example.
/// THE GAP: this does not prove how the operation computes for any concrete
/// list; the separate closed client tests that boundary.
#[test]
fn generic_successor_list_laws_are_public_owned_and_trust_free() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
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
        .expect("the checked Graphs package roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§14 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "the adaptor adds no trusted identities");
    for name in [
        "graph_from_successors",
        "graph_from_successors_sound",
        "graph_from_successors_complete",
    ] {
        let id = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&id), "{name} is owned by Dependency");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic public client checks");
}

const VERTICES_AND_LISTS: &str = r#"
import Core.Classes.LawfulClasses (DecEq)
import Data.Collections.Derived (list_elem, list_elem_head)
import Algorithm.Graphs.Dependency
  (Graph, edge, graph_from_successors,
   graph_from_successors_sound, graph_from_successors_complete)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

fn vertex3_eq (x : Vertex3) (y : Vertex3) : Bool =
  match x {
    V0 ↦ match y { V0 ↦ True; V1 ↦ False; V2 ↦ False };
    V1 ↦ match y { V0 ↦ False; V1 ↦ True; V2 ↦ False };
    V2 ↦ match y { V0 ↦ False; V1 ↦ False; V2 ↦ True }
  }

theorem vertex3_eq_sound (x : Vertex3) (y : Vertex3)
    : Equal Bool (vertex3_eq x y) True → Equal Vertex3 x y =
  match x {
    V0 ↦ match y {
      V0 ↦ λsame. Proved;
      V1 ↦ λsame. absurd same;
      V2 ↦ λsame. absurd same
    };
    V1 ↦ match y {
      V0 ↦ λsame. absurd same;
      V1 ↦ λsame. Proved;
      V2 ↦ λsame. absurd same
    };
    V2 ↦ match y {
      V0 ↦ λsame. absurd same;
      V1 ↦ λsame. absurd same;
      V2 ↦ λsame. Proved
    }
  }

theorem vertex3_eq_complete (x : Vertex3) (y : Vertex3)
    : Equal Vertex3 x y → Equal Bool (vertex3_eq x y) True =
  match x {
    V0 ↦ match y {
      V0 ↦ λsame. Proved;
      V1 ↦ λsame. absurd same;
      V2 ↦ λsame. absurd same
    };
    V1 ↦ match y {
      V0 ↦ λsame. absurd same;
      V1 ↦ λsame. Proved;
      V2 ↦ λsame. absurd same
    };
    V2 ↦ match y {
      V0 ↦ λsame. absurd same;
      V1 ↦ λsame. absurd same;
      V2 ↦ λsame. Proved
    }
  }

instance DecEq Vertex3 {
  eq = vertex3_eq;
  sound = vertex3_eq_sound;
  complete = vertex3_eq_complete
}

fn successors (s : Vertex3) : List Vertex3 =
  match s {
    V0 ↦ Cons Vertex3 V1 (Nil Vertex3);
    V1 ↦ Cons Vertex3 V2 (Nil Vertex3);
    V2 ↦ Nil Vertex3
  }

const from_lists : Graph Vertex3 =
  graph_from_successors Vertex3 DecEq_instance_Vertex3 successors

theorem listed_from_v0 : Equal Bool (edge Vertex3 from_lists V0 V1) True = Proved

theorem unlisted_from_v0 : Equal Bool (edge Vertex3 from_lists V0 V2) False = Proved

theorem listed_from_v1 : Equal Bool (edge Vertex3 from_lists V1 V2) True = Proved

theorem unlisted_from_v1 : Equal Bool (edge Vertex3 from_lists V1 V0) False = Proved

theorem listed_sound : list_elem Vertex3 V1 (successors V0) =
  graph_from_successors_sound
    Vertex3 DecEq_instance_Vertex3 successors V0 V1 listed_from_v0

theorem listed_complete : Equal Bool (edge Vertex3 from_lists V0 V1) True =
  graph_from_successors_complete
    Vertex3 DecEq_instance_Vertex3 successors V0 V1
    (list_elem_head Vertex3 V1 (Nil Vertex3))
"#;

/// MEASURED: Ken reduces both listed edges to True and both unlisted edges to
/// False for distinct source and target vertices. Both public laws apply to
/// the listed edge, while a same-form claim of the unlisted edge as True is
/// kernel-rejected with a type mismatch. CLAIMED: the adaptor's observable
/// edge follows the supplied successor lists, not a constant adjacency.
/// THE GAP: these closed rows do not prove the generic soundness/completeness
/// laws, which the package and independent generic client check separately.
#[test]
fn three_vertex_lists_distinguish_listed_edges_from_absent_edges() {
    let mut env = loaded_graph();
    env.elaborate_file(VERTICES_AND_LISTS)
        .expect("four closed edges and both public law applications check");
    let error = env
        .elaborate_file(
            "theorem false_extra_edge : Equal Bool (edge Vertex3 from_lists V0 V2) True = Proved",
        )
        .expect_err("V0's list contains V1 but not V2");
    assert!(
        matches!(
            &error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "a nonexistent edge must reject at its proof, not at parsing/import: {error:?}"
    );
}
