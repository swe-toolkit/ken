---
id: RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT
title: "Native lowering has no representation for a relay Vis (a K that forwards a pattern-bound operation, or a functional-IH reference) returned to a response owner beside a non-relay member, so such an owner is emitted Ret-only and traps -1 (or is unplanned), and the ignored rt_span_prov_native:355 and r2 (rt_escape_second_resource_native:1015) rows stay ignored. Refuse at planning, on a discriminant read from the settling plane, every owner native lowering cannot settle, and give the relay a planned native arm that settles it at interpreter parity"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [RT-NESTED-RESPONSE-OWNER-CALLER]
blocks: []
github: null
origin: "Supersedes RT-SOURCE-IH-RELAY-K-VALUE (r2), on Architect evt_59xpt6t02wdpq. Steward scope decision evt_2kmfzsvqj53zg on D1h outcome H2 (evt_6vzphntgv506e, evidence evt_nzs10vn3w8t0); Architect ruling evt_14w6eh1d2hwk4 accepted the split, made this one node and pre-ruled its D0 fork. Serves the L1 objective (operator 2026-09-17). Steward-filed per COORDINATION section 2."
---

# A relay Vis settles natively

## Objective

`sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`
(`crates/ken-cli/tests/rt_span_prov_native.rs:355`) and
`r2_cross_buffer_freeze_fails_closed_with_invalid_bounds`
(`crates/ken-cli/tests/rt_escape_second_resource_native.rs:1015`) are
un-ignored and pass on both engines.

## Settled inputs (D1h `evt_nzs10vn3w8t0`, at `b135b25b5`)

- **The reference semantics.** After owner 4's BufferAllocate (Vis 1496),
  its K, Context(1)/funcid66, returns relay Vis 361, which forwards the
  pattern-bound operation `Var(1)` (FsReadAt). The interpreter driver
  dispatches FsReadAt and resumes the IH. Context(1) later reaches
  installed Vis 528 (ResourceRelease of FsHandle) by row 5 / Context(6),
  and the run exits normally.
- **Context(1)'s census.** Two members: relay 361, with no successor, and
  installed 528 (row 5, Context(6) = funcid71/body515, which is empty).
- **Why the owner is excluded today.** The whole-owner relay exclusion
  (`returned_vis.rs:326`) is keyed on the relay IH K having no native
  value or environment representation.
- **Member shape does not separate them** (Architect `evt_ak61svpj64m6`).
  ABI-S6 owner 0 (`abi_s6_mapping_surface_native`, 6 rows) is a mixed
  relay owner and runs green at baseline. Span owner 4 and r2 owners 2 and
  3 are mixed and trap. Non-relay members are settled natively by the
  owner loop (`units.rs:3256`) or the handler-owned drive (`core.rs`
  ~6918-6950, gated on `deferred_response_at_vis` and
  `bounded_deferred_response_handler_owner`).
- **The seat plane has no relay seat.** `build_host_effect_seat_plan`
  (`effects.rs:658`) iterates static `RuntimeExpr::Effect` occurrences, so
  a forwarded operation has no seat of its own.
- **The predecessor's state.** RT-NESTED-RESPONSE-OWNER-CALLER stores each
  owner's `ResponseOwnerSettlement` (`Protocol`, `RetOnly`,
  `Excluded{Relay|Underived}`) with no planning refusal: `Excluded` owners
  keep the baseline Ret-only arm, and every native row keeps its
  `c7c05d4e6` result. Its handoff carries a per-member census (relay,
  installed, successor, drive coverage) for 9 measured suites; the rest is
  this WP's D0.

- **The r2 population** (from `RT-SOURCE-IH-RELAY-K-VALUE`, Architect
  `evt_1j5qaw2d9sqe7`). r2's owners 2 and 3 are `Excluded{Relay}`
  and mixed. The relay's K operand is `CheckedComputationalIHInvocation {
  body: Call { callee: Var(0), args: [] } }`, which lowers to
  `Specialized(RecursiveBackedge)` because `ConstructArgument`
  (`source.rs:1643-1655`) propagates a backedge past the constructor, so
  no K value exists to carry. IH marker arms: `core.rs:3574`,
  `core.rs:15382`, `source.rs:829`. That propagation is lawful
  (`RT-IH-BACKEDGE-FAIL-CLOSED`, `evt_6x4nk9x3pe0r`), so r2 is a
  representation question only.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect design; measurement only).** Done: handoff
   `evt_5gtgjrrppexvw`, ruled `evt_3hcpdavtb0r50`. Provenance (i) is
   closed and finite for all four owners. The text below is the D0 as
   framed. First, complete the
   per-member census over the native suites the predecessor did not
   measure (its 9 measured suites are in
   `local/rt-nested-response-owner-caller/census-a/`; Architect
   `evt_1kq6h67jzp7ae`). Then measure:
   - (i) the closed provenance set of static `Effect`/Vis occurrences
     whose operation can flow into each relay's `Var`;
   - (ii) the relay IH K's capture and environment layout at the return,
     against the existing pending-Vis frame region;
   - (iii) from the predecessor's census, for each `Excluded` owner,
     whether the handler-owned drive settles it: whether
     `deferred_response_at_vis` and `bounded_deferred_response_handler_owner`
     cover every forwarded Vis's construct site;

   - (iv) **admission E is live** (Adversary `evt_4kf7fa50c58ph`, at
     `2260b91cc`). `ContinuationBodyTail` (`responses.rs:3304-3319`) is
     keyed on the producer construct being `worker_body_origin()`. A Vis
     in a Bool `match` arm tail inside the K body misses it, is classified
     `UnconsumedTransportCaller` with a handler owner, and is refused at
     `RelocatedWorkMissingLoweringBinding` (`selected_pending_calls.rs:769`)
     while the interpreter exits 0 with `captured\nsecond\n`. The witness
     is the two-print fixture in `rt_selected_pending_call_admission.rs`
     with its continuation replaced by `(\_. match terminal { False |->
     host_console APartial Unit (print_line "second") ; True |->
     host_console APartial Unit (print_line "third") })`. Pin it, census
     the arm-tail shape in both populations, and do not delete E. **Ruled
     out of this WP** (`evt_5sah7xb9543hp`): E is an environment-binding
     gap in the handler-owned path (`effect_free {0,1}` against
     `effect_fields 1` at `selected_pending_calls.rs:745`), not a CBT
     classification. It stays a typed planning refusal and goes to
     `RT-PENDING-CALL-ERR-PAYLOAD-ADMISSION`;

   over both populations: the span's owner 4 and r2's owners 2 and 3. If
   one design cannot settle both, the D0 says so and the Architect rules
   whether to split.

   From (iii) the Architect rules the **planning-refusal discriminant**
   (`evt_ak61svpj64m6`). It is read from the settling plane, separates span
   owner 4 and r2 owners 2 and 3 (which trap) from ABI-S6 owner 0
   (settled), and is checked against every `Excluded` owner in the census
   before any refusal is written. If no discriminant on the planned data
   separates them, **stop** to the Steward.

   Pre-ruled (Architect `evt_14w6eh1d2hwk4`): if (i) is closed and
   finite, the relay arm dispatches over the planned seats of exactly
   that set, and the node proceeds. If (i) is open, **stop**: a precursor
   node for dynamic host-effect dispatch comes first.
2. **D1 first checkpoint and census before enable** (`evt_3hcpdavtb0r50`).
   - **Emission owner.** Before any arm, measure that the K-context bodies
     of span owner 4, r2 owners 2 and 3, and px8f owner 1 Context0 lower
     with `defining_emission_owner == O.base_owner()` (px8f owner 1:
     Spec(2)).
   - **Record-only recorder.** The first commit is a cfg'd recorder,
     emitting for each `Excluded` owner and each source F `closed`,
     `drive_settled` and `relay_drivable` (a)-(d), each separately, and
     the verdict, plus, each separately, the redex body,
     `deferred_no_unit_response_in_body(redex)` and
     `admitted_deferred_handler_owner` (`evt_12az6njpj0wyy`). It records
     nothing about `ContinuationBodyTail`.
   - **Population** (`evt_50wnhmgt00rfc`, `evt_5sah7xb9543hp`). The 38
     complete suites, run locally. The eight already run stand (12
     `Excluded` owners, all admitted). For `rt_parity_native`, run its 20
     named `checked_ih_*` and `composed_return_forward_ret_authority_*`
     tests from the D0 owner census locally; CI owns the rest of that
     suite, and the handoff says so. **Under (R3) the walk's semantics
     changed, so no earlier census record stands** (`evt_5mgw5cwr993nd`):
     re-run all 38 complete suites and the 20 named parity tests, except
     the four rows the Architect measured under R3
     (`rt_cold_lowering_path_enumeration` 14/14, the cap41 parity test,
     `px8f_buffer_native` 6/6, `abi_s6_mapping_surface_native` 12/12).
     Record omissions per owner.
   - **Expected refusals** (`evt_62wab4peq1htm`). px8ta owner 0 (base
     Spec(0), relay Vis370; P1 sources 647/600, admitted handler None) is
     an expected refusal: native lowering already fails that program at
     `ObjectEmission`. A green negative control whose expected first
     refusal would change is recorded and added to the switch-wrapped set
     below. It is a stop only if the control's program is admitted on the
     unsuppressed path.
   - **Enable gate.** The refusal is not enabled until the census clears
     the stops below.
3. **The ruled planning refusal**, at `planner.finish(`'s single caller,
   after the cfg'd recorders. As amended by `evt_12az6njpj0wyy`, whose
   code is the reference:
   - **(R1) Provenance passes through a curried closure call.** In
     `source_for_relay`, a `Call` terminal of shape `Call^n(LexicalClosure)`
     over n one-parameter closures (`curried_redex_body`) contributes the
     innermost body as a further scope, remembered as that scope's redex
     body. Any other `Call` (generated or unknown endpoint) is open, so
     the owner is refused. A revisited relay contributes nothing (least
     fixpoint). Path condition (c) is still measured by `tail_to`.
   - **(R2) `drive_settled(F, O, redex)` reads both lowering gates,** with
     no `_ =>` arm, over `deferred_response_at_vis(F) == Some(row)`:
     - `UnconsumedTransportCaller`: the construct-site gate (`core.rs:6926`),
       `bounded_deferred_response_handler_owner(&row) ==
       Some(O.base_owner())`;
     - `NoContinuationUnit`: the retained-call gate (`core.rs:6217-6222`),
       with a redex body, `deferred_no_unit_response_in_body(body) ==
       Some(row)` and `admitted_deferred_handler_owner(&row) ==
       Some(O.base_owner())`; false without a redex;
     - `ContinuationBodyTail`: false.
   - **(R3) `closed(R)` answers "can a Vis arrive" exhaustively**
     (`evt_5mgw5cwr993nd`, whose code is the reference):
     - (a) `source_result_origins_with_omissions` in `closure.rs` returns
       the result positions and every position omitted because its owner
       is another function unit. `source_result_origins_in_owner_subtree`
       keeps byte-identical behaviour by delegating to it.
     - (b) the relay source walk (`source_results_in_scope`) errs on any
       omitted position;
     - (c) `source_for_relay` returns an exhaustively classified empty set
       as `Ok` (the relay's arm is unreachable and contributes no source),
       not as an error that aborts the parent relay's whole source set;
     - (d) a relay is closed iff its trace is `Ok`. An owner whose relays
       are all closed with zero live sources is admitted; `relay_count ==
       0` still does not admit.
   - **One handler authority.** `admitted_deferred_handler_owner` moves
     unchanged from lowering (`core.rs:5800-5819`) into the plan, so the
     recorder, the refusal and the lowering read one function. Lowering
     keeps its planner-invariant mapping of the disagreement error.
   - an `Excluded{Relay}` owner is admitted when every relay member is
     closed and every source F is `drive_settled` or `relay_drivable`;
   - `Excluded{Underived}` and non-admitted `Excluded{Relay}` owners are
     refused.
   - **Test-only suppression switch** (`evt_62wab4peq1htm`, whose code is
     the reference). `with_relay_settlement_refusal_suppressed` in
     `planning/static_transition.rs`, feature-gated beside the D1
     recorder and exported next to `with_px8ds_retired_flat_order`
     (`cranelift_backend.rs:110`, `lib.rs`). Under it the refusal is
     recorded but not raised, so a lowering-time control still reaches its
     own guard. The ordinary build never selects it. The retired-flat
     control in `crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs`
     (a new path) wraps its build in both switches, and a sibling
     assertion without suppression expects the exact typed relay refusal
     naming owner 0 / relay 370, not the closure text.
   - **D3 and D4 are one atomic merge candidate** (`evt_2e92pc40cyrbc`).
     D3's admission is a prospective predicate over the state once D4
     lands: at the D3 checkpoint `d7820636f`, 105 of the census's 106
     Excluded owners with sources (in 32 green tests) are admitted only
     through (d). So D3 delivers its refusal promise only with D4, and is
     never a merge candidate alone. D4 builds on `d7820636f`, and they take
     one QA, one Architect review and one Decision. Do not add `&&
     d_gate_installed` to admission: it would refuse those 105 owners.
4. **Relay admission: settled by static fusion, never carried.**
   - `relay_drivable(F, R, O)` holds when all four hold:
     - (a) F has exactly one installed or candidate response row with a
       planned seat;
     - (b) F's operation is a `Construct(coproduct, [Construct(..)])` and
       its K is a one-parameter `LexicalClosure`;
     - (c) the walk from F to R crosses only `response_tail_edge` edges
       and `ComputationalMatch` scrutinees whose single Vis case
       satisfies `response_forwards_vis`, with no generated Call edge (a
       curried redex under (R1) is not one);
     - (d) the key is the pair (F, `O.base_owner()`), never F alone,
       read through one plan-side authority (`evt_2e92pc40cyrbc`):
       `StaticTransitionPlan::relay_drive_pair(source, owner) ->
       Result<Option<RelayDrivePair>, _>`, the single admission
       authority. The pair establishes a uniquely seated candidate, not
       a settling route or execution: static fusion consumes the
       forwarded relay at the source, and a relay that still reaches the
       owner at runtime takes the owner loop's unknown-member exit, which
       returns `-1` (`evt_4c45bzrtqevcx`). `classify` admits on (d) iff
       it returns `Ok(Some(_))`, pushing an `Err` to the owner's errors. The D3
       constants `d_pair_proposed = true` and `d_gate_installed = false`
       in `relay_settlement.rs` are deleted. A (d) axis kept as a
       constant is a block.
   - A relay construct of an admitted owner that is still reached is a
     typed planner-invariant error, never `-1`.
   - Admitted relay members leave O's returned set: span owner 4 becomes
     {528} Protocol, r2 owner 2 {746}, and r2 owner 3 {528}.

5. **D4': no drive (`evt_5hcnh155zmm5x`).** Gate 2 measured G1: with
   the drive dormant, span, r2 and `abi_s6` are green and no relay Vis
   materializes at runtime. Each relay source is lowered under its
   enclosing Computational eliminator. Its seat is emitted generically
   by several functions, so no single route performs it
   (`evt_4c45bzrtqevcx` R4). The returned-member projection is what fixed
   the trapping owners.
   - **Delete:** the relay drive and every remnant
     (`call_relay_source_token`, `relay_pair_for_source_token(_at_owner)`,
     the inline-skip extension, `call_declared_unit_target_with_frame`,
     the factored settle arm if it has no other caller, the verifier
     relay branch and its closure additions); the
     `perform_response_at_source` split, folded back into one
     `drive_response_at_source` for the Deferred route only; the
     claim-seam WIP.
   - **Retain:** D1-D3; the returned-member projection (`864c43c78`,
     `8a3f70f3f`); `relay_drive_pair` as the admission authority; the
     permanent origin, shape and claim-identity errors.
   - **Deletion controls, measured.** For each of source (f)
     (`34dd31394`), `source_seat_row` (`83ce24c9e`) and the
     forwarded-position guard (`ff7e6dc6d`, `dc1830897`), delete only
     that piece on top of D4' and run span, r2, `abi_s6` and row 32 once.
     A red retains the piece and names its consumer. With no red, (f) is
     deleted; the guard stays only if its own discriminator fixture
     reaches it without the drive, otherwise it goes with its fixture;
     `source_seat_row` stays only while admission (a) calls it. Then stop
     to the Architect with the per-piece table.
   - **Closed as falsified (`evt_4dv4h0199dsnh`, advisory
     `evt_642vbsakdwwr`).** Static fusion settles the relay's control,
     not its effect: the forwarded operation is dropped (entry 9). There
     is no D3+D4' candidate. The fall-through survives only as the
     lowering of a pair admitted with a route, where the call performs
     the operation before the continuation runs.

6. **Seat-keyed planned trap (`evt_4dv4h0199dsnh`, re-keyed
   `evt_3y7fxd70stq3x`, ruled `evt_2228y4p1zwgkg`).** The drop is a
   property of the lowering, not of admission: main, with no relay
   admission logic, and D3, which admits 707/768, both drop the read.
   - **Plan fact.** The plan records, per admitted pair,
     `relay_route: Option<member>`: the emitted response-owner member
     that handles the forwarded op, or none. Seats read this fact and
     never discover it at lowering time.
   - **Planned trap.** For each pair with route none, the planner
     registers a planned `RuntimeTrap` with code
     `RuntimeTrapCode::MissingRuntimeMetadata` and the message
     `"relay: forwarded host op <op> from source <origin> has no
     performing route"`. No new ABI variant. Lowering resolves it through
     `static_transition_plan.trap_identity(trap)?`.
   - **At each consuming seat** (`claim_and_call_continuation`,
     `active_transport`, `dispatch_fusion_owned_outer_realization`), a
     route-none pair emits the trap, returns its status and records an
     `Abort` frame terminal, the `seal_source_trap_branch` sequence
     (`joins.rs:2802-2816`), and never consumes silently. Runs that never
     reach the source are unchanged.
   - **Not a compile-time refusal**: that would redden 15 rows of tests
     whose measured runs are correct, and the successor would turn them
     back.
   - **D0, measurement only.** For every relay-admitted pair in the
     36-row census population plus in-bounds `BRANCHED_SCRUTINEE`:
     (a) the forwarded operation; (b) the emitted response-owner member
     that handles it (function, owner, member id) or none; (c) per
     executed run, whether the forwarded op appears in the native and
     interpreter traces, attributed by the recorder at the source; (d)
     done: main `7a890f13d` and D3 `d7820636f` both run the variant and
     drop the FsReadAt; (e) the fan-in of consuming seats: every
     lowering function that can consume a Vis-constructor producer
     without emitting its operation (the two known paths at
     `lowering/core.rs:7581-7648`, `dispatch_fusion_owned_outer_realization`
     and `claim_and_call_continuation`, plus every other consumer of
     `selected_computational` and fused-realization dispatch), and the
     seat that consumes source 768, from a disposable `eprintln!` at
     each listed seat. A consumption at an unlisted seat, or an
     ambiguous (c) attribution, is a stop to the Architect. The table
     goes to the Architect.
   - **Seats after rows 01-24 (WIP audit `evt_4xj2fgkjmwzhx`).**
     `active_transport` consumes without emitting and joins the gate's
     seat list. `claimed_result_substitution` is the tail of
     `claim_and_call_continuation`, one seat.
     `dispatch_fusion_owned_outer_realization` is not observed. Source
     768 is not yet attributed, because the recorder is owner-filtered and
     sits in one function. It is settled first by unfiltered prints at
     `constructor_enter`, `lower_computational_match_value_composed_once`
     and `lower_computational_producer_expr_once`, keyed on the fixture's
     roster entry. A visit under another owner or `None`, or no visit at
     any of the three, is a stop to the Architect; no visit falsifies the
     seat-keyed design. Rows 25-36 follow, with pairs whose emitted
     handler is none reported separately.
   - **Owed before the candidate** (`evt_2228y4p1zwgkg` item 5,
     `evt_3qn9d87abajsc`). The unfiltered probe on the 16 pending
     pair-builds (rows 33, 35, 36): another owner or `None`, or a seat
     outside the three, is a stop; `not lowered` is recorded. Fill pair
     707 as "op names differ only by the 768 drop; the release identity
     order flips independently". For each executed row 01-36 and both
     BRANCHED variants, whether full EffectEvent traces (operation,
     resource bindings, order) agree, citing the test's own assert where
     it has one; and out-of-bounds BRANCHED full traces on current main.
   - **The release-order flip is not this increment.** Native releases
     FsHandle before Buffer where the interpreter releases LIFO, already
     without the drop (`ac7-extra-branched.log`). Increment (i) neither
     causes nor fixes it and must not claim to. The Steward decides its
     scope from the full-trace measurement; the Architect rules its
     design after it.
   - **This WP's landing cut is increment (i):** D1-D3, `relay_drive_pair`
     and the errors, the plan fact `relay_route`, and the planned
     no-route trap at the three seats, with AC-MAIN, its mutant and the
     four controls. The D0 census instruments are removed from the
     candidate (saved as `D0-disposable-recorder-unfiltered.patch`). It
     closes the main defect. **Increment (ii), route (C)**, emitting an
     outer member that performs the forwarded op and retiring the traps,
     is a successor node, framed from the D0 table; one that restructures
     the planner's build order returns to the operator to size.
   - **Candidate gate.** No seat consumes a host-operation Vis without a
     recorded route or the planned trap. A census row whose admitted
     pair's forwarded op appears in neither trace is `vacuous for
     performance` and is not coverage.
   - **The defect on main (`evt_3s45p56abgg27`).** Main `7a890f13d`
     builds in-bounds `BRANCHED_SCRUTINEE` natively and runs it to exit
     72 while dropping the forwarded FsReadAt the interpreter performs.
     Exit-code parity cannot see it, because the program discards the
     read's outcome. This WP owns closing it; it predates D4'.

## Acceptance

- **AC-1.** The span row is un-ignored and green: both engines exit 0 with
  the exact freeze sequence it asserts.
- **AC-1b.** r2 (`:1015`) is un-ignored and green: the full native and
  interpreter operation sequences are equal, and so are the terminal
  classes. The three r2
  compile-or-inspect rows (:861, :913, :970) pin the admitted plan.
- **AC-5 (mutation, QA).** Deleting source (e) yields the ObjectEmission
  refusal (owner 0, context 2) on the span row. This is carried from the
  predecessor, where lowering is not reached.
- **AC-R3.** As listed in the recut `evt_2091hd1wtkhs5`; moved here by
  `evt_14w6eh1d2hwk4`.
- **AC-2 (control).** An `Excluded{Underived}` owner, and a relay whose
  provenance set is not planned, are refused at planning with the exact
  typed text. px8ta owner 0 (relay 370) is the second refusal witness, with
  the exact typed text. ABI-S6 is 12/12.
- **AC-3 (mutation, QA).** Removing the returned-member projection
  (relay members restored to the owner's returned set) returns the span
  row and r2 to the planning refusal, never to `-1`.
- **AC-6 (mutation, QA).** Classifying ABI-S6 owner 0 as refused reddens
  its six rows at the refusal, so the discriminant is load-bearing.
- **AC-4 (controls).** SEQUENTIAL, `one_bracket_retains_native_parity`,
  the five sibling rows and the predecessor's AC-R6 census population stay
  green; px8ta keeps its labelled failures. `px8f_buffer_native` is 6/6
  after enable, because admission changes px8f owner 1's lowering.
  `px8ds_retired_flat_order_does_not_gain_m4_representation` is green with
  both assertions, and the switch is restored after its scope.

- **AC-7 (the D4 population, `evt_2e92pc40cyrbc`).** The 32 tests that
  carry the 105 owners whose admission D4 changes run individually
  through `scripts/ken-cargo`. So do
  px8ta and the two un-ignored target rows, each with the recorder. The
  32 include the 20 `checked_ih_*` and `composed_return_*` parity rows,
  `rt_escape_second_resource_native`, `rt_span_prov_native`,
  `rt_branched_scrutinee_unit_body_port`, `rt_capture_projection_grow`,
  `rt_cold_lowering_checked_family_enumeration`,
  `rt_exactint_carried_observe` and `rt_resource_release_carried_observe`;
  the census file `census-r3/owner-source-rows.tsv` names them. Every row
  is green, every owner is admitted through `relay_drive_pair = Some`, and
  px8ta owner 0 is still the only typed refusal. CI owns everything else.
  - The Architect's four R3 rows carry 35 more owners whose emission D4
    changes, and run the same way (`evt_6r8n32qy4h9es`):
    `rt_cold_lowering_path_enumeration` (10 owners), the
    `fs_read_at_out_of_range_invalid_bounds_rejects_read_eof_witness`
    parity row (2), `px8f_buffer_native` (11; owner 1 source 1169 is
    drive-settled through P1 and gets no pair) and
    `abi_s6_mapping_surface_native` (12).
  - abi_s6 stays 12/12, every abi_s6 source logs `drive_settled=true`, and
    its suppressed-local-drive control keeps its result: the Deferred
    route keeps precedence through `deferred_drive_available`, so its
    installed pairs go unused.
  - The roster file the run uses lists all 36 tests plus px8ta and the two
    target rows, with its SHA-256 recorded.
  - **Runtime census (`evt_4c45bzrtqevcx`).** Admission is a
    completeness claim, measured at runtime, not by which function emits
    a seat. With disposable native counters (stderr markers allowed), for
    each admitted pair in each build record: (i) executions of the source
    at its fall-through point in its context function; (ii) exits through
    the pair owner's unknown-member `-1` block; (iii) the native and
    interpreter op traces and the rc. Run one row at a time from row 01,
    and stop to the Architect at the first red, any (ii) > 0 (report it
    even when the native rc is 0 and the traces agree: a silent drop), or
    any trace difference. After all rows, the sum of (i) over every
    admitted pair must be above 0; a sum of 0 is a stop, because the gate
    never executed a relay source.
- **AC-MAIN (`evt_3s45p56abgg27`, `evt_2228y4p1zwgkg`,
  `evt_45f92kn90453r`, `evt_3qn9d87abajsc`).** A permanent test,
  `branched_scrutinee_forwarded_read_is_performed_or_refused` in
  `crates/ken-cli/tests/rt_branched_scrutinee_unit_body_port.rs`, runs
  the in-bounds variant. Native stops with exactly the planned relay trap
  identity; its full EffectEvent trace before the trap (operation,
  resource bindings, order) equals the interpreter's prefix before
  `FsReadAt`; `FsReadAt` does not appear natively. Host ops after the
  trap on the abort path are measured and recorded in the handoff, not
  pinned. Main (rc 72, no `FsReadAt`) cannot pass it. Increment (ii)
  turns it into equal full traces with `FsReadAt` performed.
  - **Mutant (QA).** Deleting the trap emission returns the run to rc 72
    with no `FsReadAt`, and AC-MAIN reddens.
  - **Controls.** The four executed green tests whose source is not
    reached (row 01 builds 0 and 2, rows 14 and 17) keep identical full
    traces before and after the change.
- **AC-8 (mutation, QA).** `relay_drive_pair` returning `None` for one
  census pair (span owner 4, source 1374) fires the typed refusal for that
  owner and reddens AC-1, and is restored byte-identically.

## Stop conditions

- D0 (i) is open, or no planning-refusal discriminant separates the
  trapping owners from ABI-S6 owner 0.
- D1's emission-owner checkpoint measures an owner other than
  `O.base_owner()`: stop with the measured owner, and do not substitute
  another key. The same holds for any owner admitted only through (R2).
- `deferred_no_unit_response_in_body(1176)` is not px8f row 1169, so px8f
  owner 1 stays refused.
- The census shows a refused owner in a green execution row or a positive
  build row (its parity twin may execute), or an omission that refuses an
  owner in a green execution row: stop to the Architect, naming the owner
  and row. px8ta owner 0 flipping to admitted is also a stop. In the CI-owned
  `rt_parity_native` tail, the same refusal is a stop once the refusal is
  enabled.
- `ContinuationBodyTail`, admission E or `selected_pending_calls.rs`
  needs a change.
- The repair touches the bracket tree, moves a held ref
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, `wp/RT-BRACKET-SETTLEMENT-PLANE`,
  `4b4c8565c`, `21c039918`, `7f1a04a40`), or relaxes the `units.rs:7831`
  coverage gate or the `0x101d_0000_002a` guard.
- Any kernel, trust or spec change.
- An owner where `relay_drive_pair` returns `None` or `Err` while the R3
  census had (a)-(c) true, or any AC-7 row going red: stop to the
  Architect.

## Hard-stop inventory

- **§1a.** Question (4), E disposition and the D4 relay arm: 6
  (`evt_5sah7xb9543hp`, `evt_47yp5mk4wznjb`, `evt_3b7ws2dvx1r9v`,
  `evt_4tbaxstk921c2`, `evt_1pfhbxjg0f4he`, `evt_15tdpczht4e6g`). The
  third put D4 on hold and called a research advisory; the sixth holds
  the D4' ruling for a second advisory (`evt_5qh2rwdb1zhm8`), answered by
  the performing-route recut (`evt_4dv4h0199dsnh`, re-keyed to the
  consuming seat `evt_3y7fxd70stq3x`). The next research
  re-trigger is the ninth; the next §1b is at entry 12.
  Question (2), the refusal discriminant: 3 (`evt_12az6njpj0wyy`,
  `evt_62wab4peq1htm`, `evt_2spcf0ad9mmet`). The third was resolved with
  research's advisory (`evt_4wcce8z24a4c9`, ruling `evt_5mgw5cwr993nd`);
  the next research re-trigger is the sixth.
  Questions (1) and (3): 0.
- **1.** CBT widening by tail walk would strip handler ownership from
  handler-owned P2 rows (ABI-S6 Vis548/549; witness Vis342/364), keyed on
  producer-construct position instead of handler ownership.
- **2.** Relay closure and `drive_settled` were each keyed on one
  settlement route (an owner-local subtree that stops at a curried closure
  call; the bounded P2 handler only), not on the fan-in of lowering gates
  that settle a source (curried call → `call_declared_unit` → unit-less
  drive). The §1b predicate question fires at entry 3.
- **3.** The planning refusal pre-empts a lowering-time negative control
  (px8ds retired-flat). It is worked around by a test-only suppression
  switch, keyed on refusal order (planning before lowering).
- **§1b at entry 3** (`evt_62wab4peq1htm`). Entries 1 and 2 share a
  predicate: a planning classification re-derived a lowering decision from
  a different plane than the gate lowering reads. The closure is that
  admission calls the lowering gates' own functions, as R2 does. Any
  further drive or handler predicate must call the gate's own function,
  not re-derive it. Entry 3 is independent.
- **4.** closed(R) read an exhaustively classified empty source set
  (scrutinee yields only Ret) as open and let one dead upstream relay abort
  its parent's whole source set; the walk also silently omitted result
  positions owned by another function unit — keyed on "a Construct-Vis
  exists" rather than "can a Vis arrive at the Ret-only arm". §1b next
  fires at entry 6.
- **5.** The per-position aggregate identity check in
  `reconcile_source_aggregate` refused a forwarded value: parent
  Source(583), position 0, is Var 582, carrying occurrence 179 produced at
  760 in fn 12, under drive stack [1374, 744] and owner Spec(7) -- keyed
  on "the position has a source origin" rather than "the position is a
  producer". The guard repair stands (`evt_5zzxh9zn7f18s`).
- **6.** Under the relay drive, a continuation claim
  (`claim_and_call_resolved_continuation_inner`) finds no declared target
  in the driving owner's function -- keyed on the call site's syntactic
  enclosing owner rather than the owner that emits it. Measured
  (`evt_21g7mkxe4v43a`): the token for producer construct 1200 has
  target Spec(0) and emission owner Predeclared(10), and was claimed
  under defining owner Spec(7) with drive stack [1374, 1327]; 1200 lies
  in the K of Vis1327, not in Spec(7)'s own body.
- **§1b at entry 6: YES** (`evt_3b7ws2dvx1r9v`). Entries 4, 5 and 6 share
  a predicate, extending entry 3's: the relay drive is an emission route
  that lowering enters but the plan does not record.
  `drive_response_at_source` lowers the driven source's operation, its K
  and the frames it resumes under `defining_emission_owner`, while every
  owner-keyed plane is built before `relay_drive_pairs` exists and from an
  emission-owner authority with no relay source. The closure is
  structural: every emission route lowering enters is an emission
  population the plan knows before any owner-keyed plane is built, or the
  drive does not relocate emission. Ruled closure (`evt_7r6p7j6c5408j`):
  (C), no relocation. A pre-schema drive decision is rejected: the pair
  builder reads ownership-derived planes, and path-keyed clones would
  re-enter the RT-NATIVE-FNSPLIT predicate.
- **7.** The relay drive was placed at the source token's claim seam in
  the pair owner's function, a claim lowering never makes there: the
  token is declared there as transported, the source falls through to
  the eliminator, and its seat is performed by the installed response
  owners (`evt_4tbaxstk921c2`) -- keyed on the plan pair's existence, not
  on the route lowering takes. Entries 5, 6 and 7 share the entry 6 §1b
  cause, which recut (C) did not close. D4' (`evt_5hcnh155zmm5x`) is
  their structural closure: the drive is removed, not re-placed. §1b
  next fires at entry 9.
- **8.** Relay admission (d) admits pairs whose candidate row has no
  installed owner (AC-7 row 01, `fs-read-at-offset-single`: 735/Spec2
  row 3 seat 190 and 846/Spec2 row 1 seat 174, row still green); D4'
  named the installed owner as the settling route without measuring
  that population -- keyed on the plan's row existence, not on the
  route that performs the seat (`evt_1pfhbxjg0f4he`). The measurement
  (`evt_4c45bzrtqevcx`, non-advancing) found no single route: owner 0
  runs for member 515 and seat 174 never executes. AC-7's census is recut
  to runtime behavior.
- **9.** Static fusion at the relay source settles the relay's control
  (the fall-through to `continuation_result`) but not its effect. In
  `BRANCHED_SCRUTINEE` with the window in bounds, source 768 executes
  once natively and its forwarded FsReadAt is never performed, while the
  interpreter performs it (`evt_15tdpczht4e6g`) -- keyed on "the source
  fuses statically", not on "a route performs the forwarded operation".
- **§1b at entry 9: YES** (`evt_5qh2rwdb1zhm8`). Entries 7, 8 and 9
  share one predicate: relay settlement is attributed to a plan-structural
  fact (the pair, a candidate row, static fusion) rather than to a route
  that performs the forwarded operation. The closure, cut after the
  advisory: an admitted relay pair names a route that performs the
  forwarded operation, measured as host-op parity on a fixture where the
  interpreter performs it; without one, relay sources keep D3's typed
  refusal. D1-D3, the projection, `relay_drive_pair` as admission
  authority, the permanent errors and the census instruments carry
  forward; D4''s claim that an undriven relay is settled does not.

## Finding outside this WP

`px8ds_real_same_depth_path_runs_exact_edges`
(`px8ta_oriented_subcontinuation.rs:452`) is red at base: `ObjectEmission`,
"a specialized response owner did not materialize an exact HostResult", for
the same owner 0, while the interpreter exits 0. Its exemption in
`.github/ignored-test-exemptions.toml` classes it `policy-cost` and calls it
runnable. This WP leaves it ignored, and its failure becomes the typed
refusal. Its successor gives these P1 sources a handler, un-ignores the row
and corrects the exemption.
