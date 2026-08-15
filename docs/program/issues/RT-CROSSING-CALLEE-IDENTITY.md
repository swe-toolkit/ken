---
id: RT-CROSSING-CALLEE-IDENTITY
title: "GeneratedUnitCallInput is measured at a shared helper with six callers, so branch 1 is provisional -- record WHOSE call is being carried, and exercise the tag's unused negative arm"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CROSSING-CALL-SITE-ATTRIBUTION]
blocks: []
github: null
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

**`D3` — exercise the tag's negative arm, which no observation currently
reaches.** `BoundaryTransferInvokingSite` has two inhabitants and **every
assertion in the tree reports `GeneratedUnitCallInput`** — verified on the
candidate: exactly two, at `control.rs:6305` and `:6329` (the Architect cited
`:6303`/`:6327`, which were the pre-amendment lines). **Its discriminating power
is argued from the guard's structure, not measured.** The Architect named a real pair that is already
available: **a constructor child transfer through the store loop reaches
`transfer_into_carrier` without passing `carry_call_input`, so it should report
`Direct`.** One observation converts the tag from a constant on the observed
population into a measured discriminator.

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
is the fact already in hand.

**`AC-3`.** The predecessor's *"provisional"* qualifier is explicitly resolved,
one way or the other, in the tree.

**`AC-4`.** `D3` produces an observation that actually reports **`Direct`**.
**A comment explaining that `Direct` is reachable does not discharge it** — the
defect is precisely a two-inhabitant tag with one observed inhabitant.

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
