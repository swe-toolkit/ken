---
id: RT-AMBIENT-UNIT-REPLY-SHAPE
title: "ambient_dispatch wraps every Unit host reply in Ok, but clock_resp SleepUntil is Unit, not Result. The defect is latent while ClockSleepUntil is availability-gated, and becomes a reply-shape mismatch when SleepUntil is promoted. Shape each ambient reply by its checked response type"
status: draft
owner: runtime
size: S
gate: architect
tier: T2
depends_on: [RT-C5-HOST-RESPONSE-RESULT-SLOTS]
blocks: []
github: null
origin: "Architect AC-0 ruling on RT-C5-HOST-RESPONSE-RESULT-SLOTS (evt_4tde2bd6a2t4m item 5): carried to a separate node. Steward-filed per COORDINATION section 2."
---

# An ambient Unit reply has the shape its response type declares

## Objective

Every ambient host reply reaches the continuation in the shape its checked
response type declares. A `Unit` response is `Unit`, not `Ok Unit`.

## Settled inputs (Architect `evt_4tde2bd6a2t4m`, on `dc4f53614`; read, not run)

- `ambient_dispatch`'s `Unit` arm wraps every Unit reply in `Ok`.
- `clock_resp SleepUntil` is `Unit`, not a `Result`.
- `ClockSleepUntil` is availability-gated at `dispatch_host_op_v1`, so no
  run reaches the mismatch today.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The ambient Unit arm shapes its reply from the checked response type that
RT-C5-HOST-RESPONSE-RESULT-SLOTS threads (`result_reply_slots`): `Ok Unit`
only where that type is `Result e Unit`, otherwise bare `Unit`.

## Acceptance

- **AC-0.** Measure whether any run reaches the Unit arm with a non-Result
  response type at the landed base, gated or not. The Architect rules on
  release, which may wait for SleepUntil's promotion.
- **AC-1.** A row with a Unit response type receives `Unit`, and an
  (IOError, Unit) row still receives `Ok Unit`.
- **AC-2 (control).** Restoring the unconditional `Ok` wrap reddens the
  Unit row.

## Stop conditions

- Any availability-gate change, which is an operator question.
- Any edit to `clock_resp`, `ambient_resp` or `peel_coproduct`.
