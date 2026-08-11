---
id: LANG-SURFACE-DECIMAL-PRECISION
title: "`Decimal` is specified with an arbitrary-precision coefficient and the spec explicitly forecloses a fixed-width one, but the surface caps it at `i64` across three carriers -- `Token::DecimalLit(i64, i32)`, `NumLit::Decimal(i64, i32)`, and `NumericLitVal::Decimal { coeff: i64 }` -- and the lexer refuses a wider coefficient outright"
status: merged
owner: language
size: M
gate: none
depends_on: [LANG-SURFACE-INT-PRECISION]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/1876
origin: Steward measurement 2026-08-11 at origin/main=7f8a5e6b, taken while picking the Language successor to LANG-SURFACE-INT-PRECISION. The INT-PRECISION frame named Decimal's bounded mantissa as a real and separate gap and said to file it; this is that filing, with the spec grounding measured rather than assumed.
---

## The gap, and the spec already ruled against the current design

**`spec/30-surface/35-numbers.md:94-104` is decided and unambiguous.** A
`Decimal` is `coeff × 10^exp` with an **arbitrary-precision coefficient** and a
bounded `i32` base-10 exponent. `OQ-int` is DECIDED.

**The spec goes further than stating the rule — it names the wrong design and
rejects it**, at `:106-110`:

> **Reconcile note (don't cite the frame's struct).** An earlier draft and the
> WP frame floated a flat `{i128 coeff, i32 exp}`. The semantic requirement is
> an arbitrary-precision coefficient, not that representation. A small fast
> path is permitted, but no fixed-width coefficient may cap the value and no
> boxing/promotion scheme is observable.

⇒ **"No fixed-width coefficient may cap the value" is the exact property the
surface violates**, and it violates it at a narrower width than the one the
spec bothered to reject.

## Three carriers, all `i64`

`crates/ken-elaborator/src/lexer.rs:109`:

```rust
DecimalLit(i64, i32), // `d`-suffix: coeff × 10^exp; e.g. `0.1d` → (1,-1)
```

`crates/ken-elaborator/src/ast.rs:542`:

```rust
Decimal(i64, i32),
```

`crates/ken-elaborator/src/numbers.rs:43`:

```rust
Decimal { coeff: i64, exp: i32 },
```

**The cap is not silent, which is the one good thing here.** `lexer.rs:575`
refuses outright:

```rust
let coeff: i64 = coeff_str.parse().map_err(|_| ElabError::ParseError {
    msg: format!("decimal literal coefficient too large: {}", coeff_str),
    span: Span::new(start, self.pos),
})?;
```

So a wide `Decimal` literal is **rejected, not wrapped** — a loud refusal on a
program the spec says is well-formed. That bounds the severity: this is an
expressiveness gap, not a wrong-answer gap, and it is why the node is `M`
rather than a soundness item. **The same distinction the projection-adjacency
node turned on, and it should be measured here too rather than inherited.**

## Why this sequences after `LANG-SURFACE-INT-PRECISION`

Not a courtesy ordering. The two nodes contend on the same three files
(`lexer.rs`'s `lex_numeric`, `ast.rs`'s `NumLit`, `numbers.rs`'s
`NumericLitVal`) and on the same question — whether a payload widens in place
or a variant is added beside it. **Whatever answer `INT-PRECISION` returns for
`NumLit::Int`, this node should give the same answer for `NumLit::Decimal`**,
or state why the cases differ. Two literal classes in one enum reaching
arbitrary precision by two different mechanisms is the proliferation
`docs/PRINCIPLES.md` warns about.

`INT-PRECISION` also settles the upstream and downstream census that this node
inherits, including the `NumericLitVal` carrier the original frame missed
(`evt_72g51r95710eh`).

## The target already exists, and it is already arbitrary-precision

`crates/ken-kernel` contains no `Decimal` term — `grep -i decimal
crates/ken-kernel/src/` is empty — which initially reads as "no landed target".
**It is not.** `Decimal` is built in the elaborator as a transparent alias to a
two-field inductive whose fields are Ken's own `Int`
(`crates/ken-elaborator/src/decimal_char.rs:93-95`):

```rust
elab.elaborate_decl("data DecimalPair = MkDecimalPair Int Int")?;
elab.elaborate_decl("def Decimal = DecimalPair")?;
```

**Ken's `Int` is the arbitrary-precision one**, backed by
`Term::IntLit(num_bigint::BigInt)`. And the arithmetic is written over it: both
`decimalAdd` and `decimalEq` (`decimal_char.rs:141-183`) are Ken source over
`eq_int`, `mul_int`, `sub_int`, `leq_int` and the bounded `decimalPow10`
cascade, every one of which is `Int → Int`.

⇒ **This is the `IF`/`PAIR`/`INT` shape after all**, and that is the whole
sizing argument. The coefficient is arbitrary-precision from the constructor
inward; the cap exists only in the three surface carriers above it. Nothing in
the kernel, the pair, or the decimal arithmetic needs to change.

## Landed — `ebf24a82`, PR #1876

Merged 2026-08-11 from merge-base `0e5aba4e`; two commits, 33 paths, all under
`crates/`, `+135/-35`. Decision `dec_7ammj4fnjteg3`. The sizing argument held:
the three surface carriers widened in place to `BigInt`, the exponent stayed
`i32`, and nothing in the kernel, the pair, or the decimal arithmetic changed.

**The `AC-3` round is the part worth keeping.** The first cut built two
`MkDecimalPair` values by hand and compared them. That proves the target
comparator handles a wide `Int` — but it constructs the target directly, so it
never crosses the surface carrier this node widened, and it never reaches
`decimalAdd`. The Architect rejected it on exactly that: **a control can pass
on the property you care about while bypassing the mechanism you changed.** The
replacement is a surface expression,
`(9223372036854775808d + 1d) == 9223372036854775809d`, which forces both
operands and the result through surface `Decimal`, dispatches `+` to
`decimalAdd` and `==` to `decimalEq`, and evaluates to `Bool(true)`.

## What was not yet known, and how it came out

- **The `i32` exponent — unchanged, and correct, but the bound is still
  unpinned.** The exponent stayed `i32` as `35-numbers` requires, and a control
  now pins the `-30` case. **`-30` is nowhere near `i32`'s bound**, so the
  original concern — that the lexer computes the exponent as `-frac_places`
  from a `String` push loop with nothing pinning its behaviour *at the bound* —
  is not discharged. It was not in scope and is not a defect on any measured
  input; it is simply still open.
- **The durable big-`Decimal` encoding at tag `0x0A` (`35 §2.3`, cited to
  `41 §3a`) was not measured.** It was excluded from the cut and the
  Architect's sweep confirmed no durable-encoding path was touched — which
  establishes that this node did not disturb it, **not** that it is consistent
  with an arbitrary-precision coefficient. If that question is real it is still
  a separate node, and nothing here answers it.
- **`decimalPow10`'s `MAX_SHIFT` cascade was confirmed untouched.** The
  reasoning that it cannot interact — it bounds the **exponent** difference,
  not the coefficient — is structural rather than executed. Going STUCK beyond
  `MAX_SHIFT` remains a deliberate Architect ruling (`evt_7dwtqbmka62bf`)
  rather than a defect; **do not touch it.**

## Not this node

The digit separators and radix forms (`1_000.00d`, `0xFF`, `0b1010`, `0o17`)
are a sibling gap in the same function — the lexer implements none of them, and
`spec/30-surface/31-lexical.md:506` lists them on one line with the integer
forms. **They share a function with this node, not a mechanism.** They are
filed separately once `LANG-SURFACE-INT-PRECISION` reports which cut it took.
