---
id: SPEC-REFINEMENT-SUBSET-SIGMA
title: "The spec describes refinements as carrier types with elaborator-tracked predicates; the operator selected the core subset Σ. Make the normative text say a refinement is Σ x:A. φ with an Ω second component, introduced by a pair plus obligation and forgotten by Proj1, with no subtyping under type formers"
status: ready
owner: spec-enclave
size: M
tier: T1
gate: none
depends_on: []
blocks: [LANG-REFINEMENT-SUBSET-SIGMA]
github: null
origin: "Operator 2026-10-07: \"agreed, refinements should be real kernel types\" (OQ-refinement-representation DECIDED for the core subset Σ). Architect design and cut evt_30frdrmj45ehg. Steward-filed per COORDINATION section 2."
---

# The spec says a refinement is a subset Σ

## Objective

The normative text describes refinements exactly as the Architect's design
note (`evt_30frdrmj45ehg`, DESIGN 1-7), so the language WP builds to the
spec rather than to the note.

## Settled inputs

- `spec/90-open-decisions.md` records `OQ-refinement-representation`
  DECIDED for `{x:A|φ} ⇒ Σ x:A. φ` with an Ω second component.
- The kernel already forms, checks and converts the Σ (Architect probe on
  `1153a9fc6`): `sort_sigma` gives `Type 0`, pairs differing only in the
  proof convert, and `Σ Int φ` does not convert with `Int`.

## Deliverable

Normative edits, plus conformance rows for each behavioral claim:

- 34 §5: refinement syntax elaborates to the subset Σ; a named refinement
  is a transparent definition of that Σ.
- 21 §2, §6.3, §6.5: introduction is a pair whose proof is a discharged
  candidate or the obligation hole; elimination is `Proj1`; path
  conditions carry evidence terms.
- 22 §2.1: introduction is pair plus obligation, forgetting is `Proj1`,
  and there is no coercion under a type former or binder.
- 18a §5.9.1: `Char` is `Σ Int isScalar`, and a Char literal is a pair
  with a closed witness.
- 42: refinement proofs are never evaluated; erasure is keyed on the Ω
  classification.
- 21 §5.4: `proved` requires that no open obligation hole or unaccepted
  postulate is reachable from the certificate.
- Every other normative consumer of the representation, reconciled in
  this WP rather than a follow-on: 34 summary and §§7-8; 21 §§7-8; 22
  summary, §1.1, §2.5 and §5; the 57 refinement-view row; 62 §§3 and 9;
  18a Char law transport; and 42's strict-pair and Unknown rule. Any
  further consumer the sweep finds is in scope on the same terms
  (spec leader `evt_5b8nszx97qbtj`).

## Acceptance

- **AC-1.** Each edit above is made, and no section still states the
  carrier encoding as normative. The conformance validator votes on the
  exact tip.
- **AC-2.** Each conformance row names an observation the carrier encoding
  does not also produce, such as `List Five` against `List Int` being a
  type mismatch.

## Stop conditions

- A design point the note leaves open, or one that conflicts with another
  normative section: stop to the Architect.
