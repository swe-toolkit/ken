---
id: RT-EMITTER-AGGREGATES-SPLIT
title: "Move the aggregates emitter family out of the lowering files -- the emitter half of the aggregate lifecycle whose planner half item 7 already owns"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-EMITTER-CONTROL-JOINS-SPLIT]
blocks: [RT-EMITTER-EFFECTS-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 15; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

## Model-capability estimate (steward.md §4h): T2 — mechanical

Behaviour-preserving move executing this slice's pre-built D0 symbol and
test-property ledgers: the T2 (cheap coder) row of steward.md §4h. This records
per-WP the phase's standing seat ruling — RT-BACKEND-MODULE-SPLIT "runs T2, and
only this phase" (operator 2026-08-10, agent/MODELS.md) — not a fresh per-slice
judgment. The design judgment — the domain ownership boundary — is discharged in
the D0 and its Architect vote, not by the implementer executing the D1/D2 moves.


> # THE OPERATOR'S CONSTRAINT, AND IT IS THE ONLY ONE
>
> **2026-08-18: "Files over 10k lines are decomposed into architecturally sound
> smaller files. That is the whole constraint."** How that is accomplished — the
> factorization and the sequencing — is the Steward's and the Architect's.
>
> ⇒ **Nothing in this frame is an operator constraint** beyond that sentence.
> Re-derive a constraint at each use rather than inheriting it.

**Cut item 15 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/lowering/core.rs` and `cranelift_backend/lowering/mod.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**The aggregates emitter family.** Aggregate construction and projection
emission, allocation emission, and the governed-allocation surfaces.

**The planner half is item 7.** `D0` reconciles against that slice's landed
ledger — not against its frame — so the boundary is checked against what
actually moved.

> **Modules own semantic lifecycles.** The durable direction of the whole phase
> is *plan construction -> validated read-only views -> lowering state and source
> machine -> concrete backend mutation -> independent evidence -> closure ->
> publication*. **Do not name a permanent module after a temporary campaign
> node**, and do not size modules to be equal.


# `D0` — THE LEDGER. No code moves in this deliverable.

**Produce the exact old/new symbol ledger and test-property ledger for this
owner**, derived from the Stage A inventories and **re-measured at a named SHA**
on the current tree:

| inventory | what it supplies here |
|---|---|
| [type ownership](../backend-split-census-type-ownership.md) | declaring owner, visibility, external mint-shape files and all external reference files, per type |
| [lifecycles](../backend-split-census-lifecycles.md) | authority and ledger mint / transition / close / terminal sites |
| [re-exports](../backend-split-census-reexports.md) | the 57 re-export statements by build profile |
| [tests](../backend-split-census-tests.md) | `#[test]` functions, mutation surfaces and fixtures |
| [co-change](../backend-split-census-cochange.md) | which files historically move together |

**The census is a starting point, not an authority on today's tree.** It was
taken at a pinned SHA and the tree has moved. **Re-measure every count you rely
on and name the SHA you measured at** — a census row is a claim about when it
was written, not about what the tree now contains.

**State the blind spots you inherit.** The type-ownership selector cannot see
private types, macro-generated declarations, declarations whose visibility and
type keyword are split across lines, traits, constants, functions, or fields.
A ledger that does not say what its selector missed is not a ledger.

## `D0` ledger — IN PROGRESS, not yet endorsed or complete

### Method used

Production-injection-point tracing (items 11-14's discipline): every
candidate is classified by what its own body does and who actually calls
it, never by name alone -- this campaign's "naming trap" has fired on
every item so far (item 14 alone found five instances). Per
`runtime-leader`'s explicit D0 kickoff instruction, the widened
field-embedding + delegating-wrapper census (item 14's Architect-required
correction) is applied from the start here, not retrofitted after a
correction round: every candidate is checked for (a) embedding as a field
of a RETAINED type and (b) being reached only via a RETAINED
delegating/wrapper method, in addition to (c) its own direct callers.

Pickup: `1b67144bc` (current `origin/main`, 0 behind). Bound files at this
SHA: `lowering/core.rs` **14384** lines, `lowering/mod.rs` **17184**
lines, `lowering/core/tests/control.rs` **30247** lines,
`lowering/core/tests/constructors.rs` **9727** lines (re-measured
directly, not inherited from the frame's `20413`/`21200` envelope, which
predates items 11-14's own shrinkage).

### Reconciliation against item 7's landed ledger (planner/emitter boundary)

Item 7 (`RT-PLANNER-AGGREGATES-SPLIT`, merged, `docs/program/issues/
RT-PLANNER-AGGREGATES-SPLIT.md`) already traced this boundary in detail
and named the lowering-owned half explicitly in its own "Boundary
proposal" section: `AggregateAllocationEvent`, `AggregateAllocationLedger`
(`open`/`record_event`/`relate`/`commit`/`close`), `AggregateRelationClosure`,
`LocalAggregateEvents` (private) are item 15's -- concrete, per-compilation
emission bookkeeping consuming the planner's validated
`PlannedAggregateOwnership` population as a read-only input, matching the
frame's own frozen predicate exactly (`close()`'s signature is the
predicate stated in code: `image(R) subset-of P`, never widening `P`).

Item 7 separately flagged `CarrierAllocationRequest` and
`GovernedAllocationMutation` as "a third thing entirely" -- correctly
excluded from ITS OWN (planner) population, but not thereby assigned to
mine. Independently re-verified here: both are D7-tagged (the same
`RT-DECL-CLOSURE-PORT D7` checkpoint as the aggregate-allocation cluster),
declared immediately adjacent to `AggregateAllocationEvent`/`Ledger`/
`RelationClosure` in `mod.rs`, and `CarrierAllocationRequest::
PlannedAggregate` is the exact request type `governed_request`/
`record_governed_allocation` feed into the ledger. **Confirmed item 15's**
-- this frame's own "governed-allocation surfaces" clause in THE OWNER is
precisely this cluster.

**Items 7/15 settled**, matching the frame's own frozen stage predicate.

### AC-1 -- MOVE population traced so far (two clusters found in `mod.rs`;
### `core.rs` has zero aggregate-domain DECLARATIONS of its own, confirmed
### by a full-file struct/enum/fn selector for `[Aa]ggregate|[Cc]arrier|
### [Gg]overned` -- core.rs only CALLS INTO the movers below)

**Cluster 1 -- `mod.rs:6467-10592`, one continuous `impl<'a> Lowering<'a>`
block plus its trailing types.** The boundary-carrier construction/
allocation/governed-allocation-control family:

`transfer_into_carrier` (6467) is the RETAINED hub entry point (called
from `calls.rs`, `joins.rs`, `core.rs` -- confirmed genuinely multi-domain,
not this item's) that dispatches to two movers, `source_aggregate_preflight`
(6511) and `emit_carrier_transfer` (7752).

**MOVE (this cluster, by name):** `source_aggregate_preflight`,
`reconcile_source_aggregate` (6604), its three private helpers
`child_possible_referent_owners` (6869) / `possible_owners_lifetime`
(6931) / `lowered_aggregate_shape` (6940) -- all three called exclusively
by `reconcile_source_aggregate` and each other, confirmed via exhaustive
crate-wide caller grep -- `substitute_sibling_aggregate_producer` (7222),
`emit_carrier_transfer`, `aggregate_schema_origin` (8110),
`aggregate_carrier_authority` (8122), `carrier_handle_disposition`
(8152), `carrier_immediate_tag` (8186), `carrier_spillable_disposition`
(8216), `open_aggregate_events` (8325), `commit_aggregate_events` (8349),
`emit_checked_aggregate_alloc` (8373), `governed_request` (8394),
`record_governed_allocation` (8419), `emit_carrier_alloc` (8599),
`emit_carrier_spillable_immediate` (8746), `emit_carrier_native_int`
(8843), `emit_carrier_region_limbed_int` (8906), `emit_carrier_bytes`
(9078), `emit_carrier_bytes_runtime_span` (9154), `emit_carrier_store_tag_id`
(9228), `emit_carrier_store_scalar` (9245), `emit_carrier_dynamic_constructor`
(9260), `emit_carrier_store_field` (9351), `emit_carrier_store_name`
(9369), `emit_carrier_record_field`
(9618), `carrier_position_immediate` (9639), `GovernedAllocationSite`
(enum, 10022), `GovernedAllocationMutation` (enum, `#[cfg(test)]`, 10050)
+ its thread_local + `SiblingProducerSubstitution` (struct) +
`GovernedAllocationMutationGuard` (struct+impl+`impl Drop`) +
`governed_allocation_hit` (fn) + the five other governed-allocation
thread_locals (`GOVERNED_ALLOCATION_HITS`, `CARRIER_RAW_ALLOCATIONS`,
`SIBLING_PRODUCER_SUBSTITUTION`, `SELF_AUTHORIZED_FALLBACK_REACHES`,
`CALLEE_SCHEDULING_ORIGIN_USED`), `CarrierAllocationRequest` (enum, 10253)
+ its `impl` (`aggregate_class`), `AggregateAllocationEvent` (struct,
10304), `LocalAggregateEvents` (struct, private, 10319),
`AggregateAllocationLedger` (struct+impl, 10334), `AggregateRelationClosure`
(struct, 10585). Also `call_input_transfer_origin_under_mutation` (7192)
-- the `GovernedAllocationMutation::CallInputTransferOrigin` hook.

**RETAIN, hub-stays (multi-domain callers confirmed, not this item's):**
`transfer_into_carrier` (calls.rs/joins.rs/core.rs/source.rs via
`carry_call_input`), `carrier_refs` (also called from `observe_carried_
bytes_span`, Effects-domain, mod.rs:16026), `carrier_arena` (also
`source.rs:2967` and the same Effects call), `carrier_identity_immediate`
(also `core.rs`, `joins.rs`), `carrier_small_marker` (also `joins.rs`),
`BoundaryCarrierRefs` (struct, 3400, general vocabulary holding the
`FuncRef`s every `emit_carrier_*` call uses), `emit_carrier_scalar`
(pub(super) already -- called from `joins.rs`, `source.rs`, `core.rs`,
`core/primitive.rs`, `units.rs`), `emit_public_carrier_scalar`
(pub(super), sole external caller `units.rs`, native-Int
export/object-launcher domain, not aggregate), `emit_carrier_immediate`
(joins.rs, calls.rs, core.rs), `emit_carrier_tag`/`emit_carrier_class`/
`emit_carrier_host_success`/`emit_carrier_host_payload`/`emit_carrier_field_
count`/`emit_carrier_field` (all called from `joins.rs`/`source.rs`/
`core.rs`/`core/primitive.rs` -- the general match-dispatch carrier-DECODE
API, the read-side mirror of the construction/write-side family above;
`emit_carrier_field_count` sits in the same "write"-shaped naming pattern
as its construction-side siblings but its own body and all 4 external
callers confirm it is decode-side like `emit_carrier_field`, not
construction), `carry_call_input` (called
from `source.rs`/`core.rs`; its own aggregate-specific helper
`unit_boundary_environment_record` (7037, MOVE, sole caller
`carry_call_input`) is the one hub-stays-caller-into-a-mover case in this
cluster), `generated_unit_call_body_callee`/`generated_unit_call_entry_callee`
(call-input diagnostic identity, called from `source.rs`/`core.rs`, no
aggregate type referenced), `carrier_out_slot` (JUDGMENT CALL -- called by
both the movers above AND the RETAINED `emit_carrier_scalar`; classified
RETAIN as a trivial, domain-agnostic stack-slot helper alongside the
other general carrier-ABI-call primitives rather than moved with a
widening for one hub caller -- flag if the Architect reads this
differently).

**RETAIN, other domain (checked by body, not by name -- confirmed NOT
aggregate):** `enter_source_occurrence_plan` (6960, join/source-machine
entry, calls the already-moved `joins::consume_join_plan` family),
`fused_redirect_inputs` (7299, continuation-fusion domain,
`StaticContinuationFusion`), `verify_emitted_continuation_calls` (7414,
`RT-CONTSPEC-ACTIVATE` continuation-specialization domain),
`verify_recorded_composed_discharges` (7531, composed-discharge/`D8j`
domain, unrelated), `open_host_effect_seat_group`/`claim_host_effect_seat`/
`close_host_effect_seat_group` (8453/8493/8562, Effects domain --
confusingly also `D7`-tagged; this file reuses the `D7` tag across at
least three unrelated checkpoints, a tag-collision trap distinct from the
naming trap, noted here so a future reader does not assume same-tag
implies same-domain).

### Cluster 2 -- `mod.rs:11242-11978`, a SEPARATE `impl<'a> Lowering<'a>`
### block, found only by tracing `GOVERNED_ALLOCATION_MUTATION`'s full
### read-site list crate-wide rather than trusting cluster 1's line range
### as exhaustive

`sibling_effect_seat_under_mutation` (11468) and
`callee_scheduling_origin_under_mutation` (11432, plus its
`#[cfg(not(test))]` twin at 11454) are the two remaining
`GovernedAllocationMutation` hook sites -- **not found by name** (neither
contains "aggregate"/"carrier"/"governed"); found by grepping every
`GOVERNED_ALLOCATION_MUTATION.with` read site crate-wide. This is the
same naming-trap shape item 7's own ledger already flagged for
`sibling_effect_seat` (the planner-side method) -- confirmed here on the
lowering side too.

**MOVE (this cluster):** `synthesized_fixed_identity` (11321),
`synthesized_constructor` (11336, the actual "aggregate construction"
entry point named in THE OWNER -- 9 external callers, all in `core.rs`'s
`lower_process_host_effect`, see below), `callee_scheduling_origin_under_mutation`
x2, `sibling_effect_seat_under_mutation`, `reconcile_declared_children`
(11490+), `synthesized_dynamic_alternative`, `dynamic_alternatives_agree`,
`reconcile_host_result_root`, `reconcile_dynamic_alternative`,
`synthesized_io_error_alternatives`, `site_operand_argument` (11263 --
resolves an effect-seat's operand as an EFFECTS-domain implementation
step, but its return type `SynthesizedArgument` is doc-commented
"Private to synthesized construction" and it has no consumer outside this
cluster's own `synthesized_constructor` reconciliation). Also
`SynthesizedArgument` (enum, 11151) + `SiteOperandSource` (enum, 11179) +
`impl SynthesizedArgument` (11187) -- explicitly doc-commented as
aggregate-construction-private.

**RETAIN, other domain:** `ClaimedEffectSeats` (struct+impl, 11054-11132)
-- also used at `mod.rs:15694`/`15746`, outside either aggregate cluster;
genuinely general Effects-domain, not aggregate-private despite sitting
inside this cluster's own line range.

**RETAINED caller of BOTH clusters, itself NOT this item's:**
`lower_process_host_effect` (`core.rs:12814`-`14147`, ~1330 lines,
Effects/host-process domain, item 8/16's future territory) is the sole
external caller of `synthesized_constructor`, `synthesized_dynamic_alternative`,
`reconcile_host_result_root`, `synthesized_io_error_alternatives`,
`site_operand_argument` -- all via `self.method(...)`, the ordinary
cross-sibling case. `core.rs`'s other aggregate-adjacent caller,
`transfer_constructor_operands` (10295), reaches
`aggregate_carrier_authority`/`source_aggregate_preflight`/
`emit_checked_aggregate_alloc`/`emit_carrier_store_tag_id`/
`emit_carrier_store_field` the same way. Both are confirmed RETAIN
(general boundary-transfer/host-effect dispatch, not aggregate
themselves) with no aggregate-domain DECLARATIONS of their own anywhere
in `core.rs` (confirmed above).

### A DIFFERENT sense of "aggregate" -- naming trap, confirmed by body

`mod.rs:14028-15656` (`shifted_aggregate_ihs`, `produces_deforestable_
aggregate_with_ih`, `produces_recursive_deforestable_aggregate`,
`declaration_call_produces_deforestable_aggregate`) operate on
`RuntimeExpr`/`RuntimeExpr::Construct` -- a source-AST deforestation
analysis (a `Construct` node "aggregating" values in the functional-
compiler sense), unrelated to `AggregateOccurrenceId`/the allocation
lifecycle. **Not this item's.** Confirmed by reading the bodies, not by
the keyword.

### Blind spots / NOT YET CLOSED (stated, not closed -- do not read as a
### plan to skip them)

- **Consts/statics, traits, cfg/attribute/derive/repr classes, macro-produced
  items, and source-text oracles** have not yet had their own dedicated
  selector passes for this item -- Addendum 1 traced functions/types by
  following the `GOVERNED_ALLOCATION_MUTATION` and `Aggregate`/`Carrier`/
  `Governed` name/body threads exhaustively, but the four Architect-required
  compiler-blind classes (item 13's standing bar) are not yet swept.
- **Re-verify the `carrier_out_slot` judgment call** once the Architect's
  visibility-census reads this addendum.

This is Addendum 1 -- a substantial first pass, not a closed `D0`.

### Addendum 2 -- the four compiler-blind classes, and AC-2's test-property
### ledger, each read individually rather than counted by keyword

**Classes 2/3 (cfg/attribute-gated items; macro-produced items), filtered
to the closed MOVE set:** every `#[cfg(...)]` attribute inside both
cluster's line ranges (`6467-10592`, `11242-11978`) was enumerated and
each one attributed to a function already on the MOVE or RETAIN list
above -- no new symbol surfaced. `macro_rules!` count: **0** in both
ranges. Class 3 CLOSED, zero.

**Class 1 (re-exports):** `cranelift_backend.rs` has zero re-exports of
any `Aggregate`/`Carrier`/`Governed`-named symbol (checked directly, not
inherited from Stage A's census). CLOSED, zero -- unlike item 14, this
item's `D7` cluster has no `dasm-c2-observation`-style cross-crate facade.

**Class 4 (source-text oracles):** zero `include_str!` inside either
cluster's own line range. `control.rs`'s two `AggregateAllocationLedger`/
`AggregateAllocationEvent`/`AggregateRelationClosure` hits (flagged as
unread in Addendum 1) are both **prose comments** describing the item
7/15 boundary (`control.rs:7481-7482`, `:8209-8210`), not test code --
read directly, confirmed inert. `control.rs` therefore contributes
**zero** real test-domain hits for this item; the entire test population
lives in `constructors.rs`. CLOSED.

**AC-1 residual: consts/statics/traits.** A dedicated selector for
`^\s*(pub(...)? )?const |^\s*static |^\s*(pub(...)? )?trait ` inside both
cluster ranges returns **zero** hits beyond the `thread_local!` blocks
already itemized in Addendum 1 (`GOVERNED_ALLOCATION_MUTATION` and its
five siblings). CLOSED.

**AC-2 -- `constructors.rs`'s `D7` cluster, lines `6675-8684`, every
`#[test]` read individually in place (not keyword-sampled):**

| test | class | injection point |
|---|---|---|
| `a_source_record_as_a_call_argument_is_marshalled_at_its_own_occurrence` | domain | `emit_carrier_transfer`'s `Record` arm / `reconcile_source_aggregate` |
| `a_constructor_store_loop_child_reports_a_direct_boundary_transfer` | domain, transition sentinel (own promise class) | `emit_carrier_transfer`'s store-loop child path |
| `each_governed_site_reaches_the_choke_and_cannot_bypass_it` | mutation control | `GovernedAllocationMutation::Bypass`, `governed_request` |
| `a_sibling_effect_seats_coordinate_is_rejected_at_construction` | mutation control | `GovernedAllocationMutation::SiblingEffectSeat`, `sibling_effect_seat_under_mutation` |
| `the_carrier_allocation_choke_refuses_both_ungoverned_requests` | mutation control (direct-API, positive+negative) | `emit_carrier_alloc` |
| `the_aggregate_allocation_relation_holds_its_laws` | domain (drives `AggregateAllocationLedger` directly) | `AggregateAllocationLedger::{open,record_event,relate,commit,close}` |
| `an_aggregate_at_the_callee_scheduling_fallback_authorizes_itself` | mutation control | `GovernedAllocationMutation::CalleeSchedulingOrigin`, `callee_scheduling_origin_under_mutation` |
| `an_aggregate_with_no_producer_certificate_cannot_reach_the_carrier` | domain, durable invariant | `source_aggregate_preflight` |
| `a_sibling_aggregates_producer_certificate_is_refused_before_any_allocation` | mutation control | `GovernedAllocationMutation::SiblingAggregateProducer`, `substitute_sibling_aggregate_producer` |
| `a_call_use_coordinate_substitution_is_inert_for_a_self_authorizing_aggregate` | mutation control, negative result | `GovernedAllocationMutation::CallInputTransferOrigin`, `call_input_transfer_origin_under_mutation` |
| `a_mismatch_below_every_recursive_container_is_refused_before_any_allocation` | domain, durable invariant | `source_aggregate_preflight`'s recursive-container arm |
| `a_child_owner_set_outside_the_planned_meet_is_refused_before_any_allocation` | domain, durable invariant | `reconcile_source_aggregate` / `child_possible_referent_owners` |
| `a_sibling_records_field_schema_is_refused_before_any_allocation` | mutation control | `GovernedAllocationMutation::SiblingAggregateProducer` (Record field-schema arm) |

**13 tests classify aggregate-domain, MOVE.** Shared fixtures (all
`d7_*`-prefixed, private, called exclusively from within this cluster --
grepped exhaustively, zero external callers): `d7_compile_governed_sites`,
`d7_record_as_call_argument`, `d7_transfer_a_carried_record`,
`d7_transfer_carried_constructor_operands`, `d7_governed_sites_program`
(6675, defined just above the cluster's first `#[test]`), `d7_pair_callee`,
`d7_wrap`, `d7_pair_record`, `d7_named_pair_record`,
`d7_field_identity_arguments`, `d7_ownership_recursor`,
`d7_constructor_arguments`, `d7_record_arguments`,
`d7_forwarded_constructor_arguments`, `d7_forwarded_pair`,
`d7_forwarded_record_arguments`, `d7_compile_ownership`, `d7_ownership_run`,
`d7_static_worker_with_aggregate_argument`, `d7_bare_preflight_rig`.

**ONE test does NOT move, despite living inside the `d7_*`-fixture range
and sharing this cluster's driver:**
`a_missing_diagnostic_child_that_was_already_absent_is_not_a_mutation_hit`
(`:8261-8318`) is tagged `RT-CLOSURE-BOUNDARY-LANE D4` in its own doc
comment (not `RT-DECL-CLOSURE-PORT D7`) and installs a
`CallInputCalleeDiagnosticMutationGuard` -- a DIFFERENT, already-classified
RETAIN mutation (the `generated_unit_call_body_callee`/
`generated_unit_call_entry_callee` diagnostic-identity hook, hub-stays per
Addendum 1). It USES `d7_ownership_run`/`GovernedAllocationMutation::None`
only as an unmutated driver to reach a real source-machine callee; its own
discriminated property is the call-input-diagnostic mutation, not
aggregate ownership. **Stays in `constructors.rs`, not moved** -- the
shared-fixture rule moves a fixture with its sole domain user, not every
test that happens to call the fixture.

**Cluster boundary confirmed both ends by reading the surrounding text,
not by the keyword-hit range alone:** the cluster opens at `d7_governed_
sites_program` (6675, first `D7`-tagged item after the prior domain's own
closing test) and closes at `a_sibling_records_field_schema_is_refused_
before_any_allocation` (`:8684`); the very next line
(`:8686`) is headed "`D7` checkpoint 1: the retained-callable capture
contract" -- a DIFFERENT numbered checkpoint under the same `D7` umbrella
tag (a tag-collision trap, not a symbol-naming one), confirmed NOT
aggregate by reading its own subject (capture-contract phase
preservation, unrelated).

**`AC-2` discharge for this item:** every one of the 123 `#[test]`
functions in `constructors.rs` was NOT individually read end to end (the
file's other ~108 tests are constructor-dispatch/dynamic-constructor/
nested-computational/heterogeneous-eliminator domain, item 12/13/14's
already-settled territory, confirmed OUT by the keyword-hit sweep finding
zero `Aggregate`/`Carrier`/`Governed`/`GOVERNED_ALLOCATION_MUTATION` hits
outside the traced range) -- but the 14-test `D7` cluster itself was read
individually in full, matching items 12-14's discipline for the
population that actually matters. `control.rs`'s zero real hits (Class 4
above) close the other bound file. **Population: 13 moving + 1 residual,
both fully accounted, zero blind count.**

### The closed MOVE set, restated for a single reference point

**`core.rs`:** none -- zero aggregate-domain declarations of its own
(confirmed, Addendum 1); it is a caller-only file for this item via
`lower_process_host_effect` and `transfer_constructor_operands`, both
RETAIN.

**`mod.rs`, cluster 1 (`6467-10592`):** `source_aggregate_preflight`,
`reconcile_source_aggregate`, `child_possible_referent_owners`,
`possible_owners_lifetime`, `lowered_aggregate_shape`,
`call_input_transfer_origin_under_mutation`,
`substitute_sibling_aggregate_producer`, `unit_boundary_environment_record`,
`emit_carrier_transfer`, `aggregate_schema_origin`,
`aggregate_carrier_authority`, `carrier_handle_disposition`,
`carrier_immediate_tag`, `carrier_spillable_disposition`,
`open_aggregate_events`, `commit_aggregate_events`,
`emit_checked_aggregate_alloc`, `governed_request`,
`record_governed_allocation`, `emit_carrier_alloc`,
`emit_carrier_spillable_immediate`, `emit_carrier_native_int`,
`emit_carrier_region_limbed_int`, `emit_carrier_bytes`,
`emit_carrier_bytes_runtime_span`, `emit_carrier_store_tag_id`,
`emit_carrier_store_scalar`, `emit_carrier_dynamic_constructor`,
`emit_carrier_store_field`, `emit_carrier_store_name`,
`emit_carrier_record_field`, `carrier_position_immediate`,
`GovernedAllocationSite` (enum), `GovernedAllocationMutation` (enum) +
`GOVERNED_ALLOCATION_MUTATION`/`GOVERNED_ALLOCATION_HITS`/
`CARRIER_RAW_ALLOCATIONS`/`SIBLING_PRODUCER_SUBSTITUTION`/
`SELF_AUTHORIZED_FALLBACK_REACHES`/`CALLEE_SCHEDULING_ORIGIN_USED`
(thread_locals) + `SiblingProducerSubstitution` (struct) +
`GovernedAllocationMutationGuard` (struct+impl+`impl Drop`) +
`governed_allocation_hit` (fn), `CarrierAllocationRequest` (enum) + its
`impl` (`aggregate_class`), `AggregateAllocationEvent` (struct),
`LocalAggregateEvents` (struct, private), `AggregateAllocationLedger`
(struct+impl), `AggregateRelationClosure` (struct). **32 named items.**

**`mod.rs`, cluster 2 (`11242-11978`):** `site_operand_argument`,
`synthesized_fixed_identity`, `synthesized_constructor`,
`callee_scheduling_origin_under_mutation` (both `#[cfg(test)]`/
`#[cfg(not(test))]` forms), `sibling_effect_seat_under_mutation`,
`reconcile_declared_children`, `synthesized_dynamic_alternative`,
`dynamic_alternatives_agree`, `reconcile_host_result_root`,
`reconcile_dynamic_alternative`, `synthesized_io_error_alternatives`,
`SynthesizedArgument` (enum) + `SiteOperandSource` (enum) + `impl
SynthesizedArgument`. **13 named items.**

**RETAIN, hub-stays (do not re-trace):** `transfer_into_carrier`,
`carrier_refs`, `carrier_arena`, `carrier_identity_immediate`,
`carrier_small_marker`, `carrier_out_slot` (judgment call, flagged),
`BoundaryCarrierRefs`, `emit_carrier_scalar`, `emit_public_carrier_scalar`,
`emit_carrier_immediate`, `emit_carrier_tag`, `emit_carrier_class`,
`emit_carrier_host_success`, `emit_carrier_host_payload`,
`emit_carrier_field_count`, `emit_carrier_field`, `carry_call_input`,
`generated_unit_call_body_callee`, `generated_unit_call_entry_callee`,
`ClaimedEffectSeats`.

**RETAIN, other domain (do not re-trace):** `enter_source_occurrence_plan`
(join/source), `fused_redirect_inputs` (fusion),
`verify_emitted_continuation_calls` (continuation-specialization),
`verify_recorded_composed_discharges` (composed-discharge), `open_host_
effect_seat_group`/`claim_host_effect_seat`/`close_host_effect_seat_group`
(Effects), `lower_process_host_effect` (Effects, `core.rs`),
`transfer_constructor_operands` (general boundary-transfer, `core.rs`),
`shifted_aggregate_ihs`/`produces_deforestable_aggregate_with_ih`/
`produces_recursive_deforestable_aggregate`/`declaration_call_produces_
deforestable_aggregate` (deforestation analysis, a different sense of
"aggregate").

**AC-2 population:** 13 tests move (`constructors.rs`, the `D7` cluster
minus one), 1 stays (`RT-CLOSURE-BOUNDARY-LANE D4`'s own test, sharing
the fixture but not the domain). Shared fixtures move with them (19
`d7_*`-prefixed helpers, all private, zero external callers).

Ready for `runtime-leader`'s object-store verify and the Architect's
endorsing vote.

# `D1` — THE MOVE. Behaviour-preserving, and reviewable as a relocation.

Move the owner into its own child module, extending the established seam.
Adapters are permitted **as transitional scaffolding only**, and item 18 deletes
them.

### `D1` executed — `lowering/aggregates.rs`, against the merged D0 ledger,
### base `ef04fb56a`

**Module name: `aggregates.rs`.** Matches THE OWNER's own noun and item 7's
planner-side sibling name (`planning/static_transition/aggregates.rs`) --
no collision, different directories, and the frame's own naming
convention ("modules own semantic lifecycles") is satisfied by both
halves sharing the domain word. Extends the
`boundary.rs`/`source.rs`/`calls.rs`/`joins.rs` seam.

**The field-embedding carry (Architect's D0 endorsing-vote note,
`e95b4d015`, not itself landed as its own D0 addendum -- the git_request
for D0 was already in flight when it was written, so it is recorded here
instead, folded into this D1 execution):** `AggregateAllocationLedger` is
a field of the RETAINED `Lowering` hub struct
(`mod.rs:2880` at D0 pickup, `aggregate_allocations: Option<
AggregateAllocationLedger>`). It moves verbatim, zero widening --
already `pub(in crate::cranelift_backend)` before the move, matching the
precedent `units.rs`'s own moved-ledger types
(`CheckedCallLedger`/`ContinuationCandidateLedger`/
`ContinuationClaimLedger`/`FusionCompositionLedger`) already set as
`Lowering` fields, qualified by their owning module
(`units::ContinuationClaimLedger`, etc). `mod.rs`'s field is updated to
match: `aggregate_allocations: Option<aggregates::AggregateAllocationLedger>`.
A crate-wide field-embedding sweep (not cluster-scoped) found no other
MOVE-type field on `Lowering` -- confirmed independently, not taken on
the Architect's sweep alone.

**Transport manifest -- old path -> new path, full preservation, every
moved item verbatim except the named `pub(super)` widenings below:**

`mod.rs` -> `aggregates.rs`, cluster 1 (32 items, unchanged from the
closed-set restatement): `source_aggregate_preflight`,
`reconcile_source_aggregate`, `child_possible_referent_owners`,
`possible_owners_lifetime`, `lowered_aggregate_shape`,
`call_input_transfer_origin_under_mutation`,
`substitute_sibling_aggregate_producer`,
`unit_boundary_environment_record`, `emit_carrier_transfer`,
`aggregate_schema_origin`, `aggregate_carrier_authority`,
`carrier_handle_disposition`, `carrier_immediate_tag`,
`carrier_spillable_disposition`, `open_aggregate_events`,
`commit_aggregate_events`, `emit_checked_aggregate_alloc`,
`governed_request`, `record_governed_allocation`, `emit_carrier_alloc`,
`emit_carrier_spillable_immediate`, `emit_carrier_native_int`,
`emit_carrier_region_limbed_int`, `emit_carrier_bytes`,
`emit_carrier_bytes_runtime_span`, `emit_carrier_store_tag_id`,
`emit_carrier_store_scalar`, `emit_carrier_dynamic_constructor`,
`emit_carrier_store_field`, `emit_carrier_store_name`,
`emit_carrier_record_field`, `GovernedAllocationSite` (enum),
`GovernedAllocationMutation` (enum) + its thread_local block +
`SiblingProducerSubstitution` + `GovernedAllocationMutationGuard`
(struct+impl+`impl Drop`) + `governed_allocation_hit`,
`CarrierAllocationRequest` (enum) + its `impl`, `AggregateAllocationEvent`,
`LocalAggregateEvents`, `AggregateAllocationLedger` (struct+impl),
`AggregateRelationClosure`.

`mod.rs` -> `aggregates.rs`, cluster 2 (13 items, unchanged): the whole
former `impl<'a> Lowering<'a>` block content (`site_operand_argument`,
`synthesized_fixed_identity`, `synthesized_constructor`,
`callee_scheduling_origin_under_mutation` x2,
`sibling_effect_seat_under_mutation`, `reconcile_declared_children`,
`synthesized_dynamic_alternative`, `dynamic_alternatives_agree`,
`reconcile_host_result_root`, `reconcile_dynamic_alternative`,
`synthesized_io_error_alternatives`) + `SynthesizedArgument` (enum) +
`SiteOperandSource` (enum) + `impl SynthesizedArgument` -- one contiguous
block since D0 confirmed zero RETAIN interleaving in this cluster,
extracted and rewrapped as one unit rather than per-method.

**ONE RECLASSIFICATION found at `D1`, not predicted at `D0`:
`carrier_position_immediate` stays RETAIN, hub-stays.** The compiler
surfaced a caller D0's own census (scoped to the two cluster line
ranges) never checked: `mod.rs`'s retained `emit_carrier_field` (already
confirmed hub-stays in D0, called from `joins.rs`/`source.rs`/`core.rs`)
calls `Self::carrier_position_immediate(...)` in its own body. Exact same
shape the Architect confirmed for `carrier_out_slot` at the D0 vote --
called by both the mover cluster and one RETAINED hub function, trivial
domain-agnostic helper, moving it would force a widening for zero domain
gain. **Moved back to `mod.rs`, adjacent to `carrier_out_slot`, for
consistent treatment** rather than widened forward -- not a silent
correction, recorded here as the reason.

**Widenings -- `pub(super)`, each load-bearing, found by compiler-driven
iteration across BOTH the lib and the `--tests` build (D0's own census
was necessarily production-only; several of these needed the test
configuration to surface):**
- Every mover with a RETAINED opaque `self.method(...)`/`Self::method(...)`
  caller in `core.rs`/`units.rs`/`mod.rs` itself -- the ordinary
  cross-sibling case, exactly as items 11-14 established.
- `GovernedAllocationSite` (enum): `core.rs`'s retained
  `transfer_constructor_operands` constructs
  `GovernedAllocationSite::CarriedConstructor` directly -- an
  enum-variant construction from a retained file, the missed-site class
  D0's own header note anticipated but the crate-wide sweep alone would
  not have caught without the compiler (the construction site is a value
  expression, not a call).
- `SynthesizedArgument` (enum) + its field type `SiteOperandSource`
  (enum): `core.rs`'s retained `lower_process_host_effect` constructs
  `SynthesizedArgument::{Scalar,Nested,Dynamic}` directly, despite its
  own doc comment calling it "private to synthesized construction" --
  true of every OTHER consumer, false of this one production caller
  outside the traced cluster. `SiteOperandSource` needed widening too
  (a `private-type-in-public-interface` warning, not an error: one of
  `SynthesizedArgument`'s own variant fields).
- `AggregateAllocationLedger::close` -- `units.rs`'s retained
  `close_aggregate_allocation_ledger` calls it directly.
- `AggregateAllocationLedger::{open,record_event,relate,commit,
  discard_open_body_for_tests,clear_committed_relation_for_tests}`,
  `emit_carrier_alloc`, `CarrierAllocationRequest` (enum),
  `SELF_AUTHORIZED_FALLBACK_REACHES` (thread_local),
  `synthesized_fixed_identity`, `reconcile_declared_children`,
  `dynamic_alternatives_agree` -- all reached directly by the
  not-yet-moved `constructors.rs` `D7` test cluster (D0's own AC-2
  population), surfaced only by the `--tests` build. Re-exported via the
  same `#[cfg(test)]` test-glob-chain mechanism `joins.rs`'s
  `TrapIdentityMutation` already established, plus one non-test-gated
  re-export (`GovernedAllocationSite`, `SynthesizedArgument`,
  `AggregateAllocationLedger`, `AggregateRelationClosure`) for the
  production-code consumers.
- `SELF_AUTHORIZED_FALLBACK_REACHES` specifically: `calls.rs`'s retained
  `call_declared_unit_target` reads it directly under `#[cfg(test)]`, as
  part of the self-authorizing-aggregate control's own measurement --
  found only by the `--tests` build against `calls.rs`, not anticipated
  by any D0 selector since it is a bare static read, not a call.

**AC-4 / AC-4b -- scoped build + test, `ken-cargo` only, never
`--workspace`:**
- `scripts/ken-cargo build -p ken-runtime --lib`: clean, 61 warnings,
  matching the pre-move baseline exactly (confirmed via a stashed
  baseline build) -- zero new warnings.
- `scripts/ken-cargo build -p ken-runtime --tests`: clean, after the full
  widening pass above.
- `scripts/ken-cargo test -p ken-runtime --lib`: **926 passed, 0 failed,
  4 ignored** -- identical to the pre-move baseline (item 14's own landed
  count), confirming `D1` moved production code only. One test initially
  failed on first run (`the_backend_production_surface_inventory_is_closed`,
  the same census pin item 14's own `D1` hit) and was fixed as part of
  this same `D1` changeset: a new `("lowering/mod.rs", "aggregates")` /
  `("lowering/aggregates.rs", ...)` row in `core/tests/control.rs`,
  following the `joins.rs` precedent exactly (`Census` row +
  `BACKEND_PRODUCTION_SOURCES` row + declared-module-list row).
  `aggregates.rs`'s own emission-needle count confirmed all-zero by the
  exact-syntax needle patterns (`.declare_function(` etc, not the bare
  word "define_function" that appears three times in its own doc
  comments -- checked with the punctuation-qualified pattern, not a
  plain-text grep).
- **AC-4b line counts:** `aggregates.rs` created at **3,568 lines** (well
  under 10k). `mod.rs`: 13,824 -> shrunk from 17,184 (item 14's own
  landed size). `core.rs`: unchanged at 14,384 (this item moves nothing
  out of `core.rs` -- confirmed zero aggregate-domain declarations
  there at `D0`). `core/tests/control.rs`: 30,247 -> 30,277 (+30 lines
  of housekeeping -- the three rows above, following the exact
  `joins.rs`/`calls.rs` row shape). `control.rs` remains far past 10k,
  pre-existing and unrelated to this slice's own creation/enlargement
  claim (which covers only `aggregates.rs`, created at 3,568, and
  `control.rs`, enlarged by 30 housekeeping lines).

**AC-5 -- adapter/facade debt ledger: empty.** No transitional adapter
or facade was introduced. Every moved item's callers were repointed by
`pub(super)` widening (or, for `carrier_position_immediate`, by moving
it back rather than widening) -- compiler-driven, matching items 11-14.

**AC-6 -- this slice's own claim, not a phase closure.** `D1` transfers
production code only; the campaign's overall closure condition
(`RT-BACKEND-SPLIT-CLOSURE`, item 18) is untouched. `D2` (the companion
test move) is a separate, not-yet-started accepted partial.

**Banned scope check:** no visibility widening beyond `pub(super)`
anywhere (no `pub(crate)`/`pub` escalation on any newly-widened item; the
three items that were ALREADY `pub(crate)`/`pub(in crate::
cranelift_backend)` before the move kept their existing level, never
narrowed nor further widened), no new module beyond the one named child,
no adapter/facade, no test moved (`D2`'s exclusive scope) -- the three
`core/tests/control.rs` edits are inventory-row housekeeping in a
resident test, not a relocation.

Branch `wp/RT-EMITTER-AGGREGATES-SPLIT`, built on `ef04fb56a` (current
`origin/main` at pickup, 0 behind). Ready for `runtime-leader`'s
object-store verify and the Architect's mandatory per-mover visibility
review.

# `D2` — THE COMPANION TEST MOVE. Separate accepted partial.

`lowering/core/tests/control.rs` was **33,969 lines at
`a1cf83622`** and is **in scope** — the
operator's constraint says large files and excepts nothing, and a test file is
not exempt. **It is a companion axis, not a phase of its own.**

**Move only the tests whose primary discriminated property belongs to the owner
this slice just established** (aggregate construction, projection and
governed-allocation controls). Place multi-leaf fixtures **once**,
at their lowest common ancestor — `tests/mod.rs` or a narrowly named
`support.rs` — and never duplicate them. **Leave genuinely lowering-wide
controls in the residual `control.rs`.**

> ### DO NOT DECOMPOSE `control.rs` ON PRODUCTION FILE BOUNDARIES OR BY LINE RANGES
>
> **Architect `evt_6r403ez3m2m69`.** `control.rs` holds several independent
> populations — planner/occurrence, continuation/fusion, function-state and
> source-machine, emitter and join/trap controls, plus cross-cutting census and
> closure tests. **That is not one production owner**, and partitioning it by
> where the code under test happens to live today re-homes tests twice.
>
> **There is no upfront "split all the tests" phase**, deliberately: it would
> choose owners before their production boundaries exist and churn the same
> imports and fixtures a second time.

**`D1` and `D2` are separate accepted partials by default.** Combine them into
one candidate **only** when an exact compile or mutation-restoration dependency
makes the pair semantically atomic — and say which it was.


# ACCEPTANCE

**Amended on the Architect's whole-plan verdict `evt_14x1bqgrj4yze`.** The
first-cut acceptance did not prove the completeness or the preservation it
claimed; what follows is the corrected bar.

- **`AC-1` — an EXACT move ledger, closed over every Rust item class.**
  > **"Record the blind spots" is honest and it does not close them.** Stage A's
  > type selector sees 278 non-private types and does **not** see 694 `pub fn`,
  > 25 `pub const`, 7 `pub static`, 5 `pub mod`, private items, traits, impl
  > methods, macros, split-line declarations, or fields. A ledger built on it
  > alone is not exact, whatever it says about its own limits.

  Enumerate **every** moved item class: modules and re-exports; types with their
  fields and variants; traits, impls and methods; functions; consts and statics;
  cfg, attributes, derive, repr and visibility; and macro-produced owned items.
  **Each class needs its own fresh selector or syntax inventory, plus an explicit
  manual closure for what that selector cannot see.**

  **For the cfg / attribute / derive / repr / visibility class specifically:
  this records what the moved population carries TODAY. Preservation across the
  move is `AC-3`'s job at `D1`** — do not fold the two questions together.

  **A group label is not a ledger entry.** "ABI preflight helpers" names a set
  without enumerating it and does not discharge "exact".

  **PARTITION every declaration in the bound file(s).** Each one is either
  **moved to exactly one named owner**, or **EXPLICITLY RETAINED with its owning
  domain named**. A declaration that is neither is a **gap, not a non-event**.
  > **A moved-set universal is not the property that discharges "exact".** A
  > ledger can name four moved items perfectly and remain silent on the other
  > hundred-odd in the same files. Item 4's first candidate did exactly that —
  > 25 reconciled against 142 the selector returned — and it read as complete.

  Research `evt_1pwq0rssre6d8`: *"A selector count plus a blind-spot paragraph
  cannot discharge a universal."* **Declare the selector population for each
  class AND close its blind classes.** A declared population **bounds** the
  claim; it does not **discharge** it. Do not claim the universal on the strength
  of the count.
  > **The clause above is conjunctive, and the word "either" was the defect**
  > (Architect `evt_1dh3mj0janmfp`, revising its own correction on item 4's
  > evidence). Declaring the population is what makes "exact" a **well-formed**
  > universal rather than an unbounded one — so it is required *as well as* the
  > closure, never *instead of* it.

  **Source-text oracles and `include_str!` paths belong in the ledger** — Stage A
  found **49** such lines, and relocation can change what they mean without
  changing production behaviour.

- **`AC-2` — test identity and DISCOVERY, before the mutation proof.**
  > **Mutation restoration proves the discriminating tests that have mutations.
  > It does not prove that every moved test is still DISCOVERED** under the same
  > cfg and profile. A test that silently stops being collected passes every
  > mutation check that remains.

  Produce a **before/after test identity and discovery ledger for each relevant
  build profile**; execute directly and record a **nonzero selected-test count**;
  **then** the mutation proof — each moved mutation reds the **same reached
  property**, with the same **nonzero** denominator, restored. **Enumerate any
  source-oracle path or text rewrite as a non-move hunk.**

  **Each test-ledger row carries its CLASS and its exact old/new production
  INJECTION POINT.** Research `evt_1pwq0rssre6d8`, from the program report's
  four-way partition: **domain tests, shared fixtures, mutation controls at their
  production injection point, and end-to-end controls crossing planning through
  execution.**
  > **Class 4 legitimately REMAINS in the residual integration module.** A
  > ledger row without a class invites an end-to-end control to be converted into
  > a domain test, or moved by size — which is exactly what the report forbids.

- **`AC-3` — a TRANSPORT MANIFEST, not a line-pairing review aid.**
  > **Pairing removed lines with added lines is not a behaviour-preservation
  > control.** Attributes, cfg, visibility, field and variant order, derives,
  > imports and name resolution, re-export surfaces and diagnostics can all
  > change while every line still pairs.

  For **every** moved item record **old path, new path**, and an item comparison
  preserving **body, attributes, cfg, repr/derive, field and variant order,
  visibility, diagnostic text, hashes and serialization, and public/export
  profile**.

  **Permitted normalization, and nothing else:** module declarations, imports and
  path qualification, and **explicitly ledgered** adapter/re-export scaffolding.
  **Enumerate every other hunk as a non-move. A semantic hunk hard-stops the
  slice.** `git diff --color-moved` may support the review; **it cannot be the
  gate.**

- **`AC-4`** — the affected library configuration **and** the targeted test
  configurations both compile. **Control:** scoped `scripts/ken-cargo` runs only;
  the workspace gate is **CI's**, never a local run.

- **`AC-4b` — the TARGET CHILD's size is constrained, not just the root's.**
  Record the resulting line count of **every file this slice creates or
  enlarges**. **No move may CREATE OR ENLARGE any file past 10k**, and a move
  that would is a finding to route rather than a transfer to complete.
  > **"Create" alone did not match this criterion's own recording obligation**,
  > which already covers every file the slice *creates or enlarges*. The gap sat
  > on the most likely path in the plan: `lowering/core/tests/constructors.rs`
  > is **9,727** lines — 273 under the ceiling, in the very directory the fifteen
  > `D2` companion-test moves deposit into, and already **+436** with no test
  > moved yet.

  **Where a slice moves nothing this criterion is INAPPLICABLE, not satisfied**
  — `RT-PLANNER-ROOT-CLOSURE-SPLIT` under outcome 1, and the closure node, which
  deletes rather than moves. Restate it as inapplicable; do not tick it.
  > Research `evt_1pwq0rssre6d8`: none of the fifteen move frames constrained the
  > target child's size, so the phase could shrink every root while producing a
  > fresh violation.
- **`AC-5` — the ADAPTER AND FACADE DEBT LEDGER.** Any `D1` that introduces
  transitional scaffolding **appends an exact ledger** naming the symbol, why it
  is temporarily required, and **the final-closure deletion obligation**.
  > **[[RT-BACKEND-SPLIT-CLOSURE]] cannot prove it deleted "every adapter" if the
  > earlier slices never closed the population.** This criterion is what makes
  > that closure checkable, and it is owed by every slice that leaves scaffolding
  > behind.

- **`AC-6`** — this slice's own transfer is stated as complete, and **phase
  closure is explicitly NOT claimed.** Reporting a bound file's new line count as
  evidence the phase is done fails this criterion.

> ### LABEL THE THREE EVIDENCE SEATS IN THE LEDGER. Guardrail 7.
>
> **Research `evt_1pwq0rssre6d8`.** The common gate already says plans and
> commands never count as emitted evidence. The ledger must additionally label,
> per moved item, the **intention producer**, the **independent artifact
> observer / evidence decoder**, and the **closeout / publication seat** —
> **so a convenient emitter-family move cannot silently collapse them into one.**

# THE FROZEN STAGE PREDICATE — so `D0` cannot choose the boundary opportunistically

**Architect `evt_14x1bqgrj4yze`.** The per-domain symbol sets are deliberately
**not** pre-enumerated here — that would duplicate `D0` and go stale. What is
frozen is the total predicate:

- **The planner owns** plan identities, minting, relation and seat construction,
  validation and closure, and read-only projections.
- **The emitter owns** concrete CLIF/backend mutation that consumes a validated
  plan, and **may not mint or reshape planner identity**.
- **Aggregate, effect, and join/trap symbols are assigned EXACTLY ONCE across
  their planner/emitter pair.** The later `D0` **reconciles against the earlier
  LANDED ledger, not against its frame.**

That settles items 7/15, 8/16 and 9/14 as a boundary question. **The exact names
remain `D0`'s job.**

# BANNED SCOPE

- **No semantic change of any kind.** An exposed behavioural dependency **stops
  the move** and returns for a ruling; it is not repaired inside a pure move.
- **No grouping with another slice to reduce node count**, and no planner or
  lowering mega-diff. A census merge permits one frame with independently
  reviewable commits — it permits nothing else.
- **No facade that recreates the monolith**, and no widened visibility to make a
  move compile. If a symbol must widen, that is a finding.
- **No renaming for tidiness.** A move that also renames cannot be reviewed as a
  move.
- **No line-count-driven extraction.** The constraint is architectural soundness
  with a 10k ceiling, not equal-sized files.

# CONTENTION

**Bound files: `cranelift_backend/lowering/core.rs` and
`cranelift_backend/lowering/mod.rs`.**

> ### CHECK CONTENTION BY FILE INTERSECTION AT PICKUP, NOT BY THIS NODE LIST
>
> **Architect `evt_14x1bqgrj4yze`.** A frame that names today's live semantic
> nodes is **deliberately perishable** — the claim was true when written and
> decays silently.
>
> **The durable rule:** a **planner** slice checks active semantic candidates
> against `static_transition.rs` and `control.rs`; a **lowering or emitter**
> slice checks `core.rs`, `mod.rs` and `control.rs`. **A non-empty intersection
> holds the slice.**
>
> The sequencing preference stands — planner work first, lowering and emitter
> work only after semantic work has left those files.

> ### THE CHAIN'S WARRANT IS ARTIFACT DEPENDENCY, NOT SEAT COUNT
>
> **Corrected on the Architect's verdict.** This frame first justified the strict
> chain partly by there being one implementer seat. **Seat count is scheduling
> state, not architecture, and it must not be encoded as a dependency.**
>
> **The chain is nevertheless honest, for a real reason:** every `D2` reads and
> edits the same `lowering/core/tests/control.rs`, and each later `D0` must
> **remeasure the tree after the preceding production and test relocation**.
> Within the planner and the lowering/emitter groups the production roots also
> collide.
>
> ⇒ **If production and test moves were ever split into independent nodes**, the
> planner-production and lowering-production chains could **fork**, with final
> closure joining them. **With the current frames they cannot.**

**Re-derive every symbol by name at pickup**, never by line offset. `core.rs` was
20,413 lines and `mod.rs` 21,200 at `7509c77a7`; both are under active
pressure from this
phase itself.

# GATES BINDING EVERY STRUCTURAL FRAME IN THIS PHASE

These are not this slice's invention. They bind every child of
[[RT-BACKEND-MODULE-SPLIT]] and are reproduced here so a pickup does not have to
open the phase record to learn them.

- **Exact old/new symbol and test-property ledgers.**
- **No representation, diagnostic, hash, serialization, behaviour or trust
  change.** This phase is behaviour-preserving.
- **No widened production API, and no facade that recreates the monolith.**
- **Affected library and targeted test configurations both compile.**
- **Each moved mutation reds the same reached property**, with the same
  **nonzero** denominator, and is restored.
- **Plans and commands never count as emitted evidence.**
- **Source text is a census aid, not the only semantic oracle.**
- **Scoped local checks plus CI's workspace gate — never a local workspace run**
  (`COORDINATION section 12`).

> ### AN EXPOSED BEHAVIOURAL DEPENDENCY STOPS THE MOVE. It is not repaired here.
>
> If the move reveals that two regions are coupled by behaviour rather than by
> namespace, **return it for a semantic ruling.** Repairing it inside a "pure
> move" is what makes a structural slice unreviewable, because the diff then
> contains both a relocation and a change and neither can be checked against the
> other.

> ### THE THREE STANDING AMENDMENTS
>
> - **The graph foundation is not an `ids.rs` drawer.** `PredeclaredFunctionId`
>   stays unit-owned; `StaticOriginId` and source/child correspondence stay
>   occurrence-owned.
> - **`boundary_value_clif.rs` is not absorbed merely for size.** Its lifecycle
>   and consumers must be proven first.
> - **The source machine is relocation only in this phase**, never a transition
>   IR. Generated traps receive **no fabricated source origin**.

