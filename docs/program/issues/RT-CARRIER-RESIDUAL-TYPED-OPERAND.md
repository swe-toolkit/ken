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
   - The operand privately carries its issued slot key,
     `RecursiveCarrierSlotKey { eliminator, constructor, position }`, never
     a variant (Architect `evt_1tc9napwj4fjj`). Each producer mints the key
     from the slot it already resolved. A producer that cannot name its slot
     does not mint an R.
   - The one conversion looks up the slot. A singleton slot decodes
     directly. A labelled slot reads the label and switches over the slot's
     flow, and each arm asserts that variant's record and count, then reads
     Child and asserts Child = Kq. The arms join as K, and a label outside
     the flow fails.

     ```rust
     fn decode_residual_child(&mut self, builder: &mut FunctionBuilder<'_>, residual: CarriedResidualWord) -> Result<CarriedBoundaryWord, CraneliftBackendError>
     ```

     Its private per-variant helper, `decode_residual_variant_child`, has
     exactly one call site, inside it.

   - The consumers that need W or C take `CarriedResidualWord` by type: the
     labelled call path (`calls.rs:1147-1163`), the Tail transport
     pass-through (`core.rs:9265-9277`) and the checked-IH captured
     environment (`core.rs:8752-8756`). Each keeps the variant its
     disposition issues, and asserts at compile time that `residual.slot`
     is that specialization's slot key.
2. **The residual operand arm** (Architect `evt_3266hq42hc17h`).
   `LoweringOperand` gains `Residual(CarriedResidualWord)`, and that arm is
   the only channel for R. A binding that holds R is
   `Value(LoweringOperand::Residual(r))`. There is no separate binding arm,
   no routed-answer sum and no side table.
   - `CheckedIhEnvironmentOperand::into_operand` maps `Residual` to
     `LoweringOperand::Residual`, a pass-through that never re-mints, and
     `Synthesized` to `Carried`.
   - Site A (`source.rs:1699-1720`). The `any(matches!(.., Carried(_)))`
     test counts `Residual` as a runtime word. In
     `transfer_constructor_operands` (`core.rs:13575`), the destination's
     declared representation decides (Architect `evt_2gq38tz4qy735`). Where
     `residual_fields[position]` is `None`, the field is an ordinary K
     destination, and an arriving R is decoded before the parent is
     allocated. Where it is `Some`, an arriving R would be re-wrapped as a
     Child, and the preflight refuses it as a planner error.
   - Every store into an aggregate word follows the same rule. Declared R,
     an issued disposition or a checked-IH environment role, takes R only
     with a matching slot key. An ordinary field or capture decodes R
     first. `emit_boundary_closure_environment` (`aggregates.rs:4177`) is
     the open sibling, and decodes residual captures before allocation.
   - Site B (`source.rs:4906-4913`). `RoutedAnswer::checked` carries the R
     typed, and G355 decodes it before the edge.
   - Each refutable pattern on `LoweringOperand` in production lowering
     gets one disposition. The closure basis is the exact candidate's own
     tree, not `d9b8d81a8`: the WIP was rebased, and `d9b8d81a8` is not an
     ancestor of the candidate. The census reruns the same criterion at the
     candidate; a row unchanged from the `d9b8d81a8` table keeps its
     disposition, and every added or changed row gets one (Steward, on
     runtime QA's block `evt_1jb96xz3mdcff`). The dispositions:
     - F: R falls into a branch that already fails closed; cite the error;
     - W: R is a runtime word there, and the site becomes an exhaustive
       `match` with a `Residual` arm;
     - D: R is decoded with `decode_residual_child`;
     - E: an ABI copy, through the counted escape.

     The compiler forces a `Residual` arm at the 58 `match` arms. No boolean
     helper groups R with K.
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
- **The Tail result** is the representation-typed `LoweringOperand`
  (Architect `evt_7qzpmd32e0cvg` withdraws the `TailAnswer` sum; the dead
  enum and `into_operand` at `lowering/mod.rs:4141-4150` are deleted). The
  Child request after the Tail is enforced by `checked_ih_transport_child`'s
  exhaustive match against the planner's `wrapped()`. Tail consumers stay
  exhaustive or use `RoutedAnswer::checked`; a routed answer reaching a
  Child or W/C request gets the compile-time
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

**I-0, the mixed slot: one representation per slot** (Architect
`evt_1ns21bwepgvqv`, on WIP `a12a5db40` and research `evt_4ejgpwpmk763g`).
The ruling carries the code and is authoritative where this summary differs.
- **The rule.** A slot is issued iff some candidate in its flow has a
  `wrapped()` LexicalClosure disposition, as today. Every flow member of an
  issued slot stores R. A member without a wrapped disposition is boxed K→R
  at its writer into `[Child, Label]`.
- **Schema** (aggregates.rs):
  - `RecursiveCarrierMemberSchema::{Residual, Boxed}`;
  - `RecursiveCarrierVariant` gains `schema` and a declared `child_arity`;
  - a planner-issued `RecursiveCarrierBoxedStore`, a closed population
    issued beside the dispositions. Each store's label is the member's
    ordinal in `recursive_residual_candidates`, with `candidates.len() >= 2`
    asserted.
- **Rekeyed, the 4 representation consumers:**
  - ownership and records: one box Record per store, in the parent's meet;
  - `build_recursive_carrier_slots`: a Boxed variant per variant-less
    member, with one ConstructEmission edge per store;
  - the rewalk: `owes_edge := slot.flow.contains(..)`, which supersedes the
    `recursive_residual_for_specialization` key of `evt_34wv6tryq2v64`;
  - `slot_store_obligation`: returns a sealed `Residual | Boxed` with no
    `_ =>` arm at the writer;
  - the decoder's arity check reads `variant.child_arity`, not the
    `WorkerCapture` count.
- The totality invariant (`:10671`) and the decoder loop are unchanged and
  now hold by construction.
- A checked-IH force store of a Boxed member is refused by the planner with
  a named error.
- **Unchanged, the 7 capture-recovery consumers of `wrapped()`** named in the
  ruling: calls.rs:1146, core.rs:15238, ownership :5423/:5436/:5778, :10836
  and :10687.
- **D0 before building (non-advancing).** At G6/Node/0 of
  `invocation_return_transport_selection_is_per_producer_in_production`,
  measure:
  - S1's worker-capture count;
  - S1's context and disposition, and its `wrapped()`;
  - the G31 field-0 expression kind;
  - `boundary_closure_crossing_environment(Specialization(S0), child)` and
    its arity.

  A None environment, or a Child that is not Constructor-class, is a stop
  to the Architect.

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
  - The exits are enforced by module privacy. `CarriedResidualWord`'s word
    and slot are private to its module, and it has no `From`, `Into`,
    `Deref` or public constructor. `decode_residual_variant_child` is a
    private item of that module.
  - Every W row is an exhaustive `match`, so the compiler forces its
    `Residual` arm.
  - Review evidence in the handoff, measured at the exact SHA and not
    committed as tests (operator rule quoted in `build/qa-test-design.md`;
    Steward `evt_63ay1bp9bncyw`):
    - the 95-row disposition table;
    - the module's exit list;
    - the escape's call sites, with their count.
  - Mutation M-A: treat site A's `Residual` as `Carried`, so that it is
    wrapped as a Child. Either the compile-time refusal fires, or, with
    that refusal also removed, id41 does not pass.
  - Mutation M-A2: replace only Site A's pre-allocation decode with the
    raw R word, and `outer_carried` returns to `UnclassifiedRuntimeTrap(-1)`.
  - A `cfg(test)` counter of Site-A R arrivals at ordinary fields during
    full parity is reported with the escape counter, as a population, not
    an oracle.
  - Mutation: removing the R→K decode on the G355 checked-answer edge
    reddens `outer_carried`, with the D0-k raw -1.
  - Label mutations: the default arm falling through to arm 0 stays green
    on id41, which is S3 at arm 0. Swapping arms 0 and 1 fails id41's
    record or count assert.
  - **Mixed slot.**
    - The exact row `invocation_return_transport_selection_is_per_producer_in_production`
      passes 1/1. It is red on held `d9b8d81a8` and green on main.
    - Planner census over the parity corpus: issued slots, mixed slots and
      boxed stores. A Boxed member with a checked-IH force store is a stop
      to the Architect.
    - **Boxed lowering is refused at compile time** (Architect
      `evt_3pfe4zm2mrxs4`). All 299 parity plans have 0 mixed slots. The
      only live mixed-slot source lacks P3→P1 call authority for S1's
      worker body, so Boxed write and decode have no executed consumer.
      - A choke point in the full-compilation driver, before the first
        unit is lowered: any issued Boxed variant gives
        `unsupported("RecursiveResidual", ..)`, counted in
        `boxed_member_compile_refusals`. It does not go in `Lowering`
        construction, and the exact planning row stays green and unchanged.
      - The planner and the three Boxed lowering arms stay as built.
      - Pin `mixed_recursive_carrier_boxed_member_is_refused_before_lowering`:
        `Err`, refusals 1, `boxed_decode_arms_emitted` 0. Report the
        source's observation on the merge-base too.
      - Mutations: deleting the choke point (M-choke) and dropping Boxed
        issuance (M-issue) each redden. The arity and bare-K execution
        mutations are retired: there is no green execution for them to
        change.
      - Lifting the refusal needs a source with graph-derived call
        authority, on which those two execution mutations redden.
      - On merge-base `10daa9242`, the synthetic mixed source returns a
        value outside its range (`Option::None`, where the Leaf body is
        total into Exit). This WP's choke point refuses it, and the
        handoff carries the claim worded in `evt_75s5cr96knfde`. The bad
        value needs the nested Option match inside the recursive Leaf
        body: neither alone produces it (probe `evt_34v8scs9n0j08`). No
        source program or parity row reaches it, so there is no separate
        node (operator 2026-08-29).
  - One full `rt_parity_native` run on the candidate itself (186/186), with
    verdicts unchanged and `boxed_member_compile_refusals` 0. The pre-D0
    186/186 does not transfer.
  - Every I-2 acceptance row of `RT-NATIVE-CONTINUATION-ENV-CARRIAGE`
    holds, as do the 13-row log, mutations 1-3 and the class guard at
    `joins.rs:1109-1124`.
  - The handoff names the I-1 carries: the issuance insert point after
    `construction.rs:1442`, and `Ok(Err(infeasible))` propagation.
- **AC-2 (I-1).**
  - The escape count is 0, and the function is deleted.
  - Review evidence in the handoff: no raw `append_block_param` on a
    carried value outside the helper. Each scalar, control or pointer site
    among the 54 is listed as excluded, with its reason.
  - A K→R refusal control fixture.
  - Full parity is unchanged, and the handoff gives the fan-in table: each
    site, its operand type and the increment that changed it.

## Symptom inventory (§1b, Architect)

1. R is forwarded untyped through `LoweringOperand` containers: constructor
   arguments and `RoutedAnswer.value`. Keyed on the operand container type
   (`evt_3266hq42hc17h`, recut §1a 1).
2. R is stored as is into an ordinary constructor field, because
   `residual_fields[pos] == None` was read as a pass-through instead of as a
   K destination (`evt_2gq38tz4qy735`, recut §1a 2).
3. Wrapped-ness was read from slot flow membership, by the rewalk, the
   totality invariant and the decoder. Keyed on `slot.flow`
   (`evt_4pwphh759tcqf`, recut §1a 3, advancing; hold and research
   `evt_65eqjan51qtgp`). The rewalk obligation is keyed on
   `recursive_residual_for_specialization` (`evt_34wv6tryq2v64`); that much
   is retained.

**Shared predicate (Architect `evt_65eqjan51qtgp`):** the R/K representation
of a position (an operand, a field or a slot member) is read off a nearby
proxy instead of being declared per position. Closed by declaring
representation per slot (`evt_1ns21bwepgvqv`). The next re-trigger is the
6th.

## Stop conditions

- One use site needs both R and K without a transport read. Stop with the
  site named.
- A boundary whose slot kind cannot be decided statically by the planner.
- Any change to the kernel, the spec or a verdict outside the runtime rows
  named here. A currently passing `rt_parity_native` row that is newly
  refused, including through an F disposition, is a stop to the Architect.
- §1a is at 3. The 6th advancing stop is a hold plus research.
