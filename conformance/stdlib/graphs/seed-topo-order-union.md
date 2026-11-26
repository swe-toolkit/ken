# A topological order transfers to a graph union

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §22.

The accepted prefix checks the open transfer and its vertex law, plus a
client-derived two-order form. Its closed fixture places one edge in
`g01` and another in `h12`. The union order keeps the first graph's
vertex list and orders edges from both inputs. Check each case with a
fresh copy of the complete prefix.

## Shared accepted prefix

```ken
import Core.Classes.LawfulClasses (DecEq)

import Core.Logic.Transport (cong, trans)

import Algorithm.Graphs.Dependency
  (Graph,
    MkGraph,
    edge,
    graph_union,
    graph_union_edge_left,
    graph_union_edge_right,
    TopoOrder,
    MkTopoOrder,
    ordered_vertices,
    ordered_forward,
    contains,
    before,
    all_edges_forward,
    topo_order_union,
    topo_order_union_vertices)

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

const ordered_three : List Vertex3 =
  Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))

const g01 : Graph Vertex3 =
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
              V2 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

const h12 : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
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

const union_graph : Graph Vertex3 = graph_union Vertex3 g01 h12

theorem order01_covers
      (x : Vertex3)
    : Equal Bool (contains Vertex3 DecEq_instance_Vertex3 x ordered_three) True =
  match x {
    V0 ↦ Proved;
    V1 ↦ Proved;
    V2 ↦ Proved
  }

const order01 : TopoOrder Vertex3 DecEq_instance_Vertex3 g01 =
  MkTopoOrder
    Vertex3
    DecEq_instance_Vertex3
    g01
    ordered_three
    Proved
    order01_covers
    (λx.
      λy.
        match x {
          V0 ↦
            match y {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. Proved;
              V2 ↦ λe. absurd e
            };
          V1 ↦
            match y {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. absurd e;
              V2 ↦ λe. absurd e
            };
          V2 ↦
            match y {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. absurd e;
              V2 ↦ λe. absurd e
            }
        })

theorem h12_forward
    : all_edges_forward Vertex3 DecEq_instance_Vertex3 h12
        (ordered_vertices Vertex3 DecEq_instance_Vertex3 g01 order01) =
  λx.
    λy.
      match x {
        V0 ↦
          match y {
            V0 ↦ λpresent. absurd present;
            V1 ↦ λpresent. absurd present;
            V2 ↦ λpresent. absurd present
          };
        V1 ↦
          match y {
            V0 ↦ λpresent. absurd present;
            V1 ↦ λpresent. absurd present;
            V2 ↦ λpresent. Proved
          };
        V2 ↦
          match y {
            V0 ↦ λpresent. absurd present;
            V1 ↦ λpresent. absurd present;
            V2 ↦ λpresent. absurd present
          }
      }

fn topo_order_union_open
      (q : Type)
      (d : DecEq q)
      (g : Graph q)
      (h : Graph q)
      (order : TopoOrder q d g)
      (forward_h : all_edges_forward q d h (ordered_vertices q d g order))
    : TopoOrder q d (graph_union q g h) =
  topo_order_union q d g h order forward_h

theorem topo_order_union_vertices_open
      (q : Type)
      (d : DecEq q)
      (g : Graph q)
      (h : Graph q)
      (order : TopoOrder q d g)
      (forward_h : all_edges_forward q d h (ordered_vertices q d g order))
    : Equal
        (List q)
        (ordered_vertices q d (graph_union q g h) (topo_order_union q d g h order forward_h))
        (ordered_vertices q d g order) =
  topo_order_union_vertices q d g h order forward_h

fn topo_order_union_of_equal_lists
      (q : Type)
      (d : DecEq q)
      (g : Graph q)
      (h : Graph q)
      (order_g : TopoOrder q d g)
      (order_h : TopoOrder q d h)
      (same : Equal (List q) (ordered_vertices q d g order_g) (ordered_vertices q d h order_h))
    : TopoOrder q d (graph_union q g h) =
  topo_order_union
    q
    d
    g
    h
    order_g
    (λx.
      λy.
        λfrom_h.
          trans
            Bool
            (before q d x y (ordered_vertices q d g order_g))
            (before q d x y (ordered_vertices q d h order_h))
            True
            (cong
              (List q)
              Bool
              (ordered_vertices q d g order_g)
              (ordered_vertices q d h order_h)
              (before q d x y)
              same)
            (ordered_forward q d h order_h x y from_h))

const union_order : TopoOrder Vertex3 DecEq_instance_Vertex3 union_graph =
  topo_order_union Vertex3 DecEq_instance_Vertex3 g01 h12 order01 h12_forward

theorem union_list
    : Equal
        (List Vertex3)
        (ordered_vertices Vertex3 DecEq_instance_Vertex3 union_graph union_order)
        ordered_three =
  topo_order_union_vertices Vertex3 DecEq_instance_Vertex3 g01 h12 order01 h12_forward

theorem g01_v0_v1 : Equal Bool (edge Vertex3 g01 V0 V1) True = Proved

theorem union_order_g_edge
    : Equal Bool
        (before
          Vertex3
          DecEq_instance_Vertex3
          V0
          V1
          (ordered_vertices Vertex3 DecEq_instance_Vertex3 union_graph union_order))
        True =
  ordered_forward
    Vertex3
    DecEq_instance_Vertex3
    union_graph
    union_order
    V0
    V1
    (graph_union_edge_left Vertex3 g01 h12 V0 V1 g01_v0_v1)

theorem h12_v1_v2 : Equal Bool (edge Vertex3 h12 V1 V2) True = Proved

theorem union_order_h_edge
    : Equal Bool
        (before
          Vertex3
          DecEq_instance_Vertex3
          V1
          V2
          (ordered_vertices Vertex3 DecEq_instance_Vertex3 union_graph union_order))
        True =
  ordered_forward
    Vertex3
    DecEq_instance_Vertex3
    union_graph
    union_order
    V1
    V2
    (graph_union_edge_right Vertex3 g01 h12 V1 V2 h12_v1_v2)
```

## `stdlib/graphs/topo-order-union-accepts`

- spec: `spec/50-stdlib/62-graphs.md` §22.
- promise class: durable invariant.
- given: the open clients and the `g01`, `h12`, and `order01` fixture
  in the shared accepted prefix.
- expect: all three generic clients check. The closed transfer has the
  same ordered vertices as `order01`; `ordered_forward` proves the
  union's `V0 → V1` and `V1 → V2` edges forward.
- measured: the two closed edges are proved in different input graphs;
  the order law is applied after each edge is admitted to the union.
- claimed: an order for `g` transfers to `graph_union g h` when every
  `h` edge is forward in the list of that order.
- gap: this finite fixture does not prove the transfer for all graphs,
  carriers, orders, or edge predicates.
- why: the first and second edges come from separate graph operands,
  while the output list is fixed by the `g01` order.

## `stdlib/graphs/topo-order-union-backward-h-refuses`

- spec: `spec/50-stdlib/62-graphs.md` §22.
- promise class: durable invariant.
- given: a fresh copy of the complete accepted prefix, followed by the
  `h21` and `h21_forward` declarations below.
- expect: rejects because the `h21_forward` proof does not inhabit its
  stated false premise.
- measured: `h21` has the single edge `V2 → V1`; the premise claims
  that edge is forward in the `g01` order `[V0, V1, V2]`. The local
  refusal is pinned to the parsed `h21_forward` declaration span.
- claimed: this row supplies a false-premise control for the transfer
  contract.
- gap: this refusal precedes any use of `topo_order_union` and would
  also occur if the transfer law were absent. It is not evidence for
  the law. The generic clients and closed edge read-offs above exercise
  the law; the refusal only checks the premise boundary.
- why: `before V2 V1 [V0, V1, V2]` is `False`, so no proof of the
  stated premise can be constructed.

```ken
const h21 : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ False
            }
        })

theorem h21_forward
    : all_edges_forward Vertex3 DecEq_instance_Vertex3 h21
        (ordered_vertices Vertex3 DecEq_instance_Vertex3 g01 order01) =
  λx.
    λy.
      match x {
        V0 ↦
          match y {
            V0 ↦ λpresent. absurd present;
            V1 ↦ λpresent. absurd present;
            V2 ↦ λpresent. absurd present
          };
        V1 ↦
          match y {
            V0 ↦ λpresent. absurd present;
            V1 ↦ λpresent. absurd present;
            V2 ↦ λpresent. absurd present
          };
        V2 ↦
          match y {
            V0 ↦ λpresent. absurd present;
            V1 ↦ λpresent. Proved;
            V2 ↦ λpresent. absurd present
          }
      }
```

The premise refusal is a fixture-bound negative control, not a
conformance observation about diagnostics or source spans. The parsed
`h21_forward` declaration span identifies the failing block; no fixed
byte offset or diagnostic wording is part of the language contract.

## Evidence boundary

Architect's AC-0 measured the generic clients and closed fixture on
scratch; those results are not execution of this seed. The two extracted
Ken fences pass `ken fmt --check`. At exact assembled tip
`bb275256f38750a23d5e3009f3314d556bd3d42a`, raw `ken check` of the original
extracted accepted prefix returned `K0004` / `KernelRejected(TypeMismatch)`
in `h12_forward`. With this respin's proof shapes in extracted scratch
files against that tip's package, the accepted prefix exits 0; adding
`h21_forward` rejects with `K0004` in its declaration span at the false
`V2 → V1` premise. This is raw source-check evidence only; the official
Markdown seed runner has not been run. No CI, publication, landing, or
landed-trust result is claimed.
