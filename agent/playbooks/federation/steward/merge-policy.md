---
name: steward-merge-policy
description: >-
  Decide when accepted or safely cut work belongs on main. Publishing mechanics
  are in merge-procedure.md.
scope: federation
---

# Merge policy

This file decides when a cut should land. `merge-procedure.md` decides how.

## Default: land the green seam

Merge finished accepted work as soon as its exact-SHA gates are complete. A
partial WP may land when the cut is useful, internally coherent, and green. WP
closure and merge are different events.

A team's accepted base belongs on `main` before the team builds further work on
it. Never extend an approved-but-unlanded branch into the next unit.

## Conditions that do not justify a hold

Do not hold a cut because:

- the parent WP is incomplete;
- later siblings remain;
- the branch is serving as an evidence archive;
- a rebase would be inconvenient;
- an inactive path on `main` looks unfinished.

Ken has no compatibility obligation during initial development. The permanent
artifact and a sane git history matter more than preserving an intermediate
branch shape.

## The only holding conditions

Hold a cut when one of these is true:

1. **Semantic atomicity.** One half alone regresses behavior, admits an unsound
   state, or has no reaching witness.
2. **Required exact-SHA approval is missing.** The review belongs to another
   object or has not resolved.
3. **The cut is not green.** CI has not established the required result, or the
   base is red in a way that prevents attribution.
4. **The merge result is unresolved.** Current `main` changed a touched file and
   the semantic union has not been inspected.

Everything else is a reason to close the WP later, not a reason to keep accepted
work off `main`.

## Finding the cut

Prefer an existing straight-ancestor checkpoint. It preserves reviewed SHAs and
avoids re-anchoring the unfinished suffix.

Before routing, distinguish the candidate diff from a tree comparison against
current `main`:

```sh
BASE=$(git merge-base <SHA> origin/main)
git diff --name-only "$BASE" <SHA>
comm -12 \
  <(git diff --name-only "$BASE" <SHA> | sort) \
  <(git diff --name-only "$BASE" origin/main | sort)
```

An empty intersection means main's movement is immaterial. A non-empty
intersection requires inspection, not an automatic rebase.

Full-workspace validation runs in CI, never locally. Use targeted local checks
only (`COORDINATION.md §12`).

## Decision boundary

The Architect judges work quality. The Steward decides merge timing from the
rules above. Do not ask a reviewer to decide whether an otherwise-cleared cut
may sit on `main`; ask whether the cut is correct.