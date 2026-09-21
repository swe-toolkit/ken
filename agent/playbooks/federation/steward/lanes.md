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

## Live objective direction

- **L2 has no live operator objective** (Route B and "g then remeasure" both
  discharged); see L2's row.

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

Three lanes, in priority order:

1. Runtime
2. Language
3. Foundation

Finished accepted work still routes immediately, regardless of lane. No other
ring starts without an operator lane change.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-BRACKET-RELEASE-ORDER-PARITY` -- `ready`, M, T1. `D0c`+`D0c-2` both CLOSED: 6 reaching depth-2 native nests WRONG, 2 reaching depth-3 CORRECT; mode, inner-kind, outer-kind, homogeneity, combinator and read-vs-write all dead. **ARCHITECT RULED `evt_4t14zmba83hjm`: depth is the SELECTOR, the cause is planner classification** -- a bounded bracket-settlement continuation left on the unowned Deferred forward-`Ret` route. `D1a` AUTHORIZED only as two required arms (per-group exclusive eligibility in `static_response_phase_b_split`; `release_only_suffix` third class in `bounded_deferred_response_suffix`), each mutation-proved by its own control. `D0a` AFFIRMED `evt_17kyxq7q5v8ar` | None framed. 30 `ready` RT nodes exist but `status: ready` is a claim about a node, NOT evidence about the tree -- run `git log --oneline origin/main --grep=<NODE-ID>` before selecting. Do NOT frame a new one | Recut carrying the envelope must LAND before runtime-leader releases `D1a`; the Architect holds its inventory child until then. Objective reachability ANSWERED by the operator 2026-09-21 (see Runtime direction); `evt_5vkyp3sekgq0h` superseded. `RT-PLANNER-KRET-GRAFTED-SPINE` PARKED `draft` by stop-16 `evt_3h3dqjet1wx3q`. |
| L2 | language | **Language surface. NO live operator objective** -- Route B and "g then remeasure" both discharged | **NONE. `LANG-SEAL2-GATE-INCRATE-RELOCATION` LANDED `c2eca6e41`** (5 blobs verified by identity). The lane is DARK | `CORE-AUDIT-LABELS-ARE-ARTIFACT-IDENTITY` -- `draft`, the only live candidate. Its deferral premise HAS dissolved (`RT-DESCENT-RETIRE` merged; all 16 `V3-FO-*` merged/closed) | **Operator fork OPEN at `evt_1py9jhvehv77c`.** My earlier D0-only default is WITHDRAWN -- D0 alone is a census node, which §1 and the watchdog both forbid, and the node itself says D1 is not the ring's to decide. What makes it worth doing is that settling it lifts the Architect's standing no-new-citation-bearing-labels prohibition (`evt_2q0bm3ez5aczd`). Recommended: release D0+D1, D2 only if D1 rules for exclusion. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-CONFIGURATION-DECODER-LAWS` -- ROUTED `3c722a5ffefbc15c5fb7866417f71a13a4250f29`, Decision `dec_46thvhz2xdjbd`, 2 paths `+1031/-26` off base `7e3db6f55`. The lieutenant owns M4-M9 | `CAT-PARSING-CURSOR-LAWS` -- ALREADY FRAMED `ready`, L, T1; all three `depends_on` are `merged`. Kick when DECODER closes. Do NOT frame a successor | The Architect ordering precondition is WITHDRAWN and the publisher blocker is owned. PR #4127 registered zero workflow runs; I authorized close-and-LEAVE-CLOSED at `evt_1sc9f2p4zed1c` so the publisher takes its `else` branch and cuts a FRESH PR on the identical head -- `scripted-pr-automerge.sh:294-299` reuses only an OPEN PR, so close/reopen had preserved the broken object. Head unchanged, approvals valid. FALSIFIER: if the fresh PR also fails to register, my reading is wrong and it becomes a real operator escalation I carry. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.