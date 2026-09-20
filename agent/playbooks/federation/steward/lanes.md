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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE`, `D0`+`D1` ATOMIC, fresh branch `wp/RT-PLANNER-KRET-GRAFTED-SPINE-D0-D1` at base `a5833198a` | `D2` placement, same node | RULED 2026-09-20 at `evt_3h5hfr8sa771t`: `D0` LOSES STANDALONE RELEASE and lands atomically with `D1`; `D0` is NOT widened and the fail-closed default is NOT softened. `D0`'s refusal is its designed alarm firing on a `Predeclared` producer family it does not reconcile -- absorbing that one family is the outcome `evt_64d51mf464fmw` was written to prevent, and `D1` is closure over producers, not an extension to the families that announced themselves. Releasability was an empirical prediction the measurement refuted, so this is sequencing under `steward.md §4a`, not a scope change. Prefix regressions (`-1` at `3452d1371`; Deferred multiplicity in `3452d1371..490424db2`) are IN-NODE repair, already authorized, but classify the two independently BEFORE repair: ordinary defect -> repair in place; unmasked placement defect -> `D2` joins the atomic candidate; neither -> report for ruling. `c1195196` is SUPERSEDED, not owed, and will not be re-routed; the next candidate is a fresh authorization. Two-way wait `evt_4hs8mpjszaq3d` DISCHARGED -- its object is dead, not its question answered; the run's remaining shard failures stay unclassified and are re-measured by the new candidate's CI. Counters: stop 13, entry 13. |
| L2 | language | Build the membership operator surface on A1's landed resolver | `LANG-MEMBERSHIP-OPERATOR-SURFACE` (B) -- candidate `dce816da6` ROUTED at `evt_3ny5sra81yp8q`, queued behind the Compare publisher; lieutenant owns M4-M9 | next deliverable of the same objective | Ring is clear and owes nothing. The `not merge-ready` marker on `697351bad` was STALE: it named my own ruling `evt_6hcan5dx5pyrr`, and the tip implements it -- census diff purely ADDITIVE, 18 added / 0 removed, `Core.Classes.Membership` added with exactly the authorized vector `[And, Bottom, Equal, Prop, Proved, and_fst, and_intro, and_snd]`, `expected_clean` unchanged at four. Zero deletions is what makes that conclusive against the moved-OTHER-row stop. On a CI red the respin is Language's and a new SHA is a fresh authorization, not a retry. REFUSED and not a node: migrating `LawfulClasses` or its convenience closure. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PROCESS-ARGUMENTS-LAWS` -- RELEASED 2026-09-20, `docs/program/issues/CAT-PROCESS-ARGUMENTS-LAWS.md`, base `ecf185076`, `S`/`T1` | next of the seventeen survey follow-ons | `CAT-COMPARE-LAWS` LANDED at `ecf185076`; lieutenant owns its M7 closeout and M8 Adversary notification. The successor gives `argument_slice_location` the two-way characterization its package's `round_trip` already models: selected as the smallest genuine gap among the seventeen whose precursors are ALL landed -- `nth::some_below_length` and `nth::at_or_beyond_is_none` arrived with `CAT-COLLECTIONS-NTH-LAWS`, and `bytes_nat_length` is ordinary checked Ken, not a primitive. `ArgLocation`'s projections are PRIVATE to `Cursor.ken.md`, so field identity goes through `MkArgLocation` and no export is added. It ADDS NO CATALOG MODULE, so its census risk is a MOVED ROW: any row other than `Capability.Process.Arguments` moving is a Steward stop. `CAT-PARSING-CURSOR-LAWS` stays PARKED WHOLE at `6065b4993` pending the non-local same-`GlobalId` resolver precursor; re-baselining the sentinel stays REFUSED. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.