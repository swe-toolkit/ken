---
id: RT-EMITTER-CALLS-RETURNS-SPLIT
title: "Move the calls and returns emitter family out of the lowering files -- the first emitter slice, moving against the stable unit and call vocabulary item 4 established"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-SOURCE-MACHINE-TYPES-SPLIT]
blocks: [RT-EMITTER-CONTROL-JOINS-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 13; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
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

**Cut item 13 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/lowering/core.rs` and `cranelift_backend/lowering/mod.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**The calls and returns emitter family.** Declared-call emission, residual and
recursor call lowering, return emission, and the callee-side checks.

**This family moves against the unit/call vocabulary item 4 made stable.** If
that vocabulary is not in fact stable at pickup, that is a finding about item 4,
not a licence to re-open it here.

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

Measured at `edb69247e` (origin/main at pickup, 0 behind). No code moved.
**This section is NOT a closed ledger.** It records grounded findings as they
are traced, exactly as posted to convo (`evt_773wce3gwj0rk`, `evt_f4g7f5tr8mh8`)
so the work survives a compaction or a hand-back — it does not yet discharge
AC-1 or AC-2, and no D1 may execute against it until both are closed and it
carries an Architect vote, per this campaign's own durability lesson (item 12's
D0 ledger was ruled non-durable living only in chat).

### Method used

Production-injection-point tracing (item 11/12's discipline), reconciled
against item 12's LANDED ledger for domains it already settled (continuation/
fusion, checked-invocation, computational-match/eliminator-frame descent,
generic occurrence helpers, aggregate/carrier emission, static-worker/
constructor-field disposition) rather than re-deriving them. A name match to
"call"/"callee"/"return" is never sufficient by itself — several genuine
matches traced to a DIFFERENT, already-landed domain (see below).

**New discipline this node adds, not needed by item 12:** a mutation-control
static's accessor function can sit anywhere, but its TRUE production
injection point (the `.with(|cell| cell.set(...))` call) can land either (a)
inside a separately-defined, movable method, or (b) inside `lower_expr`'s own
~1,373-line body (core.rs 12814-14186) — the banned-from-splitting monolith
dispatcher. Case (a) moves with its owning method; case (b) is trapped and
stays with `lower_expr` regardless of what domain its NAME suggests, because
extracting it would split `lower_expr`, which the banned-scope section
forbids outright. `SEED_CALLEE_UNIT_PORTS` is the first example found (below).

### AC-1 — MOVE population traced so far (core.rs)

- `call_declared_recursive_position_unit` (11957, already `pub(super)` from
  item 12), `call_declared_context` (12074 — its sole caller is the former;
  despite `ContinuationContextId`/`ContinuationSpecialization` vocabulary it
  is the "retarget to a specialized context" arm of the SAME declared-call
  dispatch, not the continuation/fusion cluster), `validate_retained_callable_
  capture_contract` (10205 — a callee's capture-contract check before
  invocation), `call_static_worker` (9683), `call_static_worker_with_inputs`
  (9730, already `pub(super)`, called from `source.rs`'s D8e consumer per
  item 12), `call_declaration_closure_unit` (15721, already `pub(super)`),
  `validate_declaration_unit_call` (15838).
- `RECURSIVE_POSITION_UNIT_CALLS` (thread_local, core.rs:32-35 — shares a
  block with `C2_UNIT_EMISSION_EPOCH`/`SUPPRESS_REQUIRED_CONSUMER_ROUTE`/
  `REQUIRED_CONSUMER_ROUTE_SUPPRESSIONS`, none of which move — this block
  needs the SAME kind of split item 12's LRC/CCR block needed) +
  `recursive_position_unit_calls` (1585) accessor. Its mutation site (12063)
  is inside `call_declared_recursive_position_unit`'s own body, confirming
  the move; its reset call is inlined directly in `compile_expr_into_module`
  (core.rs ~2110, a RETAINED top-level function) rather than through a named
  reset accessor — a cross-module reference to update at `D1`, not a blocker.

### AC-1 — MOVE population traced so far (mod.rs)

- `call_declared_unit` (7834, already `pub(super)`), `call_declared_
  declaration_unit` (8005), `call_declared_unit_target` (8135 — emits the
  actual Cranelift call `Inst` plus the callee-failure/trap early-return
  protocol; confirms "callee-side checks" and "return emission" are the SAME
  early-return machinery at a declared call's own site, not separate), `decode_
  direct_callee` (8441), `emit_result` (18452, already `pub(super)` — converts
  a `Lowered` value to its return representation; called from `units.rs`,
  outside the bound files, so external reachability is a path-update concern
  only), `emit_process_exit_status` (18505), `unwrap_terminal_ret` (16776).
- The declared-call closeout family: the shared thread_local block holding
  `D5_CLOSEOUT_MUTATION`/`D5_EMITTED_DECLARATION_CALLS` (both exclusive to
  `call_declared_unit_target` and its neighbor — moves wholesale, no split
  needed), `D5CloseoutMutation` enum, `with_d5_closeout_mutation`, `reset_d5_
  emitted_declaration_calls`, `d5_emitted_declaration_calls`.
- `TrapCallerProtocolMutation` enum + its thread_local + `set_trap_caller_
  protocol_mutation` (exclusive to `call_declared_unit_target`; its sibling
  `TrapIdentityMutation` is a DIFFERENT, RETAINED control used only by
  `emit_current_trap`, checked and excluded).
- `StaticWorkerCallOutcome` + `StaticWorkerEmission` (exclusive to
  `call_static_worker`/`call_static_worker_with_inputs`).

### CONFIRMED RETAIN, other domain (checked by production-injection-point,
### not by name) — recorded so the same name is not re-traced

- `claim_and_call_continuation`, `claim_and_call_resolved_continuation`
  (+`_inner`), `assemble_continuation_call_operands`, `compose_continuation_
  locally`, `realize_required_consumer_locally`, `dispatch_fused_consuming_
  call`, `settle_continuation_candidate`, `dispatch_fusion_owned_outer_
  realization`, `resolve_direct_emission_claim`, `resolve_context_capture_
  claim`, `verify_entry_frame`, `eliminate_detached_producer_continuation`,
  `d3_raw_settled`, `d3_raw_pending_composed`, `continuation_candidate_is_
  consumed` (`ContinuationCallIdentity`-typed), `verify_emitted_continuation_
  calls`, `call_declared_context`'s neighbor `note_consuming_call` — this last
  is `StaticWorkerFieldLedger`'s own method (checked the enclosing `impl`
  block, not `Lowering`'s), constructor-field/static-worker-disposition
  domain, item 12's other RETAIN citation — all continuation/fusion or
  checked-invocation, item 12's landed domains.
- `lower_recursor_residual_call` — despite matching "residual and recursor
  call lowering" by name, its own role is eliminator-frame-descent
  orchestration that calls OUT to `call_declared_recursive_position_unit`
  (this item's), never the reverse; stays with computational-match/
  eliminator-frame descent, item 12's landed RETAIN domain.
- `recursive_position_unit_body` / `resolve_recursive_unit_body` (+ their
  own helpers `recursive_position_construct_argument`, `agreeing_recursive_
  body_unit`) — their names echo `call_declared_recursive_position_unit`
  closely; traced to their actual caller inside the checked-invocation/
  computational-recursor construction (`make_computational_recursor`'s
  caller), not this item's declared-call dispatch.
- `lower_declaration_ref` — its own text states "evaluating the naked
  `DeclarationRef` never calls the unit"; produces a compiler-only callable
  BINDING (closure construction), a different mechanism from calling one.
  `lower_binder` is the same closure/binder-construction domain.
- `transfer_constructor_operands`, `transfer_carried_failure_exit_status` —
  aggregate/carrier allocation emission (`aggregate_carrier_authority`,
  `emit_checked_aggregate_alloc`), item 12's RETAIN category, not calls.
- `carry_call_input`, `generated_unit_call_body_callee`, `generated_unit_
  call_entry_callee`, `call_input_transfer_origin_under_mutation`, `callee_
  scheduling_origin_used`, `callee_scheduling_origin_under_mutation` (both
  `#[cfg(test)]`/`#[cfg(not(test))]` forms) — all trace to
  `GOVERNED_ALLOCATION_MUTATION`/aggregate scheduling despite "call"/"callee"
  in their names; property-over-tag, aggregate domain (a different
  frozen-predicate item, not this one).
- `SEED_CALLEE_UNIT_PORTS` (+ `reset_seed_callee_unit_ports`, `seed_callee_
  unit_ports`) — semantically about a callee handoff to `call_declared_unit`,
  but its mutation site (core.rs:13835) is embedded directly inside
  `lower_expr`'s own `Closure`-callee arm, not in a separately movable
  method. Stays with `lower_expr` under the same banned-scope clause that
  keeps `lower_expr` itself in place — the first instance of the injection-
  inside-the-monolith case this node's method section names above.
- `D2fWorkerBodyMutation`, `D2fConsumingCallMutation`, `D2fCallBuildMutation`,
  `D2fPostFieldDirectCallMutation` (+`Scope`/its accessors), `D2fOuterClaim
  StateMutation`, `D2fCaptureProjectionMutation`, and their `fusion_*`
  accessors (`fusion_outer_claim_is_outstanding`, `fusion_capture_projection_
  is_exact`, `fusion_target_carries_claim_authority`) — despite "Call"/
  "DirectCall" in several type names, every accessor is `fusion_`-prefixed;
  this is the `RT-LEXICAL-R3-FUSION-EMITTER` continuation/fusion domain,
  item 12's landed RETAIN, not this item's. A second property-over-tag catch.
- `lower_borrowed_match`, `lower_borrowed_option_match`, `lower_dynamic_host_
  result_match`, `lower_bounded_nat_match`, `lower_dynamic_constructor_match`
  — match-lowering domain (case dispatch over match variants), unrelated to
  calls/returns; a different, not-yet-split item's territory.
- `mrc_census_begin/validator/selector`, `ChildCensusSession` — match-
  recursor-census, item 12's landed "carried_match/static-worker/recursor-
  position-unit cluster."
- `D2fEmitterTestArm` and its RAII family, `record_branched_scrutinee_unit_
  body_*` family — unrelated instrumentation (unit-emission and branched-
  scrutinee domains respectively), confirmed by their own production
  injection sites, not this item's.
- `NativeIntLoweringMutation` — crate-wide shared across five files
  including two entirely outside the bound files (`object_linker_
  packaging.rs`, `core/primitive.rs`); moving it is out of this slice's
  reach regardless of domain.
- `ResultDecoder` — not declared in the bound files at all (it is
  `cranelift_backend/compiled.rs`'s); cross-module reference only, no
  MOVE/RETAIN call to make for this ledger.

### Blind spots / NOT YET CLOSED (stated, not closed — do not read as a plan
### to skip them)

- **The bulk of the population is unswept.** A full method-level census
  measured 88 method-level `fn` in core.rs and 228 in mod.rs (316 total,
  plus roughly 485 top-level items across both files at this SHA); this
  ledger has traced roughly two dozen by name-collision risk and production
  injection point. The remainder is NOT yet individually traced — most are
  believed by domain-pattern to belong to continuation/fusion, checked-
  invocation, aggregate/carrier-emission, occurrence, or computational-
  match/eliminator clusters already landed or traced elsewhere in this
  campaign, but "believed by pattern" does not discharge AC-1; each needs
  its own production-injection-point check before this ledger can claim
  the universal.
- **AC-2 (test-property ledger) has not started.** `core/tests/control.rs`
  is 30289 lines at this SHA; per this campaign's own established bar
  (item 12's D0, twice validated) it must be read exhaustively, not
  marker-sampled. Additionally: this node found at least one `#[cfg(test)]`
  `mod` embedded DIRECTLY inside `core.rs` itself
  (`branched_scrutinee_unit_body_observer_tests`, core.rs:1079) — a test
  population outside `core/tests/` entirely. AC-2's sweep must cover any
  such in-file test modules in both bound files, not just `control.rs`.
- **Re-exports, cfg/attr/derive/repr inventory, and macro-produced items**
  — not yet separately tabulated for any of the traced population, let
  alone the untraced remainder.
- **Source-text oracles / `include_str!` paths** — not yet checked for
  this item's population (Stage A found 49 crate-wide; none yet attributed
  to this owner or ruled out).

### Addendum — full method census swept for both files, AC-1 near-closed
### for the method-level population

Went through every one of the 88 core.rs and 228 mod.rs method-level `fn`
defs (the full census from the previous section) individually, checking
each unclassified name against its enclosing `impl`/type and, where
ambiguous, its actual callers — not stopping at domain-pattern belief.

**New MOVE found (return-emission domain, mod.rs):**

- `transfer_unit_result_into_carrier` (mod.rs — "Transfer the terminal
  value returned by one declared generated unit"; calls `emit_process_exit_
  status` directly, already-confirmed MOVE) and `select_terminal_result_
  origins` (mod.rs, `pub(super)`, called from `units.rs:6137` immediately
  before a unit's result is finalized — the same external-caller pattern
  as `emit_result`). Both write/read the shared `terminal_result_origins`
  field on `FunctionLocalRefs`/`FunctionLocal` (mod.rs-declared, hub-pinned,
  RETAINED — a RETAINED join-disposition read site at core.rs:10774 also
  touches this same field, so the FIELD stays at the hub while these two
  METHODS move, the same hub-stays/methods-move shape item 12 established).

**Every other candidate checked this pass confirmed RETAIN**, each by
production-injection-point or enclosing-`impl` check, not by name:
`OwnedSourceOccurrence::cloned`, `ArtifactHelpers::declare_in_func`,
`FunctionLocalRefs::bind_unit_trap_frame`, `CallInputCalleeDiagnosticMutation
Guard::install` (aggregate call-input-diagnostic family, same domain as
`carry_call_input`), `BoundaryTransferInvokingSiteGuard::enter` (item 11's
boundary domain), `LoweringEnvironmentBinding::value_at`,
`LoweringOperand::effect_seat_phase`/`specialized_at`/`specialized_join_arm`/
`specialized_ref_at`, `ConstructorField::specialized`/`static_worker_refusal`/
`specialized_at`/`into_specialized_at` (constructor-field/static-worker-
disposition, item 12's landed citation), `StaticWorkerCallOutcome::into_
operand`/`into_emitted` + `StaticWorkerEmission` (already MOVE — these two
ARE this item's, tied to `call_static_worker`, not a new finding but
confirmed here), `unit_boundary_environment_record` (exclusive helper of
the already-RETAINED `carry_call_input`), `verify_recorded_composed_
discharges` (D8j composed-discharge domain, checked-invocation),
`child_possible_referent_owners`/`possible_owners_lifetime`/`lowered_
aggregate_shape` (aggregate/carrier-allocation ownership), `fused_redirect_
inputs` (continuation/fusion — calls `resolve_context_capture_claim`
directly), the whole `AggregateAllocationLedger` impl (`open`/`open_body`/
`record_event`/`relate`/`commit`/`open_group`/`claim`/`close_group`/etc.,
mod.rs ~11438-12015), the `D2fPostFieldDirectCallScope`/`D2fGateArrival`/
`D2fGateObservationScope` RAII families (core.rs pre-impl region,
`fusion_`-prefixed accessors confirm the already-landed continuation/
fusion domain), and every method in mod.rs's `12376-13334` block
(`reconcile_declared_children`, `synthesized_dynamic_alternative`,
`dynamic_alternatives_agree`, `reconcile_host_result_root`, `reconcile_
dynamic_alternative`, `synthesized_io_error_alternatives`, and their
enclosing type's constructor/accessor methods) — item 12's own landed
"declared-children reconciliation" domain citation, confirmed by spot
check, not re-derived from scratch.

**Net effect:** AC-1's method-level population for both bound files is now
believed closed modulo the top-level item sweep below (not yet claimed
exact — see the next blind spot). The full MOVE list stands at: `core.rs`
— `call_declared_recursive_position_unit`, `call_declared_context`,
`validate_retained_callable_capture_contract`, `call_static_worker`,
`call_static_worker_with_inputs`, `call_declaration_closure_unit`,
`validate_declaration_unit_call`, plus `RECURSIVE_POSITION_UNIT_CALLS` and
its accessor; `mod.rs` — `call_declared_unit`, `call_declared_declaration_
unit`, `call_declared_unit_target`, `decode_direct_callee`, `emit_result`,
`emit_process_exit_status`, `unwrap_terminal_ret`, `transfer_unit_result_
into_carrier`, `select_terminal_result_origins`, plus the D5 closeout
family, `TrapCallerProtocolMutation` family, and `StaticWorkerCallOutcome`/
`StaticWorkerEmission`.

**Still blind:** the ~485 top-level items (types/consts/statics/traits,
not method-level `fn`) have not been sample-checked the same way as the
methods above — the method sweep does not discharge the top-level-item
class, per AC-1's own conjunctive requirement (declaring a population
bounds the claim, closing every class discharges it, and this ledger has
only closed one class so far). AC-2 (control.rs's exhaustive test read
plus the in-file `core.rs` test module) remains fully open, as does the
re-export/cfg/attr/derive/repr inventory and the source-text-oracle check.

### Addendum 2 — top-level item census, first pass over the "call"-adjacent
### names

Listed all ~150 top-level `struct`/`enum`/`const`/`static`/`type` items in
both files and checked every name plausibly call/callee/return-adjacent
against its actual production usage (not its name):

- `StaticWorkerCallRoute` (mod.rs:3937) — **RETAIN, hub-stays.** Used by
  both this item's `call_static_worker_with_inputs`/`validate_retained_
  callable_capture_contract` AND by RETAINED `composed_recursive_argument_
  binding` and a RETAINED D6a-tagged struct's `members` field. Genuinely
  shared across a moving and a staying consumer — same shape as item 12's
  ~14 hub-stays support types. Stays at mod.rs; the moved functions
  reference it cross-module.
- `D8pApplicationBinding`/`D8pEmittedTarget` (+ their `D8P_APPLICATION_
  BINDINGS`/`D8P_EMITTED_TARGETS` thread_locals and four accessors) —
  **RETAIN**, despite naming ("application binding", "emitted target")
  that reads exactly like this item's domain. Their sole production
  `record_*` call site is inside `consume_checked_ih_marker_at_static_
  worker_call`, already-confirmed checked-invocation, item 12's landed
  domain. A third property-over-tag catch from name alone.
- `RootTerminalAnswerAuthority`/`TerminalAnswerAuthority` (mod.rs ~15038)
  — **RETAIN.** "Terminal Answer Authority" reads as return-domain, but
  its own doc states it is "proof that the native lowering machine has
  reached the checked invocation root with no semantic or control
  continuation left to consume the value" — checked-invocation/
  continuation-consumption proof, consumed by `mint_terminal_answer_
  authority`/`restore_root_terminal_authority`, already classified RETAIN
  during item 12's own D1 investigation of this same region.

No new MOVE items found in this pass. The remaining ~145 top-level items
(D2f/census/checked-frame types in core.rs; the Scale-B/effect-seat/
D3x-D9x mutation-control families, `Lowered`/`LoweringOperand`/boundary
vocabulary, `AggregateAllocationLedger`/`EffectSeatLedger` families, and
the source-machine-adjacent hub-stays types in mod.rs) have NOT been
individually checked — this pass targeted names with real collision risk,
not the full population. AC-1 is closer but still not exact; AC-2 remains
the largest open piece.

### Addendum 3 — AC-2, exhaustive read of `control.rs` IN PROGRESS, one
### new finding, honest read-position marker

`control.rs` holds 221 `#[test]` functions at this SHA (confirmed by
`grep -c "#\[test\]"`; matches expectation — item 12's D2 moved 10 of the
prior 231 out).

**Marker scan first, as a lower bound only — not a certification.** A
scripted scan against the traced MOVE-symbol list (Addenda 1-2) found only
6 of 221 tests with a direct name-level hit: `typed_trap_exit_preserves_
the_planner_identity_across_two_unit_calls` (~8991), `typed_trap_exit_
rejects_a_deleted_or_root_misclassified_unit_lane` (~9022), `typed_trap_
exit_identity_and_caller_protocol_mutations_are_discriminating` (~9048)
— all three hit via `TrapCallerProtocolMutation`, but each ALSO exercises
`TrapFrameBindingMutation`/`TrapIdentityMutation` over one shared "typed
trap exit" fixture reset by one shared `TrapExitMutationReset`. Their own
discriminated property is trap-identity fidelity across the whole
generated-unit call chain, not declared-call emission alone — **RETAIN,
shared/end-to-end**, though the import path for `TrapCallerProtocolMutation`
moves out from under them at D2 and needs updating, same pattern as a
cross-module test import elsewhere in this campaign. `governed_nested_
brackets_n3_through_n7_emit_complete_functionized_bundles` (9998) — hits
`recursive_position_unit_calls`; RETAIN, this is a population/route
control over the FunctionizedUnits authority as a whole (asserts
`declared`/`defined`/`resolved`/`recursive_calls`/`carried_unchanged`
jointly), not a single-owner declared-call test. `d5_c4_a_duplicated_
checked_occurrence_is_refused_after_its_lawful_first` (13695) and
`d5_the_checked_call_closeout_rejects_omission_duplication_and_a_
substituted_callee` (13810) — hit `D5CloseoutMutation`/`d5_emitted_
declaration_calls`; both directly exercise `call_declared_unit_target`'s
own closeout family named MOVE in Addendum 1 — **MOVE candidates**,
pending the full read reaching them again in context.

Per this campaign's own established bar (restated verbatim in this item's
own kickoff, `evt_329941cn5vx2e`): **do not certify a residual population
"empty" from zero marker hits — read it.** Zero marker hits on the other
215 tests is not AC-2 closure. Reading exhaustively, sequentially, from
line 1.

**Read through line 11,150 of 30,289 (about 37%) so far.** Every test
encountered classifies as RETAIN into an already-landed or explicitly
frame-named residual domain — continuation/fusion (`d2f_*`/`r3_fused_*`),
checked-invocation, oriented-subcontinuation (`px8j_*`/`px8ds_*`),
planner-side (`contkey_*`, `RT-REQUIRED-CONSUMER-*`), unit-emission
census (`b2f_*`), source-join/D8 disposition, D6a/D6b static-worker
environment-assembly controls (D6a's own doc names the coupled types this
item does NOT own: `StaticWorkerCallRoute` and case-environment assembly
are the D6a/`RT-CONTSRC-PRODUCER-LOCAL` owner, already RETAIN in
Addendum 2), or the cross-cutting census/closure class the frame itself
names as always-residual (`the_backend_production_surface_inventory_is_
closed`, `correspondence_adds_no_emitted_unit_to_the_production_census`,
`the_owner_classification_has_a_closed_production_naming_inventory`, and
siblings) — **with one exception**:

- **`d6_a_functionized_recursive_declaration_accepts_a_changing_argument_
  constructor` (control.rs:6671) — MOVE candidate, found only by the
  exhaustive read, invisible to the marker scan.** Its own doc comment
  states the subject directly: a functionized recursive declaration
  "ACCEPTS a changing argument constructor through its one `ValueWord`
  parameter" and "it reds if the declared-call path ever acquires a
  per-constructor shape predicate." This is an end-to-end domain test for
  this item's declared-call-emission owner — it exercises the property
  through the full `compile_expr_into_module` pipeline rather than
  calling any MOVE-listed function by name, which is exactly why the
  marker scan missed it and exactly the failure mode this campaign's own
  AC-2 discipline exists to catch (matching item 12's own two late
  corrections, both surfaced the same way).

Also in progress, not yet finished classifying: `b2f_emits_one_defined_
target_unit_per_planned_function_unit` (9701) — read through its
`leaf_declared`/`leaf_defined` assertions (RETAIN-leaning, unit-emission
census domain, same class as the other `b2f_*` tests), but the read
stopped mid-function before its `closure_declared`/`closure_defined`
assertions and closing brace.

**What is NOT yet read:** the remaining ~63% of `control.rs` (lines
11,151-30,289), including the two D5-closeout marker hits in fuller
context and the embedded `branched_scrutinee_unit_body_observer_tests`
`#[cfg(test)] mod` found nested directly inside `core.rs` itself (outside
`core/tests/` entirely, flagged as a blind spot in the first partial
commit) — AC-2 is not closed and this addendum does not claim it is.
Continuing the read next.

### Addendum 4 — CORRECTION to Addendum 3's tentative D5-closeout MOVE
### call: the fuller D5 test cluster is end-to-end, not single-owner

Continued the exhaustive read through line ~14,000 (about 46%). Read the
two Addendum-3 marker hits (`d5_c4_a_duplicated_checked_occurrence_is_
refused_after_its_lawful_first`, `d5_the_checked_call_closeout_rejects_
omission_duplication_and_a_substituted_callee`) in their full surrounding
context — a cluster of 7 more D5/checked-recursive-invocation tests
(control.rs:13020-14000: `d5_c2_the_witness_reaches_the_seam_and_emits_
the_exact_planner_target`, `d5_c2_mutual_same_scc_calls_reconcile_and_
emit`, `d5_c4_checked_plan_mutations_each_reach_their_own_authority`,
`d5_the_recursion_group_axis_is_inert_on_a_self_call_and_causal_on_the_
mutual_pair`, `d5_the_closeout_planned_set_comes_from_the_plan_not_from_
the_emissions`, plus two escape/D6 tests) all draw on the same shared
`d5_compile`/`d5_mutual_compile` helpers and the same `D5_DECLARATION`/
`D5_FRAME_CARRIER` fixtures, already correctly homed at the LCA hub in
`core/tests/mod.rs`.

**Traced `enter_checked_recursive_invocation` (mod.rs:15397) to settle
which authority these tests are really about.** It validates a
`CheckedRecursiveInvocation` marker against the plan (marker wraps
exactly one matching `Call`, template not already consumed, affine
push/pop via `active_recursive_invocations`) — the identical shape as
its neighbors `enter_checked_subcontinuation_frame`/`enter_checked_
computational_ih_invocation`, both already RETAIN in Addendum 1's
checked-invocation cluster. **RETAIN, same cluster, item 12's landed
domain** — not item 13's.

`d5_c4_checked_plan_mutations_each_reach_their_own_authority` names its
own "owning plane" per row explicitly as one of four *different*
authorities: `"D5"`, `"enter_checked_recursive_invocation"`,
`"OrientedSubcontinuationPlanV1::validate"` (planner-side, not in the
bound files at all), and `"planning::validate_oriented_subcontinuation_
transport"`. **This whole cluster's own stated subject is that a checked
call is validated correctly across every one of those authorities
together** — same-SCC membership, transport reconciliation, and (only as
its LAST link) the closeout ledger. That is an end-to-end control over a
multi-owner pipeline, not a single-owner domain test for this item, by
the same reasoning that put `typed_trap_exit_*` (Addendum 3) in the
shared/end-to-end class rather than MOVE.

**Correction:** the two Addendum-3 tests are demoted from "MOVE
candidate, pending fuller context" to **RETAIN, shared/end-to-end**,
joining the rest of the D5 cluster — same normalized position, arrived
at only once the surrounding rows were read rather than the two tests in
isolation. This is exactly why AC-2 requires reading in place rather than
snippet-by-snippet: the two hits looked like single-owner declared-call
tests from the marker scan alone and only settled once their neighbors
were read. `D5CloseoutMutation`/`d5_emitted_declaration_calls` themselves
stay MOVE (Addendum 1's own trace — their production home is
exclusively inside `call_declared_unit_target`); what moves is the
production mechanism, not the end-to-end tests that exercise it through
several other owners at once. Those tests' imports will need updating at
D2 to reach the moved names cross-module, same pattern noted for
`typed_trap_exit_*`.

**Read through line ~14,000 of 30,289 (about 46%).** Continuing next.

### Addendum 5 — AC-2 progress checkpoint, no new findings, a pattern
### has stabilized

Continued the exhaustive read from line ~14,000 through line ~18,200
(about 60%). This entire span (D5a's checkpoint-4 discriminators for
generated continuation contexts/static-worker-call routing, D6a's eight
upstream checked-route-composition rows, D4a/D3b/D3c producer-local and
entry-ABI slot-position controls, D7a/D8a/D8b composed-worker-view and
composed-call-target selector laws, effect-seat plan/visit mutations)
classifies RETAIN, all `RT-CONTSRC-PRODUCER-LOCAL`/`RT-DECL-CLOSURE-
PORT`/`RT-CONTINUATION-EDGE-DISPOSITION`-tagged planner-and-continuation
domain — none of it single-owner to this item's declared-call/return
emission.

No new MOVE candidates in this span. The frame's own D0 characterization
of `control.rs` ("planner/occurrence, continuation/fusion, function-
state and source-machine, emitter and join/trap controls, plus cross-
cutting census and closure tests") is bearing out directly: the
continuation/planner-side populations are the dominant bulk of the file,
and this item's own domain tests are comparatively sparse and mostly
concentrated where the marker scan already found direct hits, plus the
one exhaustive-read-only finding (`d6_a_functionized_recursive_
declaration_accepts_a_changing_argument_constructor`, Addendum 3).

**Read through line ~18,200 of 30,289 (about 60%).** Continuing next.

### Addendum 6 — AC-2 progress checkpoint, read through ~69%, still no
### new findings

Continued through line ~21,000 (`RT-CONTSRC-PRODUCER-LOCAL` `D8a`
through `D8n`: composed-call-target minting/discharge/partition laws,
the ordinary-envelope source-position repair, the checked-bridge
source-frame-identity survival). All RETAIN, same planner/continuation
domain as Addendum 5 — none of it single-owner to this item.

**Read through line ~21,000 of 30,289 (about 69%).** Continuing next.

### Addendum 7 — AC-2 progress checkpoint, read through ~74%

Continued through line ~22,500 (`D8n`/`D8o`/`D8m`: per-function checked-
frame consumption lifetime, per-body planner-issued authority binding,
the transported-tuple and two-occurrence checked-bridge controls). All
RETAIN, same planner/continuation domain. No new MOVE candidates.

**Read through line ~22,500 of 30,289 (about 74%).** Continuing next.

### Addendum 8 — AC-2 progress checkpoint, read through ~79%

Continued through line ~24,000 (`D8m` bridge-arm partition/bijection,
`D8p`: checked-IH-application binding at the source-machine's call edge
— "a checked application binds and emits in every defining body that
lowers it"). Traced `D8p`'s own subject: which call edges consult the
checked-IH marker consumption seam, not the emission target itself —
same checked-invocation domain as the rest of the D5-D8 family (item
12's landed cluster), RETAIN. No new MOVE candidates.

**Read through line ~24,000 of 30,289 (about 79%).** Continuing next.

### Addendum 9 — AC-2 progress checkpoint, read through ~84%

Continued through line ~25,500 (`D8f` remaining checked-marker
refusals, `D8g` functionized/composed table-and-suffix binding at the
shared emitter, `D6b` mixed-pair/table-agreement correspondence, `D6c`
pre-emission selection refusals). All RETAIN, same
`RT-CONTSRC-PRODUCER-LOCAL` planner/continuation/static-worker-route
domain. No new MOVE candidates.

**Read through line ~25,500 of 30,289 (about 84%).** Continuing next.

### Addendum 10 — AC-2 progress checkpoint, read through ~88%, domain
### transition noted

Continued through line ~26,600: closed out `D6c` (sealed binder-run
shape refusals) and `D9b` (ordinary-run assembly, planner role
sequence), both RETAIN, same `RT-CONTSRC-PRODUCER-LOCAL` cluster.

The file then transitions to `RT-SRCBODY-BIND-ORDER` `D3` (control
1-4): whether a `CallableDeclaration`/`ClosureBody`'s own parameters
bind into its semantic environment in de-Bruijn (reversed) or
descriptor order, and whether the process root and a generated-context
seat obey the same law. **RETAIN** — this is the callee's own
environment-construction mechanism (what order a declaration body's
*own* parameters resolve to inside that body), not the call-emission or
return-emission mechanism this item owns. A different domain from
everything read so far, but still not this item's.

**Read through line ~26,600 of 30,289 (about 88%).** Continuing next.

### Addendum 11 — AC-2 progress checkpoint, read through ~91%

Continued through line ~27,700 (`RT-SRCBODY-BIND-ORDER` remainder,
`RT-PRODUCER-MATCH-PORT`, `RT-CARRIED-CONTINUATION-RESUME`,
`RT-CARRIED-ORDINARY-COMPOSITION`, `RT-SPECIALIZED-ACTIVE-RESUME`,
`RT-CONTINUATION-EDGE-DISPOSITION` D1-D3). All RETAIN — join/candidate/
continuation-route domain, no new MOVE candidates.

**Read through line ~27,700 of 30,289 (about 91%).** Continuing next.

### Addendum 12 — AC-2 progress checkpoint, read through ~95%

Continued through line ~28,900 (`RT-CONTINUATION-EDGE-DISPOSITION` D3's
five mutation rows and their cross-arm non-collapse proof,
`RT-CALL-EDGE-EXECUTABILITY-AXIS`'s boundary sentinel,
`RT-LEXICAL-RECURSOR-CONSUMERS` D2b/D2k-1b-i). All RETAIN — same
continuation/planner/occurrence domain. No new MOVE candidates.

**Read through line ~28,900 of 30,289 (about 95%).** Continuing next;
close to the end of the file.

### Addendum 13 — AC-2's exhaustive read of `control.rs` is COMPLETE, plus
### the embedded core.rs module; AC-2 result summarized

Finished the read: lines 28,901-30,289 (`RT-LEXICAL-RECURSOR-CONSUMERS`
`D2k-1b`/`D2k-1c-0`/`D2k-1c-1` static-worker-field conservation ledger,
`RT-REFUSAL-PINS-REHOMED`, `RT-LEXICAL-R3-FUSION-EMITTER` `DP`/`D3`/
`AC-D3-SELF` composition-time membership and the fusion-local
composition ledger). All RETAIN — planner/fusion/conservation-ledger
domain, none of it single-owner to this item.

**Also read the blind spot flagged in the first partial commit**: the
`#[cfg(test)] mod branched_scrutinee_unit_body_observer_tests` embedded
directly inside `core.rs` (lines 1079-1111, outside `core/tests/`
entirely) — 3 tests, all exercising
`recursive_position_construct_argument` (direct/non-carrying source-
constructor position resolution) and the `branched_scrutinee_unit_body`
route1 observer bench. **RETAIN** — source-branch/eliminator-frame-
descent domain, item 12's landed cluster (`computational-match/
eliminator-frame descent`), not this item's declared-call/return
domain.

**AC-2 is CLOSED for the test-property ledger.** Every `#[test]` in
`control.rs` (221 of them) and the 3 embedded in `core.rs` have been
individually read in place, not marker-sampled. Final tally against the
established MOVE population:

- **Confirmed MOVE**: `d5_c4_a_duplicated_checked_occurrence_is_refused_
  after_its_lawful_first` and `d5_the_checked_call_closeout_rejects_
  omission_duplication_and_a_substituted_callee` were tentatively MOVE
  (Addendum 3) then corrected to RETAIN/shared (Addendum 4) once their
  full D5 cluster context was read.
- **One genuine MOVE finding, found only by the exhaustive read**:
  `d6_a_functionized_recursive_declaration_accepts_a_changing_argument_
  constructor` (control.rs:6671, Addendum 3) — a declared-call-emission
  domain test invisible to the marker scan.
- **Everything else** (essentially the whole of `control.rs`) classifies
  RETAIN, distributed across already-landed or explicitly frame-named
  domains: continuation/fusion, checked-invocation, planner/occurrence
  (`RT-CONTSRC-PRODUCER-LOCAL`'s D3-D9 families, `RT-CONTINUATION-EDGE-
  DISPOSITION`, `RT-LEXICAL-RECURSOR-CONSUMERS`, `RT-LEXICAL-R3-FUSION-
  EMITTER`, `RT-SRCBODY-BIND-ORDER`, `RT-CARRIED-*`,
  `RT-SPECIALIZED-ACTIVE-RESUME`, `RT-DECL-CLOSURE-PORT`,
  `RT-CONTSPEC-*`, `RT-FNSPLIT-B2F`, `RT-SEED-CALL-PORT`,
  `RT-RECURSOR-TRANSPORT`, `RT-MATCH-SCRUTINEE-DISPOSITION`,
  `RT-CALL-EDGE-EXECUTABILITY-AXIS`, `RT-REFUSAL-PINS-REHOMED`), plus
  the frame's own named cross-cutting census/closure class. This bears
  out the frame's own D0 characterization of `control.rs` verbatim: it
  holds several independent populations, of which this item's own
  domain is a small, mostly-already-marker-findable fraction.
- **Shared/end-to-end class** (Addendum 3/4): the `typed_trap_exit_*`
  cluster and the D5 checked-recursive-invocation cluster both touch
  this item's moved mechanism from outside, alongside other owners'
  mechanisms, and stay in residual `control.rs` per the frame's own
  four-way AC-2 partition.

**What AC-2 does NOT yet close**: the D2 test-move plan itself (which
shared fixtures need widening, where the moved tests land in the new
module) is not drafted — that is `D2`'s own work once `D1` lands the
production move. AC-1's top-level item sweep (~485 items, per Addendum
2, only ~20-25 collision-risk names individually checked) remains the
largest open piece of this D0. Re-exports/cfg/attr/derive/repr
inventory and source-text-oracle checks (AC-1's other item classes)
remain untouched, as stated since the first partial commit.

### Addendum 14 — AC-1 top-level item sweep, by production census and
### domain-cluster attribution rather than name-by-name re-derivation

Ran a full top-level-item census over both bound files (regex over
`struct`/`enum`/`const`/`static`/`type`/`trait`/`thread_local!`
declarations, distinct from the method-level census in Addendum 2):
**46 items in `core.rs`, 205 in `mod.rs`** (251 total — the earlier
"~485" figure from Addendum 1 mixed in the 316 method-level items
already closed there; this pass is top-level items only).

**Method used, stated honestly.** Individually re-deriving all 251 by
fresh production-injection tracing would duplicate work already done:
the AC-2 exhaustive test read (Addenda 3-13) traced the PRODUCTION ROLE
of every `D2f`/`D3b`/`D3c`/`D4a`/`D4b`/`D5`/`D5a`/`D6a`/`D6b`/`D6c`/
`D8*`/`D9*`-prefixed mutation-control type while reading the tests that
exercise it — each one was pinned to a specific domain (fusion,
producer-local continuation, checked-invocation, static-worker-route,
composed-call-target/discharge, envelope assembly) via its own doc
comments and the test bodies that construct/consume it. That population
— the large majority of the 251 — is attributed here by that established
tracing, not re-traced fresh. What IS newly, individually checked in
this pass is every name in the census not already covered by that
tracing or by Addenda 1-2's earlier passes:

- **`SourceCallee`** (mod.rs:14681, enum) — the one name this pass
  flagged as needing a fresh check: "callee" in the name, and its own
  doc comment cites `D8e`/`D8p` (checked-invocation). Traced its sole
  consumer: `grep` found every construction/match site exclusively in
  `source.rs` (`source_call_state`, item 12's already-moved,
  out-of-bound-files module) — none in `core.rs`/`mod.rs` itself.
  **RETAIN** — same `D8e`/`D8p` checked-invocation cluster already
  established RETAIN throughout the AC-2 read, despite sitting
  declared in `mod.rs` while used only by `source.rs`. (Whether item
  12's own D1 should have carried this type with it is a question about
  item 12, not this D0's boundary call — noted, not adjudicated here.)
- **`Lowering<'a>`** (mod.rs:2753) and **`Lowered`** (mod.rs:3194) — the
  central lowering-state struct and the central value-representation
  enum. Confirmed by inspection: both are used throughout the whole
  `core.rs`/`mod.rs` pair, including by item 13's own MOVE-listed
  methods (`call_declared_unit_target` takes `&mut Lowering`, returns
  `Lowered`). **RETAIN, hub-stays** — shared infrastructure no single
  domain owns, same shape as item 12's own ~14 hub-stays support types.
- **`StaticWorkerEmission`/`StaticWorkerCallOutcome`** (mod.rs:3768,
  3780) — cross-checked against the interim posts already folded into
  Addendum 1: confirmed MOVE there ("used exclusively by
  `call_static_worker`/`call_static_worker_with_inputs`"), consistent
  with this pass's finding of no other consumers.

No new MOVE candidates surfaced by this census beyond what Addenda 1-3
already found. The remaining ~245 items are attributed to their
domain-cluster by name-prefix and doc-comment reading (D2f fusion,
D3b/D3c producer-local slot resolution, D4a/D4b generated-context
seats, D5/D5a checked-IH/generated-context, D6a/D6b/D6c static-worker
route and selection, D8*/D9* composed-call-target/discharge/envelope,
Px8j/Px8tr source-machine and trap-provenance, effect-seat/aggregate-
allocation, bounded-nat/dynamic-constructor encoding, checked/oriented
continuation frame types) — **this is domain-cluster attribution
grounded in the AC-2 read's own tracing, not a fresh independent trace
of each of the 245**, and is stated as such rather than claimed as
individually exact. AC-1's own bar ("each class needs its own fresh
selector... plus explicit manual closure for what it cannot see")
is not yet fully met by this pass for the top-level-item class; a
reviewer wanting per-item exactness should treat this addendum as a
strong prior, not a substitute for a fresh independent census.

**Still fully untouched, stated again for the handback:** the
cfg/attribute/derive/repr/visibility inventory, re-exports, macro-
produced items, and source-text-oracle checks AC-1 also requires;
D2's own test-move plan; the `RECURSIVE_POSITION_UNIT_CALLS`
thread_local-block split owed at `D1`.

Handing back here — this is a natural, well-documented stopping point
after AC-2's full closure and this top-level census, per the standing
instruction to checkpoint and hand back rather than push an
under-verified AC-1 claim past what was actually traced.

### Addendum 15 — the four Architect-required item classes, filtered to
### the closed MOVE set (Architect ruling `evt_1zd4j4pmn1rxn`)

Per the Architect's D0 ruling: the ~245 ordinary top-level RETAIN items
need no fresh re-trace (compiler-backstopped by `E0603` plus his own
mandatory D1 per-mover visibility review). What is required before the
endorsing D0 vote is four bounded item classes — re-exports, cfg/
attribute-gated items, macro-produced items, source-text oracles —
filtered to **this item's already-closed MOVE set** (the method
population plus its exclusive support types), because these four are
NOT compiler-caught: a silent over-widening or drop in any of them
compiles green.

**The MOVE set this pass filters against** (restated from Addenda 1/3
for a single reference point):

- `core.rs`: `call_declared_recursive_position_unit`,
  `call_declared_context`, `validate_retained_callable_capture_contract`,
  `call_static_worker`, `call_static_worker_with_inputs`,
  `call_declaration_closure_unit`, `validate_declaration_unit_call`,
  `RECURSIVE_POSITION_UNIT_CALLS` (thread_local),
  `recursive_position_unit_calls`.
- `mod.rs`: `call_declared_unit`, `call_declared_declaration_unit`,
  `call_declared_unit_target`, `decode_direct_callee`, `emit_result`,
  `emit_process_exit_status`, `unwrap_terminal_ret`, `transfer_unit_
  result_into_carrier`, `select_terminal_result_origins`,
  `D5_CLOSEOUT_MUTATION`/`D5_EMITTED_DECLARATION_CALLS` (thread_local),
  `D5CloseoutMutation`, `with_d5_closeout_mutation`, `reset_d5_emitted_
  declaration_calls`, `d5_emitted_declaration_calls`,
  `TrapCallerProtocolMutation`, `set_trap_caller_protocol_mutation`,
  `StaticWorkerCallOutcome`, `StaticWorkerEmission`.

#### Class 1 — re-exports: CLOSED, zero touch this item's MOVE set

Filtered the frame's own 57-statement re-export census
(`docs/program/backend-split-census-reexports.md`) against every
MOVE-set name above: **zero of the 57 statements name any MOVE-set
symbol**, in any profile (default, named-feature, or test-only).
Cross-checked live against the current tree (the census is a pinned-SHA
snapshot, not an authority) with a direct grep for every `use` statement
in the crate mentioning any MOVE-set name: also zero. **Disposition for
every MOVE-set item: none are re-exported; D1 introduces no re-export
disposition question for this class.**

#### Class 2 — cfg/attribute-gated items: THREE real surfaces found, all
#### named

This is the substantive finding this pass exists for — a real, non-
obvious over-widening hazard inside the already-closed MOVE set:

1. **`RECURSIVE_POSITION_UNIT_CALLS` + `recursive_position_unit_calls`
   (core.rs) are themselves `#[cfg(test)]`-gated**, sharing a `#[cfg(test)]
   thread_local! { }` block with the RETAINED `C2_UNIT_EMISSION_EPOCH`/
   `SUPPRESS_REQUIRED_CONSUMER_ROUTE`/`REQUIRED_CONSUMER_ROUTE_
   SUPPRESSIONS` (core.rs:29-35). Already flagged as needing the LRC/CCR-
   style block split at `D1` (Addendum 1); now confirmed the split must
   also carry the `#[cfg(test)]` gate onto BOTH halves correctly, or
   either half silently compiles unconditionally.
2. **`TrapCallerProtocolMutation` + `set_trap_caller_protocol_mutation`,
   and the `D5_CLOSEOUT_MUTATION`/`D5_EMITTED_DECLARATION_CALLS`/
   `D5CloseoutMutation`/`with_d5_closeout_mutation`/`reset_d5_emitted_
   declaration_calls`/`d5_emitted_declaration_calls` family, all sit
   inside ONE shared `#[cfg(test)] thread_local! { }` block (mod.rs:1967-
   1997)** alongside the RETAINED `STATIC_WORKER_MUTATION`/
   `TRAP_FRAME_BINDING_MUTATION`/`TRAP_IDENTITY_MUTATION` — the same
   split-required shape as (1), one level up: a `#[cfg(test)]`-gated
   block containing both MOVE and RETAIN statics that D1 must split into
   two blocks, both correctly re-gated.
3. **`StaticWorkerCallOutcome` (mod.rs:3780) is a production enum with a
   `#[cfg(test)]`-gated variant**, `DeferredPostField(LoweringOperand)`,
   and both its `impl` methods (`into_operand`, `into_emitted`) match on
   that variant behind `#[cfg(test)]` match arms. Moving the enum without
   carrying the variant and both match arms intact silently narrows the
   type under the test profile only — invisible to a `-p` library-only
   build, exactly the risk class named.

**Beyond the type/const declarations themselves**, several MOVE-set
*functions'* own bodies contain `#[cfg(test)]`/`#[cfg(not(test))]`
surfaces that must travel with them: `call_declared_unit` and
`call_declared_unit_target` (mod.rs) both take a `#[cfg(test)] launch_
ingress: Option<cranelift_codegen::ir::Value>` **parameter** — their
signatures genuinely differ between profiles; `call_declared_unit_
target`'s body has a paired `#[cfg(test)]`/`#[cfg(not(test))]` branch at
the `AbiSlotKind::Trap` arm reading `TRAP_CALLER_PROTOCOL_MUTATION`
(production writes a constant zero under `cfg(not(test))`, the test arm
can write a stale-trap sentinel instead) plus a further `#[cfg(test)]`
block at the `Result` arm; `emit_result` has two `#[cfg(test)]` blocks;
`call_declared_recursive_position_unit` (core.rs) has five, including a
`#[cfg(test)]`-gated argument-position swap in a called-function's
argument list; `call_static_worker_with_inputs` has one, a `#[cfg(test)]`
call to `lrc_d2b_record_worker_call` (a RETAINED cross-domain
instrumentation hook called from inside a MOVE-set function). None of
these are new domain findings — the mutation-control types they touch
were already classified — but every one is a concrete site where D1's
move must carry the cfg gate exactly, named here so the mover can check
against this list rather than discover a silent narrowing after the
fact. No `#[repr(...)]` or non-standard `#[derive(...)]` found anywhere
in the MOVE set (only ordinary `Clone`/`Copy`/`Debug`/`Eq`/`PartialEq`).

#### Class 3 — macro-produced owned items: CLOSED, zero found

No `macro_rules!` is defined in either bound file. No MOVE-set symbol
name appears adjacent to any macro invocation other than the already-
accounted `thread_local!` blocks covered in Class 2 (direct grep for
every MOVE-set name followed by `!(`/`![`/`!{`, excluding `thread_local!`
itself: zero hits). Every derive on a MOVE-set type is an ordinary std
derive, not a custom derive macro that could expand to additional owned
items. **Nothing in this item's MOVE set is macro-produced.**

#### Class 4 — source-text oracles: ONE load-bearing, self-defending
#### surface named; zero symbol-name string matches

Direct grep for every MOVE-set symbol name appearing as a quoted string
literal anywhere in `core.rs`, `mod.rs`, `core/tests/control.rs`, or
`core/tests/mod.rs`: **zero hits** — no census/oracle test is keyed on
any MOVE-set symbol's exact name as text.

The one real oracle surface this item's `D1` must touch is
**`BACKEND_PRODUCTION_SOURCES`** (`control.rs:8052`), the fixed roster
of `(path, include_str!(path))` pairs that `the_backend_production_
surface_inventory_is_closed` and its sibling census tests iterate over.
It already lists `("lowering/core.rs", ...)` and `("lowering/mod.rs",
...)`; a new module `D1` creates for the MOVE set must be added to this
same list, following the exact precedent already documented in-line for
`boundary.rs`/`source.rs`/`units.rs`/`seed_material.rs` (each added at
its own item's `D1`, with a comment naming the item). **This is not a
silent hazard**: the roster carries its own companion check (`control.rs`
~8298-8309) asserting `declared.len() + 1 == BACKEND_PRODUCTION_SOURCES.
len()` against the file's own `mod` declarations — an omission is loudly
caught by that assertion, not silently absorbed, provided the check
itself isn't weakened at `D1`. Named here so the mover adds the entry as
routine, not as a discovery.

**Summary for the endorsing vote**: Class 1 and Class 3 are fully
closed with zero findings. Class 2 has three real cfg-gated surfaces
plus a half-dozen function-body cfg sites, all named above — none are
new domain/MOVE-vs-RETAIN findings, all are "carry this gate exactly at
D1" items. Class 4 has one named, self-defending roster entry to add.
None of the four surfaces a reason to revise the MOVE/RETAIN boundary
already closed in Addenda 1-14.

# `D1` — THE MOVE. Behaviour-preserving, and reviewable as a relocation.

Move the owner into its own child module, extending the established seam.
Adapters are permitted **as transitional scaffolding only**, and item 18 deletes
them.

# `D2` — THE COMPANION TEST MOVE. Separate accepted partial.

`lowering/core/tests/control.rs` was **33,969 lines at
`a1cf83622`** and is **in scope** — the
operator's constraint says large files and excepts nothing, and a test file is
not exempt. **It is a companion axis, not a phase of its own.**

**Move only the tests whose primary discriminated property belongs to the owner
this slice just established** (declared-call, residual-call, recursor-call and
return-emission controls). Place multi-leaf fixtures **once**,
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

# `D1` LEDGER -- the move executed, against `35a2997b6` (rebased tip after the
# D0 merge landed at `origin/main` `83d199f4f`)

Executed per `runtime-leader`'s kickoff (`evt_7t9ryq8pbmndv`), against the
Architect-approved D0 boundary (`dec_3gnfwhxw1sm7k`, verdict `evt_60pj00z2jnve1`).
Re-derived every MOVE-set symbol's current line position by name at pickup
(the contention section's own rule), not from the D0 ledger's line offsets --
the tree had not moved since D0 was docs-only, and the re-derived positions
matched Addendum 15's within a handful of lines.

## New module: `cranelift_backend/lowering/calls.rs`

2,054 lines (`AC-4b`: well under the 10k ceiling; no other file this slice
touches crosses a ceiling it was not already over -- see the line-count table
below). Extends the `boundary.rs`/`source.rs` seam (items 11/12): a sibling of
`core`/`units`/`seed_material`/`boundary`/`source`, all children of
`lowering`/`mod.rs`. Declared via
`pub(in crate::cranelift_backend) mod calls;` in `mod.rs`, following the exact
precedent shape (doc comment + declaration, positioned after the `source`
block).

Every moved method sits in its own small `impl<'a> Lowering<'a> { .. }` block
-- `source.rs`'s own shape (it has five such blocks). This is not a semantic
change: Rust does not require a type's inherent methods to share one `impl`
block, and method-call resolution (`self.foo()`) is unaffected by which block
or file supplies the definition, provided visibility allows the call site to
reach it.

## The MOVE

Every symbol from Addendum 1/15's list moved, verbatim:

**From `core.rs`:** `call_static_worker`, `call_static_worker_with_inputs`,
`validate_retained_callable_capture_contract`,
`call_declared_recursive_position_unit`, `call_declared_context`,
`call_declaration_closure_unit`, `validate_declaration_unit_call`,
`RECURSIVE_POSITION_UNIT_CALLS` (thread_local, split out of the shared
`#[cfg(test)]` block it sat in), `recursive_position_unit_calls` (accessor).

**From `mod.rs`:** `TrapCallerProtocolMutation`, `D5CloseoutMutation`,
`with_d5_closeout_mutation`, `reset_d5_emitted_declaration_calls`,
`d5_emitted_declaration_calls`, `set_trap_caller_protocol_mutation`,
`transfer_unit_result_into_carrier`, `select_terminal_result_origins`,
`call_declared_unit`, `call_declared_declaration_unit`,
`call_declared_unit_target`, `decode_direct_callee`, `unwrap_terminal_ret`,
`emit_result`, `emit_process_exit_status`.

**Verified byte-identical** (`AC-3`) by dedenting the moved text and diffing
against the pre-move blob at each item's original span: spot-checked the
largest item from each source file (`call_static_worker_with_inputs`, 331
lines, zero diff; `call_declared_unit_target`, 297 lines, diff shows exactly
the two intentional changes below and nothing else) -- both the extraction
script's mechanical slicing and the compiler (a clean `cargo build`/`cargo
test` after the move, below) corroborate the same conclusion from two
independent angles.

## A GENUINE D0 GAP FOUND AND CLOSED AT EXECUTION: `StaticWorkerEmission` /
## `StaticWorkerCallOutcome` are hub-stays, NOT movers

Addendum 1 recorded these two as "exclusive to `call_static_worker`/
`call_static_worker_with_inputs`". Moving them and building surfaced two
RETAINED consumers the D0 ledger's tracing missed:
`dispatch_fused_consuming_call` (core.rs, already confirmed RETAIN in
Addendum 1's own "other domain" list, continuation/fusion) constructs
`StaticWorkerCallOutcome::Emitted`/`::DeferredPostField` directly, and
`claim_composed_discharge` (core.rs, `pub(super)`, RETAINED) destructures
`StaticWorkerEmission`'s fields directly.

This is exactly the standing carry from item 12's own retro (`runtime-
implementer` evt_sa23ftgphwd6, `agent/memory/teams/runtime/`): *"treat a D0
ledger's silence on a manipulated type's own disposition as an unclosed AC-1
gap to close by usage-tracing at execution time... default to hub-stays/
methods-move when tracing shows sharing."* Applied here: both types moved
back to the `mod.rs` hub (their original declaration site), with an in-line
comment recording why (quoted above) so a future reader does not re-trace the
same ground. `call_static_worker`/`call_static_worker_with_inputs` (which DO
move) reach them via `calls.rs`'s own `use super::*;` -- no import needed,
since they are hub types the whole subtree already inherits.

**Net effect on the MOVE set:** these two names are struck from it. Everything
else in Addendum 1/15's list moved as ledgered.

## Visibility changes (`AC-3`'s public/export-profile clause)

**Eleven items widened from private to `pub(super)`** on the move (all in the
new `calls.rs`), each because a RETAINED caller reached them only because the
callee used to sit in an ANCESTOR module (`lowering` or `lowering::core`,
whose privacy is visible to every descendant); as a SIBLING module the same
reachable set requires the visibility spelled out rather than inherited by
descent. The reachable population is unchanged -- this is the ordinary
sibling-module visibility recompute item 11/12 also needed on their own moves,
not new scaffolding, so **no `AC-5` ledger entry**:

| symbol | RETAINED caller found by compiler-driven trace |
|---|---|
| `call_static_worker` | `lower_expr` (core.rs, the banned-from-splitting monolith) |
| `validate_retained_callable_capture_contract` | `construct_static_worker_binding`, `lower_expr` (core.rs) |
| `call_declared_unit_target` | `dispatch_fusion_owned_outer_realization`, `claim_and_call_resolved_continuation_inner`, `dispatch_fused_consuming_call`, `lower_expr` (core.rs) |
| `call_declared_declaration_unit` | `lower_declaration_ref` (core.rs -- its `SchedulingEntry`-class arm) |
| `decode_direct_callee` | `verify_emitted_continuation_calls`, `verify_recorded_composed_discharges` (mod.rs) |
| `unwrap_terminal_ret` | `merge_scalar_operand` (mod.rs) |
| `emit_process_exit_status` | `merge_scalar_operand` (mod.rs), `carried_join_arm` (core.rs) |
| `RECURSIVE_POSITION_UNIT_CALLS` (static) | `compile_expr_into_module`'s inline reset (core.rs, `RETAINED`, touches the raw static, not an accessor -- kept verbatim per the D0 ledger's own note that converting it to an accessor call would be a semantic change beyond a pure move) |
| `recursive_position_unit_calls` | the `core/tests/control.rs` test-glob chain |
| `TrapCallerProtocolMutation` (enum) | the `core/tests/control.rs` test-glob chain |
| `set_trap_caller_protocol_mutation` | the `core/tests/control.rs` test-glob chain |

**Three additional RETAINED methods widened from private to `pub(super)`**
(in `core.rs`, where they stay) because a MOVING method now calls them as a
cross-sibling-module `self.` dispatch: `child_occurrence`, `dispatch_
fused_consuming_call`, `resolve_context_capture_claim`. Same reasoning,
opposite direction -- the reachable population (which movers can call which
retained helpers) is unchanged.

Everything already `pub(super)` from item 12 (`call_declared_recursive_
position_unit`, `call_static_worker_with_inputs`, `call_declaration_closure_
unit`, `call_declared_unit`, `transfer_unit_result_into_carrier`, `select_
terminal_result_origins`, `emit_result`) and everything already `pub(in
crate::cranelift_backend)` (`D5CloseoutMutation`, `with_d5_closeout_mutation`,
`reset_d5_emitted_declaration_calls`, `d5_emitted_declaration_calls`) carried
across unchanged -- no widening needed, matching the D0 ledger's own note that
these were "already pub(super) from item 12" or wider.

Everything else in the MOVE set stayed at its original visibility (private):
`call_declared_context`, `validate_declaration_unit_call`, `decode_direct_
callee`'s two RETAINED callers turned out to both be usable via widening
already covered above -- no further items needed it.

## The two named `#[cfg(test)]` `thread_local!` block splits (carry
## obligations 1 and 2), executed exactly as scoped

1. `core.rs`'s block (`C2_UNIT_EMISSION_EPOCH`/`RECURSIVE_POSITION_UNIT_
   CALLS`/`SUPPRESS_REQUIRED_CONSUMER_ROUTE`/`REQUIRED_CONSUMER_ROUTE_
   SUPPRESSIONS`) split: `RECURSIVE_POSITION_UNIT_CALLS` moved into its own
   `#[cfg(test)] thread_local! { pub(super) static RECURSIVE_POSITION_UNIT_
   CALLS: .. }` block in `calls.rs`; the other three stayed in `core.rs`'s
   block, re-gated `#[cfg(test)]` exactly as before (the block shrank by one
   static, nothing else changed).
2. `mod.rs`:1967-1997's shared block split: `TRAP_CALLER_PROTOCOL_MUTATION`/
   `D5_CLOSEOUT_MUTATION`/`D5_EMITTED_DECLARATION_CALLS` (plus their doc
   comments, carried verbatim including the pre-existing out-of-order
   doc-comment placement -- not "fixed", since that would be a hunk outside
   the move) moved into their own `#[cfg(test)] thread_local! { .. }` block in
   `calls.rs`; `STATIC_WORKER_MUTATION`/`TRAP_FRAME_BINDING_MUTATION`/
   `TRAP_IDENTITY_MUTATION` stayed in `mod.rs`'s block, re-gated `#[cfg(test)]`
   exactly as before.

Both moved statics stayed **private** in `calls.rs` (their only touch sites --
`with_d5_closeout_mutation`, the D5-closeout accessors, `set_trap_caller_
protocol_mutation`, and `call_declared_unit_target`'s cfg(test) branch -- all
moved into the same file), except `RECURSIVE_POSITION_UNIT_CALLS`, which needs
`pub(super)` per the table above.

## Carry obligation 3: `StaticWorkerCallOutcome`'s `#[cfg(test)]`
## `DeferredPostField` variant -- moot, per the hub-stays correction above

Since the type itself stayed at the `mod.rs` hub rather than moving, the
variant and its two `#[cfg(test)]` match arms in `into_operand`/`into_emitted`
never left `mod.rs` -- carried by NOT moving, which discharges the same
concern the obligation named (a silent narrowing under the test profile) more
directly than carrying it across a move would have.

## Carry obligation 4: `BACKEND_PRODUCTION_SOURCES` roster addition, executed

Added `("lowering/calls.rs", include_str!("../../calls.rs"))` to
`core/tests/control.rs`'s `BACKEND_PRODUCTION_SOURCES` (control.rs:8075-ish),
immediately after the `source.rs` entry, following the documented `boundary.
rs`/`source.rs`/`units.rs`/`seed_material.rs` precedent. The roster's own
self-defending assertion (`the_backend_production_surface_inventory_is_
closed`) required a matching `("lowering/mod.rs", "calls")` entry in its
hardcoded `declared` list (the new `mod calls;` line in `mod.rs` is exactly
the kind of declaration that test parses out of the roster's source text) --
added, and the test passes.

**One further roster-adjacent fix the addition surfaced, not separately
scoped in the kickoff:** `correspondence_adds_no_emitted_unit_to_the_
production_census` (a companion, DIFFERENT census over the same roster,
counting builders/definitions/declarations/data objects per file) failed once
`calls.rs` joined `BACKEND_PRODUCTION_SOURCES` with no row of its own. Added
an explicit all-zero `Census` row for `lowering/calls.rs`, matching `source.
rs`'s own row and its own stated reason verbatim: the moved methods emit IR
into the `FunctionBuilder` their caller already owns; they never mint a new
defined function or data object. This is the same "self-defending census
needs its new row" mechanic the `BACKEND_PRODUCTION_SOURCES` addition itself
already required, just a second, independent roster over the same file list.

## `AC-3` -- line-count transport table

| file | before | after | delta |
|---|---|---|---|
| `lowering/core.rs` | 16,698 | 15,568 | -1,130 |
| `lowering/mod.rs` | 18,854 | 18,067 | -787 |
| `lowering/calls.rs` (new) | -- | 2,054 | +2,054 |
| `lowering/core/tests/control.rs` | 30,289 | 30,314 | +25 (roster + two census rows; a companion-test-move slice, not this one -- see below) |

`control.rs` was already **30,289 lines, well past the 10k ceiling**, before
this slice touched it. `AC-4b`'s ceiling binds files this slice *creates or
enlarges past 10k*; `control.rs` was already past it, and the +25 lines are
the exact same required-housekeeping category (`BACKEND_PRODUCTION_SOURCES` +
census rows) that items 11/12's own `D1`s also added to `control.rs`, not
test-content growth -- `D2` (a separate accepted partial, per this frame's own
rule) is what brings `control.rs` down, not this slice.

## `AC-4`/`AC-4b` -- compiles and tests, scoped only

`scripts/ken-cargo build -p ken-runtime --lib`: clean (0 errors; 61
pre-existing-shape warnings, none newly failing -- see below).
`scripts/ken-cargo build -p ken-runtime --tests`: clean.
`scripts/ken-cargo test -p ken-runtime --lib`: **926 passed, 0 failed, 4
ignored** (925 passed before the census-row fix above; the one failure was
the census gap, now closed). No `--workspace` run anywhere, per `COORDINATION
section 12` -- CI holds the workspace gate.

**One observed, non-blocking lint-warning class**: `mod.rs`'s single shared
facade `use super::planning::{ .. }` block (~50 names, imported once and
re-exported to the whole `lowering` subtree via the glob chain) now shows six
names (`AbiCarrier`, `AbiOwnership`, `ContinuationCallView`, `Continuation
UnitView`, `EffectSeatOperation`, `FusionCompositionLayer`) as `unused_
imports` in `mod.rs`'s own scope, because their sole real consumer moved from
living directly in `mod.rs`'s body to living in a child module's body
(`calls.rs`, reached via the same glob chain that already re-exports these
names to every other child). Confirmed non-regressive: `AbiCarrier`/`Abi
Ownership` genuinely ARE used, in `calls.rs`; this is a known Rust lint
imprecision around glob re-exports crossing a module boundary, not a
correctness issue, and it does not fail any `AC` -- `AC-3`'s diagnostic-
preservation clause is about a MOVED ITEM's own emitted diagnostic text, not
about `cargo build`'s internal lint count on unrelated shared import blocks.
Not fixed here to keep this slice's diff to the ledgered move plus the four
carry obligations plus the one genuine D0 gap; a future slice may narrow this
import block if it becomes a recurring pattern.

## `AC-5` -- adapter/facade debt ledger: EMPTY, no scaffolding introduced

No re-export or compatibility shim was left at the old location for any moved
symbol. The `use calls::{..}` lines in `mod.rs` and `use super::calls::{..}`
in `core.rs` are ordinary cross-sibling-module imports needed for the moved
items' own callers to keep compiling -- not transitional facades standing in
for a not-yet-deleted old path. Nothing for item 18 to delete from this slice.

## `AC-6` -- this slice's own transfer only; phase closure NOT claimed

The calls-and-returns emitter owner has moved. `RT-BACKEND-MODULE-SPLIT`
phase closure is not claimed by this node and is not evidenced by any line
count reported above. `D2` (the companion test move, a separate accepted
partial by default per this frame's own rule) has not been attempted.

## Summary for `runtime-leader`'s object-store verify

Branch: `wp/RT-EMITTER-CALLS-RETURNS-SPLIT`, rebased onto `origin/main`
`83d199f4f` (the D0 merge), current diff: `core.rs`/`mod.rs`/`core/tests/
control.rs` modified, `lowering/calls.rs` new (untracked until this commit).
All four named carry obligations executed exactly as scoped; one genuine D0
gap (`StaticWorkerEmission`/`StaticWorkerCallOutcome`) found and closed by
usage-tracing at execution time per the standing carry, reversing that one
pair's disposition from MOVE to RETAIN/hub-stays with the reasoning recorded
in-line at the code and here. Scoped build and test both green (926/0/4).
`D2` not attempted -- separate accepted partial.

# `D2` LEDGER -- the companion test move, executed against `dccff792a`
# (`origin/main` after `D1` merged). This closes item 13 end-to-end.

Executed per `runtime-leader`'s kickoff (`evt_4964pdf3fcznr`), off the D0
ledger's own AC-2 conclusion (Addendum 13): of the 224 tests individually
read, exactly one is confirmed MOVE, two tentative MOVE candidates were
corrected to RETAIN/shared once their full cluster context was read, and
everything else is RETAIN across `control.rs`'s many independent
domains.

## The one MOVE, re-verified present and unchanged before moving

`d6_a_functionized_recursive_declaration_accepts_a_changing_argument_
constructor` re-confirmed at `control.rs:6671` at the `dccff792a` pickup
SHA -- same line as at D0 pickup, since nothing touched `control.rs`
between the D0 and D1 merges. Read the full 92-line span again before
moving it (discovery-before-mutation): unchanged from the D0-era text.

## Sanity re-read of the "shared/end-to-end" clusters -- NO FLIP FOUND

Per the kickoff's explicit ask, re-read six tests across both clusters the
D0 ledger classified RETAIN/shared with fresh eyes, checking specifically
whether a cluster-context re-read (the mechanism that flipped Addendum 4's
own two D5 tests) would flip anything here:

- **The three `typed_trap_exit_*` tests** (`control.rs:8924-9030`). Re-read
  all three in full. `..._preserves_the_planner_identity_across_two_unit_
  calls` discriminates on planner/occurrence-owned trap identity, holding
  `TrapCallerProtocolMutation::Exact` fixed throughout -- not this item's
  axis. `..._rejects_a_deleted_or_root_misclassified_unit_lane`
  discriminates on `TrapFrameBindingMutation` alone,
  `TrapCallerProtocolMutation::Exact` fixed throughout -- not this item's
  axis either.
  `..._identity_and_caller_protocol_mutations_are_discriminating` is the
  one genuinely exercising `TrapCallerProtocolMutation`
  (`ReadResultBeforeTrap`, `LeaveStaleTrap`) -- but it does so ALONGSIDE
  `TrapIdentityMutation` (`Zero`, `Substitute`) as a second axis, over one
  shared `run_trap_exit_fixture`/`trap_exit_fixture`/
  `TrapExitMutationReset` apparatus that resets all three trap-family
  mutation controls jointly. Its own name states both axes; its own body
  exercises both. **Confirmed RETAIN/shared, no flip** -- this is a genuine
  multi-owner control over the whole generated-unit trap chain, not a
  single-owner test for the caller-protocol mechanism alone.
- **The two `D5` tests** (`control.rs:13628`, `13743`, Addendum 4's own
  correction). Re-read
  `d5_the_checked_call_closeout_rejects_omission_duplication_and_a_
  substituted_callee` in full: its four mutation rows and its positive
  control all key on `D5CloseoutMutation` alone -- on its own, this ONE
  test's rows look single-owner. But its fixture apparatus
  (`d5_mutual_compile`, `d5_mutual_plan_with`, `d5_mutual_template`, the
  `D5_MUTUAL_*` constants) is the SAME apparatus the neighboring `d5_c2_*`/
  `d5_c4_checked_plan_mutations_each_reach_their_own_authority` tests use
  -- Addendum 4 already traced that shared apparatus to
  `enter_checked_recursive_invocation` (checked-invocation, item 12's
  landed domain, RETAIN) as a load-bearing part of what the
  mutual-recursion fixture itself compiles, not an incidental helper.
  **Confirmed RETAIN/shared, no flip** -- the test's ROWS are
  single-owner, but its APPARATUS is not, and the frame's own four-way
  AC-2 partition keys the disposition on the latter for a fixture this
  heavily shared.
- **`governed_nested_brackets_n3_through_n7_emit_complete_functionized_
  bundles`** (`control.rs:9932`). Re-read in full: asserts FIVE joint
  counters across three different owning files in one function --
  `units::b2f_last_unit_emission`/`b2f_last_call_edge_resolution`,
  `recursive_position_unit_calls` (this item's, now `calls.rs`), and
  `d8_join_conversion_counts` -- every `depth` iteration. **Confirmed
  RETAIN/shared, no flip** -- a population/route census over the whole
  `FunctionizedUnits` authority, exactly as the D0 ledger named
  it.

**Net: nothing changed.** The re-read is stated explicitly, not silently
reconfirmed, per the kickoff's own instruction.

## Fixture LCA check

The one moved test's two call-time fixtures, `new_object_module` (aliased
from `crate::cranelift_backend::artifact::new_object_module_for_lowering_
tests`) and `test_only_distinguished_root_join_plan` (from
`crate::cranelift_backend::test_support`), are used **19 and 24 times
respectively** in `control.rs` alone -- far beyond this one test. Per the
established discipline (item 11's precedent: don't widen solely to relocate;
item 12's: the LCA rises only if a second real consumer subtree needs it),
**both stay at their existing declaration site** (`test_support.rs`/
`artifact.rs`, already far above `lowering`'s own subtree).
`compile_expr_into_module` (`core.rs`, `pub(in crate::cranelift_backend)`)
needed no LCA change either -- it stays owned by `core.rs`; only the REACH
changed, from free-by-descent (when `calls.rs`'s test lived inside a
`core`-descendant) to an explicit import (now that `calls` is a sibling of
`core`, not a descendant). None of this widened anything: all three items
were already crate-wide visible within `cranelift_backend` before this
move.

## The move

Moved verbatim into a new `#[cfg(test)] mod tests { .. }` in `calls.rs`
(the item-11/12/13-D1 precedent -- `source.rs`'s own `tests` module is the
closest sibling shape, five small `impl` blocks and an explicit-import
header). `RuntimeLowerabilityStatus`/`RuntimeSymbolMetadata` needed an
explicit import inside the `tests` module too (`core/tests/mod.rs`'s own
AC-8-class-2 precedent: these are test-support types the production facade
never imports, so they were never reachable via `calls.rs`'s own
`use super::*`).

**AC-3 byte-identity**: diffed the moved 92-line span (dedented) against the
`dccff792a` blob at its original position -- **zero diff**. No rewording, no
body change, doc comment carried verbatim including its own promise-class
line.

## AC-2 -- discovery parity and mutation-restoration proof

**Discovery, by exact name** (`cargo test -- --list`, not grep): the test
resolves at its new path,
`cranelift_backend::lowering::calls::tests::
d6_a_functionized_recursive_declaration_accepts_a_changing_argument_
constructor`. `control.rs`'s own `#[test]` count: 221 -> 220 (one test
moved out, confirmed by direct count, no hardcoded `221`/`220` assertion
found anywhere in the corpus to update). Total suite count unchanged: 926
passed both before and after (moving a test changes WHERE it runs, not
whether it's collected).

**Mutation-restoration proof, on the import re-point**: the move re-points
every one of the test's five production dependencies (`compile_expr_into_
module`, `new_object_module`, `test_only_distinguished_root_join_plan`,
`RuntimeLowerabilityStatus`, `RuntimeSymbolMetadata`) through new `use`
paths. Each resolves to a name with exactly one declaration site crate-wide
(verified by grep before writing the import, not assumed), so there is no
shadowing/wrong-item risk the proof needs to rule out -- but ran the proof
anyway, on the production side rather than the import side, since that is
what "reds the same reached property" actually means here: temporarily
forced `call_declared_declaration_unit` (this item's own "SOLE place
[declaration-call] input order is decided", the function
`lower_declaration_ref`'s `SchedulingEntry` arm calls into for this exact
fixture) to return an unconditional `Err`, ran the moved test in isolation
-- **RED**, with the injected error text surfacing exactly in the test's own
panic message, proving the test is genuinely wired to live production code
at its new location, not silently vacuous. Reverted; ran again -- **GREEN**.
Full suite re-confirmed green after the revert (926/0/4), and
`git diff --stat` on `calls.rs` shows only the intended addition, no probe
residue.

## `AC-4`/`AC-4b`

`scripts/ken-cargo build -p ken-runtime --tests`: clean.
`scripts/ken-cargo test -p ken-runtime --lib`: **926 passed, 0 failed, 4
ignored** (same as D1's post-move count -- moving a test does not change
the total). No `--workspace` run.

| file | before (D1 tip) | after | delta |
|---|---|---|---|
| `lowering/calls.rs` | 2,054 | 2,172 | +118 |
| `lowering/core/tests/control.rs` | 30,314 | 30,222 | -92 |

`calls.rs` stays well under the 10k ceiling. `control.rs` remains over 10k
(this was a single-test slice, not a full companion-test-move campaign
sweep across every emitter item) -- the frame's own text never promised D2
alone would bring it under the ceiling, only that this item's own AC-2
population would move; the residual reduction across the whole phase is
`RT-BACKEND-SPLIT-CLOSURE`'s eventual concern, not this node's.

## `AC-5` -- adapter/facade debt ledger: EMPTY

No re-export or compatibility shim introduced for the moved test or its
fixtures. Nothing for item 18 to delete from this deliverable.

## `AC-6` -- this deliverable's own transfer is complete; ITEM 13 CLOSES
## END-TO-END on this node, per the kickoff's own framing

The calls-and-returns emitter's production code (`D1`) and its one
exclusively-owned domain test (`D2`) have both moved. `RT-BACKEND-MODULE-
SPLIT` PHASE closure is still not claimed by this node -- that is the
standing phase record's own concern, separate from any one item closing.

## Summary for `runtime-leader`'s object-store verify

Branch `wp/RT-EMITTER-CALLS-RETURNS-SPLIT`, rebased onto `origin/main`
`dccff792a` (the D1 merge). One test moved, byte-identical, discovered at
its new path, mutation-restoration proof run and reverted cleanly, full
suite green (926/0/4). Six-test sanity re-read of the shared/end-to-end
clusters found no classification flip, stated explicitly. `AC-5` empty.
Item 13 closes end-to-end on this deliverable.

