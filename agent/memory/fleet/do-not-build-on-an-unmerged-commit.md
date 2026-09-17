---
scope: fleet
audience: (see scope README)
source: 2026-09-16, operator direction to the Steward, verbatim — "do not create
  long stacks of unmerged commits. This is an anti pattern. It creates kickoff
  confusion (which you have observed). We've been through this another time."
  The operator had said it before; it did not stick, which is why it is filed
  here rather than in a role scope.
---

# Do not build on an unmerged commit

**A new effort's base commit must be on `main`.** Not on a candidate that is
approved-but-unlanded, not on an open PR's head, not on your own previous
commit that is still in the routing queue.

## What a long unmerged stack actually costs

It is not a tidiness argument. The cost is that **status stops corresponding to
the tree**, and every seat then reads the favourable arm of an ambiguity:

Measured in a single session, all traceable to one 38-commit unmerged stack
(PR #3676) plus a router who twice built on his own unlanded candidate:

- **Four rings parked on candidates that had landed days earlier** — one of them
  six days. Their tracker nodes correctly read `merged`; only the convo status
  lines were stale, and the seats believed the status lines.
- A team leader **waiting on the Steward's routing steps for a node closed six
  days prior.**
- A code candidate whose merge into `main` **would have moved 38 commits**
  under a `+96/-4` comment-only label — the scope a gate voted on and the scope
  a merge would move were different scopes, and nobody had measured it.
- A candidate (`fefde16e`) whose **entire deliverable is absent from `main`**,
  gating a 21-node backfill program behind a file that does not exist.

⇒ **Those are not four coincidences. That is the failure mode.**

## The justification that defeats this rule

**"It supersedes the parent, so it is one merge and not two."**

This is the anti-pattern with a rationale attached, and it is seductive because
the arithmetic is true — one merge *is* fewer than two. It was used twice in one
session by the seat whose job is routing, in the same hours he was cataloguing
other people's fail-open instruments.

⇒ **The parent being contained does not make the parent merged.** Until it
lands, the branch asserts a version of the tree that no one has accepted, and
anyone who reads the ref believes it.

## How to apply

- **Route one commit. Wait for it to land. Start the next from `main`.** If a
  reviewer's correction arrives while the first is still in the queue, that is
  an argument for landing faster, not for stacking.
- **Get the increment to pass CI.** Operator, same message: *"Make what
  adjustments you need to to get the incremental commits to pass CI."* A branch
  that cannot go green is not a base to build on — it is a dam, and everything
  cut from it inherits the block.
- **When you must rescue an unpushed commit, say which kind of push it is in
  the same message.** A rescue push and a routable ref are the same `git push`
  and are not the same act. See [[a-rescue-push-and-a-routable-ref-are-the-same-command]].
- **A status string is not a queue. Build the queue from the tree** — see
  [[every-verification-instrument-passes-on-a-candidate-that-was-never-pushed]]
  for why the reachability and blob checks are both required, and why the
  favourable arm is the one every instrument reports.
