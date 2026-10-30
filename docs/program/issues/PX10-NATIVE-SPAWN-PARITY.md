---
id: PX10-NATIVE-SPAWN-PARITY
title: "PX10's spawn and wait run only on the interpreter: the native bracket planner keys every region on an acquire in head position, so withChild, whose acquire follows the plan-open response match, has no return fold and no owned settlement. Give the native planner bracket regions whose acquire is not in head position, then promote the PX10 host operations to NativeTested with interpreter parity"
status: draft
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX10-SPAWN-PLAN-WAIT]
blocks: [PX10]
github: null
origin: "PX10-SPAWN-PLAN-WAIT recut interpreter-first (Steward evt_15ra8z0j0typg) on the Architect's measured native wall (evt_4asjwp948dzqh). L1 objective: the Linux ABI program. This is PX10's native-parity slice; pidfd signalling is the next. Steward-filed per COORDINATION section 2."
---

# Native spawn and wait match the interpreter

## Objective

A natively compiled Ken program spawns, reads and waits for a child
through `withChild` exactly as the interpreter does. The PX10 host
operations are `NativeTested`, and the native bracket planner accepts
a bracket whose acquire comes after stuck effects.

## Settled inputs (measure on the cut base, which includes PX10 slice 1)

- **The wall.** `build_bracket_static_regions` takes the positional
  return fold as `folds.last()` under the acquire's continuation
  (`brackets.rs:583` on `903b06068`). For `withResource` the acquire is
  the head `Vis`, so the caller's fold fuses into it. `withChild`'s
  acquire follows the plan-open response match, which is stuck on a
  neutral response, so the fold stays an ancestor (Architect probe B,
  evt_4asjwp948dzqh).
- **The first link is measured.** The Architect's `return_fold` up-walk
  selected the same fold as the downward search on all 108 existing
  region builds in four suites, and found the fold for the PX10 smoke.
- **The next wall is measured.** With the up-walk applied,
  `returned_vis.rs:491` refuses the smoke: the settlement's owner is the
  last `push_stdio` `Vis`, outside the region.
- **The predicate.** The planner keys region structure on lexical
  containment under the acquire's own continuation. Every stage that
  reads that shape is in scope: the return fold, settlement ownership,
  body exits, and the parent forest.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the cut base).** Rule unfused-region
   support across the region builder and the settlement plane. Name
   each stage that reads the fused shape, give its repair, and give its
   negative.
2. **One runtime branch.** It carries the ruled repairs, promotes
   `ProcessPlanOpen` through `ProcessWait` and the two stream operations
   to `NativeTested` with their catalog columns, and adds native tests.

## Acceptance

- **AC-1 (parity).** PX10 slice 1's AC-1 rows pass natively, and
  `rt_parity_native` reports the same (request, outcome) pairs as the
  interpreter.
- **AC-2 (no regression).** The return-fold equivalence pin holds on
  every existing bracket fixture. `rt_native_bracket_body_once`,
  `px7f_resource_native`, `rt_escape_second_resource_native` and
  `rt_bracket_control_region_d0` stay green.
- **AC-3 (negative, QA).** A dispatch reached through a fold's `Vis`
  case, or through a `Let` value, still refuses. A mutant that accepts
  either reddens a named row.

## Stop conditions

- A repair that needs a kernel or trust change, or a post-fork callback.
- A third advancing stop on the native-lowering chain. Its §1a count is
  2 at the recut, so a third triggers the Research hold.

Checks: 4 fired: the fold search and the settlement refusal were read at
`brackets.rs:583` and `returned_vis.rs:491` on `903b06068`. The up-walk's
equivalence and the next wall are the Architect's measurements, not
inferred. 8 fired: AC-1 compares engine traces pair by pair, so a native
path that settles in a different order fails, and AC-3's mutant shows
that the refusal is not lost.
