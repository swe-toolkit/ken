---
id: LANG-PARSER-DEPTH-HONEST
title: "The record-literal arc's parser depth control has never measured depth at any SHA -- it builds match arms with `=>`, which is not a Ken token, so it dies in the lexer on the first arm and never reaches depth one; a 143-line `parser.rs` rework was written to satisfy it, so that rework's justification is unmeasured and its disposition needs a working instrument first"
status: ready
owner: language
size: S
gate: none
depends_on: [LANG-SURFACE-RECORD-LITERAL]
blocks: []
github: null
origin: Architect evt_1f9z6akt6vrj5 on the merge review of 50da348a, correcting the inference the Steward's own discriminator (evt_4b7caqfkj5zj6) would otherwise have licensed. Framed by the Steward 2026-08-13.
---

## The gap

`crates/ken-cli/tests/record_literal_parser_depth.rs`, added at `8e9baa18`,
builds its nesting with

```rust
format!("match 0 {{ _ => {body} }}")
```

**Ken's lexer has no `=>` token.** The match-arm separator is `MapsTo`, spelled
`|->` or `↦` — `crates/ken-elaborator/src/lexer.rs:107`, emitted at `:237` and
`:287`, measured at `origin/main` = `288b68c9`.

⇒ The source dies in the **lexer, on the first arm**. The parser never reaches
depth **one**, and `NESTED_MATCH_DEPTH = 31` is inert. The control fails
identically at `57688110`, `50da348a`, `766c9f07` and `8e9baa18`.

## Why a red here is not evidence in either direction

This is the whole reason the node exists, and it is not the same as "the test
is broken."

A control that dies in the lexer **cannot distinguish "the capability was
absent" from "the capability was present."** It is blind to both. So the run at
the base — which did happen, and was carried out faithfully — does not license
either conclusion:

- it is **not** evidence that the record-literal work regressed parse depth;
- it is **not** evidence that depth 31 never held.

**Do not record "depth 31 never held" anywhere.** That inference is exactly
what a working instrument is needed to settle, and the disposition it would
have produced for `50da348a`'s merge happens to be right for unrelated reasons
(see below), which is how a false premise survives.

## What is actually unmeasured

`766c9f07`, titled *bound record parser stack use*, is a **143-line rework of
`parser.rs`** written to satisfy this control. Since the control measures
nothing, **the rework's justification is unmeasured — not wrong.** There may be
a real stack problem. This node's job is to find out with a working instrument
before anyone keeps, revises, or drops those 143 lines.

The Architect's structural argument, which this node must confirm or refute
rather than inherit: the record change adds no stack frame to nested-`match`
parsing, because `brace_starts_match_arms()` breaks the argument loop *before*
`parse_record_expr` is entered — what it adds on that path is a bounded forward
token scan, **time, not stack.**

## The second deliverable, which is a different subject in the same file

Architect finding 1, owed **before** any record-pattern node starts.

`brace_starts_match_arms` at `50da348a:crates/ken-elaborator/src/parser.rs:2067`
returns `true` on `MapsTo` and `false` on the terminator set
`{ Eq, Comma, Pipe, RBrace, Eof }`. It is correct **today** only because no
pattern token from that set can precede `MapsTo`.

**Record patterns in `match` break exactly that.** A first arm
`{ x, y } |-> …` hits `Comma` at offset 2, returns `false`, and the arm
classifies as a record literal. Record patterns are `34 §3`'s and are the
record-literal frame's own excluded scope — so this is a live trap for the
node that picks them up, not a hypothetical.

## Seating constraint, and it is load-bearing

**This node cannot be worked by an OpenAI-backed seat.** `language-implementer`
(`gpt-5.6-sol`) trips OpenAI's policy layer on stack-depth subjects — *"we take
extra caution with cybersecurity requests"* — and the refusal fires on the
**subject**, so it re-trips rather than clearing on retry. Assigning this node
to that seat burns the ring's turns without producing work.

## Not this node

- **Record patterns in `match`.** Only the invariant comment that protects
  them is here.
- **Any depth claim in either direction** before the fixed instrument runs.
- **Tuning `NESTED_MATCH_DEPTH`.** The constant is not what is broken.
