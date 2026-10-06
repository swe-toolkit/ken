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
| L1 | runtime | Clear the selected ignored runtime rows | `RT-NAT-FANOUT-DETACHED-MULTI-MEMBER` (active; R6+R7+R8 building on `b1a837a12`, R8 ruled `evt_3a83pjbkea81n`; §1a count 1). `RT-CARRIED-CHILD-REFERENT-CONTAINMENT` merged (`4b2835bc8`). `RT-CARRIER-ROOT-EXIT-NESTED-MATCH-TRAP` merged (`77c6b0041`). `RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT` merged (`3695cd16a`). `RT-CHECKED-JOIN-SITE-MATCH-POPULATION` closed without repair (WIP `a7d46d6f2` never lands). `RT-NATIVE-SUPPORT-CRATE-SPLIT` merged (`4441a1422`). `RT-NATIVE-SEQUENTIAL-BRACKETS` merged (`5bad638bf`) | `RT-MATCH-MOTIVE-ADMISSION-SORT` (M, T1; `evt_37bbprs1gwajn`), then rt_escape `:691` (generated-entry projection invariant), unframed, then `RT-PRODUCER-MATCH-BORROWED-OPTION` (S, T1; `evt_1mz68b0assf2d`), then the bracket D0 (px8ta), then `RT-C5-PRODUCER-TYPE-SLOTS` (S, T1; Adversary `evt_3xsyh4fxeqnmw`), then `RT-C5-COMPOUND-HIGHER-ORDER-INDICES` (M, T1; `evt_7nf6ds3er67t1`), then `RT-SOURCE-IH-RELAY-K-VALUE` (r2, representation only) | `RT-RESPONSE-DESCENT-NESTED-COMPLETENESS` merged `c91f42e7b`. ROOT-EXIT merged (`0c0ce413a`): px7n pair and TREE-MATCH shared-bind row cleared. `rt_span_prov:355` has no successor yet. Bracket tree (px8ta LIFO row) parked on `RT-BRACKET-SOURCE-EDGE` D0 STOP (`evt_72p4vjyzr71gb`) until SEQUENTIAL-BRACKETS lands, then a fresh D0 from main. Held refs `4b4c8565c`, `21c039918`, `7f1a04a40` never moved or landed |
| L2 | language | One resolution mode: strict admits only the minimal prelude, legacy passthrough deleted, binding a prelude name anywhere (incl. local binders) is an error (operator 2026-09-25) | `LANG-SESSION-SCOPE`: commit-1 catalog-plus-inventory prefix landed `4a8f27022`; commit 2+ (door withdrawal, I/H probe rewrites, R rows) as straight-ancestor increments (`evt_6222gyxzjgv1g`); H1, H2, H-closure, Vector+Deque `ab9faa5d2` and Map `b6402c71c` fences landed; stop 13 ruled (`evt_4xtksh29xcm9k`): Map fence holds the ground comparator witnesses; rulings only on checked text; Derived private-law row next (`evt_23jbdb2ztb02v`); next trigger 15 | `LANG-REFINED-SIBLING-MATCH-TAIL` (operator 2026-09-30), split by the Architect (`evt_6e58trrr1bnfx`) into `LANG-GENERATED-J-PROOF-ASCRIPTION` (merged `b40f28977`), then `LANG-INFER-MATCH-INDEX-COVERAGE` (merged `ba2cd314c`), then `LANG-SIBLING-GOAL-REFINEMENT` (merged `0ae184458`, `61fde2caf`), then `LANG-REFINEMENT-INTRODUCTION-OBLIGATION` (merged `7ca183831`), then `LANG-ENSURES-PER-PATH-REALIZATION` (merged `b81ecf220`), then `LANG-REFINEMENT-INTRODUCTION-COVERAGE` (active, M, T1; Adversary `evt_23vexyxg1ya1e`; kicked `evt_420gcmaag9egh`), then `LANG-LET-BINDING-EQUATION` (S, T1; Adversary `evt_6rt513nmg3pzx`; ENSURES let-equation regression), then `LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION` (M, T1; Architect `evt_16s8ty301b4y4`; literal-side fail-opens F-A/F-B/F-C), then `LANG-REFINED-PARAM-REQUIRES-DESUGAR` (M, T1; operator 2026-10-03; unblocked by `VERIFY-CALL-SITE-OBLIGATION-CHANNEL`), then `LANG-INDEX-REFINEMENT-POSITIONAL` (merged `65af5c7cd`), then `LANG-INFER-MATCH-INDEXED-COMPLETE` (merged `5d139f422`), then `LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK` (merged `abc35cbcf`); then `LANG-L1-ACCEPTANCE-ROWS` (merged `ab476663a`; row 2, Int `/`, un-ignored by `KERNEL-INT-DIV-MOD-NATIVE`); then `LANG-NESTED-SPLIT-CONSTANT-MOTIVE-INDEX` (merged `5d5e7bf02`), then `LANG-NESTED-SPLIT-FIELD-DEPENDENCE` (merged `7450a0b17`), then `LANG-NESTED-MATRIX-DERIVED-TELESCOPE` (merged `db19a8d0c`; regression repair `LANG-NESTED-MATRIX-DISCOVERY-LEAF-PARITY` merged `f8bc5d4e1`), then follow-ons `LANG-FORWARD-REFERENCE-ACROSS-DATA-EXPORT` (M, T1; `evt_4js9vdbbcgmb2`) and `LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE` (S, T1; Adversary `evt_5gf6s6krtxft`); then `LANG-EXPRESSION-SIGMA`: Spec route (a) renamed export (`fn and` + `export and as And`); Spec and CV turns at `b38deb752` (`evt_bq5st60jz2j5`); Language fixture turn held until SESSION-SCOPE lands, then Architect test-delta review and CV exact-tip vote; then floor additions for ledger-witnessed keyed names (operator 2026-09-25: an ever-present intrinsic's name is reserved; no intrinsic module; `Prod`/`zip` disposition belongs here), then flip and enforce | Verify ring: `VERIFY-CALLER-OBLIGATION-REPORTING` (routed `9919c8808`, `dec_41ggzar39s9e9`), then `VERIFY-REUSED-ENV-TRUST-RESIDUE` (active, M, T1; kicked `evt_6ehqgcxtfw9nr`; operator approved the removal-only kernel `EnvMark` API 2026-10-05; verify writes both halves, kernel QA reviews the kernel diff; rollback ruled `evt_2sb1ehveqk5m0`), then `VERIFY-OBLIGATION-STABLE-IDENTITY` (M, T1; spec 46 §3.2), then `LANG-ELAB-RECURSIVE-FRAME-BUDGET` (M, T1; after REFINEMENT lands). Earlier: `VERIFY-CONTRACT-LOWERING-MULTI-REQUIRES` merged `958121efa`, then `VERIFY-CALL-SITE-PRECONDITION-DISCHARGE` merged (`392578133`); then `LANG-ACTIVE-PREMISE-RELOCATION-STACK-FRAME` (merged `8bf29cbc2`); then `VERIFY-CALL-SITE-OBLIGATION-CHANNEL` (merged `684e935fb`); then `TEST-CHAR-CONSTRUCTION-ROUTES-SCALAR` (S, T1; route (a) merged `a48d918f6`, (b)/(c) held on `LANG-REFINEMENT-INTRODUCTION-OBLIGATION`, placed after SIBLING-GOAL). Kernel ring: `KERNEL-OBS-REDUCT-WITNESS-TYPING` merged (`a02cecfed`); `KERNEL-OBS-TYPE-EQ-STRUCTURAL` merged (`884f493fe`); `KERNEL-LEQ-INT-LITERAL-REDUCTION` merged (`2df33a695`); `KERNEL-OBS-NESTED-CAST-LINEAR` merged (`90ca730f6`); `KERNEL-REFL-ENDPOINT-TYPED-CONVERSION` merged (`cc19ed853`); `KERNEL-OBS-SIGMA-QUOT-CAST-GATE` merged (`ed112ae4f`); `KERNEL-INT-DIV-MOD-NATIVE` merged (`a9e16c3a1`), all three operator-approved 2026-10-02; `KERNEL-QUOT-EQ-INTERIM-CAST-LEVEL` merged (`66d72ceb8`); `KERNEL-LEVEL-CLOSURE-CHECK` merged (`742f1b1a7`, `2244ac92a`); `KERNEL-ADMIT-BODIES-UPGRADABLE-ONLY` merged (`45a15f913`); `KERNEL-OBS-PI-CAST-GATE` merged (`b46d96fe8`); then `KERNEL-UNIT-ETA-SPEC-SCOPE` merged (`97813245e`); then `KERNEL-QUOT-FORM-EQUIVALENCE` merged (`436bb78b5`; P0; P1 census-gated and P3 measurement remain unframed, operator 2026-10-03); then `KERNEL-EQ-OMEGA-CARRIER-REDUCTION` merged (`e15c9d35c`); then `KERNEL-EQ-OMEGA-GATE-REDEX-STABLE` merged (`e140cad94`); then `KERNEL-J-NONREFL-ENDPOINT-SHARING` merged (`f973876aa`); then `KERNEL-J-NONREFL-FORMATION-FALLBACK` merged (`b2f0da751`); then `KERNEL-INT-LIT-CARRIER-CHECKED` merged (`79c392063`); then `KERNEL-STRING-CARRIER-CHECKED-SORT` (active, S, T1; Architect carry `evt_7z7g8kgk04z84`). Language ring: SESSION-SCOPE item 7 merged `96818f2e4`. Flip frame carries the Architect's SESSION-SCOPE carries (`evt_74ac7bpmpcd0c`: remaining `globals` writes, px8p harness write, raw-over-import `session_ids`, `owner_member` AC; `evt_7t76q35fvvk3e` fence globals rebind; `evt_5afmhnbmpeft7` fence-ID channel and `l3_strings_surface_acceptance.rs`; consumer floor 17 tests in 4 files, `evt_5bhtjzcryrgxa`; Parsing's ambient `charToInt`, `evt_7001gdwsnh1q1`; the channel also reports executed reject fences by label, `evt_1hqj9wp43gj96`; CAT5 D1 sentinel re-key, `evt_6bjp9pss40239`, and CAT5 client import lists not load-bearing while `expose_module` aliases remain, `evt_f356r4h7m9gb`) |
| L3 | foundation | Move definable prelude conveniences into packages and remove collisions (minimal fixed prelude, operator 2026-09-25), then resume proof backfill | `CAT-CONFIGURATION-DECODER-PRESENCE-CARRIER` (active, M, T1; Architect `evt_43cf1x0808egr`). `CAT-CONSOLE-TEXT-LAWS` merged (`a0a3f703f`; last survey row) | `CAT-AND-SORTED-PRELUDE-MOVE` when `LANG-EXPRESSION-SIGMA` lands | Architect `evt_5f1ewknxv3m6h`: no definable convenience slice remains (State/Coproduct, host types and `Perm` are keyed); L3 turns to proof backfill. `Prod`/`zip` stay (runtime and ABI keyed; L2 floor question); `fold` is a separate removal |
