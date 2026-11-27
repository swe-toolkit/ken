---
id: SPEC-GEOMETRY-ORIENT-SCALE
title: "Spec 67 says orientation survives a shared translation (§21), but says nothing about a shared scaling: a client that rescales a natural-coordinate figure by a positive factor cannot carry an orientation result to the rescaled points. Rule from a checked D0 at zero trust that multiplying every coordinate of the three points by Suc k leaves orient unchanged, and state it with a seed that separates a positive factor from the zero factor"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md). §21 gave translation invariance of orient; this gives scaling invariance. Steward-filed per COORDINATION section 2."
---

# A shared positive scaling preserves orientation

## Objective

`Algorithm.Geometry.NatOrientation` exports a public checked law. For
every `k` and six coordinates, `orient` of the three points with each
coordinate multiplied by `Suc k` equals `orient` of the original points.
It holds at zero trust, and spec 67 states it by its public type.

## Settled inputs (main `4556f3112`; anchors re-checked at release)

- **The operation.** `orient` (`NatOrientation.ken.md:35`) compares two
  sums of three coordinate products with `compare`. `orient_cross`
  (`:49`) states that equation, and `orient_translate` (`:440`) is §21.
- **The argument, for AC-0 to check.**
  - Each product `mul (mul (Suc k) x) (mul (Suc k) y)` rearranges by
    `assoc` and `comm for mul` to `mul (mul x y) s`, where `s` is `mul
    (Suc k) (Suc k)`.
  - Each side of the comparison is then a sum of three such products. So
    `mul_add_distrib_r` (`Arithmetic.ken.md:165`) factors it as `mul
    (sum) s`.
  - What remains is `compare (mul a s) (mul b s) = compare a b` for a
    positive `s`.
- **The missing Order law.** `Data.Numeric.Nat.Order` has
  `add_cancel_r for compare` (`Order.ken.md:554`). It has
  `leq_nat_mul_right_cancel` and `leq_nat_mul_right_mono` (`:990`,
  `:1008`) for `leq_nat` only. No `compare` law cancels a positive right
  factor. AC-0 rules whether that law is public in Order, as §21 did for
  `add_cancel_r`, or private in NatOrientation.
- **The factor must be positive.** At factor `Zero`, every point is the
  origin and `orient` is `Eq`, so a law over every factor is false.
- **Spec and tests.** Spec 67 ends with §21 (`67-geometry.md:1692`), so
  the new section is §22. The seed directory is
  `conformance/stdlib/geometry/`, after `seed-orient-translate.md`. The
  package test is `crates/ken-elaborator/tests/
  cat_geometry_nat_orientation.rs`, or a sibling if AC-0 so rules.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 67 §22, the catalog law with any private helper, the Order law if
AC-0 makes it public, a seed, a package test and the status lines. No §1
to §21 operation or law type moves, and there is no kernel or surface
change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GEOMETRY-ORIENT-TRANSLATE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - where the `compare` scaling law lives;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §22 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed takes the points `(0, 0)`, `(1, 0)` and `(0, 1)`,
  whose `orient` is `Gt`.
  - Its accepted block scales them by `2`, as `Suc 1`. Through the law, it
    concludes that the scaled `orient` is `Gt`.
  - Its refused block, on a fresh accepted prefix and in the same form,
    claims the same of the points with every coordinate multiplied by
    `Zero`. Their `orient` is `Eq`.
  - The two rows differ only in the factor.
- **AC-T.** The package test compares `trusted_base()` before and after
  the touched packages as identity sets, and that comparison fails when a
  package adds an `Axiom`. No Decision opens until it passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the factor's form `Suc k`.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §21 operation or law type.

Checks: 4 fired. I read `orient`, `orient_cross`, `orient_translate`, the
Order `compare` laws and the Arithmetic distributivity laws on
`4556f3112`. The argument uses only those and one new `compare`
cancellation. 3 fired. I grepped Order for a `compare` law over `mul`.
Only the `leq_nat` pair exists. 8 fired. The refused row uses the zero
factor, which collapses every point to the origin, so a law with no
positivity fails it. Its claimed `Gt` is against the actual `Eq`.
