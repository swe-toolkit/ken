---
id: CAT-SYMBOLIC-ALGEBRA-DERIVATIVE
title: "Implement spec 66 §5's formal derivative and its checked laws in Algorithm.SymbolicAlgebra.NatPolynomial, with a package test that runs the §5 seed"
status: ready
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE]
blocks: []
github: null
origin: "Catalog half of SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE. Same shape as CAT-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION. Steward-filed per COORDINATION section 2."
---

# Formal derivative in NatPolynomial

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports spec 66 §5's
`deriv` and each law §5 rules, as checked proof bodies at zero trust.

## Settled inputs (spec 66 §5, SPEC AC-1 `1697a04f9`, AC-2 `f28350f05`)

- **Surface (§5).** `deriv : Nat → Expr → Expr` with the four structural
  equations, the private `var_deriv` over the existing private
  `nat_same`, and two public laws: `deriv_respects_eval` and
  `poly_eq_deriv`. The structural equations are not exported theorems.
- **Proof route (§5.2, Architect D0 `evt_1cfjmd16qpp6x`).**
  - A private Taylor identity `deriv_taylor` over a private `bump_env`
    and a private remainder `taylor_rem`, by structural induction.
  - Its arithmetic goes through §2's `poly_eq_sound` under a private
    `cons_env`.
  - Extraction at `t = Suc (add d1 d2)` via `add_cancel_left`,
    `mul_cancel_positive` and `remainder_unique`.
  - It does not route through `canonicalize_complete`.
- **D0 evidence (§5.3).** `ken check` rc 0, `trusted_base()` 114 to 114,
  no new import. Dropping the right product-rule term is rejected with
  K0004.
- **Seed.** `conformance/stdlib/symbolic-algebra/seed-nat-polynomial-derivative.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The package extension, plus a package test in `crates/ken-elaborator/tests/`
that runs the seed's positive client and refusal.

## Acceptance

- **AC-1.** Each §5 law is a checked public export. `trusted_base()` is
  unchanged.
- **AC-2.** The seed's positive client is accepted. Its refusal is
  rejected at the claimed equality, not earlier.
- **AC-3 (mutation, QA).** Dropping one product-rule term reddens the
  seed's positive client or the headline law.

## Stop conditions

- Any law that §5 rules cannot be proved without an assumption.
- Any `trusted_base()` change.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.

Checks: 4 fired: §5's surface and the D0's private helper names are
read from AC-1 `1697a04f9`, not inferred. 8 fired: AC-3's dropped
product-rule term is the mutation the D0 measured as K0004, so a
correct build reddens on it.
