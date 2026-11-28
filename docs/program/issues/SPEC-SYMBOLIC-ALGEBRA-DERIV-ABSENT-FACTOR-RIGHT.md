---
id: SPEC-SYMBOLIC-ALGEBRA-DERIV-ABSENT-FACTOR-RIGHT
title: "Spec 66 §25 lets a left factor without the derivative variable pass through deriv (deriv_absent_factor), but its right-factor mirror is missing: deriv v (EMul e c) with v absent from c has no public law, so a client holding the factor on the right must re-derive it through poly_eq_complete and the product rule. Rule from a checked D0 at zero trust that a right factor without the variable leaves the derivative up to poly_eq, and state it with a seed whose accepted row is open"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md). NatPolynomial's absent-factor law covers the left factor only. Steward-filed per COORDINATION section 2."
---

# A right factor without the variable leaves the derivative

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law. For
every `v : Nat`, `c : Expr` and `e : Expr`, under the premise
`Equal Bool (occurs v c) False`, it states

`IsTrue (poly_eq (deriv v (EMul e c)) (EMul (deriv v e) c))`.

It holds at zero trust, and spec 66 states it by its public type.

## Settled inputs (main `a24d6a355`; anchors re-checked at release)

- **The argument, for AC-0 to check.**
  - `deriv` (`NatPolynomial.ken.md:991`) sends `EMul a b` to
    `EAdd (EMul (deriv v a) b) (EMul a (deriv v b))`, so the right factor's
    derivative is the second summand.
  - `deriv_absent_factor` (`NatPolynomial.ken.md:1573`) is the left-factor
    proof: `poly_eq_complete` over every environment, with `deriv_absent`
    taking the absent factor's derivative to `Zero`.
  - The mirror eliminates `mul x Zero` and `add y Zero`. Nat's right-zero
    proofs are private, but their bodies are `Proved` and `Refl`
    (`Nat/Arithmetic.ken.md:80`, `:30`), so both steps should close by
    computation without a new export. AC-0 picks the terms.
- **The premise is needed.** At `v = Zero`, `c = EVar Zero` and
  `e = EConst (Suc Zero)`, the derivative of `1·x` is `1`, but the claimed
  right side `0·x` is `0`.
- **No such law exists.** NatPolynomial has 44 public theorems. The one
  factor law is `deriv_absent_factor` (§25), with the factor on the left,
  and `deriv_const_factor` is its literal case.
- **Spec and tests.**
  - Spec 66 ends with §25 (`66-symbolic-algebra.md:1894`), so the new
    section is §26.
  - The seed directory is `conformance/stdlib/symbolic-algebra/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_symbolic_algebra_deriv_absent_factor.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §26, the catalog law, a seed, the package test rows and the status
lines. No §1 to §25 operation or law type moves, and there is no kernel,
surface or Nat change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-DERIV-ABSENT-FACTOR.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §26 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row is open in `e`, at `v = Zero` and
    `c = EVar (Suc Zero)`, supplying `Proved` for absence. AC-0 shows that
    the row's closing term needs the law.
  - Its refused twin, on a fresh accepted prefix and in the same form, sets
    `c = EVar Zero`. It must refuse at the absence argument, and a closed
    control at `e = EConst (Suc Zero)` carries the conclusion's falsity.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Each touched package test target passes on the exact
  candidate with no name filter. Full-workspace CI is green on the exact
  candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  absence of `v` from `c`.
- The proof needs a new public export outside NatPolynomial; report it
  rather than widening Nat.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §25 operation or law type.

Checks: 4 fired. I read `deriv`, `deriv_absent_factor` and Nat's right-zero
laws on `a24d6a355`; the mirror replaces the two left-zero eliminations with
right-zero ones. 3 fired. I listed NatPolynomial's 44 public theorems and
found one factor law, with the factor on the left. 8 fired. A closed seed
row computes without the law, so the witness row is open in `e`, and the
twin puts the derivative variable in the factor, which the `1·x`
counterexample refutes.
