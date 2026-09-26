---
id: RT-ERASURE-SELECTED-HOST-BINDER-DEPTH
title: "Erasure's runtime-selected host dispatch under-shifts the Clock and Entropy leaf continuations by one binder: clock_or_entropy is a third synthesized Match case binder, but the leaves are shifted as if under two, so a raw outer Var in their continuations reads one slot off; witness it on checked source, then fix the shift budget so every leaf's shift equals its real case depth"
status: merged
owner: runtime
size: S
gate: architect
tier: T1
depends_on: []
blocks: [RT-SELECTED-PENDING-CALL-BUILD]
github: null
origin: "RT-SELECTED-PENDING-CALL-BUILD increment 1, binder-identity stop: Architect evt_3yce3vzbddvk0 ordered one bounded attribution and a separate item; runtime-implementer attribution evt_2qn7nffaqxjyy. Increment 2 is held until this lands. Serves the L1 objective. Steward-filed per COORDINATION section 2."
---

# Selected host dispatch shifts Clock and Entropy by one too few

## Objective

In erasure's runtime-selected host dispatch, each leaf continuation is
shifted by the exact number of case binders the synthesized `Match` spine
places above it. So a raw `Var` in the continuation reads the same value the
source binding names.

## Settled inputs -- measured at `8ad968186`

- **The spine.** `crates/ken-elaborator/src/erasure.rs`,
  `lower_runtime_selected_host_operation`, builds three nested synthesized
  `Match`es, each with one-binder cases: HostIO (`fs` or ambient), then
  ambient (`console` or `clock_or_entropy`), then `clock_or_entropy`
  (`clock` or `entropy`, about `:4010`).
- **The leaf shifts** (about `:4001-4004`) are `leaf_dispatch(fs, .., 1)`,
  `console .., 2`, `clock .., 2` and `entropy .., 2`. `leaf_dispatch` shifts
  the pre-lowered continuation by `enclosing_binders + argument_shift`
  (about `:3984`).
- **Real depths** are `fs` 1, `console` 2, `clock` 3, `entropy` 3. So the
  Clock and Entropy continuations are under-shifted by one.
  `clock_or_entropy` is synthesized with no `BranchBinderRemap::enter_match`
  group.
- **Measured instance** (`evt_2qn7nffaqxjyy`): on px7l the pending IH is at
  runtime `Var(4)` by the IR's binding structure at marker origin 47, while
  the erasure-minted morphism and the callee `Var` say 3. The marker is
  bound by template and is unaffected. A raw outer `Var` under that binder
  is not.
- **Not yet measured:** whether any landed program executes a wrong read.
  The Runtime IR evaluator refuses `ComputationalMatch`, and native emission
  reads raw `Var`s.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Correct the shift budget so each leaf's shift equals its real case depth.
The Architect rules the form at D0: the leaf argument, or an `enter_match`
group for `clock_or_entropy`. Fix it at the spine, not at a consumer.

## Acceptance

- **AC-0 (D0, before the fix).** Post the leaf-by-leaf depth table (spine
  depth against shift passed) and the Architect's choice of form.
- **AC-1 (witness, red on base).** A checked-source program in which a
  Clock-leaf continuation, and separately an Entropy-leaf continuation,
  reads an outer variable distinguishable from its neighbour slot. Reach it
  through native emission.
  - It is red on base and green after the fix.
  - If no checked-source program can reach a raw outer read today, report
    that as a measurement. A unit fixture over the erased `RuntimeExpr` then
    asserts that each leaf's free-`Var` resolution matches its spine depth,
    with its positive control.
- **AC-2 (controls).** `fs` and `console` leaves come out byte-identical.
  Reverting only the shift change reddens AC-1. px7l's recorded pair becomes
  (4, 4). Targeted builds only, through `scripts/ken-cargo`; no-regression
  means green in CI.

## Stop conditions

- The fix needs a change outside the selected-host spine, or changes the
  `fs` or `console` leaves.
- Any kernel or `trusted_base()` change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## After landing

`RT-SELECTED-PENDING-CALL-BUILD` increment 2 is released
(`evt_3yce3vzbddvk0`).
