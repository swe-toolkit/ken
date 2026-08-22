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

**`main` = `8d37068bc`.** Tree clean; no publisher running. Watchdog armed
@1800s; the CronCreate daily briefing-flush schedule (`7d029bbf`, 09:37 —
session-only) present.

**THREE LANES (operator, 2026-08-21/22 trial). Roster: `steward/lanes.md` —
that file is the source of truth, not this block.** Lane 1 runtime (finish the
NHC carried-observation chain, then RT-BACKEND-MODULE-SPLIT); lane 2
verify+language (Z3 integration); lane 3 foundation (expressibility trial, the
Architect-burden probe, launched 2026-08-22 anchor `evt_4r550cbd3fvvb`). Doc
track concurrent, contention-free. Finished work still merges; framing an active
lane is lane work.

### Runtime (lane 1) — RT-EXACTINT IN REVIEW; RT-FSREADAT-REPLY removal cut; NHC held on removal

The NHC `cap41_*` blocker chain: each landed fix ADVANCES the rows to the next
distinct blocker (does not green them) until the last lands and D-final runs
all-green. Two blockers merged, third in review, fourth cut.

- RT-DEAD-ARM-EFFECT-LOWERING MERGED (`55c7f51de`, `dec_4p9n9a0b0rfqq`).
- RT-RESOURCE-RELEASE-CARRIED-OBSERVE MERGED (`ef32b6ced`, `dec_3m2p4tmgnpa9t`;
  QA + Architect APPROVE). ResourceScalar-need FAMILY CLOSURE
  (`lower_resource_token_seat`, no Avail change). M8 Adversary hunt SOUNDNESS
  CLEAN (`evt_5wx3bax63yak`).
- RT-EXACTINT-CARRIED-OBSERVE IN REVIEW (active; anchor `evt_47kvrp1esty58`,
  thread `thr_6syere95dng6r`; runtime ring, Opus 5). D0 corrected the frame: it
  is an AVAIL-MOVE, not a new route. Architect ruled (`evt_2kspreq08s3a`) a
  deliberate move of the positioned exact-`Int` seats onto the existing
  `carried_exact_int` EITHER_PHASE classification, decoded by the in-production
  fail-closed-with-validity `narrow_carried_int_u64` — the decoder IS the
  fail-closed consumer, so no route guard (does not contradict route-not-Avail,
  which was need-specific to the ResourceScalar scalar-read). ExactIntU64 closed;
  rows advance. Steward node-scope call (evt to ring): SIX-seat positioned-arm
  unit confirmed — live census FsReadAt Arg(1)/3/4, FsWriteAt 1/3/4 same emitter
  arm inert-but-correct, one decoder `narrow_positioned_int_seat`; FsChangeMode
  Arg(1) dead_arm-trapped; BufferFreeze 1/2 other arm deferred pure wiring. D1
  built, merge gate running, Architect required reviewer; Adversary hunts landed
  code.
- RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL CUT `ready` 2026-08-22. Closing
  ExactIntU64 moved the previously-deferred FsReadAt Arg(2) buffer reply-path
  gate (`effects.rs:3226`) ONTO the critical path (the "off critical path"
  ground of the deferral is invalidated; implementer re-dispositioned
  carry→cut, evt_6vxb4f1rxh3jk). ResourceScalar-family REMOVAL of a vestigial
  gate (span_origin unused; span projected from operand list at 3233), NOT a
  reroute (Architect `evt_2qdpkfvtqrxzy`). D0 = the Architect's (1)-(3)
  classify; depends on RT-EXACTINT (effects.rs contention); released to the
  ring after RT-EXACTINT merges. Also the tracked restoration home for two
  carried-observation-family TEST items (Architect `evt_4wkc748vgfhhf`): AC-4
  the ExactIntU64 runtime-half end-to-end test (observable once this gate is
  gone) and AC-5 a durable positive-cross-key keyed-on-need discriminator for
  the ResourceScalar route (replacing the vanishing-contrast one RT-EXACTINT
  had to drop). Both safe-direction coverage, not soundness holes. Size M.
  Architect required reviewer.
- NHC depends_on re-pointed onto RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL (added
  before RT-EXACTINT merges — gen-progress closed==merged hazard). Held on the
  removal node. Closes when the chain reaches all-green and D-final re-runs (fold
  with preserved slice/fixture + six-axis oracle → closes NHC + PX8-F-CAP-41
  Phase 2). PX8-F-CAP-41 held on NHC.
- Queued carry (NOT released, behind lane-1 indefinitely):
  RT-NATIVE-VOCAB-STRUCTURAL-COMPLETENESS (conjunct-(2) completeness structural;
  Architect req reviewer).
- Lane-1 objective after NHC (operator 2026-08-22): pivot to
  RT-BACKEND-MODULE-SPLIT (currently `draft` — needs framing before startable).

### Foundation (lane 3) — expressibility trial LAUNCHED 2026-08-22

Bounded trial, the Architect-burden probe (operator 2026-08-21). Five
independent CAT WPs authoring verified catalog algorithms against Ken's current
surface; charter `docs/program/wp/foundation-expressibility-trial.md`.

- Launched to the foundation ring (anchor `evt_4r550cbd3fvvb`), simplest-first:
  CAT-SORT (started) → CAT-GCD → CAT-DEQUE → CAT-BSEARCH → CAT-VEC (last,
  fully dependent, highest gap risk). Seat check: implementer gpt-5.6-sol high.
- `gate: none`, no deps, Architect NOT a default reviewer; QA reviews. Stop-on-
  gap: a surface-gap report is the trial's payoff and routes to spec/Architect;
  absent one, these are QA-reviewed expressibility authoring.
- Confirm the foundation seat transitions to Working; anchor the next CAT when
  CAT-SORT lands.

### Language + verify (lane 2) — Z3 integration; V3-FO / LANG in flight

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
