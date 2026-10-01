---
id: LANG-NESTED-SPLIT-CONSTANT-MOTIVE-INDEX
title: "A nested indexed split whose motive is constant is refused when its index is concrete or repeated, although a constant motive does not depend on the index. Run the distinct-variable-index check only when the split needs reverting"
status: merged
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
- **AC-2 (fence, Architect `evt_6a1bennx1sc5s`).** The reachable reverting
  trigger is a dependent tail: a result type never mentions the nested split
  value (`ret_ty_slot` is set only through `lower_by`). The outer constructor
  carries a second field, left `_` in the arm, whose type mentions the nested
  field (`HoldD`/`HoldPairD`), and its twin has an unrelated second field
  (`HoldN`/`HoldPairN`). The pair differs only in whether a pending sibling
  field's type mentions the nested split field.
  - The N twins are accepted.
  - The concrete D twin is refused with a reason containing "nested indexed
    split needs distinct variable indices". The repeated D twin's reason
    contains "nested indexed split repeats an index".
  - Assert the clause text. A D twin refused before the index clause is a
    stop to the Architect.
- **AC-3.** A nested omission at a concrete index is still an
  `ExhaustivenessError`. The targeted match and pattern suites stay green, and
  `trusted_base()` is unchanged.

## Stop conditions

- Lowering or the kernel rejects the accepted program: the constant path
  assumes variable indices somewhere else. Stop to the Architect.
- Any kernel or spec change.

## Closeout

Merged `5d5e7bf02` (PR #4436), exact `58fb37c26`: Language QA
`evt_4cc5wqsde428a`, Architect `evt_3s4r31fa66k1p`, Decision
`dec_6taphqt67vkn5`.

- `check_nested_index_variables` runs only when `needs_reverting`. A
  constant-motive nested split at a concrete or repeated index is accepted
  and evaluates.
- The dependent-tail fence pairs stay refused, with the exact clause text
  asserted.
- A nested omission at a concrete index is still an `ExhaustivenessError`.
- `elab.rs` and one test file changed. No kernel or spec change; zero TCB.
