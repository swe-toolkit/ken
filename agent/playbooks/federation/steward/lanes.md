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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` (RECUT, M -> XL) | `D1` producer closure, inside the same node | Nine stops established the defect is not at a site. Census `evt_5dbc20bwkmym1`: six dispatches, ONE token, residual zero -- under-release DROPPED, over-emission is the defect. Recut framed at `§0`: mint a `ReleaseObligationId` at bracket planning, carry it demand -> row -> emission claim -> dispatch claim, reconcile the two observed families in one table, and make a claim with no id a COMPILE ERROR. `D0` releasable. Cut is one WP, not precursor+consumers (substrate alone is unobservable) and not split by family (closure is over producers). Counters separated: `§1a` at stop 12, `§1b` at entry 12. |
| L2 | language | Finish the reserved-infix-glyph objective: discharge the operator's "g then remeasure" ruling | `LANG-STANDARD-INFIX-CALL-COMPLETION` (A1) | `LANG-MEMBERSHIP-OPERATOR-SURFACE` (B, blocked on A1) | **NOT blocked on the operator — I held it and did not lift the hold.** A1's candidate `ed47f3ec9328859ff80a66d10ab9ab28c991cdba` is done and approved, held only behind `LANG-REWRITE-DESCENT-FRAME-TAX`, which **landed** at `6ce8aa0e92d221e7e56a39439c273e2697a0781c`. Hold lifted. Next action: rebase A1 onto the new `modules.rs` and remeasure whether its two arms still trip `local_prebinding_preserves_legacy_map_union_stack_budget`. Detector still trips ⇒ report and stop, do not re-place the boundary. `LANG-MODULE-IMPORT-SYSTEM` is NOT the L2 objective: every build member is merged, so the 2026-08-22 ranking is discharged. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-COLLECTIONS-NTH-LAWS` respin | `CAT-PARSING-CURSOR-LAWS` | `CAT-NAT-ORDER-LAWS` is CLOSED (D1+D2, `571d216eb`). NTH-LAWS respin `338cfe922` is CI-red at `cat_derived_pub_export.rs:219` — a SECOND consumer-view harness the frame's path enumeration missed, unmasked once the Rosetta panic stopped aborting the shard early. `AC-4` reshaped from a path list to a predicate and `AC-6` added; foundation owes a superseding SHA, re-approved exact, then re-routed. `CAT-PARSING-CURSOR-LAWS` is framed and goes ready when that lands. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.