---
scope: roles/steward
audience: (see scope README) — the Steward as merge router, and the lieutenant
  holding a routed queue: any seat that can leave a `ROUTED: <SHA>` standing
source: Measured 2026-09-15. The Steward routed `c97b25b01` (a node filing),
  the node's premise was retracted by its own author about four hours later,
  and the routing was never withdrawn. Caught only during an unrelated sweep of
  not-landed candidates.
metadata:
  type: feedback
---

**A `ROUTED: <SHA>` is a standing instruction to publish a frozen tree. It does
not track the reasoning that produced it, and retracting that reasoning
anywhere else does not retract the route.**

Measured: `RT-EFFECT-CATALOG-GLOBAL-ARTIFACT-COUPLING` was filed `ready` on a
false premise, and the commit filing it was routed to the lieutenant. The
implementer retracted the premise unprompted. The Steward closed the node,
refiled the real defect as a superseding node, and corrected the operator
briefing — **and left the routing standing.** Publishing that SHA would have
reopened the retracted node as `ready` and deleted the node that superseded it,
because main had moved on and the routed tree had not.

## Why nothing else catches it

- **The publish path does not re-read content**, and should not have to. It
  verifies the SHA, the base, and a clean merge. All three stayed true. The
  tree was exactly what was routed; what changed was whether it should exist.
- **The retraction happened in a thread.** Threads do not reach queues.
- **"Not landed" reads as "still owed."** It is the same trap as
  [[a-candidate-that-is-not-landed-can-be-superseded-rather-than-owed]], with
  a sharper edge: this one was not merely superseded but *actively harmful*,
  and the ordinary superseded-vs-owed check (is main ahead?) is what surfaced
  it.

## How to apply

- **Retracting content means withdrawing its route, in the same turn.** Closing
  the node, refiling the successor, and correcting the briefing are three
  actions; the fourth is the one with a seat waiting on it. Treat the route as
  part of the artifact being retracted.
- **When sweeping not-landed candidates, diff against main rather than asking
  whether they landed.** `git diff origin/main <sha> -- <paths>` and read the
  insertions: what a stale candidate *uniquely carries* is the whole question.
  A candidate that only deletes relative to main is superseded; one that
  uniquely re-adds something is a resurrection risk.
- **The router owns this, not the publisher.** Do not push the check down into
  the publish path — that would make every merge re-litigate content, which is
  the opposite of what the M1-M4 / M5-M9 split is for.

Related:
[[i-said-routed-when-i-had-only-posted]],
[[a-precondition-attached-to-a-vote-is-a-claim-about-the-present-re-read-it-in-the-posting-turn]],
[[a-hypothesis-published-in-a-routing-post-is-executed-as-an-instruction]].
