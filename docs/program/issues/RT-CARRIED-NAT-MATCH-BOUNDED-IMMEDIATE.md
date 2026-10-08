---
id: RT-CARRIED-NAT-MATCH-BOUNDED-IMMEDIATE
title: "A host-produced ImmediateBoundedNat (PrivateBufferSpan field 3, spill class Int) observed by a carried structural Nat Match traps at the Constructor-class guard, so the reached Nat fanout aborts natively after the interpreter's own effect prefix. Decide and build structural Nat observation of a bounded immediate, or a static refusal, so the three Nat variants run with interpreter parity"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-FORWARD-TAIL-RET-CHECKED-CONTROL]
blocks: []
github: null
origin: "Architect split evt_617g7acwvmk4n of RT-FORWARD-TAIL-RET-CHECKED-CONTROL on its D6 localization (evt_564nrkb0wcycf): the Nat trap is a separate axis from the Ret sink, and AC-2 of that WP moves here. Serves the L1 objective (operator 2026-09-17). Steward-filed per COORDINATION section 2."
---

# A carried Nat match over a bounded immediate

## Objective

The three reached Nat fanout variants in
`rt_escape_second_resource_native.rs` run natively with interpreter parity,
and the Nat row is un-ignored.

## Settled inputs (Architect `evt_617g7acwvmk4n`, D6 log)

- **The site.** Carried `Match` origins 1070, 1208 and 1209, over
  `Nat::Zero`/`Nat::Suc`, observe class `Int`. The scrutinee is a
  `BufferSpan::ctor_423` projection. The trap follows the first native
  `FsReadAt`, after the native trace `[FsOpen, BufferAllocate, FsReadAt]`,
  which is the interpreter's own prefix. Log:
  `local/rt-forward-tail-ret-d1/nat-three-d6-localization.log`.
- **The producer.** `READ_PROGRESS`
  (`planning/static_transition/aggregates.rs:3772`) builds
  `PrivateBufferSpan(ResourceToken, Int, BoundedNat)`. A `BoundedNat` scalar
  is carried as `BoundaryTag::ImmediateBoundedNat` with spill class `Int`.
- **The guard.** The carried-match class check (`lowering/joins.rs:1122-1140`)
  admits only `Constructor` for a non-Bool constructor family, so every
  immediate scalar fails closed before a node-only observation. It does not
  depend on the Ret-sink route.
- **The predecessor's pin.** After `RT-FORWARD-TAIL-RET-CHECKED-CONTROL`, one
  active pin asserts the three variants end in `UnclassifiedRuntimeTrap {
  terminal_value: -1 }`. This WP replaces it.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only, at the start of the repair).**
   - (a) The population of carried constructor `Match`es whose scrutinee can
     be an immediate-scalar host field.
   - (b) Whether any active row already observes a `BoundedNat` structurally
     on another route (px8n carries the same structural Nat 1 in
     `BufferSpan`, `effects.rs` ~2314), and how.
   - The Architect rules the design at D0: structural Nat observation of a
     bounded immediate, or a static refusal.
2. **The ruled repair.**

## Acceptance

- **AC-1.** The Nat row is un-ignored. All three variants match the
  interpreter on exit, stdout and the full `EffectEvent` vector: 2 reads for
  the baseline and Zero-extra variants, 3 for Suc-extra.
- **AC-2 (controls).** The escape file's other rows, `rt_parity_native` on
  the default thread, and the D0 (b) route keep their results.
- **AC-3 (mutation, QA).** Restoring the Constructor-only class guard for
  this route returns the three variants to the `-1` trap.

## Stop conditions

- The repair needs a kernel, trust or spec change.
- A dropped or reordered effect on any row, or a default-thread overflow on
  an un-ignored row.
- The D0 (a) population reaches a scrutinee whose class the repair cannot
  name.

## Closeout

Merged `e18f9a2cc` from exact `38f3d4691` (PR #4597). Runtime QA
`evt_1m9fcv29pat53`, Architect `evt_5myvjj1h1t0xt`, Decision
`dec_2shemr38s3t6a`. The composed suffix is threaded into
`lower_bounded_nat_match_with_plan`, and the Nat fanout row of
`rt_escape_second_resource_native` runs at interpreter parity and is
un-ignored. The full `rt_parity_native` population was gated by CI.
