---
id: SPEC-NUMERICAL-INTERVAL-ADD-POINT-EXACT
title: "Spec 68 proves that interval addition encloses every sum of members (interval_add_encloses) and that adding two point intervals gives the point of the sum (interval_add_point), but no law says that shifting by a point loses nothing: a client that translates an enclosure by a known offset cannot conclude that a shifted value outside the result was outside the original. Rule from a checked D0 at zero trust that interval_contains (interval_add i (interval_point t)) (rational_add x t) equals interval_contains i x, and state it with a seed whose accepted row needs the converse"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md). Enclosure is one-directional today; translation by a point is the case where it is exact. Steward-filed per COORDINATION section 2."
---

# Shifting by a point is exact

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked law. For
every `i`, `t` and `x` it states

`Equal Bool (interval_contains (interval_add i (interval_point t))
(rational_add x t)) (interval_contains i x)`.

It holds at zero trust, and spec 68 states it by its public type.

## Settled inputs (main `767e25ec2`; anchors re-checked at release)

- **The operations.** `interval_contains` (`RationalInterval.ken.md:582`)
  is `bool_and (rational_leq lo x) (rational_leq x hi)`. `interval_add`
  (`:587`) adds bounds pairwise, and `interval_point x` (`:603`) is
  `MkInterval x x`. `Rational` is `MkRational (num : Nat) (den_tail : Nat)`,
  so every value is non-negative.
- **The argument, for AC-0 to check.**
  - The shifted interval unfolds to `MkInterval (lo + t) (hi + t)`, so the
    law reduces to two equalities of the form `rational_leq (a + t) (b + t)
    = rational_leq a b`.
  - True to True is `rational_add_mono` (`Rational.ken.md:561`) with
    `refl for rational_leq`. That is also `interval_add_encloses` (`:98`)
    with `interval_point_contains` (`:160`).
  - The converse is order cancellation. No public Rational law states it:
    `rational_add_cancel_r` (`:582`) cancels `rational_equiv`, not
    `rational_leq`. AC-0 rules the route: a private helper in
    RationalInterval, for example through `rational_sub_mono` (`:679`) and
    `respect for rational_leq` (`:240`), or by cross-multiplication on the
    representations.
- **No such law exists.** Of the 73 public names in RationalInterval,
  `interval_add_encloses` is the only addition-membership law, and it is
  one-directional. Nothing in `catalog/` or `spec/` states that translation
  is exact.
- **Spec and tests.**
  - Spec 68 ends with §23 (`68-numerical.md:2017`), so the new section is
    §24.
  - The seed directory is `conformance/stdlib/numerical/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_numeric_interval_width_zero_equiv.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §24, the catalog law with any private helper, a seed, the package
test rows and the status lines. No §1 to §23 operation or law type moves,
and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-ZERO-MEMBERS-EQUIV.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - the cancellation route and whether it stays private;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any
    count-bearing roster sentence.
- **AC-1.** §24 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed takes `i = [1, 2]` and `t = 1`.
  - Its accepted block applies the law at `x = 0` to conclude that the
    shifted interval `[2, 3]` does not contain `1`. This row needs the
    converse, because enclosure alone cannot prove a `False`.
  - Its refused block, on a fresh accepted prefix and in the same form,
    claims the unshifted point: that `[2, 3]` contains `x = 1` because
    `[1, 2]` does. That claim must refuse.
  - AC-0 picks the closing terms and may add an open row or strengthen the
    seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- The cancellation route needs a new public export from `Rational`; report
  it rather than widening another package.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §23 operation or law type.

Checks: 4 fired. I read `interval_contains`, `interval_add`,
`interval_point`, `Rational`, `rational_add_mono`, `rational_add_cancel_r`,
`rational_sub_mono` and `respect for rational_leq` on `767e25ec2`; the
forward half composes public laws, and the converse needs an order
cancellation no public law states. 3 fired. I listed the 73 public names in
RationalInterval and found one addition-membership law, one-directional.
8 fired. The accepted row concludes a `False` membership, which an
enclosure-only law cannot produce, and the refused row drops the shift.
