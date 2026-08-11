---
id: LANG-LEX-HEX-FLOAT
title: "Both spec literal tables give `0x1p-3` as a `Float` form, but the lexer has no hex-float path at all -- and unlike every other numeric form in this arc it cannot be reached by handing a string to `parse::<f64>()`, because Rust's float parser rejects hex-float syntax, so the value must be assembled and correctly rounded by hand"
status: merged
owner: language
size: M
gate: none
depends_on: [LANG-LEX-NUMERIC-FORMS]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/1885
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

## Landed

Merged at exact `9dfb3cdc8e5752247ce207a9ebb7f1bd9918e43e`, PR #1885,
`origin/main` `a28a7a33`. Two paths — `crates/ken-elaborator/src/lexer.rs` and
`crates/ken-elaborator/tests/lang_lex_hex_float.rs`, `+94/-8` from base
`44a935c5`. No `spec/` edit, no new kernel term, `trusted_base()` unchanged.

## How the three open questions came out

**The mantissa goes through `BigInt`, and the bounded-integer route was not
merely slower — it is wrong.** The frame declined to pre-empt this and that was
right, but the choice turned out to have a correctness answer rather than a
performance one. The value is assembled by exact `BigInt` bit assembly with a
guard/sticky nearest-ties-even round, covering normals, subnormals, and
underflow. **The discriminating control is a `2^1024` mantissa at exponent
`-1024`, whose exact result is `1.0`**: the rejected `mantissa -> f64 -> scale`
approach overflows in the intermediate conversion *before* compensation, so it
fails precisely where the exact assembly passes. A tie/above-half net alone
does not separate the two — the Architect rejected an earlier control for
exactly that non-discrimination.

**The `p` exponent is REQUIRED.** Decided, as the node said it must be. `0x1.8`
alone is not a hex float. Classification is an explicit mantissa/exponent state
machine, and `0x[deadbeef]` bytes and `0b`/`0o` integers are untouched by it.

**`Float32` gets no hex form.** `0x1p-3f32` is not a supported spelling,
matching the node's expectation that absent a spec line the answer is no.

## What the frame got wrong about where the difficulty was

The frame told the ring to treat the scanning as the easy half and spend the
effort on the conversion. **That was backwards.** The conversion landed in
round one and was never rejected. **All four Architect rejections were the
scanner:** classification scanning past the current token so a later `p` or `.`
could change `0b10`/`0o7` behaviour; mantissa separators not strictly
hex-digit-adjacent across the fractional boundary (`0x1._8p0`); the colon
boundary (`0xFF:p`, `0xFF:x.1`); and pre-exponent signs failing to terminate
the candidate, so `0xFF+p` had to be pinned as `Nat(255), Plus, Ident("p"),
Eof`.

The generalisation, carried into `LANG-SURFACE-RECORD-DECL` as an acceptance
criterion rather than as advice: **a new surface form is risky in proportion to
how many neighbouring constructs it sits beside, not to how intricate its own
internals are.**

## Two values that were repaired by the sibling and had never been executed

The merge was **held** at `fcc4b2c9` on this alone. `1e-9` is one of the spec's
three float examples and did not lex as a float at all until
`LANG-LEX-NUMERIC-FORMS` removed the `has_dot` gate on the exponent. Its
post-repair value had been reasoned about twice and run zero times. Both are
now committed, executed assertions: `1e-9` is `FloatLit(1e-9)`, and `3.14e5` is
`FloatLit(314000.0)` where it previously lexed to `FloatLit(0.0)`.

The `Token::FloatLit(f64)` doc comment, which said "decimal-point float" while
listing `1e-9` among its examples, now reads `decimal or hexadecimal f64:
`3.14`, `1e-9`, `0x1p-3`` at `lexer.rs:109`.

## What is still not established — the tie DIRECTION is uncontrolled

**The control set cannot distinguish nearest-ties-even from
nearest-ties-toward-zero.** Measured by the Adversary at `evt_70mrxth1wmx9c`,
on `a28a7a33`, after this node merged.

The rounding evidence is better than the merge notification claimed, and in a
different place. `2^1024 @ -1024 -> 1.0` is **exact**, and an exact result
agrees under every rounding mode — it discriminates against the rejected
intermediate-`f64` overflow path and against nothing else. The real evidence is
a proper pair:

| control | value | asserted |
|---|---|---|
| `0x100000000000008p-56` | `1 + 2^-53`, exactly halfway | `1.0` — tie resolves down, to the even mantissa |
| `0x100000000000009p-56` | a hair above the tie | `1.0000000000000002` — rounds up |

That pair pins round-to-nearest and rules out truncation of non-tie values,
which a single exact case cannot. `0x1p-1075 -> 0.0` is a second tie, at the
underflow boundary.

**The gap: both ties resolve toward zero, and in both the even neighbour IS the
lower one.** So ties-even and ties-toward-zero produce identical results on
every control present. The distinguishing case is a tie whose even neighbour is
the **upper** one — `0x100000000000018p-56` = `1 + 3*2^-53`, which ties-even
rounds up to `1.0000000000000004` while ties-toward-zero gives
`1.0000000000000002`.

**This is not a defect claim.** There is no evidence the implementation is
wrong, and nearest-ties-even is very likely what it does. The claim is that the
control set cannot tell, because every instance chosen is one where the
distinction collapses.

**Routed as a named deliverable with its own AC in Language's next frame**, not
left here as prose — prose in a merged node is a claim about the past that no
gate reads. The Adversary's own bound is carried with it: it read the controls
and not the conversion, so **if the bit assembly makes an up-tie structurally
unreachable, the deliverable is discharged by demonstrating that**, not by
adding an assertion that cannot fail.

## Not this node

`0x[deadbeef]` byte literals are `38-ffi-io`'s. They share the `0x` prefix and
nothing else, and both this node and its sibling exclude them.
