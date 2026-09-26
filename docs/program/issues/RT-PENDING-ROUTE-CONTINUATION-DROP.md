---
id: RT-PENDING-ROUTE-CONTINUATION-DROP
title: "An owner-validated pending route runs natively only when the owner runs the selected leaf's whole continuation; otherwise it is refused at compile time with its admission reason, never a silent drop and never a planner-invariant ICE"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: [RT-COMPMATCH-TREE-SCRUTINEE]
github: null
origin: "Adversary M8 on 912c44cf4 (RT-SELECTED-PENDING-CALL-BUILD increment 2), evt_2c5mcc3ktje22: F1 HIGH native drops the selected arm's bind continuation; F2 MEDIUM an admission refusal reaches the user as a compiler-bug ICE. A regression that increment 2 introduced, so it precedes further row clearing on L1. Steward-filed per COORDINATION section 2."
---

# Pending route: no dropped continuation, no refusal ICE

## Objective

Every source that increment 2's pending route admits either agrees with
the interpreter natively or is refused at compile time with a user-facing
reason. No admitted route loses host effects.

## Settled inputs -- to re-measure at the base

- **F1 (native drops work).** The landed PX7L source
  (`rt_selected_pending_call_admission.rs:4`) with only the two arms of
  `selected_body` changed:
  - `False |-> bind ... (host_console APartial (Result IOError Unit) (flush Stdout)) (\_. host_console APartial Unit (print_line "after-flush"))`
  - `True |-> host_console APartial Unit (print_line message)`

  It is admitted as `ValidatedResponseOwner` (arm 0 body 354, callee
  `StaticResponseOwner(1)`). The interpreter prints `after-flush` and its
  ops are IsTerminal, Flush, Write, Flush. Native at `912c44cf4` exits 0
  with empty stdout, ops IsTerminal, Flush, Flush, and no trap. Base
  `cd91f208c` refused the same source at ObjectEmission. The reporter's
  controls: with no runtime-selected Match it agrees, and so does the same
  leaf in the unselected arm.
- **Reported sites, unverified.** Admission is keyed on the C1 callee class
  alone (`selected_pending_calls.rs:405-408`, `:640-647`). The C2
  terminator assumes native success cannot reach it (`core.rs:14445-14450`).
  The owner's Ret check (`units.rs:3663-3677`) passes although the
  source continuation returns `Vis(print_line)`.
- **F2 (refusal becomes an ICE).** Two sources reach "planner invariant
  failed; please report this compiler bug: a refused pending call was
  delivered to an emitting frame" through production `ken native-build`:
  - PX7L with a two-print selected arm, admitted as
    `Refused(RelocatedWorkMissingLoweringBinding)`;
  - the landed px7m `ERR_PROGRAM`.

  The reported mechanism is `pending_result_validated_owner`
  (`selected_pending_calls.rs:454-456`), which turns `Refused` into a
  planner error. It is reached from `owner_fed_match_population` (`:469`),
  `core.rs:14408` and `joins.rs:2069`. The px7m err sentinel asserts only
  `result.is_err()` (`px7m_hostresult_computational_match.rs:333`), so it
  passes on the ICE.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

A repair, shaped by the Architect's D0 ruling, under which admission
accepts a pending route only when the owner provably runs the selected
leaf's whole continuation. Every refused route surfaces its admission reason
to the user.

## Acceptance

- **AC-0 (D0).** Reproduce F1 and both F2 sources at `912c44cf4` and post
  the measured ops and errors. Trace the owner/continuation mismatch to the
  point where the continuation leaves what the owner runs. Then the
  Architect rules the repair: whether the fix is in admission, in lowering,
  or both, and the population of admitted routes it changes.
- **AC-1 (pins).**
  - F1 becomes a checked-in row asserting `native == interpreted` or a named
    compile-time refusal. It must not stay green by exiting 0 with
    different output.
  - Each F2 source asserts the admission reason in the user-visible error,
    and asserts that the text is not a planner invariant. Tighten the px7m
    err sentinel the same way.
  - The landed px7l x2 and px7m ok rows stay green.
- **AC-2 (controls).** Revert the repair and the F1 row goes red. Restore
  the `Refused` to planner-error mapping and the F2 rows go red.
- **AC-3.** Targeted suites only, through `scripts/ken-cargo`: the pending-
  call rows, the px7m suite, and the new rows. No-regression means green in
  CI.

## Stop conditions

- Admission cannot decide "the owner runs the whole continuation" from
  facts the planner holds: STOP for an Architect ruling.
- A landed un-ignored row (px7l x2, px7m ok) must return to ignored.
- Any kernel, `trusted_base()` or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
