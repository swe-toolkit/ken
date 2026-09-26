---
scope: roles/steward
audience: (see scope README)
source: private memory `filing-a-program-issues-node-requires-regenerating-implementation-progress-in-the-same-pr` (R4 triage, 2026-09-26)
---

# Filing a program-issues node requires regenerating implementation progress in the same PR

`docs/program/IMPLEMENTATION-PROGRESS.md` is a generated artifact, fully
derived from `docs/program/issues/*.md`, and its currency is enforced in CI:
the `Check IMPLEMENTATION-PROGRESS.md is current` job runs `scripts/
gen-progress.sh --check` and is a required check on `main` (confirmed
current, `.github/workflows/ci.yml`). Adding, removing, or renaming an issue
node changes the input to that generator without changing its output, so a
doc-only PR that touches only the new node file leaves the tracker stale.

Filing a node this way — a new issue file added via git plumbing so the
change touched nothing else — landed clean, then went red the moment
`gen-progress.sh --check` ran against `main`: the tracker was missing the
new node's row. Because every full-CI candidate merges onto that same base,
every lane's next candidate hit the identical red, not just the PR that
caused it. A lieutenant caught it, correctly attributed it to the base
commit, reproduced it on current `main`, and held rather than fixing
blindly. The fix was a second doc-only PR: regenerate the tracker against
main's exact tree and land it alone. One candidate's stale doc cost roughly
ten minutes of fleet-wide block before the second PR landed.

**Why the blast radius is fleet-wide and not local:** the stale doc lives on
`main` itself once the first PR merges, so the red is not scoped to the PR
that introduced it — it is scoped to the base every subsequent candidate
builds on. Filing a node is not a self-contained doc edit; it is an edit to
a required generator's input, and the invariant that consumes that input is
enforced downstream of where the edit landed, not at the edit site.

## How to apply

- Whenever a `docs/program/issues/` node is added, removed, or renamed (or
  any other file `gen-progress.sh` reads changes), run `scripts/
  gen-progress.sh` and include the regenerated `IMPLEMENTATION-PROGRESS.md`
  in the *same* commit or PR as the node change.
- Verify with `scripts/gen-progress.sh --check` locally before publishing —
  it is the same check CI runs, and it is cheap.
- Regenerate against `main`'s exact tree, not a stale local worktree: if
  your working tree is behind `main`, use a detached worktree on disk
  (`git worktree add --detach <disk-path> origin/main`; not `/tmp`, which is
  tmpfs) checked out at `origin/main`, run the script there, publish that
  file, then remove the worktree. A doc regenerated against a stale tree
  will not match what CI expects on `main`.
- If a stale tracker does go red on `main`, don't fix it blindly — reproduce
  the check against `main`'s current tree first, confirm the cause, and land
  the regeneration as its own doc-only PR before resuming other work that
  merges onto the same base.
