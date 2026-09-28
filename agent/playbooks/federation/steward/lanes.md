# Live lane roster

Current state only; the operator owns lanes, order and objectives. Hard limit
80 lines: replace stale text, never append history, measurements or
transcripts. Git and the WP thread are the history.

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
**2026-09-26** ("concur with rec."): the three `l1_acceptance` ignored rows
go to L2 and `ds5b` to L1; TCB growth they need still returns to the operator.
**2026-09-26/27, CI (`CI-UNDER-TWENTY-MINUTES`):** "bring that down under 20";
then "keep the 20-job cap; fit CI under it. If 20 minutes is no longer possible,
then keep the workload balanced across the 20 jobs." Memory R1-R5, R7
(`7b545fbb1`): "You may update skills, coordination.md, and scripts as
necessary."

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-IGNORED-ROWS-NEXT-GROUP` (M, T1). `RT-C5-PRIMITIVE-TYPE-ARGUMENTS` merged `235b8cefe` (native convoy rows carried to the WP lifting the `UnsupportedDependentMotive` gate) | `RT-C5-COMPOUND-HIGHER-ORDER-INDICES` (M, T1; `evt_7nf6ds3er67t1`), then `RT-SOURCE-IH-RELAY-K-VALUE` (r2, representation only) | Runtime ring on the NEXT-GROUP kickoff (AC-0 re-measure). Bracket tree PARKED on `RT-BRACKET-SOURCE-EDGE` D0 STOP (`evt_72p4vjyzr71gb`). Held refs `4b4c8565c`, `21c039918`, `7f1a04a40` never moved or landed |
| L2 | language | One resolution mode: strict admits only the minimal prelude, legacy passthrough deleted, binding a prelude name anywhere (incl. local binders) is an error (operator 2026-09-25) | `LANG-SESSION-SCOPE`: commit-1 catalog-plus-inventory prefix landed `4a8f27022`; commit 2+ (door withdrawal, I/H probe rewrites, R rows) as straight-ancestor increments (`evt_6222gyxzjgv1g`); H1, H2, H-closure, Vector+Deque `ab9faa5d2` and Map `b6402c71c` fences landed; stop 13 ruled (`evt_4xtksh29xcm9k`): Map fence holds the ground comparator witnesses; rulings only on checked text; Derived private-law row next (`evt_23jbdb2ztb02v`); next trigger 15 | `LANG-L1-ACCEPTANCE-ROWS` (`ready`, M, T1; operator 2026-09-27): the three `l1_acceptance` ignored rows as real assertions; then `LANG-EXPRESSION-SIGMA`: Spec route (a) renamed export (`fn and` + `export and as And`); Spec and CV turns at `b38deb752` (`evt_bq5st60jz2j5`); Language fixture turn held until SESSION-SCOPE lands, then Architect test-delta review and CV exact-tip vote; then floor additions for ledger-witnessed keyed names (operator 2026-09-25: an ever-present intrinsic's name is reserved; no intrinsic module; `Prod`/`zip` disposition belongs here), then flip and enforce | Language ring on the SESSION-SCOPE respin, then the Sigma fixture turn. Flip frame carries the Architect's SESSION-SCOPE carries (`evt_74ac7bpmpcd0c`: remaining `globals` writes, px8p harness write, raw-over-import `session_ids`, `owner_member` AC) |
| L3 | foundation | Move definable prelude conveniences into packages and remove collisions (minimal fixed prelude, operator 2026-09-25), then resume proof backfill | `BYTES-CONCAT-AND-ENCODE-CONTRACTS` D3 (M, T1), resumed from checkpoint `95ecba11c` on the landed Decoder success law (`CAT-DECODER-RECURSIVE-SUCCEEDS` merged `00ffedcfd`) | `CAT-DECODER-RECURSIVE-FUEL-SEED` (S, T1; Adversary `evt_1ay9njh53gkr2`) after D3; `CAT-AND-SORTED-PRELUDE-MOVE` (`ready`, M, T1) when `LANG-EXPRESSION-SIGMA` lands | Foundation ring on D3 (thread `evt_35mgp4kt600gd`). Language catalog_or item 7 waits on D3 (`evt_1cgncc9j8w2n2`). `Prod`/`zip` stay (runtime and ABI keyed; L2 floor question); `fold` is a separate removal |

## Update rule
One value per field; an update deletes the old one. Link, never copy. Verify a
row against its issue file and `origin/main`; no standalone currency commits.