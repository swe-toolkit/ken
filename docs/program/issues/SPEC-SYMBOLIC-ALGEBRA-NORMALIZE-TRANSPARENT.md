---
id: SPEC-SYMBOLIC-ALGEBRA-NORMALIZE-TRANSPARENT
title: "Spec 66 states that normalize preserves evaluation and that poly_eq is preserved by derivative and by substitution, but no law lets a client differentiate or substitute into a normal form in place of the original. Rule from a checked D0 at zero trust that both operations agree on an expression and its normal form under poly_eq, for every expression, and state the laws with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Symbolic algebra is next in the layer rotation after Time. Same shape as SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-ABSORPTION. Steward-filed per COORDINATION section 2."
---

# Normalization is transparent to derivative and substitution

## Objective

Spec 66 states, as public checked laws over
`Algorithm.SymbolicAlgebra.NatPolynomial`, that for every `v`, `s` and
`e`:

- `poly_eq (deriv v (normalize e)) (deriv v e)` is true;
- `poly_eq (substitute s (normalize e)) (substitute s e)` is true.

## Settled inputs (spec 66 §1 to §8, main `eb54e9724`)

- **The operations.** `normalize` is `reify (monomials e)`
  (`NatPolynomial.ken.md:74`). `deriv` (`:870`) and `substitute`
  (`:762`) recurse on `Expr`. A normal form is structurally different
  from its source in general, so the laws are stated under `poly_eq`,
  not `Equal Expr`.
- **No mixed law.** §1 relates `normalize` to evaluation only. §5 and
  §3 lift `poly_eq` through `deriv` and `substitute`. No section applies
  either operation to a normal form.
- **Why it holds.** Each law composes public laws with no premise:
  - `poly_eq_complete (normalize e) e (λenv. normalize_sound env e)`
    (`:738`, `:76`) gives `IsTrue (poly_eq (normalize e) e)`;
  - `poly_eq_deriv v (normalize e) e` (`:923`) applied to it gives the
    first law;
  - `poly_eq_substitute s (normalize e) e` (`:794`) applied to it gives
    the second.
- **Seeds.** Under `conformance/stdlib/symbolic-algebra/`, next to
  `seed-derivative-substitution.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §9 in spec 66 stating the ruled laws by their public types, a closed
conformance seed under `conformance/stdlib/symbolic-algebra/`, and the
status lines, all in one candidate. The catalog half travels in the
candidate when AC-0 rules it. Otherwise it is a separate Foundation WP
built from the D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the names and the argument order;
  - whether `IsTrue (poly_eq (normalize e) e)` is itself exported;
  - whether value-level forms (by `deriv_respects_eval` and
    `eval_substitute`) are also public.

  It also confirms that no §1 to §8 operation or law type changes.
- **AC-1.** §9 states each ruled law by its public type. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses
  `e = EMul (EAdd (EVar 0) (EConst 1)) (EVar 1)`, whose normal form is
  not structurally `e`.
  - It applies each law and checks each conclusion independently by
    `Proved`.
  - Its refused block pins one false mixed claim with the variable moved:
    `poly_eq (deriv 0 (normalize e)) (deriv 1 e)`, which is `x1` against
    `x0 + 1`. It also carries the accepted same-shape control with
    `deriv 0` on both sides, and an independent `False` value: at the
    all-zero environment the two sides evaluate to 0 and 1.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §8 operation or law type.

Checks: 4 fired: both compositions were written against the public types
of `normalize_sound`, `poly_eq_complete`, `poly_eq_deriv` and
`poly_eq_substitute` on `eb54e9724`. `normalize_sound env e` has the
exact type `poly_eq_complete`'s `h` needs, and nothing is left over. 8
fired: the refused row has the accepted law's shape with only the
derivative variable moved, and for the seed's `e` the two sides differ
at the all-zero environment. A kernel that accepted it would show the
law was not checked.
