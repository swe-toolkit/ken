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
  fork and directs that the measurement follow the repair. **(g) is landed and
  the remeasure is not** — so this ruling is L2's live objective and L2 is not
  waiting on a new one.

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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE`, `D0`+`D1`+`D2` ATOMIC, branch `wp/RT-PLANNER-KRET-GRAFTED-SPINE-D0-D1`, D2 repair in progress, WIP checkpoint `3d1bf3952` on D2a control `c987c3ef0` | `D2` is IN, not next | D2a PRE-REPAIR READINGS COMPLETE at `evt_498rdcf40trtw`: 517 and 1092 both red, and the reachable-wrong-scope mutation reds on 528 NOT 517, which is what separates ownership-relative placement from global reachability. The policed before-repair order is SATISFIED; repair is authorized to proceed. Scope unchanged: D0+D1+D2 atomic, no candidate cut until all three are green together, FORCED not preferred because D0's fail-closed refusal is live on this member. It BECOMES a recut only if construction shows D0+D1 green with the member unplaced. This seat runs deep past its auto-compact threshold (161%); the WIP checkpoint is why a compaction now costs reasoning, not work. Counters: stop 13, entry 14; neither trigger fires until 15. |
| L2 | language | Remeasure the descent stack detector on the repaired descent -- operator "g then remeasure", 2026-09-19. DISCHARGED: landed `6a36cfbdd`, blob-verified, reservation now `49_152` | `LANG-ATOM-START-CLASSIFICATION-CLOSURE` -- RESUMING (`L`/`T1`, released 2026-09-17). NOT a fresh node: increments A, B1, B2a, B2b, B2c are landed. language-leader names the next increment; I do not decompose it | none owed -- ATOM-START is L and mid-flight; frame no successor until it is near closing | Anchors are perishable: the frame's measurements sit at `6acd40705` and AC-0 must be RE-RUN at the implementer's own base. I amended the issue file's LANDED INCREMENTS ledger, which had stopped at `20ddc558f` and omitted all five B-series squashes -- AC-0 tells the next implementer to name the accounting increment FROM THAT TABLE, so a current base would have reported this node's own success as a premise failure. The 2026-09-18 UNLESS amendment held; its data went stale under it. Three `wp/LANG-ATOM-START-*` branch names are STALE and read as unlanded work; the CLOSURE one points at `6ce8aa0e9`, a FRAME-TAX commit. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PROPERTY-LAWS` -- ROUTED `aca4914217f08d13659e246c68c6d248e46ff3fe` at `evt_4qj3c37f07xr0`, Decision `dec_1mch6rrdqjmz6`, FULL CI, one path `+362/-10`; M4-M9 are the lieutenant's | `CAT-SCHEMA-LAWS` -- FRAMED `ready` (`S`/`T1`); release and kick when Property lands. 11 of 17 survey follow-ons then remain unframed | M3 HIT on the routed SHA: Librarian notice owed at CLOSEOUT for `library/SOURCE-ATTESTATIONS:44` and `:116`; the attested blob matched NEITHER base nor candidate, so that row was stale BEFORE this work -- accepted drift, not this candidate's. DO NOT pick a successor from the survey's one-line recommendations -- MEASURE the package first: `CAT-CONSOLE-TEXT-LAWS` and `CAT-JSON-LAWS` were both measured and REJECTED (reasons recorded in CAT-SCHEMA-LAWS `origin`). STANDING LOW on landed `Arguments.ken.md`: `refuses_missing_argument` is weaker than unconditional, and the ORIGIN IS MY OWN FRAME -- CAT-PROCESS AC-2 forced hypotheses the law does not need. Not a node (§4c); fix it if a future node touches that file. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.