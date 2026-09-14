# ABI-S6 HS18 — closure mechanism, amendment 2

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
[closure mechanism](ABI-S6-HS18-closure-mechanism.md) and
[amendment 1](ABI-S6-HS18-closure-mechanism-amendment.md) on the ring's second
hard stop (`evt_18wh9v6f8ykts`, probe
`618484650150526281003eac0791c9b6152eb729`, not a candidate).

The ring did exactly what it was told: ran the named measurement, found the
composition fails anyway, stopped, and reported the third site instead of
widening the rule or moving the consumer. That is the right behaviour and the
report is correct.

## What the measurement found

The corrected reverse lookup works on the producer. The rows:

- consumer-owning context 0, enclosing specialization 1, body 788 —
  `DirectInvocationReturn`, source target 0;
- producer context 1, enclosing specialization 3, body 813 —
  `ProducerThroughRequiredConsumerToRet`, detached projection identity target 2,
  owning context 0.

So the answer to my question — does the owning context still produce a
constructible `TailProducerToRet`? — is **no**. It produces
`DirectInvocationReturn`, and returns without executing its detached required
consumer. Mapping stays `(1, 0)`; no tag query runs on the `ResourceBracketOk`
word `0x0f09`; `Context3` still receives outer `Result::Ok` `0x1009`.

The mechanism sentence is the ring's and it is the whole answer: *"the owning
context's Direct route is chosen in `checked_ih_fresh_result_route` before the
required-consumer lookup, because that lookup currently occurs only in the
no-direct-transport Tail branch."*

## This is my error, and it is the same error twice

**`DirectInvocationReturn` is a producer-to-sink route too.** I wrote that
sentence myself in the mechanism doc — *"Today the enum offers
`DirectInvocationReturn` and `TailProducerToRet`, and both are
producer-to-Ret"* — and then wrote the rule about one of them.

The inventory had already told me. Entry 14: *"its only remaining eliminator was
`InvocationReturn`, which returned the bare call word without projecting the
existing checked-IH direct-result disposition into affine context-Result
authority."* That is the Direct variant forwarding a bare word, recorded five
entries before I scoped the closure to Tail.

Both of my hard stops have one predicate: **I stated the rule as a property of a
particular variant or branch, so a path around the gate existed.** Amendment 1
keyed on one projection shape when two exist; amendment 2 keyed on one route
variant when two exist. Fixing a gate's *contents* twice while leaving its
*position* downstream of a fork is the same defect I withheld a point repair
over, committed in my own design.

So this amendment does not widen the rule again. It moves the gate above every
fork and makes position irrelevant.

## The producer set is closed — census, not estimate

Because I have now been wrong twice about scope, this is measured rather than
assumed. At `5d977ac79`, across the whole crate, production construction of
`CheckedIhFreshResultRoute` occurs at exactly **two** sites, both inside
`checked_ih_fresh_result_route`:

- `aggregates.rs:6722` — `DirectInvocationReturn { source, destination:
  inheritance.fresh_result_destination.clone() }`, guarded by a condition
  including `direct_control_requires_direct_arm`;
- `aggregates.rs:6777` — `TailProducerToRet { .. }`.

Every other occurrence of `CheckedIhFreshResultRoute::` in
`aggregates.rs`, `lowering/core.rs` and `lowering/source.rs` is a match or
`if let` pattern, and the remaining constructions under
`aggregates.rs:6803-6891` are `#[cfg(feature = "px8-ds-test-support")]`
mutations.

Note what site 6722 does: it takes its destination from
`inheritance.fresh_result_destination` directly. The Direct route does not
derive its destination at all — it copies a pre-existing field. It is a purer
instance of "minted independently" than the Tail route was.

**Two producers, one function, no third path.** That is what makes the rule
below a closure rather than a third patch.

## The rule, stated once and position-independently

**`checked_ih_fresh_result_route` computes the required-consumer disposition as
its first act, before any fork, from the projection set. That disposition
dominates the Direct/Tail fork.**

- **Disposition PRESENT** — the producer's continuation has its own `DirectOuter`
  projection, **or** any `DetachedReturnContext` has a `worker_return` boundary
  whose `selecting_call.target()` is the producer's enclosing specialization.
  Then the only constructible variant is
  `ProducerThroughRequiredConsumerToRet`. **Neither `DirectInvocationReturn` nor
  `TailProducerToRet` may be built.**
- **Disposition ABSENT** — the existing Direct/Tail fork runs unchanged, including
  `direct_control_requires_direct_arm`.

**And the unrepresentability rides on the disposition, not on the branch.** The
destination newtype for **all three** variants is minted only by a function
taking the disposition as a required argument. `inheritance.fresh_result_destination`
must stop being assignable to a route destination, exactly as
`checked_ih_strict_ret_sink` must.

That is the part that makes position irrelevant: no variant is constructible
anywhere without the disposition having been computed first, so a future branch
added above, below or between the current two cannot reintroduce the skip. A
rule that says "call the lookup early" would be a convention, and a convention
is what failed here twice.

## Controls — one changes, one is added

Superseding the mechanism doc's control list on these two points:

- **Control 1 (unrepresentability) now takes both variants.** The `compile_fail`
  control must attempt `DirectInvocationReturn` with a present disposition as
  well as `TailProducerToRet`, each with its paired positive control and exact
  expected error. A control that only covers Tail would have passed the design
  that just failed.
- **NEW control 6 — the owning context applies its consumer.** For Mapping,
  assert the tag query executes on the `ResourceBracketOk` word `0x0f09` in
  context 0. This is the assertion whose absence let amendment 1 look plausible:
  the producer route changed variant and nothing checked that the *consumer*
  ever ran. Route-shape rows are not execution evidence.

Control 4, the legitimate absent-disposition non-regression, becomes more
important, not less — the disposition now gates the Direct route that most
ordinary code takes.

## The escalation position, stated before it is needed

This is **hard stop 2** on the closure mechanism. My standing trigger fires on
the **3rd**: if this ruling produces another hard stop, I hold my next ruling
and call a research prior-art advisory before ruling again, rather than issuing
amendment 3.

Recording the honest read now so it is not reconstructed later under pressure:
**both stops so far were scoping defects in my statements of the rule, not
evidence that the approach is wrong.** Neither would have been prevented by
prior art. If a third arrives, that assessment should be treated as refuted
rather than repeated, because three is where "my rule was too narrow" stops
being a better explanation than "the rule is the wrong shape."

## Unchanged

Amendment 1's seat ruling stands: the consuming occurrence executes in the
generated context owning the detached projection, reached by the existing
context invocation, ownership never relocated, and the producer's context
forwards the consumer's result rather than the producer's Ret payload.
Amendment 1's `source.rs` disposition stands: Q1 semantics authoritative, blob
criterion withdrawn, addition-only and no `_ =>` wildcard.

Both review tells stand, and the first now reads on either variant. No ABI,
schema, frame, owner key, tag, route-to-runtime, stack or bound change; no
weakening of the pre-object refusal; no revert of the entry-18 verifier repair;
no relocation of emission ownership. R1 and R2 remain should-fix on the
candidate. Probe `618484650150526281003eac0791c9b6152eb729` is not a candidate.
