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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE`, `D0`+`D1`+`D2` ATOMIC, branch `wp/RT-PLANNER-KRET-GRAFTED-SPINE-D0-D1`, D2 repair in progress, WIP checkpoint `3d1bf3952` on D2a control `c987c3ef0` | `D2` is IN, not next | D2a PRE-REPAIR READINGS COMPLETE at `evt_498rdcf40trtw`: 517 and 1092 both red, and the reachable-wrong-scope mutation reds on 528 NOT 517, which is what separates ownership-relative placement from global reachability. The policed before-repair order is SATISFIED; repair is authorized to proceed. Scope unchanged: D0+D1+D2 atomic, no candidate cut until all three are green together, FORCED not preferred because D0's fail-closed refusal is live on this member. It BECOMES a recut only if construction shows D0+D1 green with the member unplaced. This seat runs deep past its auto-compact threshold (161%); the WIP checkpoint is why a compaction now costs reasoning, not work. Counters: stop 13, entry 14; neither trigger fires until 15. |
| L2 | language | Remeasure the descent stack detector on the repaired descent -- operator "g then remeasure", 2026-09-19 | `LANG-DESCENT-STACK-DETECTOR-RECALIBRATION` -- candidate `7d3da137537bd784d92e592f9104440f32bc32df` at base `d23a65021`, ARCHITECT APPROVED `evt_fb71x8824z5p`, language-qa verdict outstanding, then leader assembly + Decision + route to me | `LANG-ATOM-START-CLASSIFICATION-CLOSURE` -- ALREADY FRAMED, RELEASED AND `active` (`L`/`T1`). NO FRAMING IS OWED; do not author a successor for this lane | Result: AC-3 returned NOT-refuted -- at the old `512` BOTH states pass, so the calibration is genuinely stale; new value `49_152`, stated stack untouched at 2 MiB, boundary NOT re-placed, one path `+7/-5`. On landing, kick ATOM-START and have the implementer re-verify its anchors, which date to 2026-09-17 and are perishable. This lane's objective is DISCHARGED when this lands; ATOM-START carries it, so no operator objective question is owed yet. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PROPERTY-LAWS` -- ACTIVE, branch `wp/CAT-PROPERTY-LAWS` cut from `d23a65021`, foundation-implementer Working `evt_6w6jvnxm4tv36` | next of the seventeen survey follow-ons; 12 of 17 remain unframed | DO NOT pick the next node from the survey's one-line recommendations -- MEASURE the package first. `CAT-CONSOLE-TEXT-LAWS` looked smallest and is REJECTED: its four helpers ARE their own definitions, so the reducible half is `refl`, and the only non-trivial claim runs through `bytes_encode` and `list_char_to_string`, which that same survey classifies as opaque TCB exports. STANDING LOW on landed `Arguments.ken.md`: `refuses_missing_argument` is weaker than unconditional, and the ORIGIN IS MY OWN FRAME -- CAT-PROCESS AC-2 required each refusal to satisfy the other two premises, which forced hypotheses the law does not need. Not a node (§4c); fix it if a future node touches that file. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.