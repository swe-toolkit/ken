---
name: a-shape-deviation-from-the-frame-or-an-import-the-source-never-names-is-a-candidate-until-the-conforming-variant-is-measured
description: Before filing "the landing deviates from the frame's shape" or "this import is dead widening", build the conforming variant at the squash. The frame's shape may be refused by the elaborator, and a name absent from the source text may still be required to load.
metadata:
  type: feedback
---

# Measure the conforming variant before filing a deviation

**Measured 2026-09-23 on CAT-PARSING-LAWS partial deliverable 1.** Squash
`f915b76528ae97dd79247197c6868f8d8a1127f6`. The hunt was CLEAN, reported at
`evt_6s9jwqf3pce9y`.

Two things looked like findings, and neither survived a probe.

## The frame's shape was not expressible

The frame asked for a "public attached proof" of `ParserLaws (Syntax BoolExpr)
parse_bool_expr`. What landed was a standalone `pub theorem`. When I added
`pub proof laws for parse_bool_expr : ...` at the squash, the elaborator refused
it: "attached proof ... must mention that subject applied in its claim". The
reason is that `ParserLaws` takes the parser **unapplied**. So the standalone
form was forced. Only the word "attached" in the prose was wrong, which is a
smell whose root is the frame text.

## An import the source never names was required

The squash widened `import Capability.Parsing.Cursor` with `cursor_advance`,
`cursor_locate` and `cursor_peek`, and none of them appears anywhere in the
source text. When I removed them, loading failed with `UnresolvedCon
cursor_locate`. The elaborator needs those names in scope for the Decoder law
types it instantiates. So the widening is necessary, not dead.

## How to apply

For any finding shaped "should have been X" or "Y is unused", build X, or remove
Y, at the detached squash. Run the targeted harness, then revert. Report what
you measured, including a refusal message, because the refusal is what turns a
would-be LOW into a note. Pair this with
[[a-catalog-laws-statement-is-pinned-only-by-use-so-weaken-a-private-one-and-unpack-a-public-one]]:
the same hunt also unpacked the law through its private body (accepted), with a
false-claim control that was rejected.
