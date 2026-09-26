---
name: a-helper-inserted-between-a-doc-block-and-its-fn-steals-the-doc
description: A new item inserted between an existing `///` block and the fn it documents takes the whole doc, even with a `//` comment in between, and the original fn is left undocumented. When a diff hunk adds a `fn` right after `///` context lines, check what the next item is. An inserted `///` block needs the same check, plus a check that no added line opens a doctest fence.
metadata:
  type: feedback
---

# A helper inserted between a doc block and its fn steals the doc

**Measured 2026-09-24 on KERNEL-LITERAL-ROLLBACK-PURGE.** Squash
`ad96424560aff7095292b290710383f1248cf8d2`, reported at `evt_41h58vpcdz56x`
(thread `thr_54wc1rh7th2ev`). Smell only. The soundness fix was clean.

## The shape

`rollback_literal_decl` was added at elab.rs:13627. That is directly after the
33-line `///` block for `elaborate_recursive_view` and before its `fn`. The
helper opened with a `//` comment. A plain `//` comment does not end an outer
doc comment, so rustdoc attached the whole block to the helper, and
`elaborate_recursive_view` ended up with no doc at all. A three-line rustdoc
probe confirmed it.

## How to apply

1. In a diff, when a `+fn` hunk has `///` lines as its leading context, the
   insertion point is inside a doc attachment. Read the next item after the
   hunk.
2. To confirm, rustdoc a scratch file with `/// A`, `// B`, `fn x(){}`,
   `fn y(){}`. The result is that "A" lands on `x`.
3. File it as a smell. The fix is to move the helper, which is the owning
   team's call.

## The same check applies to a doc block that is inserted, not appended

An earlier instance (2026-08-17, a 30-line rustdoc insertion beside a mutation
narrative) is the other direction: a long `///` block inserted mid-file is
exactly where attachment drifts to the wrong item. **Confirm which `fn` follows
the block**, and **confirm no added `///` line opens a fence**: a comment-only
Rust diff can still add compiled, executed code that way, since rustdoc runs
fenced examples as doctests. Both are one command, and in that instance both
were claimed rather than obvious. The prose half of that instance is in
[[a-hedge-does-not-constrain-what-is-done-with-the-claim-it-hedges]].
