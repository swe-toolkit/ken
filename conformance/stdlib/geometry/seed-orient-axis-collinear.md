# Orientation from a shared coordinate

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §19, approved child
`7cc3e7e34c0d0b3e0cde87477f590738771e9ee8`.

These cases exercise both axis-parallel collinearity laws for three
pairwise-distinct points. The off-line control keeps the first two
points fixed and changes only the third point's x coordinate.

## Accepted client

```ken
import Algorithm.Geometry.NatOrientation
  (orient, orient_shared_x, orient_shared_y)
import Algorithm.Geometry.NatBox (Point, MkPoint)
import Core.Logic.OrdResult (OrdResult, Eq, Lt)

const n1 : Nat = Suc Zero
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4
const n6 : Nat = Suc n5
const n7 : Nat = Suc n6
const n8 : Nat = Suc n7

const p21 : Point = MkPoint n2 n1
const p23 : Point = MkPoint n2 n3
const p27 : Point = MkPoint n2 n7
const p14 : Point = MkPoint n1 n4
const p34 : Point = MkPoint n3 n4
const p84 : Point = MkPoint n8 n4
const p37 : Point = MkPoint n3 n7

theorem shared_x_is_eq
  : Equal OrdResult (orient p21 p23 p27) Eq = Proved

theorem shared_x_law_application
  : Equal OrdResult (orient p21 p23 p27) Eq =
    orient_shared_x n2 n1 n3 n7

theorem shared_y_is_eq
  : Equal OrdResult (orient p14 p34 p84) Eq = Proved

theorem shared_y_law_application
  : Equal OrdResult (orient p14 p34 p84) Eq =
    orient_shared_y n4 n1 n3 n8

theorem off_line_control_is_lt
  : Equal OrdResult (orient p21 p23 p37) Lt = Proved
```

### `stdlib/geometry/orient-axis-collinear-closed-laws`

- spec: §18.1 and §19.
- promise class: durable invariant.
- given: three distinct points sharing x, three distinct points sharing y,
  and a triple whose third point is off the shared-x line.
- expect: both shared-coordinate orientations are `Eq`; both public law
  applications typecheck; the off-line orientation is `Lt`.
- why: the axis-aligned triples satisfy the respective §19 laws. For the
  off-line triple, §18.1 gives cross sums 23 and 25, so `orient` is `Lt`.
  This control also shows the fixture evaluates before either refusal.

## Isolated refusals

For each refusal, start from a fresh copy of the complete accepted client
above and append only its theorem. Expect `TypeMismatch` at the added
theorem. Do not append both refusals to one source.

### `stdlib/geometry/refuse-off-line-eq-by-proved`

- spec: §18.1 and §19.
- promise class: durable invariant.
- given: the accepted client and its off-line triple.
- expect: reject the added claim that the off-line orientation is `Eq`.
- why: correct evaluation gives `Lt`; a bug that treats the first two
  shared x coordinates as sufficient and ignores the third point's x
  would accept this false `Eq` claim.

```ken
theorem refuse_off_line_eq_as_proved
  : Equal OrdResult (orient p21 p23 p37) Eq = Proved
```

### `stdlib/geometry/refuse-off-line-eq-via-shared-x`

- spec: §18.1 and §19.
- promise class: durable invariant.
- given: a fresh accepted-client prefix and the same off-line triple.
- expect: reject the added claim when its proof is supplied by
  `orient_shared_x`.
- why: `orient_shared_x n2 n1 n3 n7` proves `Eq` for `p21`, `p23`,
  `p27`, not `p37`. Correct `orient` gives `Lt` for `p37`; if an
  implementation instead returns `Eq` after the third x changes, this
  law application could inhabit the false goal.

```ken
theorem refuse_off_line_eq_via_shared_x
  : Equal OrdResult (orient p21 p23 p37) Eq =
    orient_shared_x n2 n1 n3 n7
```

## Evidence boundary

This seed was authored on base `d42f7ad28674d27a75978d7048121de52bfd731d`
against the approved Spec child named above. That child records that D0 did
not measure exact `trusted_base()` identity-set equality. This seed makes no
such measurement or claim; the separate Foundation AC-T package test owns
it. No Ken check, package test, official conformance execution, CI,
publication, or landing is claimed.
