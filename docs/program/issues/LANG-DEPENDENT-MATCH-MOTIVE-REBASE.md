---
id: LANG-DEPENDENT-MATCH-MOTIVE-REBASE
title: "D2b elaborator-only predecessor (the inversion idiom): generic simultaneous rebasing of the dependent-match motive, the constructor expected goal, and the direct recursive IH so that BOTH the actual indices and the outer scrutinee become local indices/value. Architect ruled NO source-level escape for the FokDerivation inversion. Elaborator-only — NO kernel/TCB change; if implementation reveals a kernel/TCB touch is required (as the prior predecessor LANG-RECORD-INDEX-REFINEMENT did), STOP and route to the Steward -> operator, do NOT make a silent kernel change."
status: merged
owner: language
size: M
gate: lang-qa+architect
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: https://github.com/swe-toolkit/ken/pull/3224
origin: "Steward, 2026-09-01. Second D2b predecessor. The first (LANG-RECORD-INDEX-REFINEMENT, kernel eq_at_inductive weaken) MERGED and cleared the record-index FokDerivation MATCH elaboration; D2b then hit a distinct blocker — the derivation INVERSION idiom (relating constructor index fields to gamma/[goal] failed across 3 approaches; language-implementer escalation evt_5m6qvgtfbfr81). The Architect ruled there is NO source-level escape (D2b-inversion ruling, thread thr_5s7mbap0049jy; language-leader to pin the exact evt in the kickoff ack), and directed a fresh elaborator-only predecessor. Request: language-leader evt_4j7hayna7t22e. D2b's Bottom/Atom/Or WIP stays held evidence only at fbfd6b6c1; D2b (V3-FO-EMBEDDING-ADEQUACY) resumes ONLY after this predecessor lands and the consumer probe is green. FO remains Unknown until then."
---

## Objective (Architect-ruled mechanism)

Implement, in the elaborator's dependent-match machinery, a **generic
simultaneous rebasing** of three things at once:

1. the dependent-match **motive**,
2. the **constructor expected goal**, and
3. the **direct recursive IH (induction hypothesis)**,

so that **both the actual indices and the outer scrutinee become local
indices/value**. This is the general inversion idiom — not a per-family or
per-constructor special case. The Architect ruled there is no source-level escape
(the idiom cannot be expressed in Ken source against the landed eliminator), so
the fix is in the elaborator.

## Hard constraints

- **No kernel/TCB change.** This is the elaborator's motive/goal/IH rebasing, not
  a kernel completeness or soundness edit. If, on implementation, the rebasing is
  found to REQUIRE a kernel/TCB touch (the exact way the first predecessor's
  elaborator hypothesis was falsified into the `eq_at_inductive` weaken), that is
  a HARD STOP: reset clean, post the measurement, and route to the Steward — the
  Steward routes any kernel/TCB touch to the operator. Do NOT make a silent
  kernel change under an elaborator-only frame.
- **No FokFin special case** — the rebasing must be generic over the recursive
  family, not keyed to `FokFin`/`FokDerivation`.
- **No reindexing** of the relation, **no axiom**, **no `Option` workaround.**
- D2b's Bottom/Atom/Or WIP (`fbfd6b6c1`) stays **held evidence only** — it is not
  a candidate and is not consumed by this predecessor.

## Required pins (acceptance)

- A **minimal flat Nat-indexed recursive family** with an
  **index-and-value-dependent Omega motive** — the smallest witness that
  exercises simultaneous rebasing of index and scrutinee.
- **Positive controls:** no-match, constant, and non-refining — each must
  elaborate/behave correctly (the rebasing must not over-fire on cases that do
  not need it).
- **Independent one-site deletion mutations for all three producers** (motive,
  expected goal, recursive IH): deleting each producer's rebasing site
  independently must RED for that producer's own claim, then restore exactly.
- **Targeted dependent-match suites** covering the idiom.

## Gate and sequencing

Candidate reviewed by **language QA + Architect** on the exact SHA (the Architect
ruled the mechanism; the review confirms the implementation against it, and the
motive/IH machinery is soundness-adjacent). Steward routes (M1-M4), lieutenant
executes (M5-M9). **D2b (V3-FO-EMBEDDING-ADEQUACY) resumes ONLY after this
predecessor lands and its consumer probe is green.** FO remains `Unknown` until
D2b itself lands.

## Contention

Language lane (lane 2). Touches the elaborator's dependent-match/motive
machinery. No `crates/ken-kernel` change (that is the hard constraint above); no
`/spec` change; no overlap with the runtime lane's composed-return SSA work or
the doc track. Base is current `main` at release.
