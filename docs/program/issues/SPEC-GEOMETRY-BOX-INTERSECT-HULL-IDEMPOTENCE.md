---
id: SPEC-GEOMETRY-BOX-INTERSECT-HULL-IDEMPOTENCE
title: "Spec 67 makes box intersection and hull commutative (§6), associative (§7), absorptive (§8) and distributive (§9) as structural Equal Box laws, but states no idempotence: §1.3, §6.2 and §8.1 each say so. A client cannot rewrite intersect b b or hull b b to b without recomposing absorption. Rule from a checked D0 at zero trust and state both laws with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry follows Optimization in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Box intersection and hull are idempotent

## Objective

Spec 67 states, as public checked laws over `Algorithm.Geometry.NatBox`,
that for every `b : Box`, `Equal Box (intersect b b) b` and
`Equal Box (hull b b) b`, including for inverted boxes.

## Settled inputs (main `d5d4318a6`)

- **The operations are corner-wise.** `intersect` takes `max` of the
  lower and `min` of the upper coordinates, and `hull` the reverse
  (`NatBox.ken.md:83`, `:106`). Each stored coordinate of `b` meets
  itself, so both laws hold for every box, inverted or not.
- **Not yet stated.** §1.3 (`67-geometry.md:169`) and §6.2 (`:692`)
  say no structural idempotence is asserted, and §8.1 (`:859`) says
  absorption exports no separate idempotence interface.
- **Helpers.** `idempotent for min` exists, private, in
  `Data/Numeric/Nat/Order.ken.md:153`; the grep found no `max`
  counterpart. The private `point_equal` and `box_equal` assemble
  structural `Equal Box`, as §6.2 to §9.2 do.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-intersect-hull-absorption.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §13 in spec 67 stating the ruled laws by their public types, the two
catalog theorems in `NatBox`, a closed conformance seed under
`conformance/stdlib/geometry/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the theorem names and placement, and whether the Nat
  idempotence selectors stay private or become public. It also
  confirms that no §1 to §12 operation or law type changes, and
  amends the three sentences above rather than contradicting them.
- **AC-1.** §13 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies both laws to a valid
  box and to the inverted `(3,3)–(1,1)` box.
  - Its refused block pins `Equal Box (intersect small large) large`,
    where `small` is strictly inside `large`. The intersection is
    `small`, so the claim is false.
  - Its accepted same-form control is `Equal Box (intersect large
    large) large`, so the refusal is the left operand, not the form.
  - The D0 rules the proof form that separates the two rows, since
    `Refl` and `Proved` differ by goal shape (refuted-range AC-0
    evt_6bgw9h9v84f0p).

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §12 operation or law type.

Checks: 4 fired: both laws are written from the read definitions of
`intersect` and `hull`, and the package and spec were grepped for an
idempotence law, with no hit beyond the three disclaimers and the
private Nat `min` selector. 8 fired: the refused row and its control
share the right operand and the claim form, and differ only in a left
operand whose intersection differs.
