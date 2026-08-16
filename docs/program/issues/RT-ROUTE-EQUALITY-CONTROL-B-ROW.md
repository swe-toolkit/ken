---
id: RT-ROUTE-EQUALITY-CONTROL-B-ROW
title: "The route-equality control's population never makes B fire, so on the shipped rows it reduces to the formula it replaced -- add the transparent-declaration Call row that exercises the second disjunct"
status: active
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary M8 hunt on merged b7bb88c6d (evt_5r2brk2f7pye8), filed by the Steward. Queued behind lane 1 per the operator's two-lane directive -- the control is under-exercised, not wrong, and nothing regresses while the row is missing."
---

## What this node is

**A one-row addition to a control that shipped sound and under-exercised.**
Nothing is broken and nothing regresses while this is open. It is filed rather
than queued in a message so the row specification survives.

[[RT-MATCH-SCRUTINEE-DISPOSITION]] `D3-narrow` (PR #2458) landed a differential
equality control comparing the residual predicate against the ordinary producer
route's **actual recorded decision**, replacing a proposed `B == false` pin that
would have passed unchanged when a third routing disjunct appeared.

**The mechanism is right. The population does not exercise it.**

## The finding, from the Adversary at `evt_5r2brk2f7pye8`

The routing decision is `A || B` where `A` is
`requires_heterogeneous_deforestation(scrutinee)` and `B` is
`self.declaration_call_produces_deforestable_aggregate(scrutinee)`. The three
shipped rows evaluate as:

| row | `subject_guard` | route | via |
|---|---|---|---|
| difference | true | false | — |
| handled-intersection | true | true | `A` |
| non-recursive | false | false | — |

**All three scrutinees are `ComputationalMatch`** (or non-recursive variants of
one), and `declaration_call_produces_deforestable_aggregate` early-returns
`false` for any non-`Call`.

⇒ **`B` is false in every row, so `observed_routes[0] == A` throughout, and on
the shipped population the equality is exactly `residual == subject_guard &&
!A` — the formula the redesign replaced.** The control cannot today distinguish
*"compares against the complete decision"* from *"compares against `A`"*.

> ### THIS ALSO BOUNDS THE MUTATION EVIDENCE, which is the part most likely to
> ### be taken at face value
>
> The recut's handback claimed *"a future disjunct `C` reds the difference
> row."* **That holds only if `C` fires there.** No row has ever had a second
> disjunct contribute at all, so **nothing yet demonstrates the control can see
> a second disjunct contributing.** The claim is about an unexercised
> capability. The Steward relayed it as demonstrated; it is not.

## Deliverable

**`D1` — add the missing row, to the Adversary's specification.** A `Match`
whose scrutinee is a **`Call` to a transparent declaration producing a recursive
deforestable aggregate**: `subject_guard = false`, `route = true` **via `B`**.

**Build the cell as specified rather than a paraphrase of it.** The property
that matters is that `B` is *observed true* in at least one row — a row where
the route fires via `A` adds nothing, and that is the whole point of the
finding.

## Acceptance criteria

**`AC-1`. At least one row records `B` true**, demonstrated by observation, not
by construction argument. **This is the entire node.**

**`AC-2`. The mutation claim becomes real.** With the new row present, show a
second-disjunct contribution is visible to the control — the standing claim that
a future `C` would red is currently unexercised and should stop being asserted
until it is.

**`AC-3`. The existing three rows are unchanged**, and the production predicate
is untouched. **This node adds coverage; it does not repair anything.**

**`AC-4`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only.

## Banned scope

- **Editing `match_scrutinee_requires_recursive_descent`** or any production
  predicate. It is correct; see [[RT-MATCH-SCRUTINEE-DISPOSITION]].
- **Re-litigating `!A` versus `!(A || B)` in production.** Architect ruled at
  `evt_292zd309yvkfb` that `!A` plus the structural argument is lawful and no
  naming refactor is required. **`B` is structurally false for the guarded
  subject; that is an exact restriction, not an approximation.**
- **Deleting or narrowing anything**, and the retirement generally.

## Sequencing

**`active`. Released to the runtime ring at `evt_7zd0ebefbgxy0`, `main` =
`6df61eafc`.** The queue condition fired on 2026-08-16: it said to release when
the ring is between lane-1 increments **or** when the retirement resolves either
way, and both happened. [[RT-MATCH-DIFFERENCE-REACHABILITY]] merged with outcome
3, and the retirement's only live gate is now
[[RT-DESCENT-RETIRE-PRIOR-ART]] — a research referral the runtime ring does not
wait on.

**This node still blocks nothing and is not on the critical path.** It is
coverage the ring can land while the referral runs.
