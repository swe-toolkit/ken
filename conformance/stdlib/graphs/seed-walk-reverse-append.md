# Walk reversal and append — closed seed

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §9.
Frame: `docs/program/issues/SPEC-GRAPHS-WALK-REVERSE-APPEND.md`.
Status: authored; not executed.

The three-vertex graph has the edges V0 → V1, V1 → V2, and the
shortcut V0 → V2. In its reversed graph, the composed chain walk
reverses to `via_v1`, while the shortcut gives a distinct direct walk
with the same endpoints. The accepted client applies the public law to
the two-part chain and checks the matching closed Walk equalities by
`Refl`. The isolated refusal changes only the right-hand walk from
`via_v1` to `direct`.

The closed observations do not replace the generic checked law. No
Walk-typed equality below uses `Proved`: §9 notes it is refused even
for a true closed Walk equality. The checked edge proofs inside
`WalkStep` use `Proved` for their `Equal Bool` goals.

## Accepted shared client

```ken
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, WalkHere, WalkStep, walk_append, graph_reverse,
   walk_reverse, walk_reverse_append)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

const shortcut : Graph Vertex3 =
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
          V2 ↦ False
        })

const walk_01 : Walk Vertex3 shortcut V0 V1 =
  WalkStep Vertex3 shortcut V0 V0 V1 Proved (WalkHere Vertex3 shortcut V0)

const walk_12 : Walk Vertex3 shortcut V1 V2 =
  WalkStep Vertex3 shortcut V1 V1 V2 Proved (WalkHere Vertex3 shortcut V1)

const via_v1 : Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0 =
  WalkStep Vertex3 (graph_reverse Vertex3 shortcut) V2 V1 V0 Proved
    (WalkStep Vertex3 (graph_reverse Vertex3 shortcut) V2 V2 V1 Proved
      (WalkHere Vertex3 (graph_reverse Vertex3 shortcut) V2))

const direct : Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0 =
  WalkStep Vertex3 (graph_reverse Vertex3 shortcut) V2 V2 V0 Proved
    (WalkHere Vertex3 (graph_reverse Vertex3 shortcut) V2)

theorem law_on_two_part_walk :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_reverse Vertex3 shortcut V0 V2
      (walk_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12))
    (walk_append Vertex3 (graph_reverse Vertex3 shortcut) V2 V1 V0
      (walk_reverse Vertex3 shortcut V1 V2 walk_12)
      (walk_reverse Vertex3 shortcut V0 V1 walk_01)) =
  walk_reverse_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12

theorem reversed_is_via_v1 :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_reverse Vertex3 shortcut V0 V2
      (walk_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12))
    via_v1 =
  Refl

theorem composed_is_via_v1 :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_append Vertex3 (graph_reverse Vertex3 shortcut) V2 V1 V0
      (walk_reverse Vertex3 shortcut V1 V2 walk_12)
      (walk_reverse Vertex3 shortcut V0 V1 walk_01))
    via_v1 =
  Refl
```

## stdlib/graphs/walk-reverse-append-on-two-part-walk

- spec: `spec/50-stdlib/62-graphs.md` §9.
- promise class: durable invariant.
- given: the two checked chain walks `walk_01` and `walk_12` on
  `shortcut`.
- expect: `law_on_two_part_walk` applies the public law; both sides of
  the equation reduce to the checked route `via_v1`.
- measured: the public theorem accepts the closed two-part walk, and
  the separate `Refl` controls check the reversed result and composed
  reversed parts against `via_v1`.
- claimed: reversing an appended walk agrees with appending its reversed
  parts in the opposite order.
- the gap: this closed witness does not establish the theorem for every
  graph and pair of walks; that is the generic checked law.
- why: the explicit reversed route makes an omitted or reordered edge
  visible in the closed equality.

## stdlib/graphs/reversed-walk-distinct-shortcut-route-refused

- spec: `spec/50-stdlib/62-graphs.md` §9.
- promise class: durable invariant.
- given: the same left side and `Refl` form as `reversed_is_via_v1`;
  only the right side changes from `via_v1` to the checked `direct` walk.
- expect: refuse `seed_neg` because the two walks have the same endpoints
  but are not definitionally equal. Do not pin a diagnostic code.
- measured: the accepted `Refl` control and the refused `Refl` claim
  differ only in the proposed right-hand walk.
- claimed: endpoint agreement does not make distinct `Walk` data equal.
- the gap: this one refusal rejects only the stated false equality; it
  does not characterize all walks with common endpoints.
- why: `direct` is itself a checked walk using the shortcut edge, so the
  refusal is about the false equality, not an invalid fixture.

First accept the entire shared client above. On a fresh copy, append
only this block. The refusal must occur at this `Refl`, not at an import,
fixture, or accepted theorem.

```ken
theorem seed_neg :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_reverse Vertex3 shortcut V0 V2
      (walk_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12))
    direct =
  Refl
```

## stdlib/graphs/reverse-append-law-does-not-prove-direct-route

- spec: `spec/50-stdlib/62-graphs.md` §9.
- promise class: durable invariant.
- given: the accepted `law_on_two_part_walk` application and the same
  false direct-route equality as `seed_neg`.
- expect: refuse the direct-route claim when proved by
  `walk_reverse_append`; the theorem yields the composed reversed route.
- measured: the accepted application uses the same theorem and arguments
  with the specified composed-route conclusion.
- claimed: the public law does not identify the distinct direct walk
  with the reversal of the composed chain.
- the gap: this closed false claim does not characterize all possible
  `Walk` equalities.
- why: only the goal's right-hand walk differs from the accepted law
  application; the theorem term and arguments stay fixed.

On a fresh copy of the accepted shared client, append only this block.
The theorem application must be refused at its result type, not because
of an import or fixture failure.

```ken
theorem seed_neg_by_law :
  Equal
    (Walk Vertex3 (graph_reverse Vertex3 shortcut) V2 V0)
    (walk_reverse Vertex3 shortcut V0 V2
      (walk_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12))
    direct =
  walk_reverse_append Vertex3 shortcut V0 V1 V2 walk_01 walk_12
```

## Evidence boundary

Architect's AC-0 used the chain plus shortcut and measured the accepted
law application, the `via_v1` `Refl` control, and refusal of the direct
same-endpoint walk. Foundation's package-owned checks are separate from
this seed. This file is authored conformance work and has not been run as
Ken or through an official conformance runner. No CI, publication,
landing, or landed-trust result is claimed.
