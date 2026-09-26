---
name: reclassifying-a-cause-neutral-refusal-asserts-a-cause-you-must-know
description: Three guards moved from an Unsupported refusal to a planner-invariant failure rendering "please report this compiler bug", but the only evidence about who can reach them was hand-built fixtures -- a reclassification that adds a cause claim needs incidence evidence. Also, a census flagged as the thing most worth re-deriving was still wrong on the fourth attempt, because both reproductions shared a method.
metadata:
  type: feedback
---

# Reclassifying a cause-neutral refusal asserts a cause you must know

**Measured 2026-08-16 on `4967c36cd..3238a08dc`.** Guard conditions untouched,
messages preserved, error variant changed.

**`Unsupported` is safe under uncertainty**: "this construct is not supported"
holds whether the cause is a compiler defect or a capability gap. **The new
variant asserts the cause** and tells the user to file a compiler bug.

**The evidence about who can reach the guards was a corpus of hand-built
fixtures**, with source-level incidence explicitly open -- the same evidentiary
situation another node had just been filed to investigate for a different
refusal. The difference: this one had already shipped the user-facing
consequence.

**A reclassification that adds a cause claim needs evidence about the cause.**
Incidence -- can a real program reach this? -- IS that evidence, so an open
incidence question is not adjacent to such a change; it is the change's missing
premise. **Note precisely what is and is not exposed**: the conditions were
byte-identical and nothing was weakened, so the entire exposure is in what the
message now claims.

## Re-derive a flagged number, and try every metric before calling it wrong

The merge said the census was "the part most worth re-deriving rather than
trusting, given it failed three times on this node." It was wrong a fourth
time: 25 before, not 21; the delta is -9, not -5.

**Establish it is not a metric mismatch before reporting.** Four readings were
measured (all textual, quoted, `.to_string()`, comments); only one produced the
reported after-figure, which fixes the metric and settles the before-figure on
it. **A count discrepancy is worth nothing until you have shown both numbers
are on the same scale.**

**Check the mechanism the author themselves warned about, and say if it is NOT
the cause.** They had flagged multi-line splitting in the same message; the
identifier never splits, so that was not it. Reporting "cause unknown, and I
ruled out the obvious one" is stronger than a guess.

**Two independent reproductions of a DERIVED figure are not independent if both
derive it the same way.** The number was reported as reproduced by two people.
What failed was not diligence but a shared method -- and the work was in fact
complete, so **what is wrong is the recorded evidence for completeness, not the
completeness.** Say that distinction explicitly, or the finding reads as an
accusation about the code. Same mechanism as
[[agreement-is-not-corroboration-when-a-premise-was-inherited]] (independent
confirmation needs independent inputs, and here, an independent method).

Related: a reclassification of a *finding* can be right and still conceal what
it was also evidence of; see
[[a-stack-budget-is-prefix-plus-depth-times-frame-and-must-agree-with-every-other-bound-on-the-recursion]].
