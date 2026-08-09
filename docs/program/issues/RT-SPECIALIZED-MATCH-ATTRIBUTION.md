---
id: RT-SPECIALIZED-MATCH-ATTRIBUTION
title: "A Match scrutinee arriving as a Specialized operand falls to the remainder arm, and neither the stage nor the seat says which Lowered class"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-BODY-OCCURRENCE-PROVENANCE]
blocks: [KERNEL-NESTED-IND]
github: null
origin: Architect residual ruling evt_2y5q0vg45g4km (2026-08-09) on the RT-BODY-OCCURRENCE-PROVENANCE AC-2 closure half, at exact accepted partial 876450ab. Ruled a Runtime successor boundary, not Kernel work and not authorized inside that candidate. Steward-filed and framed on that bound per COORDINATION §2.
---

> # THIS IS AN ATTRIBUTION NODE. IT DOES NOT BEGIN WITH A REPAIR.
>
> **[[KERNEL-NESTED-IND]] IS BLOCKED ON THIS**, and the Architect ruled the
> failure is **Runtime native lowering, not Kernel work** — Kernel has already
> admitted, erased and interpreted the witness.
>
> **Do NOT identify this with [[RT-COMPMATCH-TREE-SCRUTINEE]], `PX7-O`, or a
> generic "dynamic Match" repair.** Route to an existing authority **only** if
> the measured class is already in its population; otherwise return for a fresh
> mechanism ruling.

## What it is

[[RT-BODY-OCCURRENCE-PROVENANCE]] corrected the body-occurrence provenance, so
the source traversal now enters the **real** body for the first time. Owner 2 is
issued `SOI(26)`, the old `SOI(26)` closeout refusal is gone, and the entered
list runs `26 -> 58 -> 56 -> 55 -> 53 -> 52`.

**Owner 2 then never reaches closeout at all.** Only the siblings `{8}` and
`{14,20}` close. Lowering refuses first with

```text
Unsupported { stage: NativeLoweringOrExecution,
              construct: "Match",
              reason: "scrutinee is not a constructor value" }
```

so `{26,33,39,53}` is **reached but not closed**, and `33`/`39` are never
entered.

**Pre-existing, exposed rather than created.** The string is present 6x at base
`19d2695c`, and the raise site is untouched by the provenance candidate's diff.
**This is Trap 2 for the eighth time on this chain** — a fail-closed refusal
meeting a newly reachable population — and the campaign's standing record is
that it is not a defect in the node that exposes it.

## The seat inventory, corrected

**A construct-plus-string match does not identify this seat, and that is how the
first hypothesis went wrong.** The bare `Match` / `scrutinee is not a
constructor value` pair occurs at four places on this tree, and **only two are
production native-lowering seats.**

| site | production? | refuses on |
|---|---|---|
| `runtime_ir_evaluator.rs:1292` | yes, but **not native** | `RuntimeExpr::Match`, IR evaluator. `eval_unsupported` stamps stage `RuntimeIrEvaluation`, so the outer `NativeLoweringOrExecution` **excludes it** |
| `lowering/core.rs:6164` | **NO — `#[cfg(test)]`** | a **mutation-injected** refusal inside the `Carried(word)` arm, `SourceCarriedControlMutation::RefuseClassifiedCarried` |
| `lowering/core.rs:6181` | **yes** | `LoweringOperand::Specialized(_)` — a **wildcard**, in the source machine's `Match` scrutinee continuation |
| `lowering/core.rs:13919` | **yes** | `Specialized(Lowered::Constructor { .. })` **destructure else** — generic ordinary `Match` |

> ### `6164` IS A TEST MUTATION HOOK, NOT A THIRD FALLTHROUGH SEAT
>
> **This matters twice over.** It is not a candidate raise site, so the
> production inventory is **two**, not three. And because the hook fabricates
> the **identical** construct and reason, **any control keyed on that pair
> cannot distinguish the production refusal at `6181` from the injected
> mutation at `6164`.** A successor control that keys on the string will pass or
> fail for the wrong reason. Key on the **seat**, or on the measured operand.

> ### `Specialized(_)` IS A REMAINDER, NOT A CLASS — AND NOT A PRE-SHAPE WILDCARD
>
> **Corrected by Architect ruling `evt_3cmv9e3kms2bx`; the Steward's earlier
> reading of this seat was wrong and is withdrawn.** The arm is **not** a
> blanket refusal before any shape test, and it does **not** refuse a
> `Specialized(Lowered::Constructor)` — that is accepted at `core.rs:6086`.
>
> It is **the remainder after explicit acceptance** of six `Specialized`
> variants. **The reading is the Architect's**; the enumeration below is the
> grounding measurement, taken on `876450ab` (`evt_46t985br8ek23`):
>
> | line | arm |
> |---|---|
> | `5973` | `Specialized(Lowered::BoundedNat(..))` |
> | `5985` | `Specialized(Lowered::StructuralNat(..))` |
> | `5997` | `Specialized(Lowered::Bool { .. })` |
> | `6054` | `Specialized(Lowered::HostResult { .. })` |
> | `6075` | `Specialized(Lowered::DynamicConstructor(..))` |
> | `6086` | `Specialized(Lowered::Constructor { .. })` |
> | `6155` | `Carried(word)` |
> | **`6178`** | **`Specialized(_)` — the remainder, the firing arm** |
>
> **So the remainder still covers materially different scalar, capability,
> aggregate, closure, protocol and trap variants**, and which one arrives is
> precisely the unmeasured fact.
>
> ⇒ **`Specialized` is a phase, not an answer.** Any statement about whether the
> refusal's message is true here requires the exact `LoweredVariant`.
>
> **One source, then measured — not two sources agreeing.** The remainder
> reading came from the ruling alone; the ring's later post supplied the
> enumeration and explicitly declined credit for the reading, having named the
> arm without characterising it. **Recorded because a claim that looks
> independently corroborated is weighed more heavily than one that is not**, and
> here there was one source until the measurement made it grounded.

**There is a documented precedent at this exact seat.** The `Carried(word)` arm
immediately above carries a comment saying that without it the value *"fell past
every shape test onto the refusal below — a true sentence about the wrong thing,
naming a cause that is not the cause: the value is fine, the question is."* It
also records that the generic `lower_expr` `Match` emitter and the source
machine's `ComputationalMatchScrutinee` both already carried that arm, and
**this seat was the only one of the three missing it.**

⇒ **This refusal has already once named a cause that was not the cause, at this
seat, for an adjacent operand class.** **Weigh it as a prior. It is not a
finding, and it must not be used to skip the measurement** — the Architect ruled
that choosing consumer widening, upstream composition or terminal propagation
without the variant and producer would repeat the representation guess this stop
exists to prevent.

## Ownership, as narrowed — read this before the deliverable

**Architect ruling `evt_3cmv9e3kms2bx` settled the seat and the stage.** This is
**Runtime's source-machine ORDINARY-`Match` specialized-scrutinee boundary**, at
`lowering/core.rs:6178-6183` in `SourceContinuation::MatchScrutinee`, operand
phase `LoweringOperand::Specialized`, reached by
`lower_source_machine_with_continuation -> lower_source_machine ->
lower_carried_computational_match -> lower_computational_match_expr ->
define_unit_body`. **The candidate did not create or alter it.**

**Four owners are ruled OUT, by seat and not by resemblance:**

| not | because |
|---|---|
| Kernel | it has already admitted, erased and interpreted the witness |
| `runtime_ir_evaluator` | no inner `RuntimeIrEvaluation` error and no re-stamp |
| [[RT-COMPMATCH-TREE-SCRUTINEE]] | it owns `SourceContinuation::ComputationalMatchScrutinee`, a different arm |
| #6g [[RT-SPECIALIZED-ACTIVE-RESUME]] | its seat is `lower_computational_match_value_composed` with a first `Active` frame |

## `D0` — the four-field measurement, and NOTHING ELSE

**`876450ab` stays unchanged. Measurement only. DO NOT FRAME OR ATTEMPT A
PRODUCTION REPAIR** — the Architect authorized this node's `D0` and withheld
everything past it.

1. At the firing arm, record **`lowered.variant()` exactly**, not merely
   `Specialized`.
2. Record the ordinary `Match`'s **`static_origin`**, its **case
   constructors**, and the **immediately preceding source-machine
   value-producing occurrence/route**.
3. Record the **continuation stack at arrival**.
4. **Preserve the exact refusal and stop.**

**The backtrace identifies the CONSUMER, not the producer.** That is why field 2
is not optional.

> ### THE DISPOSITION IS DETERMINISTIC — IT IS ALREADY DECIDED BY THE VARIANT
>
> - **`ProcessExitStatus`** ⇒ route the new row into the existing **`draft`
>   `RT-PROCESS-EXIT-STATUS`** for a population/seat recut. **Do not create an
>   eighth mechanism, and do not teach this consumer to accept it by
>   assumption.**
> - **any other variant** ⇒ **return for the fresh mechanism ruling** the
>   Architect's prior stop requires. **Do not relabel it into a nearby node by
>   message similarity.**

> ### THE SEAT IS SETTLED. DO NOT RE-MEASURE IT.
>
> Routing measurement `evt_7j7jc1kj2vqsw` instrumented the `surface::unsupported`
> constructor with a forced backtrace — exhaustive, rather than probing tabled
> guesses — and found **one** raise firing **once**, with the path above. The
> Architect accepted those facts in `evt_3cmv9e3kms2bx`.
>
> **The `6164` control hazard did not confound it, and this was checked rather
> than assumed.** `source_carried_control_refusal` builds its refusal through
> **the same `surface::unsupported`** (`core.rs:2119`), so the hook was **inside
> the probe's field of view, not outside it**. No mutation was armed, and the
> backtrace carries **no `source_carried_control_refusal` frame** — so the
> attribution rests on the frame list and the arming state, **not on
> elimination**.
>
> **That instrument avoided the trap by placement, not by design. The next one
> must avoid it by design** — see `AC-7`.

**The variant and the producer are the whole substance of `D0`.** The Architect
was explicit that `Specialized(_)` does not reveal whether the rejected value is
a **boundary value, process result, protocol marker, or another `Lowered`
class**, and that this decides whether an existing composition/elimination
authority should have caught it or whether this is a new one.

## Acceptance

| AC | criterion |
|---|---|
| `AC-1` | **`lowered.variant()` recorded exactly** at the firing arm — `Specialized` alone does not discharge this |
| `AC-2` | The ordinary `Match`'s `static_origin`, case constructors, and the immediately preceding source-machine value-producing occurrence/route are recorded |
| `AC-3` | The continuation stack at arrival is recorded |
| `AC-4` | **No production change.** The fail-closed refusal preserved exactly; test-only, `crates/` behaviour unchanged |
| `AC-5` | The deterministic disposition is applied: `ProcessExitStatus` routes to `RT-PROCESS-EXIT-STATUS`, anything else returns for a fresh mechanism ruling. **Neither is chosen by resemblance** |
| `AC-6` | No consumer widening, Kernel workaround, selection/disposition change, or graph inference |
| `AC-7` | **No control keys on `("Match", "scrutinee is not a constructor value")`** — the `6164` mutation hook produces that pair by construction |

## Forbidden

**Explicitly unauthorized by the ruling:** consumer widening; a Kernel
workaround; any selection or disposition change; graph inference. Also: keying
a control on `("Match", "scrutinee is not a constructor value")`, which the
`6164` mutation hook also produces.

## Sequencing

**Runtime is single-threaded and this contends with the lane
[[RT-BODY-OCCURRENCE-PROVENANCE]] is on.** Release only after that partial
lands. Re-derive contention against the then-current tree rather than trusting
this sentence — a disjointness claim of the Steward's has died twice on this
chain.
