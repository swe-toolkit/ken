---
id: LANG-SURFACE-INT-PRECISION
title: "`Int` is specified arbitrary-precision and the kernel already carries `Term::IntLit(num_bigint::BigInt)`, but the surface truncates to `NumLit::Int(i128)` through a lossy `n as i128` cast, and the lexer implements none of the `0x`/`0b`/`0o`/`_` forms that 31-lexical lists"
status: merged
owner: language
size: M
gate: none
depends_on: []
blocks: [LANG-SURFACE-DECIMAL-PRECISION, LANG-LEX-NUMERIC-FORMS]
github: https://github.com/swe-toolkit/ken/pull/1871
origin: Steward measurement 2026-08-11 at origin/main=ae6f750a, taken while picking the Language successor to LANG-LEX-PROJECTION-ADJACENCY. Grounded in a DECIDED spec rule, not an open question -- OQ-int was decided by the operator 2026-06-27.
---

## The gap

**The spec is decided and unambiguous.** `spec/30-surface/35-numbers.md:55`
and `:69-71` state `Int` is **arbitrary-precision by default (not fixed-64)**,
and record `OQ-int` as **DECIDED** (operator, 2026-06-27; also
`spec/90-open-decisions.md:24` and `:834`, and `spec/00-overview.md:127`
— *"Real types from the start"*).

**The kernel already implements it.** `crates/ken-kernel/src/term.rs:255`:

```rust
IntLit(num_bigint::BigInt),
```

with the conversion primitive deciding two `IntLit`s by `BigInt` value
equality (`:253`).

**The surface does not.** `crates/ken-elaborator/src/ast.rs:537`:

```rust
/// Integer literal — defaults to `Int` unless an expected type is given.
Int(i128),
```

and the parser builds it through a **lossy cast** at
`crates/ken-elaborator/src/parser.rs:2254`:

```rust
Ok(Expr::ENumLit(NumLit::Int(n as i128), span))
```

So a landed arbitrary-precision kernel target is reachable only through a
fixed-width surface. **This is the same shape as [[LANG-SURFACE-IF]] and
[[LANG-SURFACE-PAIR]]** — surface catching up to a target that is already
complete and already exercised — which is why it is sized `M` rather than as a
numeric-tower project.

## The second half: the lexical forms

`spec/30-surface/31-lexical.md:506` lists the integer forms as
`0`, `42`, `1_000`, `0xFF`, `0b1010`, `0o17`, all typed `Int` (arbitrary
precision). **The lexer implements none of the last four.** It has no
`from_str_radix` call and no digit-separator handling; `_` appears only in the
identifier-continue predicate (`lexer.rs:154`) and the identifier-start branch
(`:425`).

Whether the radix forms and separators belong in this node or a sibling is a
cut question for the frame, not a spec question — the spec admits all of them
on the same line.

## What is not yet known

- Whether `Token::Nat` itself is width-bounded upstream of the `n as i128`
  cast. If it is, the truncation has **two** sites and the lexer is the first
  one; the cast is then a symptom rather than the defect. **Unmeasured.**
- Whether any downstream consumer of `NumLit::Int` assumes a fixed width —
  `elab.rs:3614`, `:3662`, `:3669`, `:3757` and `resolve.rs:560-562` all match
  on it, and `resolve.rs` compares against literal `0` and `1` for `Zero`/`Succ`
  type-level spellings.

## Resolved after merge: does the guard dominate BOTH exits?

**Adversary `evt_4q1n8wx6pn5ay` on `90ce6743` asked the right question and
could not resolve it. Recorded here so the next reader does not re-derive it.**

The report first confirmed the guard is not vacuous: across `lexer.rs`,
`parser.rs`, `ast.rs` and `numbers.rs` there are **zero** occurrences of
`as i128`, `as u128`, `i128::try_from`, `u128::try_from`, `to_i128` or
`to_u128`, so no surviving upstream width conversion can pre-empt it, and
`FixedWidthLiteralOutOfRange` has exactly one production raise site
(`elab.rs:3674`).

**Then the sharp part.** A single raise site is the strongest form when it
dominates every path and the weakest when it does not, and **source order is
not control flow.** If `:3674` sat inside the kernel-path block, a large
literal taking the evaluation exit would reach `NumericLitVal::Int` and the CLI
without ever meeting the range check — and `255`/`256 : UInt8` would still
discriminate correctly on the kernel exit. That is the same two-exit divergence
that made this frame's original `AC-1` insufficient, applied one layer lower.

**Measured, and it comes back clean for a structural reason rather than a
lucky one.**

| site | enclosing fn | path |
|---|---|---|
| `:3674` the guard | `elab_num_lit_checked` | expected-type override |
| `:3755` `NumericLitVal::Int` | `num_lit_default_type` | unconstrained default |

`num_lit_default_type` has **exactly one caller** (`:3619`) and, for
`NumLit::Int`, returns `nenv.int_id` — arbitrary-precision `Int`, **which has
no bounds to check.** A fixed-width target can only arise from an expected
type, and that routes through `elab_num_lit_checked`, where the guard runs
before `int_lit_val`.

⇒ **The guard's absence on the default path is correct by construction, not a
bypass.** It dominates every path on which a bound exists. **No defect, and no
follow-up node.**

**The load-bearing fact, so a future change knows what it would break:** this
holds because the declared default for an integer literal is `Int`. If the
default table ever yields a fixed-width type for an unconstrained literal, the
default path acquires a bound and would need its own check.
