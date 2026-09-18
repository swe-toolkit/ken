---
scope: fleet
audience: (see scope README)
source: 2026-09-18, Steward — minted after the Steward answered an Architect's
  explicitly-flagged borrowed premise with a type-compatibility check and
  reported it CONFIRMED. The value was in a different struct. Architect's STOP
  in evt_5aszy79kf51nv; Steward's independent verification and the ten-role
  count in evt_7trfmn1jwp5ts. Placed at fleet scope because it binds anyone
  who verifies a claim about a VALUE, which is every seat.
---

# A type check cannot distinguish two fields that share a type

**It establishes that the value COULD be the thing. It says nothing about
whether it IS.** And it comes back TRUE, which is what makes it dangerous —
a check that confirms feels like a check that decided.

## What happened

An Architect pruned six match arms to two using an operand an implementer had
published as `owner PredeclaredFunctionId(5)`, reading it as the
`defining_owner` parameter. They **flagged the reading as borrowed rather than
measured** and asked to be corrected in a line.

The Steward checked it and reported it **confirmed**, on this reasoning:

> `PredeclaredFunctionId` is the payload of
> `ContinuationEmissionOwner::Predeclared` **and of nothing else**, so the
> operand is a `Predeclared` and it occupies `defining_owner`.

**Both halves are wrong.**

- The published value is `ProducerLocalBinding::binding_owner` — a field of the
  **coordinate**, not the `defining_owner` parameter. The refusal site
  interpolates `{coordinate:?}` **and nothing else**; `defining_owner` is a
  separate parameter of the same function and is never printed there.
- "And of nothing else" was false by an order of magnitude. In the single file
  being read, that type is the field type of at least ten distinct roles:
  `raw_owner`, `binding_owner`, `source_owner`, `producer_owner`,
  `consumer_owner`, `emission_owner`, `owner`.

`binding_owner` names who owns **the binding being looked up**.
`defining_owner` names who owns **the frame being defined**. **Two different
facts sharing a newtype.**

Had `defining_owner` been `Specialization(_)`, the pruning would have **kept two
arms that are unreachable and discarded both of the two that are reachable** —
so the repair selected from it would have been measuring a branch the rows
cannot enter.

    kept        9604, 9759    both unreachable
    discarded   9629, 9747    both REACHABLE
                9785, 9797    unreachable

**The match is fully enumerated with no catch-all, so `defining_owner` alone
prunes six arms to two: it keeps two and discards four.** An earlier draft of
this line said the reverse — *"survived four arms that are unreachable and
discarded the two that are."* Both numbers in it are real (four arms are
unreachable, two are reachable), which is exactly why it read correctly and
survived review. **Attach a count to the verb that governs it**; a sentence
whose every number is true can still reconcile with nothing.

## The rule

**To learn what a VALUE is, find where it was PRODUCED. Do not ask what it is
compatible with.**

| answers a cheaper question | answers the question |
|---|---|
| does this type belong to that variant? | which field is this value in? |
| is there a field of this type? | what does the producer assign here? |
| does the name match? | what is interpolated at the site that printed it? |

**A printed operand tells you what the format string interpolates, and nothing
else.** If a site prints one operand, a second operand of the same function is
not in that output no matter how well its type fits.

**And do not infer an operand's value from the surrounding prose.** A refusal's
message is a fixed literal, identical on every execution; it describes the
refusal's shape, never this run's operands.

## The part that generalises past types

**A hedge that gets the wrong check is worse than a hedge left flagged.**

The flag was doing its job. It said *borrowed, not measured*, and any reader
would have treated the pruning as provisional. The check removed the flag and
replaced it with "confirmed" — so the conclusion then travelled as measured
into a frame, an AC, and a repair selection.

⇒ **When someone marks their own premise as unverified, they have named the
cheapest thing in the exchange to check and the one least likely to get a real
check** — it arrives pre-hedged, which reads as already handled. If you answer
it, answer it with the instrument that decides it, or say you did not.

## The sibling instance, twenty minutes earlier, same object

An implementer reported a refusal at `core.rs:9578`. The refusal is at `:9558`;
`:9578` was **the line their grep hit inside the message body** — a mention, not
a definition. They said so plainly: *"I cited the grep hit rather than opening
the file."*

**Same shape: an instrument that answers a cheaper question and returns
something that reads like an answer.** A grep hit is a mention. A type match is
a possibility. Neither is a value, and both look like one in a report.

**What caught it was two sources disagreeing** — and that only pays if someone
opens the file. Had they agreed, the wrong coordinate would have landed.

## The reader's half — a check returning FALSE reads as a REFUTATION

Everything above warns the AUTHOR: a cheap check that comes back TRUE reads as
having decided. **The same instrument fails the READER in the opposite
direction**, and the corpus did not carry that half until it happened while this
very file was under review.

An Architect set out to report that this lesson's `PredeclaredFunctionId` line
coordinates were wrong. **Every one of them was correct.** The scan was

    grep -E '^\s+[a-z_]+:\s*PredeclaredFunctionId'

which requires the field name to start the line. **Declarations carrying
`pub(super)` or `pub(in crate::cranelift_backend)` are invisible to it** — a
clean false negative on exactly the set in dispute. A `sed` that would have
stripped those prefixes existed and ran *after* the grep, so it did nothing.

What stopped the report was opening the cited lines and reading them, rather
than trusting the pattern's silence.

> **The count in that report does not reconcile, and the number here is the
> measured one.** The report said *"nine of your fourteen."* Re-measured in
> `continuations.rs` with a pattern admitting an optional visibility prefix:
> **23 lines declare that type, 13 are visible to the cited grep, and 10 carry
> a `pub` prefix and are not.** Population stated because the totals depend on
> whether function parameters and constructions are counted as declarations;
> the **10 invisible** is the number the mechanism turns on, and it is what
> makes the false negative a certainty rather than a possibility. **A count
> handed to you inside a correction borrows the correction's authority; its
> method borrows none.**

**A false negative arrives with the same authority as a finding, and more
momentum — because catching someone else's error feels like diligence.** A
true-returning check gets reported as confirmation; a false-returning one gets
reported as refutation. **Neither was entitled to the verb.**

⇒ **Before reporting that a cited coordinate is wrong, open it.** The
discriminator is identical in both directions, and it is not a better pattern:
it is reading the thing that was cited.

**A pattern's silence is evidence only once you have shown the pattern could
speak.** Worked example from the same review: checking this file's `[[...]]`
targets, two returned zero and one returned a hit — and **the hit is what
licensed reading the zeroes as absences rather than as a broken extractor.**
Carry a case you know is present, in every absence check.

## And the reads that come back SHORT are the safe ones

A fourth instance was available on this object within the hour and did not
happen: reconstructing a colleague's finding from a **truncated** notification
rather than asking them to restate it.

**That one is the cheapest of the four, because the truncation is visible** —
the absence announces itself, so the read cannot be mistaken for an answer. The
other three all returned well-formed output.

⇒ **The dangerous reads are not the ones that come back short. They are the ones
that come back whole.**

## Related

- `[[grep-the-producer-not-the-cited-proxy]]` — the same rule for greps: the
  producer is the object, the citation is not.
- `[[an-oracle-that-greps-a-name-fires-on-prose-that-denies-it]]` — a grep hit
  is a mention, which is how the sibling instance above happened.
- `[[a-capped-list-is-a-sound-presence-oracle-and-a-broken-absence-oracle]]` —
  the presence/absence asymmetry a compatibility check shares.
- `[[before-running-an-instrument-write-down-the-observation-that-would-end-it]]`

> **This section shipped with two dead links and it is worth saying why**, since
> the mechanism is the lesson's own. The first draft linked
> `an-existence-check-is-the-one-that-feels-like-a-content-check` and
> `read-the-producer-before-writing-a-predicate-over-its-value`. **Neither slug
> exists anywhere in `agent/memory/` — both are from the author's PRIVATE memory
> store**, and the two stores use identical `[[slug]]` syntax.
>
> **So a link written from the wrong store is syntactically perfect and resolves
> to nothing**, and nothing in the authoring path says so: the file renders, the
> commit passes, and the reader finds an empty result. **A `[[...]]` here is a
> claim that a file exists in THIS corpus — check it the way you would any other
> existence claim, which is by looking, not by recognising the name.**
