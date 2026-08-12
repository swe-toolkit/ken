---
scope: fleet
audience: (see scope README) — anyone who publishes a prescribed repair into a
  durable artifact: a frame, a finding, a review block, a leader instruction
source: 2026-08-12, `D2k-1c-1` — a confirmed fail-open finding named one
  admitted state and prescribed the check that closes it; the frame published
  the prescription; the implementer measured that the prescription leaves the
  same state reachable by a different route
---

# A fix that closes the named counterexample need not close the class

A ledger's `close()` asserted four containments, each keyed on its own map's
keys, so **nothing checked that a `transitioned` value was a key of `minted`**.
The finding named the admitted state — `transitioned[r] = T` with `T ∉ minted`,
which returns `Ok` with a constructed field whose transport is never consumed —
and prescribed the repair that closes it: `range(transitioned) ⊆ dom(minted)`,
one `contains_key` in a loop that already binds the value.

The finding was right. The prescription was published into the work-package
frame and into the kickoff as *the* repair. **It is insufficient**, and the
implementer measured it rather than arguing it:

> `transitioned[r1] = transitioned[r2] = T`. Two constructed fields transition
> to one minted transport, discharged by that transport's single lawful
> consumption. Both domain loops pass, the new containment passes, `T` is
> consumed. ⇒ **green close, one field forgotten** — the same admitted state,
> reached by a different route.

## The shape

**A named counterexample is a point. A law is a set.** Reading the repair off
the counterexample gives you the weakest predicate that excludes that point,
and the space of nearby points is not thereby empty. Here the property is *a
transition and its transport must name each other* — the agreeing bijection
`minted[transitioned[r]].recognition == r` and its converse. The prescribed
containment is a strictly weaker shadow of that, and injectivity — the thing
the second state violates — falls out of the bijection for free.

⇒ **Derive the law from the property, then check the named state falls out of
it. Never derive the law from the state.** The two produce the same text
surprisingly often, which is why the failure is easy to ship: the containment
*does* close the reported case, and a control written against the reported case
goes green.

## The control that catches it, and why the ordinary one does not

A row built from the finding's own counterexample passes under the insufficient
repair *and* under the sufficient one, so it cannot distinguish them. What
distinguishes them is an **A/B on the prescription itself**:

| mutation | result |
|---|---|
| drop the existence check | red — the named state |
| **drop the agreeing check, i.e. ship exactly the prescribed one-line repair** | red — a *second* state |
| drop the converse | red — a third |

The middle row is the whole measurement: under it the first row still passes and
only the second reds. **That is the prescription being measured as strictly
weaker, rather than asserted to be.** If your grid has no mutation that
reproduces the prescribed fix, you have not tested the prescription — you have
tested the fix you happened to write.

## What this is NOT

Not an argument against prescribing repairs. Naming a concrete repair is what
makes a finding actionable, and the one-line coordinate here was correct about
*where* — a loop that already binds the value, no new traversal. **The defect
was in treating the repair's sufficiency as inherited from the finding's
soundness.** A correct diagnosis and a sufficient fix are separate claims, and
only the first was established.

Also not a reason to widen a fix speculatively. The stronger law was not chosen
because stronger is safer; it was chosen because it is **the property the chain
argument actually rests on**, and each of its three refusals has a distinct row
that reds without it. A fourth check with no failing row would be the opposite
error.

Related:
[[the-evidence-framing-you-route-a-fork-with-can-be-too-weak-to-decide-it]] —
the same increment, and the same failure one step earlier: there a relayed
*population* selected what got measured, here a relayed *repair* selected what
got built. In both, the executing seat re-deriving is what saved it, and in both
that is luck rather than a control.
[[a-mutation-campaign-needs-a-grid-not-a-count]] — the grid above is why the
insufficiency is a measurement and not an opinion.
