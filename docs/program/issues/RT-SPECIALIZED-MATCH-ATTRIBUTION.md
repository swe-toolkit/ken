---
id: RT-SPECIALIZED-MATCH-ATTRIBUTION
title: "A Match scrutinee arriving as a Specialized operand is refused before any shape test, and the outer stage does not say which seat or which Lowered class"
status: ready
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

> ### `6181` AND `13919` DIFFER IN A WAY THAT LIKELY DECIDES THE MECHANISM
>
> `13919` destructures for `Constructor` and refuses what is not one — the
> message is true there. **`6181` refuses `Specialized(_)` as a wildcard**, so
> it refuses **even a `Specialized(Lowered::Constructor)`**, before any shape
> test runs. Whether the message is true at `6181` therefore depends on the
> operand, and that is exactly what has not been measured.

**There is a documented precedent at this exact seat, and it is the same shape.**
The `Carried(word)` arm immediately above `6181` carries a comment saying that
without it the value *"fell past every shape test onto the refusal below — a
true sentence about the wrong thing, naming a cause that is not the cause: the
value is fine, the question is."* It also records that **the generic `lower_expr`
`Match` emitter and the source machine's `ComputationalMatchScrutinee` both
already carried that arm, and this seat was the only one of the three missing
it.** ⇒ **This refusal has already once named a cause that was not the cause, at
this seat, for the adjacent operand class.** Weigh that as a prior; it is not a
finding.

## Deliverable — ONE, and it is measurement before mechanism

The Architect's ordered list, verbatim in substance:

1. **Identify the exact raise seat.**
2. **Record the `LoweringOperand` phase and the exact `LoweredVariant`.**
3. **Record the immediate producer and the continuation/frame, if any.**
4. **Preserve the current fail-closed refusal.**
5. **Then route to an existing authority only if that measured class is already
   in its population; otherwise return for a fresh mechanism ruling.**

> ### DELIVERABLE 1 MAY ALREADY BE DISCHARGED — CONFIRM, DO NOT REDO
>
> Routing measurement `evt_7j7jc1kj2vqsw` instrumented the `surface::unsupported`
> constructor with a forced backtrace — exhaustive rather than probing tabled
> guesses — and identified **one** raise, firing **once**, at `6181`:
>
> ```
> surface::unsupported
>   <- lower_source_machine_with_continuation   <- the raise
>   <- lower_source_machine
>   <- lower_carried_computational_match
>   <- lower_computational_match_expr
>   <- units::define_unit_body
> ```
>
> **That measurement and the ruling crossed in the thread**, so the ruling's
> site enumeration was written without it. **Deliverables 2 and 3 are
> untouched by it** — a backtrace names the seat, not the operand.

**`2` and `3` are the substance.** The Architect was explicit that the evidence
does not reveal whether the rejected specialized value is a **boundary value,
process result, protocol marker, or another `Lowered` class**, and that this
distinction decides whether an existing composition/elimination authority should
have caught it or whether this is a new one.

## Acceptance

| AC | criterion |
|---|---|
| `AC-1` | The exact raise seat is confirmed by instrumentation, not by string or construct match |
| `AC-2` | The `LoweringOperand` phase and the exact `LoweredVariant` are recorded |
| `AC-3` | The immediate producer and the enclosing continuation/frame are recorded |
| `AC-4` | **No production change.** The fail-closed refusal is preserved exactly; test-only, `crates/` behaviour unchanged |
| `AC-5` | Either the measured class is shown to be **already in** a named existing authority's population, or the node returns for a fresh mechanism ruling. **Neither is chosen by resemblance** |
| `AC-6` | No consumer widening, Kernel workaround, selection/disposition change, or graph inference |

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
