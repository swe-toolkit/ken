---
id: SPEC-NUMERICAL-INTERVAL-WIDTH-INSIDE-MONO
title: "Spec 68 §15 gives interval width a point law and a sum law but no relation to inclusion, so a client that narrows an enclosure cannot conclude its error bound shrank. Rule from a checked D0 at zero trust that an interval inside another has no greater width, with the truncated Rational subtraction monotonicity it needs, and state them with a conformance seed"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md, whose Numerical entry is error-bound refinements). Steward-filed per COORDINATION section 2."
---

# Width shrinks under inclusion

## Objective

`Algorithm.Numeric.RationalInterval` exports a law that an interval
inside another has no greater width, as `rational_leq` of the two
widths. It holds at zero trust, and spec 68 states it by its public
type, with any Rational or Nat law it needs.

## Settled inputs (main `c565e0cf3`)

- **The gap.** Spec 68 §15 states only `interval_width_point` and
  `interval_width_add`. No catalog law relates `interval_width` to
  `interval_inside`, and a grep of `catalog/packages` for a
  `rational_sub` monotonicity law finds 0 hits.
- **The operations.** `interval_width (MkInterval lo hi)` is
  `rational_sub hi lo`
  (`catalog/packages/Algorithm/Numeric/RationalInterval.ken.md:305`).
  `interval_inside (MkInterval a b) (MkInterval c d)` is
  `bool_and (rational_leq c a) (rational_leq b d)` (`:462`), split by
  `proof left/right for bool_and` as in `interval_inside_sound` (`:470`).
  `rational_sub` truncates `sub (mul nx (Suc dy)) (mul ny (Suc dx))`
  over `d·e` (`catalog/packages/Data/Numeric/Rational.ken.md:159`);
  `rational_leq` cross-multiplies (`:176`).
- **The Nat pieces exist, privately.** `Algorithm.Geometry.NatBox` has
  private `sub_left_mono` (`NatBox.ken.md:2525`) and
  `sub_right_antitone` (`:2491`), used by its `area_inside`. Nat.Order
  publishes `sub` (`Order.ken.md:322`) and `leq_nat_mul_right_mono`
  (`:865`), which Rational already imports (`Rational.ken.md:45`).
- **The law holds without a validity premise.** For `i = [a,b]` inside
  `k = [c,d]`, `c ≤ a` and `b ≤ d` give `b − a ≤ d − c`, and truncating
  both at zero preserves the order. An inverted `i` has width zero.
- **Seeds and tests.** Seeds sit under `conformance/stdlib/numerical/`.
  The test pattern is
  `crates/ken-elaborator/tests/cat_numeric_interval_width.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

A new spec 68 section, the catalog laws, a seed and the status lines.
No §1 to §15 operation or law type moves, and no kernel or surface
change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-GRAPHS-UNION.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the law set and names, starting from:
  - interval: `IsTrue (interval_inside i k)` implies
    `IsTrue (rational_leq (interval_width i) (interval_width k))`;
  - Rational: `rational_sub` is monotone in its first argument and
    antitone in its second, by `rational_leq`.
  It rules which helpers stay private, and whether NatBox's two Nat
  subtraction laws move to Nat.Order as public laws with NatBox
  consuming them, naming the owner of that change.
- **AC-1.** The new section states each ruled law by its public type.
  The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies each law to a closed
  instance, including `[1,2]` inside `[0,4]` (widths 1 and 4), an
  inverted inner interval, and the width of an `interval_intersect`
  bounded through `interval_intersect_inside_left`. Its refused block
  pins, in the same form as an accepted row, the reversed claim that
  `[0,4]` has width at most that of `[1,2]`, on a fresh accepted
  prefix.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the inclusion.
- A change to an existing §1 to §15 operation or law type.

Checks: 4 fired: `interval_width`, `interval_inside`, the `bool_and`
split, `rational_sub`, `rational_leq`, `leq_nat_mul_right_mono` and
NatBox's two private sub laws were read at `c565e0cf3` on the cited
lines; the interval obligation is `rational_leq (rational_sub b a)
(rational_sub d c)` from `c ≤ a` and `b ≤ d`, which is the two-sided
Rational monotonicity law. 3 fired: the grep for an existing
subtraction monotonicity law covered `catalog/packages`; its only
hits are NatBox's private pair. 8 fired: a constant-zero width and a
width that returns `hi` both satisfy the inclusion law, so it does
not characterize width; §15's point law and the package's nonzero
value row already exclude those mutants. A width that returns `lo`
fails the law at `[1,2]` inside `[0,4]`, and the reversed twin is
refused by a correct width.
