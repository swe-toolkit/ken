---
id: LANG-SURFACE-BLOCK-COMMENTS
title: "`31-lexical.md:562-567` specifies nestable block comments `{- ... -}` and doc comments `--- ...` / `{-- ... --}` attaching to the following declaration, and neither exists -- the semantic lexer's skip_ws_comments knows only whitespace and `--`, and TriviaKind carries only Whitespace and LineComment, so the two independent scanners that must agree about comments have only ever been exercised on the one form that cannot nest and cannot fail to terminate"
status: merged
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: Research surface-gap sweep evt_6qeeebh5m3fba measured at 98702040, ranked this the only unqualified M-sized Language closure available; Steward re-verified every cited coordinate from the git object at that SHA before framing. Folds the true/false keyword-map conformance gap from the same sweep as an independent rider. Steward-filed (agents cannot create tracked work per COORDINATION 2).
---

## The gap

`spec/30-surface/31-lexical.md:562-567` specifies three comment forms. One
exists.

| form | state at `98702040` |
|---|---|
| line `-- …` | works |
| block `{- … -}`, **nestable** | absent |
| doc `--- …` and `{-- … --}` | absent |

`:352-359` adds the retention and placement rules — every comment retained, doc
comments attached to the **following** declaration, leading comments held above
their node, end-of-line comments inline only within 96 columns, and a comment
between tokens forcing a group break and never moving across a syntactic
boundary.

## Why it is a small node

The semantic target already exists and is exercised: `--` comments elaborate to
nothing, are retained as trivia, have an attachment home, and are consumed by
the formatter. **This is scanner, trivia and classification completion — no new
elaboration path, no kernel change, no new AST node.**

## The real risk — two scanners that have never been tested against each other

Comments are scanned twice by code with opposite jobs. `Lexer::skip_ws_comments`
(`lexer.rs:145-158`) **skips** them and knows only whitespace and `--`. The
lossless layer **rescans and retains** them: `TriviaKind` (`lossless.rs:24-29`)
is `Whitespace | LineComment`, `append_trivia` (`:241`) tags at `:254`,
`attach_comments` (`:292`) filters at `:310`, and
`validate_attachment_totality` (`:662`) filters at `:668`.

**For `--` the two agree for free**, because a line comment always terminates
and cannot nest — so they cannot disagree about where one ends, and neither can
error.

`{- … -}` breaks both properties at once. It **nests**, so both scanners need a
depth counter and must count alike; and it can be **unterminated**, a failure
mode `--` structurally cannot have.

⇒ **If the two disagree on nesting depth or on where an unterminated comment
ends, the token stream and the trivia stream describe different programs**, and
nothing in the current tests would say so.

## Folded rider — `true` and `false`

`31-lexical.md:512` specifies them as `Bool` literals. They are absent from the
lexer's keyword map (`lexer.rs:467-520`, which carries `data`, `class` and the
rest), so they resolve as ordinary identifiers; `False` and `True` already exist
in the prelude (`prelude.rs:154-157`). A spec-conformance gap, too small to be
its own node, folded here because it lives in a file this node already opens
and is otherwise independent of the comment work.

## Frame

`docs/program/wp/LANG-SURFACE-BLOCK-COMMENTS.md` — deliverables, the
two-scanner design judgment, acceptance criteria with controls, excluded scope,
stop conditions and contention.

## Not this node

The doc generator, the LSP, and the runnable spec-fragment test framework that
`:563-567` names as doc-comment consumers. This node makes doc comments exist
and attach; it does not build a consumer. New layout rules are excluded — the
96-column and group-breaking rules already work for line comments and the new
kinds ride the existing mechanism.

Character, byte and string-escape literals are **not** this node and are not yet
framed: the same research sweep found all three M-shaped with their semantic
targets already built, but blocked on the literal-escape spec pin, which is in
the enclave.
