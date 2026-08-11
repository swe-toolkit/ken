# LANG-LEX-PROJECTION-ADJACENCY — the projection guard, stated as a rule

Owner: language. Size: S. Node: [[LANG-LEX-PROJECTION-ADJACENCY]].
Fixed inputs measured at `origin/main` = **`63d6e007`**. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

**Seat tier: T2 build ring.** Architect votes at merge (`crates/` scope); no
Spec vote — this frame touches no `spec/` or `conformance/` path.

## What this deliverable is

`LANG-SURFACE-PAIR` fixed the projection/float collision at the character
level. **This node states the same decision as a rule, so that it holds for
every spelling of the program rather than for the unspaced one.**

It is deliberately small. The mechanism is one predicate; the work is deciding
which predicate, proving the decision holds across spellings, and — the part
that is actually load-bearing — finding out whether a formatter can produce the
failing spelling.

## The design call, front-loaded

**The current state is neither of the two coherent designs, and that is the
finding.** A language may reasonably say whitespace after a projection dot is
grammatical, or that it is not. Ken currently accepts `p. 1` (one spaced
projection parses) and refuses `p. 1.2` (a chained one does not) — and the
refusal is issued by the number scanner, not by any grammar rule. **No
statement of the surface syntax produces that behaviour**, which is how you can
tell it is unintended rather than a deliberate asymmetry.

**Recommendation: whitespace-insensitive.** `p.1.2`, `p.1 .2`, `p. 1 .2` and
`p. 1.2` all mean the same thing. The reason is not taste — it is that the
alternative cannot be stated without naming the number scanner, and a surface
rule that has to describe a lexer implementation detail is not a rule.

**You may return the other answer.** If Language and the Architect conclude the
spaced form should be rejected, that is a legitimate outcome of this node — but
then it must be rejected **by a grammar rule with its own diagnostic**, applying
uniformly to `p. 1` as well, rather than falling out of a fractional-part scan.
Silence is not the third option.

## The mechanism fork, and its buildability

**`Lexer` holds only `src` and `pos` (`lexer.rs:118-121`) — there is no token
history**, and the accumulating vector lives in a local `out` at `lexer.rs:622`,
outside the struct. So "was the previously emitted token a `Dot`?" is not
answerable inside `lex_numeric` as the type stands today. That constrains the
options, and it is why the fork is stated here rather than left to discovery:

| option | change | sees past a comment |
|---|---|---|
| **A — trim the whitespace run** — `self.src[..start].trim_end().chars().next_back() == Some('.')` | one expression, no struct change | **no** |
| **B — carry the last emitted token** — a field on `Lexer`, set by the tokenize loop | a struct field plus its maintenance | **yes** |

**A is the smaller change and B is the more nearly correct one.** Pick on the
comment question, not on size: if `p. -- note` + newline + `1.2` should behave
like `p.1.2`, only B answers it, and A will look right until someone writes
that program. **State which you chose and why in one sentence** — a lexer
predicate that reads as an accident is what produced this node.

A third option exists and is worth a moment before you discard it: **decide it
in the tokenize loop rather than in `lex_numeric`**, where the emitted-token
context is already in scope at `:622` without a struct field. It may be the
cheapest correct answer; I have not measured whether the loop can suppress a
fractional scan from there.

## Deliverables

**1. The observed behaviour of all four spellings, before any fix.** State the
token stream and the resulting diagnostic (or AST) for `p.1.2`, `p.1 .2`,
`p. 1 .2`, and `p. 1.2` on unmodified `main`. **The node's severity claim rests
on this** — see AC-1.

**2. The chosen predicate**, with the mechanism named and the comment case
answered either way.

**3. A formatter reachability answer.** Can `kenfmt` or the lossless printer
emit or preserve a space between a projection dot and its index? **A yes makes
this a meaning-changing reformat**; a no bounds it to hand-written source.
Either answer discharges the deliverable — an unmeasured axis does not.

## Acceptance criteria

**AC-1 — the severity claim is measured, not inherited.** The node asserts the
current failure is a **parse error**, not a silent mis-parse, derived from the
projection loop's lookahead admitting only `Ident` or `Nat(1 | 2)`. **Confirm or
refute it by running `p. 1.2`.** A refutation — any path on which it produces a
well-formed AST that means something other than a chained projection — is a
**stop condition**: come back to me, because the node is then mis-sized and the
frame's `S` is wrong.

**AC-2 — one control over all four spellings, sharing one expected result.**
Not four independent tests that each pass for their own reason. Under the
whitespace-insensitive answer they produce the same AST; under the rejection
answer, `p. 1` and `p. 1.2` both produce the same *named* diagnostic. **The
control is the equality across spellings**, because that equality is the
property the guard exists to provide and it is currently stated nowhere.

**AC-3 — the float side keeps a positive control.** `3.14`, `1.2e5`, and a
decimal-suffixed literal still lex as single numeric literals. **This is the
half a fix here can silently break**, since both directions run through the
same `has_dot` branch, and a guard that never fires would pass AC-2 by refusing
to build any float at all.

**AC-4 — A/B the guard, do not merely assert it.** With your predicate removed,
the spelling that motivated this node must fail; restored, it must pass.
`LANG-SURFACE-PAIR`'s own AC-2 was established this way and it is why the seam
was real rather than a test that passes because the parser was special-cased.

**AC-5 — no other surface production changes.** This node states an existing
decision; it does not extend the surface.

## Excluded scope

- **No new syntax.** No projections past `.2`, no named-field positional
  hybrid, no tuple arity beyond what landed.
- **No formatter change.** Deliverable 3 asks whether a formatter *can* produce
  the spacing. If it can, that is a finding and a follow-up node — **not a fix
  to fold in here.** Say so and stop.
- **No number-literal semantics work.** Arbitrary-precision `Int` is a separate
  queued node; do not start it under this frame.

## Stop conditions — return to me, do not decide

- **AC-1 refutes the severity** — a silent mis-parse rather than a refusal.
- **The formatter can emit the spacing.** That crosses from surface wart to
  meaning-changing reformat and I will re-sequence rather than let it ride on an
  `S`.
- **Either predicate turns out to need a parser change.** The fork above is
  lexical on purpose; a fix that reaches into `parser.rs` means the decision
  boundary is not where this frame assumes.

## Contention

**None expected.** Runtime is in `crates/ken-runtime/src/cranelift_backend/`
on the `D2h` key plane; this is `crates/ken-elaborator/src/lexer.rs` plus its
tests. The intersection with the in-flight Runtime candidate is empty.

**Re-derive it anyway** at candidate time against the merge-base, per the
intersection test — `main` moves under you, and a base goes stale without your
branch moving.

## Sizing and validation

One predicate, one control, one reachability question. **If you are past an
hour and still building, the mechanism fork was the wrong one** — take option A,
land it, and file the comment case rather than growing this node.

Validate with `scripts/ken-cargo test -p ken-elaborator --test <suite>` for the
focused controls, plus the full targeted `-p ken-elaborator` floor. **No
`--workspace`** — workspace greenness is CI's gate. No public AST variant
changes here, so the `ken-cli`/`ken-interp`/`ken-verify` consumer sweep that
`LANG-SURFACE-PAIR` needed does not apply; say so rather than running it out of
habit.
