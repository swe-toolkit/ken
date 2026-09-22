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
| L1 | runtime | Clear the selected ignored runtime rows | NONE. `RT-DELAYED-OWNER-CLASSIFICATION-TWO-PASS` is PARKED at `draft` by terminal ruling `evt_4zk1ckv4czk4e`, hard stop 7. Its `D1a` census discharged and its `N` filter is measured correct, but the commit law `P == A intersect L` plus strict coverage is FALSE: all four two-operation Mapping programs committed clean and then trapped native `-1`. Call-seat presence and graph closure are not semantic authority for the response-owner route. Ring stood down; no implementation turn owed | NONE, and I may not frame one. Reopening is a separately framed architecture decision on current `main` and the REPRESENTATION IS THE OPERATOR'S TO CHOOSE -- a proof-bearing response-owner route carrying an observational-equivalence certificate, or bracket-settlement ownership and order made explicit in the control IR. Not an amendment to `D1` | **BLOCKED ON THE OPERATOR at `evt_2rkr96rxpnbex`** -- lane idle pending their call: reopen under a new representation node, or redirect L1 to a different ignored runtime row. The whole bracket family is unavailable meanwhile: no Arm-B-only candidate exists, px8ta depth 3 and both `D2b` controls depend on Arm A, the order-excluding helpers stay, and `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED` stays open. `RT-BRACKET-RELEASE-ORDER-PARITY` remains UNRESOLVED at `draft`, neither closed nor merged. Evidence only, never candidates, never reclaim those worktrees: `64fd9e6abf63072b3222124b6df4c546b7a1d243` and `9d6a9547f09b7893cbbe1c8c2bcefd6eaefec14c`. Counts 7/7, neither trigger fired, next at 9 |

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