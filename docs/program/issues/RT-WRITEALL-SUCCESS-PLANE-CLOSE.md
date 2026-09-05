---
id: RT-WRITEALL-SUCCESS-PLANE-CLOSE
title: "Close the recursive WRITE_ALL success plane so execute-then-resume admits it: supply P1's missing continuation unit (FsWriteAt vis 1229 / producer 254) whose lone NoContinuationUnit row forces has_unitless_response and holds the whole plane OPEN, WHILE discharging the boundary-move soundness obligation (P1's path composes with the specialized 267/279 siblings at the composed-return exit, right-path selection) rather than merely flipping the plane closed. The zero-TCB / outcome-(a) plane-closing predecessor to RT-NATIVE-WRITEALL-SUCCESS-FOLD's D1. A D0 measurement selects Route B (narrow the classifier veto) vs Route A (widen P1's OPEN source env); both are backend-internal planning-only."
status: active
owner: runtime
size: S
gate: none
tier: T1
depends_on: [RT-EFFECT-CONTINUATION-WRITE-NARROWING]
blocks: [RT-NATIVE-WRITEALL-SUCCESS-FOLD]
github: null
origin: "Steward-framed 2026-09-05 from the Architect's plane-closing-predecessor decomposition evt_6ek6cws2ktfdd (grounded at origin/main 5668d363d), taken after runtime-implementer's D0 hard-stop evt_1y5gbvjm17pna reported the WRITE_ALL success plane OPEN (P1 NoContinuationUnit, FsWriteAt vis 1229 / producer 254, forcing requires_execute_then_resume=false plane-wide even though the two exclusively-predeclared producer groups 267/279 already supply ordinary_stage_count=2). Frame's contingent outcome (a): OPEN, zero-TCB-closeable, within-lane sequencing, NOT an operator gate (Architect confirmed both routes are backend-internal planning-only, no new erased-IR variant / frame field / side table / continuation-identity object, kernel/ABI/wire/spec untouched). Predecessor to RT-NATIVE-WRITEALL-SUCCESS-FOLD: on landing, runtime-leader re-releases that node's D1 success-body carry, which meets RT-NATIVE-CARRIED-VALUE's four-value closure -> PX8."
---

> # ACTIVE - RELEASED. Lane 1 (runtime). Zero-TCB / outcome-(a) plane close.
> tier T1, size S if D0 => Route B, M if D0 => Route A (see D0), gate=none,
> backend-only. Base = current main 5668d363d. RE-MEASURE every line coordinate
> at that base (the numbers below are the Architect's at origin/main and decay;
> the semantic anchors - has_unitless_response, requires_execute_then_resume,
> the WRITE_ALL classifier fixture write_all_classifies_mixed_specialized_and_
> deferred_responses, P1's Vis 1229 / producer 254, groups 267/279 - are stable).
>
> Mechanism front-loaded by the Architect (evt_6ek6cws2ktfdd); the ring builds,
> the Architect reviews the candidate WITH SPECIAL ATTENTION to AC (3), the
> boundary-move soundness obligation. This predecessor MOVES an admission
> boundary; "the plane closed" alone is exactly the vacuous pass it could
> produce, so AC (3) is the load-bearing criterion, not AC (2).

## Objective

Close the recursive read-then-write WRITE_ALL SUCCESS plane so the landed
execute-then-resume mechanism admits it, unblocking RT-NATIVE-WRITEALL-SUCCESS-
FOLD's D1 success-body carry. The plane is currently OPEN because P1 (FsWriteAt,
Vis 1229, producer 254) contributes a single NoContinuationUnit row to
static_response_deferred, and the classifier's veto
`has_unitless_response = !phase_a.deferred.is_empty()` (responses.rs:2054) then
forces `requires_execute_then_resume = !has_unitless_response &&
ordinary_stage_count >= 2` (:2108) to false plane-wide - even though the two
exclusively-predeclared producer groups 267/279 already supply
ordinary_stage_count=2. Supply P1's missing continuation unit (or narrow the
veto so P1's main-lowered residual no longer counts as plane-opening), and
DISCHARGE the boundary-move soundness obligation it exposes.

## Why this is zero-TCB / outcome (a), not (b) (Architect evt_6ek6cws2ktfdd)

Supplying P1's continuation unit needs no new erased-IR variant, frame field,
side table, or continuation-identity object, and touches no kernel/ABI/wire/
spec. Both viable routes are backend-internal planning-only changes reusing the
landed execute-then-resume mechanism + the existing classifier. The ONLY escape
to outcome (b) is if D0 surfaces that closing P1's OPEN source environment
demands a genuinely new continuation-identity / frame - which the merged entry-
frame-widening arc was explicitly built to avoid. If D0 surfaces that
(unexpected), HARD-STOP to the Steward -> operator (present); it is then an
operator gate. Evidence predicts (a) / Route B.

## Mechanism - two routes, a D0 measurement selects (both are (a))

- ROUTE B (smaller, most likely): narrow the has_unitless_response veto so a
  MAIN-LOWERED P1 does not force the plane open. P1's own row states it is main-
  lowered, not an abort ("no continuation... this WP does not specialize; main
  already lowers the Vis... so it is Deferred, not an abort", responses.rs:1398-
  1401 / :347). The classifier currently counts that main-lowered residual as a
  plane-opening defect; narrowing it to count only P1s that would be REPLACED is
  a pure predicate change. SOUNDNESS OBLIGATION (non-negotiable, AC 3): discharge
  the exact concern the veto's own comment raises - "A P1 member has no owner-
  call target; partially replacing siblings selects the wrong path"
  (responses.rs:2116-2120). SHOW P1's main-lowered path composes with the
  specialized 267/279 siblings at the composed-return exit (right-path
  selection), not merely that the plane flips closed.
- ROUTE A (heavier fallback): admit P1's recursive-position K as a real
  specialization by CLOSING its OPEN source environment - the take-loop that
  declines a non-Closed / ambiguous position (continuations.rs:3663 else-return,
  :3668 `sources.len() != 1`; census OPEN[ih-binder] at :3689). This rides the
  merged RT-CONTSRC-ENTRY-FRAME-WIDEN / RT-CAPTURE-PROJECTION-GROW machinery,
  which is identity-preserving ("extend the entry-source enumeration; never
  RELAX the membership guard") - but landed witness-inert for exactly this
  recursive-position population, so it is real work, still (a).

## Deliverables (one-hour-turn atomic increments)

- D0 - ROUTE SELECTION + (a)/not-(b) RECONFIRM (gates size and route). Run the D0
  instruments on the WRITE_ALL fixture - the D4B_ADMISSION ledger +
  WorkerPrefixDeferral - and read P1's (Vis 1229) disposition.
    - P1 appears as a specialization candidate DECLINED on an Open/Ambiguous
      position => ROUTE A (env widening); WP is size M.
    - P1 is never a candidate but is a legitimately MAIN-LOWERED residual =>
      ROUTE B (classifier narrowing); WP is size S.
    - The ONLY escape to (b): if closing P1's OPEN env demands a new
      continuation-identity / frame - HARD-STOP to the Steward -> operator
      (present). Evidence predicts (a) / Route B.
- D1 - THE SELECTED ROUTE. Route B: narrow has_unitless_response
  (responses.rs:2054 / :2108) to exclude the main-lowered P1, AND discharge the
  compose obligation. Route A: close P1's OPEN source env via the existing
  widening so P1's K admits and gets a unit. Reuse the landed mechanism (no new
  IR/frame/side-table/continuation-identity).
- D2 - PLANE-CLOSES WITNESS + CONTROL. Re-point
  write_all_classifies_mixed_specialized_and_deferred_responses
  (px8f_buffer_native.rs:657 / :731) from asserting the OPEN plane
  (deferred_kinds == {NoContinuationUnit, UnconsumedTransportCaller}) to
  asserting the plane CLOSES under Route B: P1's (254, 1229, FsWriteAt)
  NoContinuationUnit row is RETAINED as the exact populated main-lowered residual
  (NOT removed - D0 proved P1 is main-lowered, not a specialization candidate),
  but it NO LONGER vetoes the ordinary has-K plane;
  requires_execute_then_resume=true across the promoted groups 267/279; the
  ordinary stages no longer carrying UnconsumedTransportCaller - via
  with_static_response_feasibility_diagnostics.

## Acceptance criteria

- AC-P1-RETAINED (supersedes the pre-ruling AC-P1-UNIT; corrected per Route B +
  runtime-implementer evt_1qa5eng0y9g73). P1 (254, 1229, FsWriteAt) REMAINS the
  exact populated main-lowered NoContinuationUnit residual in
  static_response_deferred - it is NOT removed - and it does NOT veto the ordinary
  has-K plane. (The pre-ruling text "P1's NoContinuationUnit gone from
  static_response_deferred" was mutually exclusive with the approved narrowing,
  which retains main-lowered P1 and the specialization-owned 287 P2 while
  promoting only 267/279.)
- AC-PLANE-CLOSES. The plane CLOSES - requires_execute_then_resume=true across
  groups 267/279, observable on the feasibility-diagnostics surface.
- AC-COMPOSE-SOUND (the load-bearing obligation; Architect-refined + Route B
  APPROVED, evt_3p3bk579nq5r8). D1 already exercised this AC and it CAUGHT the
  over-promotion: naive veto-removal over-promotes the specialization-owned
  606/815 Release demands and a StaticResponseDeferred escapes its owner - the
  exact "partially replacing siblings selects the wrong path" concern
  (responses.rs:2116-2120) made concrete. The approved narrowing discharges it
  STRUCTURALLY: admit execute-then-resume only on PREDECLARED-OWNED transport
  demands once >= 2 exclusively-predeclared producer groups exist; RETAIN the
  specialization-owned mixed (287 P2) and the main-lowered P1 (1229) on main.
  Three arms, ALL required; arm 3 is what keeps it non-vacuous:
    1. PLAN/LOWERING COEXISTENCE + NO-ESCAPE: pin the exact {267, 279} predeclared
       specialization plus the {P1 1229 main-lowered, 287 P2 mixed} residual, and
       that it COMPILES with NO StaticResponseDeferred escaping its exact owner.
    2. SUPPRESSION-RESTORES (the necessity arm):
       with_suppressed_execute_then_resume_response restores 267/279 to P2 - the
       plane RE-OPENS.
    3. OVER-PROMOTION-REDS (the load-bearing discriminator - ADD THIS): a control
       that the NAIVE promotion (WITHOUT the predeclared-owned restriction) REDS -
       compile-fails / the StaticResponseDeferred escapes its owner (the exact
       over-promotion D1 measured). Without arm 3, arms 1-2 pass under any
       narrowing; with it, the plan witness is discriminating - it proves the veto
       guarded a real owner-escape and the predeclared-owned restriction is
       exactly what averts it.
  "The plane closed" alone does NOT satisfy this AC. NO synthetic runtime fixture
  is required here (the plan/lowering-coexistence witness IS the predecessor
  witness).
- RUNTIME-RIGHT-PATH-DEFERRED (named deferral, NOT a dropped AC; Architect
  evt_3p3bk579nq5r8). The runtime right-path SELECTION witness is not this
  predecessor's to produce: the real fixture returns raw -1 before FsWriteAt
  because the parent body-carry has not landed. That witness is the parent's
  px8f_buffer_native:346 going green, which by construction requires the carried
  ReadSome. It is EXPLICITLY DEFERRED to RT-NATIVE-WRITEALL-SUCCESS-FOLD (the
  parent carry WP), where the carry makes the runtime right-path real - not a
  silent gap.
- AC-ZERO-TCB. No new erased-IR variant / frame field / side table / continuation-
  identity object; kernel tree hash unchanged; ABI / spec / wire = NO; reuses the
  landed execute-then-resume mechanism + the existing classifier only.
- AC-NO-REGRESSION. The read plane, the already-green SemanticErrorV1 / ReadEof
  quarters, and the classifier's other planes are unchanged (green in CI).
- gate=none, backend-only, zero TCB.

## Reviewers

Builder: runtime-implementer (T1 seat; the D0 route selection and the compose-
soundness obligation are the reasoning content). Reviewer: Architect (front-
loaded the mechanism; reviews the route selection, the plane close, and - with
special attention - AC-COMPOSE-SOUND, the boundary-move soundness obligation,
plus zero-TCB / no-new-protocol). Independent mechanics reviewer: runtime-qa.
Plus CI green on the exact SHA. Merge via Steward M1-M4 -> lieutenant M5-M9. On
landing, runtime-leader re-releases RT-NATIVE-WRITEALL-SUCCESS-FOLD's D1.
