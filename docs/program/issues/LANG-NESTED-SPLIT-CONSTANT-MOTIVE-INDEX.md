---
id: LANG-NESTED-SPLIT-CONSTANT-MOTIVE-INDEX
title: "A nested indexed split whose motive is constant is refused when its index is concrete or repeated, although a constant motive does not depend on the index. Run the distinct-variable-index check only when the split needs reverting"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-INFER-MATCH-INDEXED-COMPLETE]
blocks: []
github: null
origin: "Architect evt_5nq6e798cdgtf (carry 1 of evt_40v1n9fxgrrqb) on LANG-INFER-MATCH-INDEXED-COMPLETE exact 593cae3e6: over-restrictive, not unsound. Steward-filed per COORDINATION section 2."
---

# A constant nested motive accepts any index

## Objective

A nested split of an indexed family is accepted when its motive is constant
(Δ = ∅ and the result does not mention the split value), whatever its index
terms are. The refusals that protect a reverting motive stay.

## Settled inputs (Architect `evt_40v1n9fxgrrqb`, read at `593cae3e6`)

- `compile_match_matrix` (`elab.rs:18296`) computes
  `needs_reverting = !dependent_tail.is_empty() || result_mentions_split`.
- `check_nested_index_variables` (`elab.rs:16703`) runs unconditionally at
  `:18305`. It refuses a concrete index, a repeated index, and an index shared
  with an ambient binder.
- On the `!needs_reverting` path the motive is constant (the `debug_assert`
  at `:18298`), so it is index-agnostic. The refusal there is
  over-restrictive, not a soundness guard.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

`check_nested_index_variables` runs only when `needs_reverting`. The nested
omission check before it is unchanged, so an omitted constructor stays an
`ExhaustivenessError` at any index.

## Acceptance

- **AC-0.** On the landed base, a source program with a nested split at a
  concrete index and a constant result is refused with "nested indexed split
  needs distinct variable indices". If it is accepted, stop: the node is void.
- **AC-1.** That program is accepted and evaluates to the expected value. A
  repeated-index twin with a constant result is accepted too.
- **AC-2 (fence).** The same concrete-index and repeated-index splits with a
  result that mentions the split value are still refused with today's
  diagnostics. The pair differs only in the result type.
- **AC-3.** A nested omission at a concrete index is still an
  `ExhaustivenessError`. The targeted match and pattern suites stay green, and
  `trusted_base()` is unchanged.

## Stop conditions

- Lowering or the kernel rejects the accepted program: the constant path
  assumes variable indices somewhere else. Stop to the Architect.
- Any kernel or spec change.
