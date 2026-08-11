# LANG-LEX-HEX-FLOAT — the hex float form `0x1p-3`

Owner: language. Size: M. Node: [[LANG-LEX-HEX-FLOAT]].
Fixed inputs measured at `origin/main` = **`47a0b791`**. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if
your diff stays in `crates/` — but see the decision in "The question the spec
does not answer", which may be the exception.

> ## GATE — [[LANG-LEX-NUMERIC-FORMS]] must be on `main` first, and it is a
> ## real dependency in two directions
>
> **Direction one: the `0x` branch.** That node creates the prefix branch for
> radix integers. **Extend it. Do not build a second `0x` entry point** — the
> prefix is loaded three ways and only one scanner may own the fork.
>
> **Direction two, and this one is a stop.** That node also repairs the float
> path you are about to join. Measured by execution on `47a0b791`,
> **`3.14e5` lexes to `FloatLit(0.0)`** — `exp_str` already holds the `e`
> (`lexer.rs:555`) and the float branch formats `"{}.{}e{}"` at `:610`, so
> `"3.14ee5"` fails to parse and `unwrap_or(0.0_f64)` returns zero.
>
> **If both `unwrap_or(0.0)` sites (`:600`, `:610`) are still there when you
> start, stop and tell me.** A new literal form built on a parser that
> swallows its own failures inherits the swallow, and a hex float that
> silently becomes `0.0` is strictly worse than one that does not lex.

## What this deliverable is

One literal form: `0x1p-3`, a hex mantissa with a **binary** exponent written
in decimal, denoting an IEEE `f64`. Both spec tables list it
(`31-lexical.md:508`, `35-numbers.md:232`) and the lexer has no path for it.

**This is a small amount of scanning and a real amount of arithmetic.** The
scanning is a sibling of work already landed. The arithmetic has no precedent
in this crate.

## The reason this is its own node

Every other form in this arc ends by handing a string to something that
already parses it — `BigInt::parse`, `from_str_radix`, `f64::parse`.

**There is no such destination here.** `"0x1p-3".parse::<f64>()` is an error,
not `0.125`; Rust's float parser does not accept hex-float syntax and there is
no `from_str_radix` for floats. **You are writing the conversion.**

⇒ **The deliverable is a correctly-rounded conversion, not a scanner.** Treat
the scanning as the easy half and spend your effort on the other one.

## What correctness means here, stated before you estimate

`0x1p-3` is exactly `0.125`. `0x1.8p3` is exactly `12.0`. **Every example a
person writes by hand is exact**, which is precisely why a naive
implementation passes a hand-written suite and is still wrong.

A hex mantissa can carry more than 53 significant bits. When it does, the
result must be **correctly rounded to nearest, ties to even** — the same
rounding `f64::parse` already gives every other `Float` literal. Getting a
different answer than the decimal path would for the same real number is the
defect.

**`(mantissa as f64) * 2f64.powi(exp)` is exact only while the mantissa fits
in 53 bits and the scaling neither overflows nor goes subnormal.** It is wrong
in exactly the cases your hand-written tests will not contain. If you use it,
you owe the guard that says when it applies.

**The design choice is yours and I am not pre-empting it.** `BigInt` is
already a dependency of this crate after [[LANG-SURFACE-INT-PRECISION]], so an
exact big-integer assembly is available and obviously correct; a bounded
integer with an explicit sticky-bit round is the conventional implementation
and faster. **Pick one, state why in one sentence, and control it.**

## The question the spec does not answer, and it is a stop if you disagree

**Is the `p` exponent required or optional?** The spec gives exactly one
example, `0x1p-3`, and says nothing about `0x1.8`.

**C requires the exponent, and it requires it for a reason Ken shares:**
without it, `0x1.8` is ambiguous against a hex integer followed by something,
and the `.` is already load-bearing in Ken's lexer for positional projection
(`p.1.2`, which [[LANG-LEX-PROJECTION-ADJACENCY]] landed a guard for).

**Recommendation: require the `p` exponent.** `0x1.8` without it is a
rejection with a diagnostic that names the missing exponent. That makes the
form unambiguous, matches the one spec example, and keeps the projection guard
out of it.

**This is a lexical rule the spec does not settle, so it is yours to decide —
but if you decide the other way, that is a stop**, because permitting `0x1.8`
puts a `.` fork inside a hex literal and I want to see that reasoning before
it lands, not after.

## Separators, and the boundary that has already bitten the sibling

[[LANG-LEX-NUMERIC-FORMS]] lands digit separators under the rule **separators
only between digits**. The spec says only *"underscores are digit separators
and are ignored"* (`31 §3`) and does not say whether that reaches a hex
mantissa or a binary exponent. **`0x1_0p-3` and `0x1p-1_0` are yours to
settle.**

**Measured on the sibling, 2026-08-11, before you start.** Its first two
candidates got the unsigned exponent right and were blocked on the **signed**
one: `1e+_1` and `1e-_1` were accepted as `FloatLit(10.0)`. After consuming
`e+`, `exp_str.len() == 2`, and the underscore guard only rejected
`exp_str.len() <= 1` — so a separator immediately after the sign passed a
check meant to require a preceding digit.

⇒ **A sign is not a digit, and a length test on a buffer that already contains
the sign will not say so.** Your `p` exponent has the same sign position and
the same trap. **If you permit separators here, `0x1p+_3` and `0x1p-_3` are
rejections, and you owe both as controls** — not because they are likely, but
because the sibling proves the guard people actually write accepts them.

**Recommendation: permit separators between hex digits and between exponent
digits, matching the sibling's rule exactly.** Diverging from it would give one
lexer two separator rules, which is the proliferation `docs/PRINCIPLES.md`
warns about. If you decide hex floats take no separators at all, that is
defensible — **say so in one sentence and control the rejection**, rather than
leaving it to fall out of the implementation.

## Acceptance criteria

**AC-1 — the motivating forms have exact values.** `0x1p-3` is `0.125`,
`0x1.8p3` is `12.0`, `0x1p0` is `1.0`. **Assert the value, never the token
kind.** A kind assertion passes for every wrong value of that kind, which is
how `3.14e5 == 0.0` survived in this same function — `lang_surface_pair.rs:63`
checks `token_kinds("1.2e5")` and `FloatLit(0.0)` has the kind it expects.

**AC-2 — a mantissa needing more than 53 bits rounds correctly**, and the
expected value is derived independently of your implementation rather than
read out of its output. **This is the AC the node exists for.** State how you
obtained the expected value.

**AC-3 — the three-way `0x` fork is controlled.** `0xFF` is still the integer
`255`, `0x1p-3` is the float, and `0x[` still reaches whatever
`38-ffi-io`'s bytes form does today. **Each of the three needs its own
control**, because a fork is exactly where one branch silently captures
another's input.

**AC-4 — malformed hex floats are refused with a span**, not silently zeroed.
At minimum: a missing exponent if you took the recommendation, a `p` with no
digits, and a non-hex digit in the mantissa.

**AC-5 — the A/B.** Disable your branch and show `0x1p-3` fails; restore and
it passes.

**AC-5a — the separator rule you chose is controlled at the sign boundary.**
Whatever you decide, `0x1p+_3` and `0x1p-_3` have explicit controls. If you
permit separators, they are rejections; if you forbid them entirely, so is
`0x1_0p3`. **The sibling was blocked twice on exactly this position**, so an
uncontrolled sign boundary is a known gap here, not a hypothetical one.

**AC-6 — the existing forms and the projection seam are unchanged.** `3.14`,
`1e-9` and `3.14e5` as the sibling leaves them, `1.2d`, `1.5f32`, `42`, and
`p.1.2` / `p. 1.2`.

**AC-7 — no `spec/` edit, no new surface production, `trusted_base()`
unchanged.**

## Excluded scope

- **`0x[deadbeef]` byte literals.** `38-ffi-io`'s, not a numeric form.
- **No `Float32` hex form.** `0x1p-3f32` is in neither spec table. If you
  think it should exist, that is a spec question and a stop, not scope.
- **No repair of the decimal float path.** [[LANG-LEX-NUMERIC-FORMS]] owns
  that; if it is unrepaired when you start, the gate above says stop.
- No numeric tower, no `Float` semantics change, no overflow or conversion
  work, no performance work.

## Stop conditions — return to me, do not decide

- **The `unwrap_or(0.0)` sites are still present** (the gate).
- **You decide the `p` exponent is optional.**
- **Correct rounding turns out to need a change outside `lex_numeric`** — for
  instance a different `Float` carrier. That would mean the form is not a
  lexical addition, which changes the node.
- **The `0x` fork cannot be made unambiguous** against the bytes form.

## Contention

`crates/ken-elaborator/src/lexer.rs`, `lex_numeric` and the `0x` branch
[[LANG-LEX-NUMERIC-FORMS]] creates. **Those two contend directly and this one
is gated on the other**, so the ordering is settled rather than negotiated.
Runtime is in `crates/ken-runtime/`. **Re-derive the intersection at candidate
time** — a merge-base goes stale without your branch moving.

## Sizing and validation

`scripts/ken-cargo test -p ken-elaborator` plus the focused lexer suite.
**`Token` is crate-internal but `NumLit` is public** — you should not need to
change either enum's shape, since a hex float produces the same
`Token::FloatLit(f64)` a decimal one does. **If you find yourself changing
`NumLit`, stop and say why**; that would mean the form is not what this frame
thinks it is. **Never `--workspace`**; that is CI's gate.
