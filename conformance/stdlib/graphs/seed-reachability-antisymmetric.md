# Reachability is antisymmetric under a dependency order

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §12.
Frame: `docs/program/issues/SPEC-GRAPHS-REACHABILITY-ANTISYMMETRIC.md`.

The three rows follow AC-2 and the Architect's contract ruling
(`evt_4c72a4g06fk79`). The accepted client builds an order on the
one-way chain `V0 → V1 → V2`, supplies the real forward leg, and uses
the law to show that a putative back leg from `V1` to `V0` yields
Bottom. The law-row refusal restates that claim on a two-cycle. The
adjacent order-only refusal attempts only the two-cycle order
construction. Refusals assert `KernelRejected` only; no diagnostic text
or code is pinned, and no `Refl` appears in a Ken obligation.

The Architect's ruling requires `Proved` for order obligations,
`λe. absurd e` for absent edges, and `λe. Proved` for actual forward
edges in the chain order. The cycle order uses `λe. Proved` for both
cycle edges, making it impossible to place them both forward. The
Architect/Foundation D0 results are scratch evidence, not CV execution
of this seed. CV has not run this seed against an assembled candidate
or the official conformance runner.

## Shared client: one-way chain and checked order

The graph has exactly the edges `V0 → V1` and `V1 → V2`. The order
lists `V0`, `V1`, `V2`, in that order. The law application assumes a
back leg and derives a contradiction from the distinct endpoints.

```ken
import Core.Classes.LawfulClasses (DecEq)

import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, TopoOrder, MkTopoOrder, graph_reachable, graph_reachable_antisymmetric)

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

const vertex3_finite : Finite Vertex3 =
  MkFinite
    Vertex3
    (Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3))))
    (λv.
      match v {
        V0 ↦ list_elem_head Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)));
        V1 ↦
          list_elem_later
            Vertex3
            V1
            V0
            (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))
            (list_elem_head Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)));
        V2 ↦
          list_elem_later
            Vertex3
            V2
            V0
            (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))
            (list_elem_later
              Vertex3
              V2
              V1
              (Cons Vertex3 V2 (Nil Vertex3))
              (list_elem_head Vertex3 V2 (Nil Vertex3)))
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

const ordered_three : List Vertex3 =
  Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))

const chain_order : TopoOrder Vertex3 DecEq_instance_Vertex3 chain =
  MkTopoOrder
    Vertex3
    DecEq_instance_Vertex3
    chain
    ordered_three
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
              V2 ↦ λe. absurd e
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

theorem v0_reaches_v1
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 (vertex3_eq V1)) True =
  Proved

theorem v1_does_not_reach_v0
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V1 (vertex3_eq V0)) True
      → Bottom =
  λback.
    absurd
      (vertex3_eq_complete
        V0
        V1
        (graph_reachable_antisymmetric
          Vertex3
          vertex3_finite
          DecEq_instance_Vertex3
          chain
          chain_order
          V0
          V1
          v0_reaches_v1
          back))
```

### stdlib/graphs/reachability-antisymmetric-chain-back-leg

- spec: §12, `graph_reachable_antisymmetric`.
- promise class: durable invariant.
- given: the one-way chain, its checked order, and the real forward
  reachability proof from `V0` to `V1`.
- expect: assuming `V1` reaches `V0` lets the law equate `V0` and
  `V1`; `vertex3_eq_complete` turns that equality into the impossible
  `Equal Bool False True`, proving Bottom.
- control: `v0_reaches_v1` is a real forward reachability proof with
  the same `Proved` form used by the back-leg premise.
- claimed: under a valid order, the law refutes the putative back leg;
  the accepted witness exercises the law's force, not only its diagonal.
- the gap: this finite chain does not establish the generic law.

## Law-row refusal on the two-cycle

Append only this graph, order attempt, forward leg, and law-derived
claim to the shared client. The cycle has both edges between `V0` and
`V1`; there is no edge incident to `V2`. The proposed order uses the
same list and obligations as `chain_order`, with both cycle edges
claimed forward. The block is expected to be refused with
`KernelRejected`; no diagnostic text or code is pinned.

```ken
const cycle2 : Graph Vertex3 =
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
              V0 ↦ True;
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

const cycle2_order : TopoOrder Vertex3 DecEq_instance_Vertex3 cycle2 =
  MkTopoOrder
    Vertex3
    DecEq_instance_Vertex3
    cycle2
    ordered_three
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
              V2 ↦ λe. absurd e
            };
          V1 ↦
            match y {
              V0 ↦ λe. Proved;
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

theorem v0_reaches_v1_cycle2
    : Equal Bool (graph_reachable Vertex3 vertex3_finite cycle2 V0 (vertex3_eq V1)) True =
  Proved

theorem v1_does_not_reach_v0_cycle2
    : Equal Bool (graph_reachable Vertex3 vertex3_finite cycle2 V1 (vertex3_eq V0)) True
      → Bottom =
  λback.
    absurd
      (vertex3_eq_complete
        V0
        V1
        (graph_reachable_antisymmetric
          Vertex3
          vertex3_finite
          DecEq_instance_Vertex3
          cycle2
          cycle2_order
          V0
          V1
          v0_reaches_v1_cycle2
          back))
```

### stdlib/graphs/reachability-antisymmetric-cycle2-law-refusal

- spec: §12, `graph_reachable_antisymmetric`.
- promise class: durable invariant.
- given: the same proposed `cycle2_order` as the order-only row,
  plus the forward leg and the law-derived claim that the back leg is
  impossible.
- expect: the isolated law-row block is refused with error class
  `KernelRejected`.
- control: the accepted chain client has the same order fields and law
  application shape on a graph with no cycle.
- claimed: the refused row matches the accepted chain's law-derived
  claim except that the two-cycle has no valid forward-edge order.
- the gap: this fixed cycle does not establish the generic law or order
  theorem.

## Order-only refusal on the same two-cycle

Append only the `cycle2` graph and `cycle2_order` construction to the
shared client. Do not include a reachability law application. This
isolates refusal of the order certificate itself. It pins only refusal
with error class `KernelRejected`, never diagnostic text or code.

```ken
const cycle2 : Graph Vertex3 =
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
              V0 ↦ True;
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

const cycle2_order : TopoOrder Vertex3 DecEq_instance_Vertex3 cycle2 =
  MkTopoOrder
    Vertex3
    DecEq_instance_Vertex3
    cycle2
    ordered_three
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
              V2 ↦ λe. absurd e
            };
          V1 ↦
            match y {
              V0 ↦ λe. Proved;
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
```

### stdlib/graphs/reachability-antisymmetric-order-only-refusal

- spec: §1.3, dependency-order certificate; §12,
  `graph_reachable_antisymmetric`.
- promise class: durable invariant.
- given: the two-cycle and the proposed order on
  `[V0, V1, V2]`.
- expect: the isolated order construction is refused with error class
  `KernelRejected`.
- control: `chain_order` accepts the same ordered list and obligations
  on the one-way chain.
- claimed: the law-row refusal is attributable to the order premise,
  not to a failure to build the shared vertex and equality fixtures.
- the gap: this fixed cycle and order attempt do not establish the
  generic order theorem.

## Evidence boundary

The Architect's AC-0 and seed-contract rulings and Foundation's D0
report are attributed scratch results, not CV execution of these rows.
CV has not run the seed against an assembled candidate, run package
tests, or executed the official conformance runner. The finite cases
do not replace the checked generic proof, establish a landed trust
result, or imply CI, publication, or landing.
