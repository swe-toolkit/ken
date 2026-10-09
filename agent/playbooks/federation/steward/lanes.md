# Live lane roster

Current state only; the operator owns lanes, order and objectives. Hard limit
80 lines. Git and the WP thread are the history: link, never copy, and make
no standalone currency commits.

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
L1-ACCEPTANCE-ROWS; kernel nested-Cast node after the spine-retry repair. **2026-10-07:**
"agreed, refinements should be real kernel types" (`evt_5se8p7fp024b5`).

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-NESTED-RESPONSE-OWNER-CALLER` (active, M, T1; behaviour-preserving `evt_ak61svpj64m6`: one stored, validated settlement classification per response owner, no planning refusal, every native row keeps its `c7c05d4e6` result; the span row stays ignored). `RT-PRODUCER-MATCH-BORROWED-OPTION` merged (`cc0e6f2b6`). `RT-CARRIED-NAT-MATCH-BOUNDED-IMMEDIATE` merged (`e18f9a2cc`). `RT-ESCAPE-FSHANDLE-FANNING-UNIGNORE` merged (`e7260a1cd`). `RT-FORWARD-TAIL-RET-CHECKED-CONTROL` merged (`79f44eecb`). `RT-GENERATED-ENTRY-PROJECTION-INVARIANT` merged (`877fd56d6`). `RT-MATCH-MOTIVE-ADMISSION-SORT` merged (`bd8bdd7af`). `RT-NAT-FANOUT-DETACHED-MULTI-MEMBER` merged (`21defbec2`). `RT-CARRIED-CHILD-REFERENT-CONTAINMENT` merged (`4b2835bc8`). `RT-CARRIER-ROOT-EXIT-NESTED-MATCH-TRAP` merged (`77c6b0041`). `RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT` merged (`3695cd16a`). `RT-CHECKED-JOIN-SITE-MATCH-POPULATION` closed without repair (WIP `a7d46d6f2` never lands). `RT-NATIVE-SUPPORT-CRATE-SPLIT` merged (`4441a1422`). `RT-NATIVE-SEQUENTIAL-BRACKETS` merged (`5bad638bf`) | `RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT` (L, T1; planning refusal on a settling-plane discriminant, then un-ignores `rt_span_prov_native:355` and r2, superseding `RT-SOURCE-IH-RELAY-K-VALUE`; Architect D0 first, stop if the relay provenance set is open), then `RT-NESTED-READ-CONTINUATION-HOST-VALUE-TRAP` (M, T1; Adversary `evt_5zgtvn9jb0gkb`; `cc0e6f2b6` moved refusals to a `-1` trap; D0 first), then `RT-PENDING-CALL-ERR-PAYLOAD-ADMISSION` (M, T1; px7m err, refused at admission E; `evt_2spyd3965e84m`), then `RT-C5-PRODUCER-TYPE-SLOTS` (S, T1; Adversary `evt_3xsyh4fxeqnmw`), then `RT-C5-COMPOUND-HIGHER-ORDER-INDICES` (M, T1; `evt_7nf6ds3er67t1`), then `RT-REGION-CAPACITY-TYPED-FAULT` (M, T1; Adversary `evt_4dmtyxqj0a7pa`; after the ignored rows), then `RT-CARRIED-NAT-PREDECESSOR-RE-ELIMINATION` (M, T1; Adversary `evt_2rjw2bf3e4r8d`; fails closed), then `RT-SELECTED-LIBRARY-DECLARATION-CLOSURE` (M, T1; `evt_1x7a2b7bh5nx5`; fails closed) | `RT-RESPONSE-DESCENT-NESTED-COMPLETENESS` merged `c91f42e7b`. ROOT-EXIT merged (`0c0ce413a`): px7n pair and TREE-MATCH shared-bind row cleared. Bracket tree (px8ta LIFO row) stays parked under `RT-BRACKET-CONTROL-REGION-IR`: the fresh D0 on `ff0a35f24` (`evt_v7p1c6t7kmw6`) found the three-child cut unchanged and child 1 unstartable until a source-carried edge design exists. Held refs `4b4c8565c`, `21c039918`, `7f1a04a40` never moved or landed |
| L2 | language | One resolution mode: strict admits only the minimal prelude, legacy passthrough deleted, binding a prelude name anywhere (incl. local binders) is an error (operator 2026-09-25) | `LANG-SESSION-SCOPE`: commit-1 catalog-plus-inventory prefix landed `4a8f27022`; commit 2+ (door withdrawal, I/H probe rewrites, R rows) as straight-ancestor increments (`evt_6222gyxzjgv1g`); H1, H2, H-closure, Vector+Deque `ab9faa5d2` and Map `b6402c71c` fences landed; stop 13 ruled (`evt_4xtksh29xcm9k`): Map fence holds the ground comparator witnesses; rulings only on checked text; Derived private-law row next (`evt_23jbdb2ztb02v`); next trigger 15 | `LANG-REFINED-SIBLING-MATCH-TAIL` (operator 2026-09-30), split by the Architect (`evt_6e58trrr1bnfx`) into `LANG-GENERATED-J-PROOF-ASCRIPTION` (merged `b40f28977`), then `LANG-INFER-MATCH-INDEX-COVERAGE` (merged `ba2cd314c`), then `LANG-SIBLING-GOAL-REFINEMENT` (merged `0ae184458`, `61fde2caf`), then `LANG-REFINEMENT-INTRODUCTION-OBLIGATION` (merged `7ca183831`), then `LANG-ENSURES-PER-PATH-REALIZATION` (merged `b81ecf220`), then `LANG-REFINEMENT-INTRODUCTION-COVERAGE` (merged `4ae2f60cf`), then `LANG-LET-BINDING-EQUATION` (merged `c49297983`), then `LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION` (merged `6b938b70b`), then `LANG-MATCH-RESULT-REFINEMENT-IDENTITY` (merged `68bd8ad73`), then `LANG-NAMED-REFINEMENT-TYPE-ARGUMENT`, `LANG-LAMBDA-PI-CARRIER-REFINEMENT-INTRODUCTION` and `LANG-NAMED-REFINED-BINDER-FIRST-CLASS` (closed, superseded by subset Σ), then `LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE` (merged `440b216f1`), then the subset-Σ program (Architect `evt_30frdrmj45ehg`): `LANG-FORWARD-REFERENCE-ACROSS-DATA-EXPORT` (merged `6f3b0aceb`), then `LANG-REFINEMENT-PROOF-ERASURE` (active, M, T1; kernel-classified erasure plan in the package, `evt_7avananzmmmph`; `SPEC-REFINEMENT-SUBSET-SIGMA` merged `28d8845fb`; proof-first native Σ pinned fail-closed, `evt_55gdejjrzap0p`), then `LANG-NATIVE-SIGMA-ERASED-FIELD` (M, T1; native representation for an erased Σ field; D0 first), then `LANG-FORWARD-PROP-INTRO-EDGE` (S, T1; Adversary `evt_5ts6gg4z19rct`), then `LANG-NESTED-MATRIX-MOTIVE-CODOMAIN-SCOPE` (S, T1) and `LANG-LATE-LEVEL-SOLVE-SIBLING-SITES` (M, T1; Adversary `evt_2akanmfydmqgz`), then `LANG-PATH-CONDITION-EVIDENCE` (M, T1; AC-0 done), then `LANG-REFINEMENT-SUBSET-SIGMA` (draft, L, T1; after `VERIFY-TRANSITIVE-HONESTY` on the verify ring), then `LANG-MATCH-ARM-REFL-PAIR-GOAL` (S, T1; catalog Finding `evt_7f1fe7xrnjmn1`, worked around), then `LANG-OBLIGATION-HOLE-LEVEL-META-CLOSURE` (M, T1; F-F `evt_71vryd6g641y3`; fails closed), then `LANG-CLOSE-GOAL-PATH-CONDITIONS` (S, T1; Architect `evt_50qx9dbgqhwfe`), then `LANG-REFINED-PARAM-REQUIRES-DESUGAR` (M, T1; operator 2026-10-03; unblocked by `VERIFY-CALL-SITE-OBLIGATION-CHANNEL`), then `LANG-INDEX-REFINEMENT-POSITIONAL` (merged `65af5c7cd`), then `LANG-INFER-MATCH-INDEXED-COMPLETE` (merged `5d139f422`), then `LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK` (merged `abc35cbcf`); then `LANG-L1-ACCEPTANCE-ROWS` (merged `ab476663a`; row 2, Int `/`, un-ignored by `KERNEL-INT-DIV-MOD-NATIVE`); then `LANG-NESTED-SPLIT-CONSTANT-MOTIVE-INDEX` (merged `5d5e7bf02`), then `LANG-NESTED-SPLIT-FIELD-DEPENDENCE` (merged `7450a0b17`), then `LANG-NESTED-MATRIX-DERIVED-TELESCOPE` (merged `db19a8d0c`; regression repair `LANG-NESTED-MATRIX-DISCOVERY-LEAF-PARITY` merged `f8bc5d4e1`), then follow-on `LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE` (merged); then `LANG-EXPRESSION-SIGMA`: Spec route (a) renamed export (`fn and` + `export and as And`); Spec and CV turns at `b38deb752` (`evt_bq5st60jz2j5`); Language fixture turn held until SESSION-SCOPE lands, then Architect test-delta review and CV exact-tip vote; then floor additions for ledger-witnessed keyed names (operator 2026-09-25: an ever-present intrinsic's name is reserved; no intrinsic module; `Prod`/`zip` disposition belongs here), then flip and enforce | Verify ring: `VERIFY-CALLER-OBLIGATION-REPORTING` merged (`fd1bafb0e`), then `VERIFY-REUSED-ENV-TRUST-RESIDUE` merged (`174904b18`), then `VERIFY-PACKAGE-ROUTE-EXAMPLE-DECLARATIONS` merged (`1153a9fc6`), then `VERIFY-OBLIGATION-STABLE-IDENTITY` merged (`fe21d61c4`), then `VERIFY-PACKAGE-EXAMPLE-BINDING-SCOPE` merged (`e1b609597`), then `VERIFY-INSTANCE-OWNER-KEY` merged (`06c037f92`), then `VERIFY-STRUCTURAL-HEAD-INSTANCE-IDENTITY` merged (`b5ba6619c`), then `VERIFY-NAMED-HEAD-INSTANCE-IDENTITY` merged (`176646c5f`), then `VERIFY-GLOBALS-IDENTITY-CHECKED-INSERT` merged (`d5a522d7f`), then `VERIFY-OBLIGATION-ID-SHADOWED-OWNER` (active, M, T1; Adversary `evt_mxa6e26p25e2`; a shadowed or rebound owner's `requires` premise is dropped, order-dependent), then `VERIFY-INSTANCE-ALIAS-HEAD-KEY` (M, T1; Adversary `evt_2em0e8ma3sgwb`; pre-existing coherence gap at alias heads), then `VERIFY-NATIVE-PROGRAM-EXAMPLE-EXCLUSION` (S, T2; Architect `evt_6qc2akyd4457c`),, then `VERIFY-PACKAGE-DUPLICATE-TOP-LEVEL-REFUSAL` (S, T1; Adversary `evt_3vjznkjfzawy7`), then `VERIFY-TRANSITIVE-HONESTY` (M, T1; kernel K-b), then `LANG-ELAB-RECURSIVE-FRAME-BUDGET` (M, T1; after REFINEMENT lands). Earlier: `VERIFY-CONTRACT-LOWERING-MULTI-REQUIRES` merged `958121efa`, then `VERIFY-CALL-SITE-PRECONDITION-DISCHARGE` merged (`392578133`); then `LANG-ACTIVE-PREMISE-RELOCATION-STACK-FRAME` (merged `8bf29cbc2`); then `VERIFY-CALL-SITE-OBLIGATION-CHANNEL` (merged `684e935fb`); then `TEST-CHAR-CONSTRUCTION-ROUTES-SCALAR` (S, T1; route (a) merged `a48d918f6`, (b)/(c) held on `LANG-REFINEMENT-INTRODUCTION-OBLIGATION`, placed after SIBLING-GOAL). Kernel ring: `KERNEL-OBS-REDUCT-WITNESS-TYPING` merged (`a02cecfed`); `KERNEL-OBS-TYPE-EQ-STRUCTURAL` merged (`884f493fe`); `KERNEL-LEQ-INT-LITERAL-REDUCTION` merged (`2df33a695`); `KERNEL-OBS-NESTED-CAST-LINEAR` merged (`90ca730f6`); `KERNEL-REFL-ENDPOINT-TYPED-CONVERSION` merged (`cc19ed853`); `KERNEL-OBS-SIGMA-QUOT-CAST-GATE` merged (`ed112ae4f`); `KERNEL-INT-DIV-MOD-NATIVE` merged (`a9e16c3a1`), all three operator-approved 2026-10-02; `KERNEL-QUOT-EQ-INTERIM-CAST-LEVEL` merged (`66d72ceb8`); `KERNEL-LEVEL-CLOSURE-CHECK` merged (`742f1b1a7`, `2244ac92a`); `KERNEL-ADMIT-BODIES-UPGRADABLE-ONLY` merged (`45a15f913`); `KERNEL-OBS-PI-CAST-GATE` merged (`b46d96fe8`); then `KERNEL-UNIT-ETA-SPEC-SCOPE` merged (`97813245e`); then `KERNEL-QUOT-FORM-EQUIVALENCE` merged (`436bb78b5`; P0; P1 census-gated and P3 measurement remain unframed, operator 2026-10-03); then `KERNEL-EQ-OMEGA-CARRIER-REDUCTION` merged (`e15c9d35c`); then `KERNEL-EQ-OMEGA-GATE-REDEX-STABLE` merged (`e140cad94`); then `KERNEL-J-NONREFL-ENDPOINT-SHARING` merged (`f973876aa`); then `KERNEL-J-NONREFL-FORMATION-FALLBACK` merged (`b2f0da751`); then `KERNEL-INT-LIT-CARRIER-CHECKED` merged (`79c392063`); then `KERNEL-STRING-CARRIER-CHECKED-SORT` merged (`c7b5c2b38`); then `KERNEL-ENV-MARK-PREFIX-MUTATION-REFUSAL` merged (`26e7dce3f`). Language ring: SESSION-SCOPE item 7 merged `96818f2e4`. Flip frame carries the Architect's SESSION-SCOPE carries (`evt_74ac7bpmpcd0c`: remaining `globals` writes, px8p harness write, raw-over-import `session_ids`, `owner_member` AC; `evt_7t76q35fvvk3e` fence globals rebind; `evt_5afmhnbmpeft7` fence-ID channel and `l3_strings_surface_acceptance.rs`; consumer floor 17 tests in 4 files, `evt_5bhtjzcryrgxa`; Parsing's ambient `charToInt`, `evt_7001gdwsnh1q1`; the channel also reports executed reject fences by label, `evt_1hqj9wp43gj96`; CAT5 D1 sentinel re-key, `evt_6bjp9pss40239`, and CAT5 client import lists not load-bearing while `expose_module` aliases remain, `evt_f356r4h7m9gb`) |
| L3 | foundation | Grow the catalog along `docs/program/06-catalog-campaign.md`: post-Data layers in roadmap order, top-informed and bottom-proven, every package fully proved (operator 2026-09-13). Operator 2026-10-08: "start l3 on the establish catalog program." | `SPEC-FORMAL-LANGUAGES-REGEX-CONTRACT` (active, S, T1; spec ring; Architect D0 first). `CAT-FORMAL-LANGUAGES-NFA` merged (`89ac3d285`). `CAT-FORMAL-LANGUAGES-FINITE-REACHABILITY` merged (`48c333620`). `CAT-FORMAL-LANGUAGES-DFA` merged (`b34da79d1`). Band A merged; proof-gap population closed (`evt_655ckkc1dmfmc`) | the regex CAT node, then minimisation, then the Bytes/Cursor lexer bridge. `CAT-AND-SORTED-PRELUDE-MOVE` when `LANG-EXPRESSION-SIGMA` lands | Kick SPEC-REGEX (spec ring) after this frame's bundle lands |
