# LANG-SURFACE-BLOCK-COMMENTS — block comments, doc comments, and attachment

Owner: language. Size: M. Node: [[LANG-SURFACE-BLOCK-COMMENTS]].
Fixed inputs measured at `origin/main` = **`98702040`**, every coordinate read
from the git object at that SHA. Re-derive your merge-base from `origin/main`;
**do not take a SHA from this frame.**

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if your
diff stays in `crates/`.

## What this deliverable is

`31-lexical.md:562-567` specifies three comment forms. One exists:

| form | spec | state at `98702040` |
|---|---|---|
| line `-- …` | `:562` | **works** |
| block `{- … -}`, **nestable** | `:562` | absent |
| doc `--- …` and `{-- … --}`, attaching to the **following** declaration | `:563-567` | absent |

`:352-359` adds the retention and placement rules: every comment is retained, a
doc comment stays attached to the following declaration, a leading comment
remains immediately above the node it precedes at that node's indentation, an
end-of-line comment stays inline only when code plus two spaces plus comment
fit in 96 columns, and a comment between tokens forces the surrounding group to
break and is **never moved across a syntactic boundary**.

## Why this is M and not new core

**The semantic target already exists and is already exercised.** `--` comments
elaborate to nothing, are retained as trivia, have an attachment home, and are
consumed by the formatter. **This is scanner, trivia and classification
completion — not a new elaboration path, not a kernel change, not a new AST
node.** If you find yourself adding an `Expr` variant, stop; you are in the
wrong node.

## The design judgment, front-loaded — THERE ARE TWO SCANNERS

This is the whole risk of the node, so read it before you plan.

**Comments are scanned twice, by two independent pieces of code with opposite
jobs.**

- **The semantic lexer SKIPS them.** `Lexer::skip_ws_comments`
  (`crates/ken-elaborator/src/lexer.rs:145-158`) consumes whitespace, then — if
  the source starts with `--` — advances to the next newline. That is its whole
  comment vocabulary.
- **The lossless layer RESCANS the source and RETAINS them.** `TriviaKind`
  (`lossless.rs:24-29`) has exactly `Whitespace` and `LineComment`.
  `append_trivia` (`:241`) recognizes only `--`, tagging at `:254`.
  `attach_comments` (`:292`) filters to `LineComment` at `:310`.
  `validate_attachment_totality` (`:662`) filters the same way at `:668`.

> ### For `--` the two agree FOR FREE. For `{- -}` they do not, and that is the node.
>
> A line comment **always terminates** — at a newline or at end of input. It
> cannot nest and it cannot fail. So the two scanners cannot disagree about
> where one ends, and neither can error.
>
> `{- … -}` breaks both properties at once. **It nests**, so both scanners need
> a depth counter and both must count the same way. **And it can be
> unterminated** — a failure mode `--` structurally cannot have.
>
> ⇒ **If the two scanners disagree on nesting depth, or on where an unterminated
> comment ends, you get a token stream and a trivia stream that describe
> different programs**, and nothing in the current tests would say so. That is
> the defect this node exists to not create, and `AC-1` is its control.

**Classification order, and it must be the same in both scanners.** `{-` shares
its opening `{` with refinement types, class and instance bodies, module
bodies, and — once [[LANG-SURFACE-RECORD-LITERAL]] lands — record literals.
`{--` is a prefix-extension of `{-`. `---` is a prefix-extension of `--`.
**Specific before general, in both scanners, in the same order.** If you find
yourself backtracking to tell two comment forms apart, stop and tell me — it
means a row in the enumeration below is wrong and I would rather fix the frame.

## Deliverables

**`D0` — `true` and `false` as literals.** Independent of everything else here;
**do it first so it cannot be squeezed out at the end.** `31-lexical.md:512`
specifies them as `Bool` literals. They are **absent from the lexer's keyword
map** (`lexer.rs:467-520`, which does carry `data`, `class` and the rest), so
they currently resolve as ordinary identifiers. `False` and `True` already
exist in the prelude (`prelude.rs:154-157`), so the target is built. This is a
spec-conformance gap, not an ergonomic one, and it is folded here because it is
too small to be its own node and lives in a file this node already opens.

**`D1` — block comments.** `{- … -}`, **nestable**, recognized by *both*
scanners with the same depth rule.

**`D2` — doc comments.** `--- …` and `{-- … --}`, with `TriviaKind` extended to
carry the distinction. Retention is the existing mechanism; do not invent a
second one.

**`D3` — attachment.** A doc comment attaches to the **following** declaration.
`attach_comments` and `validate_attachment_totality` must both see the new
kinds — note they *filter* on `LineComment` today, so a new kind is invisible
to them until you say otherwise, and **an invisible kind fails
`validate_attachment_totality` silently rather than loudly.**

**`D4` — the refusals.** An unterminated block comment is an error with a span.
**Say what the span is** — the opener, or opener-through-EOF — and make both
scanners agree it is an error.

## Acceptance criteria

**AC-1 — the two scanners agree. This is the control the node exists for.**
Over a corpus covering every form and both nesting and unterminated cases,
assert that the token stream and the retained trivia describe the **same
source**: concatenating trivia and token text reproduces the input exactly.
**A test that only checks "it parses" passes for an implementation whose two
scanners disagree**, because the semantic lexer's answer is the only one
parsing consults.

**AC-2 — nesting, at two levels minimum.** `{- {- -} -}` is **one** comment.
`{- {- -}` is **unterminated**, not a complete comment. **One level cannot
distinguish "nests" from "scans to the first `-}`"** — both give the same
answer on `{- x -}`, and the two-level case is the only witness.

**AC-3 — doc attachment is to the FOLLOWING declaration, discriminatingly.**
Construct a case where attaching to the *preceding* declaration would give a
different answer, and assert the following-declaration reading. A doc comment
sitting between two declarations is the shape; a comment at the top of a file
with nothing before it is not a control.

**AC-4 — unterminated is an error, in both scanners, with a span.** Assert the
error and its span, not merely that something failed.

**AC-5 — enumerate the prefix relations and control each.** A list with a
control per entry, not a sentence. Minimum: `--`, `---`, `----`, `{-`, `{--`,
`{---`, and `{-}`. **Say what each one is.** Some of these are genuinely
ambiguous under a careless rule and the enumeration is what finds out which.

**AC-6 — the brace neighbours still parse, adjacent in the same program.** The
refinement type `{ n : Int | n ≥ 0 }`, `class C (A) { … }`,
`instance C T { … }` and `module M { … }` — each **in a program that also
contains a block comment**, not in isolation. If [[LANG-SURFACE-RECORD-LITERAL]] has landed by
the time you cut, add `{ x = 1, y = 2 }` to this list.

**Why this AC exists.** `LANG-LEX-HEX-FLOAT` took four Architect rejections,
**every one on the scanner**, while its genuinely intricate half was right in
round one. A new lexical form is risky in proportion to how many constructs
share its opening characters, and `{-` shares `{` with four.

**AC-7 — the formatter still round-trips.** The existing
`push_comments_between`/`with_comments` consumers keep working, and a block
comment survives a format cycle byte-identically.

**AC-D0 — `true` and `false` elaborate at `Bool`**, and an identifier named
`trueish` still resolves as an identifier.

**AC-8 — no `spec/` edit, no new kernel term, `trusted_base()` unchanged.**

## Excluded scope

- **The doc generator and the LSP.** `:563-567` says doc comments are consumed
  by both, and by the test framework for runnable spec fragments
  (`../50-stdlib/`, strategy T3/T4). **This node makes doc comments exist and
  attach. It does not build a consumer.**
- **New layout rules.** The 96-column and group-breaking rules at `:352-359`
  are already implemented for line comments; the new kinds ride the existing
  mechanism. **If a new kind needs a new layout rule, that is a stop, not an
  edit.**
- **Any change to the record declaration, literal forms, or the registry.**
- **Nested `--` inside `{- -}` or vice versa as a *semantic* question** — the
  scanner decides it by the classification order, and there is no separate
  rule to invent.

## Stop conditions — return to me, do not decide

- **The two scanners cannot be made to agree** without restructuring one of
  them. That is the frame's central premise failing.
- **You need to backtrack** to tell two comment forms apart.
- **`TriviaKind` cannot take a new variant** without a signature change that
  ripples beyond `lossless.rs` and the formatter.
- **`validate_attachment_totality` cannot express the doc-attachment rule**
  without weakening its existing guarantee.
- **`D0` turns out not to be independent** of the comment work.

## Contention

`crates/ken-elaborator/src/lexer.rs`, `src/lossless.rs`, and the formatter
layout path, plus `lexer.rs` alone for `D0`.

**[[LANG-SURFACE-RECORD-LITERAL]] is in flight in the same lane.** It touches
`parser.rs`, `elab.rs` and `classes.rs` — **a disjoint set from this node's**,
because `{-` is decided in the *lexer* before the parser ever sees a `{`. The
layers do not contend. **Sequence this after it anyway**, because the lane is
single-threaded and because `AC-6` wants record literals in its neighbour list
once they exist. **Re-derive the intersection at candidate time** — a
merge-base goes stale without your branch moving.

## Sizing and validation

`scripts/ken-cargo test -p ken-elaborator` plus your focused suite.
**Never `--workspace`**; that is CI's gate.

The measurement this frame is built on is the research sweep at `98702040`
(`evt_6qeeebh5m3fba`), which ranked this the **only unqualified M-sized
Language closure** currently available. Character, byte and string-escape
literals are M-shaped with their targets already built but are **blocked on the
literal-escape spec pin**, which is in the enclave now.
