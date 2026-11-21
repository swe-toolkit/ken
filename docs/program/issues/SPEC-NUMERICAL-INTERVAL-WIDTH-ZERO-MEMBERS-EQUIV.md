---
id: SPEC-NUMERICAL-INTERVAL-WIDTH-ZERO-MEMBERS-EQUIV
title: "Spec 68 §18 bounds the truncated difference of two enclosed values by the interval's width, but no law takes the limiting case, so a client holding a zero-width enclosure cannot conclude that it has pinned its value exactly without re-deriving order facts from truncated subtraction. Rule from a checked D0 at zero trust that, when interval_width i is rational_equiv to zero, any two values contained in i are rational_equiv, and state it with a seed that separates a zero-width interval from one of positive width"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md, whose Numerical entry is error-bound refinements). §18 bounds a distance by width, and no law says a zero error bound is exact. Steward-filed per COORDINATION section 2."
---

# A zero-width enclosure fixes its value

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked law. For
every `i`, `x` and `y`, from `rational_equiv (interval_width i)
rational_zero`, `IsTrue (interval_contains i x)` and
`IsTrue (interval_contains i y)`, it concludes `rational_equiv x y`. It
holds at zero trust, and spec 68 states it by its public type.

## Settled inputs (main `15e57bf39`)

- **The bound.** `interval_width_bounds_sub` (`RationalInterval.ken.md:389`)
  gives `rational_sub y x ≤ interval_width i` for two contained values.
  Applied in both orders, it bounds both truncated differences by the
  width.
- **The limit.** `rational_sub` truncates at zero (spec 68 §15), so a
  truncated difference bounded by zero says one value is at most the
  other. `interval_point_sound` (`:159`) already closes an equivalence from
  two order facts with `(proof antisym for rational_leq)`
  (`Rational.ken.md:541`).
- **The gap.** `Rational.ken.md` has no public law from
  `rational_sub a b ≤ 0` to `a ≤ b`. `interval_width_point` (`:269`) gives
  the converse case, zero width for a point, and nothing goes from zero
  width to equal members.
- **Spec and tests.** Spec 68 ends with §18 (`68-numerical.md:1703`), so
  the new section is §19. The seed directory is
  `conformance/stdlib/numerical/`. The test pattern is
  `crates/ken-elaborator/tests/cat_numeric_interval_width_bounds_sub.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §19, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §18 operation or law type moves, and
no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-BOUNDS-DISTANCE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and the binder order;
  - whether the zero-width premise is `rational_equiv` to `rational_zero`
    or `rational_leq` of the width to `rational_zero`;
  - where a truncated-subtraction-to-order helper lives, in `Rational` or
    private to the interval package.
- **AC-1.** §19 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to a closed
  zero-width interval and two contained values that are rational_equiv but
  not syntactically equal, concluding `rational_equiv`. Its refused block,
  on a fresh accepted prefix, applies the law to a positive-width interval
  in the same form, with the containment premises discharged as in the
  accepted row. The zero-width premise fails to check there, so the law is
  not usable when the width is positive. The two rows differ only in the
  interval.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond zero width and the two containments.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §18 operation or law type.

Checks: 3 fired. I listed every public width law and every public
`Rational` theorem: none goes from zero width, or from a zero truncated
difference, to order or equivalence. 4 fired. I wrote the obligation in
the delivered vocabulary: `interval_width_bounds_sub` in both orders, a
zero bound read as order through truncation, and the antisymmetry proof
`interval_point_sound` already uses. 8 fired. In the refused row the
zero-width premise is false, so it refuses exactly when the law would be
read as holding for an interval of positive width.
