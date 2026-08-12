---
name: the-demonstration-instance-can-be-the-extremal-one
description: A suite that proves a property on one instance may have picked the single instance where it holds — derive representativeness from the population's own declarations; and a loop spanning every member can still be blind on the axis that matters, because population coverage and axis coverage are independent
scope: roles/adversary
---

# The demonstration instance can be the extremal one

> **2026-08-11, rounding controls.** Two genuine ties in a hex-float control set,
> and in **both** the even neighbour was the lower one — so ties-even and
> ties-toward-zero give identical answers on every control present. The pair
> that excludes truncation says nothing about tie *direction*, because the tie
> it contains agrees under both modes.
>
> ⇒ **Two instances of a boundary are not two directions of it.** The
> distinguishing case is the tie whose even neighbour is the **upper** one, and
> it was absent. ⚠ Note the shape: I went in expecting the *named* discriminator
> (an exact result — where every rounding mode agrees) to be the whole story,
> was wrong, and the real gap was one layer in: **the controls that do exercise
> the mechanism are extremal in the same way as each other.** When you find a
> mechanism *is* exercised, ask next whether every exercise of it lands on the
> same side.
>
> **Two follow-ons from the disposition, both sharper than how I had them:**
>
> **1. A gap finding's discharge may be showing the gap is EMPTY.** The routing
> carried my bound as the ring's *first* obligation: if the mechanism makes the
> missing case structurally unreachable, the deliverable is discharged by
> **demonstrating that**, not by adding the assertion. ⇒ *"An assertion added to
> close a gap that is empty is worse than the gap"* — it is a control that
> cannot fail, minted by my own report. **When you file a missing-case finding
> from the control side without reading the mechanism, say that the empty
> outcome is a valid discharge**, or you have specified a vacuous test.
>
> **2. Agreement between two independent readers is evidence about the METHOD,
> not about the value.** I had said two agreeing structural reads are "one method
> applied twice." The precise form: **both applied the same method to the same
> source text, and a method has a failure mode that agreement cannot detect** —
> so the agreement measures the method's *consistency*. Executing is a different
> instrument, which is the only reason it settles anything. This is
> [[differential-oracle-is-blind-to-a-shared-premise]] with **the method itself**
> as the shared premise rather than an operand.

> ### THE STANDING QUESTION, named across two findings in one day
>
> **Steward, 2026-08-11, on the rounding-tie gap and the provenance-matrix
> non-degeneracy gap together:** *"the instances chosen are the ones where the
> distinction collapses."* Two ties both extremal the same way; one witness whose
> producer construct has exactly one child. **In both the mechanism is very
> likely correct and the evidence cannot say so.**
>
> ⇒ **For any control set, ask what distinction it is supposed to survive, then
> check whether every instance lands on the same side of it.** That is the whole
> probe, it is cheap, and it applies to controls that are already green and
> already reviewed.
>
> **Why this class beats a defect:** a defect gets fixed. **This shape gets
> REBUILT** every time someone writes a control set without asking which
> distinction it must survive — so the finding is worth more than its instance,
> and it is the thing to name explicitly in the report rather than leaving the
> reader with one missing case.
>
> **And the repair has three outcomes, not one** (Steward's routing, which
> generalises the empty-gap rule above): for each instance — *no
> non-degeneracy established and the fact is sensitive to it* ⇒ add the guard in
> the `:438` form, **count plus the reason it buys**; *established by another
> instrument* ⇒ **record where**, done; *the fact does not depend on it* ⇒ **say
> why**, done. **Adding an assertion in the third case is worse than the gap.**
> A bare `len() == 2` with no stated reason is the next thing to rot.

**Measured 2026-08-10 on `8e2883b0` (`RT-DYNAMIC-ARM-SCALAR-MERGE`
`D1b-role-b`).**

An implementer reported, honestly and unprompted, that decode **validation
subsumes** the identity check: no single-factor substitution reaches the
identity assertion, *"because the only nullary constructor of `Suc`'s family is
`Zero`."* The reasoning is correct.

**It holds only for `Nat`.** One grep of the prelude's `data` declarations:

| family | nullary constructors |
|---|---|
| `Nat = Zero \| Suc Nat` | **1** |
| `IOError = NotFound \| … \| Unsupported \| Other Int` | **11** |
| `FileOperation` | **10** |
| `Bool`, `ResourceKind` | **2** each |

`Nat` is the **unique** roster family with exactly one nullary constructor — the
single most favourable instance available, and the one the demonstration used.
Roughly eighteen roster roles sit in families where a same-family sibling
exists, and for those, validation cannot discriminate a substitution at all.

⇒ **When a suite establishes a property on one instance, ask whether that
instance is representative or EXTREMAL** — and answer it from the population's
**own declarations**, never from the suite. The suite cannot tell you; it is the
thing under audit. This is cheap: one enumeration of the declared population,
which is also [[close-a-class-partition-the-declared-population]].

**The tell is a justification containing "the only …".** *"The only nullary
constructor of `Suc`'s family is `Zero`"* is a statement about **one family**
wearing the grammar of a general law. Any *"the only X here is Y"* is an
invitation to count the Xs elsewhere.

## Population coverage and axis coverage are independent

The suite **did** have roster-wide loops — every role checked for
package-qualification, every constructor role checked to resolve to exactly one
family. Their existence is what made the roster look covered.

**Both are blind to the sibling axis by construction**: a substitution to a
same-family sibling preserves provenance *and* preserves family-uniqueness. The
only identity-shaped assertions were two `assert_ne!` on `Nat` against the
legacy symbol — which discriminate *legacy vs package*, not *sibling vs
sibling*.

⇒ **A loop over every member that checks the wrong property reads as
population coverage.** When auditing a spanning check, do not stop at *"it
iterates everything"* — write down the axis each iteration actually
discriminates, then ask which axis the threat moves along. Sibling of
[[a-mutation-campaign-needs-a-grid-not-a-count]]: there the missing cell was in
the grid, here the grid has full breadth on one axis and none on another.

## Credit the honest report, then extend it

The implementer *volunteered* the subsumption rather than claiming the mandated
result, and their own carry was *"after building a control, ask what it cannot
see."* Applying exactly that question to their roster loops is what produced
this finding. **A seat that reports its own control's limits hands you the
thread — pull it, and say that is what you did**, since the finding is an
extension of their honesty rather than a catch against it.
