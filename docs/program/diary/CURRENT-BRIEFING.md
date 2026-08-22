# Current briefing (live — read this first on every Steward resume)

> ## HOW TO READ THIS FILE, AND WHEN TO DISTRUST IT
>
> **`origin/main` outranks this file, always.** If anything below tells you to
> do something `git fetch origin` shows as landed, **this file is stale and the
> repository is right.** Re-read fresh, in this order:
>
> 1. `git fetch origin && git rev-parse origin/main`
> 2. the LIVE block below — **only** the LIVE block
> 3. the open tasks (do not re-derive priority from memory)
> 4. for what is HELD, DEFERRED, or WHOSE it is: **the node**
>    (`docs/program/issues/*.md`), its operative block — never this file
>
> **This file is a resume POINTER, not an archive. Git is the archive.** When a
> window closes its block is **deleted**, not demoted to a "superseded" section —
> a superseded block left in the file gets read by someone, eventually.
>
> ### THE THREE FILES, so you do not go looking in the wrong one
>
> | you want | read |
> |---|---|
> | the current window — **only** the live block lives here | this file |
> | permanent, undated material: operator rulings, preserved refs, standing traps | [`STANDING.md`](STANDING.md) |
> | what happened on day X | `2026/Mon/DD.md`, indexed by [`INDEX.md`](INDEX.md) |
>
> **This file holds ONE block: the current one. Under 250 lines.** A superseded
> block moves to the dated diary **even if it is an hour old** — "recent" is not
> the test, "current" is. Flushed daily by a delegated subagent; procedure in
> `agent/playbooks/federation/steward/briefing-flush.md`.
>
> ⚠ **It reached 4648 lines / 273 KB across 19 unflushed days before anyone
> noticed** — having already been rewritten to be small once, in July. Nothing
> reds when it grows. If you are adding a block and the file is over budget,
> flush first.

> ### PRE-2026-07-26 CONTENT IS AT BLOB `c26ee67f`
>
> ~2700 lines of windows back to 2026-07-21, archived here on 2026-07-26 --
> `git show c26ee67f`. **The rewrite audit was a SCAN, not exhaustive**: headings,
> authoritative-looking blocks, then a sweep for sole-source markers, decision
> ids, held items and preserved refs. A reader needing something from before that
> date should assume it is in the blob, not that it was considered. (The scan is
> what found two self-declared-authoritative blocks that were wrong, and a
> hand-maintained list of 6 preserved refs when origin held 26.)

## LIVE — 2026-08-22

**`main` = `75b573c1d`.** Tree clean; no publisher running (PR #2735 landed
doc-only). Re-arm on resume: the watchdog interval (fired this session, armed)
and the CronCreate daily briefing-flush schedule (session-only, auto-expires —
re-arm it).

**ONE LANE — runtime (operator, 2026-08-17; `steward.md` §0).** Lane 2
(language + verify) is retired. Finished work still merges, filings queue
behind the lane; framing for lane 1 is lane work.

### Runtime (lane 1) — NATIVE-HANDLE-CARRIER released, ring kicked

- NATIVE-HANDLE-CARRIER recut to `ready` and released (kick anchor
  `evt_7cg8h0tmpfwck`, landed via PR #2735). All 7 deps merged (last two:
  RT-RECURSIVE-POSITION-ARM-ARITY, RT-BRANCH-LOCAL-DECLARED-CALLABLE).
  Deliverable D-final: re-run the four `cap41_*` rows + the `AC-5` row on
  current main, report per-row. Disposition: all-green → fold with preserved
  slice/fixture + Architect six-axis oracle → close NHC + PX8-F-CAP-41 Phase
  2; any red → report refusal + call site and cut the next successor. Awaiting
  the runtime-implementer's per-row result; confirm the seat woke (leader ack
  in the anchor thread).
- Lane-1 frontier order after NHC (§0): RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT
  [merged], the `RT-*` nodes already at `ready`, and RT-DESCENT-RETIRE's owed
  `D6a`.

### LANG (retired lane, finishing in-flight approved work) — transport CI-red

- LANG-INDEXED-RECURSIVE-IH-DISCHARGE c-elab transport: Decision
  dec_5refwakaj0t4w APPROVE on 1b9aa9c7b, but PR #2734 CI-RED (build + test +
  all four shards). Publisher stopped clean; branch intact at 1b9aa9c7b,
  nothing landed. Routed to the language ring: implementer diagnosing
  scoped-feature / downstream repro (never `--workspace`), fixing the
  build-only delta and respinning a fresh SHA. On handback: differential
  re-review (confirm build/feature-only, no transport-semantics change), fresh
  merge Decision on the new SHA, then hand the resolved Decision to the
  lieutenant.
- Architect ruled the SCT wall is a SEPARATE, operator-gated KERNEL successor
  (route A: `sct.rs` type-directed telescope-canonicalization). AC-7 NARROWED
  to "held D3 bodies elaborate + pass kernel_check" (green today); full
  admission (kernel_check AND SCT-pass, which unblocks
  V3-FO-CHECKER-SOUNDNESS-D3) moves to the kernel successor.

### Operator questions OPEN (Pat) — none block lane 1

1. Runtime remains lane 1? [proceeding on the default yes]
2. Is language/verify an authorized lane again, or does §0's single-lane
   posture stand? (LANG is only finishing in-flight, already-approved work.)
3. Authorize the operator-gated kernel SCT successor (route A,
   "KERNEL-SCT-TELESCOPE-CANON")? Architect + research have fully specified
   it; it is the only thing that admits the LANG D3 group. Controls: arity
   from the DECLARED Pi telescope (type-directed, never a deep-lambda
   heuristic), admit==analyze the same eta-long body, MANDATORY nonterminating
   hidden-return-Pi negative control, adversary + conformance review,
   operator-gated TCB change.

### Preserved refs this session

- `preserved/steward-work-df470315` — pre-compaction briefing checkpoint
  (df4703153).
