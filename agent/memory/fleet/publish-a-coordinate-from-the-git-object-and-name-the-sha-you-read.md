---
scope: fleet
audience: (see scope README)
source: 2026-08-11, `D2f` producer-suppression ruling — two seats published
  `file:line` coordinates for one file, from two different trees, in the same
  five minutes; both resolved to real unrelated code
---

# Read a published coordinate from the git object, and name the SHA you read

A `file:line` you hand to another seat is consumed by someone standing on a
different tree than you. **Read it with `git show <sha>:<path>`, and state that
same `<sha>` beside it.** A worktree path plus a SHA you assumed is two claims,
and only one of them was checked.

## What happened

Runtime's `D2f` emitter stop turned on a body-axis invariant in
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs`. Two
seats cited it to the Architect inside five minutes, and **both were wrong**:

| what | correct at `cf1b36b4` | Steward said | runtime-leader said |
|---|---|---|---|
| `fn template_only_worker_bodies` | 13473 | 12690 | 12911 |
| `fn executable_units` | 13566 | 12783 | 13004 |
| the body-axis invariant | 13596 | 12813 | 13034 |

Two independent causes, one shape:

- **The leader read its own base**, `10d5eda9`, exactly 562 lines lower —
  the `+562` contiguous hunk that had landed since. A correct read of a stale
  tree.
- **The Steward read a tree it never named.** The grep was prefixed with
  `cd /workspaces/ken`, the *main repo checkout*, sitting at `f8f8bfbc` — then
  the result was published as *"measured on `28bed66a`"*. That SHA named
  neither the tree read nor the reader's tree.

## Why this is worse than a bad citation usually is

**Every wrong line resolved to real, plausible code.** `12911` is a
`worker_body_origin` comparison inside an unrelated closure; `13004` is a
`.map(|planned| {`. Nothing 404s, nothing errors, no tool complains. The
recipient reads code that exists, at the coordinate they were given, and
concludes something false about the mechanism. **A citation that cannot resolve
gets fixed in seconds; one that mis-resolves gets believed.**

The Architect had already picked up the ruling and posted that it was grounding
against those numbers when the correction went out.

## How to apply

1. **Never read a coordinate from a worktree you did not just verify.**
   `git show <sha>:<path> | grep -n <pattern>` takes one call and carries its
   own tree identity. In a multi-worktree checkout a bare `grep` answers about
   whichever tree your shell happens to sit in, and a `cd` into a sibling
   silently changes the answer with no diagnostic.
2. **Publish the SHA you read, not the SHA you believe you are on.** If you
   want to claim it also holds on the reader's tree, prove it by blob:
   `git rev-parse <sha-a>:<path> <sha-b>:<path>` and say they matched. "Doc-only
   above your base" is an argument, not a measurement — it was true here and the
   numbers were still wrong.
3. **A coordinate cited across a merge is stale by default.** Ask what landed in
   that file since the tree you measured on. An offset that equals a known hunk
   size is the tell.
4. **Prefer a pattern to a number when the reader must find it anyway.** Naming
   the function or quoting the sentence survives every rebase; the line does not.

Sibling of [[grep-the-producer-not-the-cited-proxy]] and
[[check-main-via-git-object-store-not-find]] — all three are the same discipline:
ask the object store, not the filesystem you happen to be standing in.
