---
id: SPEC-SYMBOLIC-ALGEBRA-DERIV-CONSTANT-FACTOR
title: "Spec 66 differentiates EMul by the product rule and decides semantic equality with poly_eq, but states no constant-factor law: the derivative of a constant times an expression is canonically equal to the constant times the expression's derivative. deriv leaves a zero summand, EAdd (EMul (EConst 0) e) (EMul (EConst k) (deriv v e)), which a client must clear by evaluating both sides and calling poly_eq_complete. Rule from a checked D0 at zero trust and state the law with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Symbolic algebra follows Time in the layer rotation. Steward-filed per COORDINATION section 2."
---

# A constant factor passes through the derivative

## Objective

Spec 66 states, as a public checked law over
`Algorithm.SymbolicAlgebra.NatPolynomial`, for all `v`, `k` and `e`:

```ken
IsTrue (poly_eq (deriv v (EMul (EConst k) e)) (EMul (EConst k) (deriv v e)))
```

## Settled inputs (main `80525e894`)

- **The neighbours exist.** `deriv` (`NatPolynomial.ken.md:942`) sends
  `EConst c` to `EConst Zero` and `EMul a b` to
  `EAdd (EMul (deriv v a) b) (EMul a (deriv v b))`. `poly_eq` (`:674`)
  and `poly_eq_complete` (`:744`) are public, so the law follows from
  equality of `eval` (`:106`) under every environment.
- **Not yet stated.** No public theorem in the package concludes a
  constant-factor form. §7's `deriv_independent` (`:1098`) concludes
  that a derivative evaluates to zero, not a factor law. Spec 66 ends at
  §11 (`66-symbolic-algebra.md:993`).
- **Seeds.** Under `conformance/stdlib/symbolic-algebra/`, next to
  `seed-nat-polynomial-derivative.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §12 in spec 66 stating the ruled law by its public type, the catalog
theorem in `NatPolynomial`, a closed conformance seed under
`conformance/stdlib/symbolic-algebra/`, the public-inventory pin
advanced, and the status lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the theorem name, argument order and placement, and
  whether a right-factor form `EMul e (EConst k)` is exported or left as
  a composition with `poly_eq_mul` and `poly_eq_trans`. It confirms that
  no §1 to §11 operation or law type changes.
- **AC-1.** §12 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law at `k = 2` to
  `EVar 0` and to a product of two variables.
  - Its refused block pins `deriv 0 (EMul (EConst 2) (EVar 0))` as
    canonically equal to `EMul (EConst 2) (EVar 0)`, which drops the
    derivative, so the claim is false.
  - Its accepted same-form control is the same left side against
    `EMul (EConst 2) (deriv 0 (EVar 0))`, so the refusal is the dropped
    derivative, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §11 operation or law type.

Checks: 4 fired: the law is written from public `deriv`, `poly_eq`,
`poly_eq_complete` and `eval`, each read at its line; the package and
spec 66 were grepped for a constant-factor law, with no hit. 8 fired:
the refused row and its control share the left side and the claim form
and differ only in whether the right side keeps the derivative.
