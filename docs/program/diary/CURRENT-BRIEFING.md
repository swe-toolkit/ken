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

**`main` = `aa40a2378`.** Tree clean; `steward/work` == `origin/main`. Publisher
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

### Runtime (lane 1) — HS=5; ESCAPING measured, co-land RE-SCOPED to land without the functional-IH piece

The NHC `cap41_*` chain, co-landing on `wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`
(thr_3j5ew8rhy35nh). Held on the branch (none affects `main`, still aa40a2378):
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
- §1a CLOSURE A WITHDRAWN (Architect evt_hftfnn4mh8jk) on the implementer's
  grounded refutation (evt_6tzrt1xndpx1e, measured on `64019430c`). At the firing
  seam (`core.rs:12519`, direct descent — NOT the source-machine sites) the
  recursive operand is a LIVE nested-recursor StaticWorker (declared_arity 1,
  captures 9). `LoweringOperand` is only `{ Specialized(Lowered),
  Carried(CarriedBoundaryWord) }` and `worker.captures` is itself
  `Vec<LoweringOperand>`, so the live worker fits NEITHER `inputs` NOR captures;
  the only conversion is a Lowered closure value — the closure-boundary crossing
  Ken forbids (the `rt_write_writable_stage` / BoundaryCarrier wall). Nothing can
  be "seated as an operand". §1b: BOTH Closure A (seat as input) and B (producer
  emits operand) shared the refuted PLACEABILITY premise — B is not resurrected
  (same wall). The marker's nullary/arity-0 semantic STANDS.
- HS 4 → 5 (count of record, Steward-confirmed evt_1jp5fvt7zt88v). Second
  premise-refutation in the arc (join-reachability, now IH-operand-placeability);
  a distinct mechanism (live-worker-cannot-be-an-operand, same boundary
  prohibition as BoundaryCarrier). NOT itself a mandatory 3/6/9 trigger; next
  mandatory re-trigger at 6, so a further hard stop on this design question fires
  it. Architect pulled research now on the merits (§1a floor).
- REFRAMED RULING LANDED (Architect evt_2f4bbmt7qfde1, research advisory
  evt_5jp0npxf78erv in hand). The marker is a NULLARY_FORCE — Ken's own
  elaborator concept (`compiler_driver.rs:1718-1743` sets
  `nullary_force = arguments.is_empty()`).
  Template arity 0 means "realize the specialized IH function VALUE, unapplied," NOT
  "call with zero args." For ITree.Vis the IH is `lambda response. rec (k response)`
  — field k fixed, response still explicit. The measured arity-1 StaticWorker is the
  applied BODY (compiler metadata, no runtime identity; `mod.rs:3578-3603`), not the
  realized value. The fork was an ESCAPE discriminator (GHC let-no-escape); the
  Architect handed runtime a decisive measurement (measure, not build).
- ESCAPE MEASUREMENT: ESCAPING (runtime-implementer evt_79jd1nxamqd95, on
  `64019430c`). The realized IH value's immediate parent is `Expression(Construct)`
  on BOTH checked-family programs — stored straight into a constructor field. Two
  marker populations: 12 fine `template=4/arity=1` (`source.rs:655`) and exactly one
  failing `template=0/arity=0` nullary_force per program (`core.rs:11699`). The
  non-escaping remedy is inapplicable here (no response arg, no in-frame application
  site). Architect confirmed the mechanism half (evt_5wvex36s7nm6d).
- STEWARD SCOPING RULING (evt_5pmk273zg5paa, on the Architect's pre-committed
  conditional ruling). CAPABILITY-GAP branch: Ken has no first-class functional-IH
  value. (1) The co-land set lands WITHOUT the functional-IH piece — join half
  (`6a45ae1a7`, APPROVED) + producer fix (`64019430c`, closed) +
  projection/disposition/join-reconcile + the re-point below. (2) AUTHORIZED: both
  checked-family runner tables re-point to documented ADVANCING-REFUSAL pins (they
  refuse at the correct deeper capability-gap point now), so the candidate is green
  with the refusal encoded. On its green, D-final closes NATIVE-HANDLE-CARRIER +
  PX8-F-CAP-41 Phase 2, then RT-BACKEND-MODULE-SPLIT. Runtime-leader assembles the
  candidate (approvals live on exact SHAs, green on cut + main, §8a) and routes the
  git_request to me. (3) New deliverable cut:
  [[RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]] (draft/deferred, owner runtime, T1,
  size L) — carries a design D0 the Architect rules before build (materialized
  closure value vs defunctionalized carried tag) and the terminal AC-REENUM. (4)
  RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT is CLOSED (superseded). HS stays 5 (ESCAPING
  is a scoping decision, not an increment; next mandatory at 6). No kernel/TCB.
- OPERATOR SURFACE (not blocking): a new large functional-IH-representation
  deliverable now exists (`RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION`, deferred). Its
  sequencing into a lane is a priority call. Nothing in the co-land, the NHC close,
  or the module-split pivot waits on it; the two checked-family programs are a
  documented advancing refusal until it lands.

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

- ESCAPING resolved + co-land re-scoped (`thr_3j5ew8rhy35nh`, 2026-08-22): the
  reframed ruling (Architect evt_2f4bbmt7qfde1) established the marker as a
  NULLARY_FORCE and made the fork an ESCAPE discriminator; the decisive measurement
  (runtime-implementer evt_79jd1nxamqd95) came back ESCAPING (realized IH value's
  parent is a Construct on both programs). Steward scoping ruling
  (evt_5pmk273zg5paa): capability-gap branch — co-land lands WITHOUT the
  functional-IH piece; both runner tables re-point to documented advancing-refusal
  pins; new deliverable RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION cut (draft/deferred,
  owner runtime, design D0 = materialized closure vs defunctionalized carried tag,
  Architect rules before build); RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT CLOSED
  (superseded). HS stays 5 (scoping decision, not an increment). Runtime-leader owes
  the assembled candidate + git_request to me. Operator surface: sequencing of the
  new representation deliverable is a priority call (nothing waits on it).
- HS=5 §1a arc (RT-FSREADAT IH-operand-placeability, `thr_3j5ew8rhy35nh`,
  2026-08-22): the runtime-implementer built to the ruled Closure A, hit two
  structural obstacles and REFUTED it (evt_6tzrt1xndpx1e, measured on 64019430c) —
  the recursive value at the firing seam (core.rs:12519) is a live nested-recursor
  StaticWorker that fits neither `inputs` nor captures and cannot become a
  LoweringOperand without the forbidden closure-boundary crossing. Architect
  ACCEPTED the refutation and WITHDREW Closure A (evt_hftfnn4mh8jk): both A and B
  shared the refuted placeability premise; B not resurrected. HS 4→5 (a distinct
  mechanism, Steward-confirmed evt_1jp5fvt7zt88v; not a mandatory 3/6/9 trigger;
  next at 6). Reframed §1a research pull issued (leading hypothesis: nullary FORCE
  of a lazy IH thunk, upstream of the gate). Steward set
  RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT to `status: draft`/held (name + mechanism
  refuted),
  to be re-framed/superseded on the reframed ruling. Nothing lands; producer fix +
  join half untouched; tables stay red. Count of record: HS=5, next mandatory at 6.
- HS=4 §1a arc (RT-FSREADAT static-worker ABI supply, `thr_3j5ew8rhy35nh`,
  2026-08-22) [Closure A later WITHDRAWN — see HS=5 above]: Language producer fix
  (64019430c) cleared the marker seam
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
