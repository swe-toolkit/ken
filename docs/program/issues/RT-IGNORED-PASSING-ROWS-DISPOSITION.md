---
id: RT-IGNORED-PASSING-ROWS-DISPOSITION
title: "Give every ignored row that now PASSES a disposition and act on it -- readmit, register, or relocate-and-delete -- so the ignored set holds only rows ignored for a reason that is still true. A pass is not evidence the defect closed: AC-1 requires a mutation showing each readmitted row goes red when the behaviour it covers is broken, because a closed defect and an assertion that stopped reaching the behaviour produce the same green."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: [CI-IGNORED-SWEEP]
blocks: []
github: null
origin: "Steward, 2026-09-16, on operator directive 2026-09-15: 'Tests that pass should be verified to be functional and desirable and un-ignored... The test to determine that instant equality is wrong is just expensive documentation. That should be noted in code comments and/or spec and/or conformance and not in an ignored test.' Frame at docs/program/wp/RT-IGNORED-PASSING-ROWS-DISPOSITION.md, landed 507bd4bd1, base-fixed at ac08fb581 on Architect evt_4hba5yyec810x. CI-IGNORED-SWEEP (merged) supplies the instrument and its own origin names 'the good-news event this node exists to route, which nothing reported' -- this node consumes that accumulated backlog. Steward-filed per COORDINATION section 2."
---

> ## RELEASED to Team Runtime 2026-09-16 — `ready`, size M, tier T1
>
> **Implementation base is `origin/main`, and that is a ruling, not a
> default.** The frame's measurement was taken at PR #3676's head
> `0f71ab5b9`, which is 36 commits ahead of `main` and 53 behind it. This is a
> distinctly new effort, so it is cut from `main` (operator, 2026-09-16: *"If
> you are starting a distinctly new effort... the base commit needs to be on
> main"*). It has no dependency on #3676 and must not be stacked on it.
>
> **`D0` is re-running the sweep at that base and re-deriving the split
> there.** The population (`34` ignored rows, `-6` registry exemptions, `28`
> selected) carries to `main` and is pinned. The `12 passed / 16 failed`
> split does **not** — it is a property of the tree the sweep ran on. Every
> count in the frame keyed to "the twelve" means *the passing set as measured
> at the implementation base*. An implementer who measures thirteen
> dispositions thirteen; one who measures eleven must not manufacture a
> twelfth.

Read the frame: `docs/program/wp/RT-IGNORED-PASSING-ROWS-DISPOSITION.md`.

## Why this is T1 rather than mechanical

The bookkeeping half — register two pre-classified rows, relocate one
operator-pre-decided row — is mechanical. `AC-1` is not. It requires
perturbing the production path each candidate readmission asserts over and
showing the row goes red, then reverting the perturbation before the diff.
That is a judgment about what the assertion actually reaches, per row, and
the judgment is the deliverable: a row that stays green under the mutation is
**vacuous** and is reported as a finding rather than readmitted.

Nine of the twelve rows carry labels naming a failure that no longer happens.
A stale label and a vacuous test produce the same green, and only the mutation
separates them.

## The recurring instrument hazard on this surface, stated once

Ken's owner-body controls have just supplied a worked example of exactly the
failure `AC-1` guards against. Two of `rt_parity_native`'s eleven owner-body
controls -- `vary-ret` and `raw-worker`, which share the `rt_read_offset_stage`
entry -- were measured applying their mutation **zero** times. The remaining
nine are unmeasured: what covers them is a source read showing the population
precondition is gated on the other entry and cannot fire for any of them, not a
census. Whether the cause is an absent owner-body population or a mutation hook
that is not on the path is still open. The control fails red for a reason
unrelated to the thing it claims to test, which is why it was believed. Do not
read a red as evidence the mutation reached, and do not read a green as
evidence the defect closed. Record the mutation you used with each disposition.

## Scope discipline

`AC-4` (no production diff outside the dispositioned rows) and `AC-3` (no row
outside the passing set changes disposition) are the boundaries. The failing
set belongs to `RT-IGNORED-FAILING-ROWS-INVENTORY` and is out of scope here.

`crates/ken-verify/src/scenario.rs` is the one file outside Runtime's usual
surface; check for an open Verify candidate against it before starting.

## Not in scope, and deliberately

Making passing-while-ignored **blocking** rather than `findings non-blocking`
is the anti-recurrence half. It is with the operator and is not an AC here:
turning the gate red before the backlog is dispositioned would red CI on the
known rows.
