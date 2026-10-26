---
id: RT-P1-TAIL-CALL-EXIT-HANDLER
title: "A P1 relay source whose K exits only by static tail calls has no admitted handler, because the deferred-handler fallback requires ret_exits > 0. On px8ds's true branch owner 0 stays Excluded{Relay} and native takes the owner loop's unknown-member exit where the interpreter runs countdown. Give such sources a settling route"
status: ready
owner: runtime
size: L
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

## Settled inputs (Architect D0 `evt_4bfsqwt3vbr7r` on `a533a4dc7`)

The D0 stopped at two design forks. It found the route has four parts,
not the original frame's one clause. The probe test, the patch and the
logs are in `local/architect-shared/rt-p1-d0/`.

- **The rows.** Relay 370's sources are 609 and 656 (K bodies 595, 643):
  P1 `NoContinuationUnit` `ConsoleIsTerminal`, `ret_exits` 0, two static
  tail calls, `handler_owner` None.
- **P1, admission (measured).** In `deferred_response_handler_owner`,
  `ret_exits + static_tail_calls > 0` replaces `ret_exits > 0`. A K
  whose every exit traps stays refused. Owner 0 is admitted, and RELAY
  admits it.
- **P2, response shape (measured).** The unit-less drive's
  HostResult-only check (`core.rs:5826`) uses `units.rs`'s
  `static_response_shape_matches` instead.
- **P3, operation descent (measured, latent).** The unit-less drive
  binds the coproduct's selected field, so a nested ambient operation
  such as `InL(ConsoleOp::IsTerminal(Stdout))` binds the wrong value.
  It descends nested injections with tag and arity guards.
- **P4, iterating drive.** K's exits return a fresh Vis, so the drive
  loops. Its soundness premise is open (Fork A).
- **Fork A (open, the Architect's).** All four tail calls target
  countdown's canonical body 28, which has no Deferred row. The rows
  live in caller-inlined copies, fn 3 and fn 5. The loop needs either a
  copy-provenance link from K's tail-call target to the row, or Deferred
  rows planned for the canonical body.
- **Fork B (open, measured after Fork A).** With P1 to P4 applied, the
  second site's drive (vis 605, call 620) is emitted but never entered,
  and native stops at three `ConsoleIsTerminal` events and a
  `PatternMatchFailure`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Architect first rules Fork A and measures Fork B on that design.
Then P1 to P4 as ruled, with the two observing rows.

## Acceptance

- **AC-0 (D0, Architect).** The Fork A ruling, and Fork B's cause under
  it with its repair.
- **AC-1.** A swapped-arm sibling of `PX8DS_SIBLING_RECURSION_PROGRAM`
  runs natively under the harness pipe as
  `[BufferAllocate, ConsoleIsTerminal x4, ResourceRelease]`, exit 0,
  equal to the interpreter's trace.
- **AC-2.** A planning pin on the original program: both P1 rows have
  `handler_owner` `Some(Specialization(0))`, via
  `with_static_response_feasibility_diagnostics`. RELAY's 36-row roster
  and the px8ds false-branch row are unchanged.
- **AC-3 (mutation, QA).** Restoring `ret_exits > 0` makes the AC-2 pin
  fail on the planner plane: `handler_owner` None, owner 0 not admitted.

## Stop conditions

- The route needs a kernel, trust or spec change.
- Any RELAY roster row changes outcome.
- Fork B's repair is not local to the drive and its continuation.
