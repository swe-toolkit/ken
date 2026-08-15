---
id: RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE
title: "Measure whether any lowered consumer reads the static-worker field constructed for row4 depth 1 and row5 after-hole, or whether the recognition is only ever seen by the ledger's close"
status: closed
owner: runtime
size: S
gate: none
depends_on: [RT-EMITTED-WORKER-CONSUMER-WALK]
blocks: [RT-MINT-SITE-STATIC-DISCRIMINATOR]
github: null
origin: "Architect ruling evt_3cxm6654d5cjb, 2026-08-15, which split the RT-EMITTED-WORKER-CONSUMER-WALK D0 population and explicitly declined to rule on this half. Every symbol below was located by name against origin/main ac8a73d1b by the Steward before filing. Steward-filed per COORDINATION section 2."
---

## CLOSED, NOT MERGED — measurement-only, and `D0` IS DELIVERED

**This node produced no candidate and never will**, so it can never reach
`merged`; a successor gated on that would wait forever. **`closed` =
resolved-without-landing.**

**`D0` result, measured at `30ee4dbf1`: `not needed`, both rows.**
row4-depth-1 (recognition 0, field 25, owner 26) and row5-after-hole
(recognition 0, field 21, owner 22) reach their `StaticWorkerFieldLedger::close`
refusal **with no lowered field-reader tag. The field values are never read.**

**The zero is DISCRIMINATING and the ring proved it** — a synthetic
ledger-minted positive control reached `specialized_at`, and test-only
observation readers were covered alongside the lowered ones. **Zero means no
reader, not a blind instrument.** Row1 and depth-2/3 were expressly excluded.

**The fork did NOT resolve to a repair.** The Architect ruled at
`evt_3czp0t9gnnz61` that the Steward's two options *"are not the two branches"*:
they differ in the **scope** of the erasure, not the standard of evidence, and
the deciding fact is whether the measured population is **statically
identifiable at the mint**. ⇒ [[RT-MINT-SITE-STATIC-DISCRIMINATOR]].

> **"Measure more rows" closes nothing and must not be framed.** The gap is not
> that two rows are too few — **a runtime observation over occurrences is the
> wrong KIND of fact for a static site.**

## The Architect declined to rule here, and the reason is the node

> **FOR DEPTH-1 AND AFTER-HOLE: I AM NOT RULING, AND I WILL NOT GUESS.**
> These have no emission and no consumption, so there is **no positive authority
> for erasure** and my old sentence still binds.

**Erasure is lawful only under positive authority at or before construction.**
For row4-depth-2 and row4-depth-3 that authority exists — a measured
consumption at `SourceContinuation::CallArgument` — and
[[RT-SECOND-RECOGNITION-ERASURE]] is ruled on it. **These two rows have no
emission and no consumption, so nothing authorizes erasing their field, and
nothing yet shows their field is needed either.**

⇒ **The open question is prior to "which repair".**

## The question, and it is answerable by the instrument already built

**Is the worker field these two construct ever needed at runtime, or is the
recognition an artifact of the template's pattern?**

| answer | repair, and it is NOT this node's to make |
|---|---|
| **needed** | transition — a real `rebind` at a static elimination that is not an exact-`Var` call, **minted per recognition so the bijection stays agreeing** |
| **not needed** | erasure — and the authority must be **established** at or before construction, never borrowed from another row's consumption |

**Both are live. This node picks neither.** It produces the measurement that
does.

## Where the recognition comes from, verified rather than inherited

`static_worker_constructor_template` (`core.rs:15545`) recognizes these two, and
its mint at **`core.rs:15579` is the sole production `recognize` call site in
the tree** at `origin/main` `ac8a73d1b` — every other hit is in
`core/tests/control.rs`. The sole production `rebind` is `mod.rs:4958`, inside
`constructor_field_bindings` (`mod.rs:4936`), which is **the transport event and
not a mint.**

**Verify both before relying on either.** This node's predecessor exists because
a cited coordinate was read past its warrant twice, and the ruling that produced
this node attributed the mint to the transport site.

## Deliverables

**`D0` — for row4-depth-1 and row5-after-hole, does any lowered consumer read
the constructed field's value, or is the recognition minted and then seen only
by `StaticWorkerFieldLedger::close`?** Tag the readers, run the exact two-row
control, report per row.

**`D1` — report by SYMBOL, not by line.** `core.rs` and `mod.rs` move under
every neighbouring merge. Name the function and the construct. **The ruling that
produced this node cited a function that does not perform the operation
attributed to it**; a symbol would have carried the error and a line did not.

**`D2` — the disposition**, stated as the fork above resolves: needed, not
needed, or **not determined by this instrument** — which is a legitimate result
and must be reported as one rather than rounded to either arm.

## Acceptance criteria

**`AC-1`.** Both rows reported **individually.** An aggregate hides the
interesting case, which is one row disagreeing with the other — and this
population has already split once under measurement.

**`AC-2`.** **Report which rows were observed and which were not.** The finding
that produced this node is that a limit stated on one axis of a claim gets read
as universal on the other. **A row the instrument could not drive is a reported
result, not a dropped row.**

**`AC-3`.** No production logic change. This node instruments and reports.
Probes reverted before handback, `git diff --stat` shown clean.

**`AC-4`.** **No repair is proposed.** The fork is the Architect's and he
declined it pending exactly this measurement. **A `D0` that arrives with a
preferred arm attached is the failure this node's predecessor was filed to
correct.**

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Proposing or implementing either arm of the fork.** `AC-4`.
- **Row4-depth-2 and row4-depth-3**, which are ruled and belong to
  [[RT-SECOND-RECOGNITION-ERASURE]]. **Do not let a shared mint site pull this
  node into that repair** — they share `core.rs:15579` and they do not share a
  disposition.
- **Row1-owned-scope**, which refuses at `NativeJoinPlanV1` and is neither
  node's subject.
- **Relaxing `close`.** Whatever the answer, the ledger's law is not the defect.
- **`D2k-1c`**, the planner-owned `ContinuationTemplate` population, and the
  continuation-source surface.

## Sequencing

**Independent of [[RT-SECOND-RECOGNITION-ERASURE]] and concurrent with it.** The
Architect was explicit that the depth-2/3 half must not wait on this answer.
Both sit on the critical path to [[RT-RECURSOR-TRANSPORT]] `D3` and
[[RT-DESCENT-RETIRE]].

## On the impossibility claim, which is still open for these two rows

`close`'s refusal says a constructor carrying an unconsumed static worker
*"has no runtime representation."* For depth-2/3 that is **false as applied** —
the worker is consumed — and [[RT-SECOND-RECOGNITION-ERASURE]] `D2` corrects it.

**For these two rows it remains an open question rather than a known falsehood.**
This node's `D0` is what decides it. **It is the third impossibility claim in
this campaign's history and the first two did not survive contact**, so it is
recorded here as a claim under test, not as a premise.
