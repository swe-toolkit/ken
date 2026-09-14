# ABI-S6 HS18 — closure mechanism, amendment 3

> **EXTENDED — NOT SUPERSEDED — by
> [amendment 4](ABI-S6-HS18-closure-mechanism-amendment-4.md).** The rule below
> stands exactly as written and remains current authority. It closes what the
> planner may CONSTRUCT; it says nothing about what lowering must REALIZE, and
> HS6 is the same invariant escaping through that half. Amendment 4 requires the
> final identity to be minted by the function that lowers the consuming
> occurrence, from the after-definition it produced. Read the two together.

Architect, 2026-09-13. Amends the
[closure mechanism](ABI-S6-HS18-closure-mechanism.md),
[amendment 1](ABI-S6-HS18-closure-mechanism-amendment.md) and
[amendment 2](ABI-S6-HS18-closure-mechanism-amendment-2.md) on the ring's third
hard stop (`evt_41kgw2qkw0xey`, non-candidate `37591f001`).

**This amendment supersedes the RULE stated in all three. They are retained as
the record of how it was reached, not as current authority.**

Issued after the §1a hold ran its course: the ruling was held
(`evt_76xhx9x5fpafq`), a scoped research prior-art advisory was called and
returned (`evt_1d8fk5nx7pbd9`, `evt_19kh2r2fh4t7q`), the parallel observer-only
A/B/C pass returned (`evt_7s1m3kc5fe0ws`), and the rule is stated with both in
hand. That is what the trigger exists for.

## What the observation found

Arm B under my stated predicate, unambiguously: `0x0f09` has no definition
reaching the selected `v81` instruction.

- The disposition-selected seat is Context0 / body 788, CLIF `block5`,
  `v81 = call fn37` (`ken_static_response_1`), reached **once**, over the single
  executed edge `block24 -> block5`. Its result definition is
  `block35: v86 = stack_load ss7+144` and it carries **`0x0205`**.
- The producer on the executed path is Context0's **later, distinct** call
  `v284 = call fn38` at `+0x1a37`, result `v289 = 0x1109`.
- `0x0f09` is defined only in Context4 / body 734, embedded as field 0 of
  `0x1009`, then of `0x1109`, and forwarded up. Context1 receives
  `v82 = 0x1109`, reads field 0 once into `v107 = 0x1009`, and calls
  `static_response_2`. The four-arm consumer block, Context1 `block89`
  expecting `0x08ea_0000_0043`, is never reached.

**The observation also refuted the hard stop's own characterization.** The word
at the seat is `0x0205`, not `0x0305`, and `0x0305` is not consumed there — it
is a field of that seat's Ret result, consumed later in Context4 / body 734.
"Already-consumed / double-consumes" is withdrawn by the measurement run to test
it. That is the second time in this arc a named observation has overturned the
reporting seat's own classification, and it is why the bounded observer-only
pass with no repair proposal attached keeps earning its cost.

## My arm partition was drawn on symptoms

I wrote B (wrong seat) and C (wrong datum at a correct seat) as independent
alternatives. They are one defect seen from two ends: **the seat is wrong
because the coordinate that named it under-determines which definition it
means.** Research's characterization of C — the missing relation is between the
projection and the exact producer definition — is the cause; B is its symptom.

Recorded because the partition is mine and it let one defect present as a fork.
Draw the next one on causes.

## The defect, in the code's own terms

`checked_ih_required_consumer_destination` (`aggregates.rs:8596`) selects by

```text
coordinate.context                 == access.context
  && coordinate.enclosing_specialization == access.enclosing_specialization
  && coordinate.worker_body_origin       == access.worker_body_origin
  && class.members.contains(transport.source_call_identity())
```

The transport's source call identity tests **class membership** and never
selects a definition. `CheckedIhGeneratedEntryCoordinate`'s own doc comment
(`aggregates.rs:751`) has said so, in the tree, since before amendment 1:

> "This is the quotient key. Source-call identity is deliberately absent: it is
> a class member, not a discriminator available at generated entry."

Body 788 holds two transport calls, `v81` at `+0xc2f` and `v284` at `+0x1a37`.
Both are class members. The first to ask gets the destination.

`derive_required_consumer_destination` (`aggregates.rs:430`) compounds it: the
`context` field is computed from `(emission_owner specialization,
producer_result_origin)` — a static origin coordinate with no value, call or
edge in it.

**The destination was always a class coordinate, and a class coordinate cannot
name a definition.** That is why three amendments about which routes are gated
could not have worked. Gating was never the defect; the carrier was, and the
carrier's own doc comment said what it could not discriminate.

## The rule

**A required-consumer destination must be minted from, and permanently paired
with, the exact emitted call whose result is the before-value. It may never be
resolved by a class coordinate and then applied to whichever transport arrives
at that coordinate.**

The invariant it serves:

```text
(exact before-value, exact consumer occurrence, exact incoming edge)
    -> (exact after-value, exact outgoing edge)
```

The sink accepts the after-value. A before-value may not be substituted for
having the same runtime representation or reaching the same code seat.

**Representation.** `RequiredConsumerDestination` gains a private field naming
the defining call identity it was minted for, set only by the minting function
from the exact transport that produced the before-value. The lowering apply site
takes that call identity as a required argument, so applying a destination at a
different call has no spelling. A planner or runtime mismatch check would be a
refusal wearing the closure's clothes, exactly as at every prior step.

**A class holding more than one transport call in the same body is not a choice
to make.** It is the planner discovering the class is too coarse for this
purpose. Per `CheckedIhGeneratedEntryProjection` (`aggregates.rs:767`) — "it
must never create another class" — the answer is not to split the quotient. The
discriminator rides alongside the class; the quotient stays intact. This is also
what closes the split-the-seat arm, on the tree's own documented invariant
rather than on my judgment or on my RT-NATIVE-FNSPLIT closure.

## No phase mechanism, and why the fence holds

The advisory is explicit that under arm B or C a dynamic phase mechanism solves
the wrong problem. Honoured: **no runtime carrier, no phase token, no new
payload tag, no ABI change.** My own hard-stop-3 hypothesis said the obligation
was "a per-activation fact" — that word overshot into dynamism and is withdrawn.
The axis is static code identity versus static value-and-edge identity, both
static, which is what keeps this repair compiler-internal and ABI-inert.

If a future measurement shows the phase is genuinely not statically determined
on each CFG edge, that is a fence question for the Steward and the operator, not
a design issued here.

## Amendment 1's application-seat ruling is SUSPENDED, not re-ruled

Amendment 1 ruled the seat as a standalone fact: the consuming occurrence
executes in the generated context owning the detached projection. The
observation puts the emitted four-arm block in Context1 `block89`, while the
destination's context is computed from the projection's emission owner.

**I am not re-ruling the seat.** Declaring it as an independent fact is the same
error shape a third time — stating as a standalone rule something that is in
fact determined by a relation. **The seat is a consequence, not a declaration:**
it is wherever the before-value's definition and the consumer's incoming edge
meet, and it is derived. If the derivation lands somewhere other than where
amendment 1 said, the derivation is right and amendment 1 was wrong.

## Controls

1. **Unrepresentability** — a `compile_fail` control attempting to apply a
   destination at a call other than the one it was minted from, with its paired
   positive control and the exact expected error. A `compile_fail` that only
   asserts failure is not evidence.
2. **The under-determination is real** — assert that for body 788 the class
   holds more than one transport call, and that the new keying selects `v284`
   rather than `v81`. Without this the repair can be inert and still look green.
3. **`block89` is reached and the tag query executes on `0x0f09`** — amendment
   2's control 6, now with the block named by the observation. Route-shape rows
   are not execution evidence.
4. **Non-regression on the absent disposition** — unchanged, and more important
   now that the disposition gates the Direct route most ordinary code takes.
5. **Retained green** — every grid and mutation control at `01d2ccb11` and
   `5d977ac79`; Q1 site preservation with addition-only, no-`_ =>`-wildcard
   match discipline; px8f pre-object refusal still honest.

## Review tells — three, and the first is the live risk

1. The value is fully present in Context1 as `v107 = 0x1009` and the consumer
   block sits there unreached. **Making Context1 branch into `block89` would fix
   Mapping and would be the withheld point repair.** What this amendment
   authorizes is in destination minting and resolution — `aggregates.rs:430` and
   `:8596` — not at a Context1 lowering site.
2. A reverse lookup implemented as a check that returns an error is the design
   not being built.
3. Authority paired with a destination or a route class rather than with an
   exact value definition and edge is the withdrawn rule under a new name.

## Unchanged

Runtime held at `5d977ac79`; `37591f001` is not a candidate. No ABI, schema,
frame, owner key, tag, route-to-runtime, stack or bound change. No weakening of
the pre-object refusal. No revert of the entry-18 verifier repair — it stands
and is what will certify the new relation. No relocation of emission ownership.
R1 and R2 remain should-fix on the candidate.
