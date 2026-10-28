# Reachability is monotone in the target predicate

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §6.
Ruling: Architect AC-0 `evt_42wetx9fh61ta`.

The closed client applies the target-monotonicity law with its pointwise
inclusion premise on a three-vertex directed chain. The narrower target
contains `V1`; the strictly wider target contains `V1` and `V2`. From
`V0`, both reachability values are independently checked with `Proved`,
and the law is applied from the narrower reachability result. From
`V2`, the wider target is reachable but the narrower target is not. The
refused converse is isolated after the complete accepted client. These
closed cases do not prove the generic law.

## Accepted client

Accept this whole client before testing the isolated refusal. It uses the
same three-vertex chain as `seed-reachability-along-walk.md`.

```ken
import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, graph_reachable, graph_reachable_target_monotone)

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

fn is_v1 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ True;
    V2 ↦ False
  }

fn not_v0 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ True;
    V2 ↦ True
  }

theorem v1_inside_not_v0
    : (v : Vertex3) → Equal Bool (is_v1 v) True → Equal Bool (not_v0 v) True =
  λv.
    match v {
      V0 ↦ λh. absurd h;
      V1 ↦ λh. Proved;
      V2 ↦ λh. Proved
    }

theorem narrow_misses_start : Equal Bool (is_v1 V0) False = Proved

theorem wider_misses_start : Equal Bool (not_v0 V0) False = Proved

theorem narrow_contains_v1 : Equal Bool (is_v1 V1) True = Proved

theorem wider_contains_v1 : Equal Bool (not_v0 V1) True = Proved

theorem narrow_misses_v2 : Equal Bool (is_v1 V2) False = Proved

theorem wider_contains_v2 : Equal Bool (not_v0 V2) True = Proved

theorem v0_reaches_v1
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v1) True =
  Proved

theorem v0_reaches_not_v0_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 not_v0) True =
  Proved

theorem v0_reaches_not_v0_by_law
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 not_v0) True =
  graph_reachable_target_monotone
    Vertex3
    vertex3_finite
    chain
    V0
    is_v1
    not_v0
    v1_inside_not_v0
    v0_reaches_v1

theorem v2_reaches_not_v0
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 not_v0) True =
  Proved

theorem v2_misses_v1
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v1) False =
  Proved
```

### `stdlib/graphs/reachability-target-monotone-strict-widening`

- spec: §6, `graph_reachable_target_monotone`.
- promise class: durable invariant.
- given: the three-vertex chain, the pointwise inclusion proof, and the
  `V0` reachability result for the narrower target.
- expect: the law proves reachability for `not_v0` from `V0`; the two
  reachability values are independently checked with `Proved`. Controls
  show that `V0` is not already a target and that `V2` belongs only to
  the wider target.
- measured: strict target widening on the same carrier and graph, plus
  agreement between the law application and the independent wider-value
  check.
- claimed: affirmative reachability for a target carries to any wider
  target that contains it pointwise.
- gap: this finite example does not prove the universally quantified
  law or cover all graph carriers and target predicates.
- why: the edge `V0 → V1` reaches the narrow target; the second edge
  makes the wider target strict without making the start a target.

## Isolated refused claim

Start from a fresh complete copy of the accepted client and append only
`seed_neg`. At `V2`, the wider target is reachable because it contains
`V2`; the narrower target is not reachable because no edge leaves `V2`.
The accepted `v2_reaches_not_v0` has the same graph, start, result
shape, and `Proved` form, changing only the target predicate. Require
refusal at the appended `Proved`, not at an import, fixture, or earlier
calculation.

### `stdlib/graphs/reject-reachability-monotonicity-converse`

- spec: §6, which does not state the converse of target monotonicity.
- promise class: durable invariant.
- given: the same graph and `V2` start, with the wider-target positive
  control and the narrower-target `False` control in the accepted prefix.
- expect: refuse the false claim that `V2` reaches the narrower target.
- measured: the same-form wider-target claim accepts while the narrower
  target independently evaluates to `False`.
- claimed: reachability of a wider target does not imply reachability
  of a narrower target.
- gap: this one counterexample does not characterize every pair of
  target predicates.
- why: target inclusion points from the narrower predicate to the
  wider one; it cannot be reversed.

```ken
theorem seed_neg : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v1) True =
  Proved
```

## Evidence boundary

Architect's AC-0 D0 on `abafe3963` checked the generic law and an
independent generic client with no new `Axiom`, and identical scratch
trusted GlobalId sets (114 → 114). Its separate closed three-vertex
client applied the law on the `V0 → V1 → V2` chain, independently
checked the `V0` results, and rejected the false converse from `V2`.
The D0 patch SHA-256 is
`02da799e35e9f0ea9d26994579da22709b1c68d6704adda2ca501f12f3fe960a`.
That patch and its checks are local scratch evidence, not a §6 catalog
export or landed proof. The §6 catalog export is absent on this
seed's authoring base. This authored seed is a conformance obligation;
it does not claim shipped §6 execution, official conformance, CI,
publication, landing, or landed trust.
