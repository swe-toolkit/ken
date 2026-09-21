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
  fork and directs that the measurement follow the repair. **DISCHARGED — both
  halves landed at `6a36cfbdd`.** L2 has no live operator objective; see its
  row for the standing direction I sequenced it against.

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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` -- UNPARKED by the precursor landing `e1aefc1f9` (12/12 blobs verified); ring compacted at the new-WP gate, kicked this turn. M, T1 | None framed. The one-attempt bound decides: on success L1 takes the next ignored-row node, on a hard stop this node parks again | **ONE final atomic D0+D1+D2 attempt, then park again regardless of outcome.** Counts stay 15 hard stops / 16 symptom entries -- the precursor did NOT spend the one-attempt bound. The anchor `exact_response_ret_identity` is re-verified at `responses.rs:1379` on the landed main; the precursor did not touch that file. It published `grafted_spine_control_graph.rs` -- `GraftedSpineValidationObservation`, `GraftedSpineActualLoweringObservation`, `with_grafted_spine_validation_mutation` -- which is the scaffolding this node builds on. `RT-DUPLICATED-RESPONSE-BLOCK` stays `active` on its remaining rows after the partial landed `ea117ce04`. |
| L2 | language | Language surface. The operator's 2026-08-25 "unblock foundation with module/import" direction is DISCHARGED -- all 13 `LANG-MOD-*` members AND `CAT-GCD-REFACTOR` are `merged`; the root `LANG-MODULE-IMPORT-SYSTEM` sits `draft` only because a campaign root is never itself released | `LANG-R-LAYER-EXPORT-RETRACTION` -- `ready`, KICKED this turn, S, T2. Architect review NOT required, the design fork is already ruled; Language QA exact-SHA, code merge is full CI + M8 | None `ready`. Draft pool is `LANG-SEAL2-GATE-INCRATE-RELOCATION` and `CORE-AUDIT-LABELS-ARE-ARTIFACT-IDENTITY`; select when RETRACTION nears landing, per section 4e timing. Do not frame ahead of that | **`LANG-FOREIGN-NAME-FORMAT-CHARS` CLOSED** -- landed `e5dcb12da`, 7/7 blobs verified against routed `3783e545a`, lieutenant requested M8. Three of my inputs to that WP were withdrawn as defective, one cause each time: found one true thing about the code and stopped looking. `LANG-MATCH-PATTERN-FORMS-ABSENT` reads `draft` but is fully merged; it is not a successor. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-CONFIGURATION-DECODER-LAWS` -- frame AMENDED in this commit on the Architect ruling `evt_44c2n2qh6y1c1`; re-release owed to foundation-leader. M, T1 | `CAT-PARSING-CURSOR-LAWS` -- ALREADY FRAMED `ready`, L, T1; all three `depends_on` are `merged`. Kick when DECODER closes. Do NOT frame a successor -- one exists | **My frame contradicted itself and stopped the ring:** AC-1 mandated a public law naming `env_config_lookup` while section 6 forbade widening any export. Ruling: publish it in place, `fn` -> `pub fn` ONLY, exact 8-identity published population, Tier-E controls updated; the nine provider identities are authorized by me as an exact ledger, not a prohibition. Held WIP `e3dfa12b3` retains kernel-checked proof content but is NOT a candidate -- it still owes QA and exact-SHA Architect review. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.