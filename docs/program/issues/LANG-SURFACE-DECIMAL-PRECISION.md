---
id: LANG-SURFACE-DECIMAL-PRECISION
title: "`Decimal` is specified with an arbitrary-precision coefficient and the spec explicitly forecloses a fixed-width one, but the surface caps it at `i64` across three carriers -- `Token::DecimalLit(i64, i32)`, `NumLit::Decimal(i64, i32)`, and `NumericLitVal::Decimal { coeff: i64 }` -- and the lexer refuses a wider coefficient outright"
status: ready
owner: language
size: M
gate: none
depends_on: [LANG-SURFACE-INT-PRECISION]
blocks: []
github: null
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

## What is not yet known

- Whether the `i32` exponent is correct as landed. The spec says the exponent
  **is** bounded `i32`, so it likely needs no change — but the lexer computes
  it as `-frac_places` from a `String` push loop, and no control pins its
  behaviour at the bound.
- Whether the durable big-`Decimal` encoding at tag `0x0A` (`35 §2.3`, cited to
  `41 §3a`) implies work outside `crates/ken-elaborator`. **If it does, that is
  a separate node and not this one.**
- Whether `decimalPow10`'s `MAX_SHIFT` cascade interacts with a wide
  coefficient. It bounds the **exponent** difference, not the coefficient, and
  going STUCK beyond `MAX_SHIFT` is a deliberate Architect ruling
  (`evt_7dwtqbmka62bf`) rather than a defect — **do not touch it**. Named here
  only so it is not rediscovered as a blocker.

## Not this node

The digit separators and radix forms (`1_000.00d`, `0xFF`, `0b1010`, `0o17`)
are a sibling gap in the same function — the lexer implements none of them, and
`spec/30-surface/31-lexical.md:506` lists them on one line with the integer
forms. **They share a function with this node, not a mechanism.** They are
filed separately once `LANG-SURFACE-INT-PRECISION` reports which cut it took.
