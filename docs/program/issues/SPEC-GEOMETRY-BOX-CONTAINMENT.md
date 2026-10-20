---
id: SPEC-GEOMETRY-BOX-CONTAINMENT
title: "Spec 67 relates boxes only through the membership of single points: nothing states that one box lies inside another, so a client cannot conclude from corners alone that every point of an intersection lies in each input, or that each input lies in its hull. Rule a corner-wise containment test and its laws from a checked D0 at zero trust, and state them as a section 3 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry is the roadmap layer after symbolic algebra (docs/program/06-catalog-campaign.md); non-Lane-B layers are deepened in place. Same shape as SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION. Steward-filed per COORDINATION section 2."
---

# Box containment by corners

## Objective

Spec 67 states a public Boolean containment test between two
`Algorithm.Geometry.NatBox` boxes, decided from their corners, with
checked laws that it is sound for point membership and that
`intersect` and `hull` stand in it to their inputs.

## Settled inputs (spec 67 §1 and §2, on `main`)

- **Carriers (§1.1).** `Point = MkPoint Nat Nat`, `Box = MkBox Point
  Point`, inclusive bounds. A box inverted on either axis contains no
  point and is still a valid `Box`; `Eq Box` is structural.
- **Operations (§1.2).** `intersect` takes corner-wise `max` of lower
  and `min` of upper bounds; `hull` takes the reverse.
- **Laws today.** §1.3's six laws and §2's `manhattan_triangle` and
  `manhattan_zero_equal`. Every box law is stated per point `p`; none
  relates two boxes directly.
- **Order vocabulary.** `Data.Numeric.Nat.Order` publishes
  `greatest for min` and `least for max`, which `intersect_complete`
  already uses, and `leq_nat` is the checked natural order.
- **Package.** `catalog/packages/Algorithm/Geometry/NatBox.ken.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 67 section stating the containment test and each ruled law
by its public type, and a closed conformance seed under
`conformance/stdlib/geometry/`.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - the test's name and corner-wise definition by `leq_nat`, and how
    it treats an inverted inner box;
  - soundness: the test holding for `b` in `c` and `p` in `b` give
    `p` in `c`, over every `b`, `c` and `p`;
  - `intersect b c` in `b` and in `c`, and `b` and `c` in `hull b c`;
  - whether a converse of soundness is a public law. On inverted boxes
    it fails as stated, so the D0 states its premise or excludes it.
- **AC-1.** The section states each ruled law by its public type. The
  Spec leader and CV review it, with a Decision.
- **AC-2.** The seed checks soundness on a closed nested pair and the
  intersect and hull laws on a closed overlapping pair, with a positive
  control on a box contained in itself. It has a refused block that
  claims containment of a box in a strictly smaller one.

## Stop conditions

- A law that needs signed coordinates, a canonical empty box,
  point-set equality of boxes or a premise beyond §1's data, unless
  the D0 rules it as the stated premise.
- Any new primitive, kernel or surface feature.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.
