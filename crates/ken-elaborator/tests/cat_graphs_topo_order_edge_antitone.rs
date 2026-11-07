//! A dependency order survives deletion of graph edges: spec 62 §13.
//! Promise class: durable invariant. The generic law, open-list conversion,
//! and a closed accepted/refused edge-inclusion pair are checked separately.
//! The closed fixtures follow Architect AC-0, not an official conformance run.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError, Term};

const GRAPH: &str = "Algorithm.Graphs.Dependency";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_graph() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§13 graph module roots-loads");
    env
}

const GENERIC_PREFIX: &str = r#"
import Core.Classes.LawfulClasses (DecEq)
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, TopoOrder, MkTopoOrder, ordered_vertices,
   ordered_complete, ordered_unique, ordered_forward, topo_order_edge_antitone)
"#;

const GENERIC_TELESCOPE: &str = "(q : Type) (d : DecEq q) (g : Graph q) (h : Graph q)
  (included : (u : q) → (v : q)
    → Equal Bool (edge q h u v) True → Equal Bool (edge q g u v) True)
  (order : TopoOrder q d g)";

fn generic_apply() -> String {
    format!(
        "{GENERIC_PREFIX}\nfn map_order {GENERIC_TELESCOPE} : TopoOrder q d h =
         topo_order_edge_antitone q d g h included order"
    )
}

fn open_same_list() -> String {
    format!(
        "{GENERIC_PREFIX}\ntheorem same_list {GENERIC_TELESCOPE}
          : Equal (List q)
            (ordered_vertices q d h (topo_order_edge_antitone q d g h included order))
            (ordered_vertices q d g order) = Refl"
    )
}

const MATCH_FORM: &str = r#"
fn match_order (q : Type) (d : DecEq q) (g : Graph q) (h : Graph q)
    (included : (u : q) → (v : q)
      → Equal Bool (edge q h u v) True → Equal Bool (edge q g u v) True)
    : TopoOrder q d g → TopoOrder q d h =
  λorder.
    match order {
      MkTopoOrder xs unique complete forward ↦
        MkTopoOrder q d h xs unique complete (λx. λy. λe. forward x y (included x y e))
    }
"#;

fn match_form_control() -> String {
    format!(
        "{GENERIC_PREFIX}\n{MATCH_FORM}\nfn match_apply {GENERIC_TELESCOPE}
          : TopoOrder q d h = match_order q d g h included order
         theorem match_same_list {GENERIC_TELESCOPE}
          : Equal (List q)
            (ordered_vertices q d h (match_order q d g h included order))
            (ordered_vertices q d g order) = Refl"
    )
}

/// MEASURED: the provider-to-Dependency trusted GlobalId sets agree, the
/// published law is module-owned, Transparent and selectively importable,
/// and a client maps an arbitrary graph order with the precise inclusion.
/// CLAIMED: §13 exports a checked general order restriction without new
/// trust. THE GAP: one generic client does not observe the unchanged list;
/// the next test establishes its definitional equality on an open order.
#[test]
fn generic_order_restriction_is_public_checked_and_zero_trust() {
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
        .expect("§13 Dependency checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "no trusted GlobalId added or removed");
    let id = env.globals[&format!("{GRAPH}.topo_order_edge_antitone")];
    assert!(owned.contains(&id), "the law is owned by Dependency");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(&format!(
        "import {GRAPH} (topo_order_edge_antitone as restricted_order)"
    ))
    .expect("the law selectively imports");
    env.elaborate_file(&generic_apply())
        .expect("an arbitrary graph order restricts along edge inclusion");
    env.elaborate_file(&format!(
        "{GENERIC_PREFIX}\nfn same_graph (q : Type) (d : DecEq q) (g : Graph q)
           (order : TopoOrder q d g) : TopoOrder q d g =
           topo_order_edge_antitone q d g g (λu. λv. λe. e) order"
    ))
    .expect("edge inclusion also permits the identical graph");
    let wrong_direction = format!(
        "{GENERIC_PREFIX}\nfn invalid_direction
           (q : Type) (d : DecEq q) (g : Graph q) (h : Graph q)
           (included : (u : q) → (v : q)
             → Equal Bool (edge q g u v) True → Equal Bool (edge q h u v) True)
           (order : TopoOrder q d g) : TopoOrder q d h =
           topo_order_edge_antitone q d g h included order"
    );
    match env.elaborate_file(&wrong_direction) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => {}
        other => panic!("the converse inclusion cannot restrict an order: {other:?}"),
    }
}

/// MEASURED: `Refl` checks equality of the order's projected list before and
/// after restriction even when the source order is open. The match-form
/// control constructs the same result but rejects that `Refl` at a stuck
/// projection. CLAIMED: list preservation follows by conversion from §13's
/// actual law, not just from a closed example. THE GAP: this only pins the
/// projected list, not the three independent order-certificate obligations.
#[test]
fn open_order_keeps_exact_list_by_conversion_not_by_match() {
    let mut env = loaded_graph();
    env.elaborate_file(&open_same_list())
        .expect("the public law leaves an open order's list definitionally unchanged");
    let control = match_form_control();
    let mut match_env = loaded_graph();
    let prefix = control
        .split_once("theorem match_same_list")
        .expect("match-form control has a typed construction")
        .0;
    match_env
        .elaborate_file(prefix)
        .expect("match-form law and generic application both check");
    let mut refusing = loaded_graph();
    match refusing.elaborate_file(&control) {
        Err(ElabError::TypeMismatch { .. }) => {}
        other => panic!("match-form open-list Refl must refuse: {other:?}"),
    }
}

// These declarations describe the fixed three-vertex fixture used in both
// directions. They are test inputs, not copied from a third-party reference.
const VERTICES_AND_TRI: &str = r#"
import Core.Classes.LawfulClasses (DecEq)
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, TopoOrder, MkTopoOrder, ordered_vertices, topo_order_edge_antitone)

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

const tri : Graph Vertex3 =
  MkGraph Vertex3 (λu. λv. match u {
    V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ True };
    V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ True };
    V2 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False }
  })

const tri_order : TopoOrder Vertex3 DecEq_instance_Vertex3 tri =
  MkTopoOrder Vertex3 DecEq_instance_Vertex3 tri
    (Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3))))
    Proved
    (λx. match x { V0 ↦ Proved; V1 ↦ Proved; V2 ↦ Proved })
    (λx. λy. match x {
      V0 ↦ match y {
        V0 ↦ λe. absurd e; V1 ↦ λe. Proved; V2 ↦ λe. Proved
      };
      V1 ↦ match y {
        V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. Proved
      };
      V2 ↦ match y {
        V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e
      }
    })
"#;

const CHAIN_AND_ACCEPTED: &str = r#"
const chain : Graph Vertex3 =
  MkGraph Vertex3 (λu. λv. match u {
    V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ False };
    V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ True };
    V2 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False }
  })

theorem chain_in_tri
    : (u : Vertex3) → (v : Vertex3)
      → Equal Bool (edge Vertex3 chain u v) True
      → Equal Bool (edge Vertex3 tri u v) True =
  λx. λy. match x {
    V0 ↦ match y {
      V0 ↦ λe. absurd e; V1 ↦ λe. Proved; V2 ↦ λe. absurd e
    };
    V1 ↦ match y {
      V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. Proved
    };
    V2 ↦ match y {
      V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e
    }
  }

const chain_order : TopoOrder Vertex3 DecEq_instance_Vertex3 chain =
  topo_order_edge_antitone Vertex3 DecEq_instance_Vertex3 tri chain chain_in_tri tri_order

theorem chain_order_list
    : Equal (List Vertex3)
      (ordered_vertices Vertex3 DecEq_instance_Vertex3 chain chain_order)
      (Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))) = Refl
"#;

const BACK_AND_REFUSED: &str = r#"
const back : Graph Vertex3 =
  MkGraph Vertex3 (λu. λv. match u {
    V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ True };
    V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ True };
    V2 ↦ match v { V0 ↦ True; V1 ↦ False; V2 ↦ False }
  })

theorem back_in_tri
    : (u : Vertex3) → (v : Vertex3)
      → Equal Bool (edge Vertex3 back u v) True
      → Equal Bool (edge Vertex3 tri u v) True =
  λx. λy. match x {
    V0 ↦ match y {
      V0 ↦ λe. absurd e; V1 ↦ λe. Proved; V2 ↦ λe. Proved
    };
    V1 ↦ match y {
      V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. Proved
    };
    V2 ↦ match y {
      V0 ↦ λe. Proved; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e
    }
  }

const back_order : TopoOrder Vertex3 DecEq_instance_Vertex3 back =
  topo_order_edge_antitone Vertex3 DecEq_instance_Vertex3 tri back back_in_tri tri_order
"#;

fn accepted_seed() -> String {
    format!("{VERTICES_AND_TRI}\n{CHAIN_AND_ACCEPTED}")
}

fn refused_seed() -> String {
    format!("{VERTICES_AND_TRI}\n{BACK_AND_REFUSED}")
}

/// MEASURED: a complete three-vertex client checks the tri order, its
/// pointwise inclusion after removing the shortcut V0→V2, the mapped chain
/// order and its unchanged vertex list. The same-form added-backward-edge
/// client is rejected at its inclusion theorem; its expected kernel goal
/// agrees with an independently stated `edge tri V2 V0 = True` failure.
/// CLAIMED: only edge removal preserves an existing order on the same list;
/// adding a backward edge cannot pass the inclusion premise. THE GAP: this
/// finite example cannot prove the generic public law, checked separately.
#[test]
fn removing_a_shortcut_preserves_order_but_added_backward_edge_refuses() {
    let accepted = accepted_seed();
    let mut positive = loaded_graph();
    positive
        .elaborate_file(&accepted)
        .expect("a real shortcut deletion preserves the exact vertex order");
    positive
        .elaborate_file(
            "theorem tri_shortcut : Equal Bool (edge Vertex3 tri V0 V2) True = Proved
             theorem chain_without_shortcut : Equal Bool (edge Vertex3 chain V0 V2) False = Proved
             theorem chain_first_edge : Equal Bool (edge Vertex3 chain V0 V1) True = Proved
             theorem chain_second_edge : Equal Bool (edge Vertex3 chain V1 V2) True = Proved",
        )
        .expect("the source order has a real shortcut removed from the target chain");

    let refused = refused_seed();
    let declaration_start = refused
        .find("theorem back_in_tri")
        .expect("the refused client has an inclusion theorem");
    let parsed = parse_decls(&refused).expect("the closed refused client parses");
    let failed_theorem_span = parsed
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, span, .. } if name == "back_in_tri" => {
                Some(span.clone())
            }
            _ => None,
        })
        .expect("the refusal has the one edge-inclusion theorem");
    let prefix = &refused[..declaration_start];
    loaded_graph()
        .elaborate_file(prefix)
        .expect("the backward-edge graph and valid tri order check before inclusion");

    let mut control = loaded_graph();
    control
        .elaborate_file(prefix)
        .expect("independent edge-table prefix checks");
    control
        .elaborate_file(
            "theorem back_has_backward_edge : Equal Bool (edge Vertex3 back V2 V0) True = Proved
             theorem tri_lacks_backward_edge : Equal Bool (edge Vertex3 tri V2 V0) False = Proved",
        )
        .expect("the only added edge runs backward against tri's order");
    let vertex3_id = control.globals["Vertex3"];
    let v0_id = control.globals["V0"];
    let mut rejecting = loaded_graph();
    match rejecting.elaborate_file(&refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, found },
            span,
        }) => {
            assert!(
                span.start >= failed_theorem_span.start && span.end <= failed_theorem_span.end,
                "failure must be in the inclusion theorem, not back_order: {span:?}"
            );
            assert!(
                matches!(*found, Term::Const { .. }),
                "the false arm supplies the checked Proved constant"
            );
            let Term::Eq(_, left, right) = *expected else {
                panic!("the refused arm must target an edge equality");
            };
            let Term::Elim {
                fam,
                methods,
                scrut,
                ..
            } = *left
            else {
                panic!("the refused arm must inspect tri's three-vertex edge row");
            };
            assert_eq!(fam, vertex3_id, "the failing edge indexes Vertex3");
            assert_eq!(methods.len(), 3, "the target graph has three destinations");
            assert_eq!(methods[0], methods[1], "tri's V2 row is all False");
            assert_eq!(methods[1], methods[2], "tri's V2 row is all False");
            assert!(
                matches!(*scrut, Term::Constructor { id, .. } if id == v0_id),
                "the failed target of the all-False V2 row is V0"
            );
            assert_ne!(
                methods[0], *right,
                "False cannot prove the expected True edge"
            );
        }
        other => panic!("added backward edge must fail in back_in_tri: {other:?}"),
    }
}
