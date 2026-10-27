# Reachability carries back along a genuine walk

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §5.
Frame: `docs/program/issues/SPEC-GRAPHS-REACHABILITY-ALONG-WALK.md`.

The graph is the directed three-vertex chain `V0 → V1 → V2` and the
target is `is_v2`; `V0` is not itself a target. The public law carries
reachability back from `V1` along a one-edge walk to `V0`, and from
`V2` along the composed two-edge walk to `V0`. Independently checked
`Proved` values establish reachability at both starts, a zero-edge
hit at `V0` for `is_v0`, and the **absence** of reachability from
`V2` to `V0`. The fixture exercises a closed instance, not a generic
proof. All cases promise durable invariants under intended extensions.

## Accepted shared client

Accept this **whole** client before testing the isolated refusal.
Its value checks do not invoke `graph_reachable_along_walk`.

```ken
import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, WalkHere, WalkStep, walk_append, graph_reachable, graph_reachable_along_walk)

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

fn is_v2 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ False;
    V2 ↦ True
  }

fn is_v0 (v : Vertex3) : Bool =
  match v {
    V0 ↦ True;
    V1 ↦ False;
    V2 ↦ False
  }

const walk_01 : Walk Vertex3 chain V0 V1 =
  WalkStep Vertex3 chain V0 V0 V1 Proved (WalkHere Vertex3 chain V0)

const walk_12 : Walk Vertex3 chain V1 V2 =
  WalkStep Vertex3 chain V1 V1 V2 Proved (WalkHere Vertex3 chain V1)

theorem start_not_target : Equal Bool (is_v2 V0) False = Proved

theorem v1_reaches_v2 : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V1 is_v2) True =
  Proved

theorem v0_reaches_v2_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True =
  Proved

theorem v0_reaches_v2_along_walk
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True =
  graph_reachable_along_walk Vertex3 vertex3_finite chain V0 V1 is_v2 walk_01 v1_reaches_v2

theorem v0_reaches_v2_along_two_edges
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True =
  graph_reachable_along_walk
    Vertex3
    vertex3_finite
    chain
    V0
    V2
    is_v2
    (walk_append Vertex3 chain V0 V1 V2 walk_01 walk_12)
    Proved

theorem v0_reaches_v0 : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v0) True =
  Proved

theorem v2_misses_v0 : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) False =
  Proved
```

### stdlib/graphs/reachability-along-one-edge

- spec: §5, `graph_reachable_along_walk`.
- promise class: durable invariant.
- given: the one-edge checked `walk_01` from `V0` to `V1` and
  independently checked `v1_reaches_v2`.
- expect: applying the law gives `V0` reachability for `is_v2`;
  `start_not_target` excludes a vacuous hit at the start, and
  `v0_reaches_v2_value` computes the answer independently.
- why: reachability at the walk endpoint carries backward to its
  start without reversing or omitting a real edge.

### stdlib/graphs/reachability-along-composed-two-edge-walk

- spec: §5 and §3, `walk_append`.
- promise class: durable invariant.
- given: `walk_01` and `walk_12` compose into `V0 → V2`, where
  `is_v2 V2 = True`.
- expect: applying the law with the composed walk and the checked
  endpoint premise gives `V0` reachability for `is_v2`.
- why: the closed law application covers a walk with two checked
  edges, not only `WalkHere` or one edge.

### stdlib/graphs/reachability-same-shape-true-control

- spec: §1.2's reflexive-transitive reachability and §5's boundary.
- promise class: durable invariant.
- given: the same `is_v0` target and `Proved` form as the refusal,
  with the true start `V0`.
- expect: `v0_reaches_v0` accepts and the independent
  `v2_misses_v0` accepts its `False` computation.
- why: refusal below must stem from a false conclusion, not the
  proof form, target predicate or finite graph fixture.

## Isolated refused claim

On a **fresh complete copy** of the accepted client, append **only**
`seed_neg` below. Its claimed `True` differs from the independent
`v2_misses_v0` result `False`, although `v0_reaches_v0` demonstrates
the same `Equal Bool (graph_reachable ... is_v0) True = Proved` form
can accept on this fixture. The graph is one-way: `V2` cannot reach
`V0`. Refuse at the appended proposition, not at an import,
fixture, value check or valid law application.

### stdlib/graphs/reject-chain-end-reaching-start

- spec: §5, false converse, and §1.2's reachability semantics.
- promise class: durable invariant.
- given: `V2`, `is_v0`, the entire accepted chain client.
- expect: reject the false end-to-start reachability claim.
- why: a forward walk and a target reachable earlier in the chain
  cannot be used to infer reachability at the later vertex.

```ken
theorem seed_neg : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) True =
  Proved
```

## Evidence boundary

The Architect's detached D0 on `454adf947` with patch SHA-256
`d18ca75b62e6a052e8d5881a8dc799e01cebed0d4f49b7b34140e9b33996da08`
checked the generic proof without a new `Axiom`, and with unchanged
scratch trusted GlobalId sets (114 → 114). The ruled accepted seed
client checked as a whole after root-loading Dependency; appending
only this false proposition returned `KernelRejected TypeMismatch`.
The author separately rechecked the exact accepted client (identical
to the ruled D0 source byte for byte) on a detached `454adf947` worktree
with only that patch applied, using the preexisting Architect shared
Ken CLI binary. The full positive returned rc 0; a fresh full accepted
prefix plus only `seed_neg` returned rc 1/K0004 `KernelRejected
TypeMismatch` at span 3093–3194 of the appended theorem. On the
unpatched base, the independent value controls without the absent-law
import or applications accepted (rc 0), while the exact full client
failed at the missing public `graph_reachable_along_walk` import
(rc 1/E0012 `UnboundName`). Logs are under `/workspaces/ken/local/`
with names `spec-author-reach-{positive,false,base-values,base-laws}.log`.
The overlay and scratch worktree were removed. The author reused a
preexisting binary, not an independently built exact-base CLI.
These earlier closed checks do not prove the generic law. The catalog
candidate exports a checked proof, and its targeted package test now
root-loads the whole accepted seed against that export, rejects the
isolated end-to-start claim, and measures unchanged trusted GlobalIds.
This is candidate package-test execution, not official conformance, CI,
publication, landing or landed trust.
