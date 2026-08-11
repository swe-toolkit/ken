---
id: LANG-LEX-NUMERIC-FORMS
title: "The lexer implements none of the numeric literal forms 31-lexical and 35-numbers list besides bare decimal -- no `1_000` separators, no `0xFF`/`0b1010`/`0o17` radix integers, no `0x1p-3` hex float -- and `1e-9`, which both spec tables give as the canonical Float example, does not lex as a float at all because the exponent branch is gated on having seen a dot"
status: merged
owner: language
size: M
gate: none
depends_on: [LANG-SURFACE-INT-PRECISION]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/1881
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

## Landed — `5f9a11f1`, PR #1881

Five non-merge commits from `24933da4`, two paths, `+128/-11`. Decision
`dec_1zav38jz0b6y3`. Separators, `0x`/`0b`/`0o` integers through `BigInt`, and
decimal exponent floats.

## What was not yet known, and how it came out

**Separators live in the scanner, per position, not in a normalisation step.**
Each of the integer, fractional, and exponent scans validates placement itself
under one rule — **a separator must sit between digits**. Trailing, doubled,
and post-sign placements refuse with spans.

**`0x1p-3` was NOT taken here, and the reason is mechanical rather than a
sizing preference.** Every other form in this arc ends by handing a string to
something that already parses it; hex floats have no such destination, since
`"0x1p-3".parse::<f64>()` is an error and there is no `from_str_radix` for
floats. That makes the deliverable a correctly-rounded conversion rather than a
scanner. It is [[LANG-LEX-HEX-FLOAT]], which this node unblocks.

**Placement adjacent to the prefix and the decimal point is settled by the
between-digits rule** — `0x_FF`, `1_.5`, `1._5` all fail it. **The rule is what
was controlled; those three spellings were not each named as controls on the
review record.**

## What this node repaired that its own frame did not predict

**Two silent wrong answers, both pre-existing, neither in the original cut.**

- **`3.14e5` lexed to `FloatLit(0.0)`**, as did `1.2e5`, `3.14E5`, `3.14e-2`.
  `exp_str` already held the `e` and the float branch formatted `"{}.{}e{}"`,
  so `"3.14ee5"` failed to parse and `unwrap_or(0.0_f64)` returned zero. **Both
  silent-zero sites are now span-bearing refusals.**
- **`1e2d` became `DecimalLit(1,0)` and `1e2f32` became `Float32Lit(1.0)`**,
  each dropping the exponent. Now lexical refusals.

**And one boundary that took two rounds.** After consuming `e+`,
`exp_str.len() == 2`, and a guard testing `len() <= 1` accepted `1e+_1` as
`FloatLit(10.0)`. **A sign is not a digit, and a length test on a buffer that
already contains the sign cannot say so.**

## Accepted limitation — exponent plus a `d`/`f32` suffix is refused

`3.14e5f32` and `1e2d` are **refused with a span**, by an explicit guard
(`lexer.rs:609-616`) that fires before both suffix branches. This is why the
`f32` construction legitimately omits `exp_str` — it is only ever entered with
that buffer empty.

**This is deliberate and it is a limitation, not a defect.** The alternative on
the table was the pre-existing behaviour, which dropped the exponent silently
and returned `DecimalLit(1,0)` or `Float32Lit(1.0)`. **A refusal beats a wrong
answer**, and adding correct suffix-exponent semantics was explicitly out of
the cut.

It is spec-consistent today: `31-lexical` and `35-numbers` give the `Float32`
row as `1.5f32`, with no exponent. **If either table ever gives an exponent on
a suffixed float, that guard is the thing to revisit** — recorded here so the
refusal is not re-filed as a defect.

## What is still not established

**`1e-9`'s post-repair value was never executed on the review record.** AC-1
was a *pre-edit* severity measurement, and it correctly reported the unrepaired
stream `Nat(1), Ident("e"), Minus, Nat(9)`. The cut then removed the `has_dot`
gate on the exponent scan, which should make `1e-9` a float for the first
time — but no reviewer executed it afterwards. **The frame reasoning says it
now lexes as `FloatLit(1e-9)`; nobody measured it.** [[LANG-LEX-HEX-FLOAT]]'s
AC-6 carries the obligation to measure and pin it.

## Not this node

`0x[deadbeef]` byte literals are `38-ffi-io`'s, not a numeric form. Do not fold
them in.
