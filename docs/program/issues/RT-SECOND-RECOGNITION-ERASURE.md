---
id: RT-SECOND-RECOGNITION-ERASURE
title: "Erase the second static-worker recognition for row4 depths 2 and 3, whose worker was already emitted and lawfully consumed, without relaxing the ledger's law or borrowing that consumption to transition a second recognition"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-EMITTED-WORKER-CONSUMER-WALK]
blocks: []
github: null
origin: "Architect ruling evt_3cxm6654d5cjb, 2026-08-15, splitting the population measured by RT-EMITTED-WORKER-CONSUMER-WALK D0. Every symbol below was located by name against origin/main ac8a73d1b by the Steward before filing, and the ruling's mechanism attribution is corrected here on that basis. Steward-filed per COORDINATION section 2."
---

## This is HALF a population. The other half is not ruled and is not here.

`RT-EMITTED-WORKER-CONSUMER-WALK` `D0` measured that the five governed
expressions are **two different situations**, and the Architect ruled only one
of them:

- **row4-depth-2, row4-depth-3** — a worker **is** emitted and **is** consumed,
  by `SourceContinuation::CallArgument`. A **second, different** recognition is
  then minted, and `StaticWorkerFieldLedger::close` refuses that one. **This
  node.**
- **row4-depth-1, row5-after-hole** — never enter either emitter. Nothing
  emitted, nothing consumed, **so no positive authority for erasure exists.**
  [[RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]], and it must be measured before
  anyone proposes a repair.
- **row1-owned-scope** — retains its `NativeJoinPlanV1` refusal, a different
  site entirely and outside both nodes.

**Nothing here depends on the other half's answer.** The Architect said so
explicitly, and this node must not wait on it.

## THE RULING'S MECHANISM ATTRIBUTION IS CORRECTED HERE. Read this before coding.

**The ruling says `constructor_field_bindings` (`mod.rs:4936`) "mints a
recognition for a field whose worker was already emitted and lawfully
consumed."** Measured against `origin/main` `ac8a73d1b`, that function does not
mint recognitions. **It is the transport event**, and its own body says so:

```rust
// **THE TRANSPORT EVENT.** The field enters lexical binding authority here
let transport = self.static_worker_fields.rebind(*recognition, self.defining_function_id)?;
```

**The production call sites are each exactly one, and they are in different
files:**

| ledger operation | sole production call site | enclosing function |
|---|---|---|
| `recognize` (mint) | `core.rs:15579` | `static_worker_constructor_template` (`core.rs:15545`) |
| `rebind` (transport) | `mod.rs:4958` | `constructor_field_bindings` (`mod.rs:4936`) |

Every other `.recognize(` / `.rebind(` hit in the tree is in
`core/tests/control.rs`. **Verify this before relying on it** — it is the
premise the deliverable is cut against, and this node's predecessor exists
because a cited coordinate was read past its warrant twice.

⇒ **The erasure goes at or before `core.rs:15579`, not at
`constructor_field_bindings`.** A ring that looks for a mint in
`constructor_field_bindings` will find only `rebind` and stall. **The ruling's
direction is right; its coordinate is not.**

## The consequence that makes this repair groundable, and it is not "which row"

`core.rs:15579` is the **same** mint that serves row4-depth-1 and
row5-after-hole. **A predicate keyed on the row would be a guess and would
reach into the unruled half.**

**It does not need to be.** `recognize`'s own doc states the enabling fact:

> **Never deduplicated by `field_origin`.** Two constructions of one occurrence
> are two constructed workers, each owing its own transition and its own
> consumption.

⇒ For depth-2/3, `static_worker_constructor_template` mints **twice for one
occurrence** — once for the worker that is emitted and consumed, once during
root-adapter lowering. **The second mint is the defect.**

⇒ **The predicate is "has this occurrence's worker already been emitted and
consumed?", not "is this row 4 at depth 2".** That is precisely the positive
authority the ruling identified, it is available at the mint, and it is
**self-limiting**: for row4-depth-1 and row5-after-hole no prior consumption
exists, so the predicate declines and the unruled half is untouched by
construction rather than by a scope rule.

## Why erasure and not transition, which is the half of the ruling to preserve

`D2k-1c-1` hardened `close` into an **agreeing** bijection against exactly this:

> `range(transitioned) ⊆ dom(minted)` still admits
> `transitioned[r1] = transitioned[r2] = T`: **two constructions sharing one
> transport, discharged by that transport's single consumption.**

**That is depth-2/3 exactly** — two constructions, one real consumption. Making
the second recognition transition could only discharge it by leaning on the
first's consumption, **which is the hole `D2k-1c-1` was written to close.**

⇒ **Transition here does not merely lose to erasure on merit; it reopens a
fixed defect.** Erasure narrows the producer and leaves the ledger's law
untouched, which is the correct direction when the ledger is doing its job.

## Deliverables

**`D0` — erase the second recognition at or before its mint**, gated on the
measured prior emission-and-consumption of that occurrence's worker. **Not on
the row, not on a depth, not on a constructor name.**

**`D1` — the control.** Row4-depth-2 and row4-depth-3 reach a disposition that
is not a `StaticWorkerBinding` refusal, and **the control must fail against the
tree as it stands today.** Demonstrate both halves; do not argue either.

**`D2` — correct the refusal message, which is now FALSE AS APPLIED.**
`close` link one says the field is *"neither consumed at an exact-`Var` call nor
erased before construction"* and that a constructor carrying it *"has no runtime
representation."* **For depth-2/3 the worker IS consumed** — measured. The
message should say what it knows: **this recognition never transitioned.** It
must stop asserting a general impossibility.

## Acceptance criteria

**`AC-1`.** The ledger's law is unchanged. **No relaxation of `close`, no second
writer of `consumed`, no widening of the agreeing bijection.** A repair that
edits `close` is the wrong repair.

**`AC-2`.** The erasure carries **positive authority at or before
construction** — the measured consumption — rather than borrowing a later one.
**State in the code which consumption authorizes it.**

**`AC-3`.** Row4-depth-1 and row5-after-hole are **behaviourally unchanged**.
Demonstrate it: their refusal is unruled and this node must not move it. **If
the predicate changes them, the predicate is keyed wrongly.**

**`AC-4`.** `D2k-0` still reds if any edge or refusal moves for the rows this
node does not govern. **A red there is information, not a test to update.**

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Relaxing or editing `close`'s law.** `AC-1`, and it is the point.
- **Anything keyed on row identity or depth.** The predicate is about prior
  consumption.
- **The depth-1 / after-hole half.** [[RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]].
- **`D2k-1c`**, the planner-owned `ContinuationTemplate` population, and the
  continuation-source surface, all still outside.

## Sequencing

**On the critical path.** [[RT-LEXICAL-RECURSOR-CONSUMERS]] is the only
un-merged `depends_on` of [[RT-RECURSOR-TRANSPORT]], whose `D3` is the joint
retirement that [[RT-DESCENT-RETIRE]] closes. **This node and
[[RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]] are independent and may run
concurrently or in either order.**

## The finding worth carrying past this node

**A limit stated on one axis of a two-axis claim gets read as universal on the
other.** The `D2k-0` rider hedged *"among the sites that were TAGGED"* — a limit
on the **interval** — and both the Architect and the Steward read the
**population** as universal. Only two of five reach an emitter.

⇒ **A measurement report must say which rows it observed and which it did not,
and a reader must refuse to generalize one that does not.** Twice on this node a
statement about instrumented sites became a statement about the population.
