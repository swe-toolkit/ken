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
- **2026-09-23, L3 REINSTATED, L2 STAYS IDLE:** operator asked to bring up
  the other two lanes if the gpt-6-luna seats were working as expected, then
  answered the L2 objective fork "Leave L2 idle for now". Roster is L1 + L3.
  SUPERSEDES the 2026-09-21 one-lane reduction.

## Catalog proof direction -- SUSPENDED WITH L3, NOT WITHDRAWN

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
- **2026-09-21, how to read L1 difficulty:** clearing ignored tests "is
  fundamentally about closing gaps left during initial implementation. Because
  these were left it is expected that at least some of the underlying issues
  are difficult ... and this difficulty could be indicative of fundamental
  weakness in the implementation and lead to restructuring."

**RESTRUCTURING IS ADMISSIBLE; difficulty is the signal, not a reason to
restate the objective.** Zero rows cleared plus N structural findings is not
zero progress. But "could be indicative" is a prior: a minimal correct repair
still wins when the structure says so, and restructuring returns to me to size
as its own node. SUPERSEDES escalation `evt_5vkyp3sekgq0h`.

That means frames stay short, investigation is the first step of a repair
rather than a separate report node, and wording hygiene does not enter a lane.

## Authorized roster

**TWO lanes: L1 Runtime and L3 Foundation.** L2 is idle with no objective by
the 2026-09-23 ruling. No other ring starts without an operator lane change and
no seat stands itself back up. Finished accepted work still routes immediately.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PROCESS-EXIT-STATUS` -- `active`, S, T1, row `r2_cross_buffer_freeze_fails_closed_with_invalid_bounds`. Run the frame's `§3.3` probe, then the landed caller-consumption discriminator; repair in classify if a leak, else say the fail-closed was right. Touch only the row's `#[ignore]` in its test file | `RT-BRACKET-SETTLEMENT-PLANE` (child 2 of 3) resumes, or child 1 takes its `§5` STOP, on the operator's answer to `evt_4xax1j2qjg62v` | **BRACKET TREE HELD.** Child 1 at `4b4c8565c543c148039ea06ac798c7c91eda2ce8` and child 2 checkpoint `21c039918` are held reviewed inputs, NEVER independently landed or moved. The held tree turns `linked_public_escape_is_exact_closed` red; the only repair found needs a `ken-kernel` origin trace (TCB growth), owed to the operator at `evt_4xax1j2qjg62v`. Hard stop 0 |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PARSING-CURSOR-LAWS` -- `active`, L, T1, RESPIN of parked candidate `b62d091ecf652084875a069bc1988d3e4da8e4b6` onto current `origin/main`. Its precursor `LANG-IMPORT-IDENTITY-ESCAPE` landed (`18832c423`). The respin adds the explicit `IsTrue` import ruled at `evt_5f4qhhwqk1hye` and carries the test edit into the relocated `crates/ken-elaborator/src/r_layer_tests/cat_tier_d_cursor_import.rs` | Next of the seventeen survey follow-ons, chosen when this one is cut | foundation ring owes the respin SHA, then QA + Architect Decision on it. Editing the census sentinel is NOT authorized |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.