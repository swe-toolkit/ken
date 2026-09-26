---
scope: build
audience: (see scope README)
source: private memory
  `inserting-a-test-before-an-existing-one-silently-splits-its-doc-comment`,
  `a-comment-only-diff-is-not-a-safe-diff-doc-comments-compile`,
  `a-doc-comment-is-not-inert-an-indented-block-inside-a-module-header-is-a-compiled-doctest`
  (R4 triage, 2026-09-26)
---

# Rust comments and attributes are not inert — both compile, and both can be stolen by a nearby edit

A "comment-only" or "pure insertion" diff feels self-evidently safe, and that
feeling is exactly where two distinct defects live: rustdoc compiles some
comment content as code, and inserting an item next to another item's
leading trivia can silently reattach that trivia — doc comment or
attribute — to the wrong item.

## A doc comment can compile as a doctest

Four-space indentation inside `//!` or `///` is a Markdown indented code
block, and rustdoc compiles an untagged code block inside it as Rust. A
comment-only header rewrite that added a plain-prose item roster, four-space
indented, turned CI red with `error: expected one of '!' or '::', found
<item name>` — the "comment" was handed to the compiler as source.

An acceptance criterion phrased as *"zero compiled change: the diff is `//!`
lines only"* cannot catch this, because the diff **is** `//!` lines only in
the presence of the defect. It encodes the premise "a doc comment is inert"
and then measures the premise instead of the property. Control the property
with something that actually compiles instead: `cargo test --doc -p
<crate>`, plus a plain `cargo build -p <crate>` for that file. Census both
`//!` and `///` — item docs are doctested too.

Two pre-existing indented blocks in the same file did **not** fire, because
they sat inside a private `mod tests` (later instance: inside private items
generally) and `cargo test --doc` only collects doctests from **public**
items. That is a fact about visibility, not a safety property — the block
arms the moment the enclosing item is made `pub`, so a green doctest run is
evidence about visibility, not evidence the file has no compiled prose.

Tag a non-Rust block `text`, not `ignore` — `ignore` means "Rust we choose
not to run," which is the wrong claim for prose that is not Rust at all. And
when a frame or spec doc illustrates a block that will be pasted verbatim
into a `//!` or `///` comment, show it already fenced and tagged: an
indented illustration is inert in a Markdown node and compiled the moment it
lands in a doc comment, and an implementer who copies the frame's shape
copies its indentation too.

## Inserting an item can steal a neighbour's leading trivia

An insertion anchored **after** an item's leading doc comment or attribute,
rather than before the start of that block, reattaches the trivia to the new
item instead of the original:

- **Doc-comment variant.** Anchoring inside an existing item's doc block
  splits it at the anchor point — everything above documents the newly
  inserted item, the original keeps only what is below. It compiles, the
  suite is green, `cargo check` is silent; nothing catches a misattributed
  doc comment.
- **Attribute variant, and this one reaches production.** Inserting a new
  item **between** a leading `#[cfg(test)]` and the item it gates attaches
  the gate (and the doc comment) to the new item instead. The originally
  gated item loses its `#[cfg(test)]` and begins compiling into production,
  while its own re-export and consumers stay test-gated — a new, unused
  production item that survived a full review pass.

**The test profile is structurally blind to a lost `cfg(test)` gate.** Under
`--profile test` both the stolen-gate item and its consumers compile and are
used regardless of which one nominally holds the attribute, so a green test
run — even a large one — says nothing about whether the gate moved.

**A zero-deletion diff is not evidence the gate survived.** Stealing a gate
by insertion needs no deletions at all, which is precisely the shape this
defect arrives in. The instrument that actually answers "did anything lose
its gate" is a positive measurement: extract the set of `#[cfg(test)]`-gated
declarations at the merge base and at the head, and diff both directions
(gated-at-base-but-not-head, and the reverse). A line-based gate tracker
built for this can itself mis-see a macro invocation that wraps items (e.g.
`thread_local! { ... }`) — the gate attaches to the macro call, and a
scanner that thinks in items reads the items nested inside it as ungated.
Read a flagged region verbatim before filing it as a real loss.

## How to apply

- Anchor an insertion at the **start** of the target's leading trivia block —
  its first `///`/`//!` line or its first attribute — never on a line inside
  it, and never between an attribute and the item it gates.
- Give an inserted item its own doc comment and attributes explicitly,
  rather than letting it inherit a neighbour's.
- Reviewing a diff near an attribute: ask "did any `cfg` attribute change
  owner," not "did the tests pass." A `+`-only hunk that begins right under a
  context-line attribute is the tell — the attribute reads as untouched
  context.
- Prove a `cfg`-gate repair, or the absence of a regression, with a
  **production-profile build** (`scripts/ken-cargo build -p <crate>`), never
  only a test run.
- Control a "comment-only" or "pure insertion" claim with a build
  (`cargo test --doc -p <crate>`, or the gated-set diff above), never with an
  argument about the diff's shape.
