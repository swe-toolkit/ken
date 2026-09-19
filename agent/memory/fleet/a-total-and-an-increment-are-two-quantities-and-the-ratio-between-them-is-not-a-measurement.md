---
title: A total and an increment are two quantities, and dividing one by the
  other produces a number that looks measured
scope: fleet
recorded: 2026-09-19
recorded_by: steward
---

# A total and an increment are two quantities, and the ratio between them is not a measurement

**Measured 2026-09-19, Steward chair. Three times in one session, on three
different objects, in under two hours.** Each time the arithmetic ran, produced
a plausible number, and the number was meaningless because its two inputs
measured different things.

    128 KiB   the TOTAL stack an RStandardOp descent consumes
      144 B   the INCREMENT two new match arms added to one frame
    ------
      ~910    "levels of descent" -- a number describing nothing

The division is type-correct (bytes over bytes) and dimensionally plausible
(bytes-per-frame gives frames), which is exactly why nothing stops it.

## THE THREE INSTANCES, BECAUSE THE SHAPE IS CLEARER THAN ANY ONE OF THEM

1. **`1968 B` vs a `512 B` reservation**, read as "4x over budget." The real
   per-frame figure was `144 B` — `0.28x`, which **inverts the conclusion**.
2. **"There is no 128 KiB figure."** I searched the detector's CONSTANTS
   (2 MiB, 512 B), found no match, and declared the figure nonexistent. It was
   never a constant; it was the descent TOTAL. I had searched the wrong
   quantity and read absence as refutation.
3. **`128 KiB / 144 B ~= 910 levels`**, above. Refuted by measuring the
   fixture: `Map.ken.md` maxes at **28 columns of indentation across 15,337
   lines**, capping expression nesting near 10-15. The depth that division
   implied does not exist.

## WHY IT SURVIVES THE CHECKS THAT CATCH OTHER ERRORS

**A ratio is self-certifying in a way a raw number is not.** A lone number
invites "measured where?" A ratio arrives already looking like the answer to
that question — it *is* a relationship between two measurements, so it reads as
having been checked against something. **The checking is the illusion: the two
inputs were each measured correctly, and neither one licenses the division.**

It also produces a number in the right ballpark, every time. `910` is a
plausible recursion depth. `4x` is a plausible overrun. Nothing about the
output flags the error — only re-deriving one input independently does.

This is [[a-stale-number-that-drifts-into-being-correct-cannot-be-caught-by-checking-it]]
run forward: there the number was right for the wrong reason, here it is wrong
for a reason no check on the number can see.

## HOW TO APPLY

- **Before dividing two measured quantities, say what each one is a
  measurement OF, in a full sentence.** "128 KiB is what the whole descent
  costs" and "144 B is what two arms added to one frame" do not compose, and
  writing them out is what makes that visible. If you cannot write both
  sentences, you do not have two measurements.
- **A TOTAL and a DELTA are the specific pair that catches people**, because
  both are reported in the same units by the same instrument, often in the same
  sentence. Ask of every figure you are handed: *total, or change in total?*
  The producer frequently does not say.
- **When a search for a figure comes back empty, check that you searched the
  right QUANTITY before reporting the figure does not exist.** An absence found
  while looking at constants says nothing about a total. See
  [[before-declaring-a-citation-unresolvable-check-that-your-instrument-could-have-produced-a-hit]].
- **A refutation of your own mechanism story is worth more than the story
  was**, and should be published with the same energy. The refuted `910` is
  what forced the real question: with depth small, FRAME SIZE must dominate —
  which changes which repair is correct, not merely which number is right.
- **Stop producing arithmetic in an area where you have already conflated
  once.** Get the instrument to emit the number instead. See
  [[prefer-the-number-the-producer-already-emits-over-one-you-count-yourself]].

Siblings:
[[a-measurement-correct-on-one-tree-becomes-a-cross-tree-claim-only-through-a-carry-argument]]
(a ratio hides the missing carry argument — same concealment, different joint),
[[state-the-split-not-the-sum-a-measurement-is-not-a-criterion]],
[[an-unclassified-residue-is-a-reading-on-your-classifier]].
