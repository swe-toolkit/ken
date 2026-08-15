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

## LIVE — 2026-08-15 11:05Z

**`main` = `030373602`.** Tree clean, nothing held, no publisher running.
Today's code PRs: #2301 (language `D4`), #2305 (runtime census), #2310 (runtime
call-site attribution). Doc-only: #2298, #2300, #2302, #2303, #2304, #2306,
#2308, #2309, #2311. Closed: #2299 (empty), #2307 (conflicting — predated the
census merge; superseded by #2308).

**TWO LANES AND NOTHING ELSE GETS A RING** (operator, 2026-08-15; `steward.md`
§0). Runtime retires `RecursiveDescent`; language + verify do the z3 round-trip.
Finished work still merges, filings queue, and framing for these two lanes is
lane work.

### LANE 1 — three merges this morning; the fork is one field from settled

**`RT-REQUIRED-CONSUMER-REACH-CENSUS` merged at `50aff29b4`** (PR #2305), `D1`
through `D5` delivered. M6 blob identity verified against the **declared**
merge-base; M7-M9 discharged.

**What `D5` established:** the required-consumer route **manufactures the
closure-bearing CROSSING** at `StaticOriginId(5)` / `Constructor.arg[0].Closure`.
Enabled rows are `(closure_present, crossing_reached) = (true, true)`; both
suppressed legs are `(false, false)` and return to `StaticWorkerBinding`.

> **BRANCH 2 IS ELIMINATED, and that is the durable result.** Its antecedent
> required the crossing to be **reached under suppression**, and it is not. It
> was the **only** branch under which these rows and [[RT-CLOSURE-BOUNDARY-LANE]]
> could ever have been one defect ⇒ **the subsumption is dead**, and that node no
> longer waits on this chain to be sized. My derivation from the Architect's
> pre-committed table, flagged as such in both nodes.

**Branches 1 and 3' are NOT separated, and suppression may not be able to
separate them at all.** `closure_path` is computed only at the crossing, so
`closure_child_present: false` on the suppressed rows is an artifact of having no
observation point — both branches predict it. Without the projection these rows
never build the subgraph, so *"does the closure pre-exist"* may be **ill-posed**.
**Do not re-run the differential.** `D5`'s `CLAIMED` line was amended (Architect
`evt_38p42gjq12br`) to stop carrying a branch label it did not establish.

**[[RT-CROSSING-CALL-SITE-ATTRIBUTION]] then MERGED at `637781f41`** (PR #2310),
`D1`-`D3` delivered. Both enabled rows enter the origin-5 crossing with
`invoking_site = GeneratedUnitCallInput` — the tag on `carry_call_input`, **not**
the realization's return surface. `D3` closed all three latent misreports,
including the Adversary's stale-origin-key check (`evt_6mpw6frz1h508`), which now
fires **before** the four-row table and was demonstrated by mutation.

> ### BRANCH 1 IS SELECTED **PROVISIONALLY**, AND THE QUALIFIER IS LOAD-BEARING
>
> The tag sits on a **shared helper with six callers** (`core.rs:17996`,
> `:18014`, `:18350`, `:18434`, `:18449`, and the `carry_source_call_inputs` loop
> at `mod.rs:7618`). It establishes *"a generated-unit call input was being
> carried"* — **not whose call**, which is exactly what separates the branches.
> Callee the source program already calls ⇒ **1**; the projected consumer's
> generated unit ⇒ **3'** survives.
>
> **The supporting claim grounds and is still not enough.**
> `realize_required_consumer_locally` ends at `RoutedAnswer::composed_answer(...)`
> and carries nothing, so this is not its **return** surface — **but ruling out
> the return surface does not rule out the realization emitting a CALL.**

**The successor is [[RT-CROSSING-CALLEE-IDENTITY]]** — `ready`, `S`, kicked at
`evt_7v2y8ptre3xfs` (**new thread**). One tag finer, not a new instrument: record
the callee's unit identity, decide the branch per row, and **resolve the
"provisional" qualifier in the tree**. Its `D3` exercises the tag's unused
negative arm — `BoundaryTransferInvokingSite` has two inhabitants and every
assertion reports the same one (`control.rs:6305`, `:6329`).

> **NO REPAIR NODE MAY BE CUT BEFORE IT MEASURES** — the Architect's sequencing
> (`dec_5m10b60wam0rz`), because a repair cut against branch 1 while 3' holds is
> the `D2k-1c` cost arriving at the last possible moment.

**Three merges on this chain this morning.** The recurring shape, worth naming:
**the measurements have been sound and the labels have overreached** — `D5`'s
`CLAIMED` line, then `D2`'s delivery inference. Both were caught by reading the
instrument rather than the result.

**No further successor is framable until `CALLEE-IDENTITY` decides the branch —
the repair node's owner and file follow from that selection. Not framing debt.**

**Chain:** `PROJECTION` (merged) → `CENSUS` (merged) → `CALL-SITE` (merged) →
`CALLEE-IDENTITY` (ready) → repair → `TRANSPORT` → `DESCENT-RETIRE`.
`RT-RECURSOR-TRANSPORT`'s `D3` gate is keyed on the **tree** (`enum
RecursiveDescentResidual`, `core.rs:1979`, two live variants), never on node
status. **Do not "fix" that wording.**

### LANE 2 — language's `D4` merged, `D2` widened, verify between work

**`LANG-INTERVENING-LET-FRAME-WEAKENING` `D4` MERGED AT `7b11bbd84`** (PR
#2301, exact `956d86921`, `dec_7r12dsg9py2a4`). M6-M9 all discharged. **The node
stays `active` — `D2` and `D3` are deferred, not done.** `D1` had already landed
at `beb31566b` (PR #2282) when its publish was requested; PR #2299 closed empty.

**`D2` is now WIDENED to all four operand assertions in
`ds5b_dependent_match_refinement_acceptance.rs`** (Adversary hunt
`evt_399y8ys1ftnee`, accepted; landed `505e9b5cd`, PR #2303; notified
`evt_1s48da3b9zzga`). `contains("@4")` was one of **three** unanchored
substrings, and the third (`:507`) sat on merged `D1`'s node — a file-level
convention split across two nodes, with one half owned by nobody.

> **The two findings do NOT stack.** Architect Finding 1 **deletes** `:599`
> and `:604` — the error class alone discriminates — so anchoring them is moot.
> **Anchoring is the live repair only at `:507`**, where `Dg67` is a name, not a
> positional level. `AC-6` states the four-site outcome. Details in the node.

**Before publishing anything:** `git diff --quiet origin/main <head> -- <declared
paths>`. A squash rewrite makes a **landed** head read as owed forever, and two
seats' statuses agreed on that stale read — which is not corroboration.

**`V3-Z3-PROCESS-ADAPTER` merged.** `blocks: []`, and the throughput successor
needs a catalog-scale corpus that does not exist. **Do NOT invent a z3
successor.**

**LIVE HAZARD:** the `z3-process-adapter` CI job is in the **required**
aggregate, so an apt failure reds every PR fleet-wide. **Do not just drop it** —
only that job witnesses SMT-LIB emission, because `stub(..)` discards stdin.

### The operator brief for 11:30 — three items, and that is the whole list

1. **Verify has no directed successor.** Between work, not blocked.
2. **`V3-KRIPKE-THEORY-CLOSURE` is `ready`, framed, enclave-owned**, and closes
   lane 2's FO Kripke half; `spec-leader` is idle awaiting a released node. **I
   did not kick it.** §0 rule 1 bans starting a third ring *"however well-framed
   and however idle the team"*, and *"this idle team unblocks a priority lane"*
   is recognizably one of the three arguments that already defeated the priority
   once today. **Operator's call, and the work is not wasted whenever it runs.**
3. **Language has no lane-2 successor.** Its current node is elaborator work
   discharging an Architect follow-up, not the z3 round-trip.

**Still owed by the operator — three, do NOT re-raise:** `evt_h6pbx30amprj`;
the `LANG-FOREIGN-NAME-FORMAT-CHARS` threat model; `evt_30gckze0jryj4`.

### Queued, and none of it may jump the lanes

- **The publisher gate misdiagnoses an already-landed candidate as an
  environment fault.** `scripts/scripted-pr-automerge.sh:519` does `git merge
  --squash` then `git commit`; when the candidate is **already contained in
  `main`** the merge stages nothing and the commit exits non-zero with *"nothing
  to commit"*. The gate carefully separates merge-failure from commit-failure
  and then reports *"an environment fault in the publisher... check that the
  scratch worktree is writable"* — **which sends the reader to look at disk and
  permissions for a condition that is neither.** Measured 2026-08-15 on PR #2299.
  **The fix is one probe:** if `git diff --quiet origin/main <head>` (or the
  squash stages nothing), say *the candidate is already contained in `main`* and
  name the landing commit. **Mine, and it queues** — it is `scripts/`, so it is
  outside the Adversary's scope by `COORDINATION §10⁻a` and is found only by
  whoever trips over it.
- The **tri-state convention** the Adversary raised (`evt_62attjpj3esa`): the
  empty-scan fallback and `validator_admitted` on `D2k-1e` are the same shape —
  a two-state observation standing in for a three-state question whose missing
  state is *the instrument did not observe*. **Two nodes owe one conversion**,
  so it wants one convention, not two half-arguments. Explicitly excluded from
  the census node's scope.
- `CONF-BLOCKER-OWNER-RESOLVABILITY` — `ready`, NOT kicked; enclave.
- ken-interp `Term::J`/`cast_reduce` — a **declared** G1 oracle boundary at
  `16 §9.1`, **not a defect**.

### Rules earned 2026-08-15

1. **"Unblocked" is not "reachable".** A surface can land, clear every tracker
   edge, and still not reach part of its population — check the guard that
   mints it, not the node's status.
2. **Scope a stop condition to the deliverable that depends on it**, never to
   the node. Mine stopped `D2` and `D4`, which never depended on the fork, and
   the ring's wide reading was the correct reading of what I wrote.
3. **A shared refusal sentence is shared syntax.** When the gate is total, every
   population reaches it; the attribution is upstream.
4. **Resolve a participant id at post time, from the script.** A guessed id
   posts successfully and notifies nobody — it looks like a delivered message
   and is a silent stall.
5. **A control's population is a claim a sibling node can falsify** without
   touching the control.
6. **Check the PASSING half before disposing of a red** — an absence-assertion
   passes for free once the row dies earlier.
7. **A pooled denominator structurally cannot report per-row facts.**
8. **Attribute before re-triggering:** is `main`'s `crates/` tree identical to
   the last **green** code merge?
9. **Two accepted findings on one site can prescribe OPPOSITE remedies.** Both
   were right; one deletes the assertion, the other anchors it, and doing both
   is a defect. When you widen a scope from a second report, state per site
   **which** remedy applies — a merged list reads as "apply everything."
10. **A defect spanning two nodes has a half owned by nobody** once one of them
    merges. Check whether the sibling half sits on a closed node before scoping
    the repair to the open one.
