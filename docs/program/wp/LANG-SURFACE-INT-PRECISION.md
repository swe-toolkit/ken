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

**2. The upstream width question, answered.** Measure whether `Token::Nat` is
itself width-bounded before the cast. **If it is, the lexer is the first
truncation site and the cast is a symptom** — fix both or say plainly that one
remains. A candidate that removes the cast while the lexer still cannot lex a
large literal has moved the defect, not closed it.

**3. The five consumer sites reconciled.** `elab.rs:3614`, `:3662`, `:3669`,
`:3757`, and `resolve.rs:560-562`. The `resolve.rs` pair compares against
literal `0` and `1` for the `Zero`/`Succ` type-level spellings — **that
comparison must keep working**, and it is the one most likely to break silently
under a payload change.

## Acceptance criteria

**AC-1 — a literal wider than `i128` round-trips.** Elaborate an integer
literal exceeding `i128::MAX` and show it reaches the kernel as an `IntLit`
with **the value written**, not a wrapped or saturated one. **This is the
node.** State the literal in the claim.

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

**AC-5 — `trusted_base()` is unchanged.** No kernel or TCB delta; the target
already exists.

**AC-6 — no `spec/` edit.** The spec already says this. If it appears not to,
that is a stop.

## Excluded scope

- **No numeric tower work.** `Decimal(i64, i32)`'s bounded mantissa is a real
  and separate gap — **do not fold it in.** File it with me if you measure it.
- **No `Float`/`Float32` change**, no overflow-obligation work (`OQ-1a`), no
  native `Int8…Int64`/`UInt8…UInt64` types.
- **No performance work.** No small-value fast path, no benchmark.
- **No new surface production.**

## Stop conditions — return to me, do not decide

- **The spec appears to disagree** with arbitrary-precision `Int`.
- **A consumer genuinely requires fixed width** — that is a design fork for the
  Architect, not something to work around with a cast.
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
