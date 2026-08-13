# LANG-RECORD-STACK-OVERFLOW — find and fix what the record-literal work adds to the compile stack

**Owner: language. Size: M. Gate: none.**

**Base: re-derive `origin/main` at cut time.** The record-literal work is
**not** on `main` — it lives on `wp/LANG-SURFACE-RECORD-LITERAL` at `50da348a`
(feature), `766c9f07` (attempted repair), `8e9baa18` (inert control). Fixed
inputs below were measured at `main` = `cbe58725`.

## The witness, and it already exists

```
ken-cli::mrc_4a_cross_crate_census mrc_4a_cross_crate_census_and_its_controls
```

A real `ken-cli` native compilation. It SIGABRTs with *"fatal runtime error:
stack overflow"* at `50da348a`, at `8e9baa18`, and in the fresh run on PR
#2092. `main` at `cbe58725` is green.

**Do not build a synthetic depth fixture to chase this.** The arc already had
one and it never ran (see D3).

## D1 — reproduce, then localize

Reproduce the abort locally with that exact test against a tree carrying the
record-literal work. Then **name what the change adds to that call chain** —
which function, and how much per level.

**Report the mechanism, not the symptom.** "The parser recurses more" is not an
answer; which entry, how many frames, and why this program reaches it is.

**Two candidates the evidence already narrows:**

- `brace_starts_match_arms` (`parser.rs:2067` at `50da348a`) performs an
  **unbounded forward token scan** from every `LBrace` in argument position.
  The Architect's argument was that this costs time, not stack — the
  measurement refutes the conclusion, so establish whether this is the site or
  whether the cost is elsewhere.
- The record forms add new mutual recursion between expression parsing and
  `parse_record_expr`.

## D2 — fix by reducing footprint

**`RUST_MIN_STACK` and any stack-limit raise are refused.** The gate-4a arc
next door repaired a stack regression by boxing a large value and forcing
inlining to remove a live frame, with the limit explicitly refused. Same
standard.

**`766c9f07` is preserved evidence, not a starting point.** It is a 143-line
`parser.rs` rework titled *bound record parser stack use*, and at `8e9baa18` —
which contains it — the test still aborts. **Do not rebase onto it and do not
assume its direction was right.** If your localization shows part of it is
correct, say which part and why, on your own measurement.

## D3 — fix the inert fixture, or delete it

`crates/ken-cli/tests/record_literal_parser_depth.rs` builds arms with `=>`.
Ken has no such token; the separator is `MapsTo` (`|->` / `↦`,
`lexer.rs:107`). The fixture dies in the lexer on the first arm and has never
reached depth one at any SHA.

Either fix the syntax so it measures something, or delete it. **A fixture that
cannot fail for its stated reason is worse than no fixture** — it reads as
coverage. If you fix it, report the depth actually reached; **do not tune
`NESTED_MATCH_DEPTH` to obtain a pass.**

## Acceptance criteria

- **AC-1 — `mrc_4a_cross_crate_census_and_its_controls` passes in CI** on a
  tree carrying the full record-literal feature. **CI, not a local targeted
  run** — a `-p ken-elaborator` run cannot reach this test, which is exactly
  why the defect survived review.
- **AC-2 — the localization from D1 is stated as a mechanism**, with the site
  and the per-level cost. An AC-1 pass without D1 is a fix nobody can review.
- **AC-3 — no stack limit is raised anywhere**, and no test is moved, skipped,
  or `#[ignore]`d to obtain the pass. If a row must be ignored, that is a hard
  stop, not a step.
- **AC-4 — a control that fails without the fix.** Whatever you land, show it
  red on the unfixed tree and green on the fixed one. **The existing witness
  qualifies** — you do not need a new one.
- **AC-5 — the record-literal ACs 1-8 still pass.** The Architect's design pass
  on those stands; this node must not weaken the feature to relieve the stack.
- **AC-6 — the D3 disposition is stated**: fixed with its measured depth, or
  deleted with the reason.

## Banned scope

- Raising or configuring any stack limit.
- Tuning `NESTED_MATCH_DEPTH`.
- Ignoring, skipping, relocating or weakening `mrc_4a_cross_crate_census`. It
  is `RT-MATCH-RECURSOR-CONSUMERS` `AC-1`'s evidence and belongs to Runtime.
- Changing what a record literal, pun, or update elaborates to.
- `view` retirement — that is `LANG-VIEW-RETIRE` and it waits on this.

## Hard stops

- **The overflow does not reproduce locally.** Report that; a CI-only
  reproduction is a real difference (release vs debug frame sizes, thread stack
  size) and it changes the repair, so it is information rather than a dead end.
- **The fix requires changing `mrc_4a_cross_crate_census`.** Stop. That test is
  Runtime's evidence and the change routes to them, not around them.
- **The correct repair is a parser architecture change** (explicit work stack,
  trampolining). Stop and report the finding with its size — that is a Steward
  cut, not something to absorb.

## Seating — READ BEFORE ASSIGNING

**The current `language-implementer` seat (`gpt-5.6-sol`, OpenAI-backed) cannot
work this node.** Its provider's policy layer refuses stack-depth subjects
outright — *"we take extra caution with cybersecurity requests"* — and the
refusal fires on the **subject**, so it re-trips rather than clearing on retry.
Retrying is prohibited.

**The whole node is that subject**, so unlike the earlier framing there is no
way to route around it. The seating disposition is an operator call because it
moves a seat between credit pools; the language leader escalated it rather than
burning turns, which is correct.

## Contention

`crates/ken-elaborator/src/parser.rs`. **`LANG-VIEW-RETIRE` touches the same
file and is sequenced after this**, on one implementer lane. No Runtime
contention on the source, but the *witness* is Runtime's test — do not modify
it.
