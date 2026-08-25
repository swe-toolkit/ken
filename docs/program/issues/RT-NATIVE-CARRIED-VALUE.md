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

- Track 0 ([[RT-NATIVE-TRACK0-REARM]]): PURE HYGIENE (CORRECTED 2026-08-22,
  Architect evt_4sp2xftkmc1mz). Re-label the stale first-order `#[ignore]`
  reasons to their measured current blocker, confirm M1/M2 gone, establish the
  sweep baseline. It lights NO PX8 witness green by itself. Its AC-2 (rows go
  green) was FALSIFIED — closing M1/M2 MOVED the checked-write rows to the
  closure-boundary seam, not to green. The decision-4 CI de-vacuuming re-homes to
  Track 1 (the rows un-ignore only when Track 1 greens them).
- Track 1 (one design decision applied at three seats): the D0 =
  [[RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]] (defunctionalization, ruled by the
  Architect). Consumers [[RT-CARRIED-IH-DISPATCH-SITEOP]] (M3) and
  [[RT-CLOSURE-BOUNDARY-RESIDUAL]] (M4). Lights ALL FOUR native full-program
  witnesses — ReadEof/ReadSome/Wrote AND positioned SemanticErrorV1 — because
  every native checked-IO full program carries the checked continuation closure,
  so the higher-order representation gates the first-order values too. Track 1
  got bigger, Track 0 got smaller; the representation decision is unchanged.

Dependency order: Track 0 (hygiene, independent) then Track 1 D0 → M6 (the
representation itself, inside the D0 node) → M3 + M4. Track 1 is the whole
remaining PX8-closure critical path.

## Symptom inventory (§1b-i)

Entry 1 (measured 2026-08-22, [[RT-NATIVE-TRACK0-REARM]] sweep): closing the
first-order (need,phase) family MOVED the checked-IO full-programs to the
closure-boundary seam, not to green — three refusals ("closure cannot cross"
M4, "static worker expects 1 arguments but call provides 0" M6, "carried
recursive hypothesis is an eliminated value" M3) are ONE defect. Keyed on: the
checked continuation is a higher-order value with no first-class native
representation. The predicate was already named; this is its measured
confirmation, and it strengthens the unification (one checked program manifests
it at multiple seams) rather than fracturing it.

Entry 2 (measured 2026-08-25, Architect umbrella arm(b) object read at merged
`d9bc68db0`, evt_7jpt4hm2nm6hh): px8ta CLOSES (its driving row reaches successful
terminal observation) but the FOUR full-program witnesses do NOT — three semantic
blockers remain, NOT an un-ignore-only residual (the px8ds 256 MiB policy is
unrelated). The remaining lane-1 critical path, in Architect-recommended order:
- SemanticErrorV1 (both positioned parity rows): SPLIT by the AC-5 hard stop
  (Architect evt_3rq4xafrf7cqf on WIP `7094c29cd`). (i)
  [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] (scope-reconciled) carries the root
  generated-unit failure identity to the LINKED reporting boundary
  (`calls.rs:2075-2090` `-4` + governed `-3`, one typed-catalog envelope) — HONEST
  reporting; lands with the row still RED as the ITree default. (ii)
  [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]] localizes the exposed ITree-default
  SELECTION occurrence and greens the exact `InvalidOffset`. (i) FIRST, (ii) after;
  must not co-run. Wider identity-collapse census (`joins.rs::emit_current_trap`,
  `aggregates.rs`, two source-machine `-3` paths) enumerated but out of (i)'s
  operative claim.
- ReadSome/Wrote (both px8f rows stop at object emission): retained body
  `StaticOriginId(1236)` has no graph-derived call target
  (`calls.rs:1631-1640`, `call_declared_unit`) ->
  [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]], now M3-unblocked. SECOND; MUST NOT
  co-run with the first (both touch `calls.rs`).
- ReadEof: no executing native closure witness today (the `rt_cap41` stages are
  unselected by any Rust test) -> a final closure/un-ignore/CI-rearm fold, framed
  after the two objects land plus a fresh four-value object read.
The lane-1 headline is NOT done; PX8 stays blocked.

## Closure condition (when PX8 re-verifies + closes)

On the NATIVE backend, a checked positioned/partial-IO program reifies each of
the four values — absolute against the LOCKED text of
`spec/30-surface/38-ffi-io.md`, co-indexed to the same request/span/buffer —
with native full-program rows GREEN and UN-IGNORED, both engines co-indexed:
ReadEof/ReadSome/Wrote by Track 0; SemanticErrorV1 by Track 1. M5
(span-provenance) is not one of the four and does not gate this.
