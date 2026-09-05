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

> # ACTIVE — D1(ii) RE-SCOPED to the §1b STRUCTURAL RECUT (Architect recut ruling
> evt_4vzw61rv1dn4h; §1a research advisory evt_62pn3g614cbrj). Lane 1 (runtime).
> WITHIN-LANE pending GATE 0 + the recut-B axis-B check; tier T1, size M,
> gate=none, backend-only. Base = current main (predecessor plane-close landed
> 34ef1c826). RE-MEASURE every line coordinate at that base; the semantic anchors —
> the test name linked_checked_write_all_observes_short_progress_and_matches_
> interpreter, the execute-then-resume owner units.rs:3240-3462, the
> WRITE-CONTINUATION SUBTREE (the write + its existing Ret-wrapper body 1374 =
> Construct{ITree::Ret,[Var(0)]} + the recursive short-write loop), the promoted
> read owner 1549 — are stable.
>
> STATE: D0 DONE (came back OPEN; the plane-close predecessor was cut + landed).
> D1(i) carried-BoundedNat adapter = WIP 0a6d564f9 (committed, kept). D1(ii) went
> through THREE hard-stops — HS#1 (the main-lowered write has no execution unit),
> HS#2 (the effect tail can't be Ret-wrapped for owner 1549), HS#3 (the bounded
> (beta) planner ownership-edge did NOT close: it needed authority-after-authority —
> claim edge, then current-lexical allowance, then a settlement flip, then a
> retained lexical-call target import). The §1b defect, research-confirmed, is an
> OWNERSHIP INVERSION: importing the handler's machinery (unit / wrap / call-target /
> settlement / lexical availability) into the write's scope one authority at a time,
> i.e. building the handler's continuation inside the effect's scope. The §1a
> research advisory + the Architect recut ruling DISCHARGE the enumeration and
> INSTALL the structural closure: the write-continuation subtree must have ONE static
> owner (the receiving handler) that returns the effect result to itself via static
> call-graph/ownership structure. Prior art (evidence-passing / handler-owns-
> continuation) achieves this STATICALLY for the SINGLE-SHOT / tail-resumptive class;
> B2F (a durable first-class closure) is NOT known-necessary here and stays RESERVED
> for the general multi-shot/non-local case only. The superseded fix-(2) (drive the
> Vis via lower_process_host_effect + a scalar cut) and the (beta) ownership-edge
> probe are BOTH RETIRED — the effect DISPATCH they proved within-lane (exact 3
> writes) is REUSED, but the piecemeal authority chain is the wrong shape; a 4th
> authority is not the move. GATE 0 (structural single-shot measurement) is the
> MANDATORY FIRST deliverable and DECIDES within-lane vs the operator fork. The
> Architect reviews the recut candidate self-contained.

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

## Mechanism (Architect evt_592eh20103vn9; D1 re-scoped evt_7hy3jk2z63bw4; D1(ii) structural recut evt_4vzw61rv1dn4h + advisory evt_62pn3g614cbrj)

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

D1(ii) REALIZATION — the §1b STRUCTURAL RECUT (detailed in D1(ii) below). The
premise correction stands (P1 has no produced Wrote to carry), but HOW (ii)
supplies + returns it is no longer fix-(2)'s piecemeal drive. After three
hard-stops the diagnosed defect is an OWNERSHIP INVERSION (building the handler's
continuation inside the effect's scope), and the research-confirmed closure is a
SINGLE STATIC OWNER of the write-continuation subtree that returns the effect
result to itself via static call-graph/ownership structure — not authority-by-
authority import. The effect DISPATCH fix-(2)/the (beta) probe proved within-lane
(`lower_process_host_effect` -> exact 3 writes (0,0,6)/(2,2,4)/(4,4,2)) is REUSED
as the write's execution; what CHANGES is WHO OWNS the continuation that returns
its result to the promoted owner 1549. B2F stays reserved for the general
(multi-shot/non-local) case; GATE 0 (below) measures that this program is not that
case.

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
    - (ii) RETURN P1's RESULT TO ITS OWNER — the §1b STRUCTURAL RECUT (Architect
      recut ruling evt_4vzw61rv1dn4h; advisory evt_62pn3g614cbrj). The write's
      execution is proven — REUSE the dispatch (`lower_process_host_effect`,
      effects.rs:2225+ -> exact 3 writes) — and 1229 stays main-lowered / Deferred,
      NOT added to the admission ledger, NOT re-classified (adding it reopens Route B
      and trips the arm-3 over-promotion the plane-close approved AGAINST). The open
      problem is RETURNING that effect result to the promoted execute-then-resume
      owner (vis 1549) as the owner's exact ITree::Ret WITHOUT importing the owner's
      machinery piecemeal. The three hard-stops were an OWNERSHIP INVERSION (building
      the handler's continuation inside the effect's scope, one authority at a time);
      the closure is a SINGLE STATIC OWNER of the write-continuation subtree that
      returns the result to itself via static structure. Build in this order:
        - GATE 0 (MANDATORY, FIRST — decides within-lane vs the operator fork).
          MEASURE STRUCTURALLY that this writeAll write-then-resume is genuinely
          SINGLE-SHOT / tail-resumptive: the continuation resumes EXACTLY ONCE per
          write, is not multi-shot, not non-local, and does not outlive the handler.
          Measure it from the continuation STRUCTURE — do NOT read it off the
          RecursiveBackedge marker (the inc3 gate; re-confirm here rather than
          inherit inc3's read-then-write result). If FALSIFIED (multi-shot /
          non-local / continuation-outlives-handler surfaces), a durable first-class
          continuation carrier (B2F) becomes KNOWN-NECESSARY and THIS is the operator
          fork -> immediate HARD-STOP to the Architect -> Steward -> operator (the
          Steward QUEUES; operator AWAY ~til 12:00 UTC). Absent that, proceed
          within-lane. Static single-owner ownership is prior-art-sound ONLY for this
          class; this measurement is why it is legitimate here.
        - recut-A (TARGET — pure decomposition, the Architect's choice). Give the
          write-continuation subtree (the write + its existing Ret-wrapper body 1374
          = Construct{ITree::Ret,[Var(0)]} + the recursive short-write loop) a SINGLE
          STATIC OWNER — the receiving handler — so the write's Ret is produced on
          that owner's OWN route, not as a main-lowered tail whose owner-machinery
          must be imported. This is the direct realization of the prior-art closure
          (the delimiter owns its continuation; the effect returns to it) and is a
          PURE DECOMPOSITION — zero new construct -> WITHIN-LANE (inc3 axis-B: a
          larger build on existing machinery is sizing, not a new construct). Reuse
          the proven dispatch; do NOT reintroduce piecemeal authority imports; do NOT
          invent a durable closure lane. Feasibility to establish: the subtree can be
          given ONE static owner by decomposition while PRESERVING the guardrail
          backbone below, with no new construct. If yes -> build recut-A.
        - recut-B (FALLBACK — ONLY IF recut-A is structurally unavailable). A single
          continuation-ownership-transfer authority that moves the whole
          statically-known write-continuation subtree to the receiving owner in ONE
          authority. GATED on the inc3 axis-B check BEFORE it counts as within-lane:
          realizable with the EXISTING unit-call ABI (no new planner primitive /
          ir::RuntimeExpr variant / frame surface, even backend-internal) =
          within-lane; requires ANY new construct = beyond funded reuse = immediate
          HARD-STOP to the Architect -> Steward -> operator. recut-A is preferred
          precisely because it avoids the axis-B question.
    - CONTINGENT HARD-STOP (gate=none unless it surfaces) = the ruled operator fork,
      fires on EITHER: (GATE 0) the single-shot measurement is FALSIFIED (multi-shot
      / non-local / outlives-handler) so B2F is known-necessary; OR (recut) recut-A
      is structurally unavailable AND recut-B needs a NEW construct. Either ->
      immediate HARD-STOP to the Architect -> Steward -> operator, with the fork
      STATED: fund B2F (a durable first-class continuation carrier) OR fund a
      sanctioned continuation-ownership-model extension OR a structural recut of the
      write/read plane decomposition. The Steward QUEUES it (operator AWAY ~til
      12:00 UTC). Grounding + the advisory predict it does NOT fire: prior art
      achieves this class statically (evidence-passing / handler-owns-continuation)
      and recut-A is a pure decomposition. §1a: this recut chain triggers a fresh
      research prior-art pull if it reaches HS#6 (Architect flag).
    - GUARDRAILS (whatever the recut): PRESERVE the landed plane-close soundness
      backbone — promoted 267/279 stay DISJOINT from P1, NO StaticResponseDeferred
      escapes its owner, the arm-3 over-promotion discriminator STILL REDS; keep
      P1/1229's execution behavior (1229 stays main-lowered / Deferred, NOT added to
      the admission ledger, NOT re-classified); no B2F, no new erased-IR / frame /
      ABI / kernel / spec surface; reuse the proven effect dispatch.
- D2 — WITNESS + UN-IGNORE. Green the WRITE_ALL success row: exit 0, output
  "abcdef", exactly 3 `FsWriteAt` events (0,0,6)/(2,2,4)/(4,4,2), native ==
  interpreter, the retained body reaching exit rather than the
  `ResourceBodyResult` PatternMatchFailure frontier; un-ignore the row; add a
  discriminating control (a mutation that drops or mis-carries the retained body
  must red); and FIX the stale `#[ignore]` attribution (it cites the closed
  RT-RESULT-CONTINUATION-BINDING-PROVENANCE).

## Symptom inventory (§1b)

- Entry 1 (HS#1). The presumed single carry seam is FALSIFIED because P1 (FsWriteAt
  1229) has no executable success producer — keyed on: a main-lowered unit-less
  Vis inside a composed-return bind is fed a tag-0 placeholder (origin 1064)
  instead of executing (unit synthesis is keyed on admission-ledger membership; the
  write isn't in it). (Architect D1 grounding evt_7hy3jk2z63bw4, base 34ef1c826.)
- Entry 2 (HS#2). The write's effect tail can't be Ret-wrapped for owner 1549 — the
  wrap seats are keyed on route purity and the effect tail is in the reject-set
  (checked_ih_body_is_pure_narrowing rejects Effect and Construct(::ITree::Vis); the
  forward-Ret-edge collapse deliberately excludes the effect tail). A bare Result
  reaches an owner expecting ITree::Ret. (Architect HS#2 evt_5s3remfbqmhy3.)
- Entry 3 (HS#3). Driving the write's Ret-wrapper continuation (target 0, body 1374
  = Construct{ITree::Ret,[Var(0)]}) into 1549 needs authority AFTER authority —
  continuation-claim edge, then current-lexical allowance for Predeclared(14), then
  a settlement flip InlineNoCall->DirectCall, and then a retained lexical-call
  target import (origin 1321) — each keyed on which SPECIALIZATION-OWNER drives.
  (ring evt_6fmjknge3r4ap; Architect §1b evt_13y1z3038392e.)
- SHARED PREDICATE (all three; the defect itself, research-confirmed). The write's
  success path is split across TWO ownership contexts — the write's own main-lowered
  / Deferred context (producer 1406 / Predeclared(14) / Specialization(2)) versus
  the generated claimant that must drive it into the promoted read owner
  (1549 / Specialization(4)). Every hard-stop is one more piece of the promoted
  owner's machinery (unit / wrap / call-target / settlement / lexical availability)
  that does NOT cross that owner boundary, imported piecemeal — the fleet's
  "a dynamic property (which owner drives) naming/gating STATIC machinery" pattern
  (the RT-NATIVE-FNSPLIT shape) one level up, at the ownership decomposition. It is
  an OWNERSHIP INVERSION (building the handler's continuation inside the effect's
  scope); in delimited-continuation terms the continuation-up-to-the-handler is
  owned by the DELIMITER, never assembled at the effect site. The structural closure
  is single-owner static ownership of the whole write-continuation subtree (recut-A
  by decomposition; recut-B by a transfer authority). This is NOT yet the §1b
  entry-3 STRUCTURAL-RECUT-to-operator trigger — it is a WITHIN-LANE recut pending
  GATE 0 + the axis-B check; the operator fork is reserved for (single-shot
  falsified) OR (recut-A infeasible AND recut-B needs a new construct).

## Acceptance criteria

- AC-P1-EXECUTES. P1 (vis 1229) is supplied an executable success producer: the
  bind continuation reads an EXECUTED Wrote (a real FsWriteAt HostResult), NOT the
  inert tag-0 `ITree::Vis` constructor word from the ordinary Construct
  fall-through (core.rs:14187-14194). A discriminating control that reverts P1 to
  that inert fall-through must re-red the row.
- AC-GATE0-SINGLE-SHOT. GATE 0 is recorded FIRST: a STRUCTURAL measurement (NOT the
  RecursiveBackedge marker) establishing this write-then-resume is single-shot /
  tail-resumptive (resumes exactly once per write, not multi-shot, not non-local,
  does not outlive the handler). A falsified measurement is NOT a landing — it is
  the operator-fork HARD-STOP (B2F known-necessary).
- AC-D1-STRUCTURAL-CLOSURE. The (ii) realization installs SINGLE-OWNER STATIC
  OWNERSHIP of the write-continuation subtree (recut-A by decomposition, the target;
  recut-B by a single ownership-transfer authority only if recut-A is structurally
  unavailable AND it passes the axis-B check = existing unit-call ABI, no new
  construct). The write's Ret reaches owner 1549 by static call-graph/ownership
  structure, NOT by importing the owner's unit / wrap / call-target / settlement /
  lexical machinery piecemeal into the write's scope. The proven effect dispatch is
  reused; 1229 stays main-lowered / Deferred, NOT added to the admission ledger, NOT
  re-classified; the candidate does NOT touch the plane classifier. A recut that
  needs a new construct (B2F / new IR / frame / ABI / kernel / spec) is a HARD-STOP,
  not a landing.
- AC-BACKBONE-PRESERVED. The landed plane-close soundness backbone holds: promoted
  267/279 stay DISJOINT from P1, NO StaticResponseDeferred escapes its owner, and
  the arm-3 over-promotion discriminator STILL REDS. A candidate that reopens any of
  these fails this AC.
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
