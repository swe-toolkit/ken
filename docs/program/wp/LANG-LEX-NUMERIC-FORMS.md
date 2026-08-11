# LANG-LEX-NUMERIC-FORMS — the rest of the numeric literal table

Owner: language. Size: M. Node: [[LANG-LEX-NUMERIC-FORMS]].
Fixed inputs measured at `origin/main` = **`90ce6743`**. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

> ## THE CONTENTION IS GONE. [[LANG-SURFACE-DECIMAL-PRECISION]] LANDED at
> ## `ebf24a82` (PR #1876), 2026-08-11.
>
> This frame was written while both nodes were live on `lex_numeric` and told
> you to sequence with your leader. **That is settled: Decimal went first and
> is on `main`.** You have the function to yourself.
>
> **What changed under you, and it touches the separators piece:**
> `Token::DecimalLit`, `NumLit::Decimal`, and `NumericLitVal::Decimal` now
> carry `num_bigint::BigInt` rather than `i64`. The exponent is still `i32`.
> Combined with the `Int` carrier from `LANG-SURFACE-INT-PRECISION`, **every
> numeric literal path you would strip a separator out of now parses into an
> arbitrary-precision carrier**, so `1_000_000_000_000_000_000_000.00d` has
> somewhere to land.
>
> ⇒ **`lex_numeric` has now changed three times this week, not twice. Read the
> landed function before you plan anything** — this frame's code descriptions
> are the oldest thing in it.

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if
your diff stays in `crates/` — the spec already gives the forms as a table.
If you want to edit `spec/`, that is a stop.

> ## GATE — [[LANG-SURFACE-INT-PRECISION]] must be on `main` first
>
> **Satisfied as of `90ce6743`**, and it is a real dependency rather than an
> ordering courtesy: `0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF` exceeds `i128`, so
> the radix forms need the arbitrary-precision carrier to land on.
> `Token::IntLit` now carries `num_bigint::BigInt` and the lexer's `i128`
> parse is gone. **Read the landed `lex_numeric` before you plan anything** —
> it has changed twice this week.

## What this deliverable is

`31-lexical.md:506` and `35-numbers.md:230-233` give the numeric literal forms
as a table. **The lexer implements bare decimal integers, decimal-point
floats, and the `d`/`f32` suffixes. Everything else in that table is absent.**

This is the fourth node of the surface-catches-up shape, but it differs from
`IF`/`PAIR`/`INT-PRECISION` in one way worth stating: **those three had a
landed target waiting. This one is lexing work with no downstream target to
reach** — a `0xFF` becomes the same `Token::IntLit` a `255` does. That makes it
simpler, not harder, and it is why the design section below is short.

## AC-1 comes first and may resize the node

**`1e-9` does not lex as a float.** The exponent branch is gated on `has_dot`,
which is set only by the fractional-part branch, so the token stream is
`Nat(1)`, `Ident("e")`, `Minus`, `Nat(9)`.

**Measure what that produces before you fix anything**, in two contexts:

1. with `e` **unbound** — expected: an unbound-identifier error, loud;
2. with `e` **bound** in scope — **if `1 e - 9` elaborates to a well-formed
   expression, this node is a silent mis-parse rather than a missing feature.**

**A silent mis-parse is a stop condition.** Come back to me: the node is then
mis-sized, its severity is wrong, and the sequencing is mine to redo. **Do not
repair it and report the severity afterwards** — the ordering is the point, and
it is the ordering `LANG-LEX-PROJECTION-ADJACENCY` used to good effect.

> **NARROWED 2026-08-11, and I expect the stop NOT to fire. Still measure it.**
>
> I wrote above that `1 e - 9` "may be a well-formed expression" if `e` is
> bound. **That was a claim about the token stream that I did not check against
> the consumer one hop down**, which is the same error the Adversary recorded
> about itself on `LANG-SURFACE-PAIR` — asserting "silent" from the producer
> without asking what happens next.
>
> Checked: **`e` is not in the prelude roster**, so with no user binding the
> result is an unbound-identifier error. And if a user *does* bind `e`,
> application binds tighter than `-`, so the expression is `(1 e) - 9` — an
> application whose head is a numeric literal. `elab.rs:3166` raises
> `ElabError::NotAFunction` when an application head is not a Pi, and `Int` is
> not.
>
> ⇒ **Loud in both contexts, so the silent-mis-parse class looks empty and this
> is a missing-feature node.** Derived structurally and **not executed** — the
> same standing as the original claim, which is exactly why `AC-1` stays a
> measurement rather than becoming an assumption. **If you measure a silent
> path, the stop still fires and I still want it.**

## The cut is yours, and I expect you to take fewer than four

Four things are in the table. **They do not share a mechanism and you should
not assume one node takes all of them.**

| piece | where it lives | note |
|---|---|---|
| **separators** `1_000`, `1_000.00d` | every numeric path | the integer path parses `int_str` straight to `BigInt`, so stripping there is one line; the decimal and float paths build their strings separately |
| **radix integers** `0xFF`, `0b1010`, `0o17` | a new prefix branch before the digit loop | needs `from_str_radix` into `BigInt`; **this is the piece the gate exists for** |
| **exponent floats** `1e-9` | the `has_dot` gate | a one-condition change, but see AC-1 |
| **hex floats** `0x1p-3` | shares only the `0x` prefix | hex mantissa, binary exponent, decimal-spelled. **Probably its own node — say so rather than growing this one.** |

**State which pieces you took and which you left.** Under the accepted-partial
policy each complete piece is a candidate the moment it is reviewed; I would
rather take separators-plus-exponent now and radix next than hold a whole
table. **If you split, say so and I frame the sibling** rather than leaving it
as prose in a merged frame.

## The design question you do have to answer

**Where separators are stripped.** In the scanner as it consumes, or in a
normalisation pass on the accumulated string before `parse`. The second is
smaller and the first gives better spans on a malformed literal.

**Placement is undecided by the spec and it is yours to settle.**
`31 §3` says only *"underscores are digit separators and are ignored"*. That
does not say whether `0x_FF`, `1_.5`, `1._5`, `1__0`, or a trailing `1_` are
legal. **Pick a rule, state it in one sentence, and control it** — the failure
this program keeps hitting is a lexical decision that falls out of an
implementation rather than being decided.

**Recommendation: separators only between digits.** It is the common rule, it
makes every case above a rejection, and it is stateable without naming the
scanner.

## Acceptance criteria

**AC-1 — the `1e-9` severity is measured, not inherited**, in both the
`e`-bound and `e`-unbound contexts. A silent mis-parse is a **stop**. State the
exact token stream and the resulting diagnostic or AST.

**AC-2 — every form you claim has a positive and a negative.** `0xFF` is 255
and `0xGG` is refused; `1_000` is 1000 and your chosen bad placement is
refused. **A form with no rejection control is a form whose boundary is
untested.**

**AC-3 — the existing forms keep working.** `3.14`, `3.14e5`, `1.2d`, `1.5f32`,
`42`, and the positional-projection seam `p.1.2` / `p. 1.2` all unchanged.
**That last one matters**: `LANG-LEX-PROJECTION-ADJACENCY` put a
previous-emitted-token guard in the same function, and a new prefix branch
that runs before it can strip the context it depends on.

**AC-4 — a radix literal exceeding `i128` round-trips**, if you take the radix
piece. It is the reason this node is gated, so leaving it untested wastes the
gate.

**AC-5 — the A/B.** For each piece, disable your branch and show the
motivating literal fails; restore and it passes.

**AC-6 — no `spec/` edit, no new surface production, `trusted_base()`
unchanged.**

## Excluded scope

- **`0x[deadbeef]` byte literals** — that is `38-ffi-io`, not a numeric form.
- **No `Decimal` coefficient work.** [[LANG-SURFACE-DECIMAL-PRECISION]] already
  took the mantissa and it is on `main`, so the coefficient is no longer yours
  to widen — but **the collision that clause was hedging against cannot happen
  any more.** Separators in `1_000.00d` are now a lexical question about one
  landed carrier, not a race with a concurrent one.
- No numeric tower, no `Float`/`Float32` semantics change, no overflow work,
  no conversion API, no performance work.

## Stop conditions — return to me, do not decide

- **AC-1 finds a silent mis-parse.**
- **A piece needs a parser change.** This is lexical on purpose.
- **Hex floats turn out to need the float path restructured.** That is the
  sibling, not this node.
- ~~**Separators collide with `DECIMAL-PRECISION`'s carrier change.**~~
  **RETIRED — this stop can no longer fire.** It was a concurrency stop, and
  the carrier change landed at `ebf24a82`. A separator problem in `1_000.00d`
  is now ordinary work inside this node, not a reason to come back to me.

## Contention

`crates/ken-elaborator/src/lexer.rs`, and specifically `lex_numeric`. **The
contention with [[LANG-SURFACE-DECIMAL-PRECISION]] is discharged** — it landed
at `ebf24a82` and the function is free. Runtime is in `crates/ken-runtime/`.
**Re-derive the intersection at candidate time** — a merge-base goes stale
without your branch moving.

## Sizing and validation

`scripts/ken-cargo test -p ken-elaborator` plus the focused lexer suite.
**`Token` is crate-internal but `NumLit` is public** — if you change neither
enum's shape, the `ken-cli`/`ken-interp`/`ken-verify` sweep does not apply and
you should say so rather than running it out of habit. **Never `--workspace`**;
that is CI's gate.

**If you are past an hour, you took too many pieces.** Land one and file the
rest.
