---
id: RT-NATIVE-WRITEALL-SUCCESS-FOLD
title: "Green the WRITE_ALL success native full-program row (linked_checked_write_all_observes_short_progress_and_matches_interpreter, px8f_buffer_native.rs:345) — one recursive read-then-write program that reifies BOTH remaining native carried-value quarters (Wrote + ReadSome, short-progress). A fresh zero-TCB / option-(a) CONSUMER of the already-landed execute-then-resume capability that carries the success retained body (WriteProgress / ReadSome span+count) through the composed return to exit. Landing it meets RT-NATIVE-CARRIED-VALUE's four-value closure condition and re-verifies + closes PX8."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-EFFECT-CONTINUATION-WRITE-NARROWING, RT-WRITEALL-SUCCESS-PLANE-CLOSE]
blocks: []
github: null
origin: "Steward-framed 2026-09-05 from the Architect's composed-return decomposition evt_592eh20103vn9 (grounded at origin/main 463998d08). The prior carrier RT-RESULT-CONTINUATION-BINDING-PROVENANCE was closed at HS14 (evt_7gnw8s9k7rh6) — its SCOPE was refuted, NOT the objective, and the general composed-return protocol change HS14 said was required HAS ALREADY LANDED zero-TCB / backend-internal (RT-COMPOSED-RETURN-SSA-SPECIALIZATION ad9905a7e, RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE merged, RT-EFFECT-CONTINUATION-WRITE-NARROWING 3911d2861). So this is a CONSUMER of merged machinery, not a new protocol/TCB/ABI change — the operator guardrail does NOT fire (Architect Correction 1). It follows the exact zero-TCB shape of RT-NATIVE-READEOF-WITNESS-FOLD (evt_5hwps9bch4zzn) and RT-EFFECT-CONTINUATION-WRITE-NARROWING, which greened three of the arc's four native rows in the prior week."
---

> # ACTIVE — D1 RE-RELEASED (re-scoped). Lane 1 (runtime). Zero-TCB / option-(a),
> WITHIN-LANE (Architect D1 ruling evt_7hy3jk2z63bw4). tier T1, size M (D1 is now
> two sub-deliverables + a measurement), gate=none, backend-only. Base = current
> main cb6599709 (predecessor plane-close landed at 34ef1c826). RE-MEASURE every
> line coordinate at that base (the numbers below are the Architect's at
> 34ef1c826 and decay; the semantic anchors — the test name
> linked_checked_write_all_observes_short_progress_and_matches_interpreter, the
> execute-then-resume owner units.rs:3240-3462, exact_continuation_source_
> environment, the composed-return/carried-match elimination — are stable).
>
> STATE: D0 DONE (came back OPEN; the plane-close predecessor was cut + landed).
> D1 PREMISE re-scoped — the "consumer-only carry" premise was FALSIFIED (P1 has
> no executable success producer); D1 now SUPPLIES P1's producer AND consumes the
> carried result, both zero-TCB. The (ii) mechanism is PINNED = fix-(2) (Architect
> evt_5cgagfmgssexs, after the ring's measurement showed 1229 is never presented
> to exact_continuation_source_environment, so fix-(1) is OUT — it would
> re-classify a main-lowered Vis). Mechanism front-loaded by the Architect
> (evt_592eh20103vn9 + D1 ruling evt_7hy3jk2z63bw4 + fix-(2) pin evt_5cgagfmgssexs);
> the ring builds (i) the carried-BoundedNat adapter and (ii) fix-(2) (drive the
> main-lowered Vis via lower_process_host_effect + plan the recursive-join scalar
> cut), WITHOUT touching the plane classifier or admission ledger; the Architect
> reviews the candidate. The ONLY option-(b) surface is the scalar-cut residual in
> D1's contingent hard-stop — grounding predicts it does not fire.

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

## Mechanism (Architect evt_592eh20103vn9; D1 re-scoped evt_7hy3jk2z63bw4)

Reuse the landed execute-then-resume mechanism: NO new `RuntimeExpr` variant, NO
new frame field, NO side table, NO continuation-identity object. Confirmed
option-(a) / zero-TCB / within-lane by the Architect's D1 grounding at landed
34ef1c826 (evt_7hy3jk2z63bw4): the discriminator — does supplying P1's execution
require a value/handle across a cranelift fn-return boundary or a new
`ir::RuntimeExpr` variant? — is NO on both. `ir::RuntimeExpr` has 22 variants and
no `Vis` variant (ir.rs:619-729); the landed execute-then-resume owner
(units.rs:3240-3462) is proof-by-construction that FsWriteAt's success producer is
IN-FRAME realizable (perform -> call-K-with-result -> store -> return, one
cranelift function; the "continuation" is a normal declared-unit call, no identity
crosses the ABI).

D1 PREMISE CORRECTION — the frame's original "consumer-only carry of an
already-produced Wrote" is FALSE. The plane-close predecessor landed, but the
recursive WRITE_ALL success plane's P1 (vis 1229) still has NoContinuationUnit:
`matching.is_empty()` at responses.rs:1414-1418 because
`exact_continuation_source_environment` (continuations.rs:3480) returned Ok(None),
so no unit has `producer_construct_origin==1229`, so P1 never entered `demands`
(responses.rs:1459-1489), so no owner body was emitted, so its Deferred
disposition falls through to the ordinary Construct arm (core.rs:14187-14194) that
BUILDS `ITree::Vis` as inert data (tag 0) and never dispatches the write. The bind
continuation reads that inert word instead of an executed Wrote. So there is
genuinely NO produced Wrote to "carry." CORRECT SCOPE: D1 must SUPPLY P1 its
executable success producer AND consume the carried result — two sub-deliverables,
both zero-TCB, both authorized by existing landed seams (no new
protocol/frame/identity). The short-progress RECURSION (writeAll recurses twice,
3 `FsWriteAt` events) and the carried VALUE (WriteProgress / ReadSome span+count,
not an error tag) remain the two ways the success path differs from the green
error path (write-narrowing).

## Deliverables (one-hour-turn atomic increments)

- D0 — DONE (came back OPEN). Runtime-implementer measured the recursive
  read-then-write SUCCESS plane OPEN (evt_1y5gbvjm17pna): P1's lone
  NoContinuationUnit forced the classifier veto plane-wide even though the two
  exclusively-predeclared groups (267/279) were present. Outcome (a)
  zero-TCB-closeable: the Steward cut the plane-closing predecessor
  RT-WRITEALL-SUCCESS-PLANE-CLOSE (Route B classifier-narrow, Architect-approved
  evt_3p3bk579nq5r8), which LANDED (main cb6599709, code merge 34ef1c826). The
  ordinary has-K plane now closes for 267/279; P1 (1229) stays main-lowered,
  disjoint from the promoted groups (the plane-close soundness backbone).
- D1 — SUPPLY P1's EXECUTABLE SUCCESS PRODUCER + CONSUME THE CARRIED RESULT (D1
  premise re-scoped by the Architect's D1 ruling evt_7hy3jk2z63bw4; two
  sub-deliverables, both zero-TCB, reusing landed machinery — no new
  IR/frame/side-table/identity):
    - (i) CARRIED BoundedNat MATCH [origin 1236] — BUILDABLE NOW, independent of
      (ii). Route the carried `ImmediateBoundedNat` scrutinee through
      `lower_bounded_nat_match` (joins.rs:1385, which handles Nat::{Zero,Suc} on a
      native Int-class value) via a small carried-entry adapter reusing the
      EXISTING decode at units.rs:3183-3197 (mask BOUNDARY_TAG_MASK, assert
      ImmediateBoundedNat, ushr BOUNDARY_TAG_BITS). Producer = ReadSome's
      synthesized BufferSpan length; `ImmediateBoundedNat` spills to
      BoundaryClass::Int (boundary.rs:1149-1152), which is why
      `lower_carried_constructor_match`'s require_i64(Constructor) guard
      (joins.rs:913-916) rejects it today. Pure backend lowering-dispatch.
    - (ii) P1 EXECUTION [Result match origin 1064] — MECHANISM PINNED = fix-(2)
      (Architect confirmed evt_5cgagfmgssexs after the ring's measurement). The
      measurement is DONE and it CORRECTED the ruling's disambiguator premise
      (§7a): `exact_continuation_source_environment` is NEVER CALLED for 1229 (0 of
      72 calls) — the unit-construction candidate population IS the D4B admission
      ledger {606,815,1024,1406,1549,1572} and 1229 is legitimately absent (D0:
      main-lowered, not an Open/Ambiguous specialization candidate). There is no
      Ok(None) verdict to widen, so fix-(1) is DEFINITIVELY OUT, not disfavored:
      minting a unit would require ADDING 1229 to the admission ledger =
      re-classifying a main-lowered Vis as a specialization candidate = reopening
      Route B and tripping the arm-3 over-promotion the plane-close approved
      AGAINST. DO NOT add 1229 to the ledger / touch the plane classifier.
      fix-(2), verified from 34ef1c826: keep 1229 main-lowered / Deferred (P1 stays
      in phase_a.deferred, DISJOINT from the promoted 267/279 — the adversary
      soundness backbone, untouched). At the Vis consumption point, DISPATCH
      `lower_process_host_effect` (effects.rs:2225+) to perform FsWriteAt, take its
      HostResult (the Wrote), and feed it to the bind K (LexicalClosure origin
      1192) IN-FRAME via the existing carried computational-match elimination
      (core.rs:13423-13511, which already matches ITree::Ret vs ITree::Vis by tag),
      instead of the current Deferred fall-through that builds the Vis as inert
      tag-0 data (ordinary Construct arm, core.rs:14187-94). K's body recursion
      (Result -> Wrote -> count/Nat -> recursive call) is ALREADY loop-compiled by
      the landed recursor (make_computational_recursor, core.rs:13556+): no stack
      re-entry, no continuation identity crossing a fn-return, no FuncRef crossing
      (K 1192 is the in-frame fall-through of the worker, not a declared-unit
      call). REMAINING within-lane work: PLAN the scalar cut for the recursive join
      before its outer suffix (NativeJoinPlanV1; the test corpus states a rig
      supplying a planned scalar cut retires the refusing sentinel into the real
      positive, core/tests/constructors.rs:4709-4730 — landed-planning work, not a
      wall). The exact driving code site (where the inert Vis word is currently
      read) is the ring's to place; the Architect confirmed the mechanism family +
      boundary, not the site.
    - CONTINGENT HARD-STOP (gate=none unless it surfaces): the single residual risk
      is the SCALAR CUT. If, on the build, the scalar cut for this recursive join
      genuinely CANNOT be planned with the landed planning machinery —
      NativeJoinPlanV1 turns out NOT closable by planning and the join needs a NEW
      construct (contra the test-corpus statement) — that is the ONLY path to an
      option-(b) surface,
      and it is an immediate HARD-STOP to the Architect -> Steward -> operator
      (operator AWAY ~til 12:00 UTC, so the Steward QUEUES it). The dispatch
      (lower_process_host_effect) and the recursion (make_computational_recursor)
      carry no such risk; the Architect's grounding predicts the cut IS plannable.
- D2 — WITNESS + UN-IGNORE. Green the WRITE_ALL success row: exit 0, output
  "abcdef", exactly 3 `FsWriteAt` events (0,0,6)/(2,2,4)/(4,4,2), native ==
  interpreter, the retained body reaching exit rather than the
  `ResourceBodyResult` PatternMatchFailure frontier; un-ignore the row; add a
  discriminating control (a mutation that drops or mis-carries the retained body
  must red); and FIX the stale `#[ignore]` attribution (it cites the closed
  RT-RESULT-CONTINUATION-BINDING-PROVENANCE).

## Symptom inventory (§1b)

- Entry 1 (D1). The presumed single carry seam is FALSIFIED because P1 (FsWriteAt
  1229) has no executable success producer — keyed on: a main-lowered unit-less
  Vis inside a composed-return bind is fed a tag-0 placeholder (origin 1064)
  instead of executing. (Architect D1 grounding evt_7hy3jk2z63bw4, base
  34ef1c826.)

## Acceptance criteria

- AC-P1-EXECUTES. P1 (vis 1229) is supplied an executable success producer: the
  bind continuation reads an EXECUTED Wrote (a real FsWriteAt HostResult), NOT the
  inert tag-0 `ITree::Vis` constructor word from the ordinary Construct
  fall-through (core.rs:14187-14194). A discriminating control that reverts P1 to
  that inert fall-through must re-red the row.
- AC-D1-MECHANISM-FIX2. The (ii) mechanism is fix-(2) (Architect-pinned
  evt_5cgagfmgssexs; the measurement showed exact_continuation_source_environment
  is never called for 1229, so fix-(1) is rejected — it would re-classify a
  main-lowered Vis). 1229 stays main-lowered / Deferred (NOT added to the admission
  ledger, NOT re-classified); the Vis is DRIVEN via lower_process_host_effect and
  its HostResult fed to K 1192 in-frame through the carried computational-match
  elimination, with the recursive join's scalar cut PLANNED. The candidate must NOT
  touch the plane classifier or the admission ledger. The sole option-(b) surface —
  the scalar cut proving unplannable and needing a new construct — is a HARD-STOP,
  not a landing.
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
