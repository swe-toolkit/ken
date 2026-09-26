---
scope: fleet
audience: (see scope README) — anyone authoring or reading a control, probe,
  census, mutation, or execution-count instrument that reports a bare
  pass/fail or a bare zero
source: private memory
  `an-instrument-that-reports-a-verdict-cannot-distinguish-inapplicable-from-false`,
  `zero-executions-and-never-called-are-two-facts-one-number` (R4 triage, 2026-09-26)
---

# An instrument that reports a verdict cannot distinguish inapplicable from false

A verdict is one bit standing for at least three worlds: the code under test
never ran, it ran over an empty population, or it ran over a real population
and the property held. A green control and a control that never had anything
to look at render as the same green. A zero-execution count has the identical
problem one level down: it cannot tell a body that is never emitted from a
body whose loop runs but is empty, from a loop that runs but never takes the
branch being counted — the zero-execution case is a second instance of the
same defect, one number standing for a union of causes rather than for a
single fact.

## The pattern, and why it recurs

Four instruments in one arc were each caught only because a companion
instrument carried population counts rather than a verdict: a passing check
that reported "zero ARMS lines" was true only because the function under test
was unreachable from that fixture; a positive control's "pass" was legible
only because its counts showed the shape was structurally absent from the
fixture rather than the guard being broken; a census that enumerates sites
which *state* a stack property was blind to a site that *needs* the property
and states none; and a mutation's failure message read as a true zero about
the wrong relation entirely, caught only because the message carried counts
instead of a bare verdict.

The same shape shows up in a zero-execution count read as a mechanism claim.
A probe placed inside a function's inner loop measured zero executions and was
reported as "no context is interned for this population." That became a
causal story ("emission owner X, so no context is interned, so no body is
emitted") that a later ruling and a dispatch were both built on. It was false:
a context *was* interned; the function containing the probe was simply never
called, because an unrelated upstream refusal short-circuited the compile
before that pass ran. The probe, sitting inside the loop, could not
distinguish "the pass never runs" from "the pass runs and the loop is empty"
from "the loop runs and the branch is never taken" — three different worlds,
one number, and the strongest of the three readings is the one that got
published and propagated into a design ruling because it was the
implementer's own measurement and nobody re-checked it.

The general form is not about zero specifically — it is about any value that
silently stands for a union, one arm of which is "the question was never
posed": absence from a filtered set means either "passed the filter" or "was
never in the source set"; a missing certificate means either "refused" or
"never asked." A set built by filtering another set inherits that other set's
population, and a measurement taken in a world a pending repair would change
is not evidence about the repaired world, because the repair changes what
gets measured. The same union can also run forward, as a prediction: a claim
that a blocked path "would close once its blocker is fixed" silently resolves
an unposed question in the favorable direction — satisfying the blocker makes
the path *reach* an arm it has never been observed to evaluate, which is not
the same as the path closing.

## How to apply

- When authoring a control, probe, census, or mutation, make it report the
  population it examined (entries, visits, discharges, matched sites)
  alongside its verdict, and make any failure message carry those numbers too.
  A verdict with a zero population is not a result; it is a missing
  measurement.
- Put the population requirement in the acceptance criterion itself — "state
  the counts measured at this venue" — because a control run at a
  zero-population venue passes for the same reason a broken one does.
- Before reporting a zero, an empty collection, a `None`, or a non-membership,
  ask whether the thing was measured and came back empty, or was never
  measured at all. These are different sentences; instrument the entry of
  every enclosing scope (function, loop, branch) so a zero is attributable to
  a named level, cheapest first at function entry.
- When a compile or pipeline can fail upstream of the pass you are
  instrumenting, first establish how far execution actually got — an upstream
  refusal makes every downstream counter read zero for a reason unrelated to
  the mechanism under test.
- Treat "X would close/pass once Y is fixed," about a path that has never
  executed, as an unposed question resolved in the favorable direction. Find
  the source fact that decides what X actually does on its first execution
  rather than asserting the favorable arm.
- Your own measurement is the least-audited input in any review: when a later
  ruling quotes your count back to you, re-derive it rather than recognizing
  it as already correct.

Related: [[a-claim-accurate-about-something-narrower-than-its-reader-infers]]
(a true sentence about a narrower subject than the reader infers is the same
family of defect, one level up from a raw count).
