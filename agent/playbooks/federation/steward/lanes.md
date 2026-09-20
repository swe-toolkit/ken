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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` (XL), `D0` in flight | `D2` placement, then `D1` producer closure, same node | BLOCKED on CI attribution for exact `c1195196`. Publisher M5-stopped on 17 completed full-CI failures across shards 1-6, 8 and native-slow. Two-way wait broken at `evt_4hs8mpjszaq3d`: the LIEUTENANT owes the failing job names plus first failing test per failure, AND those same jobs' conclusions on base `051039fa0` read as JOB conclusions, since a skipped job is not a green. RUNTIME owes attribution only after that arrives and its hold until then is correct, not a stall. I owe the fresh-SHA route if D0 is causal. If the base is red on those jobs the red becomes L1's task ahead of any respin, per the standing operator ruling that a persistent red is fixed rather than merged past. Unchanged: `D0` is the at-most-once half ONLY and is EXPECTED to leave `:348` red at two dispatches against three; `D2` owns placement with the ruled horn-independent control (`evt_321kgnhds826z`), whose mandatory falsifier is a RED at `31b9ab4800484cf8dc73839b6145484fd3206142` on member 517 naming body 586. Counters: stop 12, entry 13; next `§1a` 15, next `§1b` 15. |
| L2 | language | Build the membership operator surface on A1's landed resolver | `LANG-MEMBERSHIP-OPERATOR-SURFACE` (B), base `64d8aa755` | next deliverable of the same objective | UNBLOCKED by `evt_6hcan5dx5pyrr`. The frame's `§8` `clean` destination is WITHDRAWN because its exemplar is refuted: `Data.Numeric.Nat.Order` names `IsTrue` in an import and is IN the ambient census carrying the same eight names, while `expected_clean` holds four modules none of which imports from `LawfulClasses`. `clean` was unreachable for this module by construction, so the ring did not fail an AC. The criterion is now CONTRIBUTES NO NAME OF ITS OWN, discharged by the empty two-way set difference measured at `697351badbb6e6ba0a9d0d634e9bcdd2d1067080`. Authorized: add `Core.Classes.Membership` to `expected` at its path-sorted position; `expected_clean` stays at four. A moved OTHER row is a stop, not a mirroring update. REFUSED and not a node: migrating `LawfulClasses` or its convenience closure. Ring resumes at mutation proof and candidate commit. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-COMPARE-LAWS` -- RELEASED 2026-09-20, `docs/program/issues/CAT-COMPARE-LAWS.md`, base `051039fa0`, `L`/`T1` | next of the seventeen survey follow-ons | `CAT-PARSING-CURSOR-LAWS` stays PARKED WHOLE at `6065b4993b42ada6dde03a6c07cd255ca0b685bc` on base `051039fa0`, pending the non-local same-`GlobalId` resolver precursor; re-baselining the sentinel stays REFUSED and the thirteen proof references are not to be rewritten. `CAT-COMPARE-LAWS` is four attached proofs on `list_compare` plus the `list_eq` bridge in `Core/Logic/Compare.ken.md`, mirroring the `pair_compare` `eq`/`eq_cases` pair already in that file. It ADDS NO CATALOG MODULE, so its census risk is a MOVED ROW rather than a new one: an ambient name outside `[And, Bottom, Equal, Prop, Proved, and_fst, and_intro, and_snd]` propagates through `LawfulClasses` into `Nat.Order` and into L2's in-flight `Membership` row, and is a Steward stop. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.