# Live lane roster

Current state only; the operator owns lanes, order and objectives. Hard limit
80 lines: replace stale text, never append history, measurements or
transcripts. Git and the WP thread are the history. One value per field; link,
never copy. Verify a row against its issue file and `origin/main`; no
standalone currency commits.

## Live roster direction (operative rulings, retained verbatim)

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
- **2026-09-30, String laws** ("concur with recs"): no section postulate
  (false by NFC, `evt_h489cs74b8j`); concat/slice laws restated at zero TCB.
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

**L1 Runtime, L2 Language (+ Verify), L3 Foundation.** **2026-09-24:** L1
pending-call precursor: "authorize option (a) for pending-call. it has to be
addressed." **2026-09-26** ("concur with rec."): the three `l1_acceptance`
ignored rows go to L2; TCB growth they need still returns to the operator.
**2026-09-27, CI:** "keep the 20-job cap; fit CI under it." **2026-09-29, L1
order:** "(a) but just after the ignored tests are cleared." Bracket tree
("concur with rec."): a fresh D0 after SEQUENTIAL-BRACKETS. **2026-09-29:**
"It's worth bringing verify into one of the lanes (L2) to address the flaky
test." **2026-09-30** ("concur with recs"): the Char-route WP to Verify;
LANG-REFINED next on Language. Later ("concur with recs"): after SIBLING-GOAL,
REFINEMENT-INTRODUCTION then INDEX-REFINEMENT-POSITIONAL, both before
L1-ACCEPTANCE-ROWS; kernel nested-Cast node after the spine-retry repair.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-NATIVE-CONTINUATION-ENV-CARRIAGE` (L, T1; resumes on WIP `86b0ebb9b` over the landed precursor `RT-PLANNER-PER-EMITTER-AVAILABILITY` `c95c6a556`), then `RT-NATIVE-SEQUENTIAL-BRACKETS` as its first consumer (rt_escape `:654` second witness) | `RT-JOIN-PHASE-CASE-BINDER-CARRIED` (S, T1; rt_escape `:714` join 1244, `evt_2pf70v5ty9mf0`), then `RT-PRODUCER-MATCH-BORROWED-OPTION` (S, T1; `evt_1mz68b0assf2d`), then the bracket D0 (px8ta), then `RT-C5-PRODUCER-TYPE-SLOTS` (S, T1; Adversary `evt_3xsyh4fxeqnmw`), then `RT-C5-COMPOUND-HIGHER-ORDER-INDICES` (M, T1; `evt_7nf6ds3er67t1`), then `RT-SOURCE-IH-RELAY-K-VALUE` (r2, representation only) | `RT-RESPONSE-DESCENT-NESTED-COMPLETENESS` merged `c91f42e7b`. ROOT-EXIT merged (`0c0ce413a`): px7n pair and TREE-MATCH shared-bind row cleared. `:654`, `:714` relabelled to their successors. `rt_escape:686` and `rt_span_prov:355` have no successor yet. Bracket tree (px8ta LIFO row) parked on `RT-BRACKET-SOURCE-EDGE` D0 STOP (`evt_72p4vjyzr71gb`) until SEQUENTIAL-BRACKETS lands, then a fresh D0 from main. Held refs `4b4c8565c`, `21c039918`, `7f1a04a40` never moved or landed |
| L2 | language | One resolution mode: strict admits only the minimal prelude, legacy passthrough deleted, binding a prelude name anywhere (incl. local binders) is an error (operator 2026-09-25) | `LANG-SESSION-SCOPE`: commit-1 catalog-plus-inventory prefix landed `4a8f27022`; commit 2+ (door withdrawal, I/H probe rewrites, R rows) as straight-ancestor increments (`evt_6222gyxzjgv1g`); H1, H2, H-closure, Vector+Deque `ab9faa5d2` and Map `b6402c71c` fences landed; stop 13 ruled (`evt_4xtksh29xcm9k`): Map fence holds the ground comparator witnesses; rulings only on checked text; Derived private-law row next (`evt_23jbdb2ztb02v`); next trigger 15 | `LANG-REFINED-SIBLING-MATCH-TAIL` (operator 2026-09-30), split by the Architect (`evt_6e58trrr1bnfx`) into `LANG-GENERATED-J-PROOF-ASCRIPTION` (merged `b40f28977`), then `LANG-INFER-MATCH-INDEX-COVERAGE` (merged `ba2cd314c`), then `LANG-SIBLING-GOAL-REFINEMENT` (M; increment 1 merged `0ae184458`, increment 2 waits on `KERNEL-OBS-REDUCT-WITNESS-TYPING`), then `LANG-REFINEMENT-INTRODUCTION-OBLIGATION` (spec 34 §5; unblocked by `2df33a695`; resumes from the park `f4edc461c` after FIELD-DEPENDENCE, §1a=1), then `LANG-INDEX-REFINEMENT-POSITIONAL` (merged `65af5c7cd`), then `LANG-INFER-MATCH-INDEXED-COMPLETE` (merged `5d139f422`), then `LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK` (merged `abc35cbcf`); then `LANG-L1-ACCEPTANCE-ROWS` (merged `ab476663a`; row 2, Int `/`, stays ignored pending the operator's `div_int`/`mod_int` TCB ruling); then `LANG-NESTED-SPLIT-CONSTANT-MOTIVE-INDEX` (merged `5d5e7bf02`), then `LANG-NESTED-SPLIT-FIELD-DEPENDENCE` (active, M, T1; narrowed after §1a 6, `evt_5eahknef2gfbm`), then `LANG-NESTED-MATRIX-DERIVED-TELESCOPE` (L, T1; the structural closure), then follow-ons `LANG-FORWARD-REFERENCE-ACROSS-DATA-EXPORT` (M, T1; `evt_4js9vdbbcgmb2`) and `LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE` (S, T1; Adversary `evt_5gf6s6krtxft`); then `LANG-EXPRESSION-SIGMA`: Spec route (a) renamed export (`fn and` + `export and as And`); Spec and CV turns at `b38deb752` (`evt_bq5st60jz2j5`); Language fixture turn held until SESSION-SCOPE lands, then Architect test-delta review and CV exact-tip vote; then floor additions for ledger-witnessed keyed names (operator 2026-09-25: an ever-present intrinsic's name is reserved; no intrinsic module; `Prod`/`zip` disposition belongs here), then flip and enforce | Verify ring: `TEST-SOURCE-TEXT-ORACLE-RETIRE` merged (`46fe123c1`); `CI-TEST-BUILD-PER-SHARD` closed (operator 2026-10-01: "CI is good enough for now."); then `TEST-CHAR-CONSTRUCTION-ROUTES-SCALAR` (S, T1; route (a) merged `a48d918f6`, (b)/(c) held on `LANG-REFINEMENT-INTRODUCTION-OBLIGATION`, placed after SIBLING-GOAL). Kernel ring: `KERNEL-OBS-REDUCT-WITNESS-TYPING` merged (`a02cecfed`); `KERNEL-OBS-TYPE-EQ-STRUCTURAL` merged (`884f493fe`); `KERNEL-LEQ-INT-LITERAL-REDUCTION` merged (`2df33a695`); `KERNEL-OBS-NESTED-CAST-LINEAR` active (S, T1; N0–N2 land as the increment, `evt_1k4yc761p6rty`); then `KERNEL-J-NONREFL-ENDPOINT-SHARING` (M, T1; the S residual, AC-0 parent attribution first); `KERNEL-REFL-ENDPOINT-TYPED-CONVERSION` (S, T1; regression from 884f, Adversary `evt_1rnwn0jc09jw9`; carries the ζ conformance case) and `KERNEL-OBS-SIGMA-QUOT-CAST-GATE` (S, T1; same finding) wait on operator TCB approval; then `KERNEL-INT-LIT-CARRIER-CHECKED` (S, T1; `evt_3dj22arjesbg4`). Language ring: SESSION-SCOPE item 7 merged `96818f2e4`. Flip frame carries the Architect's SESSION-SCOPE carries (`evt_74ac7bpmpcd0c`: remaining `globals` writes, px8p harness write, raw-over-import `session_ids`, `owner_member` AC; `evt_7t76q35fvvk3e` fence globals rebind; `evt_5afmhnbmpeft7` fence-ID channel and `l3_strings_surface_acceptance.rs`; consumer floor 17 tests in 4 files, `evt_5bhtjzcryrgxa`; Parsing's ambient `charToInt`, `evt_7001gdwsnh1q1`; the channel also reports executed reject fences by label, `evt_1hqj9wp43gj96`; CAT5 D1 sentinel re-key, `evt_6bjp9pss40239`, and CAT5 client import lists not load-bearing while `expose_module` aliases remain, `evt_f356r4h7m9gb`) |
| L3 | foundation | Move definable prelude conveniences into packages and remove collisions (minimal fixed prelude, operator 2026-09-25), then resume proof backfill | `CAT-NAT-SUB-ADD-CANCEL` (S, T1; `evt_373qe5k8n5626`). `CAT-DERIVED-MAP-APPEND` merged `eb7218df5`. `CAT-VECTOR-DEFERRED-LAWS` open: five laws merged (`10af5f45f`, `79a6e5ac7`); `lookup_zip_with` and the pointwise `zip_with_map` wait on `LANG-SIBLING-GOAL-REFINEMENT` | Next §2a proof-backfill slice (Architect nominates); `CAT-AND-SORTED-PRELUDE-MOVE` when `LANG-EXPRESSION-SIGMA` lands | Architect `evt_5f1ewknxv3m6h`: no definable convenience slice remains (State/Coproduct, host types and `Perm` are keyed); L3 turns to proof backfill. `Prod`/`zip` stay (runtime and ABI keyed; L2 floor question); `fold` is a separate removal |
