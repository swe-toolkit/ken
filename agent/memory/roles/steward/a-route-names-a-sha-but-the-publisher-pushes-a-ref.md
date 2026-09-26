---
scope: roles/steward
audience: (see scope README)
source: private memory `a-route-names-a-sha-but-the-publisher-pushes-a-ref` (R4 triage, 2026-09-26)
---

# A route names a SHA but the publisher pushes a ref

A merge Decision, a QA approval, and a routing post all name a commit SHA. A
publish does not operate on a SHA — `scripts/scripted-pr-automerge.sh` takes
a `--target`, resolves it to a branch name (`resolve_branch`), then does
`head_sha="$(git rev-parse "$head_branch")"` and pushes that branch. If the
branch's tip has moved since the Decision was written — most commonly by
`git commit --amend` on a respin — every gate can have verified the right
SHA while the branch that actually gets pushed points somewhere else, and a
flawless paper trail would land the wrong object.

**How `resolve_branch` behaves today matters to which side is exposed.**
Given a branch name, it returns that name unchanged — no pin, so a moved tip
ships silently. Given a raw SHA, it looks for exactly one local `wp/` branch
at that object, then exactly one matching remote branch, and only creates a
pinning synthetic branch (`wp/scripted-merge-<short>`) when neither match is
unique. So the synthetic, SHA-pinned path is reached only on the *unusual*
input (a bare SHA with an ambiguous or absent branch match); the *ordinary*
case — routing a WP by its one live branch — resolves to a name and does not
pin. A guard that is strict on the edge case and loose on the common one
reads as robust to anyone who tests it deliberately, because deliberate
tests reach for edges. Verify which path a given `--target` will take with:

    git for-each-ref refs/heads/wp \
      --format='%(objectname) %(refname:short)' \
      | awk -v sha=<routed-sha> '$1 == sha { print $2 }'

**The fix that does not depend on the publisher's internals:** before
routing, run `git rev-parse <branch> == <routed SHA>` and make that the
first line of the routing post. Re-run it as the *last* thing before the
command that actually moves the ref, not as the first thing in the
operation — the check is perishable, and a multi-step respin spends minutes
between the check and the push. Neither a reviewer's "please re-cut" nor a
leader's HOLD posted to the router authorizes moving a routed ref; only the
router's own post does, and both of the former can feel pre-authorizing
because they point the same direction you already wanted to go.

## A correct, complete instrument can still answer the wrong question

The same node also carried a path-intersection contention check: pairwise
path overlap across live candidates came back empty, published as
"contention-free." That instrument answers "will these two conflict in
git," which is a narrower question than "can these land in any order." A
candidate that *quotes* another candidate's mutable source text (e.g. an
evidence table of `#[ignore]` reasons, or an "approved and not yet landed"
characterization of a sibling's status) is coupled to whatever edits that
text even with zero shared files, and no path-based check can see it. State
what the check ranges over beside its result — "pairwise path intersection
is empty" invites the next reader to check the other axis; "contention-free"
retires the search.

## Amend and re-cut are indistinguishable by ancestry

`git commit --amend` produces a new object with the *same parent* as the
one it replaces, so `--is-ancestor <old> <new>` returns NO for an amend
exactly as it would for a re-cut. An approval or a routing artifact that
names the old SHA is dead by construction after either kind of rewrite —
what survives a rewrite is an approval bound to a *property* re-checkable
against the new object ("any successor whose diff changes only comments and
tests"), never one bound to a SHA.

## "Not on main" is not "not being worked"

A blob check on `origin/main` answers "is it landed," not "is it being
landed right now" — both render as absence, which is what a stall looks
like. Three routed candidates read absent on `main` and were reported as a
stalled queue; one of them was publishing at that exact moment, with an
open, mergeable PR and CI in progress. Before calling a routed candidate
unpublished, check for an open PR and a live publisher process, not only
the tree — landing is a content question, being worked is a process
question, and a blob check is blind to the second by construction.

## The commit that breaks tip == routed is always the harmless one

A doc-only or memory-lesson commit landed on top of an already-routed or
already-approved branch, moving its tip by a few minutes, twice in one hour
on two different branches. Neither extra commit was risky — that is the
mechanism, not a mitigating detail: nobody hesitates before a harmless
commit on a branch that is already approved, so it is exactly the change
most likely to move a tip someone else is holding a SHA for. The gate this
implies is not "is this diff risky" but "is a route or an approval
outstanding on this branch" — a question about the branch's *state*, which
a diff review does not answer and a harmless diff actively hides. When the
tip does move under an outstanding approval, prefer re-verifying and
re-routing at the new tip over publishing a `<old> -> <new>` mapping: a
mapping only makes the drift discoverable to a router who reads it, while
the several earlier posts naming the old SHA keep reading as the
artifact's identity because they carry the approvals.

## After you hand off a branch, your own worktree is the stale one

Once a branch is released (`git checkout <role>/work` so another seat can
check the WP branch out), every later `grep` in that worktree reads a home
branch that can be hundreds of commits behind main. This produced three
wrong reads in one session — an implausible file count, an empty grep for a
string just written, two numbers in one command contradicting each other —
each caught only because the answer looked wrong, not because anything
errored. Once a candidate is handed off, measure its content only as `git
show <sha>:<path>`, never a bare `grep` or the working tree.

## How to apply

- Before routing: `git rev-parse <branch>` and compare to the SHA you are
  about to name. Re-run that same check immediately before any command
  that pushes or force-moves the ref — not earlier in the operation.
- Check which `resolve_branch` path a `--target` will take (branch name:
  unpinned; SHA: pinned only if the branch match is non-unique) before
  trusting either "it's pinned" or "it's live."
- State a contention check's scope explicitly ("path-disjoint," not
  "contention-free") and separately ask whether any candidate quotes text
  another live candidate edits.
- Treat any approval or routing artifact naming a SHA as dead the moment
  that SHA is rewritten by amend or re-cut — ancestry cannot tell the two
  apart, so don't ask it to.
- After a handoff, read a branch's content only via `git show <sha>:<path>`;
  treat a bare `grep` or working-tree read on that worktree as suspect by
  default.
- Before reporting a routed candidate stalled off an absent blob check, also
  check for an open PR and a live publisher process on it.
