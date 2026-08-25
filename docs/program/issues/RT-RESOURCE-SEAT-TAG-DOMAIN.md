---
id: RT-RESOURCE-SEAT-TAG-DOMAIN
title: "Lane-1 px8ta successor (replaces the falsified operand-pairing node) — carried resource-token seat tag-domain correction. lower_resource_token_seat's carried arm guards BoundaryTag::InvocationBorrowed == 7 (a transport tag byte in bits 0..8 of the carrier word) against emit_carrier_tag, which returns the semantic NODE_TAG_ID (0 for a borrowed opaque node); it therefore compares semantic node identity 0 with transport discriminant 7 and refuses the correct word. Replace ONLY that wrong identity query with the carrier contract's existing low-byte projection (band_imm word, BOUNDARY_TAG_MASK), keeping the InvocationBorrowed expected-tag, BorrowedOpaque class, and scalar guards exact. effects.rs is the sole production change; core.rs and units.rs stay byte-untouched; no continuation operand pairing."
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CARRIED-BOOL-ELIMINATOR-DISPATCH]
blocks: []
github: null
origin: "Steward, 2026-08-25, from Architect ruling evt_7rhvct552ts2w (thr_3t78aab0v2dh) — hard stop 1 for the falsified [[RT-CONTINUATION-RELEASE-OPERAND-PAIRING]]. The Architect UPHELD the implementer's D0 hard stop: the lineage table is correct, Parameter[0][0][1] is the exact raw capture v11 (runtime word 0x0507), and nothing in assemble_continuation_call_operands / the target-1 frame / the origin-171 Var(1) binding changes it into a neighbour. There is NO wrong operand and NO second release attempt to eliminate. The truthful object is a resource-carrier tag-domain correction at the consumer, not operand pairing. No design Decision and no Research trigger — the representation contract determines the answer. Steward owns the frame correction + fresh Runtime release. Symptom inventory durable at architect/rt-release-pairing-inventory @ 1e16fd99bf5a393eedcacc16028bc4eb959b8afd."
---

> # READY 2026-08-25 — replaces the FALSIFIED operand-pairing node
>
> [[RT-CONTINUATION-RELEASE-OPERAND-PAIRING]] is CLOSED as falsified by Architect
> ruling evt_7rhvct552ts2w (hard stop 1): its D0 was upheld, so there is no wrong
> operand and no second release attempt — its title, objective, D1, AC-2, and
> AC-3's "lower_resource_token_seat unchanged" clause are all falsified. This node
> carries the truthful object the ruling determined. Base the branch on the `main`
> the Architect grounded the repair at (`d0e85f1a` or later).

# Fixed inputs (Architect ruling evt_7rhvct552ts2w; grounded at d0e85f1a)

The pristine ignored row still fails:

```text
scripts/ken-cargo test -p ken-cli --test px8ta_oriented_subcontinuation \
  px8ds_real_same_depth_path_runs_exact_edges -- --ignored --exact --nocapture
```

Trace `BufferAllocate -> ConsoleIsTerminal(false) -> ResourceRelease`, then
`RuntimeTrap(1)`, exit 1 (pristine log SHA-256
`98077696ae5f3f129a0b6dcff80da939b0dbee194826fba2863d4c2a283fe659`).

- The operand is CORRECT. `Parameter[0][0][1]` is the exact raw capture `v11`,
  runtime carrier word `0x0507`; `assemble_continuation_call_operands`, the
  target-1 frame, and the origin-171 `Var(1)` binding all preserve it. There is NO
  neighbour substitution, NO second logical release attempt. Do NOT re-frame an
  operand-pairing repair.
- The defect is two unrelated tag vocabularies sharing an integer type at the
  resource consumer `effects.rs::lower_resource_token_seat` (carried arm):
  - `BoundaryTag::InvocationBorrowed == 7` is the TRANSPORT tag byte in bits 0..8
    of the carrier word.
  - `emit_carrier_tag` does NOT return that byte. Its exact-base contract returns
    the runtime constructor identity (NODE_TAG_ID), comparable only with
    `ConstructorIdentity::tag_abi_word()` from the same plane. It calls
    `ken_boundary_tag_local`, whose `emit_boundary_value_local_graph` defines it as
    `define_node_word(..., NODE_TAG_ID)`; for this borrowed opaque node the
    semantic `NODE_TAG_ID` is `0`.
  - The seat compares semantic node identity `0` with transport discriminant `7`,
    so the correct word `0x0507` is refused.
- Architect exact-base probes (only the failing instrumentation changed, never the
  test; all restored byte-identically):
  1. Unique `require_i64` markers → `RuntimeTrap(7443)`, emitted in Cranelift
     `UserFuncName u0:50` at an `expected=7` guard — retains the `funcid50` /
     specialization-1 location (log `5f5c98b0...`).
  2. Returning the failing operand itself → `RuntimeTrap(1287)`; `1287 == 0x0507`,
     low byte exactly `7` (log `7d284e0d...`).
  3. Returning `40 + emit_carrier_tag(...)` → `RuntimeTrap(40)`: this handle's
     node-semantic tag identity is `0`, not `7` (log `b02cb806...`).
  4. Replacing only the wrong identity query with the carrier contract's low-byte
     projection → the unchanged ignored row passes 1/1, exit 0, exact trace with
     one and only one `ResourceRelease` (log `809daf5d...`).
- Exact production blobs at base: `effects.rs 817450730bbdfff10428f7f12bc5205149a6ace7`,
  `core.rs 675585a42c4059dd320f2d59e89a552dc4e5c129`,
  `units.rs 81f5cbab4188fe43d98b3fcba802d3d66760e94d`.

# Authority-derived repair (Architect-specified) — the carried arm of `lower_resource_token_seat`

```rust
let tag = builder.ins().band_imm(
    word.word,
    crate::boundary_value::BOUNDARY_TAG_MASK as i64,
);
Self::require_i64(
    builder,
    tag,
    crate::boundary_value::BoundaryTag::InvocationBorrowed as i64,
);
let class = self.emit_carrier_class(builder, *word)?;
Self::require_i64(builder, class, BoundaryClass::BorrowedOpaque as i64);
self.emit_carrier_scalar(builder, *word)
```

This is NOT a new helper or fallback. The exact base already uses the identical
`BOUNDARY_TAG_MASK` to distinguish carrier transport tags in
`narrow_carried_int_u64`, `emit_public_carrier_scalar`, primitive lowering,
joins, and units. The expected-tag guard stays exact, the `BorrowedOpaque` class
guard stays exact, and scalar projection is unchanged. What changes is ONLY that
the tag guard reads the tag vocabulary it claims to guard.

# Deliverable

- D1 — at `lower_resource_token_seat`'s carried arm ONLY, replace the
  `emit_carrier_tag` identity query with the `BOUNDARY_TAG_MASK` low-byte
  transport-tag projection; keep the `InvocationBorrowed` expected-tag guard, the
  `BorrowedOpaque` class guard, and the scalar projection exact. `effects.rs` is
  the sole production change; `core.rs` and `units.rs` stay byte-untouched. Zero
  `trusted_base()` delta.

# Acceptance criteria (Architect-required controls)

- AC-1 — the pristine ignored row `px8ds_real_same_depth_path_runs_exact_edges`
  passes 1/1, exit 0, and its exact asserted trace still contains one and only one
  `ResourceRelease`. px8ta advances / closes at this observed closure; promise NO
  broader successor beyond it.
- AC-2 — the fix is a TAG-DOMAIN correction, proven positively: an
  `InvocationBorrowed` / `BorrowedOpaque` handle whose semantic `NODE_TAG_ID` is
  NOT `7` but whose low transport tag IS `7` is accepted (establishes the guard now
  reads the transport tag byte, not node identity).
- AC-3 — a wrong-low-tag negative refuses BEFORE class/scalar projection (the
  expected-tag guard stays exact and fail-closed).
- AC-4 — the exact `BorrowedOpaque` class guard and the scalar projection are
  preserved unchanged.
- AC-5 (mutation controls). A compile-preserving mutation that REINTRODUCES
  `emit_carrier_tag` at this seat REDS the positive control AC-2. A guard-weakening
  mutation that admits a wrong low tag REDS the hostile control AC-3.
- AC-6 — `core.rs` and `units.rs` are byte-untouched; `effects.rs` is the sole
  production change; zero `trusted_base()` delta.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli --test px8ta_oriented_subcontinuation` only, never `--workspace`.

# Reviewers

Architect (the correction reads the transport tag vocabulary the guard claims to
check; the expected-tag / class / scalar guards stay exact; `core.rs`/`units.rs`
untouched; no operand-pairing change) + runtime-qa (the two mutation controls red
as specified; the guard stays fail-closed). No Decision fork — the representation
contract determines the answer (Architect: hard stop 1, no Research trigger).

# Capability tier

T1 — the review turns on the tag-vocabulary / representation-domain argument
(which integer plane the guard must read), not a mechanical diff. Size S (a
single-arm identity-query replacement at one seat).

# Sequencing

Lane-1 (runtime, priority) — the px8ta successor, replacing the falsified
[[RT-CONTINUATION-RELEASE-OPERAND-PAIRING]]. px8ta stays OPEN until the native
carried-value program lands; this is the next object in that program. Base the
branch on `d0e85f1a` or later main.
