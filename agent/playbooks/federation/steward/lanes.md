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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` (RECUT, M -> XL) | `D1` producer closure, inside the same node | Nine stops established the defect is not at a site. Census `evt_5dbc20bwkmym1`: six dispatches, ONE token, residual zero -- under-release DROPPED, over-emission is the defect. Recut framed at `§0`: mint a `ReleaseObligationId` at bracket planning, carry it demand -> row -> emission claim -> dispatch claim, reconcile the two observed families in one table, and make a claim with no id a COMPILE ERROR. `D0` releasable. Cut is one WP, not precursor+consumers (substrate alone is unobservable) and not split by family (closure is over producers). Counters separated: `§1a` at stop 12, `§1b` at entry 12. |
| L2 | language | Build the membership operator surface on A1's landed resolver | `LANG-MEMBERSHIP-OPERATOR-SURFACE` (B) | Successor chosen when B is cut | A1 LANDED `e2e40e2b4` (Adversary NO DEFECT), discharging the "g then remeasure" ruling: option (g) held and the unchanged detector passed, so the boundary never moved. B RELEASED — all four deps landed and `§5a`'s four items pinned against A1's tree, not its approval posts: home `Core.Operators.Standard`, `StandardOperatorRole` (ALL=5, `∈` absent), `BINDING_BACKED`=4 with `expected_shape`/`shape_matches`, entry point `certify_roles`. `D0-1` is a compile error via `#![deny(private_interfaces)]`, so B's duty is not to weaken it; `D0-2` is largely answered in A1's code, leaving only whether `d.Query` is representable where the check runs. Adding a variant is a compiler-generated checklist — a catch-all arm anywhere is a hard stop. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PARSING-CURSOR-LAWS` | Next of the seventeen survey follow-ons, chosen when this one is cut | Both `depends_on` edges are discharged: `CAT-NAT-ORDER-LAWS` (`571d216eb`) and `CAT-COLLECTIONS-NTH-LAWS` (`a86ee0ca5`, Adversary NO DEFECT, closed by M7 `35c533eff`). `CAT-PARSING-CURSOR-LAWS` released: inhabit `CursorLaws ArgCursor UInt8 ArgLocation arg_cursor_ops`, `L`/`T1`. Four settled-input coordinates were read off the pre-squash `D1` candidate and repaired to symbols at release — two had drifted onto `zero_left`/`right_leq for max`, real proofs that are the wrong ones. Stop condition: a general `List` fact beyond the landed pair is a report, never a widening into `Derived.ken.md`. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.