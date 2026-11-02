---
id: SPEC-OPTIMIZATION-ARGMIN-OPTION-COST-ORDER-INVARIANCE
title: "Spec 72 §1.5 states that argmin is unchanged under a strict-order-preserving cost transform, and §1.4 carries member and least over to the Option selector, but no law carries the invariance to argmin_option. A client with a possibly empty list must split on Nil and Cons and lift argmin_cost_order_invariant through Some by hand. Rule from a checked D0 at zero trust and state the law with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization follows Symbolic algebra in the layer rotation. Steward-filed per COORDINATION section 2."
---

# The Option selector is invariant under an order-preserving cost

## Objective

Spec 72 states, as a public checked law over
`Algorithm.Optimization.Argmin`, for every `a`, `cost`, order-preserving
`f` and list `xs`:

```ken
Equal (Option a)
  (argmin_option a (comp a Nat Nat f cost) xs)
  (argmin_option a cost xs)
```

The order-preservation premise is the one `argmin_cost_order_invariant`
already takes: `(i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i)
(f j)) (lt_nat i j)`.

## Settled inputs (main `bddc92ad3`)

- **The neighbours exist.** In `Argmin.ken.md`:
  - `argmin_cost_order_invariant` (`:135`) concludes the NonEmpty form;
  - `argmin_option` (`:188`) is defined by cases;
  - `argmin_option_nil` (`:194`) and `argmin_option_cons` (`:199`) give
    its two reductions;
  - §1.4's `argmin_option_member` (`:207`) and `argmin_option_least`
    (`:212`) are the precedent for carrying a law to the Option form.
  - `comp`, `cong` and `lt_nat` are already imported (`:39`, `:43`,
    `:45`).
- **Not yet stated.** No public theorem concludes the Option form. The
  package, spec 72 and the conformance seeds were grepped for an Option
  cost-order law, with no hit. §1.5 (`72-optimization.md:301` to `:375`)
  speaks only of the NonEmpty selector, and §1.3's deferred list does
  not name this law. Spec 72's sections run to §3.6 (`:968`).
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-argmin-cost-order-invariance.md`, whose label costs and
  order-preserving and flattening transforms can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A spec 72 subsection stating the ruled law by its public type, the
catalog theorem in `Argmin`, a closed conformance seed under
`conformance/stdlib/optimization/`, the public-inventory pin advanced,
and the status lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set, or that measurement assigned to the candidate's package test. It
  rules the theorem name, argument order and placement, the subsection
  number, and whether the `argmax_option` twin travels in this
  candidate or as its own node. It confirms that no existing operation
  or law type changes.
- **AC-1.** The subsection states each ruled law by its public type.
  The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to a list with a
  cost tie and to `Nil`.
  - Its refused block pins the flattening transform, which merges
    distinct costs and so is not order-preserving, as leaving
    `argmin_option` unchanged on that list. The two sides select
    different labels, so the claim is false.
  - Its accepted same-form control is the order-preserving transform on
    the same list, so the refusal is the transform, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or any premise
  beyond order preservation.
- A change to an existing operation or law type.

Checks: 4 fired: the law is written from public `argmin_option`,
`argmin_option_nil`, `argmin_option_cons` and
`argmin_cost_order_invariant`, each read at its line, with `comp`,
`cong` and `lt_nat` already imported; the package, spec 72 and seeds
were grepped for an Option cost-order law, with no hit. 8 fired: the
refused row and its control share the list and the claim form and
differ only in whether the transform preserves the order.
