---
id: CI-DOCTEST-UNEXECUTED
title: "CI runs no --doc step on a premise that is false -- doctests are collected but never executed, and the positive control for a 20-block compile_fail set is among the dead ones"
status: merged
owner: verify
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary finding evt_142d56sf2y2f7 on origin/main 376d495d, answering a lead the Steward supplied in the CI-ROW-CLAIM-COMMENT-FORM merge notification (evt_73xs2xgm0gjxw): that the widened extractor's attachment boundary is a parser's, so the classes outside it are where to look. Three of the four named classes had zero inhabitants; the fourth is real. Both documentary facts independently re-grounded by the Steward at 376d495d. Steward-filed (agents cannot create tracked work per COORDINATION §2)."
---

> ## Frame: `docs/program/wp/CI-DOCTEST-UNEXECUTED.md`. `ready`, shovel-ready.
>
> No dependencies. The fixed inputs are documentary and are quoted in the
> frame's §2 rather than owed as a first deliverable. **The census is NOT
> pre-supplied and is deliverable `D1`** — see the trap below for why the
> Steward did not run it.

## The defect in one line

`.github/workflows/ci.yml:124-130` declines to run `cargo test --doc` on the
stated premise that the workspace has **zero** collectible doctests. **The
premise is false.** `cargo test --doc -p ken-runtime -- --list` reports **14
tests**, and CI executes none of them.

## Why this is worse than dead coverage

`crates/ken-runtime/src/values.rs:126-129` is a claim about **executed
evidence**, offered as the justification that a capability is real:

> *"It also **runs**, so the capability is shown to be genuinely available
> rather than merely well-typed"*

That sentence exists to discharge a specific, correct worry stated two lines
above it: a `compile_fail` block **passes for any compilation error**, including
a mistyped path or a missing import, so a negative-only control set establishes
nothing. `ken-runtime` carries **20** `compile_fail` occurrences.

⇒ **The positive control for that set is itself never executed.** The one block
whose job is to prove the negatives failed for the stated reason is dead, so the
distinction the sentence draws — runs versus merely well-typed — is exactly the
one not established.

## The mechanism the CI comment missed

The comment reasons about fence **markers**, and records its own method:
*"checked via `grep -rn '```' crates/ --include=*.rs` — every opening fence is
`text`."*

**`values.rs:130` opens a bare ` ``` ` with no marker at all, and rustdoc
defaults a marker-less fence to Rust.** A grep looking for markers cannot see a
fence that has none.

## The trap, and the Steward walked into it while filing this

A first pass counted marker-less fence lines per crate and got `ken-runtime` 37,
`ken-elaborator` 9, `ken-kernel` 1. **Those numbers are not a census and are not
in this node**, because the pattern matches **closing** fences too — and a block
opened ` ```text ` closes with a bare ` ``` `. There is no way to separate
openers from closers by line-local matching.

**That is the same error class as the CI comment**, reproduced one artifact
later by someone who had just finished reading why it was wrong. It is recorded
here so the node's first instinct is the right one: **ask the collector, not the
corpus.** `cargo test --doc -p <crate> -- --list` is the instrument.

It does establish one thing the Adversary's report did not: **bare fences are
not confined to `ken-runtime`.** The true workspace count is unknown and is `D1`.

## Two boundaries this node does not cross

**Do not add the CI step before the census.** If any of the collected doctests
fail, adding the step reds `main` and blocks the publisher for the whole fleet —
a mechanical blocker, not the "a working path would go red" intuition the
2026-07-28 no-users ruling retired. Measure first, then gate.

**Do not resolve a failing doctest by deleting it.** A doctest that fails is a
finding; deleting it reproduces this defect in a quieter form.
