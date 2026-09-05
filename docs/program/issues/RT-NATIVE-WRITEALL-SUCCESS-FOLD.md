---
id: RT-NATIVE-WRITEALL-SUCCESS-FOLD
title: "Green the WRITE_ALL success native full-program row (linked_checked_write_all_observes_short_progress_and_matches_interpreter, px8f_buffer_native.rs:345) — one recursive read-then-write program that reifies BOTH remaining native carried-value quarters (Wrote + ReadSome, short-progress). A fresh zero-TCB / option-(a) CONSUMER of the already-landed execute-then-resume capability that carries the success retained body (WriteProgress / ReadSome span+count) through the composed return to exit. Landing it meets RT-NATIVE-CARRIED-VALUE's four-value closure condition and re-verifies + closes PX8."
status: active
owner: runtime
size: S
gate: none
tier: T1
depends_on: [RT-EFFECT-CONTINUATION-WRITE-NARROWING, RT-WRITEALL-SUCCESS-PLANE-CLOSE]
blocks: []
github: null
origin: "Steward-framed 2026-09-05 from the Architect's composed-return decomposition evt_592eh20103vn9 (grounded at origin/main 463998d08). The prior carrier RT-RESULT-CONTINUATION-BINDING-PROVENANCE was closed at HS14 (evt_7gnw8s9k7rh6) — its SCOPE was refuted, NOT the objective, and the general composed-return protocol change HS14 said was required HAS ALREADY LANDED zero-TCB / backend-internal (RT-COMPOSED-RETURN-SSA-SPECIALIZATION ad9905a7e, RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE merged, RT-EFFECT-CONTINUATION-WRITE-NARROWING 3911d2861). So this is a CONSUMER of merged machinery, not a new protocol/TCB/ABI change — the operator guardrail does NOT fire (Architect Correction 1). It follows the exact zero-TCB shape of RT-NATIVE-READEOF-WITNESS-FOLD (evt_5hwps9bch4zzn) and RT-EFFECT-CONTINUATION-WRITE-NARROWING, which greened three of the arc's four native rows in the prior week."
---

> # ACTIVE — RELEASED. Lane 1 (runtime). Zero-TCB / option-(a) consumer.
> tier T1, size S (see the D0 contingency), gate=none, backend-only. Base =
> current main 463998d08. RE-MEASURE every line coordinate at that base (the
> numbers below are the Architect's at 463998d08 and decay; the semantic anchors
> — the test name linked_checked_write_all_observes_short_progress_and_matches_
> interpreter, the execute-then-resume plane classifier, the composed-return
> exit — are stable).
>
> Mechanism front-loaded by the Architect (evt_592eh20103vn9); the ring builds,
> the Architect reviews the candidate. D0 below is a REAL contingent gate — see
> its two distinct OPEN outcomes and the hard-stop protocol.

## Objective

Green the ONE still-ignored composed-return success row:
px8f_buffer_native.rs:345, the test
`linked_checked_write_all_observes_short_progress_and_matches_interpreter`. It is
a single recursive read-then-write WRITE_ALL program that reifies BOTH remaining
native carried-value quarters — a successful Wrote and a short-progress ReadSome.
Landing it meets RT-NATIVE-CARRIED-VALUE's four-value closure condition
(SemanticErrorV1 + ReadEof already green; Wrote + ReadSome via this row) and
re-verifies + CLOSES PX8.

## Row-inventory clarity (Architect Correction 2 — carry it)

- This composed-return arc's SOLE remaining ignore is px8f_buffer_native.rs:345.
- px8f_write_partition.rs:354 is a DIFFERENT concern:
  RT-CLOSURE-BOUNDARY-RESIDUAL ("a closure cannot cross the boundary",
  boundary.rs:1044) — a closure-boundary wall, NOT the composed-return
  Wrote/ReadSome arc. Its row still cites RT-CLOSURE-BOUNDARY-RESIDUAL after its
  origin seams resolved. It is reconciled SEPARATELY (the Steward audits whether
  it is part of RT-NATIVE-CARRIED-VALUE's four-value closure or its own concern —
  the Architect suspects the latter); it is NOT in this WP's scope and this WP
  does NOT touch it.
- The `#[ignore]` string on px8f_buffer_native.rs:345 still cites the CLOSED
  RT-RESULT-CONTINUATION-BINDING-PROVENANCE — stale attribution. Fix it when the
  row greens (D2).

## Why this is zero-TCB, not a hard-stop (Architect Correction 1)

HS14 refuted the prior node's SCOPE, not the objective, and the general
composed-return protocol change it required has ALREADY LANDED, zero-TCB and
backend-internal (the option-(a) / compile-time-SSA family: SSA-SPECIALIZATION,
TRAMPOLINE-EXHAUSTIVE, and the write-narrowing execute-then-resume plane). So
greening Wrote/ReadSome is the NEXT CONSUMER of that merged capability — the same
shape as the write-narrowing (InvalidOffset error) and the ReadEof fold
(endpoint), now for a carried SUCCESS value plus recursion. It is a consumer, not
a new change: no new protocol, TCB, ABI-wire, or spec surface.

## Mechanism (Architect-ruled evt_592eh20103vn9)

Extend the landed execute-then-resume consumption to carry the SUCCESS retained
body (the WriteProgress / ReadSome span+count payload) through the composed
return to the exit, instead of stopping at the `ResourceBodyResult`
PatternMatchFailure frontier. Reuse the landed mechanism: NO new `RuntimeExpr`
variant, NO new frame field, NO side table, NO continuation-identity object.

The success path differs from the already-green error path (write-narrowing) in
exactly two ways, and D0 measures whether the landed plane already admits them:

1. short-progress RECURSION — writeAll recurses twice, so the program emits 3
   `FsWriteAt` events;
2. it carries a real VALUE (WriteProgress / ReadSome span+count), not just an
   error tag.

## Deliverables (one-hour-turn atomic increments)

- D0 — PLANE CLASSIFICATION (the one genuine residual; gates one-vs-two
  increments). Classify the recursive read-then-write SUCCESS plane under the
  landed execute-then-resume plane classifier. Execute-then-resume admits only a
  CLOSED plane with >= 2 producer groups on exclusively predeclared transport
  sources; an open plane stays conservatively Deferred. Measure whether the
  success plane is CLOSED on current main.
    - CLOSED => proceed to D1 directly; the WP is size S.
    - OPEN, closeable zero-TCB => the ring HARD-STOPS to the Steward with the
      OPEN finding; the Steward cuts one zero-TCB plane-closing PREDECESSOR node
      first (WP becomes size M), then re-releases D1. This is a within-lane
      sequencing stop, NOT an operator gate.
    - OPEN, needs a genuinely NEW protocol change beyond the landed
      execute-then-resume => HARD-STOP to the Architect -> Steward -> operator
      (option-(b)/protocol/TCB is the operator gate). Unexpected — the mechanism
      is merged and has greened 3/4 rows; evidence predicts CLOSED.
- D1 — CARRY THE SUCCESS BODY. Extend the execute-then-resume consumption to
  carry the WriteProgress / ReadSome span+count payload through the composed
  return to exit (the same shape write-narrowing used for the InvalidOffset error
  and the ReadEof fold used for the endpoint), now for a carried success value +
  recursion. Reuse the landed mechanism (no new IR/frame/side-table/identity).
- D2 — WITNESS + UN-IGNORE. Green the WRITE_ALL success row: exit 0, output
  "abcdef", exactly 3 `FsWriteAt` events (0,0,6)/(2,2,4)/(4,4,2), native ==
  interpreter, the retained body reaching exit rather than the
  `ResourceBodyResult` PatternMatchFailure frontier; un-ignore the row; add a
  discriminating control (a mutation that drops or mis-carries the retained body
  must red); and FIX the stale `#[ignore]` attribution (it cites the closed
  RT-RESULT-CONTINUATION-BINDING-PROVENANCE).

## Acceptance criteria

- AC-WRITEALL-SUCCESS-GREEN. The WRITE_ALL success row is green and co-indexed:
  it reifies BOTH Wrote and ReadSome, exit 0, output "abcdef", exactly 3
  `FsWriteAt` events (0,0,6)/(2,2,4)/(4,4,2), native == interpreter.
- AC-RETAINED-BODY-REACHES-EXIT. The success retained body (WriteProgress /
  ReadSome span+count) reaches the process exit through the composed return, NOT
  the fail-closed `ResourceBodyResult` PatternMatchFailure frontier.
- AC-CARRY-CONTROL. A non-vacuous discriminating control on the carry: a mutation
  that drops or mis-carries the retained body must red (a neutered carry that
  still passed would fail this AC).
- AC-ZERO-TCB-NO-NEW-PROTOCOL. kernel / spec / ABI / wire unchanged; kernel tree
  hash unchanged; NO new erased-IR variant, frame field, side table, or
  continuation-identity object; the increment reuses the landed execute-then-
  resume mechanism only.
- AC-D0-RECORDED. The D0 plane classification (CLOSED / OPEN, with its producer-
  group / predeclared-transport evidence) is recorded.
- AC-ATTRIBUTION-FIXED. The stale `#[ignore]` attribution on
  px8f_buffer_native.rs:345 (citing the closed RT-RESULT-CONTINUATION-BINDING-
  PROVENANCE) is corrected.
- AC-PX8-CLOSURE (note/control). On landing, RT-NATIVE-CARRIED-VALUE's four-value
  closure condition is met (all four native rows green + un-ignored) and PX8
  re-verifies + closes. px8f_write_partition.rs:354 (RT-CLOSURE-BOUNDARY-RESIDUAL)
  is a separate concern and does NOT gate this closure (Steward reconciles it
  separately).
- gate=none, backend-only, zero TCB.

## Reviewers

Builder: runtime-implementer (T1 seat; the D0 plane classification and the
retained-body carry are the reasoning content). Reviewer: Architect (front-loaded
the mechanism; reviews the plane classification, the success-body carry, the
non-vacuous carry control, zero-TCB / no-new-protocol, and the PX8-closure
note). Independent mechanics reviewer: runtime-qa. Plus CI green on the exact SHA
(native-slow lane runs the un-ignored WRITE_ALL row). Merge via Steward M1-M4 ->
lieutenant M5-M9.
