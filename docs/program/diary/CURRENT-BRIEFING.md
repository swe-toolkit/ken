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

**`main` = `569ba3d0d`.** Tree clean; no publisher running. Watchdog re-armed
@1800s; the CronCreate daily briefing-flush schedule (`7d029bbf`, 09:37 —
session-only) survived the compaction, confirmed present.

**ONE LANE — runtime (operator, 2026-08-17; `steward.md` §0).** Lane 2
(language + verify) is retired. Finished work still merges, filings queue behind
the lane; framing for lane 1 is lane work.

### Runtime (lane 1) — RT-DEAD-ARM MERGED; RT-RESOURCE-RELEASE released next

- RT-DEAD-ARM-EFFECT-LOWERING MERGED (`55c7f51de` -> main `569ba3d0d`, Decision
  `dec_4p9n9a0b0rfqq`, Architect APPROVE + differential re-APPROVE, QA passed).
  Corrected two-conjunct deadness predicate ((1) never program-constructed AND
  (2) not runtime-producible via the sealed `NativeProcessSymbols` destructure)
  closed the D1 hard-stop hole; both refusal sites gated on one shared predicate;
  trap single-sourced; ledger keeps `claims` truthful. AC-1 (narrowed) met per
  row: all five governed rows advance to the same live ResourceRelease/
  ResourceScalar blocker. One benign CI-red round (two ken-cli transition
  sentinels the advance moved; Architect censused, implementer repointed;
  gate-gap retro = respin gate is `-p ken-runtime` all-binaries + ken-cli +
  ken-verify).
- RT-RESOURCE-RELEASE-CARRIED-OBSERVE RELEASED to the ring (was `ready`, kicked
  on RT-DEAD-ARM landing). The (A)-family carried-observation route for the live
  ResourceRelease Arg(0)/ResourceScalar refusal on the `lower_buffer_freeze_
  resource_seat` EITHER_PHASE precedent, no Avail widening. D0 grounds the
  ResourceScalar mechanism + any bounded Spec contract question (constant-vs-
  varying first, routed through me as lane-1 input if varying). Architect
  rules the specific mechanism at D0 and is required reviewer.
- CARRY CUT: RT-NATIVE-VOCAB-STRUCTURAL-COMPLETENESS (`draft`, queued, NOT
  released) — make conjunct-(2) completeness structural (route minting sites
  through `NativeProcessSymbols`, or a test asserting every module-level
  constructor const is a field). Fail-closed-sound today; Architect req
  reviewer; queues behind lane-1 indefinitely.
- NHC held on RT-RESOURCE-RELEASE only now; closes when it lands and D-final
  re-runs all-green (folds with the preserved slice/fixture + six-axis oracle
  → closes NHC + PX8-F-CAP-41 Phase 2). The ConstructorTag/FsWriteFile (A)
  instance stays DEFERRED. PX8-F-CAP-41 held on NHC.
- Lane-1 frontier after NHC (§0): RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT
  [merged], the `RT-*` nodes at `ready`, RT-DESCENT-RETIRE's owed `D6a`.

### LANG (retired lane, in-flight work finishing) — transport LANDED (partial)

- Transport landed: squash `93d82a398` (`elab.rs` blob = approved `1afbb4b6`,
  Decision `dec_1f50e3a2pnxj6` APPROVE — stack-plumbing-only respin of the
  CI-red `1b9aa9c7b`). Narrowed AC-7 met. Adversary hunted the landed object:
  CLEAN (`evt_64x1rmjbx4097`; it flagged a wrong range endpoint in the
  lieutenant's M8 handoff and self-corrected).
- LANG-INDEXED node HELD `active`, NOT merged: full admission (kernel_check AND
  SCT-pass) is the operator-gated kernel SCT successor's gate, and
  V3-FO-CHECKER-SOUNDNESS `depends_on` LANG. Do NOT close LANG before the kernel
  successor is cut and V3-FO's `depends_on` re-pointed to it (gen-progress
  `closed`==`merged` clearance hazard). Sequence on authorization: cut kernel
  successor, re-point V3-FO, then close LANG.

### Operator questions OPEN (Pat) — none block lane 1

Q3 now paces TWO threads: it unblocks V3-FO-CHECKER-SOUNDNESS-D3 AND lets
LANG-INDEXED close.

1. Runtime remains lane 1? [proceeding on the default yes]
2. Is language/verify an authorized lane again, or does §0's single-lane posture
   stand? (LANG is only finishing in-flight, already-approved work.)
3. Authorize the operator-gated kernel SCT successor (route A,
   "KERNEL-SCT-TELESCOPE-CANON")? Architect + research fully specified it.
   Controls: arity from the DECLARED Pi telescope (never a deep-lambda
   heuristic), admit==analyze the same eta-long body, MANDATORY nonterminating
   hidden-return-Pi negative control, adversary + conformance, operator-gated
   TCB change.

### Preserved refs this session

- `preserved/steward-work-df470315` — pre-compaction briefing checkpoint.
