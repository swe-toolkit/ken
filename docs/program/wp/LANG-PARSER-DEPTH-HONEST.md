# LANG-PARSER-DEPTH-HONEST — make the depth control measure depth, then dispose of the rework it justified

**Owner: language. Size: S. Gate: none.**

**Bases, and they are two different objects — read both before you start.**

- **Measurement base: `57688110`** — the tree before `50da348a`. This is where
  "did the capability exist beforehand?" is asked.
- **Candidate base: `origin/main` at cut time**, which must already contain
  `50da348a`. Re-derive it; do not take a SHA from this frame.

**Fixed inputs, measured at `origin/main` = `288b68c9` unless stated:**

| fact | where |
|---|---|
| no `=>` token exists; match-arm separator is `MapsTo` (`\|->` / `↦`) | `crates/ken-elaborator/src/lexer.rs:107`, emitted `:237`, `:287` |
| the inert fixture | `crates/ken-cli/tests/record_literal_parser_depth.rs` at `8e9baa18` |
| `NESTED_MATCH_DEPTH = 31`, child-process harness | same file |
| `brace_starts_match_arms`, terminator set `{Eq, Comma, Pipe, RBrace, Eof}` | `crates/ken-elaborator/src/parser.rs:2067` **at `50da348a`** — not present at `288b68c9` |
| the unmeasured rework, 143 lines | `766c9f07` "bound record parser stack use" |

## D1 — make the instrument run, then read it

Fix the fixture's arm syntax from `=>` to `|->` (or `↦`). Nothing else about
the fixture changes.

Then run it at **four** objects and report each number separately:
`57688110`, `50da348a`, `766c9f07`, `8e9baa18`.

**Report the depth each object actually reaches, not pass/fail at 31.** Binary
search or step the constant *for the measurement*, and say what you did. The
four numbers are the deliverable — they are what makes every later sentence in
this arc checkable.

**`57688110` vs `50da348a` is the discriminating pair.** It is the only
comparison that can say whether the record-literal work changed parse depth,
because it is the only one that brackets exactly that change.

## D2 — dispose of `766c9f07` on the evidence

With D1's four numbers in hand, choose one and say why:

- **keep** the rework — depth measurably regressed at `50da348a` and the rework
  measurably restores it;
- **drop** it — no regression exists, so 143 lines of `parser.rs` are
  unjustified churn in a file this arc has already reworked once;
- **revise** it — a real problem exists and the rework addresses it only
  partly.

Confirm or refute the Architect's structural argument as part of this:
`brace_starts_match_arms()` breaks the argument loop **before**
`parse_record_expr` is entered, so what the record change adds on the
nested-`match` path is a **bounded forward token scan — time, not stack.** If
D1 shows a depth regression, that argument is wrong and saying so is the
finding.

## D3 — pin the invariant that record patterns will break

Add a comment at `brace_starts_match_arms` recording that it is correct only
while **no pattern token from its terminator set can precede `MapsTo`**, and
that record patterns in `match` (`34 §3`) violate it: a first arm
`{ x, y } |-> …` hits `Comma` at offset 2 and misclassifies as a record
literal.

Comment only. **Do not fix the classifier here** — the fix belongs to the node
that introduces record patterns, and doing it now would be an unmeasured change
to a live parser fork.

## Acceptance criteria

- **AC-1 — the fixed fixture reaches depth greater than one at every one of the
  four objects.** This is what proves the instrument runs at all. A run that
  still dies in the lexer discharges nothing.
- **AC-2 — four depth numbers, one per object, each naming the object it was
  measured on.** Never a number attributed to "the parser" or to a role.
- **AC-3 — the D2 disposition names which of D1's numbers forces it.** A
  disposition that would read the same under any four numbers is not grounded.
- **AC-4 — the invariant comment names the terminator set and the
  record-pattern counterexample explicitly**, so the next node finds it by
  grepping either.
- **AC-5 — the handback states, in one sentence, whether a real stack problem
  exists.** Not "the control passes."

## Banned scope

- **Tuning `NESTED_MATCH_DEPTH` to make the control green.** The constant is
  not what is broken. Changing it to obtain a pass is the failure this node
  exists to correct, one level up.
- **Fixing `brace_starts_match_arms`' classification.** D3 is a comment.
- **Any depth claim before D1 runs** — including repeating "depth 31 never
  held," which is not established and is what the inert control was mistaken
  for evidence of.
- Any kernel term, `trusted_base()` change, or record-pattern work.

## Hard stops

- **If the fixed fixture still fails for a non-depth reason at any of the four
  objects, stop and report the reason.** A second broken-instrument round is a
  finding, not something to work around.
- **If the four numbers are identical**, say so plainly — that is a clean
  refutation of the regression hypothesis and it makes D2 a `drop`, which is a
  real result and not a null one.

## Seating

**Not an OpenAI-backed seat.** `gpt-5.6-sol` refuses stack-depth subjects at
the provider's policy layer and the refusal fires on the **subject**, so it
re-trips instead of clearing. If no other seat is available, that is an
operator escalation, not a reason to assign it anyway.

## Contention

Touches `crates/ken-elaborator/src/parser.rs` and one `ken-cli` test file. The
record-literal arc is the only other writer, and it is finished at `50da348a`.
No Runtime contention: nothing here is on the R3 path.
