---
id: LANG-LEX-PROJECTION-ADJACENCY
title: "The positional-projection lexer guard tests raw character adjacency, so exactly one of four spacing variants fails -- `p.1.2`, `p.1 .2` and `p. 1 .2` all lex as two projections while `p. 1.2` lexes as `Dot, FloatLit(1.2)` -- and the refusal comes from the number scanner rather than from any grammar rule"
status: merged
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: https://github.com/swe-toolkit/ken/pull/1864
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
something else. **This was derived structurally from the two sites above, and
then CONFIRMED by execution** on unmodified `40ea9ffd`, before any edit:
`p. 1.2` yields exact `ParseError { msg: "unexpected token after expression:
Dot", span: 1..2 }`. The severity claim was measured rather than inherited,
which is what `AC-1` existed to force.

## What was not yet known, and how it came out

Both open axes were measured before any repair was authorized.

**The formatter axis is empty, and the stop that guarded it was mis-worded.**
The original stop fired on a formatter that could *"emit or preserve"* a space
after a projection dot; the hazard is one that can **produce** the failing
spelling `p. 1.2`. Language measured both tools: the lossless printer preserves
`p.1 .2`, `p. 1 .2` and `p. 1` byte-exactly — every one of which parses — and
`kenfmt` canonicalizes them to `p.1.2` / `p.1`. **Neither can produce
`p. 1.2`**, so the meaning-changing-reformat class has no member and the stop
did not fire (Steward ruling `evt_3yktja0yw62nw`).

**The comment case decided the mechanism, by the frame's own criterion.**
`p. -- note` + newline + `1.2` had the same `FloatLit`/refusal shape, so raw
source-text trimming would have left the inconsistency one position over.
Language took the emitted-token context instead: `next_token` records the last
successfully emitted token after trivia is skipped, and `lex_numeric` consults
it at the existing fractional-part decision. **Whitespace and comments are
trivia at this seam**, which is the rule the node was opened to state.

## Landed

`28cebda7` (PR #1864), Decision `dec_29w9dcyd4xd76`. All four spellings plus
the comment-separated form share one token stream and one AST; `3.14`, `1.2e5`
and `1.2d` keep their numeric classes; the guard A/B is causal in both
directions; and kenfmt's canonical output of a spaced projection reparses, so
canonicalization does not cross the accept/reject boundary.
