# Live lane roster

Current state only; the operator owns lanes, order and objectives. Hard limit
80 lines. Link git and WP threads, never copy; no standalone currency commits.

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

- **2026-09-17:** clearing the ignored tests "is the top priority until it is done."
- **2026-09-19:** "It is too slow. You need to streamline the process and focus
  on forward movement, not management."
- **2026-10-17:** "your priority has been to re-activate the ignored tests.
  Parking one for no clear reason is not in line with that direction. do what
  you need to do to reactivate the remaining ignored tests."
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
"agreed, refinements should be real kernel types" (`evt_5se8p7fp024b5`). **2026-10-16, RELAY seat-plan hoist:** "a, with the Architect's review first". **2026-10-22:** kernel SCT-group companion admission (TCB growth): "approve change". Doc track: "keep doc idle". Bracket chain: "keep separate" (land link by link). **2026-11-15, CI:** "sounds good. you have permission to change ci.yml as above, or to delegate as you deem fit" (z3 job scoped to the Z3 route; the formatter whole-corpus tests gated on formatter inputs).

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | The Linux ABI program (`docs/program/10-linux-abi-completion.md` §4) to its §6 exit, in Steward order (operator 2026-10-30: "Order the linux ABI campaign as you see fit"): PX10's slices, then PX11, PX12, ABI-M2, ABI-S4, ABI-S2, ABI-FSCREATE-RECURSIVE-CONTRACT | `PX10-NATIVE-SPAWN-PARITY` Increment B1a (anchor `evt_v047rr7fkkkk`): the structural closure that classifies callee representation once before lowering (`evt_7bmbr76449ap4`; the bind stays a Let, `evt_706fb41bc2scs`), with the F1 and F2 repairs, the starter trap-classifier fix `evt_5y7e328rr2wd8`, and the B1a carries in its issue file (the B invariant, F's parity rows, C1, C3). `RT-NATIVE-FORWARDED-BOUNDARY-TRAP` closed with no trap (AC-0 LAWFUL `evt_r2abprmhd4zf`); `RT-NATIVE-SELF-EMITTER-RESPONSE-ELIGIBILITY` closed without a repair (R-ERASURE `evt_76yd7z7p2g8pj`). `RT-NATIVE-SETTLEMENT-PLACEMENT-KEY` merged (`f32af2a0b`). Both sit inside `PX10-NATIVE-SPAWN-PARITY` Increment B1 (anchor `evt_v047rr7fkkkk`). The B1a WIP2 diff is in the Runtime implementer's home (sha256 `723942de...`): never compact that seat. Earlier landings: git log and WP threads. | `RT-TEST-PX10-AC2-MASK-CONTROL-QUIESCENT` merged (`fe713d1ba`). Active: `RT-NET-UNIX-PATH-FS-ROOT-SCOPE` (M, T1; Adversary `evt_7hffqy9m7v6yb`: a Unix path escapes the FS root; Architect AC-0 rules confine or state). `PX11-NATIVE-STREAM-SOCKET-PARITY` (ready, L, T1; after the Unix ruling; closes the interpreter-admission set), then datagrams, option families, the resolver capability, native parity; then PX12, ABI-M2, ABI-S4, ABI-S2, ABI-FSCREATE-RECURSIVE-CONTRACT. Carried, not framed: de-issue self relocations and re-key the six `pre_schema_transport_sources` readers | B1a merged (`d7c0be6fa`); B2 merged (`0a5cf58eb`). `RT-NATIVE-VIS-MAY-YIELD-SUMMARY` withdrawn to draft after its D0 (`evt_5rb967199ham1`; no consumer, refusal gone). `RT-NATIVE-THEN-CHAIN-SOURCE-SLOT-TRANSFER` merged (`87db6fc14`). Active: `PX10-PIDFD-CHILD-SIGNAL` (L, T1; anchor `evt_3knzfyh9b6n4t`; AC-0 D-G `evt_26bs51hjc8nq`): Increment 1 merged (`5c557e0ad`); Increment 2 (the signal operation) `5b9bb88d1` red in CI and withdrawn (`evt_qqs60hp59zqy`); respin merged (`ef00c58be`); node closed. `LANG-PROCESS-CHILD-PIPE-WRITE-HOME` merged (`943d14a06`). `RT-NATIVE-PIPE-EARLY-CLOSE-SETTLEMENT` merged (`5e01f2ae9`, `cd9f3f907`). `RT-NATIVE-NESTED-MATCH-BOUND-VALUE` merged (`be58665f9`). `RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION` merged (Inc2 `e5177f7c1`; ResourceStreamWrite NativeTested, hold released). `RT-NATIVE-RUNTIME-STATICLIB-EXPLICIT-BUILD` merged (`4e6c4e62b`; Cargo 1.100 shard green, `evt_3fn8hx3y20hkv`). `RT-NATIVE-API-PROBE-RLIB-EXPLICIT-LOCATION` merged (`efa08017b`; second Cargo 1.100 base blocker, px4b rlib lookup; AC-3 `evt_1sb5qqwm06cyv`). `TEST-PX10-SIGNAL-PID-MARKER-RACE` merged (`82141d9f1`; CI base flake). `PX11-STREAM-SOCKET-CONNECT-ACCEPT` merged (`99d594ff7`; interpreter-admission set for the seven Net ops; native parity is its immediate successor and closes the set). `RT-NATIVE-INVOCATION-ARENA-COST-OBSERVATION` merged (`1efc8fff2`; Net cost rows carried until `LANG-NATIVE-CLOSURE-DEAD-BINDER-CAPTURE` lands). `SPEC-FFI-IO-PROCESS-NATIVE-STATUS` merged (`8da54fbc1`). P5b marker absence ruled not a criterion (Architect `evt_16d2p4ewqdptm`). `RT-NATIVE-FORWARDED-CONTINUATION-REENTRY` is draft, re-released only if B1a's F parity rows show D2. Candidate form ruled `evt_4151apntyfsmt`. Acceptance posture `evt_5vsaybryes08b`: parity is measured under an uncommitted overlay, and promotion and parity rows are in B2. Increment B is at §1a 18 (next research re-ask at 21); §1b has 22 entries. RNSPK closed at §1a 8. Carried, not framed: an outer-match-bound value used in two sibling nested arms, each followed by a continuation, still refuses natively with PlannerInvariant ("more than one ordinary fresh-result capture destination", `aggregates.rs:6979`), fail-closed, predates `be58665f9`; framed as Inc2's prerequisite if its write fixture closes on two arms (Adversary `evt_69qysjcqjrkhm`); a then-chain whose continuation runs twice fails native build with PlannerInvariant while the interpreter runs it, fail-closed (Adversary `evt_1fr8a0ebqce1v`); a typed reserved native status for residual-carrier integrity faults on the `SELECTED_CALL_INTEGRITY_STATUS_V1` pattern, spec-bearing (TerminalErrorV1), after B2 (Architect `evt_58avnxxk6khhr`); the starter labels every root `-1` "malformed borrowed process input", because the generic `require_i64` failure shares that arm, so a non-process `-1` such as F2 is mislabelled, fail-closed, exit status unaffected (Architect `evt_5e9bgw1xcwen7`); a non-Process build-time refusal names the effect constructor (e.g. `ClockOp`), not the HostOpV1, fail-closed (Architect `evt_59z2v4tgmnvs4` (c)); smoke's `visits [Process]` row is admitted by the checker and interpreter but its export refuses FS, and argv_env_pipe's export overflows a 16 MiB stack (Architect `evt_5acdhqams3hxe`); a data Match whose arms are both Ret fails closed at `aggregates.rs:465` (`evt_5efd2tp6dqsj3`); main's response-owner checks keep the same format-keyed Store match (`units.rs` ~:534, ~:4455; Architect `evt_2p0t27vsz08fh`); a scalar-result nested carried-predecessor match fails native build with PlannerInvariant, not a typed cause (Adversary `evt_62xdq24yeyew9`); the recursive checked HostIO declaration lowering counts erased Omega parameters the call site drops (Adversary `evt_1v9hkpbrm5khz`); both fail-closed; the RT-P1 tail walk does not refuse every forwarding form inside a callee, latent until imported declarations lower natively (Adversary `evt_7rcr2ge1bexpg`); Library-route body view refuses an auxiliary `<owner>#0` reference (`Failure 7`) as MissingDependencyIdentity, fail-closed, and the Library route is not a lowering route today (`evt_3bb2dnvj5mtd2`); RT-C6 siblings, type-former function values and strict field-type comparison in the cast push (Adversary `evt_5ts91xbdyhkfe`; fail-closed; the field-type half needs a design ruling); the dormant `ResponseDisposition::BracketOwned` projection (fails closed); per-instance settlement key witness, release-fault parity, Transfer necessity, GroundResult (`evt_3r41y1y7e4x20`). Held refs `4b4c8565c`, `21c039918`, `7f1a04a40` never moved or landed |
| L2 | language | One resolution mode: strict admits only the minimal prelude, legacy passthrough deleted, binding a prelude name anywhere (incl. local binders) is an error (operator 2026-09-25) | `LANG-RETIRE-STRICT-ROOTS-ALIASES` merged (`df472e411`); `LANG-INSTANCE-CLASS-NAME-SPAN` merged (`24f47951f`); `LANG-DELETE-LEGACY-RESOLUTION` merged (`fcd7e8156`); `LANG-STRICT-LOCAL-BINDER-BUILTIN-REFUSAL` merged (`9cf91c90f`). No active WP: the objective is complete. Earlier landings: git log and WP threads. | Objective's three clauses have landed; next L2 objective is an operator question (posted). Carried, not framed: `InstanceConstraint` class-name span; a private-class instance row pinning `rewrite_rdecl`'s span (Architect GAP); Perm/is_sorted; "Strict" wording in test messages; stale `FsCap` and `runState` spellings. Verify ring idle. | Waits on the operator fork `evt_gg337crdbj8g`. |
| L3 | foundation | Grow the catalog along `docs/program/06-catalog-campaign.md`: post-Data layers in roadmap order, top-informed and bottom-proven, every package fully proved (operator 2026-09-13). Operator 2026-10-08: "start l3 on the establish catalog program." | `SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTE-ABSENT-VARIABLE` merged (`8cf27bd2c`). `SPEC-OPTIMIZATION-LEAST-K-SATURATED` merged (`d42f7ad28`); carry: vacuous trust comparator in `cat_optimization_least_k_prefix.rs`, and the stale SPEC-PROGRESS §4.6 child sentence, on the next Optimization touch. Earlier landings: git log and WP threads. | `SPEC-GEOMETRY-ORIENT-AXIS-COLLINEAR` merged (`f3ecab2b9`). `SPEC-LINEAR-ALGEBRA-ADD-CANCEL` merged (`15e57bf39`). `SPEC-NUMERICAL-INTERVAL-WIDTH-ZERO-MEMBERS-EQUIV` merged (`21d4c6ccd`). `SPEC-TIME-MERGE-DISTINCT-TIMES-COMMUTE` merged (`0f7c7761f`). `SPEC-SYMBOLIC-ALGEBRA-EVAL-COINCIDENCE` merged (`82b9b2d28`). `SPEC-OPTIMIZATION-LEAST-K-APPEND-MERGE` merged (`99ddd22e3`). Active: `SPEC-GEOMETRY-BOX-AREA-POSITIVE-CONTAINS-CORNER`, M, T1 (positive area gives the low corner; Architect AC-0 first). Next: Linear algebra, framed when Geometry nears landing. Layer rotation: Optimization, Geometry, Linear algebra, Graphs, Numerical, Time, Symbolic algebra. Carried, not framed: NatBox and Rational private Nat scaling duplicates (Architect `evt_aha7hvm88ckz`); public Core `bool_eq_from_iff` and `bool_or` commutation, retiring the Rational and Dependency private copies, framed when a third law or a public client needs them (Steward `evt_4mn4f5jwytgew`). | DOC-BUNDLE-494 lands; then kick Geometry for Architect AC-0 (ring compacted at `99ddd22e3`). |
