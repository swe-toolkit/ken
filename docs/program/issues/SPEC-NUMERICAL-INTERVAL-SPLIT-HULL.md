---
id: SPEC-NUMERICAL-INTERVAL-SPLIT-HULL
title: "Spec 68 splits an enclosure so that every member lands in a half (§21) and the halves' widths sum to the whole (§22), but §21 says it does not assert that the halves contain only original members: a client bisecting an enclosure cannot conclude that a half's members are the whole's. Rule from a checked D0 at zero trust that, for lo ≤ m ≤ hi, the hull of the two halves is inside the whole and the whole is inside that hull, and state it with a seed that separates an interior split point from an exterior one"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md). §21 gave covering and §22 width; §21's own text names the missing converse. Steward-filed per COORDINATION section 2."
---

# A split reassembles the enclosure

## Objective

`Algorithm.Numeric.RationalInterval` exports two public checked laws over
`lo`, `hi` and `m`:

- **Sound:** from `IsTrue (rational_leq lo m)` and `IsTrue (rational_leq m
  hi)`, the law concludes `IsTrue (interval_inside (interval_hull
  (MkInterval lo m) (MkInterval m hi)) (MkInterval lo hi))`.
- **Complete:** with no premise, the law concludes `IsTrue (interval_inside
  (MkInterval lo hi) (interval_hull (MkInterval lo m) (MkInterval m hi)))`.

Both hold at zero trust, and spec 68 states them by their public types.
With `interval_inside_sound`, a member of either half is then a member of
the whole.

## Settled inputs (main `d7d99f1ba`; anchors re-checked at release)

- **The operations.**
  - `interval_hull` (`RationalInterval.ken.md:707`) is the `rational_min`
    of the lower ends and the `rational_max` of the upper ends.
  - `interval_inside` (`:725`) is `bool_and` of the two endpoint
    comparisons, and `interval_inside_sound` (`:733`) carries membership
    along it.
- **The argument, for AC-0 to check.**
  - Sound: `lo ≤ rational_min lo m` by `greatest for rational_min` from
    `lo ≤ lo` and `lo ≤ m`. `rational_max m hi ≤ hi` by `least for
    rational_max` from `m ≤ hi` and `hi ≤ hi`. This is
    `interval_hull_least` (`:1408`) at `k = MkInterval lo hi`, with the two
    halves' inclusions as its premises.
  - Complete: `rational_min lo m ≤ lo` by `leq_left for rational_min`, and
    `hi ≤ rational_max m hi` by the `rational_max` right bound. This is the
    shape of `interval_inside_hull_left` (`:941`), and it needs no premise.
- **Neighbours.** §21 `interval_split_covers` (`:451`) and §22
  `interval_split_width` (`:468`) are the split family. §21's text
  (`68-numerical.md:1960`) states the gap this closes.
- **No such law exists.** Nothing in `catalog/` or `spec/` relates a
  split's hull to the whole.
- **Spec and tests.**
  - Spec 68 ends with §22 (`68-numerical.md:1977`), so the new section is
    §23.
  - The seed directory is `conformance/stdlib/numerical/`, after
    `seed-interval-split-width.md`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_numeric_interval_split_width.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §23, the two catalog laws with any private helper, a seed, a
package test and the status lines. No §1 to §22 operation or law type
moves, and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-SPLIT-WIDTH-SUM.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the names, the binder order and the placement;
  - whether a halves-inside-whole corollary is also public, or left to
    clients through `interval_inside_trans`;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §23 states the laws by their public types. CV and the
  Architect review it, with a Decision.
- **AC-2.** The seed uses `lo = 0` and `hi = 1`.
  - Its accepted block takes `m = 1/2`, proves both order premises by
    `Refl`, and concludes the sound law's inclusion.
  - Its refused block, on a fresh accepted prefix and in the same form,
    takes `m = 2`. The `m ≤ hi` premise `Refl` refuses, because the hull is
    `[0, 2]`.
  - A second accepted row applies the complete law at `m = 2`, which needs
    no premise.
  - The two sound rows differ only in `m`. AC-0 may replace this seed with
    a stronger discriminator.
- **AC-T.** The package test compares `trusted_base()` before and after
  RationalInterval as identity sets, and that comparison fails when the
  package adds an `Axiom`. No Decision opens until it passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the two order premises.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §22 operation or law type.

Checks: 4 fired. I read `interval_hull`, `interval_inside`,
`interval_inside_sound`, `interval_hull_least`, `interval_inside_hull_left`
and the `rational_min` and `rational_max` bound laws they apply on
`d7d99f1ba`. 3 fired. I grepped `catalog/` and `spec/` for a split hull
law and found only §21's statement that none exists. 8 fired. At `m = 2`
the hull's upper end is `2`, not `≤ 1`, so the sound law's premise
refuses, while the complete law still holds, which separates the two laws.
