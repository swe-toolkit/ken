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
  out of `LANG-TRANSPORT-SIGMA-PREMISE-SYNTHESIS`.

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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` | Repair determined by the `:4492` consumer measurement | Re-probe resolved reading (c) `evt_2k87p66qwztzw`: the demand at `k_specialization 1` exists in `deferred`, so the join searches a population the object can never be in. Architect ruled `evt_7p1d1602np53c` — his 503 causal claim is refuted, the variant-domain mismatch stays latent, and narrowing `base_owner` would not have fixed 503. One deciding measurement outstanding: does any consumer at 503's plane demand a result identity checked at `:4492`? No consumer ⇒ no-op on zero matches, keep the `> 1` error; a consumer ⇒ upstream population defect. Count stays 6; §1a/§1b at 9. |
| L2 | language | Deliver `LANG-ACTIVE-PREMISE-KERNEL-VIEW` under the 2026-09-12 Route-B ruling | `LANG-REWRITE-DESCENT-FRAME-TAX` | `LANG-ACTIVE-PREMISE-KERNEL-VIEW` | Complete the already-requested AC-4 targeted rerun, then finish exact-SHA review and route. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-NAT-ORDER-LAWS` D2 | `CAT-PARSING-CURSOR-LAWS` | NTH-LAWS `2bd127cf7` is dead: CI-red on the `rosetta.rs` import pin, which `AC-4` forbade repairing. This commit amends `AC-4` to two paths and adds `AC-5`; foundation respins a superseding SHA, re-approved exact, then re-routed. D2 active with the implementer on the landed `cb5d4d18d` amendment, anchor `evt_2fgt1eb7hd8sq`. `CAT-PARSING-CURSOR-LAWS` is framed and goes ready once both land. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.