---
name: during-an-infrastructure-incident-absence-stops-being-evidence
description: My findings routinely rest on absence — zero occurrences, a SHA not an ancestor, a ref that has not advanced — and a degraded fetch or half-completed push produces every one of those without the underlying fact being true
metadata:
  type: feedback
---

# During an infrastructure incident, absence stops being evidence

**Operator, 2026-08-17: GitHub PRs degraded, ongoing incident.**

**My local path is unaffected and I verified it rather than assuming**: `git
fetch` exit 0, `origin/main` fresh, worktree clean. **I never touch GitHub — I
read local objects and post via convo — so the incident does not block the
seat.**

⇒ ***But a large share of what I report is an ABSENCE***: *zero occurrences of a
symbol*, *NOT in `main`, so pre-merge*, *the object resolves but is not an
ancestor*, *this refusal is expected at no real-source layer*. **A degraded fetch
or a half-completed push produces every one of those without the underlying fact
being true.**

⇒ **So during a declared incident, an absence needs its instrument checked
first:** fetch **exit status**, not just output; ref freshness; and the cited SHA
present **and** ancestral where claimed. **Report the window alongside any
finding whose evidence is a negative taken inside it.**

**This is a different cause from the usual zero-hit trap.** There the count is
right and the predicate is wrong; here **the predicate is right and the tree I
measured is not the tree.** ⇒ **Naming which one applies matters, because the
repairs are opposite: widen the predicate, versus re-take the measurement
later.**

## AN UNNOTIFIED `main` ADVANCE IS NOT A CUE

`origin/main` had moved to a commit I had no notification for. ⇒ **That is the
expected shape of a publisher-side incident, and it is still not a reason to go
looking** — the seat is event-driven and never polls. **Notification thinning is
someone else's failure mode to manage; treating it as my backlog converts their
incident into my policy violation.**

**And the incident itself is not mine to report.** Publisher and CI mechanics
sit outside the report-only edge, and the seat that runs the publisher already
knows. **Silence here is the correct output.**

Related: [[a-tools-silence-is-scoped-to-the-question-it-asks]] (the usual
absence trap, where the tree is right and the question is wrong).
