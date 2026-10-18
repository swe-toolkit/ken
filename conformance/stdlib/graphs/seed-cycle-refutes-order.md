# A cycle refutes every dependency order — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §2.
Frame: `docs/program/issues/SPEC-GRAPHS-CYCLE-REFUTES-ORDER.md`.

These closed clients exercise the three new §2 public laws. The cycle cases
use explicit graph data and checked `Cycle` witnesses. They do not claim an
executed shipped seed, catalog delivery, CI, or a measured `trusted_base()`
delta. The Architect's D0 results are scratch evidence, not execution of
this seed.

## Closed fixtures

The acyclic graph has only the edge `True → False`; `forward_order` is an
explicit checked order control. The two-cycle has both opposite edges, and the
self-loop has one real loop edge.

```ken
import Core.Classes.LawfulClasses (DecEq, DecEq_instance_Bool)
import Data.Finite.Finite (bool_finite)
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Cycle, MkCycle, WalkHere, WalkStep, TopoOrder,
   MkTopoOrder, OrderOrCycle, dependency_order_or_cycle, ordered_vertices,
   before, before_irreflexive, before_transitive, cycle_refutes_order)

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

const ordered_three : List Vertex3 =
  Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))

theorem before_irreflexive_three_instance :
  Equal Bool
    (before Vertex3 DecEq_instance_Vertex3 V1 V1 ordered_three)
    False =
  before_irreflexive Vertex3 DecEq_instance_Vertex3 V1 ordered_three

theorem before_transitive_three_instance :
  Equal Bool
    (before Vertex3 DecEq_instance_Vertex3 V0 V2 ordered_three)
    True =
  before_transitive
    Vertex3 DecEq_instance_Vertex3 V0 V1 V2 ordered_three
    Proved
    Proved

const forward : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ match v { True ↦ False; False ↦ True };
    False ↦ False
  })

const forward_result : OrderOrCycle Bool DecEq_instance_Bool forward =
  dependency_order_or_cycle Bool bool_finite DecEq_instance_Bool forward

const forward_order : TopoOrder Bool DecEq_instance_Bool forward =
  MkTopoOrder Bool DecEq_instance_Bool forward
    (Cons Bool True (Cons Bool False (Nil Bool)))
    Proved
    (λx. match x { True ↦ Proved; False ↦ Proved })
    (λx. λy. match x {
      True ↦ match y { True ↦ λe. absurd e; False ↦ λe. Proved };
      False ↦ match y { True ↦ λe. absurd e; False ↦ λe. absurd e }
    })

const two_cycle : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ match v { True ↦ False; False ↦ True };
    False ↦ match v { True ↦ True; False ↦ False }
  })

const two_cycle_witness : Cycle Bool two_cycle =
  MkCycle Bool two_cycle True False Proved
    (WalkStep Bool two_cycle False False True Proved
      (WalkHere Bool two_cycle False))

const self_loop : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ match v { True ↦ True; False ↦ False };
    False ↦ False
  })

const self_loop_witness : Cycle Bool self_loop =
  MkCycle Bool self_loop True True Proved
    (WalkHere Bool self_loop True)

theorem two_cycle_refutes_every_order :
  TopoOrder Bool DecEq_instance_Bool two_cycle → Bottom =
  λorder.
    cycle_refutes_order
      Bool DecEq_instance_Bool two_cycle order two_cycle_witness

theorem self_loop_refutes_every_order :
  TopoOrder Bool DecEq_instance_Bool self_loop → Bottom =
  λorder.
    cycle_refutes_order
      Bool DecEq_instance_Bool self_loop order self_loop_witness
```

### stdlib/graphs/before-irreflexive-closed-instance

- spec: `spec/50-stdlib/62-graphs.md` §2.
- given: `V1` and the closed list `[V0, V1, V2]` under the checked
  `DecEq Vertex3` instance.
- expect: `before V1 V1` is `False`; the public `before_irreflexive`
  theorem supplies the same closed result.
- claimed: the scan never places a vertex before itself, even in a
  nonempty list.
- the gap: this one carrier and list do not establish the generic law.

### stdlib/graphs/before-transitive-closed-instance

- spec: `spec/50-stdlib/62-graphs.md` §2.
- given: the closed list `[V0, V1, V2]`, with `V0` before `V1` and
  `V1` before `V2`.
- expect: `V0` is before `V2`; the public `before_transitive`
  theorem combines the two closed premises.
- claimed: a concrete instance of the scan's transitivity law.
- the gap: one three-vertex order does not establish the theorem for
  arbitrary carriers, dictionaries, query vertices, or lists.

### stdlib/graphs/two-cycle-refutes-order

- spec: `spec/50-stdlib/62-graphs.md` §2.
- given: the closed two-cycle and `two_cycle_witness` above, plus any
  alleged `TopoOrder` for that same graph and equality dictionary.
- expect: `two_cycle_refutes_every_order` maps every such order to
  `Bottom`; the cycle witness uses the real `True → False` edge and
  the real `False → True` return edge.
- claimed: the first edge followed by the return walk contradicts
  every forward-edge order.
- the gap: this fixed graph does not establish the generic theorem
  for every graph or carrier.

### stdlib/graphs/self-loop-refutes-order

- spec: `spec/50-stdlib/62-graphs.md` §1.1 and §2.
- given: the closed self-loop and `self_loop_witness` above, whose
  first edge is `True → True` and whose return walk is `WalkHere`.
- expect: `self_loop_refutes_every_order` maps any alleged order to
  `Bottom`; the zero-edge return walk is valid only because the
  separate first edge makes this a cycle.
- claimed: a self-loop also refutes every order; `WalkHere` alone is
  not treated as a cycle.
- the gap: one closed self-loop does not establish the generic theorem.

### stdlib/graphs/acyclic-graph-has-a-checked-order

- spec: `spec/50-stdlib/62-graphs.md` §1.3 and §2.
- given: `forward`, `forward_order`, and
  `forward_result = dependency_order_or_cycle Bool bool_finite
  DecEq_instance_Bool forward`.
- expect: the explicit `forward_order` on `[True, False]` checks its
  uniqueness, coverage, and forward-edge fields, including
  `before True False = True`; the operation result is expected to be
  `HasOrder order` with the same list.
- claimed: a positive acyclic control supplies a genuine checked order
  on the same graph used by the refused constructions.
- the gap: this two-vertex control does not decide order for every
  acyclic graph.

### stdlib/graphs/refuse-reversed-order-on-acyclic-graph

- spec: `spec/50-stdlib/62-graphs.md` §1.3 and §2.
- given: the `forward` graph and the same `MkTopoOrder` fields as
  `forward_order`, but with `[False, True]`.
- expect: the block is refused at `all_edges_forward`: the actual
  `True → False` edge is backward in the proposed list.
- claimed: the order must respect every actual graph edge.
- the gap: one reversed list does not establish refusal of every
  invalid order.

```ken
const refuse_reversed_forward_order :
  TopoOrder Bool DecEq_instance_Bool forward =
  MkTopoOrder Bool DecEq_instance_Bool forward
    (Cons Bool False (Cons Bool True (Nil Bool)))
    Proved
    (λx. match x { True ↦ Proved; False ↦ Proved })
    (λx. λy. match x {
      True ↦ match y { True ↦ λe. absurd e; False ↦ λe. Proved };
      False ↦ match y { True ↦ λe. absurd e; False ↦ λe. absurd e }
    })
```

### stdlib/graphs/refuse-order-on-two-cycle

- spec: `spec/50-stdlib/62-graphs.md` §2.
- given: the `two_cycle` graph and the same `MkTopoOrder` fields as
  `forward_order`.
- expect: the block is refused at `all_edges_forward`: the actual
  `False → True` edge is backward in `[True, False]`.
- claimed: a checked order cannot coexist with the closed two-cycle.
- the gap: this one candidate order does not establish the generic
  cycle-refutes-order theorem.

```ken
const refuse_order_on_two_cycle :
  TopoOrder Bool DecEq_instance_Bool two_cycle =
  MkTopoOrder Bool DecEq_instance_Bool two_cycle
    (Cons Bool True (Cons Bool False (Nil Bool)))
    Proved
    (λx. match x { True ↦ Proved; False ↦ Proved })
    (λx. λy. match x {
      True ↦ match y { True ↦ λe. absurd e; False ↦ λe. Proved };
      False ↦ match y { True ↦ λe. absurd e; False ↦ λe. absurd e }
    })
```

### stdlib/graphs/refuse-cycle-with-an-absent-edge

- spec: `spec/50-stdlib/62-graphs.md` §1.1 and §2.
- given: the acyclic `forward` graph and a proposed `Cycle` whose
  first edge is `False → True`, absent from `forward`; its return
  walk uses the actual `True → False` edge.
- expect: when appended alone to the positive client, the block is
  refused at the first-edge proof: `edge forward False True` computes
  to `False`, so `Proved` cannot inhabit `Equal Bool False True`.
- claimed: `Cycle` cannot be forged from a walk and a proof of an
  edge the graph does not supply.
- the gap: this single absent-edge block does not establish refusal
  for every invalid cycle construction.

```ken
const refuse_cycle_on_acyclic_graph : Cycle Bool forward =
  MkCycle Bool forward False True Proved
    (WalkStep Bool forward True True False Proved
      (WalkHere Bool forward True))
```

Architect's AC-0 results checked the three generic law forms and
rejected the reported false-law mutations on scratch. CV separately
checked the positive fixture fence as a temporary catalog client with
an existing `ken` binary: rc 0. Appending each refused block alone to
that positive client returned rc 1, `K0004`: the reversed order, the
order on `two_cycle`, and the absent-edge `Cycle`. These scratch clients
do not execute this shipped seed or establish catalog delivery. No
Cargo test, CI result, publication, landing, or post-publication
`trusted_base()` measurement is claimed.
