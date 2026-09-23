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
- **2026-09-23, L3 REINSTATED, L2 STAYS IDLE:** operator asked to bring up
  the other two lanes if the gpt-6-luna seats were working as expected, then
  answered the L2 objective fork "Leave L2 idle for now". Roster is L1 + L3.
  SUPERSEDES the 2026-09-21 one-lane reduction.

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

**L1 Runtime and L3 Foundation; L2 Language for one WP, then idle.**
**2026-09-23:** "seat the spec and kernel rings as needed": Kernel runs the L1
kernel trace beside the carrier; Spec serves BYTES D2. No other ring starts
without an operator lane change. Accepted work still routes immediately.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-SELECTED-PENDING-CALL-PACKAGE` -- `active`, L, T1, pre-code design of the new runtime representation the operator authorized 2026-09-23 ("concur with rec on L1 carrier") after the carrier's D2 STOP. D0 census, then a D1 specification (Architect `evt_4chkm2xxyfang`); STOP at D1 means park | `KERNEL-NORMALIZE-ORIGIN-TRACE` -- `active` on the **kernel** ring, concurrent, L, T1, operator-authorized TCB growth: one reducer with a no-op/collecting observer and a traced-equals-normalize gate. When it lands, the bracket tree resumes on it, with `rt_escape` row 13 and `rt_span` row 14 as extra controls (Architect `evt_2817gkyjtdaa7`) | **BRACKET TREE HELD** until the kernel trace lands. Child 1 at `4b4c8565c543c148039ea06ac798c7c91eda2ce8` and child 2 checkpoint `21c039918` are held reviewed inputs, NEVER independently landed or moved. The held tree turns `linked_public_escape_is_exact_closed` red. Hard stop 0 |
| L2 | language | One WP only (operator 2026-09-23) | `LANG-QUALIFIED-ACCESS-REQUIRES-IMPORT` -- `active`, S, T1: `resolve_ref` grants `M.foo` only under a qualified import of `M` (spec `33 §3.2`) | None: L2 returns to idle | language ring owes the candidate, then QA + Decision |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-DEQUE-POP-LAWS` -- `ready`, S, T1: in-package `popFront` list-view law, Deque stays unpublished (Architect `evt_4pd2scpkcv9x8`) | None framed | `CAT-ARGPARSE-LAWS` Laws 1-2 and `CAT-PARSING-LAWS` round trip held for the operator: per-occurrence literal identity, and the BYTES fork (fact 4 false; K3 literal bridge). Editing the census sentinel is NOT authorized |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.