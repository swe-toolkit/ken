---
id: SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-ASSOCIATIVITY
title: "Spec 68 §6 regroups interval addition and multiplication by endpoint-value inclusion, but no section regroups hull or intersection: Algorithm.Numeric.RationalInterval has commutativity, bounds and absorption for both and no associativity. Rule from a checked D0 at zero trust that each grouping of a hull, and of an intersection, is inside the other, and state the four laws with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Numerical is next in the layer rotation after Graphs. Steward-filed per COORDINATION section 2."
---

# Hull and intersection regroup by inclusion

## Objective

Spec 68 states, as public checked laws over
`Algorithm.Numeric.RationalInterval`, that for every three intervals
`i`, `j` and `k`, including inverted ones:

- `interval_hull i (interval_hull j k)` is inside
  `interval_hull (interval_hull i j) k`, and conversely;
- `interval_intersect i (interval_intersect j k)` is inside
  `interval_intersect (interval_intersect i j) k`, and conversely.

Each is an `IsTrue (interval_inside …)` law, the form §6 uses.

## Settled inputs (spec 68 §1 to §8, main `ad4c67dcc`)

- **The operations.** `interval_hull` takes the min of the lower
  endpoints and the max of the upper ones, and `interval_intersect` the
  reverse (`RationalInterval.ken.md:322`, `:330`).
- **Not yet stated.** §5 states hull and intersection commutativity, §7
  their bounds and §8 their absorption. §6's four associativity laws
  cover only `interval_add` and `interval_mul`. The package has no
  hull or intersection associativity.
- **The interval half exists.** These laws follow from public interval
  laws alone, with no new Rational law:
  - `interval_hull_least` (`:978`) and `interval_intersect_greatest`
    (`:1010`);
  - `interval_inside_hull_left`/`_right` (`:511`, `:526`) and
    `interval_intersect_inside_left`/`_right` (`:541`, `:556`);
  - `interval_inside_trans` (`:387`).
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-associativity-inside.md`, whose `i=[1/2,1]`,
  `j=[1/3,2]` and `k=[3/2,3]` can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §9 in spec 68 stating the ruled laws by their public types, the four
catalog theorems in `RationalInterval`, a closed conformance seed under
`conformance/stdlib/numerical/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the theorem names, argument order and placement;
  - which helpers stay private.

  It also confirms that no §1 to §8 operation or law type changes.
- **AC-1.** §9 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies all four laws to the
  reused `i`, `j` and `k`.
  - Its refused block pins the dropped-operand regrouping
    `interval_inside (interval_hull i (interval_hull j k))
    (interval_hull i j)`.
  - The left side is `[1/3,3]` and the right side `[1/3,2]`, so `k`'s
    upper endpoint falls outside.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §8 operation or law type.

Checks: 4 fired: on `ad4c67dcc`, the lower-endpoint obligation of the
hull law is `interval_hull_least` applied to
`interval_inside_hull_left`-style inclusions chained by
`interval_inside_trans`. The intersection dual uses
`interval_intersect_greatest`. Each cited signature was read at its
line. Spec 68 was grepped for hull and intersection associativity, with
no hit. 8 fired: the refused row drops `k` from the right side. Its
left upper endpoint 3 exceeds the right's 2, so the inclusion is
false while the four accepted rows are true.
