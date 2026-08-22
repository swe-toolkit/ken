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

**`main` = `633064e45`.** Tree clean; `steward/work` == `origin/main`. Publisher
lane: one orphan open PR (#2692, stale older RT-CONTROL node version) routed to
the lieutenant to CLOSE not merge; integrator's befc2dc4/#365 watchdog ref
answered as stale (July auto-merge tooling). Watchdog is one convo agent-interval
(1800s, server-side) — re-verify armed at each resume.

**THREE LANES (operator, 2026-08-21/22 trial). Roster: `steward/lanes.md` —
that file is the source of truth, not this block.** Lane 1 runtime (finish the
NHC carried-observation chain, then RT-BACKEND-MODULE-SPLIT); lane 2
verify+language (Z3 integration); lane 3 foundation (expressibility trial, the
Architect-burden probe). Doc track concurrent, contention-free. Finished work
still merges; framing an active lane is lane work.

### Runtime (lane 1) — HS=3 discharged; enumeration-first refusal-set bounding

The NHC `cap41_*` chain. RT-FSREADAT projection + RT-DEAD-ARM-JOIN-DISPOSITION
are BUILT+measured-sound, CO-LANDING on `wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`
(thr_3j5ew8rhy35nh); neither greens alone. Merged predecessors:
RT-DEAD-ARM-EFFECT-LOWERING (`55c7f51de`), RT-RESOURCE-RELEASE-CARRIED-OBSERVE
(`ef32b6ced`), RT-EXACTINT-CARRIED-OBSERVE (`d49a232a`, closed).

- HS=3 §1a DISCHARGED. Architect layer-3 ruling (evt_r3tt1gpv4tkn) + IH-marker
  addendum (evt_1a8tf8776fd6m) on the research advisory (evt_5f0rzjghjhmy9 +
  evt_3p0rwsjw51mjq): both validators are correct fail-closed boundaries — do NOT
  weaken; fixes are producer-side; bound the COMPLETE set before sequencing.
- ENUMERATION-FIRST. RT-COLD-LOWERING-PATH-ENUMERATION landed (#2766) and
  DELIVERED AC-3 (Architect verified done, evt_4ag90qfacmgwy): FIVE mechanisms
  over RT_PARITY_SOURCE, not two — 288+301 materialized-dead join (a CLASS),
  BoundaryCarrier (layer-1 family), closure-boundary, one non-elaborating
  TypeMismatch, and the IH-marker with ZERO entries here.
- BINDING FINDING → SIBLING node. The IH-marker lives in a SECOND witness family
  (checked-program source shared by the four cap41_* terminal pins), on
  RT-FSREADAT's AC-4/AC-5 path, never enumerated. Steward scope call: cut the
  sibling RT-COLD-LOWERING-CHECKED-FAMILY-ENUMERATION (landed `633064e45`, blob
  `762b0ac3`), released to the ring (evt_7sfny7j9sab0m). Same exhaustive
  discipline; a 2nd population may hide a 6th mechanism (collect all, do not
  stop at the IH-marker).
- SUCCESSOR SEQUENCE HELD until BOTH enumerations report. The per-gap successors
  are cut from report-1 ∪ report-2, not before (full ledger recorded in the
  sibling node: 288+301 reconciliation class; BoundaryCarrier layer-1 fold;
  closure-boundary → existing RT-CLOSURE-BOUNDARY-LANE; TypeMismatch explicit
  disposition; IH-marker producer-fix). HS stays 3; next re-trigger HS=6. No
  kernel/TCB. Whole set co-lands as one green candidate (§8); then D-final closes
  NATIVE-HANDLE-CARRIER + PX8-F-CAP-41 Phase 2, then RT-BACKEND-MODULE-SPLIT.

### Verify + language (lane 2) — Z3 integration; V3-FO D0 RESOLVED = arm (a), no TCB

- CI-Z3-BASE-IMAGE landed + closed (verify infra).
- KERNEL-SCT-TELESCOPE-CANON landed + CLOSED (code `ea9e5c14f`, closure
  `b902d574f`; route-A arity fix + synthetic isolation consumer; Kernel QA +
  Architect soundness APPROVE; gate un-weakened).
- V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY D0 RULED (Architect evt_6y5b4ks46syd2):
  arm (a) UPSTREAM, NO TCB. The four-row probe is an exhaustive discriminator;
  row-4 (two-level descent split into per-level mutual helpers, one matched-field
  peel per edge) passes full admission under UNCHANGED `size_rel`. Arms (b)/(c)
  are OFF — no `size_rel` completeness gap, no richer measure. The whole SCT arc
  closes with EXACTLY ONE kernel change (the landed arity canon) and no
  `size_rel` change — smallest-TCB, the frame's preferred outcome.
- AC-CONSUMER: restructure the real nine-member FoKripke `checker_soundness`
  clique so every recursive-call argument is a one-level matched field; the real
  clique must pass full admission (`kernel_check` AND `sct_check`) — no synthetic
  substitute. Architect required reviewer; Adversary hunts; NO kernel gate (no
  TCB). V3-FO-CHECKER-SOUNDNESS D3 resumes on this node's resolution.
- HELD: language-leader holds arm-(a) dispatch on transport prereq `1afbb4b6`
  landing on `origin/main` (QA-approved, awaiting the leader's merge Decision;
  single-ring/no-stacking). No source signature change authorized meanwhile.

### Foundation (lane 3) — expressibility trial, 2 of 5 resolved

Bounded Architect-burden probe; charter
`docs/program/wp/foundation-expressibility-trial.md`. Simplest-first, VEC last.

- CAT-SORT CLOSED (landed `ec395fd3f`, node-close `3e2e257b6`, blob-matched
  `1ee236ebe`; Adversary hunt CLEAN, no watch-item).
- CAT-GCD WORKING (`thr_2efpzygmfk5g3`; `wp/CAT-GCD`). D0 chose explicit-fuel
  subtractive Euclid — NO current-surface hard stop, so not an expressibility
  failure and no gap node warranted (it corroborates but does not duplicate the
  language SCT termination-presentation pressure). D1 checkpoint `130fa7d29`:
  entry checks; AC proofs next.
- Remaining after CAT-GCD: CAT-DEQUE → CAT-BSEARCH → CAT-VEC (last, fully
  dependent, highest gap risk).

### Operator questions — none open

Q1/Q2 resolved by the three-lane trial; Q3 answered YES ("tcb change
authorized. proceed.") and executed (KERNEL-SCT). No TCB authorization is due to
the operator right now: V3-FO D0 ruled arm (a) which needs none; the RT-FSREADAT
layer-3 stack is `ken-runtime` cranelift lowering (no kernel/TCB). Escalate only
if the Architect's layer-3 ruling or a future D0 rules in an operator-gated arm.

### Session log / escalations of record

- HS=3 §1a run COMPLETE (RT-FSREADAT + RT-DEAD-ARM chain, `thr_3j5ew8rhy35nh`,
  2026-08-22): research advisory delivered → Architect ruled (evt_r3tt1gpv4tkn +
  IH-marker addendum) → enumeration-first executed. RT-COLD-LOWERING-PATH-
  ENUMERATION AC-3 delivered (5 mechanisms) + sibling RT-COLD-LOWERING-CHECKED-
  FAMILY-ENUMERATION cut for the 2nd family (Architect binding finding). Count of
  record: HS=3, next re-trigger at HS=6.
- Orphan open PR #2692 (stale RT-CONTROL node version) routed to lieutenant to
  close; integrator befc2dc4/#365 stale-ref answered (2026-08-22).
