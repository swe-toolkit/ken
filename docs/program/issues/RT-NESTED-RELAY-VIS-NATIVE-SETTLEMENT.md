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
     the arm-tail shape in both populations, and do not delete E. The
     Architect rules whether the relay arm or a widened tail predicate
     settles it;

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
     of span owner 4 and r2 owners 2 and 3 lower with
     `defining_emission_owner == O.base_owner()`.
   - **Record-only recorder.** The first commit is a cfg'd recorder,
     emitting for each `Excluded` owner and each source F:
     - `closed`, `drive_settled` and `relay_drivable` (a)-(d), each
       separately, and the verdict;
     - each row that would move from `UnconsumedTransportCaller` to
       `ContinuationBodyTail`, with its handler owner.
   - **Population.** Run the recorder over the 38 complete suites. For
     `rt_parity_native`, either run one test per invocation or name CI as
     the check, and say which.
   - **Enable gate.** Neither the refusal nor the widening is enabled
     until the census clears the stops below.
3. **The ruled planning refusal**, at `planner.finish(`'s single caller,
   after the cfg'd recorders:
   - `drive_settled(F, O)` is `deferred_response_at_vis(F) == Some(row)`
     and `bounded_deferred_response_handler_owner(&row) ==
     Some(O.base_owner())`;
   - an `Excluded{Relay}` owner is admitted when every relay member is
     closed and every source F is `drive_settled` or `relay_drivable`;
   - `Excluded{Underived}` and non-admitted `Excluded{Relay}` owners are
     refused.
4. **The ruled relay arm: drive at the source, never carry the relay.**
   - `relay_drivable(F, R, O)` holds when all four hold:
     - (a) F has exactly one installed or candidate response row with a
       planned seat;
     - (b) F's operation is a `Construct(coproduct, [Construct(..)])` and
       its K is a one-parameter `LexicalClosure`;
     - (c) the walk from F to R crosses only `response_tail_edge` edges
       and `ComputationalMatch` scrutinees whose single Vis case
       satisfies `response_forwards_vis`, with no generated Call edge;
     - (d) the key is the pair (F, `O.base_owner()`), never F alone.
   - A second gate in `lower_computational_producer_construct`, after
     the deferred one, calls the same drive. The drive is refactored to
     take (vis_origin, operation_root_origin, effect_origin, operation),
     so there is one drive with two row sources.
   - A relay construct of an admitted owner that is still reached is a
     typed planner-invariant error, never `-1`.
   - Admitted relay members leave O's returned set: span owner 4 becomes
     {528} Protocol, r2 owner 2 {746}, and r2 owner 3 {528}.
5. **Admission E, in a separate commit.** `ContinuationBodyTail` holds
   when the walk up `response_parents` from the producer construct to
   `worker_body_origin()` crosses only `response_tail_edge` edges. This
   replaces root equality. The E guard stays.

## Acceptance

- **AC-1.** The span row is un-ignored and green: both engines exit 0 with
  the exact freeze sequence it asserts.
- **AC-1b.** r2 (`:1015`) is un-ignored and green, with the full 42 §6.4
  envelope and the terminal result agreeing on both engines. The three r2
  compile-or-inspect rows (:861, :913, :970) pin the admitted plan.
- **AC-5 (mutation, QA).** Deleting source (e) yields the ObjectEmission
  refusal (owner 0, context 2) on the span row. This is carried from the
  predecessor, where lowering is not reached.
- **AC-R3.** As listed in the recut `evt_2091hd1wtkhs5`; moved here by
  `evt_14w6eh1d2hwk4`.
- **AC-2 (control).** An `Excluded{Underived}` owner, and a relay whose
  provenance set is not planned, are refused at planning with the exact
  typed text. ABI-S6 is 12/12.
- **AC-3 (mutation, QA).** Removing the relay arm returns the span row and
  r2 to the planning refusal, never to `-1`.
- **AC-6 (mutation, QA).** Classifying ABI-S6 owner 0 as refused reddens
  its six rows at the refusal, so the discriminant is load-bearing.
- **AC-7.** The two-print arm-tail witness from D0 (iv) becomes a
  compared parity row: native and interpreter both exit 0 with stdout
  `captured\nsecond\n`. Landing at a planning refusal instead is a stop,
  not a pass.
- **AC-8 (mutation, QA).** Restoring the root-equality
  `ContinuationBodyTail` test returns AC-7's row to the typed E refusal
  `RelocatedWorkMissingLoweringBinding`.
- **AC-4 (controls).** SEQUENTIAL, `one_bracket_retains_native_parity`,
  the five sibling rows and the predecessor's AC-R6 census population stay
  green; px8ta keeps its labelled failures.

## Stop conditions

- D0 (i) is open, or no planning-refusal discriminant separates the
  trapping owners from ABI-S6 owner 0.
- D1's emission-owner checkpoint measures an owner other than
  `O.base_owner()`: stop with the measured owner, and do not substitute
  another key.
- The census shows a refused owner in a row that is green today, or a
  row moving to `ContinuationBodyTail` with a handler owner in a green
  row: stop to the Architect, naming the owner and row.
- The repair touches the bracket tree, moves a held ref
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, `wp/RT-BRACKET-SETTLEMENT-PLANE`,
  `4b4c8565c`, `21c039918`, `7f1a04a40`), or relaxes the `units.rs:7831`
  coverage gate or the `0x101d_0000_002a` guard.
- Any kernel, trust or spec change.
