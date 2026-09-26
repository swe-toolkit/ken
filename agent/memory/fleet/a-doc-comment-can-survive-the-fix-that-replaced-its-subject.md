---
name: a-doc-comment-can-survive-the-fix-that-replaced-its-subject
description: A prose node framed as repointing a wrong spec citation actually corrected a description of a mechanism that had been retired by an earlier fix — and the evidence was a regression comment in the very file the node edited, saying the checker "used to" work the way the doc still claimed
metadata:
  type: feedback
---

# A doc comment can survive the fix that replaced its subject

**Measured 2026-08-14 on `c2f285ee`, a prose-only node framed as a citation
repoint.**

The old doc: *"a redundant match arm (`34 §5`): the arm's **constructor** was
already covered."* Two defects, and only one was named.

- The citation was wrong — `§5` is *Refinement types*; reachability is `§4.2`.
- **The sentence described a per-top-level-constructor seen-set**, and the
  implementation is an `arm_used` matrix walk at three sites.

⇒ **The seen-set had been retired by an earlier gap fix, and the evidence sits
in the file this node edited**: a regression comment reading *"the reachability
checker **used to** track coverage by top-level constructor only, so `Suc Zero`
and `Suc (Suc m)` (sharing the `Suc` head) falsely tripped."*

⇒ ***A doc comment survives the fix that replaces its subject, because a
mechanism change edits the mechanism and the prose is somewhere else.*** The
node is an **as-built correction**, not a citation repoint, and the stronger
framing is the one a later reader needs — otherwise the repair reads as
bookkeeping.

**The tell is a "used to" sentence anywhere in the change's neighbourhood.**
A regression comment naming a retired mechanism is a dated pointer at every
other place that mechanism was described. **Grep the retired mechanism's own
words, not the file.**

## TRY TO BREAK A PROSE REPAIR THE STANDARD WAY, AND SAY WHEN IT HOLDS

The standard failure of a node fixing a false citation is to reach for the
spec's wording and **overshoot the code** — replacing a false description with
an aspirational one. Here it did not: the spec sentence and the `arm_used` walk
are the same property, and the distinction has **both** controls — a positive
(a genuinely redundant arm reds) and a **negative** (two arms sharing a
constructor head must *not* red).

⇒ **The negative control is the one that separates the two candidate
mechanisms**, and its existence is what makes the stronger prose safe. **When
checking whether corrected prose overshoots, look for the control that
discriminates the old description from the new one** — if it exists, the new
wording is earned.

## A CORRECTED COMMENT CAN BE CONTINGENT ON A FEATURE NOT EXISTING

The spec clause the comment now quotes names two subtleties — guarded arms, and
literal columns that never close. **Neither is constructible**: the arm struct
has no guard field, and the pattern kind is exactly `Wild | Var | Ctor`.

⇒ **That is why nothing diverges — and it means the comment's accuracy is
contingent on two surface features not existing.** Adding either makes both
caveats live at once, and the walk would need a guard exception it does not
have.

⇒ ***When a doc quotes a spec sentence with caveats, check whether the caveats
are reachable; if they are not, say the accuracy is contingent*** — the person
who makes them reachable will be reading the spec, not this file.

**Enumerate the variants rather than trusting a grep that found nothing.** I
confirmed the pattern kind exists and listed its three variants before claiming
literal patterns are absent — a failed grep on a wrong spelling and a genuine
absence are the same output.
