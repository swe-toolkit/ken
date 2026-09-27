---
scope: fleet
audience: (see scope README) — anyone about to run `git checkout <ref> --
  .` or `git checkout <branch> -- <path>` for ANY reason, especially inside
  a "just reading" or "just routing" command, and anyone reverting a
  mutation probe with it or with `git reset --hard`
source: three independent incidents, 2026-07-22 (x2) and 2026-07-25; merged
  2026-09-27 with `a-mutation-revert-restores-the-last-commit-not-the-pre-mutation-file`
  (private memory `commit-real-fix-before-any-mutation-proof-reset`,
  `committing-before-a-mutation-campaign-does-not-protect-work-added-during-it`,
  R4 triage 2026-09-26)
---

# `git checkout <ref> -- .` silently reverts every uncommitted tracked edit

`git checkout <ref> -- .` inside a survey/grep command silently reverts
every uncommitted tracked edit in the worktree; untracked files survive, so
the loss is partial and the commit that follows looks successful.

## First instance: buried in an unrelated survey command

2026-07-22. A tracked status file had been edited (`status: merged`) but
not yet committed. Two tool calls later, an unrelated WP-token survey
opened with `git checkout -q origin/main -- . 2>/dev/null` — intending only
to sync the worktree for grepping. That restored **every tracked file**
from the ref, silently discarding the pending edit. The `2>/dev/null`
guaranteed no complaint.

**Why it survived review:** the very next commit *succeeded*, pre-commit
hooks passed, a schema check reported all issue files valid, and the
publisher merged. A new file in the same commit was **untracked**, so it
was unaffected and landed — making the commit look complete. The defect
surfaced only when the file's `status:` was read back off `origin/main`
after the merge and found still `ready`.

**How to apply.**

1. **Never put `git checkout <ref> -- .` in a command whose purpose is
   read-only.** To read another ref's content use `git show <ref>:<path>`,
   `git grep <ref> -- <paths>`, or `git cat-file` — none of which touch the
   working tree. There is no reason to mutate the worktree to grep a ref.
2. **`-- .` is worktree-wide**, not scoped to whatever you were thinking
   about. Combined with `-q` and `2>/dev/null` it is a silent, total
   revert of uncommitted tracked work.
3. **Untracked files survive, so the damage is *partial*** — which is
   worse than total, because the surviving new file makes the commit read
   as successful. ⇒ **After any command that touches the worktree,
   re-verify the specific edit you were carrying**, not just that the
   commit succeeded.
4. Sibling risk: any worktree-wide mutation invoked for a narrow purpose
   (e.g. `git stash` in a shared multi-worktree clone) carries the same
   shape.

**The general shape:** a *success signal* says a thing ran, never that
it did what you meant — and here the success signal came from a commit
that was genuinely fine, just missing one hunk. Verifying on `main` after
publish is the only reason this was caught at all.

## Second instance, same root: `checkout <branch> -- <dir>` is OVERWRITE, never MERGE

Same day. Taking custody of two additive branches, both touching the same
directory's `README.md` index, applied sequentially:

```sh
git checkout branch-1 -- agent/memory/roles/adversary/ && git commit   # 2 index rows
git checkout branch-2 -- agent/memory/roles/adversary/ && git commit   # 3 index rows
```

**Branch 2's README replaced branch 1's wholesale.** Result: 10 base + 3 =
13 rows where 15 were due. All five *files* landed; two had **no index
entry** — orphaned, on disk, invisible to the only thing anyone reads.

**Nothing failed.** No conflict, no warning, both commits clean.

**First attributed to git's disjoint-hunks silent union. That was
WRONG.** Measured: these two branches **conflict loudly** — both append at
the end of the same table, so the hunks overlap.

```
git merge-tree --write-tree bd000509 5ae3ee74
  -> stages 1/2/3 on README.md = genuine CONFLICT
rows: base 10 | branch1 12 (+2) | branch2 13 (+3)   -- got 13 = branch2 EXACTLY
```

⇒ **Git never had the chance to union anything.** A 3-way merge cannot
produce "branch2's file verbatim". **`git checkout <ref> -- <path>`
bypasses merging entirely** — it is a checkout, not a merge, and takes the
blob wholesale.

**So the silence was in the choice of command, not in git.** A command
with no merge semantics was picked on files where a merge was exactly what
was needed. The loud failure git offered was never triggered because git
was never asked to merge.

**How to apply.**

1. **Applying N branches that share a file is not a merge.** `checkout --
   <path>` overwrites; use `git cherry-pick`/`git merge` for conflict
   detection, or **hand-build the union and verify it**.
2. **Count the expected result before looking at the actual one.** `base +
   Δ₁ + Δ₂` is one arithmetic line and it is the entire detector. This was
   caught only because 15 was predicted first.
3. **For any index/manifest, check BOTH directions** — every row has a
   file, every file has a row. A one-directional check passes happily on
   an orphan.

⇒ Both instances are one rule: **`git checkout <ref> -- <path>` is a
destructive write scoped to the ref's content, not to your intent.**

## THIRD instance, 2026-07-25 — from a PLAYBOOK IDIOM, so the rule didn't save it

A frame amendment (~35 lines) had just been hand-authored as an
**uncommitted** edit on a working branch. Routing it to a corpus branch, a
standard playbook sequence was run verbatim:

```sh
git branch -f wp/steward-b2o-ac12 origin/main && git switch wp/steward-b2o-ac12
git checkout steward/work -- docs/program/wp/RT-FNSPLIT-B2O-body-ownership.md   # <-- destroyed it
```

`git status` came back **empty** and the amendment was **gone** — the
checkout pulled the file's *committed* state from `steward/work`
(identical to `origin/main`) over the working-tree edit.

**Why this one is different from instances 1 and 2: the command was
CORRECT BOILERPLATE, copied from a standing playbook step.** The playbook
step says exactly this, and it is safe **there** — for the tracker, which
is *always committed* before that routing step. A path-scoped idiom was
generalized from a *committed* file to an *uncommitted* one, and the
precondition that made it safe was **never written down next to it.**

⇒ **The idiom's real precondition: `git checkout <branch> -- <path>`
transports a file only if that file is COMMITTED on `<branch>`. Applied to
an uncommitted edit it is a revert.** Same command, opposite effect,
decided by a fact about the source branch that the command does not
mention.

**How to apply.**

1. **Commit before you route.** If you hand-authored an edit this turn,
   `git add && git commit` it on the working branch *before* any
   `switch`/`checkout`.
2. **A `switch` alone would have been safe.** Git carries an
   uncommitted modification across a branch switch when it doesn't
   conflict — the file was already on the new branch correctly. **The
   `checkout -- <path>` was pure downside**: the transport it was meant to
   provide had already happened.
3. **Cheap recovery, worth knowing:** if the destroyed text is still in
   this session's earlier tool-call history, re-applying it costs one
   call. **Check the transcript before re-deriving hand-authored prose.**
4. **The detector that caught it was an EMPTY `git status`** where a
   modification was expected. ⇒ **After any worktree-touching command,
   assert the edit you were carrying is still there** — not that the
   command succeeded. Instance 1's lesson, third confirmation.

## The generalizable part — why the remedy is right for a different reason

**Danger of the wrong story:** "git silently unions disjoint hunks" ⇒ a
reader concludes **"a loud conflict means I'm safe."** Instance 3 is the
counterexample — **the conflict-avoidance path was loud in instance 2, and
in instance 3 there was no conflict at all, and content was still lost —
because the loss happened in *how the merge was avoided*, not in the
merge.**

**The real class is: a merge avoided or resolved by taking one side
wholesale.**

**A row-count + bidirectional-orphan check is the right gate precisely
because it is MECHANISM-INDEPENDENT.** It is a *post-condition on the
merged artifact*, so it catches union errors, wholesale-take errors, and
bad conflict resolutions alike **without needing to know which occurred.**
⇒ **Prefer a post-condition on the result over a guard keyed to a
mechanism story** — the story can be wrong while the post-condition still
holds.

## As a mutation revert: it restores the last commit, not the prior file

Merged from
`a-mutation-revert-restores-the-last-commit-not-the-pre-mutation-file`.
The same underlying fact, reached from a mutation loop rather than from a
"just reading" command: the command restores to a commit, not to what was
there a moment ago.

A mutation-proof loop is *plant violation, run the gate, confirm red, undo,
confirm green*. The undo step matters more than it looks: `git reset --hard`
and `git checkout -- <file>` both restore to **the last commit**, not to
"whatever this file held a moment ago." If real, uncommitted work is sitting
in that file when the mutation is planted, the undo silently deletes it along
with the mutation — with no error, because deleting uncommitted work is
exactly what those commands are for.

**What happened.** The pattern recurred five times against one memory file
already warning about it. `git reset --hard` ate a new test, then a script
dedup fix, then — in the same session, on the same hotfix — a whole new
ancestry check plus its regression test, freshly written and uncommitted,
wiped by a `git commit --allow-empty` cleanup. The memory was loaded in
context each time and did not fire; restating "commit first" does not reliably
survive contact with a live mutation loop.

Switching to `git checkout -- <file>` did not fix it, because that command
reads as *scoped* — it names one file, not the whole tree — and the scoping is
exactly what makes it feel safe. The scope is still the whole file, not the
mutation, so any uncommitted work sharing that file dies with it. One
occurrence lost ~200 lines of unrelated in-flight wiring this way; another
lost a **pin written to catch the very mutation being tested**, because the
pin and the mutation shared a file and both a green baseline and the
post-revert state read as "2 red" — the same number for opposite reasons, so
nothing looked wrong.

**The failure is worse inside a loop.** A campaign iterating over several
mutation sites with `git checkout -- <file>` as its per-iteration cleanup
destroys any uncommitted repair on the *first* iteration, and every iteration
after that measures an unpatched tree. The campaign's own report does not
expose this: a real, correct count on row one followed by blank rows reads as
"the later mutations failed to apply" — an ordinary and unalarming shape for a
mutation harness, not a data-loss alarm.

**How to apply.**

- **Never revert a mutation probe with `git reset --hard` or
  `git checkout -- <file>` / `git checkout <ref> -- <file>`.** Both restore to
  a commit, and "to a commit" is the defect whenever the file holds anything
  the commit does not. Use a plain file backup instead:

  ```
  cp <file> /tmp/<file>.bak      # before touching anything
  <plant the mutation, run the gate, confirm red>
  cp /tmp/<file>.bak <file>      # restore — not git reset, not git checkout
  <run the gate again, confirm green>
  diff /tmp/<file>.bak <file>    # confirm byte-identical
  ```

  This never touches git history, so there is no commit boundary to get wrong
  and nothing else in the tree can be collaterally destroyed.
- **In a loop, commit the moment a mutation produces real work** — a new pin,
  a fix, anything meant to survive — before running the next iteration. The
  loop structurally re-runs the destructive step, so one forgotten commit
  corrupts every iteration after the first, invisibly.
- **Read "nothing to commit, working tree clean" after a session of editing as
  a data-loss alarm**, never as "already committed." Grep for a symbol just
  written before believing the tree is clean.
- **Read a mixed result — one populated row followed by blanks — as the
  signature of a cleanup step that ate its own subject**, not as later
  mutations failing to apply. Check `git status --porcelain` on the mutated
  files before concluding anything about the anchors.
- When a git-history-level mutation genuinely cannot be avoided (proving a
  check rejects a commit that does not exist yet, or a topology only
  expressible as a real commit), do it in an isolated scratch clone or
  worktree, never in the tree carrying real uncommitted work.
