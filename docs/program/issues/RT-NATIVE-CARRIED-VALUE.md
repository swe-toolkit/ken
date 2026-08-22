---
id: RT-NATIVE-CARRIED-VALUE
title: "Native carried-value / higher-order boundary lowering — the capability program that closes PX8's native full-program half, structured as two tracks over ONE open representation decision"
status: active
owner: runtime
size: L
gate: none
depends_on: []
blocks: [PX8]
github: null
origin: "Steward, 2026-08-22, filing the Architect's native-program design frame (evt_9kat78d438cb) per operator decision 3. Replaces the earlier six-node 'native carried-value family' premise: the Architect re-verified each seam at the producer (not the #[ignore] strings) and found the premise half already-closed. Steward-filed per COORDINATION section 2."
---

> # PROGRAM ROOT — Architect frame evt_9kat78d438cb @ 6425709fb
>
> The six-member premise collapses. Two corrections, both load-bearing:
> - M1 (RT-CARRIED-RESOURCE-SCALAR) and M2 (RT-DEAD-ARM-EFFECT-LOWERING) are NOT
>   open — their mechanism is MERGED (the (need,phase)-keyed EffectSeatClaimRoute
>   protocol, effects.rs:495-505; SHAs ef32b6ced, 569ba3d0d are ancestors of
>   main). Their #[ignore] rows are STALE. Not filed as open; their only residual
>   is the Track-0 un-ignore.
> - M5 ([[RT-COMPMATCH-TREE-SCRUTINEE]]) is NOT under the predicate — a
>   match-scrutinee static-shape gap, not a carried/boundary observation, and NOT
>   one of PX8's four reified values. It is a SIBLING outside this program and
>   does not gate PX8.

## The sharpened predicate (Architect §1b)

The mandate's predicate splits on one axis, and the split separates open from
closed:

- First-order carried observation (a carried WORD observed as scalar/witness):
  SOLVED by the (need,phase) route protocol. Residual = bookkeeping.
- Higher-order representation (a function/closure/eliminated-recursor value
  crossing with no first-class runtime representation): the ONE open decision.
  Its answer is already proven in the adjacent merge —
  DEFUNCTIONALIZE (env as an admitted Record + static dispatch;
  [[RT-CLOSURE-CROSSING-ELIMINATE]] merged PR #2327 did exactly this for the
  source-authored closure population).

## Two tracks

- Track 0 ([[RT-NATIVE-TRACK0-REARM]], shovel-ready first, NO new mechanism):
  un-ignore the first-order stale rows, re-measure native, re-arm the vacuous CI
  jobs, run the ignored-sweep as the oracle. Owns the decision-4 CI
  de-vacuuming. Lights PX8's first-order native witnesses ReadEof / ReadSome /
  Wrote.
- Track 1 (one design decision applied at three seats): the D0 =
  [[RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]] (defunctionalization, ruled by the
  Architect). Consumers [[RT-CARRIED-IH-DISPATCH-SITEOP]] (M3) and
  [[RT-CLOSURE-BOUNDARY-RESIDUAL]] (M4). Lights PX8's positioned native witness
  SemanticErrorV1.

Dependency order: Track 0 first → Track 1 D0 → M6 (the representation itself,
inside the D0 node) → M3 + M4.

## Closure condition (when PX8 re-verifies + closes)

On the NATIVE backend, a checked positioned/partial-IO program reifies each of
the four values — absolute against the LOCKED text of
`spec/30-surface/38-ffi-io.md`, co-indexed to the same request/span/buffer —
with native full-program rows GREEN and UN-IGNORED, both engines co-indexed:
ReadEof/ReadSome/Wrote by Track 0; SemanticErrorV1 by Track 1. M5
(span-provenance) is not one of the four and does not gate this.
