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

## A fourth state: a passed check frozen at `in_progress`, where a rerun is right

Measured 2026-08-20 (PR #2664): `mergeable: MERGEABLE` but
`mergeStateStatus: BLOCKED`, because one required check-run had
`conclusion: success` and `completed_at` set on a `completed` run while its own
`status` stayed frozen at `in_progress`. Branch protection reads the required
context by `status`, so a check that passed still blocks the merge, and the
publisher waits on it forever. The tell is the pair `conclusion=success` plus
`status=in_progress` on a completed run.

Here the polarity of the repair is the opposite of the orphan case:

1. **`gh run rerun <runid>` on the whole run** (not `--job`, which refuses a
   job GitHub thinks is still running). It needs `actions: write` on the token.
   The PR stays open, so a waiting publisher simply sees the fresh green check
   on its next poll.
2. Without that scope, **close and reopen the PR** (`ci.yml` triggers on
   `pull_request` default types, which include `reopened`). Stop the waiting
   publisher first and relaunch it after.

The head SHA does not change either way, so a Decision bound to that SHA stays
valid; confirm `headRefOid` is unchanged afterwards.

## Why this is worth a lesson rather than a note

The states — slow CI, dead run with pending jobs, orphaned queued check, and a
passed check frozen at `in_progress` — are **indistinguishable from the
publisher's poll line**, which is the only surface most seats see. So the
instinct to "just retry the check" is reached from a view that cannot tell which
state it is in, and a retry is irreversible in one state and correct in
another.

A failed publisher merge also leaves an **orphan PR that every later check
greens over**, so the wreckage does not announce itself on the next run either.

**Before touching a stuck check: establish which state you are in.** If you
cannot, waiting is free and rerunning is not.
