# Walk append left unit and associativity — closed seed

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §4.
Frame: `docs/program/issues/SPEC-GRAPHS-WALK-APPEND-UNIT-ASSOC.md`.

A four-vertex diamond has two genuinely different two-edge routes
from `D0` to `D3`. All four edges below are witnessed by checked
`WalkStep` constructors. The accepted client applies the two generic
public laws to closed walks, puts `WalkHere` in each associativity
position, and checks one closed associativity instance directly by
`Refl`. It also includes a same-input accepted result for the joined
`D0 → D1 → D3` route. This seed promises a **durable invariant**;
closed cases are not a replacement for generic checked proof bodies.

## Accepted shared client

```ken
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, WalkHere, WalkStep, walk_append,
   walk_append_here_left, walk_append_assoc)

data Vertex4 : Type where {
  D0 : Vertex4;
  D1 : Vertex4;
  D2 : Vertex4;
  D3 : Vertex4
}

const diamond : Graph Vertex4 = MkGraph Vertex4
  (λu. λv. match u {
    D0 ↦ match v { D0 ↦ False; D1 ↦ True; D2 ↦ True; D3 ↦ False };
    D1 ↦ match v { D0 ↦ False; D1 ↦ False; D2 ↦ False; D3 ↦ True };
    D2 ↦ match v { D0 ↦ False; D1 ↦ False; D2 ↦ False; D3 ↦ True };
    D3 ↦ match v { D0 ↦ False; D1 ↦ False; D2 ↦ False; D3 ↦ False }
  })

const walk_01 : Walk Vertex4 diamond D0 D1 =
  WalkStep Vertex4 diamond D0 D0 D1 Proved
    (WalkHere Vertex4 diamond D0)

const walk_13 : Walk Vertex4 diamond D1 D3 =
  WalkStep Vertex4 diamond D1 D1 D3 Proved
    (WalkHere Vertex4 diamond D1)

const walk_02 : Walk Vertex4 diamond D0 D2 =
  WalkStep Vertex4 diamond D0 D0 D2 Proved
    (WalkHere Vertex4 diamond D0)

const walk_23 : Walk Vertex4 diamond D2 D3 =
  WalkStep Vertex4 diamond D2 D2 D3 Proved
    (WalkHere Vertex4 diamond D2)

theorem left_unit_on_two_edge_walk :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D0 D3
      (WalkHere Vertex4 diamond D0)
      (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13))
    (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13) =
  walk_append_here_left Vertex4 diamond D0 D3
    (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13)

theorem assoc_with_first_here :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D1 D3
      (walk_append Vertex4 diamond D0 D0 D1
        (WalkHere Vertex4 diamond D0) walk_01) walk_13)
    (walk_append Vertex4 diamond D0 D0 D3
      (WalkHere Vertex4 diamond D0)
      (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13)) =
  walk_append_assoc Vertex4 diamond D0 D0 D1 D3
    (WalkHere Vertex4 diamond D0) walk_01 walk_13

theorem assoc_with_second_here :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D1 D3
      (walk_append Vertex4 diamond D0 D1 D1 walk_01
        (WalkHere Vertex4 diamond D1)) walk_13)
    (walk_append Vertex4 diamond D0 D1 D3 walk_01
      (walk_append Vertex4 diamond D1 D1 D3
        (WalkHere Vertex4 diamond D1) walk_13)) =
  walk_append_assoc Vertex4 diamond D0 D1 D1 D3
    walk_01 (WalkHere Vertex4 diamond D1) walk_13

theorem assoc_with_third_here :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D3 D3
      (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13)
      (WalkHere Vertex4 diamond D3))
    (walk_append Vertex4 diamond D0 D1 D3 walk_01
      (walk_append Vertex4 diamond D1 D3 D3 walk_13
        (WalkHere Vertex4 diamond D3))) =
  walk_append_assoc Vertex4 diamond D0 D1 D3 D3
    walk_01 walk_13 (WalkHere Vertex4 diamond D3)

theorem first_here_assoc_reduces :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D1 D3
      (walk_append Vertex4 diamond D0 D0 D1
        (WalkHere Vertex4 diamond D0) walk_01) walk_13)
    (walk_append Vertex4 diamond D0 D0 D3
      (WalkHere Vertex4 diamond D0)
      (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13)) =
  Refl

theorem joined_via_d1_has_same_route :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13)
    (WalkStep Vertex4 diamond D0 D1 D3 Proved walk_01) =
  Refl
```

### stdlib/graphs/walk-append-here-left-closed

- spec: §4, `walk_append_here_left`.
- promise class: durable invariant.
- given: `WalkHere` at `D0`, then the genuine two-edge walk via `D1`.
- expect: applying the public law accepts the left-unit equation.
- measured: the law checks on this closed nonempty walk.
- claimed: left unit holds for any checked walk.
- the gap: only the generic checked theorem proves the open-walk law;
  this closed witness cannot establish induction on an arbitrary walk.

### stdlib/graphs/walk-append-assoc-each-here-position

- spec: §4, `walk_append_assoc`.
- promise class: durable invariant.
- given: the same graph, two nonempty legs, and `WalkHere` separately
  as first, second, then third operand; one first-position case also
  has an independent `Refl` control.
- expect: each of the three instances accepts the exact public law.
- measured: all three instances type-check, with the specified edge
  order, and the control accepts without invoking `walk_append_assoc`.
- claimed: generic append association preserves every edge of each
  of three arbitrary walks.
- the gap: closed instances, including the `Refl` control, do not
  prove the generic open law; that requires the checked theorem.

### stdlib/graphs/walk-append-same-route-control

- spec: §4 and §3's `walk_append` definition.
- promise class: durable invariant.
- given: `walk_01` and `walk_13`, with the exact input used by the
  refused block below.
- expect: the joined walk checks equal to its explicit `WalkStep`
  through `D1`; this accepting `Refl` precedes the refusal.
- measured: the left side reduces to a checked two-edge path through
  `D1`, not merely an endpoint witness.
- claimed: the refusal below distinguishes path identity by its
  intermediate vertex, rather than failing due to its input or proof
  syntax.
- the gap: the positive control only covers this chosen input and
  `Refl` form. It does not by itself establish a general theorem that
  unequal paths cannot be equated.

## Isolated refused claim

First accept the **entire** shared client above. On a fresh copy,
append only the following block. Its left side is the same as the
accepted `joined_via_d1_has_same_route` control; only the right side
changes to the genuinely different route via `D2`. Both sides have
the same `Walk Vertex4 diamond D0 D3` type, but different checked
intermediate vertices. The refusal must occur at this appended
claim, not at an import, fixture, or accepted theorem. Use `Refl`:
Architect's D0 found that `Proved` is refused even for the true closed
WalkStep equality, and that `λsame. absurd same` is refused for both
the true and the false one, on the measured base; neither form
discriminates.

### stdlib/graphs/walk-append-other-route-refused

- spec: §4 and §3's `walk_append` definition.
- promise class: durable invariant.
- given: the same joined `D0 → D1 → D3` walk as the accepted control;
  the proposed right side is `D0 → D2 → D3`.
- expect: reject the asserted equality at `other_route_is_same_walk`.
- measured: the false route is not accepted by the same `Refl` form.
- claimed: associativity and left unit do not identify walks that
  follow different checked edges despite equal endpoints.
- the gap: this one refusal does not prove the generic laws or rule
  out every possible bad route; it rejects the stated false client.

```ken
theorem other_route_is_same_walk :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13)
    (walk_append Vertex4 diamond D0 D2 D3 walk_02 walk_23) =
  Refl
```

## Evidence boundary

The Architect's AC-0 D0 on `d4f2b202c` checked two generic bodies,
a separate client, the closed diamond positives and the false
other-route control, with no new `Axiom` or scratch trust delta
(114 → 114). On a separate isolated worktree at the same base, the
ruled `dependency-laws.patch` (sha256
`85fa698c33016f8450f10112164d511e938267374ceafea0c7efe86c76f71720`)
was applied to the catalog package **only in scratch**. This seed's
fenced clients were extracted to temporary catalog-root files and
checked with the preexisting `/workspaces/ken/tmp/arch-argmax-target/debug/ken`
binary, built before `d4f2b202c`: the full positive client returned
rc 0; a fresh full-positive-prefix-plus-refusal returned rc 1/E0038
`TypeMismatch` at the appended `Refl` (byte offset 3653 inside the
appended theorem beginning at 3459). Logs:
`/tmp/spec-author-walk-unit-Positive.log` and
`/tmp/spec-author-walk-unit-Refusal.log`. This is a source-shaped
**D0-overlay check using an older CLI binary**, not an exact-base
binary check or execution against shipped §4 exports. Neither it nor
Architect's separate D0 establishes an official conformance run,
CI, publication, landing or landed trust.
