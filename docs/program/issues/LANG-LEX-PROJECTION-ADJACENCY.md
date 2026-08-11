---
id: LANG-LEX-PROJECTION-ADJACENCY
title: "The positional-projection lexer guard tests raw character adjacency, so exactly one of four spacing variants fails -- `p.1.2`, `p.1 .2` and `p. 1 .2` all lex as two projections while `p. 1.2` lexes as `Dot, FloatLit(1.2)` -- and the refusal comes from the number scanner rather than from any grammar rule"
status: ready
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: Adversary report evt_5xgcf74gn2rgp on the merged LANG-SURFACE-PAIR (measured on origin/main=63d6e007), triaged by the Steward. The Adversary's finding was explicitly conditional on whether the spaced form is grammatical; the Steward settled that question structurally and corrected the severity. The four-variant enumeration and the mechanism fork below are Steward findings on top of that report, not part of it.
---

## The gap

`LANG-SURFACE-PAIR` landed a lexer guard so that `p.1.2` reads as two
positional projections rather than letting the number scanner swallow the
second one into a float literal. The guard is at
`crates/ken-elaborator/src/lexer.rs:524`:

```rust
let follows_dot = self.src[..start].chars().next_back() == Some('.');
```

**It tests raw character adjacency.** Every other lexical decision in the crate
is whitespace-insensitive — `lexer.rs:140` skips whitespace between tokens, and
the parser's projection loop (`parser.rs:2219-2220`) inspects only tokens:

```rust
while matches!(self.peek(), Token::Dot)
    && matches!(self.lookahead(1), Token::Ident(_) | Token::Nat(1 | 2))
```

So whitespace around a projection dot **is** grammatical, and the guard is the
sole place in the surface where it is not.

## Exactly one of four spacing variants fails

The number scanner is entered only on a leading digit (`lexer.rs:420`), so a
dot that is *followed* by whitespace never begins a numeric scan:

| written | token stream | outcome |
|---|---|---|
| `p.1.2` | `Ident, Dot, Nat(1), Dot, Nat(2)` | two projections |
| `p.1 .2` | `Ident, Dot, Nat(1), Dot, Nat(2)` | two projections |
| `p. 1 .2` | `Ident, Dot, Nat(1), Dot, Nat(2)` | two projections |
| `p. 1.2` | `Ident, Dot, FloatLit(1.2)` | **fails** |

The failing cell is not a grammar rule. It is the number scanner reaching
across a token boundary because the character before `1` happened to be a space
rather than a dot.

## The severity is a refusal, not a silent mis-parse

**This corrects the report that produced this node**, and it is the reason the
node is `S` rather than urgent. The report reasoned that the parser would see
*"a well-formed projection of a float"*. It cannot: the projection loop's
lookahead admits only `Token::Ident(_)` or `Token::Nat(1 | 2)`, and a
`FloatLit` matches neither, so **no projection node is built at all** and the
dot is left unconsumed.

⇒ The derived outcome is a **parse error on a program that should be
accepted** — loud and attributable — not a program that silently means
something else. **Derived structurally from the two sites above and not
executed.** Establishing the observed diagnostic is the first deliverable, and
a measurement that contradicts this derivation raises the severity rather than
closing the node.

## What is not yet known

- Whether any formatter path (`kenfmt`, the lossless printer) can emit or
  preserve a space after a projection dot. If one can, a reformat could turn an
  accepted program into a rejected one. **Unmeasured, and it is the one axis
  that would make this worse than a surface wart.**
- Whether a comment between the dot and the index (`p. -- note` / newline /
  `1.2`) should behave identically. It bears directly on the mechanism fork in
  the frame, because a source-text scan cannot see past a comment and a
  token-context guard can.
