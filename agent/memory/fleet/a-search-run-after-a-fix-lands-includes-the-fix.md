---
scope: fleet
audience: (see scope README)
source: private memory `a-search-run-after-a-fix-lands-includes-the-fix` (R4
  triage, 2026-09-26)
---

# A search run after a fix lands includes the fix

A corpus is not a fixed object. Between forming a question about it and
measuring it, a repair can land inside the exact population being searched,
and the search has no way to mark which hits predate the question. This
matters because the failure is asymmetric: it reports the defect as
**absent** or already-handled, which is the direction that produces no
output to notice — the same shape as any measurement whose baseline silently
absorbs the very thing it was meant to detect.

## The evidence

A seat claimed a criterion had never existed in a WP frame's history. A count
over that frame's history came back 3 occurrences — one keystroke from
posting that the claim was wrong. All three hits were from the seat's own fix,
routed minutes earlier, sitting inside the population being searched. The
original claim (zero, before that commit) was correct. A per-revision
breakdown, not a total, is what separated them:

    for c in $(git log --format=%H --all -- <path>); do
        git show "$c:<path>" | grep -c '<term>'   # per revision, not a total
    done

A second, distinct form of the same trap: a correction that fixes "X is
refused" to "X is **NOT** refused" still matches a search for "is refused."
Re-running the detector after the fix counts every corrected instance as a
fresh violation — the repair contains the needle. The fix here is not a
smarter regex; it is closing the sweep by enumerate-and-classify and saying so
explicitly ("N blocks carry the phrase, all N deny it, none assert it") rather
than trusting a bare matcher count, because that warrant does not scale but
also does not lie the way a raw count can.

A related but separate error is scoping a count to the wrong population in
the other direction: "every commit that ever touched this file" is a
complete, correct census of **one file**, misread as answering "does this
exist anywhere in the corpus?" Both errors are about which population a
count ranges over — one included a repair that shouldn't count, the other
excluded a sibling document that should have.

## How to apply

- Before publishing any raw count over a live corpus, ask what has landed in
  it since the question was formed. If a fix for the very thing being counted
  landed in that window, it is inside the population.
- Break a total down before publishing it — per revision, per file, per
  source. A bare total cannot report that some of its members are the repair
  itself; a breakdown shows it immediately.
- This fires hardest when checking someone else's negative claim, because the
  act of raising the question is often what caused the fix to land. The more
  responsive the other seat, the more likely their fix is already inside the
  search window.
- When a search is over text that a correction could itself contain (a
  negation, a fixed phrasing), prefer enumerate-and-classify over a bare
  matcher count, and say explicitly that it was done by hand.
- State which population a count ranges over every time — a count correct
  about one file is not automatically a claim about the whole corpus, and a
  count over "everything" is not automatically free of the fix that produced
  the question.
