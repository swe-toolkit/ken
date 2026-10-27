---
id: RT-P1-TAIL-CALL-EXIT-HANDLER
title: "A P1 relay source whose K exits only by static tail calls has no admitted handler, because the deferred-handler fallback requires ret_exits > 0. On px8ds's true branch owner 0 stays Excluded{Relay} and native takes the owner loop's unknown-member exit where the interpreter runs countdown. Give such sources a settling route"
status: ready
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

## Settled inputs (Architect AC-0 `evt_26t1fsd93ta56` on `a533a4dc7`)

The ruling corrects the D0 (`evt_4bfsqwt3vbr7r`). The tail callee
`countdown` (body 28) builds no `ITree` node: it runs its effects in
place and returns the checked answer. No loop is needed, and Fork A
keeps the existing single drive. The ruling holds each part verbatim;
the probe is in `local/architect-shared/rt-p1-d0/`.

- **P1, admission.** In the `responses.rs` row admission, a static tail
  call counts as a completing exit only when
  `deferred_response_tail_callees_answer_directly` holds. That walk is
  ruled but has not been compiled.
- **P2, response shape.** The unit-less drive's HostResult-only check
  uses `units.rs`'s `static_response_shape_matches`, made `pub(super)`.
- **P3, operation descent.** The drive descends nested coproduct
  injections to the operation leaf, keyed on the planner's checked
  injection identity, not on the constructor's spelling.
- **Fork B, the sink.** Frame 551's strict Ret sink is
  PendingCheckedControl, so the drive's checked answer falls to the
  closed default. A `CarriedRetSinkAuthority` parameter lets only the
  drive-fed frame install its sink under PendingCheckedControl. The
  planner and its `responses.rs:3804` consumer are not touched.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

P1 to P3 and the Fork B sink authority as ruled, with the rows below.

## Acceptance

- **AC-1.** A swapped-arm sibling of `PX8DS_SIBLING_RECURSION_PROGRAM`
  runs natively as `[BufferAllocate, ConsoleIsTerminal x4,
  ResourceRelease]`, exit 0, equal to the interpreter.
- **AC-1b.** The original program is unchanged natively:
  `[BufferAllocate, ConsoleIsTerminal, ResourceRelease]`, exit 0.
- **AC-2.** A planning pin on the original program: both P1 rows have
  `handler_owner` `Some(Specialization(0))`. RELAY's 36-row roster and
  the false-branch row are unchanged.
- **AC-3a (mutation).** Reverting the admission guard to
  `shape.ret_exits == 0` returns AC-1 to the base runtime failure:
  native exit 1, `PatternMatchFailure` on the swapped program's
  `ITree`, and one `ConsoleIsTerminal` before `ResourceRelease`
  (Architect `evt_5m46wc7vm72z2`; there was never a planning refusal).
- **AC-3b.** The walk's positive arm is carried on the real plane by
  AC-1 with the AC-3a mutant. Its negative arm is a unit row on a
  RuntimeExpr fixture shaped like the ITree-form copy, which reddens
  when the walk's `Construct` arm returns `Ok(true)`. The row's doc
  discloses that it is not the checked 659 occurrence.
- **AC-4 (mutation, QA).** Passing `Assessed` at the drive arm reddens
  AC-1 with the ITree PatternMatchFailure.

## Stop conditions

- The route needs a kernel, trust or spec change.
- Any RELAY roster row changes outcome.
- No checked coproduct-injection identity is available for P3.
