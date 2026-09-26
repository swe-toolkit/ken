---
scope: fleet
audience: (see scope README) — anyone carrying a number, a ratio, or a "fixed"
  claim from the tree it was measured on to a different tree (a PR head to
  main, an earlier respin to a later one, a probe binary to a target binary)
source: private memory
  `a-measurement-correct-on-one-tree-becomes-a-cross-tree-claim-only-through-a-carry-argument`
  (R4 triage, 2026-09-26)
---

# A measurement correct on one tree becomes a cross-tree claim only through a carry argument

A number measured correctly on tree A is promoted to a property of tree B by a
separate step — the carry argument — and the carry, not the measurement, is
where these claims fail. Name the tree the number was measured on and the tree
it is being claimed about in the same sentence; if they differ, the carry is a
distinct thing to check, and it is almost never re-checked once the
measurement itself is confirmed correct.

## Why it hides

A measurement carries an air of settledness that its scope does not deserve.
"We measured 34" and "the population is 34" differ by an argument that is
usually one line long, phrased as a verification, and written by the same
person at the same moment to discharge the doubt they already had — so it
tends to be built from whatever instrument makes the doubt go away. When a
classification decides what that verification can see (a row is labeled as
belonging to a class the verification's instrument cannot detect), the
verification cannot fail by construction, and the false carry rides in beside
a true measurement with no apparent contradiction.

A ratio is the worst place for this to hide: both operands are the same unit,
so nothing in the arithmetic objects when they come from different trees. A
recorded "1968 bytes / 512 bytes / roughly 4x" turned out to compare two
different binaries' worth of static-TLS reservation; re-measured on the actual
detector binary the true ratio was 0.28x — it did not shrink, it inverted. The
tell in that case: a ratio that fails toward *confirming* a conclusion already
forming gets no further scrutiny, while a ratio that fails toward alarm
usually does.

A carry can also expire silently through time rather than through a tree swap.
Pinning both sides of a comparison and skipping the re-check "because the
freeze holds" is itself a carry: "if the freeze holds" is a condition
evaluated at read time, and a pin only records one operand — it makes the
later check cheap, never performed. The asymmetry that makes this dangerous:
a run destroyed before it finishes is loud, and you start over; a run that
finishes and then has the world move under it before it is read is complete,
internally consistent, correct for its own tree, and silently no longer about
the thing you asked.

The same failure recurs in a handoff sentence describing your own earlier fix
("both should-fixes are in this respin") when a later respin drops the fix but
not the sentence describing it — tree A is your own earlier draft, tree B is
what you are about to hand off, and a done-claim reads identically whether or
not the tree it describes still contains the work. It escapes review because
it feels like bookkeeping rather than a finding, and scrutiny follows claims
that feel like findings.

## How to apply

- Separate the measurement from the carry, out loud, every time: name the tree
  the number came from and the tree it is being claimed about.
- Read the carry's verification as an instrument and ask what it cannot see —
  a classification that exempts one item from the verification's method makes
  the verification vacuous for exactly the item that matters.
- When forming a ratio or any comparison of same-unit numbers, name the
  position (tree, binary, commit) of each operand before naming the result. A
  quotient of numbers from different positions is a cross-tree claim wearing
  arithmetic.
- A pin records an expected value; it does not perform the comparison. Re-run
  the check at read time rather than trusting that a recorded pin plus an
  unbroken freeze equals a completed comparison.
- Treat a "done" sentence about a specific tree as expiring at every respin of
  that tree. `git rev-parse <sha>:<path>` on both trees is the whole
  re-check, and it is cheap enough that skipping it saves nothing.
- When your conclusion is confirmed, that is evidence about the conclusion and
  none at all about the route to it — if a step in the argument was doing real
  work, check it on its own terms even after the answer lands correct.

Related: [[a-correction-on-main-does-not-reach-a-candidate-cut-before-it]] (the
same tree-boundary gap, viewed from the correction side).
