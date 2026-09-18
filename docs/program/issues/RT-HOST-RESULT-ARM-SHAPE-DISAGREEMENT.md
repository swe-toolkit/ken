---
id: RT-HOST-RESULT-ARM-SHAPE-DISAGREEMENT
title: "The C2 nested-payload row's ERROR arm is recipe-derived from FsWriteAt and asserts a BoundaryTag::ImmediateBool result the recipe yields under neither root -- HostResultOk is Fixed{Wrote,[PrivateTransferCount,[nat,nat]]} and HostResultError is a twelve-alternative Dynamic set. (The success arm is hand-built with a bool field and is fine.) This is not a fixture supplying a wrong value; it is the row's assertion and the operation's recipe disagreeing, so no fixture edit reconciles it. The question is whether the row's property is expressible against FsWriteAt at all, or whether it needs an operation whose error arm is genuinely an immediate bool."
status: ready
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18. Architect ruling in evt_760sbdhcgwx80, which took the arm question OUT of RT-CARRIER-PRODUCER-OCCURRENCE's D3 and out of fixture repair entirely: 'The row asserts a shape this operation's recipe never yields, under either root. No fixture edit reconciles that... It is a new node, and the design question there is mine -- I will take it when it is filed.' Surfaced by runtime-implementer during D3 (the OK-rooted template sitting in the error slot), whose observation the Architect confirmed and then widened by measuring the recipe tree. Second node produced by the RT-CARRIER chain. Steward-filed per COORDINATION section 2."
---

> # THIS IS NOT A FIXTURE REPAIR, AND THE CHEAPEST WRONG MOVE IS A ONE-LINE EDIT
>
> The row fails. Every instinct says *the rig supplies the wrong value, fix the
> rig.* **It does not and you cannot.** The assertion names a boundary shape
> that `FsWriteAt`'s recipe does not produce on either arm, so there is no value
> the fixture could be corrected to supply. **A repair that makes this row green
> has either changed the assertion or changed the operation, and both of those
> are design decisions, not fixture work.**
>
> The design question is the **Architect's** and they have now ruled
> (`evt_2w5kz75w4bp94`): **the leading outcome is that the error arm should
> never have been recipe-derived at all.** That ruling is a LEAN WITH A
> DISCRIMINATOR, not a finding — it names what to test first and what must be
> checked before building. The measurement is done; what is owed is the
> discriminator, not a census.

# What the row asserts

`c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload`,
`constructors.rs:3334`, body ends `:3610`. Currently `#[ignore]`d at `:2548`.

**Both arms assert a `BoundaryTag::ImmediateBool` result** — but only one of
them is recipe-derived, and that asymmetry is the whole reason the ruling lands
where it does:

    :3572-3574   success  (1u64 << BOUNDARY_TAG_BITS) | BoundaryTag::ImmediateBool
    :3585-3587   error    BoundaryTag::ImmediateBool            (payload 0)

**The two arms are built by different means.** Within the row's producer edge:

    :3449   ok     Lowered::DynamicConstructor, HAND-BUILT in the fixture, whose
                   alternative carries a `ConstructorField::specialized(
                   Lowered::Bool { .. })` -- an actual bool field (:3520-3523)
    :3467   error  compiler.synthesized_constructor(..), RECIPE-DERIVED, rooted
                   at SynthesizedAggregateRoot::HostResultOk

The consumer is a `lower_carried_match` that projects the matched
constructor's field (`:3541-3556`), so **a hand-built aggregate carrying a
`Lowered::Bool` projects to an `ImmediateBool` word legitimately.**

> **THE SUCCESS ASSERTION IS FINE AND I NEARLY FILED THIS NODE SAYING IT WAS
> NOT.** My first draft read both `ImmediateBool` assertions off the row and
> recorded the mismatch as *wider* than the Architect ruled. It is not. The
> success arm never consults `FsWriteAt`'s recipe — it is hand-built with a bool
> field and projects to exactly what it asserts. **Only the error arm is
> recipe-derived, and it is the only one that cannot be satisfied.** The
> Architect's ruling was correctly scoped and my widening was a misreading of
> which arm the recipe feeds.
>
> ⇒ **The assertions look identical and their provenance is not.** Reading the
> assertion tells you what is expected; only reading the producer tells you
> whether anything can supply it.

# What the recipe yields — verified by the Steward, not inherited

`planning/static_transition/aggregates.rs`, blob-confirmed identical to
`main` `35cf12bac1db67d18de48cd8680fe2140a2774e0` at measurement time:

    :3588   let (error, ok) = match operation {          the destructuring
    :3618   Op::FsWriteAt => (RESOURCE_SURFACE, WROTE),  so error=RESOURCE_SURFACE,
                                                         ok=WROTE
    :3534   WROTE           = N::Fixed { role: R::Wrote, children: [TRANSFER_COUNT] }
    :3358   TRANSFER_COUNT  = N::Fixed { role: R::PrivateTransferCount,
                                         children: NAT2 }
    :3354   NAT2            = [N::bounded_nat(), N::bounded_nat()]
    :3378   RESOURCE_SURFACE = N::Dynamic(Alternatives(&[ ... ]))

⇒ For `FsWriteAt`:

    HostResultOk      Fixed{Wrote,[Fixed{PrivateTransferCount,[nat,nat]}]}
    HostResultError   a Dynamic alternatives set, TWELVE alternatives

**Neither is `BoundaryTag::ImmediateBool`**, so the error arm's assertion is
unsatisfiable under either root. The template at `:3467` is rooted at
`HostResultOk`, which is the implementer's original observation; the ruling's
force is that **correcting that root does not rescue the assertion**, it only
swaps a two-nat aggregate for a twelve-alternative surface.

> **Coordinate hygiene, per the predicate this chain just adopted.** The
> Architect's post cited the destructuring at `:3624` and `TRANSFER_COUNT` at
> `:3357`; the tree has them at `:3588` and `:3358`, and `:3623` is the
> `SynthesizedHostResultTree { error, ok }` construction. **The content is
> exactly as they described and the drift changes nothing** — recorded only
> because this node's own inventory entry says to cite by content and treat
> coordinates as hints. The twelve-count and `NAT2` were re-derived here, not
> quoted.

# The question this node owns

    Is the row's property -- "a runtime host result selects a SEPARATELY
    GENERATED nested payload" -- expressible on the ERROR arm against
    `FsWriteAt` at all?

## LEADING OUTCOME `(c)` — the error arm should not be recipe-derived at all

**Architect ruling, `evt_2w5kz75w4bp94`: this is the leading outcome, not one
of three equals.** The deciding fact is one both the Steward and the
implementer noticed and neither pressed on:

    the `error` slot is filled by
      synthesized_constructor(effect_seat,
                              root(SynthesizedAggregateRoot::HostResultOk),
                              SynthesizedFixedConstructorRole::Wrote, ...)

> **The author reached into the OK recipe — the SUCCESS node — to fill the
> ERROR slot, and then asserted the result is an `ImmediateBool`.** Nobody
> modelling a real `FsWriteAt` error path does that.

⇒ **THE RECIPE WAS NEVER LOAD-BEARING FOR THIS ROW'S PROPERTY. It was a source
of a plausible well-formed value.** And that reframes the entire layer
sequence this chain spent itself on:

**BORROWING A RECIPE FOR A VALUE YOU DO NOT NEED RECIPE-DERIVED DRAGS IN EVERY
CONSTRAINT THE RECIPE CARRIES** — the seat must exist, some unit must hold the
record, the child shapes must match. **Four layers, and all four were the price
of the borrow rather than defects in the row's subject.**

It also explains the provenance asymmetry recorded above. **That is not two
design choices — it is one arm doing the right thing and one arm taking a
shortcut that looked more rigorous.**

### The discriminator — this is the whole ruling

    Does this row's stated property depend on the error arm being a
    COMPILER-SYNTHESIZED aggregate, or only on its being a WELL-FORMED one?

**If only well-formed:** the repair is `ac_c7_lowered_ctor`'s shape — hand-built
with a plan-resolved occurrence, exactly what `D3` adopted for the other three
— and **the `ImmediateBool` assertion most likely survives untouched.** That is
the cheapest outcome and the one consistent with what `D3` already established,
so **test it first.**

> **STATED AS A LEAN WITH ITS DISCRIMINATOR, NOT AS A FINDING.** The Architect
> has read the row's structure and the recipe; they have **not** established
> that nothing downstream requires a synthesized identity in that slot.
> **Whoever takes this node checks that before building.** The lean tells you
> where to look first; it does not licence skipping the check.

## The alternatives, if the discriminator comes back the other way

    (a) the ERROR ASSERTION is wrong for this operation
        => rewrite it against what FsWriteAt's error arm actually yields. The
           property may survive intact; only the boundary shape it checks
           changes. The success arm is untouched either way.
    (b) the ROW is on the wrong operation
        => re-point it at an operation whose ERROR arm IS an immediate bool.
           The assertion survives; the fixture's operation changes. Requires
           naming such an operation and showing it still exercises SELECTION.

**Do not pick (a) because it is the smallest diff.** Under `(c)` it is not even
addressing the right thing — it would rewrite an assertion to match a value the
row never needed to borrow.

# BANNED — switching the root and re-running

**Do not repair this by changing the template's root to `HostResultError` and
re-running.** The Architect's words: *"That is a rung on the withdrawn ladder —
it buys a layer, the assertion still fails, and it would be layer 5 under
another name."*

⇒ The move looks smaller than the one already declined and is not. Under either
root the shape is not an immediate bool, **so the run's outcome is known before
you make it**: it fails, one layer deeper, having told you nothing. An edit
whose result you can predict is not a measurement.

> **THE BAN WIDENS UNDER `(c)`, and this is the stronger reason.** The original
> ban said switching the root buys a layer and still fails. Under the leading
> outcome it is worse than useless: **the defect is that the row borrowed from a
> recipe at all, so switching to the "correct" root is a DEEPER BORROW — it
> takes on the twelve-alternative surface's constraints on top of the ones
> already being paid.** It is not a smaller version of the right move; it is a
> larger version of the wrong one.

# What must not happen

- **Do not un-ignore this row as part of `RT-CARRIER-PRODUCER-OCCURRENCE`'s
  `D4`.** `D4` removes the `#[ignore]` on the premise that `D3` made the row
  able to run. It does not — `D3` fixes the carrier refusal, which is upstream
  of, and unrelated to, the arm shape. **Un-ignoring it would convert a parked
  row into a red one and bury this question under a CI failure.** This is the
  concrete reason the Architect routed the arm out of that node.
- **Do not treat the row's failure as evidence about `D3`.** `D3`'s repair is
  real and its positive result stands on its own: the three plan resolutions
  succeed. This row would still fail afterwards, for this separate reason.
- **Do not size a repair before the ruling.** (a), (b) and (c) are not the same
  size and two of them are not fixture work at all.

# Related

- [[RT-CARRIER-PRODUCER-OCCURRENCE]] — the parent, and the reason this row was
  being worked at all. Its `D3` repairs the carrier refusal; **this node is
  strictly downstream of that and does not block it.** The `D4`/`AC-3`
  interaction above is the one place they touch.

  > **Deliberately NOT a `depends_on`.** The parent is where this question came
  > from, not something this node needs. The design ruling can be taken now,
  > against the tree as it stands — `D3` landing changes nothing the ruling
  > turns on, since the carrier refusal is upstream of the arm shape. **Listing
  > it as a dependency would park a T1 design question behind an unrelated
  > fixture repair**, and the schema check correctly flags a `ready` node whose
  > dependency has not landed. A read is not a dependency.
- [[RT-CARRIER-TRANSFER-UNBOUND-AUTHORITY]] — the other node this chain
  produced, from `D2`. Same parent, unrelated mechanism. **Do not fold.**
