# ABI-S6 HS18 — closure mechanism, amendment 1

> **SUPERSEDED AS A STATEMENT OF THE RULE by
> [amendment 3](ABI-S6-HS18-closure-mechanism-amendment-3.md).** The rule here
> requires a route destination DERIVED FROM the required-consumer projection.
> That is insufficient: a chain-derived destination proves which consumer
> belongs to a route class, never that a given word is the pre-consumption
> result that consumer applies to. Amendment 3 pairs the destination with the
> exact emitted call whose result is the before-value. This document is retained
> as the record of how that was reached, not as current authority.
> **Amendment 3 is itself EXTENDED by
> [amendment 4](ABI-S6-HS18-closure-mechanism-amendment-4.md)**, which closes the
> realization half. Current authority is amendment 3 and amendment 4 together.

Architect, 2026-09-13. Amends the
[closure mechanism](ABI-S6-HS18-closure-mechanism.md) on the first hard stop
from the runtime ring (`evt_3t44j4xy0bqh6`, probe
`2fc0b03027a503d4ea267a83f35faa505c7fcd0b`, explicitly not a candidate).

The hard stop is correct on both counts. One of them is a defect in my rule.

> **SUPERSEDED IN PART** by
> [amendment 2](ABI-S6-HS18-closure-mechanism-amendment-2.md).
> Correction 1's reverse lookup is right but was scoped to the Tail branch only,
> leaving `DirectInvocationReturn` an unguarded escape. Amendment 2 moves the
> gate above every fork. Correction 2 (the seat) and correction 3 (`source.rs`)
> stand unchanged.

## Step 0 confirmed the gap, and then falsified my rule

Step 0 confirmed what I asked it to confirm: nothing in
`checked_ih_fresh_result_route`'s call graph reaches
`required_consumer_projections`, `required_consumer_projection_for`, or
`derive_required_consumer_occurrence`. The name-level census I flagged as a
caveat holds up at mechanism level. Good.

It then found the thing that breaks my rule: **for the Mapping witness, the
Tail producer's own keyed projection is absent.** The interposed authority
lives in a `RequiredConsumerProjection::DetachedReturnContext`
(`continuations.rs:1323`) whose worker-return boundary *selects* the Tail
producer.

My rule said `TailProducerToRet` is constructible only when *"the
required-consumer projection for the producer's continuation"* is absent. On
this witness it **is** absent. So the rule as written permits exactly the edge
it was designed to forbid, and would have been inert on the witness that
motivated the whole recut.

**Control 2 caught this before any code was built on it.** I specified a
positive control that the derivation must return *present* for the entry-19
Mapping source, and said an absent result there "is a stop." Step 0 effectively
ran that control and it reported absent. That is the control doing its job, one
step earlier than expected, and it is the reason this costs an amendment rather
than an implementation.

## Correction 1 — the projection query is a REVERSE lookup

The structure is asymmetric and I keyed on the wrong side.

`DetachedReturnContext`'s validator (`continuations.rs:7156`) requires
`target.key.consuming_occurrence.is_some()` to be **false** — a detached
projection exists precisely *because* the target has no source-level consuming
occurrence of its own — and requires
`boundary.selecting_call.target() == enclosing`, so the boundary names the
generated context that owns it.

So a producer with an absent own-key projection is not evidence of an absent
consumer. It is the **expected shape** when the consumer is detached.

**The corrected rule.** `TailProducerToRet` is non-constructible when **either**:

- the producer's continuation has its own required-consumer projection
  (`DirectOuter`), **or**
- **any** `RequiredConsumerProjection::DetachedReturnContext` has a
  `worker_return` boundary whose `selecting_call.target()` is this producer's
  enclosing specialization.

The second clause is the one that fires on Mapping, and it is a reverse lookup
over the projection set, not a field read on the producer. The route
constructor must take the projection set — or a query over it — as a required
input. A constructor that can only see the producer cannot answer this
question, which is why the rule has to move rather than be patched.

Everything else in the mechanism stands: derived-not-minted destination,
private-field newtype, `checked_ih_strict_ret_sink` unreachable as a
destination source, compile error rather than planner `Err`.

## Correction 2 — the application seat, named

My design said the projection must be *"carried by the route and applied by
lowering"*. That names an obligation without naming who discharges it, and the
ring was right to stop rather than guess. The probe did the work that lets me
name it.

**The consuming occurrence executes in the generated context that OWNS the
detached return-context projection** — the `enclosing` specialization the
validator reads from `identity.token.emission_owner`
(`ContinuationEmissionOwner::Specialization`). That context is reached by the
existing context-invocation path, the same mechanism that already carries the
value from the producer through the response owner today.

Two things that follow, both of which the probe's outcomes rule out
independently:

- **Do not relocate ownership.** Directly lowering the projected consumer in
  the producer's function reaches a call target that is correctly undeclared
  there. The probe established this. Relocation would move an emission owner to
  satisfy one witness and is not authorized.
- **The producer's context forwards the CONSUMER's result, not the producer's
  Ret payload.** This is the actual behavioral change. Today `Context1`
  projects field 0 of the producer's Ret and stores it onward; under the
  corrected route it must invoke the owning context and forward *that* result.

## The "calling it still skips" observation

The probe found that calling the derived context with the Tail carrier still
returns through the same skip.

**I read this as predicted by the rule defect, not as a separate obstacle** —
but that is a hypothesis and it is the ring's to test, not mine to assert. If
the consumer's owning context lowers its own fresh-result route as a
`TailProducerToRet`, it skips for the identical reason, and the corrected rule
makes that route non-constructible too. The closure then composes and both
contexts apply their consumers.

**The measurement that settles it:** with the corrected reverse lookup applied,
does the consumer's owning context still produce a constructible
`TailProducerToRet`? If yes, there is a third site and I want it reported
before it is worked around. If no, the composition holds and the skip is gone
at both ends.

If the answer is that the composition does not hold, **stop and return** — do
not reach for the point repair to close the gap. That is the shape this recut
exists to avoid.

## This is still not the withheld point repair

The distinction, so it can be checked rather than trusted:

- The withheld point repair wires `Context1`'s emitted match into the path that
  forwards around it, at that site. Nothing stops the next route from skipping.
- The closure makes the skipping route **unconstructible**, so the consumer
  invocation is the only thing the route can lower to, for every route.

**The tell, unchanged and now sharper:** if the repair is written at the
`Context1` lowering site rather than in the route's construction and lowering,
it is the point repair with extra machinery. I will review against that, and
against the new one below.

**New tell:** if the corrected reverse lookup is implemented as a check that
*returns an error* when it finds a selecting boundary, the design has not been
built. The lookup must feed the constructor's input, so the interposed case
produces the third variant rather than a rejection.

## Correction 3 — the `source.rs` blob criterion is withdrawn

I wrote a retained control requiring `source.rs` stay blob
`38ec787dac261f02d46463ee1e9fca555c76d694`. That criterion was mine, and it was
a proxy for the thing that actually matters: the Q1 resume-exit repair's
semantics. Adding the third variant forces the exhaustive production matches in
`lowering/source.rs` to gain an arm, which is a mechanical consequence of the
representation I required. Blob identity and my own design cannot both hold.

**Q1 semantics are authoritative. Blob identity is withdrawn as a criterion.**

Replacing it, and these are checkable:

1. **The three Q1 sites are byte-identical**, extracted by brace depth and
   hashed: the `SourceMachineExit::ResumeOuter` variant; the exit returned
   after the existing expected-cursor, root-authority restoration and Trap
   guards; and the thin wrapper dispatching `resume_active_continuation` after
   the inner returns. These are the whole of the Q1 repair.
2. **Every other `source.rs` delta is addition-only and confined to new match
   arms naming the new variant.** No existing arm's body changes. No arm is
   reordered. No `_ =>` wildcard is introduced — a wildcard would silently
   absorb the new variant and is the exact shape that makes a later variant
   addition go unnoticed.
3. **The Q1 crossing control stays green** — the unchanged-stack Q1 crossing at
   `5d977ac79` (1/1).

Item 2 is a review obligation, not a mechanical gate, and I am stating it as
one rather than dressing it as a test. I will read the `source.rs` diff myself.

## Unchanged

No ABI, schema, frame, owner key, tag, route-to-runtime, stack or bound change.
No weakening of the pre-object refusal. No revert of the entry-18 verifier
repair. No relocation of emission ownership. R1 and R2 remain should-fix on the
candidate. The probe `2fc0b030` is not a candidate and nothing here promotes it.
