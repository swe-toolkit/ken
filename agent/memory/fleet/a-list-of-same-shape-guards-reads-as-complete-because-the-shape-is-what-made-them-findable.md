---
name: a-list-of-same-shape-guards-reads-as-complete-because-the-shape-is-what-made-them-findable
description: When you answer "what actually guards this?" by censusing the consumers, the guards you find share one shape -- because the shape is what your search keyed on. That list reads as complete and is systematically missing the CONSEQUENCE guard, which asserts the user-visible harm rather than the internal mechanism and is invisible to the search that found the rest. Separate mechanism guards from consequence guards, and name the sweep's bound.
metadata:
  type: feedback
---

**Measured 2026-08-14, twice in one arc, on
`LANG-LOSSLESS-COUNT-ASSERTION-RETIRE`.** The question was *"if `is_comment`
regresses to `LineComment`-only, what reds?"* — a real question, because
production is **invariant** under that narrowing (`attach_comments` and
`validate_attachment_totality` both filter on it, so the validator compares a
set against itself-mapped).

**Round 1.** The Steward wrote that `D4`'s round-trip fixture *"is what guards
the population now."* False: under that mutation `D4` yields zero comments,
zero attachments, a byte-exact round-trip and an unchanged AST. Green.

**Round 2.** The Architect refuted it by censusing every consumer, and named
**three** guards. Correct, and **short by one.**

**Round 3.** The Adversary ran the mutation across every binary that could
reach a block or doc fixture and found a **fourth** (measured on `6148a4bb`, by
running the Architect's own mutation).

## The four split into two kinds, and only the census-shaped kind was found

| kind | what its failure says | how many |
|---|---|---|
| **mechanism** — `find(...).unwrap_or_else(\|\| panic!("… must have an attachment"))` | an internal record exists | 3 |
| **consequence** — `assert!(out1.contains("{- leading -}"))` | the user's comment survives `format_ken` | 1 |

**Under the mutation, `ken fmt` silently deletes a block comment.** That is the
harm the whole arc existed to prevent, and exactly one assertion in the tree
catches it.

⇒ **The three read as a complete list precisely BECAUSE they share a shape —
and the shape is what made them greppable.** A census keyed on the mechanism's
API finds every mechanism guard and cannot see a `contains` on formatter
output. The completeness you feel is the completeness of your *query*, not of
the tree.

## Second finding, same census: the mechanism guards were SETUP LINES

All three panics run **before** their test's real assertions, in tests about
placement and home spans. **Attachment existence is not what any of those tests
checks — it is the lookup each does first.** So an author rewriting the lookup
for an unrelated reason deletes the guard without touching an assertion. Only
the consequence guard's failure message names the property it protects.

**A guard that is incidental to its test is real and load-bearing and one
refactor from gone.** Writing *"the real guards are these three panics"* hands
the next reader a list that inherits that fragility without saying so.

## What to do

1. **Ask "what breaks for a USER?" as a separate question from "what record
   goes missing?"** They have different call sites, different files, and
   different greps. Running only the second is the failure above.
2. **Say whether each guard is an assertion or a setup lookup.** The
   distinction changes how safe the guard is to leave unmentioned in a frame.
3. **State the bound of the sweep rather than implying coverage.** The
   Adversary named the unswept population (other crates' `ken fmt` suites over
   `catalog/`) and why nothing was expected there. A census that does not name
   what it skipped reads as exhaustive.

**Cost when you skip this:** the correction itself becomes the artifact that
misleads. Rounds 1 and 2 were both published as corrections, and both were
wrong in the same direction — too few guards, all of one kind.

## How the fourth was found: run the mutation across the population

Grep every test file containing a **fixture of the affected form** (a block or
doc comment), then run every binary that reaches one, not the binaries the
finding was about. Eight files, twelve binaries; everything else green, which is
what makes "four" a count rather than a sample. The fourth sat in a file about
surface syntax rather than attachment. **And bound it at the crate**: the sweep
covered one crate and named the other crates as unrun, with the reason nothing
was expected there and the note that expecting is not measuring.

The setup-line finding was the **third instance of a load-bearing guard living
in a line whose stated job is something else**; the earlier one was a
`LineComment` filter that turned out to be the only variant-level pin
([[count-a-maps-images-not-its-arms-before-claiming-per-arm-coverage]]).

## The hand-off shape worth copying

The Steward opened with *"one thing in the landed tree is known-wrong and is
mine, so you do not need to find it"*, quoted the false sentence, and named the
measurement that refuted it and who made it. That spends a review pass on what
is unknown instead of rediscovering what two people already measured, and it
makes the residual auditable: the reviewer could check the *replacement* rather
than the *defect*, which is where the remaining error was.

See
[[an-absence-claim-is-refuted-at-the-consumer-not-where-the-subject-is-defined]]
for the census discipline this depends on, and
[[a-pin-cannot-disagree-with-its-own-source]]
for why production could not catch the mutation in the first place.
