---
id: PX10-NATIVE-SPAWN-PARITY
title: "PX10's spawn and wait run only on the interpreter: the native bracket planner keys every region on an acquire in head position, so withChild, whose acquire follows the plan-open response match, has no return fold and no owned settlement. Give the native planner bracket regions whose acquire is not in head position, then promote the PX10 host operations to NativeTested with interpreter parity"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX10-SPAWN-PLAN-WAIT, PX10-NATIVE-SITE-IDENTITY, PX10-NATIVE-BRANCH-ALTERNATIVES]
blocks: [PX10]
github: null
origin: "PX10-SPAWN-PLAN-WAIT recut interpreter-first (Steward evt_15ra8z0j0typg) on the Architect's measured native wall (evt_4asjwp948dzqh). L1 objective: the Linux ABI program. This is PX10's native-parity slice; pidfd signalling is the next. Steward-filed per COORDINATION section 2."
---

# Native spawn and wait match the interpreter

## Objective

A natively compiled Ken program spawns, reads and waits for a child
through `withChild` exactly as the interpreter does. The PX10 host
operations are `NativeTested`, and the native bracket planner accepts
a bracket whose acquire comes after stuck effects.

## Settled inputs (measure on `58c2e1df6`, which includes PX10 slice 1)

- **The wall.** `build_bracket_static_regions` takes the positional
  return fold as `folds.last()` under the acquire's continuation
  (`brackets.rs:583` on `903b06068`). For `withResource` the acquire is
  the head `Vis`, so the caller's fold fuses into it. `withChild`'s
  acquire follows the plan-open response match, which is stuck on a
  neutral response, so the fold stays an ancestor (Architect probe B,
  evt_4asjwp948dzqh).
- **The first link is measured.** The Architect's `return_fold` up-walk
  selected the same fold as the downward search on all 108 existing
  region builds in four suites, and found the fold for the PX10 smoke.
- **The next wall is measured.** With the up-walk applied,
  `returned_vis.rs:491` refuses the smoke: the settlement's owner is the
  last `push_stdio` `Vis`, outside the region.
- **The predicate.** The planner keys region structure on lexical
  containment under the acquire's own continuation. Every stage that
  reads that shape is in scope: the return fold, settlement ownership,
  body exits, and the parent forest.
- **Slice 1 pins the native refusal.** Its AC-4 row asserts that a
  native `withChild` build refuses through `RepresentedUnavailable`.
  This WP inverts that row into the AC-1 parity row; it does not delete
  it.
- **Carried from slice 1, in scope where native:** an unused Process
  grant refuses a native build, and `prepare_native_target_sources`
  reaches lowering without the availability gate. AC-0 rules whether
  each closes here or is named as a successor.
- **The interpreter trace drops two settlements** (Adversary
  evt_2ffcbyp8azkf0, on `58c2e1df6`). A pipe the `withChild` bracket
  closes is acquired but never settled: `settle_child_pipes`
  (`effect_v1.rs:1634`) discards each Ok observation, and the pipe close
  at `:2209` cannot report `ReleaseFailed`. A `SpawnPlan` retired by
  `ProcessSpawn` (`:3777`, `:1608`) is never settled. A body close and
  terminal finalization (`~:2236`) both record a settlement, so the
  outcome depends on the exit path. Parity measured against this
  baseline would agree with a native path that drops the same records.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the cut base).** Rule unfused-region
   support across the region builder and the settlement plane. Name
   each stage that reads the fused shape, give its repair, and give its
   negative.
2. **One runtime branch.** It first records exactly one settlement for
   every acquired PX10 identity on every release path, on both engines.
   It then carries the ruled repairs, promotes
   `ProcessPlanOpen` through `ProcessWait` and the two stream operations
   to `NativeTested` with their catalog columns, and adds native tests.

## Acceptance

- **AC-1 (parity).** PX10 slice 1's AC-1 rows pass natively, and
  `rt_parity_native` reports the same (request, outcome) pairs as the
  interpreter. This includes native run parity for the smoke,
  argv_env_pipe and live_child fixtures, which
  `PX10-NATIVE-BRANCH-ALTERNATIVES` brings only to native build
  (Architect evt_4qb22p4991jmm). Those fixtures issue 3, 4 and 4
  producer alternative sets, so these rows are also the labelled
  transport's execution parity (evt_573z7twxznjt9). A mutant that swaps
  two members' labels at the writer reddens one of them, and a mutant
  that writes the bare payload without its label is caught by the issued
  reader of `RT-NATIVE-PRODUCER-ALTERNATIVE-ANSWER-READER` (C3), not by
  the seat's assertion, which only validates (evt_7xv7216msypg). An
  ordinary consumer handed a labelled answer is refused at compile time.
- **AC-1b (settlement).** For a `withChild` body that takes stdout and
  waits, the trace settles the `PipeRead` and the `SpawnPlan` exactly
  once each, before the child, in reverse acquisition order, on both
  engines. A spawn that fails settles its plan. A mutant that drops the
  bracket's pipe settlement reddens this row.
- **AC-2 (no regression).** The return-fold equivalence pin holds on
  every existing bracket fixture. `rt_native_bracket_body_once`,
  `px7f_resource_native`, `rt_escape_second_resource_native` and
  `rt_bracket_control_region_d0` stay green.
- **AC-3 (negative, QA).** A dispatch reached through a fold's `Vis`
  case, or through a `Let` value, still refuses. A mutant that accepts
  either reddens a named row.

## Increments and §1b inventory

Steward rulings evt_3yrmbfdkws1wx and evt_6kmhp6k18ypq5:

- **Increment A** is the settlement repair (R1) and AC-1b on the
  interpreter. It lands on its own.
- **Increment B** is R2, R3, the recipe rows, the promotion, the gate
  deletion and native parity. It also deletes BRANCH-ALTERNATIVES'
  AC-2g containment once its parity rows exist. It waits for
  `PX10-NATIVE-SITE-IDENTITY` and then `PX10-NATIVE-BRANCH-ALTERNATIVES`.
  It lands as B-closure, then B1 (the F1 and F2 repairs and the starter
  trap-classifier fix, evt_5y7e328rr2wd8), then B2: the Process
  promotion, the gate deletion and the containment deletion, landing
  with the native parity rows (Architect evt_5vsaybryes08b).
- **Increment B-closure** is dynamic relay forwarding (Architect
  evt_146ymmv8mz9py, on Research evt_91y97hbf0w06). It lands on its own,
  after `RT-NATIVE-IH-CONTINUATION-CLOSED-TEMPLATES`: a first-order
  backend resumes a foreign relay's K only from a closed template set,
  so B-closure forwards by operation tag and resumes from that set
  (Steward evt_6ppgz0hsqxmm, Research evt_1ebcpy76v1e9a, routing D0-23
  evt_jc75x84wmcw7). That WP merged at `45074052a`; checkpoint
  `2ad1b31d0` resumed on it (evt_47r0prwyrhxb1). Stop 11 found that a
  labelled transport answer has no reader (evt_7xv7216msypg), so
  B-closure is parked at `5cd38e8b5` on
  `RT-NATIVE-PRODUCER-ALTERNATIVE-ANSWER-READER` and reruns AC-1 after
  it lands. With compose, argv, live and null on the standard profile,
  the run records `CapacityExhausted{Invocation, Words, 256, 269}`.
  Remeasure after the closure; if demand still exceeds 256, choose
  between raising the AC-1 rows' profile and reducing the demand.
  - Every relay member of every protocol is numbered by its relay
    construct's origin (argv and null: 1247, owner 2, context 0). Its ABI
    is the carried operation word, argument 0 of the relay's own Vis, plus
    the relay K's captures. `PendingVisRecordProtocol`'s "only authority"
    sentence is amended for this member kind only.
  - Ken's handler Match is already the operation-keyed, closed dispatch:
    `host_response_routes` keys each arm, a `Let (Effect op ..)` with its
    response call, by the arm's operation constructor, and
    `selected_host_response_route` picks one arm per Vis at plan time
    (Architect correction evt_mq84ht59na9f). For a relay member, the arm
    is selected at run time from the carried operation word's
    constructor tag, and each arm's Effect keeps its existing seats.
    There is no new dispatcher table. The path set is the one handler
    leaf holding every non-relay member effect of the owner: leaf 953 for
    argv and null and leaf 394 for live, 30 arms each (D0-14). A member
    set spanning more than one leaf is a planner refusal. Every path is
    unique-or-error, and an unmatched tag takes the deterministic failure
    edge before any host request is built.
  - The resume reads no value and no effect from a response row
    (Architect evt_7cm821pbn97vt, evt_7r0mddesaz38j, on Research
    evt_szf6w3793fx4). A relay is its own member kind, numbered by its
    origin, never a `members` row. The planner's verdict names a row only
    as the code, its K context function, and the slot order; a relay with
    no seat or row resumes to a planned, named trap. Each handler arm
    runs its own Effect, chosen by the carried operation's tag. The K is
    a flat closure: its record holds the captures and then the K body's
    entry inputs, and the K word is never called (`core.rs` Carried
    refusal stays).
  - No relay member is removed from numbering. Drive pairs remain a
    fusion optimization: a fused source never builds its Vis, and any
    Vis that is built reaches the numbered relay and forwards. Admission
    and its reached-relay guard are deleted, and
    `ExcludedSettlement::Relay` loses its producer (Architect
    evt_4151apntyfsmt, after D0-15 showed no plane can prove a source
    set complete).
  - **Acceptance (Architect evt_5vsaybryes08b).** B-closure leaves the
    Process gate (`compiler_driver.rs:2997`), `HostOpV1::availability`
    and AC-2g (`compiled.rs:123`) as they are; the run-time refusal in
    `require_native_operation_v1` (`abi_v1.rs:820`) means a native
    Process run needs the promotion.
    - Landed: `px10_relays_are_numbered_protocol_members` in
      `px10_plan_census_tests.rs`, the Architect's text, pins argv's
      relays 1247 and 992 and live's 688 and 433 as numbered relay
      members kept in the settled member set, and asserts that 1247's
      and 688's successors are the rows whose Vis are 1484 and 932.
      Base `928313aa6` is red on it, with the reason recorded; a green
      base is a stop.
    - Landed evidence: full-workspace CI green on the exact candidate
      SHA, which runs the 17 D0-14 binaries (recut evt_47r0prwyrhxb1,
      Architect concurrence evt_3pwc46wwjpyma); px10's stack overflow is
      reported from its named target as base or new.
    - Scratch evidence, never committed: under the Architect's overlay
      (`unit-b-overlay.patch`, md5 629a493c) and the D0 harness
      `arch_spawnb_d0.rs`, the native result equals the interpreter's on
      argv, null and live; today they return -8500008, -8500008 and
      -8004096. Mutations under the same overlay: restoring relay
      removal for paired owners returns live to -8004096; restoring
      `Excluded{Relay}` for unpaired owners returns argv to -8500008;
      fixing dispatch to the first arm turns argv's parity red.
  - **Unmeasured, named.** Relays that are no owner's protocol member are
    built unnumbered (1171 in argv specializations 1 to 11, 612 in live
    specializations 2 to 12). AC-1 parity covers them; a failure there is
    a stop to the Architect.
- **Later, not part of parity:** `RT-NATIVE-VIS-MAY-YIELD-SUMMARY`, one
  shared per-function may-yield summary that licenses static fusion.

The §1b inventory (Architect AC-0 rounds 1 and 2, evt_1jd9djf6thfea and
evt_1jd8yzwewmtf6; Increment B-closure, evt_4z5rz8cs4nw8c,
evt_5kjjy0w0gzma, evt_p37pc6dwdk3h, evt_qkp7qcj87b3q and
evt_3nvw9veeqa274; entries 11 to 13, evt_6ap56syy0smt6,
evt_7cm821pbn97vt and evt_1339kvspms2zv) has thirteen entries:

1. W1, the returned-Vis settlement is checked against the protocol's
   root owner. Keyed on the emitting protocol; R3 closes it.
2. W1', `aggregates.rs:10581`: `host_effect_recipe_tree` gives the
   seven PX10 operations no recipe. Keyed on a second operation list;
   the recipe rows close it. This entry is independent of the others.
3. W2, `responses.rs:2030`: a finalizer emitted inside a closure cannot
   reach its selecting match. Keyed on the emitter's position.
4. W3, `responses.rs:3825`: a mixed-owner transport stage. Keyed on the
   emission unit.
5. W4, `lowering/calls.rs:1861`: a curried chain body is called from
   its emitting unit, not its static caller. Keyed on the emitting
   function.

6. The detached seat's Alternatives arm accepts an issued producer
   alternative set as discharged when no member has a labelled writer
   and no destination consumes the set. Keyed on the planner's set,
   (identity.emission_owner, result), not on what any function emits or
   forces.
7. Narrowing the transport-source population for the transport and set
   planes also narrowed the response plane's P2 deferral and the
   process-carrier domain, which read the same set as a caller whose K
   is consumed outside its emitting body. A bracket region got two
   returned-exit owners. Keyed on pre_schema_transport_sources
   membership, read under two meanings.
8. Narrowing the transport records removed the detached-result seat's
   single-member pass-through (`core.rs:12913`) for those identities,
   and five lowering mode switches read transport-source emptiness.
   Keyed on transport-record existence, read as escape crossing.
9. The relay classifier's source traversal refuses a Call that is not a
   direct lexical-closure redex, so argv's and null's relay 1247, whose
   sources sit behind `drain_pipe`, is excluded. Keyed on the producing
   Vis construct inside one function owner.
10. A relay's continuation was resumed as though it were a lexical
    closure with a planned response row, but it is the IH of a recursive
    fold, carried as a runtime word. Keyed on the K being statically
    known: an identity read off the lexical template for a value the
    runtime boundary carries.
11. The relay resume read its row's continuation inputs in the relay's
    own construct context, where D0-17 measured 0 of 20. Keyed on the
    response row's context as the relay's.
12. The relay's resume verdict was placed after the static planner,
    although the planner's settlement consumes it, and the verdict was
    installed as a `members` row. Keyed on the construct-place
    response-row plane.
13. The relay resume took its host effect and its K context's
    continuation inputs from the resolved response row, but the relay
    carries only its operation word and its K environment word. Keyed on
    the response row as resume authority.
14. The closeout's required set was keyed on the row's effect and
    argument roots, making the row the lowering authority. This shares
    entries 10-13's predicate.
15. A returned Vis that is both a pending-Vis successor row and a
    checked-IH transport destination is lowered by whichever arm runs,
    honouring only that arm's plan. Two planner planes claim one causal
    edge, with no arbitration (withdrawn at D0 round 14, reinstated at
    round 19 after a check-8 miss). Measured from the transport side,
    the labelled answer from core.rs:9455 reaches an ordinary Result
    match with no label reader, because the operand plane does not
    carry labelled-ness. This shares entries 1 and 3-6's predicate.
    The closure is C1-C3.

Entries 10 to 13 share one predicate: the relay's resume state is located
through the response-row plane, but its identity is the runtime-carried K.
The closure is a resume that reads nothing from any row
(evt_7r0mddesaz38j). Entries 1, 3, 4, 5 and 6 share one predicate: an
identity read from one plane and checked or consumed under another key,
such as the emitting unit rather than the static owner. Entries 7, 8 and 9
carry entry 6's predicate. B-closure sidesteps it for parity by forwarding
on the operation's identity, and `RT-NATIVE-VIS-MAY-YIELD-SUMMARY`
replaces the per-plane walks.

## Stop conditions

- A repair that needs a kernel or trust change, or a post-fork callback.
- A sixth advancing stop on the native-lowering chain. Its §1a count is
  5 at this recut (evt_146ymmv8mz9py), and a sixth gets the Architect's
  marginal-value re-ask to Research.

Checks: 4 fired: the fold search and the settlement refusal were read at
`brackets.rs:583` and `returned_vis.rs:491` on `903b06068`. The up-walk's
equivalence and the next wall are the Architect's measurements, not
inferred. 8 fired: AC-1 compares engine traces pair by pair, so a native
path that settles in a different order fails, and AC-3's mutant shows
that the refusal is not lost.
