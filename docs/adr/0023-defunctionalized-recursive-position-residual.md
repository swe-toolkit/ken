# ADR 0023 — Defunctionalized recursive-position residual

**Status:** Accepted for RT-NATIVE-CONTINUATION-ENV-CARRIAGE.
**Decider:** Architect, `evt_27p1zdv9c44qs`, `evt_69ktj8b1xe8tc`,
`evt_7tg07nnz0656z`, and `evt_5zrc0er5yby55`.
**Scope:** Native backend representation; no kernel or spec change.

## Context

A generated continuation call transports words. Two executions at the same
static recursive-position coordinate may construct values with different
captured operands. A compile-time side slot keyed by that coordinate cannot
recover which execution produced a returned word. Nor is the returned value a
lexical closure: the native worker body is selected statically, while its
construction-time environment must travel with the value.

The archived gate projects Child from a call-result parent through `fn13` at
index 1 (`v7461`, `v14221`, `v25769`). The three retarget inputs (`v9396`,
`v16156`, `v27704`) instead come from inline `fn18` HostResult allocation and
`fn21` Child projection. Neither a parent phi nor a HostResult allocation is
a capture suffix. The source constructor store that creates each recursive
field, not those downstream projections, is where its environment can be
attached. The parent and Child may have different emission owners; a routed
transport answer is not a destination constructor store.

## Decision

The static planner issues a slot for each exact `(eliminator, constructor
identity, recursive position)`. Its closed flow set determines one schema per
specialization: `[Child, (Label), WorkerCapture{seat, ordinal}*,
ContinuationInput{ordinal}*]`. The label is an interned specialization index
and is omitted for singleton flows. An unwrapped slot remains plain; an
issued wrapped slot is a private Record with tag 1. Its source constructor
stores that Record as its recursive child, with `Child` preserving the word
that the source expression built. The planner, not runtime class or field
count, selects the variant and each role. Runtime class, tag, arity and label
only assert the selected schema before any field projection. A bare K word
cannot substitute for a private R.

The planner issues store dispositions from source Construct occurrences,
before aggregate ownership is available. Lexical closure children use their
own boundary-closure environment records. A checked-IH invocation stored in
a slot-shaped field is another child minting population: the source expression
has an ordinary zero-argument call to a binder resolved as that slot's IH,
and the specialization worker has declared arity one. The owner unit's worker
closure seat selects its checked-IH captured-environment record. The force
record's children must match the slot variant's worker-capture roles in
ordinal, source origin and seat. The checked-IH child is the owner's existing
variant, not a relay label or a copy of an arriving dynamic label. In the
read fixture, S1/526 field 1 receives force occurrence 524 from seat 730;
S0/746 field 1 receives force occurrence 744 from seat 839. Both stores write
R with their corresponding bare K as Child. Their residual allocation record
is per creation site, distinct from the canonical lexical disposition record
that establishes the variant schema.

After ownership, every issued source store has exactly one Source Constructor
record and an edge naming its owner, origin, specialization and minting
record. An independent source walk checks the slot-shaped checked-IH store
population for missing edges. The lowering-time `slot_store_obligation`
rederives obligations from the source allocation identity and emitting owner
at all three source-identity constructor paths. A generic transfer with such
an obligation refuses rather than writing a bare child; positional capture
environments are planner-synthesized, not source Constructor stores. Checked-IH
source Call shape is provisional: before emission, the joined oriented plan
must contain the invocation's template with arity zero and its slot.

For a lexical construct, W operands come from the governed boundary-closure
environment and C from the finalized emission claims. For a checked-IH store,
W comes from the selected case's `StaticWorker` binding at the worker seat;
C comes from that same case environment's `ContinuationInput` bindings in
issued ordinal order. These are actual operands, not captures inferred from a
foreign owner's frame or from `NoClaim` availability. In particular, the
seven Spec2 Construct W claims at S3/515 are `Unfinalizable(NoClaim)` for
ordinals 0–6 under S2; its two C claims are `FinalizedFrame`. Those seven
claims do not license a worker reconstruction at the destination. A checked
transport's finalized W/C claims describe its own source provenance, but its
answer substitutes for the destination construction and gets no store edge or
new wrapper. This also preserves the distinction between S3's checked
transport force and S4's ordinary force path.

The record's lifetime is the ownership meet over its children, included in
the parent constructor's meet before allocation. The reader continues to
select a body from the exact checked eliminator and candidate set, not from a
word's runtime shape. No capture is reloaded from a side slot. Private Record
identity is refused by ordinary value conversion, canonical hashing and
plain-child readers; non-root `PersistentGround`, boundary carriers, root
exit and response descent retain their existing contracts.

## Consequences

The W2 and W7 constructions retain their own environments across word-only
calls. A missing source store edge or a wrong minting record refuses during
planning, an oriented-template disagreement refuses at installation, and a
bare child or wrong label refuses at the reader assertion. The two independent
store derivations do not prove general native parity; their runtime witness is
the targeted read and write parity tests and negative controls. The interned
key's `d099` golden is unchanged.

The former two-bracket ordinary-Match guard census observed 397 emitted
binders, none in its planned residual slot set S. It was not positive
coverage of an S binder, so the vacuous positive assertion was removed.
The zero-S ordinary-Match control remains. The read and write fixtures
exercise their exact recursive-position gates; this does not assert that an
ordinary Match of a wrapped slot has a reaching witness.

A one-bracket fixture is not necessarily unwrapped: the measured example has
five W and two missing context captures. The zero-disposition IR identity
control uses a separate fixture with W=0 and no context suffix or flow set.
