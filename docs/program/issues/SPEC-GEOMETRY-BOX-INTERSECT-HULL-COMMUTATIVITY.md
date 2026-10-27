---
id: SPEC-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY
title: "Spec 67 defines intersect and hull on natural boxes corner by corner through max and min, but states no law that either is commutative. Rule from a checked D0 at zero trust that both are commutative as structural Equal Box, and state it with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry is next in the layer rotation after Optimization. Same shape as SPEC-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY. Steward-filed per COORDINATION section 2."
---

# Box intersection and hull are commutative

## Objective

Spec 67 states, as public checked laws over
`Algorithm.Geometry.NatBox`, that `intersect b c` equals
`intersect c b` and `hull b c` equals `hull c b`, as structural
`Equal Box`.

## Settled inputs (spec 67 §1 to §5, main `0bb872591`)

- **The operations.** §1.2 defines `intersect` with lower corner
  `(max blx clx, max bly cly)` and upper corner `(min bhx chx, min bhy
  chy)`, and `hull` with `min` and `max` exchanged, at every input
  including inverted boxes (`NatBox.ken.md:80-124`).
- **No commutativity law.** §1.3, §3.2 and §5.1 state soundness,
  completeness, containment and the inclusion order. No section states
  that either operation is commutative.
- **Why structural equality can hold.** Unlike the rational interval
  swaps of spec 68 §5, which are inclusions because rational endpoints
  are equal only up to equivalence, every corner here is a `Nat`. So
  commutativity of `max` and `min` on `Nat` gives `Equal Box`, not only
  mutual inclusion. `Data.Numeric.Nat` exports no `max` or `min`
  commutativity law today; whether one is added there or proved
  privately is the Architect's ruling.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to the existing
  NatBox seeds.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A spec 67 section stating each ruled law by its public type, and a
closed conformance seed under `conformance/stdlib/geometry/`, with the
status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  names and public types, whether the needed `Nat` `max` and `min`
  laws become public in `Data.Numeric.Nat.Order` or stay private
  helpers in `NatBox`, and which helpers stay private.
- **AC-1.** The new section states each ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies both laws to a closed
  pair of overlapping, unequal boxes. Its refused block is pinned to
  one false proposition on the same pair, such as `hull b c` equal to
  `intersect c b`.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise.
- A change to an existing §1 to §5 law or operation.
