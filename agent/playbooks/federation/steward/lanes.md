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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` -- **PARKED at hard stop 15** by Architect ruling `evt_7bff93f1zg2jh`. Another direct-edge respin is REJECTED; the faithful form is a typed well-nested interprocedural control graph plus the existing `AbiSlotKind::Control` word as an affine member token. Frame section 9 records the park | `RT-DUPLICATED-RESPONSE-BLOCK` -- running; exact `1c51ddb6` has Runtime QA approval and awaits the Architect verdict. Behind it `RT-GRAFTED-SPINE-CONTROL-GRAPH` -- FILED `ready`, L, T1, the behavior-inert precursor that unparks the spine | Unpark is gated on `RT-GRAFTED-SPINE-CONTROL-GRAPH` landing on current main; then ONE final atomic D0+D1+D2 attempt, then park again. That one-attempt bound is NOT spent on the precursor, which has its own counter; counts stay 15/16. **The ruling's boundary symbols are CANDIDATE-SIDE:** `ReleaseEmissionSite`, `claim_word`, `PendingReleaseContextEdge`, `ReleasePlacementLedger` and `record_finished_function` occur ZERO times under `crates/` on main and exist only on rejected `7b03f258c`, so the precursor introduces its own function/body scoping type instead of reusing one. Do not kick it until its node file is on main. |
| L2 | language | Work the language backlog under the campaign root's standing surface-syntax direction; no live operator objective | `LANG-INSTANCE-SEARCH-SECOND-PATH` (`active`) -- closeout ROUTED exact `b54ec394f` at `evt_202d4ekg8wv6x`; 3 paths `+70/-6`, zero frame diff, full CI, lieutenant owns M4-M9 | `LANG-STANDARD-OP-GENERIC-CARRIER` -- `ready` on main, SCOPED to D0 plus AC-1/AC-2/AC-4 | On landing: flip SECOND-PATH to `closed`, compact the ring at that genuine new-WP boundary, then kick GENERIC-CARRIER. **AC-3's REPAIR branch is DEFERRED and a folded-in repair is a Steward stop** -- Row 2 needs a new `ElabCtx` field and its auditing gate `LANG-SEAL2-GATE-INCRATE-RELOCATION` is still `draft`. The `instance_search` residual is RECORDED UNREPAIRED, not closed as safe, and has no owner. `LANG-ATOM-START-CLASSIFICATION-CLOSURE` reads `active` against a 09-19 ruling that it is COMPLETE: status flip OWED, do not re-kick it. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-SCHEMA-LAWS` -- respin beyond dead `b89836796` is exact `41c76c0bc`, awaiting review; the dead cut was ATTRIBUTED at `evt_3rr9vct8pbpdn`: `cat_tier_e_decoder_import::decoder_checked_provider_and_schema_closure_is_exact` asserts left 28 right 26; the Schema provider additions grow the decoder+schema closure against a pinned exact inventory | `CAT-NONEMPTY-APPEND-HEAD-LEFT` (`ready` on main), then `CAT-PARSING-NUMERIC-LAWS` (`ready` on main) | **The respin must NOT simply bump 26 to 28.** An exact-inventory assertion re-pinned to whatever the tree now emits certifies nothing; the delta must be shown to be exactly the intended names. The precursor needs NO selector widening -- an attached `pub proof` travels with the function selector. DO NOT pick successors from the survey's one-line recommendations; MEASURE first. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.