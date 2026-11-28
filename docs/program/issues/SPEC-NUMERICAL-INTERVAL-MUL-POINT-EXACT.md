---
id: SPEC-NUMERICAL-INTERVAL-MUL-POINT-EXACT
title: "Spec 68 §24 proves that adding a point shifts interval membership exactly (interval_add_point_exact), but its multiplicative partner is missing: §1's interval_mul_encloses gives only the forward enclosure, so a client cannot infer that x is in i from x·t being in i scaled by a positive point t. Rule from a checked D0 at zero trust that, for t with a nonzero numerator, membership of rational_mul x t in interval_mul i (interval_point t) equals membership of x in i, and state it with a seed whose accepted row is open"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md). RationalInterval's exact point law covers addition only. Steward-filed per COORDINATION section 2."
---

# Scaling by a positive point scales membership exactly

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked law. For every
`i : Interval`, `t : Rational` and `x : Rational`, under a premise that `t`
is positive, it states

`Equal Bool (interval_contains (interval_mul i (interval_point t))
(rational_mul x t)) (interval_contains i x)`.

It holds at zero trust, and spec 68 states it by its public type.

## Settled inputs (main `ba243870f`; anchors re-checked at release)

- **The argument, for AC-0 to check.**
  - `interval_mul` (`RationalInterval.ken.md:625`) multiplies endpoints, so
    scaling `MkInterval lo hi` by `interval_point t` gives `MkInterval (lo·t)
    (hi·t)`.
  - `rational_mul_mono` (`Rational.ken.md:700`) gives preservation of each
    endpoint comparison.
  - `rational_leq` cross-multiplies (`Rational.ken.md:177`), so reflection
    reduces to Nat cancellation of a successor factor.
    `leq_nat_mul_right_cancel` (`Nat/Order.ken.md:1025`) is that law.
  - `bool_eq_from_iff` joins the two implications, as §24 does.
- **The premise is needed.** At `t = 0` the claim fails. For `i = [1, 2]`
  and `x = 5`, `5·0 = 0` is in `[0, 0]`, but `5` is not in `[1, 2]`.
- **No such law exists.** RationalInterval has 64 public theorems. Its
  multiplication laws are enclosure, inclusion and width laws, and its one
  exact membership law is `interval_add_point_exact` (§24).
- **Spec and tests.**
  - Spec 68 ends with §24 (`68-numerical.md:2076`), so the new section is
    §25.
  - The seed directory is `conformance/stdlib/numerical/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_numeric_interval_add_point_exact.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §25, the catalog law with any private helper, a seed, the package
test rows and the status lines. No §1 to §24 operation or law type moves,
and there is no kernel or surface change. A division or reciprocal
operation is out of scope.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-ADD-POINT-EXACT.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the positivity premise's form, the binder order and the
    placement;
  - whether the reflection steps are private helpers or inline, and whether
    any needs a public Rational or Nat export (a stop, below);
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §25 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row is open in `x`, at a fixed closed `i` and positive `t`
    such as `[1, 2]` and `2`. It transports membership of `x·2` in `[2, 4]`
    back to membership of `x` in `[1, 2]`. AC-0 shows that the row's
    closing term needs the law.
  - Its refused twin, on a fresh accepted prefix and in the same form,
    drops the positivity premise and claims the law at `t = 0`. That claim
    must refuse: the counterexample above makes it false.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Each touched package test target passes on the exact
  candidate with no name filter. Full-workspace CI is green on the exact
  candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  positivity of `t`.
- The proof needs a new public export outside RationalInterval; report it
  rather than widening Rational or Nat.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §24 operation or law type.

Checks: 4 fired. I read `interval_mul`, `rational_leq`, `rational_mul_mono`
and `leq_nat_mul_right_cancel` on `ba243870f`; the reflection step is a Nat
cancellation by a successor factor. 3 fired. I listed RationalInterval's
64 public theorems and found one exact membership law, for addition. 8
fired. A closed seed row computes without the law (ADD-POINT-EXACT AC-0,
`evt_1b1n2fsh3bvws`), so the witness row is open. The refused twin drops
the premise, which the `t = 0` counterexample refutes.
