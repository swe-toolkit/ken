---
id: SPEC-SYMBOLIC-ALGEBRA-EXPR-POWER-MUL
title: "Spec 66 §16 gives expr_pow the power rule and exponent addition but no multiplicative exponent law, so a client cannot state that a power of a power multiplies exponents or that a power of a product is the product of powers. Rule from a checked D0 at zero trust both laws by canonical poly_eq, (e^m)^n = e^(m·n) and (a·b)^n = a^n·b^n, and state them with a conformance seed"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md:376); expr_pow (spec 66 §16) has the additive exponent law only. Steward-filed per COORDINATION section 2."
---

# Multiplying exponents

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports two public checked
laws on `expr_pow`, both by canonical equality (`IsTrue (poly_eq ..)`):
**power of a power**, `expr_pow (expr_pow e m) n` against
`expr_pow e (mul m n)`; and **power of a product**,
`expr_pow (EMul a b) n` against `EMul (expr_pow a n) (expr_pow b n)`.
They hold at zero trust, and spec 66 states them by their public types.

## Settled inputs (main `90ed44bb1`)

- **The operation.** `expr_pow` (`NatPolynomial.ken.md:1325`) is
  `Zero ↦ EConst (Suc Zero); Suc k ↦ EMul (expr_pow e k) e`. Its public
  laws are `deriv_expr_pow` (`:1331`) and `expr_pow_add` (`:1342`),
  which goes through `poly_eq_complete` (`:750`).
- **The equality tools.** `poly_eq_trans` (`:779`), `poly_eq_sym`
  (`:774`), `poly_eq_mul` (`:818`) and `poly_eq_complete`.
- **The Nat floor.** `mul a (Suc b) = add (mul a b) a` by definition
  (`Data/Numeric/Nat/Arithmetic.ken.md:24`), so the power-of-a-power
  `Suc` step lands on `expr_pow_add` at `add (mul m n) m`. `comm` and
  `assoc` for `mul` are public (`:152`, `:213`), for the product law's
  evaluation route.
- **The gap.** `expr_pow` appears in four files: NatPolynomial, spec 66
  §16, its seed (`conformance/stdlib/symbolic-algebra/seed-expression-power.md`)
  and its test. None states a power of a power or a power of an `EMul`.
- **Spec and tests.** Spec 66 ends at §16 (`66-symbolic-algebra.md:1297`),
  so the new section is §17. The test pattern is
  `crates/ken-elaborator/tests/cat_symbolic_algebra_expression_power.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §17, the two catalog laws with any private helper, a seed, a
package test and the status lines. No §1 to §16 operation or law type
moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-TIME-TRACE-MERGE-ALL-COUNT.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the names, binder order, proof route
  (induction through `expr_pow_add` and `poly_eq_mul`, or evaluation
  through `poly_eq_complete`), and whether any Nat law must be added,
  naming its owner.
- **AC-1.** §17 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies each law at closed
  arguments: power of a power at `m = 2`, `n = 3`, and power of a
  product at `n = 2`, with `a` and `b` distinct variables. Its refused
  block pins, in the same form as an accepted row and on a fresh
  accepted prefix, the power of a power claimed at `add m n`, and the
  power of a product claimed as `EMul (expr_pow a n) b`.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §16 operation or law type.

Checks: 4 fired: the power-of-a-power obligation's `Suc` step was
written out: `expr_pow (expr_pow e m) (Suc n)` is
`EMul (expr_pow (expr_pow e m) n) (expr_pow e m)` by definition, and
`expr_pow e (mul m (Suc n))` is `expr_pow e (add (mul m n) m)`, which
`expr_pow_add` and `poly_eq_mul` with the hypothesis close. 3 fired:
the `expr_pow` grep covered `catalog/packages`, `spec/`, `conformance/`
and `crates/`. 8 fired: at `m = 2`, `n = 3` the additive mutant gives
exponent 5 against 6, so the refused row separates it; a product law
that drops the second power differs at `n = 2` in `b`'s degree.
