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

Had the value been the other variant, the pruning would have survived four arms
that are unreachable and discarded the two that are — and the repair selected
from it would have been measuring a branch the rows cannot enter.

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

## Related

- `[[an-existence-check-is-the-one-that-feels-like-a-content-check]]`
- `[[read-the-producer-before-writing-a-predicate-over-its-value]]`
- `[[before-running-an-instrument-write-down-the-observation-that-would-end-it]]`
