# ABI-S6 HS18 — closure mechanism, amendment 8

> **ERRATUM ON CURRENT `main`, AND IT CORRECTS AMENDMENT 7'S ORIGIN.**
> Amendment 7 (`8445b8833`) reinstated amendment 1's forward: the producer's
> context invokes the owning context and forwards *that* result. **The
> pre-build control amendment 7 carried forward has now refuted that
> forward's ORIGIN.** A seat may invoke only the continuation it itself
> constructed, so the producer's context is not entitled to invoke the owning
> context at all.
>
> **What survives from amendment 7:** the forward's DESTINATION, proved
> independently — owning context `ContinuationContextId(0)` carries Result
> contract `DenseRange { start: 4442, len: 38 }`, exactly the detached sink's
> independently derived demand, with zero edit to the finished identity chain.
> The environment axis stays settled. Emission ownership stays where it is.
> **What is withdrawn:** producer-originated forwarding, and any reading under
> which the producer seat holds authority over the consumer's world.

Architect, 2026-09-14. Rules HS12 on the fourth mandatory research advisory
and on the two pre-build controls that advisory made necessary. Grounded
against the implementer's exact WIP
`b601e2ec78989d32222b34b9ec68e01844c6d70b`, which is **not** a candidate.

## The rule

**A producer seat never invokes the consumer. Its result returns to the seat
holding the exact creation-site frame, and that seat originates the consumer
transfer.**

The fenced alternative, stated so it is refused rather than forgotten:
**closure-conversion — the continuation as a first-class runtime value
carrying its own captures — is NOT ruled.** It requires a transport that is
fenced out and absent from this WIP.

## Why twelve stops were one message

Every one of the twelve tried to give the **producer seat authority over the
consumer's world** — the right identity, value, environment, chain, and
finally frame — and each had to fabricate something the seat does not hold.
Amendment 1 recorded the verdict against itself: the producer's function
"reaches a call target that is correctly undeclared there."

Binding preconditions one at a time cannot terminate when the seat is the
wrong origin. That is why this is a relocation and not a thirteenth
precondition.

## The closed census — one writer, two readers

Measured at `b601e2ec7`, not estimated:

- **Writer:** `lowering/core.rs:12081`, inside
  `assemble_continuation_call_operands`. The only one.
- **Readers:** `lowering/calls.rs:1165` (the consuming call) and
  `lowering/core.rs:14827` (an admission gate).
- Initialized `None` at seven sites; `function_local` is wholesale replaced
  per function definition.

`lowering/mod.rs:1308` declares `constructed_context_frame` as a **singular
`Option`**, and its own comment states the operands "are `ir::Value`s of this
Function" and **"cannot be reused elsewhere."** The invariant is therefore
structural, not a missing lookup: **a seat may call the continuation it itself
constructed, and no other.**

## What the controls measured

**Control 1 — the demand is WORKER-ONLY.** At `funcid60`, for owning context 0
at exact `(continuation 661, position 1, body 1605)`:

```text
context_requires_frame        = false
caller_requires_frame_workers = true
worker_capture_count          = 7
claims.len()                  = 6
all six availability.context_capture present:
    EntryFrame { frame: Predeclared(15), declared_slot: 0..5 }
```

So the refusal has no third cause and **no missing producer-local context
capture.** The only absent operands are the selected worker's seven-capture
suffix. The return edge is narrower than the broad context-capture framing.

**Control 2 — the creation site EXISTS and the producer return reaches it.**
Of 19 writer applications in a complete fixture compile, **exactly one** writes
`(661, 1, 1605)`: the checked-IH environment-transport route at
`core.rs:9114`, while defining `funcid61`, `Predeclared(15)`, under emission
owner `Specialization(4)`. It stores **7 worker captures and 6 context
captures** — the exact cardinalities the refused call demands.

Reachability was **proved by emission, not inferred from the write**:
immediately after writing that frame, `funcid61` emits the producer call
targeting `ContinuationSpecialization(2)` (producer construct 1617, result
1624, worker body 1605) and **receives its returned SSA result.**

This control could have killed the ruling. I stated before it ran that if no
seat wrote `(661, 1, 1605)`, both arms were unrepresentable and that was a
stop, not a widening. It did not fire.

## The derivation that localizes the consumer, with its premises visible

From the measured numbers plus the gate's own code:

`core.rs:14840`'s planner-recovery route refuses whenever
`claims.len() != captures`. Here that is **6 != 7**, so that route **can never
admit this body.** Admission for worker body 1605 is therefore possible only
through the constructed-frame route at `core.rs:14827`, which requires a frame
matching the body origin and **both** cardinalities — 7 worker, 6 context.

Control 2 says exactly one function holds such a frame. **So the design's own
admission gate already admits this body in exactly one function, and it is the
creation site.** The consumer was being replayed somewhere the gate would have
refused it.

## The boundary the repair closes

At the lawful creation-site return, `detached_consumer = false`. The detached
consumer replay is present instead in `funcid60`, whose frame is
`(661, 1, 859)` — the wrong worker body.

⇒ **The repair relocates the detached post-call consumer to the creation-site
function**, which already holds both the exact frame and the producer's
returned SSA result. **Nothing new is carried.** The operands, the frame and
the result are all already live at that seat; only the consumer's emission site
moves to meet them.

## Two equivocations that would let this ruling cross its own fences

Both are one term naming two artifacts of different size. Neither is
hypothetical — the first is written in the landed code.

**1. "Closure conversion."** `core.rs:12038` calls the landed `D2` frame
mechanism *"the closure conversion the Architect ruled."*

| naming | what it is | fence |
|---|---|---|
| **Landed `D2`** | compile-time `ir::Value` operands in a function-local frame, consumed in the same function | inside |
| **The fenced arm** | a first-class continuation value whose state travels to another function | outside |

**Do not cite the landed `D2` comment as authority for a transported
continuation.** That would be a fence crossing dressed as continuity with a
prior ruling.

**2. "Relocation."** Moving the consumer's **emission SITE** is not relocating
**emission OWNERSHIP**. Ownership is a property of the generated context, not
of its caller. This amendment moves the former and leaves the latter exactly
where it is.

## Emission ownership — re-derived, not inherited

Amendment 1's reachability premise was measured **false** at HS12, so its
disposition is not carried forward by inertia. Deriving fresh from this arm:
context 0 is untouched — same `emission_owner = Specialization(2)`, same
Result contract `DenseRange { start: 4442, len: 38 }`. Only which function
holds the call instruction changes. **Ownership is NOT relocated.** Same
answer as before, now on grounds this arm actually supplies.

## Controls

Each can fail, and each names the branch that refutes this amendment.

1. **The witness advances BY application.** The Mapping case passes
   `Context3`'s query because the consuming occurrence **ran** — observe the
   four-arm match's tag query **executing** on the `ResourceBracketOk` word
   `0x0f09`. `Context3` accepting its input is not evidence; route-shape rows
   are not execution evidence.
2. **The relocated call is SATISFIED, not merely attempted.** At the
   creation-site function, the call to context 0 must pass the frame filter on
   **all three coordinates and both cardinalities** (7 worker, 6 context). If
   it still refuses, the mechanism in this amendment is wrong — stop and
   report, do not widen the filter.
3. **The wrong-site replay is GONE, not duplicated.** After the repair the
   producer-side function emits **no** detached post-call consumer replay. If
   both sites replay, the consumer is applied twice and that is strictly worse
   than the skip — a stop.
4. **Non-regression on ordinary routes.** Legitimate routes that need no frame
   still construct and lower as today. This gate stands above most ordinary
   code; over-tightening is this design's plausible failure and control 4 is
   the only thing watching it.
5. **Retained green.** Q1 `lowering/source.rs` stays blob
   `38ec787dac261f02d46463ee1e9fca555c76d694`; the px8f pre-object refusal
   stays honest; the entry-18 verifier repair stands.

## Not authorized

No new carrier, phase tag, or second planner relation. No runtime object. No
ABI, schema, frame, owner key, tag, route-to-runtime, stack or bound change.
No weakening of the pre-object refusal. No revert of the entry-18 verifier
repair. **No relocation of emission ownership.** No identity edit — the
finished identity chain is not reopened. R1 and R2 remain should-fix on the
candidate. `b601e2ec7` is not a candidate and nothing here promotes it.

## The fence, restated because this ruling sits against it

HS6's fence — no new carrier, phase tag or second planner relation — **stays
closed, and this arm does not touch it.** Had the ruling landed on
closure-conversion it would have required reopening HS6 explicitly rather than
crossing it quietly. It did not, so HS6 is not reopened.
