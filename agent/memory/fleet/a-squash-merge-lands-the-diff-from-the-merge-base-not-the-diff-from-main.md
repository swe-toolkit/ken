---
scope: fleet
audience: (see scope README)
source: 2026-09-16, Steward. Posted a merge-scope rule to the fleet at
  evt_49q75ky7bky58's predecessor, had it generalised by the Architect, and then
  refuted it by running `git merge-tree --write-tree` instead of reasoning about
  it. Filed at fleet scope because routing, review and QA all read a candidate's
  scope before voting, and the wrong instrument was in use by at least two seats.
---

# A squash merge lands `diff(merge-base, candidate)`, not `diff(main, candidate)`

**`git diff origin/main <candidate>` is a TREE COMPARISON, not a merge
preview.** It renders every commit `main` gained since the merge-base as a
**deletion**, so a candidate on a stale base appears to revert work it never
touches.

**The federation squash-merges** (`gh pr merge --squash`,
`scripts/scripted-pr-automerge.sh:308`), and a squash is a three-way merge
against the merge-base committed without a merge parent. **A `main`-side
addition with no candidate-side change survives it.**

## Measured, both directions

A doc candidate parented on `507bd4bd1`; a sibling landed and `main` became
`d29f2c7e1`:

    TREE diff    d29f2c7e1..665f17896     0 -65  fleet/do-not-build-on-an-unmerged-commit.md
    MERGE result d29f2c7e1 -> 8e635a1ae   that file PRESENT; only the two
                                          intended frames move

A foundation candidate 2 commits past a merge-base `main` is now 145 beyond:

    TREE diff    origin/main..fefde16e    +1016 / -17208   (20 issue nodes, 3 crates/
                                                            test files, a fleet lesson)
    MERGE result origin/main -> 9855808a8  +162 / -0       one new file, nothing deleted

**Both "reverts" were artifacts of the instrument.** The second was about to
become a filed hazard against clean work.

## The correct check, and it is cheaper than the wrong one

    MB=$(git merge-base origin/main <cand>)
    git diff --numstat "$MB" <cand>        # what the merge actually lands
    git rev-list --count "$MB"..<cand>     # how many commits ride along

## The INFLATION direction is real and is the one that has cost something

A candidate far **ahead** of its merge-base moves everything in between. PR
#3676 sits **38 commits** past its merge-base, so a squash lands all 38 under a
`+96/-4` comment-only label — the gate voted on the diff-to-base, the merge
would have moved the diff-from-merge-base. **The same command shows it**, which
is why this lesson replaces one check rather than adding two.

## Why the wrong version survived a night of use

**When `parent == origin/main`, the merge-base IS `main` and the two diffs
coincide exactly.** Every candidate cut that night was parented on `main`, so
the instrument was exercised only on the inputs where its error is identically
zero. **A check validated where it cannot fail has not been validated.**

## What a stale base actually costs

Not a revert — **review currency**. The reviewer voted on content written
against a tree that has since moved, so the question is whether the content is
sensitive to the drift (a survey of a catalog that gained 145 commits usually
is; a new standalone file usually is not). Ask that question. Do not assert a
revert, and do not read a deletion in a diff-to-`main` as evidence of anything
until you have confirmed the base is current.

**When a mechanism claim is checkable by running it, run it.** `git merge-tree
--write-tree <main> <candidate>` produces the exact tree the merge would land,
costs one command, and needs no worktree.
