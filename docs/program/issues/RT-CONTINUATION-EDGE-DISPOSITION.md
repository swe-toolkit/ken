---
id: RT-CONTINUATION-EDGE-DISPOSITION
title: "One planner edge carries both binding projection and a causal call obligation — split the representation so a binding candidate can be settled InlineNoCall without ever entering the call-discharge partition"
status: draft
owner: runtime
size: TBD
gate: none
depends_on: [RT-CONTINUATION-CALL-DISCHARGE]
blocks: [RT-MATCH-RECURSOR-CONSUMERS]
github: null
origin: Architect hard-stop ruling evt_dakdkqk4wbg6 (2026-08-08), which accepted the held red control a15a3e934766a1d075386ba561a9469e51a448b7 as load-bearing and withdrew the planner-side option-3 mechanism it had previously ruled at evt_4ebpfvfrvv8qy. Predecessor RT-CONTINUATION-CALL-DISCHARGE delivered D0/D1 attribution; its D2/D3 are re-homed here. Campaign docs/program/16-recursive-descent-retirement.md node #6i. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # THIS IS THE SEVENTH WALL, AND IT IS A REPRESENTATION SPLIT, NOT A REPAIR.

The first four were `BoundaryCarrier` refusals about how a carried operand may
cross or be consumed. The fifth was the value shape of a scrutinee after
ordinary lowering. The sixth ([[RT-CONTINUATION-CALL-DISCHARGE]]) asked whether
the planned population was discharged, and answered it: **no call occurred.**
**This one is what that answer costs**, and it is the first wall on this chain
whose deliverable is a new representation rather than a correction to an
existing one.

## The owned fact

**One planner edge carries two roles, and bridge selection cannot distinguish
them.**

| role | who needs it |
|---|---|
| **binding projection** | the deferred constructor environment needs the worker provenance to install IH / static-worker bindings at recursive positions |
| **causal call obligation** | only a direct specialization call, or a verified composed raw-worker call, owes a `ContinuationCallIdentity` discharge |

**Thirty-four bridge-taken edges are genuinely compositionally consumed**, so the
bridge is not a proxy for the distinction. Restricting to the ordinary arm does
not separate them either: the ruled witness and `d8e` have **identical planner
coordinates**, and the semantic difference appears only when the arm body
resolves its de Bruijn callee against the materialized environment.

> ### THE TWO OBVIOUS NARROWINGS ARE REAL FAILURES, NOT MISSING PREDICATES
>
> - Removing the edge **before interning** loses the binding, so `d8e` compiles
>   with a **shifted environment**.
> - Removing only `calls.insert` leaves an **interned-unit / caller population
>   contradiction**.
>
> **Do not look for a stronger source-side predicate.** That is the move this
> node exists to rule out; it was tried, and the failure is in the
> representation, not in the sharpness of the test.

## The mechanism the Architect ruled

**A candidate/disposition layer IN FRONT OF the unchanged causal-call
partition.** The planner mints an opaque **binding candidate** carrying the
exact worker provenance and selector. Its existence **authorizes environment
installation but does not assert a causal call.**

Lowering settles each candidate **exactly once**, from an event only lowering
can observe:

| disposition | settled when |
|---|---|
| `DirectCall` | at the verified direct producer / call seat |
| `ComposedCall` | only after the raw-worker call is emitted **and enters the existing finished-CLIF verification** |
| `InlineNoCall` | only after the exact deferred bridge scope **completes successfully** with that candidate still unconsumed |

A **static-worker binding carries the candidate authority.** Actual
source-machine consumption promotes it to `ComposedCall`; a **value-position
read still reaches the existing fail-closed `StaticWorkerBinding` guard**, so
`d8e` must retain binding count 1 and refuse.

**Closeout requires an exact, disjoint disposition for every candidate first.**
It then derives the call-obligation subset from `DirectCall ∪ ComposedCall` and
applies the existing law **unchanged**:

```
call obligations = direct-emitted  ⊎  composed-consumed
```

> ### `InlineNoCall` IS NEVER A DISCHARGE AND NEVER ENTERS THAT EQUALITY
>
> **This is deliberately NOT "add a third discharge form".** A third arm in the
> partition would falsify the meaning of the call ledger — it would let a
> program with no call satisfy a law that exists to say a call was answered.
> The new layer sits **in front of** the partition; the partition itself is
> untouched.

## Measurements come before mechanism

1. **Census the full candidate/unit population** by installed binding, direct
   emission, verified composed consumption, successful inline completion, and
   unresolved-or-double disposition.
2. **Preserve the four-cell `d8e` table as the primary discriminator.** Both
   classified variants keep **one** binding; index 1 may finish inline, while
   index 2 **must still refuse in value position**.
3. **Measure declaration/definition and ABI reachability for `InlineNoCall`
   candidates.** If permitting a binding-only candidate requires a post-lowering
   **call-graph rebuild** or changes the **planner traversal contract**, **stop
   again** rather than silently allowing an uncalled executable unit.
4. **Five mutations must independently red:** suppress binding installation;
   mark inline **before** bridge completion; mark inline **after** a composed
   call; omit a final disposition; present one candidate in **two** dispositions.
5. **Untouched until the split representation proves otherwise:**
   `ContinuationClaimLedger::close`, finished-CLIF direct and composed
   verification, the both-sets refusal, the `composed` feed, the empty resume,
   and all five landed repairs.

## Scope

Gates completion of [[RT-MATCH-RECURSOR-CONSUMERS]] and its `AC-1`. Does **not**
reopen [[RT-SPECIALIZED-ACTIVE-RESUME]]'s accepted `D2`/`D3`,
[[RT-CONTINUATION-CALL-DISCHARGE]]'s `D0`/`D1`, or any of the five landed
repairs. Does not touch rows 1-5 or the `LexicalCallArgumentRecursor` population
([[RT-LEXICAL-RECURSOR-CONSUMERS]]).

> ### THE PREDECESSOR IS NOT WRONG, AND ITS `D0`/`D1` ARE THE INPUT HERE
>
> **A seventh authority is a normal outcome on this chain, not a defect in the
> sixth.** The campaign's standing record is that the expensive mistake has
> always been treating a new authority as a fault in the previous repair. The
> exact-witness conclusion **"no call occurred" is unchanged and is load-bearing
> for this node** — it is why `InlineNoCall` must exist at all.

## Sizing is `TBD` on purpose

**Do not inherit the predecessor's `S`.** That size was granted against an
edge-exclusion repair that the ruling withdrew. This node's work is a new
representation plus a five-mutation proof obligation, and measurement 3 carries
a **named hard stop** that would fork it again. **Size it on its frame, after
the census in measurement 1.**

Frame: to be written by the Steward before release.
