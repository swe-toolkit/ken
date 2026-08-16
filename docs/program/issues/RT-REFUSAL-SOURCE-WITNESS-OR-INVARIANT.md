---
id: RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT
title: "For every refusal still holding the RecursiveDescent retirement, exhibit a Ken source program that reaches it or establish that none exists -- and record the ones with none as compiler invariants"
status: ready
owner: runtime
size: L
gate: none
depends_on: []
blocks: [RT-LEXICAL-RECURSOR-CONSUMERS, RT-RECURSOR-TRANSPORT, RT-DESCENT-RETIRE]
github: null
origin: "Operator, 2026-08-16, verbatim: the row 'represented a genuinely invalid compiler state, and its generation was due to careful creation of a fixture to trigger the error, and that such an arrangement likely could not be created by a ken surface level program, and that we should accept the refusal as a proper compiler assert/invariant. I think it was on you to dispatch the runtime team to look for a ken source program that could generate that state, or declare that such a program did not exist. At that point all conditions would be met and we could proceed to retire RecursiveDescent.' Steward-filed per COORDINATION section 2. Population measured against origin/main 4c5de1793."
---

## What this node is

**For each refusal still standing between the fleet and
[[RT-DESCENT-RETIRE]], answer one question: is there a Ken source program that
reaches this state?**

- **Yes** — it is a real capability gap. Repair it, or route it, and say which.
- **No** — the refusal is **a proper compiler assert/invariant**, recorded as
  such. The pinning control stays, now honestly described as pinning the
  lowering's **internal** contract rather than a surface capability.

**When every member of the population below has one of those two dispositions
on the record, the retirement's conditions are met** and
[[RT-LEXICAL-RECURSOR-CONSUMERS]] can close, unblocking
[[RT-RECURSOR-TRANSPORT]] and then [[RT-DESCENT-RETIRE]].

## The population, measured, and it is four items not one

| # | expression | wall | current disposition |
|---|---|---|---|
| 1 | `row 4 depth 1` | `StaticWorkerBinding` **conservation** refusal | **none** |
| 2 | `row 5 after-hole` | `StaticWorkerBinding` **conservation** refusal | **none** |
| 3 | `row 1 owned-scope` | **`NativeJoinPlanV1`** — a different construct | **none**, and no increment has ever addressed it |
| 4 | the child arm reached by `row4-depth-2/3` | `Closure` | **partial** — see below |

Items 1-3 are [[RT-LEXICAL-RECURSOR-CONSUMERS]]'s own unaccounted population
(`:455-470`, `:511`, `:560`). **That node states in its own text that it cannot
close until these are dispositioned**, and it names a successor that merged
without carrying them — so today they are owned by nobody. That is the whole
reason lane 1 has no dispatchable work.

### Item 4 is the one the operator's instruction was about, and it is nearly done

[[RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE]] `D3` **already ran this exact search**
for `row4-depth-2/3` and reported at `af13cc7e5`: the governed producer is
test-only hand-authored `RuntimeExpr` (`lowering/core/tests/control.rs:2358`),
and both exact surface routes refuse before checked-artifact emission — naming
the W-style recursive result gives `Elaboration(StructuralResultOutOfScope)`,
recomputing it by self-call gives `KernelRejected(NotTerminating)`.

⇒ **That is the operator's "declare that such a program did not exist", and it
was delivered.** What stops it being final is stated in `D3`'s own post-merge
correction, which is why this node exists rather than a status flip:

> **"This said 'not reachable from source' and the evidence does not carry
> that."** The measurement's own selection rule **excludes the
> closure-at-boundary tests** — the very population at issue — so the
> defensible form is **"no witness in the measured corpus,"** and *"whether a
> program outside the corpus reaches them is open, and it is the successor's
> whole subject."*

**This node is that successor.** Item 4's remaining work is narrow: establish
the claim over a population the corpus does not exclude, or by an argument that
does not depend on a corpus at all.

## The method gate. Read this before designing the search.

> ### A NEGATIVE EXISTENCE CLAIM IS NOT ESTABLISHED BY FAILED ATTEMPTS.
>
> *"We wrote N programs and none reached it"* is consistent with the N+1th
> reaching it. **That is exactly how `D3`'s claim had to be walked back**, and
> it is the same defect that cost [[V3-FO-CHECKER-SOUNDNESS]] `D0` a
> hard-stop: a method adopted because it is stronger than reading source is
> **weaker** than reading source for the one claim that is negative.
>
> ⇒ **A "no such program exists" disposition must be argued from the
> surface grammar, the elaborator's admission rules, and the kernel gates** —
> the things that decide what a user can write at all — and not from a sample
> of attempts, however large. `D3`'s two refusals
> (`StructuralResultOutOfScope`, `NotTerminating`) are the right *shape* of
> evidence: they are gates, not failures to find.
>
> **A positive answer is the easy direction and needs no such care** — one
> `.ken` file that reaches the state settles it outright.

**Corollary worth stating because it inverts the usual instinct:** finding a
witness is a *better* outcome for this node than not finding one, because it is
cheap to establish and it converts an open question into a routed repair. **Do
not treat "no witness" as the target.**

## Deliverables

**`D1` — items 1 and 2, the two `StaticWorkerBinding` conservation refusals.**
Search for a Ken source program reaching each. Report a witness, or the gate
argument for why none can exist. **These two share a wall and may share an
answer; establish that rather than assuming it.**

**`D2` — item 3, `row 1` owned-scope at `NativeJoinPlanV1`.** This is a
different construct from every other member and no increment has touched it.
**Expect to spend the first part of this deliverable establishing what state the
refusal actually is** before searching for a program that reaches it.

**`D3` — item 4, closing `D3`'s corpus caveat.** Establish whether any
source-reachable shape reaches the same child arm, over a population the
measured corpus does not exclude. **Do not re-derive the `row4-depth-2/3`
result** — it is landed and it stands as far as it goes.

**`D4` — the dispositions, written where the closure criterion reads them.**
Each of the four gets a recorded disposition in
[[RT-LEXICAL-RECURSOR-CONSUMERS]]'s terms: repaired, routed, or **declared a
compiler assert/invariant with its control retained and re-described.** This
deliverable is what actually unblocks the chain; a finding that is measured but
not recorded where the criterion looks leaves the lane exactly where it is.

## Acceptance criteria

**`AC-1`. Every one of the four has a disposition.** A partial answer is a
partial result and is reportable, but the retirement's conditions are not met
until all four are recorded.

**`AC-2`. Every "no such program exists" carries a gate argument, not an attempt
count.** Name the grammar production, admission rule, or kernel gate that makes
the state unreachable. **A disposition resting on "we tried and could not" is
not accepted** — see the method gate.

**`AC-3`. Every "no such program exists" states the population it is over**, and
whether that population is the measured corpus, a superset, or a
corpus-independent argument. **`D3`'s walk-back is the precedent: the claim and
its population must travel together.**

**`AC-4`. Every witness found is an actual `.ken` file in the tree**, not a
described program, and the handback names the file and the exact refusal it
provokes.

**`AC-5`. Controls retained under an invariant disposition are re-described.**
A control pinning IR unreachable from source is pinning the lowering's internal
contract — a real thing to pin, and **a different claim from what its name
currently suggests.** State the new expectation and why the control is still
worth keeping.

**`AC-6`. A hard stop is a complete result.** If one of the four turns out to
need its own node, say so and stop; do not absorb it.

**`AC-7`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only. **This node is expected to land measurements and dispositions, and may
land no production change at all** — that is a success, not a thin result.

## Banned scope

- **Retiring anything.** [[RT-DESCENT-RETIRE]] deletes the selector, the enum,
  the authority and the lane. This node only establishes that its conditions are
  met.
- **Repairing a witnessed gap in the same candidate that finds it.** A witness
  changes the disposition from invariant to capability gap; the repair is then
  sized and routed on its own. Report it and hand back.
- **Re-deriving `D3`'s `row4-depth-2/3` measurement.** It is landed.
- **The `RecursiveDescent`-as-oracle framing.** Operator, 2026-08-15:
  `RecursiveDescent` is a failed implementation attempt, not a specification.
  **What a program does under `RecursiveDescent` is not evidence about whether
  it should compile** — the interpreter is the oracle, and for this node the
  question is narrower still: can a user write it at all.

## Sequencing

**`ready` at filing, `depends_on: []`.** Nothing gates it; the population is
measured and named above.

**It blocks the whole remaining retirement chain**:
[[RT-LEXICAL-RECURSOR-CONSUMERS]] closes on `D4`, which unblocks
[[RT-RECURSOR-TRANSPORT]] (its four other deps are merged), which unblocks
[[RT-DESCENT-RETIRE]] (its six other deps are merged).

**Lane 1 under the operator's 2026-08-15 two-lane directive**, and the operator's
2026-08-14 ruling that this path outranks every other Runtime node still binds.

> **Why this node exists at all, recorded so the gap does not reform.** Three of
> the four items have had no owner since [[RT-CROSSING-CALL-SITE-ATTRIBUTION]]
> merged without carrying the dispositions
> [[RT-LEXICAL-RECURSOR-CONSUMERS]] delegated to it. The blocked node kept
> advertising the work as *"dispositioned in the successor"* while the successor
> no longer contained it. **A defect spanning two nodes has a half owned by
> nobody**, and it read from outside as a campaign waiting on a decision.
