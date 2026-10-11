---
id: RT-NESTED-RELAY-PERFORMING-ROUTE
title: "After RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT a forwarded host op with no performing route stops native at a planned relay trap where the interpreter performs it (FsReadAt), so native refuses a valid program the interpreter runs. Emit an outer member that performs the forwarded op and retire the traps (route C)"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT]
blocks: []
github: null
origin: "Increment (ii), route (C), of RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT, which names it a successor node framed from the D0 table (evt_3y63h26apw108, evt_6jxckz0adhrbm). Architect approval of the increment (i) respin evt_7vdy3sx322062 records its coverage obligation here. Steward-filed per COORDINATION section 2."
---

# Forwarded host ops are performed natively

## Objective

A relay pair whose forwarded host op has no route today performs that op
natively, in the interpreter's order, instead of stopping at the planned
relay trap.

## Settled inputs (increment (i), exact `22d5f0956`)

- **The traps.** Increment (i) plans a typed relay trap for each route-none
  (pair, identity) (`MissingRuntimeMetadata`, "relay: forwarded host op
  <op> from source <origin> has no performing route"). Three consuming
  seats emit it: terminal transport, detached result, and the ownerless
  source. R4 makes an admitted pair with no planned call edge a planner
  refusal.
- **AC-MAIN is a transition sentinel.**
  `branched_scrutinee_forwarded_read_is_performed_or_refused`
  (`crates/ken-cli/tests/rt_branched_scrutinee_unit_body_port.rs`) asserts
  that native stops at exactly the planned trap with no `FsReadAt`. This
  WP turns it into parity, with `FsReadAt` performed.
- **The D0 table** (SHA-256 `c2b23461`) is the population. Row 36's 11
  pair-builds already leave by `deferred_handler_owned`, a performing
  route.
- **Coverage obligation (Architect `evt_7vdy3sx322062`).** QA found no
  discriminating witness for the three seat guards, nor any
  `Excluded{Underived}` owner, in the targeted rows (`evt_3a4mkedqb5v3c`).
  This WP supplies one witness per seat and one Underived-owner fixture.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

AC-0 first. Then route (C): for each route-none pair, an outer member that
performs the forwarded op, with the traps retired. The Architect rules the
emission point at D0.

## Acceptance

- **AC-0 (measure).** On the landed base: the route-none population per
  (pair, identity) from the D0 table, the seat each reaches, and for each
  seat a fixture that reaches it.
- **AC-1.** AC-MAIN asserts parity. Native performs `FsReadAt`, and its
  non-release events equal the interpreter's in order, with releases
  compared as a set through the shared helpers.
- **AC-2 (witnesses).** One row per seat (terminal transport, detached
  result, ownerless source) and one `Excluded{Underived}` owner row. Each
  is performed or refused, and each reddens when its seat is mutated.
- **AC-3 (control).** Rows 01-36 and the four AC-MAIN controls keep
  parity under the fleet release rule.

## Stop conditions

- The repair restructures the planner's build order: return to the
  operator to size (operator 2026-09-21).
- Any kernel, trust or spec change.
- A red native CI row on increment (i)'s SHA is a stop to the Architect
  before this WP starts.
