---
id: LANG-RECORD-STACK-OVERFLOW
title: "The record-literal surface work aborts a real `ken-cli` native compilation with a stack overflow -- `mrc_4a_cross_crate_census_and_its_controls` SIGABRTs at every SHA of the arc including the one carrying the 143-line stack rework, so the rework is not the repair; the arc's own depth fixture never detected it because it builds match arms with `=>`, which is not a Ken token"
status: merged
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: Steward measurement 2026-08-13 after publishing 50da348a as PR #2092 and having CI reject it. Refutes the structural argument in Architect evt_1f9z6akt6vrj5 that the change adds "time, not stack". Supersedes the never-released frame LANG-PARSER-DEPTH-HONEST, whose premise (that no stack problem was measured) measurement has overtaken.
---

## The defect

```
SIGABRT (23/785) ken-cli::mrc_4a_cross_crate_census
                 mrc_4a_cross_crate_census_and_its_controls
fatal runtime error: stack overflow, aborting
(test aborted with signal 6: SIGABRT)
```

The test is `RT-MATCH-RECURSOR-CONSUMERS` `AC-1`'s cross-crate census — a real
`ken-cli` native compilation of a large program, not a synthetic nesting
fixture. It is an **existing, landed, unrelated test** that the record-literal
work breaks.

## Four facts, each from a run

| fact | evidence |
|---|---|
| `50da348a` has **never** been CI-green | run `31550144839`, 2026-08-12, same test, same abort |
| it is **not** inherited from `main` | `main` at `cbe58725` is green |
| **the 143-line rework does not fix it** | at `8e9baa18`, which contains `766c9f07`, the same test aborts (2026-08-13T05:29) |
| `766c9f07` was never CI'd alone | no check runs exist for that SHA |

⇒ **`766c9f07` "bound record parser stack use" is an attempt, not a repair.**
It must not be carried forward as though it were measured.

## Why nothing in the ring caught it

Two independent blind spots, and both are the interesting part:

1. **The arc's own depth control never ran.**
   `crates/ken-cli/tests/record_literal_parser_depth.rs` builds nesting with
   `format!("match 0 {{ _ => {body} }}")`, and **Ken's lexer has no `=>`
   token** — the match-arm separator is `MapsTo`, spelled `|->` or `↦`
   (`lexer.rs:107`). The source dies in the lexer on the first arm; the parser
   never reaches depth one. `NESTED_MATCH_DEPTH = 31` is inert. It fails
   identically at `57688110`, `50da348a`, `766c9f07` and `8e9baa18`, always for
   that reason.
2. **Local targeted runs cannot see it.** QA's greens (42/42 focused, 30/30
   match regressions) are real and were run faithfully — on `ken-elaborator`.
   The failing test is a heavy `ken-cli` native compile, which no targeted
   `-p ken-elaborator` run reaches. **The gate is CI, and this is what that
   rule is for.**

**The leader's branch status attributed `8e9baa18`'s red to the depth control.**
That is a true statement about one red that made a second red underneath it
read as accounted for.

## The refuted argument, recorded because it was offered as the reason to merge

Architect `evt_1f9z6akt6vrj5`: *"the record change adds no stack frame to
nested-`match` parsing, because `brace_starts_match_arms()` breaks the argument
loop before `parse_record_expr` is entered… what it adds on that path is a
bounded forward token scan — time, not stack."*

Measurement refutes it. The Architect asked for exactly this confirmation-or-
refutation, so this is the answer to a question that was posed, not a
retrospective complaint. **What it does not touch:** the design pass on ACs 1-8,
which stands.

## Not this node

- **Raising a stack limit.** `RUST_MIN_STACK` and friends are refused. The
  gate-4a arc next door repaired a stack regression by reducing footprint, and
  that is the standard here.
- **Tuning `NESTED_MATCH_DEPTH`.** The constant is not what is broken.
- **Building a new synthetic depth fixture.** The honest witness already exists
  in CI.
- **`view` retirement** — `LANG-VIEW-RETIRE`, which this blocks only because
  both touch `parser.rs` and the ring has one implementer lane.
