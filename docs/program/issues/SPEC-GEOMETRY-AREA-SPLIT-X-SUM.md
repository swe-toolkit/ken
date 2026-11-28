---
id: SPEC-GEOMETRY-AREA-SPLIT-X-SUM
title: "Spec 67 bounds box area by inclusion (area_inside) and computes it for point and inverted boxes, but no law says that cutting a box along a vertical line divides its area between the two pieces: a client that tiles a box into column strips cannot sum their areas without unfolding the private axis span. Rule from a checked D0 at zero trust that, when lx <= Suc m and m <= hx, the areas of the boxes [lx, m] and [Suc m, hx] on the same y range add to the area of [lx, hx], and state it with a seed whose accepted row is open"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md). NatBox's area laws cover inclusion, point boxes, inverted axes and positive-area corners; none divides area. Steward-filed per COORDINATION section 2."
---

# Cutting a box into two columns divides its area

## Objective

`Algorithm.Geometry.NatBox` exports a public checked law. For every `lx`,
`ly`, `m`, `hx` and `hy`, from `IsTrue (leq_nat lx (Suc m))` and `IsTrue
(leq_nat m hx)`, it states

`Equal Nat (add (area (MkBox (MkPoint lx ly) (MkPoint m hy))) (area (MkBox
(MkPoint (Suc m) ly) (MkPoint hx hy)))) (area (MkBox (MkPoint lx ly)
(MkPoint hx hy)))`.

It holds at zero trust, and spec 67 states it by its public type.

## Settled inputs (main `833ea912b`; anchors re-checked at release)

- **The operation.** `area` (`NatBox.ken.md:2150`) is `mul (axis_span lx
  hx) (axis_span ly hy)`, and the private `axis_span lo hi` (`:2558`) is
  `sub (Suc hi) lo`, the inclusive span.
- **The argument, for AC-0 to check.**
  - Under the two premises, `sub (Suc m) lx` plus `sub (Suc hx) (Suc m)`
    equals `sub (Suc hx) lx`. Nat.Order's only subtraction-and-addition law,
    `sub_add_same` (`Order.ken.md:502`), is private, so AC-0 rules whether
    this step is a private NatBox helper.
  - `mul_add_distrib_r` (`Arithmetic.ken.md:165`) then distributes the
    shared y span, and `cong` closes the law.
  - The premises allow an empty left column (`m` = `lx` - 1). AC-0 rules
    whether they are the weakest correct pair.
- **No such law exists.** NatBox has 43 public checked theorems. Its area
  laws are `area_inside`, `area_inverted_x`, `area_inverted_y`,
  `area_point_box` and `area_positive_contains_lo_corner`. None relates the
  areas of two boxes that share an edge.
- **Spec and tests.**
  - Spec 67 ends with §23 (`67-geometry.md:1826`), so the new section is
    §24.
  - The seed directory is `conformance/stdlib/geometry/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_geometry_box_area_positive_corner.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 67 §24, the catalog law with any private helper, a seed, the package
test rows and the status lines. No §1 to §23 operation or law type moves,
and there is no kernel or surface change. A horizontal-cut twin is out of
scope unless AC-0 shows it follows without a second export.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GEOMETRY-BOX-AREA-POSITIVE-CONTAINS-CORNER.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order, the premises and the placement;
  - whether the span-splitting step stays private;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §24 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row is open in `hx`, with both premises as hypotheses. AC-0
    shows that the row's closing term needs the law.
  - Its refused twin, on a fresh accepted prefix and in the same form,
    drops the premise `leq_nat lx (Suc m)` and claims the same sum. That
    claim must refuse.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Each touched package test target passes on the exact
  candidate with no name filter. Full-workspace CI is green on the exact
  candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the two stated.
- The proof needs a new public export outside NatBox; report it rather
  than widening Nat.Order.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §23 operation or law type.

Checks: 4 fired. I read `area`, `axis_span`, `sub_add_same` and
`mul_add_distrib_r` on `833ea912b`; the law is a span split followed by
distribution. 3 fired. I listed NatBox's 43 public theorems and its five
area laws, and found none that divides area. 8 fired. A closed seed row
computes without the law (ADD-POINT-EXACT AC-0, `evt_1b1n2fsh3bvws`), so
the witness row is open, and the refused twin drops the premise that
makes the left column well formed.
