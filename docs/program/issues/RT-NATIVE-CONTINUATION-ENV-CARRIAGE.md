---
id: RT-NATIVE-CONTINUATION-ENV-CARRIAGE
title: "A native function-typed recursive position whose closure escapes through a word-only call result gets its captures from a compile-time side slot, so two constructions of the same continuation cannot be told apart. Carry the suffix as fields of the residual word, in defunctionalized form"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [RT-PLANNER-PER-EMITTER-AVAILABILITY]
blocks: [RT-NATIVE-SEQUENTIAL-BRACKETS]
github: null
origin: "Architect recut ruling evt_4x84ykwjtmbsh on RT-NATIVE-SEQUENTIAL-BRACKETS (§1a 2), on the research advisory evt_7rqnbfnw80fe: an escaping continuation's environment must travel in the value (closure conversion). Size is provisional and is re-set at AC-0. Steward-filed per COORDINATION section 2."
---

# Continuation environments travel in the value

## Objective

A native continuation closure that escapes through a word-only call result
reaches each consumer with the captures of the construction that produced
it. No compile-time association between construction and consumer remains.

## Settled inputs (T2 `evt_5ktetkd8esyeg`, Architect `evt_4x84ykwjtmbsh`)

- **The escape.** The continuation is emitted as a direct call to a
  separate function, and only words cross, through
  `call_declared_unit_target` (`calls.rs:2067`). The result returns as
  `LoweringOperand::Carried(CarriedBoundaryWord)`. Everything that
  identified the construction is dropped at that projection
  (`core.rs:11732`).
- **The consumers** both run later in the same function, on descendants of
  the carried result:
  - the gate, through `resolve_recursive_unit_body`;
  - the retarget, through `call_declared_recursive_position_unit`.

  Nothing in hand at either reader identifies the construction, and W2 and
  W7 share every static coordinate.
- **Superseded, not repaired.** The `constructed_context_frame` side slot
  is a static association in every form tried: a single slot, a keyed map
  and a scoped stack. So is a runtime "latest slot per key", because W2's
  value can still be pending when W7 executes.
- **The residual is not a closure at runtime** (disposable trace
  `evt_68f9cwzgbakf2`: three `Some(record)` templates, zero environment
  emissions).
  - The worker's `LexicalClosure` (700, 793) exists only as a discarded
    specialized template.
  - The producer Construct takes the claimed-call return
    (`core.rs:7407-7440`). The callee runs the checked-IH case body (313,
    619) under `StaticWorkerBinding`.
  - The readers' resolver picks the static body, and the captures the
    planner cannot recover come from the side slot. This is a
    defunctionalized reference with its environment beside the value.
- **The ruled form** (Architect `evt_2wywq8pmjerv8`). This is known-best
  (a) in defunctionalized form.
  - The residual word carries the captures the planner cannot recover as
    fields.
  - The resolver still chooses the body statically. It reads those fields
    from the word it already holds: the projected `children[1]` at the
    gate, and the retarget's input.
  - The side slot retires.
  - Withdrawn: the caller buffer, the seat-equality handoff, and the
    `BoundaryClosureEnvironment` extension. All three modeled the carried
    child as the source closure. Closure-producing forms keep their
    existing records.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

After an Architect-ruled AC-0, the recursive-position residual carries the
captures the planner cannot recover as fields of its word. Both readers
decode them from the word they hold, and the side slot is retired.

**Residual key** (Architect `evt_ssrh7r6dx3ty`, after stop 3 and Research
`evt_gxbcep9fm90s`). The label is the planner's continuation specialization,
which is recorded at interning (`ContinuationSpecializationKey`). Nothing is
reconstructed from term geometry.

- **Issuer** (owner-independent, Architect `evt_5jqsyq6xfkfrn`). Each
  context C issues one disposition keyed `(S.key.producer_construct_origin,
  S.key.recursive_position, S.id)` for its enclosing specialization S. It
  applies at every emission of that field, under whatever owner lowers it;
  `S.key.emission_owner` is not a filter. The source-occurrence scan and its
  constructor-identity key are deleted.
- **Closed records.** A wrapped residual carries all C context captures, not
  only the missing ones: layout `[child, W, C]`, with the label after the
  child for a size>1 gate. The gate and retarget read every capture from the
  record, never from a frame. Each capture comes from the claims
  `RT-PLANNER-PER-EMITTER-AVAILABILITY` finalizes under the emitting owner
  (`evt_3yp3tea99hfta`).
- **Transport** (rule 4, `evt_1myz2wkf66w8a`; capture source and placement
  as revised in `evt_64jeybga2y9fy` and `evt_5np6k589ze5na`). A checked-IH
  transport whose destination is the field `(P, f)` of a disposition
  `(P, f, S)`, with S its source specialization, is a materialization point.
  - **Placement.** Each transport emission: the three
    `checked_ih_transport_emissions.push` sites in lowering `core.rs`
    (`call_checked_ih_environment_transport`, the Direct and the Tail case
    transports). Immediately after `call_declared_unit_target`, each builds
    its own residual `[child = returned, label, W, C]`, with the label as at a
    construct emission. Vis735's 15 emissions build 15.
  - **W and C** are the operands that site just passed to the call; never
    re-read or re-derived. Sites 1 and Tail record
    `worker_captures: Vec<(u32, LoweringOperand)>` in their `WorkerCapture`
    arm; Direct already has `captures`. C is the continuation-input operands
    in ordinal order, as resolved through the morphism.
  - **Census.** At a transport-destination point every W and C is
    `FinalizedTransport(TransportCarriedClaim)`: `WorkerCapture { seat,
    ordinal }` or `ContinuationInput { ordinal, destination }`. Frame claims
    are not consulted there. Construct-emission points keep the frame path.
    Interning keys stay byte-identical to the d099 pin.
  - The child's own construction (Ret721) is never wrapped, and no gate
    inspects the child's constructor.
  - **D0' before building** (`evt_5np6k589ze5na`), measure only:
    1. `checked_ih_transport_emissions.push` has exactly the three sites on
       the WIP, and every read and write transport point emits at least once
       across them;
    2. the read census after reclassification (predicted: S2 owner from
       4/44 to 41/7);
    3. whether the construct-emission point holding the 7 that stay
       unfinalizable is emitted on the AC-1 read path, by static count. If
       it is, stop and name the captures and the owner.
  - **Mutation.** Swapping two W operands at site 1 (Vis735) reddens at the
    residual.
- **Labelled force** (`evt_550cpvt3p4qw5`). At a checked-IH force whose
  arriving population is a labelled candidate set, the carried label selects
  through the existing `call_selected_recursive_position_unit` switch
  (`calls.rs:830`), generalized so each arm gets its candidate's exact worker
  body; no second switch. Each arm looks up the transport with `Some(body)`:
  a transport takes its Direct or Tail route, and `None` takes the existing
  exact-body non-transport path. Every arm ends in a carried value at the
  join; no forward-Ret collapse inside an arm. The `source.rs` guard against
  `Labelled` with body `None` stays. The switch is a selection site, not a
  materialization point.
  - **D0'' before the full build:** (a) what the S3 arm's route reads at
    `env[selected_index]` and its arity; if it is the labelled residual,
    stop and report the shape; (b) dynamic per-arm reach in the write
    fixture, saying explicitly whether the S4 arm is reached; (c) swapping
    the arm order reddens the write test.
- **Gate.** The candidate set is the specializations whose key names this
  eliminator, constructor and position, sorted by
  `ContinuationSpecializationId`. A set of size 1 behaves as today. A larger
  set dispatches on the static label.
- **The gate refuses, fail-closed,** on:
  - a label out of range;
  - a field-count mismatch;
  - a candidate set that mixes specializations with and without a residual
    disposition.
- **ADR.** The WP's `docs/adr/0023-defunctionalized-recursive-position-residual.md`
  is amended in the same build. Reynolds labels are
  continuation specializations, which are creation sites recorded at
  interning.

## Carrier-schema closure (recut, Architect `evt_v9p2kbzdccb0`)

This governs the build; the deliverable bullets above are retained where they
do not conflict. On Research `evt_9cn9mr73248`, the five §1b entries share one
predicate: a recursive child's carrier slot has no planner-owned schema, so
each consumer infers what a slot holds from local geometry. A slot's
representation must be a static function of the set of values that can flow
into it.

- **I1. A planner record per slot.** Keyed `(eliminator, constructor
  identity, recursive position)`, as `recursive_residual_candidates` keys it.
  It holds the slot's flow set (the closed set of specializations or
  producers that reach it) and its schema as a function of that set. A
  singleton may elide the label, as a recorded choice; a larger set is the
  labelled sum, the label being the interned candidate index.
- **I2. One schema per slot.** Every producer writes the recorded schema. A
  producer whose natural form differs is converted by an injection emitted
  only on a planner-recorded coercion edge.
- **I3. Consumers read the schema from the record, never from the word.**
  Class, tag and arity checks stay only as fail-closed assertions that the
  word matches. This covers Direct's `Constructor`/`declared_children` check,
  the residual decoder, and the pass-through
  `checked_ih_captured_environment_from_case_environment`.
- **I4. Field reads are by role.** `Child`, `WorkerCapture{seat, ordinal}`
  and `ContinuationInput{ordinal}`, never a raw offset in a consumer. The
  transport W and C authorities are the role definitions.
- **I5. Child classification is by the record that minted the child**
  (Architect `evt_7tg07nnz0656z`). There are two populations.
  - Lexical-closure environments: the existing
    `boundary_closure_crossing_environment` arm, unchanged.
  - Checked-IH force environments: a `CheckedComputationalIHInvocation`
    child of arity 0, whose callee resolves to `InductionHypothesis(slot)`,
    stored by a construct emitted under `Specialization(u)`. It classifies as
    variant `u`, through `checked_ih_captured_environment_record
    (Specialization(u), unit(u).worker_closure_origin())`. Plan-time
    assertions: `u` is in the slot's flow, the variant's worker-capture seat
    is that seat, and the record's children equal the variant's
    `WorkerCapture` roles.
  - Anything else, including an IH application on the non-functional
    `call_static_worker` route, is `planner_error` naming the occurrence.
- **I6. The store at (S1, 526, 1) writes R's S1 variant with K8 as `Child`,**
  through the existing `slot_store_obligation`. The S1 reader arm is
  unchanged, and the slot's sum gains no label.
- **I7. Issuance ranges over source aggregate occurrences,** not per-emitter
  `ConstructEmission` points. A plan-time assertion requires exactly one edge
  per slot each slot-shaped occurrence matches. Lowering's refusal of an
  unissued store is the independent second derivation of the same set.
- **Retained.** Owner-independent labels keyed `(construct, pos, spec id)`
  and the d099 interning-key pin; the `TransportCarriedClaim` census; the
  three transport emission sites, which no longer wrap their answer; the
  label-switch reuse with per-arm
  `Some(body)` and the `source.rs:4515` guard; the gate refusals.
- **Replaced.** Every consumer-side classification in I3, and any producer
  writing a form other than its slot's schema.
- **Sequence.** The stratified plan order below governs the build.

## Stratified plan order (recut §1a 6, Architect `evt_4zwz0gmakzf51`)

The carrier schema is decided upstream of the checked-IH transport plane
whose facts it needs (§1b predicate). The ruling is stratification (Research
`evt_4rsyvjpsra877`, option 1): the transport-source population and the
selected response callers are a pre-schema plan stratum, and carrier schema,
residual issuance and Child classification consume it. The ruling carries the
code text; build from it. Two straight-ancestor increments, each landable
alone. Retained from WIP `48fdf550c`: the minting-record Child classifier,
M-c and M-d, the plane split, and issuance over the source Construct walk.

- **I-1, the pre-schema stratum; nothing consumes it.**
  - `derive_checked_ih_transport_source_population` (aggregates.rs) and
    `preselect_static_response_callers` over
    `static_response_phase_b_split_over` (responses.rs) both run just before
    issuance (construction.rs `:1435`). Preselection must not bump the
    `px8-ds-test-support` counters.
  - **(a)** After each transport build, the transport-source identities equal
    the pre-schema population. This subsumes the `:1487-1495` guard and keeps
    its message.
  - **(b)** After phase B, the preselected callers equal the installed
    owners' selected callers, and they are empty under `Ok(Err(_))`.
  - Acceptance:
    - (a) and (b) hold on rt_escape*, the r2 relay row and
      rt_span_prov_native, on base and candidate;
    - r2's set contains the D0 identity, Spec(S5) → S6 (Construct 528,
      alternative 1, position 1);
    - dropping `has_destination` reddens (a) on a named fixture or a new
      planner unit test;
    - `validate_continuation_specialization_plan` is unchanged and green.
- **I-2, a residual only under a trivial continuation** (Architect
  `evt_744gjznjr64x6`, on Research `evt_54vkrxapf12qg` and D0
  `evt_2pwxwtktqwhta`; the ruling carries the code text). Main is the
  reference. Issuance stays exactly as on `80bcfb4`. Every static-containment
  guard, including `active_descent_emits_selected_call`, is withdrawn.
  - **The guard.** At residual selection in `recursive_position_unit_body`,
    any pending outer frame on the source machine takes Active (`Ok(None)`).
    Labelled or Exact residuals are admitted only with no pending frame.
  - **The invariant.** Every residual consumption site (the Labelled switch,
    `source.rs:4579` → `calls.rs:861`, and the Exact attach, `core.rs:15225`)
    refuses under a pending frame with `unsupported("RecursiveResidual",
    ...)`. The handoff enumerates the sites.
  - **First step (passed, `evt_61rw1srg4d3at`):** the combined probe gives
    r2 1/1 with S5/528 at main's 2, read 1/1 with G12's residual, and write
    1/1 with S5/528 at 12. The residual stays at exactly the three
    no-pending rows (r2 and write P10/G25, read P5/G12).
  - Acceptance:
    - the combined result on the committed code, with a per-row verdict log;
    - deleting the guard fires the consumption assertion at r2 G533 with
      its exact message;
    - deleting both reddens r2 with exactly "no verified selected incoming
      call";
    - the `dec_3tvethnshr68y` carry: compare installed owners only on
      `Ok(owners)`, and propagate `Err(infeasible)` with its own reason;
    - rt_escape*, rt_span_prov_native and the two parity rows stay green.
  - Dispositions left issued and unconsumed are not pruned here.
- **Stops (the 9th is hold plus research):**
  - a consumption-site assertion fires on any fixture;
  - a non-r2 verdict or selected-caller count changes;
  - a residual consumption site cannot see the pending-frame state.

## Acceptance

- **D0 (recut; measure only, on WIP `62060eea6`, read and write
  fixtures).** A table of every carrier slot `(eliminator, constructor,
  position)`: its flow set; each writer and the schema it writes today
  (capture record, labelled residual or static worker); each reader (Direct
  W read, Tail, the non-governed pass-through, the label switch, the gate
  decode). Flag every slot whose writers disagree; S5 `(533, pos 1)` is
  expected to be one. Count the population; do not infer it from shape.
- **AC-S (carrier schema).**
  - A writer emitting the other schema into a slot reddens at the
    assertion, not at a later field read.
  - Dropping a recorded coercion edge reddens.
  - Each of the 8 slots asserts at its readers: a writer storing a bare K,
    skipping the injection, reddens at the reader assertion.
  - Swapping the S3 and S4 variants in the `(533, 1)` record reddens.
  - The interning-key pin stays byte-identical.
  - The write fixture compiles and runs.
- **Slot schemas (`evt_69ktj8b1xe8tc`, `evt_6rtq4txgmrjpw`,
  `evt_7tg07nnz0656z`).** Each slot's schema is its issued R sum, with the
  label elided for a singleton flow set. K is never a slot schema.
  - Edges exist only where a store exists: the construct stores over source
    aggregate occurrences, including checked-IH force children. No transport
    edge is recorded, and no routed answer is wrapped.
  - Readers assert `Child` against its construct edge. A slot left with no
    variant keeps main's plain representation and main's reader path.
- **AC-F (IH-force classification, `evt_7tg07nnz0656z`).**
  - Measure first. M-c: `unit(S1).worker_closure_origin()` and the S1
    variant's `WorkerCapture` roles equal the (S1, 730) record's children,
    729..722. M-d: every `CheckedComputationalIHInvocation` child stored into
    a slot-shaped field across the read and write fixtures, with owner,
    forced seat and route.
  - Read parity `fs_read_at_malformed_offset_narrows_to_invalid_offset` is
    green, and write parity stays green.
  - A planner census of slot (520,1) lists every slot-shaped source
    aggregate occurrence with its Child's minting record and variant. It
    agrees one-to-one with lowering's lookups, and the variant set stays
    {S0, S1, S3}.
  - Removing the IH-force classifier arm refuses at plan time, naming 524 at
    (S1, 526, 1), not the runtime `-1`.
  - Classifying 524 as S3 refuses at the plan-time seat assertion.
  - The Spec2 seven-capture accounting is stated.
- **AC-0 (measure; no build).** Done so far: the suffix census, record
  coverage, the seat relation (a checked parent→child edge, not equality),
  and the disposable trace. Remaining (`evt_2wywq8pmjerv8`): a compile-time
  def-use trace from each projected child (`v7461`, `v14221`, `v25769`) and
  each retarget input (`v9396`, `v16156`, `v27704`) back to the
  instruction that first materializes the word and the store that wrote
  `children[1]`. For each suffix operand, state whether it is in hand
  there, by claim or ordinal provenance and not by value. The Architect
  sizes from the rows. The ruling is recorded as an ADR in `docs/adr/`,
  which lands with the first build increment.
- **AC-1.** The `RT-NATIVE-SEQUENTIAL-BRACKETS` two-bracket witness builds
  and runs natively, and its observation matches the interpreter.
- **AC-2 (controls).**
  - Two constructions of one continuation (W2 and W7) reach their
    consumers with their own captures. A mutation that shares one set of
    fields turns that row red. A missing field or a count mismatch
    refuses.
  - The landed native census shows no verdict change.
  - One-bracket control (Architect `evt_5t9n4ycp2jp40`), in two parts:
    - the measured Wrapped W5/M2 fixture keeps external stdout, stderr and
      exit parity with base `c91f42e7b`;
    - zero-disposition IR identity is checked separately, only on a fixture
      measured at W=0, M empty and |S|=0.

    No demand distinction, and no relaxation of side slots or captures.
  - The IR passes the Cranelift verifier.
- **AC-3 (residual key, `evt_ssrh7r6dx3ty`).**
  - **D0 (measure; no build).** For specializations 2 and 3, report:
    - the expression kind at `args[recursive_position]`;
    - the specialization's `emission_owner`;
    - whether every W and M capture resolves under that owner through the
      existing availability claims. `CurrentLexical` does not count as
      resolved.
  - **Controls.**
    - Swapping the two labels at gate 12 reddens.
    - Restoring the source scan changes the census or makes the gate refuse.
  - **Census pins.**
    - No disposition has a parent constructor that differs from its gate's.
    - Each gate's candidate-set size is recorded, with a count of the gates
      whose set is larger than 1.
    - There are no mixed-disposition refusals.
    - Gate and retarget capture reads from any frame: 0.
  - **Mutation.** Dropping the last C capture from the record refuses at the
    creator or goes red.
  - **Mutation.** Omitting the transport wrap reproduces the measured
    Record-class refusal.
  - **Census.** Per creation construct, its emitting owners; per label, the
    gate owners it is dispatched under.

## Symptom inventory (§1b, Architect)

1. Shape.
2. Range containment.
3. Positional index and `Var` syntax.
4. Single emission owner.
5. Transport target at a force site whose arriving population is a labelled
   {S3, S4} set: there is no static body to key the planner lookup on, and
   the WIP refuses rather than reconstructing it (`evt_550cpvt3p4qw5`, §1a
   5).

Predicate (Architect `evt_v9p2kbzdccb0`, on Research `evt_9cn9mr73248`): the
carrier slot has no planner-owned schema. This chain closes at §1a 5; the
carrier-schema recut's own count starts at 0, and its 3rd advancing stop
triggers hold-and-research.

### Carrier-schema recut (§1a 4, Architect `evt_3v81hg9te2te2`, `evt_7tg07nnz0656z`)

1. R coercion keyed on the transport call site (the routed answer), not on
   the slot store (`evt_3hm46evkpr9g`).
2. Slot-store existence decided from lowering SSA def-use, not from a
   planner fact (`evt_7ybjzpeaa5xff`).
3. The Child's specialization at a slot store comes from the lexical-closure
   classifier, so a transfer-path store whose Child is a checked-IH
   invocation thunk (S1/526, child 524) has no classification
   (`evt_69ge14q5acsta`).
4. Child classification is keyed on the lexical-closure population only. A
   checked-IH force child (524, the K8 minted by the (S1, 730)
   `CheckedIhCapturedEnvironment` record) has no arm. Keyed on minting
   population (`evt_3evd30345myh2`, §1a 4).
5. Response-owner coverage: owner 4 / context 4 requires a verified selected
   incoming call `Spec(S5) → S6` (producer Construct 528). Since the
   residual integration (`9214b25fc`), none is recorded (14 verified calls,
   none to S6). Keyed on the call graph's direct-call record after the
   residual took over that continuation (`evt_a4ph004eayw0`, §1a 5). D0
   `evt_2a1sk8vwfsebr`: disposition B, main emits two selected transport
   calls and the candidate none.
6. Residual issuance cannot see which Active descents emit selected
   response-owner calls: issuance (construction.rs:1435) precedes ownership
   → checked-IH transports → response phase B → selected callers, all of
   which it feeds. Keyed on plan-construction order (`evt_k50enr3zpmp8`,
   §1a 6).
7. The issuance guard skipped a disposition whenever the eliminator's whole
   subtree held a preselected caller. That over-blocks read G12, whose
   residual is needed (forced Active gives `BoundaryCarrier`), and 528 lies
   under G25, not G533. Keyed on whole-subtree containment at issuance
   (`evt_4hg93gfaf76y4`, §1a 7).
8. The preservation guard was keyed on a static body at the residual
   selection point: first the own case body, then the immediately pending
   frame. Rows of the same static shape disagree on displacement:
   - at G533 under S5, the residual drops S5/528 (main 2, residual 0);
   - at G751 and G755 under S3, it keeps S3/746 at 3/3 and S3/750 at 40/40,
     although each caller sits in the pending frame exactly as 528 does.

   Keyed on static containment at the selection point (`evt_30etv9z3ddq4t`,
   §1a 8; Architect `evt_62fpkx7marckq`). Entries 7 and 8 share entry 6's
   predicate. Entry 9 is the next predicate checkpoint.

Predicate (Architect `evt_2bq826gvr9cvz`, held for the Research advisory):
the carrier schema is decided upstream of the checked-IH
environment-transport plane whose facts it needs. Entries 1 to 5 each
reconstructed or missed a transport-plane fact locally, and entry 6 is the
order itself.

## Stop conditions

- A W or C capture with no finalized claim (frame or transport) at its
  materialization point: stop with that capture and owner named, for the
  Architect to rule.
- A suffix operand not in hand where the residual is materialized: the
  def-use rows name the carriage path, and the Architect sizes it. Do not
  add a second carriage mechanism.
- IH-force classification (each is the 5th advancing stop): M-c disagrees,
  meaning the variant's seat is not 730 or its roles differ from the record;
  or M-d finds a stored IH child on the non-functional route, or a store
  whose owner is not a specialization unit.
- Any kernel, `trusted_base()` or spec change: an operator question.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
