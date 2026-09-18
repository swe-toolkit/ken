---
id: RT-COMPMATCH-TREE-SCRUTINEE
title: "ComputationalMatch refuses a tree-producing scrutinee that is not Bool or a constructor (rt_span_prov)"
status: draft
owner: runtime
size: TBD
gate: none
depends_on: [RT-SRCBODY-BIND-ORDER]
blocks: []
github: null
origin: Measured by the RT-SRCBODY-BIND-ORDER D12 complete no-fail-fast enumeration (evt_2n9wq8xyj0aa1). Fails at frozen base 21fd46dc as well as at the candidate, so it is pre-existing base debt and not a regression. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## THE FRAME IS OWED. `draft`, NOT startable.
>
> It exists so a skipped CI row has an owner. **A skipped row measures nothing;
> this node owns un-skipping it.** Size is `TBD` deliberately.

## Exact signature — MEASURED, and the row now stops EARLIER

> **The row no longer reaches the refusal this node is named for.** Read this
> section, not the title.

**Measured at `04d4dd38a9cfb40da9b5d68bfeb42aaddbb76661`** by
`RT-IGNORED-FAILING-ROWS-INVENTORY` (row 14 of its ledger,
`docs/program/evidence/rt-ignored-failing-rows-ledger.md`):

```text
an exact detached required consumer has no computational occurrence
```

The ledger records `label agrees? NO`, noting the label predicts **a
tree-producing match scrutinee**. That is the refusal below, and it is not the
one the row hits first.

**Superseded as the FIRST refusal, not disproved as a property of the row:**

```text
ComputationalMatch: tree-producing match scrutinee is not Bool or a constructor
```

> ### THIS DOES NOT REFUTE THE WITNESS CLAIM BELOW. IT STOPS DEMONSTRATING IT.
>
> The section that follows makes this node **load-bearing for the descent
> campaign** — *"the sole failure was this node"*, *"a real Ken program producing
> a non-constructor `ComputationalMatch` scrutinee"*. **That argument requires
> the row to actually reach the `ComputationalMatch` consumer.** It now stops at
> an earlier, different refusal, so the row is no longer observed doing the
> thing the claim rests on.
>
> **Refuted and no-longer-demonstrated are different, and the difference is the
> whole point.** Signature depth is one layer: the ledger is explicit that a
> first stop is not a root cause and that forcing past a first refusal on other
> rows revealed a second behind it. The scrutinee-shape refusal may well still
> be there, one layer down. **Nobody has looked.**
>
> ⇒ **`D0` for this node is: force past the detached-consumer refusal and record
> whether the `ComputationalMatch` scrutinee refusal is still behind it.** Until
> that is run, **the descent campaign's Trap 1 answer — "yes, real programs
> exhibit this shape" — rests on a row that is no longer seen exhibiting it.**
> Do not cite this node as that witness in the meantime, and do not delete the
> claim either; it is unconfirmed, not wrong.
>
> **The reason this sat unnoticed is worth keeping:** the row kept failing the
> whole time. **A load-bearing claim resting on a still-red row looks exactly
> as healthy as one resting on a row that is red for the right reason.**

## IT IS THE DESCENT CAMPAIGN'S ONLY REAL-PROGRAM WITNESS FOR THIS CLASS

Steward, 2026-08-08, `evt_27jwdbz9h2t4c`, routed from
[[RT-SPECIALIZED-ACTIVE-RESUME]]'s cross-crate census.

That census ran the descent campaign's boundary instrument inside `ken-cli` and
`ken-verify` with `--include-ignored`. **The sole failure was this node** — and
that makes it load-bearing for a question the campaign has carried since node
#6b.

The campaign's Trap 1 is *"a hand-built `RuntimeExpr` fixture proves the
classifier sees the class, not that a real Ken program exhibits it."* Its
population is two hand-built values. **This node is a real Ken program producing
a non-constructor `ComputationalMatch` scrutinee** — the same class, failing at a
**different consumer**.

> ### IT SPLITS A QUESTION THAT WAS BEING CARRIED AS ONE
>
> - *Do real programs exhibit this shape?* **Yes** — this node is the witness.
> - *Is the descent campaign's specific cell reachable in production?* **No**,
>   and that is a property of the `#[cfg(test)]` activation seam, **not of the
>   language.**
>
> Only the first was ever what Trap 1 was about. Recording the split here
> because this node is where a reader arrives holding the witness.

**This does not make the node startable and does not change its owner.** It
raises what closing it is worth: it is the only place the class can be studied
on a real program rather than a fixture.

## Why it is not one of the released owners

**Distinct refusal class.** Not an effect-seat \`Need\`/\`Avail\` membership
question, so it is neither [[RT-CARRIER-BYTESPAN-OBSERVE]] nor
[[RT-CARRIED-RESOURCE-SCALAR]]. This is the scrutinee-shape refusal named in
[[RT-CONTSRC-PRODUCER-LOCAL]]'s two-population split, where it was the
signature of the **five** rows rather than the \`AC-1\` row.

## Provenance

**Fails at frozen base `21fd46dc`.** The complete surface is 40 candidate
failures, all of which fail at the base — **zero regressions**, and the
candidate additionally **fixes six** base failures. Enumerated with
`--no-fail-fast`, which is a closed enumeration because fail-fast is per
**binary**, not per test.
