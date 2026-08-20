---
id: RT-EMITTER-CALLS-RETURNS-SPLIT
title: "Move the calls and returns emitter family out of the lowering files -- the first emitter slice, moving against the stable unit and call vocabulary item 4 established"
status: ready
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

