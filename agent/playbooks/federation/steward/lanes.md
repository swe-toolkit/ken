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
| L2 | language | Build the membership operator surface on A1's landed resolver | `LANG-CORE-INSTANCE-HEAD-MATCH` (precursor) | `LANG-MEMBERSHIP-OPERATOR-SURFACE` (B, held on the precursor) | B is NOT buildable as framed: the identity-keyed resolver refuses every parameterized carrier, and all three mandated views are parameterized -- forced by `§3a`, since `Query` is a dictionary field fixed at instance registration. Architect ruled `evt_1bgpexfk5e79q`: horn (a) SCOPED. Precursor released and LANDED (`a672cab773a870fbbe16dffa92b6d5b5a4238670`); language-implementer is building `P1`+`P2` at that base. `P1` peel the application spine in `elab_standard_operator`; `P2` a core-side instance-head matcher for the `requested: None` case, **inside `resolve_instance_dictionary_inner`, not beside it**. Two bound constraints: match on `GlobalId` identity never spelling, and the carrier confirmation survives and widens to the full instantiated carrier. Coherence/orphan/re-export/`derive` are the closure's horizon, named and DEFERRED. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PARSING-CURSOR-LAWS` | Next of the seventeen survey follow-ons, chosen when this one is cut | Both `depends_on` edges are discharged: `CAT-NAT-ORDER-LAWS` (`571d216eb`) and `CAT-COLLECTIONS-NTH-LAWS` (`a86ee0ca5`, Adversary NO DEFECT, closed by M7 `35c533eff`). `CAT-PARSING-CURSOR-LAWS` candidate `b62d091ec` is CI-RED and unmerged; publisher correctly aborted. `catalog_ambient_passthrough_migration_census` (`lang_mod_strict_resolution_d0.rs:369`) fired because Cursor gained ambient `IsTrue`. RULED `evt_5f4qhhwqk1hye`: that sentinel is **directional, not a pin** — its expected set records migration debt to be retired, so a growing set is a regression. Repair is an explicit `IsTrue` import in `Cursor.ken.md` (`pub fn IsTrue` at `LawfulClasses.ken.md:54`; `Order.ken.md:37` imports it explicitly, uses it 18x, and stays out of the census; `IsTrue` is in **zero** census entries catalog-wide). Editing the sentinel is NOT authorized; if the census still differs after the import my read is refuted and the sentinel edit becomes correct. Ring unblocked on the import. `AC-3c` replaces the `AC-3a`/`AC-3b` enumerations with a mirroring-vs-directional predicate — three stops on this node were all consumer-view surfaces a list did not name. Z3 job stays with the lieutenant as transient. Next node `CAT-PARSING-DECODER-LAWS` is framed and HELD on contention, not dependency: `b62d091ec` touches `cat_tier_d_cursor_import.rs`, the file its `AC-4` mandates editing. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.