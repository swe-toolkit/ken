---
id: RT-CERTIFIER-FRAME-CALL-EXEMPT-CALLER-PIN
title: "The caller-side half of the certified frame-call retention exemption, certified_frame_call_exempt, has no test that reaches it, because only close_and_define_staged_result_bodies fills frame_calls. Deleting its store scan widens the exemption and leaves every suite green. Pin each refusing guard with rows that fill frame_calls directly and go red when the guard is removed"
status: merged
owner: runtime
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Measured gap on RT-CERTIFIER-CALLEE-SLOT-RETENTION (944cabd38), Adversary evt_67txs8v7ae9z2: with the store scan emptied, ken-runtime --lib 1081/1081, rt_span_prov_native 8/8 and px7f_resource_native 7/7 stay green while a clobberable slot is reported Exact. Check 8: the guard has no observation its removal does not also produce. Steward-filed per COORDINATION section 2."
---

# Every caller-side exemption guard has a red row

## Objective

Each guard in `certified_frame_call_exempt` that refuses the retention
exemption has a unit row that fills `frame_calls` directly. That row goes red
when the guard is removed. Production behavior does not change.

## Fixed inputs (main `944cabd38`, `lowering/units.rs`)

- **The exemption.** `certified_frame_call_exempt` (`:13495`) exempts a
  certified frame call from retention. It refuses the exemption when the
  caller stores a container address into the header or payload (the store
  scan, `:13516`), and it also checks the services argument and the location
  slot.
- **Why nothing reaches it.** `frame_calls` has one writer,
  `close_and_define_staged_result_bodies` (`:12392`). No unit test fills it,
  so in the lib suite the function returns false at its first line. The
  native population reaches it only on accepting plans.
- **The repro** (`evt_67txs8v7ae9z2`).
  - Caller slots are H (16 bytes) and P (24). `&P` is stored at H+0, and
    `0x3000` at H+8.
  - The body is `call unit(&H, 0x2000)`, then `store 7` at P+8, then
    `t = load(0x1000)`, `store 9` through t, and finally P+8 is loaded as the
    dependent.
  - With `(call → (H, P))` in `frame_calls`, the control gives
    `{Exact(v5)}`.
  - The variant that also stores `&P` at P+16 gives `{Unknown}`. With the
    scan emptied it gives `{Exact(v5)}`.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Test-only rows in `mod generated_result_protocol_verifier` that fill
`frame_calls` directly. They are the accepted control and one refused row per
guard:

- a self-pointer in the payload;
- a container address at H+8;
- services derived from H;
- the location-slot condition.

## Acceptance

- **AC-1.** The control gives `{Exact}`, and each refused row gives
  `{Unknown}`, on the candidate.
- **AC-2 (mutation, QA).** Removing each guard in turn reddens its own row,
  with the control and the sibling rows still green. Emptying the store scan
  is one of these mutants. Each is restored byte-identically.
- **AC-3.** Production code is byte-unchanged from the cut base.

## Stop conditions

- A guard whose row cannot be built without a production change.
- A refused row that gives `{Exact}` on the cut base. That is a soundness
  defect, not a missing pin: hard-stop to the Architect with the row.

Checks: 8 fired: each row is tied to one guard, and AC-2 removes that guard and
requires that row to go red, so a passing suite no longer stands in for the
guard. 2 fired: `frame_calls` has a single writer at `:12392`, as the Adversary
measured, so a row must fill it directly to reach past the first line. 3
fired: the Adversary grepped `frame_calls`, `certified_frame_call_exempt` and
`frame_non_retaining` across `crates/`, and every hit is in `units.rs`.

## Hard-stop inventory (§1b)

§1a count (caller-guard pinning): 1 (Architect `evt_38t04nvmh6gte`).

1. The location-slot guard is implied by the frame, services and scan
   guards because the caller index and container edges share decoding
   (keyed on index-protocol coupling).
