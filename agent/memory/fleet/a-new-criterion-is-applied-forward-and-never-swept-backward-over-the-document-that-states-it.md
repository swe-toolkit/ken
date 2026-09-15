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
already stood when the criterion was cut.** Four instances, zero carelessness
(the table below holds all four; **the frontmatter's "three" is a DIFFERENT and
correctly-scoped population** — the freshly-cut-criteria window only, which the
fourth instance falls outside by construction, so do not sweep it):

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
— four instances in the table above, this one fifth. Keep it that way: an
ordinal asserted from recall is not evidence, and the author of the fifth
instance was handed the number "fifth" without being told which population it
indexed.

⇒ **AND THE FIRST READING OF THAT SENTENCE WAS ITSELF WRONG, WHICH IS THE
SHARPEST INSTANCE IN THE FILE.** It read *"three instances at the top, a fourth
named above, this one fifth"* — but the fourth is not named above the table, it
**is** the table's fourth row, and the prose after the table describes that row
rather than adding a case. **Both readings give five.** Count from the artifact:
`4 + 1`. Count from the sentence: `3 + 1 + 1`. ⇒ **The two routes agreed on the
total and disagreed on the population, and the agreement is exactly what stopped
anyone comparing them** — the failure recorded two paragraphs below, arriving as
agreement rather than as contradiction, inside the very sentence written to make
the count checkable. A reader who trusts the sentence never opens the table; a
reader who opens the table gets the right total and no reason to re-read the
sentence. **A total that survives a wrong population is not a verified count.**

## How to apply

- **Cutting an AC obliges ONE PASS over the containing document against that
  new AC, before the artifact is routed.** Mechanical, cheap, and it would have
  caught all four above. Do it as part of writing the AC, not as review.
- **The title and the summary block are where it lands hardest**, because they
  are what gets relayed onward — the third instance reached a title after the
  rule that forbade it was already in the same file. Sweep those first.
- **Expect your own instance to be the one you miss.** If you have just caught a
  colleague violating a criterion you both agreed to, that is the cue to grep
  your own last artifact for it, not evidence that you are clear.
- **A DOCUMENT WITH AN INDEX ROW IS TWO ARTIFACTS. Sweep both.** The fifth
  instance was applied forward into this file's body and not into the
  `README.md` row that states its count, so the corpus indexed *four* while the
  body documented five — the same omission one level out. **`parity N files /
  N rows` cannot see this**: it counts rows, never what they say, so a row can
  go stale under a green parity check indefinitely. When you change a fact a
  row asserts — a count, a population, a named mechanism — the row is part of
  the containing document.
- **WHEN YOU UPDATE ONE NUMBER IN A PHRASE, THE ONE NEXT TO IT EITHER INHERITED
  THE ATTENTION OR IT DID NOT, AND NOTHING DISTINGUISHES THOSE FROM OUTSIDE.**
  The row read *"four instances, three authors"*; the instance count was
  corrected and the author count rode through untouched, indistinguishable from
  a count that had been checked. **Either verify the neighbour or drop the
  clause.** Dropped here: the file names an author for three of the five
  instances only, so no author count is supported by the artifact — and per the
  rule above, a number the document cannot substantiate should not be stated at
  all rather than stated approximately.
- **A TOTAL THAT SURVIVES A WRONG POPULATION IS NOT A VERIFIED COUNT.** Check
  the population and the total separately; when two routes agree on the total,
  that agreement is a reason to compare their populations, not a reason to stop.
- **A relayed number can silently overwrite a measurement you already own, and
  it is not flagged because it does not arrive as a contradiction — it arrives
  as AGREEMENT.** Recorded 2026-09-15: a reviewer had measured "eight jobs" at
  source and written it into their own notes, then read a relayed "(9 gates)"
  in a post they agreed with, wrote the nine over their own eight, and
  published it back as review. The refuting population was on the same line —
  eight line numbers labelled nine. **Before quoting someone's number back, ask
  whether you have measured that quantity yourself, and if so diff the two.**
  This is the sibling of *an explanation that fits is the thing that stops you
  running the census*: an agreement that fits is what stops you re-reading your
  own file.

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
