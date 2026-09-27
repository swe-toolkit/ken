---
name: elaboration-proves-writability-and-cannot-prove-unwritability
description: A probe established three capabilities by elaborating `.ken` rather than reading source, which is right for the two positive claims and cannot support the third — a negative existence claim over surface syntax, established by two failed spellings, and it is the hard stop the node turns on
metadata:
  type: feedback
---

# Elaboration proves writability and cannot prove unwritability

**Measured 2026-08-16 on `bfff4290d..c367ceb13`.** A buildability probe stated
its method explicitly: each part established *"by actual `.ken` elaboration --
not by reading the kernel or the elaborator source."*

⇒ **Right for the two POSITIVE claims** — elaborating a thing proves it is
writable, and reading source could not. ⇒ ***Wrong for the one NEGATIVE claim***
— *"`‖A‖` is not writable in surface syntax"* was established by **two failed
spellings**, and two failures are consistent with a third succeeding.

**And that was the HARD STOP, the result the node turns on** — plus the
premise of the follow-on node filed to add the syntax.

⇒ ***A method chosen because it is stronger than reading source is weaker than
reading source for any claim that is negative.*** **Check whether a uniform
method is being applied across claims of different polarity**; the mismatch is
invisible while the conclusion is correct.

⇒ **The grammar settles it in three greps**: no lexer/parser production, no
surface-to-term mapping in the elaborator, and the one such term in the tree
built directly in Rust rather than elaborated. **Same conclusion, sound
footing** — so file it as *"correct, and here is what actually establishes
it"*, not as a refutation.

## VERIFY EVERY CLAIM SOMEONE FLAGS AS THEIR OWN AND UNCHECKED

Three were flagged, all three held. ⇒ **Report the confirmations with their
evidence**, not just the exceptions — the point is that they are now checked.

**Give a claim two confirmations when its NAME argues against it.** A
constructor called `TruncProj` reads as an eliminator; the type doc spells it
`|t|` and the kernel's own error text lists it under *"cannot infer an
introduction form"*. ***A future reader will re-litigate on the name, so cite
the second source now.***

## A COUNT NEEDS ITS PREDICATE, NOT ONLY ITS REFS

*"Three construction sites"* is right on the **origination** reading; a naive
grep for the constructor returns **14**, the rest being structural rebuilds
inside traversals and test fixtures. ⇒ **After base/tip/count, the fourth field
is the PREDICATE** — a count reproducible only by guessing what was counted is
the same defect as one reported against an unlabelled ref.

Related: [[no-instrument-exists-is-a-claim-about-the-space-you-enumerated]]
(another negative existence claim resting on a hand-enumerated set of attempts).
