---
id: RT-P1-TAIL-CALL-EXIT-HANDLER
title: "A P1 relay source whose K exits only by static tail calls has no admitted handler, because the deferred-handler fallback requires ret_exits > 0. On px8ds's true branch owner 0 stays Excluded{Relay} and native takes the owner loop's unknown-member exit where the interpreter runs countdown. Give such sources a settling route"
status: draft
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-PX8DS-SAME-DEPTH-P1-HANDLER]
blocks: []
github: null
origin: "Residual named by the Architect's RT-PX8DS-SAME-DEPTH-P1-HANDLER D0 (evt_1jp26fw1z8cw9, section 'Residual, named, not this route'). Serves the L1 objective, native parity with the interpreter. Steward-filed per COORDINATION section 2."
---

# P1 sources that exit by tail call settle natively

## Objective

The true branch of the px8ds same-depth fixture runs natively as on the
interpreter: relay 370's P1 sources settle, and owner 0 is admitted.

## Settled inputs (D0 `evt_1jp26fw1z8cw9` on `da506265f`)

- Relay 370's sources 647 and 600 are `ConsoleIsTerminal` Vis (effect
  origin 86, sub-case `NoContinuationUnit`, P1). Their K bodies (634,
  586) have `response_uses = 1`, `ret_exits = 0` and two static tail
  calls (`countdown rest`).
- Every RELAY clause fails for both. The nearest, R2's
  `NoContinuationUnit` arm, fails only on the `shape.ret_exits > 0`
  condition of the fallback in `deferred_response_handler_owner`.
- Native behaviour on that branch is read, not run: the owner loop's
  unknown-member exit returns `-1`. The harness's stdout is not a
  terminal, so no current row observes it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules whether tail-call exits can stand for a
Ret exit in the handler fallback, and how a row observes the true
branch. Then the route and that row.

## Acceptance

- **AC-0 (D0).** The ruled clause and the observing row.
- **AC-1.** The true-branch row passes natively with the interpreter's
  trace.
- **AC-2.** RELAY's 36-row roster and the px8ds false-branch row are
  unchanged.
- **AC-3 (mutation, QA).** Restoring `ret_exits > 0` reddens the new
  row with a typed refusal, not a run-time trap.

## Stop conditions

- The route needs a kernel, trust or spec change.
- Any RELAY roster row changes outcome.
