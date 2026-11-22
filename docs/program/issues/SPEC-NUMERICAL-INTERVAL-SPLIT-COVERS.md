---
id: SPEC-NUMERICAL-INTERVAL-SPLIT-COVERS
title: "Spec 68 refines an enclosure only by inclusion and width, and no law says that splitting an interval at a point loses no member, so a client bisecting an enclosure to tighten an error bound must re-derive rational order totality at every split. Rule from a checked D0 at zero trust that every member of MkInterval lo hi is a member of MkInterval lo m or of MkInterval m hi, for any m, and state it with a seed that separates the cover from either half alone"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md, whose Numerical entry is error-bound refinements). Bisection is the refinement step of interval methods, and the package has no law for it. Steward-filed per COORDINATION section 2."
---

# Splitting an enclosure keeps every member

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked law. For
every `lo`, `hi`, `m` and `x`, from `IsTrue (interval_contains
(MkInterval lo hi) x)`, it concludes that `x` is contained in
`MkInterval lo m` or in `MkInterval m hi`. It holds at zero trust, and
spec 68 states it by its public type. With it, a bisection step keeps
every enclosed value without a fresh order argument.

## Settled inputs (main `1a98f6d8a`; anchors re-checked at release)

- **Membership.** `interval_contains` (`RationalInterval.ken.md:448`) is
  `bool_and (rational_leq lo x) (rational_leq x hi)`.
- **Totality.** `proof total for rational_leq` (`Rational.ken.md:554`)
  turns `rational_leq x m = False` into `rational_leq m x = True`. So
  `x ≤ m` puts `x` in the lower half with the premise's `lo ≤ x`, and
  otherwise `m ≤ x` puts it in the upper half with `x ≤ hi`.
- **The gap.** No public law in the package or in `Rational` concludes
  membership in either of two sub-intervals; the inclusion laws
  (`interval_inside_*`) only move members into a larger interval.
  `interval_intersect_sound` and `_complete` (`:528`, `:581`) split a
  conjunction, not a disjunction.
- **Spec and tests.** Spec 68 ends with §20, a zero-width enclosure
  (`68-numerical.md:1873`), so the new section is §21. The seed directory
  is `conformance/stdlib/numerical/`. The test pattern is
  `crates/ken-elaborator/tests/cat_numeric_interval_width_zero_equiv.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §21, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §20 operation or law type moves, and
no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-ZERO-MEMBERS-EQUIV.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and the binder order;
  - whether the conclusion is `IsTrue (bool_or ...)` or a propositional
    disjunction, by what a client can eliminate today;
  - that no premise on `m` is needed. The halves may be inverted when `m`
    lies outside the interval, and the law still holds.
- **AC-1.** §21 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to a closed
  interval, a split point and two members, one below and one above the
  split point, and eliminates each conclusion to its correct half. Its
  refused block, on a fresh accepted prefix, claims that the member above
  the split point lies in the lower half, and refuses. So the upper half
  carries members the lower half cannot.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the one containment.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §20 operation or law type.

Checks: 3 fired. I listed every public law in `RationalInterval` and
every public `rational_leq` proof on `1a98f6d8a`, and grepped `catalog/`,
`spec/50-stdlib/68-numerical.md` for split or bisect: none concludes
membership in a sub-interval. 4 fired. I wrote the obligation in the
delivered vocabulary: a case on `rational_leq x m`, the `False` arm
through `total for rational_leq`, and each half's `bool_and` built from
one projection of the premise. 8 fired. The refused row fails on a member
above the split point, so it separates the disjunction from its first
arm alone.
