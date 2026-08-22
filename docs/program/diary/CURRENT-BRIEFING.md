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

**`main` = `493672c8a`.** Tree clean; `steward/work` == `origin/main`. Publisher
lane clear (lieutenant removed a stale CAT-SORT-close `merge.lock`,
evt_10na22h4h4jkh). Watchdog + daily briefing-flush cron are session-only —
re-verify armed at each resume.

**THREE LANES (operator, 2026-08-21/22 trial). Roster: `steward/lanes.md` —
that file is the source of truth, not this block.** Lane 1 runtime (finish the
NHC carried-observation chain, then RT-BACKEND-MODULE-SPLIT); lane 2
verify+language (Z3 integration); lane 3 foundation (expressibility trial, the
Architect-burden probe). Doc track concurrent, contention-free. Finished work
still merges; framing an active lane is lane work.

### Runtime (lane 1) — carried-observation chain at a 3rd hard-stop; research advisory in flight

The NHC `cap41_*` chain advances row-by-row: each landed fix moves the compile
to the next distinct blocker until the last lands and D-final re-runs all-green.

- MERGED predecessors: RT-DEAD-ARM-EFFECT-LOWERING (`55c7f51de`),
  RT-RESOURCE-RELEASE-CARRIED-OBSERVE (`ef32b6ced`), RT-EXACTINT-CARRIED-OBSERVE
  (D1 `d49a232a`; node now CLOSED, runtime-qa retro).
- RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL + RT-DEAD-ARM-JOIN-DISPOSITION are
  CO-LANDING as one candidate on `wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`
  (thread `thr_3j5ew8rhy35nh`). Both BUILT and measured sound: the resource-aware
  carried projection meets the Architect's AC-2 envelope; the join disposition
  is 19 dispositions = the 19 measured-unconsumed joins, sourced from the
  deadness predicate's own witness (`provably_dead_arm_body_containing`), so
  disposition-follows-deadness holds BY CONSTRUCTION (predicate untouched).
  Neither greens the compile alone; together they do not either.
- HARD STOP, HS=3 (research-advisory trigger FIRED). A THIRD latent invariant
  surfaced behind the join fix: `validate_materialized_dead_join_cfg` refuses
  materialized-but-dead source join `StaticOriginId(288)` — consumed AND
  dispositioned (overlap_len=1), NOT one of the implementer's 19 (pre-existing,
  reachable for the first time now layer 2 cleared; fail-closed, safe). The
  pattern IS the finding: three individually-sound fail-closed completeness
  invariants (effect-seat need, join-consumption, materialized-dead CFG
  reachability) stack on one lowering path, each masking the next.
- §1a IN FLIGHT (happy path). Architect HOLDING its layer-3 ruling
  (evt_6jshpy0n2164n), framed a sharp research prior-art ask (meta-pattern /
  layer-3 join-phi under dead-region elimination / coverage; CompCert, CakeML,
  verified LLVM/MLIR, SSA refs). Research PICKED UP (evt_13f09d04hc9wc, Sol/T1,
  advisory only). Architect resumes + rules on the advisory; the ruling stays
  the Architect's; runtime-implementer holds (not chasing layer 3). Steward
  carries the count of record, no design opinion.
- NHC held on the chain; D-final re-runs when both nodes land and the rows go
  all-green (closes NATIVE-HANDLE-CARRIER + PX8-F-CAP-41 Phase 2). Then pivot to
  RT-BACKEND-MODULE-SPLIT (`draft` — needs framing before startable).

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

- HS=3 research-advisory escalation logged (RT-FSREADAT + RT-DEAD-ARM chain,
  `thr_3j5ew8rhy35nh`, 2026-08-22): 3rd hard-stop on one mechanism chain;
  Architect holding per §1a; research advisory pending. Count of record: HS=3,
  next re-trigger at HS=6.
