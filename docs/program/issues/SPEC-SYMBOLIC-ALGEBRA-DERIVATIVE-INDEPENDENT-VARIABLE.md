---
id: SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-INDEPENDENT-VARIABLE
title: "Spec 66 differentiates an Expr and states that the derivative respects evaluation and that two derivatives commute, but states no law for a variable the expression's value does not depend on. Rule from a checked D0 at zero trust that such a derivative evaluates to zero, and state it as a section 7 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Symbolic algebra is next in the layer rotation after Time. Same shape as SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-COMMUTATION. Steward-filed per COORDINATION section 2."
---

# A derivative in an unused variable evaluates to zero

## Objective

Spec 66 states, as a public checked law over
`Algorithm.SymbolicAlgebra.NatPolynomial`, that if `eval env e` does
not depend on variable `v`, then `eval env (deriv v e)` is `Zero` for
every environment.

## Settled inputs (spec 66 §5 and §6, main `9b6561d3b`)

- **The operation.** `deriv` and its four defining equations are in
  §5 (`NatPolynomial.ken.md:865`). §5.1 and §6 export
  `deriv_respects_eval`, `poly_eq_deriv` and `deriv_commute`. None of
  them concludes a value for a single derivative.
- **Why it holds.** The private Taylor identity `deriv_taylor`
  (`:2250`) writes `eval (bump_env v t env) e` as
  `eval env e + t · (eval env (deriv v e) + t · taylor_rem …)`. If
  the bump leaves the value unchanged, the private `deriv_extract`
  (`:2402`) with `d2 = Zero`, `r2 = Zero` and `t = Suc d1` gives
  `d1 = Zero`. This is the same extraction §5.2 already uses.
- **The premise is open.** `bump_env` and `nat_same` are private
  (`:2232`, `:1057`), so the public law cannot name them. One form
  quantifies over two environments that agree at every variable
  other than `v`. AC-0 rules the form.
- **Seeds.** Under `conformance/stdlib/symbolic-algebra/`, next to
  `seed-derivative-commutation.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §7 in spec 66 stating the ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/symbolic-algebra/`, with the
status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the name, the public form of the independence premise
  in public names only, and which helpers stay private. If the premise
  needs a new public operation, the ruling names it and its defining
  equations.
- **AC-1.** The new section states the law by its public type. The Spec
  leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to a closed
  expression in two variables, one of which is unused but still
  occurs inside a product, for example `EMul (EVar 1) (EConst Zero)`
  in variable 1. Its refused block pins one false proposition on the
  same inputs: the derivative in the variable that is used, claimed to
  be `Zero`.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  that names a private helper.
- A change to an existing §1 to §6 law or operation.

Checks: 4 fired: the extraction is written against `deriv_taylor` and
`deriv_extract` as they appear at `:2250` and `:2402` on main; the
public premise is deliberately left to AC-0 because the code's own
bump is private. 8 fired: the AC-2 accepted row has the unused variable
occurring inside a product, which a law keyed on syntactic absence
would not cover, and the refused row uses the variable the value
depends on, which the law must not cover.
