---
scope: fleet
audience: (see scope README)
source: private memory `a-boundary-claim-needs-a-measurement-at-every-position-it-names` (R4 triage, 2026-09-26)
---

# A boundary claim needs a measurement at every position it names

A claim about a boundary or an interval is only as good as the positions it
was actually measured at. Red at A and green at C tells you the transition
sits somewhere in the interval (A, C]. It does not tell you it is at A — and
"the boundary is A" is the sentence everyone writes anyway, because a single
pair of endpoints reads as a point.

## The instance

`RT-D5B-LIVE-WIRING`, 2026-09-16. One sentence in one doc comment was wrong
four ways in one hour, across two seats, and every version rested on fewer
measurements than its own scope required:

    v1  "the boundary is the four specialization assignments"
          from one red + one green. A single pair pins an interval, never
          a point inside it.
    v2  ":1406 is a bound this fixture is blind to"
          a reviewer's mechanism accepted without checking it -- the cited
          guard sat in a DIFFERENT function than the one the code path
          calls.
    v3  "the context-ABI install is not a bound" (correct, structural)
          but still carried "the red appears exactly at the assignments".
    v4  three positions actually measured; the boundary is the
          specialization ABI install, later than the assignments and
          still red.

What settled it was not a better argument. It was running the third
position instead of reasoning about it, and it cost one extra build to get
there.

## The rules

- **A boundary claim needs a measurement at every position it names.** Red
  at A and green at C bounds the transition to (A, C]; it does not locate it
  at A. Measure the middle position before naming it.
- **A single green is two worlds.** Either the dependency the check is
  probing for is genuinely absent, or the fixture cannot see it. Say which
  one was established, or say the two were not distinguished.
- **Separate the universal negative from the fixture-scoped positive**, in
  different sentences, in the artifact. "These fields are disjoint from what
  the derivation reads" is a universal, structural claim; "the red appears
  here" is one fixture's result. Collapsing the two together is how v1 and
  v3 both went wrong.

## The worse failure, wearing the costume of diligence

A correct claim was argued out by a plausible mechanism that was never
checked, and that mechanism was then written into the source as a doc
comment — while posting that the guard had been "verified at source." Only
the *first* of a pair of guards handed over had actually been resolved to
its enclosing function; the second was inherited on trust. The enclosing
function was the entire question: the first guard sat in the function the
code path calls, the second sat in a different function that nothing on
that path calls.

A correction that explains away your own measurement should **raise** your
verification bar, not lower it. Accepting it feels like humility and costs
nothing at the moment of accepting — the cost lands on whoever reads the
resulting comment next.

And "restore your original sentence" is not automatically the repair. When
told to revert to v1, measuring its neighbor first showed v1 was *also*
wrong — restoring it would have shipped a third wrong version.

## How to apply

- Whenever a claim names an interval or a boundary, list every position the
  claim depends on and confirm each one has an actual measurement behind it,
  not an inference from the endpoints.
- Before accepting a plausible correction to your own measurement, re-derive
  it rather than deferring — a correction that feels reasonable is not
  evidence it is right
  ([[a-classification-that-relieves-you-of-an-obligation-gets-more-scrutiny-not-less]]
  covers the adjacent case where the correction also relieves an obligation).
- When a citation hands you a guard, resolve it to its enclosing function
  yourself; do not inherit the second half of a pair on trust just because
  the first half checked out.
- Before reinstating a prior version on instruction — even from the person
  who caught the original error — re-measure it. The instruction to revert
  is not itself evidence the reverted version was correct.
- Treat "inapplicable" and "false" as distinct outcomes a single check can
  collapse into one bit; a green or a red that does not distinguish them is
  not yet a verdict about the boundary
  ([[an-instrument-that-reports-a-verdict-cannot-distinguish-inapplicable-from-false]]).
