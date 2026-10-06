---
id: RT-CARRIED-CHILD-REFERENT-CONTAINMENT
title: "The planner overrides an aggregate child's referent owners to NoReferent from its planned NativeScalarPair join, while lowering produces the child Carried. The runtime store check refuses the only dangling pair, so this is not memory-unsafe, but at the closure-capture paths the wrong owners can allocate a parent persistent and fail a native run the interpreter accepts. Find the first planner/lowering phase divergence and repair the planner so the plan and the lowering agree"
status: active
owner: runtime
size: S
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

## AC-0'' result (Architect `evt_61aq80xm34yb7`, on `7a1fe0473`)

- **Measured** (implementer `evt_5s32v5tm7yfpz`): 109 consumed token
  mismatches over the 1,031-row population, all Matches. 81 are
  process-composed and 28 suffix-composed. 33 events reach
  `aggregate_child_referent_owners`, all at caller `:5532` with child
  lifetime `Persistent`. Zero are decision-changing.
- **Process-composed is the defect.** `process_composed_join_plan_token`
  (`joins_traps.rs:798-809`) returns CarrierWord for every composed Match
  join under a non-empty transport set, while
  `aggregate_child_referent_owners` narrows to `[NoReferent]` from the raw
  `join_results`. Two readers give two answers.
- **Suffix-composed is by design.** `composed_join_plan_token` is the
  interface morphism: the local Match never materializes, and its
  `join_results` entry has no value consumer (0 aggregate reads).
- **RT-NAT-FANOUT is not ordered behind this WP.** Its reached joins are
  suffix-composed, so the shared-predicate stop did not fire. The
  Architect rules R7 in that WP's thread.

## Deliverable

**R1** (the ruling's code is the text): one planner-domain predicate,
`join_may_take_process_carrier(origin)`, true for a source `Match` origin
when `pre_schema_transport_sources` is non-empty.
`process_composed_join_plan_token` refuses outside that domain, and
`aggregate_child_referent_owners` narrows `NativeScalarPair` to
`[NoReferent]` only outside it. No R6 and no RT-NAT-FANOUT code.

## Acceptance

- **AC-1 (behaviour).** On the census population, the 33
  aggregate-consumed events change child owners from `[NoReferent]` to
  `lifetime_referent_affinity(Persistent)`. Every parent allocation and
  meet in the consumer records is identical, base against R1.
- **AC-2 (domain).** The new refusal in `process_composed_join_plan_token`
  never fires across the runtime lib active suite,
  `rt_escape_second_resource_native` and `rt_native_tree_match_case_of_case`.
  The full `rt_parity_native` suite is CI's; touched rows are run by name.
- **AC-3 (mutation, QA).** Restoring the unguarded `NativeScalarPair ⇒
  [NoReferent]` arm reddens a new test: the r2 relay row asserting that
  Construct782's child-0 owners contain `PersistentStore`. Widening the
  predicate to all source joins is recorded; the assertion test still
  passes.

## Stop conditions

- **Any parent allocation or meet change** under R1: stop to the
  Architect.
- Any kernel, trust or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.
