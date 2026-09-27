---
name: two-layers-can-disagree-because-the-clause-they-both-cite-mandates-it
description: I found the checked prelude returning success where the host returns an error on the identical input, with both layers citing the same spec section — the textbook shape of a defect — and the spec turned out to mandate exactly that, the checked answer plus the host check as defense in depth; read the clause both layers cite before filing a cross-layer divergence
metadata:
  type: feedback
---

# Two layers can disagree because the clause they both cite mandates it

**Measured 2026-08-17 on `f9dd79f52`.** A candidate added an admission ladder to
the checked prelude. At one exact point the two layers give opposite
dispositions on identical input:

| layer | `effective == 0` |
|---|---|
| checked prelude (new) | `Ok ReadEof` -- success |
| host `checked_buffer_range` | `InvalidBounds` -- error |

**Both cite the same spec section in their own comments.** Success versus error,
same input, same cited authority: this is the shape I file. I had it queued as
the lead finding.

**The spec mandates both.** It specifies the checked answer *"without emitting a
private read operation or visiting the host"*, names the witnessing coordinates
by literal value, **and separately requires the host to repeat its range
validation as defense in depth**, stating that the checked handle *"does not
replace those checks."* ⇒ ***The host branch becoming unreachable from checked
source is the design landing, not the guard going vacuous.***

## THE RULE

***Before filing a cross-layer divergence, open the clause the layers cite --
not the neighbouring code, the clause.*** A defense-in-depth architecture makes
"inner layer answers, outer layer would have answered differently" the **normal**
state, so the divergence carries no signal by itself. The question is only ever
whether the *outer* answer is the one the spec assigns to that input.

**The tell that should have slowed me down:** the checked layer's arithmetic was
a **line-for-line transcription** of the host's -- same `min`, same
`start > capacity` test, same zero test -- diverging at exactly one arm. Code
that mirrors another layer that closely was written *from* something, and the
something is worth reading before assuming the one difference is an accident.

Note this cuts the opposite way from
the case of a guard built to a ruling that refuses: there the
built artifact contradicted the ruling; here it implemented it, and only my
reading of the ruling was missing. **Both are settled by opening the ruling.**

## WHAT SURVIVED WAS THE COVERAGE, NOT THE BEHAVIOUR

The pass was not wasted: the branch is spec-correct **and has no executing test**
-- see
[[deleting-a-test-row-leaves-its-fixture-programs-in-the-tree]]. ***When the spec
refutes a behavioural finding, ask immediately whether the now-blessed behaviour
is witnessed anywhere*** -- a clause precise enough to refute you is usually
precise enough to be an untested obligation, and it names its own witnesses.
