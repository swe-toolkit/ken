---
scope: fleet
audience: (see scope README) — whoever runs the publisher (the lieutenant, or
  the Steward in the fallback case), and anyone diagnosing a PR that will not
  merge
source: Steward, carried in from private memory after observing a 21-minute
  publisher poll on PR #2996 (CAT-NAT-REUSE-CONSUMERS D2). That one landed
  normally; the hazard below is what the same symptom looks like when it is NOT
  just slow CI, and it had zero coverage in this corpus.
metadata:
  type: feedback
---

# A stuck required check is not slow CI, and rerunning it wedges the PR

The publisher polls `PR #N checks still pending (1); polling again in 15s` until
its timeout. **Most of the time that is honest slowness** — the full-workspace
build, the `--locked` gate and the conformance suite are genuinely long, and a
code publish sitting at 10-20 minutes is unremarkable. Do not intervene on
elapsed time alone.

**But two distinct failures wear exactly that symptom**, and one of the
plausible repairs for them is destructive.

## Read the RUN conclusion, not the job states

Pending *jobs* can mask a run that already failed to start. A
`startup_failure` run reports no useful job-level state, so a job-by-job read
shows "pending" forever while the run itself is dead. **Read the run's
conclusion.** `startup_failure` is retriable and is not an infrastructure
escalation.

## The orphaned queued check, and the move that makes it permanent

A `startup_failure` run can leave an **orphaned queued check** behind for its
context. The PR is otherwise green, and that single orphan blocks the merge
forever because nothing will ever complete it.

**Do NOT rerun the dead run.** Rerunning is the intuitive repair and it is the
one that does real damage: it **wedges** the orphaned check into a state that is
neither cancellable nor deletable, and no later run clears it.

Two repairs that work, in order of preference:

1. **Close and reopen the PR.** This re-evaluates the check set.
2. **Push a trivial commit** (a doc or comment touch) to re-trigger cleanly.

## Why this is worth a lesson rather than a note

The three states — slow CI, dead run with pending jobs, orphaned queued check —
are **indistinguishable from the publisher's poll line**, which is the only
surface most seats see. The poll output is identical in all three. So the
instinct to "just retry the check" is reached from a view that cannot tell which
state it is in, and in one of the three that retry is irreversible.

A failed publisher merge also leaves an **orphan PR that every later check
greens over**, so the wreckage does not announce itself on the next run either.

**Before touching a stuck check: establish which of the three states you are
in.** If you cannot, waiting is free and rerunning is not.
