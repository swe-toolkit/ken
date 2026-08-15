---
id: RT-CONSUMING-OCCURRENCE-ROUTE-WIRE
title: "The carried consuming occurrence is production-written and test-only-read, so no production path has ever consulted it -- wire one consumer at the refusing boundary and MEASURE what the route then does, without assuming it closes"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-CONTKEY-CONSUMER-DESCENT-CARRY]
blocks: []
github: null
origin: "Architect scope statement recorded on the RT-CONTKEY-CONSUMER-DESCENT-CARRY merge (PR #2233, exact b0f9c2ff2), verbatim in substance: required_consuming_occurrence is the bounded discovery-only increment he authorized, the carry has not been validated by any production consumer, and the successor that wires one must not treat that node as having done so. The route question was ruled a separate increment twice -- evt_6td3bs6j6g14m and evt_56dvtaft7ep38. Steward-filed per COORDINATION section 2."
---

> # MEASURED 2026-08-15 AT `46a8ba199`. `D1`-`D3` DELIVERED, NO CANDIDATE, AND
> # THAT IS THE FRAMED OUTCOME — NOT A FAILED TURN.
>
> **Runtime ran it and hit the stop condition exactly as written**
> (`evt_3kk9xbfpfwcqn`). Probe fully reverted, WP branch deleted,
> `runtime-implementer/work` clean at the base, targeted post-cleanup control
> 1 passed. **No mutation, source change, commit, or branch retained.**
>
> **The node said a measured refusal with the refusal attributed fully
> discharges it. It does, and this measurement did better than that — the route
> ADVANCED at depth 1.**
>
> ### WHAT THE ROUTE DID, per row. This is `D2`, and it is the durable record.
>
> One consumer was installed at the shared continuation-call funnel
> (`lowering/core.rs:11844`), with the resolved required value temporarily routed
> through the only existing production projection,
> `ContinuationSpecializationKey.consuming_occurrence` (populated from the
> same-level source relation at `static_transition.rs:11380`). No
> `ContinuationTemplate` and no continuation-source-projection change.
>
> | row | outcome |
> |---|---|
> | row 4 depth 1 | **ADVANCED.** Same-level required and source values coincide; the real route recognized one field, rebound once, consumed once, and moved from `StaticWorkerBinding` to a **new `Closure` refusal**. |
> | row 4 depths 2/3 | **REJECTED BEFORE LOWERING** by the independent validator at `static_transition.rs:11049-11103`: *"a continuation specialization's consuming occurrence has a mismatched `eliminator_origin`: it does not select the continuation as its position-zero child"*. |
> | row 5 after-hole | recognized once, rebound once, consumed once, then **remained behind a later `StaticWorkerBinding` refusal**. |
> | row 1 | **unchanged** at `NativeJoinPlanV1` — it remains a separate class, as this node's excluded scope predicted. |
>
> ### `D3` ANSWERED, AND IT CONFIRMS THE INERTNESS
>
> **`(None, None, None)` on the reached recognize / rebind / consume path.**
> These compiles use `RecursiveDescent`, so `defining_function_id` is absent
> throughout and **both `Option<FuncId>` inequality guards pass vacuously** —
> exactly as this node predicted. `D2k-1c-1a` is now **measured, not merely
> argued**. Do not re-file it.
>
> ### `D4` — THE RESIDUAL, and it is a REPRESENTATION boundary
>
> **The depth-2+ value cannot lawfully inhabit the existing key slot.** The
> validator's rejection is not an accident of the probe: it **proves the slot is
> source-keyed by contract**, not spare carrier storage. Putting the lagged value
> there violates the source-key validator and the key's interning law.
>
> ⇒ **A lawful depth-2+ consumer needs the required relation projected
> SEPARATELY from specialization identity into lowering.** That is
> representation/projection widening, which `AC-5` forbids here. **`AC-2` cannot
> be authored lawfully until that surface exists** — which is why the implementer
> was right to author no candidate rather than take the surface on the ring's
> authority.
>
> **The successor is a mechanism question for the Architect, routed 2026-08-15.**
> Do not frame it, and do not widen this node to swallow it.

> # THE ROUTE QUESTION IS THIS NODE. IT IS NOT AN INCREMENT ON `D2k-1c`.
>
> **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1c` is a WRONG CUT, not an unfinished
> one**, and it is not where this work goes. Both ways forward from that cut
> cross its own banned scope: one mutates the planner-owned
> `BodyEmissionDisposition::ContinuationTemplate` population, the other needs a
> projection through the excluded continuation-source surface. **A WP that must
> cross its own banned scope to discharge its AC has been cut wrong.**
>
> **Do not reopen `D2k-1c` and do not fold a carry out of its history.** Its
> `D2k-1c-0c` block reads as owed and is stale — the six `site` labels are
> already repaired in the tree in the prescribed `function@qualifier` form.

## What has landed, so you do not re-derive it

Three nodes have already run this chain down, and **each supplied an input
without closing a row.** Read this table before opening any of them.

| node | what it established | what it did NOT do |
|---|---|---|
| [[RT-CONTKEY-CONSUMING-OCCURRENCE]] (`a998d3f6`) | the source-keyed relation, **complete at depth 1** | nothing below depth 1 — the absence there is structural, not a bug |
| [[RT-CONTKEY-CONSUMER-DESCENT-CARRY]] (`b0f9c2ff2`, PR #2233) | `required(N)` = the consumer established at `N-1`, carried at the descent push | **no row closed**; it deliberately carried no closure AC |
| [[RT-CONSUMER-CARRY-CONTROL-DEBT]] | the five non-blocking carries off that merge | — |

**The mechanism is ruled and is not yours to re-litigate.**
`consuming_occurrence` is **source-keyed** — minted onto the position-zero child
of an outer `ComputationalMatch` — so at depth 2/3 the consumer is determined by
**which specialization realized the body**, a fact about generated structure. A
relation minted in the source walk cannot name that **in principle, not by
omission**. The carry is what reaches it: derive at the push, **attribute to the
child**. That is the `D5a` shape, and it landed with cross-compile equalities
between independently produced planner records rather than fixture literals.

## What this node owns

**`required_consuming_occurrence` is PRODUCTION-WRITTEN and TEST-ONLY-READ.**
The Architect recorded that plainly *"so nobody later over-reads it"*. It is
written at `planning/static_transition.rs` (the push at `:10887`/`:10899`, the
accessor `required_consuming_occurrence_for_alternative` at `:10985`) and
**no production path has ever consulted it.**

⇒ **Wire one production consumer at the refusing boundary, and measure what the
route then does.** That is the whole node.

> ## THE ONE AC YOU MAY NOT WRITE, RULED TWICE
>
> **That supplying the relation CLOSES the route is NOT established.** The
> original stop named a further `Closure`/static-worker refusal **and** a second
> recognition retained in the standalone definition. Both are still ahead.
>
> ⇒ **An AC that assumes closure assumes exactly what nobody has measured.**
> Frame your controls around *what the boundary does once it can see the
> relation*, not around rows 4 and 5 turning green. **A measurement that the
> route still refuses, with the refusal attributed, fully discharges this
> node.**

## Deliverables

**`D1` — wire exactly one production consumer.** At the boundary that currently
refuses, read `required_consuming_occurrence` on the path row 4 depth 2 takes.
**One consumer, not a sweep** — the point is to learn what the boundary does
with the fact, and a second call site adds a second failure mode to attribute.

**`D2` — measure, and attribute the outcome.** For row 4 depths 1/2/3 and row 5,
record what the route does once the relation is visible: advances, refuses at
the same boundary, or refuses at a **new** one. **Name the boundary and the
refusal in each case.** This is the deliverable even if nothing advances.

**`D3` — the deciding read that could only be taken here.** `D2k-1c-1a`, carried
forward because it needs a rebound field and so could not start earlier: **is
`defining_function_id` `Some` at the recognize / rebind / consume triple on the
path the five rows take?**

Two refusals compare scope as `Option<FuncId>` inequality — `recognized.scope
!= scope` (`mod.rs:4335`, in `rebind`) and `minted.scope != scope`
(`mod.rs:4422`, in `note_consuming_call`) — so **`None != None` is false and
both pass vacuously when the scope is absent.** `defining_function_id`'s `Some`
assignment (`mod.rs:8939`) is reached only through `open_aggregate_events`,
whose five callers are all in `units.rs`, the `FunctionizedUnits` arm. **On the
`RecursiveDescent` arm the field is `None` for the whole root descent** —
inert, not merely untested. The control set exercises the axis exclusively at
`Some` (`control.rs:31707`, `:31708`, `:31934`), so those rows prove the guard
fires when two bodies differ and **cannot show it is ever live.**

> **The fact is already written down beside the fixture and no operative row is
> on its side** (`control.rs:31704-31706`): *"Production always passes
> `defining_function_id`, which is `None` outside the emission pass."*
> **Treat a comment that states a limit as an unstarted control, not a
> discharged one.** This is the third time in this chain that a precise fact sat
> in prose while every row was on the other side of it.

**`D4` — record the residual.** Whatever `D2` finds is the next cut's fixed
input. Name it in the shape the next framer can use: the boundary, the refusal
sentence, and which rows sit behind it.

## Acceptance criteria

**`AC-1` — no closure is asserted.** No AC, control, or commit message claims a
row closed unless `D2` measured it closing. See the ruling above.

**`AC-2` — the production consumer is real.** `required_consuming_occurrence`
is read on a production path, demonstrated by a mutation: change the carried
value and show a production-visible outcome moves. **A test that reads the field
does not satisfy this** — that is exactly the state the predecessor left.

**`AC-3` — the depth-1 lag boundary is pinned.** The carry's lag is **not
uniform**: at depth 1 `required` coincides with the same level's consumer and
lags only from depth 2 on. The predecessor's test asserts nothing about that
value and the field doc does not note the boundary. **Say which regime your
consumer is in, and pin it.** This is the carry most likely to produce a wrong
successor.

**`AC-4` — no key widening.** `ContinuationSpecializationKey`'s definition stays
untouched, as it was through the predecessor.

**`AC-5` — banned surfaces stay untaken.** No `ContinuationTemplate` population
change, no continuation-source projection surface, no template restructure, no
guard weakening. **These are the two surfaces whose crossing made `D2k-1c` the
wrong cut**; a candidate that needs either is a stop, not a widening.

**`AC-6` — no-regression, in CI** (`COORDINATION §12`).

## Excluded, and one of these will look like it belongs

- **Row 1.** It is a **different class**, not a further depth — it remains at
  `NativeJoinPlanV1` and its two target edges distinguish it from rows 4/5. Its
  `None` split is `H4` of [[RT-CONTKEY-REFUSAL-PROFILE-SPLIT]], grouped by
  defect class rather than by which node noticed it. **No blocking dependency
  either way. Do not assume one node closes all three rows.**
- **Row 6** — never in this population; it is [[RT-MATCH-RECURSOR-CONSUMERS]].
- **The five carries** off the predecessor merge; they are
  [[RT-CONSUMER-CARRY-CONTROL-DEBT]] and are merged.
- **Retiring a residual class.** `enum RecursiveDescentResidual`
  (`cranelift_backend/lowering/core.rs:1979`) still carries
  `MatchScrutineeRecursor` and `LexicalCallArgumentRecursor`;
  [[RT-RECURSOR-TRANSPORT]] owns their retirement, not this node.

## Stop condition — return to the Steward, do not decide

**If wiring the consumer requires crossing `AC-5`'s banned surfaces**, stop and
report. That is the exact shape that made `D2k-1c` a wrong cut, and taking it on
the ring's authority is what the predecessor's implementer correctly refused to
do. **Report the surface you need and what it buys** — a hard stop with the
measurement attached is a complete turn, not a failure.

## Why this earns a slot

**It is the operator's priority lane.** Three nodes have each supplied an input
to this route and **not one has been consulted by production code.** Until a
production path reads the carried relation, the chain has built a mechanism
nobody has asked a question of, and the next boundary's behaviour is unknown.

**Its cheapest honest outcome is a measurement that the route still refuses**,
with the refusal attributed — which is a fixed input the next cut needs and
cannot get any other way.
