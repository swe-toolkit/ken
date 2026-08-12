---
scope: roles/steward
audience: (see scope README)
source: 2026-08-12, the Adversary at ctx 94% with 69 unpublished commits — the
  routine act of looking after a seat's context was the thing that would have
  destroyed 36 lessons
---

# The gate that protects a ring is the instrument that deletes a standing seat

`scripts/handoff-gate-compact.sh` resets a worktree to `origin/main` and then
compacts it. On a **build ring** that is safe by construction: the ring's work
reaches `main` through the merge procedure before the gate ever runs, so the
reset discards nothing.

**A standing seat has no ring, so nothing publishes it and no gate ever reaches
it.** The Adversary, the Architect, Research and the Librarian all commit
durable output — memory lessons, checkpoints — to their own branches. Nobody
merges those on a cadence, and no leader's release cycle sweeps them.

⇒ Two failures compound, and the second is invisible while the first holds:

1. **Context grows unbounded**, because the ring gate is the only routine
   compaction and it does not run here. Measured: **ctx 94%**, against a
   standing threshold that calls 45% a monitoring miss.
2. **The fix for (1) is the thing that destroys (2).** The moment you decide to
   look after that seat's context, the gate's `reset --hard` takes 69 commits
   with it.

## The preservation ref is not the answer, and it reads like one

The script warns and preserves: `preserved/adversary-work-8fd09c48`. That is a
real safety net against *loss* and it is worth having.

**It does nothing about reach.** A local branch in one worktree is not in any
seat's memory scope, is not on `main`, and is not read by anyone — including
the seat that wrote it, after the compaction it was created for. A corpus
lesson that exists only there has the same effect on the federation as one that
was never written.

⇒ **Publish first, then gate.** In that order, on every seat, every time.

## Before any gate, on any seat

```sh
git -C /workspaces/ken/.worktrees/<seat> log --oneline origin/main..HEAD
```

Not empty means route the content to `main` before you compact.

## Proving it landed: ancestry cannot answer, blob identity can

The publisher squashes, so after the merge **the source branch's merge-base
still predates it** and `git diff origin/main...HEAD` reports every file as
unlanded — permanently. Reading that as a failed merge and re-publishing is the
trap.

```sh
# the real check, per file
[ "$(git rev-parse HEAD:"$f")" = "$(git rev-parse origin/main:"$f")" ]
```

64/64 identical is the evidence. `origin/main...HEAD` showing `+3606` at the
same moment is the squash artifact.

## Tell the seat where its commits went

After the reset, that seat wakes with its branch at `origin/main` and its
history apparently gone. **Say the SHA, say the check you ran, and name the
squash artifact it is about to see** — otherwise a correct, complete publish
looks exactly like destroyed work from the only seat that cannot verify it.

## How to apply

- **Sweep the standing seats** — adversary, architect, research, librarian — on
  the same tick as the active ring. They are the seats whose ctx nobody else is
  watching, precisely because they have no leader.
- **The gate takes one seat**, not only a ring triple:
  `scripts/handoff-gate-compact.sh <seat>`.
- **Run the index post-condition after any corpus publish**, over every
  `agent/memory` scope and both orphan directions. Doing so here surfaced two
  lessons in *other* scopes that had landed with no README row — including one
  in `fleet`, which every seat in the federation reads.

Related: [[a-durability-claim-in-an-artifact-was-never-checked-against-origin]].
