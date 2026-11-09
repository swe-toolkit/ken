# Graphs from successor lists

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §14, Spec child
`f2cb02d3761459d9fcfd9f1199676a1e0bf0c3d2`.
Frame: `docs/program/issues/SPEC-GRAPHS-SUCCESSOR-LIST-ADAPTOR.md`.

Status: authored, not run as Ken or through the official conformance runner.
The closed client exercises the new generic laws on one three-vertex
carrier; it does not replace their checked generic proofs. The graph fixture
also includes a repeated listed successor and a listed self-loop. Its
observable edges remain Boolean.

The Architect's AC-0 report records a scratch check on base
`ec8ff0f308eb675d75a4f94f273d960785d79349`: no added `Axiom`, identical
scratch trusted-identity sets (114 → 114), and both generic proofs rejecting
under constant-edge mutations. Those are reported scratch measurements, not
an execution of this seed, an official conformance result, CI, publication,
landing, or landed-trust measurement.

## Accepted client

Accept the whole client below before testing the isolated refusal. Its
successor function lists `V2` before two copies of `V1` for `V0`, lists `V2`
for `V1`, and lists `V2` itself for `V2`. The sound-law instance uses a
non-head list member; the complete-law instance uses a head member.

```ken
import Core.Classes.LawfulClasses (DecEq)

import Data.Collections.Derived (list_elem, list_elem_head, list_elem_later)

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

theorem vertex3_eq_sound
      (x : Vertex3) (y : Vertex3)
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

theorem vertex3_eq_complete
      (x : Vertex3) (y : Vertex3)
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
    V0 ↦ Cons Vertex3 V2
      (Cons Vertex3 V1 (Cons Vertex3 V1 (Nil Vertex3)));
    V1 ↦ Cons Vertex3 V2 (Nil Vertex3);
    V2 ↦ Cons Vertex3 V2 (Nil Vertex3)
  }

const successor_graph : Graph Vertex3 =
  graph_from_successors Vertex3 DecEq_instance_Vertex3 successors

theorem listed_v0_v1_same_form_control
    : Equal Bool (edge Vertex3 successor_graph V0 V1) True =
  Proved

theorem v0_v1_tail_membership_control
    : list_elem Vertex3 V1 (successors V0) =
  list_elem_later
    Vertex3
    V1
    V2
    (Cons Vertex3 V1 (Cons Vertex3 V1 (Nil Vertex3)))
    (list_elem_head Vertex3 V1 (Cons Vertex3 V1 (Nil Vertex3)))

theorem v0_v1_is_a_listed_successor
    : list_elem Vertex3 V1 (successors V0) =
  graph_from_successors_sound
    Vertex3 DecEq_instance_Vertex3 successors V0 V1
    listed_v0_v1_same_form_control

theorem v1_v2_is_a_listed_successor
    : Equal Bool (edge Vertex3 successor_graph V1 V2) True =
  graph_from_successors_complete
    Vertex3 DecEq_instance_Vertex3 successors V1 V2
    (list_elem_head Vertex3 V2 (Nil Vertex3))

theorem listed_v0_v2_is_an_edge
    : Equal Bool (edge Vertex3 successor_graph V0 V2) True =
  Proved

theorem listed_v2_self_loop_is_an_edge
    : Equal Bool (edge Vertex3 successor_graph V2 V2) True =
  Proved
```

### stdlib/graphs/graph-from-successors-sound-listed-edge

- spec: §14, `graph_from_successors_sound`.
- promise class: durable invariant.
- given: the closed `successor_graph` and the listed `V0 → V1` edge.
- expect: the public sound law derives list membership from the checked
  true edge; the membership proof selects `V1` after the first `V2`.
- measured: a checked true-edge premise, the law's resulting `list_elem`
  conclusion, and an independent tail-membership control.
- claimed: every true adaptor edge corresponds to membership in the
  source's successor list.
- the gap: this one closed instance does not establish the generic
  theorem over arbitrary carriers and successor functions.
- why: the premise is independently checked as `Proved`; the target is
  the second list item, with a duplicate following it.

### stdlib/graphs/graph-from-successors-complete-listed-edge

- spec: §14, `graph_from_successors_complete`.
- promise class: durable invariant.
- given: `V2` at the head of `V1`'s successor list.
- expect: the public complete law derives a checked true `V1 → V2`
  edge from that membership proof.
- measured: the head membership proof and the law's `Equal Bool` result.
- claimed: a listed target is an edge of the adapted graph.
- the gap: this one closed instance does not establish the generic
  theorem over arbitrary carriers and successor functions.
- why: the membership proof is built from the input list, not inferred
  from the graph edge being tested.

### stdlib/graphs/graph-from-successors-listed-edge-controls

- spec: §14, `graph_from_successors`.
- promise class: durable invariant.
- given: the repeated `V1` entry in `V0`'s list, the listed `V0 → V2`
  control, and the `V2` self-loop.
- expect: `V0 → V1`, `V0 → V2`, and `V2 → V2` evaluate to `True`.
- measured: Boolean edge observations for a repeated target, another
  listed target, and a listed self-loop.
- claimed: listed successors, including a self-loop, produce true
  Boolean edges; duplicate occurrences are not a second edge value.
- the gap: the API exposes Boolean adjacency, not edge multiplicity; the
  generic checked laws establish the full list-membership contract.
- why: these closed controls exercise list-derived observations without
  requiring a traversal or a finite-vertex certificate.

## Isolated refused claim

Start from a fresh copy of the entire accepted client and append only the
following theorem. Its proposition and `Proved` form match the accepted
`listed_v0_v2_is_an_edge` control; only the target changes. `V0` is absent
from `successors V0`. Require rejection at this theorem, not at an import,
fixture, or accepted declaration. No diagnostic code is pinned.

### stdlib/graphs/reject-edge-for-unlisted-successor

- spec: §14, `graph_from_successors` and its membership contract.
- promise class: durable invariant.
- given: the accepted control for `V0 → V2` and the same source `V0`.
- expect: reject the claim that `V0 → V0` is an edge.
- measured: same-form true-edge claims for a listed target and an
  unlisted target from one source.
- claimed: an absent target is not an edge of the graph built from that
  source's list.
- the gap: one absent-target counterexample does not establish the
  generic equivalence between edges and list membership.
- why: changing only the target from listed `V2` to absent `V0` flips
  the expected result; a constant-true edge implementation accepts the
  refused claim.

Append only this theorem to a fresh copy of the accepted client:

```ken
theorem refuse_edge_for_unlisted_successor
    : Equal Bool (edge Vertex3 successor_graph V0 V0) True =
  Proved
```

## Coverage boundary

The client checks one listed edge through each public law, includes a
same-form positive edge control, and refuses a claim for an absent target.
The direct observations also cover a repeated successor occurrence and a
listed self-loop on this three-vertex carrier. These finite cases do not
replace the generic checked laws and do not establish behavior for every
`DecEq`, successor function, or carrier. No `Finite` argument or total
vertex enumeration is part of the adaptor's public type.

## Evidence boundary

The Architect's AC-0 is scratch evidence on the exact base, not an execution
of these seed blocks. CV has not run the seed through Ken or an official
conformance runner. No package test, CI, publication, landing, or
landed-trust measurement is claimed.
