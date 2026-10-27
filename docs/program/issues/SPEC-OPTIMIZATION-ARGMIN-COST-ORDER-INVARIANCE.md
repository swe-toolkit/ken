---
id: SPEC-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE
title: "Spec 72 section 1 states what argmin selects for one cost function, but no law relates two cost functions. Rule from a checked D0 at zero trust that composing the cost with a function that preserves and reflects the strict Nat order selects the same candidate, including on ties, and state it with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization is next in the layer rotation after Symbolic algebra. Same shape as SPEC-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY. Steward-filed per COORDINATION section 2."
---

# Argmin is unchanged by an order-preserving cost transform

## Objective

Spec 72 states, as a public checked law over
`Algorithm.Optimization.Argmin`, that `argmin a (comp a Nat Nat f cost)
xs` equals `argmin a cost xs` for every nonempty `xs`, whenever `f`
preserves and reflects the strict Nat order.

## Settled inputs (spec 72 §1, main `9e1d6e9cd`)

- **The operation.** `argmin` is the private `select_from` with
  `better = lt_nat` (`Argmin.ken.md:53`, `:269`, `:277`). Each step
  compares the tail's choice with the head by
  `lt_nat (cost tail_choice) (cost x)` and keeps the head unless the
  comparison is `True`. That is the earliest-on-tie rule (§1.1).
- **No two-cost law.** §1.2 and §1.4 state membership, leastness, the
  tie rule and the one-step laws `argmin_keeps_head` and
  `argmin_takes_tail` (`:115`, `:121`), each for one `cost`.
- **Why it holds.** If `lt_nat (f i) (f j) = lt_nat i j` for all `i`
  and `j`, every comparison `select_from` makes under `f ∘ cost` has
  the same Boolean as under `cost`. The selection then follows the same
  path, ties included. Strict monotonicity alone also suffices on Nat,
  because the order is total, but the proof is longer. AC-0 rules which
  premise is public.
- **Why the premise is needed.** A non-injective `f` merges distinct
  costs into a tie, and the tie rule then keeps an earlier candidate.
  With `f = λc. Zero`, `argmin` returns the head for every input.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-argmin.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §1 subsection in spec 72 stating the ruled law by its public type,
and a closed conformance seed under `conformance/stdlib/optimization/`,
with the status lines changed in the same candidate. The catalog export
is a separate Foundation WP built from the AC-0 D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the name, the premise (order equality or strict
  monotonicity) in public names only, and which helpers stay private.
- **AC-1.** The new subsection states the law by its public type. The
  Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses four labelled candidates
  whose costs include a tie that is not at the head, for example costs
  `5, 2, 7, 2`, and `f` adding three. It applies the law, and checks
  both selections independently by `Proved` to be the earlier tied
  label (AC-0 `evt_64h89e05agzat`: a lambda in type position and `Refl`
  on a closed evaluation are both refused). Its refused block pins one false proposition on the same
  candidates: with `f = λc. Zero`, the selection claimed equal to the
  cost-only `argmin`.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  that names a private helper.
- A change to an existing §1 to §3 law or operation.

Checks: 4 fired: the comparison direction and the tie rule are read
from `select_from` and `argmin_keeps_head` on `9e1d6e9cd`, not from §1.1
prose. 8 fired: the accepted row has a tie away from the head, so a
law that ignored ties would not be exercised by an untied input, and
the refused row's `f` merges costs, which the premise must exclude.
