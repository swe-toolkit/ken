---
scope: fleet
audience: (see scope README)
source: private memory `commit-real-fix-before-any-mutation-proof-reset`,
  `committing-before-a-mutation-campaign-does-not-protect-work-added-during-it`
  (R4 triage, 2026-09-26)
---

# A mutation revert restores the last commit, not the pre-mutation file

A mutation-proof loop is *plant violation, run the gate, confirm red, undo,
confirm green*. The undo step matters more than it looks: `git reset --hard`
and `git checkout -- <file>` both restore to **the last commit**, not to
"whatever this file held a moment ago." If real, uncommitted work is sitting
in that file when the mutation is planted, the undo silently deletes it
along with the mutation — with no error, because deleting uncommitted work
is exactly what those commands are for.

## What happened

The pattern recurred five times against one memory file already warning
about it. `git reset --hard` ate a new test, then a script dedup fix, then —
in the same session, on the same hotfix — a whole new ancestry check plus
its regression test, freshly written and uncommitted, wiped by a
`git commit --allow-empty` cleanup. The memory was loaded in context each
time and did not fire; restating "commit first" does not reliably survive
contact with a live mutation loop.

Switching to `git checkout -- <file>` did not fix it, because that command
reads as *scoped* — it names one file, not the whole tree — and the scoping
is exactly what makes it feel safe. The scope is still the whole file, not
the mutation, so any uncommitted work sharing that file dies with it. One
occurrence lost ~200 lines of unrelated in-flight wiring this way; another
lost a **pin written to catch the very mutation being tested**, because the
pin and the mutation shared a file and both a green baseline and the
post-revert state read as "2 red" — the same number for opposite reasons,
so nothing looked wrong.

**The failure is worse inside a loop.** A campaign iterating over several
mutation sites with `git checkout -- <file>` as its per-iteration cleanup
destroys any uncommitted repair on the *first* iteration, and every
iteration after that measures an unpatched tree. The campaign's own report
does not expose this: a real, correct count on row one followed by blank
rows reads as "the later mutations failed to apply" — an ordinary and
unalarming shape for a mutation harness, not a data-loss alarm.

## How to apply

- **Never revert a mutation probe with `git reset --hard` or
  `git checkout -- <file>` / `git checkout <ref> -- <file>`.** Both restore
  to a commit, and "to a commit" is the defect whenever the file holds
  anything the commit does not. Use a plain file backup instead:

  ```
  cp <file> /tmp/<file>.bak      # before touching anything
  <plant the mutation, run the gate, confirm red>
  cp /tmp/<file>.bak <file>      # restore — not git reset, not git checkout
  <run the gate again, confirm green>
  diff /tmp/<file>.bak <file>    # confirm byte-identical
  ```

  This never touches git history, so there is no commit boundary to get
  wrong and nothing else in the tree can be collaterally destroyed.
- **In a loop, commit the moment a mutation produces real work** — a new
  pin, a fix, anything meant to survive — before running the next
  iteration. The loop structurally re-runs the destructive step, so one
  forgotten commit corrupts every iteration after the first, invisibly.
- **Read "nothing to commit, working tree clean" after a session of editing
  as a data-loss alarm**, never as "already committed." Grep for a symbol
  just written before believing the tree is clean.
- **Read a mixed result — one populated row followed by blanks — as the
  signature of a cleanup step that ate its own subject**, not as later
  mutations failing to apply. Check `git status --porcelain` on the mutated
  files before concluding anything about the anchors.
- When a git-history-level mutation genuinely cannot be avoided (proving a
  check rejects a commit that does not exist yet, or a topology only
  expressible as a real commit), do it in an isolated scratch clone or
  worktree, never in the tree carrying real uncommitted work.

Related: [[git-checkout-ref-dot-silently-reverts-uncommitted-edits-worktree-wide]]
covers the same command from a different angle — invoked for an unrelated
"just reading" purpose rather than as a mutation-campaign revert step; both
are the same underlying fact, that the command restores to a commit rather
than to what was there a moment ago.
