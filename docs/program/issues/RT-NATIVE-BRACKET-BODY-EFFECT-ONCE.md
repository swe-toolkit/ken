---
id: RT-NATIVE-BRACKET-BODY-EFFECT-ONCE
title: "The native backend re-performs body effects inside any checked bracket: a withResource or withBuffer body that prints x then y prints x y y natively and x y in the interpreter, and a third print gives x y z y z z. A native route both performs a checked Vis in place and returns it as a construct that the enclosing route performs again. Make each checked Vis occurrence perform exactly once per dynamic arrival, in program order, and measure the px8ta LIFO row on the repaired base"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-17: \"do what you need to do to reactivate the remaining ignored tests\". Architect route ruling evt_3tpc0zkz2m1z3 (route (c), on deb570dad): the px8ta row's blockers sit on this wider miscompile, which neither the bracket-region IR nor RELEASE-ORDER-PARITY reaches. Steward-filed per COORDINATION section 2."
---

# A bracket body's effects happen once, in order

## Objective

`ken native-build` of a program whose bracket body performs two or more
effects produces the interpreter's host-effect trace. The last product
ignored test,
`px8ta_oriented_subcontinuation.rs::public_two_three_level_brackets_finish_and_release_lifo`,
is measured on the repaired base and un-ignored here if both depths pass.

## Settled inputs (Architect `evt_3tpc0zkz2m1z3`, on `deb570dad`)

- **The reproducer.** `min.ken` (in the ruling, sha256 prefix
  `3b74395944789c67`) prints x then y inside `withResource`. `ken run`
  prints `x y`, and the native binary prints `x y y`.
  - A third print gives `x y z y z z`: effect i runs i times, which is a
    suffix replay.
  - `withBuffer` duplicates the same way.
  - The same two prints without a bracket are correct.
  - A bracket-free lookalike (print o1, body, print c1) inside a real
    `withResource` gives `o1 c1 c1`.
- **The population.** Four of four bracket bodies with two or more
  sequential effects duplicate, and none of the eight controls do. Every
  failing row carries a Deferred `ContinuationBodyTail` ConsoleWrite row
  (min.ken: vis 441 on call 77).
- **Attribution.** Under lldb the two `y` writes reach `write(1)` through
  distinct emitted call sites. This is two routes each performing one
  Vis, not one continuation called twice.
- **The defect, as read.** A native route decides per row whether a
  checked Vis is performed in place or returned as a construct. Nothing
  makes those choices exclusive and ordered across the enclosing route.
  The link to px8ta's depth-2 order and depth-3 `ContinuationSpecialization`
  refusal is the Architect's reading, not a measurement.
- **Not this route.** `deferred_response_handler_owner` returns None for
  `ContinuationBodyTail` (responses.rs:4328) before either PARITY arm is
  consulted, and the region IR leaves depth-1 body work on the existing
  route.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0, scratch, measurement only; the Architect reviews it before the
   repair.**
   - Add a test-support emission ledger. Key each emitted host-dispatch
     call on (vis origin, planner disposition, Deferred sub-case or
     pending-Vis member, emitting `ContinuationEmissionOwner`, `FuncId`).
   - Add a per-site test-support suppression switch.
   - Fixtures: F1 min.ken, F2 the three-print row, F3 withBuffer.
   - Controls: the no-bracket pair, the single-print, nested-bind and
     lookalike-release rows, and px8ta depths 1, 2 and 3.
   - Report the emitting route for every executed performance. Report the
     suppression that removes exactly the extras, with a positive
     application count, while its unmutated sibling stays duplicated.
   - Prediction to refute: the extras come from the enclosing owner's
     returned-Vis route (the pending-Vis loop or an ordinary Construct)
     re-performing a `ContinuationBodyTail` Vis its worker already
     performed.
2. **The repair, in `crates/ken-runtime`.** Each checked Vis occurrence is
   performed by exactly one route per dynamic arrival, in program order. A
   construct either performs in place or returns unperformed, never both.

## Acceptance

- **AC-1.** The pre-registered observation tuple compares equal between
  the interpreter and the linked native binary. The tuple is exit,
  terminal, ordered host-effect trace, acquisition-ordinal resource
  identities and outcome class. It is checked on:
  - F1, F2 and F3, and every control;
  - the umbrella `RT-BRACKET-CONTROL-REGION-IR`'s four plus two Mapping
    rows, and O0/W0.

  Ledger closure is not acceptance, because it reads the same on the
  broken tree.
- **AC-2 (mutation, QA).** Restoring the double route reddens F2 with a
  positive count.
- **AC-3.** px8ta depths 2 and 3 are measured and reported, but not gated.
  If both pass, remove the row's `#[ignore]` in this candidate. If either
  stays red, the bracket-region IR resumes on the repaired base for the
  remainder, with the `o1 c1 c1` lookalike added to its D0 matrix.

## Stop conditions

- **D0:** the extra performances do not map to one route kind. Stop to the
  Architect.
- Any kernel, `trusted_base()` or spec change (an operator question).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or `wp/RT-BRACKET-SETTLEMENT-PLANE`.
