---
id: LANG-LEX-HEX-FLOAT
title: "Both spec literal tables give `0x1p-3` as a `Float` form, but the lexer has no hex-float path at all -- and unlike every other numeric form in this arc it cannot be reached by handing a string to `parse::<f64>()`, because Rust's float parser rejects hex-float syntax, so the value must be assembled and correctly rounded by hand"
status: ready
owner: language
size: M
gate: none
depends_on: [LANG-LEX-NUMERIC-FORMS]
blocks: []
github: null
origin: Deferred out of LANG-LEX-NUMERIC-FORMS by language-leader at evt_5a1hz938c696q, 2026-08-11, on the cut the frame invited it to declare. That frame said hex floats share only the `0x` prefix with the radix integers and are "probably its own node -- say so rather than growing this one"; the ring said so. Filed rather than left as prose in a merged frame.
---

## The gap

`spec/30-surface/31-lexical.md:508` and `spec/30-surface/35-numbers.md:232`
both give the `Float` row as:

| Literal | Examples | Default type |
|---|---|---|
| **float** | `3.14`, `1e-9`, `0x1p-3` | `Float` (IEEE f64) |

**`0x1p-3` is absent from the lexer entirely.** Measured at `origin/main` =
`47a0b791`: `lex_numeric` is entered only on a leading ASCII digit
(`crates/ken-elaborator/src/lexer.rs:431-433`), and it scans decimal digits,
one optional `.`-fraction, and an optional `e`/`E` exponent. There is no `p`
handling and no hex-digit handling anywhere in it.

So `0x1p-3` lexes as `Nat(0)`, `Ident("x1p")`, `Minus`, `Nat(3)` — the same
shape as the other missing radix forms, and loud for the same reason: `x1p` is
not bound.

## Why this is not just another row of the same table

The rest of the numeric-forms table is string manipulation feeding an existing
parser. `1_000` strips separators and hands the result to `BigInt::parse`;
`0xFF` hands it to `from_str_radix`; `1e-9` hands it to `f64::parse`.

**Hex floats have no such destination.** Rust's `str::parse::<f64>()` does not
accept hex-float syntax — `"0x1p-3".parse::<f64>()` is an error, not `0.125`.
There is no `from_str_radix` for floats. ⇒ **The value has to be assembled and
rounded by hand**, and that is the whole reason this is a separate node rather
than a fourth branch in the sibling.

That makes the real deliverable a **rounding obligation**, not a lexing one:

- `0x1p-3` is exactly `0.125` and `0x1.8p3` is exactly `12.0`, so the easy
  cases tempt a naive implementation that is exact on everything anyone tries
  by hand.
- A hex mantissa can carry more than 53 significant bits, at which point the
  result must be **correctly rounded to nearest, ties to even** — the same
  rounding every other `Float` literal already gets for free from
  `f64::parse`.
- `(mantissa as f64) * 2f64.powi(exp)` is exact only while the mantissa fits
  in 53 bits and the scaling neither overflows nor goes subnormal. **It is
  wrong in exactly the cases a hand-written test will not contain.**

## Why it sequences after `LANG-LEX-NUMERIC-FORMS`

Not courtesy, and not the arbitrary-precision reason the earlier nodes had.
**The sibling creates the `0x` prefix branch** that this node has to extend.
Building a second, independent `0x` entry point would put two scanners on one
prefix, and the spec already loads that prefix three ways
(`spec/30-surface/38-ffi-io.md:84`):

| source | form | owner |
|---|---|---|
| `0xFF` | un-bracketed hex integer | `LANG-LEX-NUMERIC-FORMS` |
| `0x1p-3` | hex float, binary exponent | **this node** |
| `0x[deadbeef]` | bracketed `Bytes` | `38-ffi-io`, neither node |

**The three-way disambiguation is the interesting part of the lexing work**,
and it only exists once the first of the three lands.

## The float path this inherits is being repaired, not preserved

`LANG-LEX-NUMERIC-FORMS` carries a wrong-answer repair this node depends on.
Measured by execution on `47a0b791`: **`3.14e5` lexes to `FloatLit(0.0)`**, as
do `1.2e5`, `3.14E5`, and `3.14e-2`. `exp_str` already contains the `e`
(`lexer.rs:555`), and the float branch formats `"{}.{}e{}"` at `:610`,
producing `"3.14ee5"`; the parse fails and `unwrap_or(0.0_f64)` swallows it.

⇒ **Do not copy the existing float branch as a model.** Both `unwrap_or(0.0)`
sites (`:600`, `:610`) are in the sibling's scope, and this node should land
on a float path where a malformed literal is a refusal with a span rather than
a silent zero. **If it is not repaired by the time you start, that is a stop,
not something to work around** — a new form built on a swallowing parser
inherits the swallow.

## What is not yet known

- Whether the mantissa should be assembled through `BigInt` (already a
  dependency of this crate after `LANG-SURFACE-INT-PRECISION`) or through a
  bounded integer with an explicit sticky-bit round. The first is obviously
  correct and probably slower; the second is the conventional implementation.
  **This is a real design choice and the frame should not pre-empt it.**
- Whether a hex float requires the `p` exponent or merely permits it. The spec
  gives one example, `0x1p-3`, and does not say whether `0x1.8` alone is a
  float, an error, or an integer followed by something. **C requires the
  exponent precisely because `0x1.8` is otherwise ambiguous**, and Ken has the
  same ambiguity. This needs deciding, not discovering.
- Whether `Float32` gets a hex form. `1.5f32` exists; `0x1p-3f32` is not in
  either spec table. Absent a spec line, the answer is probably no.

## Not this node

`0x[deadbeef]` byte literals are `38-ffi-io`'s. They share the `0x` prefix and
nothing else, and both this node and its sibling exclude them.
