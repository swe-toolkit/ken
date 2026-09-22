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
- **2026-09-21, FLEET REDUCED TO ONE LANE:** "when L2 and L3 finish their
  current tasks, stop giving them new work. gracefully reduce the fleet to the
  single L1 lane." Neither had a task in flight; both stood down at
  `evt_59a2pvderx1ne`. SUPERSEDES the 2026-08-25 ruling above, retained because
  it defines what reinstatement restores.

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

**ONE lane: L1 Runtime.** L2 and L3 are stood down by the 2026-09-21 ruling.
No other ring starts without an operator lane change and no seat stands itself
back up. Finished accepted work still routes immediately.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-BRACKET-SETTLEMENT-PLANE` -- `ready`, M, T1, child 2 of 3, RELEASED 2026-09-22 on child 1's QA approval. Consume ONLY authenticated ports from child 1; partition marked settlement instances out of the RAW response occurrence/context inventory BEFORE the ordinary demand classifier; key every instance by its exact call/return context; the independent rebuild at plan close must be EXACTLY equal. `bracket_release_only_suffix` is GONE and must not return under any name. A first-match `find_map` on `effect_origin` collapses repeated lowering contexts -- parent edges and the ledger must NEVER be built on a first-match ID. Build on `4b4c8565c543c148039ea06ac798c7c91eda2ce8` | `RT-BRACKET-LOWERING-AND-D0-REFUTER` (child 3, `draft`, L) ASSEMBLES all three and routes the ONLY candidate. Children 1 and 2 are held reviewed inputs, NEVER independently landed | **CHILD 1 ACCEPTED** at `4b4c8565c543c148039ea06ac798c7c91eda2ce8` (`evt_4z54qy2pvmw0f`), 8 commits from `a4d12698a`; hard stop 1, symptom entry 1. Umbrella `RT-BRACKET-CONTROL-REGION-IR` stays `draft` and NOT STARTABLE at hard stop 0; its law still binds and THE REPRESENTATION IS NOT IN QUESTION. Evidence, never candidates: `81f222b7f012829cd9f8d0f3dc684410a9b2b9ee`, `64fd9e6abf63072b3222124b6df4c546b7a1d243`, `9d6a9547f09b7893cbbe1c8c2bcefd6eaefec14c`. Parked predecessor keeps 7/7, next fire at 9 |

**Stood down, and the one thing still live.** `CAT-CONFIGURATION-DECODER-LAWS`
is routed at exact `3c722a5ffefbc15c5fb7866417f71a13a4250f29` and belongs to
the lieutenant, not to a lane. If its fresh PR also registers zero workflow
runs, HOLD it for the operator -- that reading is then wrong and I carry it.
`CAT-PARSING-CURSOR-LAWS` stays framed and unkicked, and
`CORE-AUDIT-LABELS-ARE-ARTIFACT-IDENTITY` stays `draft` and unreleased.

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.