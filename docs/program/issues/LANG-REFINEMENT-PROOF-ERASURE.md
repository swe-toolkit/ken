---
id: LANG-REFINEMENT-PROOF-ERASURE
title: "The interpreter is strict in Unknown on pairs and a postulate evaluates to Unknown, so a subset-Σ refinement value whose proof is an open hole would evaluate to Unknown, and the checked-core pair view rejects dependent Σ. Erase a refinement's proof, keyed on the Ω classification, in lowering and in the interpreter"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-REFINEMENT-SUBSET-SIGMA]
github: null
origin: "Language ring, sequenced after LANG-PATH-CONDITION-EVIDENCE, so L1 keeps the runtime ring (operator 2026-09-17: ignored tests are top priority). Operator 2026-10-07: \"agreed, refinements should be real kernel types\" (OQ-refinement-representation DECIDED for the core subset Σ). Architect design and cut evt_30frdrmj45ehg. Steward-filed per COORDINATION section 2. W3. Inert until LANG-REFINEMENT-SUBSET-SIGMA."
---

# A refinement value runs as its carrier

## Objective

A pair at a refinement type is represented at runtime as its carrier
value, and its proof is never evaluated. A relevant Σ is unchanged.

## Settled inputs (Architect, measured at `1153a9fc6`)

- `eval.rs` Pair arm: an Unknown component makes the pair Unknown, and a
  postulate evaluates to Unknown.
- `checked_core.rs:3578` `type_has_dependent_sigma` rejects dependent Σ in
  the checked-core pair view.
- Refinement-ness is the kernel classification `whnf(T) = Sigma(A,φ)`
  with `φ` classified at Ω, never a spelling.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Type-directed erasure keyed on the Ω classification: `Pair(v,π)` at a
refinement becomes `v`, `Proj1` at a refinement becomes the identity, and
`Proj2` is erased. This applies on both the checked-core lowering path and
the interpreter path.

## Acceptance

- **AC-0 (D0, measure only).** For each evaluator, name the plane that
  carries the Σ codomain's sort at the point erasure must decide. The
  interpreter is untyped, so this decides its mechanism (CHECK 5).
- **AC-1.** A hand-built core row with a pair at `Σ Int φ` whose proof is
  an open hole lowers and evaluates to the carrier value on every path.
- **AC-2.** A hand-built relevant Σ (second component not at Ω) stays a
  pair on every path.
- **AC-3.** Runtime parity is unchanged.
- **AC-4 (mutation).** Keying erasure on the first component alone
  reddens AC-2.

## Stop conditions

- An evaluator with no plane carrying the codomain sort: stop to the
  Architect.
- Any kernel, `trusted_base()` or spec change.
