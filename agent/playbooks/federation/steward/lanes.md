# Live lane roster

Current operational state only. The operator owns lane count, ordering, and
objectives; the Steward updates a field only with the ruling that changed it.

Hard limit: 80 lines. Replace stale text; never append history. Do not record
completed work, review transcripts, measurements, explanations, prior states, or
superseded instructions. Git and the WP thread are the history.

## Live roster direction

These rulings remain operative and are retained verbatim.

- **2026-08-22:** "playbook = stable discipline, this file = mutable roster."
- **2026-08-25:** "there are three lanes authorized right now. language (lane
  2) was unblocking rt on priority, and when that was done should have unblocked
  foundation (lane 3) with module/import."
- **2026-09-23/24:** L3 reinstated, L2 reactivated. Roster is L1 + L2 + L3.
- **2026-09-23, doc-only review:** "doc only merges may have reviewers. However, if it is a
  doc change that I directed you to make a review is not necessary."

## Catalog direction

- **2026-09-25, built-ins:** "it is a language design weakness to allow
  built-ins to be overriden. It makes for confusing code, allows obfuscation
  and is therefore a security risk. Anything in the prelude should be
  considered "built-in" and therefore a fixed part of the language surface."
  Then: "Because the built-in and prelude definitions can't be overriden,
  then -- out of kindness to the authors of ken code -- they should be the
  minimal set required." Then: "any convenience names that are not required
  by the prelude rules above (or the kernel built-ins) should be considered
  technical debt and moved to packages."
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
as its own node. Investigation opens a repair, never a separate report node.

## Authorized roster

**L1 Runtime, L2 Language, L3 Foundation.** **2026-09-24:** operator ruled
BYTES K3/F4'/four postulates ("concur with rec."), the L1 bracket redesign
("option (a)."), and the L1 pending-call precursor ("authorize option (a) for
pending-call. it has to be addressed."). Spec serves BYTES D2.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-SELECTED-PENDING-CALL-BUILD` increment 2 (increment 1 landed `303b0bf71`; released `evt_6tytznvz5qbma`) | `RT-COMPMATCH-TREE-SCRUTINEE`, whose AC-0 re-measures the first refusal of the unowned ignored rows | Increment 2 HELD: third advancing stop, Architect ruling pending Research advisory (`evt_3engggxek84h8`; shared predicate: admission decides Planned from facts the emitter does not consume); WIP `a99f92581`. Bracket tree PARKED on `RT-BRACKET-SOURCE-EDGE` D0 STOP (`evt_72p4vjyzr71gb`); px8ta stays ignored. Held refs `4b4c8565c`, `21c039918`, `7f1a04a40` never moved or landed |
| L2 | language | One resolution mode: strict admits only the minimal prelude, legacy passthrough deleted, binding a prelude name anywhere (incl. local binders) is an error (operator 2026-09-25) | `LANG-EXPRESSION-SIGMA` -- respin (`b8b344942` withdrawn: stack-budget SIGABRT on 66bf1173d, `evt_4vx785jppfwrc`) | `LANG-SESSION-SCOPE` -- `active`, M, T1 (anchor `evt_7039h2hhv5qny`); then floor additions for ledger-witnessed keyed names (operator 2026-09-25: an ever-present intrinsic's name is reserved; no intrinsic module; `Prod`/`zip` disposition belongs here), then flip and enforce | SESSION-SCOPE bases on landed privacy `66bf1173d` (Architect D0); D0 rules the prelude-family ownership carry (`evt_1nrnwakvjy2gj`) |
| L3 | foundation | Move definable prelude conveniences into packages and remove collisions (minimal fixed prelude, operator 2026-09-25), then resume proof backfill | `CAT-COLLECTIONS-MAP-FILTER-PRELUDE-MOVE` -- respin on `66bf1173d` (`d0434afc8` withdrawn: `cat_derived_pub_export` pin, `evt_59wmj2hzq2whm`) | `CAT-LAWFULFUNCTORS-LIST-MAP-SUBSUMPTION` -- `ready`, M, T1; then `And`/`and_*`/`is_sorted` after `LANG-EXPRESSION-SIGMA`; `BYTES-CONCAT-AND-ENCODE-CONTRACTS` D3 | Kick the subsumption when map/filter lands. `Prod`/`zip` stay (runtime and ABI keyed; L2 floor question); `fold` is a separate removal |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.