---
id: SPEC-GEOMETRY-ORIENT-AXIS-COLLINEAR
title: "Spec 67 §18 says orient returns Eq for collinear points, but its only Eq law is the repeated-point case, so a client holding three distinct points on one vertical or horizontal line cannot conclude they are collinear without unfolding orient's cross sums. Rule from a checked D0 at zero trust that orient returns Eq for any three points sharing an x coordinate, and for any three sharing a y coordinate, and state both with a seed that separates on-line from off-line"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md). orient's laws fix swap, rotation, repetition and the definition, with no collinearity law beyond repetition. Steward-filed per COORDINATION section 2."
---

# Three points on an axis-parallel line are collinear

## Objective

`Algorithm.Geometry.NatOrientation` exports two public checked laws. For
every `x`, `ya`, `yb` and `yc`, `orient (MkPoint x ya) (MkPoint x yb)
(MkPoint x yc)` equals `Eq` as `Equal OrdResult`. For every `xa`, `xb`,
`xc` and `y`, `orient (MkPoint xa y) (MkPoint xb y) (MkPoint xc y)`
equals `Eq`. Both hold at zero trust with no premise, including for three
distinct points. Spec 67 states them by their public types.

## Settled inputs (main `d42f7ad28`; anchors re-checked at release)

- **The operation.** `orient` (`NatOrientation.ken.md:18`) is
  `compare L R` with `L = add (add (mul xa yb) (mul xb yc)) (mul xc ya)`
  and `R = add (add (mul ya xb) (mul yb xc)) (mul yc xa)`.
- **The neighbours.** `orient_cross` (`:32`), `orient_swap` (`:41`),
  `orient_rotation` (`:166`) and `orient_repeated_point` (`:224`). The
  swap and rotation proofs already commute products and permute summands
  under `cong` and `trans`. `self_eq for compare` is public
  (spec 67 §18).
- **The gap.** Spec 67 §18.1 says `Eq` denotes collinearity, but
  `orient_repeated_point` is the only `Eq` law, and §18.2's derived
  consequences all involve a repeated point. No law covers three distinct
  collinear points.
- **Spec and tests.** Spec 67 ends with §18
  (`spec/50-stdlib/67-geometry.md:1443`), so the new section is §19. The
  seed directory is `conformance/stdlib/geometry/`. The test pattern is
  `crates/ken-elaborator/tests/cat_geometry_nat_orientation.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-0 trust discharge (2026-11-18)

Spec Leader `evt_61c89hg5wpsqr` on Architect D0 `evt_5nzr87ncz4y9y`. The
D0 checks at rc 0 with no `Axiom` or premise but did not measure
`trusted_base()`. §19 text proceeds on that D0, because it states the
laws' public types, which the trust measurement cannot change. The
identity-set equality moves to the package test: AC-T asserts
`trusted_base()` unchanged as an identity set on the assembled
candidate, and no Decision opens until it passes. A changed set is a
stop to the Architect.

## Scope

Spec 67 §19, the two catalog laws with any private helper, a seed, a
package test and the status lines. No §1 to §18 operation or law type
moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-GEOMETRY-NAT-ORIENTATION.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the names, the binder order, and whether
  the two laws share a private sum-permutation helper with the existing
  swap and rotation proofs.
- **AC-1.** §19 states both laws by their public types. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies each law at three distinct
  closed points: `(2,1), (2,3), (2,7)` for the shared x, and
  `(1,4), (3,4), (8,4)` for the shared y, each giving `Eq`.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - `orient (MkPoint 2 1) (MkPoint 2 3) (MkPoint 3 7)` claimed `Equal
    OrdResult` to `Eq`. Moving one point off the line makes `L` and `R`
    differ, so the claim fails exactly when the shared coordinate is
    dropped.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §18 operation or law type.

Checks: 3 fired. I grepped `NatOrientation.ken.md` and spec 67 for `Eq`
laws over `orient`; `orient_repeated_point` is the only one. 4 fired. I
wrote the obligation in the delivered vocabulary. With a shared `x`,
`L = x·yb + x·yc + x·ya` and `R = ya·x + yb·x + yc·x`, which agree by
commuting each product and permuting the three summands, the steps
`orient_swap` already uses; `self_eq for compare` then gives `Eq`. The
shared-`y` case is symmetric. 8 fired. The refused row differs from the
accepted shared-x row only in the third point's x, so it fails exactly
when the law is read as holding without the shared coordinate.
