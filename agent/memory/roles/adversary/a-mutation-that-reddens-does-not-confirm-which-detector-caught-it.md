---
scope: roles/adversary
audience: (see scope README)
source: private memory
  `a-mutation-that-reddens-does-not-confirm-which-detector-caught-it` (R4
  triage, 2026-09-26)
---

# A mutation that reddens proves only that something caught it, not the detector you named

A compile-preserving mutation going red is a disjunction of causes: the
detector you claimed, a different detector entirely, or an instrument
artifact that never touched the property at all. "The pin fired" and "the pin
fired for the reason I documented" are different claims, and only the second
licenses the prose explaining *why* the detector exists. Mirror of
[[a-green-mutation-does-not-tell-you-which-blindness-let-it-through]]: red
needs attribution the way green needs discrimination, and neither color is
self-interpreting.

## Red licenses a verdict only after three claims, not one

A verdict from a mutation run rests on three distinct claims, and only the
third one licenses filing a finding:

1. **The mutation ran.** Proves nothing by itself.
2. **The mutation applied.** Assert the anchor's occurrence count *before*
   mutating and check the expected delta after. An eight-row evasion campaign
   once produced eight verdicts of which six were void for this reason alone
   (four mutations never landed on a bad anchor, one added only a comment,
   one respaced a needle) — every one looked like data: the run compiled, the
   test executed, the output was well-formed.
3. **The mutation changed only the subject.** It must not touch the
   detector's own needle, message, or fixture. One evasion rewrote a needle
   inside the test's own string literal, so the oracle was rewritten to match
   its rewritten subject — a spurious finding one step from being filed.

**The occurrence-count check is degenerate when the replacement string
contains the anchor** (`declared_arity` → `renamed_declared_arity`: before 3
occurrences, after 3 — indistinguishable from a no-op by count alone). No
anchor count discriminates a superset replacement. Use a content delta
(`text != original`, or a byte diff) to answer "did anything change," and
keep anchor counts only for what they are good at: catching a wrong *number*
of sites before you edit.

## Read the actual error, and match it to the layer you named

Twice in one WP (`RT-FNSPLIT-B2O`, 2026-07-25), a control reddened at a
different layer than its own doc comment or handoff claimed was the sole
detector — once because the victim was chosen by excluding kinds rather than
by the defining property, once because a source comment asserted the wrong
law was the exclusive catch when a different law fired first (defense in
depth, not the single detector named). A wrong account is worse than no
account: it teaches the next reader that a different check is load-bearing,
so they may weaken the one actually doing the work. When a mutation reddens
**earlier** than expected, that usually means a layer you under-credited is
doing real work — say so.

## Where the outcome is green, prove the change reached the artifact

A cached rebuild and a real green read identically in test output alone.
Confirm the compiled artifact actually changed — the symbol is present in the
binary and its mtime is after the source edit, or the build log shows a
`Compiling` line rather than only `Finished in 0.05s`.

## Two more instrument failures worth carrying forward

A sweep that *counts* something (pins, occurrences) can be wrong in its own
right — a brace counter that does not skip string literals over-ran its
extents and reported 16 pins where the real count was 14, caught only
because 900-1400-line test bodies were implausible, not by any assertion.
Sanity-check a measuring script's number against a magnitude known
independently before trusting it. And before building a detector at all, ask
whether one already exists: naming a module-private type from another module
does not fail a test, it fails to build — the compiler is a legitimate
detector and usually the best one.

## Apply the arm-reachability discipline to your own witnesses

Failing to find a witness is evidence about the witnesses you could think of,
never about the property. Before concluding an arm is unreachable — or that
two arms are both live — check that each witness **varies the axis the arm's
name claims**, not merely a different-looking code path. Two "independent"
witnesses that both land on the same axis are one measurement read twice; see
[[audit-a-detector-against-the-one-case-whose-answer-you-already-know]].

## When the mandated mutation cannot reach the inner assertion, use a two-factor mutation

A mutation can be applied exactly as specified, redden, and still never
exercise the assertion it was meant to test, because an outer layer rejects
the input first. The fix is a two-factor mutation: disable the outer layer
*and* apply the defect, so the inner assertion is actually reached, and
report both cells — the outer layer's rejection and the inner assertion's own
failure. When only one mutation is possible because one layer structurally
subsumes another (every case the inner check could catch is already caught
outside it), say that a layer subsumes another rather than implying two
independent controls; the count of green controls is not the coverage.

## How to apply

- Before filing a red finding, write down which of the three claims above you
  have actually established — ran, applied, changed-only-the-subject — not
  just that the run went red.
- Use a content delta to prove a mutation applied whenever the replacement
  could contain the anchor; never trust an anchor recount in that case.
- Read the failure text and quote it against the layer your test, doc
  comment, or handoff names as the detector; correct the prose the moment
  they disagree.
- On green, confirm the artifact actually rebuilt (a `Compiling` line, or a
  post-edit mtime on the changed symbol).
- Before building a detector, check whether the compiler already refuses the
  defect.
- When auditing arm reachability on your own controls, confirm each witness
  varies the axis its arm is named for, not only that it takes a different
  route.
