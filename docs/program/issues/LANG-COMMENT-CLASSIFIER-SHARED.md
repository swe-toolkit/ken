---
id: LANG-COMMENT-CLASSIFIER-SHARED
title: "The lexer and the lossless layer each carry their own copy of the block-comment classification -- the `{--`-before-`{-` ordering twice and both end-scanners twice -- so their agreement is held by a comment saying they mirror each other `exactly` and by tests, with nothing failing to compile when they diverge; and the divergence they can reach disagrees about comment KIND rather than acceptance, which the `is_ok()`-comparing net cannot see and round-trip cannot see either"
status: active
owner: language
size: S
gate: none
depends_on: [LANG-SURFACE-BLOCK-COMMENTS]
blocks: []
github: null
origin: Architect finding at evt_6703rhjmt19gm approving LANG-SURFACE-BLOCK-COMMENTS (dec_7tr1mx06rnap2), non-blocking on that candidate and explicitly routed to the Steward for an owner. Steward re-measured both sites at fa26625b before framing and narrowed the finding's stated population -- see the frame.
---

## What this is

Frame: `docs/program/wp/LANG-COMMENT-CLASSIFIER-SHARED.md`.

`lossless.rs` reimplements the comment classification the lexer performs. The
duplicated, hazardous part is the `{--`-before-`{-` dispatch ordering and both
end-scanners. Reorder those two tests in **one** of the two sites and doc
blocks silently become plain blocks in that scanner alone — the two then
disagree about **kind** rather than about acceptance, so the formatter reflows
a doc comment as ordinary and changes its attachment.

The fix is structural rather than another test: one classifier over `(src,
pos, end)` returning the kind and the end offset, called by the lexer to
advance and by `append_trivia` to bound its slice. Divergence becomes
unrepresentable instead of tested-against.

## The measured narrowing, which the frame carries in full

The finding describes a **four-way** dispatch duplicated on both sides. The
lexer's dispatch has **three** arms: `lexer.rs:179-182` folds `---` into the
`--` arm deliberately, with the reason recorded at the site, because a doc line
comment and a line comment scan identically to end-of-line.

⇒ **The `---`-before-`--` ordering is not duplicated and is not part of this
node.** Restoring a `---` arm to the lexer to make the two dispatches look
alike would add a distinction the semantic lexer has no use for and delete a
documented decision.

## The residual this also closes

`lex_agrees_with_lossless` compares `is_ok()` booleans, so it catches a
divergence only when it flips acceptance. A both-accept-but-different-end
divergence is caught on the lexer side by the predecessor's token-stream
assertion, but nothing asserts the **lossless** layer's comment end equals the
lexer's, and `reconstruct() == src` cannot supply it — it is byte-preserving
under either attribution. `AC-3` is one direct offset comparison, and it stands
independently of how the classifier is factored.

## One severable rider, folded rather than given a node

`AC-7` carries a one-line strengthening of
`LANG-FOREIGN-NAME-CONTROL-CHARS`'s positive control, which is keyed on
absence-of-error where the property is presence-of-value (Adversary,
`evt_cxbze6z3yns8`). The Adversary declined to prescribe a repair without one
read: whether a `const` exposes its literal the way `Decl::ForeignDecl {
symbol, .. }` does.

**Measured 2026-08-13 — it does.** `const` dispatches to `parse_view_decl` with
`DefKeyword::Const` (`parser.rs:181`), yielding a `ViewDecl` whose `body` is
`Expr::EStr(String, Span)` (`ast.rs:615`). So the repair is one `assert_eq!` in
the existing `d0_foreign_names_decode_escapes_uniformly` idiom, not a new
accessor. Severity is low, no defect is present, and it is folded here rather
than filed because it is one line in the same crate's test surface.

## Not this node

- Any change to what a comment form means, where it attaches, or whether doc
  blocks nest. The predecessor settled those against `31 §5`.
- Formatter behaviour.
- A general lexer/lossless convergence project. This is comment classification
  only, and the wider version is not S-sized.
