---
id: SPEC-NUMERICAL-INTERVAL-SPLIT-WIDTH-SUM
title: "Spec 68 says a bisection step keeps every member (§21), but no law says what it does to the error bound: nothing relates the widths of MkInterval lo m and MkInterval m hi to the width of MkInterval lo hi, so a client cannot show that refining an enclosure tightens it. Rule from a checked D0 at zero trust that, for lo ≤ m ≤ hi, the two half-widths sum by value to the whole width, and state it with a seed that separates the sum from a wrong total"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md, whose Numerical entry is error-bound refinements). SPEC-NUMERICAL-INTERVAL-SPLIT-COVERS gave bisection's membership half; this is its width half. Steward-filed per COORDINATION section 2."
---

# Bisection splits the width

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked law. For
every `lo`, `m` and `hi`, from `IsTrue (rational_leq lo m)` and `IsTrue
(rational_leq m hi)`, it concludes `rational_equiv (rational_add
(interval_width (MkInterval lo m)) (interval_width (MkInterval m hi)))
(interval_width (MkInterval lo hi))`. It holds at zero trust, and spec
68 states it by its public type.

## Settled inputs (main `3542c4c60`; anchors re-checked at release)

- **Width.** `interval_width` (`RationalInterval.ken.md:490`) is
  `rational_sub hi lo`, which is truncated at zero (spec 68 §15).
- **Enablers, all public in `Rational`.**
  - `proof add_cancel for rational_sub` (`Rational.ken.md:620`):
    `y ≤ x` gives `(x ∸ y) + y ≈ x`.
  - `rational_add_cancel_r` (`:582`) cancels a common right addend.
  - `assoc`, `comm` and `respect` for `rational_add` (`:275`, `:258`,
    `:200`), and `trans for rational_leq` (`:526`).

  One route: `lo ≤ hi` by transitivity. Then both `((m ∸ lo) + (hi ∸ m))
  + lo` and `(hi ∸ lo) + lo` are equivalent to `hi`, and the right
  addend `lo` cancels. `add_distrib for rational_sub` is the other
  candidate route. AC-0 picks one.
- **The premises are needed.** With `m` above `hi`, `[1,7]` and `[7,5]`
  have widths 6 and 0 against a whole width of 4, because truncation
  zeroes the inverted half.
- **Neighbours.** `interval_split_covers` (`:450`) is §21.
  `interval_width_add` (`:283`) has the same shape of conclusion.
- **Spec and tests.** Spec 68 ends with §21 (`68-numerical.md:1934`),
  whose roster is nine functions and sixty theorems, so the new section
  is §22. The seed directory is `conformance/stdlib/numerical/`. The test
  pattern is
  `crates/ken-elaborator/tests/cat_numeric_interval_split_covers.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §22, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §21 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-SPLIT-COVERS.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name, the binder order and the placement;
  - the proof route;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §22 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to `[1, 5]` split
  at `2` and concludes that the sum is equivalent to `rational_of_nat
  4`. Its refused block, on a fresh accepted prefix and in the same form,
  claims the sum is equivalent to `rational_of_nat 3`. The two rows
  differ only in the claimed total.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the two order premises.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §21 operation or law type.

Checks: 3 fired. I listed every public law in `RationalInterval` and
`Rational` on `3542c4c60` and grepped spec 68 and the Numeric packages
for split and width together. Only §21 splits, and it says nothing about
width. 4 fired. The obligation is written with the public enablers above,
and AC-0 checks the route. 8 fired. The refused row changes only the
claimed total, so a constant-zero width or a wrong sum fails one of the
two rows. The `[1,7]`, `[7,5]` case shows why both order premises are
needed.
