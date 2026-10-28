---
id: SPEC-OPTIMIZATION-ARGMAX-COST-ORDER-INVARIANCE
title: "Spec 72 section 1.5 states that argmin is unchanged when its cost is composed with a function that preserves and reflects the strict Nat order, but section 3 has no such law for argmax. Rule from a checked D0 at zero trust the argmax counterpart, including on ties, and state it with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization is next in the layer rotation after Symbolic algebra. The §3 mirror of SPEC-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE. Steward-filed per COORDINATION section 2."
---

# Argmax is unchanged by an order-preserving cost transform

## Objective

Spec 72 states, as a public checked law over
`Algorithm.Optimization.Argmin`, that `argmax a (comp a Nat Nat f cost) xs`
equals `argmax a cost xs` for every nonempty `xs`, whenever `f` preserves
and reflects the strict Nat order.

## Settled inputs (spec 72 §1.5 and §3, main `2ab732989`)

- **The argmin law exists.** §1.5 states `argmin_cost_order_invariant`
  with the premise
  `same_order : (i j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j)`
  (`Argmin.ken.md:131`). Its proof rests on the private
  `select_from_cost_order` (`:409`), which fixes `better = lt_nat`.
- **The argmax operation.** `argmax` is `argmax_from`, that is
  `select_from` with `better = costlier`, where
  `costlier challenger incumbent = lt_nat incumbent challenger`
  (`:283` to `:286`). It keeps the head unless the tail's choice is
  strictly costlier, so ties go to the earlier candidate (§3.1).
- **No two-cost law in §3.** §3.2 and §3.4 state each law for one `cost`.
  §3.3 lists what is deferred and does not mention cost transforms.
- **Why it should hold.** `costlier` only swaps `lt_nat`'s arguments, so the
  same `same_order` premise, instantiated at the swapped pair, gives every
  `select_from` comparison under `f ∘ cost` the same Boolean as under `cost`.
  The D0 measures one thing: whether `select_from_cost_order` can be
  generalized over `better` behind a private premise, or whether a mirror
  helper is needed. Either way, the argmin law's public type must not
  change.
- **Why the premise is needed.** A non-injective `f` merges distinct costs
  into a tie, and the tie rule then keeps an earlier candidate. With
  `f = λc. Zero`, `argmax` returns the head for every input.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-argmin-cost-order-invariance.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §3 subsection in spec 72 stating the ruled law by its public type, and a
closed conformance seed under `conformance/stdlib/optimization/`, with the
status lines changed in the same candidate. The catalog half travels in the
candidate when AC-0 rules it, as it did for SPEC-TIME and SPEC-SYMBOLIC.
Otherwise it is a separate Foundation WP built from the D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base, with
  rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity set. It
  rules:
  - the name;
  - the premise, in public names only;
  - which helpers stay private.

  It also confirms that §1.5's public type and proof boundary are unchanged.
- **AC-1.** The new subsection states the law by its public type. The Spec
  leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client has four labelled candidates whose
  greatest cost is tied away from the head, for example costs `5, 7, 2, 7`,
  and `f` that adds three.
  - It applies the law.
  - It checks both selections independently by `Proved` to be the earlier
    tied label, which is the second candidate.
  - Its refused block pins one false proposition on the same candidates:
    with `f = λc. Zero`, the selection claimed equal to the cost-only
    `argmax`.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise that
  names a private helper.
- A change to an existing §1 to §3 law or operation, including §1.5's
  public type.

Checks: 4 fired: the comparison direction and the tie rule are read from
`argmax_from`, `costlier` and `select_from` on `2ab732989`, not from §3.1
prose, and the argmin helper's fixed `better = lt_nat` was read at `:409`.
8 fired: the accepted row's maximum is tied away from the head, so a law
that ignored ties would not be exercised by an untied input, and the
refused row's `f` merges costs, which the premise must exclude.
