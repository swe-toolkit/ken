---
id: SPEC-SYMBOLIC-ALGEBRA-EXPR-POWER
title: "Spec 66's Expr has EAdd and EMul but no power, so a client writing x^n must nest n multiplications by hand and has no checked statement of how a power differentiates or how exponents add. Add expr_pow over Expr and Nat, with checked laws at zero trust, both by canonical poly_eq: the power rule, that deriv of e^(n+1) equals (n+1) times e^n times deriv e, and exponent addition, that e^(m+n) equals e^m times e^n, with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md:376). Steward-filed per COORDINATION section 2."
---

# Expression powers

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports `expr_pow`, the n-th
power of an `Expr`, defined by repeated `EMul`. Two public checked laws
hold, both stated by canonical equality (`IsTrue (poly_eq ...)`).
**Power rule:** `deriv v (expr_pow e (Suc n))` is `poly_eq` to
`EMul (EMul (EConst (Suc n)) (expr_pow e n)) (deriv v e)`.
**Exponent addition:** `expr_pow e (add m n)` is `poly_eq` to
`EMul (expr_pow e m) (expr_pow e n)`. Spec 66 states the operation and
laws by their public types.

## Settled inputs (main `4b02fc24e`)

- **The carrier.** `Expr` (`NatPolynomial.ken.md:104`) is `EConst Nat |
  EVar Nat | EAdd Expr Expr | EMul Expr Expr`. It has no power
  constructor, and no `pow` exists anywhere under `catalog/packages`.
- **The derivative.** `deriv` (`:971`) takes the product rule at `EMul`
  (`:977`). The derivative laws are `deriv_respects_eval` (`:979`),
  `poly_eq_deriv` (`:1024`) and `deriv_const_factor` (`:1277`).
- **The equality tools.** `poly_eq` (`:676`) is the canonical decision.
  `poly_eq_complete` (`:746`) turns an all-environments `eval` equality
  into `poly_eq`. The congruences are `poly_eq_add` (`:794`),
  `poly_eq_mul` (`:814`), `poly_eq_trans` (`:775`) and
  `poly_eq_substitute` (`:868`).
- **The Nat floor.** `Data.Numeric.Nat.Arithmetic` exports `add`, which
  recurses on its second argument (`Arithmetic.ken.md:18`), `mul` and
  the two distributivity laws. It exports no `mul` associativity or
  commutativity law.
- **The spec.** Spec 66 ends at §15 (`66-symbolic-algebra.md:1253`), so
  the new section is §16. "Factoring" and the other deferrals are at
  `:165`.
- **Seeds and tests.** Seeds go under
  `conformance/stdlib/symbolic-algebra/`; tests are named
  `crates/ken-elaborator/tests/cat_symbolic_algebra_<slug>.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate:
- `expr_pow` and the two laws in `NatPolynomial`, with names, argument
  order and placement as AC-0 rules them;
- any Nat law the proofs need, proved, in `Data.Numeric.Nat.Arithmetic`;
- §16 in spec 66;
- a closed conformance seed;
- a package test;
- the status lines.

## Acceptance

- **AC-0 (D0, Architect specifies, Foundation implementer runs on
  scratch).** A checked development on the landed base, with rc 0, no
  `Axiom`, and `trusted_base()` unchanged as an identity set. It rules:
  - the definition, including whether `expr_pow e Zero` is
    `EConst (Suc Zero)`;
  - the proof route, and whether a Nat law must be added;
  - that no §1 to §15 operation or law type changes.
- **AC-1.** §16 states `expr_pow` and both laws by their public types.
  The Spec Leader and CV review it.
- **AC-2.** The seed's accepted client checks the power rule at a closed
  `e` and `n`, and exponent addition at closed `m` and `n`, each through
  its law. Its refused block pins, in the same form as an accepted
  control, the power rule claimed with coefficient `n` in place of
  `Suc n`. A kernel that accepts it would show the law was not checked.
- **AC-3.** Every NatPolynomial package test that checked before still
  checks. The new package test fails if `expr_pow` is redefined to drop
  one factor.

## Stop conditions

- A law needs an Axiom, or a kernel or surface change.
- A change to an existing §1 to §15 operation or law type.
- Signed coefficients, division or factoring become necessary: each
  stays deferred as a separate node.

Checks: 4 fired: `Expr`, `deriv`, `poly_eq`, `poly_eq_complete` and
the cited congruences were read at the cited lines on `4b02fc24e`. The
Nat floor was grepped and has no `mul` associativity or commutativity,
so AC-0 rules the route rather than the frame assuming it. 3 fired:
`pow`, `power` and `expo` were grepped over `catalog/packages`, spec
66 and the issues, with no hit, so there is no power to reuse.
