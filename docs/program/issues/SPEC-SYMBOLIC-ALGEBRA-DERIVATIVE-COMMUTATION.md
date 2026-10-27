---
id: SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-COMMUTATION
title: "Spec 66 section 5 differentiates an Expr in one variable and states that the derivative respects evaluation, but states no law relating derivatives in two variables. Rule that partial derivatives commute under evaluation from a checked D0 at zero trust, and state it as a section 6 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Symbolic algebra is next in the layer rotation after Time. Same shape as SPEC-TIME-WINDOW-COMPOSITION. Steward-filed per COORDINATION section 2."
---

# Partial derivatives commute under evaluation

## Objective

Spec 66 states, as a public checked law over
`Algorithm.SymbolicAlgebra.NatPolynomial`, that differentiating in `w`
then `v` and differentiating in `v` then `w` give expressions that
evaluate equal under every environment.

## Settled inputs (spec 66 §1 to §5, main `2f78d45b8`)

- **The derivative.** `deriv v e` is structural (§5): a constant goes to
  `EConst Zero`, a variable to `var_deriv (nat_same v w)`, a sum to the
  sum of derivatives, and a product by the product rule in a fixed
  order.
- **No two-variable law.** §5.1 states `deriv_respects_eval` and
  `poly_eq_deriv`, both in one variable. No section relates
  `deriv v (deriv w e)` to `deriv w (deriv v e)`.
- **Why it can be unconditional.** The two sides differ syntactically
  in the product case, so the law is pointwise `Equal Nat` of `eval`,
  not structural `Expr` equality. A variable's derivative is a
  constant, whose derivative is `EConst Zero` on both sides. The
  product case needs only the checked natural addition and
  multiplication laws that §5.2 already uses.
- **Seeds.** Under `conformance/stdlib/symbolic-algebra/`, next to the
  §5 seed.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §6 in spec 66 stating each ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/symbolic-algebra/`, with the
status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  name, argument order and public type, whether a `poly_eq` corollary
  is stated as in §5.1, and which helpers stay private.
- **AC-1.** The §6 states each ruled law by its public type. The Spec
  leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to a closed
  product such as `x * y` in variables 0 and 1. Its refused block is
  pinned to one false proposition: structural `Equal Expr` of the two
  orders on an expression where their syntax differs.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise.
- A change to an existing §1 to §5 law or operation.
