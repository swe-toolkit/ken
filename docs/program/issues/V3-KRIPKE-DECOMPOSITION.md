---
id: V3-KRIPKE-DECOMPOSITION
title: "The FO Kripke embedding is the DAG's V3 headline and has never had a tracker node -- only V3-RESIDUAL and V4-RESIDUAL exist, both merged, and what they produced is the single Int-literal refutation arm; establish what the embedding requires and how it decomposes into one-hour increments, because an L-sized node cannot be released and the adequacy lemma is kernel-facing rather than prover-facing"
status: draft
owner: verify
size: M
gate: none
depends_on: [V3-VERDICT-CENSUS, SEC1-R3-MINIMAL-ROUTE]
blocks: []
github: null
origin: Steward measurement 2026-08-13 -- attempt_fo (prover.rs:332) calls attempt_ipc unchanged and its own doc marks the translation, the World sort, the adequacy lemma and check_cert soundness as [placeholder - reifies in V4]. The DAG names V3 at 05-implementation-dag.md:166 and no V3 node exists. Operator directed this lane 2026-08-13.
---

## Why this is `draft` and what unblocks it

**Both predecessors are report-only and both are `ready`.** This node's shape
depends on their answers:

- `V3-VERDICT-CENSUS` says whether FO goals are a meaningful share of what the
  corpus cannot close, or a small tail behind case analysis and quantifier
  instantiation. **If FO is the tail, this node is mis-prioritized and the
  Steward re-sequences rather than releasing it.**
- `SEC1-R3-MINIMAL-ROUTE` says whether `AC-R3c` needs the embedding at all.

**Do not release this before both land.** Framing it now is `§4e` — a written
successor ahead of the frontier — not an instruction to start.

## What it is

**A decomposition report.** The deliverable is a set of one-hour-sized
increments with their order and their fixed inputs, not the embedding.

`attempt_fo` (`crates/ken-elaborator/src/prover.rs:332`) calls `attempt_ipc`
unchanged. Its doc names four deferred pieces: the translation `φ ↦ φ#`, the
`World` sort with its preorder and monotone forcing predicate, the
embedding-adequacy lemma `classically_valid(φ#) → φ`, and the soundness of a
deep-embedded `check_cert`.

## The part that is not the prover's

**The adequacy lemma is kernel-facing.** Spec `23 §4` route (a) requires it
mechanized *once and in the kernel*, alongside `check_cert` soundness. That is
what makes a positive solver result dischargeable by computation rather than by
trust — and it is a claim about what the kernel proves, not about search.

⇒ **Any decomposition that treats the adequacy lemma as prover work has
mis-assigned the hardest piece.** Whether it lands, and where, is an Architect
and spec-enclave question that this report surfaces rather than settles.

## Not this node

- **Building any of it**, including the translation, which looks like the easy
  first piece and is the one whose shape the adequacy lemma constrains.
- Deciding whether V3 proceeds. That is priority, and it is the operator's.
- Ruling on where the adequacy lemma lives.
- Anything about the solver. The embedding is what makes a solver *usable*
  soundly; it is not the solver, and it is deferred separately.
