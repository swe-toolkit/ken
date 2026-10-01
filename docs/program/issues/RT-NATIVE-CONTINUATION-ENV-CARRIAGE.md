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

## Acceptance

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
