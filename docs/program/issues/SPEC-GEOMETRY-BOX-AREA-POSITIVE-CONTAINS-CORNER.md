---
id: SPEC-GEOMETRY-BOX-AREA-POSITIVE-CONTAINS-CORNER
title: "Spec 67 §4 says area is zero when either axis is inverted, and a box containing a point has a derived positive area, but no law gives the converse, so a client holding a box of positive area cannot conclude it holds any point. Rule from a checked D0 at zero trust that a box whose area is at least one contains its low corner, and state it with a seed that separates a valid box from one inverted on a single axis"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md). Area's laws give zero for inverted boxes and a derived lower bound for occupied ones; nothing returns from positive area to membership. Steward-filed per COORDINATION section 2."
---

# A box of positive area contains its low corner

## Objective

`Algorithm.Geometry.NatBox` exports a public checked law. For every `lo`
and `hi`, if `area (MkBox lo hi)` is at least `Suc Zero`, then `contains
(MkBox lo hi) lo` is true. It holds at zero trust, and spec 67 states it
by its public type. With the existing derived use, positive area and
having a member then coincide.

## Settled inputs (main `d7d1e430e`; anchors re-checked at release)

- **The operation.** `area` (`NatBox.ken.md:2149`) is `mul (axis_span lx
  hx) (axis_span ly hy)`, with private `axis_span lo hi = sub (Suc hi) lo`
  (`:2492`). `contains` (`:73`) is `bool_and` of two `axis_contains`, each
  `bool_and (leq_nat lo x) (leq_nat x hi)`.
- **The neighbours.** `area_inside`, `area_inverted_x`,
  `area_inverted_y` and `area_point_box` (§4.2). `contains_lo_corner` and
  `contains_hi_corner` (§11) give the corners from any member. The
  package prose at `:2611` derives a positive lower bound on `area c` from
  `contains c p`, as a checked client rather than an export.
- **The gap.** No law takes positive area back to membership. The
  inversion laws state the zero direction only, and their contrapositive
  needs Nat order case analysis that no public law packages.
- **The vocabulary.** `Data.Numeric.Nat.Order` exports `saturates for
  sub` (`:346`) and `false_reverse_leq for lt_nat` (`:678`).
  `Arithmetic` has `zero_r for mul` (`:80`) and `zero_l for mul` (`:82`).
- **Spec and tests.** Spec 67 ends with §19, so the new section is §20.
  The seed directory is `conformance/stdlib/geometry/`. The test pattern is
  `crates/ken-elaborator/tests/cat_geometry_box_area.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 67 §20, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §19 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GEOMETRY-ORIENT-AXIS-COLLINEAR.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name, the binder order, the premise's form, and the placement
    after `area_point_box`;
  - whether the high corner is a second export or a stated corollary
    through `contains_hi_corner`;
  - whether the per-axis step (a positive span orders its endpoints) is
    a private `NatBox` helper or a generic `Nat.Order` law;
  - every consumer root of an exported `NatBox` law, swept by the sibling
    `area_point_box` across `catalog/`, `conformance/`, `crates/`,
    `spec/` and `library/`, so the candidate's scope is complete.
- **AC-1.** §20 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to `MkBox (MkPoint 1
  2) (MkPoint 3 2)`, of area `3`, and concludes that it contains `MkPoint
  1 2`. Its refused block, on a fresh accepted prefix and in the same
  form, applies it to `MkBox (MkPoint 1 2) (MkPoint 3 1)`. That box is
  inverted on y, so its area is `0`, the premise fails to check, and the
  box really holds no point. The two rows differ only in `hy`.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond positive area.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §19 operation or law type.

Checks: 4 fired. I read `area`, `axis_span` and `contains` on `d7d1e430e`,
and `saturates for sub` and `false_reverse_leq` in `Nat.Order`. The
obligation reduces to: a positive product has positive factors, and
`1 <= sub (Suc h) l` gives `l <= h` by saturation. It is writable in the
delivered vocabulary; the route is AC-0's to rule. 3 fired. I grepped
`catalog/`, `spec/`, `conformance/` and `crates/` for a positive-area or
area-to-membership law. The only hit is the package prose deriving the
opposite direction. 8 fired. In the refused row, a law stated without its
premise would still fail to give membership, because `leq_nat 2 1` is
false. So the refusal sits at the premise, and the rows differ in the
single coordinate that inverts the box.
