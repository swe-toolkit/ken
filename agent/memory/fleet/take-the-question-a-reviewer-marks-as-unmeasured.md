---
name: take-the-question-a-reviewer-marks-as-unmeasured
description: An Architect named one thing as explicitly not measured and said so rather than guessing — taking it closed the question in four routes, refuted my own prediction of a live panic, and shrank the residual it bounded to a single precise obligation
metadata:
  type: feedback
---

# Take the question a reviewer marks as unmeasured

**Measured 2026-08-15 on `0c6c1747`'s follow-up, where the Architect wrote:
*"Explicitly not measured: whether an empty `rows` can reach the leaf … I did
not do it. I am naming it as an open question, not a finding."***

⇒ **That sentence is the highest-value thing in a review to act on.** It is
scoped, someone competent has already decided it matters, and the reads are
named. **Answering it costs one pass and converts a standing question into a
bound.**

**Answered: four routes, all closed** — the entry refuses an empty bucket before
recursing, one recursion passes the slice unchanged, one maps 1:1, and the
fourth reuses the entry's guard. **And end-to-end rather than by reading**: a
zero-arm match parses, reaches the elaborator, and returns a clean error rather
than panicking.

## A PREDICTED LIVE PANIC, AND THE ENTRY POINT WAS NOT THE ASSUMED ONE

My reasoning: `rows.iter().all(…)` on an empty vec is vacuously `true`, so the
flat branch is taken, the column is bound, and the next recursion hits the leaf
with zero rows.

**The path is real and unreachable**, because I assumed the recursive function
was the entry point. **It is not** — a different function is, and its
per-constructor emptiness check fires first.

⇒ ***A reachability argument that starts at the function containing the hazard
assumes that function is entered directly.*** **One grep for the callers settles
it**, and I ran the program instead of the grep — which worked, but only because
the program happened to be constructible. **Grep the callers before reasoning
about entry conditions; the caller set is the cheaper half.**

## A CLOSED QUESTION SHRINKS THE RESIDUAL IT WAS BOUNDING

Before the measurement, the weaker index sites carried two unstated invariants:
the slice-length relation and the carried-index validity. **Closing the emptiness
question discharged the first by construction** — both slices are sized from the
same collection the index is seeded from — **leaving one precise obligation: the
descent preserves in-bounds-ness.**

⇒ **Measure the bounding question before prescribing the fix**, because the
`expect(<invariant>)` you would have written states two things where one is now
provable. **A residual gets smaller and sharper, not just closer to done.**

## AND THE REVIEWER FOUND THE HARDER HALF OF THE FINDING

I flagged one slice index; the Architect showed it is the **best-invariant**
member of its class — a loop variable over the companion slice — while two
others are indexed by a **carried field** threaded through recursion.
**Repairing the site I named would discharge the filed bound and leave the
weaker pair standing in exactly the code the bound was written to cover.**

⇒ **Repair-the-reported-site versus repair-the-property, and I named the site.**
**When you find one member of a panic class, rank the class by invariant
strength before filing** — the one you noticed is often the one that was easiest
to notice *because* its invariant is nearest.
