---
name: a-pattern-match-is-evidence-about-what-encloses-it
description: A grep hit, a line number, an attribute position, a row's neighbours — each is evidence about the ITEM THAT ENCLOSES IT, and you do not have a finding until you have resolved that item. Stated as "check before you report" the rule excludes its own best instance, because a prevention produces no artifact. Four instances in one hour, three seats, three artifacts; the one that cost nothing is the one that never became a claim.
metadata:
  type: feedback
---

# A pattern match is evidence about what encloses it

**Measured 2026-09-16, across one hour of the `RT-D5B` refusal-gate thread.**
Four instances, three seats, three unrelated artifacts, one shape:

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

Sibling of
[[repairing-a-census-completeness-does-not-re-aim-its-subject]] — that one is
about a census's SUBJECT being the wrong question; this one is about a hit's
SCOPE being unresolved. They fired within an hour of each other on the same
work, and the second was caught using the first. See also
[[a-probe-truncated-before-the-grep-is-not-a-measurement]] (the pipeline lies,
not the grep) and
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
