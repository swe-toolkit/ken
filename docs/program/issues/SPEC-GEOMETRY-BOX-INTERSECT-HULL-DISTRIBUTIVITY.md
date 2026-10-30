---
id: SPEC-GEOMETRY-BOX-INTERSECT-HULL-DISTRIBUTIVITY
title: "Spec 67 states that box intersection and hull commute, associate and absorb each other, and §7 says explicitly that none of this implies either operation distributes over the other. Rule from a checked D0 at zero trust that each distributes over the other as a structural box equality, through public min and max distribution selectors in Nat Order, and state the laws with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry is next in the layer rotation after Optimization. Same shape as SPEC-GEOMETRY-BOX-INTERSECT-HULL-ABSORPTION. Steward-filed per COORDINATION section 2."
---

# Box intersection and hull distribute over each other

## Objective

Spec 67 states, as public checked laws over
`Algorithm.Geometry.NatBox`, that for every `a`, `b` and `c`:

- `intersect a (hull b c)` equals `hull (intersect a b) (intersect a c)`;
- `hull a (intersect b c)` equals `intersect (hull a b) (hull a c)`.

Both are structural `Equal Box` equalities.

## Settled inputs (spec 67 §1 to §8, main `95814a580`)

- **The operations.** `intersect` takes `max` of the lower corners and
  `min` of the upper corners, and `hull` the reverse
  (`NatBox.ken.md:83`, `:106`). Each coordinate is computed
  independently, so a box law reduces to one Nat law per coordinate.
- **Not yet stated.** §7 says associativity does not imply that either
  operation distributes over the other (`67-geometry.md:778`). §8 adds
  absorption only.
- **The Nat half is missing.** `Data.Numeric.Nat.Order` exports `comm`,
  `assoc` and `absorb` for `min` and `max` (`Order.ken.md:106` to
  `:248`), but no distribution law. The two needed are
  `min m (max n k) = max (min m n) (min m k)` and its dual. Both hold
  for every natural, because the order on `Nat` is total.
- **Why the box law holds.** Expanding §1.2, the left side of the first
  law takes `max a (min b c)` at each lower coordinate and
  `min a (max b c)` at each upper one. The right side takes
  `min (max a b) (max a c)` and `max (min a b) (min a c)`. These are the
  two Nat distribution laws, and the same holds with the roles reversed
  for the second law. `point_equal` and `box_equal` assemble the
  coordinate equalities, as they do for §8.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-intersect-hull-absorption.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §9 in spec 67 stating the ruled laws by their public types, a closed
conformance seed under `conformance/stdlib/geometry/`, and the status
lines, all in one candidate. The catalog half, the Nat selectors and the
box theorems, travels in the candidate when AC-0 rules it, as it did for
§8.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the names and argument order of the Nat selectors and the box
    theorems, and their placement;
  - which helpers stay private.

  It also confirms that no §1 to §8 operation or law type changes.
- **AC-1.** §9 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses `a = (1,1)–(4,4)`,
  `b = (3,0)–(6,2)` and `c = (0,3)–(2,3)`. `hull b c` does not contain
  `a`, so the first law's left side is `(1,1)–(4,3)`, which is neither
  `a` nor either part.
  - It applies both laws and checks each side independently by `Refl`
    against its computed box.
  - Its refused block pins the mixed regrouping
    `intersect a (hull b c) = hull (intersect a b) c` on the same
    triple. The right side is `(0,1)–(4,3)`, so the two boxes differ
    only at the lower x coordinate.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §8 operation or law type.

Checks: 4 fired: the per-coordinate reduction was written against the
`intersect` and `hull` bodies at `NatBox.ken.md:83` and `:106` on
`95814a580`, and the Nat Order exports were read for an existing
distribution law; none exists. 8 fired: the seed's refused row has the
accepted law's shape with one inner `intersect` dropped. For the seed's
triple, its right side differs from the computed left side at the lower
x coordinate (0 against 1), so a kernel that accepted it would show the
law was not checked.
