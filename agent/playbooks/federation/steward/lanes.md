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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-INVOCATION-RESOURCE-PRECURSOR` -- `active`, L, T1 (Architect `evt_2373feaep6zh9`); D1 checkpoint routed `evt_py71tk0sgzg6` | D2 issuance; then retry body-322 capture and `RT-SELECTED-PENDING-CALL-PACKAGE` D1 | Bracket tree PARKED on `RT-BRACKET-SOURCE-EDGE` D0 STOP (`evt_72p4vjyzr71gb`); px8ta stays ignored. Held refs `4b4c8565c`, `21c039918`, `7f1a04a40` never moved or landed. Hard stop 0 |
| L2 | language | Make module import resolution load-order independent (spec 33 §3.3) | `LANG-IMPORT-LOAD-ORDER-INDEPENDENCE` -- `active`, S, T1 (anchor `evt_1vrvf6yycd29t`) | `LANG-KENFMT-AXIOM-CLOSING-PAREN` (`ready`, S, T2; serves L3 BYTES D2) | Respin `96241c352` routed `evt_5a8hne404c8md`; kick the formatter repair from main after it lands |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-ARGPARSE-LAWS` laws 1-2 -- `active`, L, T1; HS3 ruled (`evt_4xhp1k4y4w8sq`), laws check at WIP `f7e83e217`, AC-2(b) runs per `evt_58hr8a3vegsvc` | `CAT-DERIVED-FILTER-MEMBERSHIP-LAW` (`ready`, S, T1); `BYTES-CONCAT-AND-ENCODE-CONTRACTS` D3 after its Spec D2 | BYTES D2 `84c88e4a0` STOPPED on a kenfmt defect (`evt_8hpd13pfaetv`); resumes after `LANG-KENFMT-AXIOM-CLOSING-PAREN` lands. Editing the census sentinel is NOT authorized |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.