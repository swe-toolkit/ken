---
id: RT-PLANNED-CLOSURE-PREEXISTENCE
title: "Suppression cannot answer closure pre-existence because it removes the observation point along with the crossing -- ask the PLANNER instead: does the planned occurrence at origin 5 carry a closure-typed field 0 by construction?"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CROSSING-CALLEE-IDENTITY]
blocks: []
github: null
origin: Architect, 2026-08-15, resolving dec_6hwh86vdzp2ha on RT-CROSSING-CALLEE-IDENTITY. He named the increment and the reason the previous instrument could not work -- "STOP ASKING THE RUN, ASK THE PLANNER". Steward-filed (agents cannot create tracked work per COORDINATION section 2).
---

> # THE FORK IS DOWN TO TWO BRANCHES AND THIS NODE DECIDES WHICH
>
> [[RT-CROSSING-CALLEE-IDENTITY]] **closed branch 3'** by proof, not by label:
> `expected_source_callee` recovers the origin from the plan, destructures it as
> `RuntimeExpr::Match`, and asserts `default.message == "direct HostResult
> default"`. With caller `SourceLexicalClosureArgument`, the delivery is
> **source-program-authored** — the crossing is a call the source program itself
> makes, not the realization exporting through an ABI it promised to avoid.
>
> **Closing 3' does not select branch 1.** Two branches remain, and they are
> separated by exactly one unmeasured property:

| branch | claim | where the repair goes |
|---|---|---|
| **(1)** | the realization produces a **closure-shaped value** into an intended call-input route | a lowering fix in this chain |
| **(durable-lane)** | the source value **legitimately** carries a closure through a source-authored call, and Ken has no lane for it | [[RT-CLOSURE-BOUNDARY-LANE]]'s mechanism, and the recursor rows become its rows |

**Both are consistent with every row measured so far.**

## Do not reach for suppression again. It cannot answer this.

**Twice now the differential has failed to settle pre-existence, and the reason
is structural rather than a matter of instrument quality.** Without the
projection these rows never build the subgraph at all, so on the suppressed leg
there is **no observation point** — `closure_path` is computed only at the
crossing, and the crossing is what suppression removes.

> **A Steward ruling was withdrawn over exactly this**, and it is recorded in
> both [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] and [[RT-CLOSURE-BOUNDARY-LANE]].
> The claim was that the durable-lane branch was excluded because its antecedent
> required `crossing_reached = true` **under suppression** and the measurement
> read `false`. **Suppression makes that reading `false` in every possible
> world**, so the conjunct discriminates nothing. **Any argument of the form
> "the suppressed leg shows X" needs this test applied to it before it is used.**

**The question has an observation point on the planning side.** It is a *static*
property of the plan, available without executing the projected route.

## Deliverables

**`D1` — ask the planner, not the run.** Determine whether the **planned**
occurrence at `StaticOriginId(5)` carries a **closure-typed field 0 by
construction**. This is a property of the planned occurrence, so it is readable
without running the projected route and without suppressing anything. Same
`#[cfg(test)]` discipline and the same transition-sentinel promise class as the
predecessors.

**`D2` — decide the branch, per row, and route the rows.** The Architect's
disposition, stated in advance so this is a measurement and not a judgement
call:

| planner reads | then | branch | the recursor rows |
|---|---|---|---|
| field 0 is closure-typed **by construction** | the closure **pre-exists** | **durable-lane** | are [[RT-CLOSURE-BOUNDARY-LANE]]'s |
| otherwise, **while lowering produced `Lowered::Closure`** | the realization **manufactured** the shape | **1** | are this chain's, and that node is sized on its own row |

**State which row reads which**, and say plainly what it means for
`RT-CLOSURE-BOUNDARY-LANE`'s scope and sizing. That node currently carries a
retraction and an open sizing question that only this measurement closes.

**`D3` — a diagnostic that can change the outcome it exists to record.**
Architect non-blocking finding on `dec_6hwh86vdzp2ha`, carried here because it
landed in the code this chain just merged. Three sites bind a `cfg(test)` local
with `?`, all `child_static_origin(..., 0)?`. **In the test profile that `?` is
an early return production never executes**, so the test profile can refuse a
compile that production accepts — **in a chain whose entire output is which
refusal each row reached.**

> **The exact lines, verified on the candidate `9ba3950af`, because the frame is
> what you will grep from.** The `?` calls are `core.rs:17993`, `:18356` and
> `:18442`. The Architect cited `:17990`, `:18352` and `:18438` — those are the
> **starts of the same three constructs** (the `#[cfg(test)]` attribute and the
> two `let closure_origin` bindings), not the `?` itself. Same three sites,
> named at their region rather than at the defect.
>
> **`child_static_origin(...)?` appears at five further sites** — `:4047`,
> `:15222`, `:15240`, `:15565`, `:15712`. **None is `cfg(test)`-gated**, so `?`
> is correct there and they are not in scope. Confirmed by inspection, so a
> census of the call name alone does not become the work list.

It fails **loudly** rather than silently, so severity is low and this is not a
reason to hold anything. **The cheap form is strictly better: let a missing
child degrade the TAG, not the COMPILE**, so the diagnostic cannot alter the
outcome it exists to record.

## The live stop

**If the planner-side property does not separate the branches either, say so and
stop.** Three instruments will then have failed on one question, and the honest
conclusion is that pre-existence is not decidable from this chain — which is
itself the finding, and it routes to the Architect rather than to a fourth
measurement. **Do not force a selection to close a fork.**

## Acceptance criteria

**`AC-1`.** `D1` reads a **planner/plan-side** property. **A measurement taken
at or after the crossing fails this**, whatever it reports — that is the
observation point this node exists to avoid.

**`AC-2`.** `D2` selects branch 1 or durable-lane **per row**, against the table
above, and states the consequence for `RT-CLOSURE-BOUNDARY-LANE`'s scope.
**Reporting the planner property without routing the rows fails this.**

**`AC-3`.** No suppression differential is used as evidence for pre-existence.
Re-running one for another purpose is fine; **citing its suppressed leg as
evidence about the closure is not**, and the retraction above is why.

**`AC-4`.** `D3`'s three sites no longer let a missing child change the compile
outcome. **Demonstrated** — show the missing-child path degrading the tag while
the compile result is unchanged, in the shape the predecessors' mutation
evidence took.

**`AC-5`.** No repair to either branch lands here, and no ownership is inferred
beyond the routing `D2` states. **This node finishes the fork; the repair is cut
after it, from its result.**

**`AC-6`.** Nothing added to a production build surface, verified by a targeted
`ken-runtime` check rather than asserted.

**`AC-7`.** No-regression, in CI (`COORDINATION §12`).

## Why this earns a slot

**Because it is the last unmeasured step of a fork that has been narrowed three
times, and the Architect named both the increment and the instrument:** *"STOP
ASKING THE RUN, ASK THE PLANNER."*

**And because the repair cannot be cut without it.** Branch 1 puts the repair in
this chain's lowering; durable-lane puts it in `RT-CLOSURE-BOUNDARY-LANE` and
hands it the recursor rows. Different files, different owners, different node
sizing — and one static property of the plan decides it.

**This is lane 1.** `PROJECTION` (merged) → `CENSUS` (merged) → `CALL-SITE`
(merged) → `CALLEE-IDENTITY` (merged) → **this** → repair → `TRANSPORT` →
`DESCENT-RETIRE`.
