---
scope: fleet
audience: (see scope README)
source: private memory `a-hedge-does-not-constrain-what-is-done-with-the-claim-it-hedges`, `discounting-evidence-weight-is-not-declining-to-assert-it` (R4 triage, 2026-09-26)
---

# A hedge or a weight discount attaches to what evidence proves, not the act

A caveat and a discount are both statements *about* a piece of evidence —
what it does or does not establish. A downstream act (sizing a node,
scheduling a queue, writing an acceptance criterion) or a downstream
assertion (repeating the claim as fact) is a decision that *presumes* the
claim. Nothing structurally connects the two. Stating the limitation, or
correctly downgrading the evidence's weight, is checked for **presence** —
*did I say it?* — while the act or the assertion is made on the claim's
**content**, which the hedge or the discount never touched. Both moves feel
like the whole of the diligence and are only half of it.

## Three instances of the hedge failing to constrain the act (2026-09-18)

- A depth census stated its caveat correctly under "Stated limits" and
  contradicted it under "Verdict." The relay downstream carried the
  Verdict — a verdict reads as a result, a limit reads as hedging, and only
  one of the two travels.
- An implementer wrote "my span rule is too wide, I discharge anything past
  the bind line, including past the release" and then reported all eight
  discharges as real. One was past a release.
- The exception: a Steward, told a lean made a node look cheaper, **declined
  to re-size it** — "the size should move when the discriminator returns,
  not when the lean is stated." Only this one is a catch; the first two are
  the default outcome.

The erosion is invisible because it happens in a *different* artifact than
the one that stated the caveat. The lean is honest where it is written. It
becomes a finding in the sizing field, the schedule, or the acceptance
criterion — none of which quote the original hedge, and none of which look
like a claim about evidence at all. High-traffic channels this travels
through: **sizing** (an S/M/L that presumes the lean is true), **scheduling**
(a queue position that presumes the cheap outcome), **scope** (an AC written
against the leading outcome), **acceptance** ("confirmed" for work that only
tested the lean's branch).

## One instance of the discount failing to constrain the assertion

A relayed count — "the seat running the publisher passed the wrong flag
three times tonight" — arrived secondhand, read over someone's shoulder from
a terminal pane, never opened directly. It was written up with the
correctly reasoned discount "they corroborate the mechanism and do nothing
for the population" — true, and then asserted as fact anyway, twice,
including into a durable state file. The seat named in the claim denied it,
and named a prior retraction of the identical claim by the same relayer.

Discounting a claim's weight and declining to assert it are two separate
acts, and only the first was performed:

    discounting the WEIGHT   "this does not support my conclusion"    -- done
    declining to ASSERT it   "so I will not state it as fact"         -- not done

The discount protects the *argument*; it does nothing for the person the
claim is about. The harm of asserting someone's conduct does not scale with
how load-bearing the claim was to your point — an aside repeated as fact
convicts exactly as hard as a premise.

## The forbidden act, named directly

**Name the act the hedge or the discount forbids, not the uncertainty it
expresses.** "This is a lean, not a finding" does nothing on its own — it
describes an epistemic state and leaves every downstream act available.
"Do not re-size on this" removes one. Likewise, "this is only corroboration"
is a reason to delete the sentence that follows it, not a license to keep
the sentence and add a qualifier: anything you would not stake the
conclusion on is something you should not be asserting as fact, discount or
no discount.

## The remedy, when two accounts disagree, is to measure, not to pick a side

Faced with a denial and a secondhand claim, the move that actually resolved
it was not weighing a pane transcript against a seat's testimony — it was
measuring the underlying observable directly: a merge landing before its own
checks finish, checked across 20 merges by comparing `mergedAt` against the
last `completedAt` on each PR head, with file sets re-derived from the git
objects rather than an API. The result corroborated the denial, and surfaced
a finding neither account contained. **A claim about a named seat's conduct
is not assertable until it has been checked against an instrument that seat
does not control**; being able to name that instrument afterwards is not the
same as having run it before asserting. The order that keeps failing is
assert, get contradicted, then verify — the same measurement run before
asserting costs the same four tool calls and the claim never exists.

## How to apply

- When you hand over a lean, hand over its discriminator in the same
  sentence — the specific check that would convert it into a finding. A
  bare lean tells the recipient only what you suspect.
- When you receive a lean, or a discounted claim, ask what you are about to
  decide that presumes it is true. That question is the actual defense, not
  the wording of the hedge.
- Put the caveat in the headline when it qualifies the headline; a "stated
  limits" section that sits apart from the "verdict" does not travel with
  it.
- Before writing a claim about a named seat's conduct, run the check against
  an instrument that seat does not control, before asserting — not after
  being contradicted.
- If you catch yourself writing a hedge that reduces a claim's importance,
  treat that as a signal to delete the claim, not to keep it with a
  qualifier attached.
