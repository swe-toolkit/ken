# LANG-SURFACE-PAIR — pair literals, `.1`/`.2`, and the `×` type production

Owner: language. Size: M. Node: [[LANG-SURFACE-PAIR]] (`ready`).

**Re-derive your merge-base from `origin/main`; do not reuse a SHA from this
frame.** [[LANG-SURFACE-IF]] is in the publisher as this is written and touches
`parser.rs`, `ast.rs`, `lossless.rs`, `layout.rs`, and `resolve.rs` — cut after
it lands.

## What you are doing

`spec/30-surface/32-grammar.md` admits pair literals, positional projections,
and the dependent-pair type, and the kernel's `Sigma`/`Pair`/`Proj1`/`Proj2` are
complete and already exercised by records. **Build the surface and the
desugaring onto them. Do not design the semantics and do not touch the kernel.**

## Fixed inputs — measured, do not re-derive

At `origin/main = 81e90e4e`.

| layer | state |
|---|---|
| `lexer.rs:100,309,399` | `Token::Times` produced for `×` and `><` |
| `parser.rs` | **zero occurrences of `Token::Times`** — lexed, consumed by nothing |
| `parser.rs:2341-2357` | the `LParen` expr arm parses one expr then expects `RParen` |
| `parser.rs:2193` | postfix projection guarded on `lookahead(1) == Token::Ident(_)` |
| `parser.rs:545-568,606-617` | `parse_binder` / `parse_binders` for the `(x : T)` shape |
| `term.rs:299,374-383` | `Term::Sigma`, `Term::Pair`, `Term::Proj1`, `Term::Proj2` |
| `check.rs:266-288,406` | Sigma typing, and the `NotASigma` rejection already exists |
| `ast.rs:341` | record fields are "in Sigma-telescope order" — the target is exercised |
| `elab.rs:112-115` | `MetaCtx { metas: Vec<Option<Level>> }` — **level metavariables only** |

**You are not building Sigma, pairing, or projection.** All three exist in the
kernel with conversion, typing, and observational equality.

## Three design calls, made with their reasoning so evidence can overturn them

### 1. Branch after parsing the first expr — never look ahead for a comma

The `LParen` arm is already load-bearing for grouping, ascription (`(e : T)`),
and the attached-proof reference. **Parse one expr as it does today, then branch
on whether the next token is a comma.** A scan for a comma before parsing is
wrong: a comma can appear inside a nested construct, and `(f (a, b))` would
misclassify the outer paren.

**`(e)` must mean exactly what it means today.** That is the regression surface.

### 2. Arity above two right-nests into binary pairs

The kernel has binary `Sigma` and binary `Pair` only. `(a, b, c)` is
`(a, (b, c))`, and `(x : A) × (y : B) × C` associates the same way.

**Right-nesting is a claim that is true at arity 2 for either association.**
State it and pin it at arity 3 — see `AC-3`.

### 3. Inference yields the NON-dependent Sigma. This is a measured bound.

`MetaCtx` holds `Vec<Option<Level>>` — **level metavariables only**. There is no
term metavariable and no term unification: `unify_types` (`elab.rs:262`) is a
structural walk that solves levels and falls through on everything else. Class
constraints do not change this; they are resolved eagerly at the definition site
into a fully applied dictionary term (`elab.rs:4603-4635`), never by inserting a
solvable implicit.

⇒ **Inferring a bare `(a, b)` cannot recover a dependent codomain**, because
recovering one means guessing which occurrences of the first component's value
in the second component's type were meant to be bound. Infer the non-dependent
Sigma — the second component's type weakened by one, the same construction
`infer_arrow` already uses (`elab.rs:3187-3199`).

**The dependent form is reachable only in a checked position**, against a
written `(x : A) × B`. That is not a limitation to apologize for; it is the
honest boundary of the machinery that exists, and stating it is `AC-4`.

**If you find you need a term metavariable to satisfy any AC here, stop and come
back to me.** That is a different node and a much larger one.

## Deliverables

1. **The pair-literal expr production**, branching per call 1, with n-ary
   right-nesting per call 2.
2. **A pair AST node carrying its written arity**, and its resolved counterpart.
   Arity is what lossless printing needs; the desugaring is binary.
3. **The `×` type production**, consuming the already-lexed `Token::Times`.
4. **Positional projections `.1` and `.2`**, including the chained form — which
   requires the lexer change below.
5. **Elaboration** to kernel `Pair`/`Sigma`/`Proj1`/`Proj2`, in both checking and
   inference mode.
6. **Lossless print and layout** for the new nodes.
7. **A specific diagnostic** when a projection is applied to a non-pair.

## Acceptance criteria

**AC-1 — it computes.** `(1, 2).1` and `(1, 2).2` each reduce to the
corresponding component. State the reduced term. A pair that elaborates without
projecting is not this AC.

> ### AC-2 — THE LEXER TRAP, AND IT IS WHY THIS IS NOT A ONE-LINE PARSER ARM
>
> The number scanner starts on a leading digit (`lexer.rs:417`) and consumes a
> fractional part when a `.` is followed by a digit (`lexer.rs:518-523`).
>
> | source | tokens today |
> |---|---|
> | `p.1` | `Ident(p)`, `Dot`, `Nat(1)` |
> | `p.1.2` | `Ident(p)`, `Dot`, **`FloatLit(1.2)`** |
>
> **A chained positional projection is swallowed by the float scanner**, and the
> result is not a parse error — it is a valid float literal standing where a
> projection index belonged.
>
> **`p.1.2` must project twice.** Pin it, and pin that `3.14` still lexes as a
> float in ordinary expression position. **Both halves, or the fix is a
> regression in the other direction.**
>
> This is the AC nothing else reds on: `p.1` works either way, so a candidate can
> deliver positional projection, pass, and leave the chained form silently wrong.

**AC-3 — right-nesting is stated and pinned at arity three.** `(a, b, c)` must
elaborate to `Pair a (Pair b c)`. **State the elaborated term**, not that a test
passed. An association that reads correctly at arity 2 holds for either
convention, so arity 3 is the first case that discriminates.

**AC-4 — the dependent form works in a checked position, and the inferred form
is honest.** A `const` ascribed `(x : Nat) × Vec Nat x` accepts a pair whose
second component's type mentions the first component. Separately, a bare
inferred `(a, b)` yields the non-dependent Sigma. **Report both elaborated
types.** If the inferred case silently produces something dependent, a premise
above has failed.

**AC-5 — `(e)` is unchanged.** Grouping, ascription `(e : T)`, and the
attached-proof reference all behave exactly as before. **This is the
regression surface of call 1** and the corpus already exercises it heavily —
name the existing controls you relied on rather than writing new ones.

**AC-6 — a projection on a non-pair gets its own diagnostic.** `(3).1` must fail
with an error naming the projection and the expected pair type. **A leaked
kernel `NotASigma` is a fail** — it would report a kernel-level term shape to a
user who wrote surface syntax.

**AC-7 — the lossless round-trip preserves arity.** `(a, b, c)` prints back as
`(a, b, c)`, **not** as `(a, (b, c))`. That is what deliverable 2's written
arity buys, and it is the same constraint that forced a real AST node in
[[LANG-SURFACE-IF]] rather than a parser-level rewrite.

**AC-8 — no trusted-base delta.** Assert `trusted_base()` is unchanged. Every
kernel constructor this node targets is already in it.

## Excluded scope

- **Records and record literals.** `{ x = 1 }` is a separate production with a
  separate node. Both land on Sigma telescopes; that adjacency is not a licence.
- **Rebuilding named-field access.** `Expr::EProj` exists and works
  (`parser.rs:2193-2203`). Positional projection joins that loop.
- **Term metavariables or any inference machinery.** See call 3. If an AC seems
  to need one, that is a hard stop and a finding, not a widening.
- **Projections past `.2`**, and pattern-matching on pairs. Chaining reaches the
  tail; patterns are the pattern-language node.
- **Any other `expr` production.** Implicit binders, `forall`, and the literal
  surface are measured gaps with their own nodes coming. **Do not widen into
  them**, however adjacent.

## Contention

`crates/ken-elaborator` and its tests. Runtime is on `crates/ken-runtime`;
Verify is on `conformance/`. **No `spec/` or `conformance/` path, so no Spec
vote on the merge Decision** — you are implementing written spec lines, not
amending them.

## Validation

Targeted only. `-p ken-elaborator`, or `--test <name>`, **never `--workspace`**.
New public AST variants make the floor a full `-p ken-elaborator` test build, as
[[LANG-SURFACE-IF]] found: a suite-scoped run cannot observe an exhaustive
`match` in a sibling target, and this change adds variants that `lossless.rs`,
`layout.rs`, and `resolve.rs` all match on. "No regression" means green in CI.

## Sizing

One turn to a releasable increment or a genuine hard stop. Both are good
outcomes.

**The likely stop is `AC-2`.** If the lexer cannot be made to hold both `p.1.2`
and `3.14` without a decision that belongs somewhere else — a lexer mode, a
lookahead the scanner does not currently have — report the constraint rather
than shipping the case that passes. **The chained form silently working on one
example and not another is exactly the outcome this frame is built to prevent.**
