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

**`main` = `48188cde6`.** Tree clean; `steward/work` == `origin/main`. Publisher
lane clear: orphans PR #2692 + #2768 both CLOSED (confirmed); integrator's
befc2dc4/#365 watchdog ref answered as stale (July auto-merge tooling); zero open
PRs. Watchdog is one convo agent-interval (1800s, server-side) — re-verify armed
at each resume.

**THREE LANES (operator, 2026-08-21/22 trial). Roster: `steward/lanes.md` —
that file is the source of truth, not this block.** Lane 1 runtime (finish the
NHC carried-observation chain, then RT-BACKEND-MODULE-SPLIT); lane 2
verify+language (Z3 integration); lane 3 foundation (expressibility trial, the
Architect-burden probe). Doc track concurrent, contention-free. Finished work
still merges; framing an active lane is lane work.

### Runtime (lane 1) — HS=4; closure A ruled, ring building the operand seat

The NHC `cap41_*` chain, co-landing on `wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`
(thr_3j5ew8rhy35nh). Held on the branch (none affects `main`, still 48188cde6):
join half (`6a45ae1a7`, APPROVED, byte-identical), built projection + disposition,
join reconcile, IH-marker producer fix (`64019430c`). Merged predecessors:
RT-DEAD-ARM-EFFECT-LOWERING (`55c7f51de`), RT-RESOURCE-RELEASE-CARRIED-OBSERVE
(`ef32b6ced`), RT-EXACTINT-CARRIED-OBSERVE (`d49a232a`, closed).

- IH-MARKER PRODUCER FIX LANDED ON BRANCH + WORKED. Language emitted
  `Call{ func: Var(method_binder_ordinal), args: [] }` at arity 0 (erasure.rs);
  both checked-family programs cleared the marker seam (supplied 0 == arity 0,
  mbo 0 == binder_index 0). Marker seam + D5a checker correct, consumer
  byte-untouched (Architect confirmed effective, evt_3amhmvyd0sr9t).
- AC-REENUM STOPPED (evt_6p7vfbadg863p) at a NEW deeper refusal — correctly,
  tables left red. Static-worker ABI supply layer: template arity 0 vs
  `worker.declared_arity` 1 (calls.rs:220; 2/2 checked-family, 8/11 rt_parity).
  Architect ruled the binary FALSE: the two authorities measure different
  objects; the real defect is the worker's declared recursive argument is never
  SUPPLIED on the ported (checked-IH) route — the composed route seats it via
  D8d `composed_recursive_argument_binding`; the checked-IH consumption validates
  the marker but does not seat the argument. §1b: inventory entries 2+3 share one
  predicate (recursive-arg supply asserted at producer/template/worker with no
  single authority actually supplying it on the ported route) — the fix is a
  structural closure, not a point-fix.
- HS 3 → 4 (count of record, Steward-confirmed evt_423sg9t98rrf4). Genuine new
  mechanism (ABI supply layer, distinct from marker-seam representation and join
  reconciliation; masked by the depth-1 family, not in the enumeration's bounded
  set). 4 is NOT a mandatory 3/6/9 re-trigger; next mandatory at 6. Research
  pulled on the merits (§1a floor, not ceiling).
- §1a CLOSURE RESOLVED — CLOSURE A (Architect evt_3tspjkw7dhh6x,
  research-backed evt_n38ptc08a1sc; B rejected). The checked-IH marker
  is a saturated nullary closure-style invocation: template arity 0 is
  CORRECT; the recursive value is an environment operand at the plan's
  `recursive_position` that the ported route must SEAT (as the composed
  route already does via `composed_recursive_argument_binding`), not an
  explicit argument. This is the STRUCTURAL closure of §1b entries 2+3
  — no count relabeled, no gate weakened. The fixture-provisioning
  contingency did NOT fire (a different mechanism, not a further
  checked-family layer).
- SUCCESSOR CUT + OWNED: RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT (owner runtime,
  active, depends_on RT-IH-MARKER-PRODUCER-COMPLETE) carries the deliverable (seat
  the operand from the resolved slot's `recursive_position` BEFORE the
  `core.rs:14306` declared-arity gate; producer/template/marker/join untouched;
  `supplied==declared_arity` holds by supply not relabel) + the terminal AC-REENUM
  gate. RT-IH-MARKER's producer fix (`64019430c`) is a CLOSED Language deliverable;
  the seat's ownership returned to Runtime. runtime-leader dispatched it to the
  implementer on the co-land branch (evt_7s2qpj3qxnrr3, "Confirm Working, proceed").
- CO-LAND (§8, ONE green candidate, does not land in pieces): join half
  (`6a45ae1a7`, APPROVED) + producer fix (`64019430c`, closed) + runtime operand
  seat + built projection/disposition + join reconcile. Lands once AC-REENUM greens
  (both checked-family programs; the 8 rt_parity 288/301-then-marker entries
  advance) and the cap41_*/terminal rows go all-green; a FURTHER refusal ⇒
  STOP+report (new deeper mechanism, back to Architect + Steward). Then D-final
  closes NATIVE-HANDLE-CARRIER + PX8-F-CAP-41 Phase 2, then RT-BACKEND-MODULE-SPLIT.
  HS=4 (count of record); next mandatory re-trigger at 6. No kernel/TCB.

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

- HS=4 §1a arc (RT-FSREADAT static-worker ABI supply, `thr_3j5ew8rhy35nh`,
  2026-08-22): Language producer fix (64019430c) cleared the marker seam
  → AC-REENUM STOPPED (evt_6p7vfbadg863p) at a deeper refusal one layer
  below (template arity 0 vs worker.declared_arity 1) → Architect ruled
  the binary FALSE, a genuine new mechanism (HS 3→4, Steward-confirmed
  evt_423sg9t98rrf4; not a mandatory 3/6/9 trigger) → §1a research pull
  (advisory delivered evt_n38ptc08a1sc, leans A) → Architect ruled
  CLOSURE A (evt_3tspjkw7dhh6x, B rejected): marker is a saturated
  nullary closure-invocation, ported route must SEAT the
  recursive_position operand. Steward cut successor
  RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT (owner runtime, carries terminal
  AC-REENUM); RT-IH-MARKER producer fix is a closed Language
  deliverable; runtime-leader dispatched the seat (evt_7s2qpj3qxnrr3).
  Count of record: HS=4, next mandatory re-trigger at HS=6.
- HS=3 §1a arc CLOSED (RT-FSREADAT + RT-DEAD-ARM chain, `thr_3j5ew8rhy35nh`,
  2026-08-22): research advisory → Architect ruled (evt_r3tt1gpv4tkn + IH-marker
  addendum) → enumeration-first, both families reported (report 1 = 5 mechanisms,
  report 2 = IH-marker only / depth-1) → Steward cut the two bounded successors
  (RT-MATERIALIZED-DEAD-JOIN-RECONCILE, RT-IH-MARKER-PRODUCER-COMPLETE) +
  BoundaryCarrier fold + 2 recorded dispositions, released (evt_4dx1y8hskm2c5,
  landed 9c17b082f). Count of record: HS=3, next re-trigger at HS=6.
- Publisher hygiene: orphans PR #2692 (stale RT-CONTROL node) + #2768 (stale
  briefing-refresh debris) both CLOSED; integrator befc2dc4/#365 stale-ref
  answered (2026-08-22). Zero open PRs.
