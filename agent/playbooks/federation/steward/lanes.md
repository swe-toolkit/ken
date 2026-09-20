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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-PLANNER-KRET-GRAFTED-SPINE` (`active`) -- hard stop 15, ADVANCING. At non-candidate `7b03f258c` the adopted typed edge reaches its explicit `neither` arm: D0 expects source 328 / `Direct(ContinuationContext(0))`, but source 328's actual DIRECT edges are Spec4 to Context1 (func 57) and Spec5 to Context3 (func 58), while context 0 is emitted separately in func 62. Architect's precision correction, and keep the qualifier: this proves the DIRECT edge absent, NOT that no faithful multi-hop typed realization exists. Fail-closed missing-edge and the positive `517x1` cannot both hold UNDER THE DIRECT-EDGE REPRESENTATION -- that qualifier is the whole live question | `RT-DUPLICATED-RESPONSE-BLOCK` -- RE-KICKED `evt_5a11pfx6rhh07` in `thr_1d9q8kfr0ppbp`, already framed and already released, do NOT re-frame. Ring picked it up; reordered under COORDINATION 11.3 because the spine's block is long | Architect INVOKED the COORDINATION 1a research advisory at `evt_5m0f2mtah7ph5` (fifth trigger on this chain, so scoped for marginal value). Research owes the advisory in `thr_2pnjddgmsz6yx` mentioning Architect and Steward; that is the Architect's resume signal for the held D2 ruling. Symptom inventory entry 16: direct-edge adjacency used as if it were the whole semantic continuation relation. The spine resumes on that ruling and `RT-DUPLICATED-RESPONSE-BLOCK` yields to it -- do not stack them. |
| L2 | language | Work the language backlog under the campaign root's standing surface-syntax direction; no live operator objective | `LANG-INSTANCE-SEARCH-SECOND-PATH` (`active`) -- CLOSING. D0 withdrawn (refuted: the `d` disjunct is a real aliasing rule) and D1 reached its framed stop (identity material ABSENT from `ProjectionPurityCtx`). Closing on one capped cut: the two alias controls as tests plus three where-path comments, `evt_4wj4aa400zsh0` | `LANG-STANDARD-OP-GENERIC-CARRIER` -- RELEASED `ready` in this commit, SCOPED to D0 plus AC-1/AC-2/AC-4. Its `draft` tree gate was stale: A1 is merged, so `main` IS a qualifying D0 tree | **AC-3's REPAIR branch is DEFERRED and a folded-in repair is a Steward stop** -- Row 2 needs a new `ElabCtx` field and its auditing gate `LANG-SEAL2-GATE-INCRATE-RELOCATION` is still `draft`. I re-asked rather than inherited: the precondition has NOT dissolved. D0 may collapse Row 2 and moot it. The `instance_search` residual is RECORDED UNREPAIRED, not closed as safe, and has no owner. `LANG-ATOM-START-CLASSIFICATION-CLOSURE` reads `active` against a 09-19 ruling that it is COMPLETE: status flip OWED, do not re-kick it. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-SCHEMA-LAWS` -- respin `b89836796` M5-STOPPED by the lieutenant on a REAL CI red, `test shard 6/8` in PR #4094 run `35539510849`. That exact SHA will not be retried or merged | `CAT-NONEMPTY-APPEND-HEAD-LEFT` (`ready` on main), then `CAT-PARSING-NUMERIC-LAWS` (`ready` on main) | Foundation owes attribution then respin once the failed-step logs publish; the leader has it and is waiting on logs, not stalled. The precursor needs NO selector widening -- an attached `pub proof` travels with the function selector. AC-4's census delta is still a Steward stop if non-zero. DO NOT pick successors from the survey's one-line recommendations; MEASURE first. |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.