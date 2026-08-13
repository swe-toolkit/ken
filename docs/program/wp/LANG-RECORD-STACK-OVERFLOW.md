# LANG-RECORD-STACK-OVERFLOW — find and fix what the record-literal work adds to the compile stack

**Owner: language. Size: M. Gate: none.**

**Base: re-derive `origin/main` at cut time.** The record-literal work is
**not** on `main` — it lives on `wp/LANG-SURFACE-RECORD-LITERAL` at `50da348a`
(feature), `766c9f07` (attempted repair), `8e9baa18` (inert control). Fixed
inputs below were measured at `main` = `cbe58725`.

> **DELIVERED — merged as PR #2098 at `b4d38b8a`, together with the record
> literal surface itself.** This frame is historical. The dispatch-frame repair
> landed in `elab.rs`, and the depth fixture at
> `crates/ken-cli/tests/record_literal_parser_depth.rs` was rebuilt to use Ken's
> real `|->` token rather than the `=>` that made the original fixture die in
> the lexer before it reached depth one.
>
> **The node's `status:` sat at `ready` for hours after it merged**, and on
> 2026-08-13 the Steward read that stale status and kicked Language onto
> already-finished work. The language leader caught it. **A node status is not
> evidence about the tree — check whether the work landed.**

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

## Seating — RESOLVED 2026-08-13, the blocker is gone

**This section previously said the seat could not work this node. That is no
longer true, and the reseat happened FOR this node.**

The former `language-implementer` seat (`gpt-5.6-sol`, OpenAI-backed) could not
work it: the provider's policy layer refused stack-depth subjects outright
(*"we take extra caution with cybersecurity requests"*), the refusal fired on
the **subject** rather than the phrasing, and the whole node is that subject —
so it re-tripped rather than clearing on retry, and there was no way to route
around it.

**The operator reseated `language-implementer` to Sonnet 5 on 2026-08-13
(PR #2094).** That is **not** a tier change — Sonnet-class is this role's
documented T2 tier in `agent/MODELS.md`. The reason was a provider capability
gap, not task difficulty. **The seat can work this node; proceed normally.**

## Contention

`crates/ken-elaborator/src/parser.rs`. **`LANG-VIEW-RETIRE` merged first**
(PR #2103) and touched the same file, so **re-derive your base from current
`origin/main` rather than from this frame's measured SHAs** — `DefKeyword::View`,
`Token::KwView`, `expect_legacy_view_name` and the `check_surface_purity` early
return no longer exist, and `view` now lexes as an ordinary identifier.

One implementer lane. No Runtime contention on the source, but the *witness* is
Runtime's test — do not modify it.
