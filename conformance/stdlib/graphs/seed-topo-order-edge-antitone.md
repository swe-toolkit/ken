# Edge removal preserves a topological order

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §13.
Frame: `docs/program/issues/SPEC-GRAPHS-ORDER-EDGE-ANTITONE.md`.
Ruling: Architect AC-0 `evt_55fe7ahz18pqb`.

Status: authored; not executed. The generic client applies the public
function with open graph and order inputs. A separate closed client removes
one edge and checks that the order's vertex list is preserved. Neither
substitutes for the checked generic function.

## Accepted client

The open client pins both generic application and the definitional same-list
consequence. The closed fixture orders `tri`, removes its direct edge
`V0 → V2` to form `chain`, and applies the function to that subgraph.

```ken
import Core.Classes.LawfulClasses (DecEq)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, TopoOrder, MkTopoOrder, ordered_vertices,
   topo_order_edge_antitone)

fn topo_order_edge_antitone_open
      (q : Type)
      (d : DecEq q)
      (g : Graph q)
      (h : Graph q)
      (included : (u : q)
        → (v : q)
        → Equal Bool (edge q h u v) True
        → Equal Bool (edge q g u v) True)
      (order : TopoOrder q d g)
    : TopoOrder q d h =
  topo_order_edge_antitone q d g h included order

theorem topo_order_edge_antitone_same_list_open
      (q : Type)
      (d : DecEq q)
      (g : Graph q)
      (h : Graph q)
      (included : (u : q)
        → (v : q)
        → Equal Bool (edge q h u v) True
        → Equal Bool (edge q g u v) True)
      (order : TopoOrder q d g)
    : Equal (List q)
        (ordered_vertices q d h
          (topo_order_edge_antitone q d g h included order))
        (ordered_vertices q d g order) =
  Refl

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

fn vertex3_eq (x : Vertex3) (y : Vertex3) : Bool =
  match x {
    V0 ↦
      match y {
        V0 ↦ True;
        V1 ↦ False;
        V2 ↦ False
      };
    V1 ↦
      match y {
        V0 ↦ False;
        V1 ↦ True;
        V2 ↦ False
      };
    V2 ↦
      match y {
        V0 ↦ False;
        V1 ↦ False;
        V2 ↦ True
      }
  }

theorem vertex3_eq_sound
      (x : Vertex3) (y : Vertex3)
    : Equal Bool (vertex3_eq x y) True → Equal Vertex3 x y =
  match x {
    V0 ↦
      match y {
        V0 ↦ λsame. Proved;
        V1 ↦ λsame. absurd same;
        V2 ↦ λsame. absurd same
      };
    V1 ↦
      match y {
        V0 ↦ λsame. absurd same;
        V1 ↦ λsame. Proved;
        V2 ↦ λsame. absurd same
      };
    V2 ↦
      match y {
        V0 ↦ λsame. absurd same;
        V1 ↦ λsame. absurd same;
        V2 ↦ λsame. Proved
      }
  }

theorem vertex3_eq_complete
      (x : Vertex3) (y : Vertex3)
    : Equal Vertex3 x y → Equal Bool (vertex3_eq x y) True =
  match x {
    V0 ↦
      match y {
        V0 ↦ λsame. Proved;
        V1 ↦ λsame. absurd same;
        V2 ↦ λsame. absurd same
      };
    V1 ↦
      match y {
        V0 ↦ λsame. absurd same;
        V1 ↦ λsame. Proved;
        V2 ↦ λsame. absurd same
      };
    V2 ↦
      match y {
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
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

const chain : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ True
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

const removed_edge_present :
  Equal Bool (edge Vertex3 tri V0 V2) True = Proved

const removed_edge_absent :
  Equal Bool (edge Vertex3 chain V0 V2) False = Proved

const tri_order : TopoOrder Vertex3 DecEq_instance_Vertex3 tri =
  MkTopoOrder
    Vertex3
    DecEq_instance_Vertex3
    tri
    (Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3))))
    Proved
    (λx.
      match x {
        V0 ↦ Proved;
        V1 ↦ Proved;
        V2 ↦ Proved
      })
    (λx.
      λy.
        match x {
          V0 ↦
            match y {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. Proved;
              V2 ↦ λe. Proved
            };
          V1 ↦
            match y {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. absurd e;
              V2 ↦ λe. Proved
            };
          V2 ↦
            match y {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. absurd e;
              V2 ↦ λe. absurd e
            }
        })

theorem chain_in_tri
    : (u : Vertex3)
      → (v : Vertex3)
      → Equal Bool (edge Vertex3 chain u v) True
      → Equal Bool (edge Vertex3 tri u v) True =
  λu.
    λv.
      match u {
        V0 ↦
          match v {
            V0 ↦ λe. absurd e;
            V1 ↦ λe. Proved;
            V2 ↦ λe. absurd e
          };
        V1 ↦
          match v {
            V0 ↦ λe. absurd e;
            V1 ↦ λe. absurd e;
            V2 ↦ λe. Proved
          };
        V2 ↦
          match v {
            V0 ↦ λe. absurd e;
            V1 ↦ λe. absurd e;
            V2 ↦ λe. absurd e
          }
      }

const chain_order : TopoOrder Vertex3 DecEq_instance_Vertex3 chain =
  topo_order_edge_antitone
    Vertex3
    DecEq_instance_Vertex3
    tri
    chain
    chain_in_tri
    tri_order

theorem chain_order_list
    : Equal (List Vertex3)
        (ordered_vertices Vertex3 DecEq_instance_Vertex3 chain chain_order)
        (Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))) =
  Refl
```

### stdlib/graphs/topo-order-edge-antitone-open

- spec: `spec/50-stdlib/62-graphs.md` §13.
- promise class: durable invariant.
- given: arbitrary `q`, `d`, `g`, `h`, a pointwise inclusion of `h`'s
  true edges in `g`'s, and an order of `g`.
- expect: the public function returns an order of `h`; `Refl` proves that
  its `ordered_vertices` equals the input order's list with `order` open.
- claimed: edge removal preserves an existing order on the same vertex
  list.
- gap: applying the public function does not establish its checked body.

### stdlib/graphs/topo-order-edge-antitone-edge-removal

- spec: `spec/50-stdlib/62-graphs.md` §13.
- promise class: durable invariant.
- given: the three-vertex `tri` order, the inclusion proof from `chain`
  into `tri`, and the removed edge controls above.
- expect: the function constructs an order of `chain` and `Refl` proves
  its vertex list is `[V0, V1, V2]`.
- claimed: the order of `tri` also orders the subgraph `chain` after
  removing `V0 → V2`.
- gap: this fixed graph and order do not establish the generic function.

## Isolated refusal

The refusal starts from a fresh copy of the entire accepted client and
appends only the declarations below. `back` adds `V2 → V0` to `tri`;
the proposed inclusion and law application use the same form as the
accepted edge-removal client. The rejection must occur in the
`back_in_tri` arm for `V2` then `V0`, before `back_order` can be checked.

### stdlib/graphs/reject-topo-order-edge-antitone-added-backward-edge

- spec: `spec/50-stdlib/62-graphs.md` §13.
- promise class: durable invariant.
- given: the accepted client and `back`, with its additional backward
  edge `V2 → V0`.
- expect: reject the proposed inclusion at the `V2`, `V0` arm because
  it requires `Equal Bool (edge Vertex3 tri V2 V0) True`, which reduces
  to `Equal Bool False True`; do not attribute the refusal to the
  subsequent law application.
- control: `chain_order` accepts the same function form when the target
  graph is the subgraph `chain` and its inclusion premise holds.
- claimed: the order of `tri` cannot be transferred to `back` by this
  law when the pointwise edge-inclusion premise is false.
- gap: one backward edge in this fixture does not characterize every
  invalid inclusion or prove the generic function.

```ken
const back : Graph Vertex3 =
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
          V2 ↦
            match v {
              V0 ↦ True;
              V1 ↦ False;
              V2 ↦ False
            }
        })

theorem back_edge_present :
  Equal Bool (edge Vertex3 back V2 V0) True = Proved

theorem back_edge_absent_from_tri :
  Equal Bool (edge Vertex3 tri V2 V0) False = Proved

theorem back_in_tri
    : (u : Vertex3)
      → (v : Vertex3)
      → Equal Bool (edge Vertex3 back u v) True
      → Equal Bool (edge Vertex3 tri u v) True =
  λu.
    λv.
      match u {
        V0 ↦
          match v {
            V0 ↦ λe. absurd e;
            V1 ↦ λe. Proved;
            V2 ↦ λe. Proved
          };
        V1 ↦
          match v {
            V0 ↦ λe. absurd e;
            V1 ↦ λe. absurd e;
            V2 ↦ λe. Proved
          };
        V2 ↦
          match v {
            V0 ↦ λe. Proved;
            V1 ↦ λe. absurd e;
            V2 ↦ λe. absurd e
          }
      }

const back_order : TopoOrder Vertex3 DecEq_instance_Vertex3 back =
  topo_order_edge_antitone
    Vertex3
    DecEq_instance_Vertex3
    tri
    back
    back_in_tri
    tri_order
```

## Evidence boundary

Architect AC-0 reports that the generic application and open same-list
`Refl` row passed in scratch; the D0 also identified the false inclusion
at `back_in_tri`'s `V2 → V0` arm. The initial closed list proof used
`Proved` and refused; the Architect ruled that `Refl` is the correct
proof term. These are attributed scratch results, not execution of this
seed or an assembled candidate. CV has not run `ken check`, semantic
tests, or the official conformance runner. No CI, publication, or
landing is claimed. No external references or `local/refs/` were
consulted.
