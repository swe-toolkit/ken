---
id: SPEC-GEOMETRY-BOX-INTERSECT-HULL-ABSORPTION
title: "Spec 67 states that box intersection and hull each commute and associate, but no law relates the two operations. Rule from a checked D0 at zero trust that each absorbs the other as a structural box equality, through public min and max absorption selectors in Nat Order, and state the laws with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry is next in the layer rotation after Optimization. Same shape as SPEC-GEOMETRY-BOX-INTERSECT-HULL-ASSOCIATIVITY. Steward-filed per COORDINATION section 2."
---

# Box intersection and hull absorb each other

## Objective

Spec 67 states, as public checked laws over `Algorithm.Geometry.NatBox`,
that `intersect a (hull a b)` equals `a` and `hull a (intersect a b)`
equals `a`, as structural `Equal Box`, for every two boxes.

## Settled inputs (spec 67 §6 and §7, main `6e778c2d5`)

- **The operations.** `intersect` takes `max` of the lower corners and
  `min` of the upper corners. `hull` takes the opposite selectors
  (`NatBox.ken.md:82`, `:105`).
- **No mixed law.** §6 and §7 each state a law about one operation.
  §7.2 says it does not imply that either operation distributes over the
  other, or that a mixed regrouping is equal (`67-geometry.md:747`
  onward). No section states absorption.
- **Why it holds.** Each lower coordinate of `intersect a (hull a b)` is
  `max x (min x y)`, and each upper one is `min x (max x y)`. Both equal
  `x` for every `Nat`. The four coordinate equalities give `Equal Box`
  through the private `point_equal` and `box_equal` (`:1002`, `:1019`).
  `hull a (intersect a b)` is the dual. Neither law needs a validity
  premise, so both hold for inverted boxes, as §6 and §7 do.
- **The Nat selectors.** `Data.Numeric.Nat.Order` exports `comm` and
  `assoc` for `min` and `max` (`Order.ken.md:106`, `:120`, `:195`, `:209`),
  but no absorption. §6.1 and §7.1 placed those selectors as public proofs
  in their operation families. AC-0 rules the same for absorption.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-intersect-hull-associativity.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §8 in spec 67 stating the ruled laws by their public types, a closed
conformance seed under `conformance/stdlib/geometry/`, and the status lines,
all in one candidate. The catalog half travels in the candidate when AC-0
rules it, as it did for SPEC-TIME and SPEC-SYMBOLIC. Otherwise it is a
separate Foundation WP built from the D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base, with
  rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity set. It
  rules:
  - the names of the two box laws and the two Nat selectors;
  - which helpers stay private.

  It also confirms that no §1 to §7 operation or law type changes.
- **AC-1.** §8 states both laws by their public types. The Spec Leader and
  CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses two distinct boxes that overlap
  but neither contains the other.
  - It applies both laws.
  - It checks both sides of each equation independently, by `Refl` for the
    closed `Equal Box` controls and by `Proved` for the closed Nat-value
    controls (Architect AC-0 `evt_1637h9b5snrqm`).
  - Its refused block pins one false mixed claim on the same boxes, by
    `Refl`: `intersect (hull a b) b` equal to `a`. Its left side is `b`.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a validity premise.
- A change to an existing §1 to §7 operation or law type.

Checks: 4 fired: the selector directions were read from `intersect` and
`hull` on `6e778c2d5`, not from the §1.2 prose, and the Nat Order exports
were read at `Order.ken.md:106` to `:209`, with no absorption among them. 8
fired: the refused row has the accepted law's shape with the shared argument
moved. Its sides are `b` and `a`, which differ for the seed's two boxes, so a
kernel that accepted it would show the law was not checked.
