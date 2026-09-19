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
  merged, so L2 has no operator-set objective and its next one is a live
  operator fork.**

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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` | Repair determined by the double-dispatch locus | Consumer population measured zero `evt_2gs158dpvtn97`, selecting the legal-zero branch; repair `bc4764fb8` on the WP branch retires the terminal zero-match arm and keeps the `> 1` error, and the ignored row now advances past the origin-503 planner invariant to a downstream `-1`. Architect ruled the locus `evt_3wv0jdhn3x0fm`: same-route conclusion withdrawn, tag arithmetic survives, locus is the DOUBLE DISPATCH — 3 expected releases against 6 observed, the 1-Released/5-Closed split proving duplicates hit an already-closed handle. That is inventory entry 4, and entry 5 is the open fork. Count 7; §1a/§1b at 9. |
| L2 | language | None set — the Route-B objective is discharged | none | Operator fork | **BLOCKED ON THE OPERATOR, and the lane does not know it.** `LANG-ACTIVE-PREMISE-KERNEL-VIEW` and `LANG-REWRITE-DESCENT-FRAME-TAX` are both merged; the A1 hold was already discharged by Pat's "g then remeasure" ruling landing as `6ce8aa0e9`. The lane is idle awaiting an objective, not awaiting a decision it has pending. Fork put to Pat: resume A1 re-baseline then `LANG-MODULE-IMPORT-SYSTEM` (recommended — 2026-08-22 ranks module/import above surface-syntax items because it blocks the foundation catalog trial), versus `LANG-MEMBERSHIP-OPERATOR-SURFACE`. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-COLLECTIONS-NTH-LAWS` respin | `CAT-PARSING-CURSOR-LAWS` | `CAT-NAT-ORDER-LAWS` is CLOSED (D1+D2, `571d216eb`). NTH-LAWS respin `338cfe922` is CI-red at `cat_derived_pub_export.rs:219` — a SECOND consumer-view harness the frame's path enumeration missed, unmasked once the Rosetta panic stopped aborting the shard early. `AC-4` reshaped from a path list to a predicate and `AC-6` added; foundation owes a superseding SHA, re-approved exact, then re-routed. `CAT-PARSING-CURSOR-LAWS` is framed and goes ready when that lands. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.