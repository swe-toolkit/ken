---
id: RT-CARRIER-RESIDUAL-TYPED-OPERAND
title: "In native lowering a private residual word travels through the same untyped carried operand as an ordinary value, so each consumer re-derives its plane from a producer-keyed side table or not at all, and a residual reaches an ordinary match raw. Give the residual its own operand type with one decoding conversion, and let the planner issue an ABI slot kind wherever a residual crosses a frame boundary"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [RT-NATIVE-CONTINUATION-ENV-CARRIAGE]
github: null
origin: "Architect §1b predicate ruling evt_1j8rqab3xjryd on RT-NATIVE-CONTINUATION-ENV-CARRIAGE I-2 (§1a 10; D0-k evt_4x2a3b4g5fbzh): entries 1 to 4 of the recut and I-2 entries 9 and 10 share one predicate. Restructuring, sized by the Steward as its own node (operator 2026-09-21). Steward-filed per COORDINATION section 2."
---

# Residual words have their own operand type

## Objective

A private residual word R and an ordinary carried value K have different
operand types in lowering. An ordinary consumer cannot receive R without
calling the one decoding conversion. No binder, capture, frame slot, or
join or block parameter has an unknown representation: each is declared R
or K, and incoming edges coerce into it before a join.
`generated_entry_capsule_outer_carried` gives its typed `ResourceBodyResult`
(identity 41).

## Settled inputs (Architect `evt_1j8rqab3xjryd`, on D0-k at `d9b8d81a8`)

- **The predicate.** A word's carrier plane is not part of the lowering
  operand that carries it. R and K both travel as `CarriedBoundaryWord`,
  `LoweringOperand::Carried` and `LoweringEnvironmentBinding::Value`
  (`lowering/mod.rs`). Whether a word is R lives only in producer-keyed side
  tables: the slot schema at (eliminator, constructor, position), the call
  site, or the minting population. ENV-CARRIAGE recut entries 1 to 4 and I-2
  entries 9 and 10 are each a local re-derivation of that fact.
- **The measured route (entry 10).** R1075 is minted by CheckedIhForce at
  `(G533, Vis, position1)`. It passes through the G533 field binding
  (`core.rs:15114-15257`, which installs the original word), the G355
  checked-answer fallback (`core.rs:15390-15497`, which forwards
  `scrutinee.word` unchanged), lexical capture (`core.rs:16513-16572`),
  continuation assembly (`core.rs:10548-10603`) and two frame copies
  (`calls.rs:2223`). It then reaches Match514's constructor-class guard
  (`joins.rs:1109-1124`) as a Record.
- **The fan-in, by mechanism** (D0-k):
  - 19 `emit_carrier_field` sites;
  - 3 direct field-to-binder emitters (`core.rs:15114-15257`,
    `source.rs:3674-3696`, `joins.rs:1180-1199`), plus the checked-answer
    fallback;
  - 6 checked-IH force consumers with differing handling (`core.rs:8752`,
    `:8905`, `:9265`, `:9531`, `source.rs:4601`, `calls.rs:1147`);
  - 7 downstream carrier classes with no slot query: Let binding, lexical
    capture, continuation fields and inputs, declared-call frame copy, and
    generated-context raw load (`units.rs:5763-5805`).
- **Retained, already proved.** These stay as they are on the held I-2 WIP
  `d9b8d81a8`:
  - the recut slot-schema table, with its two issued store kinds
    (`RecursiveCarrierStoreKind::{ConstructEmission, CheckedIhForce}`,
    `aggregates.rs:264-267`), and the transport sources and claims as they
    stand. A transport call's routed answer is not a slot store
    (`:255-256`; Architect `evt_6hzfhbfzwrmkg`);
  - `emit_carrier_label_ordinal` at the three label readers;
  - the class guard at `joins.rs:1109-1124`, unchanged;
  - the I-1 landing `e893ecb7a`.
- **Excluded.** Uniform closures (`dec_7aajmm0eac45c`). No repair at G355,
  at G533, or at any of the 66 ordinary matches.
- **Prior art** (Research `evt_5ssk3dnwx1aks`, adopted by the Architect
  `evt_whsgcgvea866`). The typed operand with type-directed coercion at
  representation boundaries is the known-best shape (GHC levity, Leroy
  1992, TIL). Its invariant is that no binder or join parameter has an
  unknown representation. Rust newtypes vanish at a Cranelift block
  parameter, so the helper that creates one must carry the representation.

Treat anchors as perishable. If a settled input is false on the base, stop
and report the mismatch.

## Deliverable

The work resumes on the held I-2 WIP `d9b8d81a8`, on branch
`wp/RT-NATIVE-CONTINUATION-ENV-CARRIAGE-I2`. Its landing carries I-2 with
it. Five items, delivered in the two increments below:

1. **The residual operand.** Add `CarriedResidualWord`, distinct from
   `CarriedBoundaryWord`. It is produced only by the issued slot stores of
   the two `RecursiveCarrierStoreKind` kinds, and by projection of a
   recursive-position field whose issued slot schema is an R. A transport
   routed answer is not a producer. A consumer that needs R from one is a
   K→R site under item 4: it is refused at compile time and reported to the
   Architect.
   - Every ordinary API accepts `CarriedBoundaryWord` only:
     - environment `Value` bindings;
     - constructor field stores;
     - match scrutinees;
     - the checked-answer fallback;
     - lexical capture;
     - continuation inputs;
     - declared-call frame copies.
   - The one conversion asserts the record, reads Child and asserts
     Child = Kq:

     ```rust
     fn decode_residual_child(builder: &mut FunctionBuilder<'_>, residual: CarriedResidualWord, expected: AggregateOccurrenceId) -> Result<CarriedBoundaryWord, CraneliftBackendError>
     ```

   - The consumers that need W or C take `CarriedResidualWord` by type: the
     labelled call path (`calls.rs:1147-1163`), the Tail transport
     pass-through (`core.rs:9265-9277`) and the checked-IH captured
     environment (`core.rs:8752-8756`).
2. **Residual bindings.** A recursive-position binder used both as a value
   and as a transport source gets
   `LoweringEnvironmentBinding::Residual(CarriedResidualWord)`. A value read
   decodes it, and a transport read takes R.
3. **The ABI slot kind.** The planner issues a residual slot kind for any
   capture or frame slot that must carry R across a generated-context or
   declared-call boundary. Any other boundary carries the decoded Child.
   `calls.rs:2223` and `units.rs:5763-5805` become typed by this fact.
4. **Join and boundary representation.** The planner fact in item 3 extends
   to every join and boundary where predecessors can disagree. The only
   coercion is R→K, `decode_residual_child`.
   - A join declared K decodes its R predecessors before the edge. That is
     legal only when no consumer after the join reads W or C, and the
     planner checks this.
   - A join that would need K→R is refused at compile time, with a typed
     lowering error naming the join, until a K→R coercion is defined.
   - The G355 checked-answer fallback (`core.rs:15390-15497`) is such a
     boundary. It declares a representation or fails to compile.
5. **The block-parameter helper.** The one helper that creates join and
   block parameters takes `Representation::{Residual, Value}` and returns
   the matching newtype. No lowering code builds a carried block parameter
   any other way.

## Increments (Architect `evt_5mvs2wxjebfr7`, on the AC-0 tables)

The tables are `evt_5k5tjg65b7xhq` and `evt_2pxpa880p4ws9`. I-0 comes first,
then I-1, each a separate exact-SHA review with the Architect as single gate.

- **The two-origin checked-IH environment** is a sum,
  `CheckedIhEnvironmentOperand::{Residual(CarriedResidualWord),
  Synthesized(CarriedBoundaryWord)}`. `Residual` is the existing R at
  `source.rs:1699-1704`, and `Synthesized` the fresh K built by
  `checked_ih_captured_environment_from_case_environment`
  (`source.rs:4906-4913`). A W/C consumer that gets `Synthesized` is a K→R
  refusal, never a coercion.
- **The Tail result** is `TailAnswer::{PassThrough(CarriedResidualWord),
  Routed(CarriedBoundaryWord)}`. The Child request after the Tail
  (`core.rs:9531-9580`) is legal only on `PassThrough`, by type. A `Routed`
  arm that reaches a Child or W/C request gets the compile-time
  `CraneliftBackendError::ResidualRepresentationRequired { site }`.

**I-0: typed operand, producers and consumers, and the proven route.** No
ABI change.
- Items 1, 2 and 5, with the helper
  `append_carried_block_param(builder, block, representation) ->
  CarriedBlockParam` at every join I-0 touches. `decode_residual_child`
  combines two existing checks: `decode_recursive_residual`'s record assert,
  and `checked_ih_transport_child`'s assert that Child is the expected
  constructor (read at `mod.rs:8897`).
- All 19 `emit_carrier_field` sites are typed as in the AC-0 table. At
  `core.rs:15114` the decoded Child is no longer discarded.
- G355 `return_body` (`:15030`, `:15142`, `:15497`), its header carrier
  parameter (`:14867`, `:14928`) and the result merge (`:15000`) are declared
  K. R→K is decoded on the checked-answer edge before the join.
- The downstream ABI classes keep their untyped ABI until I-1:
  - lexical capture;
  - continuation fields and inputs;
  - the context gather;
  - the frame store;
  - the generated-context load.

  An R reaches them only through
  `residual_across_untyped_abi_transitional`. Each call names its site.
  Nothing else may use it.

**I-1: planner-issued representation, and the escape removed.**
- `AbiSlot` (`abi.rs:448-462`) gets a planner-issued `representation`. It
  is R only where a transport or W/C consumer is proven downstream, and K
  otherwise.
- The five ABI classes read and write through it. A K slot decodes R
  before the store.
- `JoinResultRepresentation` gains R and K. The labelled invocation merge
  (`calls.rs:912-942`) and the planned source join (`joins.rs:353-445`)
  declare one, with R decoded or K→R refused.

## Acceptance

- **AC-0 (done).** The fan-in tables above, and the cut confirmed.
- **AC-1 (I-0).**
  - `generated_entry_capsule_outer_carried` passes 1/1 with the typed
    identity-41 trap.
  - Exit census: a source-scan test shows that `CarriedResidualWord`'s
    module has exactly two exits, `decode_residual_child` and the
    transitional escape, and no `From`, `Into`, `Deref` or public field.
    This replaces a compile-fail control, since the types are crate-private.
  - The escape's call sites are pinned by count and list.
  - Mutation: removing the R→K decode on the G355 checked-answer edge
    reddens `outer_carried`, with the D0-k raw -1.
  - One full `rt_parity_native` run, with verdicts unchanged.
  - Every I-2 acceptance row of `RT-NATIVE-CONTINUATION-ENV-CARRIAGE`
    holds, as do the 13-row log, mutations 1-3 and the class guard at
    `joins.rs:1109-1124`.
  - The handoff names the I-1 carries: the issuance insert point after
    `construction.rs:1442`, and `Ok(Err(infeasible))` propagation.
- **AC-2 (I-1).**
  - The escape count is 0, and the function is deleted.
  - A grep pin shows no raw `append_block_param` on a carried value
    outside the helper. Each scalar, control or pointer site among the 54
    is listed as excluded, with its reason.
  - A K→R refusal control fixture.
  - Full parity is unchanged, and the handoff gives the fan-in table: each
    site, its operand type and the increment that changed it.

## Stop conditions

- One use site needs both R and K without a transport read. Stop with the
  site named.
- A boundary whose slot kind cannot be decided statically by the planner.
- Any change to the kernel, the spec or a verdict outside the runtime rows
  named here. A currently passing `rt_parity_native` row that is newly
  refused is a stop to the Architect.
- The 3rd advancing stop is a hold plus research (§1a starts at 0).
