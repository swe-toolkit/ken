---
id: RT-ROUTE-EQUALITY-PIN-AT-THE-BINDINGS
title: "The anti-factoring warning sits on the assertions while the duplication a tidy-up targets is the bindings 13 to 17 lines above -- put the clause where the dangerous edit happens"
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-ROUTE-EQUALITY-RECONSTRUCTION-PIN]
blocks: []
github: null
origin: "Adversary hunt on merged b4524ea4c (evt_4j3d7523jxh61), filed by the Steward. The Adversary specified the pin's location in evt_2xxr83djrtepq and corrected its own specification here. Queued behind lane 1."
---

## What this node is

**The predecessor's warning is in the wrong place, and the predecessor is
otherwise correct.** [[RT-ROUTE-EQUALITY-RECONSTRUCTION-PIN]] delivered exactly
what it specified — the defect is in the specification, which was the
Adversary's, and the Adversary is the seat that caught it.

**Nothing is broken and no detector is currently dead.** This closes a route by
which one could die unnoticed.

## The finding, verified by the Steward at `main` = `d2e046434`

**The clause landed on the assertions. The duplication a tidier would target is
the bindings.**

| control | binding | warning | gap |
|---|---|---|---|
| ordinary route | `:16528-16529` — `requires_heterogeneous_deforestation(scrutinee) \|\| lowering.declaration_call_produces_deforestable_aggregate(scrutinee)` | `:16541` | **13 lines** |
| declaration call | `:16572-16573` — `route_a` and `route_b` bound separately | `:16590` | **17 lines** |

⇒ **The natural DRY move is one shared helper called at both binding sites.**
That edit happens 13 and 17 lines above the only warning **and does not change
the line the warning sits above** — `vec![ordinary_route]` reads identically
afterwards.

> ### THE REFACTOR THAT KILLS THE SENTINEL NEVER BRINGS THE WARNING INTO VIEW.
>
> **`AC-1` of the predecessor is satisfied as written.** *"A reader editing
> either assertion meets the warning without leaving the file"* is **true**.
> **The gap is that the dangerous edit is not an assertion edit.** The clause
> protects the **use**; the refactor targets the **definition**.

**One half is already covered, and stating it keeps the fix small.** A tidier
who replaces `route_a || route_b` **at the assertion** with a helper call does
meet the warning. **It is site 1's binding — where the whole disjunction is
bound in one expression — that is reachable without it.**

## Deliverable

**`D1` — put the clause where the dangerous edit happens.** Move or duplicate it
onto the bindings at `:16528-16529` and the `:16572`/`:16573` pair, in the same
words. Duplicating rather than moving is acceptable and probably better: the
assertion-side reader is also worth catching.

## Acceptance criteria

**`AC-1`. A reader editing EITHER THE BINDING OR THE ASSERTION meets the
warning without leaving the file.** This is the predecessor's `AC-1` with the
address corrected, and the correction is the whole node.

**`AC-2`. No production change, no control behaviour change, no new rows.**
Comments only. The assertions, rows, observation domain, routing predicate and
retention guard all stay exactly as they landed.

**`AC-3`. No-regression, in CI** (`COORDINATION §12`).

## Banned scope

- **Factoring the two reconstructions together**, which remains the defect the
  whole pin exists to prevent.
- **Re-running or re-proving the `C`-arrival mutation.** It is recorded at
  [[RT-ROUTE-EQUALITY-RECONSTRUCTION-PIN]] and the Adversary confirmed it moves
  the population rather than the detector. **Do not spend the turn on it.**
- **Editing the routing predicate or the retention guard.**

## Sequencing

**`draft`, and `draft` here means QUEUED, not unframed.** Dispatchable on sight.

**Lane 1 is [[RT-MATCH-SCRUTINEE-PORT]] `D1b`**, which is the ring's work and
the retirement's critical path. **This node blocks nothing and protects a
mechanism that works today** — flip it `ready` at the next between-increments
window, exactly as its predecessor was.

## SECOND FINDING, same control: it discards its compile result

**Adversary hunt `evt_7ar8w31nr88wh` on `e2e15f8e1...8b78b48cd`, every number
re-measured by the Steward before it was written here.**

`msd_d2a_residual_equals_subject_guard_and_route_complement` discards the
compile outcome at both of its run sites:

```
control.rs:16367   let _ = rt_run_functionized(expression, excluded);
control.rs:16370   let _ = rt_run(expression);
```

**These are the ONLY two `let _ = rt_run*` sites in the whole file**, at base
`e2e15f8e1` and at merged `8b78b48cd` alike — the
[[RT-TRACE-HELPER-ABORTED-COMPILE-EVIDENCE]] repair left this population
byte-unchanged, because the helpers it deleted discarded their results *inside*
`owner(...)`/`multiplicity(...)`, which returned trace data rather than an
outcome.

### It is NOT vulnerable the way those helpers were, and the reason is the point

Its `assert_eq!(observed_routes, vec![ordinary_route])` requires **exactly one**
recorded routing decision, and the following assertion indexes
`observed_routes[0]`. **That decision can only exist if the compile reached the
ordinary-`Match` lowering.** So the control carries an independent non-vacuity
anchor that the deleted helpers lacked — they asserted trace structure without
requiring the compile to reach anything in particular.

### But reaching the routing site is not completing the compile

⇒ **Its assertions are claims about a compilation that got at least as far as
one `Match`, not about one that succeeded.** That is a real bound on a control
the retirement campaign now leans on heavily.

**Remedy, and it is one line either way:** state the bound in the control's own
comment, or assert on the returned outcome and delete the bound. **Prefer the
assertion** — a comment is not a detector.

**Note the name.** The discarding test is
`..._residual_equals_subject_guard_and_route_complement`, *not*
`msd_d2a_the_retention_and_routing_guards_have_a_concrete_difference`, which is
the `msd_d2a` named in [[RT-DESCENT-LANE-COMPLETENESS]]'s five-without-a-
refusing-construct set. Two different tests share the `msd_d2a` prefix; a
name-keyed sweep will conflate them.
