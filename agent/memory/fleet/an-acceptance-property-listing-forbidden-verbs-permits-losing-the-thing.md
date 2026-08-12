---
scope: fleet
audience: (see scope README)
source: 2026-08-12, `D2k-1b-i` — a preflight enumerating seven ways a field
  could be used, and none of the ways it could be lost; four of five rows went
  green by dropping it
---

# An acceptance property that lists forbidden verbs permits losing the thing

A frame required a whole-graph refusal for any path that would **carry,
allocate, store, join, project, return or publish** a constructor containing a
compiler-only worker field. Seven verbs, each a real way the field could escape
into emission.

The increment was built and it satisfied all seven. **Four of the five rows
went green by dropping the field**, which is none of them.

Every row recorded its worker field and zero bare reads. **No row consumed
one.** A dropped field is not carried, not allocated, not stored, not joined,
not projected, not returned and not published — so it satisfies an enumeration
of uses **vacuously**, and the suite reports it as progress.

## The shape

**An enumeration of forbidden verbs is a list of ways to USE a thing. Losing it
is not a use, so no list of uses can forbid it.** The gap is not that the list
was too short — adding *drop* would leave the next non-use uncovered. The list
is the wrong instrument.

⇒ **State the acceptance as a TOTAL over the population:**

| instead of | write |
|---|---|
| nothing may carry / allocate / store / publish X | **every X is consumed at <site>** — none dropped, none reaching <site> unconsumed |
| no path may leak the handle | every acquired handle is released on every path |
| no row may skip validation | every row records a validation verdict |

The total names what must be **true of every member**. The enumeration names
what must be **absent**, and absence is unbounded.

## My first replacement was ALSO too narrow, and that is the sharper lesson

Having caught the verb list, I wrote the total as *"every X is consumed at
`<site>`"* — one disposition. **The lane owner's ruling widened it, and the
widening is the point:**

> each recognized X is **consumed exactly once**, **erased before construction
> under positive unobservability authority**, or **refused before emission**;
> **none is dropped**.

⇒ **Take the total over the DISPOSITION SPACE, not over the one disposition you
have in mind.** Legitimate members may need a different lawful ending than the
happy path — erasure and refusal are both correct outcomes here, and a total
naming only *consume* would have made every refusing row look like a failure
while still permitting the drop it was written to stop.

**The closure clause is what does the work.** "None is dropped" is what makes
the list exhaustive rather than merely longer; without it, a three-item list is
the same instrument as a seven-item one.

## An absence is not an authority

The row that looked droppable was justified by *"zero destructures observed"*.
That is not a proof of unobservability — **it proves only that the current
route did not reach the consumer.** The source graph contained the elimination
the whole time; the lowering route did not get there.

⇒ **"We measured none" and "none can exist" are different claims, and only the
second licenses skipping the work.** The second needs positive authority plus a
mutation that makes the thing observable and flips the outcome. Read a zero as
a question about your instrument's reach before you read it as a property of
the world.

## Why it survives review

A verb list reads as rigorous — it is specific, it is long, and each entry is
individually correct. **Its holes are invisible by construction**, because you
cannot see the item that is not on a list. The total has no holes to see.

## The companion check: compiling is not passing

The same increment had a second tell that was already measurable and not
required: the rows **compiled**. A green row was being read as a discharged
row.

⇒ **Where a property says "consumed", the control must count consumptions.** A
row that compiles with **zero** consumptions is a failure, and the artifact must
say so in those words. The implementer had already built this per-row sentinel;
it was evidence rather than a deliverable, which is the only reason the defect
needed a whole turn to surface.

## How to apply

- **When you write "nothing may <verb> X", stop and ask what happens if X
  simply vanishes.** If vanishing passes, rewrite as a total.
- **A "no consumer becomes green here" clause is the same error inverted** — it
  bounds the increment by an outcome rather than by a property, and green can
  arrive for a reason the clause never contemplated.
- **A frame whose banned scope forbids the only discharge of its own criterion
  is a frame defect, not a ring failure.** Here the consumer seam that would
  have satisfied the property was explicitly out of scope, so the increment
  could not pass by any means available to it. The hard stop was the correct
  outcome and it is what surfaced this.

Related: [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
