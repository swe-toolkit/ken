---
id: RT-MATCH-SCRUTINEE-PORT
title: "Port the MatchScrutineeRecursor difference onto uniform lowering -- for the heterogeneous-case-body population the ordinary route must terminate in a lawful lowering, or the refusal is named and handed back"
status: active
owner: runtime
size: L
gate: none
depends_on: [RT-MATCH-DIFFERENCE-REACHABILITY, RT-DESCENT-RETIRE-PRIOR-ART]
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Architect ruling evt_nb12nmhd2zzk, 2026-08-16: build the retirement on the uniform-lowering family (GHC), declining the prior-art advisory's global IR invariant as the primary route. Sizing and framing assigned to the Steward in that ruling. Steward-filed per COORDINATION section 2."
---

## What this node is

**The retirement's last piece of engineering, and the campaign's own landed
method applied to its last variant.**

Four of the six original residual variants were retired by porting to uniform
lowering. `MatchScrutineeRecursor` is the survivor, and after two measurements
and a prior-art survey the ruling is that it goes the same way.

> ### THE ROUTE IS SETTLED. Do not re-derive it, and do not re-open what it closed.
>
> **Source-unreachability is dead from both directions.**
> [[RT-MATCH-DIFFERENCE-REACHABILITY]] found no grammar, admission or kernel
> rule that refuses the shape; [[RT-DESCENT-RETIRE-PRIOR-ART]] found that Lean
> and Agda **both admit it**, each with a passing regression case. **No further
> program-writing or rule-reading changes this.**
>
> **The global-IR-invariant family was declined as the primary route**
> (Architect, `evt_nb12nmhd2zzk`). It survives only as a fallback, boundary-
> scoped, and only on an uncloseable refusal plus a fresh ruling. **Proposing it
> from inside this node is out of scope.**

## Why uniform lowering is not a new argument here

**Ken's ordinary `Match` lowering already has the shape**, in production. At
`core.rs:17734-17756` the producer route is tried first, and when it declines:

```rust
let lowered_scrutinee = self.lower_expr(builder, scrutinee_occurrence, env)?;
```

**That is the GHC sentence — translate the scrutinee with the ordinary
expression translator, then construct the case around the result.** And the
scrutinee form at issue has a general arm in that same translator
(`core.rs:18071-18086` → `lower_computational_match_expr`, `core.rs:3897-3932`),
which builds a `Computational` eliminator frame and hands it to the shared
producer traversal.

⇒ **This node is a totality question about code that exists**, not the
construction of a new global property. Coordinates verified by the Steward at
`main` = `d79392582`; `crates/` is unchanged since the Architect's read at
`999909207`.

## The population, stated exactly

**The difference is the heterogeneous-case-body population.** Retention fires
when the subject is an active-recursion `ComputationalMatch`
(`core.rs:2175-2184`) that `requires_heterogeneous_deforestation` declines — and
it declines because `produces_deforestable_aggregate_with_ih` is **universal
over case bodies** (`mod.rs:16688-16697`, `cases.iter().all(...)`).

⇒ **One case body that does not produce a deforestable aggregate — a bound
scalar, as `D1`'s witness had — takes the whole subject out of the producer
route.** That population is non-empty as backend IR (`D2a` constructed a
member) and it is what the retained lane is currently carrying.

**The two quantifiers do not line up and that is the whole difference:**
retention is **existential** over a per-case field; routing is **universal**
over case bodies. Do not re-derive this; it has been settled twice.

## D1 RESULT — NON-TOTAL, WITH A NAMED REFUSAL. The fallback is NOT triggered.

**Handed back at `evt_5h96qz12red0f`, measured at exact
`5c5ee5b6c37dfc0017afa547f83cd0b9aa5f5111`. There is no candidate and no diff**
— the five-row control was disposable and was fully reverted. **This section is
the only durable record of the measurement; nothing merges to carry it.**

**Coverage (`AC-2`).** The predicate-defined finite quotient, across
selected-case recursive/nonrecursive × aggregate/scalar, including **coincident
and split existential witnesses** — the two "some"s landing on the same case and
on different cases. Not a sample.

**Instrument (`AC-1`) held.** Every row carried exact pre-exclusion
`{MatchScrutineeRecursor}` under the non-short-circuiting enumerator, and the
exclusion assertion held.

| rows | selected case body | outcome |
|---|---|---|
| 2, 4, 5 | aggregate | **lower and execute**, `Returned(Int(Small(7)))` |
| 1, 3 | **scalar** | **refuse:** `UnsupportedLowering { construct: "Match", reason: "scrutinee is not a constructor value" }`, emitted at `core.rs:17959` |

> ### THE REFUSAL IS SELECTED-BODY-SENSITIVE, AND THAT IS WHY `AC-3` IS SATISFIED
>
> **A conservative fail-closed arm would refuse the whole population.** This one
> splits it on the selected body, so it is a **named refusal about a specific
> shape** — the second arm of `AC-3`, not the silent outcome it bars.
>
> **And the scalar cells compile today through the retained lane.** Leaving them
> refusing is the capability regression `AC-3` names. **The refusal is a finding,
> not an acceptable resting state.**

**What the site is, verified by the Steward at `main` = `70fd2b69f`.**
`core.rs:17959` is the **terminal fallthrough of a `Lowered`-shape dispatch
chain** that runs *after* `lower_expr` has already succeeded on the scrutinee —
`Carried`, `BorrowedNativeValue`, `BorrowedOption`, …, `Bool`, then
`Constructor`, then the `Err`.

⇒ **The refusal is not about the scrutinee being a `ComputationalMatch`.** The
uniform-lowering sentence did its job: the scrutinee lowered. What has no arm is
the **resulting operand shape**. That is a narrower and more tractable gap than
"uniform lowering does not carry this population," and it is the fact the
mechanism ruling turns on.

**`D2` IS NOT AUTHORIZED AND WAS NOT ATTEMPTED.** The ring stopped at the
deliverable boundary as instructed. ⇒ **"Non-total" is established;
"uncloseable" is NOT** — those are different claims and only the first was
measured. `AC-3`'s stop-and-hand-back arm is conditioned on a refusal that
*cannot be closed*, which nobody has tested.

⇒ **The family-A fallback stays out of scope.** It is reached on an *uncloseable*
refusal plus a fresh ruling, and neither exists.

## Deliverables

**`D1` — activation measurement. Independently releasable, and size the turn
around it. DELIVERED — see the result section above.**
With the exclusion probe set for `MatchScrutineeRecursor`, take the
heterogeneous-case-body population through the ordinary route and report, per
shape: **lawful lowering, or a refusal quoted in full with its emitting site.**
This is the deliverable that decides whether `D2` is a port or a hard stop.

**`D2` — close what `D1` opens, inside the ordinary route.** Whatever the
ordinary path lacks for this population, supply it there. **Not a new lane, not
a new global invariant, and not a conservative arm** — see `AC-3`.

**`D3` — the executable control.** A witness in the difference population
compiles **and runs** with the variant excluded, and its decoded result agrees
with the interpreter. **The interpreter is the oracle, not `RecursiveDescent`**
(operator, 2026-08-15): what a program does under the retained lane is not
evidence that it should compile.

**`D4` — record the disposition at [[RT-DESCENT-RETIRE]]'s bar**, in the same
place the previous two results were written. A result not written there leaves
the capstone barred regardless of what was built.

## Acceptance criteria

**`AC-1`. The instrument is `enumerate_recursive_descent_residuals` with the
`selector_variant_exclusion` probe (`core.rs:2410-2428`), never the
short-circuiting selector.** Its `Match` arm tests `MatchScrutineeRecursor`
**first** (`core.rs:2118-2127`), so this is the one variant the short-circuit
reads optimistically. **The probe's `debug_assert` must be left in force** — it
is what stops the measurement running on a program that never fired the variant
and reporting an ordinary functionized compile as success.

**`AC-2`. The population is covered by its predicate, not by a sample.** State
what shape space was covered and how; a handful of programs that happen to
compile discharges nothing. This is the third node in this campaign to carry
that criterion.

**`AC-3`. TOTALITY OR A NAMED REFUSAL — and a conservative fail-closed arm is
NOT an acceptable silent outcome.** The prior ports shipped three such arms on
`ProducerMatchCall` alone (`core.rs:2058-2064`). Fail-closed is the right
direction in general, and **for this variant it is a capability regression**:
the difference population compiles today via the retained lane, so a refusal
takes away something that works. **If the refusal cannot be closed within the
port, stop and hand back.** That is a complete outcome and it routes to the
Steward.

**`AC-4`. No deletion, and the capstone stays barred by this node's own
result.** Deleting the enum, selector, authority or lane is
[[RT-DESCENT-RETIRE]]'s act, after a ruling on this evidence.

**`AC-5`. The retention guard is untouched.** It landed narrowed at PR #2458 and
is behaviour-preserving; `!A` in production is lawful (Architect,
`evt_292zd309yvkfb`) because `B` is structurally false for the guarded subject.
**Not a proxy — an exact restriction. Do not re-litigate it.**

**`AC-6`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only.

## Banned scope

- **The global `RuntimeExpr` invariant (family A).** Declined as the primary
  route. If `D1`/`D2` reach an uncloseable refusal, hand back — the fallback is
  boundary-scoped over what reaches native lowering entry, and it needs a fresh
  ruling before anyone builds it.
- **Re-opening source-unreachability**, by program-writing or rule-reading.
- **Deleting or re-narrowing anything**, and the retirement itself.
- **The `RecursiveDescent`-as-oracle framing** (operator, 2026-08-15).

## Sequencing

**`active`, and `D1` is DONE.** Assigned `evt_38he57edb7bpm`, handed back
`evt_5h96qz12red0f`, measured at exact
`5c5ee5b6c37dfc0017afa547f83cd0b9aa5f5111`. No candidate — see the result
section.

**`D2` is HELD on a mechanism ruling, routed to the Architect.** The question is
whether the scalar-selected refusal is closeable inside uniform lowering — a
component-design call, and the Steward's to route rather than to answer. **The
ring is not blocked on me for anything else and does not wait idle:**
[[RT-ROUTE-EQUALITY-RECONSTRUCTION-PIN]] is released in the meantime, which is
exactly the between-increments window that node was queued for.

**It blocks [[RT-DESCENT-RETIRE]] and becomes that node's twelfth dependency.**
The other eleven are `merged` and the capstone is still barred — every named
measurement returned and none licensed the deletion. **This node is the first
one in the chain whose success would.**

**Lane 1 under the operator's directive**, and it is the retirement itself
rather than a measurement about it.
