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
- **Retained.** Owner-independent labels keyed `(construct, pos, spec id)`
  and the d099 interning-key pin; the `TransportCarriedClaim` census; the
  three transport emission sites; the label-switch reuse with per-arm
  `Some(body)` and the `source.rs:4515` guard; the gate refusals.
- **Replaced.** Every consumer-side classification in I3, and any producer
  writing a form other than its slot's schema.
- **Sequence.** D0 below, then the Architect rules each slot's schema and
  coercion edges, then the build: the planner schema record, conforming
  writers, consumers that read the record and assert, and role-addressed
  reads.

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
  - The interning-key pin stays byte-identical.
  - The write fixture compiles and runs.
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

## Stop conditions

- A W or C capture with no finalized claim (frame or transport) at its
  materialization point: stop with that capture and owner named, for the
  Architect to rule.
- A suffix operand not in hand where the residual is materialized: the
  def-use rows name the carriage path, and the Architect sizes it. Do not
  add a second carriage mechanism.
- Any kernel, `trusted_base()` or spec change: an operator question.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
