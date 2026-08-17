---
id: RT-STATIC-WORKER-ARM-SOURCE-WITNESS
title: "Find one Ken SOURCE program that reaches the StaticWorkerBinding conservation arm specifically -- the existing bound is a search over hand-authored fixtures, and the six ignored ken-cli tests are NOT a ready-made corpus because they fail upstream at the sibling Closure arm"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-DESCENT-RETIRE]
blocks: []
github: null
origin: "Architect ruling evt_45fgeg9j7xhpd (2026-08-17), disposition 2, on Adversary hunt evt_6x0ewnvnvdq3j against the RT-DESCENT-RETIRE D3-D6/D8 landing 1aec3e3e1. Steward-filed per COORDINATION section 2."
---

> # THIS DOES NOT BLOCK ANYTHING, AND IT IS NOT A CORRECTION TO A LANDED CHANGE.
>
> **The classification it touches already stands on an independent fact.** Row
> 4 depths 1-3 moving from compiling to `Unsupported(StaticWorkerBinding)` is a
> **representability gap, not a capability loss**, because the six `ken-cli`
> tests in question were **already `#[ignore]`d at the retirement base
> `fe5778ef7`** and the population is identical at the landed squash
> `1aec3e3e1` — 6 and 6, checked at both ends.
>
> ⇒ **A test already excluded before the change cannot be capability the change
> removed.** No tightening of the bound below can disturb that.
>
> **So this node exists to size remaining risk honestly, not to unblock or to
> repair.**

## What is already answered, so this does not re-answer it

[[RT-STATIC-WORKER-WITNESS-PROGRAM]] is `closed`, **ANSWERED as a bounded
negative**: *"no reasonable Ken source program was found that reaches the
static-worker conservation refusal — every attempt that keeps the constructor
live in the executable closure also supplies a lawful disposition, and that is
a search bound, not a universal proof."*

**That verdict is honest and it is not being reopened.** What is new is the
**domain** of the bound, which was nowhere in the tree until 2026-08-17.

## The domain, stated arm-level

`boundary_transfer_admissibility` (`mod.rs:11846`) carries sibling arms, and
`D1` recorded them as **one law at two callable kinds**:

| arm | what refuses | status |
|---|---|---|
| **Closure / DeclarationClosure** | *"a runtime-local closure has no durable lane across the boundary"* | **source-reachable** — six `ken-cli` tests reach it and are `#[ignore]`d for it |
| **`StaticWorkerBinding` conservation** (`close()`) | the conservation-ledger refusal row 4 now reaches | **no source witness known** |

**A shared reason never licenses shared coverage.** That rule is why a separate
pin per construct was demanded, and it cuts the same way here: *"excluded for
the Closure arm"* is **not** *"would have reached the `StaticWorkerBinding`
arm."*

⇒ **The useful fact, and the reason this node is worth filing:** the six
exclusions **establish that the shared law IS source-reachable at the other
arm.** That does not transfer, but it is exactly what a reader needs to judge
how likely a witness is at this one.

## Deliverable

**`D1` — one Ken SOURCE program that reaches the `StaticWorkerBinding`
conservation arm.** Not a hand-authored `RuntimeExpr` fixture; a program that
enters through a real source layer.

**A bounded negative is an acceptable outcome** — but it must state its search
domain this time, at arm level, rather than leaving the next reader to
discover it.

## The cost is NOT what the finding reported. Read this before scoping.

**Un-ignoring the six does not answer the question**, and *"cheap relative to
what just landed"* is not established (Architect, `evt_45fgeg9j7xhpd`).

**They fail at the Closure arm, which is UPSTREAM.** A program refused there
**never reaches the conservation ledger**, so removing their `#[ignore]`
produces failures at the wrong arm and no evidence at the right one.

⇒ **They are not a ready-made corpus.** Scope the search at the
`StaticWorkerBinding` arm specifically, or this node will spend its turn
re-answering the question [[RT-STATIC-WORKER-WITNESS-PROGRAM]] already
answered.

## Acceptance criteria

- **`AC-1`.** Either a source program reaching the `StaticWorkerBinding`
  conservation arm is exhibited and committed, **or** a bounded negative is
  recorded **with its search domain stated at arm level.**
- **`AC-2`.** The record says explicitly whether the six Closure-arm exclusions
  were reachable-but-refused-upstream, so no later reader re-derives that they
  are a usable corpus.
- **`AC-3`.** No change to the `RT-DESCENT-RETIRE` classification. **This node
  cannot reclassify the representability gap** — that rests on the
  already-excluded-at-both-ends fact, not on this search.

## Banned scope

- **Un-ignoring the six `ken-cli` tests as a means to this end.** If they are
  un-ignored for an unrelated reason, that is a different node.
- **Repairing either arm.** Both refusals are ratified dispositions.
- **Reopening [[RT-STATIC-WORKER-WITNESS-PROGRAM]].** It is answered.
