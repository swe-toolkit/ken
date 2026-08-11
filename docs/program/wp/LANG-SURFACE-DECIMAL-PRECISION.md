# LANG-SURFACE-DECIMAL-PRECISION — the surface reaches `Decimal`'s exact coefficient

Owner: language. Size: M. Node: [[LANG-SURFACE-DECIMAL-PRECISION]].
Fixed inputs measured at `origin/main` = **`7f8a5e6b`**. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if
your diff stays in `crates/` — and it should, because the spec already says
what this node implements, in a paragraph written specifically to reject the
design that is landed.

> ## GATE — [[LANG-SURFACE-INT-PRECISION]] must be on `main` first
>
> **This is a real dependency, not an ordering courtesy.** The two nodes touch
> the same three declarations in the same three files, and they answer the same
> design question. Starting this against an unmerged `INT-PRECISION` branch
> means resolving that question twice and merging the answers by hand.
>
> More importantly: **`INT-PRECISION` sets the precedent this node must
> follow.** Read its landed diff before you write anything.

## What this deliverable is

`Decimal` is specified with an **arbitrary-precision coefficient**, and its
target is already arbitrary-precision: `data DecimalPair = MkDecimalPair Int
Int` (`decimal_char.rs:93`), over Ken's own `Int`, itself backed by
`Term::IntLit(num_bigint::BigInt)`. **The surface caps it at `i64` in three
carriers and refuses anything wider.** Remove the cap.

**This is the fourth node of the shape** after `LANG-SURFACE-IF`,
`LANG-SURFACE-PAIR` and `LANG-SURFACE-INT-PRECISION`: a spec production whose
target already exists and already works, reachable only through a surface that
does not admit it.

## The spec did the design work, and it did it against you

`spec/30-surface/35-numbers.md:106-110` is a **reconcile note that names a WP
frame's `{i128 coeff, i32 exp}` and rejects it**:

> no fixed-width coefficient may cap the value and no boxing/promotion scheme
> is observable

So three things are settled before you start, and none of them is yours to
re-open:

- **`i64` is not a smaller version of the right answer.** The spec rejected a
  *wider* fixed-width coefficient than the one that is landed.
- **A small-coefficient fast path is permitted** — the spec says so explicitly
  — **but it must not be observable.** If a program can tell which
  representation it got, the fast path is a semantic feature and it is out of
  scope here.
- **The exponent stays bounded `i32`.** That is the spec's own word. Do not
  widen it, and do not treat the coefficient change as licence to revisit it.

## The design call: follow `INT-PRECISION`, or say why not

`INT-PRECISION` settles whether `NumLit`'s payload widens in place or a new
variant is added beside it. **Give `NumLit::Decimal` the same answer.** Two
literal classes in one enum reaching arbitrary precision by two different
mechanisms is exactly the proliferation `docs/PRINCIPLES.md` warns about, and a
reviewer will ask.

**If the cases genuinely differ, that is a legitimate outcome** — `Decimal`
carries two fields where `Int` carries one, and only one of them widens. Say so
in one sentence in the candidate rather than leaving the divergence to be
inferred from the diff.

## The three carriers

1. `crates/ken-elaborator/src/lexer.rs:109` — `Token::DecimalLit(i64, i32)`
2. `crates/ken-elaborator/src/ast.rs:542` — `NumLit::Decimal(i64, i32)`
3. `crates/ken-elaborator/src/numbers.rs:43` —
   `NumericLitVal::Decimal { coeff: i64, exp: i32 }`

**Carrier 3 is the one `INT-PRECISION`'s original frame missed for `Int`**
(`evt_72g51r95710eh`), and it is on the evaluation path rather than the kernel
path. **Do not repeat that.** The census above is what I measured; re-derive it
rather than trusting it, and state your census in the candidate.

The refusal to remove is `lexer.rs:575`:

```rust
let coeff: i64 = coeff_str.parse().map_err(|_| ElabError::ParseError {
    msg: format!("decimal literal coefficient too large: {}", coeff_str),
    ...
```

## Deliverables

**1. The cap is gone from all three carriers, and unrepresentable.** Not
widened to `i128` — the spec rejected that width by name.

**2. Both exits measured, not just the kernel one.** A wide `Decimal` literal
must reach the kernel as an `MkDecimalPair` application with the written
coefficient, **and** evaluate to the right value through the interpreter path.
These are different code paths and one can be right while the other truncates.

**3. The literal-to-pair construction traced and stated.** Say how a
`NumLit::Decimal` becomes a `MkDecimalPair` term today and what changed. It
runs through `elab.rs:3690` (checked, at `decimal_id` or `decimalpair_id`) and
`elab.rs:3759` (default). **If both paths need the change and you only found
one, `AC-1` will pass on whichever you fixed.**

**4. The consumer census.** Every site matching `DecimalLit`,
`NumLit::Decimal`, or `NumericLitVal::Decimal`, including `layout.rs:1939` and
`:1960`, `parser.rs:1978` and `:2271-2274`. `NumLit` is public, so the sweep
crosses crates.

## Acceptance criteria

**AC-1 — a coefficient wider than `i64` round-trips, through both exits.**
State the literal. It must reach the kernel with the written coefficient and
evaluate to the written value. **This is the node.**

**AC-2 — the A/B.** With the `i64` parse restored, the AC-1 literal must be
**refused with the exact "decimal literal coefficient too large" message**;
with it removed, accepted with the written value. This is a cleaner A/B than
`INT-PRECISION`'s because the current failure is a named refusal rather than a
silent wrap — **use that, do not weaken it to a value comparison.**

**AC-3 — exactness is decided by the target, not by a comparator you wrote.**
`0.1d + 0.2d == 0.3d` must hold (`35 §2.3` AC6) with a wide coefficient in
play, decided through `decimalEq`. **If you wrote the comparator, it is not
this criterion.**

**AC-4 — the narrow path is unchanged.** `0.1d`, `19.99d`, `3.14d` keep their
exact `(coeff, exp)` pairs and their existing behaviour. **This is the half a
payload change silently breaks**, and it is the analogue of `INT-PRECISION`'s
`Zero`/`Succ` control.

**AC-5 — the exponent is untouched and pinned.** A control shows `i32`
exponent behaviour is the same before and after. You are changing one field of
a two-field carrier; say so with a test rather than with a sentence.

**AC-6 — no observable representation.** If you add a small-coefficient fast
path, no program can distinguish it. **Simplest way to satisfy this is not to
add one** — the spec permits it, nothing requires it, and Ken has no users.

**AC-7 — `trusted_base()` unchanged, and no `spec/` edit.** The target already
exists and the spec already says this.

## Excluded scope

- **No kernel change, no new kernel term.** `MkDecimalPair Int Int` is the
  target and it is sufficient.
- **No `decimalPow10` / `MAX_SHIFT` work.** Going STUCK beyond `MAX_SHIFT` is a
  deliberate Architect ruling (`evt_7dwtqbmka62bf`) — a structurally honest
  incompleteness, not a defect. It bounds the exponent difference, not the
  coefficient.
- **No digit separators or radix forms.** `1_000.00d`, `0xFF`, `0b1010`, `0o17`
  live in the same function and are a separate node.
- **No `Float`/`Float32` change**, no numeric tower, no `Decimal`/`Float`
  conversion work, no performance work, no new surface production.
- **No `0x0A` durable-encoding work.** If you find it forces a change here,
  that is a stop.

## Stop conditions — return to me, do not decide

- **The `0x0A` durable encoding forces work outside `crates/ken-elaborator`.**
- **A consumer genuinely requires a fixed-width coefficient.** Design fork for
  the Architect, not something to work around with a cast.
- **The wide coefficient cannot reach `MkDecimalPair`** without a kernel or
  prelude change. That would mean the target is not what this frame measured
  and the sizing is wrong.
- **`INT-PRECISION` returned an answer you cannot follow** and you believe the
  cases genuinely differ. Say which and why before building the divergence.

## Contention

`crates/ken-elaborator/src/{lexer.rs, ast.rs, numbers.rs, elab.rs, parser.rs}`.
**Re-derive the intersection at candidate time** — Runtime is in the cranelift
backend, but `main` moves and a merge-base goes stale without your branch
moving.

## Sizing and validation

The target exists and the arithmetic is already `Int`-typed, so this is a
carrier change plus its controls. **`NumLit` is public** — run
`scripts/ken-cargo check -p ken-cli -p ken-interp -p ken-verify` on top of the
targeted `scripts/ken-cargo test -p ken-elaborator` floor. **Never
`--workspace`**; that is CI's gate.

**If you are past an hour and still building carriers, stop and tell me** — the
likely cause is that the fast-path question got opened, and `AC-6`'s cheapest
answer is not to open it.
