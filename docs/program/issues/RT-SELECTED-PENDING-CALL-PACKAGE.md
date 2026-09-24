---
id: RT-SELECTED-PENDING-CALL-PACKAGE
title: "design, before any code, a typed activation-owned selected pending-call package carried beside the pending ITree word from the executed source arm through the six transport families to a one-event consuming call gate, with a runtime call-event authority, so the four px7l/px7m rows have a legitimate package to check; bounded D0 census of the present substrate first, then a D1 pre-code specification; STOP and park only if D1 cannot specify owner, extent, identity or a complete route"
status: closed
owner: runtime
size: L
gate: architect
tier: T1
depends_on: [RT-INVOCATION-RESOURCE-PRECURSOR]
blocks: []
github: null
origin: "Operator 2026-09-23 ~15:00Z: 'concur with rec on L1 carrier', authorizing the new runtime representation the Architect named in evt_7v8he9tyxftp6 after the RT-SELECTED-ARM-CALLABLE-CARRIER D2 STOP (evt_6zf0n10f0djzs). Steward-filed per COORDINATION section 2."
---

# The selected arm needs a package the call can check

## Settled inputs -- Architect `evt_7v8he9tyxftp6`, at `a64aa66e8`

- No carrier design exists on the present substrate. The executed arm is not
  represented as an activation-owned call authority across the value-only
  boundary. The planner's `producer_alternative=1` is shared by both arms.
  `ContinuationActivationId(0)` is compiler-local. The join, bind/project,
  recursor/resume and generated return lose the source ticket.
- Rows, fixtures and the six transport families are as recorded in
  `RT-SELECTED-ARM-CALLABLE-CARRIER` and `evt_4j4rakqyr5g2w`. The
  borrowed-span tuple (`evt_51cgf00j10a9p`) is a direction, not a design.
- An allocation failure today travels `BOUNDARY_ERR_CAPACITY=-6 -> -1 ->
  UnclassifiedRuntimeTrap`, which is not a typed fault.

> **Resumes after `RT-INVOCATION-RESOURCE-PRECURSOR` lands (operator
> 2026-09-24).** The first D0 census below was attempted and ended in the D1
> STOP (`evt_4rgbwg2vgk6kc`). On resumption, D0 is a capture of **body 322's
> own six S/C members** on current main; sibling 343's 3+3 frame is no
> substitute. D1 then proceeds on the precursor's resources.

## Carried from the precursor D2 (`6653de61d`; Architect `evt_gawqf96vgdzw`)

- The call-event authority now exists. D2 added an activation-owned,
  bounded, fixed-backing issuer. Tickets carry {epoch, generation, slot,
  exact selected body/call target}. The consuming gate is at
  `calls.rs::call_declared_unit_target`, before frame packing and operand
  loads. D1 **uses** this issuer and gate. It does not specify a second
  authority. Its event generations and live slots are the profiled
  resources; any other new resource is still a STOP.
- **Borrowed operands load after the consume check.** D2's operands are
  computed locally before issuance. Once a copied pending value carries
  borrowed operands, D1 must emit their loads **after** the same consume
  check, never while building the call inputs.
- **Nested-unit gate witness.** The later implementation adds a witness
  that the gate holds for a selected call inside a nested unit, not only at
  top level.

## Deliverables

- **D0, bounded census of the present substrate** (Architect
  `evt_4chkm2xxyfang`; the first D0 demanded a package that does not yet
  exist). Scratch instrumentation only. Identify the executed arm versus the
  sibling, the producer, `L1` and word-only loss points, the available
  operands and their owners, and the first refusal. Report the positive
  package and its mutations as **not yet runnable**, not as green or red.
  The baseline is `evt_30dybw9824mt`: the only frame is sibling 343's, and
  body 322's S/C values are never materialized.
- **D1, pre-code specification to the Architect.** Specify the package:
  - **Placement.** It travels beside the pending `ITree` word. Not inside
    `CarriedBoundaryWord` and not in the frozen `LoweringOperand` sum.
  - **Contents.** The executed-arm construction site for body 322's
    **own** ordered S/C values; relabeling 343's frame does not count. Name
    each member's owner, representation, extent, same-activation lifetime
    and backing. Owning the package does not establish who owns each
    borrowed span.
  - **Transport.** Its route through all six families, ending at a
    one-event consuming gate that re-declares and authenticates the target.
  - **Call-event authority.** Use D2's issuer and gate; do not specify a
    second authority. Say where the selected arm's ticket is issued (after
    selection executes, so nothing is issued for the sibling), how the
    ticket travels with the package through all six families, and that
    the one consume is D2's gate in `call_declared_unit_target`. Map the
    refusals onto D2's typed faults: a duplicate is `Spent`, an expired
    ticket is `StaleGeneration` or `WrongActivation`, and a wrong arm is
    `WrongTarget`.
  - **Live-slot lifetime.** In D2 a slot is live only between issue and
    the adjacent consume. Here it stays live from selection to the call,
    and D2's issuer frees a slot only by consuming it. Bound how many
    tickets can be live at once in one activation against
    `live_pending_slots`, and show that every issued ticket is consumed
    on every path or the activation terminates. If a path needs to give
    up a ticket without calling, specify a release on D2's issuer that
    frees only a live slot of the same epoch and never returns a
    generation. That release is D1 content, not a new resource and not a
    STOP.
  - **Effects.** Nothing takes effect before selection, and there is
    exactly one call.
  - **Later proof.** Say how the implementation will produce a positive
    selected-arm witness and independently turn the sibling, duplicate and
    missing mutations red at the real call gate.
  - **Lifetime.** A proof that it cannot outlive its borrowed bytes or carry
    foreign SSA handles.
  - **Allocation.** Say whether it needs a metered allocation.

## Acceptance criteria

- **AC-1.** The D0 census is posted in the WP thread. The positive witness
  and the mutation reds are obligations of the later implementation frame.
  This node does not claim them.
- **AC-2.** The D1 design keeps `L1`/`L2`/`L3`, one-unit `L2` and
  `gather_cannot_serve`. It does not force `Vis` or move the context call.
- **AC-3.** No production code lands from this node. A later build node is
  framed only after the Architect approves D1.

## Stop conditions

- If D1 cannot specify a representable owner, extent, identity and
  complete route, that is the §5 STOP. The Steward parks the attempt. A
  record name with no production site or transport is not D1.
- Operator 2026-09-24 authorized the resource fork as
  `RT-INVOCATION-RESOURCE-PRECURSOR`. Once it lands, this node consumes its
  bounded resources and typed terminal path. A genuinely new, unprofiled
  resource is still a STOP.
- **Held work:** never move `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the
  child-2 checkpoint.

## D1 disposition (Architect `evt_3y5xkyf1v02dj`, at `1303e5cd4`)

**REPRESENTABLE D1, no §5 STOP.** Bound to main
`1303e5cd48aa0bc2b615060ede7c81b67c6b81c7` and D0 `evt_3ywcecyzbyqkh`. It is
not an implementation authorization; the build node is
`RT-SELECTED-PENDING-CALL-BUILD`.

Measured facts it rests on, all at `1303e5cd4`:

- **Issuer and arena share one lifetime.** The issuer
  (`invocation_tickets.rs`) and the boundary arena are both owned by
  `BoundaryActivationV1` (`boundary_activation.rs:159-176`) for the whole
  invocation. `BoundaryArenaV1` has no reset path. The only `reset` is the
  interpreter store's (`store.rs:323`), with no generated-code caller.
- **The ticket is a stack slot today.** `issue_selected_call_ticket`
  (`calls.rs:1920`) writes the ticket into a stack slot of the issuing
  function. `consume_selected_call_ticket` (`calls.rs:1948`) takes a ticket
  pointer.
- **The gate consumes first.** `call_declared_unit_target` (`calls.rs:1999`)
  consumes, then packs a caller stack-slot frame from words.
- **One-word transports.** Joins carry one CarrierWord param
  (`joins.rs:375-379`), and strict Ret's `return_body` carries one word
  (`core.rs:14417`).
- **Profile.** The starter profile has `live_pending_slots: 64`.

### 0. Admission: which population takes the route

- A package is planned only for a pending producer `Match` whose arms
  declare **different** recursive body units: the exact population L2
  (`agreeing_recursive_body_unit`, `core.rs:1230`) refuses today.
- The predicate is **called read-only; L2 is not edited.** Every producer
  L1/L2/L3 admits today keeps its current route, including 343's
  constructed-frame route, `gather_cannot_serve` and the retarget route at
  `calls.rs:~1016`.
- The planner must also prove the route **linear and must-reach**: on every
  path from the arm the package reaches exactly one consuming gate, or the
  activation terminates.
  - A pending value read twice (`let p = body MkUnit in bind p (\_. bind p
    k)`), a route that reaches the generated root, or a member with an
    unclassifiable backing (item 7) is **refused at compile time with
    today's BoundaryCarrier refusal**.
  - This settles the copied-pending-value question. A legal program that
    uses the value twice gets the existing unsupported refusal, never a
    runtime `Spent`.

### 1. Placement

- A companion `PendingCallPackage` rides **beside** the pending operand. It
  lives in the lowering environment binding and `RoutedAnswer` as a
  parallel field, and in the join or return plan as extra planned params.
- It is **not** a `LoweringOperand` variant and **not** inside
  `CarriedBoundaryWord`. The pending ITree word itself is unchanged.
- Contents:
  - `plan: PackagePlanId`, a static id naming the candidate set, meaning
    the arms' declared body units;
  - `function: FunctionId`, the function whose DFG defines the words;
  - `ticket: [Value; 5]`, holding epoch, generation, slot, target.body and
    target.callee **as value words**;
  - `members: [Value; W]`, where W is the static maximum member count over
    the candidates. It is 6 for 322 and 6 for 343, so 11 words with the
    ticket, and shorter arms zero-pad.
- **A stack-slot address never enters the package.** Right after `issue`
  returns, the arm loads the 5 ticket words out of D2's slot into SSA
  values.

### 2. Contents: body 322's own ordered S/C

- Construction site: the selected arm's own block, dominated by the arm
  edge of the dynamic Bool `brif` (`core.rs:6227ff`), at the lowering of
  closure 326.
- It is a **distinct** `assemble_continuation_call_operands` call keyed on
  322's `ContinuationSpecializationId(0)` and `ContinuationContextId(1)`. It
  is **not** 343's frame, which stays keyed (continuation origin 11,
  position 1, worker 343).
- Member order is the declared order [S0, S1, S2, C0, C1, C2], checked
  against 322's declared header of 4 parameters and 3 captures.
- Each member is one `ValueWord`, 8 bytes, produced by
  `transfer_into_carrier` in the arm (a read, not an effect).

| Member | Source in the arm | Owner of the word | Lifetime |
|---|---|---|---|
| S0, S1, S2 | closure 326's lexical captures, origins 325, 324, 323 | PredeclaredFunctionId(3)'s activation | SSA until the gate, or a frame slot across a call |
| C0 | ProducerLocal 358 resolved in `producer_env` at the arm | same | same |
| C1, C2 | EntryAbi params 0 and 1 of function 3 | same | same |

C0 is recoverable here because the arm block is exactly where the claim has
a value, which is why L1 (`context_capture: None`) cannot recover it at
resumption.

**Pointee ownership is per member and classified at construction.** The
admitted classes are:

- (a) immediate scalar;
- (b) static or persistent-image pointer;
- (c) boundary-arena pointer, which lives for the activation;
- (d) capability token;
- (e) native-stack-backed, admitted **only** if the static route never
  leaves the owning function's activation by a return (families 5 and 6
  outward).

Anything else refuses (item 0). The implementation's first checkpoint
reports each of the six members' class as measured.

### 3. Transport through the six families

- **F1, arms (`core.rs:6227`).** Build the package in the arm block, then
  issue.
- **F2, joins (`joins.rs:351-418`).** `JoinPlanToken` gains a planned
  companion width. `jump_planned_join_arm` appends `5+W` params, and the
  no-package arm of a mixed join passes a zero ticket (item 5).
  `finish_planned_join` rebuilds the companion from the block params.
- **F3, Let, Var and project (`core.rs:15308-15352, 15640`).** The companion
  moves with the binding. It is moved, not copied: a second read is the
  item-0 refusal.
- **F4, recursor and decomposition, then the residual call
  (`mod.rs:9273, 12409`; `core.rs:3116`).** The residual inherits its
  scrutinee's companion. The gate is in `lower_recursor_residual_call`.
  - A new branch is taken **first**, only when a companion is present.
  - Otherwise the code is byte-for-byte today's: the `recursive_unit_body`
    one-unit call, then `reject_carried_residual_arguments`.
- **F5, strict Ret `return_body` (`core.rs:14417`).** Add `5+W` companion
  params to the shared return-body block.
- **F6, generated unit call (`calls.rs:1999-2013, 2214-2327`).**
  - Inbound: declared companion Parameter-kind slots.
  - Outbound: declared companion `Result`-kind slots in the unit's frame,
    written by the callee before return and loaded by the caller **after**
    its status check. They are fresh SSA in the caller, re-tagged with the
    caller's `function`.
  - Frame bytes are static and plan-sized, a caller stack slot like
    today's frame. **That is not a new runtime resource.**

**The gate.**

- The gate loads the untrusted copy of `ticket.target.body` and dispatches
  over the plan's **static** candidate set: one block per candidate unit
  `u`. In each block it spills the 5 ticket words to a gate-local 40-byte
  slot and calls `call_declared_unit_target(u, [residual args...,
  members...], Some(slot_ptr))`.
- The existing consume runs first and checks the copy against `u` and the
  issuer entry against `u`, which **re-declares** the target from static
  code and **authenticates** it from the issuer.
- A tag matching no candidate calls consume with expected target {0,0},
  which yields a typed `WrongTarget` and consumes nothing.
- Members are passed as already-carried words and stored into the frame
  after the consume. The callee alone dereferences them, so **every
  borrowed load follows the consume** by construction.

### 4. Call-event authority

- **Only D2's issuer.** It is issued once, in the selected arm block after
  the `brif`. The sibling's block never runs, so nothing is issued for the
  sibling. The target is (322 body, 322 callee).
- The single consume is D2's gate inside `call_declared_unit_target`. The
  faults map as follows:

| Case | Fault |
|---|---|
| duplicate | `Spent` |
| earlier issuance of a reused slot | `StaleGeneration` |
| other or zero epoch | `WrongActivation` |
| wrong arm, or forged tag | `WrongTarget` |

- **Live slots:** one per open package, from issue to its gate. The gate
  consumes **before** unit 322 runs, so recursion inside 322 never overlaps
  its own ticket. D2's direct worker tickets inside the window add at most
  one transient slot each. Overflow is D2's existing typed
  `CapacityExhausted{LivePendingSlots}` terminal, which is profiled and
  never silent. For the fixture route the peak is 2 of 64.
- **Release: none is added.** Admission rule 0 means every issued ticket
  reaches its gate or the activation terminates. If a later route needs to
  drop a package, the pre-shaped release on D2's issuer is:
  - it runs consume's checks for epoch, slot, generation, live and target;
  - it sets `live=false` and never decrements or returns a generation;
  - it performs no call.

  It lands only through a D1 amendment reviewed by the Architect.

### 5. Effects and exactly one call

- Before selection there is nothing: construction reads values, and issue
  follows the arm edge. The Vis placement and the context call are
  unchanged (AC-2).
- **Exactly one call:**
  - statically, each linear route has one gate;
  - dynamically, one-use consume allows at most one call per ticket;
  - a no-match or failed consume branches to the typed terminal before any
    frame store or call.

### 6. Later proof, as obligations of the build frame

- **Positive:** run the ignored px7l fixture natively. Assert the selected
  arm's host effect once, one issue and one consume for the selected
  target, and zero issues for the sibling target. Add a sibling fixture
  variant (`match (not terminal)`) so the **other** candidate's positive
  path is also observed. Also add the carried nested-unit gate witness.
- **Independent mutations at the real gate:** each alone, with the others
  off and the same-harness positive control green.
  - sibling: issue the other arm's target while carrying 322's members
    gives `WrongTarget`;
  - duplicate: the existing `DuplicateConsume` gives `Spent`;
  - missing: suppress the arm's issue and carry the zero ticket, which
    gives `WrongActivation` because epoch 0 is never minted;
  - forged tag: flip `target.body` in the copy gives `WrongTarget`.

  Each red must show `selected_call_integrity: Some(..)` with no host
  effect after selection. A compile-time refusal is **not** a red for
  these.

### 7. Lifetime and allocation

- **No foreign SSA.** Every use site asserts `package.function ==
  defining_function_id`, a fail-closed backend error. Words cross functions
  only by store into a declared frame slot and load on the other side (F6).
- **No outliving borrowed bytes.** Classes (a) to (d) are backed for the
  whole activation, and the issuer lives for the same activation, so no
  ticket of this epoch outlives them. Class (e) is admitted only when no
  return leaves its owning activation. The generated root is refused, so no
  package crosses activations.
- **Metered allocation: none.** The package lives in SSA, block params and
  static frame slots. The only runtime resources are one live slot and one
  generation per selection, both already profiled. **There is no new
  unprofiled resource, hence no STOP.**
