---
id: LANG-SURFACE-INT-PRECISION
title: "`Int` is specified arbitrary-precision and the kernel already carries `Term::IntLit(num_bigint::BigInt)`, but the surface truncates to `NumLit::Int(i128)` through a lossy `n as i128` cast, and the lexer implements none of the `0x`/`0b`/`0o`/`_` forms that 31-lexical lists"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
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
