---
id: RT-CROSSING-CALLEE-IDENTITY
title: "GeneratedUnitCallInput is measured at a shared helper with six callers, so branch 1 is provisional -- record WHOSE call is being carried, and exercise the tag's unused negative arm"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-CROSSING-CALL-SITE-ATTRIBUTION]
blocks: [RT-PLANNED-CLOSURE-PREEXISTENCE]
github: https://github.com/swe-toolkit/ken/pull/2314
origin: Architect, 2026-08-15, resolving dec_5m10b60wam0rz on RT-CROSSING-CALL-SITE-ATTRIBUTION (evt_752hfn288jrcs). He named this increment and its sequencing -- "that is the increment I would sequence BEFORE any repair node is cut". Steward-filed (agents cannot create tracked work per COORDINATION section 2).
---

> # NO REPAIR NODE MAY BE CUT BEFORE THIS ONE MEASURES
>
> [[RT-CROSSING-CALL-SITE-ATTRIBUTION]] selected **branch 1 — provisionally**,
> and its own `D2` GAP says why. **The provisional label is not caution; it is
> the accurate state of the evidence**, and a repair cut against branch 1 while
> 3' still holds is the `D2k-1c` cost arriving at the last possible moment.

## What is measured, and the single step that is not

**Measured, and it is real:** both enabled rows enter the origin-5 crossing with
`invoking_site = GeneratedUnitCallInput`. The construction is sound — the
thread-local's default is `Direct`, so observing the **non-default** value is
evidence the crossing happened dynamically inside `carry_call_input`. The
uninformative value is the default, which is the right way round.

**Not measured — the step the branch selection rests on:**

> *"The carrier path would therefore run for any specialized value delivered
> there."*

**That is true of the HELPER and is being used to conclude something about the
DELIVERY.** `carry_call_input` has **six callers** — `core.rs:17996`, `:18014`,
`:18350`, `:18434`, `:18449`, and the `carry_source_call_inputs` loop at
`mod.rs:7618` — and the tag sits on the helper, **one level above all of them**.
So the measurement establishes *"a generated-unit call input was being carried"*.
**It does not establish whose call.** And that is exactly what separates the
branches:

| the callee is | then | branch |
|---|---|---|
| a unit the **source program already calls** | delivery is design-intended and the **value** is the anomaly | **1** |
| the **projected consumer's** generated unit | the realization is emitting a call, against its own contract *"without exporting a compiler-only static worker through a function ABI"* — the **delivery** is the anomaly | **3'** |

**Both are consistent with `GeneratedUnitCallInput`.**

**The predecessor's supporting claim grounds and is still not enough.**
`realize_required_consumer_locally` does end at
`RoutedAnswer::composed_answer(...)` and carries nothing, so the crossing is
genuinely not on its **return** surface. **But ruling out the return surface does
not rule out the realization emitting a CALL** — those are different exits, and
only one of them was checked.

> # AMENDED — a probe settled the cheaper half of provisionality and
> broke a claim next to it
>
> Adversary hunt `evt_3qtvxnsv8184g` on `637781f41`, verified against the tree
> rather than taken. He ran a temporary `#[cfg(test)]` re-entrancy probe, then
> reverted it; the dynamic numbers below are **his measurement, not reproduced
> here**, and `D4` exists to land the structural half as a control.
>
> **The depth axis is settled, so `D2` has less to do than the frame said.**
> The tag is an **ambient thread-local held for the guard's whole dynamic
> extent** — `BoundaryTransferInvokingSiteGuard::enter` replaces the cell and
> `Drop` restores it (`mod.rs:2612-2655`, verified). So
> `GeneratedUnitCallInput` strictly means *"a `carry_call_input` guard was live
> somewhere up the dynamic stack"*, **not** *"`carry_call_input` invoked this"* —
> the mechanism is dynamic and the predecessor's `D2` describes it lexically.
> That is a second way the tag could be wrong, and it was never stated.
> **It is now measured false:** `enclosing_transfers = 0` on every transfer in
> all four compiles, so nothing nests and the immediate invoker on both enabled
> rows really is `carry_call_input`.
>
> ⇒ **Provisionality is now exactly one axis wide — breadth, the six callers.**
> Do not re-attack the depth axis, and do not write `D2` as though both halves
> were open. Record the ambient-versus-lexical distinction, because the next
> reader of the tag will assume lexical.
>
> **`Direct` is live, which makes `D3` cheaper and bounds what it proves.**
> Observed at origins 39, 31, 33, 10, 49, 51, 41, 43, 59 and 61 — and
> `transfer_unit_result_into_carrier`, the return surface branch 3' names, fires
> with `Direct`. **But `Direct`'s own doc comment defines it as *"a site not yet
> given a narrower diagnostic tag"* — it is the unlabelled default bucket, not a
> positive claim about a site.** So a `Direct` observation shows the tag
> **varies**; it does not identify anything. **That is precisely why `D1`'s
> callee identity, not the tag, is what answers *whose call*.**

## Deliverables

**`D1` — one tag finer, not a new instrument.** Distinguish the
`carry_call_input` callers, **or** record the callee's unit identity alongside
`origin` and `root_kind` on the existing event. Either discharges this; the
callee identity is the more direct answer to *"whose call"*. Same `#[cfg(test)]`
discipline, same transition-sentinel promise class.

**`D2` — decide branch 1 versus 3' on the measurement, per row.** State which
callee each enabled row's crossing belongs to and which branch it selects.
**Then remove the word "provisional" from the predecessor's `D2` GAP, or say
plainly that it stands** — a node that measures the missing step and leaves the
qualifier in place has not finished.

> **`D2`'s per-row phrasing presumes a function, and the relation is not one.**
> *"The callee for row 4 depth 2"* is only well-formed if origin 5 crosses
> once. **`(origin → crossing)` is a relation** — see `D4` — and origin 5
> crossing once is a fixture property, not a law. **Write the answer as the set
> of crossings for that origin**, even where the set is a singleton today. If a
> row turns out to carry more than one crossing with different callees, that is
> the live stop firing, not a selection to be forced.

**`D3` — exercise the tag's negative arm, which no observation currently
reaches.** `BoundaryTransferInvokingSite` has two inhabitants and **every
assertion in the tree reports `GeneratedUnitCallInput`** — verified on the
candidate: exactly two, at `control.rs:6305` and `:6329` (the Architect cited
`:6303`/`:6327`, which were the pre-amendment lines). **Its discriminating power
is argued from the guard's structure, not measured.** The Architect named a
real pair that is already available: **a constructor child transfer through
the store loop reaches
`transfer_into_carrier` without passing `carry_call_input`, so it should report
`Direct`.** One observation converts the tag from a constant on the observed
population into a measured discriminator.

**`D4` — the `panic!` beside the field you are editing states a law the
mechanism does not have.** Adversary hunt `evt_3qtvxnsv8184g`, structural half
verified on `main` at `control.rs:6247`:

```rust
_ => panic!(
    "{label}: origin 5 must identify at most one predecessor crossing, got {targeted:?}"
),
```

**Nothing in the mechanism establishes one crossing per origin.** The filter is
parameterized on `target_origin`, and the message hardcodes a general claim
about origins. His trace falsifies the general form **in the same run**: origin
39 crosses **twice, with two different tags** (`Direct`, then
`GeneratedUnitCallInput`), and origins 31, 41, 49, 51 and 61 likewise cross
twice per compile. Origin 5 crossing once is a fact about these fixtures.

**The cost is a misdiagnosis, not a missed red.** If a repair makes origin 5
cross twice — exactly what its siblings already do — the control panics with a
message asserting an invariant that never held, and the next reader concludes
*"the plan is corrupt"* rather than *"origin 5 now crosses twice, like origin 31
always has."* **A `panic!` whose message states a false law is worse than a red
assertion, because it redirects the investigation.**

**Repair:** make the arm **report rather than assert** — carry the crossings as
a `Vec` and let the four-row table pin the count. That keeps today's expectation
exactly as strong and turns a future failure into *"origin 5 now has two
crossings, tags `[Direct, GeneratedUnitCallInput]`"*. If the singular form is
kept instead, **the message must say it is a fixture property of origin 5, not a
rule about origins** — and that is the weaker option, because it leaves `D2`'s
per-row phrasing resting on an unpinned fixture fact.

> **This does not breach `AC-5`.** That ban is on landing a repair to **branch 1
> or 3'** — the value production or the routing. `D4` is a diagnostic-control
> correction in the file `D1` already edits, and it is the same class of item as
> the predecessor's `D3`, which this ring closed without re-litigating.

## The live stop

**If the callee identity does not separate the two branches either** — the
callers are indistinguishable at that point, or the rows disagree — **say so and
stop.** That outcome is informative: it would mean the branch question is not
answerable from the crossing at all, and the next move is upstream rather than
one more field. **Do not force a selection to remove a qualifier.**

## Acceptance criteria

**`AC-1`.** `D1` adds nothing to a production build surface, verified by a
targeted `ken-runtime` check rather than asserted.

**`AC-2`.** `D2` names the **callee** per enabled row and selects branch 1 or 3'
from it. **Restating `GeneratedUnitCallInput` in more words fails this** — that
is the fact already in hand. The answer is stated as the **set** of crossings
for the origin, per `D4`; a per-row answer written in the singular fails.

**`AC-3`.** The predecessor's *"provisional"* qualifier is explicitly resolved,
one way or the other, in the tree.

**`AC-4`.** `D3` produces an observation that actually reports **`Direct`**.
**A comment explaining that `Direct` is reachable does not discharge it** — the
defect is precisely a two-inhabitant tag with one observed inhabitant. **And the
observation is not over-read:** `Direct` is the unlabelled default bucket, so it
evidences that the tag **varies** and nothing about the site's identity. A
report that treats `Direct` as a positive finding about a site fails this.

**`AC-7`.** `D4` lands, and the general claim is gone from the tree: either the
arm reports a `Vec` of crossings with the count pinned by the table, or the
message says plainly that single-crossing is a **fixture property of origin 5**.
**Demonstrated, not asserted** — show the multi-crossing path producing a
readable report or a correctly-worded failure, in the shape the predecessor's
`D3` mutation evidence took.

**`AC-8`.** `D2` records that the tag is **ambient over the guard's dynamic
extent**, not lexical on `carry_call_input`, and that the depth axis is measured
closed (`enclosing_transfers = 0`). **The predecessor's wording is the thing
being corrected**, so restating it fails.

**`AC-5`.** No repair lands here, and no ownership is inferred. **This node
finishes the fork; the repair node is cut after it, from its result.**

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Why this earns a slot

**Because the repair cannot be cut without it, and that is the Architect's
sequencing, not mine:** *"that is the increment I would sequence before any
repair node is cut."*

**And because the alternative is a repair cut against an argued step.** Branch 1
puts the repair in value production; branch 3' puts it in routing and keeps the
realization local. Different files, different owners. The whole chain has now
twice had a measurement carry a branch label it did not establish — `D5`'s
`CLAIMED` line, and this predecessor's delivery inference — and both times the
catch came from reading the instrument rather than the result. **One field
closes it.**

**This is lane 1.** `PROJECTION` (merged) → `CENSUS` (merged) → `CALL-SITE` →
**this** → repair → `TRANSPORT` → `DESCENT-RETIRE`.

> ## MERGED at `dbf7957a3` (PR #2314), `D1`-`D4` delivered. Steward, 2026-08-15.
>
> **The result is not the one this node's title anticipated, and that is the
> point of having run it.** The frame expected the callee to confirm or refute a
> provisional branch 1. It did neither: **it closed branch 3'.**
>
> **`D1`/`D2` closed 3' by proof rather than by label.** `expected_source_callee`
> recovers the origin from the plan, destructures it as `RuntimeExpr::Match`, and
> asserts `default.message == "direct HostResult default"`, so *"the callee is
> the source program's direct `HostResult` `Match` body"* is **proved against the
> planned occurrence**. With caller `SourceLexicalClosureArgument` the delivery is
> source-program-authored ⇒ the crossing is **a call the source program itself
> makes**, not the realization exporting through an ABI it promised to avoid.
>
> **Closing 3' does NOT select branch 1** (Architect, `dec_6hwh86vdzp2ha`). Two
> branches remain and they are separated by **closure pre-existence** alone:
> branch 1 (the realization produces a closure-shaped value into an intended
> call-input route) versus the **durable-lane** branch (the source value
> legitimately carries a closure and Ken has no lane) — which is
> [[RT-CLOSURE-BOUNDARY-LANE]]'s mechanism after all.
>
> **A Steward ruling was withdrawn on the back of this**, in
> [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] and [[RT-CLOSURE-BOUNDARY-LANE]]: the
> durable-lane branch had been declared eliminated by an argument that
> **suppression itself forces**, and it is live.
>
> **`D3` discharged the `Direct` finding honestly**, recording that `Direct` is
> only the unlabelled default bucket and identifies no site. **`D4` replaced a
> uniqueness law that was false by refinement** — `(origin -> crossing)` is a
> relation, not a function.
>
> **Successor: [[RT-PLANNED-CLOSURE-PREEXISTENCE]]** — *"stop asking the run, ask
> the planner."* It also carries the Architect's non-blocking finding that three
> `cfg(test)` sites bind with `?`, letting the test profile refuse a compile
> production accepts.
