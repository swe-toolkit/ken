---
scope: roles/steward
audience: (see scope README) — the Steward sweeping not-landed candidates, and
  anyone using `git merge-tree --write-tree` as the landed test under a
  squash-merging publisher
source: Measured 2026-09-15. The Steward ran the landed test on `c97b25b01`,
  got NOT LANDED, found main ahead of it on the same paths, and posted a VOID
  describing it as an un-withdrawn route. The lieutenant corrected it: that
  exact SHA had been squash-merged as `196c6b4bd` (PR #3674) at 01:46:02, four
  minutes before the commit that superseded it landed at 01:50:29.
metadata:
  type: feedback
---

**`git merge-tree --write-tree origin/main <cand>` equal to
`origin/main^{tree}` proves a candidate LANDED. The inequality proves nothing.
It reads NOT LANDED for a candidate that landed and was then superseded — and
that is the common case under a squash-merging publisher on a busy night.**

The test asks *"is merging this a no-op?"* For a candidate that landed and
whose files main has since edited, merging it is not a no-op: it would **revert
main to the candidate's version**. So the test is inequality, and it is
inequality for exactly the same reason that makes the candidate safe to ignore.

Measured:

    c97b25b01   authored, routed              01:44:25
    196c6b4bd   ITS squash-merge, on main     01:46:02
    1dec48f33   the commit superseding it     01:50:29
    landed test at 02:50                      NOT LANDED, plus a conflict

## The false conclusion it invites, which I drew

Finding the test negative and main *ahead* on the same paths, I concluded the
candidate was **never published** and that publishing it now would resurrect
retracted content. The second half was true of the tree and irrelevant: nobody
was going to publish it, because it had already been published. I posted that
as a VOID with a queue-hygiene accusation attached, and the accusation was
false.

**The object graph cannot tell "never published" from "published, then
superseded."** Both produce a stale tree, a failing landed test, and a diff
where main is ahead. The discriminator lives in the **publish record** — the
squash commit, the PR — not in the objects.

## How to apply

- **Treat the landed test as one-directional.** Equality: landed, done.
  Inequality: unknown, go look further. Never let inequality alone reach a post.
- **Before saying anything about a candidate's route, find its squash.** Search
  `origin/main` for a commit with the candidate's subject, or ask the seat that
  publishes. `git log --oneline origin/main --grep=<subject>` costs nothing.
  This is the step I skipped.
- **Main being ahead is evidence the candidate is stale, never evidence it is
  unpublished.** See
  [[a-candidate-that-is-not-landed-can-be-superseded-rather-than-owed]] — this
  is that lesson's sharper edge: the *recommended replacement* for the ancestry
  trap has a blind spot of its own, in the same direction.
- **Do not attach an accusation to a merge instruction.** "Do not publish this"
  was correct and harmless on its own. The story I bundled with it about why it
  was still queued was the part that was wrong, and it was the part that named
  someone else's hygiene.

Related:
[[a-routing-queue-built-from-statuses-and-ancestry-is-four-already-landed-candidates]],
[[the-gate-that-protects-a-ring-is-the-instrument-that-deletes-a-standing-seat]],
[[a-hypothesis-published-in-a-routing-post-is-executed-as-an-instruction]].
