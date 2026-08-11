---
id: LANG-LEX-NUMERIC-FORMS
title: "The lexer implements none of the numeric literal forms 31-lexical and 35-numbers list besides bare decimal -- no `1_000` separators, no `0xFF`/`0b1010`/`0o17` radix integers, no `0x1p-3` hex float -- and `1e-9`, which both spec tables give as the canonical Float example, does not lex as a float at all because the exponent branch is gated on having seen a dot"
status: ready
owner: language
size: M
gate: none
depends_on: [LANG-SURFACE-INT-PRECISION]
blocks: []
github: null
origin: Steward measurement 2026-08-11 at origin/main=90ce6743, taken after LANG-SURFACE-INT-PRECISION reported a precision-only cut. Its frame said the lexical forms share a spec line with the precision half but not a mechanism, and that the sibling would be filed once the cut was known. This is that filing.
---

## The gap

`spec/30-surface/31-lexical.md:506` and `spec/30-surface/35-numbers.md:230-233`
give the literal forms as a table. **The lexer implements the first column of
one row.**

| spec form | examples | lexer |
|---|---|---|
| integer | `0`, `42` | yes |
| integer, separators | `1_000` | **no** |
| integer, radix | `0xFF`, `0b1010`, `0o17` | **no** |
| decimal | `3.14d`, `0.1d` | yes |
| decimal, separators | `1_000.00d` | **no** |
| float, decimal point | `3.14` | yes |
| float, exponent | `1e-9` | **no — see below** |
| float, hex | `0x1p-3` | **no** |

Measured at `origin/main` = `90ce6743`: `crates/ken-elaborator/src/lexer.rs`
contains **no `from_str_radix` call and no digit-separator handling**. `_`
appears only in the identifier-continue predicate (`:159`) and the
identifier-start branch (`:436`). The number scanner is entered only on a
leading ASCII digit (`:420`), so `0xFF` begins as `0` and `xFF` becomes a
separate identifier token.

## `1e-9` is the one that is not merely missing

Both spec tables give `1e-9` as **the** canonical exponent-form `Float`
example. The lexer's exponent branch is gated on having already consumed a
fractional part:

```rust
// crates/ken-elaborator/src/lexer.rs
if has_dot && (self.cur() == Some('e') || self.cur() == Some('E')) {
```

`has_dot` is set only inside the fractional-part branch, which requires a `.`
followed by a digit. **So for `1e-9` the exponent is never scanned**, and the
token stream is `Nat(1)`, `Ident("e")`, `Minus`, `Nat(9)`.

**This is the axis that decides the node's severity, and it is unmeasured.**
`3.14e5` works, because the dot comes first. `1e-9` does not. Whether the
failure is loud depends on the surrounding context:

- If `e` is unbound, the result is an unbound-identifier error — **loud**, and
  this is a missing-feature node.
- **If `e` is bound in scope, `1 e - 9` is a well-formed expression** —
  application of `1` to `e`, minus `9`, or however the grammar associates it.
  Then a program written `1e-9` means something other than `0.000000001` **with
  no diagnostic at all**, which is a silent mis-parse and resizes the node.

`e` is a plausible binding name in numeric code. **Establishing which of these
actually happens is the first thing the frame will ask for**, derived
structurally here and not executed — the same discipline
[[LANG-LEX-PROJECTION-ADJACENCY]] used, where a structurally-derived severity
was confirmed by running it before any repair was authorised.

## Why it sequences after `LANG-SURFACE-INT-PRECISION`

Not courtesy. `0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF` is a radix integer that
exceeds `i128`, so the radix forms need the arbitrary-precision carrier to
land on. That carrier merged at `90ce6743`: `Token::IntLit` now carries
`num_bigint::BigInt` and the lexer's `i128` parse is gone.

⇒ **The radix work now has a target and did not before.** Both nodes also
edit `lex_numeric`, so they contend directly.

## What is not yet known

- Whether separators belong in the scanner or in a normalisation step before
  `parse`. The current integer path parses `int_str` directly into `BigInt`, so
  stripping `_` from that string is one line; the decimal and float paths build
  their strings separately and would each need it.
- Whether `0x1p-3` is worth taking with the radix integers. It shares the `0x`
  prefix and nothing else — the mantissa is hex, the exponent is binary and
  decimal-spelled. **It may be its own cut.**
- Whether a separator is legal adjacent to the radix prefix or the decimal
  point (`0x_FF`, `1_.5`, `1._5`). The spec says only *"underscores are digit
  separators and are ignored"* (`31 §3`), which does not settle placement.

## Not this node

`0x[deadbeef]` byte literals are `38-ffi-io`'s, not a numeric form. Do not fold
them in.
