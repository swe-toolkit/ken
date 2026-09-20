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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE`, `D0`+`D1`+`D2` ATOMIC on `wp/RT-PLANNER-KRET-GRAFTED-SPINE-D0-D1`; implementer validating exact `e6785dd0f` | none framed; frame when D2b clears | D2b course correction from the Architect at `evt_pabbw61zf2yz`, handed to the implementer at `evt_5h08j7zb87cw2`: the finished-CFG join must require `ReleaseBracketExit::source_origin` to equal `ReleasePlacementExpectation::producer_call_origin` before edge-dispatch bypass, with a route-specific wrong-origin control. Scope and hard-stop count 13 unchanged; the audit clock reset at that correction. This seat runs deep past its auto-compact threshold by design, and MODELS.md warns that a Runtime WP running for hours is not a stall. |
| L2 | language | Work the language backlog under the campaign root's standing "surface-syntax items next" direction -- module/import and the V3-FO chain are both fully merged, so no live operator objective remains and I selected from that standing direction rather than escalating | `LANG-INSTANCE-REGISTRY-IDENTITY-KEY` -- kicked `evt_1k63fxtcgwwfd`, base `dcbb9648f`, branch `a444c2fcb`; D0-gated, and the implementer confirmed all four framed coordinates exact at `evt_2tpap3nethd9p` | none framed; frame when D0 reports | STALE ON MAIN, needs a measurement not a status flip: `LANG-ATOM-START-CLASSIFICATION-CLOSURE` reads `active` with no seat on it, but its own file says AC-0 REMAINS OWED. Do not flip it from the channel note alone. STANDING LOW on landed `map_build_acceptance.rs:39`: `49_152` is the TOP detecting value and a candidate-side SIGABRT kills the whole binary -- recorded in the DESCENT issue, origin is my own AC-1, not a node. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-SCHEMA-LAWS` -- HARD STOP at `evt_57pppnzv9ahfk` on the frame's own `pub proof` stop condition, AMENDED with a D0 that marks `schema_validate_fields` `pub fn`; ring resuming from the released base, no candidate was cut and the source blob is byte-exact | `CAT-PARSING-NUMERIC-LAWS` -- framed `ready` on main, kick when SCHEMA lands | The broken premise was MINE: the frame asked for public attached proofs without establishing the subject could carry one. Every `pub proof` in the catalog has a `pub fn` subject, zero exceptions, and `Data/Numeric/Nat/Order.ken.md` is the precedent for `pub fn` + `export` + `pub proof`. AC-4 now MEASURES the surface delta rather than asserting it. DO NOT pick a successor from the survey's one-line recommendations -- MEASURE first: `CAT-CONSOLE-TEXT-LAWS` and `CAT-JSON-LAWS` were measured and REJECTED. `CAT-CONFIGURATION-DECODER-LAWS` follows NUMERIC-LAWS and needs SCHEMA landed. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.