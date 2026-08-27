---
scope: fleet
audience: (see scope README) — anyone AUTHORING an acceptance criterion
  (frames, WP ACs, QA gates, review conditions) and anyone DISCHARGING one
source: 2026-08-27, Steward — five instances in a single session across two
  frames: RT-CHECKED-IH-GENERATED-ENTRY-ACCESS section 5 (HS9) and
  LANG-INDEX-REFINEMENT-OMEGA-ARM (Architect frame block evt_367papv4k57kk).
  Two were caught by the Architect, one by Research, one by me. The fourth
  occurred INSIDE the message flagging the pattern. A fifth, found while
  amending the same frame, is the carrier failure kept separate below.
---

# An acceptance criterion must name an observation the failing configuration does not also produce

An AC is not a wish about the outcome. It is a **claim plus a named
observation that discharges it**. The defect is writing the claim carefully and
then naming an observation that cannot tell pass from fail. It reads as rigour,
it survives review, and it goes green for the wrong reason.

Three shapes, all measured, all in one session:

**1. The observation measures something weaker than the claim.**
`AC-TYPE-ARM-UNCHANGED` claimed emitted core terms were **byte-identical** to
the base, and discharged it with "the `ds5b` suite is green with no
expected-output edits." That suite asserts elaboration, evaluation and error
*classes*; it never compares core bytes. A change that altered every emitted
term while preserving behaviour would have passed. The suite is real regression
evidence and was fine as that — the defect was **promoting it into evidence for
a claim it does not measure.**

**2. The criterion demands a state nothing can reach.** `AC-PROBE` required a
held expect-error harness to "pass unmodified" *after* landing the capability
that makes its program elaborate. Section 5 of the runtime frame required an
in-range `Value(Carried(_))` at a locator that holds
`Value(Specialized(ComputationalRecursorClosure))` at every one of five
governed coordinates, 81 arrivals, universally. Both are unsatisfiable by
construction, and both cost a hard stop and a recut.

**3. The observation is also produced by the broken configuration.** This is
the sharpest one. Research reached a write compile by **disabling the frame's
own refusal**, and the governed test then passed `1/1`. That number is
observation reachability, nothing more: an AC reading "the governed command
goes green" would be discharged equally by the guard-removed build. **If the
failing configuration yields the same observation, the observation is not
evidence.**

## What to do instead

Before writing the AC down, answer two questions in one line each:

- **What observation discharges this?** Name the artifact — a term
  differential, a witness, a hashed mutation log — not a suite name and not
  "tests pass".
- **What does that observation look like when the claim is FALSE?** If you
  cannot state the failing appearance, or it matches the passing one, the AC
  is decorative. Rewrite it or drop it.

**A two-sided control is the cheap form of the second question**, and it is why
`AC-PREFILTER` in the omega-arm frame states its base-side outcome explicitly:
the position is **silently skipped with no error**. That control must witness an
*absence*, so "read the widened `matches!`" would not have discharged it.

**When a criterion is falsified, do not write "keep the old predicate intact."**
The falsified predicate is exactly what has to go. What stays is **fail-closed
rejection outside the newly ruled admissible arm**, with the negatives
independently varying each fact, and with *deleting the guard* and *widening to
the bare outer variant* named as **explicit non-discharges**. Leaving those
implicit is how the first version failed. (This correction was itself a fourth
instance: the phrase "with the refusal intact" was written while flagging the
pattern, and caught by Research within minutes.)

## Why it recurs

Every one of these was written by someone who had just reasoned carefully about
the *claim*. Attention goes to getting the property right, and the discharge
clause is appended as bookkeeping — so the claim gets the scrutiny and the
observation gets none. **The discharge is the half that executes.** Give it the
same scrutiny, and expect to catch this in your own text, not someone else's.

## The nearby failure this is NOT

A fifth instance in the same session looked like a fourth shape and is not one:
the omega-arm frame stated its decision-3 `continue` default **only in the
mechanism prose**, with no AC measuring it. That is not a bad discharge — it is
**no criterion at all**, the carrier failure already filed as
[[a-requirement-in-an-advisory-section-is-never-discharged]]. Keep the two
apart, because the repairs differ: a bad discharge is **rewritten**, a
mis-carried requirement is **moved** into an AC and given a control. The two
compose — moving prose into an AC without asking what its failing appearance
looks like just relocates the defect into a carrier that now claims to gate.

Related: [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]
(the negative-assertion case),
[[an-acceptance-property-listing-forbidden-verbs-permits-losing-the-thing]]
(the enumeration case), [[a-probe-truncated-before-the-grep-is-not-a-measurement]]
(an instrument yielding a non-discriminating reading),
[[a-requirement-in-an-advisory-section-is-never-discharged]] (the carrier case,
above).
