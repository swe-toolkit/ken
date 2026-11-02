# A point box has area one and contains its point

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §14, Spec child
`5605fe7e96e3583827001740b1d9fec223fd18ee`.

This closed seed applies `area_point_box` at the origin and at an
off-origin point with distinct nonzero coordinates, and applies
`contains_point_box` at that off-origin point. The direct same-box
controls and isolated refusals test the closed values and proof forms;
they do not establish either generic theorem.

## Accepted client

`origin=(0,0)`, `off=(3,5)`, and `other=(3,4)`. The origin and off-origin
point boxes each have area one. `other` differs from `off` only in its y
coordinate, which is outside the singleton interval `[5,5]`.

```ken
import Core.Classes.LawfulClasses (IsTrue)

import Algorithm.Geometry.NatBox
  (Point, MkPoint, MkBox, contains, area, contains_point_box, area_point_box)

const n0 : Nat = Zero

const n1 : Nat = Suc n0

const n2 : Nat = Suc n1

const n3 : Nat = Suc n2

const n4 : Nat = Suc n3

const n5 : Nat = Suc n4

const origin : Point = MkPoint n0 n0

const off : Point = MkPoint n3 n5

const other : Point = MkPoint n3 n4

theorem origin_coordinates : Equal Point origin (MkPoint n0 n0) = Refl

theorem off_coordinates : Equal Point off (MkPoint n3 n5) = Refl

theorem other_coordinates : Equal Point other (MkPoint n3 n4) = Refl

theorem origin_area_law : Equal Nat (area (MkBox origin origin)) n1 = area_point_box origin

theorem off_area_law : Equal Nat (area (MkBox off off)) n1 = area_point_box off

theorem off_contains_law : IsTrue (contains (MkBox off off) off) = contains_point_box off

theorem off_area_value : Equal Nat (area (MkBox off off)) (Suc Zero) = Proved

theorem off_contains_value : IsTrue (contains (MkBox off off) off) = Proved
```

### `stdlib/geometry/point-box-area-and-membership`

- spec: §14.
- promise class: durable invariant.
- given: the origin and `off` point boxes.
- expect: `area_point_box` applies at the origin and at `off`, while
  `contains_point_box` applies at `off`; direct `Proved` controls
  establish the area and membership values at `off`.
- measured: when run, three generic-law applications, three fixture
  coordinate identities (`origin_coordinates`, `off_coordinates`,
  `other_coordinates`), and two same-form closed value controls at `off`.
- claimed: every point box has area one and contains its point.
- gap: these two closed points do not establish either generic law.
- why: the law applications use the public theorem names at distinct
  points, including the origin and an off-origin point.

## Refuse a zero area for a point box

Append only this theorem to a fresh copy of the accepted client above.
The claim changes only the expected area value from the accepted control's
`Suc Zero` to `Zero`; expect `KernelRejected TypeMismatch` at this
claim, not at an import or an earlier declaration.

### `stdlib/geometry/reject-point-box-zero-area`

- spec: §14 and §4.1.
- promise class: durable invariant.
- given: the same `off` point box and `Equal Nat` proof form as the
  accepted area control.
- expect: the claim that this point box has area zero is refused.
- measured: when run, the accepted same-box `Suc Zero` control and
  refusal of the same-form `Zero` claim.
- claimed: coincident inclusive bounds have span one on each axis, so
  this point box has area one rather than zero.
- gap: the refusal does not prove `area_point_box` for every point.
- why: only the claimed value differs from the accepted control.

```ken
-- Reject: inclusive point-box spans give area one, not zero.
theorem seed_area_neg : Equal Nat (area (MkBox off off)) Zero = Proved
```

## Refuse containment of a distinct point

Append only this theorem to a fresh copy of the accepted client above.
The box remains `MkBox off off` and the `IsTrue` proof form is unchanged;
only the tested point changes from `off` to `other`. Expect
`KernelRejected TypeMismatch` at this claim.

### `stdlib/geometry/reject-distinct-point-in-point-box`

- spec: §14 and §1.1.
- promise class: durable invariant.
- given: the same point box and Boolean-membership proposition as the
  accepted containment control.
- expect: the claim that `other` belongs to the point box at `off` is
  refused.
- measured: when run, the accepted same-box `off` control and refusal
  of the same-form claim for `other`.
- claimed: a point box contains its own point, not every distinct point.
- gap: this refusal does not prove `contains_point_box` generically.
- why: `other` has y-coordinate 4, outside the inclusive singleton
  interval `[5,5]`; the point box and proof form stay fixed.

```ken
-- Reject: other is outside the singleton y-coordinate of off.
theorem seed_contains_neg : IsTrue (contains (MkBox off off) other) = Proved
```

## Evidence boundary

Architect reports that AC-0 on base `fcc816c6591bc5accc13dfee0ea6a827985d04ae`
checked the generic laws and the closed seed rows in scratch, with
`ken check` rc 0, no new `Axiom`, and unchanged `trusted_base()` GlobalId
identities, 114 → 114. The positive controls were accepted; the false
area and containment claims were refused with `TypeMismatch`. These are
Architect-reported D0 measurements, not tests run by CV or assembled-tip
Foundation evidence.

This child adds only the seed. Foundation's exact-tip package checks,
the official conformance runner, CI, publication, landing and a measured
landed trust result remain unestablished here.
