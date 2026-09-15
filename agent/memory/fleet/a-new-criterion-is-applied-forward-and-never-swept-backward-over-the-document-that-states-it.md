---
scope: fleet
audience: (see scope README)
source: 2026-09-15, RT-NATIVE-COMPILE-RUNS-AT-THE-STACK-WALL — three independent
  violations of two freshly-cut criteria, by three different authors, inside the
  one-hour window in which those criteria were written, all in the artifact that
  states them
---

# A new criterion is applied forward and never swept backward over the document that states it

Two criteria were cut in one hour while a node was being written. **Both were
then violated inside that same node, by its own authors, in sentences that
already stood when the criterion was cut.** Three instances, three authors,
zero carelessness:

    AC-7 "state the build profile
          on every stack figure"      the Architect restated a cross-node
                                      predicate without its profile, ONE
                                      MESSAGE after proposing AC-7

    AC-7                              the node's TITLE claimed "94% of the
                                      budget" without saying debug -- in the
                                      node that cut AC-7, written by the Steward

    "an interval measurement has      the canary block derived every percentage
     no point value; record the       from 1936 and 336, the midpoints of
     interval"                        (1920, 1952] and (320, 352], which were
                                      never measured -- 20 lines after the
                                      interval defect had been named, and from
                                      there it reached the title

    "a claim inherits the scope of    a three-site census answering "does any
     the site you checked, not the     gate build release?" was allowed to
     scope you stated"                 answer "has anyone ever measured
                                       release?" — by the author of that very
                                       lesson, while auditing the identical
                                       shape in other people's work all night

**The mechanism is not inattention. A criterion arrives attached to the next
claim you make, and the sentences already written are not re-read as claims —
they have become the background you are writing against.** Each author caught
somebody else's instance immediately and walked past their own. **The fourth
instance extends the shape past freshly-cut criteria to long-held ones: the rule
you are actively enforcing on others is the one you stop applying to yourself.**

**FIFTH INSTANCE, 2026-09-15, and it is the sharpest — it happened in a commit
cut from the commit that landed THIS FILE.** Writing
[[a-workflow-green-can-mean-the-jobs-that-matter-were-skipped-not-that-they-passed]],
the Steward published a mechanism claim (*"the matrix never expanded"*), had it
refuted, and shipped a correction whose **commit message explicitly disowns the
phrase** — while leaving that same phrase standing in the evidence block
twenty-five lines above the line it fixed, and in the frontmatter. A reader met
the disowned wording first, sitting above the corrected text contradicting it.

⇒ **The shape extends once more: it is not only new criteria that go
unswept, but your own retractions.** Disowning a phrase in a commit message
feels like having removed it. **Cutting a correction obliges one pass over the
whole containing document for the phrasing you just disowned**, not only over
the sentence you came to fix — the same single mechanical pass this file already
prescribes for a new AC, applied to a withdrawal.

**The count is checkable against this file rather than against anyone's memory**
— three instances at the top, a fourth named above, this one fifth. Keep it
that way: an ordinal asserted from recall is not evidence, and the author of the
fifth instance was handed the number "fifth" without being told which population
it indexed.

## How to apply

- **Cutting an AC obliges ONE PASS over the containing document against that
  new AC, before the artifact is routed.** Mechanical, cheap, and it would have
  caught all three above. Do it as part of writing the AC, not as review.
- **The title and the summary block are where it lands hardest**, because they
  are what gets relayed onward — the third instance reached a title after the
  rule that forbade it was already in the same file. Sweep those first.
- **Expect your own instance to be the one you miss.** If you have just caught a
  colleague violating a criterion you both agreed to, that is the cue to grep
  your own last artifact for it, not evidence that you are clear.

## The specific arithmetic rule, because it recurs

**A bracketed measurement has no point value.** From `(lo, hi]`, report the
derived interval, never a midpoint. **Prefer the subtraction to the division**:
`2048 - (1920, 1952] = [96, 128)` is exact and needs no invented point, whereas
`2048/1936` silently invents one.

⇒ **And the absolute form is usually the one that answers the question.** "About
6% of headroom left" cannot be acted on without knowing 6% of what. **"Room for
exactly one more 64 KiB addition and not two"** is true at every point of the
bracket, carries no error bar, and is the form the decision actually needs.

Related: [[a-claim-accurate-about-something-narrower-than-its-reader-infers]],
[[a-mechanism-claim-in-a-comment-is-structurally-exempt-from-execution]].
