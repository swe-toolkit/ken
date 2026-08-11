---
id: LANG-SURFACE-PAIR
title: "Pair literals, positional projections, and the Sigma type production are required by 32-grammar and wholly absent from the surface -- `Token::Times` is lexed for `×` and consumed by nothing, `(a, b)` is a parse error, and `.1`/`.2` fall outside the projection guard -- while the kernel's Sigma/Pair/Proj1/Proj2 are complete and already exercised by records"
status: merged
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: https://github.com/swe-toolkit/ken/pull/1859
origin: Steward measurement 2026-08-11 at origin/main=81e90e4e, from the research surface-gap sweep at 8898c426 (evt_3dsd7j9t4r33a), which ranked pair/Sigma among the next Language candidates. The lexer hazard in the projection section below is a Steward finding on top of that sweep, not part of it.
---

## The gap

`spec/30-surface/32-grammar.md §3` admits three productions that nothing
implements:

```
expr ::= ... | "(" expr ("," expr)* ")"          -- tuple / pair / grouping
       | ... | expr "." ident | expr ".1" | expr ".2"   -- field / projection
type ::= ... | "(" ident ":" type ")" "×" type   -- dependent pair (Σ)
```

Measured at `origin/main = 81e90e4e`:

| layer | state |
|---|---|
| `lexer.rs:100,309,399` | `Token::Times` is produced for both `×` and `><` |
| `parser.rs` | **zero occurrences of `Token::Times`.** The token is lexed and consumed by nothing |
| `parser.rs:2341-2357` | the `LParen` expr arm parses one expr and expects `RParen`; a comma is a parse error, so `(a, b)` does not parse |
| `parser.rs:2193` | postfix projection is gated on `matches!(self.lookahead(1), Token::Ident(_))`, so `.1`/`.2` never enter the loop |
| `ast.rs`, `elab.rs`, `resolve.rs`, `lossless.rs` | no `Times` reference and no pair node anywhere |

**A lexed token that no production consumes is the cleanest possible statement
of an unimplemented surface.** `×` reaches the parser and dies there.

## The elaboration target is built and already exercised

`crates/ken-kernel/src/term.rs:299,374-383` carries `Term::Sigma`, `Term::Pair`,
`Term::Proj1`, and `Term::Proj2`, and they are complete downstream: `conv.rs`
(whnf, conversion), `check.rs:266-288,406` (typing, including the `NotASigma`
rejection), `obs.rs:78,165,318,350` (observational equality at Sigma), and
`subst.rs:36`.

**They are not dormant.** `ast.rs:341` describes record fields as being *"in
Sigma-telescope order"* — records already elaborate through this exact kernel
path, so the target is exercised by landed code rather than merely present.

⇒ **This node is surface plus desugaring onto a landed, exercised target**, the
same character as [[LANG-SURFACE-IF]] and `SURF-SPACE-CELLS`. It is not a design
fork and it adds nothing to the TCB.

> ## THE LEXER HAZARD: `.1` LEXES, `.1.2` DOES NOT
>
> The number scanner is entered on a leading digit (`lexer.rs:417`) and then
> consumes a fractional part when a `.` is followed by a digit
> (`lexer.rs:518-523`), producing `FloatLit`.
>
> So the two cases diverge:
>
> | source | tokens |
> |---|---|
> | `p.1` | `Ident(p)`, `Dot`, `Nat(1)` — fine |
> | `p.1.2` | `Ident(p)`, `Dot`, **`FloatLit(1.2)`** — the second projection is gone |
>
> **A chained positional projection is eaten by the float scanner**, and the
> failure is not a parse error at the projection — it is a well-formed float
> literal appearing where a projection index belonged.
>
> **This is the trap that reads correctly on the case anyone would write first.**
> `p.1` works, so a candidate can pin positional projection, pass, and leave
> `p.1.2` broken with no red. Any fix belongs in the lexer's decision to start a
> fraction, not in the parser.

## Scope

**IN:** the pair-literal expr production and its right-nesting for arity above
two; positional projections `.1` and `.2`, including the chained form; the
`(x : A) × B` type production consuming the already-lexed `Token::Times`; the
AST and resolved nodes; elaboration to the kernel's `Sigma`/`Pair`/`Proj1`/
`Proj2`; the lossless-print and layout path; and a specific diagnostic when a
projection is applied to a non-pair.

**OUT:**

- **Records and record literals.** `{ x = 1 }` is its own production with its
  own node. This node must not touch the record path even though both land on
  Sigma telescopes.
- **Named-field access `expr "." ident`.** Already implemented as
  `Expr::EProj`; positional projection joins it, it is not rebuilt.
- **Inferring a dependent Sigma.** See the frame — `MetaCtx` holds only level
  metavariables, so this is a measured capability bound, not a preference.
- **Projections past `.2`.** `32-grammar.md §3` lists exactly `.1` and `.2`;
  deeper positions are reached by chaining into the right-nested tail.
- **`><` as an alternative spelling.** The lexer already maps it to the same
  token; nothing further is owed here.
