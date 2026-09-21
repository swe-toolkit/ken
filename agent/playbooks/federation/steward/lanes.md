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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PX7F-LINKED-PUBLIC-ROWS` -- `ready`, M, T1, gate none, frame landed; released, and the landing it was waiting on cleared at `a17235098` | None framed. Select the next ignored-row node from the deep `ready` RT backlog when PX7F lands; do NOT frame a new one | `RT-PLANNER-KRET-GRAFTED-SPINE` is PARKED `draft` by Architect stop-16 disposition `evt_3h3dqjet1wx3q`, landed `a17235098`. Counts are 16 hard stops / 17 symptom entries; the next Research trigger is 18 and the next predicate check is 18, so neither fires. Reopening requires a separately framed architecture decision grounded on then-current `main`, NOT a successor search or another attempt. `4f6b2a454786dd66eb2a1eb9681913e1e4f69b9d` is evidence only and is never a candidate. |
| L2 | language | Language surface. The operator's 2026-08-25 "unblock foundation with module/import" direction is DISCHARGED -- all 13 `LANG-MOD-*` members AND `CAT-GCD-REFACTOR` are `merged`; the root `LANG-MODULE-IMPORT-SYSTEM` sits `draft` only because a campaign root is never itself released | `LANG-SEAL2-GATE-INCRATE-RELOCATION` -- promoted `ready` and framed in this commit. M, T2, gate none, `depends_on` empty. Architect review NOT owed; design fully ruled at `evt_6b39fyc17xzm1`, `evt_x1b90s36dtc2`, `evt_1se8wycskre80` | `CORE-AUDIT-LABELS-ARE-ARTIFACT-IDENTITY` -- `draft`, now the only pool member. Re-measure its premise before promoting; do NOT frame a new node | `LANG-R-LAYER-EXPORT-RETRACTION` LANDED `8fd30c13a`, 38/38 blobs verified; the lieutenant holds its M7 status closeout in a bounded batch, which is M9 and not mine. SEAL2 released on its own recorded condition -- it was `draft` by PRIORITY only and never technically blocked. Premise re-measured at `a17235098` and it HOLDS: `ElabEnv` carries 15 `pub` fields and exactly one `pub(crate)` field, `standard_operators` at `lib.rs:173`, crate-internal BY CONTRACT and permanent, so the walk's `..` cannot be retired from an integration test. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-CONFIGURATION-DECODER-LAWS` -- `ready`; respin `3c722a5ffefbc15c5fb7866417f71a13a4250f29` QA-approved at `evt_2s2m16ykx8smv` but NOT merge-authorized by me. M, T1 | `CAT-PARSING-CURSOR-LAWS` -- ALREADY FRAMED `ready`, L, T1; all three `depends_on` are `merged`. Kick when DECODER closes. Do NOT frame a successor -- one exists | **Blocked on an Architect ordering precondition the sanctioned tooling cannot satisfy.** Change request `evt_5e9zs7zaatyvf` requires remote exact-SHA CI history BEFORE a renewed verdict, but `scripted-pr-automerge.sh` takes six flags with no establish-only mode and pushes, opens the PR, polls checks and merges on green in one sequence (`:292`, `:298`, `:823`, `:869`); only a RED run exits first (`:841`). Fork put to the Architect at `evt_5wvy943z7cx72`: (a) verdict on local exact-SHA evidence, CI still gating the merge inside M5, or (b) hold for an operator-authorized establish-only mode. Respin against dead `6b900762` measured line-multiset IDENTICAL, 1148 lines both sides, one file -- a pure permutation, no content added or edited. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.