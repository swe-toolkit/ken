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

**`main` = `ef32b6ced`.** Tree clean; no publisher running. Watchdog armed
@1800s; the CronCreate daily briefing-flush schedule (`7d029bbf`, 09:37 —
session-only) present.

**ONE LANE — runtime (operator, 2026-08-17; `steward.md` §0).** Lane 2
(language + verify) is retired. Finished work still merges, filings queue behind
the lane; framing for lane 1 is lane work.

### Runtime (lane 1) — RT-DEAD-ARM + RT-RESOURCE-RELEASE MERGED; RT-EXACTINT cut

The NHC `cap41_*` blocker chain: each landed fix ADVANCES the rows to the next
distinct blocker (does not green them) until the last lands and D-final runs
all-green.

- RT-DEAD-ARM-EFFECT-LOWERING MERGED (`55c7f51de`, Decision `dec_4p9n9a0b0rfqq`).
  Two-conjunct deadness predicate traps whole-program-dead FSOp arms.
- RT-RESOURCE-RELEASE-CARRIED-OBSERVE MERGED (`ef32b6ced`, Decision
  `dec_3m2p4tmgnpa9t`; QA `evt_6e1kf4tdghchs` + Architect required-reviewer
  APPROVE `evt_24nyqqhs5fy1f`, all 9 soundness properties by direct read). The
  carried-observation FAMILY CLOSURE over the ResourceScalar need (Steward ruling
  `evt_5xq3hw23kamrd`, `lower_resource_token_seat`, no Avail change). AC-1: all
  three ResourceScalar seats advance; rows TERMINATE at the ExactIntU64 sibling.
  M8 Adversary hunt on the landed route in flight.
- RT-EXACTINT-CARRIED-OBSERVE cut (`ready`) — the ExactIntU64-need carried-
  observation closure for FsReadAt Arg(1), the measured terminal. Likely a pure
  wiring to the existing `carried_exact_int` EITHER_PHASE route (BufferAllocate 0
  already reads through it), so S/T2-leaning; D0 confirms. Architect required
  reviewer. KICK HELD until the RT-RESOURCE-RELEASE M8 hunt reports clean
  (effects.rs contention with any fold; single ring). NHC held on this node only.
- NHC depends_on now names RT-EXACTINT; closes when the chain reaches all-green
  and D-final re-runs (fold with preserved slice/fixture + six-axis oracle →
  closes NHC + PX8-F-CAP-41 Phase 2). PX8-F-CAP-41 held on NHC.
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
