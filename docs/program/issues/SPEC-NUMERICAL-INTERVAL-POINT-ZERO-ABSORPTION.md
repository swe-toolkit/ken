---
id: SPEC-NUMERICAL-INTERVAL-POINT-ZERO-ABSORPTION
title: "Spec 68 §10 gives point zero as the left unit of interval_add and point one as the left unit of interval_mul, but states no law that point zero absorbs interval_mul: the product of point zero with any interval is the point zero interval by endpoint-value inclusion. A client must unfold interval_mul and transport Rational's public zero_l for rational_mul through the package's private equiv_leq to rewrite it. Rule from a checked D0 at zero trust and state both inclusion directions with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Numerical follows Graphs in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Point zero absorbs interval multiplication

## Objective

Spec 68 states, as public checked laws over
`Algorithm.Numeric.RationalInterval`, for every `i : Interval`:

- `IsTrue (interval_inside (interval_mul (interval_point rational_zero)
  i) (interval_point rational_zero))`;
- `IsTrue (interval_inside (interval_point rational_zero) (interval_mul
  (interval_point rational_zero) i))`.

## Settled inputs (main `db5662e05`)

- **The neighbours exist.** `interval_mul` acts componentwise
  (`RationalInterval.ken.md:183`), `interval_point` has equal endpoints
  (`:191`), and `interval_inside` compares stored endpoint values
  (`:345`). The §10 laws `interval_mul_one_l_inside` and
  `interval_mul_one_l_rev_inside` (`:1215`, `:1233`) are the same proof
  shape with `one_l` in place of `zero_l`.
- **The rational fact is public.** `zero_l for rational_mul`
  (`Data/Numeric/Rational.ken.md:475`) concludes `rational_equiv
  (rational_mul rational_zero x) rational_zero`. `equiv_leq` (`:1251`
  of the interval package) stays private.
- **Not yet stated.** No public theorem in the package concludes either
  equation. §10 (`68-numerical.md:1082` to `:1092`) uses `interval_mul
  (interval_point rational_zero) i` only as a wrong-unit refusal, and
  records that `IsTrue (interval_inside …)` closed claims take `Proved`,
  since `Refl` is refused even for a true claim of that shape.
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-point-units.md`, whose `half`, `i = [1/2, 1]` and
  `inverted = [1, 1/2]` can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §11 in spec 68 stating the ruled laws by their public types, the
catalog theorems in `RationalInterval`, a closed conformance seed under
`conformance/stdlib/numerical/`, the public-inventory pin advanced, and
the status lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the theorem names, argument order and placement, and
  whether a right-zero form is exported or left as a composition with
  the §5 multiplication commutativity law, as §10 did for the units. It
  confirms that no §1 to §10 operation or law type changes.
- **AC-1.** §11 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies both laws to `i` and to
  `inverted`.
  - Its refused block pins the product of point zero with `i` as inside
    `interval_point rational_one`. The product's endpoints are zero and
    the target's lower bound is one, so the claim is false.
  - Its accepted same-form control is the same product inside
    `interval_point rational_zero`, so the refusal is the target point,
    not the form.
  - The D0 rules the proof form that separates the two rows; §10
    measured `Proved` for this goal shape, and the D0 confirms it here.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §10 operation or law type.

Checks: 4 fired: each law is written from public `interval_mul`,
`interval_point`, `interval_inside` and Rational's `zero_l for
rational_mul`, each read at its line; the package and spec 68 were
grepped for a public law concluding either inclusion, with no hit. 8
fired: the refused row and its control share the product and the claim
form and differ only in the target point, whose lower bound is one for
the refusal and zero for the control.
