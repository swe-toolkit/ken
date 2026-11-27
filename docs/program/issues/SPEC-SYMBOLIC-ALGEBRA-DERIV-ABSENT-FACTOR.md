---
id: SPEC-SYMBOLIC-ALGEBRA-DERIV-ABSENT-FACTOR
title: "Spec 66 pulls a literal constant out of a derivative (deriv_const_factor, for EConst k) and says a derivative in an absent variable evaluates to zero (deriv_absent), but no law pulls out a factor that is constant only because the variable does not occur in it: a client differentiating y * e in x must unfold the product rule and discharge the y-derivative by hand. Rule from a checked D0 at zero trust that, when occurs v c is False, deriv v (EMul c e) is poly_eq to EMul c (deriv v e), and state it with a seed whose accepted row is open in e"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md); the public Core bool_eq_from_iff node runs between them. deriv_const_factor covers literal constants only. Steward-filed per COORDINATION section 2."
---

# A factor without the variable leaves the derivative

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law.
For every `v`, `c` and `e`, from `Equal Bool (occurs v c) False`, it
states

`IsTrue (poly_eq (deriv v (EMul c e)) (EMul c (deriv v e)))`.

It holds at zero trust, and spec 66 states it by its public type.

## Settled inputs (main `51b938727`; anchors re-checked at release)

- **The operations.**
  - `deriv` (`NatPolynomial.ken.md:989`) sends `EMul a b` to `EAdd (EMul
    (deriv v a) b) (EMul a (deriv v b))`.
  - `occurs` (`:1183`) is `bool_or` over both operands of `EAdd` and `EMul`,
    and `False` on `EConst`.
- **The argument, for AC-0 to check.**
  - `poly_eq_complete` (`:764`) reduces the law to a pointwise equality of
    evaluations.
  - `deriv_absent` (`:1191`) gives `eval env (deriv v c) = Zero` from the
    premise.
  - The rest is `deriv_const_factor`'s (`:1566`) closing chain: `zero_l for
    mul` under `cong`, then `zero_l for add`.
- **The literal case is a corollary.** `occurs v (EConst k)` is `False` by
  computation. AC-0 rules whether `deriv_const_factor` stays as it is or is
  restated through the new law. Its type does not move.
- **No such law exists.** NatPolynomial has 61 public names. Its only
  factor law is `deriv_const_factor`, which is limited to `EConst k`.
- **Spec and tests.**
  - Spec 66 ends with §24 (`66-symbolic-algebra.md:1836`), so the new
    section is §25.
  - The seed directory is `conformance/stdlib/symbolic-algebra/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_symbolic_algebra_deriv_constant_factor.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §25, the catalog law with any private helper, a seed, the package
test rows and the status lines. No §1 to §24 operation or law type moves,
and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-DERIV-FRESH.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether `deriv_const_factor`'s body changes;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §25 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row takes `v = 0` and `c = EVar 1`, and is open in `e`. It
    applies the law, with the premise closed by computation, to conclude the
    `poly_eq` for every `e`.
  - Its refused twin, on a fresh accepted prefix and in the same form, takes
    `c = EVar 0` and claims the same factoring. That claim must refuse.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  absence.
- The proof needs a new public export outside NatPolynomial; report it
  rather than widening another package.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §24 operation or law type.

Checks: 4 fired. I read `deriv`, `occurs`, `deriv_absent`,
`deriv_const_factor` and `poly_eq_complete` on `51b938727`. The argument
composes public laws with the closing chain the literal case already uses.
3 fired. I listed NatPolynomial's 61 public names and found one factor law,
limited to literals. 8 fired. A closed seed row computes without the law
(ADD-POINT-EXACT AC-0, `evt_1b1n2fsh3bvws`), so the witness row is open in
`e`, and the refused twin makes the variable occur in the factor.
