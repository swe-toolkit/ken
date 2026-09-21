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
| L2 | language | Language surface. The operator's 2026-08-25 "unblock foundation with module/import" direction is DISCHARGED -- all 13 `LANG-MOD-*` members AND `CAT-GCD-REFACTOR` are `merged`; the root `LANG-MODULE-IMPORT-SYSTEM` sits `draft` only because a campaign root is never itself released | `LANG-FOREIGN-NAME-FORMAT-CHARS` -- `active`, RE-RELEASED `evt_pt4nk1e739` on the amended frame landed `e718a841c` restoring §1f's formatter auto-escape inverse; implementer confirmed Working | `LANG-R-LAYER-EXPORT-RETRACTION` -- `ready`, S, T2. Kick when FORMAT-CHARS closes; Architect review NOT required, the design fork is already ruled | **Section 2 now carries FIVE §1f properties; property 5 is the formatter inverse, and its absence made AC-1 demand the opposite of the spec.** `7da20c205` is DEAD twice over -- QA's three `kenfmt_b4_splicing` failures AND the normative contradiction. Three of my inputs to this WP are withdrawn as defective, one cause each time: found one true thing about the code and stopped looking. **Two language nodes read live and are NOT:** `LANG-MATCH-PATTERN-FORMS-ABSENT` is fully merged, and `LANG-ATOM-START-CLASSIFICATION-CLOSURE` is flipped `merged` in this commit per `evt_1n4k31gecsfty`. Neither is a successor. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-CONFIGURATION-DECODER-LAWS` -- `ready` on main at `d5d7e6299`, ring compacted at the new-WP gate, KICKED `evt_4n62kqs5k3vwd`. M, T1 | `CAT-PARSING-CURSOR-LAWS` -- ALREADY FRAMED `ready`, L, T1; all three `depends_on` (`CAT-PROOF-COMPLETENESS-SURVEY`, `CAT-NAT-ORDER-LAWS`, `CAT-COLLECTIONS-NTH-LAWS`) are `merged`. Kick when DECODER closes. Do NOT frame a successor -- one exists | **The deliverable instantiates the landed `schema_validate_fields::valid_coverage` at the two decoder entry points.** `env_config_validation` DISCARDS the validation's own `Valid` values and recomputes them via a second independent traversal whose `None` branch emits an empty-`Bytes` placeholder, so nothing connects `Valid` to lookup success. **If the agreement is FALSE that is a product defect and returns to me before any repair.** `CAT-PARSING-NUMERIC-LAWS` flipped `merged` in this commit -- landed `fdf20f81f`, Adversary M8 NO SOUNDNESS DEFECT. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.