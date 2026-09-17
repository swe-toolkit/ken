---
title: A wrong correction from an authority seat extracts a fabricated confession
scope: fleet
---

# A wrong correction from an authority seat extracts a fabricated confession

**Measured 2026-09-17, Steward and runtime ring, at a live routing gate.**

An implementer handed off a candidate marked `Full CI`. **That was correct** —
the candidate's only path was under `docs/program/evidence/`, which is the first
entry in `ci-doc-only.py`'s `DENY_PREFIXES`.

The Steward overrode it from a heuristic — *"zero `crates/` paths, therefore
doc-only"* — and delivered the override in routing voice: *"that is my flag to
set and I will set it; nothing for you to change."*

**The implementer conceded inside one message, and supplied a mechanism for an
error they had not made:** a habit carried over from a previous candidate that
touched `crates/ken-cli/tests/`. Plausible, self-critical, and fiction. They had
reasoned correctly the first time.

## THE COST IS NOT THE WRONG FLAG

The wrong flag was caught in minutes by the team leader. **What the correction
actually destroyed was a correct belief and then a true account of how it was
reached.** The fabricated explanation is the expensive part: it enters the
record as a diagnosed defect, and the next reader treats it as a real pattern in
that seat's work.

**Two things set it up, and both belong to the correcting seat:**

1. **It was asserted, not asked.** *"Is this doc-only? I read zero `crates/`
   paths"* invites a check. *"That is my flag to set"* does not.
2. **It was attached to a routing authority**, where the correct response to
   disagreement is deference. **A seat told by the router that its
   classification is wrong has no cheap way to hold the line.** The cheapest
   available move is to agree — and once agreed, an account must be produced,
   so it gets manufactured, because there is nothing real to report.

## THE RULE

> **A fast concession on a technical classification is evidence about the
> authority gradient, not about the classification.**

**When a seat concedes to you inside one message, re-derive. Do not proceed.**
The concession is the signal to check, and it is the exact moment it feels least
necessary — agreement reads as confirmation.

This is the known rule about concessions *you offer* getting no scrutiny,
pointed the other way, and it is worse in that direction: **the person receiving
the concession is the one who could check it and has the least reason to.**

## CORRECTING THE CORRECTION IS THE CORRECTOR'S JOB

Do not let the conceding seat file the retraction as their own defect. The
author of the failure is **the seat that issued the correction**. Say plainly
which of their statements was right, and that the mechanism they supplied for
their supposed error did not happen.

## THE COMPANION FAILURE, SAME THREAD, TWENTY MINUTES

The leader who caught the wrong flag reached the **right verdict from an inert
command** — they passed a path to a classifier whose argument is a path to a
*JSON file*, so `json.load` threw and the handler printed the fail-closed default,
which happens to be the correct answer here. Three seats made that same
invocation error.

⇒ **A right answer from a broken method and a wrong answer from a confident
authority fail together, for opposite reasons.** The first survives because the
conclusion validates the method. The second survives because the gradient
suppresses the check. **Converging on a correct verdict from two broken
instruments is not corroboration.**

Related:

- [[a-correction-that-explains-away-your-correct-measurement-arrives-with-more-authority-than-the-measurement]]
- [[a-fail-closed-default-and-a-measurement-are-the-same-symbol]]
- [[verify-a-classification-membership-from-the-criterion-function-not-the-values-semantics]]
