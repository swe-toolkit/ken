# Live lane roster

Current operational state only. The operator owns lane count, ordering, and
objectives. The Steward updates this file only when one of those fields changes
and cites the ruling that changed it.

Hard limit: 80 lines. Replace stale text; never append history. Do not record
completed work, review transcripts, measurements, explanations, prior states, or
superseded instructions. Git and the WP thread are the history.

## Live roster direction

These rulings remain operative and are retained verbatim.

- **2026-08-22:** "playbook = stable discipline, this file = mutable roster."
- **2026-08-25:** "there are three lanes authorized right now. language (lane
  2) was unblocking rt on priority, and when that was done should have unblocked
  foundation (lane 3) with module/import."
- **2026-09-05:** "btw the trial is over. 3 lanes works with some contention.
  retire the idea that there is still a trial running."

## Live objective direction

- **2026-09-12, Route B:** L2's next deliverable is
  `LANG-ACTIVE-PREMISE-KERNEL-VIEW`, the contextual kernel-query boundary split
  out of `LANG-TRANSPORT-SIGMA-PREMISE-SYNTHESIS`. **Discharged — that node is
  merged.**
- **2026-09-19, "g then remeasure":** selects option (g) on the A1 re-baseline
  fork and directs that the measurement follow the repair. **(g) is landed and
  the remeasure is not** — so this ruling is L2's live objective and L2 is not
  waiting on a new one.

## Catalog proof direction

- **2026-09-13:** "The proofs are not done. A catalog package is not finished
  until its proofs are complete. Declaring that they are tested computation is
  less valuable than proven correct behavior."
- **2026-09-13:** "computational tests are not part of the package, so a reader
  cannot trust them as intrinsics, but must believe that the implementation of
  both the package and its tests was done correctly. This is inherently
  inferior and weakens the promise that ken strives to make."
- **2026-09-13:** "schedule the proof backfill before extending the catalog."

## Runtime and streamline direction

- **2026-09-17:** "Is L1 still working on clearing the ignored tests? That is
  the top priority until it is done."
- **2026-09-19:** "It is too slow. You need to streamline the process and focus
  on forward movement, not management."

The 2026-09-19 ruling means frames stay short; investigation is the first step
of a repair, not a separate report node; label and wording hygiene do not enter
a lane.

## Authorized roster

Three lanes, in priority order:

1. Runtime
2. Language
3. Foundation

Finished accepted work still routes immediately, regardless of lane. No other
ring starts without an operator lane change.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE`, `D0`+`D1`+`D2` ATOMIC, branch `wp/RT-PLANNER-KRET-GRAFTED-SPINE-D0-D1` at base `a5833198a`, checkpoint `aa80775ea` | `D2` is IN, not next | SCOPE CONFIRMED 2026-09-20 at `evt_1zh6tdwv4msj0` on Architect ruling `evt_52sx1pt2tsz77`: outcome 2 of my `evt_3h5hfr8sa771t` is met, `D2` JOINS and the node lands whole. FORCED, not preferred -- `D0`'s fail-closed refusal is live on this member, so a `D0`+`D1` object without `D2` is RED, not a smaller green candidate; a split reproduces the cliff one step over, which is why standalone `D0` was retired. NOT a recut: recutting only helps when the pieces land separately and these cannot. It BECOMES a recut if construction shows `D0`+`D1` can be green with the member unplaced -- bring that back rather than defending the cut. `D2a` RUNS BEFORE THE REPAIR (red on the existing 517 control and on 1092, keep the reachable-wrong-scope mutation control); a placement repair validated only after the fact cannot separate a control that passes because the defect is fixed from one that never discriminated. The five-invariant envelope and the placement/ownership axis split are the Architect's and I am not ruling mechanism. Counters: stop 13, entry 14, durable in `c743baeab`; neither trigger fires until 15, so no Research advisory is owed. WIP clock reset 16:08. |
| L2 | language | Remeasure the descent stack detector on the repaired descent -- operator "g then remeasure", 2026-09-19 | `LANG-DESCENT-STACK-DETECTOR-RECALIBRATION` -- frame released in this commit, `S`/`T1`, ring not yet kicked | none framed, and deliberately so -- the successor is set by what the remeasure RETURNS, and a speculative alternative is not authorized | KICKOFF OWED to language-leader once this lands. THE NODE CAN REFUTE ITS OWN PREMISE: AC-3 runs the inline-parent mutation at the current `512`, and if `512` still discriminates identically then the premise is dead -- the ring reports that and stops, which is a PASS of this node, not a failure to deliver. The stop condition forbids RE-PLACING the detector's boundary (raising or shrinking the stated stack, enlarging the workload); option (g) was selected as the only option that does not, and a ring re-placing it here would spend that choice. If the measurement says the claim can no longer be stated as a stack boundary, the frame amendment is MINE to author, not the ring's to absorb. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PROCESS-ARGUMENTS-LAWS` -- candidate `3a4124e99` ROUTED at `evt_203cbxr4j8p7s`, FULL CI; lieutenant owns M4-M9 | `CAT-PROPERTY-LAWS` -- framed 2026-09-20, `M`/`T1`, released in this commit; 12 of the 17 survey follow-ons remain unframed | Closeout rides with the lieutenant: Librarian notification after landing (`library/SOURCE-ATTESTATIONS:19` pins `Arguments.ken.md` at blob `f6b3c110a7b5ce3b55dc47f37aef66ad5870318a`) and the M8 Adversary notice. `CAT-PARSING-CURSOR-LAWS` stays PARKED WHOLE at `6065b4993` pending the non-local same-`GlobalId` resolver precursor; re-baselining the sentinel stays REFUSED. My private-proof-machinery ruling (`evt_3862b5gwzg805`) is SPENT, NOT GENERAL -- it was measured against `CAT-COMPARE-LAWS`'s six landed private `theorem`s and the target file's own private `fn argument_bytes_at`, so the next node re-measures its own corpus rather than citing it. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.