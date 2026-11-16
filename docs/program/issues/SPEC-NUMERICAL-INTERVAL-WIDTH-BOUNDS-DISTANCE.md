---
id: SPEC-NUMERICAL-INTERVAL-WIDTH-BOUNDS-DISTANCE
title: "Spec 68 §15-§17 say how interval width propagates through inclusion, addition and multiplication, but no law turns a width into an error bound on the values an interval encloses, so a client cannot conclude that two enclosed values differ by at most the width. Rule from a checked D0 at zero trust that, for x and y both contained in i, rational_sub y x is at most interval_width i, and state it with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md, whose Numerical entry is error-bound refinements). The width laws propagate width but none bounds a distance by it. Steward-filed per COORDINATION section 2."
---

# Width bounds the distance between enclosed values

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked law that, for
every `i`, `x` and `y`, from `IsTrue (interval_contains i x)` and
`IsTrue (interval_contains i y)`, concludes
`IsTrue (rational_leq (rational_sub y x) (interval_width i))`. It holds at
zero trust, and spec 68 states it by its public type.

## Settled inputs (main `44dc2b141`)

- **The operations.** `interval_contains` (`RationalInterval.ken.md:389`) is
  `bool_and (rational_leq lo x) (rational_leq x hi)`, and `interval_width`
  (`:412`) is `rational_sub hi lo`. `rational_sub` (`Rational.ken.md:160`)
  truncates at zero, so the law holds with no order premise between `x` and
  `y`.
- **The order tool.** Public `rational_sub_mono` (`Rational.ken.md:679`) takes
  `x ≤ y` and `w ≤ z` to `rational_sub x z ≤ rational_sub y w`. With
  `y ≤ hi` and `lo ≤ x` it gives the conclusion in one step.
- **Splitting containment.** The `bool_and` `left` and `right` proofs split a
  containment premise, as `interval_add_encloses` (`:88`) does.
- **The gap.** The public width laws are `interval_width_point` (`:269`),
  `interval_width_add` (`:274`), `interval_width_mul` (`:285`) and
  `interval_width_inside` (`:674`). Each relates one width to other widths.
  None bounds `rational_sub` of two contained values.
- **Spec and tests.** Spec 68 ends at §17 (`68-numerical.md:1645`), so the
  new section is §18. The test pattern is
  `crates/ken-elaborator/tests/cat_numeric_interval_width_inside.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §18, the catalog law with any private helper, a seed, a package test
and the status lines. No §1 to §17 operation or law type moves, and no kernel
or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-INSIDE-MONO.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules the name, the binder order and whether the interval
  is taken as `i` or as `MkInterval lo hi`, and the proof route.
- **AC-1.** §18 states the ruled law by its public type. The Spec Leader and
  CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law at closed arguments:
  - an interior pair with a non-integer endpoint, such as `x = 1` and
    `y = 5/2` in `[1/2, 3]`;
  - the endpoint pair `x = lo` and `y = hi`, where the bound is tight.

  Its refused block pins, in the same form as an accepted row and on a fresh
  accepted prefix:
  - the bound claimed for `y = 5` and `x = 1` in `[1, 2]`, with the
    containment of `y` dropped.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the two containments.
- A change to an existing §1 to §17 operation or law type.

Checks: 4 fired. Splitting both containments gives `lo ≤ x` and `y ≤ hi`,
and `rational_sub_mono y hi x lo` closes the conclusion from them. 3 fired: I
grepped `catalog/packages`, `spec/` and `conformance/` for a law with
`rational_sub` of two contained values and found none. 8 fired: in `[1, 2]`,
`rational_sub 5 1` is 4 and the width is 1, so the refused row is false on
the closed fixture, and it differs from an accepted row only in the dropped
containment.
