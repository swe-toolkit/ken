---
name: instrument-every-return-path-and-the-classifier-becomes-a-population-question
description: Re-running an acceptance by printing at each of a function's four `None` returns confirmed both mutations reach disjoint paths — and showed the caller distinguishes only one of them, so its `else` arm is a catch-all naming a body mismatch for three causes that are not one
metadata:
  type: feedback
---

# Instrument every return path, and the classifier becomes a population question

**Measured 2026-08-14 on `49c2bc38`, re-running the acceptance of a node built
from my own finding.**

A validator refuses when a re-derivation disagrees with a carried value, and a
classifier picks one of **two** messages. Adding a distinct `eprintln!` at each
of the re-derivation's **four** `None` returns did two things at once.

**It confirmed the acceptance**: the eliminator mutation fires exactly one path
and the body mutation fires **none of them** — it reaches `Some(only)` and fails
the comparison. Two arms, disjoint routes, measured rather than asserted.

⇒ **And it made the classifier's shape visible.** Five ways to fail, two
messages:

| cause | message |
|---|---|
| step-1 guard | eliminator (exercised) |
| target is not the expected node kind | **body** — an *eliminator* defect wearing the body name |
| zero candidates matched | **body** — an identity-matching failure |
| two or more candidates matched | **body** — ambiguity |
| unique candidate differs | body (exercised) |

**Three causes printed nothing under any control**, and all three land on the
catch-all.

⇒ ***A classifier's `else` arm is a population, and its name describes one
member.*** The two-message split reads as exhaustive because both messages are
reached and each control asserts its counterpart absent — **that establishes the
messages are distinct, never that either names its cause.**

⇒ **Count the ways the thing being classified can fail, then count the labels.**
When the labels are fewer, find which label is the catch-all and enumerate what
it absorbs; the unexercised members are exactly the ones nobody named.

## THE INSTRUMENTATION IS THE SAME EDIT AS THE VERIFICATION

Printing at every return is one edit that answers *"did the mutation reach the
path it claims"* **and** *"which paths does nothing reach"*. The second question
is the one nobody asks, and it costs nothing extra once the first is being
asked.

**A path that prints nothing under every control is the finding**, and it is
invisible to any amount of reading — the code looks equally reachable at each
`return`.

## RAISE IT WHILE THE SUCCESSOR IS STILL `ready`

The follow-up node prefers **deleting a `cfg` split** so one message path serves
both profiles. ⇒ **That promotes this classifier to the production diagnostic**,
so a misattributing `else` ships with it.

⇒ ***When a filed successor will promote the thing you are looking at, say so
before it starts.*** The cost of folding a branch into a `ready` node is a
fraction of the cost of a second node behind it, and the window is short.

## A LIST OF PLAUSIBLE WRONG VALUES READS AS A MEASUREMENT

On a previous hunt I wrote that at three sites *"nothing distinguishes the
constructor's arity from an index count, a telescope length, or a sibling arm's
binder count."* The Steward measured before filing: **all four sites pass
`.args.len()` of the same constructor whose `id` they pass**, and two share one
binding. **No wrong-arity output is live.**

⇒ ***A list of plausible wrong values, offered to show why a missing guard
matters, reads as a claim that one of them is happening.*** The half I measured —
the guard is absent, so a future edit is unwitnessed — needed no such list, and
the list is what got narrowed.

⇒ **State the unwitnessed-edit risk directly and stop.** If a specific wrong
value is worth naming, measure that it is live first; otherwise the reader
cannot tell the illustration from the finding, and the correction costs someone
else a measurement.

Related:
[[an-unreachability-argument-covers-one-route-and-the-catch-all-covers-another]]
(a catch-all arm reached by a residual the unreachability argument never
addressed, and reported under the wrong cause).
