# A point box identifies every contained point

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §15.

This seed applies the public law to arbitrary `q` in `MkBox off off`,
where `off=(3,5)`. The paired `Refl` cases keep the box's upper corner
and the equality's right endpoint at `off`. They contrast the point box
with `MkBox other off`, where `other=(3,4)` is the lower corner and a
member. These closed examples do not prove the universal law.

## Accepted client and controls

```ken
import Core.Classes.LawfulClasses (IsTrue)

import Algorithm.Geometry.NatBox (Point, MkPoint, MkBox, contains, contains_point_box_equal)

const three : Nat = Suc (Suc (Suc Zero))

const four : Nat = Suc three

const five : Nat = Suc four

const off : Point = MkPoint three five

const other : Point = MkPoint three four

theorem point_box_member_equal
      (q : Point) (member : IsTrue (contains (MkBox off off) q))
    : Equal Point q off =
  contains_point_box_equal off q member

theorem off_in_point_box : IsTrue (contains (MkBox off off) off) = Proved

theorem other_in_two_point_box : IsTrue (contains (MkBox other off) other) = Proved

theorem member_equals_off_by_law : Equal Point off off =
  point_box_member_equal off off_in_point_box

theorem point_box_refl_control
      (member : IsTrue (contains (MkBox off off) off))
    : Equal Point off off =
  Refl

theorem point_box_refl_control_at_off : Equal Point off off =
  point_box_refl_control off_in_point_box
```

### `stdlib/geometry/point-box-singleton-law-client`

- spec: §15.
- promise class: durable invariant.
- given: arbitrary `q` and a proof of `contains (MkBox off off) q`.
- expect: the theorem applies `contains_point_box_equal` and concludes
  `Equal Point q off`.
- measured: the public law is applied at `p=off`, with `q` and its
  membership proof left generic.
- claimed: every member of this point box equals `off`.
- gap: the client fixes `p=off`; it does not prove the law for all points.
- why: the client consumes the public law rather than recomputing the
  equality.

### `stdlib/geometry/point-box-refl-control`

- spec: §1.1 and §15.
- promise class: durable invariant.
- given: `off` in `MkBox off off` and the claim `Equal Point off off`.
- expect: the same-form `Refl` control accepts.
- measured: `off_in_point_box` supplies an actual member proof, and
  `point_box_refl_control_at_off` applies the control to it.
- claimed: `Refl` accepts the equality for the point-box member.
- gap: this closed control does not prove the public law generically.
- why: it establishes the accepted side of the ruled `Refl` pair.

## Refused claim

For the refusal, use a fresh copy of the entire accepted client above and
append only the theorem below. Require refusal at this theorem's `Refl`,
not at an import or an earlier declaration. The accepted client proves
that `other` is in the two-point box; the different y coordinates make
`Equal Point other off` false.

### `stdlib/geometry/nonpoint-box-member-not-equal-to-upper`

- spec: §1.1 and §15.
- promise class: durable invariant.
- given: `other` in `MkBox other off` and the same equality form as the
  point-box control.
- expect: the appended theorem is refused at `Refl`.
- measured: `other_in_two_point_box` witnesses membership; the claim is
  `Equal Point other off`.
- claimed: membership in this non-singleton box does not imply equality
  to its upper corner.
- gap: the closed counterexample does not prove the public law
  generically.
- why: the upper corner, equality's right endpoint, and `Refl` form are
  fixed; the lower corner and member change from `off` to `other`.

```ken
-- Refuse: other is in the two-point box but differs from off in y.
theorem reject_two_point_member_equals_upper
      (member : IsTrue (contains (MkBox other off) other))
    : Equal Point other off =
  Refl
```

## Evidence boundary

Architect's final AC-0 D0 reported the generic law client and the
same-form `Refl` pair checked in scratch at the assigned base. CV did not
run a package test or the official conformance runner. This seed makes no
claim about CI, publication, landing, or a measured landed trust result.
