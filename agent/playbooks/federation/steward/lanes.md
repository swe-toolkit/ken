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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` (XL), `D0` in flight | `D2` placement, then `D1` producer closure, same node | UNBLOCKED and REPRICED. `§1a` stop-12 hold DISCHARGED: Architect `evt_695x0zxb802mb` adopts research's boundary -- **placement proves AT LEAST ONCE, a table proves AT MOST ONCE** -- and runtime's static clone map `evt_2yefr4rsaxbyd` returns **(a) yes, (b) no, (c) no**, refuting specialization collapse. 517's dispatch context exists statically and is not reached. So **`D0` is the at-most-once half ONLY and is EXPECTED to leave `:348` red at two dispatches against three** -- written into the frame in advance, because the symptom flips from too-many events to too-few and the next seat will read that red as a `D0` regression. `D0` gains a member-side compile error; the ruling's stated justification for it (would have caught `517 x0`) is REFUTED -- 517 has a site, so keep the check but do not expect it to red there. New `D2` owns placement, CFG/reach-side, acceptance 517 executing. Whether `D0` alone clears `:314` is UNMEASURED: the multiset was attributed on `:348`'s program. One sub-fork open with the Architect -- is 517's context CFG-unreachable (a static control exists) or reachable-but-never-entered (the only control is 517 executing)? `D2`'s controls wait on that; `D0` does not. Counters: stop 12, entry 13; next `§1a` 15, next `§1b` 15. |
| L2 | language | Build the membership operator surface on A1's landed resolver | `LANG-CORE-INSTANCE-HEAD-MATCH` (precursor) | `LANG-MEMBERSHIP-OPERATOR-SURFACE` (B, held on the precursor) | B is NOT buildable as framed: the identity-keyed resolver refuses every parameterized carrier, and all three mandated views are parameterized -- forced by `§3a`, since `Query` is a dictionary field fixed at instance registration. Architect ruled `evt_1bgpexfk5e79q`: horn (a) SCOPED. Precursor released and LANDED (`a672cab773a870fbbe16dffa92b6d5b5a4238670`); language-implementer is building `P1`+`P2` at that base. `P1` peel the application spine in `elab_standard_operator`; `P2` a core-side instance-head matcher for the `requested: None` case, **inside `resolve_instance_dictionary_inner`, not beside it**. Two bound constraints: match on `GlobalId` identity never spelling, and the carrier confirmation survives and widens to the full instantiated carrier. Coherence/orphan/re-export/`derive` are the closure's horizon, named and DEFERRED. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PARSING-CURSOR-LAWS` | Next of the seventeen survey follow-ons, chosen when this one is cut | Both `depends_on` edges are discharged: `CAT-NAT-ORDER-LAWS` (`571d216eb`) and `CAT-COLLECTIONS-NTH-LAWS` (`a86ee0ca5`, Adversary NO DEFECT, closed by M7 `35c533eff`). `CAT-PARSING-CURSOR-LAWS` released: inhabit `CursorLaws ArgCursor UInt8 ArgLocation arg_cursor_ops`, `L`/`T1`. Four settled-input coordinates were read off the pre-squash `D1` candidate and repaired to symbols at release — two had drifted onto `zero_left`/`right_leq for max`, real proofs that are the wrong ones. Stop condition: a general `List` fact beyond the landed pair is a report, never a widening into `Derived.ken.md`. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.