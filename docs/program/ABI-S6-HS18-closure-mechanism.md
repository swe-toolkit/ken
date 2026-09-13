# ABI-S6 HS18 — the consuming-occurrence closure mechanism

Architect, 2026-09-13. This is the mechanism design the
[recut frame](issues/ABI-S6.md) returns to me: the representation-level rule
stating what transport edge the planner may and may not construct. It
implements the closure required by the
[consuming-occurrence ruling](ABI-S6-HS18-consuming-occurrence.md).

Grounded against `5d977ac7968dff3763d330690a9b4df530925d79`, the tree the
runtime ring builds from, not against my own ruling's prose.

## The predicate, at mechanism level

`CheckedIhFreshResultRoute` (`aggregates.rs:390`) is the planner-owned directed
route from a governed K application to a Ret destination. Its
`TailProducerToRet` variant says what it does in its own doc comment: *"The
governed producer result moves directly and forward to the exact shared Ret
input."*

**That variant is the skip-edge.** It is a producer-to-sink edge, and nothing in
its construction consults the consuming occurrence the source interposes.

The sole production construction is `checked_ih_fresh_result_route`
(`aggregates.rs:6666`, building the variant at `aggregates.rs:6777`). It
obtains its destination from `checked_ih_strict_ret_sink`
(`aggregates.rs:5702`) — *"the exact strict Ret sink in its producer frame"* —
an independent structural lookup of a Ret sink in a frame. It then checks
source-to-case connectivity, sink uniqueness, and non-substitution of an
earlier transport result.

Every one of those checks is about the endpoints. **None is about what the
source places between them.** The interposed consumer survives into the route
only as `fresh_result_capture_ordinal` — where the continuation's bound result
lands in the worker's capture carrier. A location, not an obligation. The route
records where `outcome` sits and never records that the k-Match consuming it
must run.

This is "minted independently and then certified" in one function: the sink is
found by structural search, the route is built, and the verifier is then asked
to certify a relation the source does not license.

## The capability already exists and this site does not use it

The planner already carries the consuming occurrence as a first-class,
privacy-protected notion:

- `ContinuationSpecializationKey::consuming_occurrence`
  (`continuations.rs:1427`) — the source-level certificate, a key field of the
  specialization.
- `RequiredConsumerProjection` (`continuations.rs:1318`) — *"the occurrence
  required after the target body is realized: the same occurrence at depth one,
  and the unique outer consumer from depth two onward."*
- `derive_required_consumer_occurrence` (`continuations.rs:6004`) — its sole
  derivation.

`RequiredConsumerProjection`'s own doc already states the exact discipline this
closure needs: *"The fields are private and there is no constructor outside
planning. Lowering can only receive a value that the whole-plan validator has
matched against `derive_required_consumer_occurrence`."*

**So the closure is a reuse, not an invention.** The pattern is established in
this codebase, on the sibling type, for the same reason.

**Census, stated at name level because that is what it measures.** In
`aggregates.rs` at `5d977ac79`, the count of `RequiredConsumerProjection` and
`required_consum*` occurrences is **zero**. That is evidence about those names,
not proof that no equivalent consultation happens under a different spelling.
**The implementer must confirm, before building, that no other identifier in
`checked_ih_fresh_result_route`'s call graph already reaches the required-consumer
projection.** If one does, this design is wrong about the gap and I want that
back before the work starts, not after.

> **AMENDED 2026-09-13 — read
> [amendment 1](ABI-S6-HS18-closure-mechanism-amendment.md) with this document.**
> The rule below keys on the producer's OWN required-consumer projection. That
> is WRONG and inert on the Mapping witness, whose Tail producer has no own-key
> projection: the interposed authority is a `DetachedReturnContext` that
> SELECTS the producer, so the query is a reverse lookup. The amendment also
> names the application seat and withdraws the `source.rs` blob criterion.
> Everything else here stands.

## The rule

**A fresh-result route's destination must be derived from the required-consumer
projection for the producer's continuation. It may never be obtained by an
independent structural search for a Ret sink.**

With the consequence that fixes the variant:

**`TailProducerToRet` is constructible only when the required-consumer
projection is absent.** A present projection is an interposed consuming
occurrence, and an interposed consuming occurrence must be carried by the route
and applied by lowering.

## The representation

Three changes. The first is the closure; the other two are what make it
unrepresentable rather than checkable.

**1. A third variant, so the interposed case has a spelling.** Today the enum
offers `DirectInvocationReturn` and `TailProducerToRet`, and both are
producer-to-Ret. Neither can express *"apply this consumer, then Ret."* Add it:

```text
ProducerThroughRequiredConsumerToRet {
    source: CheckedIhFreshResultSource,
    required_consumer: RequiredConsumerProjection,   // carried, not summarized
    destination: <destination derived from that projection>,
    ...
}
```

The route now names the occurrence, so lowering has an obligation it can
discharge and the verifier has a relation it can certify. The missing
representation is the reason a bare forward looked like the only available
edge.

**2. The destination becomes unforgeable.** Replace the destination-bearing
fields with a single newtype whose fields are **private to the module owning
the derivation**, minted only by a function that takes the required-consumer
projection as a required argument. Rust enum variants cannot have private
fields, so the variant must carry the newtype rather than loose fields — the
same shape `RequiredConsumerProjection` already uses.

**3. `checked_ih_strict_ret_sink` stops being reachable as a destination
source.** It may remain as a structural query if other callers need it, but it
must not be able to produce a value that a route destination accepts. This is
the half that turns the rule from a convention into a closure: as long as a
function returns something a route will take as a destination, the skip-edge
keeps its spelling.

## Why this is unrepresentable and not merely refused

The frame's acceptance criterion is explicit that a test showing the verifier
refuses the edge does **not** satisfy it. Under this design:

- There is no expression that constructs `TailProducerToRet` with an interposed
  consumer, because its destination newtype cannot be minted without a
  projection and the variant is gated on that projection being absent.
- The failure is a **compile error in the planner**, not a planner error at
  runtime and not a verifier refusal. The edge has no spelling.

A planner `Err(...)` return would not satisfy the criterion. That is a refusal
wearing the closure's clothes: the edge still exists, is still constructed, and
is still rejected downstream of its construction. **If the implementation finds
itself adding a check inside `checked_ih_fresh_result_route`, the design has
not been built.**

## Controls

Each acceptance criterion gets a control that can actually fail.

1. **Unrepresentability — a `compile_fail` control** attempting to construct
   `TailProducerToRet` with an interposed consumer. **It needs a paired
   positive control** compiling the same construction with an absent
   projection, and it must assert the **exact expected error**, not merely that
   compilation failed. A `compile_fail` passes when the target fails for any
   reason; without the pairing and the error assertion it is not evidence.
2. **The derivation is live** — a positive control that
   `derive_required_consumer_occurrence` returns a **present** projection for
   the entry-19 Mapping source at `funcid53`/`Context1`/`origin560`. If it
   returns absent there, the closure is correct and inert on the witness that
   motivated it, which is a stop.
3. **The witness advances by application** — the Mapping case passes
   `Context3`'s query because the consuming occurrence ran, evidenced by the
   four-arm match's tag query executing. The frame requires this be "BY the
   consuming occurrence being applied, never by wiring around it," so the
   control must observe the tag query executing, not merely that `Context3`
   accepted its input.
4. **Non-regression on the legitimate tail case** — a positive control that a
   genuine absent-projection route still constructs and still lowers as today.
   Entry 15 established Direct and Tail are genuinely different and both
   needed; this closure must not collapse them. Without this control the
   design's plausible failure — over-tightening so real tail routes stop being
   constructible — has nothing watching it.
5. **Retained green** — every grid and mutation control at `01d2ccb11` and
   `5d977ac79`, Q1 `source.rs` blob `38ec787dac261f02d46463ee1e9fca555c76d694`
   unchanged, px8f pre-object refusal still honest.

Controls 1 and 4 are the pair that matters: 1 says the bad edge is gone, 4 says
the good edge survived. Either alone is satisfiable by a wrong design.

## Not authorized

No ABI, schema, frame, owner key, tag, route-to-runtime, stack or bound change.
No weakening of the pre-object refusal. No revert of the entry-18 verifier
repair — it stays, and it is what will certify the new route's application. No
change to identity domains. The two residuals R1 and R2 from the source-routing
determination remain should-fix on the candidate and are not folded into this
mechanism.

## The px8f prediction

The ruling predicts px8f is the same defect. Under this mechanism the
prediction becomes concrete and cheap to settle: if px8f's six `u3:60`
obligations are fed by routes whose required-consumer projection is **present**,
the closure retires px8f too. Record the answer as a finding; the frame makes it
a recorded finding, not a gate.

If those routes' projections are **absent**, the predicate is narrower than I
stated and px8f is a separate defect. I want that result explicitly either way —
a confident negative here is worth more than an assumed positive.
