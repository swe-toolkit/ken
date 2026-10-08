---
id: RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT
title: "Native lowering has no representation for a relay Vis (a K that forwards a pattern-bound operation, or a functional-IH reference) returned to a response owner, so after RT-NESTED-RESPONSE-OWNER-CALLER every such owner is refused at planning, and the ignored rt_span_prov_native:355 and r2 (rt_escape_second_resource_native:1015) rows stay ignored. Give the relay a planned native arm that settles it at interpreter parity"
status: ready
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
  value or environment representation. Lowering has no relay arm.
- **The seat plane has no relay seat.** `build_host_effect_seat_plan`
  (`effects.rs:658`) iterates static `RuntimeExpr::Effect` occurrences, so
  a forwarded operation has no seat of its own.
- **The predecessor's state.** RT-NESTED-RESPONSE-OWNER-CALLER stores each
  owner's `ResponseOwnerSettlement`, and its planning refusal names owner 4
  with `Relay`. This WP turns that `Relay` classification into a settled
  route; it does not relax the refusal for any other `Excluded` owner.

- **The r2 population** (from `RT-SOURCE-IH-RELAY-K-VALUE`, Architect
  `evt_1j5qaw2d9sqe7`). The predecessor refuses r2's owners 2 and 3
  (Relay). The relay's K operand is `CheckedComputationalIHInvocation {
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

1. **D0 (Architect design; measurement only).** Measure:
   - (i) the closed provenance set of static `Effect`/Vis occurrences
     whose operation can flow into each relay's `Var`;
   - (ii) the relay IH K's capture and environment layout at the return,
     against the existing pending-Vis frame region;

   over both populations: the span's owners 3 and 4, and r2's owners 2 and
   3. If one design cannot settle both, the D0 says so and the Architect
   rules whether to split.

   Pre-ruled (Architect `evt_14w6eh1d2hwk4`): if (i) is closed and
   finite, the relay arm dispatches over the planned seats of exactly
   that set, and the node proceeds. If (i) is open, **stop**: a precursor
   node for dynamic host-effect dispatch comes first.
2. **The ruled relay arm,** with `Relay` admitted at planning only for an
   owner whose relay provenance set is planned.

## Acceptance

- **AC-1.** The span row is un-ignored and green: both engines exit 0 with
  the exact freeze sequence it asserts.
- **AC-1b.** r2 (`:1015`) is un-ignored and green, with the full 42 §6.4
  envelope and the terminal result agreeing on both engines. The three r2
  compile-or-inspect rows pinned to the predecessor's refusal move to the
  admitted plan.
- **AC-5 (mutation, QA).** Deleting source (e) yields the ObjectEmission
  refusal (owner 0, context 2) on the span row. This is carried from the
  predecessor, where lowering is not reached.
- **AC-R3.** As listed in the recut `evt_2091hd1wtkhs5`; moved here by
  `evt_14w6eh1d2hwk4`.
- **AC-2 (control).** An `Excluded{Underived}` owner, and a relay whose
  provenance set is not planned, keep the predecessor's typed planning
  refusal.
- **AC-3 (mutation, QA).** Removing the relay arm returns the span row and
  r2 to the predecessor's `Relay` refusal, never to `-1`.
- **AC-4 (controls).** SEQUENTIAL, `one_bracket_retains_native_parity`,
  the five sibling rows and the predecessor's AC-R6 census population stay
  green; px8ta keeps its labelled failures.

## Stop conditions

- D0 (i) is open.
- The repair touches the bracket tree, moves a held ref
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, `wp/RT-BRACKET-SETTLEMENT-PLANE`,
  `4b4c8565c`, `21c039918`, `7f1a04a40`), or relaxes the `units.rs:7831`
  coverage gate or the `0x101d_0000_002a` guard.
- Any kernel, trust or spec change.
