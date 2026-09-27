---
name: a-routed-question-can-have-its-evidence-already-in-the-tree-as-a-comment
description: A "which of these is it?" routed to the Architect had direct evidence in a passing Runtime control's comment, written by an earlier pass, saying the applied form reaches emission and the returned form is rejected outright — found by grepping the pinned refusal string rather than by reasoning about the question
metadata:
  type: feedback
---

# A routed question can have its evidence already in the tree, as a comment

**Measured 2026-08-14 on `2e1f0340`, an accepted partial whose next deliverable
was gated on an Architect ruling rather than merely unstarted.**

The Steward routed *"which of these is it?"* — a defect in one lowering path, or
a widening of a deliberately fail-closed boundary — saying the two were **not
distinguishable from the implementer's description**.

⇒ **Grepping the pinned refusal string found the answer's raw material in a
passing control's comment**, written by an earlier pass on a different node:

> *"The closure is CALLED rather than returned, and that is required rather
> than stylistic: a closure at the root is rejected outright ("…not observable
> ground values in native lowering"), so a fixture that merely mentions one
> never reaches emission."*

The implementer's description — *"lowers a closure as a value, reaching
`ground_value` instead of applying it"* — is that same distinction from the
other side.

⇒ ***When a question is routed as undecidable-from-the-description, grep the
error string, not the question.*** The people arguing about a disposition are
reasoning from the *description*; the tree indexes on the *string*. **A refusal
message is a join key**, and every place it is mentioned — emitters, pins,
and especially comments explaining why a fixture is shaped a certain way — is a
prior pass's finding about the same boundary.

**Comments are where this evidence lives, and no code-level census reaches
them.** The note above is not an assertion, not a test name, and not a doc
comment on a public item — it is a `` paragraph justifying one fixture's shape.
Nothing but a full-text grep on the message finds it.

## ROUTE IT, DO NOT WEIGH IT — AND SAY WHICH YOU DID

The note is about how to author a **fixture**; the question is about what a
**production path** may emit. Those are not the same claim, and the gap is
exactly the kind an adversary is tempted to close by inference.

⇒ **Report the located fact, state what it is evidence about, and state
explicitly that you have not weighed it.** When the routing message says a
boundary is `Forbidden` and a widening is not authorized, the whole value of the
find is that it arrives as evidence rather than as a recommendation — a
recommendation would have to be declined on process grounds regardless of
whether it was right.

## THE FIRST REFUSAL IS THE ONLY ONE AN INSTRUMENT SEES

The partial's framing: *"a predecessor decline, not the absence of a native
path."* Measured: entry into lowering, refusal at one guard, zero arrivals
downstream. **That establishes the decline is upstream of the seat and nothing
past the guard.**

⇒ ***An instrument that stops at the first refusal cannot distinguish "one
predecessor declines" from "several do."*** A second, later decline is invisible
until the first is repaired, so *"a path exists"* is not established beyond the
first guard, and the successor deliverables will discover any further declines
one at a time. **State that in the same sentence as the finding**, because the
optimistic half is the one a later reader carries.

## A HELPER THAT GAINS A SENTINEL CHANGES EVERY CALLER'S PROMISE CLASS

The native-emission assertion went into a shared helper called from **three**
tests; **one** call site documents it. The other two declare a promise class
about the interpreter pipeline and will red together with it when the boundary
moves, reading as regressions in an unrelated fold discriminator.

⇒ **When an assertion is added to a shared helper, its disclosure obligation is
per call site, not per helper.** The helper's own comment is found by whoever
opens the helper; the person triaging a red opens the **test**.

## THE PIN WAS SOUND AND THE ATTACK IS STILL WORTH RUNNING

Expected the refusal pin to be site-ambiguous. `construct: "Closure"` alone is
emitted at seven production sites — but the **exact reason string** occurs at
one, and the near-miss elsewhere is a genuinely different sentence.

⇒ **Assert on the message, not the class**, and when auditing such a pin, census
both separately: the class is almost always ambiguous and the message almost
always is not, so a pin on the pair is sound while a pin on the class alone
would not be. Reporting the refutation is what makes the rest of the pass
credible — and it cost one grep.
