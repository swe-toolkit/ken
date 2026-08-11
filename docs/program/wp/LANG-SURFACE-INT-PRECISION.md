# LANG-SURFACE-INT-PRECISION — the surface reaches the kernel's bignum

Owner: language. Size: M. Node: [[LANG-SURFACE-INT-PRECISION]].
Fixed inputs measured at `origin/main` = **`ae6f750a`**. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if
your diff stays in `crates/` — and it should, because the spec already says
what this node implements. If you find yourself wanting to edit `spec/`, that
is a stop, not a deliverable.

## What this deliverable is

`Int` is specified arbitrary-precision, `OQ-int` is DECIDED, and
`Term::IntLit(num_bigint::BigInt)` is landed in the kernel. **The surface
truncates it to `i128`.** This node removes the truncation.

**This is the third node of the same shape**, after [[LANG-SURFACE-IF]] and
[[LANG-SURFACE-PAIR]]: a spec production whose target already exists and
already works, reachable only through a surface that does not admit it. That
shape has been the cheapest reliable Language work on this program, and it is
why an arbitrary-precision change is `M` and not a numeric-tower epic.

## The design call, front-loaded

**The representation is not open.** Use the kernel's `num_bigint::BigInt`.
Do not introduce a second bignum type, a wrapper, or an `enum { Small(i64),
Big(..) }` at the surface. A small-value fast path is a **performance**
question about a path nobody has measured, and Ken has no users; the whole
point of the kernel already carrying `BigInt` is that the surface has a target
to name.

**What IS open, and you should settle it before writing:** whether
`NumLit::Int` changes its payload type in place, or a new variant is added
beside it. In-place is the smaller diff and makes the truncation
unrepresentable; a new variant leaves a fixed-width path alive that nothing
forces anyone to stop using. **I recommend in place, on
subsume-don't-proliferate** (`docs/PRINCIPLES.md`) — but `NumLit` derives
`PartialEq` and `Copy`-adjacent use may exist at the five match sites below, so
measure before committing to it.

## The cut, and it is yours to make

Two halves sit behind one spec line (`31-lexical.md:506`):

1. **Precision** — `i128` to `BigInt`, and the lossy `n as i128`.
2. **Lexical forms** — `0xFF`, `0b1010`, `0o17`, and `1_000` separators, none
   of which the lexer implements.

**Take (1) alone if (2) turns out to be its own node.** They share a line in
the spec, not a mechanism. Under the accepted-partial policy a complete (1) is
a candidate the moment it is reviewed — do not hold it for (2). **State which
cut you took in the candidate**, and if you split, say so and I will frame the
sibling rather than leaving it as prose in a merged frame.

## Deliverables

**1. The truncation is gone, and unrepresentable.** `NumLit::Int` carries an
arbitrary-precision value and `parser.rs:2254`'s `n as i128` is removed rather
than widened.

**2. The width question in BOTH directions, answered.** Measure whether
`Token::Nat` is itself width-bounded before the cast. **If it is, the lexer is
the first truncation site and the cast is a symptom** — fix both or say plainly
that one remains. A candidate that removes the cast while the lexer still
cannot lex a large literal has moved the defect, not closed it.

> **AMENDED 2026-08-11 (`evt_72g51r95710eh`), mid-turn. This deliverable
> originally said "upstream" and pointed only at `Token::Nat`. That was a gap
> in the frame: there is a second `i128` carrier BELOW `NumLit`, on a different
> path than the one `AC-1` measures.**
>
> ```rust
> // crates/ken-elaborator/src/numbers.rs:39-43
> pub enum NumericLitVal { Int(i128), ... }
>
> // crates/ken-elaborator/src/numbers.rs:564
> pub fn int_lit_val(n: i128, ty: &Term, nenv: &NumericEnv) -> NumericLitVal
> ```
>
> **The two exits diverge.** The kernel path converts at `elab.rs:3616` —
> `Term::IntLit(num_bigint::BigInt::from(*n))` — so once `NumLit::Int` carries
> a `BigInt` that path is correct and **`AC-1` passes**. The evaluation path
> does not go through there: `elab.rs:3757` builds `NumericLitVal::Int(*n)` and
> `ken-cli/src/lib.rs:443` turns it into `ken_interp::EvalVal::from(*n)`, still
> `i128`. **So a candidate can satisfy `AC-1` exactly as written while the same
> literal evaluated through the CLI gives a wrapped answer.** The specified
> consumer sweep does not catch it either — `check -p ken-cli` type-checks the
> `EvalVal::from` call and says nothing about the value.
>
> `NumericLitVal::Int` and `int_lit_val`'s `n: i128` parameter are **in scope
> for this deliverable**. Widen them with the rest, or state plainly that one
> carrier remains and which programs still truncate.
>
> ~~**Do not widen the fixed-width branch.** `int_lit_val`'s `int8_id` through
> `uint64_id` arms deliberately wrap to a width; that is correct fixed-width
> semantics, not the defect.~~ **STRUCK — the claim is false.** Architect
> ruling `evt_3zj3z2x1get7p`, 2026-08-11: `35 §1:43-45` states fixed-width
> partiality is marked and never silent, `§3.2:182-200` confines modular
> behaviour to the explicitly named wrapping class, and `§5:275-295` makes a
> narrowing move that may not fit partial. **The `as` casts are an
> implementation defect, not an established literal policy**, and this frame
> asserted the opposite. See the guard section below, which replaces this
> paragraph.

**2a. The fixed-width representability guard — ADDED 2026-08-11 by Architect
ruling `evt_3zj3z2x1get7p`, atomically part of this node.**

> **Why this is here and not in a successor, because the Steward first ruled it
> out and was wrong.** My scope ruling `evt_1c5r66yh63vfc` held that the silent
> fixed-width wrap is a pre-existing defect this node inherits rather than
> creates. **That premise is false for exactly the inputs this node adds.**
> Today `const x : UInt8 = 2^128` never reaches the wrap — `lexer.rs:575`
> refuses it, because the value is not representable in `i128`. **Widening the
> carrier removes that refusal, and the value then flows to `(n as u8)` and
> silently becomes `0`.** The node does not inherit the counterexample; it
> **newly admits** it. A node that widens an acceptance surface owns the values
> it newly accepts.

The lawful boundary, and it is a component boundary rather than a suggestion:

1. **Keep the source integer as the mathematical `BigInt` through target
   selection.** Do not narrow first.
2. For a fixed-width expected type, consult **one declaration-owned descriptor**
   `(type id, min, max)` and decide `min <= n <= max` **before** creating the
   literal primitive or inserting a `NumericLitVal`.
3. **In range:** store and transport the exact same mathematical value.
   **Out of range:** a span-bearing elaboration error naming the literal, the
   target type, and the representable interval.
4. **Do not emit an overflow VC and do not defer to a runtime check.** This is a
   closed constant whose representability is decidable at elaboration.

**Forbidden outright**: calling the raw narrowing primitive, reducing modulo,
reinterpreting a high bit, clamping, or casting back to `i128`. **The
cast-back is specifically the workaround an earlier Steward instruction
suggested; it is not available.**

**This is a bounded literal-representability companion — NOT fixed-width
arithmetic or overflow work.** No `OQ-1a` VC, no wrapping-literal syntax, no
conversion API change, no arithmetic-op change, no `spec/` edit. If the guard
pulls any of those in, that is a stop.

**3. The consumer sites reconciled.** `elab.rs:3614`, `:3662`, `:3669`,
`:3757`, and `resolve.rs:560-562`, **plus `numbers.rs:564` and
`compiler_driver.rs:3658`/`:3715` from the amendment above** — the frame's
original "five" was an undercount, and the census is yours to re-derive rather
than take from this list. The `resolve.rs` pair compares against literal `0`
and `1` for the `Zero`/`Succ` type-level spellings — **that comparison must
keep working**, and it is the one most likely to break silently under a payload
change.

## Acceptance criteria

**AC-1 — a literal wider than `i128` round-trips, through BOTH exits.**
Elaborate an integer literal exceeding `i128::MAX` and show it reaches the
kernel as an `IntLit` with **the value written**, not a wrapped or saturated
one. **This is the node.** State the literal in the claim.

**Strengthened 2026-08-11 with Deliverable 2's amendment**: reach the value
through the kernel `IntLit` **and** through an evaluation of the same literal,
or say explicitly which exit the claim covers. The two paths diverge below
`NumLit`, and **a round-trip that only proves the kernel side proves the half
that was never going to be the problem.**

**AC-2 — the A/B, because AC-1 alone can pass on a lucky path.** With the
truncation restored, the AC-1 literal must produce a **different, wrong**
value; with it removed, the written one. `LANG-SURFACE-PAIR`'s lexer seam was
established this way, and it is why that seam was real rather than a test that
passed because something else was special-cased.

**AC-3 — kernel-side value equality, not surface-side.** Two spellings of one
large value are decided equal by the kernel's `BigInt` equality (`term.rs:253`),
not by a surface comparison you added. **If you wrote the comparator, it is not
this criterion.**

**AC-4 — the `Zero`/`Succ` type-level path is unbroken**, with a control that
fails if `resolve.rs:560-562` stops recognizing `0` and `1`. This is the
regression a payload change causes and nothing else would catch.

**AC-4a — the representability guard, on the Architect's discriminators.**
Added 2026-08-11 (`evt_3zj3z2x1get7p`). Each pair is a boundary, so an
off-by-one guard fails one side:

- `255 : UInt8` accepts as **exactly** 255; `256 : UInt8` rejects.
- `127 : Int8` accepts as **exactly** 127; `128 : Int8` rejects.
- `18446744073709551615 : UInt64` accepts exactly; the next integer rejects.
- the `2^128 : UInt8` probe rejects **by the same range boundary** — not by a
  leftover width refusal further up. **This is the input the widening newly
  admits, so a guard that rejects it for the wrong reason has not been tested.**
- **a causal mutation of the guard admits at least one rejected source** (or
  otherwise flips its verdict), and restoration rejects. **Not a test-only
  oracle** — mutate the guard the compiler runs.

**AC-5 — `trusted_base()` is unchanged.** No kernel or TCB delta; the target
already exists.

**AC-6 — no `spec/` edit.** The spec already says this. If it appears not to,
that is a stop.

## Excluded scope

- **No numeric tower work.** `Decimal(i64, i32)`'s bounded mantissa is a real
  and separate gap and is now framed as [[LANG-SURFACE-DECIMAL-PRECISION]] —
  **do not fold it in.**
- **No `Float`/`Float32` change**, no overflow-obligation work (`OQ-1a`), no
  native `Int8…Int64`/`UInt8…UInt64` types. **The `OQ-1a` exclusion survives
  the guard added at 2a**: literal representability is decidable at
  elaboration and is not overflow work. Deciding a closed constant fits its
  target is in scope; anything that emits a VC or defers to a runtime check is
  not.
- **No performance work.** No small-value fast path, no benchmark.
- **No new surface production.**

## Stop conditions — return to me, do not decide

- **The spec appears to disagree** with arbitrary-precision `Int`.
- **A consumer genuinely requires fixed width** — that is a design fork for the
  Architect, not something to work around with a cast.
- **The guard cannot be placed before the literal primitive is created.** The
  ruling requires the range decision to happen *before* `NumericLitVal`
  insertion; if the code shape forces it after, say so rather than moving the
  decision.
- **`Decimal`'s bounded mantissa blocks a deliverable here.** That is the
  sibling node and it is mine to frame.

## Contention

Runtime is in `crates/ken-runtime/src/cranelift_backend/`. **The `lexer.rs`
contention is CLEARED** — [[LANG-LEX-PROJECTION-ADJACENCY]] merged at
`28cebda7` (PR #1864), so `crates/ken-elaborator/src/lexer.rs` is free and this
node has no live intersection. **Re-derive it at candidate time anyway** — a
merge-base goes stale without your branch moving.

**One thing that node left you, and it is a help rather than a hazard.** It
replaced the character-adjacency fraction guard with one that consults the last
emitted token, recorded by `next_token` after trivia is skipped. So the
fractional-part decision in `lex_numeric` now has token context available at
exactly the point where Deliverable 2 asks whether `Token::Nat` is itself
width-bounded. **Read the current `lex_numeric` before you measure**, not the
frame's `ae6f750a` description of it.

## Sizing and validation

`scripts/ken-cargo test -p ken-elaborator` plus a focused suite. **`NumLit` is
public**, so run the consumer checks the pair node needed —
`scripts/ken-cargo check -p ken-cli -p ken-interp -p ken-verify` — because a
public enum's blast radius is every crate that consumes it, which is the lesson
`LANG-SURFACE-IF` paid for. **Never `--workspace`**; that is CI's gate.
