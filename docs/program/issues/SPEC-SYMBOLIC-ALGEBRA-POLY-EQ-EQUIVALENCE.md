---
id: SPEC-SYMBOLIC-ALGEBRA-POLY-EQ-EQUIVALENCE
title: "Spec 66 decides semantic equality of polynomial expressions with poly_eq and states that it is preserved by EAdd and EMul, but states no law that poly_eq is an equivalence: §10 names reflexivity, symmetry and transitivity as relational laws and exports none. A client must recompose poly_eq_sound and poly_eq_complete to rewrite in either direction or chain two facts. Rule from a checked D0 at zero trust and state the three laws with a conformance seed"
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

# Canonical polynomial equality is an equivalence

## Objective

Spec 66 states, as public checked laws over
`Algorithm.SymbolicAlgebra.NatPolynomial`, that for all expressions:

- `IsTrue (poly_eq e e)`;
- `IsTrue (poly_eq e1 e2)` gives `IsTrue (poly_eq e2 e1)`;
- `IsTrue (poly_eq e1 e2)` and `IsTrue (poly_eq e2 e3)` give
  `IsTrue (poly_eq e1 e3)`.

## Settled inputs (main `e0690449e`)

- **The public halves exist.** `poly_eq_sound` turns `IsTrue (poly_eq
  e1 e2)` into `Equal Nat (eval env e1) (eval env e2)` at every `env`,
  and `poly_eq_complete` goes back (`NatPolynomial.ken.md:720`,
  `:743`). `sym` and `trans` over `Eq` are public in
  `Core.Logic.Transport` (`:74`, `:81`), and the package already uses
  them.
- **Reflexivity has a private route.** `poly_same_refl` (`:2460`) gives
  `IsTrue (poly_same p p)`, and `poly_eq_complete`'s body uses it.
- **Not yet stated.** Spec 66 §1 to §10 and the package have no
  reflexivity, symmetry or transitivity law for `poly_eq`. §10's last
  paragraph (`66-symbolic-algebra.md:987`) says it exports none.
- **Seeds.** Under `conformance/stdlib/symbolic-algebra/`, next to
  `seed-nat-polynomial-congruence.md`. The fixtures `x0`, `x1`,
  `x0_sum` and `two_times_x0` in
  `seed-nat-polynomial-canonical-equality.md` can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §11 in spec 66 stating the ruled laws by their public types, the three
catalog theorems in `NatPolynomial`, a closed conformance seed under
`conformance/stdlib/symbolic-algebra/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the theorem names, argument order and placement, and
  which helpers stay private. It also confirms that no §1 to §10
  operation or law type changes, and amends §10's closing sentence
  rather than contradicting it.
- **AC-1.** §11 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies all three laws to the
  reused fixtures, chaining `x0_sum` to `two_times_x0` and back by
  symmetry and transitivity.
  - Its refused block pins `IsTrue (poly_eq x0_sum x1)`. Its values at
    the all-ones environment are two and one, so the claim is false.
  - Its accepted same-form control is `IsTrue (poly_eq x0_sum
    two_times_x0)`, so the refusal is the inequality, not the form.
  - The D0 rules the proof form that separates the two rows, as
    `Proved` did for the interval units (Architect evt_5y7dt3m2x9twr).

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise
  beyond those stated.
- A change to an existing §1 to §10 operation or law type.

Checks: 4 fired: each law is written from `poly_eq_sound`,
`poly_eq_complete` and `Transport`'s `sym` and `trans`, each read at
its line. Reflexivity is either `poly_same_refl` on the canonical form
or `poly_eq_complete` with `refl`. The package was grepped for an
existing equivalence law, and there was no hit. 8 fired: the refused
row and its control share the left operand and the claim form, and
differ only in a right operand whose evaluation differs.
