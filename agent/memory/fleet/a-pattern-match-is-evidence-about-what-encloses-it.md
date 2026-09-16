---
name: a-pattern-match-is-evidence-about-what-encloses-it
description: A grep hit, a line number, an attribute position, a row's neighbours — each is evidence about the ITEM THAT ENCLOSES IT, and you do not have a finding until you have resolved that item. Stated as "check before you report" the rule excludes its own best instance, because a prevention produces no artifact. Six instances in ninety minutes across three seats; the one that cost nothing is the one that never became a claim. A negative control has two readings - absence MAINTAINED (the finding is refuted) versus absence UNEXAMINED (it is unasked, and belongs to the artifact's owner) - and the same failure runs on RULES, whose enclosing item is the precondition they are scoped to.
metadata:
  type: feedback
---

# A pattern match is evidence about what encloses it

**Measured 2026-09-16, across ninety minutes of the `RT-D5B` refusal-gate
thread.** Six instances, three seats, one shape. The first four are pattern
matches whose answer turned on the enclosing item — three caught after the
number existed, one before:

    census keyed on a spelling      grepping ten Rust op-variant names against
    the subject does not use        the prelude returned "1 of 10 reachable".
                                    The Ken surface spells them differently and
                                    no rule derives one form from the other.
                                    Re-keyed on the producer table: 3 / 1 / 6.

    a true observation against      "this README row is out of alphabetical
    an invariant the artifact       position" — true, and a defect only if the
    never had                       table was sorted. It was not: 12 pre-existing
                                    out-of-order pairs. The control refuted the
                                    PREMISE, not the observation.

    a pattern counted across a      counting `Self::X =>` arms over a window
    boundary it cannot see          picked by eye gave 47 against 35 classified
                                    — "12 ops with no availability class". The
                                    window spilled past `availability()`'s close
                                    into the next match. The pattern cannot tell
                                    two matches apart; only the function can.

    an attribute position NOT       a `#[cfg(test)]` three thousand lines above
    converted into a claim          a function decorates the ITEM it precedes,
                                    not that function. Reasoned, then filed as a
                                    measurement the node owes rather than as a
                                    finding.

## State it so the fourth instance is included

> **A pattern match is evidence about what encloses it, and you do not have the
> finding until you have resolved that item.**

The weaker phrasing — *"check what contains your hit before you report it"* —
reads as advice about **reporting**, and under it the fourth instance is not an
instance at all. That is the one worth imitating: the same move run **before
the claim existed** rather than after.

**The four do not look alike from outside, and that is the trap in learning
from them.** A recovery produces a corrected number and a visible exchange; a
prevention produces **silence**. Whoever tallies the instances will find three
and miss the cheapest one.

## The artifact that arrives looking like a defect gets LESS scrutiny

`12 unclassified ops` is a **finding-shaped number.** It is not absurd, it does
not read as an artifact, and it would have been reported with a real file and
line range attached. Compare a number that arrives looking like nothing — a
round zero, an empty grep — which everyone instinctively re-runs.

⇒ **Suspicion should scale with how CONCLUSIVE a number looks, not with how odd
it looks.** The ones that cost you are the ones that arrive already shaped like
the answer you were looking for.

## A container that LACKS the property has two readings, not one

Running the control is only half the move. When it comes back *"the container
does not have that property"*:

> **"No invariant exists" refutes the defect claim. It is NOT evidence that
> anyone decided against one.**

Absence of a rule is consistent with *examined and rejected* and with *never
examined*, and those license different next steps. The discriminator is whether
the absence is **maintained**:

    README rows      12 pairs already out of order -> sortedness is actively
                     NOT maintained. A decision by practice. The finding is
                     genuinely REFUTED and there is nothing to escalate.

    cross-scope      23 links thinly spread over 5 target scopes, while the
    links            corpus rule says reading your own scopes is COMPLETE
                     -> more likely a convention nobody has examined. The
                     finding is not refuted, it is UNASKED -- and it belongs
                     to whoever owns the artifact, not to you.

The second case is the trap, because it feels identical to the first from
inside: you ran a control, it came back negative, and you moved on. **Say which
of the two you established.** Then hand the unasked one to its owner as a
measurement rather than converting it into a finding or silently dropping it.

## The enclosing item of a RULE is its precondition

The same failure runs on **rules**, not only on matches, and it is harder to see
because a rule carries no line number to resolve. A `do-not-respin` convention
is scoped to *a candidate that has been handed to a publisher*; cited by analogy
at an unrouted branch, its precondition does not hold and it forbids nothing.
The citation was made one message after helping write this lesson.

⇒ **Before applying a rule you did not write, resolve what it is scoped to** —
the same act as resolving the function that encloses a line. And note that
**deferral is itself an act with its own failure mode**: the deferred good
answer keeps living in the thread, which is what makes "fold it later" feel
free when it is not.

## What to actually do

- **Resolve the enclosing item before the hit becomes a claim** — the enclosing
  function for a line, the decorated item for an attribute, the declaration for
  a match arm, the containing artifact's invariant for a row.
- **Never pick a line window by eye for a count.** Find the enclosing item's
  bounds first; `sed -n 'A,Bp'` with an eyeballed `B` is how a match spills into
  its neighbour, and the spill is invisible in the output.
- **Before reporting a local observation as a defect, check that the container
  has the property you are measuring against.** "Out of order" presumes sorted;
  "unclassified" presumes one classifier.
- **When you do catch one, say whether it was a recovery or a prevention.** The
  preventions are the ones the next reader needs and the ones no artifact
  records.
- **After a negative control, say whether the absence is maintained or merely
  unexamined**, and route the unexamined one to the artifact's owner.

Sibling of
[[repairing-a-census-completeness-does-not-re-aim-its-subject]] — that one is
about a census's SUBJECT being the wrong question; this one is about a hit's
SCOPE being unresolved. They fired within an hour of each other on the same
work, and the second was caught using the first. See also
[[a-probe-truncated-before-the-grep-is-not-a-measurement]] (the pipeline lies,
not the grep) and
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].

**How this file came to exist is its own lesson.** Two seats had derived this
rule independently, from different incidents, and each had carried it in a
private store for months without promoting it. The trigger that would have
caught that is in
[[citing-a-private-lesson-to-another-seat-is-the-promotion-signal]].
