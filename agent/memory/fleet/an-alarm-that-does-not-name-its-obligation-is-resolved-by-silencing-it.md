---
name: an-alarm-that-does-not-name-its-obligation-is-resolved-by-silencing-it
description: A successor test was framed to red when a coupling needs attention, but its subject is the quoter while the obligation lives in the collector — so the author who trips it reads the assertion as obsolete and deletes it, never opening the file the coupling protects
metadata:
  type: feedback
---

# An alarm that does not name its obligation is resolved by silencing it

**Measured 2026-08-16 on `2b4ad0faa..2c47f4b2d`.** A comment claimed a criterion
*"forces"* a future update; the merge knew that was an overclaim and framed a
successor: assert `quote_iform` refuses `top_id`, so the coupling reds when
someone adds the arm.

⇒ **The trigger is exactly right and the SUBJECT is wrong.** The test is about
the **quoter**; the obligation is in the **collector**. An author who has just
added the quoter arm reads *"quote_iform must refuse top_id"* failing as
*"correct, I just changed that — the assertion is obsolete."*

⇒ ***The cheapest resolution of a red is to delete the red.*** **An alarm that
does not name the obligation it guards routes the author to the assertion, not
to the duty** — so it converts a silent future defect into a loud misrouted one.
**Better, but not enforcement.**

⇒ **The repair belongs in the AC, not the mechanism**: require the failure
message to state the obligation — *"if you are adding a quoter arm for this
constant, the collector must exclude it under the criterion."* ⇒ ***When
reviewing a proposed alarm, simulate the person who trips it.*** Ask what their
cheapest correct-looking action is; if that action does not include opening the
guarded file, the alarm needs a sentence, not a stronger assertion.

## A CONCLUSION CAN BE RIGHT WITH THE MECHANISM DESCRIBED BACKWARDS

The same comment said a second constant is safe because *"both the collector and
the quoter refuse together, so there is no disagreement to over-collect into."*
**The collector does not refuse it — it collects it**, and the refusal comes
from the ambiguity check: **the over-collection path**, exactly as the fixed
constant behaved before the fix.

⇒ **The conclusion holds for a different reason**: it costs nothing because that
constant is not quotable, so no quotable obligation is lost. **The fixed one was
quotable; that is the entire difference.**

⇒ ***A prose sentence can invert the very distinction its own paragraphs
establish.*** The surrounding text spent its length separating under- from
over-collection, then described an over-collection as a joint refusal. **Read a
justification against the code even when the justification is for a conclusion
you agree with** — agreement is what stops you checking.

## VERIFY A "PINS DIFFERENT ARMS" CLAIM BY MUTATING THEM SEPARATELY

Two rows were required mutation-proven **one arm each**, since a single mutation
reddening both would not show they pin different things. ⇒ **Run both mutations
independently and read the LABELS**, not just the pass/fail. Here each reddened
exactly its own row — and because the second mutation failed at the *later* row,
the earlier row demonstrably passed under it, giving independence in both
directions from two runs.
