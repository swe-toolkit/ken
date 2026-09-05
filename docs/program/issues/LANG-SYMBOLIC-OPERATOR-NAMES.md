---
id: LANG-SYMBOLIC-OPERATOR-NAMES
title: "`31-lexical.md:494` says operators are symbolic `from a fixed set plus user-defined`, and `33 section 6` says they are ordinary `fn` definitions with symbolic names -- but the lexer has no symbolic-operator token path at all, so a user operator can neither be named nor defined"
status: active
owner: language
size: S
gate: none
depends_on: []
blocks: [LANG-INFIX-APPLICATION-DEFAULT]
github: null
origin: "Architect scope ruling evt_1s7mqjg4tyxx1 (2026-08-15) on LANG-FIXITY-DECL-SURFACE. The Steward's census searched for `fixity|Fixity|infixl|InfixL` and found zero hits; the Architect reproduced it and established the gap is wider than the search term -- what is absent is user-defined operators, of which fixity is one of three parts. This is part (i) of his decomposition, and it is the bounded lexer half the Steward had concluded did not exist. Steward-filed and cut per COORDINATION section 2."
---

> # RELEASED 2026-09-05 to the language ring (lane-2, next after the formatter
> # predecessor LANDED 196392b6f). Per the operator CI-defer sequence in lanes.md:
> # symbolic-operators (THIS) -> match-patterns/quick-wins -> kernel-hardening ->
> # SEC1. Base = current main. Frame is complete (Deliverables + AC-1..5 with
> # controls, banned scope); anchors measured at e6d2716cf — re-check them against
> # landed code and escalate if any fixed input is false, do not build around it.
>
> # THE EXISTENCE OF THIS SURFACE IS NOT AN OPEN QUESTION. THE SPEC SAYS SO.
>
> `spec/30-surface/32-grammar.md:392-393`, verbatim:
>
> > *"The remaining spellings and the levels of non-arithmetic user operators
> > stay `OQ-syntax`; the existence of declared fixity, and this arithmetic
> > ordering, are **not**."*
>
> `OQ-syntax` is the open-question marker, and that sentence names two things
> as **not** open. **The spec pre-emptively answered the scope question**, so
> this is the opposite of [[CONF-FMT8-LEVELTOK]]: there the alias was landed
> behaviour nobody had ruled on; here the ruling exists and the implementation
> has not caught up.

## Treat every anchor here as perishable

If a fixed input below is false against the landed code, **say so and escalate
— do not quietly build around it.** Line numbers measured at `e6d2716cf`.

## The measurement, and why the original census understated it

The Steward's census (`fixity|Fixity|infixl|InfixL` → zero hits under
`crates/`) is accurate. **It names the attribute of a feature that is itself
entirely absent**, which is why it understated the gap:

| what | where | state |
|---|---|---|
| `BinOp` | `ast.rs:558` | **closed 5-variant enum** — `Add`, `WrappingAdd`, `Sub`, `Mul`, `EqEq`, all built-in |
| infix parsing | `parser.rs:1994` → `:2012` → `:2032` | **fixed-level recursive-descent cascade**, hard-wired to those tokens |
| symbolic operator token | `lexer.rs` | **no such path at all** |

Against `spec/30-surface/31-lexical.md:494`, verbatim:

> **`operator`** — symbolic, from a fixed set **plus user-defined** (`33`);
> fixity and precedence are declared (`infixl`/`infixr`/`infix N`).

and `spec/30-surface/33-declarations.md §6`:

> Operators are ordinary `fn`/`proc`/`const` definitions with symbolic names;
> there is nothing special about them semantically.

⇒ **`<+>` can be neither named nor defined today.** That is this node.

## Not the V0 objection — it is a category error and it is already answered

`32-grammar.md:410` lists a profile with *"no operators or fixity"*. **That is
V0, the minimal-elaborator G1 bootstrap slice**, and the same exclusion list
also says **no literals** and **no `match`**. Literals are landed. `match` is
landed — two nodes merged against it on 2026-08-15.

⇒ **Any argument that "a profile excludes it, therefore it may be unowed"
proves that `match` and literals are unowed too.** Do not re-raise it.

## Deliverables

**`D1` — a symbolic-operator token in the lexer.** A run of symbol characters
lexes to an operator token carrying its lexeme. **Report the character set you
treat as symbolic and where you got it** — `31-lexical.md` is the authority,
not convention from another language.

> **The collision question is the real work here, and it is why this is not
> one line.** The existing fixed operators (`+`, `-`, `*`, `==`) and any other
> symbolic punctuation the lexer currently special-cases must keep their
> present behaviour. **Enumerate every symbolic token the lexer emits today
> before you widen anything**, and say in the handback which ones you found and
> how each is preserved.

**`D2` — `fn` definitions with symbolic names.** `fn <+> (a : Nat) (b : Nat)
: Nat = …` defines an ordinary function whose name is symbolic, per `33 §6`.
**Semantically ordinary is the requirement** — no special binding, no special
resolution.

**`D3` — report anything that makes `D2` non-ordinary.** If a symbolic name
cannot flow through the existing name resolution, module export, or
diagnostic paths unchanged, **that is a finding and it stops here** rather than
being worked around.

## Acceptance criteria

**`AC-1`.** A symbolic name lexes and a `fn` with a symbolic name defines and
elaborates. **Control:** a fixture asserting the elaborated form, not "it
compiles".

**`AC-2` — every operator that lexes today still lexes the same way.**
**Control:** the enumeration from `D1` is in the handback, and the existing
arithmetic and comparison fixtures are named individually and stay green.
**This is the regression the whole node risks; a suite total is not evidence
for it.**

**`AC-3` — direction stated.** This **adds** accepted programs. Nothing
currently accepted becomes rejected. **If anything does, stop.**

**`AC-4`.** Infix *application* is out of scope and must not appear. Defining
`<+>` is this node; writing `a <+> b` is [[LANG-INFIX-APPLICATION-DEFAULT]].
A candidate that parses infix application has exceeded the node.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Targeted locally:
`-p ken-parser` and `-p ken-elaborator` as touched.

## Banned scope

- **No fixity table, no `infixl`/`infixr`/`infix` parsing.** That is
  [[LANG-FIXITY-DECL-SURFACE]] and it carries a design call this node does not.
- **Do not widen `BinOp`.** `ast.rs:558`'s closed enum is for built-ins; a user
  operator is an ordinary function, not a new `BinOp` variant. If you find that
  reaching the goal requires touching it, **report that** — it would mean the
  spec's *"nothing special about them semantically"* is not achievable through
  the landed AST, which is a finding worth more than the workaround.
- **Do not touch the arithmetic precedence cascade.** It is landed and
  normative — see the note below.

## One thing already checked, so it is not re-investigated

The Architect went looking for a missing built-in arithmetic precedence table,
because that would be a live correctness bug in landed code rather than an
unbuilt surface. **It is landed**: `parse_additive_expr` /
`parse_multiplicative_expr` implement the conventional split under the VAL2 #11
pin. **Hypothesis refuted — do not re-check it.**
