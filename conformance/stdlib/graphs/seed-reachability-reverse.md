# Reachability in the reversed graph

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §8.
Frame: `docs/program/issues/SPEC-GRAPHS-REVERSE-REACHABILITY.md`.
Ruling: Architect AC-0 `evt_2k0h90spm6zzb`.

The closed client reuses the three-vertex chain `V0 → V1 → V2`. Its
checked two-edge walk reverses to a walk from `V2` to `V0` in the
reversed graph. With target `is_v0`, the accepted law and the independent
value control establish reachability there. The same query on the forward
chain is false. These closed cases discriminate the direction; they do
not prove the generic reversal forms.

## Accepted client

Accept this complete client before testing the isolated refusal. It uses
only the four public forms specified in §8 and the existing graph and
reachability interface.

```ken
import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, WalkHere, WalkStep, edge, graph_reverse,
   graph_reverse_edge, walk_reverse, graph_reachable,
   graph_reachable_complete, graph_reachable_reverse)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
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

fn is_v0 (v : Vertex3) : Bool =
  match v {
    V0 ↦ True;
    V1 ↦ False;
    V2 ↦ False
  }

const chain_walk : Walk Vertex3 chain V0 V2 =
  WalkStep Vertex3 chain V0 V1 V2 Proved
    (WalkStep Vertex3 chain V0 V0 V1 Proved (WalkHere Vertex3 chain V0))

const reversed_walk : Walk Vertex3 (graph_reverse Vertex3 chain) V2 V0 =
  walk_reverse Vertex3 chain V0 V2 chain_walk

theorem reverse_edge_law_client (s : Vertex3) (t : Vertex3) :
  Equal Bool
    (edge Vertex3 (graph_reverse Vertex3 chain) s t)
    (edge Vertex3 chain t s) =
  graph_reverse_edge Vertex3 chain s t

theorem reversed_edge_v2_v1 :
  Equal Bool (edge Vertex3 (graph_reverse Vertex3 chain) V2 V1) True =
  Proved

theorem reversed_edge_v1_v0 :
  Equal Bool (edge Vertex3 (graph_reverse Vertex3 chain) V1 V0) True =
  Proved

theorem reversed_edge_v1_v2_absent :
  Equal Bool (edge Vertex3 (graph_reverse Vertex3 chain) V1 V2) False =
  Proved

theorem forward_edge_v2_v0_absent :
  Equal Bool (edge Vertex3 chain V2 V0) False = Proved

theorem forward_edge_v2_v1_absent :
  Equal Bool (edge Vertex3 chain V2 V1) False = Proved

theorem forward_edge_v2_v2_absent :
  Equal Bool (edge Vertex3 chain V2 V2) False = Proved

theorem v0_is_target : Equal Bool (is_v0 V0) True = Proved

theorem v2_is_not_target : Equal Bool (is_v0 V2) False = Proved

theorem reversed_v2_reaches_v0_value :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite
      (graph_reverse Vertex3 chain) V2 is_v0)
    True =
  Proved

theorem reversed_v2_reaches_v0_from_reversed_walk :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite
      (graph_reverse Vertex3 chain) V2 is_v0)
    True =
  graph_reachable_complete
    Vertex3 vertex3_finite (graph_reverse Vertex3 chain)
    V2 V0 is_v0 reversed_walk Proved

theorem reversed_v2_reaches_v0_by_law :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite
      (graph_reverse Vertex3 chain) V2 is_v0)
    True =
  graph_reachable_reverse
    Vertex3 vertex3_finite chain V0 V2 is_v0 chain_walk Proved

theorem forward_v2_misses_v0_value :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite chain V2 is_v0)
    False =
  Proved
```

### `stdlib/graphs/reverse-edge-flips-the-closed-chain`

- spec: §§1.1 and 8, `graph_reverse` and `graph_reverse_edge`.
- promise class: durable invariant.
- given: the finite `chain` and generic `reverse_edge_law_client` above.
- expect: `graph_reverse_edge` applied at variable `Vertex3` endpoints
  gives the swapped-edge equality. Closed values check both edges used by
  `reversed_walk`, an absent reverse-direction edge, and all three absent
  forward edges out of `V2`.
- measured: reversed `V2 → V1` and `V1 → V0` are `True`; reversed
  `V1 → V2` is `False`; the three `chain` edges from `V2` are `False`.
- claimed: on this fixture, reversed edges match the swapped `chain`
  edges without adding a shortcut; §8's public law states the general
  edge equation.
- gap: this closed carrier does not prove the edge equation for arbitrary
  carriers or graphs; the generic theorem's type states that contract.
- why: the edge controls pin both edges required by the reversed walk
  and the lack of any forward exit from its endpoint.

### `stdlib/graphs/walk-reverse-returns-the-two-edge-reversal`

- spec: §§1.1 and 8, `walk_reverse`.
- promise class: durable invariant.
- given: `chain_walk`, a checked two-edge `Walk` from `V0` to `V2`.
- expect: `reversed_walk` is a checked walk from `V2` to `V0` in the
  reversed chain; the public function constructs reversed data, not merely
  a proposition about it.
- measured: a value of the returned type is produced from the exact
  source walk by `walk_reverse`.
- claimed: the walk from `V0` to `V2` reverses into a walk from `V2` to
  `V0` in the graph with flipped edges.
- gap: this two-edge instance does not establish the function's behavior
  for arbitrary walks or graphs.
- why: the endpoint indices and each checked reversed edge constrain the
  returned `Walk`.

### `stdlib/graphs/reachability-reverses-along-a-walk`

- spec: §8, `graph_reachable_reverse`, and §1.2,
  `graph_reachable_complete`.
- promise class: durable invariant.
- given: `chain_walk`, the target hit `is_v0 V0 = True`, and the same
  carrier and finite certificate for both graph observations.
- expect: `graph_reachable_reverse` proves that `V2` reaches target
  `V0` in `graph_reverse chain`. The independent `_value` control and
  the `graph_reachable_complete` application to `reversed_walk` also
  accept. In the forward graph, the same `V2` does not reach `V0`.
- measured: `V2` is not itself a target; the reverse reachability
  computes `True`, while the forward reachability computes `False`.
- claimed: for this walk and target hit at `V0`, reachability from `V2`
  follows in the reversed graph; §8 states the generic law.
- gap: the finite controls distinguish the reversed result from the
  forward result but do not prove the generic law.
- why: the reversed two-edge walk reaches `V0` from `V2`; the forward
  chain has no edge out of `V2`.

## Isolated refused claim

Start from a fresh complete copy of the accepted client and append only
`seed_neg`. It asserts the same V2-to-V0 reachability result, but drops
`graph_reverse` from the graph argument. The accepted reversed `_value`
control establishes that the same result shape accepts for the reversed
graph; `forward_v2_misses_v0_value` checks the forward result as `False`.
Refusal must occur at the appended proposition, not at an import, fixture,
walk, edge control, or earlier calculation. Do not pin a diagnostic.

### `stdlib/graphs/refuse-forward-reachability-from-chain-end-to-start`

- spec: §8's reversed-graph law and §1.2's reachability semantics.
- promise class: durable invariant.
- given: the full accepted client and `forward_v2_misses_v0_value`; append
  only the theorem below.
- expect: refuse the claim that `V2` reaches target `V0` in `chain`.
- measured: the same-form reachability claim accepts for the reversed
  chain, while the corresponding forward value independently computes
  `False`.
- claimed: the §8 conclusion is about the reversed graph, not the
  original forward graph.
- gap: this single chain is a counterexample to the false forward claim;
  it does not characterize all graphs or walks.
- why: changing only the graph argument from `graph_reverse chain` to
  `chain` changes the observed reachability from `True` to `False`.

```ken
theorem seed_neg :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite chain V2 is_v0)
    True =
  Proved
```

## Evidence boundary

Architect's AC-0 D0 on exact base `77d13c94b22f7b9f7886edf610d8a7c3b6db31b9`
checked the four public forms and the core AC-2 client: the two-edge law
application, reversed `True` value, forward `False` value, and isolated
false-forward refusal. Those are scratch measurements, not execution of
this newly authored file or official conformance.

Foundation's handoff reports the named package-owned
`cat_graphs_reverse_reachability` target passed 2/2 on candidate
`d018c24fd96296d0541fdc6cdb9198f244d8b737`. The separate
`cat_graphs_dependency` run timed out without a verdict. Neither result
executes this seed. No official conformance-runner execution, CI,
publication, landing, or landed-trust result is claimed.
