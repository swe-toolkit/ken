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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` (`active`), D0+D1+D2 ATOMIC -- the Architect measured the split condition FALSE twice; at `b0041c959` D0+D1 are non-green precisely BECAUSE D2 is unplaced. D2b is now an INTER-FUNCTION typed context-edge join per ruling `evt_7wrm5pencaw2a`; frame amended in place, `wp/` section 8 | `RT-DUPLICATED-RESPONSE-BLOCK` -- already framed AND already released (kick `evt_20yrkxxjt5t0w`), then PREEMPTED. 4 ignored rows; scope is one predicate, `repeated_producer` at `responses.rs:3326`. Re-kick it; do NOT re-frame it | Same-function reroute is DEAD by measurement: source 328 emits in funcs 57/58, member 1092 dispatches in 62, and a Cranelift `Inst` is function-local. Section 1b predicate, entries 13-15: a projection that is NOT INJECTIVE over emitted member edges is being used as placement identity. Any proposal nominating a single projection as identity is entry 16 and returns to the Architect. My second WIP audit blamed the cut; that was wrong and section 8 records why. |
| L2 | language | Work the language backlog under the campaign root's standing "surface-syntax items next" direction; no live operator objective remains | `LANG-INSTANCE-SEARCH-SECOND-PATH` (`active`) -- kicked `evt_r13d2eh4az8s`, base `e4df15c48`, thread `thr_6hs7wr3438wyb`. `D0` WITHDRAWN: my subsumption claim was refuted at `evt_zh6jg8q0693f`; deleting the `name == "d"` disjunct admits an EFFECT ESCAPE because `d` is a registered sole-constraint dictionary alias, not an ordinary global | none framed; frame when `D1` reports | `D1` opens with a MEASUREMENT I must not prejudge: does the landed sibling `83f30f5e8` route these two `instance_search` calls through `confirm_instance_dictionary_carrier`, given `projected_instance_id` calls `ClassEnv::instance_search` directly? The answer may close this node. STOP if identity keying needs kernel state in `projected_instance_id` -- the Architect forbade that and the amendment is mine. STALE ON MAIN, needs a measurement not a status flip: `LANG-ATOM-START-CLASSIFICATION-CLOSURE` reads `active` with no seat but its file says AC-0 REMAINS OWED. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-SCHEMA-LAWS` -- RESCOPED IN PLACE at its second boundary to `D0` plus positional coverage plus no-masking. `D0` (`pub fn`) SUCCEEDED, so the amendment held; ring committing that cut and routing to QA | `CAT-NONEMPTY-APPEND-HEAD-LEFT` (framed `ready` in this commit), then `CAT-PARSING-NUMERIC-LAWS` (framed `ready` on main) | The first-rejection law is DEFERRED behind the precursor, not weakened: its `nonempty_head`/left-append equality is neither definitional nor destructible under the four authorized selectors. The precursor needs NO selector widening -- an attached `pub proof` travels with the function selector, measured from `list_append::assoc` consumed inside `NonEmpty.ken.md`. AC-4's census delta is still a Steward stop if non-zero. DO NOT pick successors from the survey's one-line recommendations; MEASURE first. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.