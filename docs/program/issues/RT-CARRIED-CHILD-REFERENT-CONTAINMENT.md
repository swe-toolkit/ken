---
id: RT-CARRIED-CHILD-REFERENT-CONTAINMENT
title: "The planner overrides an aggregate child's referent owners to NoReferent from its planned NativeScalarPair join, while lowering produces the child Carried. The runtime store check refuses the only dangling pair, so this is not memory-unsafe, but at the closure-capture paths the wrong owners can allocate a parent persistent and fail a native run the interpreter accepts. Find the first planner/lowering phase divergence and repair the planner so the plan and the lowering agree"
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
referent. The parent's allocation is then decided by the child's true owners
before either aggregate is allocated, so no native run refuses a program the
interpreter accepts.

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
- Evidence: `/workspaces/ken/local/rt-nat-fanout-ac0quintprime/`.

## AC-0 result (Architect `evt_7m30r5xfwqr54`, on `7a1fe0473`)

- **Not memory-unsafe: stop (b) holds.**
  - `ken_boundary_store_field_local` (`boundary_value_clif.rs:1980-2001`)
    refuses a persistent parent holding an invocation-owned child before
    any store. That is the only dangling owner pair.
  - `NODE_OWNER` comes from the tag at allocation, and every child-word
    write goes through `store_field`.
  - The caller fails closed (`lowering/mod.rs:14096-14112`).
- **The defect is on the static plane.** `aggregate_child_referent_owners`
  (`planning aggregates.rs:3249-3258`) returns `[NoReferent]` for a planned
  `NativeScalarPair` join, even when lowering produces the child Carried.
  - For the witnesses 781 and 1030 it has no consequence: their child
    lifetime is `Persistent`.
  - At the closure-capture paths `:5757` and `:5837`, it can turn an
    ActivationOwned child Persistent. The parent is then allocated
    persistent, and the native store returns ERR_ESCAPE where the
    interpreter succeeds.
  - The other five callers (`:4167`, `:5362`, `:5389`, `:5422`, `:5532`)
    feed the owners into the parent's meet.
- **Route (a).** Match781 and Match1030 are Carried because their scrutinee
  Vars (Var780, Var1029) are Carried, not through a claimed producer. The
  candidate shared predicate with RT-NAT-FANOUT: the planner's phase for a
  binder slot is SpecializedOnly while lowering binds a Carried word. It is
  not established.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **AC-0', static plane only (measure only, disposable probes).**
   - (a) **First divergence.** For Var780 (funcid61/Spec(3)) and Var1029
     (funcid63/Spec(1)), walk each binder back to the first point where
     the planner's slot phase (`joins_traps.rs` `summarize_result_phase`)
     says SpecializedOnly while lowering holds a Carried operand. Name the
     planner arm and line and the lowering site, and say whether it is
     RT-NAT-FANOUT's F819/F15 arm and origin kind.
   - (c) **Consequence census.** Over the same 1031-row population, every
     call to `aggregate_child_referent_owners` that returned through the
     NativeScalarPair arm for a child lowered Carried: the count, the caller
     line, the child lifetime and the parent's allocation. Separate the
     decision-changing cases, where `lifetime_referent_affinity` of the
     child's lifetime contains `InvocationArena`.
   - (d) **Executed witness, only if (c) finds a decision-changing case.**
     Run that row natively and compare it with the interpreter. The
     predicted failure is native ERR_ESCAPE (-1) against interpreter
     success.

   The Architect then rules the repair.
2. **The ruled repair.** The planner stays the single authority on
   representation and referents.

## Acceptance

- **AC-1.** A focused test pins that the witnesses' planned child owners
  match the lowered representation. If (d) produced a divergence, the
  executed witness matches the interpreter. The test fails on main before
  the repair (CHECKS 8).
- **AC-2 (controls).**
  - The (c) census is zero after the repair.
  - The runtime lib tests, `rt_escape_second_resource_native` and
    `rt_native_tree_match_case_of_case` keep their default results. The
    full `rt_parity_native` suite is CI's: it is not run locally, and
    touched parity rows are run by name.
- **AC-3 (falsifier).** Reverting the repair brings back the plan/lowering
  mismatch on the witnesses.

## Stop conditions

- **(a) finds no divergence**, meaning the planner already says Carried for
  780 and 1029: stop to the Architect.
- **(a) names RT-NAT-FANOUT's arm:** stop to the Architect, who rules one
  planner rule here. RT-NAT-FANOUT then rebuilds R6 on top of it.
- **(d) native differs from the interpreter:** a conformance defect on main.
  Stop to the Architect and the Steward.
- Any kernel, trust or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.
