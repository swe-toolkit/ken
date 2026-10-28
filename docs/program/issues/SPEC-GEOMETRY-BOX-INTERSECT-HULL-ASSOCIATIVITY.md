---
id: SPEC-GEOMETRY-BOX-INTERSECT-HULL-ASSOCIATIVITY
title: "Spec 67 states that box intersection and hull commute as boxes, but no law regroups three boxes. Rule from a checked D0 at zero trust that intersect and hull are each associative as structural box equalities, through public min and max associativity selectors in Nat Order, and state them with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry is next in the layer rotation after Optimization. Same shape as SPEC-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY. Steward-filed per COORDINATION section 2."
---

# Box intersection and hull are associative

## Objective

Spec 67 states, as public checked laws over
`Algorithm.Geometry.NatBox`, that `intersect (intersect a b) c` equals
`intersect a (intersect b c)` and `hull (hull a b) c` equals
`hull a (hull b c)`, as structural `Equal Box`, for every three boxes.

## Settled inputs (spec 67 §6, main `9e5d4bc7d`)

- **The operations.** `intersect` takes `max` of each lower coordinate
  and `min` of each upper one; `hull` takes the opposite selectors
  (`NatBox.ken.md:82`, `:105`).
- **No three-box law.** §6.2 states `intersect_comm` and `hull_comm`
  and says it does not state associativity (`67-geometry.md:659`).
- **Why it holds.** Each corner coordinate of either side is a nested
  `min` or `max` of the same three `Nat` values, so associativity of
  `min` and `max` gives four equal coordinates and then `Equal Box`
  through the private `point_equal` and `box_equal` (`:886`, `:903`).
  It holds for inverted boxes too, as §6.2's commutativity does.
- **The Nat selectors.** `Data.Numeric.Nat.Order` exports
  `comm for min` and `comm for max` (`Order.ken.md:106`, `:172`) but
  no associativity. §6.1 placed the comm selectors as public proofs
  in their operation families; AC-0 rules the same for `assoc`.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-intersect-hull-commutativity.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §7 in spec 67 stating each ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/geometry/`, with the status
lines changed in the same candidate. The catalog export is a separate
Foundation WP built from the AC-0 D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the names and public types of the two box laws and the
  two Nat Order selectors, and which helpers stay private.
- **AC-1.** The new section states each ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies both box laws to three
  distinct, pairwise overlapping boxes, with each side also computed
  independently. Its refused block pins one false proposition on the
  same boxes: the mixed regrouping `intersect (hull a b) c` equal to
  `hull a (intersect b c)`.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise.
- A change to an existing §1 to §6 law or operation.

Checks: 4 fired: the selector formulas and the private congruence
helpers are read from `NatBox.ken.md` on `9e5d4bc7d`, and the absence
of `assoc` selectors from `Order.ken.md`. 8 fired: the refused row
mixes the two operations, which differ on overlapping boxes, so a seed
that only checked one operation's grouping would not catch a law that
conflated them.
