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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` (XL), `D0` in flight | `D2` placement, then `D1` producer closure, same node | UNBLOCKED and REPRICED. `§1a` stop-12 hold DISCHARGED: Architect `evt_695x0zxb802mb` adopts research's boundary -- **placement proves AT LEAST ONCE, a table proves AT MOST ONCE** -- and runtime's static clone map `evt_2yefr4rsaxbyd` returns **(a) yes, (b) no, (c) no**, refuting specialization collapse. 517's dispatch context exists statically and is not reached. So **`D0` is the at-most-once half ONLY and is EXPECTED to leave `:348` red at two dispatches against three** -- written into the frame in advance, because the symptom flips from too-many events to too-few and the next seat will read that red as a `D0` regression. `D0` gains a member-side compile error; the ruling's stated justification for it (would have caught `517 x0`) is REFUTED -- 517 has a site, so keep the check but do not expect it to red there. New `D2` owns placement, CFG/reach-side. Whether `D0` alone clears `:314` is UNMEASURED: the multiset was attributed on `:348`'s program. `D2`'s sub-fork is RULED (`evt_321kgnhds826z`): both horns share the predicate **a member's release site is on an outgoing edge of its own bracket**, so the control is horn-independent and is a JOIN of `D0`'s table against the emitted CFG -- not a reachability walk, which answers about the program rather than the member. `D2` = control, then the (i)/(ii) placement MEASUREMENT (not a fork), then the repair it prices. Mandatory falsifier: the control must RED at `31b9ab4800484cf8dc73839b6145484fd3206142` on member 517 naming body 586; if it does not, the control is wrong, not 517. Counters: stop 12, entry 13; next `§1a` 15, next `§1b` 15. |
| L2 | language | Build the membership operator surface on A1's landed resolver | `LANG-IMPORT-IDENTITY-ESCAPE` (S) -- take this FIRST, it unblocks L3 | `LANG-MEMBERSHIP-OPERATOR-SURFACE` (B), released and ready behind it | **`LANG-IMPORT-IDENTITY-ESCAPE` is cut ahead of B on Architect ruling `evt_56ngqsbcmtavc`.** `bind_import` (`crates/ken-elaborator/src/modules.rs:256`) arm 1 fires on mere membership in `locals` and compares nothing, while arm 2 already has the idempotence escape -- so a module may not import a name it also reaches ambiently even at one identity. One clause, keyed on resolved `GlobalId` never spelling (the spellings differ by construction, so a string escape could never fire). `AC-2` is the falsifier: distinct ids must STILL refuse, and an escape written too wide passes `AC-1` and fails it. It is S, it is L2's own surface, and it is the only thing standing between L3 and a fully approved `+670/-3` proof candidate -- that is why it precedes B rather than queues behind it. B itself is released and pinned at `64d8aa755`; nothing about it is waiting on anyone. BOTH of B's holds discharged. Precursor `LANG-CORE-INSTANCE-HEAD-MATCH` landed as squash `564e9e2cd6096fb9496569214b95dfcf946fb391` (verified by blob on both paths, not by subject); Adversary NO DEFECT on the carrier-confirmation widening and dispatcher uniqueness. B released at base `64d8aa755f505098c34a3caa8293922d9f5b0698`, `L`/`T1`, build from the **frame** `docs/program/wp/LANG-MEMBERSHIP-OPERATOR-SURFACE.md` `§5`/`§6` -- the issue node's `D1`-`D3` + `AC-1`..`AC-6` are the pre-2026-09-13 ASCII-role scope and are marked SUPERSEDED in place. `§5` item 6 now means the resolver **as extended by `P2` inside `resolve_instance_dictionary_inner`**; `GlobalId` identity never spelling, and the carrier confirmation widens to the full instantiated carrier. Contention re-measured: foundation is LIVE on `catalog/` but file-disjoint (Parsing vs `Core/Classes/`). Frame now warns that any new catalog module reds `catalog_ambient_passthrough_migration_census` -- aim for `clean`, import `IsTrue` explicitly (`§5` item 4 uses it), and a module that cannot reach `clean` is a Steward stop. Coherence/orphan/re-export/`derive` remain the closure's horizon, DEFERRED. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PARSING-DECODER-LAWS` -- RELEASED 2026-09-20, work this now | Next of the seventeen survey follow-ons | **`CAT-PARSING-CURSOR-LAWS` is PARKED WHOLE behind `LANG-IMPORT-IDENTITY-ESCAPE`, and the fork I was handed had only one live horn.** The Architect asked whether `AC-3b`'s explicit-import clause defers so the rest lands; it cannot -- **deferring the AC does not remove Cursor's ambient `IsTrue`**, so the directional census reds either way. Candidate `b62d091ec` stays byte-clean, keeps all three gate passes, and re-runs CI when the precursor lands. Re-baselining the sentinel is REFUSED (Architect `evt_56ngqsbcmtavc`, Steward `evt_5f4qhhwqk1hye`): it measures a TRUE debt and the resolver is what prevents discharging it. The alias workaround is rejected; the thirteen proof references are not to be rewritten. So L3 does NOT idle -- `CAT-PARSING-DECODER-LAWS` is released and its contention hold is DISCHARGED: Cursor is parked, so Decoder reaches `cat_tier_d_cursor_import.rs` first and Cursor rebases onto it. Bridge lemmas are now KNOWN-ABSENT from main rather than pending, and `Decoder.ken.md` uses `IsTrue` zero times so it does not inherit the blocker. `AC-3c` (landed `533a2b2b0`) replaces the `AC-3a`/`AC-3b` enumerations with a mirroring-vs-directional predicate. Z3 job stays with the lieutenant as transient. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.