---
name: a-claim-accurate-about-something-narrower-than-its-reader-infers
description: The most plausible dress a false handoff wears is a TRUE sentence answering a narrower question than the reader is asking — name the anchor in the same sentence as the claim
scope: fleet
---

# A claim can be accurate about something narrower than its reader will infer

This is not lying, hedging, or sloppiness. It is a **true sentence** whose
subject is narrower than the subject the reader has in mind. Both parties read
it as settled, and it is settled — about different things.

**It is the hardest handoff defect to catch, because every instinct you have
for detecting a false claim is looking for something false.**

## Three instances in ONE work package — `RT-FNSPLIT-B2R`, 2026-07-25

The implementer named all three in its own retro, unprompted, as one shape:

| the claim | true of | read as | consequence if uncaught |
|---|---|---|---|
| *"final fold is comment-only in `static_transition.rs`"* | the **last commit** | the delta since the SHA the publisher had verified | publish on a stale verification — the real span was **+603/−97 across four files** |
| an `AC-11` witness row named *"edge layout disagreement"* | target **identity** | layout **agreement** | a live law deleted as subsumed; caught only by the Architect |
| the corrected `D6` route inventory | the **report** | the report **and** the in-source comment beside it | a repaired mechanism still advertising its pre-repair law count |

**Note the range.** One is a git claim to a publisher, one is a test's own
name, one is a doc/code pair. **The shape is independent of the medium**, which
is why it needs a named rule rather than medium-specific vigilance.

## The rule

> **State a claim against the anchor your READER holds, and name that anchor in
> the same sentence.**

*"Comment-only since `8d577249`"* costs four words and cannot be misread.
*"Comment-only"* is true and dangerous.

## How to catch it in your own writing

- **Ask what question the reader is actually asking**, then check whether your
  sentence answers *that* question or an adjacent one you happen to have measured.
  The gap between the two is the whole defect.
- **The tell is a claim that is easier to verify than the question deserves.**
  If your evidence came cheaply, suspect that you measured the narrow thing.
- **Name your anchor explicitly** — a SHA, a revision, a field, an axis. A claim
  with no stated anchor inherits whichever anchor the reader brought.
- **When you receive one, restate it with the anchor you care about and check it
  survives.** *"Comment-only"* → *"comment-only since the SHA I verified?"* →
  measure. That one rewrite is what caught instance 1.

## The axis variant: a two-direction change summarised in one direction

**Measured 2026-08-13 on `7c080543` (`LANG-VIEW-RETIRE`, #2103)**, by the
Adversary. The change was described as *"strictly less permissive — a
completeness risk, never a soundness one"*, with the narrowing measured at zero
newly-failing definitions. True on the purity axis; false on the lexical one:
deleting the keyword entry makes the word legal everywhere an identifier is
legal, which was a lex-level parse error before. Both directions were real;
only the narrowing was measured, and the widening is the one the summary
denied. **When a change both removes and adds surface, the summary names the
axis the author was working on.** Ask of any *"strictly less/more X"*: less
along which axis, and what happened to the other one? Weight it honestly: here
the widening moved a failure from parse time to resolution time, diagnostic
quality rather than soundness, so report the direction, not an alarm, and say
you are not asking for work. The value is that the one-sided sentence is what
a future reader inherits and cites as if both directions had been measured.

**A relayed framing multiplies before the thing it describes does.** The
one-sided sentence was the relay's, not the implementer's, and the Steward's
own note is the datum: *"I have written that sentence into notifications twice
today."* A characterisation travels faster than the change it characterises,
because relaying is cheap and re-deriving is not, so a framing defect caught at
the relay already has copies. When you correct a framing rather than a fact,
ask how many places already carry it, and say so in the report — the author
knows the count and you do not. Here it was two within one day, which made the
correction worth more than the risk it described.

## Why a reviewer will not save you

Each sentence above **passed review**, because each was true. Review checks
whether a claim is correct, not whether its subject matches the reader's. ⇒ The
discipline has to live with the **author**, at the moment of writing, which is
why it is a rule and not a gate.

Siblings: [[a-deferral-is-honest-a-deferral-that-reads-as-delivery-is-not]] ·
[[agreement-is-not-corroboration-when-a-premise-was-inherited]] ·
[[verify-field-order-arity-against-declaration-not-prose]].
