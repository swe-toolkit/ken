# Walk composition and order along a walk — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §3.
Frame: `docs/program/issues/SPEC-GRAPHS-WALK-COMPOSITION-ORDER.md`.

The unchanged Ken fences below are exercised by the package-owned
`cat_graphs_walk_composition` test: its positive checks precede an isolated
reversed-order refusal. This is package-test execution, not official
conformance-runner execution, CI, publication, or landing. The test also
compares exact trusted-base identities before and after the package loads;
that measurement is not a claim of a landed trust result.

Shared fixtures use `Vertex3` and its checked `DecEq` instance. The graph
has only the edges `V0 → V1` and `V1 → V2`; `chain_order` is an explicit
`TopoOrder` over `[V0, V1, V2]`. Each walk below has exactly one edge.

```ken
import Core.Classes.LawfulClasses (DecEq)
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, WalkHere, WalkStep, TopoOrder, MkTopoOrder,
   ordered_vertices, before, walk_append, ordered_walk_forward)

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

theorem vertex3_eq_complete (x : Vertex3) (y : Vertex3) :
  Equal Vertex3 x y → Equal Bool (vertex3_eq x y) True =
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

const chain_graph : Graph Vertex3 = MkGraph Vertex3
  (λu. λv. match u {
    V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ False };
    V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ True };
    V2 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False }
  })

const chain_vertices : List Vertex3 =
  Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))

const walk_01 : Walk Vertex3 chain_graph V0 V1 =
  WalkStep Vertex3 chain_graph V0 V0 V1 Proved
    (WalkHere Vertex3 chain_graph V0)

const walk_12 : Walk Vertex3 chain_graph V1 V2 =
  WalkStep Vertex3 chain_graph V1 V1 V2 Proved
    (WalkHere Vertex3 chain_graph V1)

const chain_order : TopoOrder Vertex3 DecEq_instance_Vertex3 chain_graph =
  MkTopoOrder Vertex3 DecEq_instance_Vertex3 chain_graph
    chain_vertices
    Proved
    (λx. match x { V0 ↦ Proved; V1 ↦ Proved; V2 ↦ Proved })
    (λx. λy. match x {
      V0 ↦ match y {
        V0 ↦ λe. absurd e;
        V1 ↦ λe. Proved;
        V2 ↦ λe. absurd e
      };
      V1 ↦ match y {
        V0 ↦ λe. absurd e;
        V1 ↦ λe. absurd e;
        V2 ↦ λe. Proved
      };
      V2 ↦ match y {
        V0 ↦ λe. absurd e;
        V1 ↦ λe. absurd e;
        V2 ↦ λe. absurd e
      }
    })
```

## stdlib/graphs/walk-append-composes-two-chain-edges

- spec: `spec/50-stdlib/62-graphs.md` §3.
- promise class: durable invariant.
- given: the closed walks `walk_01 : V0 → V1` and
  `walk_12 : V1 → V2` on `chain_graph`.
- expect: `walk_append` is definitionally equal to the explicit
  `WalkStep` from `V0` to `V2` whose prefix is `walk_01`.
- measured: the closed expected result retains both checked chain edges.
- claimed: appending two genuine non-trivial walks produces the path
  with the first walk as its prefix.
- the gap: this closed two-edge instance does not prove the generic
  recursion for every pair of walks.
- why: a composition that drops or misorders an edge cannot reduce to
  this expected path.

```ken
theorem walk_append_two_chain_edges :
  Equal (Walk Vertex3 chain_graph V0 V2)
    (walk_append Vertex3 chain_graph V0 V1 V2 walk_01 walk_12)
    (WalkStep Vertex3 chain_graph V0 V1 V2 Proved walk_01)
  = Refl
```

## stdlib/graphs/ordered-walk-forward-on-chain

- spec: `spec/50-stdlib/62-graphs.md` §3.
- promise class: durable invariant.
- given: `chain_order`, the checked edge `V0 → V1`, and `walk_12`
  from `V1` to `V2`.
- expect: `ordered_walk_forward` proves
  `before V0 V2 (ordered_vertices chain_order) = True`.
- measured: the law's result on the explicit three-vertex order.
- claimed: the order places the start before the end of this walk
  with a first edge.
- the gap: the closed chain instance does not establish the generic
  theorem for every `TopoOrder` and walk.
- why: reversing the result direction must not be accepted for the
  same graph, order, and walk inputs.

```ken
theorem ordered_walk_forward_on_chain :
  Equal Bool
    (before Vertex3 DecEq_instance_Vertex3 V0 V2
      (ordered_vertices Vertex3 DecEq_instance_Vertex3
        chain_graph chain_order))
    True =
  ordered_walk_forward
    Vertex3 DecEq_instance_Vertex3 chain_graph chain_order
    V0 V1 V2 Proved walk_12
```

## stdlib/graphs/ordered-walk-reversed-direction-refused

- spec: `spec/50-stdlib/62-graphs.md` §3.
- promise class: durable invariant.
- given: the same graph, `chain_order`, first-edge proof, `walk_12`,
  and exact `ordered_walk_forward` term as the accepting control above.
  Only the expected proposition changes from `before V0 V2` to
  `before V2 V0`.
- expect: the block is refused by the kernel with a type mismatch;
  the law term proves the forward proposition, not its reversal. No
  diagnostic code is pinned.
- measured: the same law term cannot inhabit the reversed proposition.
- claimed: a nonempty walk in a checked order is not accepted in the
  opposite direction.
- the gap: this one chain does not establish the generic order law.
- why: the refusal is isolated to the reversed result, not a changed
  graph, order, edge, or walk fixture.

The package test concatenates the shared-fixture fence and both positive
fences as an accepted client, then appends only this reversed theorem to a
fresh copy of that accepted source. The refusal is a type mismatch for the
reversed proposition, not a missing import or fixture name.

```ken
theorem refuse_reversed_order_on_same_chain_walk :
  Equal Bool
    (before Vertex3 DecEq_instance_Vertex3 V2 V0
      (ordered_vertices Vertex3 DecEq_instance_Vertex3
        chain_graph chain_order))
    True =
  ordered_walk_forward
    Vertex3 DecEq_instance_Vertex3 chain_graph chain_order
    V0 V1 V2 Proved walk_12
```
