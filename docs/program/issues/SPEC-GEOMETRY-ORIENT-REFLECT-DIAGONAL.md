---
id: SPEC-GEOMETRY-ORIENT-REFLECT-DIAGONAL
title: "Spec 67 proves that orientation survives a shared translation (§21) and a shared positive scaling (§22), and flips under swapping two points (§18), but no law says what reflection does: a client that mirrors three points across the diagonal, exchanging each point's coordinates, cannot rewrite the mirrored turn to the original one. Rule from a checked D0 at zero trust that the turn of the three reflected points is ord_result_flip of the original turn, and state it with a seed that separates the flip from preservation"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md). Translation and scaling gave the two orientation-preserving maps; this gives the orientation-reversing one. Steward-filed per COORDINATION section 2."
---

# Reflecting across the diagonal flips the turn

## Objective

`Algorithm.Geometry.NatOrientation` exports a public checked law. For all
six coordinates it states

`Equal OrdResult (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc))
(ord_result_flip (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)))`.

It holds at zero trust, and spec 67 states it by its public type.

## Settled inputs (main `47ccd83b0`; anchors re-checked at release)

- **The operation.** `orient` (`NatOrientation.ken.md:44`) compares the
  cross sums `L = xa yb + xb yc + xc ya` and `R = ya xb + yb xc + yc xa`.
- **The argument, for AC-0 to check.** Exchanging each point's coordinates
  turns `L` into `R` and `R` into `L`, term for term and in order. So the
  reflected turn unfolds to `compare R L`. The public Nat Order selector
  `proof swap for compare` (`Order.ken.md:530`) gives
  `Equal OrdResult (compare b a) (ord_result_flip (compare a b))`, and that
  closes it.
- **No such law exists.** `orient_swap` (`:67`) exchanges two points, not
  coordinates. `orient_translate` (`:449`) and `orient_scale` (`:685`)
  preserve the turn. Nothing in `catalog/` or `spec/` reflects points.
- **Spec and tests.**
  - Spec 67 ends with §22 (`67-geometry.md:1754`), so the new section is
    §23.
  - The seed directory is `conformance/stdlib/geometry/`, after
    `seed-orient-scale.md`.
  - The test pattern is the `orient_scale` rows in
    `crates/ken-elaborator/tests/cat_geometry_nat_orientation.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 67 §23, the catalog law, a seed, the package test rows and the status
lines. No §1 to §22 operation or law type moves, there is no new operation,
and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GEOMETRY-ORIENT-SCALE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether a reflection operation on `Point` is wanted, or the law stays
    stated over `MkPoint` as above;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §23 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed takes the §18 control `(0,0), (1,0), (0,1)`, whose
  turn is `Gt`.
  - Its accepted block applies the law to the reflected triple
    `(0,0), (0,1), (1,0)` and checks that its turn is `Lt`.
  - Its refused block, on a fresh accepted prefix and in the same form,
    claims that the reflected turn is the original `Gt`. That claim must
    refuse.
  - The two rows differ only in the claimed result. AC-0 picks the closing
    terms and may replace this seed with a stronger discriminator.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §22 operation or law type.

Checks: 4 fired. I read `orient`, `orient_swap` and `proof swap for compare`
on `47ccd83b0`. The reflected cross sums are the original ones exchanged,
term for term. 3 fired. I grepped the Geometry package and spec 67 for
reflection or mirror laws and found none. 8 fired. The control's turn is
`Gt` and its reflection's is `Lt`, so a law that claims the turn is
preserved fails the refused row, and one that drops the flip cannot
produce the accepted one.
