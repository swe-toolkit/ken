---
id: V3-KRIPKE-DECOMPOSITION
title: "The FO Kripke embedding is the DAG's V3 headline and has never had a tracker node -- only V3-RESIDUAL and V4-RESIDUAL exist, both merged, and what they produced is the single Int-literal refutation arm; establish what the embedding requires and how it decomposes into one-hour increments, because an L-sized node cannot be released and the adequacy lemma is kernel-facing rather than prover-facing"
status: merged
owner: verify
size: M
gate: none
depends_on: [V3-VERDICT-CENSUS, SEC1-R3-MINIMAL-ROUTE]
blocks: []
github: null
origin: Steward measurement 2026-08-13 -- attempt_fo (prover.rs:332) calls attempt_ipc unchanged and its own doc marks the translation, the World sort, the adequacy lemma and check_cert soundness as [placeholder - reifies in V4]. The DAG names V3 at 05-implementation-dag.md:166 and no V3 node exists. Operator directed this lane 2026-08-13.
---

> # BOTH PREDECESSORS LANDED 2026-08-13. THE PRE-STATED TEST FIRED, AND THE TWO
> # ANSWERS POINT IN DIFFERENT DIRECTIONS. STAYS `draft`, ROUTED TO THE OPERATOR.
>
> `V3-VERDICT-CENSUS` merged (#2120) and `SEC1-R3-MINIMAL-ROUTE` merged
> (#2124). `depends_on` is satisfied. **The node is not released, because the
> condition written below decides that and it came back negative.**
>
> **Test as written: *"If FO is the tail, this node is mis-prioritized and the
> Steward re-sequences rather than releasing it."*** The census measured the FO
> route at **16 Proved / 0 Disproved / 1 Unknown**. **FO is the tail.** Twenty
> of the twenty-two unclosed obligations are D-route closed bare `Const`
> atoms; the FO nested-`Pi` excluded-middle goal is **one** hole, and the HO
> free-variable equality is one more.
>
> ⇒ **Measured by corpus impact, the embedding is worth one hole out of
> twenty-two.** That refutes the prioritization this node was framed on —
> including the Steward's own earlier statement that the embedding was the
> headline blocker.
>
> **But the second predecessor answers the other way, and this is the fork.**
> `SEC1-R3-MINIMAL-ROUTE` reports that the specified automated route for
> `AC-R3c` needs **the Kripke embedding plus a checked-certificate path**, with
> solver search only in combination. So the embedding is *not* optional for
> Sec1's by-proof half, however small its corpus share.
>
> | if the goal is | then the next work is |
> |---|---|
> | reduce assumed obligations in the corpus | the **D fragment** — twenty closed atoms whose route runs through registering decidable-equality certificates, **which costs two irreducible trusted-base postulates per registrant** (`check.rs:1253`, `:1302`, `:1308`). An operator TCB decision, not a build call. |
> | unblock `Sec1`'s by-proof half | **this node.** One hole in the corpus, and a hard requirement for `AC-R3c`. |
>
> **Neither is derivable from the roadmap, and one of them grows the TCB. Both
> are operator calls under the Steward playbook §3**, so the node was held at
> `draft` with its frame written and shovel-ready rather than released on a
> Steward guess.
>
> ### RELEASED 2026-08-13 ANYWAY, AND THE REASON IS NOT THAT THE FORK CLOSED
>
> **The fork above is still open and this node does not touch it.** What changed
> is that the fork was noticed to be a comparison with **one side unpriced**.
>
> | option | its cost, as the operator would have to weigh it |
> |---|---|
> | the D fragment | **priced** — two irreducible trusted-base postulates per registrant, `check.rs:1253`/`:1302`/`:1308`, times twenty closed atoms |
> | the Kripke embedding | **unpriced** — nobody has said what it requires or how long it takes |
>
> **This node is the price of the second row.** Its deliverable is a
> decomposition report: the pieces, their real dependencies, their lane
> assignment, and a cut into one-hour increments. That output is an **input to
> the operator's decision, not a consequence of it** — it is worth the same
> whichever way the fork goes, and it is worth most before the fork is answered.
>
> **The pre-stated test above still fired, and I am not pretending otherwise.**
> It read: *"if FO is the tail, this node is mis-prioritized and the Steward
> re-sequences rather than releasing it."* FO is the tail — one hole in
> twenty-two. What the test assumed is that **corpus share is the ranking
> criterion**, and `SEC1-R3-MINIMAL-ROUTE` then established that `AC-R3c`
> requires the embedding regardless of its corpus share. A criterion that a
> later measurement showed to be insufficient does not get to decide the
> question on its own.
>
> ⇒ **Releasing a report is sequencing, which is the Steward's under §3.
> Choosing which fragment gets built is priority, which is not.** Verify was
> idle with both its nodes held behind an unanswered fork, and §1 makes an idle
> team the Steward's backlog.
>
> **THIS AUTHORIZES NO BUILD.** The banned scope in the frame is unchanged and
> is the operative constraint: not the translation, not the `World` sort, not
> the adequacy lemma. A returned decomposition is a price tag. **Nobody should
> read it, or this node's `ready` status, as a decision that V3 proceeds.**

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
