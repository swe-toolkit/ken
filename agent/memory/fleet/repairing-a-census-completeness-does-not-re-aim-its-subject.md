---
name: repairing-a-census-completeness-does-not-re-aim-its-subject
description: A census can be COMPLETE, CORRECT and green while its SUBJECT is a different question than the reader takes it for — and the repair that makes the list unable to be short does not touch that. Five true green availability sites were read as reachability; every site found by the corrected membership rule would also have been green. Two failures, one fix. A control on a different subject lives ALONGSIDE the census, never inside it.
metadata:
  type: feedback
---

# Repairing a census's completeness does not re-aim its subject

**Measured 2026-09-16 on `RT-D5B-HOST-FILE-ACQUISITION-SURFACE`, and named by
the Architect against its own earlier finding.**

Earlier the same day, `AC-AVAIL` was repaired from an **enumeration** of
availability sites to a **membership rule**, because the enumeration could
silently go short. That repair was right. **Run the corrected AC against the
slice and it still passes.**

    the LIST could not report being short        -> fixed by the predicate
    the LIST's SUBJECT was not REACHABILITY      -> untouched by that fix

Every one of the five sites is a **native**-availability site. The slice made
the operation reachable from Ken source **through the interpreter**, which
touches none of them — and a sixth, seventh or tenth site found by the
membership rule would also be native and also stay green.

    an availability census asks   "is this op marked and gated as unavailable?"
    reachability asks             "can a Ken program get here?"

Those two questions **coincided** for exactly as long as a prelude stub refused
in Ken source, above both executors. The slice deleted that stub — correctly,
and under a spec mandate — and that is the moment they stopped coinciding.
**Five green was a true measurement of the first question, presented as an
answer to the second.** Nobody reading the gate report could have caught it,
including the Architect, twice.

## The rules

- **Write, as a sentence, the question your census actually answers — and the
  question its reader will take it for.** If those are two different sentences,
  the report must say so. This is MEASURED / CLAIMED / THE GAP applied to a
  **green**; the discipline is usually reached for only on a red.
- **Repairing an instrument does not re-aim it.** *"We fixed the census, so we
  are covered"* is the wrong conclusion and the expensive one. A control whose
  subject is different has to exist **alongside** the census, and be framed as
  covering that different subject **by name** — fold it into the census and the
  next reader drops one of the two.
- **The tell is a dimension every member shares.** All five sites were native.
  A census whose members are uniform along the axis your change moves cannot
  see that change, however complete it is under its own rule.
- **A green from a check that usually passes for reasons unrelated to its
  subject is not evidence — and is indistinguishable from evidence in a gate
  report.** Same arc: a `122 passed / 0 failed` carried nothing either way
  about the one test in it that was unsound.

## Second instance: a shared shape checked for the closure, inherited by the repair

Measured 2026-08-16. A closure rested on a measurement taken through a
test-support entry point, with the four production sites re-checked by hand
and reported as sharing the refusal shape; the merge flagged that it "stands on
my recheck, not on the run as delivered". An independent read held: all four
are `let compiled = <compile>(…)?;`, and the `?` propagates before any artifact
or report literal, so a refusal cannot reach the lane. Reading them
independently turned one recheck into two, worth three minutes when a complete
negative result closes a node.

But the summary flattened a split: two sites construct a native artifact, two a
run report. **Immaterial to the negative result, decisive for the successor**
that must bind an artifact on refusal: the two report sites fall outside it or
need different treatment. *"They share the shape"* is always relative to a
question. Ask which question the shape was checked against, then whether the
next question is the same one; a sizing that inherits a summary written for the
closure will under- or over-count the repair.

Same arc, a smaller habit: an acceptance criterion required a failure
**message** to be verified by running the mutation and reading the printed
text. Do that yourself. The concern behind it was behavioural (an author
reading a red as obsolete and deleting it), so the remedy is text, and text is
only verified by being printed; an assertion can be correct while its message
is useless.

## The pattern that produced it is worth keeping

The hazard came from a good, twice-approved pattern: **land a surface
unflipped, flip it later.** It makes a surface reachable from Ken while
`availability()` still says no.

> *A pattern that creates a hazard is not a bad pattern; it is a pattern that
> has not been given its matching control yet.*

The answer is the missing control, not a retreat from the pattern — and the
control belongs to the flip, framed as covering **reachability**, explicitly
distinct from the availability census's subject.

Distinct from
[[a-claim-accurate-about-something-narrower-than-its-reader-infers]] — that is a
claim whose subject is **narrower** than the reader's; here the subject is
**orthogonal**, and the second half (the completeness repair leaves it
untouched) is what gets filed under the wrong lesson. Sibling of
[[a-list-of-same-shape-guards-reads-as-complete-because-the-shape-is-what-made-them-findable]]
(same "the members share a shape" tell, but there the list was genuinely short;
here it was not) and of
[[a-differential-over-an-aggregate-is-an-existential-not-a-universal]].

**Filed the same hour, and caught using this one:**
[[a-pattern-match-is-evidence-about-what-encloses-it]] — a distinct sentence
rather than an elaboration of this file. Here the census's SUBJECT is the wrong
question; there a hit's SCOPE is unresolved. This lesson's detector ("what do
all my members have in common?") is what caught that one's first instance.

**When the repair is a test enforcing one instance of a general criterion,**
record it as a bound, not a defect, and say which members remain and whether
they share the shape, so the bound is on the record: item 5 of
[[a-filter-or-list-keyed-on-todays-members-expires-when-the-kind-widens]].
