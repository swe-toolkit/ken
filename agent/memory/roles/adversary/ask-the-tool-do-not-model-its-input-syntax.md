---
name: ask-the-tool-do-not-model-its-input-syntax
description: A claim about what a tool collects, emits, or rejects is settled by running the tool — reasoning from input syntax is a model that drifts, and my own syntactic probe was wrong in the same direction as the comment it was auditing
scope: roles/adversary
---

# Ask the tool; do not model its input syntax

**Measured 2026-08-10 on `376d495d`.**

A CI comment reasoned its way to a conclusion about a tool:

> *"Every ``` fence inside [a doc comment] carries an explicit non-Rust marker …
> so `cargo test --doc` collects zero tests."*

Premise about **input syntax** ⇒ conclusion about **tool behaviour**. I audited
it by scanning fence markers myself and found **10** collectible opening fences.

**Running the tool: `cargo test --doc -p ken-runtime -- --list` reports 14.**

Three numbers — the comment's 0, my probe's 10, the collector's 14 — and only
the third is the answer. My probe was wrong **in the same direction as the
comment I was auditing**, because I reproduced its method: modelling the input
instead of asking the thing that consumes it.

⇒ **Whenever a claim is about what a tool collects, emits, accepts, or rejects,
the tool is the oracle.** Any argument from the shape of its input is a
reimplementation of its parser, and a reimplementation is exactly as good as
your memory of the rules — which is what
[[a-hand-written-forbidden-character-class-reports-clean-about-only-the-characters-you-remembered]]
already says about hand-written classes, arriving here as a *count*.

The tell is a comment containing a derivation: *"every X is Y, so the tool does
Z."* Two premises and an inference where one command exists.

## Dead test coverage is its own defect class

The 14 doctests are **collected and never executed** — CI runs no `--doc` step.
That is worse than a missing test, because it reads as coverage: the examples
are present, they look maintained, and nothing reports that they never ran.

It became a **false claim** rather than a mere absence at
`crates/ken-runtime/src/values.rs:129`, whose prose justifies a capability with
*"It also **runs**, so the capability is shown to be genuinely available rather
than merely well-typed."* The runs-versus-well-typed distinction is exactly the
one not established. ⇒ **When you find an unexecuted test surface, grep it for
prose asserting that it executes** — that converts a coverage gap into a
groundable finding sited in the code.

And check the instruction the false premise supports: here it was *"if a real
doctest is ever added, add the `--doc` step back."* A maintainer following it
gets 14 pre-existing tests of unknown status, not the one they wrote. **A stale
premise usually has an instruction resting on it, and the instruction is where
the cost lands** — same as
[[a-retracted-number-survives-in-its-consumers-with-the-direction-flipped]].

## Bound the measurement to what you ran

I listed the doctests; I did not run them, so I claimed nothing about whether
they pass. I measured one crate, so the count is **"at least 14"**, not 14 — the
premise was workspace-wide and a workspace `--doc` run is CI's, not mine. Say
both, or the number reads as complete.
