---
name: a-new-typed-failure-channel-covers-the-new-resource-and-a-new-input-surface-exposes-the-old-ones
description: When a change adds a typed-failure Result to a reservation function for one new resource, and also adds a deployment input surface (CLI JSON, C struct) for the whole profile, attack the old limits that same function consumes. They still use unchecked arithmetic, and the new surface now lets a deployment reach it.
metadata:
  type: feedback
---

# A new typed-failure channel covers the new resource; a new input surface exposes the old ones

**Measured 2026-09-24 on RT-INVOCATION-RESOURCE-PRECURSOR D1.** Squash
`e49cdc9b92c8ec88250d994f7bbb0971b2493ef5`, reported at `evt_5pj2w3qzkedv4`
(thread `thr_5pvqq8v8g1y9s`). The defect was repaired afterwards;
`crates/ken-runtime/src/boundary_activation.rs` cites the panic at `e49cdc9b`.

## The shape

`BoundaryActivationV1::begin` gained `Result<_, CapacityExhaustedV1>` for the
new process-wide epoch. The same change added `ken native-build ...
<profile.json>`, which lets a deployment supply all eight *pre-existing* region
limits. Before, only a hard-coded smoke profile reached them. `begin` still
called `reserve`, and `reserve` computed `(live + nodes) * NODE_WORDS`
unchecked (`boundary_value.rs`, at that squash). A declared `nodes` of
1676976733973595602 (11 x that = 2^64 + 6) gave these results:

- Dev builds panic ("attempt to multiply with overflow"). Across `extern "C"`
  the process aborts, and the fault arrives untyped.
- Release builds wrap to a node capacity of 0, so a huge declared limit
  silently becomes zero.

The epoch fault itself was clean. The defect was in what the new input surface
newly reaches.

## How to apply

1. When a change adds a parser or FFI struct for a profile or config, list
   every field it now admits. For each field, read the arithmetic in its
   consumer. A new `Result` on the function covers only the resource its author
   was thinking about.
2. Pick a value where `n * stride` wraps to a small number; that exposes both
   failure modes. Probe it in a scratch unit test with `catch_unwind` and
   print the published capacity.
3. Check whether published capacity is derived from the real storage length.
   If it is, the defect is correctness (a declared limit that is not honored),
   not memory safety. Say which one it is.

Related:
[[a-newly-reachable-allocation-producer-must-be-censused-against-its-sibling-allocators-declared-capacity-governor]]
(the sibling shape: that one is a new producer with no governor; this one is an
old governor newly fed by deployment input). The general form -- widening what
reaches existing code exposes assumptions the old, narrower input protected --
is also recorded for the kernel team in
`teams/kernel/gate-widening-exposes-latent-bugs-in-newly-reachable-code.md`.
