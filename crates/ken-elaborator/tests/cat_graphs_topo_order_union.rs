//! Source: Graphs §22 and Architect AC-0's checked accepted/refused clients.
//! These package-local fixtures do not execute the official conformance seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const GRAPH: &str = "Algorithm.Graphs.Dependency";

fn load(env: &mut ElabEnv, module: &str) -> Vec<GlobalId> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    env.elaborate_module_from_roots(&[root], module)
        .unwrap_or_else(|error| panic!("{module} roots-loads: {error:?}"))
}

fn contains_const(term: &Term, target: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } if *id == target)
        || term
            .children()
            .into_iter()
            .any(|child| contains_const(child, target))
}

/// Promise class: durable invariant. MEASURED: Dependency leaves the exact
/// provider-relative trusted identity set unchanged. The open clients and the
/// client-derived two-order form cite the Dependency-owned Transparent exports.
/// The three-vertex fixture reads off the independently fixed list through the
/// public vertex law and reads forward evidence for one edge from each input.
/// CLAIMED: an order on g and a forward proof for h on its list suffice.
/// Compatible: new checked APIs and proof refactoring. Breaking: new trust,
/// different ownership, or requiring the client to already order the union.
/// GAP: this does not audit inherited trust or exhaust all closed graphs.
#[test]
fn open_union_clients_are_owned_transparent_and_add_no_trust() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Core.Function.Combinators",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Compare",
        "Core.Classes.LawfulClasses",
        "Core.Logic.And",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Data.Collections.List",
        "Data.Collections.Derived",
        "Data.Finite.Finite",
        "Data.Sums.Combinators",
        "Algorithm.FormalLanguages.Dfa",
        "Algorithm.FormalLanguages.Reachability",
    ] {
        load(&mut env, provider);
    }
    let trusted = |env: &ElabEnv| env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let before = trusted(&env);
    let graph_owned = load(&mut env, GRAPH);
    let after = trusted(&env);
    eprintln!(
        "Dependency trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Dependency adds no trusted GlobalIds");

    let clients = env
        .elaborate_file(ACCEPTED)
        .expect("generic and closed union clients check");
    for (export, names) in [
        (
            "topo_order_union",
            &[
                "topo_order_union_open",
                "topo_order_union_of_equal_lists",
                "union_order",
            ][..],
        ),
        (
            "topo_order_union_vertices",
            &["topo_order_union_vertices_open", "union_list"][..],
        ),
    ] {
        let id = env.globals[&format!("{GRAPH}.{export}")];
        assert!(graph_owned.contains(&id), "{GRAPH} owns {export}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{export} is Transparent"
        );
        for client in names {
            let client_id = env.globals[*client];
            assert!(clients.contains(&client_id));
            let Some(Decl::Transparent { body, .. }) = env.env.lookup(client_id) else {
                panic!("{client} must have a checked body");
            };
            assert!(
                contains_const(body, id),
                "{client} cites the exact {export}"
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: on the fresh accepted prefix,
/// h21_forward is kernel-rejected at its own parsed declaration span.
/// CLAIMED: the backward edge cannot satisfy the right-graph forward premise.
/// Compatible: a different list that actually orders h21. Breaking: accepting
/// this false proof or moving its refusal to an unrelated declaration.
/// GAP: the refusal is before union law use, not evidence about the law's
/// signature. The open client distinguishes a widened union-edge premise.
#[test]
fn backward_right_edge_refuses_at_its_forward_declaration() {
    let source = format!("{ACCEPTED}{REFUSED_SUFFIX}");
    let declarations = parse_decls(&source).expect("union refusal parses");
    let Some(SourceDecl::TheoremDecl {
        name,
        span: expected,
        ..
    }) = declarations.last()
    else {
        panic!("the refused source ends in the forward theorem");
    };
    assert_eq!(name, "h21_forward");
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    load(&mut env, GRAPH);
    match env.elaborate_file(&source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            assert_eq!(
                span.start..span.end,
                expected.start..expected.end,
                "refusal must reach h21_forward"
            );
            eprintln!("h21_forward: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("h21_forward must refuse at its own declaration: {other:?}"),
    }
}

const ACCEPTED: &str = r#"import Core.Classes.LawfulClasses (DecEq)
import Core.Logic.Transport (cong, trans)
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, graph_union, before, all_edges_forward, TopoOrder,
   MkTopoOrder, ordered_vertices, ordered_forward, topo_order_union,
   topo_order_union_vertices)

fn topo_order_union_open
      (q : Type) (d : DecEq q) (g : Graph q) (h : Graph q)
      (order : TopoOrder q d g)
      (forward_h : all_edges_forward q d h (ordered_vertices q d g order))
    : TopoOrder q d (graph_union q g h) =
  topo_order_union q d g h order forward_h

theorem topo_order_union_vertices_open
      (q : Type) (d : DecEq q) (g : Graph q) (h : Graph q)
      (order : TopoOrder q d g)
      (forward_h : all_edges_forward q d h (ordered_vertices q d g order))
    : Equal
        (List q)
        (ordered_vertices q d (graph_union q g h) (topo_order_union q d g h order forward_h))
        (ordered_vertices q d g order) =
  topo_order_union_vertices q d g h order forward_h

fn topo_order_union_of_equal_lists
      (q : Type) (d : DecEq q) (g : Graph q) (h : Graph q)
      (order_g : TopoOrder q d g) (order_h : TopoOrder q d h)
      (same : Equal (List q) (ordered_vertices q d g order_g) (ordered_vertices q d h order_h))
    : TopoOrder q d (graph_union q g h) =
  topo_order_union q d g h order_g
    (λx. λy. λfrom_h.
      trans Bool
        (before q d x y (ordered_vertices q d g order_g))
        (before q d x y (ordered_vertices q d h order_h))
        True
        (cong (List q) Bool
          (ordered_vertices q d g order_g)
          (ordered_vertices q d h order_h)
          (before q d x y)
          same)
        (ordered_forward q d h order_h x y from_h))

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

theorem vertex3_eq_sound (x : Vertex3) (y : Vertex3) :
  Equal Bool (vertex3_eq x y) True → Equal Vertex3 x y =
  match x {
    V0 ↦ match y { V0 ↦ λsame. Proved; V1 ↦ λsame. absurd same; V2 ↦ λsame. absurd same };
    V1 ↦ match y { V0 ↦ λsame. absurd same; V1 ↦ λsame. Proved; V2 ↦ λsame. absurd same };
    V2 ↦ match y { V0 ↦ λsame. absurd same; V1 ↦ λsame. absurd same; V2 ↦ λsame. Proved }
  }

theorem vertex3_eq_complete (x : Vertex3) (y : Vertex3) :
  Equal Vertex3 x y → Equal Bool (vertex3_eq x y) True =
  match x {
    V0 ↦ match y { V0 ↦ λsame. Proved; V1 ↦ λsame. absurd same; V2 ↦ λsame. absurd same };
    V1 ↦ match y { V0 ↦ λsame. absurd same; V1 ↦ λsame. Proved; V2 ↦ λsame. absurd same };
    V2 ↦ match y { V0 ↦ λsame. absurd same; V1 ↦ λsame. absurd same; V2 ↦ λsame. Proved }
  }

instance DecEq Vertex3 {
  eq = vertex3_eq;
  sound = vertex3_eq_sound;
  complete = vertex3_eq_complete
}

const ordered_three : List Vertex3 =
  Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))

const g01 : Graph Vertex3 =
  MkGraph Vertex3 (λu. λv. match u {
    V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ False };
    V1 ↦ False;
    V2 ↦ False
  })

const order01 : TopoOrder Vertex3 DecEq_instance_Vertex3 g01 =
  MkTopoOrder Vertex3 DecEq_instance_Vertex3 g01 ordered_three
    Proved
    (λx. match x { V0 ↦ Proved; V1 ↦ Proved; V2 ↦ Proved })
    (λx. λy. match x {
      V0 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. Proved; V2 ↦ λe. absurd e };
      V1 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e };
      V2 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e }
    })

const h12 : Graph Vertex3 =
  MkGraph Vertex3 (λu. λv. match u {
    V0 ↦ False;
    V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ True };
    V2 ↦ False
  })

theorem h12_forward
  : all_edges_forward Vertex3 DecEq_instance_Vertex3 h12
      (ordered_vertices Vertex3 DecEq_instance_Vertex3 g01 order01) =
  λx. λy. match x {
    V0 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e };
    V1 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. Proved };
    V2 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e }
  }

const union_order : TopoOrder Vertex3 DecEq_instance_Vertex3 (graph_union Vertex3 g01 h12) =
  topo_order_union Vertex3 DecEq_instance_Vertex3 g01 h12 order01 h12_forward

theorem union_list
  : Equal (List Vertex3)
      (ordered_vertices Vertex3 DecEq_instance_Vertex3 (graph_union Vertex3 g01 h12) union_order)
      ordered_three =
  topo_order_union_vertices Vertex3 DecEq_instance_Vertex3 g01 h12 order01 h12_forward

theorem union_orders_g_edge
  : Equal Bool
      (before Vertex3 DecEq_instance_Vertex3 V0 V1
        (ordered_vertices Vertex3 DecEq_instance_Vertex3 (graph_union Vertex3 g01 h12) union_order))
      True =
  ordered_forward Vertex3 DecEq_instance_Vertex3 (graph_union Vertex3 g01 h12) union_order V0 V1 Proved

theorem union_orders_h_edge
  : Equal Bool
      (before Vertex3 DecEq_instance_Vertex3 V1 V2
        (ordered_vertices Vertex3 DecEq_instance_Vertex3 (graph_union Vertex3 g01 h12) union_order))
      True =
  ordered_forward Vertex3 DecEq_instance_Vertex3 (graph_union Vertex3 g01 h12) union_order V1 V2 Proved
"#;

const REFUSED_SUFFIX: &str = r#"
const h21 : Graph Vertex3 =
  MkGraph Vertex3 (λu. λv. match u {
    V0 ↦ False;
    V1 ↦ False;
    V2 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ False }
  })

theorem h21_forward
  : all_edges_forward Vertex3 DecEq_instance_Vertex3 h21
      (ordered_vertices Vertex3 DecEq_instance_Vertex3 g01 order01) =
  λx. λy. match x {
    V0 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e };
    V1 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e };
    V2 ↦ match y { V0 ↦ λe. absurd e; V1 ↦ λe. Proved; V2 ↦ λe. absurd e }
  }
"#;
