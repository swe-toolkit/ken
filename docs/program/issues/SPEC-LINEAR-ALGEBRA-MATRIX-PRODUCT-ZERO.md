---
id: SPEC-LINEAR-ALGEBRA-MATRIX-PRODUCT-ZERO
title: "Spec 65 §12 gives the zero matrix its additive unit laws and its matrix-vector action, but no public law says what it does to a matrix product. A client multiplying by `matrix_zero` on either side cannot conclude that the product is the zero matrix without unfolding the private `row_product`, `transpose` and `repeat`. Rule from a checked D0 at zero trust public product-annihilation laws on both sides, with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-GEOMETRY-BOX-POINT-SINGLETON]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Linear algebra follows Geometry in the layer rotation. Steward-filed per COORDINATION section 2."
---

# The zero matrix absorbs a product

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports public checked laws that,
for every `m`, `n`, `p : Nat`, `a : Vec (Vec Nat n) m` and
`b : Vec (Vec Nat p) n`:

```ken
Equal (Vec (Vec Nat p) m)
  (matrix_product m n p (matrix_zero m n) b) (matrix_zero m p)

Equal (Vec (Vec Nat p) m)
  (matrix_product m n p a (matrix_zero n p)) (matrix_zero m p)
```

Spec 65 states them by their public types. Merged at `d39f2c2a7`
(PR #5213).

## Settled inputs (main `dc9914142`)

- **The operations.** `matrix_product` (`NatMatrix.ken.md:71`) maps
  the private `row_product n p right` (`:1591`) over the left rows.
  `row_product` maps `dot_with n row` (`:1584`) over
  `transpose n p right` (`:64`). `matrix_zero` (`:477`) is the private
  `repeat (Vec Nat n) m (vector_zero n)` (`:1578`).
- **The enablers.** `dot_zero_left` (`:911`) and `dot_zero_right`
  (`:938`) give `dot` against `vector_zero` as `Zero`. The private
  `map_cong` (`:1830`) and `vector_cons_eq` (`:1554`) lift a pointwise
  equation across a vector. §12.2's `matrix_vector_zero_matrix` proof
  is the precedent route.
- **The missing step.** The left law reduces each row to
  `row_product n p b (vector_zero n)`, which is `dot_zero_left` mapped
  over `b`'s columns. The right law needs
  `transpose n p (matrix_zero n p)` to be `matrix_zero p n`. No law
  states that; the private `transpose_zero_repeat` (`:1702`) covers
  only the zero-row case.
- **Not yet stated.** Spec 65 runs §1 to §12 (`65-linear-algebra.md`,
  §12 at `:1069`, file end `:1152`). §12.2's roster is ten operations
  and thirty-six laws, forty-six public names, and none relates
  `matrix_zero` to `matrix_product` or `transpose`.
- **Seeds.** Under `conformance/stdlib/linear-algebra/`, next to
  `seed-matrix-zero-unit.md`, whose `one`, `two`, `pair`, `rows2`, the
  nonzero `a` and the zero-row `empty` can be reused. The package test
  is `crates/ken-elaborator/tests/cat_linear_algebra_matrix_zero_unit.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled laws in NatMatrix, a spec 65 section stating them by their
public types, a closed conformance seed under
`conformance/stdlib/linear-algebra/`, a named package test, and spec
65's status lines and public roster, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the laws' names, argument order and placement;
  - the section number;
  - whether `transpose` of the zero matrix is a third public law or a
    private step.

  It confirms that no §1 to §12 operation or law type changes.
- **AC-1.** The section states each ruled law by its public type and
  advances the public roster. The Spec Leader and CV review it, with a
  Decision.
- **AC-2.** The seed's accepted client applies each law to the nonzero
  `2 × 2` `a` and to a zero-row matrix.
  - Its refused block claims `matrix_product two two two a
    (identity two)` equals `matrix_zero two two`. That is false,
    because the product is `a`.
  - Its accepted same-form control claims the same equation with
    `matrix_zero two two` in place of `identity two`, so the refusal is
    the nonzero factor, not the form.
  - The D0 rules the proof form that separates the two rows; per §12.2,
    `Proved` does not discriminate an `Equal (Vec …)` goal.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §12 operation or law type.

Checks: 4 fired: the laws are written from public `matrix_product`,
`matrix_zero`, `transpose`, `identity`, `dot_zero_left` and
`dot_zero_right`, each read at its line, and the private route through
`row_product`, `repeat` and `map_cong`; the one missing step,
`transpose` of the zero matrix, is named after reading
`transpose_zero_repeat`. 3 fired: NatMatrix's 46 public names and spec
65 §1 to §12 were read for a law relating `matrix_zero` to
`matrix_product`; none exists. 8 fired: the refused row and its
control share `a`, the product form and the claimed `matrix_zero two
two`, and differ only in the right factor, `identity two` against
`matrix_zero two two`.
