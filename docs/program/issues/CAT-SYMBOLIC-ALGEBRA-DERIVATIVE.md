---
id: CAT-SYMBOLIC-ALGEBRA-DERIVATIVE
title: "Implement spec 66 §5's formal derivative and its checked laws in Algorithm.SymbolicAlgebra.NatPolynomial, with a package test that runs the §5 seed"
status: draft
owner: foundation
size: S
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

## Settled inputs

Spec 66 §5 and its seed, once `SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE`
lands. The Architect's D0 is the reference development. This frame is
completed from §5 at release.

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
