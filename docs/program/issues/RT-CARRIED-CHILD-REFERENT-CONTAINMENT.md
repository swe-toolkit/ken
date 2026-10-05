---
id: RT-CARRIED-CHILD-REFERENT-CONTAINMENT
title: "The planner records an aggregate child under a Persistent parent as NativeScalarPair with no referent, while lowering produces the same child Carried, and every static containment check skips Carried operands. Find the route and the store path, establish whether a short-lived referent can reach a persistent parent, and repair the planner so the plan and the lowering agree"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [RT-NAT-FANOUT-DETACHED-MULTI-MEMBER]
github: null
origin: "Architect ruling evt_5r92yzz8zdd32 on the RT-NAT-FANOUT AC-0 stop evt_5hhn8gd87hxyj: a candidate soundness defect on main, independent of R6. Steward-filed per COORDINATION section 2."
---

# A Carried child is contained by its parent

## Objective

Every aggregate child that lowering produces Carried is planned with its
referent, so the lowering-side containment check applies to it, and no
persistent parent stores a word whose referent dies first.

## Settled inputs (Architect `evt_5r92yzz8zdd32`, on `4177e68de`)

- **Two witnesses** in the passing, artifact-producing row
  `r2_relay_owner_is_excluded_from_pending_vis_protocol`. In each case the
  same expression pointer is planned one way and lowered another:
  - Match781 is field 0 of Construct782 (Predeclared(8), `Persistent`). It
    is planned `NativeScalarPair -> [NoReferent]` at
    `planning/static_transition/aggregates.rs:3248`, and lowered Carried in
    funcid61/Specialization(3).
  - Match1030 is field 0 of Construct1031 (Predeclared(5)). It is lowered
    Carried in funcid63/Specialization(1).
- **The static guard is skipped.** The containment check in
  `lowering/aggregates.rs:1346-1390` (held owners within planned owners,
  never shorter-lived) is reached only through
  `source_aggregate_preflight`. Its callers at `:1015`, `:1169`, `:1194` and
  `:4038` admit Specialized operands only, and a Carried operand is `None`.
- **Unmeasured:**
  - whether a runtime owner check at the field store
    (`boundary_value_clif`) refuses a short-lived referent;
  - whether the Carried word comes from a claimed producer Construct's
    continuation result. That is the predicate the RT-NAT-FANOUT trace
    found at Constructs 378, 349 and 1227, which `joins_traps.rs:479`
    summarizes `SpecializedOnly`.
- Evidence: `/workspaces/ken/local/rt-nat-fanout-ac0quintprime/`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **AC-0, at the start of the repair (measure only).**
   - (a) **Route.** For Match781 and Match1030, the route that makes each
     one Carried: is it a claimed producer Construct's continuation result,
     and if so, which construct, claim site and target?
   - (b) **Store path.** For each, the function that emits the field store
     into Construct782 or Construct1031, and whether a runtime
     referent-owner check is emitted there. Name the owners it admits and
     the parent lifetime it checks against.
   - (c) **Population.** Over the same 1031-row population, every aggregate
     child planned `[NoReferent]`, at any lifetime, that lowers Carried:
     the count, the rows and the parent lifetimes.
   - (d) **Executed witness**, if (b) shows no runtime check. A program
     where such a child's carried word is owned by the invocation arena and
     the persistent parent outlives the activation, run natively and
     compared with the interpreter.

   The Architect then rules the repair.
2. **The ruled repair.** The planner stays the single authority on
   representation and referents.

## Acceptance

- **AC-1.** A focused test pins that the witnesses' plan and lowering agree:
  the child carries its referent, and the containment check runs. If (d)
  produced a divergence, the executed witness matches the interpreter. The
  test fails on main before the repair (CHECKS 8).
- **AC-2 (controls).**
  - The (c) census is zero after the repair.
  - The runtime lib tests, `rt_escape_second_resource_native` and
    `rt_native_tree_match_case_of_case` keep their default results. The
    full `rt_parity_native` suite is CI's: it is not run locally, and
    touched parity rows are run by name.
- **AC-3 (falsifier).** Reverting the repair brings back the plan/lowering
  mismatch on the witnesses.

## Stop conditions

- **(d) gives a native result that differs from the interpreter.** That is a
  confirmed memory-safety defect on main: stop to the Architect and the
  Steward at once.
- **(b) finds a runtime check that already refuses it.** Stop to the
  Architect; the repair becomes a correctness fix on the static plane only.
- Any kernel, trust or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.
