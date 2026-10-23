---
id: CAT-OPTIMIZATION-ARGMAX
title: "Implement spec 72 §3's argmax and its checked laws in the Algorithm.Optimization package, with a package test that runs the §3 seed"
status: draft
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-OPTIMIZATION-ARGMAX]
blocks: []
github: null
origin: "Catalog half of SPEC-OPTIMIZATION-ARGMAX. Same shape as CAT-OPTIMIZATION-ARGMIN-OPTION. Completed from §3 when the spec lands. Steward-filed per COORDINATION section 2."
---

# argmax in Algorithm.Optimization

## Objective

The package exports spec 72 §3's `argmax` and each law §3 rules, as
checked proof bodies at zero trust.

## Settled inputs

Completed from spec 72 §3 and the Architect D0 when
SPEC-OPTIMIZATION-ARGMAX lands.

## Deliverable

The package extension, plus a package test in
`crates/ken-elaborator/tests/` that runs the seed's positive client and
refusal.

## Acceptance

- **AC-1.** Each §3 law is a checked public export. `trusted_base()` is
  unchanged.
- **AC-2.** The seed's positive client is accepted. Its refusal is
  rejected at the claimed result, not earlier.
- **AC-3 (mutation, QA).** A tie rule that keeps the later tied
  candidate reddens the seed's positive client or a tie law.

## Stop conditions

- Any law that §3 rules cannot be proved without an assumption.
- Any `trusted_base()` change, or any change to `argmin`'s public
  surface.
