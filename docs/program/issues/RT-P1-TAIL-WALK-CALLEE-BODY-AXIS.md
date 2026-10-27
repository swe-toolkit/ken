---
id: RT-P1-TAIL-WALK-CALLEE-BODY-AXIS
title: "RT-P1's tail walk recurses into a declaration callee from the call edge's callee_origin, which is the callee's scheduling entry, not its body. For a ComputationalMatch body the entry is the scrutinee, so a callee whose every tail builds ITree::Ret is admitted as direct-answering and its ITree word reaches the drive as the checked answer. Walk the body axis"
status: ready
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-P1-TAIL-CALL-EXIT-HANDLER]
blocks: []
github: null
origin: "Adversary M8 finding evt_390atw3v4q93h on RT-P1 squash dab59fb0a. Fail-open: it violates RT-P1's own CLAIMED line that P1 admits only direct-answering static tail callees. Serves the L1 objective. Steward-filed per COORDINATION section 2."
---

# The P1 tail walk reads the callee's body

## Objective

`tail_answers_directly` classifies a declaration callee from its body
occurrence, so a callee with an `ITree` tail is never admitted. The
body's form does not change this, including a `ComputationalMatch`.

## Settled inputs (main `aa6370449`)

- **The read.** The Call arm collects
  `edge.callee_origin()` for the declaration edges at the call site,
  and recurses from it with `inside_callee = true`
  (`planning/static_transition/responses.rs:2559-2574`). The gate is
  `deferred_response_tail_callees_answer_directly` (`:4673`).
- **The axis.** For a declaration edge, `callee_origin` is the callee
  unit's scheduling entry (`semantic_ir.rs:2364-2373`). The unit's
  body is `body_occurrence()` (`units.rs:149`). `units.rs:153-159`
  warns that the two coincide on most fixtures. A `ComputationalMatch`
  shares its entry with its scrutinee chain (`occurrences.rs:1104`).
- **The row (Adversary `evt_390atw3v4q93h`).** K is `Call(DeclarationRef
  probe, [Var 0])`, and `probe` is `LexicalClosure([x], body)` with the
  body `ComputationalMatch` on scrutinee `Value(N::Z)`, case `Z ↦
  Construct ITree::Ret`. The walk from K returns `Ok(true)`, through
  the scrutinee at origin 14. The walk from the body, at origin 9,
  returns `Ok(false)`. The same body as `Match`, `If`, `Let` or bare
  `Construct` gives `Ok(false)` from both.
- **The gap in the pins.** All four RT-P1 negative fixtures, and the
  countdown positive, have bodies whose entry equals their body.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Call arm resolves each declaration callee's body occurrence and
walks that. Add a permanent negative row in `responses.rs`'s tests for
the `ComputationalMatch`-bodied callee above, with the edge's entry
origin asserted different from the body origin in the same row.

## Acceptance

- **AC-1.** The new row's walk from K returns `Ok(false)`. The row
  asserts `entry != body` for that callee.
- **AC-2 (control).** The four `rt_p1_tail_walk_*` rows, the five
  `px8ta_oriented_subcontinuation` rows and the two
  `rt_selected_pending_call_admission` controls keep their results.
- **AC-3 (mutation, QA).** Restoring `edge.callee_origin()` as the
  recursion target reddens only the new row.

## Stop conditions

- The body occurrence is not reachable from the edge or unit at that
  site without a planner change.
- Any change to which callees the countdown positive admits.
