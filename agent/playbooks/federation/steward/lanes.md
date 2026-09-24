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
- **2026-09-23:** L3 reinstated beside L1. **2026-09-24:** L2 reactivated
  for the load-order fix. Roster is L1 + L2 + L3.

## Review of doc-only merges

- **2026-09-23:** "doc only merges may have reviewers. However, if it is a
  doc change that I directed you to make a review is not necessary."
  Narrows `evt_12a47d49frwjd`: a route cites the operator direction it
  carries out; other doc-only changes get their domain's reviewer.

## Catalog proof direction -- SUSPENDED WITH L3, NOT WITHDRAWN

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
as its own node. SUPERSEDES escalation `evt_5vkyp3sekgq0h`. Frames stay short;
investigation opens a repair, never a separate report node.

## Authorized roster

**L1 Runtime, L2 Language, L3 Foundation.** **2026-09-24:** operator ruled
BYTES K3/F4'/four postulates ("concur with rec."), the L1 bracket redesign
("option (a)."), the L1 pending-call precursor ("authorize option (a) for
pending-call. it has to be addressed."), ArgParse laws 1-2 resume after K3
("concur with rec."), and the L2 load-order fix ("concur with rec on load
order bug. fix it."). K3 landed `bfdbb9789`; Spec serves BYTES D2.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-SELECTED-PENDING-CALL-BUILD` -- `ready`, L, T1; AC-0 checkpoint first (D1 Architect `evt_3y5xkyf1v02dj`) | Follows from the rows the build leaves ignored | Bracket tree PARKED on `RT-BRACKET-SOURCE-EDGE` D0 STOP (`evt_72p4vjyzr71gb`); px8ta stays ignored. Held refs `4b4c8565c`, `21c039918`, `7f1a04a40` never moved or landed. Hard stop 0 |
| L2 | language | Make module import resolution load-order independent (spec 33 §3.3) | `LANG-FACADE-EXPORT-LOAD-ORDER` -- `active`, S, T1 (anchor `evt_42y7m4ftm872c`; Adversary `evt_5n8nhxesrbwz4`) | None | After the facade repair, L2 needs a new operator objective |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-DERIVED-FILTER-MEMBERSHIP-LAW` -- `3cc16e8d1` Full-CI red on the ambient census; re-cut per AC-4 (Architect `evt_4grsne5p7yyc1`) | `CAT-DERIVED-SORT-LAWS` (`ready`, S, T1); `BYTES-CONCAT-AND-ENCODE-CONTRACTS` D3 after its Spec D2 lands (D2 re-cut per AC-5) | Editing the census sentinel is NOT authorized except FILTER AC-4 and BYTES AC-5 (Architect `evt_6x38xqk62w4n7`) |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.