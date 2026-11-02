---
id: PX10-NATIVE-SPAWN-PARITY
title: "PX10's spawn and wait run only on the interpreter: the native bracket planner keys every region on an acquire in head position, so withChild, whose acquire follows the plan-open response match, has no return fold and no owned settlement. Give the native planner bracket regions whose acquire is not in head position, then promote the PX10 host operations to NativeTested with interpreter parity"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX10-SPAWN-PLAN-WAIT, PX10-NATIVE-SITE-IDENTITY, PX10-NATIVE-BRANCH-ALTERNATIVES]
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

## Settled inputs (measure on `58c2e1df6`, which includes PX10 slice 1)

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
- **Slice 1 pins the native refusal.** Its AC-4 row asserts that a
  native `withChild` build refuses through `RepresentedUnavailable`.
  This WP inverts that row into the AC-1 parity row; it does not delete
  it.
- **Carried from slice 1, in scope where native:** an unused Process
  grant refuses a native build, and `prepare_native_target_sources`
  reaches lowering without the availability gate. AC-0 rules whether
  each closes here or is named as a successor.
- **The interpreter trace drops two settlements** (Adversary
  evt_2ffcbyp8azkf0, on `58c2e1df6`). A pipe the `withChild` bracket
  closes is acquired but never settled: `settle_child_pipes`
  (`effect_v1.rs:1634`) discards each Ok observation, and the pipe close
  at `:2209` cannot report `ReleaseFailed`. A `SpawnPlan` retired by
  `ProcessSpawn` (`:3777`, `:1608`) is never settled. A body close and
  terminal finalization (`~:2236`) both record a settlement, so the
  outcome depends on the exit path. Parity measured against this
  baseline would agree with a native path that drops the same records.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the cut base).** Rule unfused-region
   support across the region builder and the settlement plane. Name
   each stage that reads the fused shape, give its repair, and give its
   negative.
2. **One runtime branch.** It first records exactly one settlement for
   every acquired PX10 identity on every release path, on both engines.
   It then carries the ruled repairs, promotes
   `ProcessPlanOpen` through `ProcessWait` and the two stream operations
   to `NativeTested` with their catalog columns, and adds native tests.

## Acceptance

- **AC-1 (parity).** PX10 slice 1's AC-1 rows pass natively, and
  `rt_parity_native` reports the same (request, outcome) pairs as the
  interpreter. This includes native run parity for the smoke,
  argv_env_pipe and live_child fixtures, which
  `PX10-NATIVE-BRANCH-ALTERNATIVES` brings only to native build
  (Architect evt_4qb22p4991jmm). Those fixtures issue 3, 4 and 4
  producer alternative sets, so these rows are also the labelled
  transport's execution parity (evt_573z7twxznjt9). A mutant that swaps
  two members' labels at the writer reddens one of them, and a mutant
  that writes the bare payload without its label is caught at run time
  by the reader assertion.
- **AC-1b (settlement).** For a `withChild` body that takes stdout and
  waits, the trace settles the `PipeRead` and the `SpawnPlan` exactly
  once each, before the child, in reverse acquisition order, on both
  engines. A spawn that fails settles its plan. A mutant that drops the
  bracket's pipe settlement reddens this row.
- **AC-2 (no regression).** The return-fold equivalence pin holds on
  every existing bracket fixture. `rt_native_bracket_body_once`,
  `px7f_resource_native`, `rt_escape_second_resource_native` and
  `rt_bracket_control_region_d0` stay green.
- **AC-3 (negative, QA).** A dispatch reached through a fold's `Vis`
  case, or through a `Let` value, still refuses. A mutant that accepts
  either reddens a named row.

## Increments and §1b inventory

Steward rulings evt_3yrmbfdkws1wx and evt_6kmhp6k18ypq5:

- **Increment A** is the settlement repair (R1) and AC-1b on the
  interpreter. It lands on its own.
- **Increment B** is R2, R3, the recipe rows, the promotion, the gate
  deletion and native parity. It also deletes BRANCH-ALTERNATIVES'
  AC-2g containment once its parity rows exist. It waits for
  `PX10-NATIVE-SITE-IDENTITY` and then `PX10-NATIVE-BRANCH-ALTERNATIVES`.

The §1b inventory (Architect AC-0 rounds 1 and 2, evt_1jd9djf6thfea and
evt_1jd8yzwewmtf6) has five entries:

1. W1, the returned-Vis settlement is checked against the protocol's
   root owner. Keyed on the emitting protocol; R3 closes it.
2. W1', `aggregates.rs:10581`: `host_effect_recipe_tree` gives the
   seven PX10 operations no recipe. Keyed on a second operation list;
   the recipe rows close it. This entry is independent of the others.
3. W2, `responses.rs:2030`: a finalizer emitted inside a closure cannot
   reach its selecting match. Keyed on the emitter's position.
4. W3, `responses.rs:3825`: a mixed-owner transport stage. Keyed on the
   emission unit.
5. W4, `lowering/calls.rs:1861`: a curried chain body is called from
   its emitting unit, not its static caller. Keyed on the emitting
   function.

Entries 1, 3, 4 and 5 share one predicate: identity read from the
emitting unit, not the static owner.

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
