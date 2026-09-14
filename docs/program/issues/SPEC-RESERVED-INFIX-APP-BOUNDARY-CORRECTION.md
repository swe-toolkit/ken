---
id: SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION
title: "Re-scope the application-atom contraction rows in seed-reserved-infix-names.md so they no longer gate LANG-RESERVED-INFIX-NAMES (A0). The seed pins a bare-operator-value grouping requirement and a projection-outside-application contraction that (1) exceed A0's Architect-decomposed unchanged-syntax scope and (2) regress the priority-queue catalog control. D0: Architect rules WITHDRAW vs RELOCATE. Spec-author authors, CV validates, spec-leader Decision, Steward M1-M4."
status: ready
owner: spec
size: S
gate: none
depends_on: []
blocks: [LANG-RESERVED-INFIX-NAMES]
github: null
tier: T1
origin: "Steward cut 2026-09-14 on the spec-leader's request (evt_1atgv5dm04ew4) and behavioral ruling (evt_234t0amx3ht7x). Language grounded (language-implementer evt_7x7th49xa2xv3) that the seed's application-atom contraction regresses the priority-queue catalog control priority_queue_actual_export_table_is_exactly_the_six_name_api; the clean bounded A0 checkpoint 2c63d409 omits the contraction and passes 47 focused tests + 10 mutation seams. The contraction also contradicts the Architect's A0 decomposition (evt_784ge2nq65dfy, 'unchanged syntax'). SPEC-RESERVED-INFIX-NAMES already merged (7dea59366), so this is a forward spec correction, not a withheld merge. Re-measure the exact seed rows at the cut."
---

# Objective

Correct `conformance/surface/operators/seed-reserved-infix-names.md` so its
application-atom boundary contraction no longer gates
[[LANG-RESERVED-INFIX-NAMES]] (A0). A0's released scope is bounded reserved-name
admission with **unchanged syntax** (Architect A0 decomposition
`evt_784ge2nq65dfy`); the seed over-reached into general application/projection
grammar, which both exceeds that scope and regresses an existing catalog control.

# The problem (grounded)

The merged seed marks two row groups **RED-UNTIL-LANG-RESERVED-INFIX-NAMES**,
i.e. it demands A0 make them go green:

- `surface/operators/bare-operator-value-requires-grouping` — a bare operator
  value must newly require grouping `(OP)`; an ungrouped zero-application value
  must reject.
- the **non-temporal** rows of
  `surface/operators/identifier-headed-non-atom-arguments-require-grouping` —
  ungrouped non-atom arguments must reject, and the projection row contracts
  `keep box.value` to `Proj(A(keep, box), value)` (projection **outside** the
  application) rather than the grouped `A(keep, Proj(box, value))`.

Both change general application/projection grammar. The language ring implemented
the seed interpretation and the focused catalog control
`modules::namespace_effect_tests::priority_queue_actual_export_table_is_exactly_the_six_name_api`
passed 1/1 on the base parser and **failed candidate-only** with
`TypeMismatch ... projection base's type is not a named-field owner`
(span `27523..27542`); restoring legacy application/projection parsing returned
it to 1/1. Existing catalog source relies on ungrouped projected arguments, so
satisfying the seed row would require out-of-scope catalog source migration.

No live regression on `main`: these are RED-UNTIL rows (expected-red, gated on
the future language node), and the catalog control stays green because the clean
A0 checkpoint `2c63d409` omits the contraction.

The **temporal** rows of the same table carry
**RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE** (`OQ-syntax`) — a separate gate,
unrelated to A0. Do not touch them.

# D0 — Architect design fork (resolve first)

The general application/projection grammar is the Architect's design surface, and
they own the reserved-infix decomposition. The Architect rules the disposition:

- **WITHDRAW** — the application-atom contraction is not part of the
  reserved-infix program (an over-reach vs. the operator's glyph-lexing objective
  and the A0 unchanged-syntax decomposition, and it regresses the catalog). The
  seed rows are corrected to the base-parser behavior (accept `keep box.value` as
  it parses today) and lose the RED-UNTIL-LANG-RESERVED-INFIX-NAMES tag.
- **RELOCATE** — the contraction is a deferred goal that belongs to a scoped
  later node which also owns the catalog migration. The rows keep a RED-UNTIL tag
  pointing at that named future node, not at A0.

Either disposition takes the rows off A0's gate. The seed edit below follows the
ruling.

# Deliverable

- `seed-reserved-infix-names.md` edited per the D0 ruling so the two row groups
  above no longer gate `LANG-RESERVED-INFIX-NAMES`, preserving every other row
  (the six-name admission rows, the notation-identity rows, and the temporal
  rows) exactly.
- A one-line note in the seed (or its companion prose) recording the disposition
  and its grounding, so the change is auditable.

# Acceptance criteria

- The A0-conformance meaning of the seed is exactly: bounded reserved-name
  admission with unchanged application/projection grammar. The clean A0
  checkpoint `2c63d409` conforms to the corrected seed with no application-grammar
  change and no catalog migration.
- The bare-operator-value grouping requirement and the non-temporal projection
  contraction no longer carry RED-UNTIL-LANG-RESERVED-INFIX-NAMES.
- The temporal rows (RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE) and every non-contraction
  row are byte-unchanged.
- The priority-queue catalog control stays green with the base
  application/projection parser (no catalog source migration is implied by A0).
- Disposition (withdraw vs relocate) matches the Architect's D0 ruling; a relocate
  names a real future node id.

# Not this node

- Implementing A0 itself ([[LANG-RESERVED-INFIX-NAMES]], already released;
  checkpoint `2c63d409` held pending this correction).
- Any application/projection grammar change or catalog source migration — if the
  Architect rules RELOCATE, that is the future node's scope, not this one.
- The temporal-expression surface (`RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE`).

# Sizing / tier

**Size S, tier T1.** The seed edit is small, but the disposition turns on a
design fork (Architect) and the correction must preserve every other seed row and
not silently re-introduce a grammar change. Architect required for the D0;
CV validates the corrected seed.

# Contention

Spec enclave, `conformance/surface/operators/seed-reserved-infix-names.md` only.
No `crates/` change — docs/spec content. No cross-lane contention. This UNBLOCKS
the L2 language lane's held A0 candidate; it is the precursor to A0's release.
