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

1. **D0 (Architect design; measurement only).** First, complete the
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

   - (iv) **the predecessor's successor obligation, "admission E
     unwitnessed after ContinuationBodyTail"** (Architect
     `evt_3tkas7ea7k50f`): whether any source in either population
     reaches `RelocatedWorkMissingLoweringBinding`
     (`selected_pending_calls.rs:769`). Production checks E before J, the
     bounded attempt at `9f5b430b5` reached J, and no test pins E. The
     Architect rules whether E stays as a guard, with a witness, or goes;

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
2. **The ruled planning refusal,** naming every refused owner, placed at
   `planner.finish(`'s single caller after the cfg'd recorders
   (`evt_59xpt6t02wdpq`).
3. **The ruled relay arm,** with an owner admitted at planning only when
   its relay provenance set is planned.

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
- **AC-4 (controls).** SEQUENTIAL, `one_bracket_retains_native_parity`,
  the five sibling rows and the predecessor's AC-R6 census population stay
  green; px8ta keeps its labelled failures.

## Stop conditions

- D0 (i) is open, or no planning-refusal discriminant separates the
  trapping owners from ABI-S6 owner 0.
- The repair touches the bracket tree, moves a held ref
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, `wp/RT-BRACKET-SETTLEMENT-PLANE`,
  `4b4c8565c`, `21c039918`, `7f1a04a40`), or relaxes the `units.rs:7831`
  coverage gate or the `0x101d_0000_002a` guard.
- Any kernel, trust or spec change.
